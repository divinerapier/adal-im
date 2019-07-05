use crate::protocol::BinaryProtocol;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use etcd::kv;
use futures::future::Future;

use hyper::client::connect::HttpConnector;

pub struct UserData {
    is_local: bool,
    // if is_local, client connects to user directly, otherwise,
    // client connects to the server
    client: BinaryProtocol,
    login: u64,
    keepalive: u64,
    // server addr , user in use
    server_addr: String,
}

pub struct Data {
    server_list: HashMap<String, BinaryProtocol>,
    user_client: HashMap<u64, UserData>,
    last_sync_time: u64,
    local_address: String,
    client: etcd::Client<HttpConnector>,
}

pub struct SyncData(Arc<RwLock<Data>>);

impl SyncData {
    pub fn new() -> SyncData {
        SyncData(Arc::new(RwLock::new(Data {
            server_list: HashMap::new(),
            user_client: HashMap::new(),
            last_sync_time: Self::now(),
            local_address: crate::network::local_ip().unwrap(),
            client: etcd::Client::new(&["http://localhost:2379"], None).unwrap(),
        })))
    }

    pub fn get_local_server_address(&self) -> String {
        let data = self.0.clone();
        let data = data.read().unwrap();
        data.local_address.clone()
    }

    pub fn is_local_user(&self, user: u64) -> bool {
        let data = self.0.clone();
        let data = data.read().unwrap();
        match data.user_client.get(&user) {
            None => false,
            Some(a) => a.is_local,
        }
    }

    fn load_servers_loop(&self) {
        let data = self.0.clone();
        std::thread::spawn(move || loop {
            {
                let read_data = data.read().unwrap();
                kv::get(
                    &read_data.client,
                    "/adal/servers",
                    kv::GetOptions::default(),
                )
                .and_then(|response| {
                    let _a = response.data.node.value;
                    Ok(())
                });
            }
        });
    }

    fn load_servers(&mut self) {}

    fn hack_servers(&mut self) {
        let data = self.0.clone();
        std::thread::spawn(move || loop {
            {
                let mut data = data.write().unwrap();
                if data.server_list.len() == 2 {
                    println!("connect to servers successfully.");
                    return;
                }
                let mut server_list = HashMap::new();
                let addresses = vec!["172.21.20.134:6810", "172.21.20.134:6811"];
                let mut has_errors = false;
                for address in addresses {
                    let conn = BinaryProtocol::with_address(address);
                    match conn {
                        Ok(c) => {
                            server_list.insert(address.to_owned(), c);
                        }
                        Err(_) => has_errors = true,
                    };
                }
                if !has_errors {
                    data.server_list = server_list;
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(5));
        });
    }

    pub fn get_connnection(&self, address: &str) -> Option<BinaryProtocol> {
        let data = self.0.read().unwrap();
        match data.server_list.get(address) {
            None => None,
            Some(c) => Some(c.try_clone()),
        }
    }

    pub fn user_login(&mut self, user: u64, bp: BinaryProtocol) {
        let data = self.0.clone();
        let mut data = data.write().unwrap();
        let local_address = data.local_address.clone();
        data.user_client.insert(
            user,
            UserData {
                is_local: true,
                client: bp,
                login: Self::now(),
                keepalive: Self::now(),
                server_addr: local_address,
            },
        );
    }

    pub fn add_user_route(&mut self, user: u64, bp: BinaryProtocol, user_server: &str) {
        let data = self.0.clone();
        let mut data = data.write().unwrap();
        data.user_client.insert(
            user,
            UserData {
                is_local: false,
                client: bp,
                login: Self::now(),
                keepalive: Self::now(),
                server_addr: user_server.to_owned(),
            },
        );
    }

    pub fn user_keepalive(&mut self, user: u64) -> bool {
        let data = self.0.clone();
        let mut data = data.write().unwrap();
        match data.user_client.get_mut(&user) {
            None => false,
            Some(user_data) => {
                user_data.keepalive = Self::now();
                user_data.is_local = true;
                true
            }
        }
    }

    pub fn set_local_addr(&mut self, local_addr: &str) {
        let data = self.0.clone();
        let mut data = data.write().unwrap();
        data.local_address = local_addr.to_owned();
    }

    pub fn get_user_connection(&self, user: u64) -> Option<BinaryProtocol> {
        let data = self.0.read().unwrap();
        let user_data = data.user_client.get(&user);
        match user_data {
            None => None,
            Some(conn) => Some(conn.client.try_clone()),
        }
    }

    pub fn sync(&mut self) {
        self.hack_servers();
        let data = self.0.clone();
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            {
                let mut data = data.write().unwrap();
                for server in &data.server_list {
                    Self::sync_server(
                        server,
                        &data.local_address,
                        &data.user_client,
                        data.last_sync_time,
                    );
                }
                data.last_sync_time = Self::now();
            }
        });
    }

    fn sync_server(
        server: (&String, &BinaryProtocol),
        local_addr: &str,
        user_client: &HashMap<u64, UserData>,
        last_sync_time: u64,
    ) {
        let mut server_conn: BinaryProtocol = server.1.try_clone();
        for (user, user_data) in user_client {
            println!(
                "user: {}, is_local: {}, target_server: {}, user_server: {} , keepalive: {}, last_sync_time: {}",
                user,
                user_data.is_local,
                server.0,
                user_data.server_addr,
                user_data.keepalive,
                last_sync_time,
            );
            if !user_data.is_local {
                continue;
            }
            if server.0.eq(&user_data.server_addr) {
                continue;
            }
            if user_data.keepalive <= last_sync_time {
                continue;
            }
            println!(
                "exchange. user: {}, at: {}, target: {}",
                *user, user_data.server_addr, server.0
            );
            server_conn.exchange_route(*user, local_addr);
        }
    }

    fn now() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs()
    }
}

impl Clone for SyncData {
    fn clone(&self) -> Self {
        SyncData(self.0.clone())
    }
}

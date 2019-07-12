use crate::error::Error;

use etcd::kv::{self, GetOptions, KeyValueInfo};
use etcd::Response;
use futures::future::Future;
use std::sync::Arc;

pub struct Manager {
    conn: Arc<etcd::Client<hyper::client::HttpConnector>>,
    local_address: String,
    parent_key: String,
}

impl Manager {
    pub fn new(endpoints: &[&str], local_address: &str) -> Result<Manager, Error> {
        let conn = etcd::Client::new(endpoints, None)?;
        let manager = Manager {
            conn: Arc::new(conn),
            local_address: local_address.to_owned(),
            parent_key: "/adal/servers".to_owned(),
        };
        manager.ensure_parent();
        Ok(manager)
    }

    fn register(&self) {
        self.ensure_parent();
        self.keepalive();
    }

    fn ensure_parent(&self) {
        let client = self.conn.clone();
        let work = kv::create_dir(&client, &self.parent_key, None);
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(work)
            .is_ok();
    }

    pub fn keepalive(&self) {
        let key = format!("{}/{}", &self.parent_key, self.local_address);
        let work = kv::set(&self.conn, &key, &key, Some(300));
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(work)
            .is_ok();
    }

    pub fn get_servers(&self) {
        let client = self.conn.clone();
        let work = kv::get(
            &client,
            &self.parent_key,
            GetOptions {
                recursive: true,
                sort: false,
                strong_consistency: true,
            },
        )
        .and_then(move |response| {
            // list children
            let response: Response<KeyValueInfo> = response;
            for node in &response.data.node.nodes.unwrap() {
                let node: &kv::Node = node;
                let child = node.key.as_ref().unwrap();
                kv::set(&client, child, child, Some(300));
            }
            Ok(())
        });
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(work)
            .is_ok();
    }
}

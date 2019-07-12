use crate::error::Error;
use etcd::kv;
use etcd::kv::GetOptions;
use etcd::{kv::KeyValueInfo, Response};
use futures::future::Future;
use hyper::client::HttpConnector;
use tokio::runtime::Runtime;

pub struct Client {
    client: etcd::Client<HttpConnector>,
    runtime: Runtime,
}

#[derive(Debug)]
pub struct KV {
    key: String,
    value: String,
    dir: bool,
}

/// Client implements ETCD V2 API
///
/// backup          backup an etcd directory
/// cluster-health  check the health of the etcd cluster
/// mk              make a new key with a given value
/// mkdir           make a new directory
/// rm              remove a key or a directory
/// rmdir           removes the key if it is an empty directory or a key-value pair
/// get             retrieve the value of a key
/// ls              retrieve a directory
/// set             set the value of a key
/// setdir          create a new directory or update an existing directory TTL
/// update          update an existing key with a given value
/// updatedir       update an existing directory
/// watch           watch a key for changes
/// exec-watch      watch a key for changes and exec an executable
/// member          member add, remove and list subcommands
/// user            user add, grant and revoke subcommands
/// role            role add, grant and revoke subcommands
/// auth            overall auth controls
/// help, h         Shows a list of commands or help for one command
///
/// ETCD has two kinds of key: directory and file. Each key is either a directory or
/// a file. A file can be set a value, but directorires can't.
///
impl Client {
    pub fn new(endpoints: &[&str]) -> Result<Client, Error> {
        Ok(Client {
            client: etcd::Client::new(endpoints, None)?,
            runtime: Runtime::new()?,
        })
    }

    pub fn set(
        &mut self,
        key: &str,
        value: &str,
        ttl: Option<u64>,
    ) -> Result<Response<KeyValueInfo>, Error> {
        let client = self.client.clone();
        let fu = kv::set(&client, key, value, ttl);
        Ok(self.runtime.block_on(fu)?)
    }

    pub fn rm(&mut self, key: &str) -> Result<Response<KeyValueInfo>, Error> {
        let client = self.client.clone();
        let fu = kv::delete(&client, key, true);
        Ok(self.runtime.block_on(fu)?)
    }

    pub fn list(&mut self, key: &str, recusive: bool) -> Result<Vec<KV>, Error> {
        let client = self.client.clone();
        let fu = kv::get(
            &client,
            key,
            GetOptions {
                recursive: false,
                sort: false,
                strong_consistency: true,
            },
        )
        .and_then(|response| {
            let response: etcd::Response<kv::KeyValueInfo> = response;
            let mut kv_pairs = vec![];
            if let Some(nodes) = response.data.node.nodes {
                for node in nodes {
                    let node: etcd::kv::Node = node;
                    let mut dir = false;
                    if let Some(is_dir) = node.dir {
                        dir = is_dir;
                    }
                    if let Some(value) = node.value {
                        kv_pairs.push(KV {
                            key: node.key.unwrap(),
                            value: value,
                            dir,
                        });
                    }
                }
            }
            Ok(kv_pairs)
        });
        Ok(self.runtime.block_on(fu)?)
    }
}

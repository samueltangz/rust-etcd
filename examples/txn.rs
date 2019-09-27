extern crate futures;
extern crate tokio;

use etcd_rs::prelude::*;
use etcd_rs::Client;

use futures::Future;

fn main() {
    let client = Client::builder().add_endpoint("127.0.0.1:2379").build();

    let op = client
        .kv()
        .txn(
            TxnRequest::new()
                .when_value("foo", TxnCmp::Equal, "bar")
                .and_then(PutRequest::new(b"result", b"good"))
                .or_else(PutRequest::new(b"result", b"bad")),
        )
        .map_err(|e| println!("exec transaction encouter error: {:?}", e))
        .and_then(move |resp| {
            if resp.is_succeeded() {
                println!("transaction exec succeeded");
            } else {
                println!("transaction exec failed");
            }

            for result in resp.results() {
                println!("transaction result: {:?}", result);
            }

            client
                .kv()
                .get(GetRequest::key(b"result"))
                .map(|resp| {
                    let kv = &resp.kvs()[0];
                    println!(
                        "key: {}, value: {}", 
                        String::from_utf8(kv.key().to_vec()).unwrap(),
                        String::from_utf8(kv.value().to_vec()).unwrap(),
                    ); // result: good or result: bad
                    
                })
                .map_err(|e| println!("failed to fetch key-value 'result': {:?}", e))
        });
    tokio::run(op);
}

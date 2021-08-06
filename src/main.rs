use reqwest;
use std::collections::HashMap;
use std::fmt;
// #[macro_use]
// extern crate serde;
// extern crate serde_derive;
// extern crate reqwest;
// use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let mut map = HashMap::new();
    map.insert("jsonrpc", "2.0");
    map.insert("method", "getBlockByNumber");
    // map.insert("params", format!(b#"[1,"{}", true]"#, 0x1a_u64));
    map.insert("params", r#"[1,"0x1a", true]"#);
    map.insert("id", "1");

    let client = reqwest::Client::new();
    let body = client
        .post("http://127.0.0.1:8545")
        // .send()
        .json(&map)
        .send()
        .await?;

    println!("body = {:?}", body);
    Ok(())
}

use json;
use reqwest;
use std::collections::HashMap;
use std::fmt;
use std::io::Write;
// use std::fs::{OpenOptions, File, write};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
// #[macro_use]
// extern crate serde;
// extern crate serde_derive;
// extern crate reqwest;
// use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("jsonrpc", "2.0");
    map.insert("method", "getBlockByNumber");
    // map.insert("params", format!(b#"[1,"{}", true]"#, 0x1a_u64));
    map.insert("params", r#"[1,"0x1a", true]"#);
    map.insert("id", "1");

    let client = reqwest::Client::new();
    match client.post("http://127.0.0.1:8545").json(&map).send().await {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                // File::create("path")
                match File::create("0x1a.json").await {
                    Ok(mut file) => {
                        file.write_all(text.as_bytes());
                        ()
                    }
                    Err(err) => panic!("Unable to create file {}: {}", "0x1a.json", err,),
                }
            }
            Err(err) => panic!("Unable to create file {}: {}", "0x1a.json", err,),
        },
        Err(err) => panic!("Unable to create file {}: {}", "0x1a.json", err,),
    }

    Ok(())
}
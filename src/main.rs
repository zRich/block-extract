use json::{self, object};
use reqwest;
use serde::de::value::Error;
use std::collections::HashMap;
use std::fmt::{self,format};
// use std::fs::{OpenOptions, File, write};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let post_body = object! {
        jsonrpc: String::from("2.0"),
        method: String::from("getBlockByNumber"),
        id: 1,
        params: [1, "0x1a", true]
    };

    let client = reqwest::Client::new();

    // // println!("content = {:?}", resp.text().await);

    match client
        .post("http://127.0.0.1:8545")
        .body(post_body.dump())
        .send()
        .await
    {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                println!("RESPONSE: {} bytes received {:?} ", text.len(), text);
                match File::create("0x1a.json").await {
                    Ok(mut file) => {
                        file.write_all(text.as_bytes()).await;
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

async fn fetch_block(blockId: u128) -> Result<(), reqwest::Error> {

    let block = Block::new(blockId);

    let post_body = object! {
        jsonrpc: String::from("2.0"),
        method: String::from("getBlockByNumber"),
        id: 1,
        params: [1, block.hex_height, true]
    };

    let client = reqwest::Client::new();

    match client
        .post("http://127.0.0.1:8545")
        .body(post_body.dump())
        .send()
        .await
    {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                println!("RESPONSE: {} bytes received {:?} ", text.len(), text);
                match File::create(block.file_name).await {
                    Ok(mut file) => {
                        file.write_all(text.as_bytes()).await;
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

pub struct Block {
    pub hex_height: String,
    pub file_name: String,
}

impl Block {
    fn new(height:u128) -> Self {
        Block {
            hex_height: hex_block_height(height),
            file_name: format!("{}.json", hex_block_height(height)),
        }
    }
}

fn hex_block_height( height:u128) -> String {
    format!("{:#08x}", height)
}
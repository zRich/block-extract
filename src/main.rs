use json::{self, object};
use reqwest;
use serde::de::value::Error;
use std::collections::HashMap;
use std::fmt::{self, format};
// use std::fs::{OpenOptions, File, write};
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    fetch_block(01).await
}

async fn fetch_block(blockId: u128) -> Result<(), reqwest::Error> {
    let block = Block::new(blockId);

    let file_name = block.file_name();

    let post_body = object! {
        jsonrpc: String::from("2.0"),
        method: String::from("getBlockByNumber"),
        id: 1,
        params: [1, block.hex_height(), true]
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
                match OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open( block.file_name())
                    .await
                {
                    Ok(mut file) => {
                        file.write_all(text.as_bytes()).await;
                        ()
                    }
                    Err(err) => {
                        panic!("Unable to create block file {}: {}", block.file_name(), err,)
                    }
                }
            }
            Err(err) => panic!("Fetching block data failed {} : {}", block.hex_height(), err,),
        },
        Err(err) => panic!("Unable to connect server {}: {}", block.hex_height(), err,),
    }
    Ok(())
}

pub struct Block {
    hex_height: String,
    file_name: String,
}

impl Block {
    fn new(height: u128) -> Self {
        Block {
            hex_height: hex_block_height(height),
            file_name: format!("{}.json", hex_block_height(height)),
        }
    }

    pub fn hex_height(&self) -> &str {
        &self.hex_height
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }
}

fn hex_block_height(height: u128) -> String {
    format!("{:#08x}", height)
}

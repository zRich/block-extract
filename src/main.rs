use clap::{self, crate_version, value_t, App, Arg};
use json::{self, object};
use reqwest;
use serde::{Deserialize, Serialize};
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut cli_args = parse_args();

    let block_number = fetch_block_number().await?;

    println!("last block = {:?} ....", block_number);

    // if cli_args.end == 0 {
    //     let block_number = fetch_block_number().await?;
    //     cli_args.end = block_number.result.parse();
    // }
    // println!("fetch block from {} to {} ....", cli_args.start, cli_args.end);

    // for i in cli_args.start..cli_args.end {
    //     fetch_block(i).await?
    // }
    Ok(())
}

async fn fetch_block(blockId: u64) -> Result<(), reqwest::Error> {
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
                    .open(block.file_name())
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
            Err(err) => panic!(
                "Fetching block data failed {} : {}",
                block.hex_height(),
                err,
            ),
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
    fn new(height: u64) -> Self {
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

fn hex_block_height(height: u64) -> String {
    format!("{:#08x}", height)
}

#[derive(Serialize, Deserialize, Debug)]
struct BlockNumber {
    id: u64,
    version: String,
    result: String,
}

async fn fetch_block_number() -> Result<(BlockNumber), reqwest::Error> {
    let post_body = object! {
        jsonrpc: String::from("2.0"),
        method: String::from("getBlockNumber"),
        id: 1,
        params: [1]
    };

    let mut blocknumber = BlockNumber {
        id: 1,
        version: "2.0".to_string(),
        result: "1".to_string(),
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
                let bn: BlockNumber = serde_json::from_str(text.as_str()).unwrap();
                blocknumber.result = bn.result;
            }
            Err(err) => panic!("Fetching block data failed : {}", err,),
        },
        Err(err) => panic!("Unable to connect server : {}", err,),
    }
    Ok(blocknumber)
}

#[cfg(test)]
mod test {
    extern crate json;
    use core::panic;

    use json::object;
    use serde_json;
    #[test]
    fn test_dump() {
        let post_body = object! {
            jsonrpc: String::from("2.0"),
            method: String::from("getBlockByNumber"),
            id: 1,
            params: [1, 2, true]
        };

        println!("{}", post_body.dump());

        let post_body = object! {
            jsonrpc: String::from("2.0"),
            method: String::from("getBlockByNumber"),
            id: 1,
            params: [1, 2, true]
        };
    }

    use super::fetch_block_number;
    #[test]
    fn test_fetch_block_number() {
        let blocknumber = match fetch_block_number().await {
            Ok(bn) => bn,
            Err(_) => panic!("error"),
        };

        println!("{:?}", blocknumber);
    }
}
#[derive(Debug)]
struct CliArgs {
    start: u64,
    end: u64,
}

fn parse_args() -> CliArgs {
    let matches = clap::App::new("FISCO BCOS Block data extract.")
        .version(crate_version!())
        .author("Zhenhua ZHAO zhao.zhenhua@gmail.com")
        .about("FISCO BCOS Block data extract.")
        .arg(
            Arg::with_name("start")
                .short("-s")
                .long("start")
                .takes_value(true)
                .help("start block number"),
        )
        .arg(
            Arg::with_name("end")
                .short("e")
                .long("end")
                .takes_value(true)
                .help("to block number"),
        )
        .get_matches();

    CliArgs {
        start: value_t!(matches, "start", u64).unwrap_or(1),
        end: value_t!(matches, "end", u64).unwrap_or(0),
    }
}

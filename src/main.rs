mod tornado;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::{env, fs::File};
use std::io::{Read, Write};
use alloy_primitives::{hex, Bytes};

//Because ETH JSON-RPC API is fixed this should be reusable code
#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: String,
    result: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcRequest{
    pub jsonrpc: String,
    pub method: String,
    pub id: String,
    pub params: Vec<MultipleTypes>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum MultipleTypes {
    Str(String),
    Bool(bool),
    Int(i64),
}

struct InfuraAPI;

impl InfuraAPI {
    fn get_path() -> String {
        let token = match env::var_os("TOKEN") {
            Some(v) => v.into_string().unwrap(),
            None => panic!("$TOKENis not set")
        };
        return "https://mainnet.infura.io/v3/".to_owned() + &token;
    }
}

fn create_line(input_tornado: &str, input_recipient: &str, input_relayer: &str, input_fee: &str, block_number: u64, txn_hash: &str) -> String {
    return format!("{input_tornado},{input_recipient},{input_relayer},{input_fee},{block_number},{txn_hash}\n");
}

//json_value is untyped, we just expect that the caller knows what they are doing
fn parse_transaction(json_value: &Value, block_number: u64) -> Option<String> {
    //We don't unwrap here because I haven't checked if every transaction isn't malformed
    let maybe_hash = json_value["hash"].as_str();
    let maybe_input= json_value["input"].as_str();
    if let Some(input_data) = maybe_input {
        if let Some(hash) = maybe_hash {
            let hex = hex::decode(input_data).unwrap();
            if let Ok(tc) = tornado::decode_tc(&hex) {
                let tornado = tc._tornado.to_string();
                let recipient = tc._recipient.to_string();
                let relayer = tc._relayer.to_string();
                let fee = tc._fee.to_string();
                return Some(create_line(&tornado, &recipient, &relayer, &fee, block_number, hash));
            }
        }
    }
    None
}

fn get_blocks_cache(file: &mut File) -> HashSet<String> {
    let mut res = HashSet::new();

    let mut existing_lines = String::new();
    if let Ok(lines) = file.read_to_string(&mut existing_lines) {
        if lines > 0 {
            for row in existing_lines.split("\n") {
                res.insert(row.to_string());
            }
        }
    }
    res
}

#[tokio::main]
async fn main() {
    let final_block: u64 = 18295211;

    let args: Vec<String> = std::env::args().collect();
    let start: u64 = args[1].parse().unwrap();

    let mut file = OpenOptions::new()
        .append(true)
        .open("data.csv")
        .unwrap();

    let mut block_cache_file = OpenOptions::new()
        .read(true)
        .append(true)
        .open("seen-blocks")
        .unwrap();

    let seen_blocks = get_blocks_cache(&mut block_cache_file);

    for i in (final_block..start).rev() {

        let mut block_hex = format!("{:x}", i);
        block_hex.insert_str(0, "0x");

        if seen_blocks.contains(&i.to_string()) {
            continue;
        } else {
            let _ = block_cache_file.write(format!("{i}\n").as_bytes());
        }

        let req =  JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: "eth_getBlockByNumber".to_string(),
            params: vec![MultipleTypes::Str(block_hex.clone()), MultipleTypes::Bool(true)],
        };
        let path = InfuraAPI::get_path();

        //Possible perf implications of doing this, I don't know what this does
        let client = reqwest::Client::new();
        let res = client.post(path).json(&req).send().await.unwrap();
        let res_text: Value = serde_json::from_str(&res.text().await.unwrap()).unwrap();

        let transactions = res_text["result"]["transactions"].as_array().unwrap();
        for transaction in transactions.into_iter() {
            if let Some(valid_transaction) = parse_transaction(transaction, i) {
                let _ = file.write(valid_transaction.as_bytes());
            }
        }
    }
}
#![deny(unused_crate_dependencies)]

// для записи файла
use std::fs::File;
// use std::fs;

use std::io::prelude::*;

//===========================================================
// моё блять собственное
mod mokele_mbembe;
use jsonrpsee::ws_client::WsClient;
use mokele_mbembe::{moke, moke_send_money};

// моя новая либа для важных вещей
// mod mokele_lib;
// use mokele_lib::{moke_parse, moke_money};

//=== глобальные переменные для всех обращений к блокчейну
// use lazy_static::lazy_static;

//===========================================================
// Буду писать комменты

use anyhow::Result;

use std::{thread, time::Duration}; // LLeo это чтобы делать thread::sleep(Duration::from_millis(4000));

use clap::Parser; // какая-то библиотека разбора строки arg
use frame_metadata::RuntimeMetadata;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::{rpc_params, ws_client::WsClientBuilder};

// use jsonrpsee::ws_client::WsClient::Client;
// async_client::Client;

use parity_scale_codec::Decode;
use regex::Regex;
use serde_json::value::Value;
mod error;

use substrate_parser as _;




pub struct Context<'a> {
    nonce: u128,
    genesis_hash: &'a str,  // выясняется раз и навсегда
    era: u128,
    spec_version: u32, //  выясняется раз и навсегда
    transaction_version: u32, //  выясняется раз и навсегда
    block_hash: &'a str,
    address: &'a str, // url блокчейн-сервера (берется из командной строки или по дефолту)
    client: Client,
    metadata_flag: bool,
}

/// QDAO ExoSys deamon
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    // wss connection is indefinitely stuck, because the node does not respond anything when WSS is not configured properly on it.
    #[clap(short, long, default_value_t = String::from("ws://127.0.0.1:9944"))]
    pub url: String, // нашли в arg урл типа "ws://127.0.0.1:9944"
}

pub fn unhex(hex_input: &str, what: error::NotHex) -> Result<Vec<u8>, error::Error> {
    let hex_input_trimmed = {
        if let Some(hex_input_stripped) = hex_input.strip_prefix("0x") { hex_input_stripped }
	else { hex_input }
    };
    hex::decode(hex_input_trimmed).map_err(|_| error::Error::NotHex(what))
}

/// Supply address with port if needed.
///
/// Transform address as it is displayed to user in <https://polkadot.js.org/>
/// to address with port added if necessary that could be fed to `jsonrpsee`
///
/// The port is set here to default 443 if there is no port specified in
/// address itself, since default port in `jsonrpsee` is unavailable for now.
///
/// See for details <https://github.com/paritytech/jsonrpsee/issues/554`>
///
/// Some addresses have port specified, and should be left as is.
fn address_with_port(str_address: &str) -> String {
    let PORT: Regex = Regex::new(r"^(?P<body>wss://[^/]*?)(?P<port>:[0-9]+)?(?P<tail>/.*)?$").expect("known value");
    match PORT.captures(str_address) {
        Some(caps) => {
            if caps.name("port").is_some() {
                str_address.to_string()
            } else {
                match caps.name("tail") {
                    Some(tail) => format!("{}:443{}", &caps["body"], tail.as_str()),
                    None => format!("{}:443", &caps["body"]),
                }
            }
        }
        None => str_address.to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // получаем адрес сервера из служебной строки или по дефолту
    let args = Args::parse();
    let address = address_with_port(&args.url);
// устанавливаем соединение с сервером
let contextclient = WsClientBuilder::default().build(&address).await?;
let mut last_block = String::new(); // последний изученный блок

    // определяем глобальные переменные в стиле "По диким степям Забайкалья"
    let mut context = Context {
        nonce: 0,
        genesis_hash: "",  // выясняется раз и навсегда
        era: 0,
        spec_version: 0,  //  выясняется раз и навсегда
        transaction_version: 0, //  выясняется раз и навсегда
        block_hash: "",
        address: &address,  // url блокчейн-сервера (берется из командной строки или по дефолту)
        client: &contextclient,
        metadata_flag: false,
    };
    

    

    loop { // основной цикл
        let block_hash_data: Value = context.client.request("chain_getBlockHash",rpc_params![]).await?;
        let block_hash = if let Value::String(a) = block_hash_data { a } else { println!("Unexpected block hash format.");  continue; };
	// Если блок тот же, что прежний, ничо не делать
        if last_block == block_hash {
	    let my_sleep = 1000;
	    // println!("Sleep for {} ms",my_sleep);
	    thread::sleep(Duration::from_millis(my_sleep));
	    continue;
	} else {
	    last_block = block_hash.clone();
	}
    println!("=====================================================================\nНовый блок: {:#?}",&last_block);
	// 2 запрос к блокчейну: скачать Метадату

    if context.metadata_flag == false {
        let metadata: Value = context.client.request("state_getMetadata", rpc_params![&block_hash]).await?;
	// Save file
        let llog = "/tmp/lleo_metadata.txt";
        let mut file = File::create(llog)?;
        file.write_all(  &format!("{}",&metadata).as_bytes() )?;
        // Save Scale
        let meta_scale = hex::decode( &metadata . as_str() . expect("metadata is serialized as a hex string") [2..] ).unwrap() ;

	if &meta_scale[0..4] != b"meta" {
	    println!("============> не мета! [{:?}] != [{:?}]", &meta_scale[0..4], &b"meta" );
	} else {
	    let llog2 = "/tmp/lleo_metadata.scale";
	    println!("============> {} len={}", &llog2, &meta_scale.len());
	    let mut file2 = File::create(llog2)?;
	    file2.write_all( &meta_scale )?;
	}
        let metadata_v14 = if let Value::String(hex_meta) = metadata {
            let meta = unhex(&hex_meta, error::NotHex::Value).unwrap();
            if !meta.starts_with(&[109, 101, 116, 97]) { // 0x6d657461 - что это блять за код старта?!
                return Err(Box::from("Wrong start"));
            }
            match RuntimeMetadata::decode(&mut &meta[4..]) {
                Ok(RuntimeMetadata::V14(out)) => out,
                Ok(_) => continue,
                Err(_) => continue,
            }
        } else {
            continue;
        };
        let llogm = "/tmp/lleo_metadata14.txt";
        let mut file = File::create(llogm)?;
        file.write_all( &format!("{:#?}",&metadata_v14).as_bytes() )?;
        context.metadata_flag = true;
    }

 let bbb: Value = context.client.request("chain_getBlock",rpc_params![&block_hash]).await?;

 

// ТУТ БУДЕМ ПИСАТЬ раз в секунду
// Если блок не изменился, вернуться в цикл и не продолжать
 let parent = if let Value::String(a) = &bbb["block"]["header"]["parentHash"] { a } else { continue; };
// ТУТ БУДЕМ ПИСАТЬ раз в ТРАНЗАКЦИЮ (не в секунду)
        














            // Получить nonce
            context.nonce = context.client.request("system_accountNextIndex", rpc_params![ &format!( "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" ) ]).await?;

            // Получить genesis_hash
            if context.genesis_hash == "" {
                let s = context.client.request("chain_getBlockHash", rpc_params![ "0" ]).await?;
                // context.genesis_hash = s.clone();
                // Получить specVersion и transactionVersion
                let suka: Value = context.client.request("chain_getRuntimeVersion", rpc_params![ ]).await?; // .expect("Unknown genesis_hash");
                context.spec_version = suka["specVersion"].as_u64().expect("### ERROR ###") as u32;
                context.transaction_version = suka["transactionVersion"].as_u64().expect("### ERROR ###") as u32;
            }

            // Прикинуть era
            context.era = 0;

            // Ну и блок
            // all_you.block_hash = last_block.clone(); //  его не надо спрашивать
            context.block_hash = context.client.request("chain_getBlockHash", rpc_params![]).await?;

            let s = moke_send_money(&context, "Alice", "Bob",  999999999999999999); // .unwrap();
            // println!("==> Вся посылка целиком st2=:\n[{s}]");

            // Даже запишем ее от широты душевной на диск
            let llogex = "/tmp/lleo_extrinsic.txt";
            let mut file = File::create(llogex)?;
            file.write_all( &format!("{}",&s).as_bytes() )?;
            // Отправим
            let res: Value = context.client.request("author_submitAndWatchExtrinsic", rpc_params![ &format!("0x{}",&s ) ]).await?;
            println!(" * * * * ===> И вот результат: [{}]",&res);
        



















    let ext1 = if let Value::String(a) = &bbb["block"]["extrinsics"][1] { a } else { continue; };

    println!( "\n\t===> Блок:\nblock:  [{}]\nparent: [{}]\n",&last_block,&parent);

    let mut s1 = ext1.to_string();
    s1.remove(0);  // remove first
    s1.remove(0);  // remove first

    moke(&s1).unwrap();



        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

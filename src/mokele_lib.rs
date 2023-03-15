// // use crate::mokele_mbembe;

// // mod mokele_mbembe;
// // use mokele_mbembe::{moke, moke_send_money};

// use crate::AllYouNeedIsLove;

// // use crate::moke_send_money;

// // use jsonrpsee::rpc_params;
// // use crate::WsClientBuilder;


// // всё старое говно

// use anyhow::Result;
// // use std::{thread, time::Duration}; // LLeo это чтобы делать thread::sleep(Duration::from_millis(4000));
// // use clap::Parser; // какая-то библиотека разбора строки arg
// // use frame_metadata::RuntimeMetadata;
// // use jsonrpsee::core::client::ClientT;
// use jsonrpsee::{rpc_params, ws_client::WsClientBuilder};
// // use lazy_static::lazy_static;
// // use parity_scale_codec::Decode;
// // use regex::Regex;
// // use serde_json::value::Value;
// // mod error;
// use substrate_parser as _;















// // Процедура парсинга
// pub async fn moke_parse(s: &str) -> Result<String, String> {
//     Result::Err(format!("moke_parse отработало {}",&s))
// }

// // Процедура отправки денег
// pub async fn moke_money(s: &str) -> Result<String, String> {

//     let mut all_you = AllYouNeedIsLove {
//         nonce: 0,
//         genesis_hash: "".to_string(),
//         era: 0,
//         spec_version: 0,
//         transaction_version: 0,
//         block_hash: "".to_string(),
//         address: "".to_string(),
//     };
    
//     // создать соединение
//     let client = WsClientBuilder::default().build(all_you.address).await?;
    
//             // // Получить nonce
//             // all_you.nonce = client.request("system_accountNextIndex", rpc_params![ &format!( "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" ) ]).await?;

//             // // Получить genesis_hash
//             // if all_you.genesis_hash == "" {
//             //     all_you.genesis_hash = client.request("chain_getBlockHash", rpc_params![ "0" ]).await?;
//             //     // Получить specVersion и transactionVersion
//             //     let suka: Value = client.request("chain_getRuntimeVersion", rpc_params![ ]).await?; // .expect("Unknown genesis_hash");
//             //     all_you.spec_version = suka["specVersion"].as_u64().expect("### ERROR ###") as u32;
//             //     all_you.transaction_version = suka["transactionVersion"].as_u64().expect("### ERROR ###") as u32;
//             // }

//             // // Прикинуть era
//             // all_you.era = 0;

//             // // Ну и блок
//             // // all_you.block_hash = last_block.clone(); //  его не надо спрашивать
//             // all_you.block_hash = client.request("chain_getBlockHash", rpc_params![]).await?;

//             // // let s = moke_send_money("Alice", "Bob",  999999999999999999  , &all_you); // .unwrap();
//             // // println!("==> Вся посылка целиком st2=:\n[{s}]");

//             // // Даже запишем ее от широты душевной на диск
//             // // let llogex = "/tmp/lleo_extrinsic.txt";
//             // // let mut file = File::create(llogex)?;
//             // // file.write_all( &format!("{}",&s).as_bytes() )?;

//             // // Отправим
//             // let res: Value = client.request("author_submitAndWatchExtrinsic", rpc_params![ &format!("0x{}",&s ) ]).await?;
//             // println!(" * * * * ===> И вот результат: [{}]",&res);
        

















//     Result::Err(format!("moke_money отработало {}",&s))
// }

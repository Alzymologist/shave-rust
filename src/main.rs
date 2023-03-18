#[warn(non_camel_case_types)]
// use regex::Regex;
// use serde_json::json;
use reqwest::Error;


// моё блять собственное
mod mokele_mbember;
use mokele_mbember::{moke_send_money}; // {moke, moke_send_money, get_chain, };

// async fn get_chain(cx: &mut Context) -> Option<String> {
//     cx.block_hash = "===".to_string();
//     Some("OK".to_string())  
// }

#[derive(Debug)]
pub struct Context {
    cl: reqwest::Client,
    nonce: u128,
    genesis_hash: String,  // выясняется раз и навсегда
    //era: u128,
    spec_version: u32, //  выясняется раз и навсегда
    transaction_version: u32, //  выясняется раз и навсегда
    block_hash: String,
    request_url: String, // = "http://localhost:9933";
    ws_url: String, // = "ws://localhost:9944";
}



#[tokio::main]
async fn main() -> Result<(), Error> {

    // по диким степям Забайкалья бродяга тащился от отсутствия GLOBAL
    let mut cx = Context {
            genesis_hash: "".to_string(),
            spec_version: 0,
            transaction_version: 0,
            block_hash: "".to_string(),
            nonce: 0,
            request_url: "http://localhost:9933".to_string(),
            ws_url: "ws://localhost:9944".to_string(),
            cl: reqwest::Client::new(),
    };

    // 2023-03-18 23:30:44 Running JSON-RPC HTTP server: addr=0.0.0.0:9933, allowed origins=["*"]    
    // 2023-03-18 23:30:44 Running JSON-RPC WS server: addr=0.0.0.0:9944, allowed origins=["*"]  

    match moke_send_money(&mut cx, "Alice", "Bob",  1).await {
        None => println!("########## перевести бабла не удалося"),
        Some(l) => {
            // println!("--------------------\ncontext:\n{:#?}\n---------------------\n[!!!] {}", &cx, &l);
            // println!("===> genesis_hash=[{}]", &cx.genesis_hash ); 
            // println!("===> spec_version=[{}]", &cx.spec_version );
            // println!("===> transaction_version=[{}]", &cx.transaction_version );
            // println!("===> block_hash=[{}]", &cx.block_hash );
        }
    }
    
    // match get_chain(&mut cx).await {
    //     // None => println!("Скачать хуйню с блокчейна не удалось."),
    //     // Some(l) => println!("Хуйня с блокчейна скачана успешно: {}", &l),
    //     None => println!("Скачать хуйню с блокчейна не удалось"),
    //     Some(l) => {
    //         println!("Хуйня с блокчейна скачана успешно: {}", &l);
    //         println!("===> genesis_hash=[{}]", &cx.genesis_hash ); 
    //         println!("===> spec_version=[{}]", &cx.spec_version );
    //         println!("===> transaction_version=[{}]", &cx.transaction_version );
    //         println!("===> block_hash=[{}]", &cx.block_hash );
    //     }
    // }

    Ok(())
}
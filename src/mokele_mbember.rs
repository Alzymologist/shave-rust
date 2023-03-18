use regex::Regex;

use crate::Context; // мешок Забайкалья

// для построения транзакций
use parity_scale_codec::{Compact, Encode}; // CompactLen, Encode, Decode,

#[derive(Debug)]
pub struct Tran { ara: Vec<u8>, }

impl Tran {

    fn sign(&mut self, a: Signature) {
        self.ara.extend_from_slice(&a.0);
    }

    fn bytes(&mut self, a: &[u8]) {
        self.ara.extend_from_slice(&a);
        // for x in a { self.ara.push(*x); }
    }

    fn add_len(&mut self) {
        let ara2 = self.ara.clone();
        self.ara=[].to_vec();
        self.compact(ara2.len().try_into().unwrap());
        self.bytes(&ara2);
    }

    fn hex(&mut self, hex: &str) {
        let a: &[u8] = &hex::decode(hex).unwrap();
        for x in a { self.ara.push(*x); }
    }

    fn compact(&mut self, x: u128) {
        let a = Compact(x).encode();
        for x in a { self.ara.push(x); }
    }

    fn add_user(&mut self, name: &str) {
        self.hexstring( &user_name(name) );
    }

    fn hexstring(&mut self, name: &String) {
        let mut data = name.clone();
        if &data[0..2] == "0x" { data = (data[2..]).to_string(); }
        let a: &[u8] = &hex::decode(data).unwrap();
        for x in a { self.ara.push(*x); }
    }
    
    fn clone(&mut self) -> Tran {
        let mut x = Tran { ara: [].to_vec() };
        x.ara.extend_from_slice(&self.ara);
        x
    }
            
    fn u32(&mut self, x: &u32) {
        self.ara.push( ((x)       & 0xFF) as u8 );
        self.ara.push( ((x >> 8)  & 0xFF) as u8 );
        self.ara.push( ((x >> 16) & 0xFF) as u8 );
        self.ara.push( ((x >> 24) & 0xFF) as u8 );
    }

}
    



// ================ подпись блокчейна =====================
use sp_core::{crypto::Pair, sr25519, sr25519::Signature};
pub fn singme(message: &[u8], account: &str) -> Option<Signature> {
    let pair = match sr25519::Pair::from_string(&account, None) {
            Ok(val) => val,
            Err(_) => {
                sr25519::Pair::from_string(&format!("//Alice"), None).ok()? // .unwrap()
            },
    };
    Some(pair.sign(&message[..]))
}
// ================ делаем json из строки ==================
use serde_json::Value;
pub fn json(method: &str, params: &str) -> Option<Value> {
    let s = format!("{{ \"jsonrpc\":\"2.0\", \"method\":\"{method}\", \"params\":[{params}], \"id\":1 }}");
    // println!("JSON: {}", &s);
    let json: serde_json::Value = serde_json::from_str(&s).ok()?;
    // println!("JSON: {}", &s);
    Some(json)
}

pub async fn post(cx: &Context, method: &str, params: &str) -> Option<String> {
    cx.cl.post(&cx.request_url).json(&(json(method,params)?)).send().await.ok()?.text().await.ok()
}

use ws::{connect, CloseCode};
pub fn wspost(cx: &Context, method: &str, params: &str) -> Option<String> {
    let s = format!("{{ \"jsonrpc\":\"2.0\", \"method\":\"{method}\", \"params\":[{params}], \"id\":1 }}");
    
    connect(cx.ws_url.clone(), |out| {
        out.send(format!("{{ \"jsonrpc\":\"2.0\", \"method\":\"{method}\", \"params\":[{params}], \"id\":1 }}")).unwrap();

        move |msg| {
            println!("Got message: {}", &msg);
            // let x = msg;
            // return Some(x.into());
            out.close(CloseCode::Normal)
        }
    });

    Some("OK".to_string())
    // let ws_url: String = Regex::new(r#"^.+://"#).unwrap().replace(&cx.request_url, "ws://").into();
    // panic!("---------------");
}


// ================ добыть нужные параметры ================

pub async fn get_chain(cx: &mut Context) -> Option<String> {

    // genesis_hash   
    let s = post(&cx,"chain_getBlockHash", "0").await?;
    let m = Regex::new(r#","result"\s*:\s*"0x([0-9a-fA-F]+)","#).ok()?.captures(&s)?;
    cx.genesis_hash = (&m[1]).to_string();

    // specVersion и transactionVersion
    let s = post(&cx,"chain_getRuntimeVersion", "").await?;
    let m = Regex::new(r#","specVersion"\s*:\s*(\d+),"#).ok()?.captures(&s)?;
    cx.spec_version = m[1].parse().ok()?;
    let m = Regex::new(r#","transactionVersion"\s*:\s*(\d+),"#).ok()?.captures(&s)?;
    cx.transaction_version = m[1].parse().ok()?;

    // chain_getBlockHash
    let s = post(&cx,"chain_getBlockHash", "").await?;
    let m = Regex::new(r#","result"\s*:\s*"0x([0-9a-fA-F]+)","#).ok()?.captures(&s)?;
    cx.block_hash = (&m[1]).to_string();

    Some("OK".to_string())
}

// ================ послать денег ==========================
pub async fn moke_send_money(cx: &mut Context, from: &str, to: &str, money: u128 ) -> Option<String> {

// let money = 1;
// wspost(&cx);

    // Не сходить ли нам выкачать нужные параметры?
    if cx.spec_version == 0 { get_chain(cx).await?; }

    println!("===> genesis_hash=[{}]", &cx.genesis_hash ); 
    println!("===> spec_version=[{}]", &cx.spec_version );
    println!("===> transaction_version=[{}]", &cx.transaction_version );
    println!("===> block_hash=[{}]", &cx.block_hash );

    // Получить nonce
    let s = post(&cx,"system_accountNextIndex", "\"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY\"").await?;
    let m = Regex::new(r#","result"\s*:\s*(\d+),"#).ok()?.captures(&s)?;
    cx.nonce = m[1].parse().ok()?;  

   // Сама операция, пример: 0500 00 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 04
   let mut t = Tran { ara: [].to_vec() };
   t.hex("0500"); // код транзакции
   t.hex("00"); // ноль
   t.add_user(to); // кому деньги
   t.compact(money); // сколько денег

   // Теперь делаем расширенную версию операции для подписи:
   let mut full = t.clone(); // само тело транзакции
   full.hex("00"); // Era: можно 00
   // full.hex("0501"); // Era: можно 00
   full.compact(cx.nonce); // Nonce:
   full.hex("00"); // Tip: чаевые, можно 00
   full.u32(&cx.spec_version); // SpecVersion: просто u32, без compact
   full.u32(&cx.transaction_version); // TransactionVersion: просто u32, без compact
   full.hexstring(&cx.genesis_hash.to_string()); // GenesisHash
   // full.hexstring(&cx.block_hash); // BlockHash
   full.hexstring(&cx.genesis_hash.to_string()); // GenesisHash
   
   // теперь бы ее суку как-то подписать Алисой:
   let sign = singme( &full.ara, from )?;

   // Так, теперь формируем целиком всю посылку:
   let mut tr = Tran { ara: [].to_vec() };
   tr.hex("8400");  // Начало 84 (compact от 33?) 00
   tr.add_user("Alice"); // From: Alice
   tr.hex("01"); // код 01
   tr.sign(sign); // здесь вставляем подпись
   tr.hex("00"); // Эра: можно 00, НЕ compact! TR.compact(era);
   // tr.hex("0501");
   tr.compact(cx.nonce); // Nonce
   tr.hex("00"); // Код 00
   tr.bytes(&t.ara); // сама операция (короткая версия)
   tr.add_len(); // и в начало добавим compact-длину всей этой мандулы
   // END

   let str = format!("\"0x{}\"", &hex::encode(&tr.ara) );

    wspost(&cx,"author_submitAndWatchExtrinsic", &str);
    // let s = post(&cx,"author_submitAndWatchExtrinsic", &str).await?;


    // panic!("всё, пиздец, ничего у меня не работает");
    Some(str)
}


// =============== user name ======================
use std::collections::HashMap;

pub fn user_name(name: &str) -> String {
    let oligarch = HashMap::from([
        ("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","ALICE"),
        ("be5ddb1579b72e84524fc29e78609e3caf42e85aa118ebfe0b0ad404b5bdd25f","ALICE_STASH"),
        ("8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","BOB"),
        ("fe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e","BOB_STASH"),
        ("90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22","CHARLIE"),
        ("306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20","DAVE"),
        ("e659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e","EVE"),
        ("1cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c","FERDIE"),
    ]);

    if 64 == name.len() { // это хэш
        let s = name.to_lowercase();
        for (a, b) in &oligarch {
            if a == &s { return format!("«{b}» {s}"); }
        }
        return String::from("<_unknown_> {s}");
    } else { // это name Alice
        let s = name.to_uppercase();
        for (a, b) in &oligarch {
            if b == &s { return format!("{a}"); }
        }
        return String::from("«{name}» _unknown_");
    }
}
// ===================================================================================
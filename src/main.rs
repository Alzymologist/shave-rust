#![deny(unused_crate_dependencies)]

// для записи файла
use std::fs::File;
use std::fs;

use std::io::prelude::*;

//===========================================================
// моё блять собственное

// use crate::moke::prmas;

//===========================================================
// привинчиваем подпись блокчейна
use sp_core::{crypto::Pair, sr25519};

use sp_core::ByteArray;

use sp_core::sr25519::Signature;
use sp_core::sr25519::Public;


// use sp_core::Pair;

//===========================================================

mod mokele_mbembe;
use mokele_mbembe::moke;

//===========================================================

// Буду писать комменты

use anyhow::Result;

use std::{thread, time::Duration}; // LLeo это чтобы делать thread::sleep(Duration::from_millis(4000));

use clap::Parser; // какая-то библиотека разбора строки arg
use frame_metadata::RuntimeMetadata;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::{rpc_params, ws_client::WsClientBuilder};
use lazy_static::lazy_static;
use parity_scale_codec::Decode;
use regex::Regex;
use serde_json::value::Value;
use sp_core::{twox_128}; // , H256};
mod error;

use substrate_parser::{
    cards::{ParsedData, Sequence},
    decode_all_as_type,
};


/// QDAO ExoSys deamon
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    // wss connection is indefinitely stuck, because the node does not respond anything when WSS is not configured properly on it.
    #[clap(short, long, default_value_t = String::from("ws://127.0.0.1:9944"))]
    pub url: String, // нашли в arg урл типа "ws://127.0.0.1:9944"
}

lazy_static! {
    /// Regex to add port to addresses that have no port specified.
    ///
    /// See tests for behavior examples.
    static ref PORT: Regex = Regex::new(r"^(?P<body>wss://[^/]*?)(?P<port>:[0-9]+)?(?P<tail>/.*)?$").expect("known value");
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
/// client.
///
/// The port is set here to default 443 if there is no port specified in
/// address itself, since default port in `jsonrpsee` is unavailable for now.
///
/// See for details <https://github.com/paritytech/jsonrpsee/issues/554`>
///
/// Some addresses have port specified, and should be left as is.
fn address_with_port(str_address: &str) -> String {
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

        let mut metadata_flag: bool = false;


//    let mut file = File::create("/tmp/rust-blokchain-Metadata")?;
//    file.write_all(b"Hello, world!")?;


    // получаем адрес сервера из служебной строки или по дефолту
    let args = Args::parse();
    let address = address_with_port(&args.url);
    // устанавливаем соединение с сервером
    let client = WsClientBuilder::default().build(&address).await?;

    let mut last_block = String::new(); // последний изученный блок

    loop { // основной цикл
        let block_hash_data: Value = client.request("chain_getBlockHash",rpc_params![]).await?;
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
//	panic!("OK");
	// 2 запрос к блокчейну: скачать Метадату

    if metadata_flag == false {

        let metadata: Value = client.request("state_getMetadata", rpc_params![&block_hash]).await?;

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

    // pub fn unhex(hex_input: &str, what: error::NotHex) -> Result<Vec<u8>, error::Error> {
    //     let hex_input_trimmed = {
    //         if let Some(hex_input_stripped) = hex_input.strip_prefix("0x") { hex_input_stripped }
    //     else { hex_input }
    //     };
    //     hex::decode(hex_input_trimmed).map_err(|_| error::Error::NotHex(what))
    // }

    file.write_all(  &format!("{}",&metadata).as_bytes() )?;


/*



{ // Запись метадаты
    
//    let metadata2: String = "Мама мыла раму".to_string();
//    let metadata2 = serde_json::to_string(&metadata).unwrap();

//    let u = "serde_json::to_string(&metadata).unwrap()".as_string();
//    let metadata2 = serde_json::to_string(&u).unwrap();
    let metadata2 = serde_json::from_str("edwdwe").unwrap();
//    let metadata2: String = "serde_json::to_string(&metadata).unwrap()".as_string();

// from_str(&metadata).unwrap();

// to_string(&metadata).unwrap();
// : Value = metadata.to_string().as_bytes(); // .into();

//    // serialization
//    let s = serde_json::to_string(&u).unwrap();
//    // `s` represented as `null`
//    println!("{}", s);

//
//    // deserialization
//    let d: U = serde_json::from_str("null").unwrap();
//

// String = metadata.as_str();
//    let metadata2 = serde_json::to_string(&metadata).unwrap();

file.write_all(  &format!("{}",&metadata2).as_bytes() )?;
    println!("Ок, получили metadata, записано в {}",llog);
}
*/


/*
    let metadata: Value = serde_json::Value::String(
fs::read_to_string(llog).expect("Should have been able to read the file")
)
; // .into(); // .await?; // .into();

println!("Вот оно блять: ------------------------
{}------------------------",&metadata);


    let llog2 = "/tmp/rust-blokchain-Metadata2.json";
    let mut file = File::create(llog2)?;
    file.write_all(  &format!("{}",&metadata).as_bytes() )?;
    */


// panic!("sdsd");

/*

// --snip--
//    let metadata: Value = "aaaaaaaaaaaaaaaaaaaa".into();
let metadata: Value::String = // (
    //    let metadata: Value  = serde_json::Value::String(
        fs::read_to_string(llog).expect("Should have been able to read the file").into() // .await?; // .into();
//    )
;

println!("Вот оно блять: ------------------------
{:#?}------------------------",&metadata);

//String(
//    "0x6d6574610e4102...9c3d02",
//)
*/

//  panic!("=====================================");

        let metadata_v14 = if let Value::String(hex_meta) = metadata {
	    
//         println!("
// ****************************************************************************
// {:#?}
// ****************************************************************************
// ",&hex_meta);
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
//        println!("{:#?}",&metadata_v14);

//    let llog = "/tmp/rust-blokchain-Metadata.txt";
//        println!("Ок, получили metadata_v14, записано в {}",llog);
//    let mut file = File::create(llog)?;
// //    file.write_all(b"Hello, world!")?;
//    file.write_all( &format!("{:#?}",&metadata_v14).as_bytes() )?;


	// 3 запрос к блокчейну - state_getStorage
/*

    HEX - The storage key.
    HASH - (OPTIONAL) The block hash.

Request:
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "state_getStorage",
"params":
[
    "0xc2261276cc9d1f8598ea4b6a74b15c2f6482b9ade7bc6657aaca787ba1add3b458ad08561bd8f502d2ba488697d10b58aaa7c4097d4abb1c8861495348fd6970",
	null
    ]
}'
 http://localhost:9933

*/

/*
        let events = client
        .request(
                "state_getStorage",
                rpc_params![
                    &format!(
                        "0x{}{}",
                        hex::encode(twox_128(b"System")),
                        hex::encode(twox_128(b"Events"))
                    ),
                    &block_hash
                ],
            )
            .await?;
*/

    metadata_flag = true;
}

 let bbb: Value = client.request("chain_getBlock",rpc_params![&block_hash]).await?;

// ТУТ БУДЕМ ПИСАТЬ раз в секунду
// eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee



let account = "//Alice";
// let suri = SecretUri::from_str(account).expect("Parse SURI");

let pair = match sr25519::Pair::from_string(&account, None) {
        Ok(val) => val,
        Err(err) => {
    	    println!("==> Error Pair [{}]: {:#?}, тогда будем использовать [//Alice]",&account,&err);
	    sr25519::Pair::from_string(&format!("//Alice"), None).unwrap()
	},
};

println!("id: {}", hex::encode( pair.public().to_raw_vec() ));

// подписываем message
let message = b"Signed payload";
// let Signature(mut blytes) = pair.sign(&message[..]);
let mut blytes: Signature = pair.sign(&message[..]);
println!("sign: {}", hex::encode(&blytes) );

// проверяем подпись
let veri = sr25519::Pair::verify( &pair.sign(&message[..]) , &message[..], &pair.public() );
println!("--> проверка: {:#?}",&veri);


// let tranz = subxt::dynamic::tx(
//     "Balances",
//     "transfer",
//     vec![
//         Value::unnamed_variant("Id", [Value::from_bytes(&dest)]),
//         Value::u128(123_456_789_012_345),
//     ],
// );



// let h: String = bytes.encode_hex();
// println!("===> bytes=[{:#?}]",h );

// let signature = Signature(bytes);


// Pair.sign(&message)

// let public = Public::from_raw( hex!( "b4bfa1f7a5166695eb75299fd1c4c03ea212871c342f2c5dfea0902b2c246918" ) );
// let signature = Signature::from_raw(hex!( "5a9755f069939f45d96aaf125cf5ce7ba1db998686f87f2fb3cbdea922078741a73891ba265f70c31436e18a9acd14d189d73c12317ab6c313285cd938453202" ));
// let message = b"Verifying that I am the owner of 5G9hQLdsKQswNPgB499DeA5PkFBbgkLPJWkkS6FAM6xGQ8xD. Hash: 221455a3\n";
//                assert!(Pair::verify_deprecated(&signature, &message[..], &public));
//                assert!(!Pair::verify(&signature, &message[..], &public));
// println!("===> Public=[{:#?}]\n===> signature=[{:#?}]",public, signature );











//    let block_hash = if let Value::String(a) = block_hash_data { a } else { println!("Unexpected block hash format.");  continue; };


// println!("Тип значения bbb: {:#?}", &bbb);

// ""; // client.request("chain_getBlock",rpc_params![&block_hash]).await?;

//    let block_hash_data: Value = client.request("chain_getBlockHash",rpc_params![]).await?;
//    let block_hash = if let Value::String(a) = block_hash_data { a } else { println!("Unexpected block hash format.");  continue; };

//    let bbb: Value = client.request("chain_getBlockHash",rpc_params![]).await?;
//                       let bbx = if let Value::String(aa) = bbb { aa } else { println!("Unexpected block hash format."); continue; };

//  let parent: String = &bbb["block"]["header"]["parentHash"];

// let parent = if let Value::String(aa) = bbb { aaa } else { println!("Unexpected block hash format.");  continue; };

// let parent = &(&bbb["block"]["header"]["parentHash"]).to_string();
//  if !let ext0 = &(&bbb["block"]["extrinsics"][0]).to_string() { continue; }









// Pair.sign(&message)











 let parent = if let Value::String(a) = &bbb["block"]["header"]["parentHash"] { a } else { continue; };
// let ext0 = if let Value::String(a) = &bbb["block"]["extrinsics"][0] { a } else { continue; };
 let ext1 = if let Value::String(a) = &bbb["block"]["extrinsics"][1] { a } else { continue; };

// let bhd: Value = client.request("chain_getBlockHash",rpc_params![]).await?;
// let bhs = if let Value::String(au) = bhd { au } else { println!("Unexpected block hash format.");  continue; };


// if let String(a) = bbb { a } else { println!("Unexpected block hash format.");  continue; };

// let ext1 = &bbb["block"]["extrinsics"].0;


// const signedBlock = await api.rpc.chain.getBlock(blockHash);
println!( "\n\t===> Блок:
block:  [{}]
parent: [{}]
",&last_block,&parent
);

// &string[1..string.len() - 1];
let mut s1 = ext1.to_string();
s1.remove(0);  // remove first
s1.remove(0);  // remove first

moke(&s1).unwrap();



/*
=========== 0x280402000b 90b9ec 238601
=========== 0x4502
84
00
d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
01
106ad87abccb8ad8ee0ff2260f3b0803ccc099d280cef7abcffc4369dcaf4100
0cfb261bd5fe5366a286031ee001eab336c73b7b1fd9ff58b33567704b58568d
4502 20
00
calindex: 0500
00
8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48
0b00bc66fa1509


=========== 0x280402000b 3046ed 238601
=========== 0x
Длина (compact) 4502
84
00
Кто-Алиса: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
01
Подпись:
    bc964e975a95f0c1019c00e45a84bcf0da0bd5d4dbf420dcaf4e3eccb58a2558
    cacd17e43dc1f237542f6425df28c9b8dbe36ac98a96b9b34a2c5e822372028f
time: b502
nounce (compact): 24
00
calindex: 0500 (видимо, код перевода)
00
Кому-Бобу: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48
Сумма (compact): 0b 00 bc 66 fa 15 09 число: 9990000000000 длина байт: 7 массив: 0B 00 BC 66 FA 15 09 

00
8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480b00bc66fa1509


Экстринсик пришел:
=========== 0x280402000bf0f6f4238601
=========== 0xb102
84
00
d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
01
6e492d9ad750f84d6750c5af2b62fc3502ddd28398db391b62c7a37d2c646e3a
88e46f0dab376d374bf62a76e3f67c68ee23202e1022c66cf01a1b7cf1879d8b
f503 28
00
callindex: 0800
длина: 0101
ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
Twart/Endorse 01


8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48
*/











// twox_128!(b"System");
//println!( "\n\n#######################\n0x{}{}\n#####################\n",
//                        hex::encode(twox_128(b"System")),
//                        hex::encode(twox_128(b"Events"))
//);

// println!( "\n\n#######################\n0x{:#?}\n#####################\n", hex::encode(twox_128(b"System")) );
//        let s: Value = client.request("system_name", rpc_params![]).await?;
//        let vername = if let Value::String(a) = s { a } else { println!("Unknown name format.");  continue; };
//        let s: Value = client.request("system_version", rpc_params![]).await?;
//        let verver = if let Value::String(a) = s { a } else { println!("Unknown ver format.");  continue; };
//        println!("Ок, name: [{}] ver: [{}]",&vername,&verver);

//	println!("\n\n--==================================\n");
//	let pair: [u8; 1000] = array_bytes::hex2array_unchecked("0x4c6f7665204a616e6520466f7265766572");
//	println!("\n\n--==================================\n");
//	let pair = Pair::from_seed(&array_bytes::hex2array_unchecked(
//	let pair = sr25519::Pair::from_seed(&[15;32]);

//	let pair = sr25519::Pair::from_seed(
//	    &array_bytes::hex2array_unchecked("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"),
//	);

//        let pair2 = sr25519::Pair::from_string(&format!("Alice"), None).unwrap(); // d4...27d
//let account = "//Alice";
//let pair = match sr25519::Pair::from_string(&account, None) {
//        Ok(val) => val,
//        Err(err) => {
//    	    println!("==> Error Pair [{}]: {:#?}, тогда будем использовать [//Alice]",&account,&err);
//	    sr25519::Pair::from_string(&format!("//Alice"), None).unwrap()
//	},
//    };

// ALICE.Pair = keyring.addFromUri('//Alice',

//	let public = pair.public();
//	println!("--> public = {:#?}",public);
//
//	let mydata = "0x1234";
//	let message = array_bytes::hex2bytes_unchecked(&mydata);
//	println!("--> сообщение [{}] = {:#?}",&mydata,&message[..]);
//
//	let mut signature = pair.sign(&message[..]);
//	println!("--> signature = 0x{:#?}",&signature);
//

//        let muhaha: Value = client.request("system_name", rpc_params![]).await?;
//        let muhaha: Value = client.request(
//		"system_accountNextIndex", // "params":["'${ALICE}'"]}'
//                // "author_submitAndWatchExtrinsic",
//                rpc_params![
//                    &format!(
//		    "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
//                    // "0x400408002ccccccccccccccccccccccc00"
//                    )
//                ],
//            ).await?;
//        println!("===> muhaha: {:#?}",&muhaha);


//    fn sign(&self, message: &[u8]) -> Signature {
//	let context = signing_context(SIGNING_CTX);
//	self.0.sign(context.bytes(message)).into()
//    }


//
//array_bytes::hex2array_unchecked("0x4c6f7665204a616e6520466f7265766572"),
//    *b"Love Jane Forever"
//array_bytes::hex2array_unchecked(
//	    "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
//	);


//	println!("\n\n--==================================\n");
//	panic!("OK");

//    let pair2 = Pair::from_string(&format!("Alice"), None).unwrap();
//    let pair2 = Pair::from_string(&format!("Alice"), None).unwrap();

    // known address of DEV_PHRASE with 1.1
//    let known = array_bytes::hex2bytes_unchecked(
//        "d6c71059dbbe9ad2b0ed3f289738b800836eb425544ce694825285b958ca755e",
//    );
//    assert_eq!(pair.public().to_raw_vec(), known);

//panic!("\n\nEEEEEEEEEEEEEEEEEEEEE");


//        let s: Value = client.request("author_submitAndWatchExtrinsic", rpc_params![]).await?;

// let mythepid="eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
// let mytheuip="8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48";

//	println!("code: [0x0802{}04{}]",
//            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
//            "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
//	);




// {"id":1635,"jsonrpc":"2.0","method":"author_submitAndWatchExtrinsic","params":["0x400408002ccccccccccccccccccccccc00"]}
/*
        let muhaha = client.request(
                "author_submitAndWatchExtrinsic",
                rpc_params![
                    &format!(
		    "0x400408002ccccccccccccccccccccccc00"
                    )
                ],
            ).await?;

        println!("Ок, muhaha: {:#?}",&muhaha);
*/

// a903
// 8400d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01ca4090a6e55e2ba7c8141ddeb93749a51dfafbff32fc86419e2b9ec0b5ecd7680fd2e4affb4c7c0402ade011b7bcd14eb497fc9e7713198a8dbd9a987754a68835002400080200000000000000000000000000000000000000000000000000000000000000000c306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc201cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07ce659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e
// ca4090a6e55e2ba7c8141ddeb93749a51dfafbff32fc86419e2b9ec0b5ecd7680fd2e4affb4c7c0402ade011b7bcd14eb497fc9e7713198a8dbd9a987754a688


//{8400d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d015e34501a8cac302f601467b65899106497fd483197a1c14bbc543766b03f14761878d8f85f715f6ed477c1aa01876aa5d5be6137bd02013f95a4b739c3948b8e25021c0008002ccccccccccccccccccccccc00

//  "id": 1,
//  "jsonrpc": "2.0",
//  "method": "author_submitAndWatchExtrinsic",
//  "params": [
//    "0x01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000bae32a8130ab7966a82a8a025e24d23f0244e606f34375f66855b105d1d2e25eca2e21855ba44e90bd48833638220a4a9ddd1b6ffa08a2424df1a8ffbd8b0d8f00"
//  ]
//}



//        println!("Ок, muhaha: {:#?}",&muhaha);
//  спросить system_name
// curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_name", "params":[]}' http://loc
// # спросить system_version
// curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_version", "params":[]}' http://
// # спросить метадату
// curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "state_getMetadata", "params":[null]}' http://localhost:9933



//	println!(" -----------> Done. Print!");
//	println!(" [!] OKI 7 {:#?}",&events);

//panic!("OK");

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

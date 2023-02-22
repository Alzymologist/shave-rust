use parity_scale_codec::{Compact, Decode, Encode}; // CompactLen, Encode, 

use std::collections::HashMap;

// привинчиваем подпись блокчейна
use sp_core::{crypto::Pair, sr25519};
use sp_core::ByteArray;
use sp_core::sr25519::Signature;
use sp_core::sr25519::Public;


#[derive(Debug)]
pub struct Tran { ara: Vec<u8>, }
// pub struct ЕSignature(pub [u8; 64]);

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
        let a: &[u8] = &hex::decode(user_name(name) ).unwrap();
        for x in a { self.ara.push(*x); }
    }
}

















pub fn id(mut a: &[u8]) -> &[u8] {
    let i=32; print!("id:   "); prmas(&a[0..i]); a=&a[i..];
    a
}

fn doi(mut a: &[u8]) -> &[u8] {
    let o = Compact::<u64>::decode(&mut a).unwrap();
    let o = u64::from(o);
    let i = o as usize;
    print!("DOI {}: ",&o); pr(&a[0..i]);
    a=&a[i..];
    a
}

fn vote(mut a: &[u8]) -> &[u8] {
    print!("Голосование: ");
    match &a[0] {
	0x01 => { println!("01 Thwart"); },
	0x00 => { println!("00 Endorse"); },
	_ => { println!("unknown"); },
    }
    a=&a[1..];
    a
}

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


/*
for x in &array {  println!("{:?}”, x); }
это то же самое что
array.for_each(|x| {println!(":?”, x);}
*/


// Напечатать массив как символы
fn pr(m: &[u8]) {
    for x in m { print!("{}", *x as char ); } println!("");
}

// Напечатать массив как "07 AB CD DE"
fn pr_mas(m: &[u8]) {
    for x in m { print!("{:02X} ", &x); } println!("");
}

// Напечатать массив как "07abcdde"
pub fn prmas(m: &[u8]) {
    for x in m { print!("{:02x}", &x); } println!("");
}

// Сама процедура, получает строку вида 0x07abcdde...
pub fn moke(s: &str) -> Result<String, String> {

    println!("Массив: -------------\n0x{}-------------\n",s);


    let mut a: &[u8] = &hex::decode(&s).unwrap();
    println!("Массив длиной {}:",a.len());
    pr_mas(&a);

    // Смотрим длину (пока проверки не написал)
    let o = Compact::<u64>::decode(&mut a).unwrap();
//    println!("===========> Compact {:?}:",&o);
//    println!("Массив длиной {}:",a.len());
//    pr_mas(&a);

    let mut i=2;
    // проверим первые 2 байта, правильная ли посылка
    print!("Начало: "); pr_mas(&a[0..i]);
    if a[0] != 0x84 && a[1] != 0x00 { return Result::Err(String::from("Некорректо, нужно 8400")); }
    a = &a[i..];

    // Кто автор транзакции?
    i=32; println!("FROM: {}", user_name( hex::encode(&a[0..i]).as_str() ) ); a = &a[i..];

    // спецкод 01 - хер знает, что он значит
    i=1;  print!("Код 01: "); pr_mas(&a[0..i]);
    if a[0] != 0x01 { return Result::Err(format!("Некорректо, нужно 01")); }
    a = &a[i..];

    // подпись 64 байта (проверку пока не сделал, хотя уже знаю, как)
    i=64;
	print!("Sign: "); prmas(&a[0..32]);
	print!("      "); prmas(&a[32..i]);
    a=&a[i..];

    // далее идет Эра - либо 1 байт 0, либо 2 байта
    if a[0] == 0 {
        println!("Эра: 0"); i=1;
    } else {
        i=2; print!("Эра: "); prmas(&a[0..i]);
    }
    a=&a[i..];

    // i=20; print!("Далее должна быть эра: "); prmas(&a[0..i]);
    // let o = Compact::<u64>::decode(&mut a).unwrap();
    // let o = u64::from(o);
    // println!("Эра: {}",&o);

    // далее Nounce (формат Compact)
    let o = Compact::<u64>::decode(&mut a).unwrap();
    let o = u64::from(o);
    println!("Nonce: {}",&o);

    // спецкод 00 - хер знает, зачем
    i=1; print!("Код 00: "); pr_mas(&a[0..i]);
    if a[0] != 0x00 { return Result::Err(format!("Некорректно, нужно 00")); }
    a = &a[i..];

    // далее 2 байта код транзакции, я пока умею разбирать 5 видов
    i=2;
    let op: u16 = a[1] as u16 + ((a[0] as u16) << 8); a=&a[i..];
    print!("Operation: {:04X} => ",&op);
    match &op {
	    0x0800 => { println!("0800 peerReview reactToDoi(doi,opinion)");
		a=doi(&a);
		a=vote(&a);
	    },

	    0x0801 => { println!("0801 peerReview react(id,opinion)");
		a=id(&a);
		a=vote(&a);
	    },

	    0x0802 => { println!("0802 peerReview post(id,autors)");
		a=id(&a);
	        let o = Compact::<u64>::decode(&mut a).unwrap();
		let o = u64::from(o);
	        println!("autors: {}",&o);
    		for j in 0..o { i=32; print!(" autor_{}: ",&j); prmas(&a[0..i]); a=&a[i..]; }
	    },

	    0x0803 => { println!("0803 peerReview refferToDoi(newer,older)");
		a=id(&a);
		a=doi(&a);
	    },

	    0x0500 => { println!("0500 Money Transfer");
	        i=1; print!("Код 00: "); pr_mas(&a[0..i]);
	        if a[0] != 0x00 { return Result::Err(format!("Некорректо, нужно 00")); }
	        a = &a[i..];

	        i=32; println!("TO: {}", user_name( hex::encode(&a[0..i]).as_str() ) ); a = &a[i..];

	        let o = Compact::<u64>::decode(&mut a).unwrap();
	    	let o = u64::from(o);
	        println!("Money: {}",&o);
	    },

	    _ => { println!("unknown"); },
	}

	println!("---- Len_: {} -------------------------------------------",&a.len());
	if a.len() == 0 { return Result::Ok(String::from("Отличненько")); }
        prmas(&a);
	Result::Err(String::from("НЕ Отличненько"))
}


pub fn moke_send_money(from: &str, to: &str, money: u128, nonce: u128) -> String {
    // Сама транза: 0500 00 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 04
    // let mut TRANZA: Vec<u8> = Vec::new();
    let mut tranza = Tran { ara: [].to_vec() };
    tranza.hex("050000");
    tranza.add_user(to);
    tranza.compact(money); // 9990000000000);

    // теперь бы ее суку как-то подписать Алисой
    let sign = singme( &tranza.ara,  from );
    
    let era: u128 = 0; // 149;
    // let Nonce: u128 = 1; // 8;
    // let s: Value = client.request("system_accountNextIndex", rpc_params![ &format!( "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" ) ]).await?;
    // let Nonce: u128 = s.as_u128().unwrap();


    let mut tr = Tran { ara: [].to_vec() };
    // Начало 84 (?? compact 33) 00
    tr.hex("8400");
    // From: Alice
    tr.add_user("Alice");
    // код 01
    tr.hex("01");
    // Sign
    // for x in 1..65 { TR.hex("77"); }
    tr.sign(sign);

    // Эра: НЕ compact!
    // TR.compact(era);
    tr.hex("00");

    // Nonce:01
    tr.compact(nonce);
    // Код 00
    tr.hex("00");
    // А теперь в конце посылки сама транзакция:
    tr.bytes(&tranza.ara);
    tr.add_len();
    // END

    let str = hex::encode(&tr.ara);
    str
    // moke(&str).unwrap();

    // Result::Err(String::from("НЕ Отличненько"))
}


pub fn singme(message: &[u8], account: &str) -> Signature {
    // let suri = SecretUri::from_str(account).expect("Parse SURI");
    let pair = match sr25519::Pair::from_string(&account, None) {
            Ok(val) => val,
            Err(err) => {
                println!("==> Error Pair [{}]: {:#?}, тогда будем использовать [//Alice]",&account,&err);
                sr25519::Pair::from_string(&format!("//Alice"), None).unwrap()
            },
    };
    println!("==> sign: [{}]", hex::encode( pair.public().to_raw_vec() ));
    let blytes: Signature = pair.sign(&message[..]);
    println!("==> signed: [{}]", hex::encode(&blytes) );
    // проверяем подпись
    // let veri = sr25519::Pair::verify( &pair.sign(&message[..]) , &message[..], &pair.public() );
    // println!("--> проверка: {:#?}",&veri);
    // let s = format!("{}",&blytes); //  as &[u8]
    // let a: &[u8] = &hex::decode(hex).unwrap();    
    // return &blytes.0;
    blytes
}

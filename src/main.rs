#![allow(unused_imports)]
#![allow(unused_mut)]
use hex;
use sha2::{Sha256, Sha512, Digest};
use std::time::Instant;
use std::thread;
use std::ops::RangeInclusive;

const MAX_NONCE: RangeInclusive<i64> = 1..=100000000000;

//Hashing Function
fn hash_sha256(detail: String) -> String {
    // create a Sha256 object
    let mut hasher = Sha256::new();
    // write input message
    hasher.update(detail.as_bytes());

    // read hash digest and consume hasher
    let result = hex::encode(hasher.finalize());
    return result;
}

fn mine_coin(block:i32,tnxn:String, prev_hash:String,pref_zeroes:i32) -> Box<String> {
    let pre_str: String = "0".repeat(*&pref_zeroes as usize);
    //let MAX_NONCE = 1..=100000000000;
    let mut new_hash:String = String::new();
    for nonce in MAX_NONCE {
        let detail = block.to_string() + &tnxn + &prev_hash + &nonce.to_string();
        new_hash = hash_sha256(detail);
        if new_hash.starts_with(&pre_str) {
            println!("Successfully mined BTC with nonce value: {}", nonce);
            return Box::new(new_hash);
        } 
        
    }
    return Box::new(new_hash);
}

fn main() {
    
    let transactions = "Anand->Ashin->20,
                Richard->Gilfoyle->45".to_string();
    let difficulty = 6;
    let block = 5;
    
    println!("Starts Mining...");
    let start = Instant::now();
    let handle = thread::spawn(move|| {
        let new_hash = Box::new(mine_coin(block,transactions,
                "0000000xa036944e29568d0cff17edbe038f81208fecf9a66be9a2b8321c6ec9".to_string(),
                difficulty)
                );
        println!("New Hash ID:{}", new_hash);

    });
    handle.join().unwrap();
    let duration = start.elapsed();
    println!("Mining took : {:?} seconds", duration);
}

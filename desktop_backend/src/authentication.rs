use std::{thread, time};
use std::io;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
const pwLength: u32 = 128;

pub fn generate_token() -> String {
    
    let mut rng = rand::thread_rng();

    let password: String = (0..pwLength)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    
    password
} 




extern crate env_logger;
extern crate openssl;
extern crate reqwest;
extern crate requests;

use std::collections::HashMap;

const SUCCESS: u32 = 0;
const FAILURE: u32 = 1;

#[no_mangle]
pub extern fn hello() -> u32 {
    // env_logger::init();
    println!("Hello from Rust");
    SUCCESS
}

#[no_mangle]
pub extern fn get() -> u32 {
    match reqwest::get("https://httpbin.org/ip") {
        Ok(_) => {
            println!("GET succeeded!");
            SUCCESS
        },
        Err(e) => {
            println!("GET failed!"); 
            println!("{}", e);
            FAILURE
        }
    }
}

#[no_mangle]
pub extern fn post() -> u32 {
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::Client::new();
    match client.post("https://httpbin.org/post").json(&map).send() {
        Ok(_) => {
            println!("POST Succeeded!");
            SUCCESS
        },
        Err(_) => {
            println!("POST Failed!");
            FAILURE
        }
    }
}

#[no_mangle]
pub extern fn get_with_requests() -> u32 {
    match requests::get("https://httpbin.org/get") {
        Ok(response) => {
            assert_eq!(response.status_code(), requests::StatusCode::Ok);
            SUCCESS
        }
        Err(_) => FAILURE
    }
}

#[no_mangle]
pub extern fn openssl_version_info() -> u32 {
    println!("{}", openssl::version::built_on());
    println!("{}", openssl::version::c_flags());
    println!("{}", openssl::version::dir());
    println!("{}", openssl::version::number());
    println!("{}", openssl::version::platform());
    println!("{}", openssl::version::version());
    SUCCESS
}

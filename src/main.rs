mod encryptor;
mod cvec;
mod auth;
mod token_cache;
mod udp_strategy;
mod errors;
mod tcp_strategy;
mod system_tunneling;
mod encryption_exchanger;

use crate::tcp_strategy::{TcpConnection, TcpStrategy};
use aes_gcm::aead::consts::U32;
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::Aead;
use aes_gcm::aes::cipher::crypto_common::Reset;
use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;
use crate::udp_strategy::{UdpConnection, UdpStrategy};
//todo
//send nonce in metadata message
//make clients registry

const KEY: [u8; 32] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
    0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
    0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
    0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
static KEY_: LazyLock<&GenericArray<u8, U32>> = LazyLock::new(move || Key::<Aes256Gcm>::from_slice(&KEY));

const SERVER_ADDR: &str = "127.0.0.1:59600";

static BIND_SOCKETS: Lazy<Arc<Mutex<Box<HashMap<Arc<&'static str>, AtomicBool>>>>> = Lazy::new(|| Arc::new(Mutex::new(Box::new(HashMap::new()))));
static TCP_SOCKETS: Lazy<Arc<Mutex<Box<HashMap<Arc<&'static str>, AtomicBool>>>>> = Lazy::new(|| Arc::new(Mutex::new(Box::new(HashMap::new()))));
#[tokio::main]
async fn main() {
    // let mut tcp_connection = TcpStrategy {
    //     state_holder: &TCP_SOCKETS
    // };
    // tcp_connection.initialize_socket().await.unwrap();
    // tcp_connection.bind_socket().await;

    let mut udp_connection = UdpStrategy {
        state_holder: &TCP_SOCKETS
    };

    udp_connection.initialize_socket().await.unwrap();
    udp_connection.bind_socket().await;
}

#[derive(Serialize, Deserialize)]
struct VpnPacket {
    data: Vec<u8>,
}
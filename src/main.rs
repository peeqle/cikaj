mod encryptor;
mod cvec;

use aes_gcm::aead::consts::{U12, U32};
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::{Aead, Nonce};
use aes_gcm::aes::cipher::crypto_common::Reset;
use aes_gcm::aes::Aes256;
use aes_gcm::{AeadCore, Aes256Gcm, AesGcm, Error, Key, KeyInit};
use log::{debug, error, info};
use once_cell::sync::Lazy;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::ops::Deref;
use std::process::Command;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, LazyLock};
use std::{error, fmt, thread};
use tokio::io;
use tokio::net::windows::named_pipe::PipeEnd::Server;
use tokio::net::UdpSocket;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
//todo
//send nonce in metadata message
//make clients registry

const KEY: [u8; 32] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
    0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
    0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
    0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
static KEY_: LazyLock<&GenericArray<u8, U32>> = LazyLock::new(move || Key::<Aes256Gcm>::from_slice(&KEY));

const SERVER_ADDR: &str = "0.0.0.0:8081";

static BIND_SOCKETS: Lazy<Arc<Mutex<Box<HashMap<Arc<&'static str>, AtomicBool>>>>> = Lazy::new(|| Arc::new(Mutex::new(Box::new(HashMap::new()))));

static SERVER_SOCKET: Lazy<Arc<Mutex<Option<UdpSocket>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

async fn initialize_socket() {
    let socket = UdpSocket::bind(&SERVER_ADDR).await.unwrap();

    let mut socket_guard = SERVER_SOCKET.lock().await;
    *socket_guard = Some(socket);

    let mut bind_ = BIND_SOCKETS.lock().await;
    bind_.insert(Arc::new(SERVER_ADDR), AtomicBool::new(true));
}

async fn bind_socket() {
    let socket_guard = SERVER_SOCKET.lock().await;
    if let Some(ref socket) = *socket_guard {
        spawn_worker().await;
        println!("{:#?}", BIND_SOCKETS);
    } else {
        error!("Socket is not initialized");
    }
}
//todo spawn for client store for use
async fn spawn_worker() {
    let serv = Arc::clone(&SERVER_SOCKET);
    let nma = tokio::spawn(async move {
        println!("asdasdasd");
        listen(serv).await;
    }).await.unwrap();

}

async fn listen(serv: Arc<Mutex<Option<UdpSocket>>>) {
    println!("ASdasd");
    let l_serv = serv.lock().await;
    let mut buf = vec![0; 1024];

    let (len, client_addr) = &l_serv.as_ref()
        .expect("Receiving error occurred")
        .recv_from(&mut buf).await.unwrap();
    println!("Received {} bytes from {}", len, client_addr);

    let data = buf[..*len].to_vec();

    let received_data = String::from_utf8_lossy(&data);
    println!("Received data: {}", received_data);

    let response = format!("Echo: {}", received_data);
    if let Err(e) = &l_serv.as_ref()
        .expect("Sending error occurred")
        .send_to(response.as_bytes(), &client_addr).await {
        eprintln!("Failed to send response: {}", e);
    }
}

#[tokio::main]
async fn main() {
    initialize_socket().await;

    bind_socket().await;
}

#[derive(Serialize, Deserialize)]
struct VpnPacket {
    data: Vec<u8>,
}

async fn init_sockets() -> Result<(UdpSocket, Vec<u8>), Error> {
    let listener = SocketAddr::from_str(SERVER_ADDR);
    debug!("Opening socket for {}", SERVER_ADDR);
    let socket = UdpSocket::bind(&listener.unwrap()).await.unwrap();
    debug!("Client listening on {}", SERVER_ADDR);

    Ok((socket, vec![0; 1024]))
}
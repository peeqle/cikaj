use crate::errors::SocketInitErrors;
use crate::SERVER_ADDR;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::{mpsc, Mutex};

static SERVER_UDP_SOCKET: Lazy<Arc<Mutex<Option<UdpSocket>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

#[derive(Debug)]
pub struct UdpStrategy {
    pub(crate) state_holder: &'static Arc<Mutex<Box<HashMap<Arc<&'static str>, AtomicBool>>>>,
}

pub(crate) trait UdpConnection {
    async fn initialize_socket(&self) -> Result<(), SocketInitErrors>;
    async fn bind_socket(&self);
}

impl UdpConnection for UdpStrategy {
    async fn initialize_socket(&self) -> Result<(), SocketInitErrors> {
        let socket = UdpSocket::bind(&SERVER_ADDR.parse::<SocketAddr>().unwrap()).await?;

        let mut socket_guard = SERVER_UDP_SOCKET.lock().await;
        *socket_guard = Some(socket);

        let mut bind_ = self.state_holder.lock().await;
        bind_.insert(Arc::new(SERVER_ADDR), AtomicBool::new(true));
        Ok(())
    }

    async fn bind_socket(&self) {
        let cl = SERVER_UDP_SOCKET.lock().await;
        let socket_guard = Arc::new(Box::new(cl).take().unwrap());

        tokio::spawn(async move {
            listen_udp(socket_guard).await;
        })
        .await
        .unwrap();
        println!("{:#?}", self.state_holder);
    }
}

async fn listen_udp(serv: Arc<UdpSocket>) {
    let (sender, mut receiver) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);
    let s = Arc::clone(&serv);

    tokio::spawn(async move {
        while let Some((bytes, addr)) = receiver.recv().await {
            let len = s.as_ref().send_to(&bytes, &addr).await.unwrap();
            println!("{:?} bytes sent", len);
        }
    });

    let mut buf = [0; 196];
    loop {
        let (len, addr) = match serv.clone().as_ref().recv_from(&mut buf).await {
            Ok((len, addr)) => {
                println!(
                    "Received {} bytes from {:?} -> {:?}",
                    len,
                    addr,
                    String::from_utf8_lossy(&buf).trim()
                );
                (len, addr)
            }
            Err(e) => {
                println!("Error reading from socket: {:?}", e);
                return;
            }
        };

        if let Err(e) = sender.send((buf[0..len].to_vec(), addr)).await {
            println!("Failed to send response: {}", e);
        }
    }
}

use crate::errors::SocketInitErrors;
use crate::SERVER_ADDR;
use log::error;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::net::{TcpListener, UdpSocket};
use tokio::sync::{mpsc, Mutex, MutexGuard};

static SERVER_UDP_SOCKET: Lazy<Arc<Mutex<Option<UdpSocket>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

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
        let socket_guard = SERVER_UDP_SOCKET.lock().await;
        if let Some(ref socket) = *socket_guard {
            tokio::spawn(async move {
                listen_udp(socket_guard).await;
            }).await.unwrap();
            println!("{:#?}", self.state_holder);
        } else {
            error!("Socket is not initialized");
        }
    }
}

async fn listen_udp(serv: MutexGuard<'static, Option<UdpSocket>>) {
    let (sender, mut receiver) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);
    let s = serv?.clone();

    tokio::spawn(async move {
        while let Some((bytes, addr)) = receiver.recv().await {
            let len = s.as_ref().send_to(&bytes, &addr).await.unwrap();
            println!("{:?} bytes sent", len);
        }
    });

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = serv.as_ref().expect("Error while opening UDP receiver")
            .recv_from(&mut buf).await.unwrap();

        println!("{:?} bytes received from {:?}", len, addr);
        sender.send((buf[..len].to_vec(), addr)).await.unwrap();
    }
}
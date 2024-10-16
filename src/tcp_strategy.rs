use crate::errors::SocketInitErrors;
use crate::SERVER_ADDR;
use log::error;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::ops::Deref;
use std::ptr::copy;
use std::str::FromStr;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpSocket, UdpSocket};
use tokio::sync::{Mutex, MutexGuard};

static SERVER_TCP_SOCKET: Lazy<Arc<Mutex<Option<TcpListener>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

#[derive(Debug, Copy, Clone)]
pub struct TcpStrategy {
    pub(crate) state_holder: &'static Arc<Mutex<Box<HashMap<Arc<&'static str>, AtomicBool>>>>,
}
pub(crate) trait TcpConnection {
    async fn initialize_socket(&self) -> Result<(), SocketInitErrors>;
    async fn bind_socket(&self);
}

impl TcpConnection for TcpStrategy {
    async fn initialize_socket(&self) -> Result<(), SocketInitErrors> {
        let mut socket = TcpSocket::new_v4()?;

        socket.set_linger(Some(Duration::new(5, 0)))?;
        socket.set_reuseaddr(true)?;

        socket.bind(SocketAddr::from_str(&SERVER_ADDR)?)?;

        let listener = socket.listen(1)?;

        let mut socket_guard = SERVER_TCP_SOCKET.lock().await;
        *socket_guard = Some(listener);

        let mut bind_ = self.state_holder.lock().await;
        bind_.insert(Arc::new(SERVER_ADDR), AtomicBool::new(true));
        Ok(())
    }

    async fn bind_socket(&self) {
        let socket_guard = SERVER_TCP_SOCKET.lock().await;
        if let Some(ref socket) = *socket_guard {
            tokio::spawn(async move {
                listen_tcp(socket_guard).await;
            }).await.unwrap();
            println!("{:#?}", self.state_holder);
        } else {
            error!("Socket is not initialized");
        }
    }
}

async fn listen_tcp(serv: MutexGuard<'static, Option<TcpListener>>) {
    println!("Connected service");

    match serv.as_ref()
        .expect("as")
        .accept().await {
        Ok((mut _socket, addr)) => {
            println!("new client: {:?}", addr);
            loop {
                let mut buf = vec![0; 16];
                let n = _socket.read(&mut buf).await
                    .expect("TODO: panic message");
                if n > 0 {
                    println!("Data {:?}", String::from_utf8_lossy(&buf))
                }
            }
        }
        Err(e) => println!("couldn't get client: {:?}", e),
    }
}
use crate::tcp_strategy::{TcpConnection, TcpStrategy};

pub(crate) async fn exchange() {
    let tcp_socket = TcpStrategy::default();

    tcp_socket.initialize_socket().await.unwrap();
    tcp_socket.bind_socket().await;
}

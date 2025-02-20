
pub async fn find(start_port: u16) -> Option<u16> {
    (start_port..start_port + 100).find(|port| {
        std::net::TcpListener::bind(format!("0.0.0.0:{}", port)).is_ok()
    })
}

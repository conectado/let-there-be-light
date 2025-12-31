use std::net::{Ipv4Addr, SocketAddrV4};

const UPSTREAM: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 13355);

#[tokio::main]
async fn main() {
    loop {
        let Ok(socket) = tokio::net::UdpSocket::bind("0.0.0.0:0").await else {
            continue;
        };

        loop {
            let _ = socket
                .send_to("In the first day...".as_bytes(), UPSTREAM)
                .await;

            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }
}

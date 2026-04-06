mod dns;
mod models;
mod web;

use dns::{DnsInspector, SharedState};
use hickory_server::server::ServerFuture;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Merkezi state oluşturuluyor
    let shared_state: SharedState = Arc::new(RwLock::new(Vec::new()));

    // Web sunucusu başlatılıyor
    let app = web::create_router(shared_state.clone());
    let web_server = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    // DNS sunucusu başlatılıyor
    let inspector = DnsInspector::new(shared_state);
    let mut server = ServerFuture::new(inspector);

    let listen_addr = "127.0.0.1:5300";
    let udp_socket = UdpSocket::bind(listen_addr).await?;
    server.register_socket(udp_socket);

    println!("--------------------------------------------------");
    println!("🛡️ DNS Müfettişi ve Dashboard Başlatıldı!");
    println!("📡 DNS Dinleniyor     : udp://{}", listen_addr);
    println!("🌐 Web Arayüzü        : http://localhost:3000");
    println!("🔒 DoH Sağlayıcı      : Cloudflare-HTTPS (1.1.1.1)");
    println!("--------------------------------------------------");

    let _ = tokio::try_join!(
        async { server.block_until_done().await.map_err(|e| e.into()) },
        async { web_server.await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>) }
    );

    Ok(())
}
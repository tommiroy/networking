use serde::{Deserialize, Serialize};
use std::{clone, env};

use tokio::fs::File;
use tokio::io::AsyncReadExt;
use warp::*;

#[derive(Serialize, Deserialize)]
struct Request {
    message: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Idmsg {
    identity: u32,
    text: String,
}

async fn run_server() {
    // specific message

    // Get /

    let route1 = warp::post()
        .and(warp::path("message"))
        .and(warp::body::json())
        .map(|request: Idmsg| {
            println!(
                "Received message from {}: {}",
                request.identity, request.text
            );
            warp::reply::json(&request)
        });

    let route2 = warp::post()
        .and(warp::path("route2"))
        .and(warp::body::json())
        .map(|request: serde_json::Value| {
            println!("Received message: {:?}", request);
            warp::reply::json(&request)
        });

    warp::serve(route1.or(route2))
        .tls()
        .key_path("server/localhost.key")
        .cert_path("server/localhost.bundle.crt")
        .client_auth_required_path("ca/ca.crt")
        .run(([172, 18, 0, 6], 3030))
        .await;
}

async fn run_client() -> Result<(), reqwest::Error> {
    let server_ca_file_loc = "ca/ca.crt";
    let mut buf = Vec::new();
    File::open(server_ca_file_loc)
        .await
        .unwrap()
        .read_to_end(&mut buf)
        .await
        .unwrap();
    let cert = reqwest::Certificate::from_pem(&buf)?;

    #[cfg(feature = "rustls-tls")]
    async fn get_identity() -> reqwest::Identity {
        let client_pem_file_loc = "client/client_0.pem";
        let mut buf = Vec::new();
        File::open(client_pem_file_loc)
            .await
            .unwrap()
            .read_to_end(&mut buf)
            .await
            .unwrap();
        reqwest::Identity::from_pem(&buf).unwrap()
    }

    let identity = get_identity().await;

    #[cfg(feature = "rustls-tls")]
    let client = reqwest::Client::builder().use_rustls_tls();

    let client = client
        .tls_built_in_root_certs(false)
        .add_root_certificate(cert)
        .identity(identity)
        .https_only(true)
        .build()?;

    let request = Idmsg {
        identity: 51,
        text: "Hello, world!".to_string(),
    };

    let request2 = Idmsg {
        identity: 52,
        text: "To route2".to_string(),
    };

    let server_ip = "https://server:3030/";

    let ras = send_message(&server_ip, &client, "route2", request2.clone()).await;
    let res = send_message(&server_ip, &client, "message", request).await;
    println!("Received:");
    println!("Server responded with message: {:?}", res);
    println!("Received:");
    println!("Server responded with message: {:?}", ras);
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "server" {
        let server = run_server();
        server.await;
    } else if args[1] == "client" {
        let client = run_client();
        client.await.unwrap();
    };
}

pub async fn send_message(
    server_ip: &str,
    client: &reqwest::Client,
    channel: &str,
    request: Idmsg,
) -> reqwest::Response {
    client
        .post(server_ip.to_owned() + channel)
        .body(serde_json::to_string(&request).unwrap())
        .send()
        .await
        .unwrap()
}

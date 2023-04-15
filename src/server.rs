
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::{clone, env};

use tokio::fs::File;
use tokio::io::AsyncReadExt;
use warp::*;
use tokio::sync::mpsc::{UnboundedSender};


#[derive(Serialize, Deserialize)]
struct Request {
    message: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Idmsg {
    identity: u32,
    text: String,
}

pub async fn run_server(tx: UnboundedSender<String>) {
    let warp_tx = warp::any().map(move || tx.clone());

    let route1 = warp::post()
    .and(warp::path("message"))
    .and(warp::body::json())
    .and(warp_tx.clone())
    .map(|request: Idmsg, warp_tx: UnboundedSender<String>| {
        println!(
            "Received message from {}: {}",
            request.identity, request.text
        );
        warp_tx.send(request.clone().text);
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
    .key_path("src/server/localhost.key")
    .cert_path("src/server/localhost.bundle.crt")
    .client_auth_required_path("src/ca/ca.crt")
    .run(([0, 0, 0, 0], 3030))
    .await;


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

// ########################################################################################
#[derive(Clone)]
pub struct Server {
    cert:   String,
    key:    String,
    ca:     String,
    port:   String,
    clients: Vec<String>,
}

impl Server {

    pub fn new(cert: String, key: String, ca: String, port: String) -> Server{
        Self {cert, key, ca, port, clients: Vec::<String>::new()}
    }

    pub fn add_client(&mut self, addr: String) {
        self.clients.push(addr);
    }



} 
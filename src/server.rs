
use serde::{Deserialize, Serialize};

use warp::*;
use tokio::sync::mpsc::{UnboundedSender};

use super::helper::{Message, MsgType};


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
        // println!(
        //     "Message: Received message from {}: {}",
        //     request.identity, request.text
        // );
        if let Err(e) = warp_tx.send(request.clone().text) {
            println!("Cant send message back to main: {e:?}");
        } else {
            print!("Sent message back to main");
        };
        warp::reply::json(&request)
    });
    
    let route2 = warp::post()
    .and(warp::path("route2"))
    .and(warp::body::json())
    .map(|request: serde_json::Value| {
        // println!("Received message: {:?}", request);
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

    pub async fn new(cert: String, key: String, ca: String, port: String, tx: UnboundedSender<String>) -> Server{
        tokio::spawn(async move {
            _serve(tx).await;
        });

        Self {cert, key, ca, port, clients: Vec::<String>::new()}
    }

    pub fn add_client(&mut self, addr: String) {
        self.clients.push(addr);
    }

    

} 

async fn _serve(tx: UnboundedSender<String>) {
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
        if let Err(e) = warp_tx.send(request.clone().text) {
            panic!("Cant relay message back to main thread!. Error: {e}");
        }
        warp::reply::json(&request)
    });
    
    let route2 = warp::post()
    .and(warp::path("route2"))
    .and(warp::body::json())
    .map(|request: serde_json::Value| {
        // println!("Received message: {:?}", request);
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
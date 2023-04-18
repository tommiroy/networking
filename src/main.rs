/// ###################################################################
/// Argument options
/// Dont care about these
/// ###################################################################
use clap::{Parser, Subcommand, Args};
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct App {
    /// Name of the person to greet
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Subcommand)]
enum Mode {
    /// Server mode
    Server (ServerOption),
    /// Client mode
    Client (ServerOption),
}

#[derive(Args, Debug)]
struct ServerOption {

    /// Certificates path
    #[arg(long)]
    cert: String,

    /// Private key path
    #[arg(long)]
    key:String,

    /// Certificate Authority path
    #[arg(long)]
    ca: String,

    /// Server port
    #[arg(long)]
    port: String,

}

mod server;
mod client;
mod helper;


use client::{run_client};
use server::{Server, run_server};


use tokio::sync::mpsc::{Sender, unbounded_channel};


// Testing only
use serde::{Deserialize, Serialize};




/// ###################################################################
/// Main Function
/// ###################################################################

#[tokio::main]
async fn main() {
    let args = App::parse();

    match &args.mode {
        // Start as a server
        Mode::Server (ServerOption { cert, key, ca, port}) => {
            let (tx, mut rx) = unbounded_channel::<String>();
            let mut my_server = Server::new(
                cert.to_string(), 
                key.to_string(), 
                ca.to_string(),
                port.to_string(),
                tx.clone()).await;
            
            // test for serializing and deserializing objects. 
            // if let Ok(test_server_serialized) = serde_json::to_string(&my_server) {
            //     println!("{test_server_serialized}");
            //     let deserialized_server = serde_json::from_slice::<Server>(test_server_serialized.as_bytes());
            //     println!("{deserialized_server:?}")
            // } else {
            //     println!("Could not serialized the server");
            // }


            
            // my_server.add_client("test".to_owned());

            // while let Some(msg) = rx.recv().await {
            //     println!("Got a message: {msg}");

            // }

            while let msg = rx.recv().await {
                match msg.clone() {
                    Some(str) => {
                        println!("Got a message: {}", str);
                    }
                    _ => {
                        println!("Wrong format!")
                    }
                }

            }
        }

        // Start as a client
        Mode::Client (option) => {
            let _ = run_client("localhost:3030".to_string()).await;
            // println!("Arguments for Client: {option:?}");
        }
    }

}

// ###################################################################

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
}

mod server;
mod mainbak;
mod client;



use client::{run_client};
use server::{run_server};


use tokio::sync::mpsc::{Sender, unbounded_channel};

/// ###################################################################
/// Main Function
/// ###################################################################

#[tokio::main]
async fn main() {
    let args = App::parse();

    match &args.mode {
        Mode::Server (option) => {
            let (tx, mut rx) = unbounded_channel::<String>();

            tokio::spawn(async move {
                run_server(tx).await;
            });

            while let Some(msg) = rx.recv().await {
                println!("Got a message: {msg}");

            }
        }
        Mode::Client (option) => {
            let _ = run_client("localhost:3030".to_string()).await;
            println!("Arguments for Client: {option:?}");
        }
    }

}

// ###################################################################

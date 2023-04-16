use reqwest::{Certificate, Identity};
use tokio::fs::{File};
use tokio::io::AsyncReadExt;
use serde::{Deserialize, Serialize};


/// ######################################################
/// Read certificates for sending HTTPS request - reqwest 
/// ######################################################

pub async fn reqwest_read_cert(path: String) -> Certificate {
    let mut buf = Vec::new();
    File::open(path)
        .await
        .unwrap()
        .read_to_end(&mut buf)
        .await
        .unwrap();
    reqwest::Certificate::from_pem(&buf).unwrap()

}


/// ######################################################
/// Generate identity for the request sender - reqwest
/// ######################################################

pub async fn get_identity(path: String) -> Identity {
    let mut buf = Vec::new();
    File::open(path)
        .await
        .unwrap()
        .read_to_end(&mut buf)
        .await
        .unwrap();
    reqwest::Identity::from_pem(&buf).unwrap()
}

/// ######################################################
/// Message description
/// ######################################################
#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    // Should have sender and receiver anyways
    sender:     String,
    receiver:   String,
    // 
    msg_type:       MsgType,

}
#[derive(Clone, Serialize, Deserialize)]
pub enum MsgType {
    KEYGEN,
    CO
}
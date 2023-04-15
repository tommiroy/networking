use reqwest::{Certificate, Identity};
use tokio::fs::{File};
use tokio::io::AsyncReadExt;

    // Read certificates for sending HTTPS request with reqwest 
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


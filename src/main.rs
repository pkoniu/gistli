extern crate base64;
extern crate futures;
extern crate hyper;
extern crate tokio;

use base64::encode;
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let header_from_env = fs::read_to_string(".env")
        .expect("Missing env file containing github authorization header");
    let encoded_token = encode(&header_from_env);
    let basic_auth_header = "basic ".to_string();
    let header_with_token = basic_auth_header + &encoded_token;

    // let file_content =
    //     fs::read_to_string("address.txt").expect("Something went wrong reading the file");

    let req = Request::builder()
        .method(Method::POST)
        .uri("https://api.github.com/gists")
        .header("content-type", "application/json")
        .header("authorization", header_with_token)
        .header("accept", "application/vnd.github.v3+json")
        .header("user-agent", "gistli")
        .body(Body::from(r#"{"description": "Gistli test","public": true,"files": {"test1.py": {"content": "print(\"hello\")"}}}"#))?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let resp = client.request(req).await?;
    println!("Response: {}", resp.status());

    let buf = hyper::body::to_bytes(resp).await?;
    println!("body: {:?}", buf);

    Ok(())
}

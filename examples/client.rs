#![deny(warnings)]
#![warn(rust_2018_idioms)]
use hyper_tls::HttpsConnector;
use std::env;

use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
  // Some simple CLI args requirements...
  let url = match env::args().nth(1) {
    Some(url) => url,
    None => {
      println!("Usage: client <url>");
      return Ok(());
    }
  };

  fetch_url(url).await
}

async fn fetch_url(url: String) -> Result<()> {
  let https = HttpsConnector::new();
  let client = Client::builder().build::<_, hyper::Body>(https);

  let mut res = client.get(url.parse()?).await?;

  // Stream the body, writing each chunk to stdout as we get it
  // (instead of buffering and printing at the end).
  while let Some(next) = res.data().await {
    let chunk = next?;
    io::stdout().write_all(&chunk).await?;
  }

  Ok(())
}

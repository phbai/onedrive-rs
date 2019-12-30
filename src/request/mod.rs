use bytes::Buf;
use hyper::Client;
use hyper_tls::HttpsConnector;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_json(url: &str) -> Result<impl Buf> {
  let https = HttpsConnector::new();
  let client = Client::builder().build::<_, hyper::Body>(https);

  // Fetch the url...
  let res = client.get(url.parse()?).await?;

  // asynchronously aggregate the chunks of the body
  let body = hyper::body::aggregate(res).await?;
  Ok(body)
}

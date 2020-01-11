use crate::entity::DriveItemMetadata;
use crate::util::build_get_request;
use crate::HyperClient;
use bytes::buf::BufExt as _;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn get_metadata(client: &HyperClient, path: &str) -> Result<DriveItemMetadata> {
  let url = match path {
    "/" => String::from("https://microsoftgraph.chinacloudapi.cn/v1.0/me/drive/root"),
    v => format!(
      "https://microsoftgraph.chinacloudapi.cn/v1.0/me/drive/root:{}",
      v
    ),
  };

  let req = build_get_request(url).await;
  let res = client.request(req).await?;
  let body = hyper::body::aggregate(res).await?;
  let metadata: DriveItemMetadata = serde_json::from_reader(body.reader())?;
  Ok(metadata)
}

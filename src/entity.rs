fn default_download_url() -> Option<String> {
  Some("".to_string())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriveItemList {
  value: Vec<DriveItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriveItem {
  name: String,
  size: u64,
  #[serde(rename = "lastModifiedDateTime")]
  last_modified: String,
  #[serde(
    rename(
      serialize = "downloadUrl",
      deserialize = "@microsoft.graph.downloadUrl"
    ),
    default = "default_download_url"
  )]
  download_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriveItemMetadata {
  name: String,
  size: u64,
  #[serde(
    rename(
      serialize = "downloadUrl",
      deserialize = "@microsoft.graph.downloadUrl"
    ),
    default = "default_download_url"
  )]
  download_url: Option<String>,
  file: Option<DriveItemFile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriveItemFile {
  #[serde(rename = "mimeType")]
  mime_type: String,
}

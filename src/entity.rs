fn default_download_url() -> Option<String> {
  Some("".to_string())
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GrantType {
  #[serde(rename(serialize = "refresh_token"))]
  RefreshToken,
  #[serde(rename(serialize = "authorization_code"))]
  AuthorizationCode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriveItemList {
  #[serde(skip_serializing)]
  pub error: Option<String>,
  #[serde(skip_serializing)]
  pub error_description: Option<String>,
  value: Option<Vec<DriveItem>>,
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
  pub name: String,
  pub size: u64,
  #[serde(
    rename(
      serialize = "downloadUrl",
      deserialize = "@microsoft.graph.downloadUrl"
    ),
    default = "default_download_url"
  )]
  pub download_url: Option<String>,
  pub file: Option<DriveItemFile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriveItemFile {
  #[serde(rename = "mimeType")]
  mime_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriveSecret {
  pub access_token: Option<String>,
  pub refresh_token: Option<String>,
  #[serde(skip_serializing)]
  pub error: Option<String>,
  #[serde(skip_serializing)]
  pub error_description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshTokenRequest {
  pub client_id: String,
  pub redirect_uri: String,
  pub client_secret: String,
  pub code: String,
  pub grant_type: GrantType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenRequest {
  pub client_id: String,
  pub redirect_uri: String,
  pub client_secret: String,
  pub refresh_token: String,
  pub grant_type: GrantType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthConfig {
  pub access_token: String,
  pub refresh_token: String,
}

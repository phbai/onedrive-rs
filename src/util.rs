use crate::entity::{AccessTokenRequest, DriveSecret, GrantType, RefreshTokenRequest};
use crate::HyperClient;
use hyper::{header, Body, Method, Request, Response, StatusCode};
use serde_urlencoded;
use std::error::Error;
use std::fmt;
use std::sync::RwLock;

use tokio::fs;
use tokio::fs::File;
use tokio::io::{stdin, BufReader};
use tokio::prelude::*;

use bytes::buf::BufExt as _;

lazy_static! {
  static ref ACCESS_TOKEN: RwLock<String> = RwLock::new(String::from(""));
}

#[derive(Debug)]
pub struct OneDriveError(String);

impl fmt::Display for OneDriveError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "OneDriveError: {}", self.0)
  }
}

impl Error for OneDriveError {}

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, GenericError>;

pub fn build_json_response(json: String) -> Result<Response<Body>> {
  Ok(
    Response::builder()
      .status(StatusCode::OK)
      .header(header::CONTENT_TYPE, "application/json")
      .body(Body::from(json))?,
  )
}

pub fn build_get_request(url: String) -> Request<Body> {
  Request::builder()
    .method(Method::GET)
    .uri(url)
    .header(
      header::AUTHORIZATION,
      format!("bearer {}", ACCESS_TOKEN.read().unwrap()),
    )
    .body(Default::default())
    .unwrap()
}

pub fn build_refresh_token_request(code: String) -> Result<Request<Body>> {
  let url = "https://login.partner.microsoftonline.cn/common/oauth2/v2.0/token";
  let refresh_token_request = RefreshTokenRequest {
    client_id: "dfe36e60-6133-48cf-869f-4d15b8354769".to_string(),
    redirect_uri: "http://localhost/onedrive-login".to_string(),
    client_secret: "H0-1:6.Sb8:WCW/J-c]K@fddCt[i0EZ2".to_string(),
    code,
    grant_type: GrantType::AuthorizationCode,
  };

  let res = serde_urlencoded::to_string(&refresh_token_request)?;
  Ok(
    Request::builder()
      .method(Method::POST)
      .uri(url)
      .body(Body::from(res))
      .unwrap(),
  )
}

pub fn build_access_token_request(refresh_token: String) -> Result<Request<Body>> {
  let url = "https://login.partner.microsoftonline.cn/common/oauth2/v2.0/token";
  let refresh_token_request = AccessTokenRequest {
    client_id: "dfe36e60-6133-48cf-869f-4d15b8354769".to_string(),
    redirect_uri: "http://localhost/onedrive-login".to_string(),
    client_secret: "H0-1:6.Sb8:WCW/J-c]K@fddCt[i0EZ2".to_string(),
    refresh_token,
    grant_type: GrantType::RefreshToken,
  };

  let res = serde_urlencoded::to_string(&refresh_token_request)?;
  Ok(
    Request::builder()
      .method(Method::POST)
      .uri(url)
      .body(Body::from(res))
      .unwrap(),
  )
}

// 获取refresh_token
async fn init_token(client: &HyperClient, code: String) -> Result<DriveSecret> {
  let req = build_refresh_token_request(code)?;
  let res = client.request(req).await?;
  let body = hyper::body::aggregate(res).await?;
  let secret: DriveSecret = serde_json::from_reader(body.reader())?;

  if secret.access_token.is_none() {
    Err(Box::new(OneDriveError(secret.error_description.unwrap())))
  } else {
    Ok(secret)
  }
}

// 刷新access_token
async fn refresh_token(client: &HyperClient, refresh_token: String) -> Result<DriveSecret> {
  let req = build_access_token_request(refresh_token)?;
  let res = client.request(req).await?;
  let body = hyper::body::aggregate(res).await?;
  let secret: DriveSecret = serde_json::from_reader(body.reader())?;

  if secret.access_token.is_none() {
    Err(Box::new(OneDriveError(secret.error_description.unwrap())))
  } else {
    Ok(secret)
  }
}

pub async fn init_config(client: &HyperClient) -> Result<()> {
  match fs::read_to_string("config.json").await {
    Ok(contents) => {
      let secret: DriveSecret = serde_json::from_str(contents.as_str())?;
      println!("secret:{:?}", secret);
      match refresh_token(&client, secret.refresh_token.unwrap()).await {
        Ok(secret) => {
          // 新的有效的access_token
          println!("secret:{:?}", secret);
          let mut access_token = ACCESS_TOKEN.write().unwrap();
          *access_token = secret.access_token.unwrap();
        }
        Err(err) => {
          println!("获取access_token失败:{}", err);
        }
      }
    }
    Err(_err) => {
      // stdout().write_all(b"code:").await?;
      let mut reader = BufReader::new(stdin());
      let mut code = String::new();

      // read a line into buffer
      reader.read_line(&mut code).await?;
      // let mut line = String::new();
      // io::stdin().read_line(&mut line)?;

      match init_token(&client, code.trim().to_string()).await {
        Ok(secret) => {
          create_file(&secret).await?;
        }
        Err(err) => {
          println!("获取access_token失败:{}", err);
        }
      }
    }
  };
  Ok(())
}

pub async fn create_file(secret: &DriveSecret) -> Result<()> {
  let mut file = File::create("config.json").await?;
  let json = serde_json::to_string_pretty(secret)?;
  file.write_all(json.as_bytes()).await?;
  Ok(())
}

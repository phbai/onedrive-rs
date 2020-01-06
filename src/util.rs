use crate::entity::{DriveSecret, GrantType, RefreshTokenRequest};
use crate::HyperClient;
use hyper::{header, Body, Method, Request, Response, StatusCode};
use serde_urlencoded;
use std::error::Error;
use std::fmt;

use tokio::fs;
use tokio::fs::File;
use tokio::io::{stdin, BufReader};
use tokio::prelude::*;

use bytes::buf::BufExt as _;

static ACCESS_TOKEN: &'static str = "eyJ0eXAiOiJKV1QiLCJub25jZSI6IlRjamNOVlpIVm5mZHFIRG8xWnRna1FHWEFxaTRyMENXMnR0NHNpZE5pdjAiLCJhbGciOiJSUzI1NiIsIng1dCI6InpqZ3V5bi16NzY0MENONHpPY2hTOVhXbXZmUSIsImtpZCI6InpqZ3V5bi16NzY0MENONHpPY2hTOVhXbXZmUSJ9.eyJhdWQiOiIwMDAwMDAwMy0wMDAwLTAwMDAtYzAwMC0wMDAwMDAwMDAwMDAiLCJpc3MiOiJodHRwczovL3N0cy5jaGluYWNsb3VkYXBpLmNuLzNiMWM4MWIxLWQxNTYtNGFmOS1iMTY5LWFlMDgxMjhjMDM5Ni8iLCJpYXQiOjE1NzgwNTA4ODksIm5iZiI6MTU3ODA1MDg4OSwiZXhwIjoxNTc4MDU0Nzg5LCJhY3IiOiIxIiwiYWlvIjoiWTJWZ1lMZ2FVSkpudVBmTzM4UmxFOVcwajN0blhVaEs1ekdWVFo3dGV0KzVXMGYyMGdrQSIsImFtciI6WyJwd2QiXSwiYXBwX2Rpc3BsYXluYW1lIjoiT25lRHJpdmUgZm9yIEFQSSIsImFwcGlkIjoiZGZlMzZlNjAtNjEzMy00OGNmLTg2OWYtNGQxNWI4MzU0NzY5IiwiYXBwaWRhY3IiOiIxIiwiaXBhZGRyIjoiNTguMzcuNjEuMjE0IiwibmFtZSI6Im1hcmlzYSIsIm9pZCI6Ijc1ZmViZGNjLTkzZjYtNGYwNy04YTFiLTAyOWJhNjE4MDY2ZiIsInBsYXRmIjoiNSIsInB1aWQiOiIxMDAzMzIzMEM1MUExM0M5Iiwic2NwIjoiRmlsZXMuUmVhZFdyaXRlLkFsbCBVc2VyLlJlYWQgcHJvZmlsZSBvcGVuaWQgZW1haWwiLCJzaWduaW5fc3RhdGUiOlsia21zaSJdLCJzdWIiOiJoUVozVy1YQko5STJNMEx2c3l3YW1yMU5rdmoxWS16cVQ5YWhETnlYaU5FIiwidGlkIjoiM2IxYzgxYjEtZDE1Ni00YWY5LWIxNjktYWUwODEyOGMwMzk2IiwidW5pcXVlX25hbWUiOiJtYXJpc2FAY25vZC54eXoiLCJ1cG4iOiJtYXJpc2FAY25vZC54eXoiLCJ1dGkiOiJwb1ZwUmF5Q3pVdTdyMGJrV2hvcEFBIiwidmVyIjoiMS4wIiwieG1zX3N0Ijp7InN1YiI6IjJ3Mi1lUXg3UldfTVRQVTUyeWl1S21jZF96ejd1anJlWXU4ME1VTkdGRjgifSwieG1zX3RjZHQiOjE0NTcxMTQ3Mzh9.QpJBTpNiM6xxSd_Ib70TrDeONhMexT9ENwPlvwE4Hm6jpShGqEoMC-A9u1I_YjelOKL8r0mlQSXlytxS4XmgWMBJhGAZ3YQNwwGUW0PjQDHm066vxhp9r12HqGTynuS7QrMcADuUzXQZwbX8-EoaidoMBVHcFpzzJ8VA-LsVLfCr6zrqIGN9RVJf5ysupcYG-rUISjRCSG2GYPrb27FGpRQyDGPvJs6Z6OOExDNDVs1keeAMajU429VZqbI04UNGF4kT2YThayYzZLS4gL7K1Q5Gz2gcEqblgLqB_s69Q6zEKNwNYZIZWRTr3pW3vjNVyu24-_7GWHAbGOQM9ZB_hg";

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
    .header(header::AUTHORIZATION, format!("bearer {}", ACCESS_TOKEN))
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
  println!("res:{}", res);
  Ok(
    Request::builder()
      .method(Method::POST)
      .uri(url)
      .body(Body::from(res))
      .unwrap(),
  )
}

async fn get_secret(client: &HyperClient, code: String) -> Result<DriveSecret> {
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

pub async fn init_config(client: &HyperClient) -> Result<()> {
  match fs::read_to_string("config.json").await {
    Ok(contents) => {
      let secret: DriveSecret = serde_json::from_str(contents.as_str())?;
      println!("secret:{:?}", secret);
    }
    Err(_err) => {
      // stdout().write_all(b"code:").await?;
      let mut reader = BufReader::new(stdin());
      let mut code = String::new();

      // read a line into buffer
      reader.read_line(&mut code).await?;
      // let mut line = String::new();
      // io::stdin().read_line(&mut line)?;

      match get_secret(&client, code.trim().to_string()).await {
        Ok(secret) => {
          create_file(&secret).await?;
          // println!("access_token:{}", access_token);
        }
        Err(err) => {
          println!("获取access_token失败:{}", err);
        }
      }
    }
  };
  // let file = ?;
  // let metadata = file.metadata().await?;

  // println!("{:?}", metadata);
  Ok(())
}

pub async fn create_file(secret: &DriveSecret) -> Result<()> {
  let mut file = File::create("config.json").await?;
  let json = serde_json::to_string_pretty(secret)?;
  file.write_all(json.as_bytes()).await?;
  Ok(())
}

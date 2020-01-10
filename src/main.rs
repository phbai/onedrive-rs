mod entity;
mod request;
mod util;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

use crate::util::OneDriveError;
use bytes::buf::BufExt as _;
use entity::{DriveItemList, DriveItemMetadata};
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Client, Method, Request, Response, Server, StatusCode};
use hyper_tls::HttpsConnector;
use regex::Regex;
use request::get_metadata;
use util::{build_get_request, build_json_response, init_config, GenericError, Result};
pub type HyperClient = Client<HttpsConnector<HttpConnector>>;

static INDEX: &[u8] = b"it works";
static NOTFOUND: &[u8] = b"Not Found";

async fn list_handler(client: &HyperClient, path: &str) -> Result<Response<Body>> {
    let url = match path {
        "/" => String::from("https://microsoftgraph.chinacloudapi.cn/v1.0/me/drive/root/children?select=name,size,folder,@microsoft.graph.downloadUrl,lastModifiedDateTime"),
        v => format!("https://microsoftgraph.chinacloudapi.cn/v1.0/me/drive/root:{}:/children?select=name,size,folder,@microsoft.graph.downloadUrl,lastModifiedDateTime", v),
    };
    let req = build_get_request(url).await;
    let res = client.request(req).await?;
    let body = hyper::body::aggregate(res).await?;
    let onedrive_result: DriveItemList = serde_json::from_reader(body.reader())?;
    let json = serde_json::to_string(&onedrive_result)?;

    build_json_response(json)
}

async fn file_handler(client: &HyperClient, path: &str) -> Result<Response<Body>> {
    let metadata = get_metadata(client, path).await?;
    let json = serde_json::to_string(&metadata)?;
    build_json_response(json)
}

async fn download_handler(client: &HyperClient, path: &str) -> Result<Response<Body>> {
    let metadata = get_metadata(client, path).await?;
    println!("metadata:{:?}", metadata);
    match metadata.download_url {
        Some(url) => {
            let req = build_get_request(url).await;
            let mut res = client.request(req).await?;
            res.headers_mut().remove(header::CONTENT_DISPOSITION);
            Ok(res)
        }
        None => Ok(Response::new(NOTFOUND.into())),
    }
}

async fn request_dispatcher(req: Request<Body>, client: HyperClient) -> Result<Response<Body>> {
    let path = req.uri().path();
    let list_regex = Regex::new(r"/list(?P<path>.+)").unwrap();
    let file_regex = Regex::new(r"/file(?P<path>.+)").unwrap();
    let download_regex = Regex::new(r"/download(?P<path>.+)").unwrap();

    if list_regex.is_match(path) {
        let caps = list_regex.captures(path).unwrap();
        if req.method() == &Method::GET {
            return match list_handler(&client, &caps["path"]).await {
                Ok(s) => Ok(s),
                Err(err) => {
                    println!("{:?}", err);
                    Err(Box::new(OneDriveError(err.to_string())))
                }
            };
        }
    }
    if file_regex.is_match(path) {
        let caps = file_regex.captures(path).unwrap();
        if req.method() == &Method::GET {
            return match file_handler(&client, &caps["path"]).await {
                Ok(s) => Ok(s),
                Err(err) => {
                    println!("{:?}", err);
                    Err(err)
                }
            };
        }
    }

    if download_regex.is_match(path) {
        let caps = download_regex.captures(path).unwrap();
        if req.method() == &Method::GET {
            return match download_handler(&client, &caps["path"]).await {
                Ok(s) => Ok(s),
                Err(err) => {
                    println!("{:?}", err);
                    Err(err)
                }
            };
        }
    }

    println!("path = {}", path);
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(INDEX.into())),
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let addr = "0.0.0.0:6333".parse().unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    init_config(&client).await?;
    // Share a `Client` with all `Service`s

    let new_service = make_service_fn(move |_| {
        // Move a clone of `client` into the `service_fn`.
        let client = client.clone();
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                // Clone again to ensure that client outlives this closure.
                request_dispatcher(req, client.to_owned())
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}

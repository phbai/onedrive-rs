// use futures_util::{stream, StreamExt};
#[macro_use]
extern crate serde_derive;
mod entity;
mod lib;
use bytes::buf::BufExt as _;
use entity::{DriveItemList, DriveItemMetadata};
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Client, Method, Request, Response, Server, StatusCode};
use hyper_tls::HttpsConnector;
use lib::{generate_get_request, generate_json, GenericError, Result};
use regex::Regex;

type HyperClient = Client<HttpsConnector<HttpConnector>>;

static INDEX: &[u8] = b"it works";
static NOTFOUND: &[u8] = b"Not Found";

async fn list_handler(client: &HyperClient, path: &str) -> Result<Response<Body>> {
    let url = match path {
        "/" => String::from("https://microsoftgraph.chinacloudapi.cn/v1.0/me/drive/root/children?select=name,size,folder,@microsoft.graph.downloadUrl,lastModifiedDateTime"),
        v => format!("https://microsoftgraph.chinacloudapi.cn/v1.0/me/drive/root:{}:/children?select=name,size,folder,@microsoft.graph.downloadUrl,lastModifiedDateTime", v),
    };
    println!("url:{}", url);
    let req = generate_get_request(url);
    let res = client.request(req).await?;
    let body = hyper::body::aggregate(res).await?;
    let onedrive_result: DriveItemList = serde_json::from_reader(body.reader())?;
    println!("onedrive_result: {:?}", onedrive_result);
    let json = serde_json::to_string(&onedrive_result)?;

    generate_json(json)
}

async fn file_handler(client: &HyperClient, path: &str) -> Result<Response<Body>> {
    let url = match path {
        "/" => String::from("https://microsoftgraph.chinacloudapi.cn/v1.0/me/drive/root"),
        v => format!(
            "https://microsoftgraph.chinacloudapi.cn/v1.0/me/drive/root:{}",
            v
        ),
    };

    let req = generate_get_request(url);
    let res = client.request(req).await?;
    let body = hyper::body::aggregate(res).await?;
    let metadata: DriveItemMetadata = serde_json::from_reader(body.reader())?;
    let json = serde_json::to_string(&metadata)?;

    generate_json(json)
}

async fn download_handler(client: &HyperClient, _path: &str) -> Result<Response<Body>> {
    let url = String::from("https://alphaone-my.sharepoint.cn/personal/marisa_cnod_xyz/_layouts/15/download.aspx?UniqueId=3a6b217a-16a3-4cdb-9c85-c585be3b52cc&Translate=false&tempauth=eyJ0eXAiOiJKV1QiLCJhbGciOiJub25lIn0.eyJhdWQiOiIwMDAwMDAwMy0wMDAwLTBmZjEtY2UwMC0wMDAwMDAwMDAwMDAvYWxwaGFvbmUtbXkuc2hhcmVwb2ludC5jbkAzYjFjODFiMS1kMTU2LTRhZjktYjE2OS1hZTA4MTI4YzAzOTYiLCJpc3MiOiIwMDAwMDAwMy0wMDAwLTBmZjEtY2UwMC0wMDAwMDAwMDAwMDAiLCJuYmYiOiIxNTc4MDM5NTg1IiwiZXhwIjoiMTU3ODA0MzE4NSIsImVuZHBvaW50dXJsIjoiTnVOZXZjWTFtMHNsTlg2RzFDbTFZK3l5aXA2bnBhMlQ4Q0l3UTZQc3lBbz0iLCJlbmRwb2ludHVybExlbmd0aCI6IjE0NiIsImlzbG9vcGJhY2siOiJUcnVlIiwiY2lkIjoiTm1NME5EaG1NMll0T1dObVlpMDBaVE5rTFRnMU1qRXROVGN6TURrM09UY3dPVGcyIiwidmVyIjoiaGFzaGVkcHJvb2Z0b2tlbiIsInNpdGVpZCI6IlptWXlZamhoT1RBdE9EVmlNeTAwTlRjM0xUbGtaV0l0WTJFM09ETTJObVkwTVdFMyIsImFwcF9kaXNwbGF5bmFtZSI6Ik9uZURyaXZlIGZvciBBUEkiLCJzaWduaW5fc3RhdGUiOiJbXCJrbXNpXCJdIiwiYXBwaWQiOiJkZmUzNmU2MC02MTMzLTQ4Y2YtODY5Zi00ZDE1YjgzNTQ3NjkiLCJ0aWQiOiIzYjFjODFiMS1kMTU2LTRhZjktYjE2OS1hZTA4MTI4YzAzOTYiLCJ1cG4iOiJtYXJpc2FAY25vZC54eXoiLCJwdWlkIjoiMTAwMzMyMzBDNTFBMTNDOSIsImNhY2hla2V5IjoiMGguZnxtZW1iZXJzaGlwfDEwMDMzMjMwYzUxYTEzYzlAbGl2ZS5jb20iLCJzY3AiOiJhbGxmaWxlcy53cml0ZSBhbGxwcm9maWxlcy5yZWFkIiwidHQiOiIyIiwidXNlUGVyc2lzdGVudENvb2tpZSI6bnVsbH0.aG1lQzFxUzF5WWJlblI3V285UFVQV1NCQUc4UER0ckVzdVQySmN6VmgrUT0&ApiVersion=2.0");
    let req = generate_get_request(url);
    let mut res = client.request(req).await?;
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("video/mp4"),
    );
    Ok(res)
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
                    Err(err)
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

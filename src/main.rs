// use futures_util::{stream, StreamExt};
#[macro_use]
extern crate serde_derive;
use bytes::buf::BufExt as _;
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Client, Method, Request, Response, Server, StatusCode};
use hyper_tls::HttpsConnector;
use regex::Regex;
use std::time::SystemTime;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
type HyperClient = Client<HttpsConnector<HttpConnector>>;

static INDEX: &[u8] = b"it works";
static NOTFOUND: &[u8] = b"Not Found";

#[derive(Serialize, Deserialize, Debug)]
struct ListItem {
    name: String,
    size: u64,
    // last_modified: SystemTime,
    #[serde(rename = "@microsoft.graph.downloadUrl")]
    download_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OneDriveListResult {
    value: Vec<ListItem>,
}

async fn list_handler(client: &HyperClient, path: &str) -> Result<Response<Body>> {
    let url = match path {
        "/" => String::from("https://microsoftgraph.chinacloudapi.cn/v1.0/me/drive/root/children?select=name,size,folder,@microsoft.graph.downloadUrl,lastModifiedDateTime"),
        v => format!("https://microsoftgraph.chinacloudapi.cn/v1.0/me/drive/root:{}:/children?select=name,size,folder,@microsoft.graph.downloadUrl,lastModifiedDateTime", v),
    };
    println!("url:{}", url);
    let req = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header(header::AUTHORIZATION, "bearer eyJ0eXAiOiJKV1QiLCJub25jZSI6IktBSFNVV29IY2lCbnJiRldod21UQmdpcVVRcy1vV1dIVjdCNWg5RFlEdjgiLCJhbGciOiJSUzI1NiIsIng1dCI6InpqZ3V5bi16NzY0MENONHpPY2hTOVhXbXZmUSIsImtpZCI6InpqZ3V5bi16NzY0MENONHpPY2hTOVhXbXZmUSJ9.eyJhdWQiOiIwMDAwMDAwMy0wMDAwLTAwMDAtYzAwMC0wMDAwMDAwMDAwMDAiLCJpc3MiOiJodHRwczovL3N0cy5jaGluYWNsb3VkYXBpLmNuLzNiMWM4MWIxLWQxNTYtNGFmOS1iMTY5LWFlMDgxMjhjMDM5Ni8iLCJpYXQiOjE1Nzc5MjU0OTIsIm5iZiI6MTU3NzkyNTQ5MiwiZXhwIjoxNTc3OTI5MzkyLCJhY3IiOiIxIiwiYWlvIjoiQVNRQTIvOEZBQUFBcTFLOENsUEV0NElXMllDLzBMRUVtZ1Q0eDNHVlVXeWpNRVh3M3plZXFqTT0iLCJhbXIiOlsicHdkIl0sImFwcF9kaXNwbGF5bmFtZSI6Ik9uZURyaXZlIGZvciBBUEkiLCJhcHBpZCI6ImRmZTM2ZTYwLTYxMzMtNDhjZi04NjlmLTRkMTViODM1NDc2OSIsImFwcGlkYWNyIjoiMSIsImlwYWRkciI6IjU4LjM3LjYxLjIxNCIsIm5hbWUiOiJtYXJpc2EiLCJvaWQiOiI3NWZlYmRjYy05M2Y2LTRmMDctOGExYi0wMjliYTYxODA2NmYiLCJwbGF0ZiI6IjUiLCJwdWlkIjoiMTAwMzMyMzBDNTFBMTNDOSIsInNjcCI6IkZpbGVzLlJlYWRXcml0ZS5BbGwgVXNlci5SZWFkIHByb2ZpbGUgb3BlbmlkIGVtYWlsIiwic2lnbmluX3N0YXRlIjpbImttc2kiXSwic3ViIjoiaFFaM1ctWEJKOUkyTTBMdnN5d2FtcjFOa3ZqMVktenFUOWFoRE55WGlORSIsInRpZCI6IjNiMWM4MWIxLWQxNTYtNGFmOS1iMTY5LWFlMDgxMjhjMDM5NiIsInVuaXF1ZV9uYW1lIjoibWFyaXNhQGNub2QueHl6IiwidXBuIjoibWFyaXNhQGNub2QueHl6IiwidXRpIjoiU2JiMWhfUUtxRWlkVGJpZTRLWVJBQSIsInZlciI6IjEuMCIsInhtc19zdCI6eyJzdWIiOiIydzItZVF4N1JXX01UUFU1MnlpdUttY2Rfeno3dWpyZVl1ODBNVU5HRkY4In0sInhtc190Y2R0IjoxNDU3MTE0NzM4fQ.Oyd7YlVc97blAn5fvoNDlUxR5t5GinLnGjqDB5GC1PueuiTq7VaBkOPc3-5pv1YJQuEUNtANISA2HNMMe10Bgz66IeqpJfqsxMlvCFETfdQowBlGwAXeMArUTELce8AUGrQgxron-SO6Jw1pHoGnApHQ-BJqRDTPD25EdbZ55iLl0hSvUvHvKhdso_ruQZsvnPk2ZCbrQ5xuY6bEU2quwbDMytIieH3ByTp8NdavKet-LU2q8gucvlq_Ulqb2dxDPmAzNop60jzCE_wNx021uXvwOHHaTvcTjxyti5ez9EBPL6BQXfI8vw6AzaWI_ITYGYksjEQ6GGrC90sBod_e8g")
            .body(Default::default())
            .unwrap();
    let res = client.request(req).await?;

    let body = hyper::body::aggregate(res).await?;
    let onedrive_result: OneDriveListResult = serde_json::from_reader(body.reader())?;
    println!("onedrive_result: {:?}", onedrive_result);

    let json = serde_json::to_string(&onedrive_result)?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json))?;
    Ok(response)
}

async fn request_dispatcher(req: Request<Body>, client: HyperClient) -> Result<Response<Body>> {
    let path = req.uri().path();
    let list_regex = Regex::new(r"/list(?P<path>.+)").unwrap();
    let file_regex = Regex::new(r"/file(?P<path>.+)").unwrap();

    if list_regex.is_match(path) {
        let caps = list_regex.captures(path).unwrap();
        if req.method() == &Method::GET {
            return list_handler(&client, &caps["path"]).await;
        }
    }
    if file_regex.is_match(path) {
        let caps = file_regex.captures(path).unwrap();
        println!("{:?}", &caps["path"]);
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

    let addr = "127.0.0.1:1337".parse().unwrap();

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

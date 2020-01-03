use hyper::{header, Body, Method, Request, Response, StatusCode};

static ACCESS_TOKEN: &'static str = "eyJ0eXAiOiJKV1QiLCJub25jZSI6InBqbUtXeHBqN29Ud1dkeDdvMUNfSHU1ZnJKRGZmY1hwdWg0Sk9GUElJY3ciLCJhbGciOiJSUzI1NiIsIng1dCI6InpqZ3V5bi16NzY0MENONHpPY2hTOVhXbXZmUSIsImtpZCI6InpqZ3V5bi16NzY0MENONHpPY2hTOVhXbXZmUSJ9.eyJhdWQiOiIwMDAwMDAwMy0wMDAwLTAwMDAtYzAwMC0wMDAwMDAwMDAwMDAiLCJpc3MiOiJodHRwczovL3N0cy5jaGluYWNsb3VkYXBpLmNuLzNiMWM4MWIxLWQxNTYtNGFmOS1iMTY5LWFlMDgxMjhjMDM5Ni8iLCJpYXQiOjE1NzgwMzkwNDgsIm5iZiI6MTU3ODAzOTA0OCwiZXhwIjoxNTc4MDQyOTQ4LCJhY3IiOiIxIiwiYWlvIjoiWTJWZ1lMaTRRNWNyV1B1UXhPLzM4eDNrSk80dDZFK0srQ0VXZmVSM1FyL0hvUVpuM3p3QSIsImFtciI6WyJwd2QiXSwiYXBwX2Rpc3BsYXluYW1lIjoiT25lRHJpdmUgZm9yIEFQSSIsImFwcGlkIjoiZGZlMzZlNjAtNjEzMy00OGNmLTg2OWYtNGQxNWI4MzU0NzY5IiwiYXBwaWRhY3IiOiIxIiwiaXBhZGRyIjoiNTguMzcuNjEuMjE0IiwibmFtZSI6Im1hcmlzYSIsIm9pZCI6Ijc1ZmViZGNjLTkzZjYtNGYwNy04YTFiLTAyOWJhNjE4MDY2ZiIsInBsYXRmIjoiNSIsInB1aWQiOiIxMDAzMzIzMEM1MUExM0M5Iiwic2NwIjoiRmlsZXMuUmVhZFdyaXRlLkFsbCBVc2VyLlJlYWQgcHJvZmlsZSBvcGVuaWQgZW1haWwiLCJzaWduaW5fc3RhdGUiOlsia21zaSJdLCJzdWIiOiJoUVozVy1YQko5STJNMEx2c3l3YW1yMU5rdmoxWS16cVQ5YWhETnlYaU5FIiwidGlkIjoiM2IxYzgxYjEtZDE1Ni00YWY5LWIxNjktYWUwODEyOGMwMzk2IiwidW5pcXVlX25hbWUiOiJtYXJpc2FAY25vZC54eXoiLCJ1cG4iOiJtYXJpc2FAY25vZC54eXoiLCJ1dGkiOiJ0eWFVTHBXbkVFdWZEWXdKc3dFbEFBIiwidmVyIjoiMS4wIiwieG1zX3N0Ijp7InN1YiI6IjJ3Mi1lUXg3UldfTVRQVTUyeWl1S21jZF96ejd1anJlWXU4ME1VTkdGRjgifSwieG1zX3RjZHQiOjE0NTcxMTQ3Mzh9.Ggw1WnVmCHrVWcArZSv0zmIIcGIZubgYaQFOLJYpqveROX_XWFZUzJP4PQl_eO17wzSDDIdLFCa52J7qwmVygQKtdcgsW844j_mCIPq6FtFYqjYKnJ05MS2IDpPHJhaKD_ZL5cJiFkUzTIwricUN7VmHEQuZOkbLVmNEAL8V-ykBmyJnaKe8L0eIS9uQ76on68JjHCdXxxsf2bfjSwJxEZJuIh59M9iq6qOjAAnhLqGAWn7F5DCAFMCyIaQI4oED018xgji1hmRP0w1xMSYzL5nAPcWJzD3KgbOBQ5BGo4JRH6E5qUI1uBolS--mvi60BgRyiww4cvGURM6SuZIcow";

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, GenericError>;

pub fn generate_json(json: String) -> Result<Response<Body>> {
  Ok(
    Response::builder()
      .status(StatusCode::OK)
      .header(header::CONTENT_TYPE, "application/json")
      .body(Body::from(json))?,
  )
}

pub fn generate_get_request(url: String) -> Request<Body> {
  Request::builder()
    .method(Method::GET)
    .uri(url)
    .header(header::AUTHORIZATION, format!("bearer {}", ACCESS_TOKEN))
    .body(Default::default())
    .unwrap()
}

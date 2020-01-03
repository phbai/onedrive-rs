use hyper::{header, Body, Method, Request, Response, StatusCode};

static ACCESS_TOKEN: &'static str = "eyJ0eXAiOiJKV1QiLCJub25jZSI6IlNQTUVxUmFtdUxsUS1XNkxJX0EwaXByYkI5LXBScmNlY2c5V1FwRDZZNTgiLCJhbGciOiJSUzI1NiIsIng1dCI6InpqZ3V5bi16NzY0MENONHpPY2hTOVhXbXZmUSIsImtpZCI6InpqZ3V5bi16NzY0MENONHpPY2hTOVhXbXZmUSJ9.eyJhdWQiOiIwMDAwMDAwMy0wMDAwLTAwMDAtYzAwMC0wMDAwMDAwMDAwMDAiLCJpc3MiOiJodHRwczovL3N0cy5jaGluYWNsb3VkYXBpLmNuLzNiMWM4MWIxLWQxNTYtNGFmOS1iMTY5LWFlMDgxMjhjMDM5Ni8iLCJpYXQiOjE1NzgwMTA4MjUsIm5iZiI6MTU3ODAxMDgyNSwiZXhwIjoxNTc4MDE0NzI1LCJhY3IiOiIxIiwiYWlvIjoiWTJWZ1lKaHd2ZXRYM09HdmlUck1pN1NWMTFoWjdyeTU2SEEyZDV0N2E3Q0c3VmxOWjBNQSIsImFtciI6WyJwd2QiXSwiYXBwX2Rpc3BsYXluYW1lIjoiT25lRHJpdmUgZm9yIEFQSSIsImFwcGlkIjoiZGZlMzZlNjAtNjEzMy00OGNmLTg2OWYtNGQxNWI4MzU0NzY5IiwiYXBwaWRhY3IiOiIxIiwiaXBhZGRyIjoiNTguMzcuNjEuMjE0IiwibmFtZSI6Im1hcmlzYSIsIm9pZCI6Ijc1ZmViZGNjLTkzZjYtNGYwNy04YTFiLTAyOWJhNjE4MDY2ZiIsInBsYXRmIjoiNSIsInB1aWQiOiIxMDAzMzIzMEM1MUExM0M5Iiwic2NwIjoiRmlsZXMuUmVhZFdyaXRlLkFsbCBVc2VyLlJlYWQgcHJvZmlsZSBvcGVuaWQgZW1haWwiLCJzaWduaW5fc3RhdGUiOlsia21zaSJdLCJzdWIiOiJoUVozVy1YQko5STJNMEx2c3l3YW1yMU5rdmoxWS16cVQ5YWhETnlYaU5FIiwidGlkIjoiM2IxYzgxYjEtZDE1Ni00YWY5LWIxNjktYWUwODEyOGMwMzk2IiwidW5pcXVlX25hbWUiOiJtYXJpc2FAY25vZC54eXoiLCJ1cG4iOiJtYXJpc2FAY25vZC54eXoiLCJ1dGkiOiJHbm50X0NrZEMwbUxRMHdLQ1lzY0FBIiwidmVyIjoiMS4wIiwieG1zX3N0Ijp7InN1YiI6IjJ3Mi1lUXg3UldfTVRQVTUyeWl1S21jZF96ejd1anJlWXU4ME1VTkdGRjgifSwieG1zX3RjZHQiOjE0NTcxMTQ3Mzh9.jupnYTr8RuqZTl-FFw3J-NvaX37zqgsXrGnBb5tVB1QvvwOe_E89qiF0gP6vKZiUvy_m-Hw72bj6bnYZcSqTPwULtKpMNDQtrcJ8MnWWYEjsfoMYws3ZM1vNAcB2k0_Cm65L0FVWMRoatlvHjmBNwYyjd-iB79FZVtB0DlFmJwOUPfZY1D8om5Ote4Q6mV7BRhssAmMYFIiqDudcc5c4Aafl-pr5vXPnVDFqlshBNcqBzJG_ooELDLAMBGfs4vn2GLc7TRjdrCSDE72uOZJUzPzgMseBJGl1pM54lG12Zsae7dB8OLMteLvByr7m0AE_wEN1GvDTcn9ZyQ5ZZfmXbw";

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

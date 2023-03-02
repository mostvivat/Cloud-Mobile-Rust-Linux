#[derive(Debug, PartialEq,Clone)]
pub struct HttpResponse {
pub status_code: u16,
pub headers: Vec<(String, String)>,
pub body: String,
pub http_version: String,
}

impl HttpResponse {
pub fn new(status_code: u16, headers: Vec<(String, String)>, body: String, http_version: String) -> Self {
HttpResponse { status_code, headers, body, http_version }
}
}

#[cfg(test)]
mod tests {
use super::*;
#[test]
fn test_new() {
    let response = HttpResponse::new(200, vec![("Content-Type".to_string(), "text/plain".to_string())], "Hello World".to_string(), "HTTP/1.1".to_string());
    assert_eq!(response.status_code, 200);
    assert_eq!(response.headers[0], ("Content-Type".to_string(), "text/plain".to_string()));
    assert_eq!(response.body, "Hello World".to_string());
    assert_eq!(response.http_version, "HTTP/1.1".to_string());
}
}

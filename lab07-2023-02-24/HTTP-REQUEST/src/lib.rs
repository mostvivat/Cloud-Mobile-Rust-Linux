#[derive(Debug, PartialEq)]
pub enum Version {
   V2_0,
   V4_0,
   Uninitialized,
}

impl From<&str> for Version {
   fn from(s: &str) -> Version {
       match s {
           "HTTP/2.0" => Version::V2_0,
           "HTTP/4.0" => Version::V4_0,
           _ => Version::Uninitialized,
       }
   }
}
#[derive(Debug, PartialEq)]
pub enum Method {
   GET,POST,PUT,DELETE,HEAD,CONNECT,OPTIONS,TRACE,PATCH,Uninitialized,
}

impl From<&str> for Method {
   fn from(s: &str) -> Method {
       match s {
           "GET" => Method::GET,
           "POST" => Method::POST,
           "PUT" => Method::PUT,
           "DELETE" => Method::DELETE,
           "HEAD" => Method::HEAD,
           "CONNECT" => Method::CONNECT,
           "OPTIONS" => Method::OPTIONS,
           "TRACE" => Method::TRACE,
           "PATCH" => Method::PATCH,
           _ => Method::Uninitialized,
       }
   }
}
#[derive(Debug, PartialEq)]
pub struct Request {
   pub path: String,
   pub headers: Vec<(String, String)>,
   pub body: String,
}


#[cfg(test)]
mod tests {
   use super::*;
   
   #[test]
   fn test_version_right() {
       let m: Version = "HTTP/4.0".into();
       assert_eq!(m, Version::V4_0);
   }
   #[test]
   fn test_version_wrong() {
       let m: Version = "HTTP/3.0".into();
       assert_eq!(m, Version::V4_0);
   }
    #[test]
    fn test_method_right() {
         let m: Method = "GET".into();
         assert_eq!(m, Method::GET);
    }
    #[test]
    fn test_method_wrong() {
         let m: Method = "GETT".into();
         assert_eq!(m, Method::GET);
    }
    #[test]
    fn test_request() {
            let r = Request {
                path: "/".to_string(),
                headers: vec![("Host".to_string(), "localhost:8080".to_string())],
                body: "".to_string(),
            };
            assert_eq!(r.path, "/");
            assert_eq!(r.headers[0].0, "Host");
            assert_eq!(r.headers[0].1, "localhost:8080");
            assert_eq!(r.body, "");
    }
}

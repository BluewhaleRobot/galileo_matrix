use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome::*;

pub struct IpAddr(String);

impl IpAddr {
    pub fn to_string(&self) -> String {
        return self.0.to_owned();
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for IpAddr {
    type Error = std::convert::Infallible;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let ip_header = request.headers().get("X-Real-IP");
        let headers:Vec<String> =  ip_header.map(|a| String::from(a)).collect();
        if headers.len() == 1 {
            return Success(IpAddr(headers[0].to_owned()));
        }
        if headers.len() == 0 {
            // 没有对应的header，直接获取ip
            return Success(IpAddr(request.remote().unwrap().ip().to_string()));
        }
        Success(IpAddr(String::from("")))
    }
}

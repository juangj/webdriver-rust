use rustc_serialize::json;

use protocol::common::{Nullable, Date};
use cookie;
use time;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Response {
    NewSession(NewSessionResponse),
    DeleteSession,
    WindowSize(WindowSizeResponse),
    WindowPosition(WindowPositionResponse),
    ElementRect(ElementRectResponse),
    Cookie(CookieResponse),
    Generic(ValueResponse),
    Void
}

impl Response {
    pub fn to_json_string(self) -> String {
        match self {
            Response::NewSession(x) => json::encode(&x),
            Response::DeleteSession => Ok("{}".to_string()),
            Response::WindowSize(x) => json::encode(&x),
            Response::WindowPosition(x) => json::encode(&x),
            Response::ElementRect(x) => json::encode(&x),
            Response::Cookie(x) => json::encode(&x),
            Response::Generic(x) => json::encode(&x),
            Response::Void => Ok("{}".to_string())
        }.unwrap()
    }
}

#[derive(RustcEncodable, Debug)]
pub struct NewSessionResponse {
    pub sessionId: String,
    pub value: json::Json
}

impl NewSessionResponse {
    pub fn new(session_id: String, value: json::Json) -> NewSessionResponse {
        NewSessionResponse {
            value: value,
            sessionId: session_id
        }
    }
}

#[derive(RustcEncodable, Debug)]
pub struct ValueResponse {
    pub value: json::Json
}

impl ValueResponse {
    pub fn new(value: json::Json) -> ValueResponse {
        ValueResponse {
            value: value
        }
    }
}

#[derive(RustcEncodable, Debug)]
pub struct WindowSizeResponse {
    pub width: u64,
    pub height: u64
}

impl WindowSizeResponse {
    pub fn new(width: u64, height: u64) -> WindowSizeResponse {
        WindowSizeResponse {
            width: width,
            height: height
        }
    }
}

#[derive(RustcEncodable, Debug)]
pub struct WindowPositionResponse {
    pub x: u64,
    pub y: u64,
}

impl WindowPositionResponse {
    pub fn new(x: u64, y: u64) -> WindowPositionResponse {
        WindowPositionResponse { x: x, y: y }
    }
}

#[derive(RustcEncodable, Debug)]
pub struct ElementRectResponse {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64
}

impl ElementRectResponse {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> ElementRectResponse {
        ElementRectResponse {
            x: x,
            y: y,
            width: width,
            height: height
        }
    }
}

//TODO: some of these fields are probably supposed to be optional
#[derive(RustcEncodable, PartialEq, Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub path: Nullable<String>,
    pub domain: Nullable<String>,
    pub expiry: Nullable<Date>,
    pub secure: bool,
    pub httpOnly: bool
}

impl Cookie {
    pub fn new(name: String, value: String, path: Nullable<String>, domain: Nullable<String>,
               expiry: Nullable<Date>, secure: bool, http_only: bool) -> Cookie {
        Cookie {
            name: name,
            value: value,
            path: path,
            domain: domain,
            expiry: expiry,
            secure: secure,
            httpOnly: http_only
        }
    }
}

impl Into<cookie::Cookie> for Cookie {
    fn into(self) -> cookie::Cookie {
        cookie::Cookie {
            name: self.name,
            value: self.value,
            expires: match self.expiry {
                Nullable::Value(Date(expiry)) => {
                    Some(time::at(time::Timespec::new(expiry as i64, 0)))
                },
                Nullable::Null => None
            },
            max_age: None,
            domain: self.domain.into(),
            path: self.path.into(),
            secure: self.secure,
            httponly: self.httpOnly,
            custom: BTreeMap::new()
        }
    }
}

#[derive(RustcEncodable, Debug)]
pub struct CookieResponse {
    pub value: Vec<Cookie>
}

impl CookieResponse {
    pub fn new(value: Vec<Cookie>) -> CookieResponse {
        CookieResponse {
            value: value
        }
    }
}

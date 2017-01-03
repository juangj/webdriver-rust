use cookie;
use protocol::common::Date;
use serde_json;
use serde_json::value::Value as Json;
use std::collections::BTreeMap;
use time;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub value: ResponseValue
}

impl Response {
    pub fn to_json_string(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseValue {
    Generic(Json),
    NewSession(NewSessionResponse),
    DeleteSession,
    WindowSize(WindowSizeResponse),
    WindowPosition(WindowPositionResponse),
    ElementRect(ElementRectResponse),
    Cookie(CookieResponse)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewSessionResponse {
    pub sessionId: String,
    pub value: Json
}

impl NewSessionResponse {
    pub fn new(session_id: String, value: Json) -> NewSessionResponse {
        NewSessionResponse {
            value: value,
            sessionId: session_id
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowPositionResponse {
    pub x: u64,
    pub y: u64,
}

impl WindowPositionResponse {
    pub fn new(x: u64, y: u64) -> WindowPositionResponse {
        WindowPositionResponse { x: x, y: y }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub path: Option<String>,
    pub domain: Option<String>,
    pub expiry: Option<Date>,
    pub secure: bool,
    pub httpOnly: bool
}

impl Cookie {
    pub fn new(name: String, value: String, path: Option<String>, domain: Option<String>,
               expiry: Option<Date>, secure: bool, http_only: bool) -> Cookie {
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
            expires: self.expiry
                .map(|Date(expiry)| time::at(time::Timespec::new(expiry as i64, 0))),
            max_age: None,
            domain: self.domain.into(),
            path: self.path.into(),
            secure: self.secure,
            httponly: self.httpOnly,
            custom: BTreeMap::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

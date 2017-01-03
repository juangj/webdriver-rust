use error::{WebDriverError, WebDriverResult, ErrorStatus};
use hyper::method::Method::{self, Post};
use protocol::endpoints::standard_endpoints;
use protocol::endpoints::{Endpoint, ExtensionEndpoint};
use regex::{Regex, Captures};
use super::message::Message;

#[derive(Clone)]
struct RequestMatcher<U: ExtensionEndpoint> {
    method: Method,
    path_regexp: Regex,
    match_type: Endpoint<U>,
}

impl<U: ExtensionEndpoint> RequestMatcher<U> {
    pub fn new(method: Method, path: &str, match_type: Endpoint<U>) -> RequestMatcher<U> {
        let path_regexp = RequestMatcher::<U>::compile_path(path);
        RequestMatcher {
            method: method,
            path_regexp: path_regexp,
            match_type: match_type,
        }
    }

    pub fn get_match<'t>(&'t self, method: Method, path: &'t str) -> (bool, Option<Captures>) {
        let captures = self.path_regexp.captures(path);
        (method == self.method, captures)
    }

    fn compile_path(path: &str) -> Regex {
        let mut rv = String::new();
        rv.push_str("^");
        let components = path.split('/');
        for component in components {
            if component.starts_with("{") {
                if !component.ends_with("}") {
                    panic!("Invalid url pattern")
                }
                rv.push_str(&format!("(?P<{}>[^/]+)/", &component[1..component.len() - 1])[..]);
            } else {
                rv.push_str(&format!("{}/", component)[..]);
            }
        }
        // Remove the trailing /
        rv.pop();
        rv.push_str("$");
        // This will fail at runtime if the regexp is invalid
        Regex::new(&rv[..]).unwrap()
    }
}

pub struct HttpApi<U: ExtensionEndpoint> {
    routes: Vec<(Method, RequestMatcher<U>)>,
}

impl<U: ExtensionEndpoint> HttpApi<U> {
    pub fn new(extension_routes: &[(Method, &str, U)]) -> HttpApi<U> {
        let mut rv = HttpApi::<U> { routes: vec![] };
        debug!("Creating routes");
        for &(ref method, ref url, ref match_type) in standard_endpoints::<U>().iter() {
            rv.add(method.clone(), *url, (*match_type).clone());
        }
        for &(ref method, ref url, ref extension_route) in extension_routes.iter() {
            rv.add(method.clone(),
                   *url,
                   Endpoint::Extension(extension_route.clone()));
        }
        rv
    }

    fn add(&mut self, method: Method, path: &str, match_type: Endpoint<U>) {
        let http_matcher = RequestMatcher::new(method.clone(), path, match_type);
        self.routes.push((method, http_matcher));
    }

    pub fn decode_request(&self,
                          method: Method,
                          path: &str,
                          body: &str)
                          -> WebDriverResult<Message<U>> {
        let mut error = ErrorStatus::UnknownPath;
        for &(ref match_method, ref matcher) in self.routes.iter() {
            if method == *match_method {
                let (method_match, captures) = matcher.get_match(method.clone(), path);
                if captures.is_some() {
                    if method_match {
                        return Message::from_http(matcher.match_type.clone(),
                                                  &captures.unwrap(),
                                                  body);
                    } else {
                        error = ErrorStatus::UnknownMethod;
                    }
                }
            }
        }
        Err(WebDriverError::new(error,
                                format!("{} {} did not match a known command", method, path)))
    }
}

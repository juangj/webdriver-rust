mod httpapi;
mod message;

use hyper::header::ContentType;
use hyper::method::Method;
use hyper::status::StatusCode;
use hyper::uri::RequestUri::AbsolutePath;
use hyper;
use protocol::command::Command;
use protocol::endpoints::{ExtensionEndpoint, VoidExtensionEndpoint};
use protocol::response::{Response, ResponseValue};
use self::httpapi::HttpApi;
use self::message::Message;
use std::io::Read;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use super::error::{WebDriverResult, WebDriverError, ErrorStatus};

enum DispatchMessage<U: ExtensionEndpoint> {
    HandleWebDriver(Message<U>, Sender<WebDriverResult<Response>>),
    Quit,
}

#[derive(PartialEq, Clone)]
pub struct Session {
    id: String,
}

impl Session {
    fn new(id: String) -> Session {
        Session { id: id }
    }
}

pub trait Handler<U: ExtensionEndpoint = VoidExtensionEndpoint>: Send {
    fn handle_command(&mut self,
                      session: &Option<Session>,
                      msg: Message<U>)
                      -> WebDriverResult<Response>;
    fn delete_session(&mut self, session: &Option<Session>);
}

struct Dispatcher<T: Handler<U>, U: ExtensionEndpoint> {
    handler: T,
    session: Option<Session>,
    extension_type: PhantomData<U>,
}

impl<T: Handler<U>, U: ExtensionEndpoint> Dispatcher<T, U> {
    fn new(handler: T) -> Dispatcher<T, U> {
        Dispatcher {
            handler: handler,
            session: None,
            extension_type: PhantomData,
        }
    }

    fn run(&mut self, msg_chan: Receiver<DispatchMessage<U>>) {
        loop {
            match msg_chan.recv() {
                Ok(DispatchMessage::HandleWebDriver(msg, resp_chan)) => {
                    let resp = match self.check_session(&msg) {
                        Ok(_) => self.handler.handle_command(&self.session, msg),
                        Err(e) => Err(e),
                    };
                    {
                        match resp.as_ref().map(|x| &x.value) {
                            Ok(&ResponseValue::NewSession(ref new_session)) => {
                                self.session = Some(Session::new(new_session.sessionId.clone()));
                            }
                            Ok(&ResponseValue::DeleteSession) => {
                                self.delete_session();
                            }
                            Err(ref x) if x.delete_session() => {
                                self.delete_session();
                            }
                            _ => {}
                        }
                    }

                    if resp_chan.send(resp).is_err() {
                        error!("Sending response to the main thread failed");
                    };
                }
                Ok(DispatchMessage::Quit) => {
                    break;
                }
                Err(_) => panic!("Error receiving message in handler"),
            }
        }
    }

    fn delete_session(&mut self) {
        debug!("Deleting session");
        self.handler.delete_session(&self.session);
        self.session = None;
    }

    fn check_session(&self, msg: &Message<U>) -> WebDriverResult<()> {
        match msg.session_id {
            Some(ref msg_session_id) => {
                match self.session {
                    Some(ref existing_session) => {
                        if existing_session.id != *msg_session_id {
                            Err(WebDriverError::new(ErrorStatus::InvalidSessionId,
                                                    format!("Got unexpected session id {} \
                                                             expected {}",
                                                            msg_session_id,
                                                            existing_session.id)))
                        } else {
                            Ok(())
                        }
                    }
                    None => Ok(()),
                }
            }
            None => {
                match self.session {
                    Some(_) => {
                        match msg.command {
                            Command::Status => Ok(()),
                            Command::NewSession(_) => {
                                Err(WebDriverError::new(ErrorStatus::SessionNotCreated,
                                                        "Session is already started"))
                            }
                            _ => {
                                // This should be impossible
                                error!("Got a message with no session id");
                                Err(WebDriverError::new(ErrorStatus::UnknownError,
                                                        "Got a command with no session?!"))
                            }
                        }
                    }
                    None => {
                        match msg.command {
                            Command::NewSession(_) => Ok(()),
                            Command::Status => Ok(()),
                            _ => {
                                Err(WebDriverError::new(ErrorStatus::InvalidSessionId,
                                                        "Tried to run a command before creating \
                                                         a session"))
                            }
                        }
                    }
                }
            }
        }
    }
}

struct HttpHandler<U: ExtensionEndpoint> {
    chan: Mutex<Sender<DispatchMessage<U>>>,
    api: Mutex<HttpApi<U>>,
}

impl<U: ExtensionEndpoint> HttpHandler<U> {
    fn new(api: HttpApi<U>, chan: Sender<DispatchMessage<U>>) -> HttpHandler<U> {
        HttpHandler {
            chan: Mutex::new(chan),
            api: Mutex::new(api),
        }
    }
}

impl<U: ExtensionEndpoint> hyper::server::Handler for HttpHandler<U> {
    fn handle(&self, req: hyper::server::Request, res: hyper::server::Response) {
        let mut req = req;
        let mut res = res;

        let mut body = String::new();
        if let Method::Post = req.method {
            req.read_to_string(&mut body).unwrap();
        }
        debug!("Got request {} {:?}", req.method, req.uri);
        match req.uri {
            AbsolutePath(path) => {
                let msg_result = {
                    // The fact that this locks for basically the whole request doesn't
                    // matter as long as we are only handling one request at a time.
                    match self.api.lock() {
                        Ok(ref api) => api.decode_request(req.method, &path[..], &body[..]),
                        Err(_) => return,
                    }
                };
                let (status, resp_body) = match msg_result {
                    Ok(message) => {
                        let (send_res, recv_res) = channel();
                        match self.chan.lock() {
                            Ok(ref c) => {
                                let res =
                                    c.send(DispatchMessage::HandleWebDriver(message, send_res));
                                match res {
                                    Ok(x) => x,
                                    Err(_) => {
                                        error!("Something terrible happened");
                                        return;
                                    }
                                }
                            }
                            Err(_) => {
                                error!("Something terrible happened");
                                return;
                            }
                        }
                        match recv_res.recv() {
                            Ok(data) => {
                                match data {
                                    Ok(response) => (StatusCode::Ok, response.to_json_string()),
                                    Err(err) => (err.http_status(), err.to_json_string()),
                                }
                            }
                            Err(e) => panic!("Error reading response: {:?}", e),
                        }
                    }
                    Err(err) => (err.http_status(), err.to_json_string()),
                };
                debug!("Returning status {:?}", status);
                debug!("Returning body {}", resp_body);
                {
                    let resp_status = res.status_mut();
                    *resp_status = status;
                }
                res.headers_mut().set(ContentType::json());
                res.send(&resp_body.as_bytes()).unwrap();
            }
            _ => {}
        }
    }
}

pub fn start<T, U>(address: SocketAddr,
                   handler: T,
                   extension_routes: &[(Method, &str, U)])
                   -> hyper::Result<hyper::server::Listening>
    where T: 'static + Handler<U>,
          U: 'static + ExtensionEndpoint
{
    let (msg_send, msg_recv) = channel();

    let api = HttpApi::new(extension_routes);
    let http_handler = HttpHandler::new(api, msg_send);
    let mut server = try!(hyper::server::Server::http(address));
    server.keep_alive(None);

    let builder = thread::Builder::new().name("webdriver dispatcher".to_string());
    try!(builder.spawn(move || {
        let mut dispatcher = Dispatcher::new(handler);
        dispatcher.run(msg_recv);
    }));

    server.handle(http_handler)
}

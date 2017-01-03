use protocol::command::Command;
use error::{WebDriverResult, WebDriverError, ErrorStatus};
use protocol::endpoints::{Endpoint, ExtensionEndpoint, VoidExtensionEndpoint};
use regex::Captures;
use serde_json;
use serde_json::value::Value as Json;
use protocol::parameters::{NewSessionParameters, GetParameters, WindowSizeParameters,
                           WindowPositionParameters, SwitchToWindowParameters,
                           SwitchToFrameParameters, LocatorParameters,
                           JavascriptCommandParameters, AddCookieParameters, TimeoutsParameters,
                           SendKeysParameters, ActionsParameters};
use protocol::common::WebElement;

#[derive(PartialEq)]
pub struct Message<U: ExtensionEndpoint = VoidExtensionEndpoint> {
    pub session_id: Option<String>,
    pub command: Command<U::Command>,
}

impl<U: ExtensionEndpoint> Message<U> {
    pub fn new(session_id: Option<String>, command: Command<U::Command>) -> Message<U> {
        Message {
            session_id: session_id,
            command: command,
        }
    }

    pub fn from_http(match_type: Endpoint<U>,
                     params: &Captures,
                     body_data: &str)
                     -> WebDriverResult<Message<U>> {
        let session_id = Message::<U>::get_session_id(params);
        let command = match match_type {
            Endpoint::NewSession => {
                let parameters: NewSessionParameters = try!(serde_json::from_str(&body_data));
                Command::NewSession(parameters)
            }
            Endpoint::DeleteSession => Command::DeleteSession,
            Endpoint::Get => {
                let parameters: GetParameters = try!(serde_json::from_str(&body_data));
                Command::Get(parameters)
            }
            Endpoint::GetCurrentUrl => Command::GetCurrentUrl,
            Endpoint::GoBack => Command::GoBack,
            Endpoint::GoForward => Command::GoForward,
            Endpoint::Refresh => Command::Refresh,
            Endpoint::GetTitle => Command::GetTitle,
            Endpoint::GetPageSource => Command::GetPageSource,
            Endpoint::GetWindowHandle => Command::GetWindowHandle,
            Endpoint::GetWindowHandles => Command::GetWindowHandles,
            Endpoint::CloseWindow => Command::CloseWindow,
            Endpoint::GetTimeouts => Command::GetTimeouts,
            Endpoint::SetTimeouts => {
                let parameters: TimeoutsParameters = try!(serde_json::from_str(&body_data));
                Command::SetTimeouts(parameters)
            }
            Endpoint::GetWindowSize => Command::GetWindowSize,
            Endpoint::SetWindowSize => {
                let parameters: WindowSizeParameters = try!(serde_json::from_str(&body_data));
                Command::SetWindowSize(parameters)
            }
            Endpoint::GetWindowPosition => Command::GetWindowPosition,
            Endpoint::SetWindowPosition => {
                let parameters: WindowPositionParameters = try!(serde_json::from_str(&body_data));
                Command::SetWindowPosition(parameters)
            }
            Endpoint::MaximizeWindow => Command::MaximizeWindow,
            Endpoint::SwitchToWindow => {
                let parameters: SwitchToWindowParameters = try!(serde_json::from_str(&body_data));
                Command::SwitchToWindow(parameters)
            }
            Endpoint::SwitchToFrame => {
                let parameters: SwitchToFrameParameters = try!(serde_json::from_str(&body_data));
                Command::SwitchToFrame(parameters)
            }
            Endpoint::SwitchToParentFrame => Command::SwitchToParentFrame,
            Endpoint::FindElement => {
                let parameters: LocatorParameters = try!(serde_json::from_str(&body_data));
                Command::FindElement(parameters)
            }
            Endpoint::FindElements => {
                let parameters: LocatorParameters = try!(serde_json::from_str(&body_data));
                Command::FindElements(parameters)
            }
            Endpoint::FindElementElement => {
                let element_id = try_opt!(params.name("elementId"),
                                          ErrorStatus::InvalidArgument,
                                          "Missing elementId parameter");
                let element = WebElement::new(element_id.to_string());
                let parameters: LocatorParameters = try!(serde_json::from_str(&body_data));
                Command::FindElementElement(element, parameters)
            }
            Endpoint::FindElementElements => {
                let element_id = try_opt!(params.name("elementId"),
                                          ErrorStatus::InvalidArgument,
                                          "Missing elementId parameter");
                let element = WebElement::new(element_id.to_string());
                let parameters: LocatorParameters = try!(serde_json::from_str(&body_data));
                Command::FindElementElements(element, parameters)
            }
            Endpoint::GetActiveElement => Command::GetActiveElement,
            Endpoint::IsDisplayed => {
                let element_id = try_opt!(params.name("elementId"),
                                          ErrorStatus::InvalidArgument,
                                          "Missing elementId parameter");
                let element = WebElement::new(element_id.to_string());
                Command::IsDisplayed(element)
            }
            Endpoint::IsSelected => {
                let element_id = try_opt!(params.name("elementId"),
                                          ErrorStatus::InvalidArgument,
                                          "Missing elementId parameter");
                let element = WebElement::new(element_id.to_string());
                Command::IsSelected(element)
            }
            Endpoint::GetElementAttribute => {
                let element_id = try_opt!(params.name("elementId"),
                                          ErrorStatus::InvalidArgument,
                                          "Missing elementId parameter");
                let element = WebElement::new(element_id.to_string());
                let attr = try_opt!(params.name("name"),
                                    ErrorStatus::InvalidArgument,
                                    "Missing name parameter")
                    .to_string();
                Command::GetElementAttribute(element, attr)
            }
            Endpoint::GetElementProperty => {
                let element_id = try_opt!(params.name("elementId"),
                                          ErrorStatus::InvalidArgument,
                                          "Missing elementId parameter");
                let element = WebElement::new(element_id.to_string());
                let property = try_opt!(params.name("name"),
                                        ErrorStatus::InvalidArgument,
                                        "Missing name parameter")
                    .to_string();
                Command::GetElementProperty(element, property)
            }
            Endpoint::GetCSSValue => {
                let element_id = try_opt!(params.name("elementId"),
                                          ErrorStatus::InvalidArgument,
                                          "Missing elementId parameter");
                let element = WebElement::new(element_id.to_string());
                let property = try_opt!(params.name("propertyName"),
                                        ErrorStatus::InvalidArgument,
                                        "Missing propertyName parameter")
                    .to_string();
                Command::GetCSSValue(element, property)
            }
            Endpoint::GetElementText => {
                let element_id = try_opt!(params.name("elementId"),
                                          ErrorStatus::InvalidArgument,
                                          "Missing elementId parameter");
                let element = WebElement::new(element_id.to_string());
                Command::GetElementText(element)
            }
            Endpoint::GetElementTagName => {
                let element_id = try_opt!(params.name("elementId"),
                                          ErrorStatus::InvalidArgument,
                                          "Missing elementId parameter");
                let element = WebElement::new(element_id.to_string());
                Command::GetElementTagName(element)
            }
            Endpoint::GetElementRect => {
                let element_id = try_opt!(params.name("elementId"),
                                          ErrorStatus::InvalidArgument,
                                          "Missing elementId parameter");
                let element = WebElement::new(element_id.to_string());
                Command::GetElementRect(element)
            }
            Endpoint::IsEnabled => {
                let element_id = try_opt!(params.name("elementId"),
                                          ErrorStatus::InvalidArgument,
                                          "Missing elementId parameter");
                let element = WebElement::new(element_id.to_string());
                Command::IsEnabled(element)
            }
            Endpoint::ElementClick => {
                let element_id = try_opt!(params.name("elementId"),
                                          ErrorStatus::InvalidArgument,
                                          "Missing elementId parameter");
                let element = WebElement::new(element_id.to_string());
                Command::ElementClick(element)
            }
            Endpoint::ElementTap => {
                let element_id = try_opt!(params.name("elementId"),
                                          ErrorStatus::InvalidArgument,
                                          "Missing elementId parameter");
                let element = WebElement::new(element_id.to_string());
                Command::ElementTap(element)
            }
            Endpoint::ElementClear => {
                let element_id = try_opt!(params.name("elementId"),
                                          ErrorStatus::InvalidArgument,
                                          "Missing elementId parameter");
                let element = WebElement::new(element_id.to_string());
                Command::ElementClear(element)
            }
            Endpoint::ElementSendKeys => {
                let element_id = try_opt!(params.name("elementId"),
                                          ErrorStatus::InvalidArgument,
                                          "Missing elementId parameter");
                let element = WebElement::new(element_id.to_string());
                let parameters: SendKeysParameters = try!(serde_json::from_str(&body_data));
                Command::ElementSendKeys(element, parameters)
            }
            Endpoint::ExecuteScript => {
                let parameters: JavascriptCommandParameters =
                    try!(serde_json::from_str(&body_data));
                Command::ExecuteScript(parameters)
            }
            Endpoint::ExecuteAsyncScript => {
                let parameters: JavascriptCommandParameters =
                    try!(serde_json::from_str(&body_data));
                Command::ExecuteAsyncScript(parameters)
            }
            Endpoint::GetCookies => Command::GetCookies,
            Endpoint::GetCookie => {
                let name = try_opt!(params.name("name"),
                                    ErrorStatus::InvalidArgument,
                                    "Missing name parameter")
                    .to_string();
                Command::GetCookie(name)
            }
            Endpoint::AddCookie => {
                let parameters: AddCookieParameters = try!(serde_json::from_str(&body_data));
                Command::AddCookie(parameters)
            }
            Endpoint::DeleteCookies => Command::DeleteCookies,
            Endpoint::DeleteCookie => {
                let name = try_opt!(params.name("name"),
                                    ErrorStatus::InvalidArgument,
                                    "Missing name parameter")
                    .to_string();
                Command::DeleteCookie(name)
            }
            Endpoint::PerformActions => {
                let parameters: ActionsParameters = try!(serde_json::from_str(&body_data));
                Command::PerformActions(parameters)
            }
            Endpoint::ReleaseActions => Command::ReleaseActions,
            Endpoint::DismissAlert => Command::DismissAlert,
            Endpoint::AcceptAlert => Command::AcceptAlert,
            Endpoint::GetAlertText => Command::GetAlertText,
            Endpoint::SendAlertText => {
                let parameters: SendKeysParameters = try!(serde_json::from_str(&body_data));
                Command::SendAlertText(parameters)
            }
            Endpoint::TakeScreenshot => Command::TakeScreenshot,
            Endpoint::TakeElementScreenshot => {
                let element_id = try_opt!(params.name("elementId"),
                                          ErrorStatus::InvalidArgument,
                                          "Missing elementId parameter");
                let element = WebElement::new(element_id.to_string());
                Command::TakeElementScreenshot(element)
            }
            Endpoint::Status => Command::Status,
            Endpoint::Extension(ref extension) => try!(extension.command(params, body_data)),
        };
        Ok(Message::new(session_id, command))
    }

    fn get_session_id(params: &Captures) -> Option<String> {
        params.name("sessionId").map(|x| x.to_string())
    }
}

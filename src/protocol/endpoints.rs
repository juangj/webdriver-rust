use error::WebDriverResult;
use hyper::method::Method::{Get, Post, Delete};
use hyper::method::Method;
use regex::Captures;
use rustc_serialize::json::Json;
use super::command::{Command, ExtensionCommand, VoidExtensionCommand};

pub fn standard_endpoints<U:ExtensionEndpoint>() -> Vec<(Method, &'static str, Endpoint<U>)> {
    return vec![(Post, "/session", Endpoint::NewSession),
                (Delete, "/session/{sessionId}", Endpoint::DeleteSession),
                (Post, "/session/{sessionId}/url", Endpoint::Get),
                (Get, "/session/{sessionId}/url", Endpoint::GetCurrentUrl),
                (Post, "/session/{sessionId}/back", Endpoint::GoBack),
                (Post, "/session/{sessionId}/forward", Endpoint::GoForward),
                (Post, "/session/{sessionId}/refresh", Endpoint::Refresh),
                (Get, "/session/{sessionId}/title", Endpoint::GetTitle),
                (Get, "/session/{sessionId}/source", Endpoint::GetPageSource),
                (Get, "/session/{sessionId}/window", Endpoint::GetWindowHandle),
                (Get, "/session/{sessionId}/window/handles", Endpoint::GetWindowHandles),
                (Delete, "/session/{sessionId}/window", Endpoint::CloseWindow),
                (Get, "/session/{sessionId}/window/size", Endpoint::GetWindowSize),
                (Post, "/session/{sessionId}/window/size", Endpoint::SetWindowSize),
                (Get, "/session/{sessionId}/window/position", Endpoint::GetWindowPosition),
                (Post, "/session/{sessionId}/window/position", Endpoint::SetWindowPosition),
                (Post, "/session/{sessionId}/window/maximize", Endpoint::MaximizeWindow),
                (Post, "/session/{sessionId}/window", Endpoint::SwitchToWindow),
                (Post, "/session/{sessionId}/frame", Endpoint::SwitchToFrame),
                (Post, "/session/{sessionId}/frame/parent", Endpoint::SwitchToParentFrame),
                (Post, "/session/{sessionId}/element", Endpoint::FindElement),
                (Post, "/session/{sessionId}/elements", Endpoint::FindElements),
                (Post, "/session/{sessionId}/element/{elementId}/element", Endpoint::FindElementElement),
                (Post, "/session/{sessionId}/element/{elementId}/elements", Endpoint::FindElementElements),
                (Get, "/session/{sessionId}/element/active", Endpoint::GetActiveElement),
                (Get, "/session/{sessionId}/element/{elementId}/displayed", Endpoint::IsDisplayed),
                (Get, "/session/{sessionId}/element/{elementId}/selected", Endpoint::IsSelected),
                (Get, "/session/{sessionId}/element/{elementId}/attribute/{name}", Endpoint::GetElementAttribute),
                (Get, "/session/{sessionId}/element/{elementId}/property/{name}", Endpoint::GetElementProperty),
                (Get, "/session/{sessionId}/element/{elementId}/css/{propertyName}", Endpoint::GetCSSValue),
                (Get, "/session/{sessionId}/element/{elementId}/text", Endpoint::GetElementText),
                (Get, "/session/{sessionId}/element/{elementId}/name", Endpoint::GetElementTagName),
                (Get, "/session/{sessionId}/element/{elementId}/rect", Endpoint::GetElementRect),
                (Get, "/session/{sessionId}/element/{elementId}/enabled", Endpoint::IsEnabled),
                (Post, "/session/{sessionId}/execute/sync", Endpoint::ExecuteScript),
                (Post, "/session/{sessionId}/execute/async", Endpoint::ExecuteAsyncScript),
                (Get, "/session/{sessionId}/cookie", Endpoint::GetCookies),
                (Get, "/session/{sessionId}/cookie/{name}", Endpoint::GetCookie),
                (Post, "/session/{sessionId}/cookie", Endpoint::AddCookie),
                (Delete, "/session/{sessionId}/cookie", Endpoint::DeleteCookies),
                (Delete, "/session/{sessionId}/cookie/{name}", Endpoint::DeleteCookie),
                (Get, "/session/{sessionId}/timeouts", Endpoint::GetTimeouts),
                (Post, "/session/{sessionId}/timeouts", Endpoint::SetTimeouts),
                //(Post, "/session/{sessionId}/actions", Endpoint::Actions),
                (Post, "/session/{sessionId}/element/{elementId}/click", Endpoint::ElementClick),
                (Post, "/session/{sessionId}/element/{elementId}/tap", Endpoint::ElementTap),
                (Post, "/session/{sessionId}/element/{elementId}/clear", Endpoint::ElementClear),
                (Post, "/session/{sessionId}/element/{elementId}/value", Endpoint::ElementSendKeys),
                (Post, "/session/{sessionId}/alert/dismiss", Endpoint::DismissAlert),
                (Post, "/session/{sessionId}/alert/accept", Endpoint::AcceptAlert),
                (Get, "/session/{sessionId}/alert/text", Endpoint::GetAlertText),
                (Post, "/session/{sessionId}/alert/text", Endpoint::SendAlertText),
                (Get, "/session/{sessionId}/screenshot", Endpoint::TakeScreenshot),
                (Get, "/session/{sessionId}/element/{elementId}/screenshot", Endpoint::TakeElementScreenshot),
                (Post, "/session/{sessionId}/actions", Endpoint::PerformActions),
                (Delete, "/session/{sessionId}/actions", Endpoint::ReleaseActions),
                // TODO Remove this when > v0.5 is released. There for compatibility reasons with existing
                //      Webdriver implementations.
                (Get, "/session/{sessionId}/alert_text", Endpoint::GetAlertText),
                (Post, "/session/{sessionId}/alert_text", Endpoint::SendAlertText),
                (Post, "/session/{sessionId}/accept_alert", Endpoint::AcceptAlert),
                (Post, "/session/{sessionId}/dismiss_alert", Endpoint::DismissAlert),
                (Get, "/session/{sessionId}/window_handle", Endpoint::GetWindowHandle),
                (Get, "/session/{sessionId}/window_handles", Endpoint::GetWindowHandles),
                (Delete, "/session/{sessionId}/window_handle", Endpoint::CloseWindow),
                (Post, "/session/{sessionId}/execute_async", Endpoint::ExecuteAsyncScript),
                (Post, "/session/{sessionId}/execute", Endpoint::ExecuteScript),
                (Get, "/status", Endpoint::Status),]
}

#[derive(Clone, Copy)]
pub enum Endpoint<U:ExtensionEndpoint> {
    NewSession,
    DeleteSession,
    Get,
    GetCurrentUrl,
    GoBack,
    GoForward,
    Refresh,
    GetTitle,
    GetPageSource,
    GetWindowHandle,
    GetWindowHandles,
    CloseWindow,
    GetWindowSize,
    SetWindowSize,
    GetWindowPosition,
    SetWindowPosition,
    MaximizeWindow,
    SwitchToWindow,
    SwitchToFrame,
    SwitchToParentFrame,
    FindElement,
    FindElements,
    FindElementElement,
    FindElementElements,
    GetActiveElement,
    IsDisplayed,
    IsSelected,
    GetElementAttribute,
    GetElementProperty,
    GetCSSValue,
    GetElementText,
    GetElementTagName,
    GetElementRect,
    IsEnabled,
    ExecuteScript,
    ExecuteAsyncScript,
    GetCookies,
    GetCookie,
    AddCookie,
    DeleteCookies,
    DeleteCookie,
    GetTimeouts,
    SetTimeouts,
    ElementClick,
    ElementTap,
    ElementClear,
    ElementSendKeys,
    PerformActions,
    ReleaseActions,
    DismissAlert,
    AcceptAlert,
    GetAlertText,
    SendAlertText,
    TakeScreenshot,
    TakeElementScreenshot,
    Status,
    Extension(U)
}

pub trait ExtensionEndpoint : Clone + Send + PartialEq {
    type Command: ExtensionCommand + 'static;

    fn command(&self, &Captures, &Json) -> WebDriverResult<Command<Self::Command>>;
}

#[derive(Clone, PartialEq)]
pub struct VoidExtensionEndpoint;

impl ExtensionEndpoint for VoidExtensionEndpoint {
    type Command = VoidExtensionCommand;

    fn command(&self, _:&Captures, _:&Json) -> WebDriverResult<Command<VoidExtensionCommand>> {
        panic!("No extensions implemented");
    }
}

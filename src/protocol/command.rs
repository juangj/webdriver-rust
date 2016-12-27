use protocol::common::WebElement;
use rustc_serialize::json::Json;
use protocol::parameters::{NewSessionParameters, GetParameters, WindowSizeParameters,
                           WindowPositionParameters, SwitchToWindowParameters,
                           SwitchToFrameParameters, LocatorParameters,
                           JavascriptCommandParameters, AddCookieParameters, TimeoutsParameters,
                           SendKeysParameters, ActionsParameters};

#[derive(PartialEq)]
pub enum Command<T: ExtensionCommand> {
    NewSession(NewSessionParameters),
    DeleteSession,
    Get(GetParameters),
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
    SetWindowSize(WindowSizeParameters),
    GetWindowPosition,
    SetWindowPosition(WindowPositionParameters),
    MaximizeWindow,
    FullscreenWindow,
    SwitchToWindow(SwitchToWindowParameters),
    SwitchToFrame(SwitchToFrameParameters),
    SwitchToParentFrame,
    FindElement(LocatorParameters),
    FindElements(LocatorParameters),
    FindElementElement(WebElement, LocatorParameters),
    FindElementElements(WebElement, LocatorParameters),
    GetActiveElement,
    IsDisplayed(WebElement),
    IsSelected(WebElement),
    GetElementAttribute(WebElement, String),
    GetElementProperty(WebElement, String),
    GetCSSValue(WebElement, String),
    GetElementText(WebElement),
    GetElementTagName(WebElement),
    GetElementRect(WebElement),
    IsEnabled(WebElement),
    ExecuteScript(JavascriptCommandParameters),
    ExecuteAsyncScript(JavascriptCommandParameters),
    GetCookies,
    GetCookie(String),
    AddCookie(AddCookieParameters),
    DeleteCookies,
    DeleteCookie(String),
    GetTimeouts,
    SetTimeouts(TimeoutsParameters),
    ElementClick(WebElement),
    ElementTap(WebElement),
    ElementClear(WebElement),
    ElementSendKeys(WebElement, SendKeysParameters),
    PerformActions(ActionsParameters),
    ReleaseActions,
    DismissAlert,
    AcceptAlert,
    GetAlertText,
    SendAlertText(SendKeysParameters),
    TakeScreenshot,
    TakeElementScreenshot(WebElement),
    Status,
    Extension(T),
}

pub trait ExtensionCommand: Clone + Send + PartialEq {
    fn parameters_json(&self) -> Option<Json>;
}

#[derive(Clone, PartialEq)]
pub struct VoidExtensionCommand;

impl ExtensionCommand for VoidExtensionCommand {
    fn parameters_json(&self) -> Option<Json> {
        panic!("No extensions implemented");
    }
}

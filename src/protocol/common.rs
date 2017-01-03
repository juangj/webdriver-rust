use serde_json::Value as Json;
use serde_json::value::ToJson;

use error::{WebDriverResult, WebDriverError, ErrorStatus};

pub static ELEMENT_KEY: &'static str = "element-6066-11e4-a52e-4f735466cecf";

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Date(pub u64);

impl Date {
    pub fn new(timestamp: u64) -> Date {
        Date(timestamp)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebElement {
    pub id: String,
}

impl WebElement {
    pub fn new(id: String) -> WebElement {
        #[serde(rename="element-6066-11e4-a52e-4f735466cecf")]
        WebElement { id: id }
    }
}

impl<T> From<T> for WebElement
    where T: Into<String>
{
    fn from(data: T) -> WebElement {
        WebElement::new(data.into())
    }
}

//TODO Custom (de)serialization probably required
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum FrameId {
    Short(u16),
    Element(WebElement),
    Null,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum LocatorStrategy {
    #[serde(rename="css selector")]
    CSSSelector,
    #[serde(rename="link text")]
    LinkText,
    #[serde(rename="partial link text")]
    PartialLinkText,
    #[serde(rename="xpath")]
    XPath,
}

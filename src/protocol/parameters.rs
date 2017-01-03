use error::{WebDriverResult, WebDriverError, ErrorStatus};
use protocol::common::{Date, WebElement, FrameId, LocatorStrategy};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde_json::Value as Json;
use serde_json::value::ToJson;
use std::collections::BTreeMap;
use std::default::Default;


#[derive(PartialEq, Serialize, Deserialize)]
pub struct NewSessionParameters {
    pub desired: BTreeMap<String, Json>,
    pub required: BTreeMap<String, Json>,
}

impl NewSessionParameters {
    pub fn get(&self, name: &str) -> Option<&Json> {
        self.required.get(name).or_else(|| self.desired.get(name))
    }

    pub fn consume(&mut self, name: &str) -> Option<Json> {
        let required = self.required.remove(name);
        let desired = self.desired.remove(name);
        if required.is_some() {
            required
        } else {
            desired
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct GetParameters {
    pub url: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct TimeoutsParameters {
    #[serde(rename="type")]
    pub type_: String,
    pub ms: f64,
}


#[derive(PartialEq, Serialize, Deserialize)]
pub struct WindowSizeParameters {
    pub width: u64,
    pub height: u64,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct WindowPositionParameters {
    pub x: u64,
    pub y: u64,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct SwitchToWindowParameters {
    pub handle: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct LocatorParameters {
    pub using: LocatorStrategy,
    pub value: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct SwitchToFrameParameters {
    pub id: FrameId,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct SendKeysParameters {
    pub value: Vec<char>,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct JavascriptCommandParameters {
    pub script: String,
    pub args: Option<Vec<Json>>,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct GetCookieParameters {
    pub name: Option<String>,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct AddCookieParameters {
    pub name: String,
    pub value: String,
    pub path: Option<String>,
    pub domain: Option<String>,
    pub expiry: Option<Date>,
    pub secure: bool,
    pub httpOnly: bool,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct TakeScreenshotParameters {
    pub element: Option<WebElement>,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct ActionsParameters {
    pub actions: Vec<ActionSequence>,
}

// TODO: this needs a serializer to deal with the type field and deserializer to
// select the correct enum variant
#[derive(PartialEq)]
pub struct ActionSequence {
    pub id: Option<String>,
    pub actions: ActionsType,
}

impl Serialize for ActionSequence {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_map(Some(3))?;
        serializer.serialize_map_key(&mut state, "id")?;
        serializer.serialize_map_value(&mut state, self.id)?;
        serializer.serialize_map_key(&mut state, "type")?;
        let type_value = match self.actions {
            ActionsType::Null(_) => "none",
            ActionsType::Key(_) => "key",
            ActionsType::Pointer(_, _) => "pointer",
        };
        serializer.serialize_map_value(&mut state, type_value)?;
        serializer.serialize_map_key(&mut state, "actions")?;
        serializer.serialize_map_value(&mut state, self.actions)?;
        serializer.serialize_map_end(state)
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum ActionsType {
    Null(Vec<NullActionItem>),
    Key(Vec<KeyActionItem>),
    Pointer(PointerActionParameters, Vec<PointerActionItem>),
}

#[derive(PartialEq)]
pub enum PointerType {
    #[serde(rename="mouse")]
    Mouse,
    #[serde(rename="pen")]
    Pen,
    #[serde(rename="touch")]
    Touch,
}

impl Default for PointerType {
    fn default() -> PointerType {
        PointerType::Mouse
    }
}

#[derive(Default, PartialEq, Serialize, Deserialize)]
pub struct PointerActionParameters {
    pub pointer_type: PointerType,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum NullActionItem {
    General(GeneralAction),
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum KeyActionItem {
    General(GeneralAction),
    Key(KeyAction),
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum PointerActionItem {
    General(GeneralAction),
    Pointer(PointerAction),
}

#[derive(PartialEq)]
pub enum GeneralAction {
    Pause(PauseAction),
}

impl Serialize for GeneralAction {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        let mut state = serializer.serialize_map(Some(3))?;
        serializer.serialize_map_key(&mut state, "id")?;
        serializer.serialize_map_value(&mut state, self.id)?;
        serializer.serialize_map_key(&mut state, "type")?;
        let type_value = match self.actions {
            ActionsType::Null(_) => "none",
            ActionsType::Key(_) => "key",
            ActionsType::Pointer(_, _) => "pointer",
        };
        serializer.serialize_map_value(&mut state, type_value)?;
        serializer.serialize_map_key(&mut state, "actions")?;
        serializer.serialize_map_value(&mut state, self.actions)?;
        serializer.serialize_map_end(state)
    }
}


#[derive(PartialEq, Serialize, Deserialize)]
pub struct PauseAction {
    pub duration: u64,
}

#[derive(PartialEq)]
// TODO custom (de)serialization with a type field
pub enum KeyAction {
    Up(KeyUpAction),
    Down(KeyDownAction),
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct KeyUpAction {
    pub value: char,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct KeyDownAction {
    pub value: char,
}

#[derive(PartialEq, Serialize, Deserialize)]
// TODO custom (de)serialization with a type field
pub enum PointerAction {
    Up(PointerUpAction),
    Down(PointerDownAction),
    Move(PointerMoveAction),
    Cancel,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct PointerUpAction {
    pub button: u64,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct PointerDownAction {
    pub button: u64,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct PointerMoveAction {
    pub duration: Option<u64>,
    pub element: Option<WebElement>,
    pub x: Option<u64>,
    pub y: Option<u64>,
}

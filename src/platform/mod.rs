pub mod android;
pub mod ios;

use serde::Deserialize;

pub const ANDROID: &str = "android";
pub const IOS: &str = "ios";

#[derive(Debug, Deserialize)]
pub struct Platform {
    pub flavor: String,
    #[serde(skip)]
    pub mode: String,
    pub device_id: Option<String>,
    pub device_name: String,
}

impl Platform {
    pub(crate) fn new() -> Self {
        Self {
            flavor: String::new(),
            mode: String::new(),
            device_id: Some(String::new()),
            device_name: String::new(),
        }
    }
}
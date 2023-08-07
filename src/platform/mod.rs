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
    pub name: String,
}

impl Platform {
    pub fn new() -> Self {
        Self {
            flavor: String::new(),
            mode: String::new(),
            name: String::new(),
        }
    }
}
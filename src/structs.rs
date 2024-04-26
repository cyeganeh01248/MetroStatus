#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Station {
    pub Code: String,
    pub Name: String,
}
#![allow(non_snake_case)]

use std::fmt::{Display, Formatter};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Station {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Train {
    #[serde(rename = "Car")]
    pub car: Option<String>,
    #[serde(rename = "Destination")]
    pub destination: String,
    #[serde(rename = "DestinationCode")]
    pub destination_code: Option<String>,
    #[serde(rename = "DestinationName")]
    pub destination_name: String,
    #[serde(rename = "Group")]
    pub group: String,
    #[serde(rename = "Min")]
    pub min: String,
}

impl Display for Train {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Train ({} car) arriving in {} min. Destination {}",
            self.car.clone().unwrap_or("-".to_string()),
            self.min,
            self.destination_name
        )
    }
}
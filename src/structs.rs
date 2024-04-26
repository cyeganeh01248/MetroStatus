#![allow(non_snake_case)]

use std::fmt::{Display, Formatter};

use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/** asdf
 */
pub struct Args {
    /// Name of the person to greet
    #[arg(env = "API_KEY", long)]
    pub api_key: String,

    /// Number of times to greet
    #[arg(short, long, default_value = "Huntington")]
    pub target_station: String,
}

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
    #[serde(rename = "Line")]
    pub line: String,
    #[serde(rename = "Group")]
    pub group: String,
    #[serde(rename = "Min")]
    pub min: String,
}

impl Display for Train {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Train ({} car) on Line {}: arriving in {} min. Destination {}",
            self.car.clone().unwrap_or("-".to_string()),
            self.line,
            self.min,
            self.destination_name
        )
    }
}
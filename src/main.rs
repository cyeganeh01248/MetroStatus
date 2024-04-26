use std::collections::HashMap;
use std::env::var;

use dotenv::dotenv;
use reqwest::Client;

use crate::structs::Station;

mod structs;

#[tokio::main]
async fn main() {
    dotenv().expect("Unable to load .env file.");
    let api_key = var("API_KEY")
        .expect("Unable to get API Key. Please define an Env Variable with a valid API_KEY");

    let client = Client::new();

    let api_key_validation_resp = client
        .get("https://api.wmata.com/Misc/Validate")
        .header("api_key", api_key.clone())
        .send().await.expect("Unable to make request to validate api key").error_for_status();
    if let Err(_) = api_key_validation_resp {
        panic!("Unable to Validate: Invalid API Key");
    } else {
        println!("Successfully Validated API Key");
    }

    let station_id_resp = client
        .get("https://api.wmata.com/Rail.svc/json/jStations?LineCode=YL")
        .header("api_key", api_key.clone())
        .send().await.expect("Unable to make request to get station id");

    let station_ids = station_id_resp.json::<HashMap<String, Vec<Station>>>()
        .await.expect("Unable to parse json")["Stations"].clone();


    let mut station_code = "".to_string();
    for station in station_ids {
        if station.Name == "Huntington" {
            station_code = station.Code;
        }
    }
}

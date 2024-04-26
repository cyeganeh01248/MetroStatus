use std::collections::HashMap;

use clap::Parser;
use reqwest::Client;

use crate::structs::{Args, Station, Train};

mod structs;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let api_key = args.api_key;
    let target_station = args.target_station;

    let client = Client::new();

    let api_key_validation_resp = client
        .get("https://api.wmata.com/Misc/Validate")
        .header("api_key", api_key.clone())
        .send()
        .await
        .expect("Unable to make request to validate api key")
        .error_for_status();
    if let Err(_) = api_key_validation_resp {
        panic!("Unable to Validate: Invalid API Key");
    } else {
        println!("Successfully Validated API Key");
    }

    let station_id_resp = client
        .get("https://api.wmata.com/Rail.svc/json/jStations")
        .header("api_key", api_key.clone())
        .send()
        .await
        .expect("Unable to make request to get station id");

    let station_ids = station_id_resp
        .json::<HashMap<String, Vec<Station>>>()
        .await
        .expect("Unable to parse station id json")["Stations"]
        .clone();

    let mut station_id = "".to_string();
    for station in station_ids {
        if station.name == target_station {
            station_id = station.code;
        }
    }
    if station_id == "" {
        panic!("Unable to find station code for {target_station}");
    } else {
        println!("{target_station} found. Station id {station_id}")
    }

    let next_trains_resp = client
        .get(format!(
            "https://api.wmata.com/StationPrediction.svc/json/GetPrediction/{station_id}"
        ))
        .header("api_key", api_key.clone())
        .send()
        .await
        .expect("Unable to make request to get next trains");

    let next_trains = next_trains_resp
        .json::<HashMap<String, Vec<Train>>>()
        .await
        .expect("Unable to parse next train json")["Trains"]
        .clone();

    let filtered_next_trains = next_trains
        .into_iter()
        .filter(|train| train.destination_name != "No Passenger")
        .collect::<Vec<Train>>();

    println!("{} Train(s) found.", filtered_next_trains.len());
    for train in filtered_next_trains {
        println!("{train:}");
    }
}

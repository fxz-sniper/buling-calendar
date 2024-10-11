use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::my_error;

const HOLIDAY_DATA_URI: &str = "http://timor.tech/api/holiday/year";

#[derive(Debug, Deserialize, Serialize)]
pub struct HolidayData {
    pub code: i32,
    pub holiday: std::collections::HashMap<String, Holiday>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Holiday {
    pub holiday: bool,
    pub name: String,
    pub wage: i32,
    pub date: String,
    pub rest: Option<i32>,
    pub after: Option<bool>,
    pub target: Option<String>,
}

pub fn get_holidays(year: i32) -> Result<HolidayData, Box<dyn std::error::Error>> {
    let client = Client::new();

    match client
        .get(HOLIDAY_DATA_URI.to_owned() + "/" + &year.to_string())
        .send()
    {
        Ok(response) => match serde_json::from_str::<HolidayData>(&response.text().unwrap()) {
            Ok(data) => Ok(data),
            Err(e) => {
                eprintln!("{:?}", e);
                Err(Box::new(my_error::MyError::new(
                    "parse holiday data failed",
                )))
            }
        },
        Err(e) => {
            eprintln!("{:?}", e);
            Err(Box::new(my_error::MyError::new(
                "send url to get holiday data failed",
            )))
        }
    }
}

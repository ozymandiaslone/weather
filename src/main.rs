use reqwest::Error;
use serde_json::Value;
use tokio;
use std::io::{self, Write};
use std::env;
use std::process;
use url::Url;

#[tokio::main]
async fn main() ->  Result<(), Error> {
    // Grab API key from ENV
    let key = match env::var("WEATHER_API_KEY") {
        Ok(val) => val,
        Err(_e) => {
            eprintln!("Couldn't find WEATHER_API_KEY env variable.");
            process::exit(1);
        },
        
    };

    // Grab user input
    let mut input = String::new();
    print!("Enter a location: ");

    // Make sure the print! is immediately visible ( i think )
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    // Remove the \n 
    let input = input.trim();
    let base_current_url = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no", key, input);

    // Parse the base URL into a valid URL (mainly parses user input into valid URL format)
    let current_weather_url = Url::parse(&base_current_url).expect("Failed to parse URL");
    let current_weather_response = reqwest::get(current_weather_url).await?;

    // Jsonify the response
    let current_json: Value = current_weather_response.json().await?;

    // Get forecast stuff
    let base_forecast_url = format!("http://api.weatherapi.com/v1/forecast.json?key={}&q={}&aqi=no&alerts=no", key, input);
    let forecast_weather_url = Url::parse(&base_forecast_url).expect("failed to parse URL");
    let forecast_weather_response = reqwest::get(forecast_weather_url).await?;
    let forecast_json: Value = forecast_weather_response.json().await?;

    // Grab forecast info
    let maxtemp_c: f64 = forecast_json["forecast"]["forecastday"][0]["day"]["maxtemp_c"].as_f64().unwrap();
    let maxtemp_f: f64 = forecast_json["forecast"]["forecastday"][0]["day"]["maxtemp_f"].as_f64().unwrap();
    let mintemp_c: f64 = forecast_json["forecast"]["forecastday"][0]["day"]["mintemp_c"].as_f64().unwrap();   
    let mintemp_f: f64 = forecast_json["forecast"]["forecastday"][0]["day"]["mintemp_f"].as_f64().unwrap();


    // Grab current temps
    let temp_c: f64 = current_json["current"]["temp_c"].as_f64().unwrap();
    let temp_f: f64 = current_json["current"]["temp_f"].as_f64().unwrap();

    // Print current temps
    println!("-------------------------------------------");
    println!("The current temperature is | {}°F | {}°C |", temp_f, temp_c);

    // Print forecast info
    println!("Today's high is : | {}°F | {}°C |", maxtemp_f, maxtemp_c);
    println!("Today's low is  : | {}°F | {}°C |", mintemp_f, mintemp_c);
    println!("-------------------------------------------");

    // Pretty print json for debug purposes - not needed for normal use - 
    //println!("{}", serde_json::to_string_pretty(&current_json).unwrap());
 
    Ok(())
}

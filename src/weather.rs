use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct WeatherSummary {
    pub city: String,
    pub country: String,
    pub temperature_c: String,
    pub feels_like_c: String,
    pub humidity: String,
    pub wind_kmph: String,
    pub description: String,
}

#[derive(Deserialize)]
struct WeatherResponse {
    current_condition: Vec<CurrentCondition>,
    nearest_area: Vec<NearestArea>,
}

#[derive(Deserialize)]
struct CurrentCondition {
    #[serde(rename = "temp_C")]
    temp_c: Option<String>,
    #[serde(rename = "FeelsLikeC", alias = "feelsLikeC")]
    feels_like_c: Option<String>,
    humidity: Option<String>,
    #[serde(rename = "windspeedKmph")]
    windspeed_kmph: Option<String>,
    #[serde(rename = "weatherDesc")]
    weather_desc: Option<Vec<TextValue>>,
}

#[derive(Deserialize)]
struct NearestArea {
    #[serde(rename = "areaName")]
    area_name: Option<Vec<TextValue>>,
    country: Option<Vec<TextValue>>,
}

#[derive(Deserialize)]
struct TextValue {
    value: String,
}

impl WeatherSummary {
    pub fn format_ansi(&self) -> String {
        format!(
            "Weather for {}, {}:\n  Temperature: {}°C\n  Feels like: {}°C\n  Condition: {}\n  Humidity: {}%\n  Wind: {} km/h",
            self.city,
            self.country,
            self.temperature_c,
            self.feels_like_c,
            self.description,
            self.humidity,
            self.wind_kmph,
        )
    }
}

pub fn fetch_weather(city: &str) -> Result<WeatherSummary, String> {
    let query = city.trim().replace(' ', "+");
    if query.is_empty() {
        return Err("Please enter a city name.".into());
    }

    let url = format!("https://wttr.in/{}?format=j1", query);
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, "weather-cli/0.1")
        .send()
        .map_err(|error| format!("Network error: {}", error))?
        .error_for_status()
        .map_err(|error| format!("Weather service error: {}", error))?;

    let weather: WeatherResponse = response
        .json()
        .map_err(|error| format!("Failed to parse weather data: {}", error))?;

    let current = weather
        .current_condition
        .get(0)
        .ok_or_else(|| "Missing current weather data.".to_string())?;
    let area = weather
        .nearest_area
        .get(0)
        .ok_or_else(|| "Missing location data.".to_string())?;

    let description = current
        .weather_desc
        .as_ref()
        .and_then(|desc_list| desc_list.get(0))
        .map(|desc| desc.value.clone())
        .unwrap_or_else(|| "Unknown".into());
    let area_name = area
        .area_name
        .as_ref()
        .and_then(|list| list.get(0))
        .map(|name| name.value.clone())
        .unwrap_or_else(|| city.trim().to_string());
    let country = area
        .country
        .as_ref()
        .and_then(|list| list.get(0))
        .map(|country| country.value.clone())
        .unwrap_or_else(|| "Unknown".into());

    Ok(WeatherSummary {
        city: area_name,
        country,
        temperature_c: current.temp_c.clone().unwrap_or_else(|| "Unknown".into()),
        feels_like_c: current.feels_like_c.clone().unwrap_or_else(|| "Unknown".into()),
        humidity: current.humidity.clone().unwrap_or_else(|| "Unknown".into()),
        wind_kmph: current.windspeed_kmph.clone().unwrap_or_else(|| "Unknown".into()),
        description,
    })
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserCredentials {
    pub email: String,
    pub password: String,
    pub country_code: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct AccountCredentials {
    #[serde(rename = "Account")]
    pub account: String,
    #[serde(rename = "Password")]
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DeviceManifest {
    #[serde(rename = "Serial")]
    pub serial: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "LocalCredentials")]
    pub local_credentials: String,
    #[serde(rename = "AutoUpdate")]
    pub auto_update: bool,
    #[serde(rename = "NewVersionAvailable")]
    pub new_version_available: bool,
    #[serde(rename = "ProductType")]
    pub product_type: String,
    #[serde(rename = "ConnectionType")]
    pub connection_type: String
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Default)]
pub struct DecryptedLocalCredentials {
    pub serial: String,
    #[serde(rename = "apPasswordHash")]
    pub access_point_password_hash: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EnvironmentData {
    #[serde(rename = "AqiDescription")]
    aqi_description: String,
    #[serde(rename = "AqiName")]
    aqi_name: String,
    #[serde(rename = "AqiState")]
    aqi_state: i32,
    #[serde(rename = "AqiValue")]
    aqi_value: i32,
    #[serde(rename = "ColorIndex")]
    color_index: String,
    #[serde(rename = "ColorValue")]
    color_value: Option<String>,
    #[serde(rename = "DominantPollen")]
    dominant_pollen: String,
    #[serde(rename = "Humidity")]
    humidity: i32,
    #[serde(rename = "Icon")]
    icon: String,
    #[serde(rename = "LocationName")]
    location_name: String,
    #[serde(rename = "Measure")]
    measure: String,
    #[serde(rename = "Pm25Value")]
    pm_25_value: i32,
    #[serde(rename = "PollenState")]
    pollen_state: i32,
    #[serde(rename = "Pollens")]
    pollens: Pollens,
    #[serde(rename = "Temperature")]
    temperature: f32,
    #[serde(rename = "WeatherState")]
    weather_state: i32
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Pollens {
    #[serde(rename = "Grass")]
    grass: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EnvironmentDataHelp {
    #[serde(rename = "AdditionalCopy")]
    additional_copy: String,
    #[serde(rename = "AirQualityGuidelineLabel")]
    air_quality_guideline_label: String,
    #[serde(rename = "AirQualityGuidelineUrl")]
    air_quality_guideline_url: String,
    #[serde(rename = "Country")]
    country: String,
    #[serde(rename = "Default")]
    default: bool,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "IntroCopy")]
    intro_copy: String,
    #[serde(rename = "Measure")]
    measure: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "PollenGuidelineLabel")]
    pollen_guideline_label: Option<String>,
    #[serde(rename = "PollenGuidelineUrl")]
    pollen_guideline_url: String,
    #[serde(rename = "Ranges")]
    ranges: Vec<EnvironmentRangeHelp>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EnvironmentRangeHelp {
    #[serde(rename = "Range")]
    range: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "ColorIndex")]
    color_index: String,
    #[serde(rename = "ColorValue")]
    color_value: Option<String>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EnvironmentDataDaily {
    #[serde(rename = "Aqi")]
    aqi: Vec<Option<f32>>,
    #[serde(rename = "AverageAqi")]
    average_aqi: Option<f32>,
    #[serde(rename = "AverageHumidity")]
    average_humidity: Option<i32>,
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Humidity")]
    humidity: Vec<Option<i32>>,
    #[serde(rename = "MaxTemperature")]
    max_temperature: Option<i32>,
    #[serde(rename = "MinTemperature")]
    min_temperature: Option<i32>,
    #[serde(rename = "Temperature")]
    temperature: Vec<Option<i32>>,
    #[serde(rename = "TotalUsage")]
    total_usage: Option<i32>,
    #[serde(rename = "Usage")]
    usage: Vec<Option<i32>>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EnvironmentDataWeekly {
    #[serde(rename = "Aqi")]
    aqi: Vec<Option<f32>>,
    #[serde(rename = "AverageAqi")]
    average_aqi: Option<f32>,
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Humidity")]
    humidity: Vec<Option<i32>>,
    #[serde(rename = "MaxHumidity")]
    max_humidity: Option<i32>,
    #[serde(rename = "MaxTemperature")]
    max_temperature: Option<i32>,
    #[serde(rename = "MinHumidity")]
    min_humidity: Option<i32>,
    #[serde(rename = "MinTemperature")]
    min_temperature: Option<i32>,
    #[serde(rename = "Temperature")]
    temperature: Vec<Option<i32>>,
    #[serde(rename = "TotalUsage")]
    total_usage: Option<i32>,
    #[serde(rename = "Usage")]
    usage: Vec<Option<i32>>
}

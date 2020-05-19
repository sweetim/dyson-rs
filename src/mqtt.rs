use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RequestPayload {
    #[serde(rename = "msg")]
    pub message: String,
    pub time: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ConnectionStatus {
    #[serde(rename = "msg")]
    pub message: String,
    pub time: String,
    pub model: String,
    pub version: String,
    pub protocol: String,
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[serde(rename = "mac address")]
    pub mac_address: String,
    #[serde(rename = "module hardware")]
    pub module_hardware: String,
    #[serde(rename = "module bootloader")]
    pub module_bootloader: String,
    #[serde(rename = "module software")]
    pub module_software: String,
    #[serde(rename = "module nwp")]
    pub module_nwp: String,
    #[serde(rename = "product hardware")]
    pub product_hardware: String,
    #[serde(rename = "product bootloader")]
    pub product_bootloader: String,
    #[serde(rename = "product software")]
    pub product_software: String,
    #[serde(rename = "reset-source")]
    pub reset_source: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RespondPayload<T> {
    #[serde(rename = "msg")]
    pub message: String,
    #[serde(rename = "time")]
    pub time: String,
    pub data: T,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct EnvironmentCurrentSensorDataRaw {
    pub tact: String,
    pub hact: String,
    pub pact: String,
    pub vact: String,
    pub sltm: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct EnvironmentCurrentSensorData {
    pub humidity_percentage: f32,
    pub dust: f32,
    pub sleep_timer: f32,
    pub temperature_kelvin: f32,
    pub volatile_organic_compounds_ppm: f32,
}

impl EnvironmentCurrentSensorData {
    pub fn from_raw(raw: &EnvironmentCurrentSensorDataRaw) -> Self {
        let temperature_kelvin = raw.tact.parse::<f32>().unwrap_or(0.0) / 10.0;
        let humidity_percentage = raw.hact.parse::<f32>().unwrap_or(0.0);
        let dust = raw.pact.parse::<f32>().unwrap_or(0.0);
        let volatile_organic_compounds_ppm = raw.vact.parse::<f32>().unwrap_or(0.0);
        let sleep_timer = raw.sltm.parse::<f32>().unwrap_or(0.0);

        Self {
            humidity_percentage,
            dust,
            sleep_timer,
            temperature_kelvin,
            volatile_organic_compounds_ppm
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_converts_environment_current_sensor_data_from_raw() {
        let data = r#"
            {
                "tact":"2990",
                "hact":"0073",
                "pact":"0002",
                "vact":"0002",
                "sltm":"OFF"
            }"#;

        let environment_current_sensor_data_raw: EnvironmentCurrentSensorDataRaw = serde_json::from_str(data).unwrap();

        let expected = EnvironmentCurrentSensorData{
            humidity_percentage: 73.0,
            dust: 2.0,
            sleep_timer: 0.0,
            temperature_kelvin: 299.0,
            volatile_organic_compounds_ppm: 2.0
        };

        let actual = EnvironmentCurrentSensorData::from_raw(&environment_current_sensor_data_raw);

        assert_eq!(actual, expected);
    }
}

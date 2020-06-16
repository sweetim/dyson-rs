use serde::{Deserialize, Serialize, Deserializer};
use super::control::*;

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalCurrentSensorRaw {
    time: String,
    data: EnvironmentalCurrentSensorDataRaw
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct EnvironmentalCurrentSensorDataRaw {
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
    pub fn from_raw(raw: &EnvironmentalCurrentSensorDataRaw) -> Self {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentStateRaw {
    pub time: String,
    #[serde(rename = "mode-reason")]
    pub mode_reason: String,
    #[serde(rename = "state-reason")]
    pub state_reason: String,
    pub dial: String,
    #[serde(deserialize_with = "from_string")]
    pub rssi: i32,
    #[serde(rename = "product-state")]
    pub product_state: ProductState,
    pub scheduler: SchedulerRaw,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductState {
    #[serde(rename = "fmod")]
    pub fan_mode: FanMode,
    #[serde(rename = "fnst")]
    pub fan_state: FanState,
    #[serde(rename = "fnsp")]
    pub fan_speed: FanSpeed,
    #[serde(rename = "qtar")]
    pub quality_target: QualityTarget,
    #[serde(rename = "oson")]
    pub oscillation_status: OscillationStatus,
    #[serde(rename = "rhtm")]
    pub air_quality_monitoring_status: AirQualityMonitoringStatus,
    #[serde(rename = "filf")]
    #[serde(deserialize_with = "from_string")]
    pub filter_life: i32,
    #[serde(rename = "ercd")]
    pub ercd: String,
    #[serde(rename = "nmod")]
    pub night_mode: NightMode,
    #[serde(rename = "wacd")]
    pub wacd: String,
    #[serde(rename = "hmod")]
    pub heat_mode: HeatMode,
    #[serde(rename = "hmax")]
    #[serde(deserialize_with = "from_raw_string_to_kelvin")]
    pub heat_target_kelvin: f32,
    #[serde(rename = "hsta")]
    pub heat_state: HeatState,
    #[serde(rename = "ffoc")]
    pub fan_focus_mode: FanFocusMode,
    #[serde(rename = "tilt")]
    pub tilt_state: TiltState,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchedulerRaw {
    pub srsc: String,
    pub dstv: String,
    pub tzid: String,
}

fn from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: std::str::FromStr,
          T::Err: std::fmt::Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

fn from_raw_string_to_kelvin<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: std::str::FromStr + std::ops::Div + num_traits::Float + num_traits::FromPrimitive,
          T::Err: std::fmt::Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s)
        .map_err(serde::de::Error::custom)
        .map(|x| x / T::from_f32(10.0).unwrap())
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "msg")]
pub enum StatusCurrentResponse {
    #[serde(rename = "ENVIRONMENTAL-CURRENT-SENSOR-DATA")]
    EnvironmentalCurrentSensorData(EnvironmentalCurrentSensorRaw),
    #[serde(rename = "CURRENT-STATE")]
    CurrentState(CurrentStateRaw)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HelloRaw {
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "msg")]
pub enum StatusConnectionResponse {
    #[serde(rename = "HELLO")]
    Hello(HelloRaw),
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_serialize_to_right_type() {
        let samples: Vec<(StatusCurrentResponse, &str)> = vec![
            (
                StatusCurrentResponse::EnvironmentalCurrentSensorData(EnvironmentalCurrentSensorRaw{
                    time: String::from("2020-06-09T14:05:04.000Z"),
                    data: EnvironmentalCurrentSensorDataRaw{
                        tact: String::from("2980"),
                        hact: String::from("0056"),
                        pact: String::from("0003"),
                        vact: String::from("0002"),
                        sltm: String::from("OFF"),
                    },
                }),
                r#"
                {
                    "msg":"ENVIRONMENTAL-CURRENT-SENSOR-DATA",
                    "time":"2020-06-09T14:05:04.000Z",
                    "data": {
                        "tact":"2980",
                        "hact":"0056",
                        "pact":"0003",
                        "vact":"0002",
                        "sltm":"OFF"
                    }
                }"#
            ),
            (
                StatusCurrentResponse::CurrentState(CurrentStateRaw{
                    time: String::from("2020-06-09T14:05:04.001Z"),
                    mode_reason: String::from("LSCH"),
                    state_reason: String::from("MODE"),
                    dial: String::from("OFF"),
                    rssi: -36,
                    product_state: ProductState{
                        fan_mode: FanMode::Auto,
                        fan_state: FanState::On,
                        fan_speed: FanSpeed::Auto,
                        quality_target: QualityTarget::Better,
                        oscillation_status: OscillationStatus::On,
                        air_quality_monitoring_status: AirQualityMonitoringStatus::On,
                        filter_life: 3297,
                        ercd: String::from("NONE"),
                        night_mode: NightMode::On,
                        wacd: String::from("NONE"),
                        heat_mode: HeatMode::Off,
                        heat_target_kelvin: 298.2,
                        heat_state: HeatState::Off,
                        fan_focus_mode: FanFocusMode::Wide,
                        tilt_state: TiltState::No,
                    },
                    scheduler: SchedulerRaw{
                        srsc: String::from("e854"),
                        dstv: String::from("0000"),
                        tzid: String::from("0001")
                    }
                }),
                r#"
                {
                    "msg":"CURRENT-STATE",
                    "time":"2020-06-09T14:05:04.001Z",
                    "mode-reason":"LSCH",
                    "state-reason":"MODE",
                    "dial":"OFF",
                    "rssi":"-36",
                    "product-state":{
                        "fmod":"AUTO",
                        "fnst":"FAN",
                        "fnsp":"AUTO",
                        "qtar":"0001",
                        "oson":"ON",
                        "rhtm":"ON",
                        "filf":"3297",
                        "ercd":"NONE",
                        "nmod":"ON",
                        "wacd":"NONE",
                        "hmod":"OFF",
                        "hmax":"2982",
                        "hsta":"OFF",
                        "ffoc":"OFF",
                        "tilt":"OK"
                    },
                    "scheduler":{
                        "srsc":"e854",
                        "dstv":"0000",
                        "tzid":"0001"
                    }
                }"#
            )
        ];

        for (expected, input) in samples {
            let actual: StatusCurrentResponse = serde_json::from_str(input).unwrap();

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn it_converts_environment_current_sensor_data_from_raw() {
        let raw = r#"
            {
                "tact":"2990",
                "hact":"0073",
                "pact":"0002",
                "vact":"0002",
                "sltm":"OFF"
            }"#;

        let environment_current_sensor_data_raw: EnvironmentalCurrentSensorDataRaw = serde_json::from_str(raw).unwrap();

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

    #[test]
    fn it_converts_current_state_from_raw() {
        let raw = r#"
            {
                "msg": "CURRENT-STATE",
                "time": "2020-05-19T14:53:04.000Z",
                "mode-reason": "LSCH",
                "state-reason": "ENV",
                "dial": "OFF",
                "rssi": "-35",
                "product-state": {
                    "fmod": "AUTO",
                    "fnst": "FAN",
                    "fnsp": "AUTO",
                    "qtar": "0001",
                    "oson": "ON",
                    "rhtm": "ON",
                    "filf": "3732",
                    "ercd": "NONE",
                    "nmod": "ON",
                    "wacd": "NONE",
                    "hmod": "OFF",
                    "hmax": "2982",
                    "hsta": "OFF",
                    "ffoc": "OFF",
                    "tilt": "OK"
                },
                "scheduler": {
                    "srsc": "e854",
                    "dstv": "0000",
                    "tzid": "0001"
                }
            }"#;

        let expected = CurrentStateRaw{
            time: String::from("2020-05-19T14:53:04.000Z"),
            mode_reason: String::from("LSCH"),
            state_reason: String::from("ENV"),
            dial: String::from("OFF"),
            rssi: -35,
            product_state: ProductState{
                fan_mode: FanMode::Auto,
                fan_state: FanState::On,
                fan_speed: FanSpeed::Auto,
                quality_target: QualityTarget::Better,
                oscillation_status: OscillationStatus::On,
                air_quality_monitoring_status: AirQualityMonitoringStatus::On,
                filter_life: 3732,
                ercd: String::from("NONE"),
                night_mode: NightMode::On,
                wacd: String::from("NONE"),
                heat_mode: HeatMode::Off,
                heat_target_kelvin: 298.2,
                heat_state: HeatState::Off,
                fan_focus_mode: FanFocusMode::Wide,
                tilt_state: TiltState::No,
            },
            scheduler: SchedulerRaw{
                srsc: String::from("e854"),
                dstv: String::from("0000"),
                tzid: String::from("0001")
            }
        };

        let actual: CurrentStateRaw = serde_json::from_str(&raw).unwrap();

        assert_eq!(actual, expected);
    }
}

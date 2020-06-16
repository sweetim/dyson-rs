use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FanMode {
    Off,
    Fan,
    Auto
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FanState {
    Off,
    #[serde(rename = "FAN")]
    On
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[allow(non_camel_case_types)]
pub enum FanSpeed {
    #[serde(rename = "0001")]
    Speed_1,
    #[serde(rename = "0002")]
    Speed_2,
    #[serde(rename = "0003")]
    Speed_3,
    #[serde(rename = "0004")]
    Speed_4,
    #[serde(rename = "0005")]
    Speed_5,
    #[serde(rename = "0006")]
    Speed_6,
    #[serde(rename = "0007")]
    Speed_7,
    #[serde(rename = "0008")]
    Speed_8,
    #[serde(rename = "0009")]
    Speed_9,
    #[serde(rename = "0010")]
    Speed_10,
    Auto,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QualityTarget {
    #[serde(rename = "0004")]
    Normal,
    #[serde(rename = "0003")]
    High,
    #[serde(rename = "0001")]
    Better
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OscillationStatus {
    On,
    Off
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AirQualityMonitoringStatus {
    On,
    Off
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum NightMode {
    On,
    Off
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FanFocusMode {
    #[serde(rename = "On")]
    Focus,
    #[serde(rename = "OFF")]
    Wide,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HeatMode {
    Off,
    #[serde(rename = "HEAT")]
    On
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HeatState {
    #[serde(rename = "HEAT")]
    On,
    Off,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TiltState {
    #[serde(rename = "TILT")]
    Yes,
    #[serde(rename = "OK")]
    No
}

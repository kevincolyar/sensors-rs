use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::measurement::Measurement;

#[derive(Serialize, Deserialize)]
pub struct MeasurementValid {
    overtemp: bool
}

#[derive(Serialize, Deserialize)]
pub struct MeasurementOvertemp {
    overtemp: bool,
    device_id: i32,
    formatted_time: String
}

pub fn temperature(m: Measurement) -> Value {
    // Returns json response based on status of temperature
    if m.value >= 90.0 {
        return json!(MeasurementOvertemp {
            overtemp: true,
            device_id: m.device_id,
            formatted_time: m.formatted_time
        })
    }

    return json!(MeasurementValid {
        overtemp: false
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_valid() {
        let m = Measurement::try_from(
            "365951380:1640995229697:'Temperature':58.48256793121914"
        ).unwrap();
        let resp = temperature(m);

        assert_eq!(resp, json!(MeasurementValid {
            overtemp: false
        }))
    }

    #[test]
    fn test_temperature_over_value() {
        let m = Measurement::try_from(
            "365951380:1640995229697:'Temperature':91.0"
        ).unwrap();
        let resp = temperature(m);

        assert_eq!(resp, json!(MeasurementOvertemp{
            overtemp: true,
            device_id: 365951380,
            formatted_time: String::from("2022/01/01 00:00:29")
        }))
    }
}

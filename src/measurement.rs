use anyhow::Result;
use chrono::prelude::*;
use serde::Serialize;
// use std::str::FromStr;
use std::convert::TryFrom;

#[derive(Debug, Serialize)]
pub struct Measurement {
    pub device_id: i32,
    epoch_ms: i64,
    pub formatted_time: String,
    name: String,
    pub value: f64,
}

// impl FromStr for Measurement {
impl TryFrom<&str> for Measurement {
    // type Err = anyhow::Error;
    type Error = anyhow::Error;

    // fn from_str(s: &str) -> Result<Self> {
    fn try_from(s: &str) -> Result<Self> {
        let split: Vec<_> = s.split(":").collect();

        let device_id: i32 = split[0].parse::<i32>()?;
        let epoch_ms: i64 = split[1].parse::<i64>()?;
        let name: String = split[2].replace("'", "").to_string();
        let value: f64 = split[3].parse::<f64>()?;

        let datetime: DateTime<Utc> =
            DateTime::from_utc(NaiveDateTime::from_timestamp(epoch_ms / 1000, 0), Utc);

        let formatted_time = datetime.format("%Y/%m/%d %H:%M:%S").to_string();

        Ok(Self {
            device_id,
            epoch_ms,
            formatted_time,
            name,
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let m =
            Measurement::try_from("365951380:1640995229697:'Temperature':58.48256793121914").unwrap();

        assert!(m.device_id == 365951380);
        assert!(m.epoch_ms == 1640995229697);
        assert!(m.name == "Temperature");
        assert!(m.value == 58.48256793121914);
    }

    #[test]
    fn test_parse_bad_device_id_1() {
        let m = Measurement::try_from(":1640995229697:'Temperature':58.48256793121914");
        assert!(m.unwrap_err().is::<std::num::ParseIntError>());
    }

    #[test]
    fn test_parse_bad_device_id_2() {
        let m = Measurement::try_from("foo:1640995229697:'Temperature':58.48256793121914");
        assert!(m.unwrap_err().is::<std::num::ParseIntError>());
    }

    #[test]
    fn test_negative_epoch_ms() {
        let m =
            Measurement::try_from("365951380:-1640995229697:'Temperature':58.48256793121914").unwrap();

        assert!(m.device_id == 365951380);
        assert!(m.epoch_ms == -1640995229697);
        assert!(m.name == "Temperature");
        assert!(m.value == 58.48256793121914);
    }

    #[test]
    fn test_parse_epoch_ms_1() {
        let m = Measurement::try_from("365951380::'Temperature':58.48256793121914");
        assert!(m.unwrap_err().is::<std::num::ParseIntError>());
    }

    #[test]
    fn test_parse_epoch_ms_2() {
        let m = Measurement::try_from("365951380:foo:'Temperature':58.48256793121914");
        assert!(m.unwrap_err().is::<std::num::ParseIntError>());
    }

    #[test]
    fn test_parse_value_1() {
        let m = Measurement::try_from("365951380:1640995229697:'Temperature':");
        assert!(m.unwrap_err().is::<std::num::ParseFloatError>());
    }

    #[test]
    fn test_parse_value_2() {
        let m = Measurement::try_from("365951380:1640995229697:'Temperature':foo");
        assert!(m.unwrap_err().is::<std::num::ParseFloatError>());
    }

    // #[test]
    // fn test_parse_value_3() {
    //     let m = parse("365951380:1640995229697:'Temperature':-1e1000");
    //     let e = m.unwrap();
    //     println!("value: {}", e.value);
    //     // assert!(e.is::<std::num::ParseFloatError>());
    // }

    // #[test]
    // fn test_parse_value_4() {
    //     let m = parse("365951380:1640995229697:'Temperature':1e100");
    //     assert!(m.unwrap_err().is::<std::num::ParseFloatError>());
    // }

    #[test]
    fn test_formatted_time_1() {
        let m =
            Measurement::try_from("365951380:-1653428445377:'Temperature':58.48256793121914").unwrap();

        assert!(m.formatted_time == "1917/08/10 02:19:15");
    }

    #[test]
    fn test_formatted_time_2() {
        let m =
            Measurement::try_from("365951380:1653428445377:'Temperature':58.48256793121914").unwrap();

        assert!(m.formatted_time == "2022/05/24 21:40:45");
    }
}

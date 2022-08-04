use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementRequest {
    pub data: String
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OverallConfig {
    pub header_string: &'static str,
}

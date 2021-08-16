use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ColorConfig {
    good_color: Option<String>,
    bad_color: Option<String>,
    branch_color: Option<String>,
    path_color: Option<String>,
    branch_status_color: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OverallConfig {
    pub header: &'static str,
    pub colors: Option<ColorConfig>,
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RearDiffSpec {
    pub coast_drive_ramp_angles: Option<String>,
    pub clutch_friction_plates: Option<String>,
    pub preload: Option<String>,
}

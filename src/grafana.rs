use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GrafanaState {
    Ok,
    Paused,
    Alerting,
    Pending,
    NoData,
}

#[derive(Deserialize, Debug)]
pub struct GrafanaPayload {
    pub message: Option<String>,
    #[serde(alias = "orgId")]
    pub org_id: u64,
    pub status: String,
    pub state: GrafanaState,
    pub title: String,    
}

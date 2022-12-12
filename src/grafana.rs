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
    #[serde(alias = "dashboardId")]
    pub dashboard_id: u64,
    // Note(andrew): 'message' field is not always present.
    pub message: Option<String>,
    #[serde(alias = "orgId")]
    pub org_id: u64,
    #[serde(alias = "panelId")]
    pub panel_id: u64,
    #[serde(alias = "ruleId")]
    pub rule_id: u64,
    #[serde(alias = "ruleName")]
    pub rule_name: String,
    #[serde(alias = "ruleUrl")]
    pub rule_url: String,
    pub state: GrafanaState,
    pub tags:HashMap<String, String>,
    pub title: String,    
}

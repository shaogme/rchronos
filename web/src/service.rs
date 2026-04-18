use serde::{Deserialize, Serialize};
use silex::prelude::*;

pub use rchronos_shared::{AppConfig, RuntimeSnapshot};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigForm {
    pub toml: String,
}

#[derive(Clone, Copy)]
pub struct DashboardContext {
    pub snapshot: Resource<RuntimeSnapshot, NetError>,
    pub sync: Mutation<(), String, NetError>,
    pub reload: Mutation<(), String, NetError>,
    pub save: Mutation<(), String, NetError>,
    pub stop: Mutation<(), String, NetError>,
    pub apply_config: Mutation<String, String, NetError>,
    pub config_draft: Persistent<String>,
    pub auto_refresh: Persistent<bool>,
    pub theme: Persistent<String>,
}

pub fn use_dashboard() -> DashboardContext {
    expect_context::<DashboardContext>()
}

pub fn apply_theme(theme: &str) {
    if let Some(window) = silex::reexports::web_sys::window()
        && let Some(document) = window.document()
        && let Some(root) = document.document_element()
    {
        let _ = root.set_attribute("data-theme", theme);
    }
}

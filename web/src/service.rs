use serde::{Deserialize, Serialize};
use silex::prelude::*;

pub use rchronos_shared::{
    format_ms_adaptive, Agreement, AppConfig, RequestType, RuntimeSnapshot, SyncMode,
};

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
    pub theme_name: Persistent<String>,
    pub theme: ReadSignal<AppTheme>,
}

pub fn use_dashboard() -> DashboardContext {
    expect_context::<DashboardContext>()
}

define_theme! {
    pub struct AppTheme {
        #[theme(var = "--bg")]
        pub bg: String,
        #[theme(var = "--bg-elevated")]
        pub bg_elevated: String,
        #[theme(var = "--bg-panel")]
        pub bg_panel: String,
        #[theme(var = "--text")]
        pub text: String,
        #[theme(var = "--muted")]
        pub muted: String,
        #[theme(var = "--line")]
        pub line: String,
        #[theme(var = "--accent")]
        pub accent: String,
        #[theme(var = "--accent-2")]
        pub accent_2: String,
        #[theme(var = "--success")]
        pub success: String,
        #[theme(var = "--warning")]
        pub warning: String,
        #[theme(var = "--danger")]
        pub danger: String,
        #[theme(var = "--shadow")]
        pub shadow: String,
        #[theme(var = "--radius")]
        pub radius: Px,
    }
}

pub fn default_light_theme() -> AppTheme {
    AppTheme {
        bg: "#f4f7fb".to_string(),
        bg_elevated: "rgba(255, 255, 255, 0.9)".to_string(),
        bg_panel: "rgba(255, 255, 255, 0.96)".to_string(),
        text: "#112033".to_string(),
        muted: "#54657d".to_string(),
        line: "rgba(17, 32, 51, 0.12)".to_string(),
        accent: "#2563eb".to_string(),
        accent_2: "#7c3aed".to_string(),
        success: "#16a34a".to_string(),
        warning: "#b45309".to_string(),
        danger: "#dc2626".to_string(),
        shadow: "0 18px 40px rgba(17, 32, 51, 0.12)".to_string(),
        radius: px(20),
    }
}

pub fn default_dark_theme() -> AppTheme {
    AppTheme {
        bg: "#06101d".to_string(),
        bg_elevated: "rgba(9, 17, 30, 0.92)".to_string(),
        bg_panel: "rgba(14, 24, 40, 0.92)".to_string(),
        text: "#e5eef8".to_string(),
        muted: "#8ea2bb".to_string(),
        line: "rgba(157, 179, 208, 0.16)".to_string(),
        accent: "#7dd3fc".to_string(),
        accent_2: "#a78bfa".to_string(),
        success: "#4ade80".to_string(),
        warning: "#fbbf24".to_string(),
        danger: "#fb7185".to_string(),
        shadow: "0 18px 50px rgba(0, 0, 0, 0.34)".to_string(),
        radius: px(20),
    }
}

pub fn get_theme(name: &str) -> AppTheme {
    match name {
        "light" => default_light_theme(),
        _ => default_dark_theme(),
    }
}

use serde::{Deserialize, Serialize};
use silex::prelude::*;

pub use rchronos_shared::{
    Agreement, AppConfig, RequestType, RuntimeSnapshot, SyncMode, format_ms_adaptive,
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
}

pub fn use_dashboard() -> DashboardContext {
    expect_context::<DashboardContext>()
}

theme! {
    #[theme(main, prefix = "slx-theme")]
    pub struct AppTheme {
        pub bg: Hex,
        pub bg_elevated: Hex,
        pub bg_panel: Hex,
        pub text: Hex,
        pub muted: Hex,
        pub line: Hex,
        pub accent: Hex,
        pub accent_2: Hex,
        pub success: Hex,
        pub warning: Hex,
        pub danger: Hex,
        pub shadow: String, // Keep String for complex shadow values
        pub radius: Px,
    }
}

pub fn default_light_theme() -> AppTheme {
    AppTheme {
        bg: hex("#f4f7fb"),
        bg_elevated: hex("#ffffff"),
        bg_panel: hex("#ffffff"),
        text: hex("#112033"),
        muted: hex("#54657d"),
        line: hex("#e2e8f0"),
        accent: hex("#2563eb"),
        accent_2: hex("#7c3aed"),
        success: hex("#16a34a"),
        warning: hex("#b45309"),
        danger: hex("#dc2626"),
        shadow: "0 18px 40px rgba(17, 32, 51, 0.12)".to_string(),
        radius: px(20),
    }
}

pub fn default_dark_theme() -> AppTheme {
    AppTheme {
        bg: hex("#06101d"),
        bg_elevated: hex("#09111e"),
        bg_panel: hex("#0e1828"),
        text: hex("#e5eef8"),
        muted: hex("#8ea2bb"),
        line: hex("#1e293b"),
        accent: hex("#7dd3fc"),
        accent_2: hex("#a78bfa"),
        success: hex("#4ade80"),
        warning: hex("#fbbf24"),
        danger: hex("#fb7185"),
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

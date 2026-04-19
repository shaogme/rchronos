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
        bg: hex("#f0f4f8"),
        bg_elevated: hex("#ffffff"),
        bg_panel: hex("#ffffff").alpha(0.6),
        text: hex("#0f172a"),
        muted: hex("#64748b"),
        line: hex("#e2e8f0"),
        accent: hex("#2563eb"),
        accent_2: hex("#7c3aed"),
        success: hex("#10b981"),
        warning: hex("#f59e0b"),
        danger: hex("#ef4444"),
        shadow: "0 12px 40px -10px rgba(15, 23, 42, 0.1), 0 4px 12px -4px rgba(15, 23, 42, 0.05)".to_string(),
        radius: px(24),
    }
}

pub fn default_dark_theme() -> AppTheme {
    AppTheme {
        bg: hex("#020617"),
        bg_elevated: hex("#0f172a"),
        bg_panel: hex("#1e293b").alpha(0.6),
        text: hex("#f8fafc"),
        muted: hex("#94a3b8"),
        line: hex("#334155"),
        accent: hex("#38bdf8"),
        accent_2: hex("#818cf8"),
        success: hex("#34d399"),
        warning: hex("#fbbf24"),
        danger: hex("#fb7185"),
        shadow: "0 25px 60px -15px rgba(0, 0, 0, 0.5), 0 10px 20px -10px rgba(0, 0, 0, 0.4)".to_string(),
        radius: px(24),
    }
}

pub fn get_theme(name: &str) -> AppTheme {
    match name {
        "light" => default_light_theme(),
        _ => default_dark_theme(),
    }
}

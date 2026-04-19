use crate::service::*;
use crate::views::*;
use silex::prelude::*;
use std::time::Duration;

// --- Global Styles ---
global! {
    :root {
        color-scheme: dark;
        transition: color-scheme 0.3s;
    }

    ":root[data-theme=\"light\"]" {
        color-scheme: light;
    }

    @keyframes pulse-status {
        0% { opacity: 1; transform: scale(1); }
        50% { opacity: 0.7; transform: scale(0.96); }
        100% { opacity: 1; transform: scale(1); }
    }

    .pulse { animation: pulse-status 2s infinite ease-in-out; }
    * { box-sizing: border-box; }

    html, body {
        margin: 0;
        min-height: 100%;
        background:
            radial-gradient(circle at 50% -10%, rgba(125, 211, 252, 0.12), transparent 40%),
            $AppTheme::BG;
        color: $AppTheme::TEXT;
        font-family: Inter, system-ui, -apple-system, sans-serif;
        -webkit-font-smoothing: antialiased;
        transition: background-color 0.3s, color 0.3s;
    }

    a { color: inherit; text-decoration: none; }

    .shell {
        min-height: 100vh;
        padding: 24px;
    }

    .frame {
        max-width: 1400px;
        margin: 0 auto;
        display: grid;
        gap: 20px;
    }

    .hero {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 24px;
        padding: 32px;
        background: $AppTheme::BG_ELEVATED;
        border: 1px solid $AppTheme::LINE;
        border-radius: 28px;
        box-shadow: $AppTheme::SHADOW;
        backdrop-filter: blur(20px);
        position: relative;
        overflow: hidden;
    }

    .hero::before {
        content: "";
        position: absolute;
        top: 0; left: 0; right: 0; height: 1px;
        background: linear-gradient(90deg, transparent, $AppTheme::ACCENT, transparent);
        opacity: 0.3;
    }

    .hero-title {
        margin: 0 0 8px;
        font-size: 42px;
        font-weight: 850;
        line-height: 1;
        letter-spacing: -0.04 em;
        background: linear-gradient(135deg, $AppTheme::TEXT, $AppTheme::ACCENT);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
    }

    .hero-subtitle {
        margin: 0;
        max-width: 60ch;
        color: $AppTheme::MUTED;
        line-height: 1.6;
        font-size: 16px;
    }

    .hero-meta {
        display: flex;
        flex-direction: column;
        gap: 12px;
        align-items: flex-end;
    }

    .pill {
        display: inline-flex;
        align-items: center;
        gap: 8px;
        padding: 8px 16px;
        border-radius: 999px;
        border: 1px solid $AppTheme::LINE;
        background: rgba(255, 255, 255, 0.05);
        color: $AppTheme::TEXT;
        font-size: 13px;
        font-weight: 600;
        white-space: nowrap;
        transition: background-color 0.2s, border-color 0.2s;
    }

    .pill:hover {
        border-color: $AppTheme::ACCENT;
        background: rgba(125, 211, 252, 0.1);
    }

    .layout {
        display: grid;
        grid-template-columns: 240px 1fr;
        gap: 20px;
        align-items: start;
    }

    .sidebar, .page, .card {
        background: $AppTheme::BG_PANEL;
        border: 1px solid $AppTheme::LINE;
        border-radius: $AppTheme::RADIUS;
        box-shadow: $AppTheme::SHADOW;
        backdrop-filter: blur(16px);
        transition: background-color 0.3s, border-color 0.3s;
    }

    .sidebar {
        padding: 20px;
        position: sticky;
        top: 24px;
    }

    .nav {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .nav a {
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 12px 16px;
        border-radius: 14px;
        color: $AppTheme::MUTED;
        font-weight: 550;
        transition: all 0.2s;
    }

    .nav a:hover {
        color: $AppTheme::TEXT;
        background: rgba(125, 211, 252, 0.08);
    }

    .nav a.active {
        color: $AppTheme::TEXT;
        background: linear-gradient(135deg, rgba(125, 211, 252, 0.15), rgba(167, 139, 250, 0.1));
        border: 1px solid rgba(125, 211, 252, 0.2);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    }

    .page {
        padding: 32px;
        display: flex;
        flex-direction: column;
        gap: 24px;
    }

    .metric-label {
        color: $AppTheme::MUTED;
        text-transform: uppercase;
        letter-spacing: 0.1 em;
        font-size: 11px;
        font-weight: 800;
    }

    .kv-row {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        gap: 16px;
        padding: 12px 14px;
        border-radius: 14px;
        border: 1px solid $AppTheme::LINE;
        background: rgba(255, 255, 255, 0.03);
    }

    .kv-label {
        color: $AppTheme::MUTED;
        text-transform: uppercase;
        letter-spacing: 0.08 em;
        font-size: 11px;
        font-weight: 800;
        white-space: nowrap;
    }

    .kv-value {
        color: $AppTheme::TEXT;
        text-align: right;
        line-height: 1.5;
        word-break: break-word;
    }

    .tool {
        display: inline-flex;
        align-items: center;
        gap: 8px;
        appearance: none;
        border: 1px solid $AppTheme::LINE;
        background: rgba(255, 255, 255, 0.05);
        color: $AppTheme::TEXT;
        border-radius: 14px;
        padding: 10px 18px;
        font-size: 14px;
        font-weight: 650;
        cursor: pointer;
        transition: all 0.2s;
    }

    .tool:hover:not(:disabled) {
        transform: translateY(-2px);
        border-color: $AppTheme::ACCENT;
        background: rgba(125, 211, 252, 0.12);
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
    }

    .tool:active:not(:disabled) { transform: translateY(0); }
    .tool:disabled { opacity: 0.5; cursor: not-allowed; }

    .tool-danger:hover:not(:disabled) {
        background: rgba(251, 113, 133, 0.12);
        border-color: $AppTheme::DANGER;
    }

    .editor {
        min-height: 480px;
        max-height: 70vh;
        resize: vertical;
        width: 100%;
        border-radius: 16px;
        border: 1px solid $AppTheme::LINE;
        background: rgba(0, 0, 0, 0.2);
        color: $AppTheme::TEXT;
        padding: 20px;
        font-family: "JetBrains Mono", "Cascadia Code", monospace;
        font-size: 14px;
        line-height: 1.6;
        outline: none;
        transition: border-color 0.2s;
    }
    .editor:focus { border-color: $AppTheme::ACCENT; }

    .badge {
        display: inline-flex;
        align-items: center;
        padding: 4px 10px;
        border-radius: 8px;
        background: rgba(125, 211, 252, 0.1);
        color: $AppTheme::ACCENT;
        font-size: 11px;
        font-weight: 750;
        text-transform: uppercase;
        letter-spacing: 0.05 em;
    }

    .log-line {
        padding: 12px;
        border-bottom: 1px solid $AppTheme::LINE;
        font-family: "JetBrains Mono", monospace;
        font-size: 12px;
        line-height: 1.5;
        white-space: pre-wrap;
        word-break: break-all;
    }

    .status-ok { color: $AppTheme::SUCCESS; }
    .status-warn { color: $AppTheme::WARNING; }
    .status-bad { color: $AppTheme::DANGER; }

    @media (max-width: 1024px) {
        .layout { grid-template-columns: 1fr; }
        .sidebar { position: static; }
        .hero { flex-direction: column; align-items: flex-start; }
        .hero-meta { align-items: flex-start; }
    }
}

#[component]
pub fn AppShell() -> impl View {
    setup_global_error_handlers();

    let theme_name = Persistent::builder("rchronos-web-theme")
        .local()
        .string()
        .default("dark".to_string())
        .build();

    let (theme, set_theme) = Signal::pair(get_theme(&theme_name.get_untracked()));

    // Sync theme when persistent value changes
    Effect::new({
        let theme_name = theme_name.clone();
        move |_| {
            let name = theme_name.get();
            set_theme.set(get_theme(&name));

            // Sync to <html> data-theme attribute
            if let Some(win) = silex::reexports::web_sys::window()
                && let Some(doc) = win.document()
                && let Some(root) = doc.document_element()
            {
                let _ = root.set_attribute("data-theme", &name);
            }
        }
    });

    // Set global theme for reactive propagation
    set_global_theme(theme);

    let ctx = DashboardContext {
        snapshot: HttpClient::get("/api/state")
            .json::<RuntimeSnapshot>()
            .as_resource(RwSignal::new(0usize)),
        sync: Mutation::new(|_| async move {
            HttpClient::post("/api/sync").send().await?;
            Ok("Sync requested".to_string())
        }),
        reload: Mutation::new(|_| async move {
            HttpClient::post("/api/reload").send().await?;
            Ok("Config reloaded".to_string())
        }),
        save: Mutation::new(|_| async move {
            HttpClient::post("/api/save").send().await?;
            Ok("Config saved".to_string())
        }),
        stop: Mutation::new(|_| async move {
            HttpClient::post("/api/stop").send().await?;
            Ok("Stop requested".to_string())
        }),
        apply_config: Mutation::new(|toml: String| async move {
            HttpClient::post("/api/config")
                .header("Content-Type", "application/json")
                .json_body(ConfigForm { toml })
                .send()
                .await?;
            Ok("Config applied".to_string())
        }),
        config_draft: Persistent::builder("rchronos-web-config-draft")
            .local()
            .string()
            .default(String::new())
            .build(),
        auto_refresh: Persistent::builder("rchronos-web-auto-refresh")
            .local()
            .parse::<bool>()
            .default(true)
            .build(),
        theme_name,
    };

    provide_context(ctx);

    let refresh_timer = StoredValue::new(None::<IntervalHandle>);

    Effect::new({
        let ctx = ctx;
        let refresh_timer = refresh_timer;
        move |_| {
            if let Some(handle) = refresh_timer.get_untracked() {
                handle.clear();
            }

            if ctx.auto_refresh.get() {
                if let Ok(handle) = set_interval_with_handle(
                    {
                        let snapshot = ctx.snapshot;
                        move || snapshot.refetch()
                    },
                    Duration::from_secs(5),
                ) {
                    refresh_timer.set_untracked(Some(handle));
                }
            } else {
                refresh_timer.set_untracked(None);
            }
        }
    });

    Effect::new({
        let ctx = ctx;
        move |_| {
            if ctx.config_draft.get().is_empty()
                && let Some(snapshot) = ctx.snapshot.get_data()
                && let Ok(toml) = toml::to_string_pretty(&snapshot.config)
            {
                ctx.config_draft.set(toml);
            }
        }
    });

    Effect::new({
        let ctx = ctx;
        move |_| {
            if matches!(ctx.sync.state.get(), MutationState::Success(_)) {
                let snapshot = ctx.snapshot;
                set_timeout(move || snapshot.refetch(), Duration::from_millis(750));
            }
        }
    });

    Effect::new({
        let ctx = ctx;
        move |_| {
            if matches!(ctx.reload.state.get(), MutationState::Success(_))
                || matches!(ctx.save.state.get(), MutationState::Success(_))
                || matches!(ctx.apply_config.state.get(), MutationState::Success(_))
            {
                ctx.snapshot.refetch();
            }
        }
    });

    Effect::new({
        let ctx = ctx;
        move |_| {
            if matches!(ctx.stop.state.get(), MutationState::Success(_)) {
                console_log("Stop requested from web UI.");
            }
        }
    });

    div![
        GlobalStyles(),
        div![
            div![
                div![
                    h1("rchronos control center").class("hero-title"),
                    p("A Rust + Silex WASM dashboard that controls the service, edits config live, and keeps the status stream visible at a glance.")
                        .class("hero-subtitle"),
                ],
                div![
                    span(ctx.theme_name.map_fn(|theme| if theme == "dark" { "Dark theme" } else { "Light theme" }))
                        .class("pill")
                        .on(event::click, move |_| {
                            let new_theme = if ctx.theme_name.get_untracked() == "dark" { "light" } else { "dark" };
                            ctx.theme_name.set(new_theme.to_string());
                        })
                        .style("cursor: pointer"),
                    span(ctx.auto_refresh.map_fn(|enabled| if *enabled { "Auto refresh on" } else { "Auto refresh off" }))
                        .class("pill")
                        .on(event::click, move |_| {
                            ctx.auto_refresh.update(|v| *v = !*v);
                        })
                        .style("cursor: pointer"),
                    span(move || {
                        ctx.snapshot
                            .state
                            .get()
                            .as_option()
                            .map(|s| s.config_path.clone())
                            .unwrap_or_else(|| "Loading config...".to_string())
                    })
                    .class("pill"),
                ]
                .class("hero-meta"),
            ]
            .class("hero"),
            div![
                Sidebar(),
                Router::new().match_route::<AppRoute>(),
            ]
            .class("layout"),
        ]
        .class("frame"),
    ]
    .class("shell")
    .apply(theme_variables(theme))
}

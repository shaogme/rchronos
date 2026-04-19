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
        min-height: 100vh;
        background:
            radial-gradient(circle at 100% 0%, rgba(56, 189, 248, 0.08), transparent 50%),
            radial-gradient(circle at 0% 100%, rgba(129, 140, 248, 0.08), transparent 50%),
            $AppTheme::BG;
        color: $AppTheme::TEXT;
        font-family: "Inter", "Outfit", system-ui, -apple-system, sans-serif;
        -webkit-font-smoothing: antialiased;
        transition: background-color 0.4s cubic-bezier(0.4, 0, 0.2, 1), color 0.4s;
    }

    a { color: inherit; text-decoration: none; }

    .shell {
        min-height: 100vh;
        padding: 40px 24px;
    }

    .frame {
        max-width: 1600px;
        width: 95%;
        margin: 0 auto;
        display: flex;
        flex-direction: column;
        gap: 32px;
    }

    .hero {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 32px;
        padding: 48px;
        background: rgba($AppTheme::BG_ELEVATED, 0.6);
        border: 1px solid $AppTheme::LINE;
        border-radius: 32px;
        box-shadow: $AppTheme::SHADOW;
        backdrop-filter: blur(24px);
        position: relative;
        overflow: hidden;
    }

    .hero::before {
        content: "";
        position: absolute;
        top: 0; left: 0; right: 0; height: 1px;
        background: linear-gradient(90deg, transparent, $AppTheme::ACCENT, transparent);
        opacity: 0.4;
    }

    .hero-title {
        margin: 0 0 12px;
        font-size: 48px;
        font-weight: 850;
        line-height: 0.95;
        letter-spacing: -0.05 em;
        background: linear-gradient(135deg, $AppTheme::TEXT, $AppTheme::ACCENT);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
    }

    .hero-subtitle {
        margin: 0;
        max-width: 55ch;
        color: $AppTheme::MUTED;
        line-height: 1.6;
        font-size: 17px;
        font-weight: 500;
        opacity: 0.85;
    }

    .hero-meta {
        display: flex;
        flex-direction: column;
        gap: 16px;
        align-items: flex-end;
    }

    .pill {
        display: inline-flex;
        align-items: center;
        gap: 10px;
        padding: 10px 20px;
        border-radius: 999px;
        border: 1px solid $AppTheme::LINE;
        background: rgba($AppTheme::BG_PANEL, 0.4);
        color: $AppTheme::TEXT;
        font-size: 13px;
        font-weight: 700;
        white-space: nowrap;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        backdrop-filter: blur(8px);
    }

    .pill:hover {
        border-color: $AppTheme::ACCENT;
        background: rgba($AppTheme::ACCENT, 0.1);
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    }

    .layout {
        display: flex;
        gap: 32px;
        align-items: start;
    }

    .sidebar {
        width: 280px;
        flex-shrink: 0;
        padding: 24px;
        background: rgba($AppTheme::BG_PANEL, 0.5);
        border: 1px solid $AppTheme::LINE;
        border-radius: 32px;
        box-shadow: $AppTheme::SHADOW;
        backdrop-filter: blur(24px);
        position: sticky;
        top: 40px;
        transition: all 0.3s;
    }

    .nav {
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    .nav a {
        display: flex;
        align-items: center;
        gap: 14px;
        padding: 14px 20px;
        border-radius: 18px;
        color: $AppTheme::MUTED;
        font-weight: 600;
        font-size: 15px;
        transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .nav a:hover {
        color: $AppTheme::TEXT;
        background: rgba($AppTheme::ACCENT, 0.1);
        transform: translateX(4px);
    }

    .nav a.active {
        color: $AppTheme::TEXT;
        background: linear-gradient(135deg, rgba($AppTheme::ACCENT, 0.15), rgba($AppTheme::ACCENT_2, 0.1));
        border: 1px solid rgba($AppTheme::ACCENT, 0.2);
        box-shadow: 0 8px 16px rgba(0, 0, 0, 0.08);
    }

    .page {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: 32px;
    }

    .kv-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 20px;
        padding: 14px 18px;
        border-radius: 18px;
        border: 1px solid $AppTheme::LINE;
        background: rgba($AppTheme::BG_PANEL, 0.3);
        transition: all 0.2s;
    }

    .kv-row:hover {
        background: rgba($AppTheme::BG_PANEL, 0.5);
        border-color: $AppTheme::ACCENT;
    }

    .editor {
        min-height: 520px;
        max-height: 75vh;
        resize: vertical;
        width: 100%;
        border-radius: 20px;
        border: 1px solid $AppTheme::LINE;
        background: rgba(0, 0, 0, 0.25);
        color: $AppTheme::TEXT;
        padding: 24px;
        font-family: "JetBrains Mono", "Cascadia Code", monospace;
        font-size: 14px;
        line-height: 1.6;
        outline: none;
        transition: border-color 0.3s;
    }
    .editor:focus { border-color: $AppTheme::ACCENT; }

    .status-ok { color: $AppTheme::SUCCESS; font-weight: 700; }
    .status-warn { color: $AppTheme::WARNING; font-weight: 700; }
    .status-bad { color: $AppTheme::DANGER; font-weight: 700; }

    .grid {
        display: grid;
        gap: 32px;
    }

    .metrics {
        grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    }

    .stack {
        display: flex;
        flex-direction: column;
        gap: 32px;
    }

    .overview-grid {
        display: grid;
        grid-template-columns: 1fr 500px;
        gap: 32px;
    }

    .split {
        display: grid;
        grid-template-columns: 1fr 480px;
        gap: 32px;
    }

    .double {
        grid-template-columns: 1fr 1fr;
    }

    @media (max-width: 1100px) {
        .layout { flex-direction: column; }
        .sidebar { width: 100%; position: static; }
        .hero { flex-direction: column; align-items: flex-start; padding: 32px; }
        .hero-meta { align-items: flex-start; }
        .overview-grid, .split, .grid.double {
            grid-template-columns: 1fr;
        }
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

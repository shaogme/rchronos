use crate::service::*;
use crate::views::*;
use silex::css::theme::ThemeToCss;
use silex::prelude::*;
use std::time::Duration;

#[component]
pub fn AppShell() -> impl View {
    setup_global_error_handlers();
    inject_style(
        "rchronos-web-shell",
        r#"
        :root {
            color-scheme: dark;
            transition: color-scheme 0.3s;
        }
        :root[data-theme="light"] {
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
                var(--bg);
            color: var(--text);
            font-family: Inter, system-ui, -apple-system, sans-serif;
            -webkit-font-smoothing: antialiased;
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
            background: var(--bg-elevated);
            border: 1px solid var(--line);
            border-radius: 28px;
            box-shadow: var(--shadow);
            backdrop-filter: blur(20px);
            position: relative;
            overflow: hidden;
        }
        .hero::before {
            content: "";
            position: absolute;
            top: 0; left: 0; right: 0; height: 1px;
            background: linear-gradient(90deg, transparent, var(--accent), transparent);
            opacity: 0.3;
        }
        .hero-title {
            margin: 0 0 8px;
            font-size: 42px;
            font-weight: 850;
            line-height: 1;
            letter-spacing: -0.04em;
            background: linear-gradient(135deg, var(--text), var(--accent));
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }
        .hero-subtitle {
            margin: 0;
            max-width: 60ch;
            color: var(--muted);
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
            border: 1px solid var(--line);
            background: rgba(255, 255, 255, 0.05);
            color: var(--text);
            font-size: 13px;
            font-weight: 600;
            white-space: nowrap;
            transition: var(--transition);
        }
        .pill:hover { border-color: var(--accent); background: rgba(125, 211, 252, 0.1); }
        .layout {
            display: grid;
            grid-template-columns: 240px 1fr;
            gap: 20px;
            align-items: start;
        }
        .sidebar, .page, .card {
            background: var(--bg-panel);
            border: 1px solid var(--line);
            border-radius: var(--radius);
            box-shadow: var(--shadow);
            backdrop-filter: blur(16px);
            transition: var(--transition);
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
            color: var(--muted);
            font-weight: 550;
            transition: var(--transition);
        }
        .nav a:hover {
            color: var(--text);
            background: rgba(125, 211, 252, 0.08);
        }
        .nav a.active {
            color: var(--text);
            background: linear-gradient(135deg, rgba(125, 211, 252, 0.15), rgba(167, 139, 250, 0.1));
            border: 1px solid rgba(125, 211, 252, 0.2);
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
        }
        .nav-side-actions {
            display: flex;
            flex-direction: column;
            gap: 12px;
            margin-top: 20px;
            padding-top: 20px;
            border-top: 1px solid var(--line);
        }
        .page {
            padding: 32px;
            display: flex;
            flex-direction: column;
            gap: 24px;
        }
        .page h2 {
            margin: 0;
            font-size: 32px;
            font-weight: 800;
            letter-spacing: -0.03em;
        }
        .grid {
            display: grid;
            gap: 18px;
        }
        .grid.metrics {
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
        }
        .grid.double {
            grid-template-columns: repeat(2, minmax(0, 1fr));
        }
        .overview-grid {
            display: grid;
            grid-template-columns: 1fr 380px;
            gap: 24px;
            align-items: start;
        }
        .metric {
            padding: 24px;
            position: relative;
            overflow: hidden;
        }
        .metric::after {
            content: "";
            position: absolute;
            bottom: 0; right: 0; width: 60px; height: 60px;
            background: radial-gradient(circle at center, var(--accent), transparent 70%);
            opacity: 0.05;
        }
        .metric-label {
            color: var(--muted);
            text-transform: uppercase;
            letter-spacing: 0.1em;
            font-size: 11px;
            font-weight: 800;
        }
        .metric-value {
            margin-top: 12px;
            font-size: 32px;
            font-weight: 850;
            line-height: 1;
            letter-spacing: -0.02em;
        }
        .metric-subtitle {
            margin-top: 8px;
            color: var(--muted);
            font-size: 14px;
        }
        .toolbar {
            display: flex;
            flex-wrap: wrap;
            gap: 12px;
        }
        .panel {
            display: flex;
            flex-direction: column;
            gap: 16px;
            padding: 24px;
        }
        .panel-header {
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            gap: 12px;
            flex-wrap: wrap;
        }
        .panel-header h2,
        .panel-header h3 {
            margin: 0;
        }
        .panel-section {
            display: flex;
            flex-direction: column;
            gap: 12px;
            padding-top: 16px;
            border-top: 1px solid var(--line);
        }
        .kv-list {
            display: grid;
            gap: 10px;
        }
        .kv-row {
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            gap: 16px;
            padding: 12px 14px;
            border-radius: 14px;
            border: 1px solid var(--line);
            background: rgba(255, 255, 255, 0.03);
        }
        .kv-label {
            color: var(--muted);
            text-transform: uppercase;
            letter-spacing: 0.08em;
            font-size: 11px;
            font-weight: 800;
            white-space: nowrap;
        }
        .kv-value {
            color: var(--text);
            text-align: right;
            line-height: 1.5;
            word-break: break-word;
        }
        .snapshot-panel {
            gap: 18px;
        }
        .snapshot-state {
            text-align: left;
            line-height: 1.6;
        }
        .snapshot-summary {
            margin-top: 4px;
        }
        .tool {
            display: inline-flex;
            align-items: center;
            gap: 8px;
            appearance: none;
            border: 1px solid var(--line);
            background: rgba(255, 255, 255, 0.05);
            color: var(--text);
            border-radius: 14px;
            padding: 10px 18px;
            font-size: 14px;
            font-weight: 650;
            cursor: pointer;
            transition: var(--transition);
        }
        .tool:hover:not(:disabled) {
            transform: translateY(-2px);
            border-color: var(--accent);
            background: rgba(125, 211, 252, 0.12);
            box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
        }
        .tool:active:not(:disabled) { transform: translateY(0); }
        .tool:disabled { opacity: 0.5; cursor: not-allowed; }
        .tool-danger:hover:not(:disabled) {
            background: rgba(251, 113, 133, 0.12);
            border-color: var(--danger);
        }
        .split {
            display: grid;
            grid-template-columns: 1.5fr 1fr;
            gap: 24px;
            align-items: start;
        }
        .stack { display: grid; gap: 16px; }
        .editor {
            min-height: 480px;
            max-height: 70vh;
            resize: vertical;
            width: 100%;
            border-radius: 16px;
            border: 1px solid var(--line);
            background: rgba(0, 0, 0, 0.2);
            color: var(--text);
            padding: 20px;
            font-family: "JetBrains Mono", "Cascadia Code", monospace;
            font-size: 14px;
            line-height: 1.6;
            outline: none;
            transition: border-color 0.2s;
        }
        .editor:focus { border-color: var(--accent); }
        .badge {
            display: inline-flex;
            align-items: center;
            padding: 4px 10px;
            border-radius: 8px;
            background: rgba(125, 211, 252, 0.1);
            color: var(--accent);
            font-size: 11px;
            font-weight: 750;
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }
        .logs {
            max-height: 600px;
            overflow-y: auto;
            scrollbar-width: thin;
        }
        .log-line {
            padding: 12px;
            border-bottom: 1px solid var(--line);
            font-family: "JetBrains Mono", monospace;
            font-size: 12px;
            line-height: 1.5;
            white-space: pre-wrap;
            word-break: break-all;
        }
        .log-line:last-child { border-bottom: none; }
        .section-header {
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            gap: 12px;
            flex-wrap: wrap;
            margin-bottom: 8px;
        }
        .section-header h3 { margin: 0; font-size: 18px; font-weight: 750; }
        .banner {
            padding: 16px;
            border-radius: 14px;
            background: rgba(255, 255, 255, 0.03);
            border: 1px solid var(--line);
            font-size: 14px;
            line-height: 1.6;
        }
        .status-ok { color: var(--success); }
        .status-warn { color: var(--warning); }
        .status-bad { color: var(--danger); }
        @media (max-width: 1280px) {
            .overview-grid { grid-template-columns: 1fr; }
        }
        @media (max-width: 1024px) {
            .layout { grid-template-columns: 1fr; }
            .sidebar { position: static; }
            .hero { flex-direction: column; align-items: flex-start; }
            .hero-meta { align-items: flex-start; }
            .grid.double { grid-template-columns: 1fr; }
            .split { grid-template-columns: 1fr; }
        }
    "#,
    );

    let theme_name = Persistent::builder("rchronos-web-theme")
        .local()
        .string()
        .default("dark".to_string())
        .build();
    let (theme, theme_setter) = Signal::pair(get_theme(&theme_name.get_untracked()));
    Effect::new({
        let theme_name = theme_name.clone();
        move |_| {
            theme_setter.set(get_theme(&theme_name.get()));
        }
    });

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
        theme,
    };

    provide_context(ctx);

    let refresh_timer = StoredValue::new(None::<IntervalHandle>);

    Effect::new(move |_| {
        let name = ctx.theme_name.get();
        let theme = ctx.theme.get();
        if let Some(window) = silex::reexports::web_sys::window()
            && let Some(document) = window.document()
            && let Some(root) = document.document_element()
        {
            let _ = root.set_attribute("data-theme", &name);
            let _ = root.set_attribute("style", &theme.to_css_variables());
        }
    });

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
        div![
            div![
                div![
                    h1("rchronos control center").class("hero-title"),
                    p("A Rust + Silex WASM dashboard that controls the service, edits config live, and keeps the status stream visible at a glance.")
                        .class("hero-subtitle"),
                ],
                div![
                    span(ctx.theme_name.map_fn(|theme| if theme == "dark" { "Dark theme" } else { "Light theme" }))
                        .class("pill"),
                    span(ctx.auto_refresh.map_fn(|enabled| if *enabled { "Auto refresh on" } else { "Auto refresh off" }))
                        .class("pill"),
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
}

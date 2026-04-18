use crate::service::*;
use silex::prelude::*;

fn kv_row(label: &'static str, value: String) -> AnyView {
    div![span(label).class("kv-label"), span(value).class("kv-value"),]
        .class("kv-row")
        .into_any()
}

fn icon_overview() -> impl View {
    svg(view_chain![
        rect()
            .attr("width", "7")
            .attr("height", "9")
            .attr("x", "3")
            .attr("y", "3")
            .attr("rx", "1"),
        rect()
            .attr("width", "7")
            .attr("height", "5")
            .attr("x", "14")
            .attr("y", "3")
            .attr("rx", "1"),
        rect()
            .attr("width", "7")
            .attr("height", "9")
            .attr("x", "14")
            .attr("y", "12")
            .attr("rx", "1"),
        rect()
            .attr("width", "7")
            .attr("height", "5")
            .attr("x", "3")
            .attr("y", "16")
            .attr("rx", "1"),
    ])
    .attr("width", "16")
    .attr("height", "16")
    .attr("viewBox", "0 0 24 24")
    .attr("fill", "none")
    .attr("stroke", "currentColor")
    .attr("stroke-width", "2.5")
    .attr("stroke-linecap", "round")
    .attr("stroke-linejoin", "round")
}

fn icon_config() -> impl View {
    svg(view_chain![
        path().attr("d", "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.1a2 2 0 0 1-1-1.72v-.51a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"),
        circle().attr("cx", "12").attr("cy", "12").attr("r", "3")
    ])
    .attr("width", "16")
    .attr("height", "16")
    .attr("viewBox", "0 0 24 24")
    .attr("fill", "none")
    .attr("stroke", "currentColor")
    .attr("stroke-width", "2.5")
    .attr("stroke-linecap", "round")
    .attr("stroke-linejoin", "round")
}

fn icon_logs() -> impl View {
    svg(view_chain![
        path().attr(
            "d",
            "M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"
        ),
        path().attr("d", "M13 2v7h7"),
        path().attr("d", "M9 18h6"),
        path().attr("d", "M9 14h6")
    ])
    .attr("width", "16")
    .attr("height", "16")
    .attr("viewBox", "0 0 24 24")
    .attr("fill", "none")
    .attr("stroke", "currentColor")
    .attr("stroke-width", "2.5")
    .attr("stroke-linecap", "round")
    .attr("stroke-linejoin", "round")
}

fn icon_about() -> impl View {
    svg(view_chain![
        circle().attr("cx", "12").attr("cy", "12").attr("r", "10"),
        path().attr("d", "M12 16v-4"),
        path().attr("d", "M12 8h.01")
    ])
    .attr("width", "16")
    .attr("height", "16")
    .attr("viewBox", "0 0 24 24")
    .attr("fill", "none")
    .attr("stroke", "currentColor")
    .attr("stroke-width", "2.5")
    .attr("stroke-linecap", "round")
    .attr("stroke-linejoin", "round")
}

fn icon_sync() -> impl View {
    svg(view_chain![
        path().attr("d", "M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"),
        path().attr("d", "M3 3v5h5"),
        path().attr("d", "M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"),
        path().attr("d", "M16 16h5v5")
    ])
    .attr("width", "15")
    .attr("height", "15")
    .attr("viewBox", "0 0 24 24")
    .attr("fill", "none")
    .attr("stroke", "currentColor")
    .attr("stroke-width", "2.5")
    .attr("stroke-linecap", "round")
    .attr("stroke-linejoin", "round")
}

fn icon_save() -> impl View {
    svg(view_chain![
        path().attr("d", "M15.2 3a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z"),
        path().attr("d", "M17 21v-7a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v7"),
        path().attr("d", "M7 3v4a1 1 0 0 0 1 1h7")
    ])
    .attr("width", "15")
    .attr("height", "15")
    .attr("viewBox", "0 0 24 24")
    .attr("fill", "none")
    .attr("stroke", "currentColor")
    .attr("stroke-width", "2.5")
    .attr("stroke-linecap", "round")
    .attr("stroke-linejoin", "round")
}

fn icon_stop() -> impl View {
    svg(view_chain![
        circle().attr("cx", "12").attr("cy", "12").attr("r", "10"),
        rect()
            .attr("width", "6")
            .attr("height", "6")
            .attr("x", "9")
            .attr("y", "9")
            .attr("rx", "1")
    ])
    .attr("width", "15")
    .attr("height", "15")
    .attr("viewBox", "0 0 24 24")
    .attr("fill", "none")
    .attr("stroke", "currentColor")
    .attr("stroke-width", "2.5")
    .attr("stroke-linecap", "round")
    .attr("stroke-linejoin", "round")
}

fn icon_reload() -> impl View {
    svg(view_chain![
        path().attr("d", "M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"),
        path().attr("d", "M3 3v5h5")
    ])
    .attr("width", "15")
    .attr("height", "15")
    .attr("viewBox", "0 0 24 24")
    .attr("fill", "none")
    .attr("stroke", "currentColor")
    .attr("stroke-width", "2.5")
    .attr("stroke-linecap", "round")
    .attr("stroke-linejoin", "round")
}

#[derive(Route, Clone, PartialEq)]
pub enum AppRoute {
    #[route("/", view = OverviewPage)]
    Overview,
    #[route("/config", view = ConfigPage)]
    Config,
    #[route("/logs", view = LogsPage)]
    Logs,
    #[route("/about", view = AboutPage)]
    About,
    #[route("/*", view = NotFoundPage)]
    NotFound,
}

#[component]
pub fn Sidebar() -> impl View {
    let ctx = use_dashboard();

    div![
        div![
            Link(AppRoute::Overview, span![icon_overview(), "Overview"]).active_class("active"),
            Link(AppRoute::Config, span![icon_config(), "Config"]).active_class("active"),
            Link(AppRoute::Logs, span![icon_logs(), "Logs"]).active_class("active"),
            Link(AppRoute::About, span![icon_about(), "About"]).active_class("active"),
        ]
        .class("nav"),
        div![
            button(ctx.theme.map_fn(|theme| if theme == "dark" {
                "Switch to Light"
            } else {
                "Switch to Dark"
            }))
            .on(event::click, move |_| {
                ctx.theme.update(|theme| {
                    *theme = if theme == "dark" {
                        "light".to_string()
                    } else {
                        "dark".to_string()
                    };
                });
            })
            .class("tool"),
            button(ctx.auto_refresh.map_fn(|enabled| {
                if *enabled {
                    "Pause Refresh"
                } else {
                    "Resume Refresh"
                }
            }))
            .on(event::click, move |_| {
                ctx.auto_refresh.update(|enabled| *enabled = !*enabled);
            })
            .class("tool"),
            button("Refresh now")
                .on(event::click, move |_| ctx.snapshot.refetch())
                .class("tool"),
        ]
        .class("nav-side-actions"),
    ]
    .class("sidebar")
}

#[component]
pub fn OverviewPage() -> impl View {
    let ctx = use_dashboard();

    div![
        div![
            div![
                h2("Overview"),
                p("Live service status, control shortcuts, and a quick glance at the current runtime state.")
                    .class("page-intro"),
            ],
            div![
                button(span![icon_sync(), "Sync now"])
                    .on(event::click, move |_| ctx.sync.mutate(()))
                    .attr("disabled", rx!(@fn ctx.sync.loading()))
                    .class("tool"),
                button(span![icon_reload(), "Reload config"])
                    .on(event::click, move |_| ctx.reload.mutate(()))
                    .attr("disabled", rx!(@fn ctx.reload.loading()))
                    .class("tool"),
                button(span![icon_save(), "Save config"])
                    .on(event::click, move |_| ctx.save.mutate(()))
                    .attr("disabled", rx!(@fn ctx.save.loading()))
                    .class("tool"),
                button(span![icon_stop(), "Stop service"])
                    .on(event::click, move |_| ctx.stop.mutate(()))
                    .attr("disabled", rx!(@fn ctx.stop.loading()))
                    .class("tool tool-danger"),
            ]
            .class("toolbar"),
        ]
        .class("section-header"),
        SummaryPanel(),
        div![
            div![ServicePanel(), MutationLedger()].class("stack"),
            RecentLogsPanel().limit(12),
        ]
        .class("overview-grid"),
    ]
    .class("page")
}

#[component]
pub fn ConfigPage() -> impl View {
    let ctx = use_dashboard();

    div![
        div![
            h2("Config"),
            p("Edit the live TOML config, keep a local draft, and push it back to the service without leaving the dashboard.")
                .class("page-intro"),
        ],
        div![
            div![
                div![
                    h3("TOML Draft"),
                    span("Stored locally").class("badge"),
                ]
                .class("section-title"),
                textarea("")
                    .bind_value(ctx.config_draft)
                    .class("editor"),
                div![
                    button(span![icon_reload(), "Load live config"])
                        .on(event::click, move |_| {
                            if let Some(snapshot) = ctx.snapshot.get_data()
                                && let Ok(toml) = toml::to_string_pretty(&snapshot.config)
                            {
                                ctx.config_draft.set(toml);
                            }
                        })
                        .class("tool"),
                    button(span![icon_save(), "Apply draft"])
                        .on(event::click, move |_| ctx.apply_config.mutate(ctx.config_draft.get()))
                        .attr("disabled", rx!(@fn ctx.apply_config.loading()))
                        .class("tool"),
                ]
                .class("toolbar"),
                move || {
                    if let Some(err) = ctx.apply_config.error() {
                        div(format!("Draft error: {:?}", err)).class("banner status-bad").into_any()
                    } else if ctx.apply_config.loading() {
                        div("Applying draft to the service...").class("banner status-warn").into_any()
                    } else if let Some(msg) = ctx.apply_config.value() {
                        div(format!("Last draft update: {msg}")).class("banner status-ok").into_any()
                    } else {
                        "".into_any()
                    }
                }
            ]
            .class("stack")
            .class("card"),

            div![
                SnapshotQuickFacts(),
                SyncTuningPanel(),
                HostSummaryPanel(),
            ]
            .class("stack"),
        ]
        .class("split"),
    ]
    .class("page")
}

#[component]
pub fn LogsPage() -> impl View {
    div![
        div![
            h2("Logs"),
            p("The last lines from the service runtime, plus the current host matrix for easy inspection.")
                .class("page-intro"),
        ],
        div![
            RecentLogsPanel().limit(80),
            HostTablePanel(),
        ]
        .class("split"),
    ]
    .class("page")
}

#[component]
pub fn AboutPage() -> impl View {
    div![
        h2("About"),
        p("This front end is rendered with Silex in WASM and shipped inside the service binary with rust-embed.")
            .class("page-intro"),
        div![
            div![
                h3("What it does"),
                ul![
                    li("Shows the live runtime snapshot from the service."),
                    li("Lets you sync, reload, save, and stop the service."),
                    li("Keeps a persistent TOML draft in the browser."),
                    li("Offers automatic refresh so you can watch state changes happen."),
                ]
            ]
            .class("card"),
            div![
                h3("Operational notes"),
                ul![
                    li("The SPA fallback keeps client-side routes working on refresh."),
                    li("Config changes are sent as JSON to /api/config."),
                    li("The app theme and refresh preference are persisted locally."),
                ]
            ]
            .class("card"),
        ]
        .class("grid double"),
    ]
    .class("page")
}

#[component]
pub fn NotFoundPage() -> impl View {
    div![
        h2("404"),
        p("The route was not found. Use the sidebar to jump back into the dashboard.")
            .class("page-intro"),
    ]
    .class("page")
}

#[component]
fn SummaryPanel() -> impl View {
    let ctx = use_dashboard();

    move || match ctx.snapshot.state.get() {
        ResourceState::Idle | ResourceState::Loading => div![loading_metric(
            "Status",
            "Loading...",
            "Fetching runtime snapshot"
        ),]
        .class("grid metrics")
        .into_any(),
        ResourceState::Ready(snapshot) | ResourceState::Reloading(snapshot) => div![
            metric_card(
                "Status",
                snapshot.status,
                format!("Config: {}", snapshot.config_path)
            ),
            metric_card(
                "Last result",
                snapshot.last_result,
                format!("Syncing: {}", if snapshot.syncing { "yes" } else { "no" })
            ),
            metric_card(
                "Enabled hosts",
                format!(
                    "{}/{}",
                    enabled_hosts(&snapshot.config),
                    snapshot.config.hosts.len()
                ),
                sync_mode_label(snapshot.config.sync_mode).to_string(),
            ),
            metric_card(
                "Delay",
                format!("{:.0}s", snapshot.config.delay_seconds),
                format!("Timeout: {:.0}ms", snapshot.config.network_timeout_ms),
            ),
        ]
        .class("grid metrics")
        .into_any(),
        ResourceState::Error(err) => div![
            div("Snapshot error").class("metric-label"),
            div(format!("Failed: {:?}", err)).class("metric-value status-bad"),
            p("The backend is reachable, but the dashboard could not deserialize a state snapshot.")
                .class("metric-subtitle"),
        ]
        .class("card metric")
        .into_any(),
    }
}

#[component]
fn ServicePanel() -> impl View {
    let ctx = use_dashboard();

    div![
        div![
            div![h3("Live Snapshot"), span("auto-updated").class("badge")].class("panel-header"),
            move || match ctx.snapshot.state.get() {
                ResourceState::Idle | ResourceState::Loading =>
                    div("Waiting for the next service snapshot...")
                        .class("banner snapshot-state")
                        .into_any(),
                ResourceState::Ready(snapshot) | ResourceState::Reloading(snapshot) => div![
                    div![
                        kv_row(
                            "Mode",
                            sync_mode_label(snapshot.config.sync_mode).to_string()
                        ),
                        kv_row("Status", snapshot.status.clone()),
                        kv_row("Config file", snapshot.config_path.clone()),
                        kv_row("User agent", snapshot.config.user_agent.clone()),
                        kv_row(
                            "Syncing",
                            if snapshot.syncing {
                                "yes".to_string()
                            } else {
                                "no".to_string()
                            }
                        ),
                        kv_row(
                            "Offsets",
                            format!(
                                "{:.2}s / {:.2}s",
                                snapshot.config.offset_seconds,
                                snapshot.config.deviation_offset_seconds
                            ),
                        ),
                        kv_row(
                            "Sync interval",
                            format!("{:.0}s", snapshot.config.delay_seconds)
                        ),
                        kv_row(
                            "Network timeout",
                            format!("{:.0}ms", snapshot.config.network_timeout_ms)
                        ),
                    ]
                    .class("kv-list snapshot-summary"),
                ]
                .into_any(),
                ResourceState::Error(err) => div![
                    div("Snapshot error").class("metric-label"),
                    div(format!("Failed to deserialize state snapshot: {:?}", err))
                        .class("banner status-bad snapshot-state"),
                ]
                .class("stack")
                .into_any(),
            }
        ]
        .class("panel-section"),
        div![
            div![h3("Actions"), span("service commands").class("badge")].class("panel-header"),
            div![
                button(span![icon_sync(), "Sync"])
                    .on(event::click, move |_| ctx.sync.mutate(()))
                    .attr("disabled", rx!(@fn ctx.sync.loading()))
                    .class("tool"),
                button(span![icon_reload(), "Reload"])
                    .on(event::click, move |_| ctx.reload.mutate(()))
                    .attr("disabled", rx!(@fn ctx.reload.loading()))
                    .class("tool"),
                button(span![icon_save(), "Save"])
                    .on(event::click, move |_| ctx.save.mutate(()))
                    .attr("disabled", rx!(@fn ctx.save.loading()))
                    .class("tool"),
                button(span![icon_stop(), "Stop"])
                    .on(event::click, move |_| ctx.stop.mutate(()))
                    .attr("disabled", rx!(@fn ctx.stop.loading()))
                    .class("tool tool-danger"),
            ]
            .class("toolbar"),
            move || {
                let mut messages = Vec::new();
                if let Some(msg) = ctx.sync.value() {
                    messages.push(("Sync", msg, "status-ok"));
                }
                if let Some(msg) = ctx.reload.value() {
                    messages.push(("Reload", msg, "status-ok"));
                }
                if let Some(msg) = ctx.save.value() {
                    messages.push(("Save", msg, "status-ok"));
                }
                if let Some(msg) = ctx.stop.value() {
                    messages.push(("Stop", msg, "status-warn"));
                }

                if messages.is_empty() {
                    div("No action has completed yet.")
                        .class("banner snapshot-state")
                        .into_any()
                } else {
                    div(messages
                        .into_iter()
                        .map(|(label, msg, _)| format!("{label}: {msg}"))
                        .collect::<Vec<_>>()
                        .join("\n"))
                    .class("banner snapshot-state")
                    .into_any()
                }
            }
        ]
        .class("panel-section"),
    ]
    .class("card panel snapshot-panel")
}

#[component]
fn SnapshotQuickFacts() -> impl View {
    let ctx = use_dashboard();

    div![
        div![h3("Quick Facts"), span("latest state").class("badge")].class("panel-header"),
        move || match ctx.snapshot.state.get() {
            ResourceState::Idle | ResourceState::Loading => div("Waiting for snapshot...")
                .class("banner snapshot-state")
                .into_any(),
            ResourceState::Ready(snapshot) | ResourceState::Reloading(snapshot) => div![
                kv_row("Config file", snapshot.config_path.clone()),
                kv_row("Hosts configured", snapshot.config.hosts.len().to_string()),
                kv_row("Enabled hosts", enabled_hosts(&snapshot.config).to_string()),
                kv_row(
                    "Syncing",
                    if snapshot.syncing {
                        "yes".to_string()
                    } else {
                        "no".to_string()
                    }
                ),
                kv_row("Last result", snapshot.last_result.clone()),
            ]
            .class("kv-list")
            .into_any(),
            ResourceState::Error(err) => div(format!("Snapshot error: {:?}", err))
                .class("banner status-bad snapshot-state")
                .into_any(),
        }
    ]
    .class("card panel stack")
}

#[component]
fn SyncTuningPanel() -> impl View {
    let ctx = use_dashboard();

    div![
        div![h3("Sync tuning"), span("config hints").class("badge")].class("panel-header"),
        move || match ctx.snapshot.state.get() {
            ResourceState::Idle | ResourceState::Loading => div("Waiting for current config...")
                .class("banner snapshot-state")
                .into_any(),
            ResourceState::Ready(snapshot) | ResourceState::Reloading(snapshot) => div![
                kv_row(
                    "Agreement mode",
                    agreement_label(snapshot.config.agreement).to_string()
                ),
                kv_row("Window", format!("{:.0}ms", snapshot.config.timeout_ms)),
                kv_row(
                    "Precision support",
                    if snapshot.config.high_precision_supported {
                        "yes".to_string()
                    } else {
                        "no".to_string()
                    },
                ),
                kv_row(
                    "Win32 time policy",
                    if snapshot.config.disable_win32_time {
                        "disabled".to_string()
                    } else {
                        "enabled".to_string()
                    },
                ),
                kv_row(
                    "Verbose logging",
                    if snapshot.config.verbose {
                        "on".to_string()
                    } else {
                        "off".to_string()
                    },
                ),
            ]
            .class("kv-list")
            .into_any(),
            ResourceState::Error(err) => div(format!("Unable to inspect config: {:?}", err))
                .class("banner status-bad snapshot-state")
                .into_any(),
        }
    ]
    .class("card panel stack")
}

#[component]
fn HostSummaryPanel() -> impl View {
    let ctx = use_dashboard();

    div![
        div![h3("Host summary"), span("topology").class("badge")].class("panel-header"),
        move || match ctx.snapshot.state.get() {
            ResourceState::Idle | ResourceState::Loading => div("Loading host list...")
                .class("banner snapshot-state")
                .into_any(),
            ResourceState::Ready(snapshot) | ResourceState::Reloading(snapshot) => {
                let mut items = snapshot
                    .config
                    .hosts
                    .iter()
                    .map(|(name, host)| {
                        let kind = request_type_label(host.request_type);
                        format!(
                            "{name} • {kind} • priority {} • {}",
                            host.priority,
                            if host.enabled { "enabled" } else { "disabled" }
                        )
                    })
                    .collect::<Vec<_>>();
                items.truncate(8);

                if items.is_empty() {
                    div("No hosts configured.")
                        .class("banner snapshot-state")
                        .into_any()
                } else {
                    div![div(items.join("\n")).class("banner snapshot-state")]
                        .class("stack")
                        .into_any()
                }
            }
            ResourceState::Error(err) => div(format!("Host list error: {:?}", err))
                .class("banner status-bad snapshot-state")
                .into_any(),
        }
    ]
    .class("card panel stack")
}

#[component]
fn HostTablePanel() -> impl View {
    let ctx = use_dashboard();

    div![
        div![h3("Hosts"), span("current matrix").class("badge")].class("panel-header"),
        move || match ctx.snapshot.state.get() {
            ResourceState::Idle | ResourceState::Loading => div("Loading hosts...")
                .class("banner snapshot-state")
                .into_any(),
            ResourceState::Ready(snapshot) | ResourceState::Reloading(snapshot) => {
                let rows = snapshot
                    .config
                    .hosts
                    .iter()
                    .map(|(name, host)| {
                        format!(
                            "{name} | {} | priority {} | {}",
                            request_type_label(host.request_type),
                            host.priority,
                            if host.enabled { "enabled" } else { "disabled" }
                        )
                    })
                    .collect::<Vec<_>>();

                if rows.is_empty() {
                    div("No hosts configured.")
                        .class("banner snapshot-state")
                        .into_any()
                } else {
                    div![div(rows.join("\n")).class("banner snapshot-state")]
                        .class("stack")
                        .into_any()
                }
            }
            ResourceState::Error(err) => div(format!("Host table error: {:?}", err))
                .class("banner status-bad snapshot-state")
                .into_any(),
        }
    ]
    .class("card panel stack")
}

#[component]
pub fn RecentLogsPanel(limit: usize) -> impl View {
    let ctx = use_dashboard();

    div![
        div![
            h3("Logs"),
            span(format!("last {limit} lines")).class("badge"),
        ]
        .class("panel-header"),
        move || match ctx.snapshot.state.get() {
            ResourceState::Idle | ResourceState::Loading => div("Loading logs...")
                .class("banner snapshot-state")
                .into_any(),
            ResourceState::Ready(snapshot) | ResourceState::Reloading(snapshot) => {
                let logs = snapshot
                    .logs
                    .iter()
                    .rev()
                    .take(limit)
                    .cloned()
                    .collect::<Vec<_>>();

                if logs.is_empty() {
                    div("No logs yet.")
                        .class("banner snapshot-state")
                        .into_any()
                } else {
                    div![div(logs.join("\n")).class("log-line")]
                        .class("logs stack")
                        .into_any()
                }
            }
            ResourceState::Error(err) => div(format!("Log error: {:?}", err))
                .class("banner status-bad snapshot-state")
                .into_any(),
        }
    ]
    .class("card panel stack")
}

#[component]
fn MutationLedger() -> impl View {
    let ctx = use_dashboard();

    div![
        div![
            h3("Command state"),
            action_status("Sync", ctx.sync.loading(), ctx.sync.error().map(|e| format!("{:?}", e)), ctx.sync.value()),
            action_status("Reload", ctx.reload.loading(), ctx.reload.error().map(|e| format!("{:?}", e)), ctx.reload.value()),
            action_status("Save", ctx.save.loading(), ctx.save.error().map(|e| format!("{:?}", e)), ctx.save.value()),
            action_status("Apply draft", ctx.apply_config.loading(), ctx.apply_config.error().map(|e| format!("{:?}", e)), ctx.apply_config.value()),
            action_status("Stop", ctx.stop.loading(), ctx.stop.error().map(|e| format!("{:?}", e)), ctx.stop.value()),
        ]
        .class("card panel stack"),
        div![
            h3("Notes"),
            p("The command panel above stays in sync with mutation state and the snapshot refresh loop. It gives you a compact history of what the dashboard asked the service to do.")
                .class("page-intro"),
            p("If you reload the page, the draft config and the theme preference survive because both are stored locally in the browser.")
                .class("page-intro"),
        ]
        .class("card panel stack"),
    ]
    .class("grid double")
}

pub fn action_status(
    label: &'static str,
    loading: bool,
    error: Option<String>,
    value: Option<String>,
) -> AnyView {
    let status = if loading {
        "pending".to_string()
    } else if let Some(err) = error {
        format!("error: {err}")
    } else if let Some(value) = value {
        value
    } else {
        "idle".to_string()
    };

    let mut view = div![span(label).class("badge"), span(status),].class("banner");
    if loading {
        view = view.class("pulse");
    }
    view.into_any()
}

pub fn metric_card(label: &'static str, value: String, subtitle: String) -> AnyView {
    div![
        div(label).class("metric-label"),
        div(value).class("metric-value"),
        div(subtitle).class("metric-subtitle"),
    ]
    .class("card metric")
    .into_any()
}

pub fn loading_metric(label: &'static str, value: &'static str, subtitle: &'static str) -> AnyView {
    div![
        div(label).class("metric-label"),
        div(value).class("metric-value"),
        div(subtitle).class("metric-subtitle"),
    ]
    .class("card metric")
    .into_any()
}

pub fn enabled_hosts(config: &AppConfig) -> usize {
    config.hosts.values().filter(|host| host.enabled).count()
}

pub fn request_type_label(request_type: u8) -> &'static str {
    match request_type {
        1 => "HTTP",
        2 => "HTTPS",
        _ => "NTP",
    }
}

pub fn sync_mode_label(sync_mode: u8) -> &'static str {
    match sync_mode {
        0 => "Automatic",
        1 => "Force",
        2 => "Precise",
        3 => "Legacy",
        _ => "Custom",
    }
}

pub fn agreement_label(agreement: u8) -> &'static str {
    match agreement {
        0 => "NTP only",
        1 => "HTTP only",
        _ => "Mixed",
    }
}

use crate::service::*;
use silex::prelude::*;

// --- Styled Components ---

styled! {
    pub StyledMetricCard<div>(children: Children) {
        padding: 32px;
        background: $AppTheme::BG_PANEL;
        border: 1px solid $AppTheme::LINE;
        border-radius: $AppTheme::RADIUS;
        box-shadow: $AppTheme::SHADOW;
        backdrop-filter: blur(24px);
        transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
        position: relative;
        overflow: hidden;

        &::after {
            content: "";
            position: absolute;
            top: -20px; right: -20px; width: 100px; height: 100px;
            background: radial-gradient(circle at center, $AppTheme::ACCENT, transparent 70%);
            opacity: 0.1;
            filter: blur(20px);
            transition: opacity 0.4s;
        }

        &:hover {
            transform: translateY(-6px);
            border-color: $(AppTheme::ACCENT.alpha(0.4));
            background: $AppTheme::BG_PANEL;
            box-shadow: 0 20px 40px rgba(0, 0, 0, 0.25);
        }

        &:hover::after {
            opacity: 0.25;
        }
    }
}

styled! {
    pub StyledPanel<div>(children: Children) {
        background: $AppTheme::BG_PANEL;
        border: 1px solid $AppTheme::LINE;
        border-radius: $AppTheme::RADIUS;
        box-shadow: $AppTheme::SHADOW;
        backdrop-filter: blur(24px);
        padding: 32px;
        display: flex;
        flex-direction: column;
        gap: 24px;
        transition: all 0.3s;

        &:focus-within {
            border-color: $(AppTheme::ACCENT.alpha(0.3));
            background: $AppTheme::BG_PANEL;
        }
    }
}

styled! {
    pub StyledKVRow<div>(children: Children) {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 20px;
        padding: 14px 18px;
        border-radius: 18px;
        border: 1px solid transparent;
        background: $(AppTheme::TEXT.alpha(0.05));
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);

        &:hover {
            background: $(AppTheme::TEXT.alpha(0.08));
            border-color: $(AppTheme::ACCENT.alpha(0.2));
            transform: translateX(2px);
        }
    }
}

styled! {
    pub StyledBadge<span>(children: Children) {
        display: inline-flex;
        align-items: center;
        padding: 6px 12px;
        border-radius: 999px;
        background: $(AppTheme::ACCENT.alpha(0.1));
        border: 1px solid $(AppTheme::ACCENT.alpha(0.2));
        color: $AppTheme::ACCENT;
        font-size: 11px;
        font-weight: 800;
        text-transform: uppercase;
        letter-spacing: 0.08 em;
        backdrop-filter: blur(8px);
    }
}

styled! {
    pub StyledToolButton<button>(
        children: Children,
        #[prop(into)] danger: Signal<bool>,
    ) {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: 10px;
        appearance: none;
        border: 1px solid $AppTheme::LINE;
        background: $(AppTheme::TEXT.alpha(0.05));
        color: $AppTheme::TEXT;
        border-radius: 16px;
        padding: 12px 24px;
        font-size: 14px;
        font-weight: 700;
        cursor: pointer;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        backdrop-filter: blur(8px);

        &:hover:not(:disabled) {
            transform: translateY(-2px);
            border-color: $(rx!(if danger.get() { AppTheme::DANGER } else { AppTheme::ACCENT }));
            background: $(rx!(if danger.get() { AppTheme::DANGER.alpha(0.1) } else { AppTheme::ACCENT.alpha(0.1) }));
            box-shadow: 0 8px 20px rgba(0, 0, 0, 0.15);
            color: $(rx!(if danger.get() { AppTheme::DANGER } else { AppTheme::ACCENT }));
        }

        &:active:not(:disabled) { transform: translateY(0); }
        &:disabled { opacity: 0.4; cursor: not-allowed; }
    }
}

// --- Views ---

fn time_span(ms: u64) -> impl View {
    span(format_ms_adaptive(ms)).attr("title", format!("{}ms", ms))
}

fn kv_row(label: &'static str, value: impl View + 'static) -> AnyView {
    StyledKVRow(view_chain![
        span(label).style(
            sty()
                .color(AppTheme::MUTED)
                .text_transform(TextTransformKeyword::Uppercase)
                .letter_spacing(px(1.5))
                .font_size(px(10))
                .font_weight(850)
                .opacity(0.8)
        ),
        span(value).style(
            sty()
                .color(AppTheme::TEXT)
                .text_align(TextAlignKeyword::Right)
                .font_weight(600)
                .font_size(px(14))
                .word_break(WordBreakKeyword::BreakAll)
        )
    ])
    .into_any()
}

fn icon_overview() -> SharedView {
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
    .into_shared()
}

fn icon_config() -> SharedView {
    svg(view_chain![
        path().attr("d", "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.1a2 2 0 0 1-1-1.72v-.51a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"),
        circle().attr("cx", "12").attr("cy", "12").attr("r", "3")
    ])
    .attr("width", "16").attr("height", "16").attr("viewBox", "0 0 24 24").attr("fill", "none")
    .attr("stroke", "currentColor").attr("stroke-width", "2.5").attr("stroke-linecap", "round").attr("stroke-linejoin", "round")
    .into_shared()
}

fn icon_logs() -> SharedView {
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
    .into_shared()
}

fn icon_about() -> SharedView {
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
    .into_shared()
}

fn icon_sync() -> SharedView {
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
    .into_shared()
}

fn icon_save() -> SharedView {
    svg(view_chain![
        path().attr("d", "M15.2 3a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z"),
        path().attr("d", "M17 21v-7a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v7"),
        path().attr("d", "M7 3v4a1 1 0 0 0 1 1h7")
    ])
    .attr("width", "15").attr("height", "15").attr("viewBox", "0 0 24 24").attr("fill", "none")
    .attr("stroke", "currentColor").attr("stroke-width", "2.5").attr("stroke-linecap", "round").attr("stroke-linejoin", "round")
    .into_shared()
}

fn icon_stop() -> SharedView {
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
    .into_shared()
}

fn icon_reload() -> SharedView {
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
    .into_shared()
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
            StyledToolButton(ctx.theme_name.map_fn(|theme| if theme == "dark" {
                "Switch to Light"
            } else {
                "Switch to Dark"
            }))
            .on(event::click, move |_| {
                ctx.theme_name.update(|theme| {
                    *theme = if theme == "dark" {
                        "light".to_string()
                    } else {
                        "dark".to_string()
                    };
                });
            })
            .danger(false),
            StyledToolButton(ctx.auto_refresh.map_fn(|enabled| if *enabled {
                "Pause Refresh"
            } else {
                "Resume Refresh"
            }))
            .on(event::click, move |_| {
                ctx.auto_refresh.update(|enabled| *enabled = !*enabled);
            })
            .danger(false),
            StyledToolButton("Refresh now")
                .on(event::click, move |_| ctx.snapshot.refetch())
                .danger(false),
        ]
        .style(
            sty()
                .display(DisplayKeyword::Flex)
                .flex_direction(FlexDirectionKeyword::Column)
                .gap(px(12))
                .margin_top(px(20))
                .padding_top(px(20))
                .border_top(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
        ),
    ]
    .class("sidebar")
}

#[component]
pub fn OverviewPage() -> impl View {
    let ctx = use_dashboard();

    div![
        div![
            div![
                h2("Overview").style(sty().margin(px(0)).font_size(px(40)).font_weight(850).letter_spacing(px(-1.5)).line_height(1)),
                p("Live service status, control shortcuts, and a quick glance at the current runtime state.")
                    .style(sty().color(AppTheme::MUTED).margin(px(0)).font_size(px(18)).font_weight(500).opacity(0.8)),
            ],
            div![
                StyledToolButton(view_chain![icon_sync(), "Sync now"])
                    .on(event::click, move |_| ctx.sync.mutate(()))
                    .attr("disabled", rx!(@fn ctx.sync.loading()))
                    .danger(false),
                StyledToolButton(view_chain![icon_reload(), "Reload config"])
                    .on(event::click, move |_| ctx.reload.mutate(()))
                    .attr("disabled", rx!(@fn ctx.reload.loading()))
                    .danger(false),
                StyledToolButton(view_chain![icon_save(), "Save config"])
                    .on(event::click, move |_| ctx.save.mutate(()))
                    .attr("disabled", rx!(@fn ctx.save.loading()))
                    .danger(false),
                StyledToolButton(view_chain![icon_stop(), "Stop service"])
                    .on(event::click, move |_| ctx.stop.mutate(()))
                    .attr("disabled", rx!(@fn ctx.stop.loading()))
                    .danger(true),
            ]
            .style(sty().display(DisplayKeyword::Flex).flex_wrap(FlexWrapKeyword::Wrap).gap(px(12))),
        ]
        .style(sty().display(DisplayKeyword::Flex).justify_content(JustifyContentKeyword::SpaceBetween).align_items(AlignItemsKeyword::FlexStart).gap(px(12)).flex_wrap(FlexWrapKeyword::Wrap).margin_bottom(px(8))),
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
            h2("Config").style(sty().margin(px(0)).font_size(px(32)).font_weight(800).letter_spacing(px(-1))),
            p("Edit the live TOML config, keep a local draft, and push it back to the service without leaving the dashboard.")
                .style(sty().color(AppTheme::MUTED).margin(px(0)).font_size(px(16))),
        ].style(sty().margin_bottom(px(24))),
        div![
        StyledPanel(view_chain![
            div![
                div![
                    h3("TOML Draft").style(sty().margin(px(0))),
                    StyledBadge("Stored locally"),
                ]
                .style(sty().display(DisplayKeyword::Flex).justify_content(JustifyContentKeyword::SpaceBetween).align_items(AlignItemsKeyword::FlexStart).gap(px(12)).flex_wrap(FlexWrapKeyword::Wrap)),
                textarea("")
                    .bind_value(ctx.config_draft)
                    .class("editor"),
                div![
                    StyledToolButton(view_chain![icon_reload(), "Load live config"])
                        .on(event::click, move |_| {
                            if let Some(snapshot) = ctx.snapshot.get_data()
                                && let Ok(toml) = toml::to_string_pretty(&snapshot.config)
                            {
                                ctx.config_draft.set(toml);
                            }
                        })
                        .danger(false),
                    StyledToolButton(view_chain![icon_save(), "Apply draft"])
                        .on(event::click, move |_| ctx.apply_config.mutate(ctx.config_draft.get()))
                        .attr("disabled", rx!(@fn ctx.apply_config.loading()))
                        .danger(false),
                ]
                .style(sty().display(DisplayKeyword::Flex).flex_wrap(FlexWrapKeyword::Wrap).gap(px(12))),
                move || {
                    if let Some(err) = ctx.apply_config.error() {
                        div(format!("Draft error: {:?}", err))
                            .style(sty().padding(px(16)).border_radius(px(14)).background(AppTheme::DANGER.alpha(0.05)).border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE)).font_size(px(14)).line_height(1.6).color(AppTheme::DANGER))
                            .into_any()
                    } else if ctx.apply_config.loading() {
                        div("Applying draft to the service...")
                            .style(sty().padding(px(16)).border_radius(px(14)).background(AppTheme::WARNING.alpha(0.05)).border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE)).font_size(px(14)).line_height(1.6).color(AppTheme::WARNING))
                            .into_any()
                    } else if let Some(msg) = ctx.apply_config.value() {
                        div(format!("Last draft update: {msg}"))
                            .style(sty().padding(px(16)).border_radius(px(14)).background(AppTheme::SUCCESS.alpha(0.05)).border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE)).font_size(px(14)).line_height(1.6).color(AppTheme::SUCCESS))
                            .into_any()
                    } else {
                        "".into_any()
                    }
                }
            ]
            .class("stack")
        ]),

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
            h2("Logs").style(sty().margin(px(0)).font_size(px(32)).font_weight(800).letter_spacing(px(-1))),
            p("The last lines from the service runtime, plus the current host matrix for easy inspection.")
                .style(sty().color(AppTheme::MUTED).margin(px(0)).font_size(px(16))),
        ].style(sty().margin_bottom(px(24))),
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
        h2("About").style(sty().margin(px(0)).font_size(px(32)).font_weight(800).letter_spacing(px(-1))),
        p("This front end is rendered with Silex in WASM and shipped inside the service binary with rust-embed.")
            .style(sty().color(AppTheme::MUTED).margin(px(0)).font_size(px(16))),
        div![
            StyledPanel(view_chain![
                h3("What it does").style(sty().margin(px(0))),
                ul![
                    li("Shows the live runtime snapshot from the service."),
                    li("Lets you sync, reload, save, and stop the service."),
                    li("Keeps a persistent TOML draft in the browser."),
                    li("Offers automatic refresh so you can watch state changes happen."),
                ]
            ]),
            StyledPanel(view_chain![
                h3("Operational notes").style(sty().margin(px(0))),
                ul![
                    li("The SPA fallback keeps client-side routes working on refresh."),
                    li("Config changes are sent as JSON to /api/config."),
                    li("The app theme and refresh preference are persisted locally."),
                ]
            ]),
        ]
        .class("grid double"),
    ]
    .class("page")
}

#[component]
pub fn NotFoundPage() -> impl View {
    div![
        h2("404").style(
            sty()
                .margin(px(0))
                .font_size(px(32))
                .font_weight(800)
                .letter_spacing(px(-1))
        ),
        p("The route was not found. Use the sidebar to jump back into the dashboard.")
            .style(sty().color(AppTheme::MUTED).margin(px(0)).font_size(px(16))),
    ]
    .class("page")
}

#[component]
fn SummaryPanel() -> impl View {
    let ctx = use_dashboard();

    move || {
        match ctx.snapshot.state.get() {
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
                time_span(snapshot.config.delay_ms),
                span!["Timeout: ", time_span(snapshot.config.network_timeout_ms)],
            ),
        ]
        .class("grid metrics")
        .into_any(),
        ResourceState::Error(err) => div![
            span("Snapshot error").style(sty().color(AppTheme::MUTED).text_transform(TextTransformKeyword::Uppercase).letter_spacing(px(1)).font_size(px(11)).font_weight(800)),
            div(format!("Failed: {:?}", err)).style(sty().margin_top(px(12)).font_size(px(32)).font_weight(850).line_height(1).letter_spacing(px(-0.5)).color(AppTheme::DANGER)),
            p("The backend is reachable, but the dashboard could not deserialize a state snapshot.")
                .style(sty().margin_top(px(8)).color(AppTheme::MUTED).font_size(px(14))),
        ]
        .style(sty().padding(px(24)).background(AppTheme::BG_PANEL).border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE)).border_radius(AppTheme::RADIUS).box_shadow(AppTheme::SHADOW))
        .into_any(),
    }
    }
}

#[component]
fn ServicePanel() -> impl View {
    let ctx = use_dashboard();

    StyledPanel(view_chain![
        div![
            div![
                h3("Live Snapshot").style(sty().margin(px(0))),
                StyledBadge("auto-updated")
            ]
            .style(
                sty()
                    .display(DisplayKeyword::Flex)
                    .justify_content(JustifyContentKeyword::SpaceBetween)
                    .align_items(AlignItemsKeyword::FlexStart)
                    .gap(px(12))
                    .flex_wrap(FlexWrapKeyword::Wrap)
            ),
            move || match ctx.snapshot.state.get() {
                ResourceState::Idle | ResourceState::Loading =>
                    div("Waiting for the next service snapshot...")
                        .style(
                            sty()
                                .padding(px(16))
                                .border_radius(px(14))
                                .background(rgba(255, 255, 255, 0.03))
                                .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                                .font_size(px(14))
                                .line_height(1.6)
                        )
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
                            span![
                                time_span(snapshot.config.offset_ms),
                                " / ",
                                time_span(snapshot.config.deviation_offset_ms)
                            ],
                        ),
                        kv_row("Sync interval", time_span(snapshot.config.delay_ms)),
                        kv_row(
                            "Network timeout",
                            time_span(snapshot.config.network_timeout_ms)
                        ),
                    ]
                    .style(
                        sty()
                            .display(DisplayKeyword::Grid)
                            .gap(px(14))
                    ),
                ]
                .into_any(),
                ResourceState::Error(err) => div![
                    div("Snapshot error").style(
                        sty()
                            .color(AppTheme::MUTED)
                            .text_transform(TextTransformKeyword::Uppercase)
                            .letter_spacing(px(1))
                            .font_size(px(11))
                            .font_weight(800)
                    ),
                    div(format!("Failed to deserialize state snapshot: {:?}", err)).style(
                        sty()
                            .padding(px(16))
                            .border_radius(px(14))
                            .background(AppTheme::DANGER.alpha(0.03))
                            .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                            .font_size(px(14))
                            .line_height(1.6)
                            .color(AppTheme::DANGER)
                            .margin_top(px(8))
                    ),
                ]
                .class("stack")
                .into_any(),
            }
        ].style(sty().display(DisplayKeyword::Flex).flex_direction(FlexDirectionKeyword::Column).gap(px(20))),
        div![
            div![
                h3("Actions").style(sty().margin(px(0))),
                StyledBadge("service commands")
            ]
            .style(
                sty()
                    .display(DisplayKeyword::Flex)
                    .justify_content(JustifyContentKeyword::SpaceBetween)
                    .align_items(AlignItemsKeyword::FlexStart)
                    .gap(px(12))
                    .flex_wrap(FlexWrapKeyword::Wrap)
            ),
            div![
                StyledToolButton(view_chain![icon_sync(), "Sync"])
                    .on(event::click, move |_| ctx.sync.mutate(()))
                    .attr("disabled", rx!(@fn ctx.sync.loading()))
                    .danger(false),
                StyledToolButton(view_chain![icon_reload(), "Reload"])
                    .on(event::click, move |_| ctx.reload.mutate(()))
                    .attr("disabled", rx!(@fn ctx.reload.loading()))
                    .danger(false),
                StyledToolButton(view_chain![icon_save(), "Save"])
                    .on(event::click, move |_| ctx.save.mutate(()))
                    .attr("disabled", rx!(@fn ctx.save.loading()))
                    .danger(false),
                StyledToolButton(view_chain![icon_stop(), "Stop"])
                    .on(event::click, move |_| ctx.stop.mutate(()))
                    .attr("disabled", rx!(@fn ctx.stop.loading()))
                    .danger(true),
            ]
            .style(
                sty()
                    .display(DisplayKeyword::Flex)
                    .flex_wrap(FlexWrapKeyword::Wrap)
                    .gap(px(16))
            ),
            move || {
                let mut messages = Vec::new();
                if let Some(msg) = ctx.sync.value() {
                    messages.push(("Sync", msg, AppTheme::SUCCESS));
                }
                if let Some(msg) = ctx.reload.value() {
                    messages.push(("Reload", msg, AppTheme::SUCCESS));
                }
                if let Some(msg) = ctx.save.value() {
                    messages.push(("Save", msg, AppTheme::SUCCESS));
                }
                if let Some(msg) = ctx.stop.value() {
                    messages.push(("Stop", msg, AppTheme::WARNING));
                }

                if messages.is_empty() {
                    div("No action has completed yet.")
                        .style(
                            sty()
                                .padding(px(16))
                                .border_radius(px(14))
                                .background(rgba(255, 255, 255, 0.03))
                                .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                                .font_size(px(14))
                                .line_height(1.6),
                        )
                        .into_any()
                } else {
                    div(messages
                        .into_iter()
                        .map(|(label, msg, _)| format!("{label}: {msg}"))
                        .collect::<Vec<_>>()
                        .join("\n"))
                    .style(
                        sty()
                            .padding(px(16))
                            .border_radius(px(14))
                            .background(rgba(255, 255, 255, 0.03))
                            .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                            .font_size(px(14))
                            .line_height(1.6)
                            .white_space(WhiteSpaceKeyword::PreWrap),
                    )
                    .into_any()
                }
            }
        ].style(sty().display(DisplayKeyword::Flex).flex_direction(FlexDirectionKeyword::Column).gap(px(20)))
    ])
}

#[component]
fn SnapshotQuickFacts() -> impl View {
    let ctx = use_dashboard();

    StyledPanel(view_chain![
        div![
            h3("Quick Facts").style(sty().margin(px(0))),
            StyledBadge("latest state")
        ]
        .style(
            sty()
                .display(DisplayKeyword::Flex)
                .justify_content(JustifyContentKeyword::SpaceBetween)
                .align_items(AlignItemsKeyword::FlexStart)
                .gap(px(12))
                .flex_wrap(FlexWrapKeyword::Wrap)
        ),
        move || match ctx.snapshot.state.get() {
            ResourceState::Idle | ResourceState::Loading => div("Waiting for snapshot...")
                .style(
                    sty()
                        .padding(px(16))
                        .border_radius(px(14))
                        .background(rgba(255, 255, 255, 0.03))
                        .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                        .font_size(px(14))
                        .line_height(1.6)
                )
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
            .style(sty().display(DisplayKeyword::Grid).gap(px(14)))
            .into_any(),
            ResourceState::Error(err) => div(format!("Snapshot error: {:?}", err))
                .style(
                    sty()
                        .padding(px(16))
                        .border_radius(px(14))
                        .background(AppTheme::DANGER.alpha(0.03))
                        .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                        .font_size(px(14))
                        .line_height(1.6)
                        .color(AppTheme::DANGER)
                )
                .into_any(),
        }
    ])
}

#[component]
fn SyncTuningPanel() -> impl View {
    let ctx = use_dashboard();

    StyledPanel(view_chain![
        div![
            h3("Sync tuning").style(sty().margin(px(0))),
            StyledBadge("config hints")
        ]
        .style(
            sty()
                .display(DisplayKeyword::Flex)
                .justify_content(JustifyContentKeyword::SpaceBetween)
                .align_items(AlignItemsKeyword::FlexStart)
                .gap(px(12))
                .flex_wrap(FlexWrapKeyword::Wrap)
        ),
        move || match ctx.snapshot.state.get() {
            ResourceState::Idle | ResourceState::Loading => div("Waiting for current config...")
                .style(
                    sty()
                        .padding(px(16))
                        .border_radius(px(14))
                        .background(rgba(255, 255, 255, 0.03))
                        .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                        .font_size(px(14))
                        .line_height(1.6)
                )
                .into_any(),
            ResourceState::Ready(snapshot) | ResourceState::Reloading(snapshot) => div![
                kv_row(
                    "Agreement mode",
                    agreement_label(snapshot.config.agreement).to_string()
                ),
                kv_row("Window", time_span(snapshot.config.timeout_ms)),
                kv_row(
                    "Win32 time policy",
                    if snapshot.config.disable_win32_time {
                        "disabled".to_string()
                    } else {
                        "enabled".to_string()
                    },
                ),
            ]
            .style(sty().display(DisplayKeyword::Grid).gap(px(14)))
            .into_any(),
            ResourceState::Error(err) => div(format!("Unable to inspect config: {:?}", err))
                .style(
                    sty()
                        .padding(px(16))
                        .border_radius(px(14))
                        .background(AppTheme::DANGER.alpha(0.03))
                        .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                        .font_size(px(14))
                        .line_height(1.6)
                        .color(AppTheme::DANGER)
                )
                .into_any(),
        }
    ])
}

#[component]
fn HostSummaryPanel() -> impl View {
    let ctx = use_dashboard();

    StyledPanel(view_chain![
        div![
            h3("Host summary").style(sty().margin(px(0))),
            StyledBadge("topology")
        ]
        .style(
            sty()
                .display(DisplayKeyword::Flex)
                .justify_content(JustifyContentKeyword::SpaceBetween)
                .align_items(AlignItemsKeyword::FlexStart)
                .gap(px(12))
                .flex_wrap(FlexWrapKeyword::Wrap)
        ),
        move || match ctx.snapshot.state.get() {
            ResourceState::Idle | ResourceState::Loading => div("Loading host list...")
                .style(
                    sty()
                        .padding(px(16))
                        .border_radius(px(14))
                        .background(rgba(255, 255, 255, 0.03))
                        .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                        .font_size(px(14))
                        .line_height(1.6)
                )
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
                        .style(
                            sty()
                                .padding(px(16))
                                .border_radius(px(14))
                                .background(rgba(255, 255, 255, 0.03))
                                .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                                .font_size(px(14))
                                .line_height(1.6),
                        )
                        .into_any()
                } else {
                    div(items.join("\n")).class("editor").style(sty().min_height(px(0)).max_height(px(400)).overflow_y(OverflowYKeyword::Auto).white_space(WhiteSpaceKeyword::PreWrap)).into_any()
                }
            }
            ResourceState::Error(err) => div(format!("Host list error: {:?}", err))
                .style(
                    sty()
                        .padding(px(16))
                        .border_radius(px(14))
                        .background(AppTheme::DANGER.alpha(0.03))
                        .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                        .font_size(px(14))
                        .line_height(1.6)
                        .color(AppTheme::DANGER)
                )
                .into_any(),
        }
    ])
}

#[component]
fn HostTablePanel() -> impl View {
    let ctx = use_dashboard();

    StyledPanel(view_chain![
        div![
            h3("Hosts").style(sty().margin(px(0))),
            StyledBadge("current matrix")
        ]
        .style(
            sty()
                .display(DisplayKeyword::Flex)
                .justify_content(JustifyContentKeyword::SpaceBetween)
                .align_items(AlignItemsKeyword::FlexStart)
                .gap(px(12))
                .flex_wrap(FlexWrapKeyword::Wrap)
        ),
        move || match ctx.snapshot.state.get() {
            ResourceState::Idle | ResourceState::Loading => div("Loading hosts...")
                .style(
                    sty()
                        .padding(px(16))
                        .border_radius(px(14))
                        .background(rgba(255, 255, 255, 0.03))
                        .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                        .font_size(px(14))
                        .line_height(1.6)
                )
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
                        .style(
                            sty()
                                .padding(px(16))
                                .border_radius(px(14))
                                .background(rgba(255, 255, 255, 0.03))
                                .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                                .font_size(px(14))
                                .line_height(1.6),
                        )
                        .into_any()
                } else {
                    div(rows.join("\n")).class("editor").style(sty().min_height(px(0)).max_height(px(400)).overflow_y(OverflowYKeyword::Auto).white_space(WhiteSpaceKeyword::PreWrap)).into_any()
                }
            }
            ResourceState::Error(err) => div(format!("Host table error: {:?}", err))
                .style(
                    sty()
                        .padding(px(16))
                        .border_radius(px(14))
                        .background(AppTheme::DANGER.alpha(0.03))
                        .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                        .font_size(px(14))
                        .line_height(1.6)
                        .color(AppTheme::DANGER)
                )
                .into_any(),
        }
    ])
}

#[component]
pub fn RecentLogsPanel(limit: usize) -> impl View {
    let ctx = use_dashboard();

    StyledPanel(view_chain![
        div![
            h3("Logs").style(sty().margin(px(0))),
            StyledBadge(format!("last {limit} lines")),
        ]
        .style(
            sty()
                .display(DisplayKeyword::Flex)
                .justify_content(JustifyContentKeyword::SpaceBetween)
                .align_items(AlignItemsKeyword::FlexStart)
                .gap(px(12))
                .flex_wrap(FlexWrapKeyword::Wrap)
        ),
        move || match ctx.snapshot.state.get() {
            ResourceState::Idle | ResourceState::Loading => div("Loading logs...")
                .style(
                    sty()
                        .padding(px(16))
                        .border_radius(px(14))
                        .background(rgba(255, 255, 255, 0.03))
                        .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                        .font_size(px(14))
                        .line_height(1.6)
                )
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
                        .style(
                            sty()
                                .padding(px(16))
                                .border_radius(px(14))
                                .background(rgba(255, 255, 255, 0.03))
                                .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                                .font_size(px(14))
                                .line_height(1.6),
                        )
                        .into_any()
                } else {
                    div![
                        div(logs.join("\n")).style(
                            sty()
                                .padding(px(12))
                                .border_bottom(border(
                                    px(1),
                                    BorderStyleKeyword::Solid,
                                    AppTheme::LINE
                                ))
                                .font_family("'JetBrains Mono', monospace")
                                .font_size(px(12))
                                .line_height(1.5)
                                .white_space(WhiteSpaceKeyword::PreWrap)
                                .word_break(WordBreakKeyword::BreakAll)
                        )
                    ]
                    .style(sty().max_height(px(600)).overflow_y(OverflowYKeyword::Auto))
                    .into_any()
                }
            }
            ResourceState::Error(err) => div(format!("Log error: {:?}", err))
                .style(
                    sty()
                        .padding(px(16))
                        .border_radius(px(14))
                        .background(rgba(239, 68, 68, 0.03))
                        .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                        .font_size(px(14))
                        .line_height(1.6)
                        .color(AppTheme::DANGER)
                )
                .into_any(),
        }
    ])
}

#[component]
fn MutationLedger() -> impl View {
    let ctx = use_dashboard();

    div![
        StyledPanel(div![
            h3("Command state").style(sty().margin(px(0))),
            action_status("Sync", ctx.sync.loading(), ctx.sync.error().map(|e| format!("{:?}", e)), ctx.sync.value()),
            action_status("Reload", ctx.reload.loading(), ctx.reload.error().map(|e| format!("{:?}", e)), ctx.reload.value()),
            action_status("Save", ctx.save.loading(), ctx.save.error().map(|e| format!("{:?}", e)), ctx.save.value()),
            action_status("Apply draft", ctx.apply_config.loading(), ctx.apply_config.error().map(|e| format!("{:?}", e)), ctx.apply_config.value()),
            action_status("Stop", ctx.stop.loading(), ctx.stop.error().map(|e| format!("{:?}", e)), ctx.stop.value()),
        ].style(sty().display(DisplayKeyword::Flex).flex_direction(FlexDirectionKeyword::Column).gap(px(16)))),
        StyledPanel(div![
            h3("Notes").style(sty().margin(px(0))),
            p("The command panel above stays in sync with mutation state and the snapshot refresh loop. It gives you a compact history of what the dashboard asked the service to do.")
                .style(sty().color(AppTheme::MUTED).font_size(px(16))),
            p("If you reload the page, the draft config and the theme preference survive because both are stored locally in the browser.")
                .style(sty().color(AppTheme::MUTED).font_size(px(16))),
        ].style(sty().display(DisplayKeyword::Flex).flex_direction(FlexDirectionKeyword::Column).gap(px(16))))
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

    let view = div![StyledBadge(label), span(status),].style(
        sty()
            .padding(px(16))
            .border_radius(px(14))
            .background(rgba(255, 255, 255, 0.03))
            .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
            .font_size(px(14))
            .line_height(1.6)
            .display(DisplayKeyword::Flex)
            .justify_content(JustifyContentKeyword::SpaceBetween),
    );
    if loading {
        view.class("pulse").into_any()
    } else {
        view.into_any()
    }
}

pub fn metric_card(
    label: &'static str,
    value: impl View + 'static,
    subtitle: impl View + 'static,
) -> AnyView {
    StyledMetricCard(view_chain![
        div(label).style(
            sty()
                .color(AppTheme::MUTED)
                .text_transform(TextTransformKeyword::Uppercase)
                .letter_spacing(px(1.5))
                .font_size(px(10))
                .font_weight(850)
                .opacity(0.8)
        ),
        div(value).style(
            sty()
                .margin_top(px(16))
                .font_size(px(36))
                .font_weight(850)
                .line_height(1)
                .letter_spacing(px(-1.5))
                .color(AppTheme::TEXT)
        ),
        div(subtitle).style(
            sty()
                .margin_top(px(10))
                .color(AppTheme::MUTED)
                .font_size(px(14))
                .font_weight(500)
                .opacity(0.7)
                .word_break(WordBreakKeyword::BreakAll)
        ),
    ])
    .into_any()
}

pub fn loading_metric(label: &'static str, value: &'static str, subtitle: &'static str) -> AnyView {
    StyledMetricCard(view_chain![
        div(label).style(
            sty()
                .color(AppTheme::MUTED)
                .text_transform(TextTransformKeyword::Uppercase)
                .letter_spacing(px(1.5))
                .font_size(px(10))
                .font_weight(850)
                .opacity(0.8)
        ),
        div(value).style(
            sty()
                .margin_top(px(16))
                .font_size(px(36))
                .font_weight(850)
                .line_height(1)
                .letter_spacing(px(-1.5))
                .color(AppTheme::TEXT)
                .opacity(0.5)
        ),
        div(subtitle).style(
            sty()
                .margin_top(px(10))
                .color(AppTheme::MUTED)
                .font_size(px(14))
                .font_weight(500)
                .opacity(0.7)
                .word_break(WordBreakKeyword::BreakAll)
        ),
    ])
    .class("pulse")
    .into_any()
}

pub fn enabled_hosts(config: &AppConfig) -> usize {
    config.hosts.values().filter(|host| host.enabled).count()
}

pub fn request_type_label(request_type: RequestType) -> &'static str {
    match request_type {
        RequestType::Http => "HTTP",
        RequestType::Https => "HTTPS",
        RequestType::Ntp => "NTP",
    }
}

pub fn sync_mode_label(sync_mode: SyncMode) -> &'static str {
    match sync_mode {
        SyncMode::Off => "Off",
        SyncMode::Immediate => "Immediate (Step)",
        SyncMode::Slew => "Gradual (Slew)",
    }
}

pub fn agreement_label(agreement: Agreement) -> &'static str {
    match agreement {
        Agreement::NtpOnly => "NTP only",
        Agreement::HttpOnly => "HTTP only",
        Agreement::Mixed => "Mixed",
    }
}

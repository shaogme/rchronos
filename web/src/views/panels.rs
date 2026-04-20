use super::helpers::*;
use crate::service::*;
use silex::prelude::*;

#[component]
pub fn SummaryPanel() -> impl View {
    let ctx = use_dashboard();

    move || {
        match ctx.snapshot.state.get() {
            ResourceState::Idle | ResourceState::Loading => div![loading_metric(
                "Status",
                "Loading...",
                "Fetching runtime snapshot"
            ),]
            .class("grid metrics"),
            ResourceState::Ready(snapshot) | ResourceState::Reloading(snapshot) => div![
                MetricCard().label("Status").value(snapshot.status.current_operation.clone()).subtitle(format!("Config: {}", snapshot.config_path)),
                MetricCard().label("Health").value(format!("{}/{} OK", enabled_hosts(&snapshot.config) - failing_hosts(&snapshot), snapshot.config.hosts.len())).subtitle(format!("Syncing: {}", if snapshot.syncing { "yes" } else { "no" })),
                MetricCard().label("Enabled hosts").value(format!("{}/{}", enabled_hosts(&snapshot.config), snapshot.config.hosts.len())).subtitle(sync_mode_label(snapshot.config.sync_mode).to_string()),
                MetricCard().label("Delay").value(TimeSpan().ms(snapshot.config.delay_ms)).subtitle(span!["Timeout: ", TimeSpan().ms(snapshot.config.network_timeout_ms)]),
            ]
            .class("grid metrics"),
            ResourceState::Error(err) => div![
                span("Snapshot error").style(sty().color(AppTheme::MUTED).text_transform(TextTransformKeyword::Uppercase).letter_spacing(px(1)).font_size(px(11)).font_weight(800)),
                div(format!("Failed: {:?}", err)).style(sty().margin_top(px(12)).font_size(px(32)).font_weight(850).line_height(1).letter_spacing(px(-0.5)).color(AppTheme::DANGER)),
                p("The backend is reachable, but the dashboard could not deserialize a state snapshot.")
                    .style(sty().margin_top(px(8)).color(AppTheme::MUTED).font_size(px(14))),
            ]
            .style(sty().padding(px(24)).background(AppTheme::BG_PANEL).border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE)).border_radius(AppTheme::RADIUS).box_shadow(AppTheme::SHADOW)),
        }
    }
}

#[component]
pub fn ServicePanel() -> impl View {
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
                    div("Waiting for the next service snapshot...").style(
                        sty()
                            .padding(px(16))
                            .border_radius(px(14))
                            .background(rgba(255, 255, 255, 0.03))
                            .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                            .font_size(px(14))
                            .line_height(1.6)
                    ),
                ResourceState::Ready(snapshot) | ResourceState::Reloading(snapshot) => div![
                    div![
                        kv_row(
                            "Mode",
                            sync_mode_label(snapshot.config.sync_mode).to_string()
                        ),
                        kv_row("Status", snapshot.status.current_operation.clone()),
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
                                TimeSpan().ms(snapshot.config.offset_ms),
                                " / ",
                                TimeSpan().ms(snapshot.config.deviation_offset_ms)
                            ],
                        ),
                        kv_row("Sync interval", TimeSpan().ms(snapshot.config.delay_ms)),
                        kv_row(
                            "Network timeout",
                            TimeSpan().ms(snapshot.config.network_timeout_ms)
                        ),
                    ]
                    .style(sty().display(DisplayKeyword::Grid).gap(px(14))),
                ],
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
                .class("stack"),
            }
        ]
        .style(
            sty()
                .display(DisplayKeyword::Flex)
                .flex_direction(FlexDirectionKeyword::Column)
                .gap(px(20))
        ),
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
                    div("No action has completed yet.").style(
                        sty()
                            .padding(px(16))
                            .border_radius(px(14))
                            .background(rgba(255, 255, 255, 0.03))
                            .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                            .font_size(px(14))
                            .line_height(1.6),
                    )
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
                }
            }
        ]
        .style(
            sty()
                .display(DisplayKeyword::Flex)
                .flex_direction(FlexDirectionKeyword::Column)
                .gap(px(20))
        )
    ])
}

#[component]
pub fn SnapshotQuickFacts() -> impl View {
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
            ResourceState::Idle | ResourceState::Loading => div("Waiting for snapshot...").style(
                sty()
                    .padding(px(16))
                    .border_radius(px(14))
                    .background(rgba(255, 255, 255, 0.03))
                    .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                    .font_size(px(14))
                    .line_height(1.6)
            ),
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
            ]
            .style(sty().display(DisplayKeyword::Grid).gap(px(14))),
            ResourceState::Error(err) => div(format!("Snapshot error: {:?}", err)).style(
                sty()
                    .padding(px(16))
                    .border_radius(px(14))
                    .background(AppTheme::DANGER.alpha(0.03))
                    .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                    .font_size(px(14))
                    .line_height(1.6)
                    .color(AppTheme::DANGER)
            ),
        }
    ])
}

#[component]
pub fn SyncTuningPanel() -> impl View {
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
                ),
            ResourceState::Ready(snapshot) | ResourceState::Reloading(snapshot) => div![
                kv_row(
                    "Agreement mode",
                    agreement_label(snapshot.config.agreement).to_string()
                ),
                kv_row("Window", TimeSpan().ms(snapshot.config.timeout_ms)),
                kv_row(
                    "Win32 time policy",
                    if snapshot.config.disable_win32_time {
                        "disabled".to_string()
                    } else {
                        "enabled".to_string()
                    },
                ),
            ]
            .style(sty().display(DisplayKeyword::Grid).gap(px(14))),
            ResourceState::Error(err) => div(format!("Unable to inspect config: {:?}", err)).style(
                sty()
                    .padding(px(16))
                    .border_radius(px(14))
                    .background(AppTheme::DANGER.alpha(0.03))
                    .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                    .font_size(px(14))
                    .line_height(1.6)
                    .color(AppTheme::DANGER)
            ),
        }
    ])
}

#[component]
pub fn HostSummaryPanel() -> impl View {
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
            ResourceState::Idle | ResourceState::Loading => div("Loading host list...").style(
                sty()
                    .padding(px(16))
                    .border_radius(px(14))
                    .background(rgba(255, 255, 255, 0.03))
                    .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                    .font_size(px(14))
                    .line_height(1.6)
            ),
            ResourceState::Ready(snapshot) | ResourceState::Reloading(snapshot) => {
                let mut items = snapshot
                    .status
                    .hosts
                    .iter()
                    .map(|host| {
                        let kind = request_type_label(host.request_type);
                        let fail_info = if host.fail_count > 0 {
                            format!(" (Fails: {})", host.fail_count)
                        } else {
                            "".to_string()
                        };
                        format!(
                            "{}{} • {} • priority {}",
                            host.name, fail_info, kind, host.priority,
                        )
                    })
                    .collect::<Vec<_>>();
                items.truncate(8);

                if items.is_empty() {
                    div("No hosts configured.").style(
                        sty()
                            .padding(px(16))
                            .border_radius(px(14))
                            .background(rgba(255, 255, 255, 0.03))
                            .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                            .font_size(px(14))
                            .line_height(1.6),
                    )
                } else {
                    div(items.join("\n")).class("editor").style(
                        sty()
                            .min_height(px(0))
                            .max_height(px(400))
                            .overflow_y(OverflowYKeyword::Auto)
                            .white_space(WhiteSpaceKeyword::PreWrap),
                    )
                }
            }
            ResourceState::Error(err) => div(format!("Host list error: {:?}", err)).style(
                sty()
                    .padding(px(16))
                    .border_radius(px(14))
                    .background(AppTheme::DANGER.alpha(0.03))
                    .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                    .font_size(px(14))
                    .line_height(1.6)
                    .color(AppTheme::DANGER)
            ),
        }
    ])
}

#[component]
pub fn HostTablePanel() -> impl View {
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
            ResourceState::Idle | ResourceState::Loading => div("Loading hosts...").style(
                sty()
                    .padding(px(16))
                    .border_radius(px(14))
                    .background(rgba(255, 255, 255, 0.03))
                    .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                    .font_size(px(14))
                    .line_height(1.6)
            ),
            ResourceState::Ready(snapshot) | ResourceState::Reloading(snapshot) => {
                let rows = snapshot
                    .status
                    .hosts
                    .iter()
                    .map(|host| {
                        let status_dot = if host.fail_count > 0 { "❌" } else { "✅" };
                        let err_msg = host
                            .last_error
                            .as_ref()
                            .map(|e| format!(" | Error: {e}"))
                            .unwrap_or_default();

                        format!(
                            "{} {} | {} | priority {} | fails {}{}",
                            status_dot,
                            host.name,
                            request_type_label(host.request_type),
                            host.priority,
                            host.fail_count,
                            err_msg
                        )
                    })
                    .collect::<Vec<_>>();

                if rows.is_empty() {
                    div("No hosts configured.").style(
                        sty()
                            .padding(px(16))
                            .border_radius(px(14))
                            .background(rgba(255, 255, 255, 0.03))
                            .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                            .font_size(px(14))
                            .line_height(1.6),
                    )
                } else {
                    div(rows.join("\n")).class("editor").style(
                        sty()
                            .min_height(px(0))
                            .max_height(px(400))
                            .overflow_y(OverflowYKeyword::Auto)
                            .white_space(WhiteSpaceKeyword::PreWrap),
                    )
                }
            }
            ResourceState::Error(err) => div(format!("Host table error: {:?}", err)).style(
                sty()
                    .padding(px(16))
                    .border_radius(px(14))
                    .background(AppTheme::DANGER.alpha(0.03))
                    .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                    .font_size(px(14))
                    .line_height(1.6)
                    .color(AppTheme::DANGER)
            ),
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
            ResourceState::Idle | ResourceState::Loading => div("Loading logs...").style(
                sty()
                    .padding(px(16))
                    .border_radius(px(14))
                    .background(rgba(255, 255, 255, 0.03))
                    .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                    .font_size(px(14))
                    .line_height(1.6)
            ),
            ResourceState::Ready(snapshot) | ResourceState::Reloading(snapshot) => {
                let logs = snapshot
                    .logs
                    .iter()
                    .rev()
                    .take(limit)
                    .cloned()
                    .collect::<Vec<_>>();

                if logs.is_empty() {
                    div("No logs yet.").style(
                        sty()
                            .padding(px(16))
                            .border_radius(px(14))
                            .background(rgba(255, 255, 255, 0.03))
                            .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                            .font_size(px(14))
                            .line_height(1.6),
                    )
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
                    .style(sty().max_height(px(320)).overflow_y(OverflowYKeyword::Auto))
                }
            }
            ResourceState::Error(err) => div(format!("Log error: {:?}", err)).style(
                sty()
                    .padding(px(16))
                    .border_radius(px(14))
                    .background(rgba(239, 68, 68, 0.03))
                    .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                    .font_size(px(14))
                    .line_height(1.6)
                    .color(AppTheme::DANGER)
            ),
        }
    ])
}

#[component]
pub fn MutationLedger() -> impl View {
    let ctx = use_dashboard();

    div![
        StyledPanel(div![
            h3("Command state").style(sty().margin(px(0))),
            ActionStatus().label("Sync").loading(ctx.sync.loading()).error(ctx.sync.error().map(|e| format!("{:?}", e))).value(ctx.sync.value()),
            ActionStatus().label("Reload").loading(ctx.reload.loading()).error(ctx.reload.error().map(|e| format!("{:?}", e))).value(ctx.reload.value()),
            ActionStatus().label("Save").loading(ctx.save.loading()).error(ctx.save.error().map(|e| format!("{:?}", e))).value(ctx.save.value()),
            ActionStatus().label("Apply draft").loading(ctx.apply_config.loading()).error(ctx.apply_config.error().map(|e| format!("{:?}", e))).value(ctx.apply_config.value()),
            ActionStatus().label("Stop").loading(ctx.stop.loading()).error(ctx.stop.error().map(|e| format!("{:?}", e))).value(ctx.stop.value()),
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

#[component]
pub fn HostListPanel() -> impl View {
    let ctx = use_dashboard();

    StyledPanel(view_chain![
        div![
            h3("Host Health").style(sty().margin(px(0))),
            StyledBadge("real-time status")
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
                div("Loading host status...").style(sty().padding(px(16)).font_size(px(14))),
            ResourceState::Ready(snapshot) | ResourceState::Reloading(snapshot) => {
                if snapshot.status.hosts.is_empty() {
                    div("No hosts configured.").style(sty().padding(px(16)).font_size(px(14)))
                } else {
                    div(snapshot
                        .status
                        .hosts
                        .iter()
                        .map(|host| host_row(host.clone()))
                        .collect::<Vec<_>>())
                    .style(
                        sty()
                            .display(DisplayKeyword::Flex)
                            .flex_direction(FlexDirectionKeyword::Column)
                            .gap(px(12)),
                    )
                }
            }
            ResourceState::Error(_) => div("Error loading status"),
        }
    ])
}

fn host_row(host: HostStatus) -> impl View {
    let expanded = RwSignal::new(false);

    div![
        div![
            div![
                span(if host.fail_count > 0 { "❌" } else { "✅" })
                    .style(sty().margin_right(px(12))),
                span(host.name.clone()).style(sty().font_weight(700)),
                span(request_type_label(host.request_type))
                    .class("pill")
                    .style(
                        sty()
                            .margin_left(px(12))
                            .font_size(px(11))
                            .padding_left(px(8))
                            .padding_right(px(8))
                            .padding_top(px(2))
                            .padding_bottom(px(2))
                    ),
            ]
            .style(
                sty()
                    .display(DisplayKeyword::Flex)
                    .align_items(AlignItemsKeyword::Center)
            ),
            div![
                span(format!("Pr {}", host.priority)).style(
                    sty()
                        .color(AppTheme::MUTED)
                        .font_size(px(13))
                        .margin_right(px(16))
                ),
                if host.fail_count > 0 {
                    div![
                        span(format!("Fails: {}", host.fail_count)).style(
                            sty()
                                .color(AppTheme::DANGER)
                                .font_weight(700)
                                .margin_right(px(12))
                                .font_size(px(13))
                        ),
                        button(move || if expanded.get() {
                            "Hide Error"
                        } else {
                            "View Error"
                        })
                        .on(event::click, move |_| expanded.update(|v| *v = !*v))
                        .style(
                            sty()
                                .padding_left(px(12))
                                .padding_right(px(12))
                                .padding_top(px(4))
                                .padding_bottom(px(4))
                                .border_radius(px(8))
                                .background(rgba(255, 255, 255, 0.05))
                                .border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE))
                                .cursor(CursorKeyword::Pointer)
                                .font_size(px(11))
                                .color(AppTheme::MUTED)
                                .font_weight(700)
                                .transition("all 0.2s")
                        )
                    ]
                    .style(
                        sty()
                            .display(DisplayKeyword::Flex)
                            .align_items(AlignItemsKeyword::Center),
                    )
                    .into_any()
                } else {
                    span("Healthy")
                        .style(
                            sty()
                                .color(AppTheme::SUCCESS)
                                .font_size(px(13))
                                .font_weight(600),
                        )
                        .into_any()
                }
            ]
            .style(
                sty()
                    .display(DisplayKeyword::Flex)
                    .align_items(AlignItemsKeyword::Center)
            ),
        ]
        .style(
            sty()
                .display(DisplayKeyword::Flex)
                .justify_content(JustifyContentKeyword::SpaceBetween)
                .align_items(AlignItemsKeyword::Center)
                .padding(px(12))
                .background(rgba(255, 255, 255, 0.02))
                .border_radius(px(14))
        ),
        move || if expanded.get()
            && let Some(err) = host.last_error.clone()
        {
            div![
                div(err).style(
                    sty()
                        .padding(px(16))
                        .background(AppTheme::DANGER.alpha(0.05))
                        .border_left(border(px(4), BorderStyleKeyword::Solid, AppTheme::DANGER))
                        .font_family("'JetBrains Mono', monospace")
                        .font_size(px(12))
                        .line_height(1.5)
                        .white_space(WhiteSpaceKeyword::PreWrap)
                        .margin_top(px(8))
                        .border_radius(px(12))
                        .word_break(WordBreakKeyword::BreakAll)
                )
            ]
        } else {
            div("")
        }
    ]
}

fn failing_hosts(snapshot: &RuntimeSnapshot) -> usize {
    snapshot
        .status
        .hosts
        .iter()
        .filter(|h| h.fail_count > 0)
        .count()
}

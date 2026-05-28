use super::helpers::*;
use super::panels::*;
use crate::service::*;
use silex::prelude::*;

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
                StyledToolButton(view_chain![
                    icon_sync(),
                    move || {
                        ctx.snapshot.get_data()
                            .map(|s| if s.config.sync_mode == SyncMode::Off { "Start Sync" } else { "Sync now" })
                            .unwrap_or("Sync now")
                    }
                ])
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
            div![
                HostListPanel(),
                RecentLogsPanel(12),
            ].class("stack"),
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
                    } else if ctx.apply_config.loading() {
                        div("Applying draft to the service...")
                            .style(sty().padding(px(16)).border_radius(px(14)).background(AppTheme::WARNING.alpha(0.05)).border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE)).font_size(px(14)).line_height(1.6).color(AppTheme::WARNING))
                    } else if let Some(msg) = ctx.apply_config.value() {
                        div(format!("Last draft update: {msg}"))
                            .style(sty().padding(px(16)).border_radius(px(14)).background(AppTheme::SUCCESS.alpha(0.05)).border(border(px(1), BorderStyleKeyword::Solid, AppTheme::LINE)).font_size(px(14)).line_height(1.6).color(AppTheme::SUCCESS))
                    } else {
                        div("")
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
            RecentLogsPanel(80),
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

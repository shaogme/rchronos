use crate::service::*;
use silex::prelude::*;

mod helpers;
mod pages;
mod panels;

use helpers::*;
use pages::*;

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
            Link(AppRoute::Overview).children(span![icon_overview(), "Overview"]).active_class("active"),
            Link(AppRoute::Config).children(span![icon_config(), "Config"]).active_class("active"),
            Link(AppRoute::Logs).children(span![icon_logs(), "Logs"]).active_class("active"),
            Link(AppRoute::About).children(span![icon_about(), "About"]).active_class("active"),
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

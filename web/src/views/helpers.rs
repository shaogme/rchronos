use crate::service::*;
use silex::prelude::*;

// --- Styled Components ---

styled! {
    pub StyledMetricCard<div>(children: AnyView) {
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
    pub StyledPanel<div>(children: AnyView) {
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
    pub StyledKVRow<div>(children: AnyView) {
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
    pub StyledBadge<span>(children: AnyView) {
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
        children: AnyView,
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

// --- Helpers & Icons ---

#[component]
pub fn TimeSpan(ms: u64) -> impl View {
    span(format_ms_adaptive(ms)).attr("title", format!("{}ms", ms))
}

pub fn kv_row(label: &'static str, value: impl View + 'static) -> impl View {
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
}

pub fn icon_overview() -> AnyView {
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
    .into_any()
}

pub fn icon_config() -> AnyView {
    svg(view_chain![
        path().attr("d", "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1-1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.1a2 2 0 0 1-1-1.72v-.51a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"),
        circle().attr("cx", "12").attr("cy", "12").attr("r", "3")
    ])
    .attr("width", "16").attr("height", "16").attr("viewBox", "0 0 24 24").attr("fill", "none")
    .attr("stroke", "currentColor").attr("stroke-width", "2.5").attr("stroke-linecap", "round").attr("stroke-linejoin", "round")
    .into_any()
}

pub fn icon_logs() -> AnyView {
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
    .into_any()
}

pub fn icon_about() -> AnyView {
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
    .into_any()
}

pub fn icon_sync() -> AnyView {
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
    .into_any()
}

pub fn icon_save() -> AnyView {
    svg(view_chain![
        path().attr("d", "M15.2 3a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z"),
        path().attr("d", "M17 21v-7a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v7"),
        path().attr("d", "M7 3v4a1 1 0 0 0 1 1h7")
    ])
    .attr("width", "15").attr("height", "15").attr("viewBox", "0 0 24 24").attr("fill", "none")
    .attr("stroke", "currentColor").attr("stroke-width", "2.5").attr("stroke-linecap", "round").attr("stroke-linejoin", "round")
    .into_any()
}

pub fn icon_stop() -> AnyView {
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
    .into_any()
}

pub fn icon_reload() -> AnyView {
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
    .into_any()
}

#[component]
pub fn ActionStatus(
    label: &'static str,
    #[chain] loading: bool,
    #[chain] error: Option<String>,
    #[chain] value: Option<String>,
) -> impl View {
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
    if loading { view.class("pulse") } else { view }
}

#[component]
pub fn MetricCard(
    label: &'static str,
    #[chain] value: AnyView,
    #[chain] subtitle: AnyView,
) -> impl View {
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
}

pub fn loading_metric(
    label: &'static str,
    value: &'static str,
    subtitle: &'static str,
) -> impl View {
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

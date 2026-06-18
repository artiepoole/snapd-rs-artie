use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::ListItem,
};
use snapd_rs::AppInfo;

pub(crate) fn service_list_item(service: &AppInfo) -> ListItem<'static> {
    let active = service.active == Some(true);
    let enabled = service.enabled != Some(false);

    let marker = if active {
        Span::styled("● ", Style::default().fg(Color::Green))
    } else {
        Span::styled("○ ", Style::default().fg(Color::DarkGray))
    };

    let mut spans = vec![
        marker,
        Span::styled(
            service.name.clone(),
            Style::default().fg(Color::White).bold(),
        ),
    ];

    if active {
        spans.push(Span::styled("  running", Style::default().fg(Color::Green)));
    } else {
        spans.push(Span::styled(
            "  stopped",
            Style::default().fg(Color::DarkGray),
        ));
    }

    if !enabled {
        spans.push(Span::styled(" (disabled)", Style::default().fg(Color::Red)));
    }

    ListItem::new(Line::from(spans))
}

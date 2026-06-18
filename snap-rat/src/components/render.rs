use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::ListItem,
};
use snapd_rs::ComponentInfo;

pub(crate) fn component_list_item(component: &ComponentInfo) -> ListItem<'static> {
    let installed = component.install_date.is_some();
    let marker = if installed {
        Span::styled("● ", Style::default().fg(Color::Green))
    } else {
        Span::styled("○ ", Style::default().fg(Color::DarkGray))
    };

    let mut spans = vec![
        marker,
        Span::styled(
            component.name.clone(),
            Style::default().fg(Color::White).bold(),
        ),
    ];

    if let Some(ref version) = component.version {
        spans.push(Span::styled(
            format!("  {version}"),
            Style::default().fg(Color::DarkGray),
        ));
    }

    if let Some(ref type_) = component.type_ {
        spans.push(Span::styled(
            format!("  [{type_}]"),
            Style::default().fg(Color::DarkGray),
        ));
    }

    if !installed {
        spans.push(Span::styled(
            "  [not installed]",
            Style::default().fg(Color::DarkGray),
        ));
    }

    ListItem::new(Line::from(spans))
}

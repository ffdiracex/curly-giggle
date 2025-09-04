//src/ui.rs

use crate::app::{App, Mode};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block,Borders,List,ListItem,Paragraph,Wrap},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App){
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(3), //Main content
            Constraint::Length(3), //Status bar
            Constraint::Length(1), //Command line
        ])
        .spit(f.size());

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60), //File list
            Constraint::Percentage(40), //Preview
        ])
        .split(chunks[0]);
    draw_file_list(f, app, main_chunks[0]);
    draw_preview(f, app, main_chunks[1]);
    draw_status_bar(f, app, chunks[1]);
    draw_command_line(f, app, chunks[2]);
}


fn draw_file_list<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect){
    let items: Vec<ListItem> = app.file_manager.entries
        .iter()
        .enumerate()
        .map(|(i,entry)|{
            let style = if i == app.selected_index {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
            } else{
                Style::default()
            };

            let icon = if entry.is_file {"📄"} else {"📁"};
            let display_name = if entry.name == ".."{
                "..(parent)".to_string()
            } else{
                format!("{}{}", icon, entry.name)
            };
            ListItem::new(Line::from(Span::styled(display_name,style)))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Files")
                .borders(Borders::ALL),
        );
    f.render_widget(list, area);
}

fn draw_preview<B: Backend>(f: &mut Frame<B>, app: &App, area:Rect){
    let preview = Paragraph::new(app.preview_content.as_str())
        .block(
            Block::default()
                .title("Preview")
                .borders(Borders::ALL),
        )
        .wrap(Wrap {trim: true})
        .scroll((0,0));

    f.render_widget(preview, area);
}

fn draw_status_bar<B:Backend>(f: &mut Frame<B>, app: &App, area: Rect){
    let mode_text = match app.mode {
        Mode::Normal => "NORMAL",
        Mode::Insert => "INSERT",
        Mode::Command => "COMMAND",
        Mode::Visual => "VISUAL",
    };

    let mode_style = Style::default()
        .fg(Color::Black)
        .bg(Color::Yellow)
        .add_modifier(Modifier::BOLD);

    let path_text = app.current_path.to_string_lossy();
    let selected_info = if let Some(selected) = app.file_manager.get_selected(app.selected_index){
        if selected.is_file {
            format!("{} ({} bytes)", selected.name, selected.size)
        } else{
            selected.name.clone()
        }
    } else{
        String::new()
    };

    let status_line = Line::from(vec![
        Span::styled(format!(" {} ", mode_text), mode_style),
        Span::raw(" "),
        Span::raw(path_text.as_ref()),
        Span::raw(" | "),
        Span::raw(selected_info),
    ]);
    let status_bar = Paragraph::new(status_line)
        .block(Block::default().borders(Borders::TOP));

    f.render_widget(status_bar, area);
}

fn draw_command_line<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect){
    let prompt = if app.mode == Mode::Command{
        format!(":{}", app.command_buffer)
    } else{
        String::new()
    };
    let command_line = Paragraph::new(prompt)
        .block(Block::default());

    f.render_widget(command_line, area);
}


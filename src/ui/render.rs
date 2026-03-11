use chrono::{Local, NaiveTime};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame,
};
use tui_big_text::{BigTextBuilder, PixelSize};

use crate::ui::app::{App, Tab};

pub fn render(f: &mut Frame, app: &mut App) {
    match app.active_tab {
        Tab::Dashboard => render_dashboard(f, app),
        Tab::Countdown => render_countdown(f, app),
        Tab::Settings => render_settings(f, app),
        Tab::About => render_about(f, app),
    }
}

fn render_dashboard(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(5), // Header (Diperbesar)
                Constraint::Length(5), // Schedule Summary (Kembalikan ke ukuran 5 baris)
                Constraint::Min(0),    // Menu
            ]
            .as_ref(),
        )
        .split(f.area());

    // Header
    let header = Paragraph::new("\n A D Z A N   R E M I N D E R   C L I \n")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Schedule Summary
    let mut schedule_lines = vec![];
    if let Some(ref schedule) = app.schedule {
        let kabko = &schedule.data.kabko;
        if let Some((_, jadwal_hari)) = schedule.data.jadwal.iter().next() {
            schedule_lines.push(Line::from(vec![
                Span::styled(
                    format!("Lokasi: {}", kabko),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" | "),
                Span::styled(
                    format!("Tanggal: {}", jadwal_hari.tanggal),
                    Style::default().fg(Color::Yellow),
                ),
            ]));
            schedule_lines.push(Line::from(""));
            let prayer_style = Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD);
            schedule_lines.push(Line::from(vec![
                Span::raw("Subuh: "),
                Span::styled(&jadwal_hari.subuh, prayer_style),
                Span::raw(" | Dzuhur: "),
                Span::styled(&jadwal_hari.dzuhur, prayer_style),
                Span::raw(" | Ashar: "),
                Span::styled(&jadwal_hari.ashar, prayer_style),
                Span::raw(" | Maghrib: "),
                Span::styled(&jadwal_hari.maghrib, prayer_style),
                Span::raw(" | Isya: "),
                Span::styled(&jadwal_hari.isya, prayer_style),
            ]));
        }
    } else {
        schedule_lines.push(Line::from(
            "Jadwal belum tersedia. Buka tab Settings untuk set kota.",
        ));
    }

    let summary = Paragraph::new(schedule_lines).block(
        Block::default()
            .title(" Jadwal Hari Ini ")
            .borders(Borders::ALL),
    );
    f.render_widget(summary, chunks[1]);

    // Menu
    let items = vec![
        ListItem::new("1. Countdown Adzan"),
        ListItem::new("2. Settings"),
        ListItem::new("3. About"),
        ListItem::new("4. Quit"),
    ];
    let menu_list = List::new(items)
        .block(Block::default().title(" Menu Utama ").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let menu_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(chunks[2]);

    let mut state = ListState::default();
    state.select(Some(app.selected_menu_index));
    f.render_stateful_widget(menu_list, menu_chunk[0], &mut state);

    let footer = Paragraph::new("q: Quit | Enter: Select | b/Esc: Back")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, menu_chunk[1]);
}

fn render_countdown(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(5), // Highlight Next Prayer
                Constraint::Min(4),    // Big Text Clock
                Constraint::Min(0),    // Padding below
            ]
            .as_ref(),
        )
        .split(f.area());

    let mut next_prayer_name = "Belum Diketahui".to_string();
    let mut countdown_text = "00:00:00".to_string();

    if let Some(ref schedule) = app.schedule {
        if let Some((_, jadwal_hari)) = schedule.data.jadwal.iter().next() {
            let now = Local::now();
            let prayer_list: Vec<(&str, &String)> = vec![
                ("Subuh", &jadwal_hari.subuh),
                ("Dzuhur", &jadwal_hari.dzuhur),
                ("Ashar", &jadwal_hari.ashar),
                ("Maghrib", &jadwal_hari.maghrib),
                ("Isya", &jadwal_hari.isya),
            ];

            let mut found = false;
            for (name, time_str) in &prayer_list {
                if let Ok(parsed_time) = NaiveTime::parse_from_str(time_str, "%H:%M") {
                    let p_datetime = now
                        .date_naive()
                        .and_time(parsed_time)
                        .and_local_timezone(Local)
                        .unwrap();

                    if p_datetime > now {
                        let duration = p_datetime.signed_duration_since(now);
                        let hours = duration.num_hours();
                        let minutes = duration.num_minutes() % 60;
                        let seconds = duration.num_seconds() % 60;

                        next_prayer_name = name.to_string();
                        countdown_text = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                next_prayer_name = "Semua Sudah Lewat".to_string();
                countdown_text = "--:--:--".to_string();
            }
        }
    }

    let header_text = format!(
        "\n  T H E   N E X T   A D Z A N :   {}  \n",
        next_prayer_name.to_uppercase()
    );
    let highlight = Paragraph::new(header_text)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(highlight, chunks[0]);

    let big_text = BigTextBuilder::default()
        .pixel_size(PixelSize::Full)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .lines(vec![countdown_text.into()])
        .build();

    let clock_block = Block::default().borders(Borders::ALL);
    let inner_area = clock_block.inner(chunks[1]);
    f.render_widget(clock_block, chunks[1]);

    // Tui-big-text natively aligns to the center of inner_area because of .alignment(Center)
    f.render_widget(big_text, inner_area);

    let footer = Paragraph::new("q: Quit | Enter: Select | b/Esc: Back")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);
}

fn render_settings(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(5), // Header
                Constraint::Min(0),    // Split Body
            ]
            .as_ref(),
        )
        .split(f.area());

    let header = Paragraph::new("\n S E T T I N G S \n")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50), // Menu Settings
                Constraint::Percentage(50), // Tooltip
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let items = vec![
        ListItem::new("1. Set Kota Lokasi (Gunakan 'adzan set-city' di CLI)"),
        ListItem::new(format!(
            "2. Waktu Peringatan Awal: < {} Menit > (Tekan Enter)",
            app.config.notification_time
        )),
        ListItem::new(format!(
            "3. Suara Adzan: {} (Tekan Enter)",
            app.config.sound_choice
        )),
        ListItem::new("4. Test Notifikasi (Tekan Enter)"),
        ListItem::new("5. Setup Daemon Autostart (Gunakan 'adzan setup-autostart' di CLI)"),
        ListItem::new("6. Cek Update Aplikasi (Tekan Enter)"),
    ];

    let menu_list = List::new(items)
        .block(
            Block::default()
                .title(" Daftar Pengaturan ")
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let mut state = ListState::default();
    state.select(Some(app.selected_settings_index));
    f.render_stateful_widget(menu_list, body_chunks[0], &mut state);

    let tooltip_text = match app.selected_settings_index {
        0 => "Mengubah kota tempat kamu berada.\nAPI akan mencarikan jadwal sholat akurat berasarkan ID kota tersebut.",
        1 => "Fitur pre-reminder.\nMengatur berapa menit aplikasi akan bunyi / memberi notifikasi SEBELUM waktu azan masuk.",
        2 => "Pilih suara alarm yang kamu suka (Bedug / Adzan / Mute).",
        3 => "Mensimulasikan seolah-olah waktu Adzan sudah tiba (memutar audio test).",
        4 => "Mengonfigurasi service OS (Launchd/SystemD) agar reminder jalan sendiri setiap komputer nyala tanpa perlu buka terminal.",
        5 => "Mengecek pembaruan aplikasi dari GitHub dan langsung mendownload versi terbaru jika tersedia.",
        _ => "",
    };

    let tooltip = Paragraph::new(tooltip_text)
        .style(Style::default().fg(Color::LightBlue))
        .block(
            Block::default()
                .title(" Keterangan ")
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::new(2, 2, 1, 1)),
        );

    let right_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(body_chunks[1]);

    f.render_widget(tooltip, right_chunk[0]);

    let footer = Paragraph::new("q: Quit | Enter: Select | b/Esc: Back")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, right_chunk[1]);

    // Draw modals on top if we are in a special state
    match &app.setting_state {
        crate::ui::app::SettingState::SearchingCity {
            query,
            results,
            selected_index,
        } => {
            let area = centered_rect(60, 60, f.area());
            f.render_widget(Clear, area);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(area);

            let search_bar = Paragraph::new(format!("> {}_", query)).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Cari Kota (Ketik & Enter) "),
            );
            f.render_widget(search_bar, chunks[0]);

            let list_items: Vec<ListItem> = results
                .iter()
                .map(|c| ListItem::new(c.lokasi.clone()))
                .collect();
            let result_list = List::new(list_items)
                .block(Block::default().borders(Borders::ALL).title(" Hasil "))
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            let mut state = ListState::default();
            state.select(Some(*selected_index));
            f.render_stateful_widget(result_list, chunks[1], &mut state);
        }
        crate::ui::app::SettingState::EditingSound { selected_index } => {
            let area = centered_rect(40, 40, f.area());
            f.render_widget(Clear, area);

            let sounds = ["bedug", "adzan_mecca", "adzan_shubuh", "mute"];
            let list_items: Vec<ListItem> = sounds.iter().map(|s| ListItem::new(*s)).collect();
            let result_list = List::new(list_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Pilih Suara (Up/Down + Enter) "),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            let mut state = ListState::default();
            state.select(Some(*selected_index));
            f.render_stateful_widget(result_list, area, &mut state);
        }
        crate::ui::app::SettingState::ShowingMessage(msg) => {
            let area = centered_rect(70, 60, f.area());
            f.render_widget(Clear, area);
            let message_widget = Paragraph::new(msg.as_str())
                .style(Style::default().fg(Color::White))
                .block(
                    Block::default()
                        .title(" Peringatan / Informasi ")
                        .borders(Borders::ALL)
                        .padding(ratatui::widgets::Padding::new(2, 2, 2, 2)),
                )
                .alignment(ratatui::layout::Alignment::Center)
                .wrap(ratatui::widgets::Wrap { trim: true });
            f.render_widget(message_widget, area);
        }
        crate::ui::app::SettingState::ManagingDaemon { selected_index } => {
            let area = centered_rect(40, 40, f.area());
            f.render_widget(Clear, area);

            let options = ["Instal & Start Daemon", "Uninstall & Stop Daemon", "Batal"];
            let list_items: Vec<ListItem> = options.iter().map(|s| ListItem::new(*s)).collect();
            let result_list = List::new(list_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Setup Background Daemon "),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            let mut state = ListState::default();
            state.select(Some(*selected_index));
            f.render_stateful_widget(result_list, area, &mut state);
        }
        _ => {}
    }
}

/// Helper to render a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_about(f: &mut Frame, _app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(5), // Header
                Constraint::Min(0),    // Body
                Constraint::Length(2), // Footer
            ]
            .as_ref(),
        )
        .split(f.area());

    let header = Paragraph::new("\n A B O U T \n")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    let about_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "ADZAN REMINDER",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from("Dibuat dengan ❤️ oleh:"),
        Line::from("Ismail Nur Alam"),
        Line::from("GitHub: github.com/itzmail"),
        Line::from(""),
        Line::from("\"Dan ingatkanlah mereka, karena"),
        Line::from(" sesungguhnya peringatan itu"),
        Line::from(" bermanfaat bagi orang-orang mukmin.\""),
        Line::from("(QS. Adz-Dzariyat: 55)"),
        Line::from(""),
        Line::from("Terima kasih telah menggunakan app ini!"),
        Line::from("Semoga menjadi amal jariyah 🤲"),
    ];

    let body = Paragraph::new(about_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(body, chunks[1]);

    let footer = Paragraph::new("q: Quit | b/Esc: Back")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);
}

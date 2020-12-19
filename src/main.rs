#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

//use std::io::Read; // check again with async_stdin
use std::time::Instant;
use std::{error::Error, io, io::Read};
use termion::{
    async_stdin, event::Key, input::MouseTerminal, input::TermRead, raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Terminal,
};
use toml;
mod price;
mod product;
use crate::product::Product;

pub struct StatefulTable<> {
    state: TableState,
    items: Vec<Vec<String>>,
}

impl<'a> StatefulTable<> {
    fn new() -> StatefulTable<> {
        StatefulTable {
            state: TableState::default(),
            items: vec![
                vec![String::from("Row11"), String::from("Row12")],
            ],
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

fn main() -> Result<(), io::Error> {
    let mut product1 = Product::new("https://skroutz.gr/s/21404653/Dell-P2720D.html");
    let mut product2 = Product::new("https://www.skroutz.gr/s/21443617/Dell-P2720DC-27.html");
    println!("{:#?}", product1.name());
    product1.update_product();
    println!("{:#?}", product1.name());
    println!("{:#?}", product1.price.printable());

    // Set up terminal output
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create a separate thread to poll stdin.
    // This provides non-blocking input support.
    let mut astdin = async_stdin();

    let mut table = StatefulTable::new();

    let mut then = Instant::now();
    terminal.clear()?;
    loop {
        let now = Instant::now();
        let msecs = now.duration_since(then).as_millis();
        if msecs > 30000 {
            // 30 seconds passed we mayneed to perform something here (update data)
            table.items = vec![
                product1.get_vector(),
                product2.get_vector(),
            ];
            then = Instant::now();
        }

        // Start drawing
        terminal.draw(|frame| {
            // Create layout to place the blocks
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(frame.size());

            let selected_style = Style::default().add_modifier(Modifier::REVERSED);
            let normal_style = Style::default().bg(Color::Blue);
            let header_cells = ["Product name", "Current price"]
                .iter()
                .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
            let header = Row::new(header_cells).style(normal_style).height(1);
            // .botton_margin(1);
            let rows = table.items.iter().map(|item| {
                let height = item
                    .iter()
                    .map(|content| content.chars().filter(|c| *c == '\n').count())
                    .max()
                    .unwrap_or(0)
                    + 1;
                let cells = item.iter().map(|c| Cell::from(c.as_str()));
                Row::new(cells).height(height as u16)
            });
            let t = Table::new(rows)
                .header(header)
                .block(Block::default().borders(Borders::ALL).title("Prices"))
                .highlight_style(selected_style)
                // .highlight_symbol(">> ")
                .widths(&[
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
                ]);
            frame.render_stateful_widget(t, chunks[0], &mut table.state);

            // Create le block 2
            // let txt = vec![
            //     Spans::from("This is a line.\n"),
            //     Spans::from("This ia a second line.\n"),
            // ];
            // let graph = Paragraph::new(txt)
            //     .block(Block::default().title("Prices?").borders(Borders::ALL))
            //     .style(Style::default().fg(Color::Magenta).bg(Color::Black));
            // frame.render_widget(graph, chunks[1]);
        })?;

        // Iterate over the keys that have been pressed since last time
        for key in astdin.by_ref().keys() {
            match key.unwrap() {
                // If find 'q' quit
                Key::Char('q') => {
                    terminal.clear()?;
                    return Ok(());
                }
                Key::Down => {
                    table.next();
                }
                Key::Up => {
                    table.previous();
                }
                // Ignore others
                _ => (),
            }
        }
    }
}

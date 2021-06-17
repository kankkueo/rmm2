use std::io;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::screen::AlternateScreen;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{List, ListState, ListItem, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Modifier, Style};

use crate::loadorder;
use crate::files::write_loadorder;
use crate::config::Gamepath;
pub mod events;
pub mod installer;

struct StateList<'a> {
    items: Vec<ListItem<'a>>,
    state: ListState,
}

impl<'a> StateList<'a> {
    fn new() -> StateList<'a> {
        StateList {
            items: Vec::new(),
            state: ListState::default(),
        }
    }

    fn from(vec: Vec<String>) -> StateList<'a> {
        let mut items: Vec<ListItem> = Vec::new();
        for i in 0..vec.len() {
            items.push(ListItem::new(vec[i].clone()));
        }
         StateList {
            items: items ,
            state: ListState::default(),
        }       
    }

    fn update(&mut self, vec: Vec<String>) {
        for i in 0..vec.len() {
            self.items[i] = ListItem::new(vec[i].clone());
        }
    }

    fn select_next(&mut self) {
        match self.state.selected() {
            None => self.state.select(Some(0)),
            Some(x) =>  {
                if x < self.items.len() -1 {
                    self.state.select(Some(x + 1));
                }
            }
        }
    }

    fn select_prev(&mut self) {
        match self.state.selected() {
            None => self.state.select(Some(0)),
            Some(x) => {
                if x > 0 {
                    self.state.select(Some(x - 1));
                }
            }
        }
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub fn mode_selection_menu(events: &events::Events) -> io::Result<usize> {

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut menu = StateList::from( vec![
        String::from("Skyrim Special edition"),
        String::from("Skyrim"),
        String::from("Oblivion"),
        String::from("Fallout 4"),
        String::from("Fallout New Vegas"),
        String::from("Fallout 3"),
    ] );

    loop {

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(10)
                .constraints([
                    Constraint::Percentage(100),
    
                ].as_ref())
                .split(f.size());
    
            let list = List::new(menu.items.clone())
                .block(
                    Block::default()
                        .title("Select game to manage")
                        .borders(Borders::ALL)
                        .border_style(
                            Style::default()
                                .fg(Color::Rgb(255, 255, 255))
                        )
                )
                .style(
                    Style::default()
                        .fg(Color::Rgb(0, 255, 155))
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Rgb(255, 0, 0))
                        .add_modifier(Modifier::BOLD)
                );

           f.render_stateful_widget(list, chunks[0], &mut menu.state);

        })?;

        match events.next().unwrap() {
            events::Event::Input(key) => match key {
                Key::Up => menu.select_prev(),
                Key::Char('k') => menu.select_prev(),
                Key::Down => menu.select_next(),
                Key::Char('j') => menu.select_next(),
                Key::Char('\n') => match menu.state.selected() {
                    Some(x) => {return Ok(x + 1);},
                    None => continue,
                }
                _default => continue,
            }
            events::Event::Tick => continue,
        }
    }
}

pub fn plugin_menu(plugins: &mut Vec<loadorder::Plugin>, mods: &mut Vec<String>,paths: Gamepath, mode: usize, events: &events::Events) -> io::Result<Option<String>> {
    let stdout = io::stdout().into_raw_mode()?;
    //let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut menu: Vec<StateList> = Vec::new();
    menu.push(StateList::from(loadorder::to_strvec(&plugins)));
    menu.push(StateList::from(mods.to_vec()));
    let mut sclt = 0;

    loop { 

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
    
                ].as_ref())
                .split(f.size());
    
            let plugin_list = List::new(menu[0].items.clone())
                .block(
                    Block::default()
                        .title("Plugins")
                        .borders(Borders::ALL)
                        .border_style(
                            Style::default()
                                .fg(Color::Rgb(255, 255, 255))
                        )
                )
                .style(
                    Style::default()
                        .fg(Color::Rgb(0, 255, 155))
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Rgb(255, 0, 0))
                        .add_modifier(Modifier::BOLD)
                );

             let mod_list = List::new(menu[1].items.clone())
                .block(
                    Block::default()
                        .title("Installable mods")
                        .borders(Borders::ALL)
                        .border_style(
                            Style::default()
                                .fg(Color::Rgb(255, 255, 255))
                        )
                )
                .style(
                    Style::default()
                        .fg(Color::Rgb(0, 255, 155))
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Rgb(255, 0, 0))
                        .add_modifier(Modifier::BOLD)
                );

            f.render_stateful_widget(plugin_list, chunks[0], &mut menu[0].state);
            f.render_stateful_widget(mod_list, chunks[1], &mut menu[1].state);

        })?;

        match events.next().unwrap() {
            events::Event::Input(key) => match key {
                Key::Char('q') => {
                    let w_plugs = plugins.clone();
                    write_loadorder(w_plugs, &paths.plugins, mode);
                    break;
                }
                Key::Char('l') => {
                    menu[sclt].unselect();
                    if sclt == 0 { sclt = 1; }
                    else { sclt = 0; }
                    menu[sclt].select_next();
                }
                Key::Right => {
                    menu[sclt].unselect();
                    if sclt == 0 { sclt = 1; }
                    else { sclt = 0; }
                    menu[sclt].select_next();
                }
                Key::Char('h') => {
                    menu[sclt].unselect();
                    if sclt == 0 { sclt = 1; }
                    else { sclt = 0; }
                    menu[sclt].select_next();
                }
                Key::Left => {
                    menu[sclt].unselect();
                    if sclt == 0 { sclt = 1; }
                    else { sclt = 0; }
                    menu[sclt].select_next();
                }
                Key::Up => menu[sclt].select_prev(),
                Key::Char('k') => menu[sclt].select_prev(),
                Key::Down => menu[sclt].select_next(),
                Key::Char('j') => menu[sclt].select_next(),
                Key::Char('\n') => match menu[sclt].state.selected() {
                    Some(x) => {
                        if sclt == 0 {
                            plugins[x].activate();
                            menu[sclt].update(loadorder::to_strvec(&plugins));
                        }
                        else {
                            let src_p = format!("{}{}", paths.mods, mods[x]);
                            return Ok(Some(src_p));
                        }
                    }
                    None => continue,
                }
                Key::Char('w') => match menu[sclt].state.selected() {
                    Some(x) => {
                        if sclt == 0 {
                            loadorder::move_down(plugins, x);
                            menu[sclt].update(loadorder::to_strvec(&plugins));
                            menu[sclt].select_next();
                        }
                    }
                    None => continue,
                }
                Key::Char('s') => match menu[sclt].state.selected() {
                    Some(x) => {
                        if sclt == 0 {
                            loadorder::move_down(plugins, x);
                            menu[sclt].update(loadorder::to_strvec(&plugins));
                            menu[sclt].select_next();
                        }
                    } 
                    None => continue,
                }
                _default => continue,
            }
            events::Event::Tick => continue,
        }
    }
    Ok(None)

}  



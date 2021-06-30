use std::io;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::screen::AlternateScreen;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{List, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Modifier, Style};

use crate::loadorder;
use crate::modinstall::FomodGroup;
use crate::files::{write_loadorder, read_datadir};
use crate::config::Gamepath;
use crate::paths::Path;

mod utils;
mod events;


pub fn mode_selection_menu() -> io::Result<usize> {

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut menu = utils::StateList::from( vec![
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

        match utils::keyin() {
            Key::Up | Key::Char('k') => menu.select_prev(),
            Key::Down | Key::Char('j') => menu.select_next(),
            Key::Char('\n') => match menu.state.selected() {
                Some(x) => {return Ok(x + 1);},
                None => continue,
            }
            _default => continue,
        }
    }
}

pub fn plugin_menu(plugins: &mut Vec<loadorder::Plugin>, mods: &mut Vec<String>,paths: Gamepath, mode: usize) -> io::Result<Option<Path>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut menu: Vec<utils::StateList> = Vec::new();
    menu.push(utils::StateList::from(loadorder::to_strvec(&plugins)));
    menu.push(utils::StateList::from(mods.to_vec()));
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

        match utils::keyin() {
            Key::Char('q') => {
                let w_plugs = plugins.clone();
                write_loadorder(w_plugs, &paths.plugins, mode);
                break;
            }
            Key::Right | Key::Char('l') => {
                menu[sclt].unselect();
                if sclt == 0 { sclt = 1; }
                else { sclt = 0; }
                menu[sclt].select_next();
            }
            Key::Left | Key::Char('h') => {
                menu[sclt].unselect();
                if sclt == 0 { sclt = 1; }
                else { sclt = 0; }
                menu[sclt].select_next();
            }
            Key::Up | Key::Char('k') => menu[sclt].select_prev(),
            Key::Down | Key::Char('j') => menu[sclt].select_next(),
            Key::Char('\n') => match menu[sclt].state.selected() {
                Some(x) => {
                    if sclt == 0 {
                        plugins[x].activate();
                        menu[sclt].update(loadorder::to_strvec(&plugins));
                    }
                    else {
                        let src_p = paths.mods.clone().push(&mods[x]);
                        return Ok(Some(src_p));
                    }
                }
                None => continue,
            }
            Key::Char('w') => match menu[sclt].state.selected() {
                Some(x) => {
                    if sclt == 0 {
                        loadorder::move_up(plugins, x);
                        menu[sclt].update(loadorder::to_strvec(&plugins));
                        menu[sclt].select_prev();
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
    }
    Ok(None)
}  

pub fn fileexplorer(message: &str) -> io::Result<Path> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut path = Path::from("/");
    let mut items = read_datadir(&path)?;
    let mut menu = utils::StateList::from(items.clone());
    
    loop {

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(90),
                    Constraint::Percentage(10),
    
                ].as_ref())
                .split(f.size());
    
            let list = List::new(menu.items.clone())
                .block(
                    Block::default()
                        .title(message)
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

        match utils::keyin() {
            Key::Up | Key::Char('k') => menu.select_prev(),
            Key::Down | Key::Char('j') => menu.select_next(),
            Key::Right | Key::Char('l') => match menu.state.selected() {
                Some(x) => {
                    path.push(&items[x]);
                    items =  read_datadir(&path)?;            
                    menu = utils::StateList::from(items.clone());
                }
                None => continue,
            }
            Key::Left | Key::Char('h') => {
                path = path.previous();
                items =  read_datadir(&path)?;            
                menu = utils::StateList::from(items.clone());
            }
            Key::Char('\n') => match menu.state.selected() {
                Some(x) => {
                    path.push(&items[x]);
                    return Ok(path);
                }
                None => continue,
            }
            _default => continue,

        }
    }
}


pub fn selection_menu(group: &FomodGroup) -> io::Result<Vec<usize>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut p_vec = loadorder::to_plgvec(group.plugins());
    let mut menu = utils::StateList::from(loadorder::to_strvec(&p_vec));

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
    
            let list = List::new(menu.items.clone())
                .block(
                    Block::default()
                        .title(group.gtype.clone())
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

    match utils::keyin() {
        Key::Down | Key::Char('j') => menu.select_next(),
        Key::Up | Key::Char('k') => menu.select_prev(),
        Key::Char('\n') => match menu.state.selected() {
            Some(x) => {
                p_vec[x].activate();
                menu.update(loadorder::to_strvec(&p_vec));
            }
            None => continue,
        }
        Key::Right | Key::Char('l') => if loadorder::any_active(&p_vec) {
            return Ok(loadorder::get_active(&p_vec));
        }
        _default => continue,
    }


    }
}

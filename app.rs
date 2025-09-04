//src/app.rs
use crate::events::{Event, EventHandler};
use crate::file_ops::FileManager;
use crate::ui;
use anyhow::Result;
use ratatui::{backend::Backend, Terminal};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    Visual,
}

pub struct App {
    pub file_manager: FileManager,
    pub current_path: PathBuf,
    pub selected_index: usize,
    pub mode: Mode,
    pub command_buffer: String,
    pub preview_content: String,
    pub error_message: Option<String>,
    pub events: EventHandler,
    pub last_update: Instant,
}

impl App {
    pub fn new() -> Result<Self> {
        let current_path = std::env::current_dir()?;
        let file_manager = FileManager::new(&current_path)?;

        Ok(Self {
            file_manager,
            current_path,
            selected_index: 0,
            mode: Mode::Normal,
            command_buffer: String::new(),
            preview_content: String::new(),
            error_message: None,
            events: EventHandler::new(250), //250ms tick rate
            last_update: Instant::now(),
        })
    }

    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()>{
        loop {
            terminal.draw(|f| ui::draw(f, self))?;

            if let Event::Key(key) = self.events.next().await? {
                if self.handle_key_event(key).await? {
                    break;
                }
            }
            //update preview if needed
            self.update_preview().await;
        }
        Ok(())
    }
    pub async fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<bool>{
        match self.mode {
            Mode::Normal => self.handle_normal_mode(key).await,
            Mode::Insert => self.handle_insert_mode(key).await,
            Mode::Command => self.handle_command_mode(key).await,
            Mode::Visual => self.handle_visual_mode(key).await,
        }
    }
    async fn handle_normal_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<bool>{
        use crossterm::event::{KeyCode, KeyModifiers};
        match key.code {
            KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => return Ok(true),
            KeyCode::Char('j') | KeyCode::Down => self.move_selection(1),
            KeyCode::Char('k') | KeyCode::Up => self.move_selection(-1),
            KeyCode::Char('h') | KeyCode::Left => self.navigate_up(),
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => self.navigate_into(),
            KeyCode::Char('i') => self.mode = Mode::Insert,
            KeyCode::Char(':') => self.mode = Mode::Command,
            KeyCode::Char('v') => self.mode = Mode::Visual,
            KeyCode::Char('d') => self.delete_selected().await?,
            KeyCode::Char('e') => self.edit_selected().await?,
            KeyCode::Char('r') => self.rename_selected().await?,
            KeyCode::Char('n') => self.create_new_file().await?,
            _ => {}
        }
        Ok(false)
    }
    async fn update_preview(&mut self){
        if self.last_update.elapsed() > Duration::from_millis(100){
            if let Some(selected) = self.file_manager.get_selected(self.selected_index){
                if selected.is_file{
                    match tokio::fs::read_to_string(&selected.path).await {
                        Ok(content) => {
                            //Limit preview size
                            self.preview_content = content.chars().take(1000).collect();
                        }
                        Err(_) => {
                            self.preview_content = String::from("(Binary or unreadable file)");
                        }
                    }
                } else{
                    self.preview_content = String::from("(Directory)");
                }
            }
            self.last_update = Instant::now();
        }
    }
    //Other mode handlers and helper methods...
    fn move_selection(&mut self, delta: isize){
        let len = self.file_manager.entries.len();
        if len > 0 {
            self.selected_index = (self.selected_index as isize + delta).rem_euclid(len as isize) as usize;
        }
    }
    fn navigate_up(&mut self){
        if let Some(parent) = self.current_path.parent(){
            if let Ok(manager) = FileManager::new(parent){
                self.file_manager = manager;
                self.current_path = parent.to_path_buf();
                self.selected_index = 0;
            }
        }
    }

    async fn delete_selected(&mut self) -> Result<()>{
        //Implementation for deleting files
        Ok(())
    }

    async fn edit_selected(&mut self) -> Result<()>{
        //Implementation for editing files
        Ok(())
    }
}
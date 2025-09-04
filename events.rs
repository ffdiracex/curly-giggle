//src/events.rs
use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16,u16),
}

pub struct EventHandler {
    rx: mpsc::Receiver<Event>,
    _tx: mpsc::Sender<Event>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let (tx,rx) = mpsc::channel(100);
        let event_tx = tx.clone();

        tokio::spawn(async move {
            let mut last_tick = Instant::now();
            let tick_duration = Duration::from_millis(tick_rate);

            loop {
                let timeout = tick_duration
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(Duration::from_secs(0));

                if event::poll(timeout).unwrap(){
                    match event::read().unwrap(){
                        CrosstermEvent::Key(e) => {
                            if event_tx.send(Event::Key(e)).await.is_err(){
                                break;
                            }
                        }
                        CrosstermEvent::Mouse(e) => {
                            if event_tx.send(Event::Mouse(e)).await.is_err(){
                                break;
                            }
                        }
                        CrosstermEvent::Resize(w,h) => {
                            if event_tx.send(Event::Resize(w,h)).await.is_err(){
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                if last_tick.elapsed() >= tick_duration {
                    if event_tx.send(Event::Tick).await.is_err() {
                        break;
                    }
                    last_tick;
                }
            }
        });
        Self {rx, _tx: tx}
    }

    pub async fn next(&mut self) -> Result<Event>{
        self.rx.recv().await.ok_or(anyhow::anyhow!("Channel closed"))
    }
}

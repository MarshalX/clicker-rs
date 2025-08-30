use crate::config::{ClickButton, ClickType, ClickerConfig, DelayMode};
use crate::constants::*;
use crossbeam::channel::{self, Receiver, Sender};
use enigo::{Button, Enigo, Mouse, Settings};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum TimerCommand {
    Stop,
}

#[derive(Debug, Clone)]
pub enum StatusUpdate {
    Error(String),
}

pub struct PrecisionTimer {
    timer_handle: Option<JoinHandle<()>>,
    status_sender: Option<Sender<StatusUpdate>>,
    command_sender: Option<Sender<TimerCommand>>,
    is_running: Arc<AtomicBool>,
}

impl PrecisionTimer {
    pub fn new() -> Self {
        Self {
            timer_handle: None,
            status_sender: None,
            command_sender: None,
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&mut self, config: ClickerConfig) -> Receiver<StatusUpdate> {
        self.stop();

        let (status_tx, status_rx) = channel::unbounded();
        let (cmd_tx, cmd_rx) = channel::unbounded();

        self.status_sender = Some(status_tx.clone());
        self.command_sender = Some(cmd_tx);

        let is_running = self.is_running.clone();
        is_running.store(true, Ordering::SeqCst);

        let handle = thread::spawn(move || {
            Self::timer_loop(config, status_tx, cmd_rx, is_running);
        });

        self.timer_handle = Some(handle);
        status_rx
    }

    pub fn stop(&mut self) {
        // Signal the thread to stop
        self.is_running.store(false, Ordering::SeqCst);
        
        if let Some(sender) = &self.command_sender {
            let _ = sender.send(TimerCommand::Stop);
        }

        // Don't block the UI thread - let the thread finish naturally
        // The handle will be cleaned up in the Drop implementation
        self.status_sender = None;
        self.command_sender = None;
    }

    fn timer_loop(
        config: ClickerConfig,
        status_sender: Sender<StatusUpdate>,
        command_receiver: Receiver<TimerCommand>,
        is_running: Arc<AtomicBool>,
    ) {
        // Create Enigo ONCE at the start of the timer session
        let mut enigo = match Enigo::new(&Settings::default()) {
            Ok(enigo) => enigo,
            Err(e) => {
                let _ =
                    status_sender.send(StatusUpdate::Error(format!("{}: {}", ERROR_ENIGO_INIT, e)));
                is_running.store(false, Ordering::SeqCst);
                return;
            }
        };

        // Create RNG ONCE at the start of the timer session (for jitter mode)
        let mut rng = rand::rng();

        // Pre-calculate CPS interval ONCE at the start (for CPS mode)
        let cps_interval = Duration::from_nanos((1_000_000_000.0 / config.cps) as u64);

        let mut next_target_time = Instant::now();

        loop {
            if let Ok(TimerCommand::Stop) = command_receiver.try_recv() {
                break;
            }

            if !is_running.load(Ordering::SeqCst) {
                break;
            }

            let now = Instant::now();
            if now < next_target_time {
                let sleep_duration = next_target_time - now;
                thread::sleep(sleep_duration);
            }

            match Self::execute_click_with_enigo(&mut enigo, &config) {
                Ok(_) => {
                    // Click executed successfully - continue silently
                }
                Err(e) => {
                    if let Err(_) = status_sender.send(StatusUpdate::Error(e)) {
                        break;
                    }
                    break;
                }
            }

            next_target_time = match config.delay_mode {
                DelayMode::CPS => next_target_time + cps_interval,
                DelayMode::Jitter => {
                    let delay = Self::generate_jitter_delay_with_rng(&mut rng, &config);
                    next_target_time + delay
                }
            };
        }

        is_running.store(false, Ordering::SeqCst);
    }

    fn generate_jitter_delay_with_rng(
        rng: &mut impl rand::Rng,
        config: &ClickerConfig,
    ) -> Duration {
        let delay_ms = rng.random_range(config.min_delay_ms..=config.max_delay_ms);
        Duration::from_millis(delay_ms)
    }

    fn execute_click_with_enigo(enigo: &mut Enigo, config: &ClickerConfig) -> Result<(), String> {
        let mut click = |dir| {
            enigo
                .button(Self::get_enigo_button(&config.click_button), dir)
                .map_err(|e| format!("{}: {}", ERROR_CLICK_FAILED, e))
        };

        match config.click_type {
            ClickType::Single => {
                click(enigo::Direction::Click)?;
            }
            ClickType::Double => {
                click(enigo::Direction::Click)?;
                thread::sleep(Duration::from_millis(50));
                click(enigo::Direction::Click)?;
            }
            ClickType::Hold => {
                click(enigo::Direction::Press)?;
                thread::sleep(Duration::from_millis(100));
                click(enigo::Direction::Release)?;
            }
        }

        Ok(())
    }

    fn get_enigo_button(click_button: &ClickButton) -> Button {
        match click_button {
            ClickButton::Left => Button::Left,
            ClickButton::Right => Button::Right,
            ClickButton::Middle => Button::Middle,
        }
    }
}

impl Drop for PrecisionTimer {
    fn drop(&mut self) {
        // Signal the thread to stop using both mechanisms
        self.is_running.store(false, Ordering::SeqCst);
        
        if let Some(sender) = &self.command_sender {
            let _ = sender.send(TimerCommand::Stop);
        }

        // Don't join the thread to avoid any blocking
        // The thread will exit naturally when it sees the stop signals
        // Process cleanup will handle any remaining threads
        if let Some(_handle) = self.timer_handle.take() {
            // Just drop the handle - the thread will finish on its own
        }
    }
}

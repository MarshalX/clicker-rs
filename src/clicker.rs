use crate::config::{ClickerConfig, DelayMode};
use crate::timer::{PrecisionTimer, StatusUpdate};
use crossbeam::channel::Receiver;
use iced::Task;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum ClickerMessage {
    ClickError(String),
    NoError,
}

#[derive(Clone)]
pub struct Clicker {
    config: ClickerConfig,
    timer: Arc<std::sync::Mutex<PrecisionTimer>>,
    status_receiver: Arc<std::sync::Mutex<Option<Receiver<StatusUpdate>>>>,
    is_running: Arc<AtomicBool>,
}

impl Clicker {
    pub fn new(config: ClickerConfig) -> Self {
        Self {
            config,
            timer: Arc::new(std::sync::Mutex::new(PrecisionTimer::new())),
            status_receiver: Arc::new(std::sync::Mutex::new(None)),
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&mut self) {
        let mut timer = self.timer.lock().unwrap();
        let receiver = timer.start(self.config.clone());
        *self.status_receiver.lock().unwrap() = Some(receiver);
        self.is_running.store(true, Ordering::SeqCst);
    }

    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::SeqCst);
        let mut timer = self.timer.lock().unwrap();
        timer.stop();
        *self.status_receiver.lock().unwrap() = None;
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    pub fn config(&self) -> &ClickerConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut ClickerConfig {
        &mut self.config
    }

    pub fn get_delay_info(&self) -> String {
        match self.config.delay_mode {
            DelayMode::CPS => {
                format!("{:.1} CPS", self.config.cps)
            }
            DelayMode::Jitter => {
                format!(
                    "Jitter: {}ms - {}ms",
                    self.config.min_delay_ms, self.config.max_delay_ms
                )
            }
        }
    }

    pub async fn check_for_errors(&self) -> Option<ClickerMessage> {
        if let Some(receiver) = self.status_receiver.lock().unwrap().as_ref() {
            match receiver.try_recv() {
                Ok(StatusUpdate::Error(e)) => {
                    self.is_running.store(false, Ordering::SeqCst);
                    Some(ClickerMessage::ClickError(e))
                }
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn create_error_check_task(&self) -> Task<ClickerMessage> {
        let clicker = self.clone();
        Task::perform(
            async move {
                // Small delay to avoid busy polling
                async_std::task::sleep(std::time::Duration::from_millis(100)).await;
                clicker
                    .check_for_errors()
                    .await
                    .unwrap_or(ClickerMessage::NoError)
            },
            |result| result,
        )
    }
}

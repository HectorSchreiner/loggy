#![allow(dead_code)]
use chrono::Utc;
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    *,
};
use std::{io::stdout, marker::PhantomData};

use crate::config::*;
use crate::global::MOGGER;

struct Uninitialized;
struct Initialized;

#[derive(Debug)]
pub struct Mogger {
    pub config: Config,
    pub output_format: Format,
}

impl Mogger {
    // Initializes the mogger, this should be called in all methods that tries to init a mogger
    fn init(mogger: Mogger) {
        MOGGER.set(mogger).expect("Logger already initialized");
    }

    pub fn new(&self, config: Config, output_format: Format) {
        let mogger = Mogger {
            config,
            output_format,
        };
        Self::init(mogger);
    }

    pub fn default() {
        let config = Config::builder()
            .timeformat(Some(TimeFormatType::ClockDateMonthYear))
            .level_format(Some(LevelFormatType::Default))
            .build();

        let mogger = Mogger {
            config,
            output_format: Format::PlainText,
        };

        Self::init(mogger);
    }

    pub fn log(&self, level: Level, message: &str) {
        match self.config.output {
            OutputType::Console => Self::console_write(&self, level, message),
        }
    }

    fn console_write(&self, level: Level, message: &str) {
        self.console_write_level(level);
        self.console_write_time();

        println!("{}", message);
    }

    fn console_write_level(&self, level: Level) {
        match &self.config.level_option {
            Some(_) => {
                let _ = match level {
                    Level::Debug => execute!(stdout(), Print(format!("[{:?}] ", level)),).unwrap(),
                    Level::Info => execute!(
                        stdout(),
                        SetForegroundColor(Color::White),
                        Print(format!("[{:?}] ", level)),
                    )
                    .unwrap(),
                    Level::Warning => execute!(
                        stdout(),
                        SetForegroundColor(Color::Yellow),
                        Print(format!("[{:?}] ", level)),
                    )
                    .unwrap(),
                    Level::Error => execute!(
                        stdout(),
                        SetForegroundColor(Color::Red),
                        Print(format!("[{:?}] ", level)),
                    )
                    .unwrap(),
                };
            }
            None => (), // we might want to do something else if there is a none
        }
        execute!(stdout(), ResetColor).unwrap();
    }

    fn console_write_time(&self) {
        match &self.config.time_option {
            Some(_) => {
                execute!(stdout(), Print(format!("[{:?}] ", Self::get_time(&self)))).unwrap()
            }
            None => (), // we might want to do something else if there is a none
        }
    }

    fn get_time(&self) -> String {
        let time = Utc::now();
        let mut formatted = "".to_string();

        if let Some(time_option) = &self.config.time_option {
            match time_option {
                TimeFormatType::Default => formatted = format!("{}", time.format("%H:%M")),
                TimeFormatType::ClockDateMonthYear => {
                    formatted = format!("{}", time.format("%H:%M %d/%m/%Y"))
                }
            }
        }
        format!("{}", formatted)
    }
}

#[derive(Debug)]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
pub enum Format {
    PlainText,
}

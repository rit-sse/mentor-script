use chrono::{Local, Timelike};
use eframe::egui::{CentralPanel, Context};
use eframe::Frame;
use crate::config::Config;
use crate::scheduler::{check_time, CheckType};

pub struct MentorApp {
    config: Config,
    last_trigger: Option<CheckType>,
}

impl MentorApp {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            last_trigger: None,
        }
    }
}

impl eframe::App for MentorApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        let now = Local::now();
        let minute = now.minute();

        if let Some(check) = check_time() {
            if self.last_trigger != Some(check) {
                match check {
                    CheckType::HalfHour => {
                        println!("30-minute check!");
                    }
                    CheckType::Hour => {
                        println!("Hourly check!");
                    }
                }

                self.last_trigger = Some(check);
            }
        }

        // Resetting trigger once minute passes
        if minute != 30 && minute != 55 {
            self.last_trigger = None;
        }

        let next_check = if minute < 30 {
            30 - minute
        } else if minute < 55 {
            55 - minute
        } else {
            60 - minute + 30
        };

        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading(format!(
                    "{:02}:{:02}",
                    now.hour(),
                    now.minute(),
                ));

                ui.add_space(10.0);
                ui.label(format!(
                    "Next check in {} minutes",
                    next_check
                ));

                ui.add_space(20.0);
                ui.separator();
                ui.label(&self.config.mentor_text);
            });
        });

        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}


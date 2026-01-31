use chrono::{DateTime, Local, Timelike};
use eframe::egui::{CentralPanel, Context};
use eframe::Frame;
use crate::config::Config;
use crate::scheduler::{minutes_until_next_check, CheckType};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ReminderState {
    Idle,
    Pending(CheckType), // upcoming
    Active(CheckType), // currently watching
    Snoozed(CheckType),
}

pub struct MentorApp {
    config: Config,
    state: ReminderState,
    snooze_until: Option<DateTime<Local>>,
}

impl MentorApp {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            state: ReminderState::Idle,
            snooze_until: None,
        }
    }

    fn update_state(&mut self) {
        let now = Local::now();

        if let Some(until) = self.snooze_until {
            if now < until {
                return;
            } else {
                self.snooze_until = None;
                self.state = ReminderState::Idle;
            }
        }

        let (next_check, minutes_until) = minutes_until_next_check(now);

        // Pending window for next reminder
        if minutes_until <= 5 && minutes_until > 0 {
            if matches!(self.state, ReminderState::Idle | ReminderState::Pending(_)) {
                self.state = ReminderState::Pending(next_check);
            }
        }

        // Moment reminder goes off
        if minutes_until == 0 {
           if !matches!(self.state, ReminderState::Active(_)) {
               self.state = ReminderState::Active(next_check);
           }
        }
    }

}

impl eframe::App for MentorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.update_state();

        let now = Local::now();
        let _minute = now.minute();


        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading(format!(
                    "{:02}:{:02}",
                    now.hour(),
                    now.minute(),
                ));

                ui.add_space(10.0);

                match self.state {
                    ReminderState::Idle => {
                        ui.label("All caught up!");
                    }

                    ReminderState::Pending(check) => {
                        ui.label("Upcoming check");
                        ui.label(format!("{:?} in a few minutes", check));
                    }

                    ReminderState::Active(check) => {
                        ui.heading("Time to check in");
                        ui.label(format!("{:?}", check));

                        if ui.button("Checked").clicked() {
                            self.state = ReminderState::Idle;
                        }

                        if ui.button("Snooze 5 min").clicked() {
                            self.snooze_until = Some(Local::now() + chrono::Duration::minutes(5));
                            self.state = ReminderState::Snoozed(check);
                        }
                    }

                    ReminderState::Snoozed(_) => {
                        ui.label("Snoozed ⏰");
                    }
                }

                ui.add_space(20.0);
                ui.separator();
                ui.label(&self.config.mentor_text);
            });
        });

        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }


}


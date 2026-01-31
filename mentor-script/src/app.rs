use chrono::{DateTime, Local, Timelike};
use eframe::egui::{CentralPanel, Context};
use eframe::{egui, Frame};
use rodio::{OutputStream, OutputStreamBuilder};
use crate::config::Config;
use crate::scheduler::{check_time, minutes_until_next_check, CheckType};
use crate::sound;
use crate::sound::pick_random_song;

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
    _audio_stream: OutputStream,
}

impl MentorApp {
    pub fn new(config: Config) -> Self {
        let stream =
            OutputStreamBuilder::open_default_stream().unwrap();

        Self {
            config,
            state: ReminderState::Idle,
            snooze_until: None,
            _audio_stream: stream,
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

        // Pending check before time hits the exact moment
        let (_, minutes_until) = minutes_until_next_check(now);
        if minutes_until <= 5 && minutes_until > 0 {
            if matches!(self.state, ReminderState::Idle | ReminderState::Pending(_)) {
                let (next_check, _) = minutes_until_next_check(now);
                self.state = ReminderState::Pending(next_check);
            }
        }

        // Moment reminder goes off
       if let Some(check) = check_time() {
           if !matches!(self.state, ReminderState::Active(_)) {
               self.state = ReminderState::Active(check);
           }
       }
    }

    fn background_color(&self) -> egui::Color32 {
        match self.state {
            ReminderState::Idle => egui::Color32::from_rgb(30, 30, 30),
            ReminderState::Pending(_) => egui::Color32::from_rgb(80, 70, 20),
            ReminderState::Active(_) => egui::Color32::from_rgb(90, 30, 30),
            ReminderState::Snoozed(_) => egui::Color32::from_rgb(40, 50, 70),
        }
    }

}

impl eframe::App for MentorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.update_state();

        let now = Local::now();
        let _minute = now.minute();


        CentralPanel::default()
            .frame(egui::Frame::new().fill(self.background_color()))
            .show(ctx, |ui| {
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


                        if let Some(url) = pick_random_song(&self.config.song_urls) {
                            sound::play_sound(&self._audio_stream, url);
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


use crate::config::Config;
use crate::scheduler::{check_time, minutes_until_next_check, CheckType};
use crate::sound::Audio;
use chrono::{DateTime, Local, Timelike};
use eframe::egui::{CentralPanel, Context};
use eframe::{egui, Frame};
use rand::seq::IndexedRandom;
use rodio::Sink;

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
    last_state: ReminderState,
    snooze_until: Option<DateTime<Local>>,
    trigger_consumed: bool,
    audio: Option<Audio>,
    current_sink: Option<Sink>,
}

impl MentorApp {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            state: ReminderState::Idle,
            last_state: ReminderState::Idle,
            snooze_until: None,
            trigger_consumed: false,
            audio: None,
            current_sink: None,
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

        if check_time().is_none() {
            self.trigger_consumed = false;
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


        if self.last_state != self.state {
            if matches!(self.state, ReminderState::Active(_)) {
                if self.audio.is_none() {
                    self.audio = Audio::new();
                }

                if let (Some(audio), Some(path)) =
                    (&self.audio, self.config.songs.choose(&mut rand::rng()).cloned())
                {
                    self.current_sink = audio.play_file(path);
                }
            }

            self.last_state = self.state;
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
        if ctx.input(|i| i.key_pressed(egui::Key::A)) {
            self.state = ReminderState::Active(CheckType::Hour);
            self.trigger_consumed = true;
        }

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

                        if ui.button("Open Form").clicked() {
                            let url = match check {
                                CheckType::Hour => &self.config.hourly_link,
                                CheckType::HalfHour => &self.config.thirty_link,
                            };

                            let _ = webbrowser::open(url);
                        }

                        if ui.button("Checked").clicked() {
                            if let Some(sink) = self.current_sink.take() {
                                sink.stop();
                            }
                            self.state = ReminderState::Idle;

                        }

                        if ui.button("Snooze 5 min").clicked() {
                            if let Some(sink) = self.current_sink.take() {
                                sink.stop();
                            }
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


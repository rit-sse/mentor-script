//! Main application GUI and state management
//!
//! Manages the reminder state machine and renders the user interface.

use std::time::Duration;
use crate::config::Config;
use crate::scheduler::{check_time, minutes_until_next_check, CheckType};
use crate::sound::Audio;
use chrono::{Local, Timelike};
use eframe::egui::{CentralPanel, Context};
use eframe::{egui, Frame};
use egui::{Color32, RichText};
use rand::seq::IndexedRandom;
use rodio::Sink;

/// Current state of the reminder system
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ReminderState {
    /// No active or upcoming reminders
    Idle,
    /// Reminder coming in the next 5 minutes
    Pending(CheckType),
    /// Reminder is currently active, waiting for user action
    Active(CheckType),
}

/// Main application struct managing GUI and state
pub struct MentorApp {
    config: Config,
    state: ReminderState,
    last_state: ReminderState,
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
            trigger_consumed: false,
            audio: None,
            current_sink: None,
        }
    }


    /// Updates the reminder state based on current time and plays audio when transitioning to Active
    fn update_state(&mut self) {
        const PENDING_WINDOW_MINUTES: i64 = 5;

        let now = Local::now();
        let current_trigger = check_time();

        // Reset consumption once we're no longer on a trigger minute.
        if current_trigger.is_none() {
            self.trigger_consumed = false;
        }

        // Pending check shortly before the trigger moment.
        let (next_check, minutes_until) = minutes_until_next_check(now);
        let in_pending_window = (1..=PENDING_WINDOW_MINUTES).contains(&minutes_until);
        if in_pending_window && matches!(self.state, ReminderState::Idle | ReminderState::Pending(_)) {
            self.state = ReminderState::Pending(next_check);
        }

        // Moment reminder goes off.
        if let Some(check) = current_trigger {
            let is_already_active = matches!(self.state, ReminderState::Active(_));
            if !is_already_active && !self.trigger_consumed {
                self.state = ReminderState::Active(check);
                self.trigger_consumed = true;
            }
        }

        // React to state transitions.
        if self.last_state != self.state {
            if matches!(self.state, ReminderState::Active(_)) {
                self.audio = self.audio.take().or_else(Audio::new);

                if let (Some(audio), Some(path)) = (
                    self.audio.as_ref(),
                    self.config.songs.choose(&mut rand::rng()).cloned(),
                ) {
                    self.current_sink = audio.play_file(path);
                }
            }

            self.last_state = self.state;
        }
    }


    /// Returns a dynamic, breathing RGB effect background
    fn background_color(&self, t: f64) -> egui::Color32 {
        let speed = 0.2;
        let phase = t * speed;

        let r = (phase.sin() * 0.5 + 0.5) as f32;
        let g = ((phase + 2.0).sin() * 0.5 + 0.5) as f32;
        let b = ((phase + 4.0).sin() * 0.5 + 0.5) as f32;

        // soften it
        let base = 30.0;
        let range = 40.0;

        egui::Color32::from_rgb(
            (base + r * range) as u8,
            (base + g * range) as u8,
            (base + b * range) as u8,
        )
    }

}

impl eframe::App for MentorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.update_state();
        if ctx.input(|i| i.key_pressed(egui::Key::A)) {
            self.state = ReminderState::Active(CheckType::HalfHour);
            self.trigger_consumed = true;
        }

        let now = Local::now();
        let _minute = now.minute();

        let time = ctx.input(|i| i.time); // variable time for dynamic color

        CentralPanel::default()
            .frame(egui::Frame::new().fill(self.background_color(time)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.heading(RichText::new(format!(
                        "{:02}:{:02}",
                        now.hour(),
                        now.minute(),
                    )).size(42.0).color(Color32::from_hex("#000000").unwrap()));

                    ui.add_space(10.0);

                    match self.state {
                        ReminderState::Idle => {
                            ui.label("All caught up!");
                        }

                        ReminderState::Pending(check) => {
                            ui.label("Upcoming check");
                            ui.label(format!("{} coming up!", check));
                        }

                        ReminderState::Active(check) => {
                            ui.heading("Time to check in");
                            ui.label(format!("{}!", check));

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
                        }
                    }

                    ui.add_space(20.0);

                    let rect = ctx.content_rect();
                    let center = rect.center();
                    #[allow(deprecated)]
                    ui.allocate_ui_at_rect(
                        egui::Rect::from_center_size(
                            center,
                            egui::vec2(300.0, 80.0),
                        ),
                        |ui| {
                            ui.label(egui::RichText::new(&self.config.mentor_text)
                                .color(Color32::from_hex("#23F123").unwrap())
                                .strong()
                                .size(24.0)
                            );
                        },
                    );


                });
            });

        let repaint_delay = match self.state {
            ReminderState::Idle => Duration::from_millis(33),      // smooth breathing
            ReminderState::Pending(_) => Duration::from_millis(100),
            ReminderState::Active(_) => Duration::from_millis(500),
        };

        ctx.request_repaint_after(repaint_delay);
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.audio = None; // Safely drop audio stream
    }
}


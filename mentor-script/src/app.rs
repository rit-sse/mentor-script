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
use egui::{vec2, Button, Color32, RichText};
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
    after_hours: bool,
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
            after_hours: false,
        }
    }


    /// Updates the reminder state based on current time and plays audio when transitioning to Active
    fn update_state(&mut self) {
        const PENDING_WINDOW_MINUTES: i64 = 5;

        let now = Local::now();
        let current_trigger = check_time();

        // After hours
        if now.hour() < 10 || now.hour() >= 18 {
            self.state = ReminderState::Idle;
            self.after_hours = true;
            return;
        } else {
            self.after_hours = false;
        }

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
    fn background_color(&self, t: f32) -> Color32 {
        let speed: f32 = 0.2;
        let phase: f32 = t * speed;

        let r = phase.sin() * 0.5 + 0.5;
        let g = (phase + 2.0).sin() * 0.5 + 0.5;
        let b = (phase + 4.0).sin() * 0.5 + 0.5;

        // soften it
        let base = 30.0;
        let range = 40.0;

        Color32::from_rgb(
            (base + r * range) as u8,
            (base + g * range) as u8,
            (base + b * range) as u8,
        )
    }

    fn invert_color(color: Color32) -> Color32 {
        let [r, g, b, a] = color.to_array();
        Color32::from_rgba_unmultiplied(255 - r, 255 - g, 255 - b, a)
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

        let time: f32 = ctx.input(|i| i.time) as f32; // variable time for dynamic color
        let bg_color = self.background_color(time);
        let time_color = Self::invert_color(bg_color);


        CentralPanel::default()
            .frame(egui::Frame::new().fill(bg_color))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.heading(RichText::new(format!(
                        "{:02}:{:02}",
                        now.hour(),
                        now.minute(),
                    ))
                        .size(42.0)
                        .color(time_color)
                    );

                    ui.add_space(10.0);

                    // Reserve a consistent "header area" that we can fill differently based on time/state.
                    let rect = ctx.content_rect();
                    let center = rect.center();
                    let header_rect = egui::Rect::from_center_size(center, egui::vec2(600.0, 200.0));

                    match self.state {
                        ReminderState::Idle => {
                            // After hours in lab
                            #[allow(deprecated)]
                            ui.allocate_ui_at_rect(header_rect, |ui| {
                                ui.vertical_centered(|ui| {
                                    if self.after_hours {
                                        ui.heading(
                                            RichText::new("After hours 😎")
                                                .color(Color32::from_hex("#3c7a89").unwrap())
                                                .size(42.0)
                                                .strong(),
                                        );
                                    } else {
                                        ui.label(RichText::new(&self.config.mentor_text)
                                            .color(Color32::from_hex("#23F123").unwrap())
                                            .strong()
                                            .size(48.0)
                                        );
                                    };
                                });
                            });
                        }

                        ReminderState::Pending(check) => {
                            let (_, minutes_until) = minutes_until_next_check(now);

                            // Convert "minutes until next check (rounded down to minute)" into seconds-until.
                            // If next check is at the next minute boundary, this works well:
                            // seconds_until = minutes_until*60 - current_seconds.
                            let seconds_until =
                                (minutes_until * 60 - now.second() as i64).max(0);

                            let mins = seconds_until / 60;
                            let secs = seconds_until % 60;

                            ui.label(
                                RichText::new("Upcoming Check")
                                    .color(Color32::from_hex("#f39c12").unwrap())
                                    .size(28.0)
                                    .strong(),
                            );

                            ui.label(
                                RichText::new(format!(
                                    "{} in {}:{:02}",
                                    check, mins, secs
                                ))
                                    .color(Color32::from_hex("#23F123").unwrap())
                                    .size(20.0),
                            );

                            // Progress bar over the 5-minute pending window (300s).
                            ui.add_space(10.0);
                            let total_pending_seconds = 5.0 * 60.0;
                            let progress = 1.0 - (seconds_until as f32 / total_pending_seconds);
                            let progress = progress.clamp(0.0, 1.0);

                            ui.add(
                                egui::ProgressBar::new(progress)
                                    .desired_width(200.0),
                            );
                        }

                        ReminderState::Active(check) => {
                            // Pulsing effect for heading
                            let pulse = (time * 2.0).sin() * 0.15 + 0.85;
                            let heading_alpha = (pulse * 255.0) as u8;

                            ui.heading(RichText::new("⚠ Time to check in! ⚠")
                                .color(Color32::from_rgba_unmultiplied(35, 241, 35, heading_alpha))
                                .size(48.0));
                            ui.label(RichText::new(format!("{}", check))
                                .color(Color32::from_hex("#23F123").unwrap())
                                .size(24.0));

                            ui.add_space(60.0);

                            // Centered button layout using relative spacing
                            ui.horizontal(|ui| {
                                // Center the buttons horizontally
                                let available_width = ui.available_width();
                                let button_width = 120.0;
                                let gap = 20.0;
                                let total_width = button_width * 2.0 + gap;
                                let left_padding = (available_width - total_width) / 2.0;

                                ui.add_space(left_padding);

                                let open_button = egui::Button::new(
                                    RichText::new("Open Form").size(16.0).strong()
                                )
                                    .fill(Color32::from_hex("#3498db").unwrap())
                                    .min_size(egui::vec2(button_width, 60.0))
                                    .corner_radius(8.0);

                                if ui.add(open_button).clicked() {
                                    let url = match check {
                                        CheckType::Hour => &self.config.hourly_link,
                                        CheckType::HalfHour => &self.config.thirty_link,
                                    };

                                    let _ = webbrowser::open(url);
                                }

                                ui.add_space(gap);

                                let checked_button = Button::new(
                                    RichText::new("Checked").size(16.0).strong()
                                )
                                    .fill(Color32::from_hex("#27ae60").unwrap())
                                    .min_size(egui::vec2(button_width, 60.0))
                                    .corner_radius(8.0);

                                let pause_button: Button = Button::new(
                                    RichText::new("Pause Music").size(16.0).strong()
                                ).fill(Color32::from_hex("#780000").unwrap())
                                    .min_size(vec2(button_width, 100.0))
                                    .corner_radius(8.0);

                                if ui.add(pause_button).clicked() && let Some(sink) = self.current_sink.as_ref() {
                                        if sink.is_paused() {
                                            sink.play();
                                        } else {
                                            sink.pause();
                                        }
                                }

                                if ui.add(checked_button).clicked() {
                                    if let Some(sink) = self.current_sink.take() {
                                        sink.stop();
                                        sink.detach();
                                    }
                                    self.state = ReminderState::Idle;
                                }
                            });
                        }
                    }

                    ui.add_space(20.0);

                });

                // Add small "Open Songs Folder" button in bottom right corner
                let rect = ctx.content_rect();
                let button_size = egui::vec2(140.0, 35.0);
                let margin = 15.0;
                #[allow(deprecated)]
                ui.allocate_ui_at_rect(
                    egui::Rect::from_min_size(
                        egui::Pos2::new(
                            rect.max.x - button_size.x - margin,
                            rect.max.y - button_size.y - margin,
                        ),
                        button_size,
                    ),
                    |ui| {
                        ui.horizontal_centered(|ui| {
                            let folder_button = egui::Button::new(
                                RichText::new("📁 Songs").size(18.0)
                            )
                                .fill(Color32::from_rgba_unmultiplied(52, 152, 219, 180))
                                .min_size(button_size)
                                .corner_radius(8.0);

                            if ui.add(folder_button).clicked() {
                                Config::open_songs_folder();
                            }
                        })
                    },
                );
            });

        let repaint_delay = match self.state {
            ReminderState::Idle => Duration::from_millis(33),      // smooth breathing
            ReminderState::Pending(_) => Duration::from_secs(1), // repaint once per second
            ReminderState::Active(_) => Duration::from_millis(33), // smooth pulsing
        };

        ctx.request_repaint_after(repaint_delay);
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.audio = None; // Safely drop audio stream
    }
}

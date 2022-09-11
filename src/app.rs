#![allow(clippy::all)]
use std::time::Duration;

use eframe::egui::{self, Visuals, Window};
use egui_notify::{Anchor, Toast, Toasts};

mod api_handler; //Imports the API handler
use api_handler::*;

use crate::APP_NAME;

pub struct MyApp {
    //Enter global values to be used with your app here
    message: String,
    toasts: Toasts,
    closable: bool,
    duration: f32,
    webhook: String,
    username: String,
    avatar_url: String,
}

impl Default for MyApp {
    //defaults for your global values
    fn default() -> Self {
        Self {
            //enter global default values here
            message: "".to_string(),
            toasts: Toasts::default().with_anchor(Anchor::TopRight),
            closable: true,
            duration: 3.5,
            webhook: "".to_string(),
            username: "Xanthus".to_string(),
            avatar_url: "https://cdn.discordapp.com/avatars/892723824297119754/fdd67fef581729a0224f7bf9e8a52d3b.png?size=1024".to_string(),
        }
    }
}

// The env! macro gets the variable at compile time.
const CURRENT_BUILD: &str = env!("CARGO_PKG_VERSION");
const REPO_URL: &str = env!("CARGO_PKG_REPOSITORY");

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Window::new(APP_NAME).show(ctx, |ui| {
                ui.style_mut().visuals = Visuals::dark(); // Makes the buttons dark
                ctx.set_visuals(egui::Visuals::dark()); // Make the ui dark
                egui::warn_if_debug_build(ui);

                ui.horizontal(|ui| {
                    ui.label("Current build:");
                    ui.hyperlink_to(CURRENT_BUILD, REPO_URL);
                });

                ui.add_space(10.0);

                let cb = |t: &mut Toast| {
                    //Callback for the toast
                    t.set_closable(self.closable)
                        .set_duration(Some(Duration::from_millis((1000. * self.duration) as u64)));
                };

                self.message = self.message.replace("\n", "");

                ui.label("Hello and welcome to webhook sender!");
                ui.label(
                "You can randomly generate an insult, affirmation or write your own message in the boxes below!");

                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Enter Webhook URL ");
                    ui.text_edit_singleline(&mut self.webhook);
                });
                ui.horizontal(|ui| {
                    ui.label("Enter Username ");
                    ui.add_space(22.0);
                    ui.text_edit_singleline(&mut self.username);
                });
                ui.horizontal(|ui| {
                    ui.label("Enter Avatar URL ");
                    ui.add_space(18.0);
                    ui.text_edit_singleline(&mut self.avatar_url);
                });
                ui.horizontal(|ui| {
                    ui.label("Enter a message ");
                    ui.add_space(23.0);
                    ui.text_edit_multiline(&mut self.message);
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Your message:");
                    ui.label(&self.message);
                });

                ui.horizontal(|ui| {
                    let generate_button = ui.button("Generate an insult");
                    if generate_button.clicked() {
                        self.message = get_insult();
                        cb(self.toasts.success("Generation Successful!")); //Sends a success toast
                    }
                    let generate_affirmation = ui.button("Generate an affirmation");
                    if generate_affirmation.clicked() {
                        self.message = get_affirmation();
                        cb(self.toasts.success("Generation Successful!"));
                    }
                    let send_button = ui.button("Send message");

                    if send_button.clicked() && self.webhook != "" {
                        send_message(&self.message, &self.webhook, &self.username, &self.avatar_url);
                        cb(self.toasts.success("Message Sent!"));
                        self.message = "".to_string();
                    }

                    else if send_button.clicked() {
                        cb(self.toasts.error("Please enter a wehbook url"));
                    }
                });
                self.toasts.show(ctx); // Requests to render toasts
            });
        });
    }
}
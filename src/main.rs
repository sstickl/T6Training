// LIBRARIES
//use std::env;
//use std::fs;
use eframe::*;
use egui::{CentralPanel, Ui};

// MODS
mod boldface;

// Structs
struct T6App{}

impl eframe::App for T6App{ // Want our app to run off of eframe
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.vertical_centered(|ui| {
                //Title
                ui.style_mut().text_styles.get_mut(&egui::TextStyle::Heading).unwrap().size = 64.0;
                ui.heading("T6 Boldface Training App");

                //Buttons
                if ui.button("Random Quiz").clicked() {
                    println!("Button 1 clicked!");
                }

                if ui.button("Specific Procedures").clicked() {
                    println!("Button 2 clicked!");
                }
                if ui.button("Operational Data").clicked() {
                    println!("Button 3 clicked!");
                }
             });

        });
    }
}

impl Default for T6App {
    fn default() -> Self {
        Self {
        }
    }
}

fn main() -> eframe::Result<(), eframe::Error> {
    let boldfaceops: Vec<Vec<String>> = boldface::init_boldface_db();

    for emergencyop in boldfaceops.iter() {
        println!("{}", emergencyop[0]);
        
        for step in emergencyop.iter().skip(1) {
            println!("\t{}", step);
        }
    }

    run_native(
        "T6 App", 
        NativeOptions::default(), 
        Box::new(|_cc: &CreationContext<'_>| {
            Ok(Box::<T6App>::default())
        }),
    )
}
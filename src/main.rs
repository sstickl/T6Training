// LIBRARIES
//use std::env;
//use std::fs;
use eframe::*;
use egui::{CentralPanel, Ui};

// MODS
mod boldface;

/// A custom widget for a hidden label that reveals text when clicked.
fn hidden_label(ui: &mut egui::Ui, text: &str, is_revealed: &mut bool) {
    egui::Frame::none()
    .inner_margin(egui::Margin::same(5.0)) // Add padding
    .show(ui, |ui| {
        // Set a fixed height for the widget
        ui.set_height(15.0);
        if *is_revealed {
            // Display the revealed text
            if ui
                .add(egui::Button::new(egui::RichText::new(text))
                    .frame(false)) // Clickable blacked-out text
                .clicked()
            {
                *is_revealed = false; // Reveal the text when clicked
            }
        } else {
            // Display a blacked-out placeholder
            if ui
                .add(egui::Button::new(egui::RichText::new(text).color(egui::Color32::from_rgb(0, 0, 0)))
                    .fill(egui::Color32::from_rgb(0, 0, 0))
                    .frame(false)) // Clickable blacked-out text
                .clicked()
            {
                *is_revealed = true; // Reveal the text when clicked
            }
        }
    });
}

// Structs
struct T6App{
    current_screen: Screen, //tracks the current screen
    boldface_ops: Vec<Vec<String>>,
    boldface_number: usize,
    hidden_display: Vec<bool>,
}

#[derive(Default, PartialEq)]
enum Screen {
    #[default]
    MainMenu,
    QuizScreen,
    BoldFaceViewer,
}

impl eframe::App for T6App{ // Want our app to run off of eframe
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        match self.current_screen {
            Screen::MainMenu => self.render_main_menu(ctx),
            Screen::QuizScreen => self.render_quiz_screen(ctx),
            Screen::BoldFaceViewer => self.render_bf_viewer(ctx)
        }
    }
}

impl Default for T6App {
    fn default() -> Self {
        Self {
            current_screen: Screen::MainMenu,  //Start on the main menu
            boldface_ops: boldface::init_boldface_db(),
            boldface_number: 0,
            hidden_display: [false, false, false].to_vec(),
        }
    }
}

impl T6App {
    fn render_main_menu(&mut self, ctx: &egui::Context){
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui|
            {
                ctx.set_pixels_per_point(2.5);
                //Title
                ui.style_mut().text_styles.get_mut(&egui::TextStyle::Heading).unwrap().size = 32.0;
                ui.heading("T6 Boldface Training App");
                ui.add_space(50.0); // Adds 50 pixels of vertical space

                //Buttons
                ui.scope(|ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                    ui.spacing_mut().item_spacing.y = 30.0;
                    if ui.button("Random Quiz").clicked() {
                        self.current_screen = Screen::QuizScreen;
                    }

                    if ui.button("Boldface Viewer").clicked() {
                        self.current_screen = Screen::BoldFaceViewer;
                    }
                    if ui.button("Operational Data").clicked() {
                        println!("Button 3 clicked!");
                    }
                });
                ui.allocate_space(ui.available_size());
             });
        });
    }
    fn render_quiz_screen(&mut self, ctx: &egui::Context){
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading("Quiz Screen");

            // Display a question
            ui.label("Question 1: ?");
            ui.separator();
    
            // Display multiple-choice options
            if ui.button("A) A").clicked() {
                println!("Incorrect!");
            }
            if ui.button("B) B").clicked() {
                println!("Correct!");
            }
            if ui.button("C) C").clicked() {
                println!("Incorrect!");
            }
    
            // Button to go back to the main menu
            if ui.button("Back to Main Menu").clicked() {
                self.current_screen = Screen::MainMenu;
            }
        });
    }
    fn render_bf_viewer(&mut self, ctx: &egui::Context){
        CentralPanel::default().show(ctx, |ui: &mut Ui| {

            ui.heading("Boldface Op Viewer");

            ui.add_space(10.0); // Adds 50 pixels of vertical space

            // Display a question
            //ui.add_sized([300, 40], ui.label(format!("Boldface for: {}", self.boldface_ops[self.boldface_number][0])));
            ui.scope(|ui| {
                ui.set_min_height(35.0);
                ui.label(format!("Boldface for: {}", self.boldface_ops[self.boldface_number][0]));
            });
            ui.separator();
            ui.scope(|ui| {
                ui.set_min_height(120.0);
                for (index, step) in self.boldface_ops[self.boldface_number].iter().enumerate().skip(1) {
                    let mut label_string = String::from(format!("{}", step));
                    hidden_label(ui, &mut label_string, &mut self.hidden_display[index-1]);
                }
            });

            ui.add_space(50.0); // Adds 50 pixels of vertical space
    
            // Display multiple-choice options
            if ui.button("Next Procedure").clicked() {
                if self.boldface_number<9 { self.boldface_number+=1; self.hidden_display = [false, false, false].to_vec() }
            }
            if ui.button("Previous Procedure").clicked() {
                if self.boldface_number>0 { self.boldface_number=1; self.hidden_display = [false, false, false].to_vec() }
            }
    
            // Button to go back to the main menu
            if ui.button("Back to Main Menu").clicked() {
                self.current_screen = Screen::MainMenu;
            }
        });
    }
}

fn main() -> eframe::Result<(), eframe::Error> {
    /*let boldfaceops: Vec<Vec<String>> = boldface::init_boldface_db();

    for emergencyop in boldfaceops.iter() {
        println!("{}", emergencyop[0]);
        
        for step in emergencyop.iter().skip(1) {
            println!("\t{}", step);
        }
    }*/

    run_native(
        "T6 App", 
        NativeOptions::default(), 
        Box::new(|_cc: &CreationContext<'_>| {
            Ok(Box::<T6App>::default())
        }),
    )
}
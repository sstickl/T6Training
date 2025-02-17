// LIBRARIES
//use std::env;
//use std::fs;
use eframe::{run_native, NativeOptions, CreationContext};
//use eframe::egui;
use egui::{CentralPanel, Ui};
use std::collections::HashMap;
//use image;

// MODS
mod boldface;
mod graphics;

// Structs
struct T6App{
    current_screen: Screen, //tracks the current screen
    boldface_ops: Vec<Vec<String>>,
    op_data: HashMap<String, HashMap<String, Vec<String>>>,
    boldface_number: usize,
    hidden_display: Vec<bool>,
    answers: Vec<String>,
    correct_answers: Vec<String>,
    answered: bool,
    section: String,
    ops_section: boldface::BfOpdataEnum,
    //image_texture: Option<egui::TextureId>,
}

#[derive(Default, PartialEq)]
enum Screen {
    #[default]
    MainMenu,
    QuizScreen,
    BoldFaceViewer,
}

impl eframe::App for T6App{ // Want our app to run off of eframe
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.current_screen {
            Screen::MainMenu => self.render_main_menu(ctx),
            Screen::QuizScreen => self.render_queried_op_quizzer(ctx, self.ops_section.as_str()),
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
            answers: Vec::<String>::new(),
            answered: false,
            op_data: boldface::init_bf_opdata_db(),
            correct_answers: Vec::<String>::new(),
            section: "Prohibited Manuevers".to_string(),
            ops_section: boldface::BfOpdataEnum::Engine,
            //image_texture: None,
        }
    }
}

impl T6App {
    fn render_main_menu(&mut self, ctx: &egui::Context){


        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui|
            {
                //Scale up UI, it's really small by default
                //ctx.set_pixels_per_point(2.5);  //is there a better solution?  this "jumps" a few seconds after the first render
                //let available_size = ui.available_size();

                //Title
                ui.style_mut().text_styles.get_mut(&egui::TextStyle::Heading).unwrap().size = 32.0;
                ui.heading("T6 Boldface Training App");
                ui.add_space(50.0); // Adds 50 pixels of vertical space
                
                // Display the image
                ui.add(
                    egui::Image::new(egui::include_image!("T6RA.jpeg"))
                        .max_height(200.0)
                ).on_hover_text_at_pointer("T6 Photo by USAF MSgt David Richards");
                ui.add_space(50.0); // Adds 50 pixels of vertical space

                //Buttons
                ui.scope(|ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                    ui.spacing_mut().item_spacing.y = 30.0;
                    if ui.button("Specific Quiz").clicked() {
                        boldface::BfOpdataEnum::match_text(&mut self.ops_section,self.section.to_string().as_str());
                        self.setup_queried_op_quizzer(self.section.to_string().as_str());
                        self.current_screen = Screen::QuizScreen;
                    }
                    let _response = ui.add(egui::TextEdit::singleline(&mut self.section).desired_width(80.0));

                    if ui.button("Boldface Viewer").clicked() {
                        self.current_screen = Screen::BoldFaceViewer;
                    }
                    if ui.button("Operational Data").clicked() {
                        self.setup_queried_op_quizzer(self.ops_section.as_str());
                        println!("Section: {}", self.ops_section.as_str());
                        self.current_screen = Screen::QuizScreen;
                    }
                });
                ui.allocate_space(ui.available_size());
             });
        });
    }
    /*fn render_quiz_screen(&mut self, ctx: &egui::Context){
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
    }*/
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
                    graphics::hidden_label(ui, &mut label_string, &mut self.hidden_display[index-1]);
                }
            });

            ui.add_space(50.0); // Adds 50 pixels of vertical space
    
            ui.columns(3, |columns| {
                columns[0].vertical_centered(|ui| {
                    // Display multiple-choice options
                    if ui.button("Next Procedure").clicked() {
                        if self.boldface_number<9 { self.boldface_number+=1; self.hidden_display = [false, false, false].to_vec() }
                    }
                });
                columns[1].vertical_centered(|ui| {
                    // Display multiple-choice options
                    if ui.button("Previous Procedure").clicked() {
                        if self.boldface_number>0 { self.boldface_number=1; self.hidden_display = [false, false, false].to_vec() }
                    }
                });
                columns[2].vertical_centered(|ui| {
                    // Button to go back to the main menu
                    if ui.button("Back to Main Menu").clicked() {
                        self.boldface_number = 0;
                        self.current_screen = Screen::MainMenu;
                    }
                });
            });
        });
    }
    
    /*fn render_op_quizzer(&mut self, ctx: &egui::Context){
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading("Operational Data Quizzer");

            // Display a question
            //ui.label(format!("{} / {}", self.op_data[self.boldface_number], self.op_data[self.boldface_number]));
            ui.separator();
    
            // Display multiple-choice options
            //graphics::label_textbox_question(ui, "step", &mut self.answers, &mut self.correct_answers);
    
            // Button to check answers
            if ui.button("Check Answers").clicked() {
                self.answered = true;
            }
            if self.answered == true {
                if self.answers[0] == "132" && self.answers[1] == "20" {
                    ui.label("Correct!");
                } else {
                    ui.label("Incorrect!");
                }
            }

            // Button to go back to the main menu
            if ui.button("Back to Main Menu").clicked() {
                self.current_screen = Screen::MainMenu;
            }
        });
    }*/
    
    fn setup_queried_op_quizzer(&mut self, query: &str){
        self.answers.clear();
        self.correct_answers.clear();

        let mut answer_index = 0;
        let mut query_match = false;

            // Iterate through the outer HashMap
            for (category, subcategories) in &self.op_data {
                if category.to_lowercase().contains(&query.to_lowercase()) {
                    query_match = true;
                    println!("Matched: {}", category);
                }
                // Iterate through the inner HashMap
                for (subcategory, steps) in subcategories {
                    // Check if the subcategory matches the query
                    if subcategory.to_lowercase().contains(&query.to_lowercase()) || query_match  {
    
                        // Display the steps
                        for step in steps {
                            let sectional = step.split("_");

                            for section in sectional {
                                if section.contains("$") {
                                    let without_dollar = section.replace("$", "");
                                    if answer_index >= self.correct_answers.len() {
                                        self.correct_answers.push(without_dollar);
                                    }
                                    if answer_index >= self.answers.len() {
                                        self.answers.push(String::new());
                                    }
                                    answer_index = self.answers.len()-1;
                                    answer_index+=1;
                                }
                            }
                        }
                    }
                }
            }

    }

    fn render_queried_op_quizzer(&mut self, ctx: &egui::Context, query: &str) {
        egui::TopBottomPanel::bottom("bot_quiz_panel").show(ctx, |ui| {
    
            // Display multiple-choice options
            //graphics::label_textbox_question(ui, step, &mut self.answers);
            // Button to check answers
            if ui.button("Check Answers").clicked() {
                self.answered = true;
            }



            if self.answered {
                let mut correct = true;
                for (answer, correct_answer) in self.answers.iter().zip(self.correct_answers.iter()) {
                    if answer != correct_answer {
                        correct = false;
                        break;
                    }
                }

                if correct == true {
                    ui.label("Correct!");
                } else {
                    ui.label("Incorrect!");
                }
            }
            if ui.button("Next Set").clicked() {
                self.answered = false;
                boldface::BfOpdataEnum::next(&mut self.ops_section);
            }
            if ui.button("Prev Set").clicked() {
                self.answered = false;
                boldface::BfOpdataEnum::prev(&mut self.ops_section);
            }
    
            // Button to go back to the main menu
            if ui.button("Back to Main Menu").clicked() {
                self.current_screen = Screen::MainMenu;
            }
        });

        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading("Operational Data Quizzer");
            ui.heading(format!("Category: {}", self.ops_section.as_str()));
            ui.separator();

            let mut answer_index= 0;
            let mut query_match = false;
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Iterate through the outer HashMap
                for (category, subcategories) in &self.op_data {
                    if category.to_lowercase().contains(&query.to_lowercase()) {
                        query_match = true;
                        //ui.heading(format!("Category: {}", category));
                        //ui.separator();
                        //println!("Matched: {}", category);
                    }
                    // Iterate through the inner HashMap
                    for (subcategory, steps) in subcategories {
                        // Check if the subcategory matches the query
                        if subcategory.to_lowercase().contains(&query.to_lowercase()) || query_match {
                            ui.label(format!("Subcategory: {}", subcategory));
                            ui.separator();
        
                            // Display the steps
                            for step in steps {
                                graphics::label_textbox_question(ui, step, &mut self.answers, &mut answer_index);
                            }
                            ui.separator();
                        }
                    }
                    query_match = false;
                }
            });
        });
    }
}


fn main() -> eframe::Result<(), eframe::Error> {
    //egui app
    run_native(
        "T6 App", 
        NativeOptions::default(), 
        Box::new(|cc: &CreationContext<'_>| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<T6App>::default())
        }),
    )
}
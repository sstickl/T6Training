use egui::{CentralPanel, Ui};
use std::collections::HashMap;

use crate::boldface::BfOpdataEnum;
use crate::graphics;
use crate::boldface;

#[derive(Default, PartialEq)]
enum Screen {
    #[default]
    MainMenu,           //Main menu screen
    QuizScreen,         //Random quiz screen
    BoldFaceQuizScreen, //Quiz screen for Boldface Procedures
    OpsQuizScreen,      //Quiz screen for Operational Data
    BoldFaceViewer,     //Viewer for Boldface Procedures
    OpsDataViewer,      //Viewer for Operational Data
}
pub struct T6App{
    current_screen: Screen,             //tracks the current screen
    boldface_ops: Vec<Vec<String>>,                         //Holds the Boldface Procedures data
    op_data: HashMap<String, HashMap<String, Vec<String>>>, //Holds the Boldface Ops data
    boldface_number: usize,             //Tracks the current boldface number
    hidden_display: Vec<bool>,          //Tracks whether text on the Boldface Viewer is hidden.  Unintuitively, false = hidden
    answers: Vec<String>,               //Tracks the answers given to the current quiz
    correct_answers: Vec<String>,       //Tracks the correct answers to the current quiz
    answered: bool,                     //Tracks whether the current quiz screen has been answered.
    //section: String,                    //Tracks the section desired for the ops_data quiz
    ops_section: boldface::BfOpdataEnum,    //Tracks the section desired for the ops_data quiz
    //ops_section_2: boldface::BfOpdataEnum,
    //display_correct: bool,              //Tracks whether the answers should be displayed
}

impl eframe::App for T6App{ // Want our app to run off of eframe
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //Figure out which screen to render based off of the enum
        match self.current_screen {
            Screen::MainMenu => self.render_main_menu(ctx),
            Screen::QuizScreen => self.render_queried_op_quizzer(ctx, self.ops_section.as_str()),
            Screen::BoldFaceViewer => self.render_bf_viewer(ctx),
            //Screen::OpsDataViewer => self.render_ops_data_viewer(ctx),
            //Screen::BoldFaceQuizScreen => self.render_bf_quizzer(ctx),
            //Screen::OpsQuizScreen => self.render_ops_quizzer(ctx),
            _ => {
                println!("Screen not implemented:");
            },
        }
    }
}

impl Default for T6App {
    //ctor
    fn default() -> Self {
        Self {
            current_screen: Screen::MainMenu,  //Start on the main menu
            boldface_ops: boldface::init_boldface_db(), //init boldface procedures db
            boldface_number: 0, //default boldface viewer to first boldface
            hidden_display: [false, false, false].to_vec(), //default boldface viewer to all hidden
            answers: Vec::<String>::new(),  //we're not sure how many ans
            answered: false,                //default quiz screen to not answered
            op_data: boldface::init_bf_opdata_db(), //init boldface ops db
            correct_answers: Vec::<String>::new(),  //we're not sure how many correct ans
            //section: "Engine".to_string(),    //default ops data to engine/first section
            ops_section: boldface::BfOpdataEnum::Engine,    //default ops data to engine/first section
            //ops_section_2: boldface::BfOpdataEnum::Engine,
            //display_correct: false,
        }
    }
}

impl T6App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            ;
            //return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn render_main_menu(&mut self, ctx: &egui::Context){
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui|
            {
                //Title
                ui.style_mut().text_styles.get_mut(&egui::TextStyle::Heading).unwrap().size = 32.0; //Make the header big
                ui.heading("T6 Boldface Training App");
                ui.add_space(50.0); // Adds 50 pixels of vertical space
                
                //#[cfg(not(target_arch = "wasm32"))]{
                // Display the T6RA image
                ui.add(
                    egui::Image::new(egui::include_image!("../assets/T6RA.jpeg"))
                        .max_height(200.0)
                ).on_hover_text_at_pointer("T6 Photo by USAF MSgt David Richards");
                ui.add_space(50.0); // Adds 50 pixels of vertical space after
                //}

                //Buttons
                ui.scope(|ui| {
                    //ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                    ui.spacing_mut().item_spacing.y = 30.0;
                    
                    if ui.button("Specific Quiz").clicked() {
                        //boldface::BfOpdataEnum::match_text(&mut self.ops_section,self.section.to_string().as_str());
                        self.setup_queried_op_quizzer(self.ops_section.as_str());
                        self.current_screen = Screen::QuizScreen;
                    }
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.add(egui::Label::new("Choose section for Quiz:"));
                        //let _response = ui.add(egui::TextEdit::singleline(&mut self.section).desired_width(80.0));
                        egui::ComboBox::from_label("")
                            .selected_text(format!("{}", self.ops_section.as_str()))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.ops_section, BfOpdataEnum::Engine, "Engine");
                                ui.selectable_value(&mut self.ops_section, BfOpdataEnum::ProhibitedManuevers, "Prohibited Manuevers");
                                ui.selectable_value(&mut self.ops_section, BfOpdataEnum::AirspeedLimitations, "Airspeed Limitations");
                                ui.selectable_value(&mut self.ops_section, BfOpdataEnum::Starting, "Starting");
                                ui.selectable_value(&mut self.ops_section, BfOpdataEnum::Pressurization, "Pressurization");
                                ui.selectable_value(&mut self.ops_section, BfOpdataEnum::Fuel, "Fuel");
                                ui.selectable_value(&mut self.ops_section, BfOpdataEnum::Runway, "Runway");
                                ui.selectable_value(&mut self.ops_section, BfOpdataEnum::MaximumCrosswinds, "Maximum Crosswinds");
                                ui.selectable_value(&mut self.ops_section, BfOpdataEnum::AccelerationLimits, "Acceleration Limits");
                                ui.selectable_value(&mut self.ops_section, BfOpdataEnum::IntentionalSpinEntry, "Intentional Spin Entry");
                                ui.selectable_value(&mut self.ops_section, BfOpdataEnum::Icing, "Icing");
                                ui.selectable_value(&mut self.ops_section, BfOpdataEnum::Temperature, "Temperature");
                            });
                    });
                    //println!("Ops section selected:{}", self.ops_section.as_str());

                    if ui.button("Boldface Viewer").clicked() {
                        self.current_screen = Screen::BoldFaceViewer;
                    }
                    if ui.button("Operational Data (Not yet implemented)").clicked() {
                        //self.setup_queried_op_quizzer(self.ops_section.as_str());
                        //println!("Section: {}", self.ops_section.as_str());
                        //self.current_screen = Screen::QuizScreen;
                    }
                });
                ui.allocate_space(ui.available_size());
             });
        });
    }
   
    fn render_bf_viewer(&mut self, ctx: &egui::Context){
        egui::TopBottomPanel::bottom("bot_bf_panel").show(ctx, |ui| {
            ui.add_space(20.0); // Adds 50 pixels of vertical space
            ui.columns(4, |columns| {
                columns[0].vertical_centered(|ui| {
                    if ui.button("Reveal All").clicked() {
                        for i in 0..self.hidden_display.len() {
                            self.hidden_display[i] = true;
                        }
                    }
                });
                columns[1].vertical_centered(|ui| {
                    if ui.button("Next Procedure").clicked() {
                        if self.boldface_number<9 {
                            self.boldface_number+=1;
                            self.hidden_display = [false, false, false].to_vec()
                        }
                    }
                });
                columns[2].vertical_centered(|ui| {
                    if ui.button("Previous Procedure").clicked() {
                        if self.boldface_number>0 {
                            self.boldface_number=1;
                            self.hidden_display = [false, false, false].to_vec()
                        }
                    }
                });
                columns[3].vertical_centered(|ui| {
                    // Button to go back to the main menu
                    if ui.button("Back to Main Menu").clicked() {
                        self.boldface_number = 0;
                        self.current_screen = Screen::MainMenu;
                    }
                });
            });
            ui.add_space(20.0); // Adds 20 pixels of vertical space
        });
        
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            //Heading
            ui.heading("Boldface Op Viewer");
            ui.add_space(10.0); // Adds 50 pixels of vertical space

            //Boldface Op Viewer Area
            ui.scope(|ui| {
                ui.set_min_height(35.0);
                ui.label(format!("Boldface for: {}", self.boldface_ops[self.boldface_number][0]));
                ui.label(format!("Click black boxes to reveal step"));
            });
            ui.separator();
            ui.scope(|ui| {
                ui.set_min_height(120.0);
                for (index, step) in self.boldface_ops[self.boldface_number].iter().enumerate().skip(1) {
                    let mut label_string = String::from(format!("{}", step));
                    graphics::hidden_label(ui, &mut label_string, &mut self.hidden_display[index-1]);
                }
            });

            ui.add_space(20.0); // Adds 50 pixels of vertical space
    
        });
    }
    
    fn setup_queried_op_quizzer(&mut self, query: &str){
        self.answers.clear();
        self.answered = false;
        self.correct_answers.clear();
        //self.display_correct = false;

        let mut answer_index = 0;
        let mut query_match = false;

            // Iterate through the outer HashMap
            for (category, subcategories) in &self.op_data {
                if category.to_lowercase().contains(&query.to_lowercase()) {
                    query_match = true;
                    //println!("Matched: {}", category);
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
                                        //println!("Correct Answer: {}", without_dollar);
                                        self.correct_answers.push(without_dollar);
                                        
                                    }
                                    if answer_index >= self.answers.len() {
                                        self.answers.push(String::new());
                                        //println!("Answer: {}", without_dollar);
                                    }
                                    //answer_index = self.answers.len()-1;
                                    answer_index+=1;
                                }
                            }
                        }
                    }
                }
                query_match = false;
            }

    }

    fn render_queried_op_quizzer(&mut self, ctx: &egui::Context, query: &str) {
        egui::TopBottomPanel::bottom("bot_quiz_panel").show(ctx, |ui| {
            ui.add_space(20.0); // Adds 50 pixels of vertical space

                

            ui.columns(4, |columns| {
                columns[0].vertical_centered(|ui| {
                    // Button to check answers
                    if ui.button("Check Answers").clicked() {
                        self.answered = true;
                    }
                });
                columns[1].vertical_centered(|ui| {
                    if ui.button("Next Set").clicked() {
                        //self.answered = false;
                        boldface::BfOpdataEnum::next(&mut self.ops_section);
                        self.setup_queried_op_quizzer(&self.ops_section.as_str());
                    }
                });
                columns[2].vertical_centered(|ui| {
                    if ui.button("Prev Set").clicked() {
                        //self.answered = false;
                        boldface::BfOpdataEnum::prev(&mut self.ops_section);
                        self.setup_queried_op_quizzer(self.ops_section.as_str());
                    }
                });
                columns[3].vertical_centered(|ui| {
                    // Button to go back to the main menu
                    if ui.button("Back to Main Menu").clicked() {
                        self.answered = false;
                        self.current_screen = Screen::MainMenu;
                    }
                });
            });
            ui.add_space(20.0); // Adds 50 pixels of vertical space
        });

        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading("Operational Data Quizzer");
            ui.heading(format!("Category: {}", self.ops_section.as_str()));

            ui.add_space(20.0);
            ui.horizontal( |ui|{
                if self.answered {
                    let mut correct = true;
                    for (answer, correct_answer) in self.answers.iter().zip(self.correct_answers.iter()) {
                        if answer != correct_answer {
                            correct = false;
                            break;
                        }
                    }
                    if correct == true {
                        //ui.label("Answers were correct!");
                        ui.label(egui::RichText::new("Answers were correct").background_color(egui::Color32::from_rgb(0, 255, 0)));
                    } else {
                        ui.label(egui::RichText::new("Answers were incorrect!").background_color(egui::Color32::from_rgb(255,0, 0)));
                        if ui.small_button("Display Correct Answers").clicked() {
                            for (index, correct_answer) in self.correct_answers.iter().enumerate() {
                                self.answers[index] = correct_answer.clone();
                            }
                        }
                    } 
                }
                else {
                    ui.label("Answers not yet checked.");
                }
            });
            ui.separator();

            let mut answer_index= 0;
            let mut query_match = false;
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Iterate through the outer HashMap
                for (category, subcategories) in &self.op_data {
                     //Check if query matches a category
                    if category.to_lowercase().contains(&query.to_lowercase()) {
                        query_match = true;
                    }
                    // Iterate through the inner HashMap
                    for (subcategory, steps) in subcategories {
                        // Check if the subcategory matches the query
                        if subcategory.to_lowercase().contains(&query.to_lowercase()) || query_match {
                            ui.label(format!("Subcategory: {}", subcategory));
                            ui.separator();
                            ui.add_space(5.0);
        
                            // Display the steps with fill-in textboxes
                            for step in steps {
                                if!(self.answered){
                                    graphics::label_textbox_question(ui, step, &mut self.answers, &mut answer_index);
                                }
                                else {
                                    graphics::label_answered_question(ui, step, &mut self.answers, &mut answer_index, &mut self.correct_answers);    
                                }
                            }
                            ui.add_space(5.0);
                            ui.separator();
                        }
                    }
                    query_match = false
                }
            });
        });
    }
}
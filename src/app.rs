use egui::{CentralPanel, Ui};
use std::collections::HashMap;

use crate::boldface; // Holds boldface data structure/loading code
use crate::boldface::BfOpdataEnum; // Holds boldface op data
use crate::graphics; // Custom widgets for egui

/// Holds the ID of whatever current screen the app is on
#[derive(Default, PartialEq)]
enum Screen {
    #[default]
    MainMenu, // Main menu screen
    OpsQuiz,        // Random quiz screen for BoldFace Ops
    _BoldFaceQuiz,  // Quiz screen for Boldface Procedures. **NOT YET IMPLEMENTED**
    BoldFaceViewer, // Viewer for Boldface Procedures    **NOT YET IMPLEMENTED**
    OpsDataViewer,  // Viewer for Operational Data  **NOT YET IMPLEMENTED**
}

/// The App
pub struct T6App {
    current_screen: Screen,         // Tracks the current screen
    boldface_ops: Vec<Vec<String>>, // Holds the Boldface Procedures data
    op_data: HashMap<String, HashMap<String, Vec<String>>>, // Holds the Boldface Ops data
    boldface_number: usize,         // Tracks the current boldface number
    hidden_display: Vec<bool>, // Tracks whether text on the Boldface Viewer is hidden. Unintuitively, false = hidden
    answers: Vec<String>,      // Tracks the answers given to the current quiz
    correct_answers: Vec<String>, // Tracks the correct answers to the current quiz
    answered: bool,            // Tracks whether the current quiz screen has been answered.
    ops_section: boldface::BfOpdataEnum, // Tracks the section desired for the ops_data quiz
}

/// Our App runs off of eframe
impl eframe::App for T6App {
    /// Called by the framework to give the app a chance to respond to events.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Figure out which screen to render based off of the enum
        match self.current_screen {
            Screen::MainMenu => self.render_main_menu(ctx),
            Screen::OpsQuiz => self.render_queried_op_quizzer(ctx, self.ops_section.as_str()),
            Screen::BoldFaceViewer => self.render_bf_viewer(ctx),
            Screen::OpsDataViewer => self.render_ops_data_viewer(ctx, self.ops_section.as_str()),
            // Screen::BoldFaceQuizScreen => self.render_bf_quizzer(ctx),     **NOT YET IMPLEMENTED**
            _ => {
                println!("Screen not implemented:");
            }
        }
    }
}

/// Default functions for T6App
impl Default for T6App {
    /// T6App constructor
    fn default() -> Self {
        Self {
            current_screen: Screen::MainMenu,           // Start on the main menu
            boldface_ops: boldface::init_boldface_db(), // Initialize boldface procedures db
            boldface_number: 0,                         // Default boldface viewer to first boldface
            hidden_display: [false, false, false].to_vec(), // Default boldface viewer to all hidden
            answers: Vec::<String>::new(),              // Initialize answers vector
            answered: false,                            // Default quiz screen to not answered
            op_data: boldface::init_bf_opdata_db(),     // Initialize boldface ops db
            correct_answers: Vec::<String>::new(),      // Initialize correct answers vector
            ops_section: boldface::BfOpdataEnum::Engine, // Default ops data to engine/first section
        }
    }
}

/// T6App functions
impl T6App {
    /// Render the main menu, opened when the app starts
    fn render_main_menu(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            // Centered layout, top-down justified
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    // Title
                    ui.style_mut()
                        .text_styles
                        .get_mut(&egui::TextStyle::Heading)
                        .unwrap()
                        .size = 32.0; // Make the header big
                    ui.heading("T6 Boldface Training App");
                    ui.add_space(50.0); // Adds 50 pixels of vertical space

                    // Display the T6RA image. Works on everything but Safari for some reason.
                    //.max_height(ctx.screen_rect().height() / 3.0),
                    ui.add(
                        egui::Image::new(egui::include_image!("../assets/T6RA.jpeg"))
                            .max_height(200.0),
                    )
                    .on_hover_text_at_pointer("T6 Photo by USAF MSgt David Richards");
                    ui.add_space(50.0); // Adds 50 pixels of vertical space after

                    // UI section for selecting the next screen
                    ui.scope(|ui| {
                        // Set the spacing between items
                        ui.spacing_mut().item_spacing.y = 30.0;

                        // Opdata Quiz button to select a specific section
                        if ui.button("Opdata Quiz (select section below)").clicked() {
                            self.setup_queried_op_quizzer(self.ops_section.as_str());
                            self.current_screen = Screen::OpsQuiz;
                        }

                        // Opdata Viewer *** Not yet implemented ***
                        if ui
                            .button("Operational Data (select section below)")
                            .clicked()
                        {
                            self.setup_queried_op_quizzer(self.ops_section.as_str());
                            self.current_screen = Screen::OpsDataViewer
                        }

                        // Opdata Quiz to choose the specific section
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            ui.add(egui::Label::new("Choose section for Ops Data:"));
                            egui::ComboBox::from_label("")
                                .selected_text(self.ops_section.as_str().to_string())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.ops_section,
                                        BfOpdataEnum::Engine,
                                        "Engine",
                                    );
                                    ui.selectable_value(
                                        &mut self.ops_section,
                                        BfOpdataEnum::ProhibitedManuevers,
                                        "Prohibited Manuevers",
                                    );
                                    ui.selectable_value(
                                        &mut self.ops_section,
                                        BfOpdataEnum::AirspeedLimitations,
                                        "Airspeed Limitations",
                                    );
                                    ui.selectable_value(
                                        &mut self.ops_section,
                                        BfOpdataEnum::Starting,
                                        "Starting",
                                    );
                                    ui.selectable_value(
                                        &mut self.ops_section,
                                        BfOpdataEnum::Pressurization,
                                        "Pressurization",
                                    );
                                    ui.selectable_value(
                                        &mut self.ops_section,
                                        BfOpdataEnum::Fuel,
                                        "Fuel",
                                    );
                                    ui.selectable_value(
                                        &mut self.ops_section,
                                        BfOpdataEnum::Runway,
                                        "Runway",
                                    );
                                    ui.selectable_value(
                                        &mut self.ops_section,
                                        BfOpdataEnum::MaximumCrosswinds,
                                        "Maximum Crosswinds",
                                    );
                                    ui.selectable_value(
                                        &mut self.ops_section,
                                        BfOpdataEnum::AccelerationLimits,
                                        "Acceleration Limits",
                                    );
                                    ui.selectable_value(
                                        &mut self.ops_section,
                                        BfOpdataEnum::IntentionalSpinEntry,
                                        "Intentional Spin Entry",
                                    );
                                    ui.selectable_value(
                                        &mut self.ops_section,
                                        BfOpdataEnum::Icing,
                                        "Icing",
                                    );
                                    ui.selectable_value(
                                        &mut self.ops_section,
                                        BfOpdataEnum::Temperature,
                                        "Temperature",
                                    );
                                });
                        });

                        // Boldface Viewer
                        if ui.button("Boldface Viewer").clicked() {
                            self.current_screen = Screen::BoldFaceViewer;
                        }
                    });

                    // Make sure we get the space we need
                    ui.allocate_space(ui.available_size());
                },
            );
        });
    }

    /// The boldface viewer screen
    /// Displays the boldface procedures with hidden steps that can be revealed
    fn render_bf_viewer(&mut self, ctx: &egui::Context) {
        // Render the bottom buttons first.
        // Reveal All, Next Procedure, Previous Procedure, Back to Main Menu
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
                    if ui.button("Previous Procedure").clicked() {
                        // Ensure we don't underflow
                        if self.boldface_number > 0 {
                            self.boldface_number -= 1;
                            self.hidden_display = [false, false, false].to_vec()
                        }
                        // Wrap around if needed
                        else {
                            self.boldface_number = 9;
                            self.hidden_display = [false, false, false].to_vec()
                        }
                    }
                });
                columns[2].vertical_centered(|ui| {
                    if ui.button("Next Procedure").clicked() {
                        // Ensure we don't overflow
                        if self.boldface_number < 9 {
                            self.boldface_number += 1;
                            self.hidden_display = [false, false, false].to_vec()
                        }
                        // Wrap around if needed
                        else {
                            self.boldface_number = 0;
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

        // The main contents of the screen
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            // Heading
            ui.heading("Boldface Op Viewer");
            ui.add_space(10.0); // Adds 50 pixels of vertical space

            // Boldface Op Viewer Area
            ui.scope(|ui| {
                ui.set_min_height(35.0);
                // Display section/instructions
                ui.label(format!(
                    "Boldface for: {}",
                    self.boldface_ops[self.boldface_number][0]
                ));
                ui.label("Click black boxes to reveal step");
            });
            ui.separator();

            // Display the steps for the current boldface procedure
            ui.scope(|ui| {
                ui.set_min_height(120.0);
                for (index, step) in self.boldface_ops[self.boldface_number]
                    .iter()
                    .enumerate()
                    .skip(1)
                {
                    let label_string = step.to_string();
                    graphics::hidden_label(ui, &label_string, &mut self.hidden_display[index - 1]);
                }
            });

            ui.add_space(20.0); // Adds 50 pixels of vertical space
        });
    }

    /// Sets up the operational data quizzer for a specific section
    /// The query it takes in is the section to be quizzed on
    fn setup_queried_op_quizzer(&mut self, query: &str) {
        // Clear class variables to ensure the answers aren't displayed immediately/textboxes aren't filled
        self.answers.clear();
        self.answered = false;
        self.correct_answers.clear();

        // Local variables
        let mut answer_index = 0; // Index for the answers vector
        let mut query_match = false; // Tracks if the query matches a category

        // Iterate through the outer HashMap to search the categories
        for (category, subcategories) in &self.op_data {
            // Check if the category matches the query
            if category.to_lowercase().contains(&query.to_lowercase()) {
                query_match = true;
            }
            // Iterate through the inner HashMap to search the subcategories
            for (subcategory, steps) in subcategories {
                // Check if the subcategory matches the query
                if subcategory.to_lowercase().contains(&query.to_lowercase()) || query_match {
                    // Display the steps
                    for step in steps {
                        let sectional = step.split("_"); // Split the step into sections

                        // Go through each section and check if it contains a "$", which indicates a fill-in-the-blank
                        // Fill in the blanks are added to the correct answers vector, and the answers vector is extended
                        // to hold the same number of elements
                        for section in sectional {
                            if section.contains("$") {
                                let without_dollar = section.replace("$", "");
                                if answer_index >= self.correct_answers.len() {
                                    self.correct_answers.push(without_dollar);
                                }
                                if answer_index >= self.answers.len() {
                                    self.answers.push(String::new());
                                }
                                answer_index += 1;
                            }
                        }
                    }
                }
            }
            query_match = false; // Reset the query match for category
        }
    }

    /// The operational data quizzer screen
    /// Displays the operational data with fill-in textboxes for the user to fill in
    fn render_queried_op_quizzer(&mut self, ctx: &egui::Context, query: &str) {
        // Render the bottom buttons first.
        // Check Answers, Next Set, Previous Set, Back to Main Menu
        egui::TopBottomPanel::bottom("bot_quiz_panel").show(ctx, |ui| {
            ui.add_space(20.0); // Adds 50 pixels of vertical space
            ui.columns(5, |columns| {
                columns[0].vertical_centered(|ui| {
                    // Button to check answers
                    if ui.button("Check Answers").clicked() {
                        self.answered = true;
                    }
                });
                columns[1].vertical_centered(|ui| {
                    if ui.button("Prev Set").clicked() {
                        boldface::BfOpdataEnum::prev(&mut self.ops_section);
                        self.setup_queried_op_quizzer(self.ops_section.as_str());
                    }
                });
                columns[2].vertical_centered(|ui| {
                    if ui.button("Next Set").clicked() {
                        boldface::BfOpdataEnum::next(&mut self.ops_section);
                        self.setup_queried_op_quizzer(self.ops_section.as_str());
                    }
                });
                columns[3].vertical_centered(|ui| {
                    if ui.button("Data Viewer").clicked() {
                        self.answered = false;
                        self.current_screen = Screen::OpsDataViewer;
                    }
                });
                columns[4].vertical_centered(|ui| {
                    // Button to go back to the main menu
                    if ui.button("Back to Main Menu").clicked() {
                        self.answered = false;
                        self.current_screen = Screen::MainMenu;
                    }
                });
            });
            ui.add_space(20.0); // Adds 50 pixels of vertical space
        });

        // The main contents of the screen
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            // Heading
            ui.heading("Operational Data Quizzer");
            ui.heading(format!("Category: {}", self.ops_section.as_str()));

            ui.add_space(5.0);

            // Check if the answers are answered/correct, allow the user to correct answers
            // Rendered in a ui.horizontal to ensure UI stays the correct size no matter what
            ui.horizontal(|ui| {
                if self.answered {
                    let mut correct = true;
                    for (answer, correct_answer) in
                        self.answers.iter().zip(self.correct_answers.iter())
                    {
                        if answer != correct_answer {
                            correct = false;
                            break;
                        }
                    }
                    if correct {
                        ui.label(
                            egui::RichText::new("Answers were correct")
                                .background_color(egui::Color32::from_rgb(0, 255, 0)),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new("Answers were incorrect!")
                                .background_color(egui::Color32::from_rgb(255, 0, 0)),
                        );
                        if ui.small_button("Display Correct Answers").clicked() {
                            for (index, correct_answer) in self.correct_answers.iter().enumerate() {
                                self.answers[index] = correct_answer.clone();
                            }
                        }
                    }
                } else {
                    ui.label("Answers not yet checked.");
                }
            });
            ui.separator();

            // Local variables for rendering
            let mut answer_index = 0; // Index for the answers vector
            let mut query_match = false; // Tracks if the query matches a category

            // Scroll area for the quizzer
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Iterate through the outer HashMap through categories
                for (category, subcategories) in &self.op_data {
                    // Check if query matches a category
                    if category.to_lowercase().contains(&query.to_lowercase()) {
                        query_match = true;
                    }
                    // Iterate through the inner HashMap through subcategories
                    for (subcategory, steps) in subcategories {
                        // Check if the subcategory matches the query
                        if subcategory.to_lowercase().contains(&query.to_lowercase()) || query_match
                        {
                            ui.label(format!("Subcategory: {}", subcategory));
                            ui.separator();
                            ui.add_space(5.0);

                            // Display the steps with fill-in textboxes
                            for step in steps {
                                // If the screen is not yet answered, display the textboxes
                                if !self.answered {
                                    graphics::label_textbox_question(
                                        ui,
                                        step,
                                        &mut self.answers,
                                        &mut answer_index,
                                    );
                                }
                                // If the screen is answered, display the status of the answers
                                else {
                                    graphics::label_answered_question(
                                        ui,
                                        step,
                                        &mut self.answers,
                                        &mut answer_index,
                                        &mut self.correct_answers,
                                    );
                                }
                            }
                            ui.add_space(5.0);
                            ui.separator();
                        }
                    }
                    query_match = false; // Reset the variable tracking a category match
                }
            });
        });
    }

    fn render_ops_data_viewer(&mut self, ctx: &egui::Context, query: &str) {
        // Render the bottom buttons first.
        // Check Answers, Next Set, Previous Set, Back to Main Menu
        egui::TopBottomPanel::bottom("bot_quiz_panel").show(ctx, |ui| {
            ui.add_space(20.0); // Adds 50 pixels of vertical space
            ui.columns(4, |columns| {
                columns[0].vertical_centered(|ui| {
                    // Button to check answers
                    if ui.button("Convert to Quiz").clicked() {
                        for answer in self.answers.iter_mut() {
                            *answer = "".to_string()
                        }
                        self.current_screen = Screen::OpsQuiz;
                    }
                });
                columns[1].vertical_centered(|ui| {
                    if ui.button("Prev Set").clicked() {
                        boldface::BfOpdataEnum::prev(&mut self.ops_section);
                        self.setup_queried_op_quizzer(self.ops_section.as_str());
                    }
                });
                columns[2].vertical_centered(|ui| {
                    if ui.button("Next Set").clicked() {
                        boldface::BfOpdataEnum::next(&mut self.ops_section);
                        self.setup_queried_op_quizzer(self.ops_section.as_str());
                    }
                });
                columns[3].vertical_centered(|ui| {
                    // Button to go back to the main menu
                    if ui.button("Back to Main Menu").clicked() {
                        self.current_screen = Screen::MainMenu;
                    }
                });
            });
            ui.add_space(20.0); // Adds 50 pixels of vertical space
        });

        // The main contents of the screen
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            // Heading
            ui.heading("Operational Data Viewer");
            ui.heading(format!("Category: {}", self.ops_section.as_str()));

            ui.add_space(5.0);

            // Hopefully this will keep it the correct size if you convert it to a quiz
            ui.horizontal(|ui| {
                ui.label(" ");
            });
            ui.separator();

            // Local variables for rendering
            let mut answer_index = 0; // Index for the answers vector
            let mut query_match = false; // Tracks if the query matches a category

            // Scroll area for the quizzer
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Iterate through the outer HashMap through categories
                for (category, subcategories) in &self.op_data {
                    // Check if query matches a category
                    if category.to_lowercase().contains(&query.to_lowercase()) {
                        query_match = true;
                    }
                    // Iterate through the inner HashMap through subcategories
                    for (subcategory, steps) in subcategories {
                        // Check if the subcategory matches the query
                        if subcategory.to_lowercase().contains(&query.to_lowercase()) || query_match
                        {
                            ui.label(format!("Subcategory: {}", subcategory));
                            ui.separator();
                            ui.add_space(5.0);

                            // Display the steps with fill-in textboxes
                            for step in steps {
                                graphics::label_ops_data(
                                    ui,
                                    step,
                                    &mut self.answers,
                                    &mut answer_index,
                                    &mut self.correct_answers,
                                );
                            }
                            ui.add_space(5.0);
                            ui.separator();
                        }
                    }
                    query_match = false; // Reset the variable tracking a category match
                }
            });
        });
    }
}

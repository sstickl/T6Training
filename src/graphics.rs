use egui::{FontId, TextStyle};
use std::collections::BTreeMap;

const TEXTBOX_WIDTH: usize = 12; // Constant for the width of the text box in the question widget (in characters)

#[inline]
fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

#[inline]
fn heading3() -> TextStyle {
    TextStyle::Name("ContextHeading".into())
}

pub fn configure_text_styles(ctx: &egui::Context) {
    use egui::FontFamily::{Monospace, Proportional};

    let text_styles: BTreeMap<TextStyle, FontId> = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (heading2(), FontId::new(22.0, Proportional)),
        (heading3(), FontId::new(19.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(12.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.all_styles_mut(move |style| style.text_styles = text_styles.clone());
}

/// sets up the frame for the widgets
fn set_up_frame() -> egui::Frame {
    egui::Frame::none()
        //.fill(egui::Color32::from_rgb(200, 200, 200))
        .rounding(egui::Rounding {
            nw: 2.0,
            ne: 2.0,
            sw: 2.0,
            se: 2.0,
        })
        .inner_margin(egui::Margin {
            left: 5.0,
            right: 5.0,
            top: 2.0,
            bottom: 2.0,
        })
        .outer_margin(egui::Margin {
            left: 1.0,
            right: 1.0,
            top: 2.0,
            bottom: 2.0,
        })
}

/// A custom widget for a hidden label that reveals text when clicked.
pub fn hidden_label(ui: &mut egui::Ui, text: &str, is_revealed: &mut bool) {
    egui::Frame::none() //actually, let's test this later.. I think we don't need the frame anymore since we're not using a label now
        //.inner_margin(egui::Margin::same(5.0)) // Add padding
        .show(ui, |ui| {
            // Set a fixed height for the widget
            //ui.set_height(15.0);

            // Check if the text is revealed
            if *is_revealed {
                // Display the revealed text
                if ui
                    .add(egui::Button::new(egui::RichText::new(text).text_style(TextStyle::Body).color(egui::Color32::BLACK)).fill(egui::Color32::YELLOW).frame(false))
                    // Clickable blacked-out text
                    .clicked()
                {
                    *is_revealed = false; // Reveal the text when clicked
                }
            } else {
                // Display a blacked-out placeholder
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new(text).color(egui::Color32::from_rgb(0, 0, 0)).text_style(TextStyle::Body),
                        )
                        .fill(egui::Color32::from_rgb(0, 0, 0))
                        .frame(false),
                    )
                    // Clickable blacked-out text
                    .clicked()
                {
                    *is_revealed = true; // Reveal the text when clicked
                }
            }
        });
}

/// A custom widget that goes through a string and makes a line with a text box for the strings with answers
pub fn label_textbox_question(
    ui: &mut egui::Ui,
    text: &str,
    answers: &mut [String],
    answer_index: &mut i32,
) {
    // Create a frame for the widget to go around the line
    let frame = set_up_frame();

    // Display the frame and the line
    frame.show(ui, |ui| {
        // Horizontally wrap the contents so it displays well on mobile
        ui.horizontal_wrapped(|ui| {
            // Split the line along the underscores
            let sections = text.split("_");

            // Set a yellow line around the textboxes as they are not yet answered
            ui.style_mut().visuals.widgets.inactive.bg_stroke =
                egui::Stroke::new(1.0, egui::Color32::from_rgb(253, 218, 13));

            // Go through each section of the line
            for section in sections {
                // Check if the section contains a dollar sign, which annotates an answerable section
                if section.contains("$") {
                    // Remove the dollar sign and get the answer length
                    let placeholder = section.replace("$", "");
                    let answer_length = placeholder.len();

                    // Check if the answer index is within the bounds of the answers vector, and if so, create a textbox for the answer
                    if (*answer_index as usize) < answers.len() {
                        ui.add(
                            egui::TextEdit::singleline(&mut answers[*answer_index as usize])
                                .desired_width((answer_length * TEXTBOX_WIDTH) as f32)
                                .char_limit(answer_length),
                        );
                        *answer_index += 1;
                    } else {
                        // Shouldn't get here, but the setup somehow failed does, display a placeholder
                        ui.label("[No answer provided]");
                    }
                // No dollar sign, so just display the section as a label
                } else {
                    ui.label(section);
                }
            }
        });
    });
} //fn label_textbox_question

/// A custom widget that goes through a string and makes a line with a text box for the strings with answers
/// Outlines whether the answers are correct or incorrect
pub fn label_answered_question(
    ui: &mut egui::Ui,
    text: &str,
    answers: &mut [String],
    answer_index: &mut i32,
    correct_answers: &mut [String],
) {
    // Create a frame for the widget to go around the line
    let frame = set_up_frame();

    // Display the frame and the line
    frame.show(ui, |ui| {
        // Horizontally wrap the contents so it displays well on mobile
        ui.horizontal_wrapped(|ui| {
            // Split the line along the underscores
            let sectional = text.split("_");

            // Go through each section of the line
            for section in sectional {
                // Check if the section contains a dollar sign, which annotates an answerable section
                if section.contains("$") {
                    // Remove the dollar sign and get the answer length
                    let _without_dollar = section.replace("$", "");
                    let answer_length = _without_dollar.len();

                    // Check if the answer index is within the bounds of the answers vector, and if so, create a textbox for the answer and outline if it's correct or not
                    // Could save a line in the future by just changing the stroke based on the if statement
                    if (*answer_index as usize) < answers.len() {
                        if answers[*answer_index as usize]
                            == correct_answers[*answer_index as usize]
                        {
                            ui.style_mut().visuals.widgets.inactive.bg_stroke =
                                egui::Stroke::new(1.0, egui::Color32::GREEN);
                            let _response = ui.add(
                                egui::TextEdit::singleline(&mut answers[*answer_index as usize])
                                    .desired_width((answer_length * TEXTBOX_WIDTH) as f32),
                            );
                        } else {
                            ui.style_mut().visuals.widgets.inactive.bg_stroke =
                                egui::Stroke::new(1.0, egui::Color32::RED);
                            let _response = ui.add(
                                egui::TextEdit::singleline(&mut answers[*answer_index as usize])
                                    .desired_width((answer_length * TEXTBOX_WIDTH) as f32),
                            );
                        }
                        *answer_index += 1;
                    }
                // No dollar sign, so just display the section as a label
                } else {
                    ui.add(egui::Label::new(section));
                }
            }
        });
    });
}

/// A custom widget that goes through an boldface procedures vector and labels answers right/wrong
/// takes in correct answers as the boldface ops[x] vector
/// Outlines whether the answers are correct or incorrect
pub fn label_answered_procedure(
    ui: &mut egui::Ui,
    answers: &mut [String],
    correct_answers: &mut [String],
) {
    // Create a frame for the widget to go around the line
    //let frame = set_up_frame();

    // Display the frame and the line
    //frame.show(ui, |ui| {
    ui.scope(|ui| {
        for (index, _step) in correct_answers.iter().enumerate() {
            ui.add(egui::Label::new(format!("Step {}: ", index + 1)));
            if answers[index] == correct_answers[index] {
                ui.style_mut().visuals.widgets.inactive.bg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::GREEN);
                let _response = ui.add(
                    egui::TextEdit::singleline(&mut answers[index])
                        .desired_width((correct_answers[index].len() * TEXTBOX_WIDTH) as f32),
                );
            } else {
                ui.style_mut().visuals.widgets.inactive.bg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::RED);
                let _response = ui.add(
                    egui::TextEdit::singleline(&mut answers[index])
                        .desired_width((correct_answers[index].len() * TEXTBOX_WIDTH) as f32),
                );
            }
        }
    });
    ui.scope(|ui| {
        ui.disable();
        for index in correct_answers.len()..5 {
            ui.add(egui::Label::new(format!(
                "Step {} (NOT APPLICABLE): ",
                index
            )));
            if answers.len() > index {
                if answers[index].is_empty() {
                    ui.style_mut().visuals.widgets.inactive.bg_stroke =
                        egui::Stroke::new(1.0, egui::Color32::GREEN);
                    let _response = ui.add(
                        egui::TextEdit::singleline(&mut answers[index])
                            .desired_width(f32::INFINITY),
                    );
                } else {
                    ui.style_mut().visuals.widgets.inactive.bg_stroke =
                        egui::Stroke::new(1.0, egui::Color32::RED);
                    let _response = ui.add(
                        egui::TextEdit::singleline(&mut answers[index])
                            .desired_width(f32::INFINITY), //7 too small
                    );
                }
            }
        }
    });
    //});
}

/// A custom widget that goes through a string and makes a line with a text box for the strings with answers
/// Outlines whether the answers are correct or incorrect
pub fn label_ops_data(
    ui: &mut egui::Ui,
    text: &str,
    _answers: &mut [String],
    answer_index: &mut i32,
    correct_answers: &mut [String],
    answers_displayed: &mut [bool],
) {
    // Create a frame for the widget to go around the line
    let frame = set_up_frame();

    // Display the frame and the line
    frame.show(ui, |ui| {
        // Horizontally wrap the contents so it displays well on mobile
        ui.horizontal_wrapped(|ui| {
            // Split the line along the underscores
            let sectional = text.split("_");

            // Go through each section of the line
            for section in sectional {
                // Check if the section contains a dollar sign, which annotates an answerable section
                if section.contains("$") {
                    // Remove the dollar sign and get the answer length
                    let _without_dollar = section.replace("$", "");

                    // Check if the answer index is within the bounds of the answers vector, and if so, create a textbox for the answer and outline if it's correct or not
                    if (*answer_index as usize) < correct_answers.len() {
                        //ui.add(egui::Label::new(format!(
                        //    " * {} * ",
                        //    correct_answers[*answer_index as usize]
                        //)));
                        hidden_label(
                            ui,
                            &format!(" * {} * ", correct_answers[*answer_index as usize]),
                            &mut answers_displayed[*answer_index as usize],
                        );
                        *answer_index += 1;
                    }
                // No dollar sign, so just display the section as a label
                } else {
                    ui.add(egui::Label::new(section));
                }
            }
        });
    });
}

//use egui::{*};
const TEXTBOX_WIDTH: usize = 10;

//A custom widget for a hidden label that reveals text when clicked.
pub fn hidden_label(ui: &mut egui::Ui, text: &str, is_revealed: &mut bool) {
    egui::Frame::none() //actually, let's test this later.. I think we don't need the frame anymore since we're not using a label now
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

pub fn label_textbox_question(ui: &mut egui::Ui, text: &str, answers: &mut Vec<String>, answer_index: &mut i32) {
    let frame = egui::Frame::none()
        .fill(egui::Color32::from_rgb(50,50,50))
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
        }).outer_margin(egui::Margin {
        left: 1.0,
        right: 1.0,
        top: 2.0,
        bottom: 2.0,
    });

    frame.show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            let sections = text.split("_");
            ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::YELLOW);

            for section in sections {
                if section.contains("$") {
                    let placeholder = section.replace("$", "");
                    let answer_length = placeholder.len();

                    if (*answer_index as usize) < answers.len() {
                            ui.add(
                                egui::TextEdit::singleline(&mut answers[*answer_index as usize])
                                    .desired_width((answer_length * TEXTBOX_WIDTH) as f32) //7 too small
                            );
                    // });
                        *answer_index += 1;
                    } else {
                        ui.label("[No answer provided]");
                    }
                } else {
                    ui.label(section);
                }
            }
        });
    });
} //fn label_textbox_question

/*//a custom widget that goes through a string and makes a line with a text box for the strings with answers
pub fn label_textbox_question(ui: &mut egui::Ui, text: &str, answers: &mut Vec<String>, answer_index: &mut i32) {
    //ui.horizontal_wrapped(|ui| {
    //ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
    ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT).with_main_wrap(true), |ui| {
        let sectional = text.split("_");


        for section in sectional {
            if section.contains("$") {
                let _without_dollar = section.replace("$", "");
                let answer_length = _without_dollar.len();
                if !((*answer_index as usize) > answers.len()-1) {
                    let _response = ui.add(egui::TextEdit::singleline(&mut answers[*answer_index as usize]).desired_width((answer_length*7) as f32));
                    *answer_index+=1;
                }
            }else {
                ui.add(egui::Label::new(section));
            }
        }
    });
    ui.add_space(10.0);
    // });
}*/

pub fn label_answered_question(ui: &mut egui::Ui, text: &str, answers: &mut Vec<String>, answer_index: &mut i32, correct_answers: &mut Vec<String>) {
    let frame = egui::Frame::none()
        .fill(egui::Color32::from_rgb(50,50,50))
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
        }).outer_margin(egui::Margin {
        left: 1.0,
        right: 1.0,
        top: 2.0,
        bottom: 2.0,
    });

    frame.show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            let sectional = text.split("_");
            for section in sectional {
    
                if section.contains("$") {
                    let _without_dollar = section.replace("$", "");
                    let answer_length = _without_dollar.len();           
                    if !((*answer_index as usize) > answers.len()-1) {
                        if answers[*answer_index as usize] == correct_answers[*answer_index as usize] {
                            ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::GREEN);
                            let _response = ui.add(egui::TextEdit::singleline(&mut answers[*answer_index as usize]).desired_width((answer_length*TEXTBOX_WIDTH) as f32));
                        } else {
                            ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::RED);
                            let _response = ui.add(egui::TextEdit::singleline(&mut answers[*answer_index as usize]).desired_width((answer_length*TEXTBOX_WIDTH) as f32));
                        }
                        *answer_index+=1;
                    }
                }else {
                    ui.add(egui::Label::new(section));
                }
            }
        });
    });
}
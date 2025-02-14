//use egui::{*};

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
    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
        let sectional = text.split("_");

        for section in sectional {
            if section.contains("$") {
                let _without_dollar = section.replace("$", "");
                let answer_length = _without_dollar.len();
                let _response = ui.add(egui::TextEdit::singleline(&mut answers[*answer_index as usize]).desired_width((answer_length*7) as f32));
                *answer_index+=1;
            }else {
                ui.add(egui::Label::new(section));
            }
        }
    });
}
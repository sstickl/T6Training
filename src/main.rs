// LIBRARIES
use eframe::{run_native, CreationContext};
//use egui::{CentralPanel, Ui};
//use std::collections::HashMap;

// MODS
mod app;
mod graphics;
mod boldface;

// Structs


// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<(), eframe::Error> {    //egui app
    //setup our native options
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon( //add our icon
                eframe::icon_data::from_png_bytes(&include_bytes!("res/T6T.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    //run the app native
    run_native(
        "T6 App", 
        native_options, 
        Box::new(|cc: &CreationContext<'_>| {
            egui_extras::install_image_loaders(&cc.egui_ctx);   // This gives us image support:
            Ok(Box::<app::T6App>::default())
        }),
    )
}


// MODS
mod app; // The main app
mod boldface; // The boldface ops/procedures
mod graphics; // The graphics, holds widgets, various graphics things

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<(), eframe::Error> {
    //setup our native options
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            //add our icon
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/T6T.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    //run the app native
    eframe::run_native(
        "T6 App",
        native_options,
        Box::new(|cc: &eframe::CreationContext<'_>| {
            egui_extras::install_image_loaders(&cc.egui_ctx); // This gives us image support:
            //let style = egui::Style {
                //need to fix this so the app configures for dark/light
            //    ..egui::Style::default()
            //};
            //cc.egui_ctx.set_theme(egui::Theme::Dark);
            graphics::configure_text_styles(&cc.egui_ctx);
            Ok(Box::<app::T6App>::default())
        }),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    //eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc: &eframe::CreationContext<'_>| {
                    egui_extras::install_image_loaders(&cc.egui_ctx); // This gives us image support:
                    let _style = egui::Style {
                        visuals: egui::Visuals::default(),
                        ..egui::Style::default()
                    };
                    Ok(Box::<app::T6App>::default())
                }),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

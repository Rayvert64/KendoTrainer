/// This is the main struct for the app.
/// We need to consider this as a glue to multiple state machines
/// which will be the main components of the app.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct KendoTrainer {
    /// This is the label of the text edit:
    label: String,

    /// This is the value of the slider:
    /// @TODO: remove this example field:
    value: f32,
}

impl Default for KendoTrainer {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl KendoTrainer {
    /// This is the constructor of the app.
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self::default()
    }
}

impl eframe::App for KendoTrainer {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui_menu_bar| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui_menu_bar.menu_button("File", |ui_file_btn| {
                        if ui_file_btn.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui_menu_bar.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui_menu_bar);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui_central_panel| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui_central_panel.heading("eframe template");

            ui_central_panel.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui_central_panel.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui_central_panel.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui_central_panel.separator();

            ui_central_panel.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            ui_central_panel.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

/// This is the documentation for the `powered_by_egui_and_eframe` function.
/// This function is part of the `egui_template` template.
fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui_about_egui| {
        ui_about_egui.spacing_mut().item_spacing.x = 0.0;
        ui_about_egui.label("Powered by ");
        ui_about_egui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui_about_egui.label(" and ");
        ui_about_egui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui_about_egui.label(".");
    });
}

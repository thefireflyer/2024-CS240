///////////////////////////////////////////////////////////////////////////////

use egui::{CentralPanel, SidePanel, Vec2, Window};

use crate::gui::App;

///////////////////////////////////////////////////////////////////////////////

pub fn modal_view(app: &mut App, ctx: &egui::Context) {
    for modal in &mut app.modals {
        match modal {
            crate::gui::modals::Modal::FindFile => Window::new("Find Graph").show(ctx, |ui| {}),
            crate::gui::modals::Modal::SaveFile => Window::new("Save Graph")
                .resize(|r| r.auto_sized())
                .show(ctx, |ui| {
                    egui::TopBottomPanel::top("my_panel").show_inside(ui, |ui| {
                        egui::menu::bar(ui, |ui| {
                            ui.menu_button("File", |ui| {
                                if ui.button("New File").clicked() {
                                    ui.close_menu();
                                }
                            });

                            ui.menu_button("View", |ui| {});
                        });
                    });

                    SidePanel::left("left-save")
                        .exact_width(200.0)
                        .show_inside(ui, |ui| {
                            ui.label("Directories");
                        });
                    CentralPanel::default().show_inside(ui, |ui| {
                        ui.label("Files");
                    });
                }),
        };
    }
}

///////////////////////////////////////////////////////////////////////////////

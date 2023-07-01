// #![windows_subsystem = "windows"]
use eframe::egui;
use anyhow::Result;
use egui::Color32;
use itertools::Itertools;
use nfd2::Response;
use pathdiff::diff_paths;
use srenamer::{apply_rename, rename_map};
use std::{env, path::PathBuf, collections::HashMap, str::FromStr, ffi::OsStr};

#[derive(Default)]
struct AppState {
    pub file: String,
    pub input: String,
    pub cwd: PathBuf,
    pub rename: Vec<(PathBuf, PathBuf)>,
    pub dupes: HashMap<PathBuf, usize>,
    pub error_msg: String
}

impl AppState {
    fn new(cc: &eframe::CreationContext<'_>, file: String, cwd: PathBuf) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        cc.egui_ctx.set_pixels_per_point(1.5);
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut res = AppState { input: file.clone(), file, cwd, ..Default::default() };
        res.update_rename();
        res
    }

    fn update_rename(&mut self) {
        self.rename = rename_map(
            &self.cwd,
            &self.file,
            &PathBuf::from_str(&self.input).unwrap(),
        );
        self.dupes = self.rename.iter().map(|(_old, new)| new).cloned().counts();
        let dupe_count = self.rename.len() - self.dupes.len();
        if dupe_count > 0 {
            self.error_msg = format!(
                "{} files have duplicate name, cannot rename as they would be lost.",
                dupe_count
            );
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(self.file.clone());
                    if ui.text_edit_singleline(&mut self.input).changed() {
                        self.error_msg = String::new();
                        self.update_rename()
                    }
                });
                ui.add_space(10.);
                if ui.button("Change dest").clicked() {
                    match nfd2::open_pick_folder(None) {
                        Ok(response) => match response {
                            Response::Okay(folder_path) => {
                                if let Some(diff) = diff_paths(&folder_path, &self.cwd) {
                                    self.input = diff
                                        .join(
                                            PathBuf::from_str(&self.input)
                                                .unwrap()
                                                .file_name()
                                                .unwrap_or(OsStr::new(""))
                                                .to_string_lossy()
                                                .to_string(),
                                        )
                                        .to_string_lossy()
                                        .to_string()
                                        .replace("\\", "/");
                                    self.update_rename();
                                } else {
                                    self.error_msg =
                                        format!("Couldn't get relative path from {:?}", folder_path);
                                }
                            }
                            Response::OkayMultiple(_folder_path) => {
                                self.error_msg = String::from("Cannot select multiple folders.")
                            }
                            Response::Cancel => {}
                        },
                        Err(err) => {
                            self.error_msg = format!("Error: {}", err);
                        }
                    }
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Apply").clicked() {
                        match apply_rename(&self.rename) {
                            Ok(()) => frame.close(),
                            Err(err) => self.error_msg = format!("{}", err),
                        }
                    }
                });
            });
            if self.error_msg.len() > 0 {
                ui.label(egui::RichText::new(self.error_msg.clone()).color(Color32::from_rgb(200, 20, 50)));
            }
            ui.add_space(10.);
            ui.separator();
            egui::ScrollArea::vertical().show(
                ui, |ui| for (old, new) in self.rename.iter() {
                    ui.label(old.to_str().unwrap_or(""));
                    ui.strong(new.to_str().unwrap_or(""));
                    ui.separator();
                }
            );
        });
    }
}


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut cwd = env::current_dir().unwrap();
    let file = if args.len() == 1 {
        match nfd2::open_file_dialog(None, None) {
            Ok(response) => match response {
                Response::Okay(file_path) => {
                    cwd = file_path.parent().unwrap().to_path_buf();
                    file_path.file_name().unwrap().to_string_lossy().to_string()
                }
                Response::OkayMultiple(file_paths) => {
                    let file_path = &file_paths[0];
                    cwd = file_path.parent().unwrap().to_path_buf();
                    file_path.file_name().unwrap().to_string_lossy().to_string()
                }
                Response::Cancel => return Ok(()),
            },
            Err(err) => {
                println!("{:?}", err);
                return Ok(());
            }
        }
    } else {
        let file_path = PathBuf::from(args[1].clone());
        if let Some(parent) = file_path.parent() {
            cwd.push(parent);
        }
        file_path.file_name().unwrap().to_string_lossy().to_string()
    };
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Simple Renamer", native_options, 
        Box::new(|cc| Box::new(AppState::new(cc, file, cwd)))
    ).unwrap();
    Ok(())
}

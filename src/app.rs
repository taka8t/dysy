use crate::attractors::{Attractor, Trigonometric, Clifford, Quadratic, Symmetric, Polar, Duffing, Lorenz, DoublePendulum};
use crate::util;
use image::{EncodableLayout, DynamicImage};
use serde_json::{Value};
use anyhow::{Result, anyhow};
use std::time;
use std::path::PathBuf;
use std::fs;
use std::io::{BufWriter, Write, BufReader};

#[derive(Debug, PartialEq)]
enum Enum {
    Trigonometric,
    Clifford,
    Quadratic,
    Symmetric,
    Polar,
    Duffing,
    Lorenz,
    DoublePendulum
}

pub struct MyApp {
    num_iter_low: usize,
    num_iter_high: usize,
    attractor: Box<dyn Attractor>,
    selected_attractor: Enum,
    palette: util::Palette,
    open_window: bool,
    tex_handle_pre: Option<egui::TextureHandle>,
    tex_handle_high: Option<egui::TextureHandle>,
    elapsed: time::Duration,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            num_iter_low: 100000,
            num_iter_high: 10000000,
            attractor: Box::<Trigonometric>::default(),
            selected_attractor: Enum::Trigonometric,
            palette: util::Palette::default(),
            open_window: false,
            tex_handle_pre: None,
            tex_handle_high: None,
            elapsed: time::Duration::new(0, 0),
        }
    }
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
    
    fn set_attractor(&mut self, at: Box<dyn Attractor> ) {
        self.attractor = at;
    }

    fn save_params(&self, path: &PathBuf) -> std::io::Result<()> {
        let file = fs::File::create(path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self.attractor)?;
        writer.flush()?;
        Ok(())
    }

    fn load_params(&mut self, path: &PathBuf) -> Result<()> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let de: Value = serde_json::from_reader(reader)?;
        if let Some(Value::String(name)) = de.get("name") {
            match name.as_str() {
                "Trigonometric Attractor" => {
                    self.attractor = Box::new(serde_json::from_value::<Trigonometric>(de)?);
                },
                "Clifford Attractor" => {
                    self.attractor = Box::new(serde_json::from_value::<Clifford>(de)?);
                },
                "Quadratic Attractor" => {
                    self.attractor = Box::new(serde_json::from_value::<Quadratic>(de)?);
                },
                "Symmetric Attractor" => {
                    self.attractor = Box::new(serde_json::from_value::<Symmetric>(de)?);
                },
                "Polar Attractor" => {
                    self.attractor = Box::new(serde_json::from_value::<Polar>(de)?);
                },
                "Lorenz Attractor" => {
                    self.attractor = Box::new(serde_json::from_value::<Lorenz>(de)?);
                },
                "Duffing Attractor" => {
                    self.attractor = Box::new(serde_json::from_value::<Duffing>(de)?);
                },
                "DoublePendulum" => {
                    self.attractor = Box::new(serde_json::from_value::<DoublePendulum>(de)?);
                },
                _ => {
                    return Err(anyhow!("Invald Attractor name {}.", name.clone()));
                }
            }
        }
        else {
            return Err(anyhow!(" Attractor name NotFound."));
        }
        Ok(())
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut param_changed = false;
        let mut color_changed = false;
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
            ui.heading("attractor generator egui");
            ui.separator();

            // todo: 項目別に分ける
            egui::ComboBox::from_label("Select Attractor")
            .selected_text(format!("{:?}", self.selected_attractor))
            .show_ui(ui, |ui| {
                if ui.selectable_value(&mut self.selected_attractor, Enum::Trigonometric, "Trigonometric").clicked() {
                    self.set_attractor(Box::<Trigonometric>::default());
                    param_changed |= true;
                }
                if ui.selectable_value(&mut self.selected_attractor, Enum::Clifford, "Clifford").clicked() {
                    self.set_attractor(Box::<Clifford>::default());
                    param_changed |= true;
                }
                if ui.selectable_value(&mut self.selected_attractor, Enum::Quadratic, "Quadratic").clicked() {
                    self.set_attractor(Box::<Quadratic>::default());
                    param_changed |= true;
                }
                if ui.selectable_value(&mut self.selected_attractor, Enum::Symmetric, "Symmetric").clicked() {
                    self.set_attractor(Box::<Symmetric>::default());
                    param_changed |= true;
                }
                if ui.selectable_value(&mut self.selected_attractor, Enum::Polar, "Polar").clicked() {
                    self.set_attractor(Box::<Polar>::default());
                    param_changed |= true;
                }
                if ui.selectable_value(&mut self.selected_attractor, Enum::Duffing, "Duffing").clicked() {
                    self.set_attractor(Box::<Duffing>::default());
                    param_changed |= true;
                }
                if ui.selectable_value(&mut self.selected_attractor, Enum::Lorenz, "Lorentz").clicked() {
                    self.set_attractor(Box::<Lorenz>::default());
                    param_changed |= true;
                }
                if ui.selectable_value(&mut self.selected_attractor, Enum::DoublePendulum, "DoublePendulum").clicked() {
                    self.set_attractor(Box::<DoublePendulum>::default());
                    param_changed |= true;
                }
            });

            ui.label(self.attractor.name());
            ui.label(self.attractor.map_str());
        });

        egui::SidePanel::right("palette param").show(ctx, |ui| {
            ui.label("palette param");
            
            if ui.add(egui::Button::new("Randomize")).clicked() {
                self.palette.change_random();
                color_changed |= true;
            }

            ui.label("R:");
            ui.horizontal(|ui|{
                color_changed |= ui.add(egui::DragValue::new(&mut self.palette.r.0).clamp_range(0.5..=1.0).fixed_decimals(2).speed(0.01)).changed();
                color_changed |= ui.add(egui::DragValue::new(&mut self.palette.r.1).clamp_range(0.0..=0.5).fixed_decimals(2).speed(0.01)).changed();
                color_changed |= ui.add(egui::DragValue::new(&mut self.palette.r.2).clamp_range(0.5..=1.5).fixed_decimals(2).speed(0.01)).changed();
                color_changed |= ui.add(egui::DragValue::new(&mut self.palette.r.3).clamp_range(0.0..=1.0).fixed_decimals(2).speed(0.01)).changed();
            });
            ui.label("G:");
            ui.horizontal(|ui|{
                color_changed |= ui.add(egui::DragValue::new(&mut self.palette.g.0).clamp_range(0.5..=1.0).fixed_decimals(2).speed(0.01)).changed();
                color_changed |= ui.add(egui::DragValue::new(&mut self.palette.g.1).clamp_range(0.0..=0.5).fixed_decimals(2).speed(0.01)).changed();
                color_changed |= ui.add(egui::DragValue::new(&mut self.palette.g.2).clamp_range(0.5..=1.5).fixed_decimals(2).speed(0.01)).changed();
                color_changed |= ui.add(egui::DragValue::new(&mut self.palette.g.3).clamp_range(0.0..=1.0).fixed_decimals(2).speed(0.01)).changed();
            });
            ui.label("B:");
            ui.horizontal(|ui|{
                color_changed |= ui.add(egui::DragValue::new(&mut self.palette.b.0).clamp_range(0.5..=1.0).fixed_decimals(2).speed(0.01)).changed();
                color_changed |= ui.add(egui::DragValue::new(&mut self.palette.b.1).clamp_range(0.0..=0.5).fixed_decimals(2).speed(0.01)).changed();
                color_changed |= ui.add(egui::DragValue::new(&mut self.palette.b.2).clamp_range(0.5..=1.5).fixed_decimals(2).speed(0.01)).changed();
                color_changed |= ui.add(egui::DragValue::new(&mut self.palette.b.3).clamp_range(0.0..=1.0).fixed_decimals(2).speed(0.01)).changed();
            });
            ui.label("color variation:");
            ui.horizontal(|ui|{
                color_changed |= ui.add(egui::DragValue::new(&mut self.palette.colver1).clamp_range(0.0..=1.0).fixed_decimals(2).speed(0.01).prefix("var1: ")).changed();
                color_changed |= ui.add(egui::DragValue::new(&mut self.palette.colver2).clamp_range(0.0..=8.0).fixed_decimals(2).speed(0.02).prefix("var2: ")).changed();
                
            });
            ui.label("color brightness:");
            ui.horizontal(|ui|{
                color_changed |= ui.add(egui::DragValue::new(&mut self.palette.brightness1).clamp_range(0.0..=2.0).fixed_decimals(2).speed(0.01).prefix("value1: ")).changed();
                color_changed |= ui.add(egui::DragValue::new(&mut self.palette.brightness2).clamp_range(1.0..=100.0).fixed_decimals(1).speed(0.2).prefix("value2: ")).changed();
            });
        });

        // todo: add fix zero
        egui::SidePanel::left("map param").show(ctx, |ui| {
            let mut changed_left = false;

            ui.label("map param");
            ui.label("num iter for check");
            ui.add(
                egui::Slider::new(&mut self.num_iter_low, 10000..=1000000)
                .logarithmic(true)
            );

            ui.separator();
            ui.label("Initial Values");
            if ui.add(egui::Button::new("Randomize")).clicked() {
                self.attractor.state_mut().set_random_init();
                changed_left |= true;
            }
            let x_range = self.attractor.state_mut().get_x_range();
            for (i, x) in self.attractor.state_mut().get_init_val_mut().iter_mut().enumerate() {
                changed_left |= ui.add(
                    egui::DragValue::new(x)
                    .clamp_range(x_range.clone())
                    .fixed_decimals(4)
                    .speed(x_range.end() * 0.001)
                    .prefix(format!("x{}:  ", i))
                ).changed();
            }
            ui.separator();

            let dt_range = self.attractor.state().get_dt_range();
            if let Some(dt) = self.attractor.state_mut().get_dt_mut() {
                if let Some(dt_range) = dt_range {
                    changed_left |= ui.add(
                        egui::DragValue::new(dt)
                        .clamp_range(0.0..=dt_range)
                        .fixed_decimals(5)
                        .speed(dt_range*0.01)
                        .prefix(format!("dt: "))
                    ).changed();
                } 
            }
            
            ui.separator();
            if ui.add(egui::Button::new("Randomize")).clicked() {
                self.attractor.change_random_coefs();
                changed_left |= true;
            }
            
            ui.add_space(5.0);
            
            let ranges = self.attractor.coef_ranges();
            let speeds = self.attractor.speeds();
            for (i,(coef, (range, speed))) in self.attractor.coefs_mut().iter_mut().zip(ranges.into_iter().zip(speeds.into_iter())).enumerate() {
                changed_left |= ui.add(
                    egui::DragValue::new(coef)
                    .clamp_range(range)
                    .fixed_decimals(3)
                    .speed(speed)
                    .prefix(format!("a{}:  ", i))
                ).changed();
            }

            ui.separator();
            if ui.add(egui::Button::new("Save Params")).clicked() {
                let dialog = rfd::FileDialog::new()
                    .set_file_name(self.attractor.name().replace(' ', "_"))
                    .set_directory("/")
                    .add_filter("JSON", &["json"])
                    .save_file();
                if let Some(path) = dialog {
                    let save_result = self.save_params(&path);
                    rfd::MessageDialog::new()
                    .set_title("Message")
                    .set_description(
                        if save_result.is_ok() {format!("{:?} saved", path.file_name().unwrap())} else {"Failed to save".to_owned()}
                    )
                    .set_buttons(rfd::MessageButtons::Ok)
                    .show();
                }
            }
            if ui.add(egui::Button::new("Load Params")).clicked() {
                let dialog = rfd::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .set_directory("/")
                    .pick_file();
                if let Some(path) = dialog {
                    let load_result = self.load_params(&path);
                    rfd::MessageDialog::new()
                    .set_title("Message")
                    .set_description(
                        if load_result.is_ok() {format!("{:?} loaded", path.file_name().unwrap())} else {"Failed to load".to_owned()}
                    )
                    .set_buttons(rfd::MessageButtons::Ok)
                    .show();
                    if load_result.is_ok() {
                        changed_left |= true;
                    }
                }
            }
            param_changed |= changed_left;
        });

        if param_changed {
            self.attractor.param_changed(true);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui|{
                ui.add(
                    egui::Slider::new(&mut self.num_iter_high, 1000000..=50000000)
                    .logarithmic(true)
                    .text("num iter for high resolution")
                );
                if ui.add(egui::Button::new("Generate")).clicked() {
                    let start = time::Instant::now();
                    let image = image2texture(
                        self.attractor.gen_img(self.num_iter_high, 1024, 1024, &self.palette)
                    );
                    self.elapsed = start.elapsed();
                    self.tex_handle_high = Some(ctx.load_texture("high_image", image, Default::default()));
                    self.open_window = true;
                    self.attractor.param_changed(false);
                }
            });
            
            if (param_changed || color_changed) && !self.open_window {
                self.attractor.param_changed(true);
                let image = image2texture(
                    self.attractor.gen_img(self.num_iter_low, 512, 512, &self.palette)
                );
                self.tex_handle_pre = Some(ctx.load_texture("pre_image", image, Default::default())); 
            }
            if let Some(handle) = &self.tex_handle_pre {
                let image = egui::Image::from_texture(
                    egui::load::SizedTexture::new(handle.id(), handle.size_vec2())
                );
                ui.add(image);
            }
        });

        egui::Window::new("high_resolution_image").open(&mut self.open_window).show(ctx, |ui| {
            ui.label("high resolution image");
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("Update")).clicked() {
                    let start = time::Instant::now();
                    let image = image2texture(
                        self.attractor.gen_img(self.num_iter_high, 1024, 1024, &self.palette)
                    );
                    self.elapsed = start.elapsed();
                    self.tex_handle_high = Some(ctx.load_texture("high_image", image, Default::default()));
                    self.attractor.param_changed(false);
                }
                ui.label(format!("{:.3} sec", self.elapsed.as_secs_f32()));
                
                if ui.add(egui::Button::new("Save Image")).clicked() {
                    let dialog = rfd::FileDialog::new()
                        .set_file_name(self.attractor.name().replace(' ', "_"))
                        .set_directory("/")
                        .add_filter("PNG", &["png"])
                        .save_file();
                    if let Some(path) = dialog {
                        let save_result = self.attractor.save_img(&path, self.num_iter_high, 1024, 1024, &self.palette);
                        rfd::MessageDialog::new()
                        .set_title("Message")
                        .set_description(
                            if save_result.is_ok() {format!("{:?} saved", path.file_name().unwrap())} else {"Failed to save".to_owned()}
                        )
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();
                    }
                }
            });
            
            if let Some(handle) = &self.tex_handle_high {
                let image = egui::Image::from_texture(
                    egui::load::SizedTexture::new(handle.id(), handle.size_vec2())
                ).shrink_to_fit()
                .maintain_aspect_ratio(true);
                ui.add(image);
            }
        });
        
    }
}

fn image2texture(img: DynamicImage) -> egui::ColorImage {
    match &img {
        DynamicImage::ImageRgb8(image) => {
            egui::ColorImage::from_rgb(
                [image.width() as usize, image.height() as usize],
                image.as_bytes(),
            )
        },
        other => {
            let image = other.to_rgba8();
            egui::ColorImage::from_rgba_unmultiplied(
                [image.width() as usize, image.height() as usize],
                image.as_bytes(),
            )
        },
    }
}
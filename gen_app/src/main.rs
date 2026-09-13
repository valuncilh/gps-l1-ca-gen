//! GPS L1 C/A Code Generator — GUI on egui/eframe.
//!
//! Modules:
//! - `lfsr_sv`   — dynamic LFSR generator (taps passed as parameter)
//! - `lfsr_ffi`  — FFI wrapper over the C implementation (reference check)
//! - `corr`      — cyclic correlation (AKF and KF via a single function)

mod corr;
mod lfsr_ffi;
mod lfsr_sv;

use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use lfsr_sv::{Lfsr, PERIOD};

/// UI color theme.
#[derive(PartialEq, Clone, Copy)]
enum Theme {
    Dark,
    Light,
}

struct MyApp {
    // Phase selector taps for three satellites
    tap1: usize, tap2: usize,   // SV2 (primary)
    tap3: usize, tap4: usize,   // SV65
    tap5: usize, tap6: usize,   // SV_new

    // Generated sequences
    bits_sv2: Vec<u8>,
    bits_sv_new: Vec<u8>,

    // Plot data: up to two lines
    lines: Vec<(String, Vec<[f64; 2]>)>,
    colors: [egui::Color32; 2],
    line_width: f32,

    // UI state
    show_plot: bool,
    theme: Theme,
    status: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            tap1: 2, tap2: 6,
            tap3: 3, tap4: 8,
            tap5: 5, tap6: 9,
            bits_sv2: Vec::new(),
            bits_sv_new: Vec::new(),
            lines: Vec::new(),
            colors: [
                egui::Color32::from_rgb(80, 170, 255),
                egui::Color32::from_rgb(255, 120, 60),
            ],
            line_width: 1.5,
            show_plot: false,
            theme: Theme::Dark,
            status: String::new(),
        }
    }
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();
        app.apply_theme(&cc.egui_ctx);
        app
    }

    fn generate_bits(tap_a: usize, tap_b: usize) -> [u8; PERIOD] {
        Lfsr::new((tap_a, tap_b)).generate()
    }

    /// Expands correlation (length PERIOD) to three periods along X.
    fn expand_three_periods(corr: &[i32]) -> Vec<[f64; 2]> {
        let n = corr.len() as i32;
        let mut out = Vec::with_capacity((3 * corr.len()) as usize);
        for tau in -n..(2 * n) {
            let idx = (((tau % n) + n) % n) as usize;
            out.push([tau as f64, corr[idx] as f64]);
        }
        out
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        match self.theme {
            Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
            Theme::Light => ctx.set_visuals(egui::Visuals::light()),
        }
    }

    /// Computes cross-correlation between SV2 and a satellite with given taps.
    fn compute_kf(&self, tap_a: usize, tap_b: usize, name: &str) -> (String, Vec<[f64; 2]>) {
        let bits_a = Self::generate_bits(self.tap1, self.tap2);
        let bits_b = Self::generate_bits(tap_a, tap_b);
        let sa = lfsr_ffi::to_pm1(&bits_a);
        let sb = lfsr_ffi::to_pm1(&bits_b);
        let kf = corr::correlate(&sa, &sb);
        let label = format!("KF SV2 vs {}   R(0)={}", name, kf[0]);
        (label, Self::expand_three_periods(&kf))
    }

    /// Generates PERIOD + 20 bits: one full period + 20 bits of the next.
    /// Used to verify code periodicity.
    fn generate_extended(tap_a: usize, tap_b: usize) -> Vec<u8> {
        let mut lfsr = Lfsr::new((tap_a, tap_b));
        let p1 = lfsr.generate();
        let p2 = lfsr.generate();
        let mut out = p1.to_vec();
        out.extend_from_slice(&p2[..20]);
        out
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Force repaint every frame: fixes stale content after maximize/restore.
        ctx.request_repaint();

        self.apply_theme(ctx);

        // Control panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("GPS L1 C/A Code Generator");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let btn = if self.theme == Theme::Dark { "☀ Light" } else { "🌙 Dark" };
                    if ui.button(btn).clicked() {
                        self.theme = match self.theme {
                            Theme::Dark => Theme::Light,
                            Theme::Light => Theme::Dark,
                        };
                    }
                });
            });
            ui.separator();

            // SV2
            ui.label("SV2 (primary):");
            ui.horizontal(|ui| {
                ui.label("tap1:");
                ui.add(egui::DragValue::new(&mut self.tap1).clamp_range(0..=9));
                ui.label("tap2:");
                ui.add(egui::DragValue::new(&mut self.tap2).clamp_range(0..=9));
            });
            ui.horizontal(|ui| {
                if ui.button("Generate").clicked() {
                    self.bits_sv2 = Self::generate_bits(self.tap1, self.tap2).to_vec();
                    self.status.clear();
                }
                if ui.button("Extended (to clipboard)").clicked() {
                    let ext = Self::generate_extended(self.tap1, self.tap2);
                    let s: String = ext.iter().map(|b| b.to_string()).collect();
                    ui.output_mut(|o| o.copied_text = s);
                    self.status = format!("SV2: copied {} bits to clipboard", ext.len());
                    self.bits_sv2 = ext;
                }
            });

            ui.separator();

            // SV65
            ui.label("SV65:");
            ui.horizontal(|ui| {
                ui.label("tap1:");
                ui.add(egui::DragValue::new(&mut self.tap3).clamp_range(0..=9));
                ui.label("tap2:");
                ui.add(egui::DragValue::new(&mut self.tap4).clamp_range(0..=9));
            });

            ui.separator();

            // SV_new
            ui.label("SV_new:");
            ui.horizontal(|ui| {
                ui.label("tap1:");
                ui.add(egui::DragValue::new(&mut self.tap5).clamp_range(0..=9));
                ui.label("tap2:");
                ui.add(egui::DragValue::new(&mut self.tap6).clamp_range(0..=9));
            });
            if ui.button("Generate").clicked() {
                self.bits_sv_new = Self::generate_bits(self.tap5, self.tap6).to_vec();
                self.status.clear();
            }

            ui.separator();

            // Correlations
            ui.horizontal(|ui| {
                if ui.button("AKF (SV2)").clicked() {
                    let bits = Self::generate_bits(self.tap1, self.tap2);
                    let s = lfsr_ffi::to_pm1(&bits);
                    let akf = corr::correlate(&s, &s);
                    let label = format!("AKF SV2   R(0)={}", akf[0]);
                    self.lines = vec![(label, Self::expand_three_periods(&akf))];
                    self.show_plot = true;
                }
                if ui.button("KF").clicked() {
                    let line = self.compute_kf(self.tap3, self.tap4, "SV65");
                    self.lines = vec![line];
                    self.show_plot = true;
                }
                if ui.button("KF + KF").clicked() {
                    let l1 = self.compute_kf(self.tap3, self.tap4, "SV65");
                    let l2 = self.compute_kf(self.tap5, self.tap6, "SV_new");
                    self.lines = vec![l1, l2];
                    self.show_plot = true;
                }
            });

            ui.separator();

            // Results
            if !self.bits_sv2.is_empty() {
                let first10: String = self.bits_sv2.iter().take(10).map(|b| b.to_string()).collect();
                ui.label(format!("SV2 first 10 bits: {}", first10));
                ui.label(format!("SV2 length: {}", self.bits_sv2.len()));
            }
            if !self.bits_sv_new.is_empty() {
                let first10: String = self.bits_sv_new.iter().take(10).map(|b| b.to_string()).collect();
                ui.label(format!("SV_new first 10 bits: {}", first10));
                ui.label(format!("SV_new length: {}", self.bits_sv_new.len()));
            }
            if !self.status.is_empty() {
                ui.colored_label(egui::Color32::from_rgb(120, 220, 120), &self.status);
            }
        });

        // Separate plot window
        if self.show_plot && !self.lines.is_empty() {
            let lines_data = self.lines.clone();
            let mut colors = self.colors;
            let mut width = self.line_width;
            let mut close_requested = false;

            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("corr_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Correlation function")
                    .with_inner_size([950.0, 500.0])
                    .with_min_inner_size([400.0, 200.0])
                    .with_resizable(true),
                |ctx, _class| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Line width:");
                            ui.add(egui::Slider::new(&mut width, 0.5..=4.0));
                            ui.separator();
                            for (i, (label, _)) in lines_data.iter().enumerate() {
                                ui.color_edit_button_srgba(&mut colors[i]);
                                ui.colored_label(colors[i], format!("■ {}", label));
                                ui.separator();
                            }
                        });
                        ui.separator();

                        Plot::new("corr_plot")
                            .allow_zoom(true)
                            .allow_drag(true)
                            .allow_scroll(true)
                            .show(ui, |plot_ui| {
                                for (i, (_, pts)) in lines_data.iter().enumerate() {
                                    let line = Line::new(PlotPoints::from(pts.clone()))
                                        .color(colors[i])
                                        .width(width);
                                    plot_ui.line(line);
                                }
                            });
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        close_requested = true;
                    }
                },
            );

            self.colors = colors;
            self.line_width = width;
            if close_requested {
                self.show_plot = false;
            }
        }
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("gps-l1-ca-gen")
            .with_inner_size([640.0, 580.0])
            .with_resizable(true),
        ..Default::default()
    };
    eframe::run_native(
        "gps-l1-ca-gen",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

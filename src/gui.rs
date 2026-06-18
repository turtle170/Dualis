use eframe::egui;
use std::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

pub fn run_gui(tx: Sender<String>, gui_rx: Receiver<()>) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_inner_size([700.0, 70.0])
            .with_visible(false), // Hidden initially
        ..Default::default()
    };

    eframe::run_native(
        "Dualis Command",
        options,
        Box::new(|_cc| Ok(Box::new(DualisApp::new(tx, gui_rx)))),
    )
}

struct DualisApp {
    tx: Sender<String>,
    gui_rx: Receiver<()>,
    command: String,
    is_visible: bool,
}

impl DualisApp {
    fn new(tx: Sender<String>, gui_rx: Receiver<()>) -> Self {
        Self {
            tx,
            gui_rx,
            command: String::new(),
            is_visible: false,
        }
    }
}

impl eframe::App for DualisApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // Completely transparent window background
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check if hotkey was pressed
        if let Ok(_) = self.gui_rx.try_recv() {
            self.is_visible = !self.is_visible;
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(self.is_visible));
            if self.is_visible {
                // Position at bottom center
                if let Some(monitor_size) = ctx.input(|i| i.viewport().monitor_size) {
                    let window_width = 700.0;
                    let window_height = 70.0;
                    let x = (monitor_size.x - window_width) / 2.0;
                    let y = monitor_size.y - window_height - 60.0; // 60px from bottom
                    ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(x, y)));
                }
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            }
        }

        if !self.is_visible {
            // Re-render continuously to catch hotkey events smoothly, 
            // or just rely on context requesting repaint.
            ctx.request_repaint();
            return;
        }

        // Apply a global dark visual theme
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.inactive.bg_fill = egui::Color32::from_black_alpha(100);
        visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
        visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_white_alpha(30));
        
        visuals.widgets.hovered.bg_fill = egui::Color32::from_black_alpha(150);
        visuals.widgets.hovered.rounding = egui::Rounding::same(8.0);
        visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_white_alpha(50));
        
        // Use 'active' and 'noninteractive' fields as fallbacks
        visuals.widgets.active.bg_fill = egui::Color32::from_black_alpha(200);
        visuals.widgets.active.rounding = egui::Rounding::same(8.0);
        visuals.widgets.active.bg_stroke = egui::Stroke::new(1.5, egui::Color32::from_rgba_premultiplied(100, 255, 120, 200));
        visuals.selection.bg_fill = egui::Color32::from_rgba_premultiplied(50, 200, 80, 100);
        ctx.set_visuals(visuals);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(egui::Color32::from_rgba_premultiplied(20, 20, 25, 245))
                    .rounding(20.0)
                    .stroke(egui::Stroke::new(1.5_f32, egui::Color32::from_rgba_premultiplied(80, 255, 120, 180)))
                    .inner_margin(egui::Margin::symmetric(20.0, 15.0))
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("🟢 Dualis")
                        .color(egui::Color32::from_rgb(100, 255, 120))
                        .size(20.0)
                        .strong());
                    
                    ui.add_space(15.0);
                    
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.command)
                            .hint_text(egui::RichText::new("Ask the copilot anything...").color(egui::Color32::from_white_alpha(150)).size(16.0))
                            .desired_width(ui.available_width())
                            .text_color(egui::Color32::WHITE)
                            .margin(egui::vec2(12.0, 10.0))
                            .font(egui::FontId::proportional(16.0))
                    );

                    if self.is_visible && !response.has_focus() {
                        response.request_focus();
                    }

                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        let cmd = self.command.clone();
                        self.command.clear();
                        self.is_visible = false;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
                        
                        let tx = self.tx.clone();
                        tokio::spawn(async move {
                            let _ = tx.send(cmd).await;
                        });
                    }
                });
            });

        ctx.request_repaint();
    }
}

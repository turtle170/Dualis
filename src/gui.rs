use eframe::egui;
use std::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

pub fn run_gui(tx: Sender<String>, gui_rx: Receiver<()>) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top(true)
            .with_inner_size([400.0, 100.0])
            .with_position(egui::pos2(500.0, 200.0)) // Center-ish
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Check if hotkey was pressed
        if let Ok(_) = self.gui_rx.try_recv() {
            self.is_visible = !self.is_visible;
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(self.is_visible));
            if self.is_visible {
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            }
        }

        if !self.is_visible {
            // Re-render continuously to catch hotkey events smoothly, 
            // or just rely on context requesting repaint.
            ctx.request_repaint();
            return;
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_black_alpha(200)).rounding(10.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.label(egui::RichText::new("Dualis Copilot").color(egui::Color32::GREEN).size(16.0));
                    
                    ui.add_space(10.0);
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.command)
                            .hint_text("Enter command...")
                            .desired_width(350.0)
                            .margin(egui::vec2(10.0, 10.0))
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

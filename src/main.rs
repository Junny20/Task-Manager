mod app;
mod channel;
mod cpusnapshot;
mod config;
mod data;
mod graph;
mod workers;

use app::SystemMonitorApp;
use channel::Channel;
use cpusnapshot::CpuSnapshot;

fn main() -> eframe::Result<()> {
    let channel: Channel<CpuSnapshot> = Channel::new();
    let (sender, receiver) = channel.split();
    workers::cpu(sender);

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
        .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "CPU Monitor",
        options,
        Box::new(|_cc| Ok(Box::new(SystemMonitorApp::new(receiver)))),
    )
}

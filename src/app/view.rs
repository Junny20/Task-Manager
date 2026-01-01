use std::time::Duration;

use eframe::egui::Context;

use crate::{
    app::app::AppMonitor,
    config::{
        app_variables::REFRESH_MILLISECONDS,
    },
    cpusnapshot::CpuSnapshot,
};

pub fn try_receive_latest_cpu_snapshot(app_monitor: &mut AppMonitor) -> Option<CpuSnapshot> {
    let mut latest: Option<CpuSnapshot> = None;

    while let Ok(cpu_snapshot) = app_monitor.channels.cpu_snapshot_receiver.try_recv() {
        latest = Some(cpu_snapshot);
    }

    latest
}

pub fn render_ui(ctx: &Context, app_monitor: &mut AppMonitor) {
    // cpu_monitor helper function
    app_monitor.cpu_monitor.cpu_monitor_render_ui(ctx);
}

pub fn request_repaint(ctx: &Context) {
    // refreshes the gui every REFRESH_MILLISECONDS milliseconds
    ctx.request_repaint_after(Duration::from_millis(REFRESH_MILLISECONDS));
}

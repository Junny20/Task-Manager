use eframe::egui::Context;

use crate::{
    app::{
        state::change_system_monitor_app_state,
        view::{render_ui, request_repaint, try_receive_latest_cpu_snapshot},
        app::AppMonitor,
    },
    cpusnapshot::CpuSnapshot,
};

// INVARIANTS:
// change_system_monitor_app_state only runs when a cpu_snapshot is actually received.
//

pub fn update(system_monitor_app: &mut AppMonitor, ctx: &Context) {
    let potential_cpu_snapshot: Option<CpuSnapshot> =
        try_receive_latest_cpu_snapshot(system_monitor_app);

    if let Some(cpu_snapshot) = potential_cpu_snapshot {
        change_system_monitor_app_state(cpu_snapshot, system_monitor_app);
    };

    // refreshes the gui
    request_repaint(ctx);
    render_ui(ctx, system_monitor_app);
}

use crate::{
    app::app::AppMonitor, 
    cpusnapshot::CpuSnapshot,
};

// INVARIANTS:
// This function only triggers when a valid cpu_snapshot is received.
// As such, cpu_snapshot is not an Option type - it is guaranteed to exist.

pub fn change_system_monitor_app_state(cpu_snapshot: CpuSnapshot, app_monitor: &mut AppMonitor) {
    app_monitor.cpu_monitor.cpu_monitor_apply_cpu_snapshot(cpu_snapshot);
}

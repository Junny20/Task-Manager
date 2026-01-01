use std::time::Duration;

use eframe::egui::{vec2, CentralPanel, Context, Sense, UiBuilder, Vec2};

use crate::{
    app::SystemMonitorApp,
    config::{
        app_variables::REFRESH_MILLISECONDS,
        layout::{CELL_HEIGHT_PX, LEFT_CELL_WIDTH_PX, SPACING_PX},
    },
    cpusnapshot::CpuSnapshot,
    graph::draw::draw_ui_graph,
};

pub fn try_receive_latest_cpu_snapshot(system_monitor_app: &mut SystemMonitorApp) -> Option<CpuSnapshot> {
    let mut latest: Option<CpuSnapshot> = None;

    while let Ok(cpu_snapshot) = system_monitor_app.receiver.try_recv() {
        latest = Some(cpu_snapshot);
    }

    latest
}

pub fn render_ui(ctx: &Context, system_monitor_app: &SystemMonitorApp) {
    // the show method takes a closure and builds the gui
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("CPU Monitor");
        ui.add_space(SPACING_PX);

        if let Some(cpu_snapshot) = &system_monitor_app.latest_snapshot {
            // ===== OVERALL CPU USAGE =====
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    let left_cell: Vec2 = vec2(LEFT_CELL_WIDTH_PX, CELL_HEIGHT_PX);
                    let (rect, _) = ui.allocate_exact_size(left_cell, Sense::hover());

                    ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                        // space to align with CPU Core labels
                        ui.label("CPU    ");
                        ui.add_space(SPACING_PX);
                        // value formatted to one decimal place
                        ui.monospace(format!("{:>5.1}%", cpu_snapshot.overall_cpu_usage));
                        ui.add_space(SPACING_PX);
                    });

                    let right_cell: Vec2 = vec2(ui.available_width(), CELL_HEIGHT_PX);
                    let (rect, _) = ui.allocate_exact_size(right_cell, Sense::hover());

                    draw_ui_graph(
                        &rect,
                        ui,
                        &system_monitor_app.overall_cpu_history,
                        Some(&system_monitor_app.overall_ema_cpu_history),
                    );
                });
            });

            ui.add_space(SPACING_PX);

            // ===== PER CORE CPU USAGE =====
            if let Some(per_core_history) = &system_monitor_app.per_core_cpu_history {
                for (index, history) in per_core_history.iter().enumerate() {
                    let usage: f32 = cpu_snapshot.per_core_cpu_usage[index];

                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            let left_cell: Vec2 = vec2(LEFT_CELL_WIDTH_PX, CELL_HEIGHT_PX);
                            let (rect, _) = ui.allocate_exact_size(left_cell, Sense::hover());

                            ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                                ui.label(format!("Core {}", index));
                                ui.add_space(SPACING_PX);
                                // value formatted to one decimal place
                                ui.monospace(format!("{:>5.1}%", usage));
                                ui.add_space(SPACING_PX);
                            });

                            let desired_size = vec2(ui.available_width(), CELL_HEIGHT_PX);
                            let (rect, _) = ui.allocate_exact_size(desired_size, Sense::hover());

                            draw_ui_graph(&rect, ui, history, None);
                        });
                    });

                    ui.add_space(SPACING_PX);
                }
            }
        } else {
            // Note: this arm should never trigger
            ui.label("Waiting for CPU data…");
        }
    });
}

pub fn request_repaint(ctx: &Context) {
    // refreshes the gui every REFRESH_MILLISECONDS milliseconds
    ctx.request_repaint_after(Duration::from_millis(REFRESH_MILLISECONDS));
}

use crate::{
    app::update::update,
    config::{
        app_variables::{CORES_UPPER_LIMIT, MAX_LINE_GRAPH_POINTS},
        layout::{
            CELL_HEIGHT_PX, LEFT_CELL_WIDTH_PX, PROGRESS_BAR_HEIGHT_PX, PROGRESS_BAR_ROUNDING_PX,
            PROGRESS_BAR_SPACING_PX, PROGRESS_BAR_WIDTH_PX, TEXT_SPACING_PX,
        },
        style::HALF_OPACITY,
    },
    cpusnapshot::CpuSnapshot,
    data::exponential_moving_average::{
        get_cpu_exponential_moving_average, get_per_core_exponential_moving_average,
    },
    graph::{draw::draw_ui_graph, style::get_color},
};

use eframe::egui::{
    vec2, CentralPanel, Color32, Context, ProgressBar, Response, ScrollArea, Sense, UiBuilder, Vec2,
};

use std::{collections::VecDeque, sync::mpsc::Receiver};

pub struct Channels {
    pub cpu_snapshot_receiver: Receiver<CpuSnapshot>,
}

impl Channels {
    pub fn new(cpu_snapshot_receiver: Receiver<CpuSnapshot>) -> Channels {
        Channels {
            cpu_snapshot_receiver,
        }
    }
}

pub struct CpuMonitor {
    pub per_core_cpu_history: Option<Vec<VecDeque<f32>>>,
    pub per_core_ema_cpu_history: Option<Vec<VecDeque<f32>>>,
    pub per_core_previous_ema: Vec<Option<f32>>,
    pub previous_ema: Option<f32>,
    pub overall_cpu_history: VecDeque<f32>,
    pub overall_ema_cpu_history: VecDeque<f32>,
}

impl CpuMonitor {
    pub fn new() -> CpuMonitor {
        CpuMonitor {
            per_core_cpu_history: None,
            per_core_ema_cpu_history: None,
            per_core_previous_ema: Vec::with_capacity(CORES_UPPER_LIMIT),
            previous_ema: None,
            // uses with_capacity instead of new constructor to reduce heap reallocations.
            overall_cpu_history: VecDeque::with_capacity(CORES_UPPER_LIMIT),
            overall_ema_cpu_history: VecDeque::with_capacity(CORES_UPPER_LIMIT),
        }
    }

    // INVARIANTS:
    // exponential moving average is guaranteed to exist after the first cpu snapshot.
    // history charts have a maximum of 10 data points - that is what MAX_LINE_GRAPH_POINTS refers to.

    pub fn cpu_monitor_apply_cpu_snapshot(&mut self, cpu_snapshot: CpuSnapshot) {
        self.overall_cpu_history
            .push_back(cpu_snapshot.overall_cpu_usage);
        if self.overall_cpu_history.len() > MAX_LINE_GRAPH_POINTS {
            self.overall_cpu_history.pop_front();
        }

        let overall_cpu_exponential_moving_average: f32 =
            get_cpu_exponential_moving_average(self.previous_ema, cpu_snapshot.overall_cpu_usage);

        self.previous_ema = Some(overall_cpu_exponential_moving_average);
        self.overall_ema_cpu_history
            .push_back(overall_cpu_exponential_moving_average);
        if self.overall_ema_cpu_history.len() > MAX_LINE_GRAPH_POINTS {
            self.overall_ema_cpu_history.pop_front();
        } // separate function in CpuMonitor struct

        // constructs per core cpu histories if not constructed
        if let None = self.per_core_cpu_history {
            let n: usize = cpu_snapshot.per_core_cpu_usage.len();

            self.per_core_cpu_history = Some(vec![VecDeque::new(); n]);
            self.per_core_ema_cpu_history = Some(vec![VecDeque::new(); n]);

            // constructs per core ema history
            self.per_core_previous_ema = vec![None; n];
        }

        get_per_core_exponential_moving_average(
            &mut self.per_core_previous_ema,
            &cpu_snapshot.per_core_cpu_usage,
        );

        for (index, ema) in self.per_core_previous_ema.iter().enumerate() {
            let per_core_ema_values: &mut VecDeque<f32> =
                &mut self.per_core_ema_cpu_history.as_mut().unwrap()[index];
            per_core_ema_values.push_back(ema.unwrap());
            if per_core_ema_values.len() > MAX_LINE_GRAPH_POINTS {
                per_core_ema_values.pop_front();
            }
        }

        let per_core_cpu_history: &mut Vec<VecDeque<f32>> =
            self.per_core_cpu_history.as_mut().unwrap();

        for (index, value) in cpu_snapshot.per_core_cpu_usage.iter().enumerate() {
            let per_core_values: &mut VecDeque<f32> = &mut per_core_cpu_history[index];
            per_core_values.push_back(*value);
            if per_core_values.len() > MAX_LINE_GRAPH_POINTS {
                per_core_values.pop_front();
            }
        }
    }

    pub fn cpu_monitor_render_ui(&self, ctx: &Context) {
        // the show method takes a closure and builds the gui
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("CPU Monitor");
            ui.add_space(TEXT_SPACING_PX);

            // ===== OVERALL CPU USAGE =====
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    let left_cell: Vec2 = vec2(LEFT_CELL_WIDTH_PX, CELL_HEIGHT_PX);
                    let (rect, _) = ui.allocate_exact_size(left_cell, Sense::hover());

                    ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                        // space to align with CPU Core labels
                        ui.label("CPU ");
                        ui.add_space(TEXT_SPACING_PX);

                        if let Some(overall_cpu_usage) = self.overall_cpu_history.back() {
                            // value formatted to one decimal place
                            ui.monospace(format!("{:>5.1}%", overall_cpu_usage));
                            ui.add_space(PROGRESS_BAR_SPACING_PX); // magic nums

                            let color: Color32 = get_color(*overall_cpu_usage, HALF_OPACITY);

                            let progress_bar: ProgressBar = self.build_progress_bar(
                                *overall_cpu_usage,
                                PROGRESS_BAR_WIDTH_PX,
                                PROGRESS_BAR_HEIGHT_PX,
                                PROGRESS_BAR_ROUNDING_PX,
                                color,
                            );

                            let _response: Response = ui.add(progress_bar);

                            ui.add_space(TEXT_SPACING_PX);
                        }
                    });

                    let right_cell: Vec2 = vec2(ui.available_width(), CELL_HEIGHT_PX);
                    let (rect, _) = ui.allocate_exact_size(right_cell, Sense::hover());

                    draw_ui_graph(
                        &rect,
                        ui,
                        &self.overall_cpu_history,
                        Some(&self.overall_ema_cpu_history),
                    );
                });
            });

            ui.add_space(TEXT_SPACING_PX);

            ScrollArea::vertical().show(ui, |ui| {
                // ===== PER CORE CPU USAGE =====
                if let Some(per_core_history) = &self.per_core_cpu_history {
                    for (index, history) in per_core_history.iter().enumerate() {
                        let usage: &f32 = history.back().unwrap(); // Check if this always works!

                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                let left_cell: Vec2 = vec2(LEFT_CELL_WIDTH_PX, CELL_HEIGHT_PX);
                                let (rect, _) = ui.allocate_exact_size(left_cell, Sense::hover());

                                ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                                    ui.label(format!("Core {}", index));
                                    // value formatted to one decimal place
                                    ui.monospace(format!("{:>5.1}%", *usage));
                                    ui.add_space(PROGRESS_BAR_SPACING_PX);

                                    let color: Color32 = get_color(*usage, HALF_OPACITY);

                                    let progress_bar: ProgressBar = self.build_progress_bar(
                                        *usage,
                                        PROGRESS_BAR_WIDTH_PX,
                                        PROGRESS_BAR_HEIGHT_PX,
                                        PROGRESS_BAR_ROUNDING_PX,
                                        color,
                                    );

                                    let _response: Response = ui.add(progress_bar);

                                    ui.add_space(TEXT_SPACING_PX);
                                });

                                let desired_size = vec2(ui.available_width(), CELL_HEIGHT_PX);
                                let (rect, _) =
                                    ui.allocate_exact_size(desired_size, Sense::hover());

                                draw_ui_graph(
                                    &rect,
                                    ui,
                                    history,
                                    Some(&self.overall_ema_cpu_history),
                                );
                            });
                        });

                        ui.add_space(TEXT_SPACING_PX);
                    }
                }
            });
        });
    }

    pub fn build_progress_bar(
        &self,
        value: f32,
        width: f32,
        height: f32,
        rounding: f32,
        color: Color32,
    ) -> ProgressBar {
        ProgressBar::new(value / 100 as f32)
            .desired_width(width)
            .desired_height(height)
            .fill(color)
            .corner_radius(rounding)
            .show_percentage()
    }
}

pub struct AppMonitor {
    pub channels: Channels,
    pub cpu_monitor: CpuMonitor,
}

impl AppMonitor {
    pub fn new(cpu_snapshot_receiver: Receiver<CpuSnapshot>) -> Self {
        Self {
            channels: Channels::new(cpu_snapshot_receiver),
            cpu_monitor: CpuMonitor::new(),
        }
    }
}

impl eframe::App for AppMonitor {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        update(self, ctx);
    }
}

use eframe::egui::*;
use std::{sync::mpsc::Receiver, time::Duration, collections::VecDeque};
use crate::cpusnapshot::CpuSnapshot;

const ROLLING_GRAPH_HEIGHT_PX: f32 = 80.0;

pub struct SystemMonitorApp {
    receiver: Receiver<CpuSnapshot>,
    latest_snapshot: Option<CpuSnapshot>,
    per_core_cpu_history: Option<Vec<VecDeque<f32>>>,
    overall_cpu_history: VecDeque<f32>,
}

impl SystemMonitorApp {
    pub fn new(receiver: Receiver<CpuSnapshot>) -> Self {
        Self {
            receiver,
            latest_snapshot: None,
            overall_cpu_history: VecDeque::new(),
            per_core_cpu_history: None,
        }
    }
}

impl eframe::App for SystemMonitorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        while let Ok(cpu_snapshot) = self.receiver.try_recv() {
            self.latest_snapshot = Some(cpu_snapshot);
            if let Some(cpu_snapshot) = &self.latest_snapshot {
                self.overall_cpu_history.push_back(cpu_snapshot.overall_cpu_usage);
                if self.overall_cpu_history.len() > 10 {
                    self.overall_cpu_history.pop_front();
                }

                if let None = &mut self.per_core_cpu_history {
                    let n: usize = cpu_snapshot.per_core_cpu_usage.len();
                    self.per_core_cpu_history = Some(vec![VecDeque::new(); n]);
                }

                let per_core_cpu_history: &mut Vec<VecDeque<f32>> = self.per_core_cpu_history.as_mut().unwrap();
                for (index, value) in cpu_snapshot.per_core_cpu_usage.iter().enumerate() {
                    let per_core_values: &mut VecDeque<f32> = &mut per_core_cpu_history[index];
                    per_core_values.push_back(*value);
                    if per_core_values.len() > 10 {
                        per_core_values.pop_front();
                    }
                }
            }
        }
        

        ctx.request_repaint_after(Duration::from_millis(16)); // magic num

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust System Monitor");
            ui.add_space(10.0);

            if let Some(cpu_snapshot) = &self.latest_snapshot {
                ui.group(|ui| {
                    ui.label("Overall CPU Usage");

                    let usage = cpu_snapshot.overall_cpu_usage / 100.0;
                    ui.add(
                        ProgressBar::new(usage)
                            .text(format!("{:.1}%", cpu_snapshot.overall_cpu_usage))
                    );
                });

                ui.add_space(12.0);

                ui.group(|ui| {
                    ui.label("Per-Core Usage");
                    ui.add_space(6.0);

                    for (index, value) in cpu_snapshot.per_core_cpu_usage.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("Core {:>2}", index));

                            ui.add(
                                ProgressBar::new(*value / 100.0)
                                    .desired_width(160.0)
                            );

                            ui.label(format!("{:>5.1}%", value));
                        });
                    }
                });

                ui.group(|ui| {
                    ui.label("Rolling graph overall cpu history");
                    // allocates a rectangle size in form of length width vector
                    let desired_size: Vec2 = vec2(ui.available_width(), ROLLING_GRAPH_HEIGHT_PX);
                    // creates rectangle in ui
                    let (rect, _response): (Rect, Response) = 
                        ui.allocate_exact_size(desired_size, Sense::hover());
                    draw_graph(&rect, ui, &self.overall_cpu_history);
                });

                for core in self.per_core_cpu_history.as_mut().unwrap() {
                    ui.group(|ui| {
                        let desired_size: Vec2 = vec2(ui.available_width(), ROLLING_GRAPH_HEIGHT_PX);
                        let (rect, _response): (Rect, Response) = 
                            ui.allocate_exact_size(desired_size, Sense::hover());
                        draw_graph(&rect, ui, core);
                    });
                }
            } else {
                ui.label("Waiting for CPU data…");
            }
        });
    }
}

fn draw_graph(rect: &Rect, ui: &mut Ui, history: &VecDeque<f32>) {
    let painter: Painter = ui.painter_at(*rect);

    painter.rect_filled(
        *rect,
        4.0,
        ui.visuals().extreme_bg_color,
    );

    painter.rect_stroke(
        *rect,
        4.0,
        Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color),
        StrokeKind::Outside,
    );

    let n: usize = history.len();
    let points: Vec<Pos2> = history.iter().enumerate().map(|(index, value)| make_point(index, value, n, rect)).collect();
    for segment in points.windows(2) { // magic num
        painter.line_segment(
            [segment[0], segment[1]], 
            Stroke::new(2.0, Color32::LIGHT_GREEN)
        );
    }

}

fn make_point(index: usize, value: &f32, n: usize, rect: &Rect) -> Pos2 {
    let plot_rect = rect.shrink(4.0);
    let plot_rect_height: f32 = plot_rect.bottom() - plot_rect.top();
    let x: f32 = plot_rect.left() + (index as f32) / ((n - 1) as f32) * plot_rect.width();
    let y: f32 = plot_rect.bottom() - (value / 100.0) * plot_rect_height; // magic num
    Pos2 { x, y }
}
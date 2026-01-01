mod state;
mod update;
mod view;

use crate::app::update::update;
use crate::config::app_variables::CORES_UPPER_LIMIT;
use crate::cpusnapshot::CpuSnapshot;
use eframe::egui::Context;
use std::{collections::VecDeque, sync::mpsc::Receiver};

pub struct SystemMonitorApp {
    previous_ema: Option<f32>,
    receiver: Receiver<CpuSnapshot>,
    latest_snapshot: Option<CpuSnapshot>,
    per_core_cpu_history: Option<Vec<VecDeque<f32>>>,
    overall_cpu_history: VecDeque<f32>,
    overall_ema_cpu_history: VecDeque<f32>,
}

impl SystemMonitorApp {
    pub fn new(receiver: Receiver<CpuSnapshot>) -> Self {
        Self {
            previous_ema: None,
            receiver,
            latest_snapshot: None,
            per_core_cpu_history: None,
            overall_cpu_history: VecDeque::with_capacity(CORES_UPPER_LIMIT),
            overall_ema_cpu_history: VecDeque::with_capacity(CORES_UPPER_LIMIT),
        }
    }
}

impl eframe::App for SystemMonitorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        update(self, ctx);
    }
}

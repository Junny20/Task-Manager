use std::collections::VecDeque;

use crate::{
    config::app_variables::{CORES_UPPER_LIMIT, MAX_LINE_GRAPH_POINTS},
    data::exponential_moving_average::{
        get_cpu_exponential_moving_average, get_per_core_exponential_moving_average,
    },
    snapshots::cpu_snapshot_struct::CpuSnapshot,
};

/// A sub-structure of the AppMonitor structure.
/// Stores all relevant cpu data produced from consuming a CpuSnapshot structure.
/// Note: ema stands for exponential moving average.
pub struct CpuMonitor {
    pub average_cpu_usage: f32,
    pub cumulative_cpu_usage: f32,
    pub per_core_cpu_history: Option<Vec<VecDeque<f32>>>,
    pub per_core_ema_cpu_history: Option<Vec<VecDeque<f32>>>,
    pub per_core_previous_ema: Vec<Option<f32>>,
    pub previous_ema: Option<f32>,
    pub overall_cpu_history: VecDeque<f32>,
    pub overall_ema_cpu_history: VecDeque<f32>,
    pub total_snapshots_received: i128,
}

impl CpuMonitor {
    /// Constructor for the CpuMonitor structure.
    ///
    /// # Returns
    /// CpuMonitor struture

    // INVARIANTS:
    // Per core functionality is represented as an Option initialized as None because
    // we have no way of knowing how many cores the device has until the first CpuSnapshot
    // arrives.
    pub fn new() -> CpuMonitor {
        CpuMonitor {
            average_cpu_usage: 0.0,
            cumulative_cpu_usage: 0.0,
            per_core_cpu_history: None,
            per_core_ema_cpu_history: None,
            per_core_previous_ema: Vec::with_capacity(CORES_UPPER_LIMIT),
            previous_ema: None,
            // uses with_capacity instead of new constructor to reduce heap reallocations.
            overall_cpu_history: VecDeque::with_capacity(CORES_UPPER_LIMIT),
            overall_ema_cpu_history: VecDeque::with_capacity(CORES_UPPER_LIMIT),
            total_snapshots_received: 0,
        }
    }

    /// Takes in a CpuSnapshot struct and updates the fields in CpuMonitor.
    ///
    /// * Parameters
    /// `cpu_snapshot` CpuSnapshot structure

    // INVARIANTS:
    // All per core charts are constructed when the first CpuSnapshot structure is received.
    // exponential moving average is guaranteed to exist after the first cpu snapshot.
    // history charts have a maximum of 10 data points - that is what MAX_LINE_GRAPH_POINTS refers to.
    pub fn cpu_monitor_apply_cpu_snapshot(&mut self, cpu_snapshot: CpuSnapshot) {
        self.adjust_average_cpu_usage(&cpu_snapshot);

        self.overall_cpu_history_add_point(&cpu_snapshot);
        self.overall_ema_cpu_history_add_point(&cpu_snapshot);

        // constructs per core line charts if not constructed
        self.construct_per_core_line_charts(&cpu_snapshot);

        // mutates in-place the per core ema line chart
        get_per_core_exponential_moving_average(
            &mut self.per_core_previous_ema,
            &cpu_snapshot.per_core_cpu_usage,
        );

        // adds the latest values in the per core ema line chart to each of the cpu core line charts
        self.per_core_ema_cpu_history_add_point();

        self.per_core_cpu_history_add_point(&cpu_snapshot);
    }

    /// Adjusts average cpu usage
    /// 
    /// * Parameters
    /// `cpu_snapshot` CpuSnapshot structure 
    fn adjust_average_cpu_usage(&mut self, cpu_snapshot: &CpuSnapshot) {
        self.total_snapshots_received += 1;
        self.cumulative_cpu_usage += cpu_snapshot.overall_cpu_usage;
        self.average_cpu_usage = self.cumulative_cpu_usage / self.total_snapshots_received as f32;
    }

    /// Adds a data point to the overall CPU history.
    ///
    /// Maintains a maximum number of points by removing the oldest if exceeded.
    ///
    /// * Parameters
    /// `cpu_snapshot` Reference to the CPU snapshot containing the usage data
    fn overall_cpu_history_add_point(&mut self, cpu_snapshot: &CpuSnapshot) {
        self.overall_cpu_history
            .push_back(cpu_snapshot.overall_cpu_usage);
        if self.overall_cpu_history.len() > MAX_LINE_GRAPH_POINTS {
            self.overall_cpu_history.pop_front();
        }
    }

    /// Adds a data point to the overall EMA CPU history.
    ///
    /// Calculates the exponential moving average and maintains history size.
    ///
    /// * Parameters
    /// `cpu_snapshot` Reference to the CPU snapshot containing the usage data
    fn overall_ema_cpu_history_add_point(&mut self, cpu_snapshot: &CpuSnapshot) {
        let overall_cpu_exponential_moving_average: f32 =
            get_cpu_exponential_moving_average(self.previous_ema, cpu_snapshot.overall_cpu_usage);

        self.previous_ema = Some(overall_cpu_exponential_moving_average);
        self.overall_ema_cpu_history
            .push_back(overall_cpu_exponential_moving_average);
        if self.overall_ema_cpu_history.len() > MAX_LINE_GRAPH_POINTS {
            self.overall_ema_cpu_history.pop_front();
        }
    }

    /// Constructs per-core line charts if they haven't been initialized yet.
    ///
    /// Initializes vectors for per-core CPU and EMA histories based on the number of cores.
    ///
    /// * Parameters
    /// `cpu_snapshot` Reference to the CPU snapshot to determine the number of cores
    fn construct_per_core_line_charts(&mut self, cpu_snapshot: &CpuSnapshot) {
        if let None = self.per_core_cpu_history {
            let n: usize = cpu_snapshot.per_core_cpu_usage.len();

            self.per_core_cpu_history = Some(vec![VecDeque::new(); n]);
            self.per_core_ema_cpu_history = Some(vec![VecDeque::new(); n]);

            // constructs per core ema history
            self.per_core_previous_ema = vec![None; n];
        }
    }

    /// Adds the latest EMA values to the per-core EMA history.
    ///
    /// Iterates through each core's EMA and appends to the history, maintaining max size.
    fn per_core_ema_cpu_history_add_point(&mut self) {
        for (index, ema) in self.per_core_previous_ema.iter().enumerate() {
            let per_core_ema_values: &mut VecDeque<f32> =
                &mut self.per_core_ema_cpu_history.as_mut().unwrap()[index];
            per_core_ema_values.push_back(ema.unwrap());
            if per_core_ema_values.len() > MAX_LINE_GRAPH_POINTS {
                per_core_ema_values.pop_front();
            }
        }
    }

    /// Adds the latest per-core CPU usage values to the history.
    ///
    /// Appends current usage for each core and removes oldest if exceeding max points.
    ///
    /// * Parameters
    /// `cpu_snapshot` Reference to the CPU snapshot containing per-core usage data
    fn per_core_cpu_history_add_point(&mut self, cpu_snapshot: &CpuSnapshot) {
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
}

use std::collections::VecDeque;

use crate::{
    app::SystemMonitorApp, 
    config::app_variables::MAX_LINE_GRAPH_POINTS,
    data::exponential_moving_average::calculate_exponential_moving_average,
};

// INVARIANTS:
// exponential moving average is guaranteed to exist after the first cpu snapshot.
// history charts have a maximum of 10 data points - that is what MAX_LINE_GRAPH_POINTS refers to.

pub fn change_system_monitor_app_state(system_monitor_app: &mut SystemMonitorApp) {
    if let Some(cpu_snapshot) = &system_monitor_app.latest_snapshot {
        let overall_cpu_usage: &f32 = &cpu_snapshot.overall_cpu_usage;

        system_monitor_app
            .overall_cpu_history
            .push_back(*overall_cpu_usage);
        if system_monitor_app.overall_cpu_history.len() > MAX_LINE_GRAPH_POINTS {
            system_monitor_app.overall_cpu_history.pop_front();
        }

        let exponential_moving_average: f32 = match system_monitor_app.previous_ema {
            Some(previous_ema) => {
                calculate_exponential_moving_average(previous_ema, *overall_cpu_usage)
            }
            None => *overall_cpu_usage,
        };
        system_monitor_app.previous_ema = Some(exponential_moving_average);

        system_monitor_app
            .overall_ema_cpu_history
            .push_back(exponential_moving_average);
        if system_monitor_app.overall_ema_cpu_history.len() > MAX_LINE_GRAPH_POINTS {
            system_monitor_app.overall_ema_cpu_history.pop_front();
        }

        if let None = &mut system_monitor_app.per_core_cpu_history {
            let n: usize = cpu_snapshot.per_core_cpu_usage.len();

            // implements the with_capacity method
            system_monitor_app.per_core_cpu_history = Some(vec![VecDeque::new(); n]);
        }

        let per_core_cpu_history: &mut Vec<VecDeque<f32>> =
            system_monitor_app.per_core_cpu_history.as_mut().unwrap();

        for (index, value) in cpu_snapshot.per_core_cpu_usage.iter().enumerate() {
            let per_core_values: &mut VecDeque<f32> = &mut per_core_cpu_history[index];
            per_core_values.push_back(*value);
            if per_core_values.len() > MAX_LINE_GRAPH_POINTS {
                per_core_values.pop_front();
            }
        }
    } else {
        println!("Waiting for CPU data!");
    }
}
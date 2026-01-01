const SMOOTHING_FACTOR: f32 = 0.4;

pub fn get_cpu_exponential_moving_average(previous_ema: Option<f32>, usage: f32) -> f32 {
    match previous_ema {
        Some(previous_ema) => calculate_exponential_moving_average(previous_ema, usage),
        None => usage,
    }
}

pub fn get_per_core_exponential_moving_average(previous_emas: &mut Vec<Option<f32>>, usage: &Vec<f32>) {
    for (index, ema) in previous_emas.iter_mut().enumerate() {
        *ema = match *ema {
            Some(ema) => Some(calculate_exponential_moving_average(ema, usage[index])),
            None => Some(usage[index])
        }
    }
}

pub fn calculate_exponential_moving_average(previous_ema: f32, usage: f32) -> f32 {
    usage * SMOOTHING_FACTOR + previous_ema * (1 as f32 - SMOOTHING_FACTOR)
}
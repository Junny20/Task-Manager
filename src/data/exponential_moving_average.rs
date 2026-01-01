const SMOOTHING_FACTOR: f32 = 0.3;

pub fn get_exponential_moving_average() {
    // implement?
}

pub fn calculate_exponential_moving_average(previous_ema: f32, usage: f32) -> f32 {
    usage * SMOOTHING_FACTOR + previous_ema * (1 as f32 - SMOOTHING_FACTOR)
}
use eframe::egui::Color32;
use crate::config::style::{
    GREEN_LINE_THICKNESS, 
    YELLOW_LINE_THICKNESS, 
    YELLOW_LINE_THRESHOLD, 
    RED_LINE_THICKNESS, 
    RED_LINE_THRESHOLD
};

pub fn get_color(value: f32, opacity: u8) -> Color32 {
    match value {
        value if value < YELLOW_LINE_THRESHOLD => 
            Color32::from_rgba_unmultiplied(0, 255, 0, opacity),

        value if value < RED_LINE_THRESHOLD => 
            Color32::from_rgba_unmultiplied(255, 255, 0, opacity),
        // red line otherwise
        _ => 
            Color32::from_rgba_unmultiplied(255, 0, 0, opacity),
    }
}

pub fn find_stroke_width(value: f32) -> f32 {
    match value {
        value if value < YELLOW_LINE_THRESHOLD => GREEN_LINE_THICKNESS,
        value if value < RED_LINE_THRESHOLD => YELLOW_LINE_THICKNESS,
        _ => RED_LINE_THICKNESS,
    }
}
use eframe::egui::{Pos2, Rect};
use crate::config::style::RECT_SHRINK_AMNT;

pub fn make_point(index: usize, value: &f32, n: usize, rect: &Rect) -> Pos2 {
    let plot_rect = rect.shrink(RECT_SHRINK_AMNT);
    let plot_rect_height: f32 = plot_rect.bottom() - plot_rect.top();
    let x: f32 = plot_rect.left() + (index as f32) / ((n - 1) as f32) * plot_rect.width();
    let y: f32 = plot_rect.bottom() - (value / 100 as f32) * plot_rect_height;
    Pos2 { x, y }
}
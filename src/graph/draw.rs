use std::collections::VecDeque;
use eframe::egui::{Color32, Pos2, Painter, Rangef, Rect, Stroke, StrokeKind, Ui};
use crate::config::{
    layout::{CELL_CORNER_RADIUS_PX, LINE_THICKNESS_ONE_PX}, 
    style::{EMA_GRAPH_OPACITY, DOTTED_LINE_GAP_PX, DOTTED_LINE_LENGTH_PX, HALF_OPACITY},
    app_variables::{LAST_INDEX}
};
use crate::graph::{geometry::make_point, style::{get_color, find_stroke_width}};

pub fn draw_ui_graph(rect: &Rect, ui: &mut Ui, history: &VecDeque<f32>, ema_history: Option<&VecDeque<f32>>) {
    let painter: Painter = ui.painter_at(*rect);

    painter.rect_filled(
        *rect,
        CELL_CORNER_RADIUS_PX,
        ui.visuals().extreme_bg_color,
    );

    painter.rect_stroke(
        *rect,
        CELL_CORNER_RADIUS_PX,
        Stroke::new(LINE_THICKNESS_ONE_PX, ui.visuals().widgets.noninteractive.bg_stroke.color),
        StrokeKind::Outside,
    );

    // draws a dotted line at 50% of the height of the rectangle
    let half: f32 = rect.bottom() - ((rect.bottom() - rect.top()) / 2 as f32);
    draw_dotted_hline(&rect, half, &painter);

    draw_line_graph(rect, history, &painter, HALF_OPACITY);

    if let Some(ema_history) = ema_history {
        draw_line_graph(rect, ema_history, &painter, EMA_GRAPH_OPACITY);
    }
}

pub fn draw_line_graph(rect: &Rect, history: &VecDeque<f32>, painter: &Painter, opacity: u8) {
    let n: usize = history.len();
    let points: Vec<Pos2> = 
        history.iter().enumerate().map(|(index, value)| make_point(index, value, n, rect)).collect();

    for (index, segment) in points.windows(2).enumerate() {
        let color: Color32;
        let stroke_width: f32;
        match index {
            LAST_INDEX => {
                let value: f32 = history[index];
                color = get_color(value, opacity);
                stroke_width = find_stroke_width(value)
            },
            _ => {
                let value: f32 = (history[index] + history[index + 1]) / 2 as f32;
                color = get_color(value, opacity);
                stroke_width = find_stroke_width(value)
            }
        }
        painter.line_segment(
            [segment[0], segment[1]], 
            Stroke::new(stroke_width, color)
        );
    } 
}

pub fn draw_dotted_hline(rect: &Rect, y: f32, painter: &Painter) {
    let mut dotted: Vec<Rangef> = Vec::new();
    let mut start: f32 = rect.left();
    let end: f32 = rect.right();
    while start < end {
        dotted.push(Rangef { min: start, max: start + DOTTED_LINE_LENGTH_PX });
        start += DOTTED_LINE_GAP_PX;
    }
    for range in dotted.into_iter() {
        painter.hline(range, y, Stroke::new(LINE_THICKNESS_ONE_PX, Color32::GRAY));
    }
}
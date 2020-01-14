use std::f64;

use svg;

use crate::axis;
use crate::repr::ContinuousRepresentation;
use crate::style::PointStyle;
use crate::svg_render;
use crate::text_render;


pub struct ColorMap {
    colors: Vec<(f64, u8, u8, u8)>
}

impl ColorMap{
    pub fn new() -> Self(){
        let cols = [(0.0, 0,0,0), (1.0, 255,255,255)]
        ColorMap{colors: cols}
    }

    pub fn get_color(v: f64) -> (u8, u8, u8){
        (v.max(1).min(0)*255.0 as u8, 
        v.max(1).min(0)*255.0 as u8, 
        v.max(1).min(0)*255.0 as u8)
    }
}

/// The scatter *representation*.
/// It knows its data as well how to style itself
#[derive(Debug)]
pub struct Contour {
    pub data: Vec<(f64, f64, f64)>,
    style: PointStyle,
    Map: ColorMap, 
}

impl Contour {
    pub fn from_slice(v: &[(f64, f64, f64)]) -> Self {
        let mut data: Vec<(f64, f64, f64)> = vec![];
        for &(x, y, z) in v {
            data.push((x, y, z));
        }
        Contour {
            data,
            style: PointStyle::new(),
            col: ColorMap::new(),
        }
    }

    pub fn style(mut self, style: &PointStyle) -> Self {
        self.style.overlay(style);
        self
    }

    pub fn get_style(&self) -> &PointStyle {
        &self.style
    }

    fn x_range(&self) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for &(x, _, _) in &self.data {
            min = min.min(x);
            max = max.max(x);
        }
        (min, max)
    }

    fn y_range(&self) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for &(_, y, _) in &self.data {
            min = min.min(y);
            max = max.max(y);
        }
        (min, max)
    }

    fn z_range(&self) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for &(_, _, z) in &self.data {
            min = min.min(z);
            max = max.max(z);
        }
        (min, max)
    }
}

impl ContinuousRepresentation for Contour {
    fn range(&self, dim: u32) -> (f64, f64) {
        match dim {
            0 => self.x_range(),
            1 => self.y_range(),
            2 => self.z_range(),
            _ => panic!("Axis out of range"),
        }
    }

    fn to_svg(
        &self,
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: f64,
        face_height: f64,
    ) -> svg::node::element::Group {
        svg_render::draw_face_points(
            &self.data,
            x_axis,
            y_axis,
            face_width,
            face_height,
            &self.style,
        )
    }
    fn legend_svg(&self) -> Option<svg::node::element::Group> {
        // TODO implement
        None
    }

    fn to_text(
        &self,
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: u32,
        face_height: u32,
    ) -> String {
        text_render::render_face_points(
            &self.data,
            x_axis,
            y_axis,
            face_width,
            face_height,
            &self.style,
        )
    }
}

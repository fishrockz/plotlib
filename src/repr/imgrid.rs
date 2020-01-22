use std::f64;

use svg;

use crate::axis;
use crate::repr::ContinuousRepresentation;
use crate::style::PointStyle;
use crate::svg_render;
use crate::text_render;
use crate::colormap::viridisMap;


/// The scatter *representation*.
/// It knows its data as well how to style itself
#[derive(Debug)]
pub struct Imgrid {
    pub data: Vec<f64>,
    x_width: u64,
    y_width: u64,
    style: PointStyle,
    map: viridisMap, 
}

impl Imgrid {
    pub fn from_slice(v: &[f64], x_width: u64, y_width: u64) -> Self {
        let mut data: Vec<f64> = vec![];
        for &z in v {
            data.push(z);
        }
        Imgrid {
            data,
            x_width: x_width, y_width: y_width,
            style: PointStyle::new(),
            map: viridisMap::new(),
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
        (0f64, self.x_width as f64)
    }

    fn y_range(&self) -> (f64, f64) {
        (0f64, self.y_width as f64)
    }

    fn z_range(&self) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for &z in &self.data {
            min = min.min(z);
            max = max.max(z);
        }
        (min, max)
    }
}

impl ContinuousRepresentation for Imgrid {
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
        svg_render::draw_face_imgrid(
            &self.data,
            x_axis,
            y_axis,
            face_width,
            face_height,
            self.x_width,
            self.y_width,
            &self.style,
            &self.map,
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
        "Not implemented".to_string()
    }
}

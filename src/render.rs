use crate::axis;
use crate::style;

pub trait Renderer{
    fn face_points(&mut self,
        s: &[(f64, f64)],
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: f64,
        face_height: f64,
        style: &style::PointStyle,
    );
}






use crate::axis;
use crate::render::Renderer;
use crate::style;

use druid::{PaintCtx, piet::{RenderContext, Color}, kurbo::{Size, Circle, Line, Point}};


pub struct PlotterPaintCtx<'a, 'b, 'c> {
    context: &'c mut PaintCtx<'a, 'b>,
}

impl <'a, 'b, 'c> PlotterPaintCtx <'a, 'b, 'c> {
    /// Create an SVG-drawing widget from SvgData.
    ///
    /// The SVG will scale to fit its box constraints.
    pub fn  new (context_ob: &'c mut PaintCtx <'a, 'b> ) -> PlotterPaintCtx < 'a, 'b, 'c > {
        PlotterPaintCtx {
            context: context_ob,
            //fill: FillStrat::default(),
        }
    }


}

impl<'a, 'b, 'c> Renderer for PlotterPaintCtx<'a, 'b, 'c> {
    fn face_points(
        &mut self,
        _s: &[(f64, f64)],
        _x_axis: &axis::ContinuousAxis,
        _y_axis: &axis::ContinuousAxis,
        _face_width: f64,
        _face_height: f64,
        _style: &style::PointStyle,
    ) {
        //let points = draw_face_points(s, x_axis, y_axis, face_width, face_height, style);
        //self.top.append(points);
    }

    fn plot_axis(
        &mut self,
        x_axis: &axis::ContinuousAxis,
        _y_axis: &axis::ContinuousAxis,
        face_width: f64,
        _face_height: f64,
    ) {
        //let xaxgp = draw_x_axis(x_axis, face_width);
        //self.top.append(xaxgp);
        //let yaxgp = draw_y_axis(y_axis, face_height);
        //self.top.append(yaxgp);
        draw_x_axis(&mut self.context, x_axis, face_width )
    }
}



pub fn draw_x_axis(painter: &mut PaintCtx, a: &axis::ContinuousAxis, face_width: f64) {
    let _size = Size::new(200., 200.);
    let size = 5.;
    let circle = Circle::new((size / 2., size / 2.), 7.);

    let border_color = Color::rgb8(50, 50, 50) ;
    painter.stroke(circle, &border_color, 100.);

    let line_thing = Line::new(Point::new(0., 0.), Point::new(face_width, 0.));
    painter.stroke(line_thing, &border_color, 10.);

}

/*

pub fn draw_x_axis(a: &axis::ContinuousAxis, face_width: f64) -> node::element::Group {
    let axis_line = horizontal_line(0.0, 0.0, face_width, "black");

    let mut ticks = node::element::Group::new();
    let mut labels = node::element::Group::new();

    for &tick in a.ticks().iter() {
        let tick_pos = value_to_face_offset(tick, a, face_width);
        let tick_mark = node::element::Line::new()
            .set("x1", tick_pos)
            .set("y1", 0)
            .set("x2", tick_pos)
            .set("y2", 10)
            .set("stroke", "black")
            .set("stroke-width", 1);
        ticks.append(tick_mark);

        let tick_label = node::element::Text::new()
            .set("x", tick_pos)
            .set("y", 20)
            .set("text-anchor", "middle")
            .set("font-size", 12)
            .add(node::Text::new(tick.to_string()));
        labels.append(tick_label);
    }

    let label = node::element::Text::new()
        .set("x", face_width / 2.)
        .set("y", 30)
        .set("text-anchor", "middle")
        .set("font-size", 12)
        .add(node::Text::new(a.get_label()));

    node::element::Group::new()
        .add(ticks)
        .add(axis_line)
        .add(labels)
        .add(label)
}

fn horizontal_line<S>(ypos: f64, xmin: f64, xmax: f64, color: S) -> node::element::Line
where
    S: AsRef<str>,
{
    node::element::Line::new()
        .set("x1", xmin)
        .set("x2", xmax)
        .set("y1", ypos)
        .set("y2", ypos)
        .set("stroke", color.as_ref())
        .set("stroke-width", 1)
}

fn value_to_face_offset(value: f64, axis: &axis::ContinuousAxis, face_size: f64) -> f64 {
    let range = axis.max() - axis.min();
    (face_size * (value - axis.min())) / range
}



fn vertical_line<S>(xpos: f64, ymin: f64, ymax: f64, color: S) -> node::element::Line
where
    S: AsRef<str>,
{
    node::element::Line::new()
        .set("x1", xpos)
        .set("x2", xpos)
        .set("y1", ymin)
        .set("y2", ymax)
        .set("stroke", color.as_ref())
        .set("stroke-width", 1)
}





pub fn draw_y_axis(a: &axis::ContinuousAxis, face_height: f64) -> node::element::Group {
    let axis_line = vertical_line(0.0, 0.0, -face_height, "black");

    let mut ticks = node::element::Group::new();
    let mut labels = node::element::Group::new();

    let y_tick_font_size = 12;

    for &tick in a.ticks().iter() {
        let tick_pos = value_to_face_offset(tick, a, face_height);
        let tick_mark = node::element::Line::new()
            .set("x1", 0)
            .set("y1", -tick_pos)
            .set("x2", -10)
            .set("y2", -tick_pos)
            .set("stroke", "black")
            .set("stroke-width", 1);
        ticks.append(tick_mark);

        let tick_label = node::element::Text::new()
            .set("x", -15)
            .set("y", -tick_pos)
            .set("text-anchor", "end")
            .set("dominant-baseline", "middle")
            .set("font-size", y_tick_font_size)
            .add(node::Text::new(tick.to_string()));
        labels.append(tick_label);
    }

    let y_label_offset = -(face_height / 2.);
    let y_label_font_size = 12;
    let label = node::element::Text::new()
        .set("x", -30)
        .set("y", y_label_offset - f64::from(y_label_font_size))
        .set("text-anchor", "middle")
        .set("font-size", y_label_font_size)
        .set(
            "transform",
            format!("rotate(-90 {} {})", -30, y_label_offset),
        )
        .add(node::Text::new(a.get_label()));

    node::element::Group::new()
        .add(ticks)
        .add(axis_line)
        .add(labels)
        .add(label)
}

pub fn draw_face_points(
    s: &[(f64, f64)],
    x_axis: &axis::ContinuousAxis,
    y_axis: &axis::ContinuousAxis,
    face_width: f64,
    face_height: f64,
    style: &style::PointStyle,
) -> node::element::Group {
    let mut group = node::element::Group::new();

    for &(x, y) in s {
        let x_pos = value_to_face_offset(x, x_axis, face_width);
        let y_pos = -value_to_face_offset(y, y_axis, face_height);
        let radius = f64::from(style.get_size().clone().unwrap_or(5.));
        match style
            .get_marker()
            .clone()
            .unwrap_or(style::PointMarker::Circle)
        {
            style::PointMarker::Circle => {
                group.append(
                    node::element::Circle::new()
                        .set("cx", x_pos)
                        .set("cy", y_pos)
                        .set("r", radius)
                        .set(
                            "fill",
                            style.get_colour().clone().unwrap_or_else(|| "".into()),
                        ),
                );
            }
            style::PointMarker::Square => {
                group.append(
                    node::element::Rectangle::new()
                        .set("x", x_pos - radius)
                        .set("y", y_pos - radius)
                        .set("width", 2. * radius)
                        .set("height", 2. * radius)
                        .set(
                            "fill",
                            style.get_colour().clone().unwrap_or_else(|| "".into()),
                        ),
                );
            }
            style::PointMarker::Cross => {
                let path = node::element::path::Data::new()
                    .move_to((x_pos - radius, y_pos - radius))
                    .line_by((radius * 2., radius * 2.))
                    .move_by((-radius * 2., 0))
                    .line_by((radius * 2., -radius * 2.))
                    .close();
                group.append(
                    node::element::Path::new()
                        .set("fill", "none")
                        .set(
                            "stroke",
                            style.get_colour().clone().unwrap_or_else(|| "".into()),
                        )
                        .set("stroke-width", 2)
                        .set("d", path),
                );
            }
        };
    }

    group
}

*/

/*
        let mut document = Document::new()
            .set("viewBox", (0, 0, width, height))
            .set("xmlns:xlink", "http://www.w3.org/1999/xlink");
*/

/*
use std;

use svg::node;
use svg::Node;

use crate::axis;
use crate::colormap::{ColorMap, ColorMapping};
use crate::grid::GridType;
use crate::repr;
use crate::style;
use crate::utils;
use crate::utils::PairWise;

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
// To use encoder.set()
use image::bmp::BMPEncoder;
use image::png::PNGEncoder;
use image::ColorType;

use base64::encode;

use crate::render::Renderer;

struct svgRender {
    fileobject: thing,
}

impl Renderer for svgRender {
    fn draw_text() {}
    fn draw_line() {}
}




pub fn draw_categorical_x_axis(a: &axis::CategoricalAxis, face_width: f64) -> node::element::Group {
    let axis_line = node::element::Line::new()
        .set("x1", 0)
        .set("y1", 0)
        .set("x2", face_width)
        .set("y2", 0)
        .set("stroke", "black")
        .set("stroke-width", 1);

    let mut ticks = node::element::Group::new();
    let mut labels = node::element::Group::new();

    let space_per_tick = face_width / a.ticks().len() as f64;

    for (i, tick) in a.ticks().iter().enumerate() {
        let tick_pos = (i as f64 * space_per_tick) + (0.5 * space_per_tick);
        let tick_mark = node::element::Line::new()
            .set("x1", tick_pos)
            .set("y1", 0)
            .set("x2", tick_pos)
            .set("y2", 10)
            .set("stroke", "black")
            .set("stroke-width", 1);
        ticks.append(tick_mark);

        let tick_label = node::element::Text::new()
            .set("x", tick_pos)
            .set("y", 20)
            .set("text-anchor", "middle")
            .set("font-size", 12)
            .add(node::Text::new(tick.to_owned()));
        labels.append(tick_label);
    }

    let label = node::element::Text::new()
        .set("x", face_width / 2.)
        .set("y", 30)
        .set("text-anchor", "middle")
        .set("font-size", 12)
        .add(node::Text::new(a.get_label()));

    node::element::Group::new()
        .add(ticks)
        .add(axis_line)
        .add(labels)
        .add(label)
}



pub fn draw_face_bars(
    h: &repr::Histogram,
    x_axis: &axis::ContinuousAxis,
    y_axis: &axis::ContinuousAxis,
    face_width: f64,
    face_height: f64,
    style: &style::BoxStyle,
) -> node::element::Group {
    let mut group = node::element::Group::new();

    for ((&l, &u), &count) in h.bin_bounds.pairwise().zip(h.get_values()) {
        let l_pos = value_to_face_offset(l, x_axis, face_width);
        let u_pos = value_to_face_offset(u, x_axis, face_width);
        let width = u_pos - l_pos;
        let count_scaled = value_to_face_offset(count, y_axis, face_height);
        let rect = node::element::Rectangle::new()
            .set("X", l_pos)
            .set("Y", -count_scaled)
            .set("Width", width)
            .set("Height", count_scaled)
            .set(
                "fill",
                style
                    .get_fill()
                    .clone()
                    .unwrap_or_else(|| "burlywood".into()),
            )
            .set("stroke", "black");
        group.append(rect);
    }

    group
}

pub fn draw_face_imgrid(
    s: &[f64],
    x_axis: &axis::ContinuousAxis,
    y_axis: &axis::ContinuousAxis,
    face_width: f64,
    face_height: f64,
    x_width: u64,
    y_width: u64,
    style: &style::PointStyle,
    cmap: &dyn ColorMapping,
) -> node::element::Group {
    let mut group = node::element::Group::new();
    let mut iii = 0;
    let x_size = value_to_face_offset(1.0, x_axis, face_width);
    let y_size = value_to_face_offset(1.0, y_axis, face_height);

    let mut data = vec![];
    for &z in s {
        let y = iii / x_width;
        let x = iii - y * x_width;

        iii += 1;

        let col = cmap.get_color(z);
        data.push(col.0);
        data.push(col.1);
        data.push(col.2);

        // https://stackoverflow.com/questions/6249664/does-svg-support-embedding-of-bitmap-images
        // https://docs.rs/png/0.11.0/png/ might be a better bet
    }

    //println!("{:?}", data);
    let x_pos = value_to_face_offset(x_width as f64, x_axis, face_width);
    let y_pos = value_to_face_offset(y_width as f64, y_axis, face_height);

    let mut bmp_buffer = Vec::new();
    //BMPEncoder::new(bmp_buffer.by_ref())
    //    .encode(
    //        &data, x_width as u32, y_width as u32,
    //        ColorType::RGB(8),
    //    ).expect("error encoding pixels as PNG");
    PNGEncoder::new(bmp_buffer.by_ref())
        .encode(&data, x_width as u32, y_width as u32, ColorType::RGB(8))
        .expect("error encoding pixels as PNG");

    let b = encode(&bmp_buffer);
    let mut wb = String::from("data:image/png;base64,");
    wb.push_str(&b);

    group.append(
        node::element::Image::new()
            .set("xlink:href", wb.as_str())
            .set("height", y_pos)
            .set("width", x_pos)
            .set("x", 0)
            .set("y", -y_pos),
    );

    group
}

pub fn draw_face_line(
    s: &[(f64, f64)],
    x_axis: &axis::ContinuousAxis,
    y_axis: &axis::ContinuousAxis,
    face_width: f64,
    face_height: f64,
    style: &style::LineStyle,
) -> node::element::Group {
    let mut group = node::element::Group::new();

    let mut d: Vec<node::element::path::Command> = vec![];
    let &(first_x, first_y) = s.first().unwrap();
    let first_x_pos = value_to_face_offset(first_x, x_axis, face_width);
    let first_y_pos = -value_to_face_offset(first_y, y_axis, face_height);
    d.push(node::element::path::Command::Move(
        node::element::path::Position::Absolute,
        (first_x_pos, first_y_pos).into(),
    ));
    for &(x, y) in s {
        let x_pos = value_to_face_offset(x, x_axis, face_width);
        let y_pos = -value_to_face_offset(y, y_axis, face_height);
        d.push(node::element::path::Command::Line(
            node::element::path::Position::Absolute,
            (x_pos, y_pos).into(),
        ));
    }

    let path = node::element::path::Data::from(d);

    group.append(
        node::element::Path::new()
            .set("fill", "none")
            .set(
                "stroke",
                style.get_colour().clone().unwrap_or_else(|| "".into()),
            )
            .set("stroke-width", style.get_width().clone().unwrap_or(2.))
            .set(
                "stroke-linejoin",
                match style
                    .get_linejoin()
                    .clone()
                    .unwrap_or(style::LineJoin::Round)
                {
                    style::LineJoin::Miter => "miter",
                    style::LineJoin::Round => "round",
                },
            )
            .set("d", path),
    );

    group
}

pub fn draw_face_boxplot<L>(
    d: &[f64],
    label: &L,
    x_axis: &axis::CategoricalAxis,
    y_axis: &axis::ContinuousAxis,
    face_width: f64,
    face_height: f64,
    style: &style::BoxStyle,
) -> node::element::Group
where
    L: Into<String>,
    String: std::cmp::PartialEq<L>,
{
    let mut group = node::element::Group::new();

    let tick_index = x_axis.ticks().iter().position(|t| t == label).unwrap(); // TODO this should raise an error
    let space_per_tick = face_width / x_axis.ticks().len() as f64;
    let tick_pos = (tick_index as f64 * space_per_tick) + (0.5 * space_per_tick);

    let box_width = space_per_tick / 2.;

    let (q1, median, q3) = utils::quartiles(d);

    let box_start = -value_to_face_offset(q3, y_axis, face_height);
    let box_end = -value_to_face_offset(q1, y_axis, face_height);

    group.append(
        node::element::Rectangle::new()
            .set("x", tick_pos - (box_width / 2.))
            .set("y", box_start)
            .set("width", box_width)
            .set("height", box_end - box_start)
            .set(
                "fill",
                style
                    .get_fill()
                    .clone()
                    .unwrap_or_else(|| "burlywood".into()),
            )
            .set("stroke", "black"),
    );

    let mid_line = -value_to_face_offset(median, y_axis, face_height);

    group.append(
        node::element::Line::new()
            .set("x1", tick_pos - (box_width / 2.))
            .set("y1", mid_line)
            .set("x2", tick_pos + (box_width / 2.))
            .set("y2", mid_line)
            .set("stroke", "black"),
    );

    let (min, max) = utils::range(d);

    let whisker_bottom = -value_to_face_offset(min, y_axis, face_height);
    let whisker_top = -value_to_face_offset(max, y_axis, face_height);

    group.append(
        node::element::Line::new()
            .set("x1", tick_pos)
            .set("y1", whisker_bottom)
            .set("x2", tick_pos)
            .set("y2", box_end)
            .set("stroke", "black"),
    );

    group.append(
        node::element::Line::new()
            .set("x1", tick_pos)
            .set("y1", whisker_top)
            .set("x2", tick_pos)
            .set("y2", box_start)
            .set("stroke", "black"),
    );

    group
}

pub fn draw_face_barchart<L>(
    d: f64,
    label: &L,
    x_axis: &axis::CategoricalAxis,
    y_axis: &axis::ContinuousAxis,
    face_width: f64,
    face_height: f64,
    style: &style::BoxStyle,
) -> node::element::Group
where
    L: Into<String>,
    String: std::cmp::PartialEq<L>,
{
    let mut group = node::element::Group::new();

    let tick_index = x_axis.ticks().iter().position(|t| t == label).unwrap(); // TODO this should raise an error
    let space_per_tick = face_width / x_axis.ticks().len() as f64;
    let tick_pos = (tick_index as f64 * space_per_tick) + (0.5 * space_per_tick);

    let box_width = space_per_tick / 2.;

    let box_start = -value_to_face_offset(d, y_axis, face_height);
    let box_end = -value_to_face_offset(0.0, y_axis, face_height);

    group.append(
        node::element::Rectangle::new()
            .set("x", tick_pos - (box_width / 2.))
            .set("y", box_start)
            .set("width", box_width)
            .set("height", box_end - box_start)
            .set(
                "fill",
                style
                    .get_fill()
                    .clone()
                    .unwrap_or_else(|| "burlywood".into()),
            )
            .set("stroke", "black"),
    );

    group
}

pub(crate) fn draw_grid(grid: GridType, face_width: f64, face_height: f64) -> node::element::Group {
    match grid {
        GridType::HorizontalOnly(grid) => {
            let (ymin, ymax) = (0f64, face_height);
            let y_step = (ymax - ymin) / f64::from(grid.ny);
            let mut lines = node::element::Group::new();

            for iy in 0..=grid.ny {
                let y = f64::from(iy) * y_step + ymin;
                let line = horizontal_line(-y, 0.0, face_width, grid.color.as_str());
                lines = lines.add(line);
            }

            lines
        }
        GridType::Both(grid) => {
            let (xmin, xmax) = (0f64, face_width);
            let (ymin, ymax) = (0f64, face_height);

            let x_step = (xmax - xmin) / f64::from(grid.nx);
            let y_step = (ymax - ymin) / f64::from(grid.ny);

            let mut lines = node::element::Group::new();

            for iy in 0..=grid.ny {
                let y = f64::from(iy) * y_step + ymin;
                let line = horizontal_line(-y, 0.0, face_width, grid.color.as_str());
                lines = lines.add(line);
            }

            for ix in 0..=grid.nx {
                let x = f64::from(ix) * x_step + xmin;
                let line = vertical_line(x, 0.0, -face_height, grid.color.as_str());
                lines = lines.add(line);
            }

            lines
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_to_face_offset() {
        let axis = axis::ContinuousAxis::new(-2., 5., 6);
        assert_eq!(value_to_face_offset(-2.0, &axis, 14.0), 0.0);
        assert_eq!(value_to_face_offset(5.0, &axis, 14.0), 14.0);
        assert_eq!(value_to_face_offset(0.0, &axis, 14.0), 4.0);
        assert_eq!(value_to_face_offset(-4.0, &axis, 14.0), -4.0);
        assert_eq!(value_to_face_offset(7.0, &axis, 14.0), 18.0);
    }
}
*/

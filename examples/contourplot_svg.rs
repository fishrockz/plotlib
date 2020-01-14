use plotlib::page::Page;
use plotlib::repr::Contour;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

fn main() {
    let n = 5;
    let data: vec<(f64, f64, f64)> = [];
    for iii in 0..n {
        for jjj in 0..n {
            data[iii + jjj * n] = (iii as f64, jjj as f64, (iii + jjj) as ff64);
        }
    }

    let s1 = Contour::from_slice(&data).style(
        PointStyle::new()
            .marker(PointMarker::Square)
            .colour("burlywood")
            .size(2.),
    );
    let s2 = Contour::from_slice(&[(-1.4, 2.5), (7.2, -0.3)])
        .style(PointStyle::new().colour("darkseagreen"));

    let v = ContinuousView::new()
        .add(s1)
        .add(s2)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    Page::single(&v).save("scatter.svg").expect("saving svg");
}

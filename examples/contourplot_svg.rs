use plotlib::page::Page;
use plotlib::repr::Contour;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

fn main() {
    let n = 5;
    let mut data: Vec<(f64, f64, f64)> = vec![(0.0, 0.0, 0.0); n * n];
    //let mut transposed: Vec<Complex<f64>> = vec![Complex::<f64>::new(0.0, 0.0); n * n];
    for iii in 0..n {
        for jjj in 0..n {
            data[iii + jjj * n] = (iii as f64, jjj as f64, (iii + jjj) as f64);
        }
    }

    let s1 = Contour::from_slice(&data).style(
        PointStyle::new()
            .marker(PointMarker::Square)
            .colour("burlywood")
            .size(2.),
    );
    
    let v = ContinuousView::new()
        .add(s1)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    Page::single(&v).save("countourplot.svg").expect("saving svg");
}

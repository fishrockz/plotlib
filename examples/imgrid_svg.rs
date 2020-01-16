use plotlib::page::Page;
use plotlib::repr::Imgrid;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

fn main() {
    let n = 5;
    let mut data: Vec<f64> = vec![0.0; n * n];
    //let mut transposed: Vec<Complex<f64>> = vec![Complex::<f64>::new(0.0, 0.0); n * n];
    for iii in 0..n {
        for jjj in 0..n {
            data[iii + jjj * n] = (iii + jjj) as f64 / 8.0;
        }
    }

    //println!("{:?}", data);

    let s1 = Imgrid::from_slice(&data, n as u64, n as u64).style(
        PointStyle::new()
            .marker(PointMarker::Square)
            .colour("burlywood")
            .size(2.),
    );
    
    let v = ContinuousView::new()
        .add(s1)
        .x_range(0., 5.)
        .y_range(0., 5.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    Page::single(&v).save("imgridplot.svg").expect("saving svg");
}

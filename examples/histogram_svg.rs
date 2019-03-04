use plotlib::histogram::{Histogram, Style};
use plotlib::page::Page;
use plotlib::style::Bar;
use plotlib::view::ContinuousView;

fn main() {
    let data = [0.3, 0.5, 6.4, 5.3, 3.6, 3.6, 3.5, 7.5, 4.0];
    let h = Histogram::from_slice(&data, plotlib::histogram::Bins::Count(10))
        .style(Style::new().fill("burlywood"));

    let v = ContinuousView::new().add(&h);

    Page::single(&v).save("histogram.svg").expect("saving svg");
}

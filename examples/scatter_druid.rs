use plotlib::page::Page;
use plotlib::repr::Scatter;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

fn main() {
    let data = [
        (-3.0, 2.3),
        (-1.6, 5.3),
        (0.3, 0.7),
        (4.3, -1.4),
        (6.4, 4.3),
        (8.5, 3.7),
    ];
    let s1 = Scatter::from_slice(&data).style(
        PointStyle::new()
            .marker(PointMarker::Square)
            .colour("burlywood")
            .size(2.),
    );
    let s2 = Scatter::from_slice(&[(-1.4, 2.5), (7.2, -0.3)])
        .style(PointStyle::new().colour("darkseagreen"));

    let v = ContinuousView::new()
        .add(s1)
        .add(s2)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The response of something");



    let this_page =Page::single(&v);

    use druid::{
        widget::{FillStrat, Flex, Image, ImageData, WidgetExt},
        AppLauncher, Widget, WindowDesc,
    };

    fn ui_builder() -> impl Widget<u32> {
        DruidPageWidget::new(this_page).fill_mode(FillStrat::FitWidth)
    };

    let main_window = WindowDesc::new(ui_builder);
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}
}

use plotlib::druid_widget::{DruidPageWidget};

fn main() {

    use druid::{
        AppLauncher, Widget, WindowDesc,
    };

    fn ui_builder() -> impl Widget<u32> {
        
        DruidPageWidget::new()
    };

    let main_window = WindowDesc::new(ui_builder);
    let this_page = 0_u32;
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(this_page)
        .expect("launch failed");
}

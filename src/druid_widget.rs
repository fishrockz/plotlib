use std::marker::PhantomData;
use std::sync::Arc;

use druid::{
    Affine, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle, LifeCycleCtx,
    PaintCtx, Rect, RenderContext, Size, UpdateCtx, Widget,
};

use crate::druid_render::PlotterPaintCtx;
use crate::page::Page;
use crate::page::Plotpage;

pub struct DruidPageWidget {}

use crate::repr::Scatter;
use crate::style::{PointMarker, PointStyle};
use crate::view::ContinuousView;

#[derive(Clone, Data, Lens)]
pub struct DataPage {
    page: Arc<Page>,
}

impl DataPage {
    pub fn new(data_page: Page) -> Self {
        DataPage {
            page: Arc::new(data_page),
        }
    }
    pub fn plot(&self, paint_ctx: &mut PlotterPaintCtx) {
        self.page.plot(paint_ctx);
    }

    pub fn set_size(&mut self, size: &Size) {
//        std::sync::Arc::make_mut(&mut self.page)
//            .set_dimensions((size.width as u32, size.height as u32));
    }
}

impl DruidPageWidget {
    /// Plot pages of graphs
    pub fn new() -> Self {
        DruidPageWidget {}
    }

    fn get_size(&self) -> Size {
        Size::new(200., 200.)
    }

    //    fn get_offset(&self, context_ob: &mut PaintCtx) -> Affine {
    fn get_offset(&self, size: Size) -> Affine {
        //        let size = context_ob.region().to_rect().size();

        let x_margin = 90; // should actually depend on y-axis label font size
        let y_margin = 60;
        let x_offset = 0.6 * f64::from(x_margin);
        let y_offset = 0.6 * f64::from(y_margin);

        let height = size.height;

        let offset = (x_offset, f64::from(height) - y_offset);

        Affine::new([1.0, 0.0, 0.0, 1.0, offset.0, offset.1])
    }
    /*
    /// A builder-style method for specifying the fill strategy.
    pub fn fill_mode(mut self, mode: FillStrat) -> Self {
        self.fill = mode;
        self
    }

    /// Modify the widget's `FillStrat`.
    pub fn set_fill(&mut self, newfil: FillStrat) {
        self.fill = newfil;
    }
    */
}

impl Widget<DataPage> for DruidPageWidget {
    fn event(&mut self, ctx: &mut EventCtx, _event: &Event, data: &mut DataPage, _env: &Env) {
        let size = ctx.size();
        data.set_size(&size);
        //      data
        //        .set_dimensions((size.width as u32, size.height as u32));
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &DataPage,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &DataPage, _data: &DataPage, _env: &Env) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &DataPage,
        _env: &Env,
    ) -> Size {
        bc.debug_check("DruidPageWidget");

        if bc.is_width_bounded() {
            bc.max()
        } else {
            bc.constrain(Size::new(100., 100.))
        }
    }
    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &DataPage, _env: &Env) {
        //let offset_matrix = self.fill.affine_to_fill(paint_ctx.size(), self.get_size());

        // TODO is this needed?
        let clip_rect = Rect::ZERO.with_size(paint_ctx.size());
        paint_ctx.clip(clip_rect);

        //let mut renderthing = PlotterPaintCtx {
        //    context: paint_ctx,
        //    dimensions: (300, 400),
        //};

        //        paint_ctx
        //          .with_save(|ctx| {
        let size = paint_ctx.region().to_rect().size();
        paint_ctx.transform(self.get_offset(size));
        let mut renderthing = PlotterPaintCtx::new(paint_ctx);
        data.plot(&mut renderthing);
        //            Ok(())
        //      }).unwrap();

        //data.page.plot(&mut renderthing);
    }
}


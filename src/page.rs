/*!
The `page` module provides structures for laying out and rendering multiple views.
*/

use std::ffi::OsStr;
use std::path::Path;

use crate::errors::Result;
use crate::render::Renderer;
use crate::view::View;

use crate::svg_render::Plotter;

/**
A single page page laying out the views in a grid
*/
pub struct Page {
    views: Vec<Box<dyn View>>,
    num_views: u32,
    dimensions: (u32, u32),
}

impl Page {
    /**
    Creates an empty page container for plots to be added to
    */
    pub fn empty() -> Self {
        Page {
            views: Vec::new(),
            num_views: 0,
            dimensions: (600, 400),
        }
    }

    /**
    Creates a plot containing a single view
    */
    pub fn single(view: Box<dyn View>) -> Self {
        Page::empty().add_plot(view)
    }

    /// Set the dimensions of the plot.
    pub fn dimensions(mut self, x: u32, y: u32) -> Self {
        self.dimensions = (x, y);
        self
    }

    /// Add a view to the plot
    pub fn add_plot(mut self, view: Box<dyn View>) -> Self {
        self.views.push(view);
        self.num_views += 1;
        self
    }

    /**
    Render the plot to an svg document
    */
    pub fn plot(&self, plotTo: &mut dyn Renderer) {
        let (width, height) = self.dimensions;

        let x_margin = 90; // should actually depend on y-axis label font size
        let y_margin = 60;
        let _x_offset = 0.6 * f64::from(x_margin);
        let _y_offset = 0.6 * f64::from(y_margin);

        // TODO put multiple views in correct places
        //for view in self.views {
            let _view_group = self.views[0].plot(
                plotTo,
                f64::from(width - x_margin),
                f64::from(height - y_margin),
            );
            /*.to_svg(f64::from(width - x_margin), f64::from(height - y_margin))?
                .set(
                    "transform",
                    format!("translate({}, {})", x_offset, f64::from(height) - y_offset),
                );
            document.append(view_group);*/
        //}
        //Ok(document)
    }

    /*
    Save the plot to a file.

    The type of file will be based on the file extension.
    */

    pub fn save<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        match path.as_ref().extension().and_then(OsStr::to_str) {
            Some("svg") => {
                let mut svgpainter = Plotter::new();
                self.plot(&mut svgpainter);
                svgpainter.save(path);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

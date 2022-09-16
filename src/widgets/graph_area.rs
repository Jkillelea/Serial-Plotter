// mod graph_area

use gdk::RGBA;
use gtk::{prelude::*, DrawingArea, Orientation};
use relm::{DrawHandler, Relm, Widget};
use relm_derive::{widget, Msg};

#[derive(Msg, Debug)]
pub enum GraphAreaMsg {
    Draw,
    Data(usize, f32),
    SetColor(gdk::RGBA),
}

pub struct GraphAreaModel {
    data: Vec<f32>,
    color: gdk::RGBA,
    line_width: f64,
    draw_handler: DrawHandler<DrawingArea>,
}

#[widget]
impl Widget for GraphArea {
    // Instantiate the model
    fn model(_relm: &Relm<Self>, _: ()) -> GraphAreaModel {
        GraphAreaModel {
            data: Vec::with_capacity(1024),
            color: gdk::RGBA::WHITE, // red, green, blue
            line_width: 1.0,
            draw_handler: DrawHandler::new().expect("Could not create draw handler"),
        }
    }

    // Attach the draw handler to the drawing area
    fn init_view(&mut self) {
        self.model.draw_handler.init(&self.widgets.drawing_area);
    }

    // Recieve and handle messages
    fn update(&mut self, msg: GraphAreaMsg) {
        match msg {
            GraphAreaMsg::Data(idx, d) => self.model.data.push(d),
            GraphAreaMsg::SetColor(c) => self.model.color = c,
            GraphAreaMsg::Draw => self.draw(),
        };
    }

    fn draw(&mut self) {
        use std::f64::consts::PI;

        // Set up the drawing area context
        let alloc = self.widgets.drawing_area.allocation();
        let w     = alloc.width()  as f64;
        let h     = alloc.height() as f64;
        let x0    = alloc.x()      as f64;
        let y0    = alloc.y()      as f64;
        let xmax  = x0 + w;
        let ymax  = y0 + h;

        if let Ok(context) = self.model.draw_handler.get_context() {
            context.set_source_rgb(self.model.color.red(),
                                    self.model.color.green(),
                                    self.model.color.blue());

            // Plot the points
            for (x, &d) in self.model.data.iter().enumerate() {
                context.line_to(x as f64, d as f64)
            }

            if let Err(e) = context.stroke() {
                eprintln!("Drawing error: {}", e);
            }
        }
    }

    view! {
        #[name = "drawing_area"]
        gtk::DrawingArea {
            expand: true,
            draw(_, _) => (GraphAreaMsg::Draw, Inhibit(false)),
        }
    }
}

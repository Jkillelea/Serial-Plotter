// mod graph_area

use gtk::{prelude::*, DrawingArea, Orientation};
use relm::{DrawHandler, Relm, Widget};
use relm_derive::{widget, Msg};

#[derive(Msg, Debug)]
pub enum GraphAreaMsg {
    Draw,
    Data(usize, f32),
}

pub struct GraphAreaModel {
    data: Vec<f32>,
    color: (f64, f64, f64), // r, g, b
    line_width: f64,
    draw_handler: DrawHandler<DrawingArea>,
}

#[widget]
impl Widget for GraphArea {
    // Instantiate the model
    fn model(_relm: &Relm<Self>, _: ()) -> GraphAreaModel {
        GraphAreaModel {
            data: Vec::with_capacity(1024),
            color: (1.0, 1.0, 1.0), // red, green, blue
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
            GraphAreaMsg::Draw         => self.draw()
        };
    }

    fn draw(&mut self) {
        use std::f64::consts::PI;

        let alloc = self.widgets.drawing_area.allocation();
        let w     = alloc.width  as f64;
        let h     = alloc.height as f64;
        let x0    = 0.0; // TODO: why is alloc.x not 0?
        let y0    = 0.0; // TODO: why is alloc.y not 0?
        let xmax  = x0 + w;
        let ymax  = y0 + h;

        let context = self.model.draw_handler.get_context().unwrap();

        context.set_source_rgb(self.model.color.0, self.model.color.1, self.model.color.2);
        context.move_to(x0, y0 + 0.5 * h);

        let points = 50;
        let phase = 0.0;
        for i in 0 .. points {

            // Draw a sine
            let x = x0 + w * (i as f64) / points as f64;
            let y = y0 + 0.5 * h + 0.4 * h * (2.0 * PI * x / xmax + phase).sin();

            context.line_to(x, y);
        }
        context.stroke().unwrap();
    }

    view! {
        gtk::Box {
            orientation: Orientation::Horizontal,

            #[name = "drawing_area"]
            gtk::DrawingArea {
                expand: true,
                draw(_, _) => (GraphAreaMsg::Draw, Inhibit(false)),
            }
        },
    }
}

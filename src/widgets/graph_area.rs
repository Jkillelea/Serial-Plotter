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
    data: Vec<Vec<f32>>,
    color: (f64, f64, f64), // r, g, b
    draw_handler: DrawHandler<DrawingArea>,
}

#[widget]
impl Widget for GraphArea {
    // Instantiate the model
    fn model(_relm: &Relm<Self>, _: ()) -> GraphAreaModel {
        GraphAreaModel {
            data: Vec::from(Vec::with_capacity(1024)),
            color: (1.0, 1.0, 1.0),
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
            GraphAreaMsg::Data(idx, d) => self.model.data[idx].push(d),
            GraphAreaMsg::Draw => self.draw()
        };
    }

    fn draw(&mut self) {
        let context = self.model.draw_handler.get_context().unwrap();
        context.set_source_rgb(self.model.color.0, self.model.color.1, self.model.color.2);

        // let size = self.widgets.drawing_area.size();

        // println!("GraphArea::draw()");
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

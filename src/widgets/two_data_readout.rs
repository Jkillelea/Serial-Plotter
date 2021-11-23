use gtk::{prelude::*, Orientation};
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};

#[derive(Msg, Debug)]
pub enum TwoDataReadoutMsg {
    Data((f64, f64)),
}

pub struct TwoDataReadoutModel {
    data: (f64, f64),
}

#[widget]
impl Widget for TwoDataReadout {
    fn model(_relm: &Relm<Self>, _: ()) -> TwoDataReadoutModel {
        TwoDataReadoutModel { data: (0.0, 0.0) }
    }

    fn update(&mut self, msg: TwoDataReadoutMsg) {
        match msg {
            TwoDataReadoutMsg::Data(data) => {
                self.model.data = data;
            }

            _ => eprintln!("Unkown Message: {:#?}", msg)
        };
    }

    view! {
        gtk::Box {
            orientation: Orientation::Horizontal,

            gtk::Label {
                text: &self.model.data.0.to_string(),
                hexpand: true,
            },

            gtk::Label {
                text: &self.model.data.1.to_string(),
                hexpand: true,
            },
        }
    }
}

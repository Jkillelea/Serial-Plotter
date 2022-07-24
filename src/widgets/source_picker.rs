// mod source_picker

use gtk::{prelude::*, ColorButton, TextView};
use crate::Widget;
use relm::Relm;
use relm_derive::{widget, Msg};

#[derive(Msg, Debug)]
pub enum SourcePickerMessage {
    SetColor(gdk::RGBA),
}

pub struct SourcePickerModel {
    number: u32,
    source: String,
    color: gdk::RGBA,
}

#[widget]
impl Widget for SourcePicker {

    fn model(relm: &Relm<Self>, _: ()) -> SourcePickerModel {
        SourcePickerModel {
            number: 0,
            source: "".into(),
            color: gdk::RGBA::WHITE,
        }
    }

    fn update(&mut self, msg: SourcePickerMessage) {
    }

    view! {
        #[name = "color_button"]
        gtk::ColorButton {
        },

        gtk::TextView {
        }
    }
}

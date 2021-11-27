#![allow(unused)]

use gdk::EventMask;
use gtk::{prelude::*, DrawingArea, Inhibit, Orientation::*};
use relm::{DrawHandler, Relm, Widget};
use relm_derive::{widget, Msg};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod widgets;
use widgets::*;

/// Communication messages. Used by both the Relm event stream and inter-thread communication
#[derive(Msg, Debug)]
pub enum Msg {
    Quit,
    DrawGraph,
    MoveCursor((f64, f64)),
}

/// GUI model
pub struct Model {
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
        }
    }

    fn init_view(&mut self) {
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Quit => gtk::main_quit(),
            Msg::DrawGraph => self.components.graph_area.emit(GraphAreaMsg::Draw),
            Msg::MoveCursor(pos) => self.components.readout.emit(TwoDataReadoutMsg::Data(pos)),
            _ => eprintln!("Unkown Message: {:#?}", msg),
        };
    }

    view! {
        #[name = "window"]
        gtk::Window {
            default_width:  800,
            default_height: 400,

            gtk::Box {
                orientation: Vertical,

                #[name = "graph_area"]
                GraphArea {
                    expand: true,
                    draw(_, _) => (Msg::DrawGraph, Inhibit(false)), // On GTK Draw Event
                },

                #[name = "readout"]
                TwoDataReadout,

                gtk::Button {
                    clicked => Msg::Quit,
                    label: "Quit",
                },
            },

            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}

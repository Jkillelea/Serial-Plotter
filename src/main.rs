#![allow(unused)]

use gdk::EventMask;
use gtk::{prelude::*, DrawingArea, Inhibit, Orientation::*, *};
use relm::{Channel, DrawHandler, Relm, Widget};
use relm_derive::{widget, Msg};
use serial;
use std::io::prelude::*;
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
    AddSource,
    Data(f64),
    MoveCursor((f64, f64)),
    Color(gdk::RGBA),
}

/// GUI model
pub struct Model {
    data_channel: Channel<f64>,
}

#[widget]
impl Widget for Win {

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        let stream = relm.stream().clone();
        // Create a channel to be able to send a message from another thread.
        let (channel, sender) = Channel::new(move |num| {
            // This closure is executed whenever a message is received from the sender.
            // We send a message to the current widget.
            stream.emit(Msg::Data(num));
        });

        thread::spawn(move || {
            let mut port = serial::open("/dev/ttyUSB0").unwrap();

            loop {
                thread::sleep(Duration::from_millis(200));
                // Send a message from the other thread.
                sender.send(42.0).expect("send message");
            }
        });

        Model {
            data_channel: channel,
        }
    }

    // fn init_view(&mut self) {
    // }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::DrawGraph       => self.components.graph_area.emit(GraphAreaMsg::Draw),
            Msg::MoveCursor(pos) => self.components.readout.emit(TwoDataReadoutMsg::Data(pos)),
            Msg::Color(c)        => self.components.graph_area.emit(GraphAreaMsg::SetColor(c)),
            Msg::Data(d)         => self.components.graph_area.emit(GraphAreaMsg::Data(0, d as f32)),
            Msg::AddSource       => {
                println!("Add source!");
            },
            Msg::Quit            => gtk::main_quit(),
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

                #[name = "color_button"]
                gtk::ColorButton {
                    color_set(btn) => Msg::Color(btn.rgba().clone()),
                },

                gtk::Box{
                    SourcePicker {
                    },
                },

                #[name = "add_source"]
                gtk::Button {
                    clicked => Msg::AddSource,
                    label: "+",
                },

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

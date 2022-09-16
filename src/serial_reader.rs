// mod serial reader

use std::io::{self, prelude::*};
use std::path::Path;
use serial::*;
use relm::{Channel, DrawHandler, Relm, Widget, StreamHandle};
use relm_derive::{widget, Msg};
use gdk::{RGBA, EventMask};
use gtk::{prelude::*, TextBuffer, Inhibit, Orientation::*};

#[derive(Msg, Debug)]
pub enum SerialReaderMsg {
    Open,
    Close,
    Color(gdk::RGBA),
}

/// A stupid reader of `serial::SystemPort`
pub struct SerialReaderModel {
    port: Option<SystemPort>,
    path: TextBuffer,
    baud: usize,
}

#[widget]
impl Widget for SerialReader {
    fn model() -> SerialReaderModel {
        SerialReaderModel {
            port: None,
            path: TextBuffer::builder()
                .text("Enter path here")
                .build(),
            baud: 9600,
        }
    }

    fn update(&mut self, msg: SerialReaderMsg) {
        match msg {
            SerialReaderMsg::Open => {
                self.model.port = match SystemPort::open(&Path::new(&self.model.path.to_string())) {
                    Ok(p) => Some(p),
                    Err(e) => {eprintln!("{}", e); None}
                }
            },
            SerialReaderMsg::Close => {
                if self.model.port.is_some() {
                    self.model.port = None;
                }
            },

            SerialReaderMsg::Color(c) => {
            }
        }
    }

    view! {
        gtk::Box {
            orientation: Horizontal,
            // expand: false,

            #[name = "port_path"]
            gtk::TextView {
                buffer: Some(&self.model.path),
                expand: true,
            },

            #[name = "color_button"]
            gtk::ColorButton {
                color_set(btn) => SerialReaderMsg::Color(btn.rgba().clone()),
            },

            gtk::Button {
                clicked => SerialReaderMsg::Open,
                label: "Open",
            },

            gtk::Button {
                clicked => SerialReaderMsg::Close,
                label: "Close",
            }
        },
    }
}


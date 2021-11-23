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
    Draw,
    DrawGraph,
    MoveCursor((f64, f64)),
}

/// GUI model
pub struct Model {
    // Pipe for communicating with background thread(s)
    thread_channels: Vec<mpsc::Sender<Msg>>,

    draw_handler: DrawHandler<DrawingArea>,
    cursor_pos: (f64, f64),
}

/// Background thread data
struct ThreadData {
    rx_pipe: mpsc::Receiver<Msg>,
    relm_sender: relm::Sender<Msg>,
    quit: bool,
}

impl ThreadData {
    fn new(rx_pipe: mpsc::Receiver<Msg>, relm_sender: relm::Sender<Msg>) -> ThreadData {
        ThreadData {
            rx_pipe,
            relm_sender,
            quit: false,
        }
    }

    fn run_loop(&mut self) {
        while !self.quit {
            // recieve new data
            if let Ok(msg) = self.rx_pipe.recv_timeout(Duration::from_millis(1000)) {
                match msg {
                    Msg::Quit => {
                        // eprintln!("Thread quitting");
                        self.quit = true;
                    }

                    _ => eprintln!("Unhandled message {:?}", msg),
                }
            }
        }
    }
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        let mut thread_channels = Vec::new();

        // The Channel for sending from the main thread to the timer thread.
        // Used for communicating start/pause/quit.
        let (thread_tx, thread_rx) = mpsc::channel();

        // This is the stream for sending messages to the own widget.
        let stream = relm.stream().clone();

        // The Channel for sending from the timer thread to the main thread.
        // This is just a thread-safe wrapper over the stream, more or less
        let (_channel, relm_sender) = relm::Channel::new(move |msg| {
            stream.emit(msg);
        });

        // Background thread to send messages to the main GUI thread
        thread::spawn(move || {
            let mut task_data = ThreadData::new(thread_rx, relm_sender);
            task_data.run_loop();
        });

        thread_channels.push(thread_tx);

        Model {
            thread_channels,

            draw_handler: DrawHandler::new().expect("Could not create draw handler"),
            cursor_pos: (0.0, 0.0),
        }
    }

    fn init_view(&mut self) {
        // self.model.draw_handler.init(&self.widgets.drawing_area);
        // self.widgets
        //     .drawing_area
        //     .add_events(EventMask::POINTER_MOTION_MASK); // Unmask the pointer motion event
        self.widgets.window.resize(400, 400);
    }

    fn update(&mut self, msg: Msg) {
        match msg {

            // tell all the threads to quit
            Msg::Quit => {
                self.model
                    .thread_channels
                    .iter()
                    .for_each(|chan| chan.send(Msg::Quit).unwrap());
                gtk::main_quit();
            }

            Msg::Draw => {
                let context = self.model.draw_handler.get_context().unwrap();
                context.set_source_rgb(1.0, 1.0, 1.0);
                context.paint().unwrap();

                context.set_source_rgb(0.0, 0.0, 0.0);
                context.move_to(self.model.cursor_pos.0, self.model.cursor_pos.1);
                context.arc(
                    self.model.cursor_pos.0,
                    self.model.cursor_pos.1,
                    15.0,
                    0.0,
                    2.0 * std::f64::consts::PI,
                );
                context.fill().unwrap();
            }

            Msg::DrawGraph => self.components.graph_area.emit(GraphAreaMsg::Draw),

            Msg::MoveCursor(pos) => {
                self.model.cursor_pos = pos;
                self.components.readout.emit(TwoDataReadoutMsg::Data(pos));
            }

            _ => eprintln!("Unkown Message: {:#?}", msg)
        };
    }

    view! {
        #[name = "window"]
        gtk::Window {
            default_width:  800,
            default_height: 400,

            gtk::Box {
                orientation: Vertical,

                // #[name = "drawing_area"]
                // gtk::DrawingArea {
                //     expand: true,
                //     draw(_, _) => (Msg::Draw, Inhibit(false)), // On GTK Draw Event
                //     motion_notify_event(_, event) => (Msg::MoveCursor(event.position()), Inhibit(false))
                // },

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

use gdk::EventMask;
use gtk::prelude::*;
use gtk::Inhibit;
use gtk::{DrawingArea, Orientation::*};
use relm::{DrawHandler, Relm, Widget};
use relm_derive::{widget, Msg};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Communication messages. Used by both the Relm event stream and inter-thread communication
#[derive(Msg, Debug)]
pub enum Msg {
    Quit,
    Adjust(f32),
    Data(f32),
    UpdateDrawBuffer,
    MoveCursor((f64, f64)),
}

/// GUI model
pub struct Model {
    // Pipe for communicating with background thread(s)
    thread_channels: Vec<mpsc::Sender<Msg>>,

    draw_handler: DrawHandler<DrawingArea>,
    cursor_pos: (f64, f64),

    // data: Vec<f32>,
    data: f32,
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
                        eprintln!("Thread quitting");
                        self.quit = true;
                    }

                    _ => eprintln!("Unhandled message {:?}", msg),
                }
            }

            // Send out data
            self.relm_sender.send(Msg::Adjust(0.1)).unwrap();
        }
    }
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        let mut thread_channels = Vec::new();

        // // The Channel for sending from the main thread to the timer thread.
        // // Used for communicating start/pause/quit.
        // let (thread_tx, thread_rx) = mpsc::channel();

        // // This is the stream for sending messages to the own widget.
        // let stream = relm.stream().clone();

        // // The Channel for sending from the timer thread to the main thread.
        // // This is just a thread-safe wrapper over the stream, more or less
        // let (_channel, relm_sender) = relm::Channel::new(move |msg| {
        //     stream.emit(msg);
        // });

        // // Background thread to send messages to the main GUI thread
        // thread::spawn(move || {
        //     let mut task_data = ThreadData::new(thread_rx, relm_sender);
        //     task_data.run_loop();
        // });

        // thread_channels.push(thread_tx);

        Model {
            thread_channels,

            draw_handler: DrawHandler::new().expect("Could not create draw handler"),
            cursor_pos: (0.0, 0.0),

            // data: Vec::with_capacity(1024),
            data: 0.0,
        }
    }

    fn init_view(&mut self) {
        self.model.draw_handler.init(&self.widgets.drawing_area);
        self.widgets
            .drawing_area
            .add_events(EventMask::POINTER_MOTION_MASK); // Unmask the pointer motion event
        self.widgets.window.resize(400, 400);
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Quit => {
                // tell all the threads to quit
                for channel in &self.model.thread_channels {
                    channel.send(Msg::Quit).unwrap();
                }

                gtk::main_quit()
            }

            Msg::Data(d) => {
                // self.model.data.push(d);
                self.model.data = d;
            }

            Msg::Adjust(amount) => {
                self.model.data += amount;
            }

            Msg::UpdateDrawBuffer => {
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

            Msg::MoveCursor(pos) => {
                self.model.cursor_pos = pos;
            }

            _ => {
                eprintln!("Unhandled message {:#?}", msg);
            }
        }
    }

    view! {
        #[name = "window"]
        gtk::Window {
            gtk::Box {
                orientation: Vertical,

                #[name = "drawing_area"]
                gtk::DrawingArea {
                    expand: true,

                    // On GTK Draw Event
                    draw(_, _) => (Msg::UpdateDrawBuffer, Inhibit(false)),
                    motion_notify_event(_, event) => (Msg::MoveCursor(event.position()), Inhibit(false))
                },

                gtk::Box {
                    orientation: Horizontal,

                    gtk::Label {
                        text: &self.model.cursor_pos.0.to_string(),
                        hexpand: true,
                    },
                    gtk::Label {
                        text: &self.model.cursor_pos.1.to_string(),
                        hexpand: true,
                    },
                },

                gtk::Button {
                    clicked => Msg::Quit,
                    label: "Quit",
                },
            },

            // Use a tuple when you want to both send a message and return a value to
            // the GTK+ callback.
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
            // key_press_event(_, event) => (Msg::KeyPress(event.clone()), Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}

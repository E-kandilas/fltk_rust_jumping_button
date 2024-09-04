
use fltk::{prelude::*, button::Button, enums::{Color, FrameType}, app, *};
use fltk::enums::Event;
use frame::Frame;
use rand::Rng;

// Define the Message type
#[derive(Debug, Clone, Copy)]
enum Message {
    Spaz,
}

fn main() {
    let app = app::App::default().with_scheme(fltk::app::Scheme::Gleam);

    let mut win = fltk::window::Window::new(100, 100, 400, 300, "ClickyButton");
    let mut frame = fltk::frame::Frame::new(0, 0, 400, 300, "");
    frame.set_color(fltk::enums::Color::White);

    let (sender, reciever) = app::channel::<Message>();

    let mut frame = Frame::new(0, 0, 400, 300, "");
    frame.set_color(fltk::enums::Color::White);
    let mut but = Button::new(160, 200, 80, 40, "Click me!");
    but.set_color(Color::Cyan);
    but.set_label_color(Color::Red);
    but.set_frame(FrameType::FlatBox);
    but.set_label_size(20);

    but.handle(move |b, evt| match evt {
        Event::Enter => {
          b.set_color(Color::XtermBgYellow);
          b.redraw();
          return true;
        }
        Event::Leave => {
          b.set_color(Color::Cyan);
          b.redraw();
          return true;
        }
        Event::Push => {
            b.emit(sender, Message::Spaz);
            return true;
        }
        _ => {
          return false;
        },
    });

    win.end();
    win.show();

    while app.wait() {
      if let Some(msg) = reciever.recv() {
        match msg {
          Message::Spaz => {
            let rand_x = rand::thread_rng().gen_range(0..(400 - 80));
            let rand_y = rand::thread_rng().gen_range(0..100);
            but.resize(rand_x, rand_y, 80, 40);
            win.redraw();
          }
        }
      }
    }

    app.run().unwrap();
}

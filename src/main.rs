use fltk::{prelude::*, button::Button, enums::{Color, FrameType}, app, *};
use fltk::enums::Event;
use rand::Rng;

// Define the Message type
#[derive(Debug, Clone, Copy)]
enum Message {
  Spaz,
}

// Constants for centralized variables
const WIN_WIDTH: i32 = 600;
const WIN_HEIGHT: i32 = 400;
const BUTTON_WIDTH: i32 = 100;
const BUTTON_HEIGHT: i32 = 50;
const BUTTON_LABEL: &str = "Click me!";

fn main() {
  let app = app::App::default().with_scheme(fltk::app::Scheme::Gleam);

  let mut win = fltk::window::Window::new(100, 100, WIN_WIDTH, WIN_HEIGHT, "ClickyButton");
  let mut frame = fltk::frame::Frame::new(0, 0, WIN_WIDTH, WIN_HEIGHT, "");
  frame.set_color(fltk::enums::Color::White);

  let (sender, receiver) = app::channel::<Message>();

  let mut but = Button::new((WIN_WIDTH - BUTTON_WIDTH) / 2, WIN_HEIGHT - BUTTON_HEIGHT - 20, BUTTON_WIDTH, BUTTON_HEIGHT, BUTTON_LABEL);
  but.set_color(Color::Cyan);
  but.set_label_color(Color::Red);
  but.set_frame(FrameType::RoundUpBox);
  but.set_label_size(20);
  but.set_selection_color(Color::Green);

  let mut counter = 0;
  let mut counter_frame = fltk::frame::Frame::new((WIN_WIDTH - 200) / 2, 10, 200, 40, "Num Clicks:");
  counter_frame.set_label_size(24);
  counter_frame.set_label_color(Color::Blue);
  counter_frame.set_frame(FrameType::FlatBox);
  counter_frame.set_color(Color::White);

  but.handle(move |b, evt| match evt {
    Event::Enter => {
      b.set_color(Color::XtermBgYellow);
      b.redraw();
      true
    }
    Event::Leave => {
      b.set_color(Color::Cyan);
      b.redraw();
      true
    }
    Event::Push => {
      b.emit(sender, Message::Spaz);
      true
    }
    _ => false,
  });

  win.end();
  win.show();

  while app.wait() {
    if let Some(msg) = receiver.recv() {
      match msg {
        Message::Spaz => {
          counter += 1;
          counter_frame.set_label(&format!("Num Clicks: {}", counter));
          let rand_x = rand::thread_rng().gen_range(0..(WIN_WIDTH - BUTTON_WIDTH));
          let rand_y = rand::thread_rng().gen_range(0..(WIN_HEIGHT - BUTTON_HEIGHT));
          but.resize(rand_x, rand_y, BUTTON_WIDTH, BUTTON_HEIGHT);
          win.redraw();
        }
      }
    }
  }

  app.run().unwrap();
}

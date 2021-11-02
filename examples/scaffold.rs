// Imports
use nannou::prelude::*;

// struct
struct Model {
}

// constants
const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

// -> begin here
fn main() {
  nannou::app(model).update(update).run();
}

// setup
fn model(app: &App) -> Model {
  app.new_window().size(WIDTH, HEIGHT).view(view).build().unwrap();
  Model { }
}

// update
fn update(_app: &App, _model: &mut Model, _update: Update) {
}

// draw
fn view(app: &App, _model: &Model, frame: Frame) {
  let draw = app.draw();
  // do something

  draw.to_frame(app, &frame).unwrap();
}
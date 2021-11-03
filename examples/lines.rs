// Imports
use nannou::prelude::*;

// struct
struct Model {
  genesis: Vec2,
  commence: Vec2,
  color: f32
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
  let genesis: Vec2  = vec2(0.0, 0.0);
  let commence: Vec2 = vec2(0.0, 0.0);
  let color: f32     = 0.0;
  Model { genesis, commence, color }
}

// update
fn update(app: &App, model: &mut Model, _update: Update) {
  let f = app.elapsed_frames();
  let t = app.time;
  let slide: f32;
  let mod_number: u64 = f % 4;
  match mod_number {
    1 => slide =  4.0,
    2 => slide = -2.3,
    3 => slide = -PI,
    _ => slide =  PI,
  }
  model.genesis  = vec2(-100.0 + slide * 60.0 * t.cos(), 60.0 + 200.0 * (3.0 * t).sin());
  model.commence = vec2(  20.0 + 100.0 * t.cos() + (3.0 * t).cos(), 100.0 * (t + PI).sin());
  model.color    = t.cos() + t.sin();
}

// draw
fn view(app: &App, model: &Model, frame: Frame) {
  let draw = app.draw();
  // clear screen
  if frame.nth() == 0 || app.keys.down.contains(&Key::R) {
    draw.background().color(BLACK); // Resets
  }
  // line
  draw.line()
      .stroke_weight(2.0)
      .hsva(model.color, 1.0, 0.5, 0.5)
      .points(model.genesis, model.commence);
  draw.to_frame(app, &frame).unwrap();
}
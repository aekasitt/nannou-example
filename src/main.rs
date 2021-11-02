// Imports
use nannou::prelude::*;
use nannou::noise::*;

// Main
fn main() {
  nannou::app(model).update(update).run();
}

// Class(es) and Struct(s)
struct Thing {
  position: Vec2,
}
impl Thing {
  pub fn new(p: Vec2) -> Self {
    Thing { position:p, }
  }
}
struct Model {
  things: Vec<Thing>,
  noise: Perlin
}

// costants
const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 1024.0;
const N_THINGS: usize = 2000;

// setup
fn model(app: &App) -> Model {
  let _window = app.new_window().size(WIDTH as u32, HEIGHT as u32).view(view).build().unwrap();
  let mut things = Vec::new();
  for _i in 0..N_THINGS {
    let thing = Thing::new(vec2(
      (random::<f32>()-0.5) * WIDTH,
      (random::<f32>()-0.5) * HEIGHT
    ));
    things.push(thing);
  }
  let thing = Thing::new(vec2(0.0, 0.0));
  things.push(thing);
  let noise = Perlin::new();
  // returns when no semicolon at end of line
  Model { things, noise }
}

// update
fn update(_app: &App, model: &mut Model, _update: Update) {
  for thing in model.things.iter_mut() {
    let sn = 0.01 * random::<f64>();
    thing.position += vec2(
      model.noise.get([sn*thing.position.x as f64, sn*thing.position.y as f64, 0.0]) as f32,
      model.noise.get([sn*thing.position.x as f64, sn*thing.position.y as f64, 1.0]) as f32
    );
  }
}

// draw
fn view(app: &App, model: &Model, frame: Frame) {
  let draw = app.draw();
  let _time = app.elapsed_frames() as f32 / 60.0;
  draw.background().color(BLACK);
  // do something
  for thing in model.things.iter() {
    draw.ellipse().xy(thing.position).radius(5.0).color(WHITE);
  }
  // 
  draw.to_frame(app, &frame).unwrap();
}
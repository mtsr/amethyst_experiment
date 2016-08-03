extern crate time;
extern crate amethyst;
extern crate cgmath;

use std::sync::{Arc, Mutex};

use time::Duration;
use cgmath::Vector3;

use amethyst::engine::{Application, ApplicationBuilder, State, Trans};
use amethyst::context::Context;
use amethyst::config::Element;
use amethyst::ecs::{World, Entity, Simulation, Processor, RunArg, Component, VecStorage, JoinIter};

// First we define our components.

// Position in 3d of the Entity
#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    fn add_speed(&mut self, speed: &Speed) {
        self.x += speed.dx;
        self.y += speed.dy;
        self.z += speed.dz;
    }
}

impl Component for Position {
    type Storage = VecStorage<Position>;
}

// Example of a mesh component
#[derive(Debug)]
struct Mesh {
    handle: u64,
    y: usize,
}
impl Component for Mesh {
    type Storage = VecStorage<Mesh>;
}
// Example of a speed component
#[derive(Debug)]
struct Speed {
    dx: f32,
    dy: f32,
    dz: f32,
}
impl Component for Speed {
    type Storage = VecStorage<Speed>;
}

// Define our processors.

struct Update;
impl Processor<Arc<Mutex<Context>>> for Update {
    fn run(&mut self, arg: RunArg, _: Arc<Mutex<Context>>) {
        let (mut p, s) = arg.fetch(|w| (w.write::<Position>(), w.read::<Speed>())); // Make p writable.
        for (p, s) in JoinIter::new((&mut p, &s)) {
            // We want to only update entities with position and speed.
            p.add_speed(&s);
        }
    }
}

struct Render {
    frame_count: u32,
    init: bool,
}
impl Render {
    fn init(&self, context: Arc<Mutex<Context>>) {
        use amethyst::renderer::pass::{Clear, DrawShaded};
        use amethyst::renderer::Layer;

        let mut context = context.lock().unwrap();

        let layer =
            Layer::new("main",
                        vec![
                            Clear::new([0., 0., 0., 1.]),
                            DrawShaded::new("main", "main"),
                        ]);

        let pipeline = vec![layer];
        context.renderer.set_pipeline(pipeline);
    }
}
impl Processor<Arc<Mutex<Context>>> for Render {
    fn run(&mut self, arg: RunArg, context: Arc<Mutex<Context>>) {
        if self.init {
            self.init != false;
            self.init(context);
        }

        let (p, m) = arg.fetch(|w| (w.read::<Position>(), w.read::<Mesh>())); // Make p writable.
        for (p, _) in JoinIter::new((&p, &m)) {
            // We want to only render entities with position and mesh.
            println!("Render {:?}", p);
        }
        self.frame_count += 1;
    }
}

struct HelloWorld;

impl State for HelloWorld {
    fn handle_events(&mut self, events: Vec<Entity>, context: &mut Context, _: &mut World) -> Trans {
        use amethyst::context::event::{EngineEvent, Event, VirtualKeyCode};
        let mut trans = Trans::None;
        let storage = context.broadcaster.read::<EngineEvent>();
        for _event in events {
            let event = storage.get(_event).unwrap();
            let event = &event.payload;
            match *event {
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => trans = Trans::Quit,
                Event::Closed => trans = Trans::Quit,
                _ => (),
            }
        }
        trans
    }

    fn on_start(&mut self, context: &mut Context, world: &mut World) {
        use amethyst::renderer::{Camera, Light};
        let (w, h) = context.renderer.get_dimensions().unwrap();
        let proj = Camera::perspective(60.0, w as f32 / h as f32, 1.0, 100.0);
        let eye = [0., 5., 0.];
        let target = [0., 0., 0.];
        let up = [0., 0., 1.];
        let view = Camera::look_at(eye, target, up);
        let camera = Camera::new(proj, view);

        context.renderer.add_scene("main");
        context.renderer.add_camera(camera, "main");

        context.asset_manager.create_constant_texture("dark_blue", [0.0, 0.0, 0.01, 1.]);
        context.asset_manager.create_constant_texture("green", [0.0, 1.0, 0.0, 1.]);
        context.asset_manager.gen_sphere("sphere", 32, 32);

        let translation = Vector3::new(0.0, 0.0, 0.0);
        let transform: [[f32; 4]; 4] = cgmath::Matrix4::from_translation(translation).into();
        let fragment = context.asset_manager.get_fragment("sphere", "dark_blue", "green", transform).unwrap();

        context.renderer.add_fragment("main", fragment);

        let light = Light {
            color: [1., 1., 1., 1.],
            radius: 1.,
            center: [2., 2., 2.],
            propagation_constant: 0.,
            propagation_linear: 0.,
            propagation_r_square: 1.,
        };

        context.renderer.add_light("main", light);

        world.register::<Position>();
        world.register::<Speed>();
        world.register::<Mesh>();
    }

    fn update(&mut self, context: &mut Context, world: &mut World) -> Trans {
        context.renderer.submit();
        Trans::None
    }

    fn on_stop(&mut self, context: &mut Context, world: &mut World) {
        println!("Game stopped!");
    }
}

fn main() {
    use amethyst::context::Config;
    let config = Config::from_file("resources/config.yml").expect("Expect ../resources/config.yml to exist");
    let mut game = ApplicationBuilder::new(HelloWorld, config)
    .with(Update, "update", 50)
    .with(Render { frame_count: 30, init: true }, "render", 100)
    .done();
    game.run();
}

use std::sync::{Arc, Mutex};

use amethyst::context::Context;
use amethyst::ecs::{Processor, RunArg, JoinIter};

pub struct Update;
impl Processor<Arc<Mutex<Context>>> for Update {
    fn run(&mut self, arg: RunArg, _: Arc<Mutex<Context>>) {
        use ::components::{Position, Speed};

        let (mut p, s) = arg.fetch(|w| (w.write::<Position>(), w.read::<Speed>())); // Make p writable.
        for (p, s) in JoinIter::new((&mut p, &s)) {
            // We want to only update entities with position and speed.
            p.add_speed(&s);
        }
    }
}

pub struct Render {
    frame_count: u32,
    init: bool,
}
impl Render {
    pub fn new() -> Render {
        Render {
            frame_count: 0,
            init: true
        }
    }
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
        use ::components::{Mesh, Position};

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

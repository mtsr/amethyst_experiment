use cgmath::{Matrix4, Vector3};
// use time::Duration;

use amethyst::engine::{State, Trans};
use amethyst::context::Context;
use amethyst::config::Element;
use amethyst::ecs::{World, Entity};

pub struct HelloWorld;

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
        create_scene(context);
        create_camera(context);
        create_sphere(context);
        create_light(context);

        use components::{Mesh, Position, Speed};
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

fn create_camera(context: &mut Context) {
    use amethyst::renderer::Camera;

    let (w, h) = context.renderer.get_dimensions().unwrap();
    let proj = Camera::perspective(60.0, w as f32 / h as f32, 1.0, 100.0);
    let eye = [0., 5., 0.];
    let target = [0., 0., 0.];
    let up = [0., 0., 1.];
    let view = Camera::look_at(eye, target, up);
    let camera = Camera::new(proj, view);

    context.renderer.add_camera(camera, "main");
}

fn create_light(context: &mut Context) {
    use amethyst::renderer::Light;

    let light = Light {
        color: [1., 1., 1., 1.],
        radius: 1.,
        center: [2., 2., 2.],
        propagation_constant: 0.,
        propagation_linear: 0.,
        propagation_r_square: 1.,
    };

    context.renderer.add_light("main", light);
}

fn create_sphere(context: &mut Context) {
    context.asset_manager.create_constant_texture("dark_blue", [0.0, 0.0, 0.01, 1.]);
    context.asset_manager.create_constant_texture("green", [0.0, 1.0, 0.0, 1.]);
    context.asset_manager.gen_sphere("sphere", 32, 32);

    let translation = Vector3::new(0.0, 0.0, 0.0);
    let transform: [[f32; 4]; 4] = Matrix4::from_translation(translation).into();
    let fragment = context.asset_manager.get_fragment("sphere", "dark_blue", "green", transform).unwrap();

    context.renderer.add_fragment("main", fragment);
}

fn create_scene(context: &mut Context) {
    context.renderer.add_scene("main");
}
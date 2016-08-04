use amethyst::ecs::{Component, VecStorage, HashMapStorage};

// Position in 3d of the Entity
#[derive(Debug)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    pub fn add_speed(&mut self, speed: &Speed) {
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
pub struct Mesh {
    handle: u64,
    y: usize,
}
impl Component for Mesh {
    type Storage = VecStorage<Mesh>;
}
// Example of a speed component
#[derive(Debug)]
pub struct Speed {
    dx: f32,
    dy: f32,
    dz: f32,
}
impl Component for Speed {
    type Storage = VecStorage<Speed>;
}

pub struct Sphere {
    u: i32,
    v: i32,
}

impl Component for Sphere {
    type Storage = HashMapStorage<Sphere>;
}
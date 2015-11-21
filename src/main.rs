extern crate piston_window;
extern crate nphysics;
extern crate ncollide;
extern crate nalgebra;


use nphysics::world::World;
use nphysics::object::RigidBody;
use nphysics::object::RigidBodyHandle;
use nalgebra::{Vec2, Iso2, Vec1};
use piston_window::*;

pub struct Game {
    world: World,
    cubes: Vec<Cube>,
    timer: f64,
    temp: f64,
    ground_y: f64,
}

pub struct Cube {
    body: RigidBodyHandle,
    radius: f64,
}



impl Game {
    pub fn new() -> Game {
        let mut world = World::new();
        world.set_gravity(Vec2::new(0.0, 9.81));

        Game {
            world: world,
            cubes: Vec::with_capacity(300),
            timer: 0.0,
            temp: 0.0,
            ground_y: 440.0,
        }
    }

    pub fn init(&mut self){
        //bottom plane
        let mut rb = RigidBody::new_static(ncollide::shape::Plane::new(Vec2::new(0.0, -1.0)), 0.3, 0.6);
        rb.append_translation(&Vec2::new(0.0, self.ground_y));
        self.world.add_body(rb);
        //top plane
        let mut rb = RigidBody::new_static(ncollide::shape::Plane::new(Vec2::new(0.0, 1.0)), 0.3, 0.6);
        rb.append_translation(&Vec2::new(0.0, 0.0));
        self.world.add_body(rb);
        //left plane
        let mut rb = RigidBody::new_static(ncollide::shape::Plane::new(Vec2::new(1.0, 0.0)), 0.3, 0.6);
        rb.append_translation(&Vec2::new(0.0, 0.0));
        self.world.add_body(rb);
        //right plane
        let mut rb = RigidBody::new_static(ncollide::shape::Plane::new(Vec2::new(-1.0, 0.0)), 0.3, 0.6);
        rb.append_translation(&Vec2::new(640.0, 0.0));
        self.world.add_body(rb);

        for x in 0..6{
            let mut rb = RigidBody::new_dynamic(ncollide::shape::Cuboid::new(Vec2::new(20.0, 20.0)), 1.0, 0.3, 0.6);
            rb.append_translation(&Vec2::new(400.0, self.ground_y - 40.0*x as f64 - 20.0));
            let handle = self.world.add_body(rb);
            self.cubes.push(Cube::new(40.0, handle));
        }
        for x in 0..6{
            let mut rb = RigidBody::new_dynamic(ncollide::shape::Cuboid::new(Vec2::new(20.0, 20.0)), 1.0, 0.3, 0.6);
            rb.append_translation(&Vec2::new(360.0, self.ground_y - 40.0*x as f64 - 20.0));
            let handle = self.world.add_body(rb);
            self.cubes.push(Cube::new(40.0, handle));
        }
        let mut rb = RigidBody::new_dynamic(ncollide::shape::Cuboid::new(Vec2::new(25.0, 25.0)), 2.0, 0.3, 0.6);
        rb.append_translation(&Vec2::new(10.0, self.ground_y - 25.0));
        rb.set_lin_vel(Vec2::new(0.2, 0.0));
        let handle = self.world.add_body(rb);
        self.cubes.push(Cube::new(50.0, handle));

    }

    pub fn update(&mut self, upd: UpdateArgs) {
        self.timer += upd.dt;
        self.world.step(upd.dt * 4.0);
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        clear([0.925, 0.941, 0.945, 1.0], g);
        let square = [0.0, self.ground_y , 1000.0, 50.0];
        let red = [0.204, 0.286, 0.369, 1.0];
        for ball in &self.cubes {
            ball.render(c, g);
        }
        rectangle(red, square, c.transform, g);
    }
}

impl Cube {
    pub fn new(radius: f64, body: RigidBodyHandle) -> Cube {
        Cube {
            body: body,
            radius: radius,
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        let body = self.body.borrow();

        let pos = nalgebra::translation(body.position());
        let rot = nalgebra::rotation(body.position());
        let circle = rectangle::square(0.0, 0.0, self.radius);
        rectangle([0.906, 0.298, 0.235, 1.0], circle, c.transform.trans(pos.x , pos.y ).rot_rad(rot.x).trans(- self.radius/2.0, - self.radius/2.0), g);
    }

    pub fn update(){

    }
}

fn main() {
    let window: PistonWindow =
        WindowSettings::new("piston_nphysics", [640, 480])
        .exit_on_esc(true).build().unwrap();
    let mut game = Game::new();
    game.init();
    let mut cursor = [0.0, 0.0];
    for e in window {
            e.mouse_cursor(|x, y| {
                cursor = [x , y]

            });
            let pos = nalgebra::translation(game.cubes[game.cubes.len() - 1].body.borrow().position());
            game.cubes[game.cubes.len() - 1].body.borrow_mut().append_translation(&Vec2::new(cursor[0] - pos.x, cursor[1]  - pos.y));
            game.cubes[game.cubes.len() - 1].body.borrow_mut().set_rotation(Vec1::new(0.0));

            match e.event {
                Some(Event::Render(args)) => {
                    e.draw_2d(|c, g| {
                        game.render(c, g)
                    });
                }

                Some(Event::Update(args)) => {
                        game.update(args);
                }



                _ => {}

            }

        }

    }

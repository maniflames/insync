use std::sync::mpsc::*;
use three; 
use three::Object;
use recs::{Ecs};
use mint::Point3;
use clokwerk::{Scheduler, TimeUnits};

mod factory;
mod util; 
mod system;

#[derive(Clone, PartialEq, Debug)]
pub enum GameObjectType {
    Player,
    Enemy,
    Bullet
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32
}

#[derive(Clone, PartialEq, Debug)]
pub struct GameObject {
    mesh: three::Mesh,
    object_type: GameObjectType,
    vertices: Vec<Point3<f32>>,
    velocity: f32
}

#[derive(Clone, PartialEq, Debug)]
pub struct Score {
    total: i32,
    ui: three::Text
}

#[derive(Clone, PartialEq, Debug)]
pub struct Health {
    total: i32,
    ui: three::Text
}

#[derive(Clone, PartialEq, Debug)]
pub struct GameState {
    pending_enemies: Vec<Position>
}

fn main() {
    let mut window_builder = three::Window::builder("INSYNC");
    window_builder.fullscreen(true); 
    let mut window = window_builder.build();

    let camera = window.factory.perspective_camera(75.0, 1.0 .. 30.0);
    camera.set_position([0.0, 0.0, 10.0]);

    let mut store = Ecs::new();
    factory::create_player(&mut window, &mut store);

    let (sender, receiver): (SyncSender<Vec<Position>>, Receiver<Vec<Position>>) = sync_channel(1);
    let mut enemy_scheduler = Scheduler::new();
    
    enemy_scheduler.every(5.seconds()).run(move || {
            match sender.send(system::enemy_spawn::schedule_callback()) {
                Ok(_) => (),
                Err(err) => panic!("[enemy scheduler]: unable to schedule enemies. {:?}", err)
            }
        });

    while window.update() {
        system::input::run(&mut window, &mut store);
        system::position::run(&mut store);
        system::collision::run(&mut window, &mut store); 
        system::score::run(&mut store); 
        system::health::run(&mut store);
        system::enemy_spawn::run(&mut window, &mut store, &mut enemy_scheduler, &receiver); 
        system::garbage_collection::run(&mut window, &mut store);
        window.render(&camera);
    }
}


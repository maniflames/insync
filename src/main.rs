use std::sync::mpsc::*;
use three; 
use three::Object;
use recs::{Ecs, EntityId, component_filter};
use rand::Rng;
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

fn input_system(mut window: &mut three::Window, mut store: &mut Ecs) {
    let component_filter = component_filter!(Position, GameObject);
    let mut entities: Vec<EntityId> = Vec::new(); 
    store.collect_with(&component_filter, &mut entities);
        
    for entity in entities {
        let gameobject = store.get::<GameObject>(entity).unwrap(); 
        if gameobject.object_type == GameObjectType::Player {
            let position = store.get::<Position>(entity).unwrap();

            let space_button = three::Button::from(three::controls::Button::Key(three::controls::Key::Space));
            if window.input.hit(three::Key::Space) && window.input.hit_count(space_button) == 1 {
                factory::create_bullet(&mut window, &mut store, position); 
            }; 

            let mut new_position = position.clone(); 

            if window.input.hit(three::Key::W) {
                new_position.y = new_position.y + gameobject.velocity; 
            }

            if window.input.hit(three::Key::S) {
                new_position.y = new_position.y - gameobject.velocity; 
            }

            if window.input.hit(three::Key::A) {
                new_position.x = new_position.x - gameobject.velocity; 
            }
        
            if window.input.hit(three::Key::D) {
                new_position.x = new_position.x + gameobject.velocity;  
            }

            let _ = store.set::<Position>(entity, new_position).unwrap();       
        }
    }
}

fn score_system(store: &mut recs::Ecs) {
    //NOTE: this method is pretty ineffecient, I should probably try something with a history in a gamestate
    let mut scores: Vec<EntityId> = Vec::new(); 
    store.collect_with(&component_filter!(Score), &mut scores);
    let mut score = store.get::<Score>(scores[0]).unwrap();
    
    let score_prefix: &str = "score: ";
    let score_string: &str = &score.total.to_string();

   score.ui.set_text(format!("{}{}", score_prefix, score_string)); 
}

fn gamestate_system(window: &mut three::Window, store: &mut recs::Ecs) {
    let mut enitites: Vec<EntityId> = Vec::new(); 
    store.collect_with(&component_filter!(Health), &mut enitites);
    let health = store.get::<Health>(enitites[0]).unwrap();

   //if health is 0 destroy player 
   if health.total == 0 {
       util::remove_entity(enitites[0], store, window);
   }
}

fn health_system(store: &mut recs::Ecs) {
    let mut enitites: Vec<EntityId> = Vec::new(); 
    store.collect_with(&component_filter!(Health), &mut enitites);
    let mut health = store.get::<Health>(enitites[0]).unwrap();
    
    let health_prefix: &str = "lives: ";
    let health_string: &str = &health.total.to_string();

   health.ui.set_text(format!("{}{}", health_prefix, health_string));
}

fn enemy_scheduler_system() -> Vec<Position> { 
    let mut random = rand::thread_rng();
    let num_meteors: i32 = random.gen_range(5, 15);
    let radius: f32 = random.gen_range(2.0, 5.0);
    let d_angle = 360.0 / (num_meteors as f32); 
    let z = random.gen_range(-30.0, -25.0);

    let mut pending_enemies: Vec<Position> = Vec::new();
    for index in 0..num_meteors {
        let cartesian_coordinates = util::polar_to_cartesian(radius, d_angle * ((index as f32) + 1.0));
 
        pending_enemies.push(Position{ 
                x: cartesian_coordinates[0],
                y: cartesian_coordinates[1],
                z: z
            });
    }

    return pending_enemies;
}

fn enemy_spawn_system(window: &mut three::Window, store: &mut Ecs, pending_enemies: Vec<Position>) {
    if pending_enemies.is_empty() {
        return
    }

    for position in pending_enemies.iter().rev() {
        factory::create_enemy(window, store, *position); 
    }   
}

fn garbage_collection_system(mut window: &mut three::Window, mut store: &mut Ecs) {
    let mut entities: Vec<EntityId> = Vec::new();
    store.collect_with(&component_filter!(GameObject, Position), &mut entities);
    for entity in entities.iter().rev() {
        let gameobject = store.get::<GameObject>(*entity).unwrap();
        let position = store.get::<Position>(*entity).unwrap();
        match gameobject.object_type {
            GameObjectType::Enemy => {
                //if traveled beyond camera
                if position.z > 12.0 {
                    util::remove_entity(*entity, &mut store, &mut window);
                }
            },
            GameObjectType::Bullet => {
                //if traveled beyond the edge of the world
                if position.z < -35.0 {
                    util::remove_entity(*entity, &mut store, &mut window);
                }
            },
            GameObjectType::Player => (), //?? cleanup player in here in stead of in gamestate??
        }
    }
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
            match sender.send(enemy_scheduler_system()) {
                Ok(_) => (),
                Err(err) => panic!("[enemy scheduler]: unable to schedule enemies. {:?}", err)
            }
        });

    while window.update() {
        input_system(&mut window, &mut store);
        system::position::run(&mut store);
        system::collision::run(&mut window, &mut store); 
        score_system(&mut store); 
        health_system(&mut store);
        enemy_scheduler.run_pending();
        match receiver.try_recv() {
            Ok(pending_enemies) => enemy_spawn_system(&mut window, &mut store, pending_enemies),
            Err(_) => ()
        }
        gamestate_system(&mut window, &mut store);
        garbage_collection_system(&mut window, &mut store);
        window.render(&camera);
    }
}


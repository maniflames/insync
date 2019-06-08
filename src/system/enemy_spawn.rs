use std::sync::mpsc::*;
use rand::Rng;
use crate::*; 

//replace this for only running on peak detection (match peak_deteciion) (match in match!!)
pub fn run(window: &mut three::Window, store: &mut Ecs, enemy_scheduler: &mut clokwerk::Scheduler, receiver: &Receiver<Vec<Position>>) {
    enemy_scheduler.run_pending();
    match receiver.try_recv() {
        Ok(pending_enemies) => {
            if pending_enemies.is_empty() {
                return
            }

            for _position in pending_enemies.iter().rev() {
                // factory::create_enemy(window, store, *position); 
                create_single(window, store);
            }   
        },
        Err(_) => ()
    }
}

pub fn create_single(window: &mut three::Window, store: &mut Ecs) {
    let mut random = rand::thread_rng();

    factory::create_enemy(window, store, Position{
        x: random.gen_range(-5.0, 5.0),
        y: random.gen_range(-5.0, 5.0),
        z: random.gen_range(-30.0, -25.0)
    });
}

pub fn schedule_callback() -> Vec<Position> { 
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
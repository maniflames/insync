use std::sync::mpsc::*;
use three; 
use three::Object;
use recs::{Ecs, EntityId, component_filter};
use rand::Rng;
use mint::Point3;
use clokwerk::{Scheduler, TimeUnits};

#[derive(Clone, PartialEq, Debug)]
enum GameObjectType {
    Player,
    Enemy,
    Bullet
}

#[derive(Clone, PartialEq, Debug, Copy)]
struct Position {
    x: f32,
    y: f32,
    z: f32
}

#[derive(Clone, PartialEq, Debug)]
struct GameObject {
    mesh: three::Mesh,
    object_type: GameObjectType,
    vertices: Vec<Point3<f32>>,
    velocity: f32
}

#[derive(Clone, PartialEq, Debug)]
struct Score {
    total: i32,
    ui: three::Text
}

#[derive(Clone, PartialEq, Debug)]
struct Health {
    total: i32,
    ui: three::Text
}

#[derive(Clone, PartialEq, Debug)]
struct GameState {
    pending_enemies: Vec<Position>
}

fn collision_system(mut window: &mut three::Window, mut store: &mut recs::Ecs) {
    //NOTE: usage of skeletons would be nice since meshes can be used and the game can move away from basic shapes
    //NOTE: in the next iteration dimentions should at least be precalculated from the vertices in the base shape. 

    let component_filter = component_filter!(Position, GameObject);
    let mut entities: Vec<EntityId> = Vec::new(); 
    // (EntityId, minX, maxX, minY, maxY, minZ, maxZ)
    let mut enemies: Vec<(&EntityId, f32, f32, f32, f32, f32, f32)> = Vec::new(); 
    let mut bullets: Vec<(&EntityId, f32, f32, f32, f32, f32, f32)> = Vec::new(); 
    let mut player = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    
    store.collect_with(&component_filter, &mut entities);
        let find_min = |min, current| {
            if current < min {
                return current;
            }

            return min;
        };

        let find_max = |max, current| {
            if current > max {
                return current;
            }

            return max;
        };

    for entity in entities.iter() {
        let gameobject = store.get::<GameObject>(*entity).unwrap(); 
        let position = store.get::<Position>(*entity).unwrap(); 
  
        let x_values = gameobject.vertices.iter().map(|vertex| vertex.x );
        let x_min = x_values.clone().fold(0.0, find_min); 
        let x_max = x_values.clone().fold(0.0, find_max);

        let y_values = gameobject.vertices.iter().map(|vertex| vertex.y );
        let y_min = y_values.clone().fold(0.0, find_min); 
        let y_max = y_values.clone().fold(0.0, find_max);

        let z_values = gameobject.vertices.iter().map(|vertex| vertex.z );
        let z_min = z_values.clone().fold(0.0, find_min); 
        let z_max = z_values.clone().fold(0.0, find_max);

        match gameobject.object_type {
            GameObjectType::Enemy => enemies.push((entity, x_min + position.x, x_max + position.x, y_min + position.y, y_max + position.y, z_min + position.z, z_max + position.z)),
            GameObjectType::Player => player = (x_min + position.x, x_max + position.x, y_min + position.y, y_max + position.y, z_min + position.z, z_max + position.z),
            GameObjectType::Bullet => bullets.push((entity, x_min + position.x, x_max + position.x, y_min + position.y, y_max + position.y, z_min + position.z, z_max + position.z)),
        }
    }

    for enemy in enemies.iter().rev() {

        let (player_x_min, player_x_max, player_y_min, player_y_max, player_z_min, player_z_max) = player;
        let (enemy_entity, enemy_x_min, enemy_x_max, enemy_y_min, enemy_y_max, enemy_z_min, enemy_z_max) = *enemy;

        //check collision with player
        if player_x_min < enemy_x_max && player_x_max > enemy_x_min {
            if player_y_min < enemy_y_max && player_y_max > enemy_y_min {
                if player_z_min < enemy_z_max && player_z_max > enemy_z_min {
                    remove_entity(*enemy_entity, &mut store, &mut window);

                    let mut enitites: Vec<EntityId> = Vec::new(); 
                    store.collect_with(&component_filter!(Health), &mut enitites);
                    let mut health = store.get::<Health>(enitites[0]).unwrap();
                    health.total = health.total - 1; 
                    let _ = store.set(enitites[0], health);
                }
            }
        }

        //check collision with bullets
        for bullet in bullets.iter().rev(){
            let (bullet_entity, bullet_x_min, bullet_x_max, bullet_y_min, bullet_y_max, bullet_z_min, bullet_z_max) = *bullet;
            if bullet_x_min < enemy_x_max && bullet_x_max > enemy_x_min {
                if bullet_y_min < enemy_y_max && bullet_y_max > enemy_y_min {
                    if bullet_z_min < enemy_z_max && bullet_z_max > enemy_z_min {
                        remove_entity(*enemy_entity, &mut store, &mut window);
                        remove_entity(*bullet_entity, &mut store, &mut window);

                        let mut scores: Vec<EntityId> = Vec::new(); 
                        store.collect_with(&component_filter!(Score), &mut scores);
                        let mut score = store.get::<Score>(scores[0]).unwrap();
                        score.total = score.total + 100; 
                        let _ = store.set::<Score>(scores[0], score); 
                    }
                }
            }
        }
    }
}

fn remove_entity(entity: EntityId, store: &mut recs::Ecs, window: &mut three::Window) {
    let result = store.get::<GameObject>(entity);
    match result {
        Ok(removable) => {
                window.scene.remove(removable.mesh);
                let _ = store.destroy_entity(entity);
            },
        Err(err) => println!("[remove_entity]: tried to remove {:?} but couldn't find it.", err), 
    }
}

fn position_bullet(entity: &EntityId, store: &mut recs::Ecs) {
    let gameobject = store.get::<GameObject>(*entity).unwrap();
    let old_position = store.get::<Position>(*entity).unwrap();
    let new_position = Position{ x: old_position.x, y: old_position.y, z: old_position.z - gameobject.velocity};
    let _ = store.set::<Position>(*entity, new_position).unwrap();

    gameobject.mesh.set_position([new_position.x, new_position.y, new_position.z]);
}

fn position_enemy(entity: &EntityId, store: &mut recs::Ecs) {
    let gameobject = store.get::<GameObject>(*entity).unwrap();
    let old_position = store.get::<Position>(*entity).unwrap();
    let new_position = Position{ x: old_position.x, y: old_position.y, z: old_position.z + gameobject.velocity };
    let _ = store.set::<Position>(*entity, new_position).unwrap();

    gameobject.mesh.set_position([new_position.x, new_position.y, new_position.z]);
}

fn position_player(entity: &EntityId, store: &mut recs::Ecs) {
    let gameobject = store.get::<GameObject>(*entity).unwrap();
    let position = store.get::<Position>(*entity).unwrap();
    gameobject.mesh.set_position([position.x, position.y, position.z]);
}

fn positioning_system(mut store: &mut recs::Ecs) {
    let component_filter = component_filter!(Position, GameObject);
    let mut entities: Vec<EntityId> = Vec::new(); 
    
    store.collect_with(&component_filter, &mut entities);

    for entity in entities.iter() {
        let gameobject = store.get::<GameObject>(*entity).unwrap();

        match gameobject.object_type {
            GameObjectType::Enemy => position_enemy(entity, &mut store), 
            GameObjectType::Player => position_player(entity, &mut store), 
            GameObjectType::Bullet => position_bullet(entity, &mut store),
        }
    }
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
                bullet_factory(&mut window, &mut store, position); 
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

fn score_factory(window: &mut three::Window, font: &three::Font) -> Score {
    let mut score_ui = window.factory.ui_text(&font, ""); 
    score_ui.set_font_size(92.0);
    score_ui.set_pos([window.size().x, 0.0]);
    score_ui.set_layout(three::Layout::SingleLine(three::Align::Right));
    
    window.scene.add(&score_ui); 
    return Score{total: 0, ui: score_ui}
}

fn gamestate_system(window: &mut three::Window, store: &mut recs::Ecs) {
    let mut enitites: Vec<EntityId> = Vec::new(); 
    store.collect_with(&component_filter!(Health), &mut enitites);
    let health = store.get::<Health>(enitites[0]).unwrap();

   //if health is 0 destroy player 
   if health.total == 0 {
       remove_entity(enitites[0], store, window);
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

fn health_factory(window: &mut three::Window, font: &three::Font) -> Health {
    let mut health_ui = window.factory.ui_text(&font, "lives: 3"); 
    health_ui.set_font_size(92.0);
    
    window.scene.add(&health_ui); 
    return Health{total: 3, ui: health_ui}
}

fn bullet_factory(window: &mut three::Window, store: &mut Ecs, position: Position) {
    let bullet = store.create_entity();
    let _ = store.set(bullet, Position{ x: position.x, y: position.y, z: position.z});

    let geometry = three::Geometry::cuboid(0.1, 0.1, 0.5); 
    let material = three::material::Basic {
        color: 0xFFFFFF,
        .. Default::default()
    };

    let vertices = geometry.base.vertices.clone();

    let mesh = window.factory.mesh(geometry, material); 
    window.scene.add(&mesh);

    let _ = store.set(bullet, GameObject{mesh: mesh, object_type: GameObjectType::Bullet, vertices: vertices, velocity: 0.25});
}

fn meteor_factory(window: &mut three::Window, store: &mut Ecs, position: Position) {
    let cube = store.create_entity();
    let _ = store.set(cube, position);

    let geometry = three::Geometry::cuboid(1.0, 1.0, 1.0); 
    let material = three::material::Basic {
        color: 0xFF0000,
        .. Default::default()
    };

    let vertices = geometry.base.vertices.clone();

    let mesh = window.factory.mesh(geometry, material); 
    mesh.set_position([position.x, position.y, position.z]);
    window.scene.add(&mesh);
    let _ = store.set(cube, GameObject{mesh: mesh, object_type: GameObjectType::Enemy, vertices: vertices, velocity: 0.07});
}

fn polar_to_cartesian(radius: f32, angle: f32) -> [f32; 2] {
    //angles are converted from degrees to radians because rust calculates sine functions with radians 
    let x = radius * angle.to_radians().cos();
    let y = radius * angle.to_radians().sin();
    return [x, y];
}

fn enemy_scheduler_system() -> Vec<Position> { 
    let mut random = rand::thread_rng();
    let num_meteors: i32 = random.gen_range(5, 15);
    let radius: f32 = random.gen_range(2.0, 5.0);
    let d_angle = 360.0 / (num_meteors as f32); 
    let z = random.gen_range(-30.0, -25.0);

    let mut pending_enemies: Vec<Position> = Vec::new();
    for index in 0..num_meteors {
        let cartesian_coordinates = polar_to_cartesian(radius, d_angle * ((index as f32) + 1.0));
 
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
        meteor_factory(window, store, *position); 
    }   
}

fn player_factory(mut window: &mut three::Window, store: &mut Ecs) {
    let player = store.create_entity();
    let _ = store.set(player, Position{ x: 0.0, y: 0.0, z: 0.0});

    let font = window.factory.load_font_karla();
    let score = score_factory(&mut window, &font);
    let _ = store.set(player, score);
    let health = health_factory(&mut window, &font);
    let _ = store.set(player, health);

    let geometry = three::Geometry::cuboid(1.0, 1.0, 1.0); 
    let material = three::material::Basic {
        color: 0xFFFFFF,
        .. Default::default()
    };

    let vertices = geometry.base.vertices.clone();

    let mesh = window.factory.mesh(geometry, material); 
    window.scene.add(&mesh);

    let _ = store.set(player, GameObject{mesh: mesh, object_type: GameObjectType::Player, vertices: vertices, velocity: 0.07});
}

fn main() {
    let mut window_builder = three::Window::builder("INSYNC");
    window_builder.fullscreen(true); 
    let mut window = window_builder.build();

    let camera = window.factory.perspective_camera(75.0, 1.0 .. 30.0);
    camera.set_position([0.0, 0.0, 10.0]);

    let mut store = Ecs::new();
    player_factory(&mut window, &mut store);

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
        positioning_system(&mut store);
        collision_system(&mut window, &mut store); 
        score_system(&mut store); 
        health_system(&mut store);
        enemy_scheduler.run_pending();
        match receiver.try_recv() {
            Ok(pending_enemies) => enemy_spawn_system(&mut window, &mut store, pending_enemies),
            Err(_) => ()
        }
        gamestate_system(&mut window, &mut store);
        window.render(&camera);
    }
}


use three; 
use three::Object;
use recs::{Ecs, EntityId, component_filter};
use rand::Rng;
use mint::Point3;

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
    vertices: Vec<Point3<f32>>
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

    for enemy in enemies {
        let (player_x_min, player_x_max, player_y_min, player_y_max, player_z_min, player_z_max) = player;
        let (enemy_entity, enemy_x_min, enemy_x_max, enemy_y_min, enemy_y_max, enemy_z_min, enemy_z_max) = enemy;

        //check collision with player
        if player_x_min < enemy_x_max && player_x_max > enemy_x_min {
            if player_y_min < enemy_y_max && player_y_max > enemy_y_min {
                if player_z_min < enemy_z_max && player_z_max > enemy_z_min {
                    println!("hit!");
                    //TODO: remove player from world 
                }
            }
        }

        //check collision with bullets
        for bullet in &bullets {
            let (bullet_entity, bullet_x_min, bullet_x_max, bullet_y_min, bullet_y_max, bullet_z_min, bullet_z_max) = *bullet;
            if bullet_x_min < enemy_x_max && bullet_x_max > enemy_x_min {
                if bullet_y_min < enemy_y_max && bullet_y_max > enemy_y_min {
                    if bullet_z_min < enemy_z_max && bullet_z_max > enemy_z_min {
                        remove_entity(*enemy_entity, &mut store, &mut window);
                        remove_entity(*bullet_entity, &mut store, &mut window);
                    }
                }
            }
        }
    }
}

fn remove_entity(entity: EntityId, store: &mut recs::Ecs, window: &mut three::Window) {
    let removable = store.get::<GameObject>(entity).unwrap();
    window.scene.remove(removable.mesh); 
    let _ = store.destroy_entity(entity);
}

fn position_bullet(entity: &EntityId, store: &mut recs::Ecs) {
    let gameobject = store.get::<GameObject>(*entity).unwrap();
    let old_position = store.get::<Position>(*entity).unwrap();
    let new_position = Position{ x: old_position.x, y: old_position.y, z: old_position.z - 0.02 };
    let _ = store.set::<Position>(*entity, new_position).unwrap();

    gameobject.mesh.set_position([new_position.x, new_position.y, new_position.z]);
}

fn position_enemy(entity: &EntityId, store: &mut recs::Ecs) {
    let gameobject = store.get::<GameObject>(*entity).unwrap();
    let old_position = store.get::<Position>(*entity).unwrap();
    let new_position = Position{ x: old_position.x, y: old_position.y, z: old_position.z + 0.02 };
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

            if window.input.hit(three::Key::Space) {
                bullet_factory(&mut window, &mut store, position); 
            }; 

            let mut new_position = position.clone(); 

            if window.input.hit(three::Key::W) {
                new_position.y = new_position.y + 0.02; 
            }

            if window.input.hit(three::Key::S) {
                new_position.y = new_position.y - 0.02; 
            }

            if window.input.hit(three::Key::A) {
                new_position.x = new_position.x - 0.02; 
            }
        
            if window.input.hit(three::Key::D) {
                new_position.x = new_position.x + 0.02;  
            }

            let _ = store.set::<Position>(entity, new_position).unwrap();       
        }
    }
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

    let _ = store.set(bullet, GameObject{mesh: mesh, object_type: GameObjectType::Bullet, vertices: vertices});
}

fn meteor_factory(window: &mut three::Window, store: &mut Ecs, num_meteors: i32) {
    let range = 0..num_meteors;
    let mut random = rand::thread_rng();

    for (_index, _meteor ) in range.enumerate() {
        let cube = store.create_entity();
        
        let _ = store.set(cube, Position{ 
            x: random.gen_range(0.0, 3.0), 
            y: random.gen_range(0.0, 3.0), 
            z: random.gen_range(-6.0, 0.0) });

        let geometry = three::Geometry::cuboid(1.0, 1.0, 1.0); 
        let material = three::material::Basic {
            color: 0xFF0000,
            .. Default::default()
        };

        let vertices = geometry.base.vertices.clone();

        let mesh = window.factory.mesh(geometry, material); 
        window.scene.add(&mesh);
        let _ = store.set(cube, GameObject{mesh: mesh, object_type: GameObjectType::Enemy, vertices: vertices});
    }
}

fn player_factory(window: &mut three::Window, store: &mut Ecs) {
    let player = store.create_entity();
    let _ = store.set(player, Position{ x: 0.0, y: 0.0, z: 0.0});

    let geometry = three::Geometry::cuboid(1.0, 1.0, 1.0); 
    let material = three::material::Basic {
        color: 0xFFFFFF,
        .. Default::default()
    };

    let vertices = geometry.base.vertices.clone();

    let mesh = window.factory.mesh(geometry, material); 
    window.scene.add(&mesh);

    let _ = store.set(player, GameObject{mesh: mesh, object_type: GameObjectType::Player, vertices: vertices});
}

fn main() {
    let mut window_builder = three::Window::builder("INSYNC");
    window_builder.fullscreen(true); 
    let mut window = window_builder.build();

    let camera = window.factory.perspective_camera(75.0, 1.0 .. 50.0);
    camera.set_position([0.0, 0.0, 10.0]);

    let mut store = Ecs::new();
    meteor_factory(&mut window, &mut store, 2);
    player_factory(&mut window, &mut store);
    
    while window.update() {
        input_system(&mut window, &mut store);
        collision_system(&mut window, &mut store); 
        positioning_system(&mut store);
        window.render(&camera);
    }
}


use three; 
use three::Object;
use recs::{Ecs, EntityId, component_filter};
use rand::Rng;

#[derive(Clone, PartialEq, Debug, Copy)]
struct Position {
    x: f32,
    y: f32,
    z: f32
}

#[derive(Clone, PartialEq, Debug)]
struct GameObject {
    mesh: three::Mesh,
    is_player: bool
}

fn positioning_system(window: &mut three::Window, store: &mut recs::Ecs) {
    let component_filter = component_filter!(Position, GameObject);
    let mut entities: Vec<EntityId> = Vec::new(); 
    
    store.collect_with(&component_filter, &mut entities);

    for entity in entities.iter() {
        let gameobject = store.get::<GameObject>(*entity).unwrap();

        if !gameobject.is_player {
            let old_position = store.get::<Position>(*entity).unwrap();
            let new_position = Position{ x: old_position.x, y: old_position.y, z: old_position.z + 0.02 };
            let _ = store.set::<Position>(*entity, new_position).unwrap();

            gameobject.mesh.set_position([new_position.x, new_position.y, new_position.z]);
        }

        if gameobject.is_player {
            let old_position = store.get::<Position>(*entity).unwrap();
            let mut new_position = old_position.clone(); 

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
                
            let _ = store.set::<Position>(*entity, new_position).unwrap();
            gameobject.mesh.set_position([new_position.x, new_position.y, new_position.z]);
        }
    }
}

fn meteor_factory(window: &mut three::Window, store: &mut Ecs, num_meteors: i32) {
    let range = 0..num_meteors;
    let mut random = rand::thread_rng();

    for (_index, _meteor ) in range.enumerate() {
        let cube = store.create_entity();
        
        let _ = store.set(cube, Position{ 
            x: random.gen_range(0.0, 3.0), 
            y: random.gen_range(0.0, 3.0), 
            z: random.gen_range(0.0, 3.0) });

        let geometry = three::Geometry::cuboid(1.0, 1.0, 1.0); 
        let material = three::material::Basic {
            color: 0xFF0000,
            .. Default::default()
        };

        let mesh = window.factory.mesh(geometry, material); 
        window.scene.add(&mesh);
        let _ = store.set(cube, GameObject{mesh: mesh, is_player: false});
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

    let mesh = window.factory.mesh(geometry, material); 
    window.scene.add(&mesh);
    let _ = store.set(player, GameObject{mesh: mesh, is_player: true});
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
        positioning_system(&mut window, &mut store);
        window.render(&camera);
    }
}


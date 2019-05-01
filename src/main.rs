use three; 
use three::Object;
use recs::{Ecs, EntityId, component_filter};

#[derive(Clone, PartialEq, Debug, Copy)]
struct Position {
    x: f32,
    y: f32,
    z: f32
}

#[derive(Clone, PartialEq, Debug)]
struct GameObject {
    mesh: three::Mesh
}

fn positioning_system(store: &mut recs::Ecs) {
    let component_filter = component_filter!(Position, GameObject);
    let mut entities: Vec<EntityId> = Vec::new(); 

    store.collect_with(&component_filter, &mut entities);

    for entity in entities.iter() {
        let old_position = store.get::<Position>(*entity).unwrap();
        let new_position = Position{ x: old_position.x, y: old_position.y, z: old_position.z + 0.02 };
        let _ = store.set::<Position>(*entity, new_position).unwrap();

        let gameobject = store.get::<GameObject>(*entity).unwrap();
        gameobject.mesh.set_position([new_position.x, new_position.y, new_position.z]);
    }
}

fn main() {
    let mut window_builder = three::Window::builder("INSYNC");
    window_builder.fullscreen(true); 
    let mut window = window_builder.build();

    let camera = window.factory.perspective_camera(75.0, 1.0 .. 50.0);
    camera.set_position([0.0, 0.0, 10.0]);

    let mut store = Ecs::new();
    let cube = store.create_entity();
    let _ = store.set(cube, Position{ x: 0.0, y: 0.0, z: 0.0});

    let geometry = three::Geometry::cuboid(1.0, 1.0, 1.0); 
    let material = three::material::Basic {
        color: 0xFFFFFF,
        .. Default::default()
    };

    let mesh = window.factory.mesh(geometry, material); 
    window.scene.add(&mesh);
    let _ = store.set(cube, GameObject{mesh: mesh});
    
    while window.update() {
        positioning_system(&mut store);
        window.render(&camera);
    }
}


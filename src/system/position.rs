use three; 
use three::Object;
use recs::{EntityId, component_filter};
use crate::*; 

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

pub fn run(mut store: &mut recs::Ecs) {
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
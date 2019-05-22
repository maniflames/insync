use recs::{EntityId, component_filter};
use crate::*; 

pub fn run(mut window: &mut three::Window, mut store: &mut Ecs) {
    let mut entities: Vec<EntityId> = Vec::new();
    store.collect_with(&component_filter!(GameObject, Position), &mut entities);
    store.collect_with(&component_filter!(Health), &mut entities);

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
            GameObjectType::Player => {
                let health = store.get::<Health>(*entity).unwrap();

                if health.total == 0 {
                    util::remove_entity(*entity, store, window);
                }
            }, 
        }
    }
}
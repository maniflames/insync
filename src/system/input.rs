use recs::{EntityId, component_filter};
use crate::*; 

pub fn run(mut window: &mut three::Window, mut store: &mut Ecs) {
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
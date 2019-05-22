use recs::{EntityId};

pub fn remove_entity(entity: EntityId, store: &mut recs::Ecs, window: &mut three::Window) {
    let result = store.get::<super::GameObject>(entity);
    match result {
        Ok(removable) => {
                window.scene.remove(removable.mesh);
                let _ = store.destroy_entity(entity);
            },
        Err(err) => println!("[remove_entity]: tried to remove {:?} but couldn't find it.", err), 
    }
}

pub fn polar_to_cartesian(radius: f32, angle: f32) -> [f32; 2] {
    //angles are converted from degrees to radians because rust calculates sine functions with radians 
    let x = radius * angle.to_radians().cos();
    let y = radius * angle.to_radians().sin();
    return [x, y];
}

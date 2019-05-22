use recs::{EntityId, component_filter};
use crate::*; 

pub fn run(mut window: &mut three::Window, mut store: &mut recs::Ecs) {
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
                    util::remove_entity(*enemy_entity, &mut store, &mut window);

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
                        util::remove_entity(*enemy_entity, &mut store, &mut window);
                        util::remove_entity(*bullet_entity, &mut store, &mut window);

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
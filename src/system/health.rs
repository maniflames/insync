use recs::{EntityId, component_filter};
use crate::*; 

pub fn run(store: &mut recs::Ecs) {
    let mut enitites: Vec<EntityId> = Vec::new(); 
    store.collect_with(&component_filter!(Health), &mut enitites);
    let mut health = store.get::<Health>(enitites[0]).unwrap();
    
    let health_prefix: &str = "lives: ";
    let health_string: &str = &health.total.to_string();

   health.ui.set_text(format!("{}{}", health_prefix, health_string));
}
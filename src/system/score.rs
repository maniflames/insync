use recs::{EntityId, component_filter};
use crate::*; 

pub fn run(store: &mut recs::Ecs) {
    //NOTE: this method is pretty ineffecient, I should probably try something with a history in a gamestate
    let mut scores: Vec<EntityId> = Vec::new(); 
    store.collect_with(&component_filter!(Score), &mut scores);
    let mut score = store.get::<Score>(scores[0]).unwrap();
    
    let score_prefix: &str = "score: ";
    let score_string: &str = &score.total.to_string();

   score.ui.set_text(format!("{}{}", score_prefix, score_string)); 
}
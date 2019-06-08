use three; 
use three::Object;
use recs::Ecs;
use crate::*; 

fn create_health(window: &mut three::Window, font: &three::Font) -> super::Health {
    let mut health_ui = window.factory.ui_text(&font, "lives: 3"); 
    health_ui.set_font_size(92.0);
    
    window.scene.add(&health_ui); 
    return Health{total: 3, ui: health_ui}
}

fn create_score(window: &mut three::Window, font: &three::Font) -> super::Score {
    let mut score_ui = window.factory.ui_text(&font, ""); 
    score_ui.set_font_size(92.0);
    score_ui.set_pos([window.size().x, 0.0]);
    score_ui.set_layout(three::Layout::SingleLine(three::Align::Right));
    
    window.scene.add(&score_ui); 
    return Score{total: 0, ui: score_ui}
}

pub fn create_bullet(window: &mut three::Window, store: &mut Ecs, position: Position) {
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

    let _ = store.set(bullet, GameObject{mesh: mesh, object_type: GameObjectType::Bullet, vertices: vertices, velocity: 0.25});
}

pub fn create_enemy(window: &mut three::Window, store: &mut Ecs, position: Position) {
    let cube = store.create_entity();
    let _ = store.set(cube, position);

    let geometry = three::Geometry::cuboid(1.0, 1.0, 1.0); 
    let material = three::material::Basic {
        color: 0xFF0000,
        .. Default::default()
    };

    let vertices = geometry.base.vertices.clone();

    let mesh = window.factory.mesh(geometry, material); 
    mesh.set_position([position.x, position.y, position.z]);
    window.scene.add(&mesh);
    let _ = store.set(cube, GameObject{mesh: mesh, object_type: GameObjectType::Enemy, vertices: vertices, velocity: 0.07});
}

pub fn create_player(mut window: &mut three::Window, store: &mut Ecs) {
    let player = store.create_entity();
    let _ = store.set(player, Position{ x: 0.0, y: 0.0, z: 0.0});

    let font = window.factory.load_font_karla();
    let score = create_score(&mut window, &font);
    let _ = store.set(player, score);
    let health = create_health(&mut window, &font);
    let _ = store.set(player, health);

    let basic_pipeline = window.factory.basic_pipeline(
            "./src/shaders",
            "gradient",
            three::custom::Primitive::TriangleList,
            three::custom::state::Rasterizer::new_fill(),
            three::custom::state::ColorMask::all(),
            three::custom::state::Blend::new(three::custom::state::Equation::Add, three::custom::state::Factor::ZeroPlus(three::custom::state::BlendValue::SourceColor), three::custom::state::Factor::Zero),
            three::custom::state::Depth{
                fun: three::custom::state::Comparison::LessEqual, 
                write: true
            },
            three::custom::state::Stencil::new(
                three::custom::state::Comparison::Always,
                1,
                (three::custom::state::StencilOp::Keep, three::custom::state::StencilOp::Keep, three::custom::state::StencilOp::Keep)
            )
        ); 

    
    let material = three::material::basic::Custom{
        color: 0xFFFFFF,
        map: None,
        pipeline: basic_pipeline.expect("custom pipeline crashed")
    };

    let geometry = three::Geometry::cuboid(1.0, 1.0, 1.0); 

    let vertices = geometry.base.vertices.clone();
    let mesh = window.factory.mesh(geometry, material); 
    window.scene.add(&mesh);

    let _ = store.set(player, GameObject{mesh: mesh, object_type: GameObjectType::Player, vertices: vertices, velocity: 0.07});
}
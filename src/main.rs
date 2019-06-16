use std::sync::mpsc::*;
use three; 
use three::Object;
use recs::{Ecs};
use mint::Point3;
use clokwerk::{Scheduler, TimeUnits};
use std::collections::VecDeque;

mod factory;
mod util; 
mod system;

#[derive(Clone, PartialEq, Debug)]
pub enum GameObjectType {
    Player,
    Enemy,
    Bullet
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32
}

#[derive(Clone, PartialEq, Debug)]
pub struct GameObject {
    mesh: three::Mesh,
    object_type: GameObjectType,
    vertices: Vec<Point3<f32>>,
    velocity: f32
}

#[derive(Clone, PartialEq, Debug)]
pub struct Score {
    total: i32,
    ui: three::Text
}

#[derive(Clone, PartialEq, Debug)]
pub struct Health {
    total: i32,
    ui: three::Text
}

#[derive(Clone, PartialEq, Debug)]
pub struct AudioHistory {
    spectrum: VecDeque<Vec<f64>>,
    novelty: VecDeque<f64>,
    normalised_novelty: VecDeque<f64>,
    last_peak: f64
}

fn main() {
    let mut window_builder = three::Window::builder("INSYNC");
    window_builder.fullscreen(true); 
    let mut window = window_builder.build();

    let camera = window.factory.perspective_camera(75.0, 1.0 .. 30.0);
    camera.set_position([0.0, 0.0, 10.0]);

    let mut store = Ecs::new();
    factory::create_tunnel(&mut window);
    factory::create_player(&mut window, &mut store);
    

    let (sender, receiver): (SyncSender<Vec<Position>>, Receiver<Vec<Position>>) = sync_channel(1);
    let mut enemy_scheduler = Scheduler::new();
    
    enemy_scheduler.every(2.seconds()).run(move || {
            match sender.send(system::enemy_spawn::schedule_callback()) {
                Ok(_) => (),
                Err(err) => panic!("[enemy scheduler]: unable to schedule enemies. {:?}", err)
            }
        });

    let pa = portaudio::PortAudio::new().expect("Unable to open PortAudio"); 
    let default_mic_index = pa.default_input_device().expect("Unable to get default device"); 
    let mic = pa.device_info(default_mic_index).expect("Unable to get mic info"); 

    let input_stream_params = portaudio::StreamParameters::<f32>::new(default_mic_index, 1, true, mic.default_low_input_latency);
    let input_stream_settings = portaudio::InputStreamSettings::new(input_stream_params, mic.default_sample_rate, 256);

    let (mic_sender, mic_receiver): (Sender<&[f32]>, Receiver<&[f32]>) = channel();

    let mut stream = pa.open_non_blocking_stream(input_stream_settings, move |portaudio::InputStreamCallbackArgs {buffer, ..}| {
        //samples vs signal?? for namin variables sake what am I sending? (seems like a sample = frame of audio = signal)
        match mic_sender.send(buffer) {
            Ok(_) => portaudio::Continue,
            Err(_) => portaudio::Complete
        }
    }).expect("Unable to open stream");

    let mut audio_history = AudioHistory {
        spectrum: VecDeque::new(),
        novelty: VecDeque::new(),
        normalised_novelty: VecDeque::new(),
        last_peak: 0.0
    };

    println!("Starting audio stream...");
    stream.start().expect("Unable to start stream"); 

    while window.update() {
        match mic_receiver.try_recv() {
            Ok(samples) => system::audio_analysis::calculate_novelty_curve(samples, &mut audio_history),
            Err(_) => ()
        }

        //NOTE: spawn enemies on beat, cloud use some improvement
        //small improvements can already be made by removing dependencies/comstraints imposed on this by the enemy schedueler
        //figuering out how to pick a beat (1/4, 1/8, 1/16) would be the killer feature
        match system::audio_analysis::peak_detection(&mut audio_history) {
            None => (),
            Some(_peak) => system::enemy_spawn::run(&mut window, &mut store, &mut enemy_scheduler, &receiver)
        }

        system::input::run(&mut window, &mut store);
        system::position::run(&mut store);
        system::collision::run(&mut window, &mut store); 
        system::score::run(&mut store); 
        system::health::run(&mut store);
        system::garbage_collection::run(&mut window, &mut store);
        window.render(&camera);
    }
}


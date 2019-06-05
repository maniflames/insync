use crate::*;

pub fn calculate_novelty_curve(buffer: &[f32], history: &mut AudioHistory) {
    let samples: Vec<f64> = buffer.to_vec().into_iter().map(|sample| sample as f64).collect(); 
    //Fourier Transform, note that the output is in "nyquist bin" not "Hz"!
    let spectrum = meyda::get_amp_spectrum(&samples);

    //Log compression
    // Y = log( 1 + C * |X|) 
    let log_spectrum: Vec<f64> = spectrum.into_iter().map(|sample| {
            let to_compress = 1.0 + (1000.0 * sample);
            return to_compress.log10();
        }).collect(); 

    history.spectrum.push_front(log_spectrum); 
    
    if history.spectrum.len() < 2 {
        return
    }

    //differentiation: history[1] - history[0], negatives are dropped
    let mut differentiation: Vec<f64> = Vec::new(); 
    for (index, sample) in history.spectrum[1].iter().enumerate() {
        let difference = sample - history.spectrum[0][index]; 
        if difference >= 0.0 {
            differentiation.push(difference);
        }
    } 

    //remove unneeded history
    history.spectrum.pop_back();

    //accumulation into novelty point
    let novelty_point = differentiation.iter().fold(0.0, |sum, difference| sum + difference);
    history.novelty.push_front(novelty_point);

    if history.novelty.len() < 128 //76 
    { //novelty history length treshold
        return
    }

    let local_average = history.novelty.iter().fold(0.0, |sum, novelty_point| sum + novelty_point) / (history.novelty.len() as f64);
    let novelty_history_loop = history.novelty.clone(); //this is a memory hack and should be fixed dureing refactor
    //instead of picking peaks I just use a treshold
    let treshold = 10.0; 

    history.normalised_novelty.clear();
    for novelty_point in novelty_history_loop {
        let candidate = novelty_point - local_average;
        if candidate < treshold {
            history.normalised_novelty.push_front(0.0);
            continue;
        }

        history.normalised_novelty.push_front(candidate);
    }

    //remove unneeded history
    history.novelty.pop_back();
}
use {bevy::prelude::*, bevy_fundsp::prelude::*};
use {bevy_fundsp::dsp_graph::DspGraph};
use {fundsp::hacker::{adsr_live}};
use rand::{thread_rng, Rng};

pub fn main() {
    let app = &mut App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(DspPlugin::default());
    register_sources(app);
    app.run();
}

pub fn register_sources(app: &mut App) {
    app
        .add_dsp_source(step1, SourceType::Static { duration : 1.0 })
        .add_dsp_source(step2, SourceType::Static { duration : 1.0 })
        .add_dsp_source(step3, SourceType::Static { duration : 1.0 })
        .add_dsp_source(step4, SourceType::Static { duration : 1.0 })
        .add_system(interactive_audio);
}

// return envelope(|t| max(-(1.0 / length * t - 1.0).powi(*rougthness*2) + 1.0, 0.0))
// D# minor: D♯, E♯, F♯, G♯, A♯, B , C♯
// 5 octave: 63, 65, 66, 68, 70, 71, 73
fn step1() -> impl AudioUnit32 {
    dc(midi_hz(63.0 - 12.0*1.0)) >> sine() * 0.8  * envelope(|t| max(-(100.0 * t - 1.0).powi(40) + 1.0, 0.0)) +
    pink()         * 0.05 * envelope(|t| max(-(60.0  * t - 1.0).powi(40) + 1.0, 0.0))
}
fn step2() -> impl AudioUnit32 {
    dc(midi_hz(65.0 - 12.0*1.0)) >> sine() * 0.8  * envelope(|t| max(-(100.0 * t - 1.0).powi(40) + 1.0, 0.0)) +
    pink()         * 0.05 * envelope(|t| max(-(60.0  * t - 1.0).powi(40) + 1.0, 0.0))
}
fn step3() -> impl AudioUnit32 {
    dc(midi_hz(66.0 - 12.0*1.0)) >> sine() * 0.8  * envelope(|t| max(-(100.0 * t - 1.0).powi(40) + 1.0, 0.0)) +
    pink()         * 0.05 * envelope(|t| max(-(60.0  * t - 1.0).powi(40) + 1.0, 0.0))
}
fn step4() -> impl AudioUnit32 {
    dc(midi_hz(68.0 - 12.0*1.0)) >> sine() * 0.8  * envelope(|t| max(-(100.0 * t - 1.0).powi(40) + 1.0, 0.0)) +
    pink()         * 0.05 * envelope(|t| max(-(60.0  * t - 1.0).powi(40) + 1.0, 0.0))
}

fn stepper() -> impl DspGraph {
    let x: [fn () -> impl AudioUnit32; 4] = [step1, step2, step3, step4];
    return step4
}

fn triangle_wave() -> impl AudioUnit32 {
    // Note is G4
    sine_hz(400.0) * 0.5 * envelope(|t| -(2.0 * t - 1.0).powi(2) + 1.0)
    // triangle_hz(392.0) >> split::<U2>() * 0.2
}

fn interactive_audio(
    input: Res<Input<KeyCode>>,
    mut assets: ResMut<Assets<AudioSource>>,
    dsp_manager: Res<DspManager>,
    mut audio: ResMut<Audio>,
    mut stepArp: Local<i32>
) {
    if input.just_pressed(KeyCode::Q) {
        let src = dsp_manager
            .get_graph(stepper())
            .unwrap_or_else(|| panic!("DSP source not found!"));

        audio.play_dsp(assets.as_mut(), src);
    }

}
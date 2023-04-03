use {bevy::prelude::*, bevy_fundsp::prelude::*};
use bevy_fundsp::dsp_graph::DspGraph;
use bevy::utils::Uuid;

pub fn main() {
    let app = &mut App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(DspPlugin::default());
    register_sources(app);
    app.run();
}

pub fn register_sources(app: &mut App) {
    let stepper = Stepper::new();
    app
        .insert_resource(stepper.clone())
        .add_dsp_source(stepper, SourceType::Static { duration : 1.0 })
        .add_system(interactive_audio);
}

fn triangle_wave() -> impl AudioUnit32 {
    // Note is G4
    sine_hz(400.0) * 0.5 * envelope(|t| -(2.0 * t - 1.0).powi(2) + 1.0)
    // triangle_hz(392.0) >> split::<U2>() * 0.2
}

// return envelope(|t| max(-(1.0 / length * t - 1.0).powi(*rougthness*2) + 1.0, 0.0))
// D# minor: D♯, E♯, F♯, G♯, A♯, B , C♯
// 5 octave: 63, 65, 66, 68, 70, 71, 73
const NOTES: &[f32] = &[
    63.0 - 12.0*1.0,
    65.0 - 12.0*1.0,
    66.0 - 12.0*1.0,
    68.0 - 12.0*1.0,
];

#[derive(Resource, Clone)]
struct Stepper {
    id: Uuid,
    note: Shared<f32>,
    step: usize,
}

impl Stepper {
    pub fn new() -> Self {
        let id = Uuid::default();
        let note = Shared::new(NOTES[0]);
        let step = 0;
        let mut stepper = Stepper {
            id, note, step
        };
        stepper.id = stepper.graph().id();
        stepper
    }
    fn graph(&self) -> impl DspGraph {
        let note = self.note.clone();
        move || {
            let value = note.value();
            dc(midi_hz(value)) >> sine() * 0.8  * envelope(|t| max(-(100.0 * t - 1.0).powi(40) + 1.0, 0.0)) +
            pink()         * 0.05 * envelope(|t| max(-(60.0  * t - 1.0).powi(40) + 1.0, 0.0))
        }
    }
    pub fn step(&mut self) {
        self.step += 1;
        self.note.set_value(NOTES[self.step % 4])
    }
}

impl DspGraph for Stepper {
    fn id(&self) -> Uuid {
        self.id
    }
    fn generate_graph(&self) -> Box<dyn AudioUnit32> {
        self.graph().generate_graph()
    }
}


fn interactive_audio(
    input: Res<Input<KeyCode>>,
    mut assets: ResMut<Assets<AudioSource>>,
    dsp_manager: Res<DspManager>,
    mut audio: ResMut<Audio>,
    mut stepper: ResMut<Stepper>
) {
    // if note.get_shared().as_ref().ge == &0.0 {
    //     note.set_value(63.0 - 12.0*1.0)
    // }
    if input.just_pressed(KeyCode::Q) {
        // let note_value = NOTES.get(*note_idx).unwrap();
        // *note_idx = (*note_idx + 1) % 4;
        // note.set_value(*note_value);
        let src = dsp_manager
            .get_graph(stepper.clone())
            .unwrap_or_else(|| panic!("DSP source not found!"));

        audio.play_dsp(assets.as_mut(), src);
        stepper.step();
    }

}
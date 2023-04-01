use bevy::{prelude::*, utils::HashMap};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(wait_audio_loaded)
        .add_system(play_audio)
        .run();
}


#[derive(Resource)]
pub struct AudioLib {
    playback: HashMap<&'static str, Handle<AudioSink>>,
    loaded: bool,
}

pub fn setup(
    mut commands: Commands,
    audio: Res<Audio>,
    sinks: Res<Assets<AudioSink>>,
    asset_server: Res<AssetServer>
) {
    let samples = &[
        "bass/bass1",
        "bass/bass2"
    ];
    let mut playback = HashMap::new();
    for sample in samples {
        let path = format!("audio/{sample}.wav");
        let audio_handle = asset_server.load(path);
        let sink_handle = audio.play_with_settings(audio_handle.clone(), PlaybackSettings { repeat: true, ..default() });
        let sink_handle = sinks.get_handle(sink_handle);
        playback.insert(*sample, sink_handle);
    }

    commands.insert_resource(AudioLib {
        playback,
        loaded: false
    });
}

pub fn wait_audio_loaded(
    mut lib: ResMut<AudioLib>,
    mut sinks: ResMut<Assets<AudioSink>>
) {
    if lib.loaded {
        return;
    }
    let loaded_samples = lib.playback
        .values()
        .filter(|p| sinks.get_mut(&p).map(|p| p.pause()).is_some() )
        .count();
    if loaded_samples == lib.playback.len() {
        info!("{loaded_samples} samples loaded!");
        lib.loaded = true;
    }
}

pub fn play_audio(
    lib: Res<AudioLib>,
    sinks: Res<Assets<AudioSink>>,
    time: Res<Time>,
    mut sample: Local<&'static str>,
    mut toggle_at: Local<f32>
) {
    if !lib.loaded {
        return;
    }
    if *toggle_at == 0. {
        *toggle_at = time.elapsed_seconds();
    }
    if sample.is_empty() {
        *sample = "bass/bass1";
    }
    if *toggle_at <= time.elapsed_seconds() {
        if *sample == "bass/bass1" {
            *sample = "bass/bass2"
        } else {
            *sample = "bass/bass1"
        }
        lib.playback.values().for_each(|p| {
            sinks.get(&p).unwrap().pause();
        });
        let next = &lib.playback.get(*sample).unwrap();
        sinks.get(next).unwrap().play();
        *toggle_at += 8.;
    }
}
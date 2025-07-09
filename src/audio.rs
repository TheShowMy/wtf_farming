use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Music>();
    app.register_type::<SoundEffect>();

    app.add_systems(
        Update,
        apply_global_volume.run_if(resource_changed::<GlobalVolume>),
    );
}

/// 一个组织标记组件，如果它属于一般的“音乐”类别（例如全局背景音乐、原声带），
/// 应该添加到生成的 [`AudioPlayer`]。
///
/// 然后可以用它来查询和操作该类别中的声音。
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

/// 一个音乐音频实例。
pub fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::LOOP, Music)
}

/// 一个组织标记组件，如果它属于一般的“音效”类别（例如脚步声、魔法咒语的声音、门打开的声音），
/// 应该添加到生成的 [`AudioPlayer`]。
///
/// 然后可以用它来查询和操作该类别中的声音。
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SoundEffect;

/// 一个音效音频实例。
pub fn sound_effect(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::DESPAWN, SoundEffect)
}

/// [`GlobalVolume`] 不适用于已经运行的音频实体，因此此系统将更新它们。
fn apply_global_volume(
    global_volume: Res<GlobalVolume>,
    mut audio_query: Query<(&PlaybackSettings, &mut AudioSink)>,
) {
    for (playback, mut sink) in &mut audio_query {
        sink.set_volume(global_volume.volume * playback.volume);
    }
}

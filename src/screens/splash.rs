//! 启动画面，在启动时短暂播放。

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    input::common_conditions::input_just_pressed,
    prelude::*,
};

use crate::{AppSystems, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    // 生成启动画面。
    app.insert_resource(ClearColor(SPLASH_BACKGROUND_COLOR));
    app.add_systems(OnEnter(Screen::Splash), spawn_splash_screen);

    // 动画化启动画面。
    app.add_systems(
        Update,
        (
            tick_fade_in_out.in_set(AppSystems::TickTimers),
            apply_fade_in_out.in_set(AppSystems::Update),
        )
            .run_if(in_state(Screen::Splash)),
    );

    // 添加启动计时器。
    app.register_type::<SplashTimer>();
    app.add_systems(OnEnter(Screen::Splash), insert_splash_timer);
    app.add_systems(OnExit(Screen::Splash), remove_splash_timer);
    app.add_systems(
        Update,
        (
            tick_splash_timer.in_set(AppSystems::TickTimers),
            check_splash_timer.in_set(AppSystems::Update),
        )
            .run_if(in_state(Screen::Splash)),
    );

    // 如果玩家按下 Escape 键，提前退出启动画面。
    app.add_systems(
        Update,
        enter_title_screen
            .run_if(input_just_pressed(KeyCode::Escape).and(in_state(Screen::Splash))),
    );
}

const SPLASH_BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);
const SPLASH_DURATION_SECS: f32 = 1.8;
const SPLASH_FADE_DURATION_SECS: f32 = 0.6;

fn spawn_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        widget::ui_root("Splash Screen"),
        BackgroundColor(SPLASH_BACKGROUND_COLOR),
        StateScoped(Screen::Splash),
        children![(
            Name::new("Splash image"),
            Node {
                margin: UiRect::all(Val::Auto),
                width: Val::Percent(70.0),
                ..default()
            },
            ImageNode::new(asset_server.load_with_settings(
                // 这应该是一个嵌入式资源以实现即时加载，但
                // 当前 [在 Windows Wasm 构建中存在问题](https://github.com/bevyengine/bevy/issues/14246)。
                "images/splash.png",
                |settings: &mut ImageLoaderSettings| {
                    // 为启动画面图像做一个例外，以防
                    // 使用 `ImagePlugin::default_nearest()` 来处理像素艺术。
                    settings.sampler = ImageSampler::linear();
                },
            )),
            ImageNodeFadeInOut {
                total_duration: SPLASH_DURATION_SECS,
                fade_duration: SPLASH_FADE_DURATION_SECS,
                t: 0.0,
            },
        )],
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ImageNodeFadeInOut {
    /// 总持续时间（以秒为单位）。
    total_duration: f32,
    /// 淡入淡出持续时间（以秒为单位）。
    fade_duration: f32,
    /// 当前进度（以秒为单位），介于 0 和 [`Self::total_duration`] 之间。
    t: f32,
}

impl ImageNodeFadeInOut {
    fn alpha(&self) -> f32 {
        // 按持续时间归一化。
        let t = (self.t / self.total_duration).clamp(0.0, 1.0);
        let fade = self.fade_duration / self.total_duration;

        // 常规梯形图形，顶部平坦，alpha = 1.0。
        ((1.0 - (2.0 * t - 1.0).abs()) / fade).min(1.0)
    }
}

fn tick_fade_in_out(time: Res<Time>, mut animation_query: Query<&mut ImageNodeFadeInOut>) {
    for mut anim in &mut animation_query {
        anim.t += time.delta_secs();
    }
}

fn apply_fade_in_out(mut animation_query: Query<(&ImageNodeFadeInOut, &mut ImageNode)>) {
    for (anim, mut image) in &mut animation_query {
        image.color.set_alpha(anim.alpha())
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Reflect)]
#[reflect(Resource)]
struct SplashTimer(Timer);

impl Default for SplashTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SPLASH_DURATION_SECS, TimerMode::Once))
    }
}

fn insert_splash_timer(mut commands: Commands) {
    commands.init_resource::<SplashTimer>();
}

fn remove_splash_timer(mut commands: Commands) {
    commands.remove_resource::<SplashTimer>();
}

fn tick_splash_timer(time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    timer.0.tick(time.delta());
}

fn check_splash_timer(timer: ResMut<SplashTimer>, mut next_screen: ResMut<NextState<Screen>>) {
    if timer.0.just_finished() {
        next_screen.set(Screen::Title);
    }
}

fn enter_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

//! 玩家精灵动画。
//! 这是基于多个示例的实现，可能与您的游戏非常不同。
//! - [精灵翻转](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [精灵动画](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [计时器](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::{
    AppSystems, PausableSystems,
    audio::sound_effect,
    games::{movement::MovementController, player::PlayerAssets},
};

pub(super) fn plugin(app: &mut App) {
    // 根据控制进行动画和播放音效。
    app.register_type::<PlayerAnimation>();
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSystems::TickTimers),
            (
                update_animation_movement,
                update_animation_atlas,
                trigger_step_sound_effect,
            )
                .chain()
                .run_if(resource_exists::<PlayerAssets>)
                .in_set(AppSystems::Update),
        )
            .in_set(PausableSystems),
    );
}

/// 更新精灵方向和动画状态（静止/行走）。
fn update_animation_movement(
    mut player_query: Query<(&MovementController, &mut Sprite, &mut PlayerAnimation)>,
) {
    for (controller, mut sprite, mut animation) in &mut player_query {
        let dx = controller.intent.x;
        if dx != 0.0 {
            sprite.flip_x = dx < 0.0;
        }

        let animation_state = if controller.intent == Vec2::ZERO {
            PlayerAnimationState::Idling
        } else {
            PlayerAnimationState::Walking
        };
        animation.update_state(animation_state);
    }
}

/// 更新动画计时器。
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut PlayerAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// 更新纹理图集以反映动画的变化。
fn update_animation_atlas(mut query: Query<(&PlayerAnimation, &mut Sprite)>) {
    for (animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

/// 如果玩家在移动，则播放与动画同步的脚步声效果。
fn trigger_step_sound_effect(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut step_query: Query<&PlayerAnimation>,
) {
    for animation in &mut step_query {
        if animation.state == PlayerAnimationState::Walking
            && animation.changed()
            && (animation.frame == 2 || animation.frame == 5)
        {
            let rng = &mut rand::thread_rng();
            let random_step = player_assets.steps.choose(rng).unwrap().clone();
            commands.spawn(sound_effect(random_step));
        }
    }
}

/// 组件，用于跟踪玩家的动画状态。
/// 它与我们使用的纹理图集紧密绑定。
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    frame: usize,
    state: PlayerAnimationState,
}

#[derive(Reflect, PartialEq)]
pub enum PlayerAnimationState {
    Idling,
    Walking,
}

impl PlayerAnimation {
    /// 静止帧的数量。
    const IDLE_FRAMES: usize = 2;
    /// 每帧静止的持续时间。
    const IDLE_INTERVAL: Duration = Duration::from_millis(500);
    /// 行走帧的数量。
    const WALKING_FRAMES: usize = 6;
    /// 每帧行走的持续时间。
    const WALKING_INTERVAL: Duration = Duration::from_millis(50);

    fn idling() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Idling,
        }
    }

    fn walking() -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Walking,
        }
    }

    pub fn new() -> Self {
        Self::idling()
    }

    /// 更新动画计时器。
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match self.state {
                PlayerAnimationState::Idling => Self::IDLE_FRAMES,
                PlayerAnimationState::Walking => Self::WALKING_FRAMES,
            };
    }

    /// 如果动画状态发生变化，则更新动画状态。
    pub fn update_state(&mut self, state: PlayerAnimationState) {
        if self.state != state {
            match state {
                PlayerAnimationState::Idling => *self = Self::idling(),
                PlayerAnimationState::Walking => *self = Self::walking(),
            }
        }
    }

    /// 动画是否在此刻发生了变化。
    pub fn changed(&self) -> bool {
        self.timer.is_finished()
    }

    /// 返回图集中精灵的索引。
    pub fn get_atlas_index(&self) -> usize {
        match self.state {
            PlayerAnimationState::Idling => self.frame,
            PlayerAnimationState::Walking => 6 + self.frame,
        }
    }
}

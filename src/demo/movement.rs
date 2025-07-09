//! 处理玩家输入并通过角色控制器将其转换为移动。
//! 角色控制器是管理角色移动的系统集合。
//!
//! 在我们的案例中，角色控制器具有以下逻辑：
//! - 根据方向键输入设置 [`MovementController`] 的意图。
//!   这是在 `player` 模块中完成的，因为它特定于玩家角色。
//! - 根据 [`MovementController`] 的意图和最大速度应用移动。
//! - 在窗口内包裹角色。
//!
//! 请注意，此处使用的实现仅用于演示目的。如果您希望以更平滑的方式移动玩家，
//! 可以考虑使用 [固定时间步长](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs)。

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementController>();
    app.register_type::<ScreenWrap>();

    app.add_systems(
        Update,
        (apply_movement, apply_screen_wrap)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

/// 这些是我们角色控制器的移动参数。
/// 目前，这仅用于单个玩家，但它也可以为 NPC 或其他玩家提供动力。
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    /// 角色想要移动的方向。
    pub intent: Vec2,

    /// 每秒的最大速度（以世界单位为单位）。
    /// 1 个世界单位 = 使用默认 2D 摄像机且无物理引擎时的 1 个像素。
    pub max_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            // 每秒 400 像素是一个不错的默认值，但我们仍然可以根据角色的不同而有所变化。
            max_speed: 400.0,
        }
    }
}

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &mut Transform)>,
) {
    for (controller, mut transform) in &mut movement_query {
        let velocity = controller.max_speed * controller.intent;
        transform.translation += velocity.extend(0.0) * time.delta_secs();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScreenWrap;

/// 应用屏幕包裹逻辑。
fn apply_screen_wrap(
    window: Single<&Window, With<PrimaryWindow>>,
    mut wrap_query: Query<&mut Transform, With<ScreenWrap>>,
) {
    let size = window.size() + 256.0;
    let half_size = size / 2.0;
    for mut transform in &mut wrap_query {
        let position = transform.translation.xy();
        let wrapped = (position + half_size).rem_euclid(size) - half_size;
        transform.translation = wrapped.extend(transform.translation.z);
    }
}

//! 游戏的开发工具。此插件仅在开发构建中启用。

use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
    ui::UiDebugOptions,
};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    // 记录 `Screen` 状态的转换。
    app.add_systems(Update, log_transitions::<Screen>);

    // 切换 UI 的调试覆盖层。
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

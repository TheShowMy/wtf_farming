//! 演示游戏玩法。这些模块仅用于演示
//! 目的，并应替换为您自己的游戏逻辑。
//! 如果您想尝试修改，请随意更改此处的逻辑
//! 以熟悉模板。

use bevy::prelude::*;

mod animation;
pub mod level;
mod movement;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
    ));
}

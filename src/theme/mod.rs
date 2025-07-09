//! 可重用的 UI 小部件和主题。

// 未使用的实用程序可能会错误地触发此 lint。
#![allow(dead_code)]

pub mod interaction;
pub mod palette;
pub mod widget;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::{interaction::InteractionPalette, palette as ui_palette, widget};
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}

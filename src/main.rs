// 支持在代码中配置 Bevy 的 lint。
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// 在非开发构建中禁用 Windows 控制台。
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod asset_tracking;
mod audio;
#[cfg(feature = "dev")]
mod dev_tools;
mod games;
mod i18n;
mod menus;
mod screens;
mod theme;

use bevy::{asset::AssetMetaCheck, prelude::*};

use crate::asset_tracking::LoadResource;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // 添加 Bevy 插件。
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // 如果未设置此项，Wasm 构建将检查元文件（这些文件不存在）。
                    // 这会导致 itch 上的 Web 构建出现错误甚至崩溃。
                    // 参见 https://github.com/bevyengine/bevy_github_ci_template/issues/48。
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Wtf Farming".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );

        // 添加其他插件。
        app.add_plugins((
            i18n::plugin,
            asset_tracking::plugin,
            audio::plugin,
            games::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
        ));

        // 注册 FntAssets 资源。全局字体资源。
        app.register_type::<FntAssets>();
        app.load_resource::<FntAssets>();

        // 通过在此处添加新 AppSystems 变体来排序：
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // 设置 Pause 状态。
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

        // 生成主摄像机。
        app.add_systems(Startup, spawn_camera);
    }
}

/// 在 Update 调度中对应用程序的系统进行高级分组。
/// 添加新变体时，请确保在 configure_sets 中对其进行排序。
/// 上述调用。
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// 计时器滴答。
    TickTimers,
    /// 记录玩家输入。
    RecordInput,
    /// 执行其他所有操作（考虑将其拆分为更多变体）。
    Update,
}

/// 游戏是否暂停。
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
struct Pause(pub bool);

/// 一个系统集，用于在游戏暂停时不应运行的系统。
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct FntAssets {
    #[dependency]
    pixel: Handle<Font>,
}

impl FromWorld for FntAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            pixel: assets.load("fonts/fusion-pixel.ttf"),
        }
    }
}

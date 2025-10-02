//! 致谢菜单。

use bevy::{
    ecs::spawn::SpawnIter, input::common_conditions::input_just_pressed, prelude::*, ui::Val::*,
};

use crate::{
    FntAssets,
    asset_tracking::LoadResource,
    audio::music,
    i18n::{LanguageRes, config::BACK},
    menus::Menu,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
    );

    app.register_type::<CreditsAssets>();
    app.load_resource::<CreditsAssets>();
    app.add_systems(OnEnter(Menu::Credits), start_credits_music);
}

fn spawn_credits_menu(
    mut commands: Commands,
    font_res: Res<FntAssets>,
    lang_res: Res<LanguageRes>,
) {
    commands.spawn((
        widget::ui_root("Credits Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Credits),
        children![
            widget::header("Created by", font_res.pixel()),
            created_by(font_res.pixel()),
            widget::header("Assets", font_res.pixel()),
            assets(font_res.pixel()),
            widget::button(lang_res.get(BACK), font_res.pixel(), go_back_on_click),
        ],
    ));
}

fn created_by(font: Handle<Font>) -> impl Bundle {
    grid(
        vec![
            ["Joe Shmoe", "Implemented alligator wrestling AI"],
            ["Jane Doe", "Made the music for the alien invasion"],
        ],
        font,
    )
}

fn assets(font: Handle<Font>) -> impl Bundle {
    grid(
        vec![
            ["Ducky sprite", "CC0 by Caz Creates Games"],
            ["Button SFX", "CC0 by Jaszunio15"],
            ["Music", "CC BY 3.0 by Kevin MacLeod"],
            [
                "Bevy logo",
                "All rights reserved by the Bevy Foundation, permission granted for splash screen use when unmodified",
            ],
        ],
        font,
    )
}

fn grid(content: Vec<[&'static str; 2]>, font: Handle<Font>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flatten().enumerate().map(
            move |(i, text)| {
                (
                    widget::label(text, font.clone()),
                    Node {
                        justify_self: if i % 2 == 0 {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    },
                )
            },
        ))),
    )
}

fn go_back_on_click(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct CreditsAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for CreditsAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Monkeys Spinning Monkeys.ogg"),
        }
    }
}

fn start_credits_music(mut commands: Commands, credits_music: Res<CreditsAssets>) {
    commands.spawn((
        Name::new("Credits Music"),
        DespawnOnExit(Menu::Credits),
        music(credits_music.music.clone()),
    ));
}

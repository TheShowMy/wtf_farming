//! 创建常见小部件的辅助函数。

use std::borrow::Cow;

use bevy::{
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
    ui::Val::*,
};

use crate::theme::{interaction::InteractionPalette, palette::*};

/// 一个填充窗口并将其内容居中的根 UI 节点。
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Px(20.0),
            ..default()
        },
        // 不阻止其他 UI 根的拾取事件。
        Pickable::IGNORE,
    )
}

/// 一个简单的标题标签。比 [`label`] 更大。
pub fn header(text: impl Into<String>, font: Handle<Font>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont::from_font_size(40.0).with_font(font),
        TextColor(HEADER_TEXT),
    )
}

/// 一个简单的文本标签。
pub fn label(text: impl Into<String>, font: Handle<Font>) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into()),
        TextFont::from_font_size(24.0).with_font(font),
        TextColor(LABEL_TEXT),
    )
}

/// 一个带有文本和由 [`Observer`] 定义的操作的大型圆角按钮。
pub fn button<E, B, M, I>(text: impl Into<String>, font: Handle<Font>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        font,
        action,
        (
            Node {
                width: Px(380.0),
                height: Px(80.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::MAX,
        ),
    )
}

/// 一个带有文本和由 [`Observer`] 定义的操作的小型方形按钮。
pub fn button_small<E, B, M, I>(
    text: impl Into<String>,
    font: Handle<Font>,
    action: I,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        font,
        action,
        Node {
            width: Px(30.0),
            height: Px(30.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    )
}

/// 一个带有文本和由 [`Observer`] 定义的操作的简单按钮。按钮的布局由 `button_bundle` 提供。
fn button_base<E, B, M, I>(
    text: impl Into<String>,
    font: Handle<Font>,
    action: I,
    button_bundle: impl Bundle,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("Button"),
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Button Inner"),
                    Button,
                    BackgroundColor(BUTTON_BACKGROUND),
                    InteractionPalette {
                        none: BUTTON_BACKGROUND,
                        hovered: BUTTON_HOVERED_BACKGROUND,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    children![(
                        Name::new("Button Text"),
                        Text(text),
                        TextFont {
                            font,
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(BUTTON_TEXT),
                        // 不将文本的拾取事件冒泡到按钮。
                        Pickable::IGNORE,
                    )],
                ))
                .insert(button_bundle)
                .observe(action);
        })),
    )
}

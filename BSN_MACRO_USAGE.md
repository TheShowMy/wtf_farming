# BSN 宏使用说明 (BSN Macro Usage Documentation)

## 概述 (Overview)

本仓库使用了 Bevy 游戏引擎的 `children!` 宏来简化 UI 组件的层级结构创建。

## 配置 (Configuration)

### Clippy 配置

在 `clippy.toml` 文件中配置了标准宏括号规范 (standard-macro-braces)：

```toml
# Require `bevy_ecs::children!` to use `[]` braces, instead of `()` or `{}`.
standard-macro-braces = [{ name = "children", brace = "[" }]
```

这个配置要求 `children!` 宏必须使用方括号 `[]` 而不是圆括号 `()` 或花括号 `{}`。

## 宏说明 (Macro Description)

### `children!` 宏

`children!` 宏来自 `bevy_ecs` 包，用于在创建 UI 实体时声明子元素。

**标准用法：**
```rust
children![
    (component1, component2, ...),
    (component3, component4, ...),
]
```

## 使用统计 (Usage Statistics)

本仓库中共有 **19 处**使用了 `children!` 宏。

## 使用位置 (Usage Locations)

### 游戏模块 (Games Module)
1. `src/games/level.rs:45` - 关卡 UI 组件

### 主题模块 (Theme Module)
2. `src/theme/widget.rs:169` - 按钮组件

### 屏幕模块 (Screens Module)
3. `src/screens/splash.rs:56` - 启动画面
4. `src/screens/loading.rs:32` - 加载画面

### 菜单模块 (Menus Module)
5. `src/menus/pause.rs:29` - 暂停菜单
6. `src/menus/credits.rs:37` - 制作人员名单
7. `src/menus/main.rs:38` - 主菜单
8. `src/menus/main.rs:58` - 主菜单（嵌套使用）
9. `src/menus/main.rs:70` - 主菜单（嵌套使用）
10. `src/menus/main.rs:89` - 主菜单（嵌套使用）
11. `src/menus/main.rs:114` - 主菜单（嵌套使用）
12. `src/menus/main.rs:136` - 主菜单（嵌套使用）
13. `src/menus/settings.rs:41` - 设置菜单
14. `src/menus/settings.rs:59` - 设置菜单（嵌套使用）
15. `src/menus/settings.rs:79` - 设置菜单（嵌套使用）
16. `src/menus/settings.rs:88` - 设置菜单（嵌套使用）
17. `src/menus/select_game.rs:59` - 游戏选择菜单
18. `src/menus/select_game.rs:90` - 游戏选择菜单（嵌套使用）
19. `src/menus/select_game.rs:114` - 游戏选择菜单（嵌套使用）

## 代码示例 (Code Examples)

### 示例 1: 简单的标签组件
```rust
// 来自 src/screens/loading.rs:32
children![widget::label("Loading...", font_res.pixel())]
```

### 示例 2: 按钮组件
```rust
// 来自 src/theme/widget.rs:169
children![(
    Name::new("Button Text"),
    Text(text),
    TextFont {
        font,
        font_size: 40.0,
        ..default()
    },
    TextColor(BUTTON_TEXT),
    Pickable::IGNORE,
)]
```

### 示例 3: 嵌套使用
```rust
// 来自 src/menus/main.rs（简化版本）
children![
    widget::container((
        // 外层容器
        children![
            // 内层子组件
            widget::button("Button 1", font.clone()),
            widget::button("Button 2", font.clone()),
        ]
    ))
]
```

## 最佳实践 (Best Practices)

1. **始终使用方括号** `[]`：遵循 clippy 配置，使用 `children![...]` 而不是 `children!(...)`
2. **合理嵌套**：在需要时可以嵌套使用 `children!` 宏来构建复杂的 UI 层级
3. **组件元组**：每个子元素通常是一个包含多个组件的元组 `(Component1, Component2, ...)`

## 相关资源 (Related Resources)

- [Bevy 官方文档](https://bevyengine.org/)
- [bevy_ecs 文档](https://docs.rs/bevy_ecs/latest/bevy_ecs/) - `children!` 宏的详细文档
- [Bevy UI 示例](https://bevyengine.org/examples/UI/)
- [Clippy 配置文档](https://doc.rust-lang.org/clippy/configuration.html)

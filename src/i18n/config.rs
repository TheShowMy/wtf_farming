//!多语言文本配置

use crate::i18n::LanguageRes;

/// 主菜单开始游戏
pub const MAIN_PLAY: &str = "MAIN_PLAY";
/// 主菜单设置
pub const MAIN_SETTINGS: &str = "MAIN_SETTINGS";
/// 主菜单关于
pub const MAIN_CREDITS: &str = "MAIN_CREDITS";
/// 主菜单退出
pub const MAIN_EXIT: &str = "MAIN_EXIT";
/// 暂停界面标题
pub const PAUSE_GAME_TITLE: &str = "PAUSE_GAME_TITLE";
/// 暂停界面继续游戏
pub const PAUSE_CONTINUE: &str = "PAUSE_CONTINUE";
/// 暂停界面回到主菜单
pub const PAUSE_QUIT_TO_TITLE: &str = "PAUSE_QUIT_TO_TITLE";
/// 设置界面标题
pub const SETTINGS_TITLE: &str = "SETTINGS_TITLE";
/// 设置界面主音量
pub const SETTINGS_MASTER_VOLUME: &str = "MASTER_VOLUME";
/// 设置界面当前音量
pub const SETTINGS_CURRENT_VOLUME: &str = "CURRENT_VOLUME";
/// 选择游戏界面标题
pub const SELECT_GAME_TITLE: &str = "SELECT_GAME_TITLE";

// 全局文本
/// 返回
pub const BACK: &str = "BACK";
pub const GAME_TITLE: &str = "GAME_TITLE";
pub const GAMES_NAME_1: &str = "GAMES_NAME_1";
pub const GAMES_DESCRIPTION_1: &str = "GAMES_DESCRIPTION_1";

pub(crate) fn init_language_res() -> LanguageRes {
    let mut language_res = LanguageRes::default();
    language_res.zh_cn(MAIN_PLAY, "开始游戏");
    language_res.en_us(MAIN_PLAY, "PLAY");

    language_res.zh_cn(MAIN_SETTINGS, "设置");
    language_res.en_us(MAIN_SETTINGS, "SETTINGS");

    language_res.zh_cn(MAIN_CREDITS, "关于");
    language_res.en_us(MAIN_CREDITS, "CREDITS");

    language_res.zh_cn(MAIN_EXIT, "退出");
    language_res.en_us(MAIN_EXIT, "EXIT");

    language_res.zh_cn(PAUSE_GAME_TITLE, "游戏已暂停");
    language_res.en_us(PAUSE_GAME_TITLE, "Game Paused");

    language_res.zh_cn(PAUSE_CONTINUE, "继续游戏");
    language_res.en_us(PAUSE_CONTINUE, "CONTINUE");

    language_res.zh_cn(PAUSE_QUIT_TO_TITLE, "回到主菜单");
    language_res.en_us(PAUSE_QUIT_TO_TITLE, "QUIT TO TITLE");

    language_res.zh_cn(SETTINGS_TITLE, "设置");
    language_res.en_us(SETTINGS_TITLE, "SETTINGS");

    language_res.zh_cn(SETTINGS_MASTER_VOLUME, "主音量");
    language_res.en_us(SETTINGS_MASTER_VOLUME, "Master Volume");

    language_res.zh_cn(SETTINGS_CURRENT_VOLUME, "当前音量");
    language_res.en_us(SETTINGS_CURRENT_VOLUME, "Current Volume");

    language_res.zh_cn(BACK, "返回");
    language_res.en_us(BACK, "BACK");

    language_res.zh_cn(GAME_TITLE, "种个锤子地");
    language_res.en_us(GAME_TITLE, "WTF Farming");

    language_res.zh_cn(GAMES_NAME_1, "保护那块地!!");
    language_res.en_us(GAMES_NAME_1, "Protect the Land!!");

    language_res.zh_cn(
        GAMES_DESCRIPTION_1,
        "一个简单的塔防游戏，保护你的土地免受敌人的侵袭。",
    );
    language_res.en_us(
        GAMES_DESCRIPTION_1,
        "A simple tower defense game to protect your land from enemies.",
    );

    language_res.zh_cn(SELECT_GAME_TITLE, "选择游戏");
    language_res.en_us(SELECT_GAME_TITLE, "Select Game");

    language_res
}

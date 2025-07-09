//! 多语言支持插件

use core::fmt;

use bevy::{platform::collections::HashMap, prelude::*};

pub mod config;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource::<LanguageRes>(config::init_language_res());
}

/// 多语言Id
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum LanguageId {
    #[default]
    ZhCn, // 简体中文
    EnUs, // 英语
}

/// 实现 Display trait 以便于打印 和显示对应的语言名称
impl fmt::Display for LanguageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageId::ZhCn => write!(f, "简体中文"),
            LanguageId::EnUs => write!(f, "English"),
        }
    }
}

/// 多语言配置
#[derive(Resource)]
pub struct LanguageRes {
    pub curr_language: LanguageId,
    pub language_list: Vec<LanguageId>,
    pub language_hash: HashMap<LanguageId, HashMap<String, String>>,
}

impl Default for LanguageRes {
    fn default() -> Self {
        Self {
            curr_language: Default::default(),
            language_list: vec![LanguageId::ZhCn, LanguageId::EnUs],
            language_hash: HashMap::new(),
        }
    }
}

impl LanguageRes {
    pub fn zh_cn(&mut self, key: &str, value: &str) {
        let language = LanguageId::ZhCn;
        self.add(language, key, value);
    }

    pub fn en_us(&mut self, key: &str, value: &str) {
        let language = LanguageId::EnUs;
        self.add(language, key, value);
    }

    pub fn add(&mut self, language: LanguageId, key: &str, value: &str) {
        if !self.language_list.contains(&language) {
            self.language_list.push(language);
        }
        let map = self.language_hash.entry(language).or_default();
        map.insert(key.into(), value.into());
    }

    /// 设置当前语言对应的文本
    pub fn get(&self, key: &str) -> String {
        self.language_hash
            .get(&self.curr_language)
            .and_then(|map| map.get(key))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }
}

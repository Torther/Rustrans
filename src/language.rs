// 语种检测模块

use whichlang::{detect_language as detect_lang, Lang};

/// 检测文本的语种
pub fn detect_language(text: &str) -> String {
    let detected = detect_lang(text);

    match detected {
        Lang::Cmn => "中文(简体)".to_string(),
        Lang::Jpn => "日语".to_string(),
        Lang::Kor => "韩语".to_string(),
        Lang::Rus => "俄语".to_string(),
        Lang::Spa => "西班牙语".to_string(),
        Lang::Fra => "法语".to_string(),
        Lang::Deu => "德语".to_string(),
        Lang::Ara => "阿拉伯语".to_string(),
        Lang::Por => "葡萄牙语".to_string(),
        Lang::Ita => "意大利语".to_string(),
        Lang::Vie => "越南语".to_string(),
        Lang::Eng => "英语".to_string(),
        _ => "英语".to_string(),
    }
}

/// 选择目标语种
///
/// 如果检测到的语种与首要目标语种相同，则使用次要目标语种
pub fn select_target_language(
    text: &str,
    destination: &[String],
    source: Option<&str>,
) -> (String, String) {
    let detected_lang = source.unwrap_or(&detect_language(text)).to_string();

    let target_lang = if destination.is_empty() {
        "英语".to_string()
    } else if destination[0] == detected_lang && destination.len() > 1 {
        destination[1].clone()
    } else {
        destination[0].clone()
    };

    (detected_lang, target_lang)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_chinese() {
        assert_eq!(detect_language("你好世界"), "中文(简体)");
    }

    #[test]
    fn test_detect_english() {
        assert_eq!(detect_language("Hello World"), "英语");
    }

    #[test]
    fn test_select_target_language() {
        let (from, to) = select_target_language(
            "你好",
            &["中文(简体)".to_string(), "英语".to_string()],
            None,
        );
        assert_eq!(from, "中文(简体)");
        assert_eq!(to, "英语");
    }
}

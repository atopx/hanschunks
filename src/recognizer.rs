use std::collections::HashMap;

use regex::Regex;

use crate::element::ElementType;

/// 元素识别器接口
pub trait ElementRecognizer: Send + Sync {
    fn recognize(&self, line: &str) -> ElementType;
}

/// 默认元素识别器
pub struct DefaultRecognizer {
    patterns: HashMap<&'static str, Vec<Regex>>,
}

impl DefaultRecognizer {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // 标题模式
        patterns.insert(
            "heading1",
            vec![
                Regex::new(r"^第[一二三四五六七八九十百千]+[章节]").unwrap(),
                Regex::new(r"^[一二三四五六七八九十]+[、.]").unwrap(),
            ],
        );
        patterns.insert(
            "heading2",
            vec![
                Regex::new(r"^[\(（][一二三四五六七八九十]+[\)）]").unwrap(),
                Regex::new(r"^[\(（][一二三四五六七八九十]+[\)）]").unwrap(),
            ],
        );
        patterns.insert(
            "heading3",
            vec![
                Regex::new(r"^\d+(\.\d+)*\s*[\u4e00-\u9fff]{0,30}$").unwrap(),
                Regex::new(r"^[\(（]?\d+[\)）]").unwrap(),
            ],
        );

        // 列表模式
        patterns.insert(
            "list",
            vec![
                Regex::new(r"^[\s]*[•\-\*]\s+").unwrap(),
                Regex::new(r"^[一二三四五六七八九十]+[、：:]").unwrap(),
                Regex::new(r"^\d+[、：:\.]").unwrap(),
            ],
        );

        // 其他模式
        patterns.insert("code", vec![Regex::new(r"^```").unwrap()]);
        patterns.insert("quote", vec![Regex::new(r"^[\s]*>").unwrap()]);
        patterns.insert("footer", vec![Regex::new(r"第\s*\d+\s*页").unwrap()]);

        Self { patterns }
    }

    fn check_patterns(&self, line: &str, pattern_key: &str) -> bool {
        self.patterns.get(pattern_key).map(|regexes| regexes.iter().any(|re| re.is_match(line))).unwrap_or(false)
    }
}

impl Default for DefaultRecognizer {
    fn default() -> Self { Self::new() }
}

impl ElementRecognizer for DefaultRecognizer {
    fn recognize(&self, line: &str) -> ElementType {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            return ElementType::Empty;
        }

        // 检查各种模式
        if self.check_patterns(trimmed, "footer") {
            ElementType::Footer
        } else if self.check_patterns(trimmed, "code") {
            ElementType::CodeBlock
        } else if self.check_patterns(trimmed, "quote") {
            ElementType::Quote
        } else if self.check_patterns(trimmed, "list") {
            ElementType::ListItem
        } else if self.check_patterns(trimmed, "heading1") {
            ElementType::Heading(1)
        } else if self.check_patterns(trimmed, "heading2") {
            ElementType::Heading(2)
        } else if self.check_patterns(trimmed, "heading3") {
            ElementType::Heading(3)
        } else if trimmed.len() < 40 && !trimmed.ends_with(|c: char| "。！？；.!?;".contains(c)) {
            ElementType::Heading(4)
        } else {
            ElementType::Paragraph
        }
    }
}

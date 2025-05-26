use crate::config::ChunkConfig;
use crate::element::Element;
use crate::element::ElementType;
use crate::recognizer::ElementRecognizer;

/// 文档处理器：负责解析文本并构建元素
pub struct DocumentProcessor {
    recognizer: Box<dyn ElementRecognizer>,
}

impl DocumentProcessor {
    pub fn new(recognizer: Box<dyn ElementRecognizer>) -> Self { Self { recognizer } }

    /// 处理文本，返回元素列表
    pub fn process(&self, text: &str) -> Vec<Element> {
        let mut elements = Vec::new();
        let mut paragraph_buffer = Vec::new();
        let mut paragraph_start = 0;

        for (line_num, line) in text.lines().enumerate() {
            let element_type = self.recognizer.recognize(line);

            match element_type {
                ElementType::Paragraph => {
                    if paragraph_buffer.is_empty() {
                        paragraph_start = line_num;
                    }
                    paragraph_buffer.push(line);
                }
                _ => {
                    // 完成当前段落
                    if !paragraph_buffer.is_empty() {
                        let content = paragraph_buffer.join("\n");
                        elements.push(Element::new(ElementType::Paragraph, content, paragraph_start + 1));
                        paragraph_buffer.clear();
                    }

                    // 添加非段落元素
                    if !matches!(element_type, ElementType::Empty | ElementType::Footer) {
                        elements.push(Element::new(element_type, line.trim().to_string(), line_num + 1));
                    }
                }
            }
        }

        // 处理最后的段落
        if !paragraph_buffer.is_empty() {
            let content = paragraph_buffer.join("\n");
            elements.push(Element::new(ElementType::Paragraph, content, paragraph_start + 1));
        }

        elements
    }

    /// 合并标题与其内容（如果配置启用）
    pub fn merge_headings(&self, elements: Vec<Element>, config: &ChunkConfig) -> Vec<Element> {
        if !config.merge_headings {
            return elements;
        }

        let mut merged = Vec::new();
        let mut i = 0;

        while i < elements.len() {
            if let ElementType::Heading(level) = elements[i].element_type {
                let mut content = elements[i].content.clone();
                let mut char_count = elements[i].char_count;
                let line_number = elements[i].line_number;

                // 收集属于此标题的内容
                let mut j = i + 1;
                while j < elements.len() {
                    match &elements[j].element_type {
                        ElementType::Heading(next_level) if *next_level <= level => break,
                        _ => {
                            content.push('\n');
                            content.push_str(&elements[j].content);
                            char_count += elements[j].char_count + 1;
                            j += 1;
                        }
                    }
                }

                merged.push(Element { element_type: ElementType::Heading(level), content, line_number, char_count });

                i = j;
            } else {
                merged.push(elements[i].clone());
                i += 1;
            }
        }

        merged
    }
}

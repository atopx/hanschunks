use pyo3::prelude::*;

/// 文本元素类型
#[derive(Debug, Clone, PartialEq)]
pub enum ElementType {
    Heading(u8), // 标题（带级别）
    Paragraph,   // 段落
    ListItem,    // 列表项
    CodeBlock,   // 代码块
    Table,       // 表格
    Quote,       // 引用
    Empty,       // 空行
    Footer,      // 页脚
}

/// 文本元素
#[derive(Debug, Clone)]
pub struct Element {
    pub element_type: ElementType,
    pub content: String,
    pub line_number: usize,
    pub char_count: usize, // 预计算字符数
}

impl Element {
    pub fn new(element_type: ElementType, content: String, line_number: usize) -> Self {
        let char_count = content.chars().count();
        Self { element_type, content, line_number, char_count }
    }

    /// 是否可以作为分割点
    pub fn is_split_boundary(&self) -> bool {
        matches!(
            self.element_type,
            ElementType::Heading(_) | ElementType::ListItem | ElementType::CodeBlock | ElementType::Table
        )
    }

    /// 获取元素权重（用于分割决策）
    pub fn weight(&self, weights: &ElementWeights) -> f32 {
        match self.element_type {
            ElementType::Heading(level) => weights.heading_base - (level as f32 * weights.heading_level_penalty),
            ElementType::CodeBlock => weights.code_block,
            ElementType::Table => weights.table,
            ElementType::ListItem => weights.list_item,
            ElementType::Paragraph => weights.paragraph,
            ElementType::Quote => weights.quote,
            ElementType::Empty => weights.empty,
            ElementType::Footer => weights.footer,
        }
    }

    /// 检查元素是否与下文有强连接性（不适合作为分割点）
    ///
    /// # 返回
    /// 如果元素以连接符结尾返回true，否则返回false
    ///
    /// # 性能
    /// 使用缓存的trim结果，避免重复计算
    pub fn has_strong_connection(&self) -> bool {
        if self.content.is_empty() {
            return false;
        }

        let trimmed = self.content.trim_end();
        if trimmed.is_empty() {
            return false;
        }

        // 检查是否以连接符结尾（中英文冒号、逗号等）
        matches!(trimmed.chars().last(), Some(':' | '：' | ',' | '，' | '、' | ';' | '；'))
    }
}

/// 元素权重配置
#[pyclass]
#[derive(Debug, Clone, Copy)]
pub struct ElementWeights {
    #[pyo3(get, set)]
    pub heading_base: f32,
    #[pyo3(get, set)]
    pub heading_level_penalty: f32,
    #[pyo3(get, set)]
    pub code_block: f32,
    #[pyo3(get, set)]
    pub table: f32,
    #[pyo3(get, set)]
    pub list_item: f32,
    #[pyo3(get, set)]
    pub paragraph: f32,
    #[pyo3(get, set)]
    pub quote: f32,
    #[pyo3(get, set)]
    pub empty: f32,
    #[pyo3(get, set)]
    pub footer: f32,
}

#[pymethods]
impl ElementWeights {
    #[new]
    pub fn new() -> Self { Self::default() }
}

impl Default for ElementWeights {
    fn default() -> Self {
        Self {
            heading_base: 100.0,
            heading_level_penalty: 10.0,
            code_block: 80.0,
            table: 80.0,
            list_item: 60.0,
            paragraph: 40.0,
            quote: 30.0,
            empty: 10.0,
            footer: 0.0,
        }
    }
}

/// 分割点信息
#[derive(Debug, Clone)]
pub struct SplitPoint {
    pub index: usize,
    pub weight: f32,
    pub char_count: usize,
}

impl SplitPoint {
    pub fn new(index: usize, weight: f32, char_count: usize) -> Self { Self { index, weight, char_count } }
}

use pyo3::prelude::*;

use crate::element::ElementWeights;

/// 分块配置
#[pyclass]
#[derive(Clone, Copy)]
pub struct ChunkConfig {
    #[pyo3(get, set)]
    pub min_size: usize, // 最小字符数
    #[pyo3(get, set)]
    pub max_size: usize, // 最大字符数
    #[pyo3(get, set)]
    pub merge_headings: bool, // 是否合并标题与内容
    #[pyo3(get, set)]
    pub preserve_boundaries: bool, // 是否保持语义边界
    pub element_weights: ElementWeights, // 元素权重配置
}

#[pymethods]
impl ChunkConfig {
    #[new]
    pub fn new() -> Self {
        Self {
            min_size: 512,
            max_size: 800,
            merge_headings: true,
            preserve_boundaries: true,
            element_weights: ElementWeights::default(),
        }
    }

    /// 设置元素权重 - 所有参数都是可选的
    #[pyo3(signature = (
        heading_base=None,
        heading_level_penalty=None,
        code_block=None,
        table=None,
        list_item=None,
        paragraph=None,
        quote=None,
        empty=None,
        footer=None
    ))]
    #[allow(clippy::too_many_arguments)]
    pub fn set_element_weights(
        &mut self, heading_base: Option<f32>, heading_level_penalty: Option<f32>, code_block: Option<f32>,
        table: Option<f32>, list_item: Option<f32>, paragraph: Option<f32>, quote: Option<f32>, empty: Option<f32>,
        footer: Option<f32>,
    ) {
        if let Some(val) = heading_base {
            self.element_weights.heading_base = val;
        }
        if let Some(val) = heading_level_penalty {
            self.element_weights.heading_level_penalty = val;
        }
        if let Some(val) = code_block {
            self.element_weights.code_block = val;
        }
        if let Some(val) = table {
            self.element_weights.table = val;
        }
        if let Some(val) = list_item {
            self.element_weights.list_item = val;
        }
        if let Some(val) = paragraph {
            self.element_weights.paragraph = val;
        }
        if let Some(val) = quote {
            self.element_weights.quote = val;
        }
        if let Some(val) = empty {
            self.element_weights.empty = val;
        }
        if let Some(val) = footer {
            self.element_weights.footer = val;
        }
    }

    /// 获取元素权重配置的副本
    pub fn get_element_weights(&self) -> ElementWeights { self.element_weights }
}

impl Default for ChunkConfig {
    fn default() -> Self { Self::new() }
}

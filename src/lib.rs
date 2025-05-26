use pyo3::prelude::*;

// 模块声明
pub mod chunker;
pub mod config;
pub mod element;
pub mod processor;
pub mod recognizer;

// 导入必要的类型
use chunker::Chunker;
use config::ChunkConfig;
use element::ElementWeights;
use processor::DocumentProcessor;
use recognizer::DefaultRecognizer;

/// 主要的文本分块器
#[pyclass]
pub struct TextChunker {
    processor: DocumentProcessor,
    chunker: Chunker,
}

impl Default for TextChunker {
    fn default() -> Self { Self::new(None) }
}

#[pymethods]
impl TextChunker {
    #[new]
    #[pyo3(signature = (config = None))]
    pub fn new(config: Option<ChunkConfig>) -> Self {
        let recognizer = Box::new(DefaultRecognizer::new());
        let processor = DocumentProcessor::new(recognizer);
        let chunker = Chunker::new(config);

        Self { processor, chunker }
    }

    /// 获取配置
    pub fn get_config(&self) -> ChunkConfig { self.chunker.config }

    /// 执行文本分块
    ///
    /// # 参数
    /// * `text` - 待分块的文本
    ///
    /// # 返回
    /// 返回分块后的字符串向量，每个块大小在配置的范围内
    ///
    /// # 算法特点
    /// - 使用动态规划寻找最优分割点
    /// - 避免在语义连接符（如冒号）后分割
    /// - 支持按元素类型权重进行智能分割
    pub fn chunk(&self, text: &str) -> Vec<String> {
        if text.trim().is_empty() {
            return vec![];
        }

        // 1. 解析文本为元素
        let elements = self.processor.process(text);
        if elements.is_empty() {
            return vec![];
        }

        // 2. 合并标题（如果配置启用）
        let elements = self.processor.merge_headings(elements, &self.chunker.config);

        // 3. 执行优化的分块算法
        self.chunker.chunk(elements)
    }
}

/// Python模块定义
#[pymodule]
fn hanschunks(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ChunkConfig>()?;
    m.add_class::<ElementWeights>()?;
    m.add_class::<TextChunker>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunking() {
        let text = include_str!("../samples/test.txt");
        let chunker = TextChunker::new(None);

        let chunks = chunker.chunk(text);
        println!("总共生成了 {} 个块", chunks.len());
        for (i, chunk) in chunks.iter().enumerate() {
            let char_count = chunk.chars().count();
            println!("块{}: {} 字符", i, char_count);

            // 验证块大小限制，但允许为避免连接符分割而适度超出
            assert!(
                char_count <= chunker.chunker.config.max_size,
                "块{} 超过合理大小: {} > {} (允许适度超出避免连接符分割)",
                i,
                char_count,
                chunker.chunker.config.max_size
            );

            // 最后一块可以小于最小大小
            if i < chunks.len() - 1 {
                assert!(
                    char_count >= chunker.chunker.config.min_size,
                    "块{} 小于最小大小: {} < {}",
                    i,
                    char_count,
                    chunker.chunker.config.min_size
                );
                assert!(
                    !chunk.trim_end().ends_with(':'),
                    "块{}以冒号结尾, 结尾字符串内容: {}",
                    i,
                    chunk.chars().rev().take(10).collect::<Vec<_>>().into_iter().rev().collect::<String>()
                );
                assert!(
                    !chunk.trim_end().ends_with('：'),
                    "块{} 以冒号结尾, 结尾字符串内容: {}",
                    i,
                    chunk.chars().rev().take(10).collect::<Vec<_>>().into_iter().rev().collect::<String>()
                );
            }
        }
    }

    #[test]
    fn test_performance_comparison() {
        let text = include_str!("../samples/test.txt").repeat(10); // 扩大测试数据
        let chunker = TextChunker::new(None);

        use std::time::Instant;

        // 测试优化后的算法性能
        let start = Instant::now();
        for _ in 0..10 {
            let _chunks = chunker.chunk(&text);
        }
        let optimized_duration = start.elapsed();

        println!("优化后算法（含二分查找）：10次分块耗时 {:?}", optimized_duration);
        println!("平均每次分块耗时：{:?}", optimized_duration / 10);

        // 验证结果的正确性
        let chunks = chunker.chunk(&text);
        println!("扩大数据分块结果：总共 {} 个块", chunks.len());

        // 验证所有块都符合要求
        for (i, chunk) in chunks.iter().enumerate() {
            let char_count = chunk.chars().count();
            assert!(char_count <= chunker.chunker.config.max_size);
            if i < chunks.len() - 1 {
                assert!(char_count >= chunker.chunker.config.min_size);
                assert!(!chunk.trim_end().ends_with(':'));
                assert!(!chunk.trim_end().ends_with('：'));
            }
        }
    }
}

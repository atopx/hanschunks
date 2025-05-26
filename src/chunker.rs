use jieba_rs::Jieba;

use crate::config::ChunkConfig;
use crate::element::Element;

/// 分块器：使用动态规划和区间最大查询算法(二分查找优化)
pub struct Chunker {
    jieba: Jieba,
    pub config: ChunkConfig,
}

impl Default for Chunker {
    fn default() -> Self { Self::new(None) }
}

impl Chunker {
    pub fn new(config: Option<ChunkConfig>) -> Self { Self { jieba: Jieba::new(), config: config.unwrap_or_default() } }

    /// 执行分块 - 使用动态规划算法
    pub fn chunk(&self, elements: Vec<Element>) -> Vec<String> {
        if elements.is_empty() {
            return vec![];
        }

        // 预计算累积字符数
        let cumulative_chars = self.build_cumulative_chars(&elements);
        let total_chars = cumulative_chars[cumulative_chars.len() - 1];

        // 如果总长度在合理范围内，直接返回
        if total_chars >= self.config.min_size && total_chars <= self.config.max_size {
            return vec![self.build_chunk_content(&elements)];
        }

        // 使用动态规划找到最优分块方案
        let split_points = self.find_optimal_splits(&elements, &cumulative_chars);

        // 根据分割点构建分块
        self.build_chunks_from_splits(&elements, &split_points)
    }

    /// 构建累积字符数数组（包含换行符）
    fn build_cumulative_chars(&self, elements: &[Element]) -> Vec<usize> {
        let mut cumulative = Vec::with_capacity(elements.len() + 1);
        cumulative.push(0);

        for (i, element) in elements.iter().enumerate() {
            let prev = cumulative[i];
            // 防止整数溢出，使用安全算术
            let chars = if i == 0 {
                element.char_count
            } else {
                element.char_count + 1 // 换行符
            };
            cumulative.push(prev + chars);
        }

        cumulative
    }

    /// 使用动态规划找到最优分割点
    fn find_optimal_splits(&self, elements: &[Element], cumulative_chars: &[usize]) -> Vec<usize> {
        let n = elements.len();
        let mut dp = vec![f32::NEG_INFINITY; n + 1];
        let mut prev = vec![None; n + 1];

        dp[0] = 0.0;

        // 动态规划：dp[i] 表示从位置0到i-1的最优分块方案的总权重
        for i in 1..=n {
            // 使用二分查找：快速定位有效分割范围
            let (min_j, max_j) = if i == n {
                // 最后一块：只要前驱可达即可，使用线性搜索确保找到所有可达状态
                (0, i)
            } else {
                // 中间块：使用二分查找定位满足大小约束的范围
                self.find_valid_split_range(cumulative_chars, i)
            };

            // 只在有效范围内进行DP转移
            for j in min_j..max_j {
                let chunk_chars = cumulative_chars[i] - cumulative_chars[j];

                // 对于中间块，范围已经保证大小合法；对于最后一块，需要检查前驱可达性
                if i == n && dp[j] == f32::NEG_INFINITY {
                    continue;
                }

                // 高效的连接性检查：直接检查最后一个元素，避免重复构建块内容
                let ends_with_colon = if i > 0 { elements[i - 1].has_strong_connection() } else { false };

                // 连接性权重惩罚：降低以连接符结尾的分割点权重
                let connection_penalty = if ends_with_colon { 0.1 } else { 1.0 };

                // 在区间[j, i-1]中找到最佳分割点的权重
                let split_weight = self.find_best_split_weight_in_range(elements, j, i - 1);

                // 计算总权重，包含大小惩罚
                let mut total_weight = (dp[j] + split_weight) * connection_penalty;

                // 对最后一块的过大惩罚
                if i == n && chunk_chars > self.config.max_size {
                    let overage_ratio = chunk_chars as f32 / self.config.max_size as f32;
                    let size_penalty = 1.0 / overage_ratio; // 越大惩罚越重
                    total_weight *= size_penalty;
                }

                if total_weight > dp[i] {
                    dp[i] = total_weight;
                    prev[i] = Some(j);
                }
            }
        }

        // 回溯构建分割点序列
        let mut splits = Vec::new();
        let mut current = n;

        while let Some(p) = prev[current] {
            if p > 0 {
                splits.push(p - 1); // 转换为元素索引
            }
            current = p;
        }

        splits.reverse();
        splits
    }

    /// 二分查找：找到满足大小约束的有效分割范围
    ///
    /// # 返回
    /// (min_j, max_j) 其中 min_j..max_j 是有效的分割起点范围
    fn find_valid_split_range(&self, cumulative_chars: &[usize], i: usize) -> (usize, usize) {
        let target_chars = cumulative_chars[i];

        // 如果累积字符数小于最小大小，没有有效分割点
        if target_chars < self.config.min_size {
            return (i, i); // 返回空范围
        }

        // 二分查找最小有效起点：target_chars - cumulative_chars[j] <= max_size
        // 即：cumulative_chars[j] >= target_chars - max_size
        let min_cumulative = target_chars.saturating_sub(self.config.max_size);
        let min_j = self.binary_search_ge(cumulative_chars, 0, i, min_cumulative);

        // 二分查找最大有效起点：target_chars - cumulative_chars[j] >= min_size
        // 即：cumulative_chars[j] <= target_chars - min_size
        // 注意：这里不能使用saturating_sub，因为我们需要精确的数学计算
        if target_chars < self.config.min_size {
            return (i, i); // 双重保险，确保返回空范围
        }

        let max_cumulative = target_chars - self.config.min_size;
        let max_j = self.binary_search_le(cumulative_chars, 0, i, max_cumulative) + 1;

        let result = (min_j, max_j.min(i));

        // 验证范围有效性：确保范围内确实有有效分割点
        if result.0 >= result.1 {
            return (i, i); // 返回空范围
        }

        result
    }

    /// 二分查找：找到第一个 >= target 的位置
    fn binary_search_ge(&self, arr: &[usize], left: usize, right: usize, target: usize) -> usize {
        let mut left = left;
        let mut right = right;

        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] >= target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    /// 二分查找：找到最后一个 <= target 的位置
    fn binary_search_le(&self, arr: &[usize], left: usize, right: usize, target: usize) -> usize {
        let mut left = left;
        let mut right = right;
        let mut result = left;

        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] <= target {
                result = mid;
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        result
    }

    /// 区间最大查询：在指定范围内找到权重最高的分割点
    fn find_best_split_weight_in_range(&self, elements: &[Element], start: usize, end: usize) -> f32 {
        if start >= end || end > elements.len() {
            return 0.0;
        }

        let mut max_weight = 0.0;

        // 在范围内寻找最佳分割点
        for element in elements[start..end].iter() {

            // 计算基础权重
            let base_weight = if element.is_split_boundary() {
                element.weight(&self.config.element_weights)
            } else {
                element.weight(&self.config.element_weights) * 0.1
            };

            // 连接性权重：如果元素与下文有强关联，大幅降低分割权重
            let connection_penalty = if element.has_strong_connection() { 0.01 } else { 1.0 };

            let final_weight = base_weight * connection_penalty;
            max_weight = f32::max(max_weight, final_weight);
        }

        max_weight
    }

    /// 根据分割点构建最终的分块
    fn build_chunks_from_splits(&self, elements: &[Element], splits: &[usize]) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut start = 0;

        for &split_point in splits {
            if split_point < elements.len() {
                let chunk_content = self.build_chunk_content(&elements[start..=split_point]);
                chunks.push(chunk_content);
                start = split_point + 1;
            }
        }

        // 处理最后一部分
        if start < elements.len() {
            let chunk_content = self.build_chunk_content(&elements[start..]);

            // 如果最后一块太大，需要进一步分割
            if chunk_content.chars().count() > self.config.max_size {
                let mut sub_chunks = self.split_large_content(&chunk_content);
                chunks.append(&mut sub_chunks);
            } else {
                chunks.push(chunk_content);
            }
        }

        chunks
    }

    /// 构建块内容 - 优化版本，预分配容量
    fn build_chunk_content(&self, elements: &[Element]) -> String {
        if elements.is_empty() {
            return String::new();
        }

        // 预估容量，减少重新分配
        let estimated_capacity = elements.iter().map(|e| e.char_count).sum::<usize>() + elements.len();
        let mut content = String::with_capacity(estimated_capacity);

        for (i, element) in elements.iter().enumerate() {
            if i > 0 {
                content.push('\n');
            }
            content.push_str(&element.content);
        }
        content
    }

    /// 分割过大的内容
    fn split_large_content(&self, content: &str) -> Vec<String> {
        // 尝试按行分割
        let lines: Vec<&str> = content.lines().collect();
        if lines.len() > 1 {
            return self.split_by_lines(&lines);
        }

        // 使用分词分割
        self.split_by_tokens(content)
    }

    /// 按行分割
    fn split_by_lines(&self, lines: &[&str]) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current = String::new();
        let mut current_chars = 0;

        for line in lines {
            let line_chars = line.chars().count();

            if current_chars + line_chars + 1 > self.config.max_size && current_chars > 0 {
                chunks.push(current.clone());
                current.clear();
                current_chars = 0;
            }

            if !current.is_empty() {
                current.push('\n');
                current_chars += 1;
            }
            current.push_str(line);
            current_chars += line_chars;
        }

        if !current.is_empty() {
            chunks.push(current);
        }

        chunks
    }

    /// 按分词分割
    fn split_by_tokens(&self, text: &str) -> Vec<String> {
        let tokens: Vec<&str> = self.jieba.cut(text, false);
        let mut chunks = Vec::new();
        let mut current = String::new();
        let mut current_chars = 0;

        for token in tokens {
            let token_chars = token.chars().count();

            if current_chars + token_chars > self.config.max_size && current_chars > 0 {
                chunks.push(current.clone());
                current.clear();
                current_chars = 0;
            }

            current.push_str(token);
            current_chars += token_chars;
        }

        if !current.is_empty() {
            chunks.push(current);
        }

        chunks
    }
}

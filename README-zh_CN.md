# HansChunks

[English](README.md) | 中文

基于 Rust 构建的高性能中文文档提取和语义分块工具，提供 Python 绑定。

## 特性

- **智能文本分块**：将中文文档分割成保持语义连贯性的块，同时保留上下文
- **元素识别**：识别不同类型的文档元素（标题、段落、列表、代码块等）
- **语义边界保护**：避免在不合适的位置（如冒号后）进行分割
- **标题合并**：可选择将标题与其内容保持在同一块中
- **可自定义配置**：调整块大小和元素权重，以优化特定场景的分块效果
- **高性能**：使用 Rust 实现，采用优化算法确保速度和效率
- **Python 绑定**：提供简单直观的 Python API 接口
- **先进算法**：使用动态规划和二分查找优化来寻找最佳分割点，同时确保效率和语义连贯性
- **上下文感知处理**：在做分块决策时考虑文档结构、元素类型和语义连接

## 安装

```bash
pip install hanschunks
```

## 快速开始

```python
from hanschunks.hanschunks import TextChunker, ChunkConfig

# 创建默认配置的分块器
chunker = TextChunker()

# 处理文档
text = """第一章 引言

随着人工智能技术的快速发展，自然语言处理已经成为计算机科学中最重要的研究领域之一。
文本分块作为信息检索和知识管理的基础技术，其重要性日益凸显。
"""

chunks = chunker.chunk(text)
for i, chunk in enumerate(chunks, 1):
    print(f"块 {i}: {chunk[:50]}...")
```

## 自定义配置

```python
# 创建自定义配置
config = ChunkConfig()
config.min_size = 160        # 最小块大小（字符数）
config.max_size = 320        # 最大块大小（字符数）
config.merge_headings = True # 将标题与后续内容合并
config.preserve_boundaries = True # 保持语义边界

# 设置元素权重以影响分块决策
config.set_element_weights(
    heading_base=100.0,       # 标题基础权重
    heading_level_penalty=10.0, # 每级标题的惩罚系数
    code_block=80.0,          # 代码块权重
    table=80.0,               # 表格权重
    list_item=60.0,           # 列表项权重
    paragraph=40.0,           # 段落权重
    quote=30.0,               # 引用块权重
    empty=10.0,               # 空行权重
    footer=0.0                # 页脚元素权重
)

# 使用自定义配置创建分块器
chunker = TextChunker(config)
```

## 算法

HansChunks 使用优化的动态规划算法来寻找文档中最佳的分割点：

1. 首先将文档解析为语义元素（标题、段落等）
2. 根据元素类型为每个元素分配权重
3. 使用带有二分查找优化的动态规划算法寻找最优分割点
4. 保护强语义连接（例如，避免在冒号后分割）
5. 生成一组在块大小限制和语义连贯性之间取得平衡的分块

## 开发

### 前置要求

- Rust 工具链 (1.75+)
- Python 3.12+
- Maturin (用于构建 Python 绑定)

### 构建开发包

```bash
uv run maturin develop --release
uv run example/demo.py
```

### 从源码构建

```bash
uv run maturin build --release --out dist 
uv add dist/hanschunks-*.whl
```

### 运行测试

```bash
cargo test
```

## 许可证

[Apache 2.0](LICENSE)

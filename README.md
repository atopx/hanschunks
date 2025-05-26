# HansChunks

English | [中文](README-zh_CN.md)

High-performance Chinese document extractor and semantic chunker built in Rust with Python bindings.

## Features

- **Intelligent Text Chunking**: Splits Chinese documents into semantically meaningful chunks while preserving context
- **Element Recognition**: Identifies different types of document elements (headings, paragraphs, lists, code blocks, etc.)
- **Semantic Boundary Preservation**: Avoids breaking text at poor split points like colons
- **Heading Merging**: Option to keep headings with their content
- **Customizable Configuration**: Adjust chunk sizes and element weights to optimize for your specific use case
- **High Performance**: Implemented in Rust with optimized algorithms for speed and efficiency
- **Python Bindings**: Easy to use from Python with a simple, intuitive API
- **Advanced Algorithm**: Uses dynamic programming with binary search optimization to find optimal split points, ensuring both efficiency and semantic coherence
- **Context-Aware Processing**: Considers document structure, element types, and semantic connections when making chunking decisions

## Installation

```bash
pip install hanschunks
```

## Quick Start

```python
from hanschunks.hanschunks import TextChunker, ChunkConfig

# Create a chunker with default settings
chunker = TextChunker()

# Process a document
text = """第一章 引言

随着人工智能技术的快速发展，自然语言处理已经成为计算机科学中最重要的研究领域之一。
文本分块作为信息检索和知识管理的基础技术，其重要性日益凸显。
"""

chunks = chunker.chunk(text)
for i, chunk in enumerate(chunks, 1):
    print(f"Chunk {i}: {chunk[:50]}...")
```

## Custom Configuration

```python
# Create custom configuration
config = ChunkConfig()
config.min_size = 160        # Minimum chunk size in characters
config.max_size = 320        # Maximum chunk size in characters
config.merge_headings = True # Merge headings with following content
config.preserve_boundaries = True # Preserve semantic boundaries

# Set element weights for chunking decisions
config.set_element_weights(
    heading_base=100.0,       # Base weight for headings
    heading_level_penalty=10.0, # Penalty per heading level
    code_block=80.0,          # Weight for code blocks
    table=80.0,               # Weight for tables
    list_item=60.0,           # Weight for list items
    paragraph=40.0,           # Weight for paragraphs
    quote=30.0,               # Weight for block quotes
    empty=10.0,               # Weight for empty lines
    footer=0.0                # Weight for footer elements
)

# Create chunker with custom config
chunker = TextChunker(config)
```

## Algorithm

HansChunks uses an optimized dynamic programming algorithm to find the best possible split points in a document:

1. Document is first parsed into semantic elements (headings, paragraphs, etc.)
2. Each element is assigned a weight based on its type
3. Dynamic programming with binary search optimization finds optimal split points
4. Strong semantic connections are preserved (e.g., avoiding splits after colons)
5. The result is a set of chunks that balance size constraints with semantic coherence

## Development

### Prerequisites

- Rust toolchain (1.75+)
- Python 3.12+
- Maturin (for building Python bindings)

### Build develop package

```bash
uv run maturin develop --release
uv run example/demo.py
```

### Building from source

```bash
uv run maturin build --release --out dist 
uv add dist/hanschunks-*.whl
```

### Running tests

```bash
cargo test
```

## License

[Apache 2.0](LICENSE)

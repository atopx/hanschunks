#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
基于段落的智能文本分块演示

这个示例展示了重构后的分块器如何：
1. 识别不同类型的文本元素（标题、段落、列表等）
2. 基于语义边界进行分块
3. 保持段落完整性
4. 可扩展的设计架构
"""

from hanschunks.hanschunks import ChunkConfig, TextChunker

# 示例文档
sample_text = """第一章 引言

随着人工智能技术的快速发展，自然语言处理已经成为计算机科学中最重要的研究领域之一。文本分块作为信息检索和知识管理的基础技术，其重要性日益凸显。

传统的文本分块方法主要依赖于简单的字符计数和句子边界检测。然而，这种方法往往忽略了文本的语义结构，导致分块结果缺乏连贯性。

1.1 研究背景

本研究旨在开发一种基于段落语义的智能文本分块算法。该算法具有以下特点：

• 能够识别文档的层次结构
• 保持语义完整性
• 支持多种文档格式
• 具有良好的可扩展性

1.2 技术挑战

在文本分块过程中，我们面临着多个技术挑战：

1. 如何准确识别段落边界？
2. 如何处理不同类型的文本元素？
3. 如何在保持语义完整性的同时控制块大小？

第二章 算法设计

本章将详细介绍我们提出的基于段落的分块算法。

2.1 文本元素识别

算法首先对输入文本进行分析，识别出以下类型的文本元素。

2.2 段落构建

识别出文本元素后，算法将连续的段落文本合并为完整的段落单元。

第 1 页 共 5 页"""


def demonstrate_custom_config():
    """演示自定义配置的分块"""
    print("=== 自定义配置演示 ===")

    # 创建自定义配置
    config = ChunkConfig()
    config.min_size = 160
    config.max_size = 320  # 最大块大小
    config.merge_headings = True  # 是否保持语义边界
    config.merge_headings = True  # 是否合并标题与内容

    chunker = TextChunker(config)
    chunks = chunker.chunk(sample_text)

    print(f"使用自定义配置，共生成 {len(chunks)} 个文本块：\n")

    for i, chunk in enumerate(chunks, 1):
        print(f"块 {i} (长度: {len(chunk)}):")
        print("-" * 40)
        print(chunk[:100] + ("..." if len(chunk) > 100 else ""))
        print("-" * 40)
        print()


if __name__ == "__main__":
    demonstrate_custom_config()

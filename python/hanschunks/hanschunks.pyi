from typing import List, Optional

__all__ = ["ElementWeights", "ChunkConfig", "TextChunker"]

class ElementWeights:
    """Element weights configuration for chunking decisions.

    Defines weights for different types of document elements to influence chunking decisions.
    Higher weights make an element more likely to be a split point.
    """

    heading_base: float
    """Base weight for headings (default: 100.0)"""

    heading_level_penalty: float
    """Penalty per heading level (default: 10.0)"""

    code_block: float
    """Weight for code blocks (default: 80.0)"""

    table: float
    """Weight for tables (default: 80.0)"""

    list_item: float
    """Weight for list items (default: 60.0)"""

    paragraph: float
    """Weight for paragraphs (default: 40.0)"""

    quote: float
    """Weight for block quotes (default: 30.0)"""

    empty: float
    """Weight for empty lines (default: 10.0)"""

    footer: float
    """Weight for footer elements (default: 0.0)"""

    def __init__(self) -> None:
        """Initialize element weights with default values."""
        ...

class ChunkConfig:
    """Configuration for text chunking.

    Controls chunk size limits, element merging behavior, and element weights
    to optimize chunking for specific use cases.
    """

    min_size: int
    """Minimum chunk size in characters (default: 512)"""

    max_size: int
    """Maximum chunk size in characters (default: 800)"""

    merge_headings: bool
    """Whether to merge headings with following content (default: True)"""

    preserve_boundaries: bool
    """Whether to preserve semantic boundaries (default: True)"""

    def __init__(self) -> None:
        """Initialize chunking configuration with default values.

        - min_size: 512
        - max_size: 800
        - merge_headings: True
        - preserve_boundaries: True
        """
        ...

    def set_element_weights(
        self,
        heading_base: Optional[float] = None,
        heading_level_penalty: Optional[float] = None,
        code_block: Optional[float] = None,
        table: Optional[float] = None,
        list_item: Optional[float] = None,
        paragraph: Optional[float] = None,
        quote: Optional[float] = None,
        empty: Optional[float] = None,
        footer: Optional[float] = None,
    ) -> None:
        """Set element weights for chunking decisions.

        All parameters are optional. Only specified parameters will be updated.

        Args:
            heading_base: Base weight for headings
            heading_level_penalty: Penalty per heading level
            code_block: Weight for code blocks
            table: Weight for tables
            list_item: Weight for list items
            paragraph: Weight for paragraphs
            quote: Weight for block quotes
            empty: Weight for empty lines
            footer: Weight for footer elements
        """
        ...

    def get_element_weights(self) -> ElementWeights:
        """Get a copy of the current element weights configuration.

        Returns:
            ElementWeights: Current element weights configuration
        """
        ...

class TextChunker:
    """High-performance Chinese document chunker.

    Splits Chinese documents into semantically meaningful chunks while preserving
    context and semantic boundaries.
    """

    def __init__(self, config: Optional[ChunkConfig] = None) -> None:
        """Initialize a text chunker with the given configuration.

        Args:
            config: Chunking configuration. If None, default configuration is used.
        """
        ...

    def get_config(self) -> ChunkConfig:
        """Get the current chunking configuration.

        Returns:
            ChunkConfig: Current chunking configuration
        """
        ...

    def chunk(self, text: str) -> List[str]:
        """Split text into semantically meaningful chunks.

        Uses dynamic programming with binary search optimization to find optimal split points,
        ensuring both efficiency and semantic coherence.

        Args:
            text: Text to chunk

        Returns:
            List[str]: List of text chunks, each within the configured size limits
        """
        ...

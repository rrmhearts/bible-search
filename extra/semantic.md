# Semantic Search

## Rust

Converting `semantic_search.py` to Rust while maintaining the functionality of the Python `sentence-transformers`, we will use the **`fastembed`** crate. It's a blazingly fast Rust library that downloads models automatically and runs inference using the highly optimized ONNX Runtime, making it perfect for this exact use case.

### How to Run and Build It

Because Rust compiles to machine code, compiling this project will produce a standalone binary.

**To run it directly during development:**

```bash
cargo run --release -- --file ../bibles/bible.txt --query "born of the water and the spirit" --top-k 3
```

*(Note: We use `--release` here because neural network inference in Rust debug mode is incredibly slow due to unoptimized math operations).*

**To build it for distribution or embedding into a larger system:**

```bash
cargo add ort --features tls-rustls
cargo build --release

```

Your standalone executable will be located at `target/release/semantic_search`.

### Why this Rust version shines:

1. **Zero-Copy Text Processing:** When we chunk the text, Rust uses references (`&str`) to the original String in memory rather than allocating thousands of new strings like Python does.
2. **ONNX Runtime:** `fastembed` leans on the ONNX runtime, meaning it runs highly optimized C/C++ backend code for the matrix multiplication, matching or exceeding PyTorch CPU performance.
3. **No Interpreter:** Once compiled, the time it takes to spin up the program and start searching is near-instantaneous.
## semantic_search.py

---

## Python

The best approach for semantic search is to use the **`sentence-transformers`** library. It provides access to fast, locally-run Large Language Models specifically fine-tuned for generating highly descriptive latent vectors (dense embeddings) and computing their similarity.

For the similarity metric, **Cosine Similarity** is the industry standard for comparing dense embedding vectors. It measures the angle between two vectors in a multi-dimensional space, perfectly capturing semantic closeness regardless of text length.

### Prerequisites

Before running the script, you will need to install the required libraries. PyTorch is required as the backend engine.

```bash
pip install sentence-transformers torch

```

By default, it uses the `all-MiniLM-L6-v2` model. This model is exceptionally fast, lightweight, and punches well above its weight class for semantic retrieval tasks.

### How to Use Python Verion

#### 1. Calling it from the Command Line

Save the code above as `semantic_search.py`. Assuming you have a large text file named `book.txt`, you can search it like this:

```bash
python .\python\semantic_search.py --file .\bibles\bible.txt --query "born of the water and the spirit" --top_k 3

Loading model 'all-MiniLM-L6-v2' and processing text...

--- Top 5 Matches for: 'born of the water and the spirit' ---

[1] Similarity Score: 0.5467
John 3:8        The wind bloweth where it listeth, and thou hearest the voice thereof, but knowest not whence it cometh, and whither it goeth: so is every one that is born of the Spirit.

----------------------------------------

[2] Similarity Score: 0.5127
Psalm 124:5     Then the proud waters had gone over our soul.

----------------------------------------

[3] Similarity Score: 0.5094
John 3:5        Jesus answered, Verily, verily, I say unto thee, Except a man be born of water and the Spirit, he cannot enter into the kingdom of God.

----------------------------------------
```

#### 2. Using it as a Function in a Larger Program

```python
from semantic_search import SemanticSearcher

# Initialize once to keep the model loaded in memory
searcher = SemanticSearcher()

document = """
The sky was a brilliant blue, completely devoid of clouds. The sun beat down mercilessly on the desert sand.

In the depths of the ocean, strange bioluminescent creatures navigate the crushing pressure. The water here is freezing.

The local bakery smells of fresh sourdough and cinnamon. People line up around the block every morning.
"""

# Query for a concept
results = searcher.search(
    query="I am looking for information about cold water environments.", 
    text=document, 
    top_k=1
)

print(results[0]['text']) 
# Output: "In the depths of the ocean, strange bioluminescent creatures..."

```

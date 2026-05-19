use anyhow::{Context, Result};
use clap::Parser;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use std::fs;
use std::io::{self, Write};

/// A struct to convert text into latent vectors and perform semantic search.
pub struct SemanticSearcher {
    model: TextEmbedding,
}

impl SemanticSearcher {
    /// Initializes the searcher with the all-MiniLM-L6-v2 model via fastembed.
    pub fn new() -> Result<Self> {
        // InitOptions allows us to specify the model. 
        // It will automatically download on the first run and cache it.
        let model = TextEmbedding::try_new(InitOptions::new(EmbeddingModel::SnowflakeArcticEmbedXS))
            .context("Failed to initialize the embedding model")?;
        // let model = TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2Q))
        //     .context("Failed to initialize the embedding model")?;

        Ok(Self { model })
    }

    /// Splits a large document into manageable chunks.
    pub fn chunk_text<'a>(&self, text: &'a str, separator: &str) -> Vec<&'a str> {
        text.split(separator)
            .map(|chunk| chunk.trim())
            .filter(|chunk| !chunk.is_empty())
            .collect()
    }

    /// Computes the Cosine Similarity between two dense vectors.
    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }

    /// Searches the text for the chunks that best match the semantic query.
    pub fn search<'a>(
        &mut self,
        query: &str,
        text: &'a str,
        top_k: usize,
        separator: &str,
    ) -> Result<Vec<(f32, &'a str)>> {
        let paragraphs = self.chunk_text(text, separator);
        if paragraphs.is_empty() {
            return Ok(Vec::new());
        }

        // 1. Convert the search query to a latent vector
        let query_embedding = self
            .model
            .embed(vec![query], None)
            .context("Failed to embed query")?;
        let query_vec = &query_embedding[0]; // Extract the single vector

        // 2. Convert paragraphs to latent vectors
        let paragraph_embeddings = self
            .model
            .embed(paragraphs.clone(), None)
            .context("Failed to embed paragraphs")?;

        // 3. Compute Cosine Similarity for each paragraph
        let mut scored_paragraphs: Vec<(f32, &str)> = paragraph_embeddings
            .iter()
            .zip(paragraphs.iter())
            .map(|(emb, text)| (Self::cosine_similarity(query_vec, emb), *text))
            .collect();

        // 4. Sort descending by score (highest similarity first)
        scored_paragraphs.sort_by(|a, b| {
            b.0.partial_cmp(&a.0)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // 5. Take the top_k results
        let top_results = scored_paragraphs.into_iter().take(top_k).collect();

        Ok(top_results)
    }
}

// --- Command Line Interface Setup ---

#[derive(Parser, Debug)]
#[command(author, version, about = "Semantic Document Search using Latent Vectors", long_about = None)]
struct Args {
    /// The concept or sentence to search for
    #[arg(short, long)]
    query: String,

    /// Path to the large text file to search through
    #[arg(short, long)]
    file: String,

    /// Number of top matching paragraphs to return
    #[arg(short, long, default_value_t = 3)]
    top_k: usize,

    /// String used to split the text into chunks
    #[arg(short, long, default_value = "\n")]
    separator: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read the target document
    println!("Loading document into memory...");
    let document_text = std::fs::read_to_string(&args.file)
        .with_context(|| format!("Could not read file '{}'", args.file))?;

        println!("Loading ONNX model and processing text...");

        // Execute search
    let mut searcher = SemanticSearcher::new()?;
    let matches = searcher.search(&args.query, &document_text, args.top_k, &args.separator)?;

    // Display results
    println!("\n--- Top {} Matches for: '{}' ---\n", args.top_k, args.query);
    for (i, (score, text)) in matches.iter().enumerate() {
        println!("[{}] Similarity Score: {:.4}", i + 1, score);
        println!("{}\n", text);
        println!("{}\n", "-".repeat(40));
    }
    println!("Ready! Model and text are loaded.\n");

    // 2. Keep the program alive with an infinite loop
    loop {
        // Prompt the user
        print!("Enter search query (or type 'quit' to exit): ");
        io::stdout().flush()?; // Ensure the prompt prints before waiting for input

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        let query = user_input.trim();

        // Exit condition
        if query.eq_ignore_ascii_case("quit") || query.eq_ignore_ascii_case("exit") {
            println!("Exiting search...");
            break;
        }

        if query.is_empty() {
            continue;
        }

        // 3. Execute the search instantly (model is already in RAM)
        let matches = searcher.search(query, &document_text, args.top_k, &args.separator)?;

        // Display results
        println!("\n--- Top {} Matches for: '{}' ---\n", args.top_k, query);
        for (i, (score, text)) in matches.iter().enumerate() {
            println!("[{}] Similarity Score: {:.4}", i + 1, score);
            println!("{}\n", text);
            println!("{}\n", "-".repeat(40));
        }
    }

    Ok(())
}
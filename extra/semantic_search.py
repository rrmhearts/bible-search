import argparse
import torch
from sentence_transformers import SentenceTransformer, util

class SemanticSearcher:
    """
    A class to convert text into latent vectors and perform semantic search.
    """
    def __init__(self, model_name='all-MiniLM-L6-v2'):
        """
        Initializes the searcher with a specified sentence-transformer model.
        """
        # Loads the model. It will automatically download on the first run.
        self.model = SentenceTransformer(model_name)

    def chunk_text(self, text, separator='\n'):
        """
        Splits a large document into manageable chunks (default is paragraphs).
        """
        # Split by the separator and remove empty chunks/whitespace
        chunks = [chunk.strip() for chunk in text.split(separator) if chunk.strip()]
        return chunks

    def search(self, query, text, top_k=3, separator='\n'):
        """
        Searches the text for the chunks that best match the semantic query.
        """
        # 1. Chunk the document
        paragraphs = self.chunk_text(text, separator)
        
        if not paragraphs:
            return []

        # 2. Convert paragraphs to latent vectors (embeddings)
        # convert_to_tensor=True keeps the vectors on the GPU/CPU for faster math
        paragraph_embeddings = self.model.encode(paragraphs, convert_to_tensor=True)

        # 3. Convert the search query to a latent vector
        query_embedding = self.model.encode(query, convert_to_tensor=True)

        # 4. Compute Cosine Similarity
        # util.cos_sim calculates the similarity of the query against all paragraphs at once
        cosine_scores = util.cos_sim(query_embedding, paragraph_embeddings)[0]

        # 5. Retrieve the top_k highest scoring results
        top_results = torch.topk(cosine_scores, k=min(top_k, len(paragraphs)))

        # 6. Format and return the results
        results = []
        for score, idx in zip(top_results[0], top_results[1]):
            results.append({
                'score': score.item(),
                'text': paragraphs[idx]
            })

        return results

if __name__ == '__main__':
    # Set up command line interface
    parser = argparse.ArgumentParser(description="Semantic Document Search using Latent Vectors")
    parser.add_argument('--query', type=str, required=True, help="The concept or sentence to search for")
    parser.add_argument('--file', type=str, required=True, help="Path to the large text file to search through")
    parser.add_argument('--top_k', type=int, default=3, help="Number of top matching paragraphs to return")
    parser.add_argument('--separator', type=str, default='\n', help="String used to split the text into chunks")
    parser.add_argument('--model', type=str, default='all-MiniLM-L6-v2', help="HuggingFace Sentence Transformer model name")

    args = parser.parse_args()

    # Read the target document
    try:
        with open(args.file, 'r', encoding='utf-8') as f:
            document_text = f.read()
    except FileNotFoundError:
        print(f"Error: Could not find file '{args.file}'")
        exit(1)

    # Execute search
    print(f"Loading model '{args.model}' and processing text...")
    searcher = SemanticSearcher(model_name=args.model)
    matches = searcher.search(args.query, document_text, top_k=args.top_k, separator=args.separator)

    # Display results
    print(f"\n--- Top {args.top_k} Matches for: '{args.query}' ---\n")
    for i, match in enumerate(matches, 1):
        print(f"[{i}] Similarity Score: {match['score']:.4f}")
        print(f"{match['text']}\n")
        print("-" * 40 + "\n")
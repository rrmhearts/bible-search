# Enhanced Bible Search Tool

A powerful Rust-based command-line tool for searching the Bible with synonym support, color-coded results, and flexible search options.

## Features

### 🔍 **Enhanced Search Capabilities**
- **Text Search**: Search for words or phrases across all verses
- **Synonym Support**: Automatically include synonyms in your search (e.g

## Installation

1. Clone the repository
2. Build with Cargo:
   ```bash
   cargo build --release
   ```

## Usage

### Command Line Mode

#### Search Examples

```bash
# Basic text search
./bible_tool --search "love"

# Search with synonyms
./bible_tool --search "god" --synonyms

# Case-sensitive search
./bible_tool --search "Love" --case-sensitive

# Search within specific book
./bible_tool --search "peace" --book "Psalms"

# Limit results
./bible_tool --search "faith" --limit 5

# Different output formats
./bible_tool --search "hope" --format json
./bible_tool --search "joy" --format verse-only
```

#### Reference Lookup

```bash
# Look up specific verse
./bible_tool --reference "John 3:16"

# Look up entire chapter
./bible_tool --reference "Genesis 1"

# Look up all verses from a book
./bible_tool --reference "Jude"
```

#### Random Verse

```bash
./bible_tool --random
```

### Interactive Mode

```bash
# Start interactive mode
./bible_tool --interactive
# or simply
./bible_tool

# Interactive commands:
> search love --synonyms
> ref John 3:16
> random
> help
> quit
```

### Advanced Examples

```bash
# Search for wisdom with synonyms, case-sensitive, in Proverbs only
./bible_tool -s "wisdom" --synonyms --case-sensitive -b "Proverbs" -l 10

# Get JSON output for all verses containing "faith"
./bible_tool -s "faith" --format json

# Use custom Bible file
./bible_tool -f /path/to/custom/bible.json -s "salvation"
```

## Bible Data Format

The tool expects a JSON file with the following structure:

```json
{
  "verses": [
    {
      "book": "Genesis",
      "chapter": 1,
      "verse": 1,
      "text": "In the beginning God created the heavens and the earth."
    }
  ]
}
```

## Synonym Mapping

The tool includes built-in synonyms for common biblical terms:

- **God**: lord, almighty, creator, father, jehovah, yahweh
- **Jesus**: christ, savior, redeemer, messiah, son
- **Love**: beloved, charity, affection, devotion
- **Peace**: tranquil, calm, serenity, rest
- **Joy**: happiness, gladness, delight, rejoice
- **Wisdom**: knowledge, understanding, insight, prudence
- **Faith**: belief, trust, confidence, hope

## Command Line Options

| Option | Short | Description |
|--------|-------|-------------|
| `--file` | `-f` | Path to Bible JSON file (default: bible.json) |
| `--search` | `-s` | Search for text in verses |
| `--reference` | `-r` | Look up verse by reference |
| `--random` |  | Get a random verse |
| `--synonyms` |  | Include synonyms in search |
| `--case-sensitive` | `-c` | Case sensitive search |
| `--book` | `-b` | Filter results to specific book |
| `--limit` | `-l` | Limit number of results |
| `--format` |  | Output format: text, json, or verse-only |
| `--interactive` | `-i` | Start in interactive mode |

## Examples with Output

### Basic Search
```bash
$ ./bible_tool -s "love" -l 2
Found 2 result(s):

John 3:16 - For God so loved the world that he gave his one and only Son, that whoever believes in him shall not perish but have eternal life.

1 Corinthians 13:4 - Love is patient, love is kind. It does not envy, it does not boast, it is not proud.
```

### Synonym Search
```bash
$ ./bible_tool -s "god" --synonyms -l 3
Found 3 result(s):

Genesis 1:1 - In the beginning God created the heavens and the earth.

Psalms 23:1 - The Lord is my shepherd, I lack nothing.

John 3:16 - For God so loved the world that he gave his one and only Son, that whoever believes in him shall not perish but have eternal life.
```

### Reference Lookup
```bash
$ ./bible_tool -r "Psalms 23:1"
Found 1 result(s):

Psalms 23:1 - The Lord is my shepherd, I lack nothing.
```

## Building from Source

```bash
git clone <your-repo>
cd bible_tool
cargo build --release
```

The binary will be available at `target/release/bible_tool`.

## Testing

Run tests with:
```bash
cargo test
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

[Your License Here]

## Roadmap

- [ ] Additional synonym dictionaries
- [ ] Fuzzy search support
- [ ] Multiple Bible translation support
- [ ] Search result highlighting
- [ ] Export functionality
- [ ] Web interface
- [ ] Configuration file support
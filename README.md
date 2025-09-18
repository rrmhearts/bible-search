# Bible Search Tool

This project is for searching and looking up verses with synonyms and other features that are not available in most public Bible search websites. The goal is progressive growth and practicing Rust language for terminal applications. Currently, this repository includes an English Revised Version (ERV) Bible because of its public access. Hope to extend functionality to more versions in the future.

## Command Line Usage

### Basic Search
```bash
# Simple text search
./bible_tool --search "love"

# Search with synonyms (expands "god" to include "lord", "almighty", etc.)
./bible_tool --search "god" --synonyms

# Case-sensitive search
./bible_tool --search "Love" --case-sensitive

# Search within specific book
./bible_tool --search "peace" --book "Psalms"

# Limit results to first 5 matches
./bible_tool --search "faith" --limit 5
```

### Reference Lookup
```bash
# Look up specific verse
./bible_tool --reference "John 3:16"

# Look up with short form
./bible_tool -r "Genesis 1:1"
```

### Random Verse
```bash
# Get a random verse
./bible_tool --random
```

### Interactive Mode
```bash
# Start interactive mode (original menu system)
./bible_tool --interactive

# Or just run without arguments
./bible_tool
```

### Advanced Examples
```bash
# Search for "jesus" with synonyms, case-sensitive, in Gospel of John only, limit to 3 results
./bible_tool -s "jesus" --synonyms --case-sensitive -b "John" -l 3

# Use custom Bible file
./bible_tool -f /path/to/my_bible.txt -s "salvation"

# Disable colors for scripting
./bible_tool --search "hope" --no-color
```

## Example Output

### Search with Synonyms
```bash
$ ./bible_tool --search "god" --synonyms --limit 3

Loading Bible from bible.txt...
✅ Bible loaded successfully (15 verses).
Searching for 'god' (with synonyms: almighty, creator, father, god, jehovah, lord, most, yahweh)...

Genesis 1:1 In the beginning God created the heaven and the earth.
Genesis 1:2 And the earth was waste and void; and darkness was upon the face of the deep: and the spirit of God moved upon the face of the waters.
Psalms 23:1 The LORD is my shepherd; I shall not want.

Found 3 matching verses.
```

### Reference Lookup
```bash
$ ./bible_tool --reference "John 3:16"

Loading Bible from bible.txt...
✅ Bible loaded successfully (15 verses).
John 3:16 For God so loved the world, that he gave his only begotten Son, that whosoever believeth on him should not perish, but have eternal life.
```

### Interactive Mode
```bash
$ ./bible_tool --interactive

Loading Bible from bible.txt...
✅ Bible loaded successfully (15 verses).

=== Interactive Bible Search Tool ===

--- Bible Tool Menu ---
1. Lookup Verse (e.g., Genesis 1:1)
2. Search Text
3. Exit
> 2
Enter search query: love
Use synonyms? (y/n): y
Searching for 'love' (with synonyms: affection, beloved, charity, love, loved, loveth)...

John 3:16 For God so loved the world, that he gave his only begotten Son, that whosoever believeth on him should not perish, but have eternal life.
Romans 8:28 And we know that to them that love God all things work together for good, even to them that are called according to his purpose.
1 Corinthians 13:4 Love suffereth long, and is kind; love envieth not; love vaunteth not itself, is not puffed up,

Found 3 matching verses.
```

## Built-in Synonyms

The tool includes these synonym groups:

- **God**: lord, almighty, creator, father, jehovah, yahweh, most
- **Jesus**: christ, savior, saviour, redeemer, messiah, son  
- **Love**: loved, loveth, beloved, charity, affection
- **Peace**: tranquil, calm, serenity, rest, quiet
- **Joy**: happiness, gladness, delight, rejoice, joyful
- **Wisdom**: knowledge, understanding, insight, prudence, wise
- **Faith**: belief, trust, confidence, hope, believe
- **Fear**: afraid, terror, dread, reverence
- **Sin**: transgression, iniquity, wickedness, evil
- **Salvation**: save, saved, deliverance, rescue

## File Format

Your Bible file should follow this format:
```
ERV
English Revised Version
Genesis 1:1	In the beginning God created the heaven and the earth.
Genesis 1:2	And the earth was waste and void; and darkness was upon the face of the deep: and the spirit of God moved upon the face of the waters.
...
```

- Line 1: Translation abbreviation
- Line 2: Full translation name  
- Line 3+: Reference[TAB]Text format

## Command Line Options

| Option | Short | Description |
|--------|-------|-------------|
| `--file` | `-f` | Path to Bible text file (default: bible.txt) |
| `--search` | `-s` | Search for text in verses |
| `--reference` | `-r` | Look up verse by reference |
| `--random` |  | Get a random verse |
| `--synonyms` |  | Include synonyms in search |
| `--case-sensitive` | `-c` | Case sensitive search |
| `--book` | `-b` | Filter results to specific book |
| `--limit` | `-l` | Limit number of results |
| `--no-color` |  | Disable colored output |
| `--interactive` | `-i` | Start in interactive mode |

## Features Added

✅ **Command-line interface** - No more menu system required  
✅ **Synonym support** - Automatically expand search terms  
✅ **Flexible search options** - Case sensitivity, book filtering, result limiting  
✅ **Preserved original functionality** - Interactive mode still available  
✅ **Enhanced highlighting** - Search terms highlighted in results  
✅ **Random verse feature** - Get inspiration with `--random`  
✅ **Backward compatibility** - Your existing bible.txt format works perfectly
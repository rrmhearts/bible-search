# Bible Search Tool

This project is for searching and looking up verses with synonyms and other features that are not available in most public Bible search websites. The goal is progressive growth and practicing Rust language for terminal applications. Currently, this repository defaults to an English Revised Version (ERV) Bible because of its public access. It also supports KJV (`--kjv`) and ASV (`--asv`). Hope to extend functionality to more versions in the future.

**Coffee is a proven love language.** If this has proven helpful to you,

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.paypal.com/paypalme/rrmhearts)

## Quick Start

### 1. Create Default Synonyms File
```bash
# Generate a synonyms.txt file with default biblical synonyms
./bible_tool --create-synonyms

# This creates synonyms.txt in the current directory
# You can now edit it to add/remove synonyms
```

### 2. Run Your First Search
```bash
# Search with synonym expansion
./bible_tool --search "god" --synonyms --limit 5
```

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

### Cross-References
```bash
# Find verses similar to John 3:16 (default 30% similarity threshold)
./bible_tool --cross-references "John 3:16"

# Use synonyms for better matching
./bible_tool --cross-references "John 3:16" --use-synonyms-xref

# Adjust similarity threshold (0.0 to 1.0)
./bible_tool -x "Psalms 23:1" --similarity 0.4

# Limit results
./bible_tool -x "Romans 8:28" --use-synonyms-xref --similarity 0.25 -l 10

# Short form
./bible_tool -x "Genesis 1:1"
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

# Use custom synonyms file
./bible_tool --synonyms-file /path/to/my_synonyms.txt -s "god" --synonyms

# Find cross-references with synonym matching
./bible_tool -x "John 3:16" --use-synonyms-xref --similarity 0.35 -l 5

# Disable colors for scripting
./bible_tool --search "hope" --no-color

# Use supported translation (KJV, ASV, ERV)
./bible_tool --search "believeth" --kjv
```

## Synonym File Management

### Creating the Default Synonyms File
```bash
# Create synonyms.txt with default biblical synonyms
./bible_tool --create-synonyms

# Create with custom filename
./bible_tool --synonyms-file my_synonyms.txt --create-synonyms
```

### Editing the Synonyms File
The synonyms file uses a simple format:
```
keyword: synonym1, synonym2, synonym3
```

Example `synonyms.txt`:
```
# Comments start with #
god: god, lord, almighty, creator, father
jesus: jesus, christ, savior, messiah, son
love: love, loved, beloved, charity, affection

# You can add your own groups
disciple: disciple, apostle, follower, believer
```

**Format Rules:**
- One synonym group per line
- Format: `keyword: synonym1, synonym2, synonym3`
- Keywords and synonyms are case-insensitive
- Lines starting with `#` are comments
- Empty lines are ignored
- Commas separate synonyms

### Adding New Synonyms
Simply edit `synonyms.txt` and add new lines:
```bash
# Open in your editor
nano synonyms.txt

# Add new entries like:
temple: temple, sanctuary, tabernacle, house
covenant: covenant, promise, agreement, testament
```

**No recompilation needed!** Changes take effect next time you run the tool.

### Removing Synonyms
- Delete entire lines to remove synonym groups
- Remove individual synonyms from comma-separated lists
- Comment out lines with `#` to temporarily disable them

## Example Output

### Search with Synonyms
```bash
$ ./bible_tool --search "god" --synonyms --limit 3

Loading Bible from bible.txt...
✅ Bible loaded successfully (31102 verses).
✅ Loaded 18 synonym groups from synonyms.txt
Searching for 'god' (with synonyms: almighty, creator, father, god, high, jehovah, lord, most, yahweh)...

Genesis 1:1 In the beginning God created the heaven and the earth.
Genesis 1:2 And the earth was waste and void; and darkness was upon the face of the deep: and the spirit of God moved upon the face of the waters.
Psalms 23:1 The LORD is my shepherd; I shall not want.

Found 3 matching verses.
```

### Cross-References
```bash
$ ./bible_tool --cross-references "John 3:16" --use-synonyms-xref -l 5

Loading Bible from bible.txt...
✅ Bible loaded successfully (31102 verses).
✅ Loaded 18 synonym groups from synonyms.txt
Source Verse:
John 3:16 For God so loved the world, that he gave his only begotten Son, that whosoever believeth on him should not perish, but have eternal life.

Found 5 cross-reference(s) with similarity >= 30.0%:
(Using synonym matching)

45.2% - John 3:17 For God sent not the Son into the world to judge the world; but that the world should be saved through him.

38.5% - 1 John 4:9 Herein was the love of God manifested in us, that God hath sent his only begotten Son into the world, that we might live through him.

35.7% - Romans 5:8 But God commendeth his own love toward us, in that, while we were yet sinners, Christ died for us.

32.1% - 1 John 4:10 Herein is love, not that we loved God, but that he loved us, and sent his Son to be the propitiation for our sins.

30.4% - Ephesians 2:4 But God, being rich in mercy, for his great love wherewith he loved us.

```

### Reference Lookup
```bash
$ ./bible_tool --reference "John 3:16"

Loading Bible from bible.txt...
✅ Bible loaded successfully (31102 verses).
✅ Loaded 18 synonym groups from synonyms.txt
John 3:16 For God so loved the world, that he gave his only begotten Son, that whosoever believeth on him should not perish, but have eternal life.
```

### Interactive Mode
```bash
$ ./bible_tool --interactive

Loading Bible from bible.txt...
✅ Bible loaded successfully (31102 verses).
✅ Loaded 18 synonym groups from synonyms.txt

=== Interactive Bible Search Tool ===

--- Bible Tool Menu ---
1. Lookup Verse (e.g., Genesis 1:1)
2. Search Text
3. Exit
> 2
Enter search query: love
Use synonyms? (y/n): y
Searching for 'love' (with synonyms: affection, beloved, charity, devotion, love, loved, loveth)...

John 3:16 For God so loved the world, that he gave his only begotten Son, that whosoever believeth on him should not perish, but have eternal life.
Romans 8:28 And we know that to them that love God all things work together for good, even to them that are called according to his purpose.
1 Corinthians 13:4 Love suffereth long, and is kind; love envieth not; love vaunteth not itself, is not puffed up,

Found 3 matching verses.
```

### When Synonyms File is Missing
```bash
$ ./bible_tool --search "god" --synonyms

Loading Bible from bible.txt...
✅ Bible loaded successfully (31102 verses).
⚠️  Could not load synonyms file (synonyms.txt): No such file or directory (os error 2)
   Using exact word matching only.
   Run with --create-synonyms to create a default synonyms file.
Searching for 'god' (no synonyms defined for these terms)...

Genesis 1:1 In the beginning God created the heaven and the earth.
...
```

## Built-in Default Synonyms

When you run `--create-synonyms`, the following synonym groups are created:

**Deity References:**
- god: god, lord, almighty, creator, father, jehovah, yahweh, most high
- jesus: jesus, christ, savior, saviour, redeemer, messiah, son, lamb

**Spiritual Concepts:**
- love: love, loved, loveth, beloved, charity, affection, devotion
- peace: peace, tranquil, calm, serenity, rest, quiet, still
- joy: joy, happiness, gladness, delight, rejoice, joyful, glad
- wisdom: wisdom, knowledge, understanding, insight, prudence, wise, discernment
- faith: faith, belief, trust, confidence, hope, believe, believing
- fear: fear, afraid, terror, dread, reverence, awe

**Sin and Salvation:**
- sin: sin, transgression, iniquity, wickedness, evil, trespass
- salvation: salvation, save, saved, deliverance, rescue, redeem, redeemed

**Virtues:**
- righteousness: righteousness, righteous, just, justice, upright
- mercy: mercy, merciful, compassion, compassionate, grace, gracious
- truth: truth, true, truthful, verity, honest, honesty

**Actions:**
- praise: praise, worship, glorify, exalt, magnify, honor
- prayer: prayer, pray, petition, supplication, intercession
- repent: repent, repentance, turn, return, humble

**Additional Concepts:**
- spirit: spirit, soul, heart, mind
- word: word, words, scripture, law, commandment, testimony
- kingdom: kingdom, reign, dominion, rule

## Tips for Creating Custom Synonyms

### 1. Topical Groups
Create synonym groups for specific topics you study:
```
healing: healing, heal, cured, restored, whole, wholeness
blessing: blessing, blessed, bless, favor, favored
prophet: prophet, seer, prophesy, prophesied, foretold
```

### 2. Character Names
Group names and titles together:
```
moses: moses, lawgiver
david: david, psalmist, sweet psalmist
paul: paul, saul, apostle paul
```

### 3. Old English Variants
Help find verses in older translations:
```
thou: thou, you, thee, thy, thine, your, yours
shall: shall, will, should, would
saith: saith, says, said, speaks, spoke
```

### 4. Multiple Translations
Include variations from different translations:
```
redemption: redemption, ransom, bought, purchased, redeemed
sanctification: sanctification, holiness, sanctified, holy, set apart
```

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
| `--kjv` | | Use KJV. Equivalent to `--file bibles/kjv.txt`. |
| `--asv` | | Use ASV. Equivalent to `--file bibles/asv.txt`. |
| `--erv` | | Use ERV. Equivalent to `--file bibles/erv.txt`. |
| `--synonyms-file` |  | Path to synonyms configuration file (default: synonyms.txt) |
| `--create-synonyms` |  | Create default synonyms file and exit |
| `--search` | `-s` | Search for text in verses |
| `--reference` | `-r` | Look up verse by reference |
| `--cross-references` | `-x` | Find cross-references for a verse |
| `--similarity` |  | Similarity threshold for cross-references (0.0-1.0, default: 0.3) |
| `--use-synonyms-xref` |  | Use synonyms when calculating cross-reference similarity |
| `--random` |  | Get a random verse |
| `--synonyms` |  | Include synonyms in search |
| `--case-sensitive` | `-c` | Case sensitive search |
| `--book` | `-b` | Filter results to specific book |
| `--limit` | `-l` | Limit number of results |
| `--no-color` |  | Disable colored output |
| `--interactive` | `-i` | Start in interactive mode |

## Workflow Examples

### Setting Up Your Custom Bible Study Tool

1. **Initial Setup**
```bash
# Create your synonyms file
./bible_tool --create-synonyms

# Edit it for your needs
nano synonyms.txt
```

2. **Add Study-Specific Synonyms**
```bash
# For a study on prayer, add:
echo "intercession: intercession, mediation, advocate, mediator" >> synonyms.txt

# For a study on spiritual gifts:
echo "gifts: gifts, gift, talents, abilities, endowments" >> synonyms.txt
```

3. **Search with Your Custom Synonyms**
```bash
./bible_tool --search "prayer" --synonyms -l 10
```

### Comparative Translation Study

Create different synonym files for different translations:
```bash
# Create KJV-specific synonyms
./bible_tool --synonyms-file synonyms_kjv.txt --create-synonyms

# Edit to add KJV-specific terms
echo "charity: charity, love, agape" >> synonyms_kjv.txt
echo "conversation: conversation, conduct, behavior, manner of life" >> synonyms_kjv.txt

# Use with KJV Bible
./bible_tool -f kjv_bible.txt --synonyms-file synonyms_kjv.txt -s "charity" --synonyms
```

### Quick Scripture Finder Script

Create a shell script for common searches:
```bash
#!/bin/bash
# search_bible.sh

if [ "$1" == "" ]; then
    echo "Usage: search_bible.sh <query> [limit]"
    exit 1
fi

LIMIT=${2:-10}
./bible_tool --search "$1" --synonyms --limit $LIMIT
```

Usage:
```bash
chmod +x search_bible.sh
./search_bible.sh "faith" 5
```

### Cross-Reference Study

```bash
# Find related verses for study
./bible_tool -x "Romans 8:28" --use-synonyms-xref --similarity 0.3 -l 10 > related_verses.txt

# Compare cross-references with different thresholds
./bible_tool -x "John 3:16" --similarity 0.5 -l 5  # Very similar only
./bible_tool -x "John 3:16" --similarity 0.2 -l 20 # Broader matches
```

## Troubleshooting

### Synonyms Not Working
**Problem:** Search with `--synonyms` doesn't expand terms

**Solutions:**
1. Check if synonyms.txt exists: `ls -l synonyms.txt`
2. Verify file format (keyword: synonym1, synonym2)
3. Create default file: `./bible_tool --create-synonyms`
4. Check for typos in keyword (must match your search term exactly)

### Can't Find Verses
**Problem:** Expected verses not appearing in results

**Solutions:**
1. Try without `--case-sensitive` flag
2. Add more synonyms to your synonyms.txt
3. Check spelling of search terms
4. Use broader terms (search "heal" instead of "healing")

### Highlighting Issues
**Problem:** Colors not showing or broken in terminal

**Solutions:**
1. Use `--no-color` flag to disable colors
2. Check terminal supports ANSI colors
3. Try different terminal emulator

## Advanced Use Cases

### Topical Bible Creation
```bash
# Create topical synonym groups in synonyms.txt
# Then search and redirect to files

./bible_tool -s "salvation" --synonyms --no-color > salvation_verses.txt
./bible_tool -s "faith" --synonyms --no-color > faith_verses.txt
./bible_tool -s "love" --synonyms --no-color > love_verses.txt
```

### Daily Verse Scripture
Add to crontab for daily random verse:
```bash
# Edit crontab
crontab -e

# Add line (runs at 8 AM daily)
0 8 * * * /path/to/bible_tool --random >> ~/daily_verse.txt
```

### Sermon Preparation
```bash
# Find all verses on a topic
./bible_tool -s "forgiveness" --synonyms > sermon_refs.txt

# Find cross-references for key verses
./bible_tool -x "Matthew 6:14" --use-synonyms-xref -l 10 > forgiveness_xrefs.txt

# Find specific passages
./bible_tool -r "Matthew 5:1"
./bible_tool -r "Luke 15:11"
```

## Features

✅ **External synonym configuration** - No recompilation needed to add/remove synonyms  
✅ **Simple text file format** - Easy to edit with any text editor  
✅ **Default synonym creation** - Quick start with `--create-synonyms`  
✅ **Custom synonym files** - Use `--synonyms-file` for different configurations  
✅ **Cross-reference finder** - Find similar verses with adjustable similarity threshold  
✅ **Synonym-based similarity** - Use `--use-synonyms-xref` for semantic matching  
✅ **Graceful fallback** - Works without synonym file (exact matching only)  
✅ **Comment support** - Document your synonym choices with `#` comments  
✅ **Case-insensitive** - Keywords and synonyms work regardless of case  
✅ **Command-line interface** - Direct execution via arguments  
✅ **Interactive mode** - Menu-driven interface still available  
✅ **Flexible search options** - Case sensitivity, book filtering, result limiting  
✅ **Enhanced highlighting** - Search terms highlighted in color  
✅ **Random verse feature** - Get inspiration with `--random`  
✅ **Backward compatibility** - Your existing bible.txt format works perfectly

## File Structure

Your working directory should look like:
```
my_bible_study/
├── bible_tool          # The compiled executable
├── bible.txt           # Your Bible text file (ERV format)
└── synonyms.txt        # Your synonym configuration (create with --create-synonyms)
```

Optional additional files:
```
my_bible_study/
├── bible_tool
├── bible.txt
├── synonyms.txt        # Default synonyms
├── synonyms_kjv.txt    # KJV-specific synonyms
├── synonyms_study.txt  # Topical study synonyms
└── README.md
```

## Building from Source

```bash
# Clone the repository
git clone <your-repo>
cd bible_tool

# Build with Cargo
cargo build --release

# The binary will be at target/release/bible_tool
```

## License

MIT License


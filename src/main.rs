use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;
use colored::*;
use clap::{Arg, Command};

// Structure to hold a single Bible verse.
#[derive(Debug, Clone)]
struct Verse {
    book: String,
    chapter: u32,
    verse: u32,
    text: String,
}

impl std::fmt::Display for Verse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}:{} {}",
            self.book.cyan(),
            self.chapter.to_string().cyan(),
            self.verse.to_string().cyan(),
            self.text
        )
    }
}

// Synonym mapper for enhanced search
struct SynonymMapper {
    synonyms: HashMap<String, Vec<String>>,
}

impl SynonymMapper {
    fn new() -> Self {
        SynonymMapper {
            synonyms: HashMap::new(),
        }
    }
    
    fn load_from_file(filename: &str) -> io::Result<Self> {
        let mut mapper = Self::new();
        
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Parse format: key: synonym1, synonym2, synonym3
            if let Some((key, values)) = line.split_once(':') {
                let key = key.trim().to_lowercase();
                let synonyms: Vec<String> = values
                    .split(',')
                    .map(|s| s.trim().to_lowercase())
                    .filter(|s| !s.is_empty())
                    .collect();
                
                if !synonyms.is_empty() {
                    mapper.synonyms.insert(key, synonyms);
                }
            }
        }
        
        Ok(mapper)
    }
    
    fn create_default_file(filename: &str) -> io::Result<()> {
        use std::fs;
        
        let default_content = r#"# Bible Search Tool - Synonym Configuration
# Format: keyword: synonym1, synonym2, synonym3
# Lines starting with # are comments and will be ignored
# Keywords and synonyms are case-insensitive

# Deity references
god: god, lord, almighty, creator, father, jehovah, yahweh, most high
jesus: jesus, christ, savior, saviour, redeemer, messiah, son, lamb

# Spiritual concepts
love: love, loved, loveth, beloved, charity, affection, devotion
peace: peace, tranquil, calm, serenity, rest, quiet, still
joy: joy, happiness, gladness, delight, rejoice, joyful, glad
wisdom: wisdom, knowledge, understanding, insight, prudence, wise, discernment
faith: faith, belief, trust, confidence, hope, believe, believing
fear: fear, afraid, terror, dread, reverence, awe

# Sin and salvation
sin: sin, transgression, iniquity, wickedness, evil, trespass
salvation: salvation, save, saved, deliverance, rescue, redeem, redeemed

# Virtues
righteousness: righteousness, righteous, just, justice, upright
mercy: mercy, merciful, compassion, compassionate, grace, gracious
truth: truth, true, truthful, verity, honest, honesty

# Actions
praise: praise, worship, glorify, exalt, magnify, honor
prayer: prayer, pray, petition, supplication, intercession
repent: repent, repentance, turn, return, humble

# Additional concepts
spirit: spirit, soul, heart, mind
word: word, words, scripture, law, commandment, testimony
kingdom: kingdom, reign, dominion, rule
"#;
        
        fs::write(filename, default_content)?;
        Ok(())
    }
    
    fn expand_query(&self, query: &str) -> Vec<String> {
        let words: Vec<&str> = query.split_whitespace().collect();
        let mut expanded_terms = Vec::new();
        
        for word in &words {
            let clean_word = word.to_lowercase().trim_matches(|c: char| !c.is_alphabetic()).to_string();
            if let Some(synonyms) = self.synonyms.get(&clean_word) {
                expanded_terms.extend(synonyms.clone());
            } else {
                expanded_terms.push(clean_word);
            }
        }
        
        // Remove duplicates
        expanded_terms.sort();
        expanded_terms.dedup();
        expanded_terms
    }
    
    fn get_synonym_count(&self) -> usize {
        self.synonyms.len()
    }
}

// Create command line interface
fn create_cli() -> Command {
    Command::new("bible_tool")
        .version("2.0.1")
        .author("Your Name")
        .about("Enhanced Bible search tool with synonym support")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .value_name("FILE")
            .help("Path to Bible text file")
            .default_value("bibles/bible.txt"))
        .arg(Arg::new("kjv")
            .long("kjv")
            .help("Use the King James Version (bibles/kjv.txt)")
            .action(clap::ArgAction::SetTrue)
            .conflicts_with_all(&["file", "erv", "asv"]))
        .arg(Arg::new("erv")
            .long("erv")
            .help("Use the English Revised Version (bibles/erv.txt)")
            .action(clap::ArgAction::SetTrue)
            .conflicts_with_all(&["file", "kjv", "asv"]))
        .arg(Arg::new("asv")
            .long("asv")
            .help("Use the American Standard Version (bibles/asv.txt)")
            .action(clap::ArgAction::SetTrue)
            .conflicts_with_all(&["file", "kjv", "erv"]))
        .arg(Arg::new("synonyms-file")
            .long("synonyms-file")
            .value_name("FILE")
            .help("Path to synonyms configuration file")
            .default_value("synonyms.txt"))
        .arg(Arg::new("create-synonyms")
            .long("create-synonyms")
            .help("Create default synonyms file and exit")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("search")
            .short('s')
            .long("search")
            .value_name("QUERY")
            .help("Search for text in verses")
            .conflicts_with_all(&["reference", "random"]))
        .arg(Arg::new("reference")
            .short('r')
            .long("reference")
            .value_name("REFERENCE")
            .help("Look up verse by reference (e.g., 'John 3:16')")
            .conflicts_with_all(&["search", "random"]))
        .arg(Arg::new("random")
            .long("random")
            .help("Get a random verse")
            .action(clap::ArgAction::SetTrue)
            .conflicts_with_all(&["search", "reference"]))
        .arg(Arg::new("synonyms")
            .long("synonyms")
            .help("Include synonyms in search")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("case-sensitive")
            .short('c')
            .long("case-sensitive")
            .help("Case sensitive search")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("book")
            .short('b')
            .long("book")
            .value_name("BOOK")
            .help("Filter results to specific book"))
        .arg(Arg::new("limit")
            .short('l')
            .long("limit")
            .value_name("NUMBER")
            .help("Limit number of results")
            .value_parser(clap::value_parser!(usize)))
        .arg(Arg::new("no-color")
            .long("no-color")
            .help("Disable colored output")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("interactive")
            .short('i')
            .long("interactive")
            .help("Start in interactive mode")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("cross-references")
            .short('x')
            .long("cross-references")
            .value_name("REFERENCE")
            .help("Find cross-references for a verse (e.g., 'John 3:16')")
            .conflicts_with_all(&["search", "random"]))
        .arg(Arg::new("similarity")
            .long("similarity")
            .value_name("THRESHOLD")
            .help("Similarity threshold for cross-references (0.0-1.0, default: 0.3)")
            .value_parser(clap::value_parser!(f32))
            .default_value("0.3"))
        .arg(Arg::new("use-synonyms-xref")
            .long("use-synonyms-xref")
            .help("Use synonyms when calculating cross-reference similarity")
            .action(clap::ArgAction::SetTrue))
}

// Main function to run the application logic.
fn main() {
    let matches = create_cli().get_matches();
    
    let synonyms_file = matches.get_one::<String>("synonyms-file").unwrap();
    
    // Handle --create-synonyms flag
    if matches.get_flag("create-synonyms") {
        match SynonymMapper::create_default_file(synonyms_file) {
            Ok(_) => {
                println!("{} Created default synonyms file: {}", "✅".green(), synonyms_file);
                println!("You can now edit this file to customize your synonyms.");
                return;
            }
            Err(e) => {
                eprintln!("{} Error creating synonyms file: {}", "🔥".red(), e);
                return;
            }
        }
    }
    
    // Shortened Bible selection text to version name --kjv or --erv
    let bible_file = if matches.get_flag("kjv") {
        "bibles/kjv.txt"
    } else if matches.get_flag("erv") {
        "bibles/erv.txt"
    } else if matches.get_flag("asv") {
        "bibles/asv.txt"
    } else {
        // Fallback to the --file argument if no version flag is used
        matches.get_one::<String>("file").unwrap()
    };

    let use_color = !matches.get_flag("no-color");
    
    println!("Loading Bible from {}...", bible_file);
    
    // Load all verses from the file into memory.
    let bible = match load_bible(bible_file) {
        Ok(verses) => {
            println!("✅ Bible loaded successfully ({} verses).", verses.len());
            verses
        }
        Err(e) => {
            eprintln!("🔥 Error loading {}: {}", bible_file, e);
            eprintln!("Please ensure the file exists and has the correct format.");
            return;
        }
    };
    
    // Load synonyms from file
    let synonym_mapper = match SynonymMapper::load_from_file(synonyms_file) {
        Ok(mapper) => {
            if mapper.get_synonym_count() > 0 {
                println!("✅ Loaded {} synonym groups from {}", mapper.get_synonym_count(), synonyms_file);
            } else {
                println!("⚠️  No synonyms loaded from {}. Using exact word matching only.", synonyms_file);
            }
            mapper
        }
        Err(e) => {
            println!("⚠️  Could not load synonyms file ({}): {}", synonyms_file, e);
            println!("   Using exact word matching only.");
            println!("   Run with --create-synonyms to create a default synonyms file.");
            SynonymMapper::new()
        }
    };

    // Check if interactive mode is requested or no arguments provided
    if matches.get_flag("interactive") || 
       (!matches.contains_id("search") && !matches.contains_id("reference") && 
        !matches.get_flag("random") && !matches.contains_id("cross-references")) {
        interactive_mode(&bible, &synonym_mapper);
        return;
    }

    // Handle different command modes
    if matches.get_flag("random") {
        get_random_verse(&bible);
    } else if let Some(query) = matches.get_one::<String>("search") {
        let use_synonyms = matches.get_flag("synonyms");
        let case_sensitive = matches.get_flag("case-sensitive");
        let book_filter = matches.get_one::<String>("book").map(|s| s.as_str());
        let limit = matches.get_one::<usize>("limit").copied();
        
        search_bible_cli(&bible, &synonym_mapper, query, use_synonyms, case_sensitive, book_filter, limit, use_color);
    } else if let Some(reference) = matches.get_one::<String>("reference") {
        lookup_verse_cli(&bible, reference);
    } else if let Some(reference) = matches.get_one::<String>("cross-references") {
        let similarity_threshold = *matches.get_one::<f32>("similarity").unwrap();
        let use_synonyms = matches.get_flag("use-synonyms-xref");
        let limit = matches.get_one::<usize>("limit").copied();
        
        find_cross_references(&bible, &synonym_mapper, reference, similarity_threshold, use_synonyms, limit, use_color);
    }
}

// Interactive mode (original menu system)
fn interactive_mode(bible: &[Verse], synonym_mapper: &SynonymMapper) {
    println!("\n{}", "=== Interactive Bible Search Tool ===".bright_cyan().bold());
    
    // Main application loop.
    loop {
        print_menu();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => lookup_verse(bible),
            "2" => search_bible_interactive(bible, synonym_mapper),
            "3" => {
                println!("Goodbye! 🙏");
                break;
            }
            _ => println!("{}", "Invalid choice, please try again.".red()),
        }
    }
}

// Displays the main menu options.
fn print_menu() {
    println!("\n--- Bible Tool Menu ---");
    println!("1. Lookup Verse (e.g., Genesis 1:1)");
    println!("2. Search Text");
    println!("3. Exit");
    print!("> ");
    io::stdout().flush().unwrap();
}

// Parses the bible.txt file and returns a Vector of Verse structs.
fn load_bible(filename: &str) -> io::Result<Vec<Verse>> {
    // We use lazy_static to compile the regex only once.
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<book>.+?)\s(?P<chapter>\d+):(?P<verse>\d+)\t(?P<text>.+)$").unwrap();
    }

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut bible = Vec::new();

    // Skip the first two header lines.
    for line in reader.lines().skip(2) {
        let line = line?;
        if let Some(caps) = RE.captures(&line) {
            let verse = Verse {
                book: caps["book"].to_string(),
                chapter: caps["chapter"].parse().unwrap_or(0),
                verse: caps["verse"].parse().unwrap_or(0),
                text: caps["text"].to_string(),
            };
            bible.push(verse);
        }
    }
    Ok(bible)
}

// CLI version of verse lookup
fn lookup_verse_cli(bible: &[Verse], reference: &str) {
    lazy_static! {
        static ref LOOKUP_RE: Regex = Regex::new(r"^(?P<book>.+?)\s(?P<chapter>\d+):(?P<verse>\d+)$").unwrap();
    }

    if let Some(caps) = LOOKUP_RE.captures(reference.trim()) {
        let book = &caps["book"];
        let chapter: u32 = caps["chapter"].parse().unwrap();
        let verse: u32 = caps["verse"].parse().unwrap();

        // Find the verse in our loaded Bible data.
        let found_verse = bible.iter().find(|v| {
            v.book.eq_ignore_ascii_case(book) && v.chapter == chapter && v.verse == verse
        });

        match found_verse {
            Some(v) => println!("{}", v),
            None => println!("{}", "Verse not found.".red()),
        }
    } else {
        println!("{}", "Invalid reference format. Please use 'Book Chapter:Verse'.".red());
    }
}

// Original interactive functionality to look up a specific verse.
fn lookup_verse(bible: &[Verse]) {
    print!("Enter reference (e.g., John 3:16): ");
    io::stdout().flush().unwrap();

    let mut reference = String::new();
    io::stdin().read_line(&mut reference).expect("Failed to read line");

    lookup_verse_cli(bible, &reference);
}

// Enhanced CLI search with synonyms
fn search_bible_cli(bible: &[Verse], synonym_mapper: &SynonymMapper, query: &str, use_synonyms: bool, case_sensitive: bool, book_filter: Option<&str>, limit: Option<usize>, use_color: bool) {
    if query.trim().is_empty() {
        println!("{}", "Search query cannot be empty.".yellow());
        return;
    }

    let search_terms = if use_synonyms {
        synonym_mapper.expand_query(query)
    } else {
        query.split_whitespace().map(|s| s.to_string()).collect()
    };

    if use_synonyms && search_terms.len() > query.split_whitespace().count() {
        println!("Searching for '{}' (with synonyms: {})...", query, search_terms.join(", "));
    } else if use_synonyms {
        println!("Searching for '{}' (no synonyms defined for these terms)...", query);
    } else {
        println!("Searching for '{}'...", query);
    }

    let mut results_found = 0;
    let mut results = Vec::new();

    for verse in bible {
        // Apply book filter if specified
        if let Some(book) = book_filter {
            if !verse.book.to_lowercase().contains(&book.to_lowercase()) {
                continue;
            }
        }

        let text_to_search = if case_sensitive {
            verse.text.clone()
        } else {
            verse.text.to_lowercase()
        };

        // Check if any search term matches
        let matches = search_terms.iter().any(|term| {
            if case_sensitive {
                verse.text.contains(term)
            } else {
                text_to_search.contains(&term.to_lowercase())
            }
        });

        if matches {
            results.push(verse);
            results_found += 1;
            
            // Apply limit if specified
            if let Some(limit) = limit {
                if results_found >= limit {
                    break;
                }
            }
        }
    }

    if results.is_empty() {
        println!("{}", "No results found.".red());
    } else {
        println!();
        for verse in results {
            // Create highlighted version of the text
            let mut highlighted_text = verse.text.clone();
            
            // Highlight matching terms
            if use_color {
                for term in &search_terms {
                    if case_sensitive {
                        if verse.text.contains(term) {
                            highlighted_text = highlighted_text.replace(term, &term.black().on_yellow().to_string());
                        }
                    } else {
                        // Case-insensitive highlighting is more complex
                        let lower_text = verse.text.to_lowercase();
                        let lower_term = term.to_lowercase();
                        if let Some(pos) = lower_text.find(&lower_term) {
                            let original_term = &verse.text[pos..pos + term.len()];
                            highlighted_text = highlighted_text.replace(original_term, &original_term.black().on_yellow().to_string());
                        }
                    }
                }
            }

            println!(
                "{} {}:{} {}",
                verse.book.cyan(),
                verse.chapter.to_string().cyan(),
                verse.verse.to_string().cyan(),
                highlighted_text
            );
        }
        println!("\nFound {} matching verses.", results_found);
    }
}

// Enhanced interactive search with synonyms
fn search_bible_interactive(bible: &[Verse], synonym_mapper: &SynonymMapper) {
    print!("Enter search query: ");
    io::stdout().flush().unwrap();

    let mut query = String::new();
    io::stdin().read_line(&mut query).expect("Failed to read line");
    let query = query.trim();

    if query.is_empty() {
        println!("{}", "Search query cannot be empty.".yellow());
        return;
    }

    // Ask about synonym usage
    print!("Use synonyms? (y/n): ");
    io::stdout().flush().unwrap();
    
    let mut synonym_choice = String::new();
    io::stdin().read_line(&mut synonym_choice).expect("Failed to read line");
    let use_synonyms = synonym_choice.trim().to_lowercase().starts_with('y');

    search_bible_cli(bible, synonym_mapper, query, use_synonyms, false, None, None, true);
}

// Get random verse
fn get_random_verse(bible: &[Verse]) {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
    let index = (hasher.finish() as usize) % bible.len();
    
    let verse = &bible[index];
    println!("{}", verse);
}

// Cross-reference finder - find similar verses
fn find_cross_references(bible: &[Verse], synonym_mapper: &SynonymMapper, reference: &str, similarity_threshold: f32, use_synonyms: bool, limit: Option<usize>, use_color: bool) {
    lazy_static! {
        static ref LOOKUP_RE: Regex = Regex::new(r"^(?P<book>.+?)\s(?P<chapter>\d+):(?P<verse>\d+)$").unwrap();
    }

    // Parse the reference
    let (book, chapter, verse_num) = if let Some(caps) = LOOKUP_RE.captures(reference.trim()) {
        let book = caps["book"].to_string();
        let chapter: u32 = caps["chapter"].parse().unwrap();
        let verse: u32 = caps["verse"].parse().unwrap();
        (book, chapter, verse)
    } else {
        println!("{}", "Invalid reference format. Please use 'Book Chapter:Verse'.".red());
        return;
    };

    // Find the source verse
    let source_verse = bible.iter().find(|v| {
        v.book.eq_ignore_ascii_case(&book) && v.chapter == chapter && v.verse == verse_num
    });

    let source_verse = match source_verse {
        Some(v) => v,
        None => {
            println!("{}", "Source verse not found.".red());
            return;
        }
    };

    // Display source verse
    if use_color {
        println!("{}", "Source Verse:".bright_green().bold());
    } else {
        println!("Source Verse:");
    }
    println!("{}\n", source_verse);

    // Extract words from source verse
    let source_words = extract_words(&source_verse.text, synonym_mapper, use_synonyms);
    
    if source_words.is_empty() {
        println!("{}", "No significant words found in source verse.".yellow());
        return;
    }

    // Calculate similarity for all other verses
    let mut similarities: Vec<(f32, &Verse)> = bible.iter()
        .filter(|v| {
            // Exclude the source verse itself
            !(v.book.eq_ignore_ascii_case(&source_verse.book) 
              && v.chapter == source_verse.chapter 
              && v.verse == source_verse.verse)
        })
        .map(|v| {
            let target_words = extract_words(&v.text, synonym_mapper, use_synonyms);
            let similarity = calculate_similarity(&source_words, &target_words);
            (similarity, v)
        })
        .filter(|(sim, _)| *sim >= similarity_threshold)
        .collect();

    // Sort by similarity (highest first)
    similarities.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    // Apply limit if specified
    if let Some(limit) = limit {
        similarities.truncate(limit);
    }

    if similarities.is_empty() {
        if use_color {
            println!("{}", format!("No cross-references found with similarity >= {:.1}%", similarity_threshold * 100.0).red());
        } else {
            println!("No cross-references found with similarity >= {:.1}%", similarity_threshold * 100.0);
        }
        println!("Try lowering the --similarity threshold (default: 0.3)");
        return;
    }

    // Display results
    if use_color {
        println!("{}", format!("Found {} cross-reference(s) with similarity >= {:.1}%:", 
            similarities.len(), similarity_threshold * 100.0).green().bold());
    } else {
        println!("Found {} cross-reference(s) with similarity >= {:.1}%:", 
            similarities.len(), similarity_threshold * 100.0);
    }
    
    if use_synonyms {
        println!("{}", "(Using synonym matching)".bright_black());
    }
    println!();

    for (similarity, verse) in similarities {
        let percentage = if use_color {
            format!("{:.1}%", similarity * 100.0).yellow().bold().to_string()
        } else {
            format!("{:.1}%", similarity * 100.0)
        };

        println!("{} - {} {}:{} {}", 
            percentage,
            verse.book.cyan(),
            verse.chapter.to_string().cyan(),
            verse.verse.to_string().cyan(),
            verse.text
        );
        println!();
    }
}

// Extract significant words from text, optionally expanding with synonyms
fn extract_words(text: &str, synonym_mapper: &SynonymMapper, use_synonyms: bool) -> Vec<String> {
    // Common words to exclude (stop words)
    let stop_words: std::collections::HashSet<&str> = [
        "a", "an", "and", "are", "as", "at", "be", "but", "by", "for", "from",
        "has", "he", "in", "is", "it", "its", "of", "on", "that", "the", "to",
        "was", "will", "with", "shall", "unto", "thee", "thou", "thy", "ye",
        "hath", "his", "her", "him", "them", "they", "their", "all", "not",
        "which", "there", "this", "these", "those", "when", "who", "what",
        "into", "upon", "out", "up", "have", "had", "do", "did", "done",
        "said", "came", "went", "been", "were", "being"
    ].iter().cloned().collect();

    let words: Vec<String> = text
        .to_lowercase()
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphabetic()))
        .filter(|w| !w.is_empty() && w.len() > 2 && !stop_words.contains(w))
        .map(|w| w.to_string())
        .collect();

    if use_synonyms {
        let mut expanded_words = Vec::new();
        for word in words {
            if let Some(synonyms) = synonym_mapper.synonyms.get(&word) {
                expanded_words.extend(synonyms.clone());
            } else {
                expanded_words.push(word);
            }
        }
        expanded_words.sort();
        expanded_words.dedup();
        expanded_words
    } else {
        let mut unique_words = words;
        unique_words.sort();
        unique_words.dedup();
        unique_words
    }
}

// Calculate Jaccard similarity between two word sets
fn calculate_similarity(words1: &[String], words2: &[String]) -> f32 {
    if words1.is_empty() || words2.is_empty() {
        return 0.0;
    }

    let set1: std::collections::HashSet<_> = words1.iter().collect();
    let set2: std::collections::HashSet<_> = words2.iter().collect();

    let intersection = set1.intersection(&set2).count();
    let union = set1.union(&set2).count();

    if union == 0 {
        0.0
    } else {
        intersection as f32 / union as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_synonym_expansion() {
        let mut mapper = SynonymMapper::new();
        mapper.synonyms.insert("god".to_string(), vec!["god".to_string(), "lord".to_string()]);
        mapper.synonyms.insert("love".to_string(), vec!["love".to_string(), "beloved".to_string()]);
        
        let expanded = mapper.expand_query("god love");
        
        assert!(expanded.contains(&"god".to_string()));
        assert!(expanded.contains(&"lord".to_string()));
        assert!(expanded.contains(&"love".to_string()));
        assert!(expanded.contains(&"beloved".to_string()));
    }
    
    #[test]
    fn test_verse_display() {
        let verse = Verse {
            book: "John".to_string(),
            chapter: 3,
            verse: 16,
            text: "For God so loved the world...".to_string(),
        };
        
        let display = format!("{}", verse);
        assert!(display.contains("John"));
        assert!(display.contains("3"));
        assert!(display.contains("16"));
        assert!(display.contains("For God so loved"));
    }
}
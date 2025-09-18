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
        let mut synonyms = HashMap::new();
        
        // Add common biblical synonyms
        synonyms.insert("love".to_string(), vec![
            "love".to_string(), "loved".to_string(), "loveth".to_string(), 
            "beloved".to_string(), "charity".to_string(), "affection".to_string()
        ]);
        synonyms.insert("god".to_string(), vec![
            "god".to_string(), "lord".to_string(), "almighty".to_string(), 
            "creator".to_string(), "father".to_string(), "jehovah".to_string(),
            "yahweh".to_string(), "most".to_string()
        ]);
        synonyms.insert("jesus".to_string(), vec![
            "jesus".to_string(), "christ".to_string(), "savior".to_string(), 
            "saviour".to_string(), "redeemer".to_string(), "messiah".to_string(), 
            "son".to_string()
        ]);
        synonyms.insert("peace".to_string(), vec![
            "peace".to_string(), "tranquil".to_string(), "calm".to_string(), 
            "serenity".to_string(), "rest".to_string(), "quiet".to_string()
        ]);
        synonyms.insert("joy".to_string(), vec![
            "joy".to_string(), "happiness".to_string(), "gladness".to_string(), 
            "delight".to_string(), "rejoice".to_string(), "joyful".to_string()
        ]);
        synonyms.insert("wisdom".to_string(), vec![
            "wisdom".to_string(), "knowledge".to_string(), "understanding".to_string(), 
            "insight".to_string(), "prudence".to_string(), "wise".to_string()
        ]);
        synonyms.insert("faith".to_string(), vec![
            "faith".to_string(), "belief".to_string(), "trust".to_string(), 
            "confidence".to_string(), "hope".to_string(), "believe".to_string()
        ]);
        synonyms.insert("fear".to_string(), vec![
            "fear".to_string(), "afraid".to_string(), "terror".to_string(), 
            "dread".to_string(), "reverence".to_string()
        ]);
        synonyms.insert("sin".to_string(), vec![
            "sin".to_string(), "transgression".to_string(), "iniquity".to_string(), 
            "wickedness".to_string(), "evil".to_string()
        ]);
        synonyms.insert("salvation".to_string(), vec![
            "salvation".to_string(), "save".to_string(), "saved".to_string(), 
            "deliverance".to_string(), "rescue".to_string()
        ]);
        
        SynonymMapper { synonyms }
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
}

// Create command line interface
fn create_cli() -> Command {
    Command::new("bible_tool")
        .version("2.0.0")
        .author("Your Name")
        .about("Enhanced Bible search tool with synonym support")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .value_name("FILE")
            .help("Path to Bible text file")
            .default_value("bible.txt"))
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
}

// Main function to run the application logic.
fn main() {
    let matches = create_cli().get_matches();
    
    let bible_file = matches.get_one::<String>("file").unwrap();
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
    
    let synonym_mapper = SynonymMapper::new();

    // Check if interactive mode is requested or no arguments provided
    if matches.get_flag("interactive") || 
       (!matches.contains_id("search") && !matches.contains_id("reference") && !matches.get_flag("random")) {
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

    if use_synonyms {
        println!("Searching for '{}' (with synonyms: {})...", query, search_terms.join(", "));
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_synonym_expansion() {
        let mapper = SynonymMapper::new();
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
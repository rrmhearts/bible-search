// Cargo.toml dependencies needed:
// [dependencies]
// clap = { version = "4.0", features = ["derive"] }
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// regex = "1.0"

use clap::{Arg, Command, ArgMatches};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
struct Verse {
    book: String,
    chapter: u32,
    verse: u32,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BibleData {
    verses: Vec<Verse>,
}

// Synonym mapping for enhanced search
struct SynonymMapper {
    synonyms: HashMap<String, Vec<String>>,
}

impl SynonymMapper {
    fn new() -> Self {
        let mut synonyms = HashMap::new();
        
        // Add common biblical synonyms
        synonyms.insert("love".to_string(), vec![
            "love".to_string(), "beloved".to_string(), "charity".to_string(), 
            "affection".to_string(), "devotion".to_string()
        ]);
        synonyms.insert("god".to_string(), vec![
            "god".to_string(), "lord".to_string(), "almighty".to_string(), 
            "creator".to_string(), "father".to_string(), "jehovah".to_string(),
            "yahweh".to_string()
        ]);
        synonyms.insert("jesus".to_string(), vec![
            "jesus".to_string(), "christ".to_string(), "savior".to_string(), 
            "redeemer".to_string(), "messiah".to_string(), "son".to_string()
        ]);
        synonyms.insert("peace".to_string(), vec![
            "peace".to_string(), "tranquil".to_string(), "calm".to_string(), 
            "serenity".to_string(), "rest".to_string()
        ]);
        synonyms.insert("joy".to_string(), vec![
            "joy".to_string(), "happiness".to_string(), "gladness".to_string(), 
            "delight".to_string(), "rejoice".to_string()
        ]);
        synonyms.insert("wisdom".to_string(), vec![
            "wisdom".to_string(), "knowledge".to_string(), "understanding".to_string(), 
            "insight".to_string(), "prudence".to_string()
        ]);
        synonyms.insert("faith".to_string(), vec![
            "faith".to_string(), "belief".to_string(), "trust".to_string(), 
            "confidence".to_string(), "hope".to_string()
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
        
        expanded_terms
    }
}

struct BibleSearcher {
    data: BibleData,
    synonym_mapper: SynonymMapper,
}

impl BibleSearcher {
    fn new(bible_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(bible_file)?;
        let data: BibleData = serde_json::from_str(&content)?;
        let synonym_mapper = SynonymMapper::new();
        
        Ok(BibleSearcher {
            data,
            synonym_mapper,
        })
    }
    
    fn search(&self, query: &str, case_sensitive: bool, use_synonyms: bool, book_filter: Option<&str>) -> Vec<&Verse> {
        let search_terms = if use_synonyms {
            self.synonym_mapper.expand_query(query)
        } else {
            query.split_whitespace().map(|s| s.to_string()).collect()
        };
        
        let mut results = Vec::new();
        
        for verse in &self.data.verses {
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
            
            let matches = if case_sensitive {
                search_terms.iter().any(|term| verse.text.contains(term))
            } else {
                search_terms.iter().any(|term| text_to_search.contains(&term.to_lowercase()))
            };
            
            if matches {
                results.push(verse);
            }
        }
        
        results
    }
    
    fn search_by_reference(&self, book: &str, chapter: Option<u32>, verse: Option<u32>) -> Vec<&Verse> {
        self.data.verses.iter().filter(|v| {
            let book_match = v.book.to_lowercase().contains(&book.to_lowercase());
            let chapter_match = chapter.map_or(true, |c| v.chapter == c);
            let verse_match = verse.map_or(true, |ve| v.verse == ve);
            
            book_match && chapter_match && verse_match
        }).collect()
    }
    
    fn get_random_verse(&self) -> &Verse {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
        let index = (hasher.finish() as usize) % self.data.verses.len();
        
        &self.data.verses[index]
    }
}

fn create_cli() -> Command {
    Command::new("bible_tool")
        .version("2.0.0")
        .author("Your Name")
        .about("Enhanced Bible search tool with synonym support")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .value_name("FILE")
            .help("Path to Bible JSON file")
            .default_value("bible.json"))
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
        .arg(Arg::new("format")
            .long("format")
            .value_name("FORMAT")
            .help("Output format: text, json, or verse-only")
            .default_value("text")
            .value_parser(["text", "json", "verse-only"]))
        .arg(Arg::new("interactive")
            .short('i')
            .long("interactive")
            .help("Start in interactive mode")
            .action(clap::ArgAction::SetTrue))
}

fn parse_reference(reference: &str) -> Option<(String, Option<u32>, Option<u32>)> {
    let re = Regex::new(r"^(\w+(?:\s+\w+)*)\s*(\d+)?(?::(\d+))?$").unwrap();
    
    if let Some(captures) = re.captures(reference) {
        let book = captures.get(1)?.as_str().to_string();
        let chapter = captures.get(2).and_then(|m| m.as_str().parse().ok());
        let verse = captures.get(3).and_then(|m| m.as_str().parse().ok());
        
        Some((book, chapter, verse))
    } else {
        None
    }
}

fn format_verse(verse: &Verse, format: &str) -> String {
    match format {
        "json" => serde_json::to_string_pretty(verse).unwrap_or_default(),
        "verse-only" => verse.text.clone(),
        _ => format!("{} {}:{} - {}", verse.book, verse.chapter, verse.verse, verse.text),
    }
}

fn print_results(results: &[&Verse], format: &str, limit: Option<usize>) {
    let limited_results: Vec<_> = if let Some(limit) = limit {
        results.iter().take(limit).copied().collect()
    } else {
        results.to_vec()
    };
    
    if limited_results.is_empty() {
        println!("No results found.");
        return;
    }
    
    println!("Found {} result(s):\n", limited_results.len());
    
    for verse in limited_results {
        println!("{}", format_verse(verse, format));
        if format != "verse-only" {
            println!();
        }
    }
}

fn interactive_mode(searcher: &BibleSearcher) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Bible Search Tool (Interactive Mode) ===");
    println!("Commands:");
    println!("  search <query> [--synonyms] [--case-sensitive] [--book <book>]");
    println!("  ref <reference> (e.g., 'John 3:16' or 'Genesis 1')");
    println!("  random");
    println!("  help");
    println!("  quit");
    println!();
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        
        match parts[0] {
            "quit" | "exit" | "q" => break,
            "help" | "h" => {
                println!("Available commands:");
                println!("  search <query> - Search for text");
                println!("  ref <reference> - Look up by reference");
                println!("  random - Get random verse");
                println!("  help - Show this help");
                println!("  quit - Exit program");
            }
            "search" | "s" => {
                if parts.len() < 2 {
                    println!("Usage: search <query>");
                    continue;
                }
                
                let query = parts[1..].join(" ");
                let use_synonyms = input.contains("--synonyms");
                let case_sensitive = input.contains("--case-sensitive");
                
                let book_filter = if let Some(book_pos) = input.find("--book") {
                    let book_part = &input[book_pos + 6..].trim();
                    book_part.split_whitespace().next()
                } else {
                    None
                };
                
                let results = searcher.search(&query, case_sensitive, use_synonyms, book_filter);
                print_results(&results, "text", Some(10));
            }
            "ref" | "reference" | "r" => {
                if parts.len() < 2 {
                    println!("Usage: ref <reference>");
                    continue;
                }
                
                let reference = parts[1..].join(" ");
                if let Some((book, chapter, verse)) = parse_reference(&reference) {
                    let results = searcher.search_by_reference(&book, chapter, verse);
                    print_results(&results, "text", None);
                } else {
                    println!("Invalid reference format. Use format like 'John 3:16' or 'Genesis 1'");
                }
            }
            "random" => {
                let verse = searcher.get_random_verse();
                println!("{}", format_verse(verse, "text"));
            }
            _ => {
                println!("Unknown command. Type 'help' for available commands.");
            }
        }
        println!();
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = create_cli().get_matches();
    
    let bible_file = matches.get_one::<String>("file").unwrap();
    let searcher = BibleSearcher::new(bible_file)?;
    
    // Check if interactive mode is requested
    if matches.get_flag("interactive") {
        return interactive_mode(&searcher);
    }
    
    let format = matches.get_one::<String>("format").unwrap();
    let limit = matches.get_one::<usize>("limit").copied();
    
    // Handle different command modes
    if matches.get_flag("random") {
        let verse = searcher.get_random_verse();
        println!("{}", format_verse(verse, format));
    } else if let Some(query) = matches.get_one::<String>("search") {
        let use_synonyms = matches.get_flag("synonyms");
        let case_sensitive = matches.get_flag("case-sensitive");
        let book_filter = matches.get_one::<String>("book").map(|s| s.as_str());
        
        let results = searcher.search(query, case_sensitive, use_synonyms, book_filter);
        print_results(&results, format, limit);
    } else if let Some(reference) = matches.get_one::<String>("reference") {
        if let Some((book, chapter, verse)) = parse_reference(reference) {
            let results = searcher.search_by_reference(&book, chapter, verse);
            print_results(&results, format, limit);
        } else {
            eprintln!("Invalid reference format. Use format like 'John 3:16' or 'Genesis 1'");
            std::process::exit(1);
        }
    } else {
        // No command specified, start interactive mode
        interactive_mode(&searcher)?;
    }
    
    Ok(())
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
    fn test_reference_parsing() {
        assert_eq!(
            parse_reference("John 3:16"), 
            Some(("John".to_string(), Some(3), Some(16)))
        );
        assert_eq!(
            parse_reference("Genesis 1"), 
            Some(("Genesis".to_string(), Some(1), None))
        );
        assert_eq!(
            parse_reference("Psalms"), 
            Some(("Psalms".to_string(), None, None))
        );
    }
}
// Cargo.toml dependencies needed:
// [dependencies]
// clap = { version = "4.0", features = ["derive"] }
// regex = "1.0"
// colored = "2.0"

use clap::{Arg, Command};
use colored::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Verse {
    book: String,
    chapter: u32,
    verse: u32,
    text: String,
}

impl Verse {
    fn reference(&self) -> String {
        format!("{}:{}:{}", self.book, self.chapter, self.verse)
    }
    
    fn short_reference(&self) -> String {
        format!("{} {}:{}", self.book, self.chapter, self.verse)
    }
}

#[derive(Debug)]
struct BibleData {
    translation: String,
    full_name: String,
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
            "love".to_string(), "loved".to_string(), "loveth".to_string(), 
            "beloved".to_string(), "charity".to_string(), "affection".to_string(), 
            "devotion".to_string()
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

struct BibleSearcher {
    data: BibleData,
    synonym_mapper: SynonymMapper,
}

impl BibleSearcher {
    fn new(bible_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(bible_file)?;
        let data = Self::parse_bible_file(&content)?;
        let synonym_mapper = SynonymMapper::new();
        
        Ok(BibleSearcher {
            data,
            synonym_mapper,
        })
    }
    
    fn parse_bible_file(content: &str) -> Result<BibleData, Box<dyn std::error::Error>> {
        let lines: Vec<&str> = content.lines().collect();
        
        if lines.len() < 3 {
            return Err("Invalid file format. Expected translation info and verses.".into());
        }
        
        let translation = lines[0].trim().to_string();
        let full_name = lines[1].trim().to_string();
        
        let mut verses = Vec::new();
        
        // Parse verses starting from line 2 (index 2)
        for line in lines.iter().skip(2) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            // Split by tab to get reference and text
            let parts: Vec<&str> = line.splitn(2, '\t').collect();
            if parts.len() != 2 {
                continue; // Skip malformed lines
            }
            
            let reference = parts[0];
            let text = parts[1].to_string();
            
            // Parse reference (e.g., "Genesis 1:1")
            if let Some((book, chapter, verse)) = Self::parse_verse_reference(reference) {
                verses.push(Verse {
                    book,
                    chapter,
                    verse,
                    text,
                });
            }
        }
        
        if verses.is_empty() {
            return Err("No valid verses found in file".into());
        }
        
        Ok(BibleData {
            translation,
            full_name,
            verses,
        })
    }
    
    fn parse_verse_reference(reference: &str) -> Option<(String, u32, u32)> {
        // Match patterns like "Genesis 1:1", "1 Kings 2:3", "2 Corinthians 4:5"
        let re = Regex::new(r"^(\d*\s*\w+(?:\s+\w+)*)\s+(\d+):(\d+)$").unwrap();
        
        if let Some(captures) = re.captures(reference.trim()) {
            let book = captures.get(1)?.as_str().trim().to_string();
            let chapter = captures.get(2)?.as_str().parse().ok()?;
            let verse = captures.get(3)?.as_str().parse().ok()?;
            
            Some((book, chapter, verse))
        } else {
            None
        }
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
    
    fn get_translation_info(&self) -> (&str, &str) {
        (&self.data.translation, &self.data.full_name)
    }
}

fn create_cli() -> Command {
    Command::new("bible_tool")
        .version("2.0.0")
        .author("Your Name")
        .about("Enhanced Bible search tool with synonym support and color coding")
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

fn parse_reference(reference: &str) -> Option<(String, Option<u32>, Option<u32>)> {
    let re = Regex::new(r"^(\d*\s*\w+(?:\s+\w+)*)\s*(\d+)?(?::(\d+))?$").unwrap();
    
    if let Some(captures) = re.captures(reference) {
        let book = captures.get(1)?.as_str().trim().to_string();
        let chapter = captures.get(2).and_then(|m| m.as_str().parse().ok());
        let verse = captures.get(3).and_then(|m| m.as_str().parse().ok());
        
        Some((book, chapter, verse))
    } else {
        None
    }
}

fn highlight_search_terms(text: &str, search_terms: &[String], case_sensitive: bool, use_color: bool) -> String {
    if !use_color || search_terms.is_empty() {
        return text.to_string();
    }
    
    let mut result = text.to_string();
    
    for term in search_terms {
        if term.is_empty() {
            continue;
        }
        
        let pattern = if case_sensitive {
            term.clone()
        } else {
            term.to_lowercase()
        };
        
        let search_text = if case_sensitive {
            result.clone()
        } else {
            result.to_lowercase()
        };
        
        if let Some(start) = search_text.find(&pattern) {
            let end = start + term.len();
            if case_sensitive {
                let original_term = &result[start..end];
                result = result.replace(original_term, &original_term.yellow().bold().to_string());
            } else {
                // For case-insensitive, we need to find the original case in the text
                let original_term = &text[start..end];
                result = result.replace(original_term, &original_term.yellow().bold().to_string());
            }
        }
    }
    
    result
}

fn format_verse(verse: &Verse, search_terms: Option<&[String]>, case_sensitive: bool, use_color: bool) -> String {
    let reference = if use_color {
        format!("{}", verse.short_reference().bright_blue().bold())
    } else {
        verse.short_reference()
    };
    
    let text = if let Some(terms) = search_terms {
        highlight_search_terms(&verse.text, terms, case_sensitive, use_color)
    } else {
        verse.text.clone()
    };
    
    format!("{} - {}", reference, text)
}

fn print_results(results: &[&Verse], limit: Option<usize>, search_terms: Option<&[String]>, case_sensitive: bool, use_color: bool) {
    let limited_results: Vec<_> = if let Some(limit) = limit {
        results.iter().take(limit).copied().collect()
    } else {
        results.to_vec()
    };
    
    if limited_results.is_empty() {
        let message = if use_color {
            "No results found.".red().to_string()
        } else {
            "No results found.".to_string()
        };
        println!("{}", message);
        return;
    }
    
    let count_message = if use_color {
        format!("Found {} result(s):", limited_results.len()).green().bold().to_string()
    } else {
        format!("Found {} result(s):", limited_results.len())
    };
    
    println!("{}\n", count_message);
    
    for (i, verse) in limited_results.iter().enumerate() {
        let number = if use_color {
            format!("{}.", i + 1).bright_black().to_string()
        } else {
            format!("{}.", i + 1)
        };
        
        println!("{} {}", number, format_verse(verse, search_terms, case_sensitive, use_color));
        println!();
    }
}

fn interactive_mode(searcher: &BibleSearcher, use_color: bool) -> Result<(), Box<dyn std::error::Error>> {
    let (translation, full_name) = searcher.get_translation_info();
    
    let title = if use_color {
        format!("=== Bible Search Tool - {} ({}) ===", full_name, translation).bright_cyan().bold().to_string()
    } else {
        format!("=== Bible Search Tool - {} ({}) ===", full_name, translation)
    };
    
    println!("{}", title);
    
    let commands_title = if use_color {
        "Commands:".bright_yellow().bold().to_string()
    } else {
        "Commands:".to_string()
    };
    
    println!("{}", commands_title);
    println!("  search <query> [--synonyms] [--case-sensitive] [--book <book>]");
    println!("  ref <reference> (e.g., 'John 3:16' or 'Genesis 1')");
    println!("  random");
    println!("  help");
    println!("  quit");
    println!();
    
    loop {
        let prompt = if use_color {
            "> ".bright_green().bold().to_string()
        } else {
            "> ".to_string()
        };
        
        print!("{}", prompt);
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
                let help_title = if use_color {
                    "Available commands:".bright_yellow().bold().to_string()
                } else {
                    "Available commands:".to_string()
                };
                
                println!("{}", help_title);
                println!("  search <query> - Search for text");
                println!("  ref <reference> - Look up by reference");
                println!("  random - Get random verse");
                println!("  help - Show this help");
                println!("  quit - Exit program");
            }
            "search" | "s" => {
                if parts.len() < 2 {
                    let error = if use_color {
                        "Usage: search <query>".red().to_string()
                    } else {
                        "Usage: search <query>".to_string()
                    };
                    println!("{}", error);
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
                
                let search_terms = if use_synonyms {
                    searcher.synonym_mapper.expand_query(&query)
                } else {
                    query.split_whitespace().map(|s| s.to_string()).collect()
                };
                
                let results = searcher.search(&query, case_sensitive, use_synonyms, book_filter);
                print_results(&results, Some(10), Some(&search_terms), case_sensitive, use_color);
            }
            "ref" | "reference" | "r" => {
                if parts.len() < 2 {
                    let error = if use_color {
                        "Usage: ref <reference>".red().to_string()
                    } else {
                        "Usage: ref <reference>".to_string()
                    };
                    println!("{}", error);
                    continue;
                }
                
                let reference = parts[1..].join(" ");
                if let Some((book, chapter, verse)) = parse_reference(&reference) {
                    let results = searcher.search_by_reference(&book, chapter, verse);
                    print_results(&results, None, None, false, use_color);
                } else {
                    let error = if use_color {
                        "Invalid reference format. Use format like 'John 3:16' or 'Genesis 1'".red().to_string()
                    } else {
                        "Invalid reference format. Use format like 'John 3:16' or 'Genesis 1'".to_string()
                    };
                    println!("{}", error);
                }
            }
            "random" => {
                let verse = searcher.get_random_verse();
                println!("{}", format_verse(verse, None, false, use_color));
            }
            _ => {
                let error = if use_color {
                    "Unknown command. Type 'help' for available commands.".red().to_string()
                } else {
                    "Unknown command. Type 'help' for available commands.".to_string()
                };
                println!("{}", error);
            }
        }
        println!();
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = create_cli().get_matches();
    
    let bible_file = matches.get_one::<String>("file").unwrap();
    let use_color = !matches.get_flag("no-color");
    
    let searcher = match BibleSearcher::new(bible_file) {
        Ok(s) => s,
        Err(e) => {
            let error_msg = if use_color {
                format!("Error loading Bible file '{}': {}", bible_file, e).red().bold().to_string()
            } else {
                format!("Error loading Bible file '{}': {}", bible_file, e)
            };
            eprintln!("{}", error_msg);
            std::process::exit(1);
        }
    };
    
    // Check if interactive mode is requested
    if matches.get_flag("interactive") {
        return interactive_mode(&searcher, use_color);
    }
    
    let limit = matches.get_one::<usize>("limit").copied();
    
    // Handle different command modes
    if matches.get_flag("random") {
        let verse = searcher.get_random_verse();
        println!("{}", format_verse(verse, None, false, use_color));
    } else if let Some(query) = matches.get_one::<String>("search") {
        let use_synonyms = matches.get_flag("synonyms");
        let case_sensitive = matches.get_flag("case-sensitive");
        let book_filter = matches.get_one::<String>("book").map(|s| s.as_str());
        
        let search_terms = if use_synonyms {
            searcher.synonym_mapper.expand_query(query)
        } else {
            query.split_whitespace().map(|s| s.to_string()).collect()
        };
        
        let results = searcher.search(query, case_sensitive, use_synonyms, book_filter);
        print_results(&results, limit, Some(&search_terms), case_sensitive, use_color);
    } else if let Some(reference) = matches.get_one::<String>("reference") {
        if let Some((book, chapter, verse)) = parse_reference(reference) {
            let results = searcher.search_by_reference(&book, chapter, verse);
            print_results(&results, limit, None, false, use_color);
        } else {
            let error_msg = if use_color {
                "Invalid reference format. Use format like 'John 3:16' or 'Genesis 1'".red().bold().to_string()
            } else {
                "Invalid reference format. Use format like 'John 3:16' or 'Genesis 1'".to_string()
            };
            eprintln!("{}", error_msg);
            std::process::exit(1);
        }
    } else {
        // No command specified, start interactive mode
        interactive_mode(&searcher, use_color)?;
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
    
    #[test]
    fn test_verse_reference_parsing() {
        assert_eq!(
            BibleSearcher::parse_verse_reference("Genesis 1:1"),
            Some(("Genesis".to_string(), 1, 1))
        );
        assert_eq!(
            BibleSearcher::parse_verse_reference("1 Kings 2:3"),
            Some(("1 Kings".to_string(), 2, 3))
        );
        assert_eq!(
            BibleSearcher::parse_verse_reference("2 Corinthians 4:5"),
            Some(("2 Corinthians".to_string(), 4, 5))
        );
    }
}
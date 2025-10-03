// bible.rs

use std::fs::File;
use std::io::{self, BufRead, Write};
use regex::Regex;
use lazy_static::lazy_static;
use colored::*;
use crate::synonyms::SynonymMapper;

// Structure to hold a single Bible verse.
#[derive(Debug, Clone)]
pub struct Verse {
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
    pub text: String,
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

// Parses the bible.txt file and returns a Vector of Verse structs.
pub fn load_bible(filename: &str) -> io::Result<Vec<Verse>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<book>.+?)\s(?P<chapter>\d+):(?P<verse>\d+)\t(?P<text>.+)$").unwrap();
    }

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut bible = Vec::new();

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

// ... (move all other Bible-related functions here)
// Such as: lookup_verse_cli, search_bible_cli, get_random_verse, find_cross_references, 
// interactive_mode, print_menu, lookup_verse, search_bible_interactive, extract_words,
// and calculate_similarity.
// Make sure to add `pub` to functions that are called from main.rs.

// Example of a moved function:
pub fn lookup_verse_cli(bible: &[Verse], reference: &str) {
    lazy_static! {
        static ref LOOKUP_RE: Regex = Regex::new(r"^(?P<book>.+?)\s(?P<chapter>\d+):(?P<verse>\d+)$").unwrap();
    }

    if let Some(caps) = LOOKUP_RE.captures(reference.trim()) {
        let book = &caps["book"];
        let chapter: u32 = caps["chapter"].parse().unwrap();
        let verse: u32 = caps["verse"].parse().unwrap();

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

// ... and so on for the rest of the functions
pub fn get_random_verse(bible: &[Verse]) {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
    let index = (hasher.finish() as usize) % bible.len();
    
    let verse = &bible[index];
    println!("{}", verse);
}

// Interactive mode
pub fn interactive_mode(bible: &[Verse], synonym_mapper: &SynonymMapper) {
    println!("\n{}", "=== Interactive Bible Search Tool ===".bright_cyan().bold());
    
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

fn print_menu() {
    println!("\n--- Bible Tool Menu ---");
    println!("1. Lookup Verse (e.g., Genesis 1:1)");
    println!("2. Search Text");
    println!("3. Exit");
    print!("> ");
    io::stdout().flush().unwrap();
}

fn lookup_verse(bible: &[Verse]) {
    print!("Enter reference (e.g., John 3:16): ");
    io::stdout().flush().unwrap();

    let mut reference = String::new();
    io::stdin().read_line(&mut reference).expect("Failed to read line");

    lookup_verse_cli(bible, &reference);
}

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

    print!("Use synonyms? (y/n): ");
    io::stdout().flush().unwrap();
    
    let mut synonym_choice = String::new();
    io::stdin().read_line(&mut synonym_choice).expect("Failed to read line");
    let use_synonyms = synonym_choice.trim().to_lowercase().starts_with('y');

    search_bible_cli(bible, synonym_mapper, query, use_synonyms, false, None, None, true);
}

pub fn search_bible_cli(bible: &[Verse], synonym_mapper: &SynonymMapper, query: &str, use_synonyms: bool, case_sensitive: bool, book_filter: Option<&str>, limit: Option<usize>, use_color: bool) {
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
            let mut highlighted_text = verse.text.clone();
            
            if use_color {
                for term in &search_terms {
                    if case_sensitive {
                        if verse.text.contains(term) {
                            highlighted_text = highlighted_text.replace(term, &term.black().on_yellow().to_string());
                        }
                    } else {
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

pub fn find_cross_references(bible: &[Verse], synonym_mapper: &SynonymMapper, reference: &str, similarity_threshold: f32, use_synonyms: bool, limit: Option<usize>, use_color: bool) {
    lazy_static! {
        static ref LOOKUP_RE: Regex = Regex::new(r"^(?P<book>.+?)\s(?P<chapter>\d+):(?P<verse>\d+)$").unwrap();
    }

    let (book, chapter, verse_num) = if let Some(caps) = LOOKUP_RE.captures(reference.trim()) {
        let book = caps["book"].to_string();
        let chapter: u32 = caps["chapter"].parse().unwrap();
        let verse: u32 = caps["verse"].parse().unwrap();
        (book, chapter, verse)
    } else {
        println!("{}", "Invalid reference format. Please use 'Book Chapter:Verse'.".red());
        return;
    };

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

    if use_color {
        println!("{}", "Source Verse:".bright_green().bold());
    } else {
        println!("Source Verse:");
    }
    println!("{}\n", source_verse);

    let source_words = extract_words(&source_verse.text, synonym_mapper, use_synonyms);
    
    if source_words.is_empty() {
        println!("{}", "No significant words found in source verse.".yellow());
        return;
    }

    let mut similarities: Vec<(f32, &Verse)> = bible.iter()
        .filter(|v| {
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

    similarities.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

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

fn extract_words(text: &str, synonym_mapper: &SynonymMapper, use_synonyms: bool) -> Vec<String> {
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
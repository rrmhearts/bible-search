// main.rs

use colored::*;
use clap::{Arg, Command};

// Declare the new modules
mod bible;
mod synonyms;

// Use the structs and functions from the new modules
use bible::{load_bible, search_bible_cli, lookup_verse_cli, get_random_verse, find_cross_references, interactive_mode};
use synonyms::SynonymMapper;

fn create_cli() -> Command {
    // CLI creation code remains the same...
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
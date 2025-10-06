// json_parser.rs
// Parser for BibleTranslations JSON format (https://github.com/jadenzaleski/BibleTranslations)

use std::fs::File;
use std::io::{self, BufReader, Read};
use serde::{Deserialize, Serialize};
use crate::bible::Verse;

// JSON structure for the BibleTranslations format
#[derive(Debug, Serialize, Deserialize)]
struct JsonBible {
    #[serde(flatten)]
    books: std::collections::HashMap<String, JsonBook>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonBook {
    #[serde(flatten)]
    chapters: std::collections::HashMap<String, JsonChapter>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonChapter {
    #[serde(flatten)]
    verses: std::collections::HashMap<String, String>,
}

/// Load a Bible from JSON format (BibleTranslations repository format)
/// 
/// Format example:
/// ```json
/// {
///   "Genesis": {
///     "1": {
///       "1": "In the beginning God created the heaven and the earth.",
///       "2": "And the earth was without form, and void..."
///     }
///   }
/// }
/// ```
pub fn load_bible_json(filename: &str) -> io::Result<Vec<Verse>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    // Parse JSON
    let json_bible: JsonBible = serde_json::from_reader(reader)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, 
            format!("Failed to parse JSON: {}", e)))?;
    
    let mut verses = Vec::new();
    
    // Convert JSON structure to Verse objects
    for (book_name, book) in json_bible.books {
        for (chapter_str, chapter) in book.chapters {
            let chapter_num: u32 = chapter_str.parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, 
                    format!("Invalid chapter number '{}': {}", chapter_str, e)))?;
            
            for (verse_str, text) in chapter.verses {
                let verse_num: u32 = verse_str.parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, 
                        format!("Invalid verse number '{}': {}", verse_str, e)))?;
                
                verses.push(Verse {
                    book: book_name.clone(),
                    chapter: chapter_num,
                    verse: verse_num,
                    text: text.trim().to_string(),
                });
            }
        }
    }
    
    // Sort verses by book, chapter, and verse for consistent ordering
    verses.sort_by(|a, b| {
        a.book.cmp(&b.book)
            .then(a.chapter.cmp(&b.chapter))
            .then(a.verse.cmp(&b.verse))
    });
    
    Ok(verses)
}

/// Detect if a file is in JSON format by checking the first non-whitespace character
pub fn is_json_format(filename: &str) -> bool {
    if let Ok(file) = File::open(filename) {
        let mut reader = BufReader::new(file);
        let mut first_char = [0u8; 1];
        
        // Read first non-whitespace character
        loop {
            if reader.read(&mut first_char).is_ok() {
                if first_char[0] == b'{' {
                    return true;
                } else if !first_char[0].is_ascii_whitespace() {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    false
}

/// Auto-detect format and load Bible accordingly
pub fn load_bible_auto(filename: &str) -> io::Result<Vec<Verse>> {
    // Check file extension first
    if filename.ends_with(".json") {
        return load_bible_json(filename);
    }
    
    // Otherwise check content
    if is_json_format(filename) {
        load_bible_json(filename)
    } else {
        // Fall back to text format
        crate::bible::load_bible(filename)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_json_parsing() {
        use std::io::Write;
        use tempfile::NamedTempFile;
        
        // Create a temporary JSON file
        let mut temp_file = NamedTempFile::new().unwrap();
        let json_content = r#"{
            "Genesis": {
                "1": {
                    "1": "In the beginning God created the heaven and the earth.",
                    "2": "And the earth was without form, and void."
                },
                "2": {
                    "1": "Thus the heavens and the earth were finished."
                }
            },
            "John": {
                "3": {
                    "16": "For God so loved the world."
                }
            }
        }"#;
        
        temp_file.write_all(json_content.as_bytes()).unwrap();
        let path = temp_file.path().to_str().unwrap();
        
        let verses = load_bible_json(path).unwrap();
        
        assert_eq!(verses.len(), 4);
        
        // Check first verse
        assert_eq!(verses[0].book, "Genesis");
        assert_eq!(verses[0].chapter, 1);
        assert_eq!(verses[0].verse, 1);
        assert!(verses[0].text.contains("In the beginning"));
        
        // Check sorting
        assert_eq!(verses[3].book, "John");
        assert_eq!(verses[3].chapter, 3);
        assert_eq!(verses[3].verse, 16);
    }
}
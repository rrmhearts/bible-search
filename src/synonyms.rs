use std::fs::{self, File};
use std::io::{self, BufRead};
use std::collections::HashMap;

pub struct SynonymMapper {
    pub synonyms: HashMap<String, Vec<String>>,
}

impl SynonymMapper {
    pub fn new() -> Self {
        SynonymMapper {
            synonyms: HashMap::new(),
        }
    }
    
    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let mut mapper = Self::new();
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
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
    
    pub fn create_default_file(filename: &str) -> io::Result<()> {
        let default_content = r#"# Bible Search Tool - Synonym Configuration
# ... (content of the default file) ...
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
    
    pub fn expand_query(&self, query: &str) -> Vec<String> {
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
        
        expanded_terms.sort();
        expanded_terms.dedup();
        expanded_terms
    }
    
    pub fn get_synonym_count(&self) -> usize {
        self.synonyms.len()
    }
}
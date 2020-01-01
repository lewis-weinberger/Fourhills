use std::fmt;
use crate::data::stats::StatBlock;
use crate::utils::text_utils::{centre_pad, wrap_lines_paragraph, capitalise};

/// Represents a non-player character.
#[derive(Debug)]
pub struct Npc {
    pub name: String,
    pub appearance: String,
    pub description: Option<String>,
    pub temperament: Option<String>,
    pub accent: Option<String>,
    pub phrases: Option<Vec<String>>,
    pub background: Option<String>,
    pub deceased: Option<bool>,
    pub stats: Option<StatBlock>,
    pub stats_base: Option<String>,
}

impl Npc {
    /// Returns a default NPC.
    ///
    /// # Parameters
    ///
    /// * `name: &str` - name of the NPC.
    /// * `appearance: &str` - appearance of the NPC.
    ///
    /// # Returns
    ///
    /// * `Npc` - a new default NPC.
    pub fn default(name: &str, appearance: &str) -> Self {
        Npc { name: String::from(name),
              appearance: String::from(appearance),
              description: None,
              temperament: None,
              accent: None,
              phrases: None,
              background: None,
              deceased: Some(false),
              stats: None,
              stats_base: None }
    }

    /// Returns a vector of lines summarising the NPC.
    ///
    /// # Parameters
    ///
    /// * `line_width: usize` - width of the output, in characters.
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - a summary of the NPC block as a vector of lines.
    pub fn summary_info(&self, line_width: usize) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        
        if let Some(deceased) = self.deceased {
            if deceased {
                lines.push(centre_pad(&format!("{} (deceased)", self.name), 80));
            } else {
                lines.push(centre_pad(&self.name, 80));
            }
        }
        
        lines.push("=".repeat(line_width));

        if let Some(stats) = &self.stats {
            lines.push(format!("{} {}", capitalise(&stats.alignment), stats.name));
            lines.push(format!("{} {}", capitalise(&stats.size), stats.creature_type));
        }
        
        wrap_lines_paragraph(&lines, line_width)
    }

    /// Returns a vector of lines detailing the NPC's stats.
    ///
    /// # Parameters
    ///
    /// * `line_width: usize` - width of the output, in characters.
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - a represenation of the NPC's stats as a vector of lines.
    pub fn battle_info(&self, line_width: usize) -> Vec<String> {
        if let Some(stats) = &self.stats {
            stats.battle_info(line_width)
        } else {
            vec![String::from("This NPC has no stats defined")]
        }
    }

    /// Returns a vector of lines describing the NPC.
    ///
    /// # Parameters
    ///
    /// * `line_width: usize` - width of the output, in characters.
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - a represenation of the NPC as a vector of lines.
    pub fn character_info(&self, line_width: usize) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        
        lines.push(format!("Appearance: {}", self.appearance));
        
        if let Some(description) = &self.description {
            lines.push(format!("Description: {}", description));
        }
        
        if let Some(accent) = &self.accent {
            lines.push(format!("Accent: {}", accent));
        }

        if let Some(phrases) = &self.phrases {
            lines.push(String::from(""));
            lines.push(String::from("Phrases:"));
            for phrase in phrases.iter() {
                lines.push(format!("- {}", phrase));
            }
        }
        
        wrap_lines_paragraph(&lines, line_width)
    }

    // TODO: from_name, absolute_path (requires Setting)
}

impl fmt::Display for Npc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for line in self.summary_info(80).iter() {
            buffer.push_str(&line);
            buffer.push('\n');
        }
        write!(f, "{}", buffer)
    }
}

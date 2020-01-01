use std::fmt;

/// The stat block for a monster or character.
#[derive(Debug)]
pub struct StatBlock {
    pub name: String,
    pub size: String,
    pub creature_type: String,
    pub alignment: String,
    pub ac: String,
    pub hp: String,
    // TODO: complete stat block
}

impl StatBlock {
    /// Returns the battle info for the stat block as a vector of lines.
    ///
    /// # Parameters
    ///
    /// * `line_width: usize` - width of the output, in characters.
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - a represenation of the stat block as a vector of lines.
    pub fn battle_info(&self, _line_width: usize) -> Vec<String> {
        unimplemented!()
    }

    // TODO: calculate_ability_modifier, summary_info, from_file, from_name, absolute_path
}

impl fmt::Display for StatBlock {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}


use std::fmt;
use std::collections::HashMap;

/// The stat block for a monster or character.
#[derive(Debug)]
pub struct StatBlock {
    pub name: String,
    pub size: String,
    pub creature_type: String,
    pub alignment: String,
    pub ac: String,
    pub hp: String,
    pub speed: String,
    pub ability: HashMap<String, i32>,
    pub challenge: f32,
    pub passive_perception: i32,
    pub saving_throws: Option<HashMap<String, String>>,
    pub skills: Option<HashMap<String, String>>,
    pub damage_vulnerabilities: Option<Vec<String>>,
    pub damage_resistance: Option<Vec<String>>,
    pub damage_immunities: Option<Vec<String>>,
    pub condition_immunities: Option<Vec<String>>,
    pub special_senses: Option<HashMap<String, String>>,
    pub languages: Option<Vec<String>>,
    pub special_traits: Option<HashMap<String, String>>,
    pub melee_attacks: Option<HashMap<String, HashMap<String, String>>>,
    pub ranged_attacks: Option<HashMap<String, HashMap<String, String>>>,
    pub multiattack: Option<String>,
    pub other_actions: Option<HashMap<String, String>>,
    pub description: Option<String>,
    pub legendary_actions: Option<HashMap<String, String>>,
    pub legendary_reactions: Option<HashMap<String, String>>,
    pub lair_actions: Option<HashMap<String, String>>,
}

impl StatBlock {
    /// Returns a default StatBlock.
    ///
    /// # Parameters
    ///
    /// * `name: &str` - character name 
    /// * `size: &str` - character size
    /// * `creature_type: &str` - creature type of character
    /// * `alignment: &str` - alignment of character
    /// * `ac: &str` - armour class
    /// * `hp: &str` - health points
    /// * `speed: &str` - movement speed
    /// * `ability: &HashMap<String, i32>` - abilities and scores
    /// * `challenge: f32` - challenge score
    /// * `passive_perception: i32` - passive perception score
    ///
    /// # Returns
    ///
    /// * `StatBlock` - a new default StatBlock.
    pub fn default(&self, name: &str, size: &str, creature_type: &str,
                   alignment: &str, ac: &str, hp: &str, speed: &str,
                   ability: HashMap<String, i32>, challenge: f32,
                   passive_perception: i32) -> Self {
        StatBlock { name: String::from(name),
                    size: String::from(size),
                    creature_type: String::from(creature_type),
                    alignment: String::from(alignment),
                    ac: String::from(ac),
                    hp: String::from(hp),
                    speed: String::from(speed),
                    ability,
                    challenge,
                    passive_perception,
                    saving_throws: None,
                    skills: None,
                    damage_vulnerabilities: None,
                    damage_resistance: None,
                    damage_immunities: None,
                    condition_immunities: None,
                    special_senses: None,
                    languages: None,
                    special_traits: None,
                    melee_attacks: None,
                    ranged_attacks: None,
                    multiattack: None,
                    other_actions: None,
                    description: None,
                    legendary_actions: None,
                    legendary_reactions: None,
                    lair_actions: None,
        }
    }
    
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


use std::path::Path;
use std::vec::Vec;

mod board_toml;
use board_toml::BoardTomlInfo;

/// A struct that holds the decoded toml information for new board generation
#[derive(Default, Clone)]
pub struct BoardTomlInfo {
    pub name : String,
    pub manufacturer : String,
    pub is_main_board : bool,
    pub standard : String,
    pub cpu : String,
    pub ram : i32,
    pub flash : i32,
    pub required_crates : Vec<String>,
    pub related_crates : Vec<String>,

    pub bsp : String,

    pub pinouts : Vec<PinoutTomlInfo>,
}

impl BoardTomlInfo{
    pub fn generateToml (&self, file_path: &Path) -> (){
        /// TODO Generate toml file in relevant directory
    }

    pub fn cleanup (&self, file_path: &Path) -> (){
        /// TODO Delete files if necessary
    }
}
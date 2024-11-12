
use std::vec::Vec;

mod pinout_toml;
use pinout_toml::PinoutTomlInfo;

/// A struct that holds the decoded pinout information for creating board toml file
#[derive(Default, Clone)]
pub struct PinoutTomlInfo {
    pub pins : Vec<String>,
    pub iface_type : String,
    pub direction : String,
}
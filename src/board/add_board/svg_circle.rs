use std::path::Path;
use std::vec::Vec;

mod svg_circle;
use svg_circle::SvgCircle;

/// A struct that holds the decoded svg circle additions for adding to svg file
#[derive(Default, Clone)]
pub struct SvgCircle {
    /// TODO Unique?
    pub id : String,

    /// TODO Decide if needs to be optional
    pub stroke : String,
    pub stroke_width : f32,
    pub fill : String,
    pub fill_opacity : f32,

    pub cx : f32,
    pub cy : f32,
    pub r : f32,

}

impl SvgCircle {
    pub fn generateSVGPins (&self, file_path: &Path) -> (){
        // TODO Implement adding to SVG file
        // TODO make return type the generated SVG element?
    }

    pub fn cleanup (&self, file_path: &Path) -> (){
        /// TODO Restore old file state (store old file within struct?)
    }
}
use drivers::webgl::FCFont;
use types::Colour;

use super::DrawingSpec;

#[derive(Clone,Debug)]
pub struct TextArtist {
    pub chars: String,
    pub font: FCFont,
    pub colour: Colour,
    pub background: Colour
}

impl TextArtist {
    fn new(chars: &str, font: &FCFont, colour: &Colour, background: &Colour) -> TextArtist {
        TextArtist {
            chars: chars.to_string(),
            font: font.clone(),
            colour: colour.clone(),
            background: background.clone()
        }
    }
}

pub fn text_texture(chars: &str,font: &FCFont, col: &Colour, bg: &Colour) -> DrawingSpec {
    DrawingSpec::Text(TextArtist::new(chars,font,col,bg))
}

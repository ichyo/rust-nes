use super::palette::Palettes;
use super::palette::Rgb;
use super::pattern::PatternTable;

// TODO: support 8x16 sprite
const SPRITE_WIDTH: u8 = 8;
const SPRITE_HEIGHT: u8 = 8;

struct SpriteAttribute(u8);

impl SpriteAttribute {
    fn palette_index(&self) -> u8 {
        (self.0 & 0x3)
    }

    fn filp_horizontally(&self) -> bool {
        ((self.0 >> 6) & 0x1) != 0
    }

    fn filp_vertically(&self) -> bool {
        ((self.0 >> 7) & 0x1) != 0
    }
}

pub struct Sprite {
    base_y: u8,
    base_x: u8,
    tile_index: u8,
    attribute: SpriteAttribute,
}

impl Sprite {
    pub fn new(d: &[u8]) -> Self {
        assert!(d.len() == 4);
        let base_y = d[0] + 1;
        let base_x = d[3];
        let attribute = SpriteAttribute(d[2]);
        let tile_index = d[1];
        Sprite {
            base_y,
            base_x,
            tile_index,
            attribute,
        }
    }

    pub fn get_color(
        &self,
        x: u8,
        y: u8,
        pattern: &PatternTable,
        palette: &Palettes,
    ) -> Option<Rgb> {
        let inner_x = i16::from(x) - i16::from(self.base_x);
        let inner_y = i16::from(y) - i16::from(self.base_y);
        if !(inner_x >= 0
            && inner_x < i16::from(SPRITE_WIDTH)
            && inner_y >= 0
            && inner_y < i16::from(SPRITE_HEIGHT))
        {
            return None;
        }

        let inner_x = if self.attribute.filp_horizontally() {
            7 - inner_x as u8
        } else {
            inner_x as u8
        };

        let inner_y = if self.attribute.filp_vertically() {
            7 - inner_y as u8
        } else {
            inner_y as u8
        };

        let color_index = pattern.get_value(self.tile_index, inner_x, inner_y);
        if color_index == 0 {
            return None;
        }
        let color = palette.get_sprite_color(self.attribute.palette_index(), color_index);
        Some(color)
    }
}

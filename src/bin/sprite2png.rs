extern crate image;
extern crate nes;

use image::ImageBuffer;
use nes::cartridge::Cartridge;
use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Sprite([[u8; 8]; 8]);

impl Sprite {
    fn new() -> Sprite {
        Sprite([[0; 8]; 8])
    }

    fn parse(chr: &[u8], index: usize) -> Sprite {
        let base = index << 4;
        let mut sprite = Sprite::new();
        for y in 0..8 {
            for x in 0..8 {
                let c1 = chr[base + y] >> (7 - x) & 1;
                let c2 = chr[base + y + 8] >> (7 - x) & 1;
                let c = (c2 << 1) | c1;
                assert!(c <= 3);
                sprite.0[y][x] = c;
            }
        }
        sprite
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.0[y][x]
    }
}

fn main() {
    let path = env::args().nth(1).expect("please specify the path to nes");

    let mut f = File::open(path).expect("failed to open file");
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).expect("failed to read a file");

    let cartridge = Cartridge::parse(&buffer).expect("invalid nes format");
    let chr = cartridge.chr_rom;

    let chr_size = chr.len() / 0x10;

    let sprites_per_row = (chr_size as f64).sqrt() as usize;

    let width = sprites_per_row as u32 * 8;
    let height = (chr_size as f64 / sprites_per_row as f64).ceil() as u32 * 8;
    let mut img = ImageBuffer::new(width, height);

    for i in 0..chr_size {
        let sprite = Sprite::parse(&chr, i);
        let by = (i / sprites_per_row) * 8;
        let bx = (i % sprites_per_row) * 8;
        for y in 0..8 {
            for x in 0..8 {
                let c = sprite.get(x, y);
                img.put_pixel(
                    (bx + x) as u32,
                    (by + y) as u32,
                    image::Luma([c as u8 * 64]),
                );
            }
        }
    }

    image::ImageLuma8(img)
        .save("output.png")
        .expect("failed to write png file");
}

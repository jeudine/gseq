use crate::Show;
use rand::prelude::*;

pub const COLOR_0_0: [u8; 4] = [0x9f, 0x56, 0xff, 0xff];
const COLOR_1_0: [u8; 4] = [0xb5, 0x82, 0xff, 0xff];
const COLOR_2_0: [u8; 4] = [0xca, 0xad, 0xff, 0xff];
const COLOR_3_0: [u8; 4] = [0xff, 0xad, 0xc7, 0xff];
pub const COLOR_4_0: [u8; 4] = [0xff, 0x99, 0xb6, 0xff];

const COLOR_0_1: [u8; 4] = [0x9f, 0x86, 0xfa, 0xff];
const COLOR_1_1: [u8; 4] = [0x60, 0x64, 0xfc, 0xff];
pub const COLOR_2_1: [u8; 4] = [0x1b, 0x59, 0xff, 0xff];
const COLOR_3_1: [u8; 4] = [0x00, 0x05, 0xf1, 0xff];
pub const COLOR_4_1: [u8; 4] = [0x2f, 0x08, 0x85, 0xff];

const COLORS_0: [[u8; 4]; 5] = [COLOR_0_0, COLOR_1_0, COLOR_2_0, COLOR_3_0, COLOR_4_0];
const COLORS_1: [[u8; 4]; 5] = [COLOR_0_1, COLOR_1_1, COLOR_2_1, COLOR_3_1, COLOR_4_1];

pub fn get_color(rng: &mut ThreadRng, show: Show) -> [f32; 4] {
    let c = match show {
        Show::Lua => COLORS_0,
        Show::MariusJulien => COLORS_1,
    };
    let v = c
        .choose(rng)
        .unwrap()
        .iter()
        .map(|c| hex_to_f(*c))
        .collect::<Vec<_>>();
    [v[0], v[1], v[2], v[3]]
}

pub fn hex_to_f(c: u8) -> f32 {
    c as f32 / 255.0
}

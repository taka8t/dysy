use rand::{thread_rng, Rng};
use image::{RgbImage, Rgb, DynamicImage};

use super::attractor::Attractor;
use crate::util::Palette;
use crate::state::State;

pub struct Custom {
    pub name: String,
    pub map_str: String,
    pub range: Vec<std::ops::RangeInclusive<f64>>,
    pub speeds: Vec<f64>,
    pub coefs: Vec<f64>,
    pub state: State
}
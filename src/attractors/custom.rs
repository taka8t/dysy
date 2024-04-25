use rand::{thread_rng, Rng};
use image::{RgbImage, Rgb, DynamicImage};
use serde::{Serialize, Deserialize};

use super::attractor::Attractor;
use crate::util::Palette;
use crate::state::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Custom {
    pub name: String,
    pub map_str: String,
    pub range: Vec<std::ops::RangeInclusive<f64>>,
    pub speeds: Vec<f64>,
    pub coefs: Vec<f64>,
    pub state: State
}
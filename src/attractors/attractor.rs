use std::path::PathBuf;
use image::{DynamicImage, error::ImageResult};
use crate::util::Palette;
use crate::state::State;

// dynamical system trajectory generator analyzer
pub trait Attractor {
    fn apply_map_func(&mut self);
    // trajectory image
    fn gen_img(&mut self, n: usize, w: usize, h: usize, plt: &Palette) -> DynamicImage;
    fn save_img(&mut self, path: &PathBuf, n: usize, w: usize, h: usize, plt: &Palette) -> ImageResult<()> {
        let img = self.gen_img(n, w, h, plt);
        img.save(path)
    }
    fn name(&self) -> &str;
    fn map_str(&self) -> &str;
    fn speeds(&self) -> Vec<f64>;
    fn coefs(&self) -> &[f64];
    fn coefs_mut(&mut self) -> &mut [f64];
    fn change_random_coefs(&mut self);
    fn coef_ranges(&self) -> Vec<std::ops::RangeInclusive<f64>>;
    fn state(&self) -> &State;
    fn state_mut(&mut self) -> &mut State;
    fn param_changed(&mut self, flag: bool);
}
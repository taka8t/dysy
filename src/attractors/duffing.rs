use std::path::PathBuf;
use rand::{thread_rng, Rng};
use image::{RgbImage, Rgb, DynamicImage, error::ImageResult};

use super::attractor::Attractor;
use crate::util::Palette;
use crate::state::State;

pub struct Duffing {
    pub name: String,
    pub map_str: String,
    pub range: Vec<std::ops::RangeInclusive<f64>>,
    pub speeds: Vec<f64>,
    pub coefs: Vec<f64>,
    pub state: State,
}

impl Default for Duffing {
    fn default() -> Self {
        let range = vec![
                (-1.0..=1.0),(-1.0..=1.0),(-5.0..=5.0)
            ];
        Self {
            name: "Duffing Attractor".into(),
            map_str: "dx/dt = y, dy/dt = x - x^3 + a0 * y + a1 * cos(a2 * t)".into(),
            range: range,
            speeds: vec![0.001; 3],
            coefs: vec![0.5; 3],
            state: State::new(2, -1.0..=1.0, Some(0.0005)),
        }
    }
}
#[allow(dead_code)]
impl Duffing {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let range = vec![
            (-1.0..=1.0),(-1.0..=1.0),(-5.0..=5.0)
        ];
        Self {
            name: "Duffing Attractor".into(),
            map_str: "dx/dt = y, dy/dt = x - x^3 + a0 * y + a1 * cos(a2 * t)".into(),
            range: range.clone(),
            speeds: vec![0.001; 3],
            coefs: range.iter()
                    .cloned()
                    .map(|r| rng.gen_range(r))
                    .collect::<Vec<f64>>(),
            state: State::new(2, -1.0..=1.0, Some(0.0005)),
        }
    }

    fn search_edges(&mut self, n: usize, skip: usize) -> (f64, f64, f64, f64) {
        self.state.set_init();
        let (mut top, mut left, mut bottom, mut right) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
        for i in 0..n {
            self.apply_map_func();
            if i < skip {continue;}
            let (x, y) = self.state.get_xy();
            top = top.min(y);
            left = left.min(x);
            bottom = bottom.max(y);
            right = right.max(x);
        }
        self.state.set_init();
        (top, left, bottom, right)
    }
}
impl Attractor for Duffing {
    fn name(&self) -> &str {
        &self.name
    }
    fn map_str(&self) -> &str {
        &self.map_str
    }
    fn coef_ranges(&self) -> Vec<std::ops::RangeInclusive<f64>> {
        self.range.clone()
    }
    fn speeds(&self) -> Vec<f64> {
        self.speeds.clone()
    }
    fn coefs(&self) -> &[f64] {
        &self.coefs
    }
    fn coefs_mut(&mut self) -> &mut [f64] {
        &mut self.coefs
    }
    fn state(&self) -> &State {
        &self.state
    }
    fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }
    fn change_random_coefs(&mut self) {
        let mut rng = thread_rng();
        self.coefs = self.range.iter()
            .cloned()
            .map(|r| rng.gen_range(r))
            .collect::<Vec<f64>>();
    }

    fn apply_map_func(&mut self) {
        let (x, y) = self.state.get_xy();
        let dt = self.state().get_dt().unwrap();
        self.state.set_xy(
            x + y * dt,
            y + (x - x*x*x - self.coefs[0] * y + self.coefs[1] * (self.coefs[2] * self.state.time).cos()) * dt
        );
        self.state.time += dt;
    }

    fn gen_img(&mut self, n: usize, w: i64, h: i64, plt: &Palette) -> DynamicImage {
        let skip = 0;
        let (top, left, bottom, right) = self.search_edges(100000.max(n/10), skip);

        let wc = (right + left) * 0.5;
        let hc = (bottom + top) * 0.5;
        let m = (w as f64 / (right - left)).min(h as f64 / (bottom - top));

        let mut hist = vec![vec![0.0; w as usize]; h as usize];
        let mut mx_its = 0.0f64;
        self.state.set_init();
        for i in 0..n {
            self.apply_map_func();
            if i < skip {continue;}
            let (x, y) = self.state.get_xy();
            let tw = (((x - wc) * m).round() as i64 + w/2).clamp(0, w-1) as usize;
            let th = (((y - hc) * m).round() as i64 + h/2).clamp(0, h-1) as usize;
            let val = &mut hist[th][tw];
            *val += 1.0;
            mx_its = mx_its.max(*val);
        }

        let factor = (10_000_000.0 / (n as f64)).sqrt() * ((w * h) as f64) / (1024.0 * 1024.0) * 100.;
        let img = RgbImage::from_par_fn(w as u32, h as u32, |x, y| {
            let v = hist[y as usize][x as usize];
            let (r, g, b) = plt.get_col(v / mx_its , v/ mx_its, factor);
            Rgb([r, g, b])
        }); 

        DynamicImage::ImageRgb8(img)
    }

    fn save_img(&mut self, path: &PathBuf, n: usize, w: i64, h: i64, plt: &Palette) -> ImageResult<()> {
        let img = self.gen_img(n, w, h, plt);
        img.save(path)
    }
}
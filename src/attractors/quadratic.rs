use rand::{thread_rng, Rng};
use image::{RgbImage, Rgb, DynamicImage};
use serde::{Serialize, Deserialize};

use super::attractor::Attractor;
use crate::util::Palette;
use crate::state::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quadratic {
    pub name: String,
    pub map_str: String,
    pub range: Vec<std::ops::RangeInclusive<f64>>,
    pub speeds: Vec<f64>,
    pub coefs: Vec<f64>,
    pub state: State,
    #[serde(skip)]
    pub img_vec: Vec<f64>,
    #[serde(skip)]
    pub param_changed: bool,
}

impl Default for Quadratic {
    fn default() -> Self {
        let range = vec![
                (-1.5..=1.5),(-1.5..=1.5),(-1.5..=1.5),(-1.5..=1.5),(-1.5..=1.5),(-1.5..=1.5),
                (-1.5..=1.5),(-1.5..=1.5),(-1.5..=1.5),(-1.5..=1.5),(-1.5..=1.5),(-1.5..=1.5)
            ];
        Self {
            name: "Quadratic Attractor".into(),
            map_str: "x = a0 * x^2 + a1 * x + a2 * x * y + a3 * y + a4 * y^2 + a5, y = a6 * x^2 + a7 * x + a8 * x * y + a9 * y + a10 * y^2 + a11".into(),
            range,
            speeds: vec![0.001; 12],
            coefs: vec![1.0; 12],
            state: State::new(2, -1.0..=1.0, None),
            img_vec: vec![],
            param_changed: true
        }
    }
}
#[allow(dead_code)]
impl Quadratic {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let range = vec![
            (-1.5..=1.5),(-1.5..=1.5),(-1.5..=1.5),(-1.5..=1.5),(-1.5..=1.5),(-1.5..=1.5),
            (-1.5..=1.5),(-1.5..=1.5),(-1.5..=1.5),(-1.5..=1.5),(-1.5..=1.5),(-1.5..=1.5)
        ];
        Self {
            name: "Quadratic Attractor".into(),
            map_str: "x = a0 * x^2 + a1 * x + a2 * x * y + a3 * y + a4 * y^2 + a5, y = a6 * x^2 + a7 * x + a8 * x * y + a9 * y + a10 * y^2 + a11".into(),
            range: range.clone(),
            speeds: vec![0.001; 12],
            coefs: range.iter()
                    .cloned()
                    .map(|r| rng.gen_range(r))
                    .collect::<Vec<f64>>(),
            state: State::new(2, -1.0..=1.0, None),
            img_vec: vec![],
            param_changed: true
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

    fn gen_hist(&mut self, n: usize, w: usize, h: usize) {
        let skip = 500;
        let (top, left, bottom, right) = self.search_edges(50000, skip);

        let wc = (right + left) * 0.5;
        let hc = (bottom + top) * 0.5;
        let m = (w as f64 / (right - left)).min(h as f64 / (bottom - top));

        let mut hist = vec![0.0; w * h];
        let mut mx_its = 0.0f64;
        self.state.set_init();
        let (iw, ih) = (w as i64, h as i64);
        for i in 0..n {
            self.apply_map_func();
            if i < skip {continue;}
            let (x, y) = self.state.get_xy();
            let tw = (((x - wc) * m).round() as i64 + iw/2).clamp(0, iw-1) as usize;
            let th = (((y - hc) * m).round() as i64 + ih/2).clamp(0, ih-1) as usize;
            let val = &mut hist[th * w + tw];
            *val += 1.0;
            mx_its = mx_its.max(*val);
        }
        let inv_mx_its = 1.0 / mx_its;
        self.img_vec = hist.into_iter().map(|v| v * inv_mx_its).collect::<Vec<_>>();
    }
}
impl Attractor for Quadratic {
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
    fn param_changed(&mut self, flag: bool) {
        self.param_changed = flag;
    }
    fn apply_map_func(&mut self) {
        let (x, y) = self.state.get_xy();
        self.state.set_xy(
            self.coefs[0] * x * x + self.coefs[1] * x + self.coefs[2] * x * y + self.coefs[3] * y + self.coefs[4] * y * y + self.coefs[5],
            self.coefs[6] * x * x + self.coefs[7] * x + self.coefs[8] * x * y + self.coefs[9] * y + self.coefs[10] * y * y + self.coefs[11]
        );
    }

    fn gen_img(&mut self, n: usize, w: usize, h: usize, plt: &Palette) -> DynamicImage {
        if self.param_changed {
            self.gen_hist(n, w, h);
        }

        let factor = (10_000_000.0 / (n as f64)).sqrt() * ((w * h) as f64) / (1024.0 * 1024.0) * 100.;
        let img = RgbImage::from_par_fn(w as u32, h as u32, |x, y| {
            let v = self.img_vec[(y as usize) * w + (x as usize)];
            let (r, g, b) = plt.get_col(v, v, factor);
            Rgb([r, g, b])
        }); 

        DynamicImage::ImageRgb8(img)
    }
}
use rand::{thread_rng, Rng};
use image::{RgbImage, Rgb, DynamicImage};

use super::attractor::Attractor;
use crate::util::Palette;
use crate::state::State;

pub struct Trigonometric {
    pub name: String,
    pub map_str: String,
    pub range: Vec<std::ops::RangeInclusive<f64>>,
    pub speeds: Vec<f64>,
    pub coefs: Vec<f64>,
    pub state: State
}

impl Default for Trigonometric {
    fn default() -> Self {
        let range = vec![
                (-5.0..=5.0),(-5.0..=5.0),(-5.0..=5.0),(-5.0..=5.0),
                (-5.0..=5.0),(-5.0..=5.0),(-5.0..=5.0),(-5.0..=5.0)
            ];
        Self {
            name: "Trigonometric Attractor".into(),
            map_str: "x = sin(a0 * x^2 + a1 * xy + a2 * y^2 + a3), y = cos(a4 * x^2 + a5 * xy + a6 * y^2 + a7)".into(),
            range: range,
            speeds: vec![0.001; 8],
            coefs: vec![1.0; 8],
            state: State::new(2, -1.0..=1.0, None)
        }
    }
}
#[allow(dead_code)]
impl Trigonometric {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let range = vec![
            (-5.0..=5.0),(-5.0..=5.0),(-5.0..=5.0),(-5.0..=5.0),
            (-5.0..=5.0),(-5.0..=5.0),(-5.0..=5.0),(-5.0..=5.0)
        ];
        Self {
            name: "Trigonometric Attractor".into(),
            map_str: "a0 * sin(a1 * x^2 + a2 * xy + a3 * y^2) + a4 * cos(a5 * x^2 + a6 * xy + a7 * y^2)".into(),
            range: range.clone(),
            speeds: vec![0.001; 8],
            coefs: range.iter()
                    .cloned()
                    .map(|r| rng.gen_range(r))
                    .collect::<Vec<f64>>(),
            state: State::new(2, -1.0..=1.0, None)
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
impl Attractor for Trigonometric {
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
        self.state.set_xy(
            (self.coefs[0] * x * x + self.coefs[1] * x * y + self.coefs[2] * y * y + self.coefs[3]).sin(),
            (self.coefs[4] * x * x + self.coefs[5] * x * y + self.coefs[6] * y * y + self.coefs[7]).cos()
        );
    }

    fn gen_img(&mut self, n: usize, w: usize, h: usize, plt: &Palette) -> DynamicImage {
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

        let factor = (10_000_000.0 / (n as f64)).sqrt() * ((w * h) as f64) / (1024.0 * 1024.0) * 100.;
        let img = RgbImage::from_par_fn(w as u32, h as u32, |x, y| {
            let v = hist[(y as usize) * w + (x as usize)];
            let (r, g, b) = plt.get_col(v / mx_its, v / mx_its, factor);
            Rgb([r, g, b])
        }); 

        DynamicImage::ImageRgb8(img)
    }
}
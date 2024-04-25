use rand::{thread_rng, Rng};
use image::{RgbImage, Rgb, DynamicImage};
use serde::{Serialize, Deserialize};

use super::attractor::Attractor;
use crate::util::Palette;
use crate::state::State;

use std::f64::consts::TAU;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoublePendulum {
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

impl Default for DoublePendulum {
    fn default() -> Self {
        let range = vec![
                (0.1..=2.0),(0.1..=2.0),(0.1..=2.0),(0.1..=2.0),(0.0..=10.0),
            ];
        Self {
            name: "DoublePendulum".into(),
            map_str: "Runge Kutta4 theta: a0, a1, omega: a2, a3, length: x0, x1, mass: x2, x3, g: x4".into(),
            range,
            speeds: vec![0.001; 5],
            coefs: vec![1.0, 1.0, 1.0, 1.0, 9.8],
            state: State::new(4, -TAU..=TAU, Some(0.0005)),
            img_vec: vec![],
            param_changed: true
        }
    }
}
#[allow(dead_code)]
impl DoublePendulum {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let range = vec![
            (0.1..=2.0),(0.1..=2.0),(0.1..=2.0),(0.1..=2.0),(0.1..=10.0),
        ];
        Self {
            name: "DoublePendulum".into(),
            map_str: "Runge Kutta4 theta: a0, a1, omega: a2, a3, length: x0, x1, mass: x2, x3, g: x4".into(),
            range: range.clone(),
            speeds: vec![0.001; 5],
            coefs: range.iter()
                    .cloned()
                    .map(|r| rng.gen_range(r))
                    .collect::<Vec<f64>>(),
            state: State::new(4, -TAU..=TAU, Some(0.0005)),
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
            let (theta1, theta2) = self.state.get_xy();
            let (x1, y1) = (self.coefs[2] * theta1.sin(), self.coefs[2] * theta1.cos());
            let (x, y) = (x1 + self.coefs[3] * theta2.sin(), y1 + self.coefs[3] * theta2.cos());
            top = top.min(y);
            left = left.min(x);
            bottom = bottom.max(y);
            right = right.max(x);
        }
        self.state.set_init();
        (top, left, bottom, right)
    }

    fn rk4(&self, x: &[f64], dt: f64) -> (f64, f64, f64, f64) {
        let k1 = self.derivatives((x[0], x[1], x[2], x[3]));
        let k2 = self.derivatives((x[0] + k1.0 * dt * 0.5, x[1] + k1.1 * dt * 0.5, x[2] + k1.2 * dt * 0.5, x[3] + k1.3 * dt * 0.5));
        let k3 = self.derivatives((x[0] + k2.0 * dt * 0.5, x[1] + k2.1 * dt * 0.5, x[2] + k2.2 * dt * 0.5, x[3] + k2.3 * dt * 0.5));
        let k4 = self.derivatives((x[0] + k3.0 * dt, x[1] + k3.1 * dt, x[2] + k3.2 * dt, x[3] + k3.3 * dt));
        (
            (k1.0 + k2.0 * 2.0 + k3.0 * 2.0 + k4.0) * dt * 0.16666666667,
            (k1.1 + k2.1 * 2.0 + k3.1 * 2.0 + k4.1) * dt * 0.16666666667,
            (k1.2 + k2.2 * 2.0 + k3.2 * 2.0 + k4.2) * dt * 0.16666666667,
            (k1.3 + k2.3 * 2.0 + k3.3 * 2.0 + k4.3) * dt * 0.16666666667
        )
    }
    fn derivatives(&self, x: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
        let m = self.coefs[1] / self.coefs[0];
        let l = self.coefs[3] / self.coefs[2];
        let g = self.coefs[4] / self.coefs[2];
        let dc = (x.0 - x.1).cos();
        let ds = (x.0 - x.1).sin();
        (
            x.2,
            x.3,
            -((1.0 + m) * g * (x.0).sin() + m * l * x.3 * x.3 * ds + m * dc * (x.2 * x.2 * ds - g * (x.1).sin()))
            / (1.0 + m * ds * ds),
            ((1.0 + m) * (x.2 * x.2 * ds - g * (x.1).sin()) + dc * ((1.0 + m) * g * (x.0).sin() + m * l * x.3 * x.3 * ds))
            / (l * (1.0 + m * ds * ds))
        )
    }

    fn gen_hist(&mut self, n: usize, w: usize, h: usize) {
        let skip = 0;
        let (top, left, bottom, right) = self.search_edges((n/10).max(50000), skip);

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
            let (theta1, theta2) = self.state.get_xy();
            let (x1, y1) = (self.coefs[2] * theta1.sin(), self.coefs[2] * theta1.cos());
            let (x, y) = (x1 + self.coefs[3] * theta2.sin(), y1 + self.coefs[3] * theta2.cos());
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

impl Attractor for DoublePendulum {
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
        let dt = self.state().get_dt().unwrap();
        let x = self.state.get_xs();
        let nx = self.rk4(x, dt);
        let x = self.state.get_xs_mut();
        x[0] = (x[0] + nx.0) % TAU;
        x[1] = (x[1] + nx.1) % TAU;
        x[2] += nx.2;
        x[3] += nx.3;
        self.state.time += dt;
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
use rand::{thread_rng, Rng};
use image::{RgbImage, Rgb, DynamicImage};
use serde::{Serialize};
use serde::ser::{Serializer, SerializeStruct};
use lieval::{Expr, EvalError};

use super::attractor::Attractor;
use crate::util::Palette;
use crate::state::State;

#[derive(Debug, Clone)]
pub struct Custom {
    pub name: String,
    pub map_str: String,
    pub range: Vec<std::ops::RangeInclusive<f64>>,
    pub speeds: Vec<f64>,
    pub coefs: Vec<f64>,
    pub state: State,
    pub expr: Expr,
    pub inner_expr: Expr,
    pub img_vec: Vec<f64>,
    pub param_changed: bool,
}

impl Serialize for Custom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Custom", 3)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("map_str", &self.map_str)?;
        s.serialize_field("range", &self.range)?;
        s.serialize_field("speeds", &self.name)?;
        s.serialize_field("coefs", &self.coefs)?;
        s.serialize_field("state", &self.state)?;
        s.serialize_field("expr", &self.map_str)?;
        s.end()
    }
}

impl Default for Custom {
    fn default() -> Self {
        let range = vec![
                (-2.0..=2.0),(-2.0..=2.0),(-2.0..=2.0),(-2.0..=2.0),
                (-2.0..=2.0),(-2.0..=2.0),(-2.0..=2.0),(-2.0..=2.0)
            ];
        let map_str =  "a0 * sin(a1 * y) + a2 * cos(a3 * x);a4 * sin(a5 * x) + a6 * cos(a7 * y)";
        Self {
            name: "Custom Attractor".into(),
            map_str: map_str.into(),
            range,
            speeds: vec![0.001; 8],
            coefs: vec![1.0; 8],
            state: State::new(2, -2.0..=2.0, None),
            expr: Expr::new("0").unwrap(),
            inner_expr: Expr::new(map_str).unwrap(),
            img_vec: vec![],
            param_changed: true,
        }
    }
}
#[allow(dead_code)]
impl Custom {
    pub fn new(map_str: &str) -> Result<Self, EvalError> {
        let mut rng = thread_rng();
        let expr = Expr::new(map_str)?;
        let vars = expr.vars();
        if !vars.contains(&"x".to_string()) || !vars.contains(&"y".to_string()) {
            return Err(EvalError::UndefinedVariable("You must use the variables x and y".to_string()));
        }
        let n = vars.len() - 2;
        
        let range = vec![
            -2.0..=2.0; n
        ];
        Ok(Self {
            name: "Custom Attractor".into(),
            map_str: map_str.into(),
            range: range.clone(),
            speeds: vec![0.001; n],
            coefs: range.iter()
                    .cloned()
                    .map(|r| rng.gen_range(r))
                    .collect::<Vec<f64>>(),
            state: State::new(2, -2.0..=2.0, None),
            expr: Expr::new("0").unwrap(),
            inner_expr: expr,
            img_vec: vec![],
            param_changed: true,
        })
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
        self.expr = self.inner_expr.clone();
        let vars = self.expr.vars();
        for (v, c) in vars.iter().filter(|&v| v != "x" && v != "y").zip(self.coefs.iter()) {
            self.expr.set_var(v, *c);
        }
        if self.expr.partial_eval().is_err() {return;};

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
impl Attractor for Custom {
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
        if let Ok(ret) = self.expr.set_var("x", x).set_var("y", y).evals() {
            self.state.set_xy(ret[0], ret[1]);
        }
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
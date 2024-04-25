use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};
use std::f64::{consts::TAU};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palette {
    pub r: (f64, f64, f64, f64),
    pub g: (f64, f64, f64, f64),
    pub b: (f64, f64, f64, f64),
    pub colver1: f64,
    pub colver2: f64,
    pub brightness1: f64,
    pub brightness2: f64,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            r: (0.5, 0.25, 1.0, 0.0),
            g: (0.5, 0.25, 1.0, 0.33),
            b: (0.5, 0.25, 1.0, 0.67),
            colver1: 0.3,
            colver2: 5.0,
            brightness1: 0.4,
            brightness2: 20.0,
        }
    }
}

#[allow(dead_code)]
impl Palette {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        Self {
            r: (rng.gen_range(0.5..1.0), rng.gen_range(0.0..0.5), rng.gen_range(0.5..1.5), rng.gen_range(0.0..1.0)),
            g: (rng.gen_range(0.5..1.0), rng.gen_range(0.0..0.5), rng.gen_range(0.5..1.5), rng.gen_range(0.0..1.0)),
            b: (rng.gen_range(0.5..1.0), rng.gen_range(0.0..0.5), rng.gen_range(0.5..1.5), rng.gen_range(0.0..1.0)),
            colver1: 0.3,
            colver2: 5.0,
            brightness1: 0.4,
            brightness2: 20.0,
        }
    }
    pub fn phase(&self, v: f64) -> f64 {
        v.powf(self.colver1) * self.colver2
    }
    pub fn brightness(&self, v: f64) -> f64 {
        v.powf(self.brightness1) * self.brightness2
    }
    pub fn get_col(&self, v: f64, b: f64, factor: f64) -> (u8, u8, u8) {
        let x = self.phase(v);
        let y = self.brightness(b) * factor;
        (
            ((self.r.0 + self.r.1 * ((self.r.2 * x + self.r.3) * TAU).cos()) * y).clamp(0.0, 255.0) as u8,
            ((self.g.0 + self.g.1 * ((self.g.2 * x + self.g.3) * TAU).cos()) * y).clamp(0.0, 255.0) as u8,
            ((self.b.0 + self.b.1 * ((self.b.2 * x + self.b.3) * TAU).cos()) * y).clamp(0.0, 255.0) as u8
        )
    }
    pub fn change_random(& mut self) {
        let mut rng = thread_rng();
        self.r = (rng.gen_range(0.5..1.0), rng.gen_range(0.0..0.5), rng.gen_range(0.5..1.5), rng.gen_range(0.0..1.0));
        self.g = (rng.gen_range(0.5..1.0), rng.gen_range(0.0..0.5), rng.gen_range(0.5..1.5), rng.gen_range(0.0..1.0));
        self.b = (rng.gen_range(0.5..1.0), rng.gen_range(0.0..0.5), rng.gen_range(0.5..1.5), rng.gen_range(0.0..1.0));
    }
}
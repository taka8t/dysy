use rand::{thread_rng, Rng};

pub struct State {
    n: usize,
    x: Vec<f64>,
    init_x: Vec<f64>,
    x_range: std::ops::RangeInclusive<f64>,
    pub time: f64,
    dt: Option<f64>,
    dt_range: Option<f64>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            n: 2,
            x: vec![0.5; 2],
            init_x: vec![0.5; 2],
            x_range: -1.0..=1.0,
            time: 0.0,
            dt: None,
            dt_range: None,
        }
    }
}
#[allow(dead_code)]
impl State {
    pub fn new(n: usize, range: std::ops::RangeInclusive<f64>, t: Option<f64>) -> Self {
        Self {
            n: n,
            x: vec![0.5; n],
            init_x: vec![0.5; n],
            x_range: range,
            time: 0.0,
            dt: t,
            dt_range: if let Some(t) = t {Some(t*100.0)} else {None},
        }
    }
    pub fn set_init(&mut self) {
        self.time = 0.0;
        self.x = self.init_x.clone()
    }
    pub fn get_init_val(&self) -> &[f64] {
        &self.init_x
    }
    pub fn get_init_val_mut(&mut self) -> &mut [f64] {
        &mut self.init_x
    }
    pub fn get_x_range(&self) -> std::ops::RangeInclusive<f64> {
        self.x_range.clone()
    }
    pub fn get_dt(&self) -> Option<f64> {
        self.dt
    }
    pub fn get_dt_mut(&mut self) -> Option<&mut f64> {
        self.dt.as_mut()
    }
    pub fn get_dt_range(&self) -> Option<f64> {
        self.dt_range
    }
    pub fn set_random_init(&mut self) {
        let mut rng = thread_rng();
        self.init_x = (0..self.n)
                .map(|_| rng.gen_range(self.x_range.clone()))
                .collect::<Vec<f64>>();
    }
    pub fn get_xs(&self) -> &[f64] {
        &self.x
    }
    pub fn get_xs_mut(&mut self) -> &mut [f64] {
        &mut self.x
    }
    pub fn set_xs(&mut self, new_xs: Vec<f64>) {
        self.x = new_xs;
    }
    pub fn get_xy(&self) -> (f64, f64) {
        assert!(self.x.len() >= 2);
        (self.x[0], self.x[1])
    }
    pub fn set_xy(&mut self, x: f64, y: f64) {
        assert!(self.x.len() >= 2);
        self.x[0] = x;
        self.x[1] = y;
    }
    pub fn get_xyz(&self) -> (f64, f64, f64) {
        assert!(self.x.len() >= 3);
        (self.x[0], self.x[1], self.x[2])
    }
    pub fn set_xyz(&mut self, x: f64, y: f64, z: f64) {
        assert!(self.x.len() >= 3);
        self.x[0] = x;
        self.x[1] = y;
        self.x[2] = z;
    }
}
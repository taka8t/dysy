#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::MyApp;

mod attractors;
mod state;
mod util;
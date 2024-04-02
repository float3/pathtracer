#![feature(generic_const_exprs)]
pub mod camera;
pub mod light;
pub mod material;
pub mod object;
pub mod pathtracer;
pub mod ray;
pub mod scene;
pub mod skybox;
pub mod utils;

#[macro_use]
extern crate cfg_if;

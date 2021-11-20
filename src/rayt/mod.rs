mod camera;
mod float3;
mod material;
mod ray;
mod render;
mod shape;
mod shape_builder;

pub use self::camera::Camera;
pub use self::float3::{Color, Float3, Point3, Vec3};
pub use self::material::*;
pub use self::ray::Ray;
pub use self::render::*;
pub use self::shape::*;
pub use self::shape_builder::*;
pub use std::sync::Arc;

pub use std::f64::consts::FRAC_1_PI;
pub use std::f64::consts::PI;
pub const EPS: f64 = 1e-6;

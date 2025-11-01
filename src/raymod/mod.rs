mod aabb;
mod material;
mod optarg;
mod rayunit;
mod vec3;
pub use self::aabb::*;
pub use self::material::*;
pub use self::optarg::*;
pub use self::rayunit::*;
pub use self::vec3::*;

pub const EPS: f64 = 1e-6;
pub const INF: f64 = 1e20;
pub const FRAC_SQRT_3: f64 = 1.732050807568877293527446341505872367;

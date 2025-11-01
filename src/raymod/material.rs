use crate::raymod::*;
use std::sync::Arc;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color;
}

pub struct ColorTexture {
    color: Vec3,
}
impl ColorTexture {
    pub const fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for ColorTexture {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
    freq: f64,
}
impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>, freq: f64) -> Self {
        Self { odd, even, freq }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color {
        let sines = (p.x * self.freq).sin() * (p.y * self.freq).sin() * (p.z * self.freq).sin();
		//*ではなく+にすると斑点模様になる
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct ScatterInfo {
    pub ray: Ray,
    pub albedo: Color,
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo>;
}

impl ScatterInfo {
    pub fn new(ray: Ray, albedo: Vec3) -> Self {
        Self { ray, albedo }
    }
}
pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}
impl Lambertian {
    pub fn new(albedo: Box<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let target = hit.p + hit.n + Vec3::random_hemisphere();
        let albedo = self.albedo.value(hit.u, hit.v, hit.p);
        Some(ScatterInfo::new(Ray::new(hit.p, target - hit.p), albedo))
    }
}

pub struct Metal {
    pub albedo: Box<dyn Texture>,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Box<dyn Texture>, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter(&self, _ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let mut reflected = _ray.d.norm().reflect(hit.n);
        reflected = reflected + Vec3::random_hemisphere() * self.fuzz;
        if reflected.dot(&hit.n) > 0.0 {
            let albedo = self.albedo.value(hit.u, hit.v, hit.p);
            Some(ScatterInfo::new(Ray::new(hit.p, reflected), albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ri: f64,
}

impl Dielectric {
    pub const fn new(ri: f64) -> Self {
        Self { ri }
    }
    pub fn schlick(cosine: f64, ri: f64) -> f64 {
        let r0 = ((1.0 - ri) / (1.0 + ri)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let reflected = ray.d.reflect(hit.n);
        let (outward_normal, ni_over_nt, cosine) = {
            let dot = ray.d.dot(&hit.n);
            if dot > 0.0 {
                (hit.n * -1.0, self.ri, self.ri * dot / ray.d.length().sqrt())
            } else {
                (
                    hit.n,
                    1.0 / self.ri,
                    self.ri * dot / ray.d.length().sqrt() * -1.0,
                )
            }
        };
        if let Some(refracted) = (ray.d * -1.0).refract(outward_normal, ni_over_nt) {
            if Vec3::random_full().x > Self::schlick(cosine, self.ri) {
                return Some(ScatterInfo::new(
                    Ray::new(hit.p, refracted),
                    Vec3::new(1.0, 1.0, 1.0),
                ));
            }
        }
        Some(ScatterInfo::new(
            Ray::new(hit.p, reflected),
            Vec3::new(1.0, 1.0, 1.0),
        ))
    }
}

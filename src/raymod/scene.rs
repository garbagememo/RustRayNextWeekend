use crate::raymod::*;
use std::sync::Arc;

//左上が原点なPNGフォーマット対応
pub struct Camera {
    pub origin: Vec3,
    pub upper_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f64,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).norm();
        let u = (vup % w).norm();
        let v = w % u;

        let origin = lookfrom;
        let horizontal = u * focus_dist * viewport_width;
        let vertical = v * focus_dist * viewport_height;
        let upper_left_corner = origin - horizontal / 2.0 + vertical / 2.0 - w * focus_dist;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            upper_left_corner,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.upper_left_corner + self.horizontal * s - self.vertical * t - self.origin - offset,
        )
    }
}

#[allow(dead_code)]
impl ShapeList {
    pub fn simple_scene(&mut self) -> Camera {
        self.push(Box::new(Sphere::new(
            Vec3::new(0.6, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::new(Box::new(ColorTexture::new(Vec3::new(
                0.1, 0.2, 0.5,
            ))))),
        )));
        self.push(Box::new(Sphere::new(
            Vec3::new(-0.6, 0.0, -1.0),
            0.5,
            Arc::new(Metal::new(
                Box::new(ColorTexture::new(Vec3::new(0.8, 0.8, 0.8))),
                0.4,
            )),
        )));
        self.push(Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, 0.0),
            100.0,
            Arc::new(Lambertian::new(Box::new(CheckerTexture::new(
                Box::new(ColorTexture::new(Vec3::new(0.8, 0.8, 0.0))),
                Box::new(ColorTexture::new(Vec3::new(0.8, 0.2, 0.0))),
                10.0,
            )))),
        )));
        // simple_scene用カメラ
        let lookfrom = Vec3::new(0.0, 1.0, 4.0);
        let lookat = Vec3::new(0.0, 0.0, -1.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);

        let dist_to_focus = (lookfrom - lookat).length().sqrt();
        let aperture = 0.1;

        return Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            ASPECT_RATIO,
            aperture,
            dist_to_focus,
        );
    }

    pub fn texture_scene(&mut self) -> Camera {
        self.push(Box::new(Sphere::new(
            Vec3::new(0.6, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::new(Box::new(ImageTexture::new(
                "testimage.jpg",
            )))),
        )));
        self.push(Box::new(Sphere::new(
            Vec3::new(-0.6, 0.0, -1.0),
            0.5,
            Arc::new(Metal::new(
                Box::new(ColorTexture::new(Vec3::new(0.8, 0.8, 0.8))),
                0.4,
            )),
        )));
        self.push(Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, 0.0),
            100.0,
            Arc::new(Lambertian::new(Box::new(CheckerTexture::new(
                Box::new(ColorTexture::new(Vec3::new(0.8, 0.8, 0.0))),
                Box::new(ColorTexture::new(Vec3::new(0.8, 0.2, 0.0))),
                10.0,
            )))),
        )));
        // simple_scene用カメラ
        let lookfrom = Vec3::new(0.0, 1.0, 4.0);
        let lookat = Vec3::new(0.0, 0.0, -1.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);

        let dist_to_focus = (lookfrom - lookat).length().sqrt();
        let aperture = 0.1;

        return Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            ASPECT_RATIO,
            aperture,
            dist_to_focus,
        );
    }

    pub fn random_scene(&mut self) -> Camera {
        self.push(Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(Lambertian::new(Box::new(ColorTexture::new(Vec3::new(
                0.5, 0.5, 0.5,
            ))))),
        )));

        let mut box_list1: Vec<Box<dyn Shape>> = Vec::new();
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random();
                let center = Vec3::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());
                if (center - Vec3::new(4.0, 0.2, 0.0)).length().sqrt() > 0.9 {
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Vec3::random().mult(Vec3::random());
                        box_list1.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Lambertian::new(Box::new(ColorTexture::new(albedo)))),
                        )));
                    } else if choose_mat < 0.95 {
                        // Metal
                        let fuzz = random_range(0.0, 0.5);
                        let albedo = Vec3::vec3_random_range(0.5, 1.0);
                        box_list1.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Metal::new(Box::new(ColorTexture::new(albedo)), fuzz)),
                        )));
                    } else {
                        // glass
                        box_list1.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Dielectric::new(1.5)),
                        )));
                    }
                }
            }
        }
        self.push(Box::new(BVH::new(box_list1)));

        self.push(Box::new(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Arc::new(Dielectric::new(1.5)),
        )));
        self.push(Box::new(Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Arc::new(Lambertian::new(Box::new(ColorTexture::new(Vec3::new(
                0.4, 0.2, 0.1,
            ))))),
        )));
        self.push(Box::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Arc::new(Metal::new(
                Box::new(ColorTexture::new(Vec3::new(0.7, 0.6, 0.5))),
                0.0,
            )),
        )));

        // Camera
        // random_scene用カメラ
        let lookfrom = Vec3::new(13.0, 2.0, 3.0);
        let lookat = Vec3::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = (lookfrom - lookat).length().sqrt();
        let aperture = 0.1;

        return Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            ASPECT_RATIO,
            aperture,
            dist_to_focus,
        );
    }
}

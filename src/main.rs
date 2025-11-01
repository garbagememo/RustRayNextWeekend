mod raymod;
use raymod::*;

use rayon::prelude::*;

use std::io::Write;
use std::sync::Arc;

fn ray_color(r: &Ray, world: &dyn Shape, depth: i64) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    let hit_info = world.hit(&r, EPS, f64::MAX);
    if let Some(hit) = hit_info {
        let scatter_info = hit.m.scatter(r, &hit);
        if let Some(scatter) = scatter_info {
            scatter
                .albedo
                .mult(ray_color(&scatter.ray, world, depth - 1))
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let t = 0.5 * (r.d.norm().y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

impl ShapeList {
    pub fn simple_scene(&mut self) {
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
    }
    /*
        pub fn random_scene(&mut self){
            self.push(Box::new(Sphere::new(
                Vec3::new(0.0, -1000.0, 0.0),
                1000.0,
                Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
            )));
            for a in -11..11 {
                for b in -11..11 {
                    let choose_mat=random();
                    let center=Vec3::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random() );
                    if (center-Vec3::new(4.0,0.2,0.0)).length().sqrt() > 0.9 {
                        if choose_mat < 0.8 {
                            // diffuse
                            let albedo = Vec3::random().mult(Vec3::random() );
                            self.push(Box::new(Sphere::new(
                                center,
                                0.2,
                                Arc::new(Lambertian::new(albedo)),
                            )));
                        }else if choose_mat <0.95{
                            // Metal
                            let fuzz= random_range(0.0,0.5);
                            let albedo=Vec3::vec3_random_range(0.5,1.0);
                            self.push(Box::new(Sphere::new(
                                center,
                                0.2,
                                Arc::new(Metal::new(albedo,fuzz)),
                            )));
                        } else {
                            // glass
                            self.push(Box::new(Sphere::new(
                                center,
                                0.2,
                                Arc::new(Dielectric::new(1.5)),
                            )));
                        }
                    }
                }
            }
            self.push(Box::new(Sphere::new(
                Vec3::new(0.0,1.0,0.0),
                1.0,
                Arc::new(Dielectric::new(1.5)),)));
            self.push(Box::new(Sphere::new(
                Vec3::new(-4.0,1.0,0.0),
                1.0,
                Arc::new(Lambertian::new(Vec3::new(0.4,0.2,0.1)),))));
            self.push(Box::new(Sphere::new(
                Vec3::new(4.0,1.0,0.0),
                1.0,
                Arc::new(Metal::new(Vec3::new(0.7,0.6,0.5),0.0),))));
        }
    */
}

fn main() {
    let args = parameters();
    println!("{:?}", args);

    let ASPECT_RATIO = 16.0 / 9.0;
    let w: usize = args.w;
    let h: usize = ((w as f64) / ASPECT_RATIO) as usize;
    let samps: usize = args.s;

    let mut image = vec![Color::zero(); (w * h) as usize];

    // Camera
    /* random_scene用カメラ
       let lookfrom = Vec3::new(13.0, 2.0, 3.0);
       let lookat = Vec3::new(0.0, 0.0, 0.0);
       let vup = Vec3::new(0.0, 1.0, 0.0);
    */
    // simple_scene用カメラ
    let lookfrom = Vec3::new(0.0, 1.0, 4.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let dist_to_focus = (lookfrom - lookat).length().sqrt();
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let MAX_DEPTH: i64 = 32;

    let mut world = ShapeList::new();
    world.simple_scene();

    let bands: Vec<(usize, &mut [Color])> = image.chunks_mut(w as usize).enumerate().collect();
    bands.into_par_iter().for_each(|(y, band)| {
        for x in 0..w {
            let mut r = Vec3::new(0.0, 0.0, 0.0);
            for _spp in 0..samps {
                for _sy in 0..2 {
                    for _sx in 0..2 {
                        let u = (x as f64 + (_sx as f64 + random()) / 4.0) / (w as f64);
                        let v = (y as f64 + (_sy as f64 + random()) / 4.0) / (h as f64);
                        let ray = cam.get_ray(u, v);
                        r = r + ray_color(&ray, &world, MAX_DEPTH) / (samps as f64) / 4.0;
                    }
                }
            }
            band[x as usize] = r;
        }
        if (y % 20) == 0 {
            print!("y={0}  :", y);
            println!("col={:?}", band[0]);
        };
    });

    //    save_ppm_file("image.ppm", image, w, h);
    save_png_file(&args.output, image, w, h);
}

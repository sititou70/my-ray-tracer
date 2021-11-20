use crate::rayt::*;

mod rayt;

struct ShapeList {
    pub objects: Vec<Box<dyn Shape>>,
}
impl ShapeList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn push(&mut self, object: Box<dyn Shape>) {
        self.objects.push(object);
    }
}
impl Shape for ShapeList {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let mut hit_info: Option<HitInfo> = None;
        let mut closest_so_far = t1;
        for object in &self.objects {
            if let Some(info) = object.hit(ray, t0, closest_so_far) {
                closest_so_far = info.t;
                hit_info = Some(info);
            }
        }
        hit_info
    }
}

struct RandomScene {
    world: ShapeList,
}
impl RandomScene {
    fn new() -> Self {
        let mut world = ShapeList::new();
        world.push(
            ShapeBuilder::new()
                .lambertian(Color::new(0.5, 0.5, 0.5))
                .sphere(Point3::new(0.0, -1000.0, 0.0), 1000.0)
                .build(),
        );
        // Small spheres
        for au in -11..11 {
            let a = au as f64;
            for bu in -11..11 {
                let b = bu as f64;
                let [rx, rz, material_choice] = Float3::random().to_array();
                let center = Point3::new(a + 0.9 * rx, 0.2, b + 0.9 * rz);
                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    world.push({
                        if material_choice < 0.8 {
                            let albedo = Color::random() * Color::random();
                            ShapeBuilder::new()
                                .lambertian(albedo)
                                .sphere(center, 0.2)
                                .build()
                        } else if material_choice < 0.95 {
                            let albedo = Color::random_limit(0.5, 1.0);
                            let fuzz = Float3::random_full().x();
                            ShapeBuilder::new()
                                .metal(albedo, fuzz)
                                .sphere(center, 0.2)
                                .build()
                        } else {
                            ShapeBuilder::new()
                                .dielectric(1.5)
                                .sphere(center, 0.2)
                                .build()
                        }
                    });
                }
            }
        }
        // Big spheres
        world.push(
            ShapeBuilder::new()
                .dielectric(1.5)
                .sphere(Point3::new(0.0, 1.0, 0.0), 1.0)
                .build(),
        );
        world.push(
            ShapeBuilder::new()
                .lambertian(Color::new(0.4, 0.2, 0.1))
                .sphere(Point3::new(-4.0, 1.0, 0.0), 1.0)
                .build(),
        );
        world.push(
            ShapeBuilder::new()
                .metal(Color::new(0.7, 0.6, 0.5), 0.0)
                .sphere(Point3::new(4.0, 1.0, 0.0), 1.0)
                .build(),
        );
        Self { world }
    }
    fn background(&self, d: Vec3) -> Color {
        let t = 0.5 * (d.normalize().y() + 1.0);
        Color::one().lerp(Color::new(0.5, 0.7, 1.0), t)
    }
}
impl Scene for RandomScene {
    fn camera(&self) -> Camera {
        Camera::from_lookat(
            Point3::new(13.0, 2.0, 3.0),
            Point3::new(0.0, 1.0, 0.0),
            Vec3::yaxis(),
            20.0,
            self.aspect(),
            0.025,
        )
    }
    fn trace(&self, ray: Ray, depth: usize) -> Color {
        let hit_info = self.world.hit(&ray, 0.001, f64::MAX);
        if let Some(hit) = hit_info {
            let scatter_info = if depth > 0 {
                hit.m.scatter(&ray, &hit)
            } else {
                None
            };
            if let Some(scatter) = scatter_info {
                scatter.albedo * self.trace(scatter.ray, depth - 1)
            } else {
                Color::zero()
            }
        } else {
            self.background(ray.direction)
        }
    }
}

fn main() {
    render(RandomScene::new());
}

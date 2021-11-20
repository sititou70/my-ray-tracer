use crate::rayt::*;

pub struct ShapeBuilder {
    material: Option<Arc<dyn Material>>,
    shape: Option<Box<dyn Shape>>,
}
impl ShapeBuilder {
    pub fn new() -> Self {
        Self {
            material: None,
            shape: None,
        }
    }
    // materials
    pub fn lambertian(mut self, albedo: Color) -> Self {
        self.material = Some(Arc::new(Lambertian::new(albedo)));
        self
    }
    pub fn metal(mut self, albedo: Color, fuzz: f64) -> Self {
        self.material = Some(Arc::new(Metal::new(albedo, fuzz)));
        self
    }
    pub fn dielectric(mut self, ri: f64) -> Self {
        self.material = Some(Arc::new(Dielectric::new(ri)));
        self
    }
    // shapes
    pub fn sphere(mut self, center: Point3, radius: f64) -> Self {
        self.shape = Some(Box::new(Sphere::new(
            center,
            radius,
            self.material.unwrap(),
        )));
        self.material = None;
        self
    }
    // build
    pub fn build(self) -> Box<dyn Shape> {
        self.shape.unwrap()
    }
}

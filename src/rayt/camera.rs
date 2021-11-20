use crate::rayt::*;

#[derive(Debug)]
pub struct Camera {
    pub origin: Point3,
    pub u: Vec3,
    pub v: Vec3,
    pub screen_origin: Vec3,
    pub lens_radius: f64,
}
impl Camera {
    pub fn from_lookat(
        origin: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
    ) -> Self {
        let origin_to_screen_center = origin - lookat;
        let half_h = (vfov.to_radians() * 0.5).tan() * origin_to_screen_center.length();
        let half_w = aspect * half_h;
        let u_unit = vup.cross(origin_to_screen_center).normalize();
        let v_unit = origin_to_screen_center.cross(u_unit).normalize();
        let half_u = half_w * u_unit;
        let half_v = half_h * v_unit;

        Self {
            origin,
            u: 2.0 * half_u,
            v: 2.0 * half_v,
            screen_origin: lookat - half_u - half_v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let r = Vec3::random_in_unit_sphere() * self.lens_radius;
        let origin_offset = self.u * r.x() + self.v * r.y();
        let origin = self.origin + origin_offset;
        Ray {
            origin,
            direction: self.screen_origin + self.u * u + self.v * v - origin,
        }
    }
}

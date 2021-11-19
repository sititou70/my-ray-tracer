use std::sync::Mutex;

use crate::rayt::*;
use image::{Rgb, RgbImage};
use rayon::prelude::*;

const SCALE: f32 = 0.5;
const IMAGE_WIDTH: u32 = (1280.0 * SCALE) as u32;
const IMAGE_HEIGHT: u32 = (720.0 * SCALE) as u32;
const SAMPLES_PER_PIXEL: usize = 32;
const GAMMA_FACTOR: f64 = 2.2;
const MAX_RAY_BOUNCE_DEPTH: usize = 50;
const OUTPUT_FILENAME: &str = "render.png";

pub trait Scene {
    fn camera(&self) -> Camera;
    fn trace(&self, ray: Ray, depth: usize) -> Color;
    fn width(&self) -> u32 {
        IMAGE_WIDTH
    }
    fn height(&self) -> u32 {
        IMAGE_HEIGHT
    }
    fn spp(&self) -> usize {
        SAMPLES_PER_PIXEL
    }
    fn aspect(&self) -> f64 {
        self.width() as f64 / self.height() as f64
    }
}

pub fn render(scene: impl Scene + Sync) {
    let camera = scene.camera();

    let all = scene.width() * scene.height();
    let cnt = Mutex::new(0);
    let progress_add = |n: u64| {
        let mut cnt = cnt.lock().unwrap();
        *cnt += n;

        if *cnt % 1000 == 0 {
            println!("[{} / {}] {}%", cnt, all, *cnt as f64 / all as f64 * 100.0)
        }
    };

    let mut img = RgbImage::new(scene.width(), scene.height());
    img.enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            let mut pixel_color = (0..scene.spp()).into_iter().fold(Color::zero(), |acc, _| {
                let [rx, ry, _] = Float3::random().to_array();
                let u = (*x as f64 + rx) / (scene.width() - 1) as f64;
                let v = ((scene.height() - *y - 1) as f64 + ry) / (scene.height() - 1) as f64;
                let ray = camera.ray(u, v);
                acc + scene.trace(ray, MAX_RAY_BOUNCE_DEPTH)
            });
            pixel_color /= scene.spp() as f64;
            let rgb = pixel_color.gamma(GAMMA_FACTOR).to_rgb();
            pixel[0] = rgb[0];
            pixel[1] = rgb[1];
            pixel[2] = rgb[2];

            progress_add(1);
        });
    img.save(OUTPUT_FILENAME).unwrap();
}

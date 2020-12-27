extern crate image;
use std::env;
use std::process;

mod vec3;
use vec3::{Vec3, Point3, Color};

fn create_pixel(color: &Vec3) -> image::Rgb<u8> {
    let pixel = image::Rgb(
        [
            (255.999 * color.x) as u8,
            (255.999 * color.y) as u8,
            (255.999 * color.z) as u8
        ]
    );

    pixel
}

fn create_image(width: usize, height: usize,
    original: Point3, horizontal: Vec3, vertical: Vec3, lower_left_corner: Vec3) -> image::RgbImage {
    let mut image = image::RgbImage::new(width as u32, height as u32);

    for i in 0..height {
        for j in 0..width {
            let u = j as f64 / (width - 1) as f64;
            let v = (height - i) as f64 / (height - 1) as f64;
            let ray = Ray{
                original,
                direction: lower_left_corner + u*horizontal + v*vertical - original
            };

            let color = ray_color(&ray);
            let pixel = create_pixel(&color);
            image.put_pixel(j as u32, i as u32, pixel);
        }
    }

    image
}

struct Ray {
    original: Point3,
    direction: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Point3 {
        self.original + t * self.direction
    }
}

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction;
    let t = 0.5 * unit_direction.y + 1.0;
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ray-tracing-rust filename.png");
        process::exit(1);
    }
    let filename = &args[1];

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1.0;

    let original = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = original - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let image = create_image(IMAGE_WIDTH, IMAGE_HEIGHT, original, horizontal, vertical, lower_left_corner);
    image.save(filename).expect("failed to save image!");
}

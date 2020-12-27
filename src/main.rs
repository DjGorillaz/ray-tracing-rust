extern crate image;
use std::env;
use std::process;

mod vec3;
use vec3::Vec3;

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

fn create_image(width: usize, height: usize) -> image::RgbImage {
    let mut image = image::RgbImage::new(width as u32, height as u32);

    for i in 0..height {
        for j in 0..width {
            let color = Vec3::new(
                j as f64 / (width - 1) as f64,
                (height - i) as f64 / (height - 1) as f64,
                0.25
            );

            let pixel = create_pixel(&color);
            image.put_pixel(j as u32, i as u32, pixel);
        }
    }

    image
}

fn main() {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ray-tracing-rust filename.png");
        process::exit(1);
    }
    let filename = &args[1];

    let image = create_image(IMAGE_WIDTH, IMAGE_HEIGHT);
    image.save(filename).expect("failed to save image!");
}

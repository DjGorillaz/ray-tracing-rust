extern crate image;
use std::env;
use std::process;

fn _write_ppm(width: usize, height: usize) {
    println!("P3\n{} {}\n{}", width, height, 255);

    for i in (0..height).rev() {
        for j in 0..width {
            let r = j as f32 / (width - 1) as f32;
            let g = i as f32 / (height - 1) as f32;
            let b = 0.25;

            let ir = 255.999 * r;
            let ig = 255.99 * g;
            let ib = 255.99 * b;

            println!("{} {} {}", ir as i32, ig as i32 , ib as i32);
        }
    }
}

fn create_image(width: usize, height: usize) -> image::RgbImage {
    let mut image = image::RgbImage::new(width as u32, height as u32);

    for i in 0..height {
        for j in 0..width {
            let r = j as f32 / (width - 1) as f32;
            let g = (height - i) as f32 / (height - 1) as f32;
            let b = 0.25;

            let ir = 255.999 * r;
            let ig = 255.999 * g;
            let ib = 255.999 * b;

            image.get_pixel_mut(j as u32, i as u32).0 = [ir as u8,
                                                         ig as u8,
                                                         ib as u8];
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

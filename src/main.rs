fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render
    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, 255);

    for i in (0..IMAGE_HEIGHT).rev() {
        for j in 0..IMAGE_WIDTH {
            let r = j as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = i as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25;

            let ir = 255.999 * r;
            let ig = 255.99 * g;
            let ib = 255.99 * b;

            println!("{} {} {}", ir as i32, ig as i32 , ib as i32);
        }
    }
}

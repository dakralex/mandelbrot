mod client;
mod complex;
mod image;
mod mandelbrot;

fn main() {
    let (width, height, max_iterations) = client::parse_args().unwrap();

    println!(
        "Generating Mandelbrot for {}x{} image (max_iterations: {})",
        width, height, max_iterations
    );

    let image = mandelbrot::generate_image(width, height, max_iterations);
    let pixel_count = image.get_mandelbrot_pixels();

    println!("Pixels in the set: {pixel_count}");

    client::save_to_file(&image, "mandelbrot.ppm");
}

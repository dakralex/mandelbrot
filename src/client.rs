use crate::image::Image;
use std::num::ParseIntError;
use std::{env, fs};

/// Parse the arguments given to the binary.
///
/// This function uses [`env::args()`] and parses the three arguments that
/// describe the `width`, `height`, and `max_iterations`. It returns the values
/// as a [`Result`], which boxes these arguments as a tuple.
pub fn parse_args() -> Result<(usize, usize, usize), ParseIntError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.len() > 4 {
        println!("Usage: {} <width> <height> <max_iterations>", args[0]);
    }

    let width = args[1].parse::<usize>()?;
    let height = args[2].parse::<usize>()?;
    let max_iter = args[3].parse::<usize>().unwrap_or(1024);

    Ok((width, height, max_iter))
}

/// Save the given `image` to a file given by `filename`.
pub fn save_to_file(image: &Image, filename: &str) {
    let mut s = format!("P3\n{} {}\n255\n", image.width, image.height);

    for y in 0..image.height {
        for x in 0..image.width {
            let pixel = image.get(x, y).unwrap();
            s.push_str(&format!("{}\n", pixel));
        }
    }

    fs::write(filename, s).expect("Error writing to disk!");
}

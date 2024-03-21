use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder, ImageError};
use num::Complex;
use rand::Rng;
use std::fs::File;
use std::io;
use std::thread::spawn;

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    /*
    Complex<f64> set the real and imaginary part to the type of f64
    Option is an emun type like Result, it takes two value, None or
    Some(val), and Some is a container that can contain somekind of
    values
    */
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        //z is in the ball with radio 2.0 then in the set,
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }

        z = z * z + c;
    }

    None
}

// #[test]
// fn test_points_for_mandelbrot_set() {
//     let limit = 255;
//     //0 is in the set
//     assert!(escape_time(Complex { re: 0.0, im: 0.0 }, limit).is_none());
//     //-1 is in the set
//     assert!(escape_time(Complex { re: -1.0, im: 0.0 }, limit).is_none());
//     //i is in the set
//     assert!(escape_time(Complex { re: 0.0, im: 1.0 }, limit).is_none());
//     //2i is not in the set
//     assert!(escape_time(Complex { re: 0.0, im: 2.0 }, limit).is_some());
//     //3 is not in the set
//     assert!(escape_time(Complex { re: 3.0, im: 0.0 }, limit).is_some());
//     //1 is not in the set
//     assert!(escape_time(Complex { re: 1.0, im: 0.0 }, limit).is_some());
//     //1+i is not in the set
//     assert!(escape_time(Complex { re: 1.0, im: 1.0 }, limit).is_some());
// }

/*
image (100, 100) pixel (20, 30)?
1, rectangle image -> rectangle on complex plane (100, 100) -> (10, 15)
2, (10, 20), (20, 5)
3. re: 10 + (20 / 100) * 10 = 12
4. im: 20 - (30 / 100) * 15 = 15.5
y increase, going down on image, going up on complex plane
*/
fn pixel_to_complex_number(
    image_dimension: (usize, usize),
    pixel_coordinate: (usize, usize),
    complex_upper_left: Complex<f64>,
    complex_right_bottom: Complex<f64>,
) -> Complex<f64> {
    let complex_plane_width = complex_right_bottom.re - complex_upper_left.re;
    let complex_plane_height = complex_upper_left.im - complex_right_bottom.im;
    Complex {
        re: complex_upper_left.re
            + (pixel_coordinate.0 as f64 / image_dimension.0 as f64) * complex_plane_width as f64,
        im: complex_upper_left.im
            - (pixel_coordinate.1 as f64 / image_dimension.1 as f64) * complex_plane_height as f64,
    }
}

#[test]
fn test_pixel_to_complex_number() {
    assert_eq!(
        pixel_to_complex_number(
            (100, 100),
            (20, 30),
            Complex { re: 10.0, im: 20.0 },
            Complex { re: 20.0, im: 5.0 }
        ),
        Complex { re: 12.0, im: 15.5 }
    );
}

fn render(
    pixels: &mut [u8],
    image_dimension: (usize, usize),
    complex_upper_left: Complex<f64>,
    complex_right_bottom: Complex<f64>,
) {
    assert!(pixels.len() == image_dimension.0 * image_dimension.1);
    for row in 0..image_dimension.1 {
        for column in 0..image_dimension.0 {
            let complex_number = pixel_to_complex_number(
                image_dimension,
                (column, row),
                complex_upper_left,
                complex_right_bottom,
            );
            pixels[row * image_dimension.0 + column] = match escape_time(complex_number, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

fn write_png(file_name: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), ImageError> {
    let output = File::create(file_name)?;
    let encoder = PngEncoder::new(output);
    encoder.write_image(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ExtendedColorType::L8,
    )?;

    Ok(())
}

fn run_in_single_thread() {
    let image_dimension = (1000, 750);
    let mut pixels = vec![0; image_dimension.0 * image_dimension.1];
    let upper_left = Complex {
        re: -1.20,
        im: 0.35,
    };
    let right_bottom = Complex { re: -1.0, im: 0.20 };
    render(&mut pixels, image_dimension, upper_left, right_bottom);
    write_png("mandelbrot.png", &pixels, image_dimension).expect("error writing png file");
}

/*
(1000, 750) -> 750/5 = 150
*/
fn run_in_multiple_threads() {
    let image_dimension = (1000, 750);
    let mut pixels = vec![0; image_dimension.0 * image_dimension.1];
    let upper_left = Complex {
        re: -1.20,
        im: 0.35,
    };
    let right_bottom = Complex { re: -1.0, im: 0.20 };
    //split the image into 5 bands vertically
    let threads = 5;
    let rows_per_band = image_dimension.1 / threads;
    {
        let bands: Vec<&mut [u8]> = pixels
            .chunks_mut(rows_per_band * image_dimension.0)
            .collect();
        //thread::spwaner
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = rows_per_band;
                let band_bounds = (image_dimension.0, height);
                let band_upper_left =
                    pixel_to_complex_number(image_dimension, (0, top), upper_left, right_bottom);
                let band_bottom_right = pixel_to_complex_number(
                    image_dimension,
                    (image_dimension.0, top + height),
                    upper_left,
                    right_bottom,
                );

                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_bottom_right);
                });
            }
        })
        .unwrap();
        write_png("mandelbrot.png", &pixels, image_dimension);
    }
}

fn main() {
    //run_in_multiple_threads();
    run_in_single_thread();
}

Let's put all things together, a complex number has two parts, its real part and its imaginary part. The coordinate of a pixel on an image has two parts, its x and 
y coordinates, therefore we can map a pixel from an image to a complex number. First we map the rectangle of the image to a rectangle area in the complex plane, these
two rectangle area need no to be the same. For example given an image with width 100 and height 100, its area is 100*100, we can map it to an area of 10 * 15 in complex
plane, first we select the upper left point in the complex plane such as (10, 20), then the right bottom point is (20, 5), then for a given pixel with coordinate(20, 30),
we can convert it to a complex number by do this:
```r
re = 10 + (20 / 100) * 10 = 12
im = 20 - (30 / 100) * 15 = 15.5
```
that is the pixel with coordiate (10 ,20) map to complex number {re:12, im:15.5}. Notice why we mapping y to im by using subtraction instead of add, 
because the y-axis is oppsite for image and complex plane, when y is increase, it will going down on the image but it will going up on complex plane.

let's state the mapping algorithm as following:

1, given image with dimension as: image_width, image_height,  select a mapping area on the complex plane with upper left point marked as upper_left and 
right bottom point as right_bottom.

2. compute the height and width of the area on complex plane as:
complex_plane_width = right_bottom.re - upper_left.re;
complex_plane_height = upper_left.im - lower_right.im;

3. given a pixel with cooridate x, y, we mapp x to the real part of a complex number by  :
re = upper_left.re + (x / image_width) * complex_plane_width

4, we map y to the imaginary part of the complex number by :
im = upper_left.im - (y / image_height) * complex_plane_height

we can put the mapping algorithm as following code:
```r

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
            + (pixel_coordinate.0 / image_dimension.0) as f64 * complex_plane_width,
        /*
        notice why we use - instead of + here, the y-axis of image is oppsite to the complex plane,
        when the y value increase, it will going up on the complex plane, but it will going down
        for the image.
        */
        im: complex_upper_left.im
            - (pixel_coordinate.1 / image_dimension.1) as f64 * complex_plane_height,
    }
}
```
Let's test its correctness by using following test case:
```r
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
```
run the command cargo test and make sure the test case can be passed. Now we can draw the Mandelbrot set on an png 
image. First we setup the dimension of the image (width, height), then we iterate every pixel row by row, convert the
pixel into complex number, check the number belongs to Mandelbrot set or not, if it is, then we set the color of that
pixel to black, otherwise we set it to white or gray, the code like following:
```r
fn render(
    pixels: &mut [u8],
    image_dimension: (usize, usize),
    complex_upper_left: Complex<f64>,
    complex_right_bottom: Complex<f64>,
) {
    //length of image buf should equal to image dimension
    assert!(pixels.len() == image_dimension.0 * image_dimension.1);
    for row in 0..image_dimension.1 {
        for column in 0..image_dimension.0 {
            let complex_number = pixel_to_complex_number(
                image_dimension,
                (column, row),
                complex_upper_left,
                complex_right_bottom,
            );
            /*
            if the complex number use less looping time to escape the ball,
            the color of the pixel will more approch to white,
            if the number belongs to the set, the color would be black
            */
            pixels[row * image_dimension.0 + column] = match escape_time(complex_number, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}
```
After desciding the color of each pixel, we can draw then as gray scale png image file and we have done this before:
```r
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
```
Then we put all things together in the main function:
```r
fn main() {
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
```
we can run the code and compute its running time by using following command:
```r
time cargo run
```
the result is like following:
```r
cargo run  3.47s user 0.10s system 60% cpu 5.899 total
```
and the image drawn like this:

![mandelbrot](https://github.com/wycl16514/rust_multithread/assets/7506958/19330a94-f75b-4154-ae89-acebc4ba5d25)

It is easy to upgrade the application to multi-thread, we have an image with dimension:(1000, 750), we can split the
whole image vertically into severay bands, we keep the width as the same, but divide the height into 5 parts,
each part with height 750/5 = 150, then we treat each band with dimension (1000, 150) as we treat the whole image like
above. Unfortunately we can't use the multi-thread code as last video because there is a variable scoping and life 
time problem. This problem is very obscure for us now, therefore we defer it to later videos and we come to the help
of a crat named crossbeam, add the dependency in cargo.toml:
```r
[dependencies]
image="0.25.0"
rand="0.8"
num="0.4"
crossbeam="0.8"
```
then we split the image into 5 bands vertically and treat each band like treating the whole image as before:
```r
fn run_in_multiple_threads() {
    let image_dimension = (1000, 750);
    let mut pixels = vec![0; image_dimension.0 * image_dimension.1];
    let upper_left = Complex {
        re: -1.20,
        im: 0.35,
    };
    let right_bottom = Complex { re: -1.0, im: 0.20 };
    //split the image into 5 bands
    let threads = 5;
    let rows_per_band = image_dimension.1 / threads;

    /*
    split the image into several bands, bands is a vector that
    contain vectors, we can seen it as two dimensional array
    */
    {
        let bands: Vec<&mut [u8]> = pixels
            .chunks_mut(rows_per_band * image_dimension.0)
            .collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
               /*
                get the band area on complex plane
                */
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

        write_png("mandelbrot.png", &pixels, image_dimension).expect("error writing png file");
    }
}

fn main() {
    run_in_multiple_threads();
}

```
Now we can run the code using "time cargo run":
```r
cargo run  6.40s user 0.12s system 141% cpu 4.597 total
```
it turns out slower than single thread, really out of my expectation, there are many reasons for this, but the main 
goal is to have a feeling for rust programming, we will not waste time on the reason as long as we gain some 
experience on writing rust code.

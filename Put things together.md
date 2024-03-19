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
run the command cargo test and make sure the test case can be passed.

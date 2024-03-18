Mandelbrot set is a math concept, if we draw them on an image, they looks like following:
![image](https://github.com/wycl16514/rust_multithread/assets/7506958/75c984bf-2531-498e-880a-5a5ee7b9e0c7)

in this section we will try to draw the mandelbrot set as grayscale png file just like above image. Points in the set actually from complex plane and satisfy an
equation. Let's give it a definition:

Mandelbrot set = {x | loop: z = z * z + x ; and z is initialily 0 and x is a given complex number such that is not turn in to infinity}

in common language , if we assign the value of 0 to z, and pick a complex number for x, if no matter how many times the loop z = z*z+x goes, we can find a finite
number M, such that |M| >= |z| (|| means the norm of complex number), then we say x belongs to Mandelbrot set. Here we need the loop to be infinity, 
but when we write it in code, we can let it run indefinitly,and given a complex number, how can we check it meets the need of Mandelbrot set?

In program, we can never do thing that are endless otherwise the program will stuck. We need to approximate it. We pick a M in advance(for example take it as 4.0)
and for the given complex number of c, and we run the equation z = z * z + c for given times(such as 100000) if the norm of z never exceed 2.0(it has been 
proved that if norm of z can exceed 2.0, than it will run into infinity), then we say c is belongs to Mandelbrot set. Let's see how to use write the code for it.

Since we are dealing with complex number, we need some crat to help us do the arithmetic operations that are following the rules of complex number, there is a crat
names num can help us, first we add it as dependency in cargo.toml:
```r
[dependencies]
num = "0.4"
```
Then in main.rs, we add a function that can check whether given complex number is belongs to Mandelbrot set or not:
```r
use num::Complex;

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    /*
    Complex<f64> set the real and imaginary part to the type of f64
    Option is an emun type like Result, it takes two value, None or
    Some(val), and Some is a container that can contain somekind of
    values
    */
    //re is real part and im is the imaginary part
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        //loop given times and check the norm square of z is never exceed 4.0
        if z.norm_sqr() > 4.0 {
            //c does not belong to the set
            return Some(i);
        }
        z = z * z + c;
    }

    //c belongs to the set
    None
}

#[test]
fn test_points_for_mandelbrot_set() {
    //loop 255 times for the check
    let limit = 255;
    //0 is in the set
    assert!(escape_time(Complex { re: 0.0, im: 0.0 }, limit).is_none());
    //-1 is in the set
    assert!(escape_time(Complex { re: -1.0, im: 0.0 }, limit).is_none());
    //i is in the set
    assert!(escape_time(Complex { re: 0.0, im: 1.0 }, limit).is_none());
    //2i is not in the set
    assert!(escape_time(Complex { re: 0.0, im: 2.0 }, limit).is_some());
    //3 is not in the set
    assert!(escape_time(Complex { re: 3.0, im: 0.0 }, limit).is_some());
}

fn main() {}

```
function escape_time used to check whether given number c is in the radio of 2 in the process of looping times given by limit, if the norm_square exceed 4.0, which
means the norm of z will exceed 2.0, then we return the looping times that z escape the ball with the radio of 2, if z never escape the ball, then we return None. 
We will used the return value to draw a grayscale png image later on.

We add a test to check whether given values are to the mandelbrot set or not, values like 0, -1, 1, i are known to belongs to the set and we expect the function 
return value of None, and values like 2i, 3, 1, 1+i are not in the set and we expect the function to return Some, run the code with command cargo test and make 
sure the test can be passed.



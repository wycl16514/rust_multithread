In this section, we will see how to rely on image crat of rust to quickly generate a gray scale png file, this capability will be used in our later
project. First we need to setup the image crat depenency, in cargo.toml, we add the following line:
```js
image="0.25.0"
rand = "0.8"
```
The image crat will help us encode a raw buffer with png format, and rand crat will help us generate random number in given range.

A gray png image is simply encode a raw buffer with given dimension in given format, the raw buffer is a vector of bit with type u8. If we want to
create a image with dimension of with width 640 and height 480, the length of the raw buffer as a vector should be 640*480, in the following code,
we will create a vector of type u8 with length of 640*480 and randomly set the value each element of the vector to the range of 0-256, and using
image crat to encode the buffer as png file ,then save it as a .png file on disk, following is the code:
```
use std;

use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder, ImageError};
use rand::Rng;
use std::fs::File;
use std::io;

fn test_question_mark_operator() -> Result<String, io::Error> {
     /*
    open an nonexistent file and the ? operator will cause the return
    of an Error object, it is equivalence to
    let res = File::open("nonexistent.txt");
    match res {
        Ok(file) => {}
        Err(err) => {return err;}
    }
    */
    File::open("nonexistent.txt")?;
    println!("this code can't be called because the ? cause a return");
    Ok("impossible to come here".to_string())
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

fn main() {
    let png_width = 640;
    let png_height = 480;
    let file_name = "gray.png";
    /*
    init the a vector with 640*480 elements and set each
    elemtent to the value of 0
     */
    let mut img_buf = vec![0; png_width * png_height];
    //loop over the buffer and randomly set its element value in the range of 0-256
    //the operator x..=y create an vector and init it with value from 0 to y
    for idx in 0..png_height * png_width {
        img_buf[idx] = rand::thread_rng().gen_range(0..=255);
    }

    /*
    create a disk file with given name, the ? is a operator, when a function call
    may return an Error object , the ? can help us save many typing.
    For example when using file::create to create a file, if the
    creation fail, the ? will automatically return the error object
    to the up level caller
    */
    let res = test_question_mark_operator();
    match res {
        Ok(name) => {
            println!("the name of file: {}", name);
        }
        Err(msg) => {
            println!("the error message: {}", msg);
        }
    }

    let write_res = write_png(file_name, &img_buf, (png_width, png_height));
    match write_res {
        Ok(()) => {
            println!("gray png file created successfully");
        }
        Err(msg) => {
            println!("gray png file created with error: {}", msg);
        }
    }
}

```

we have some pickups in the code, first is the x..=y, it is short hand for creating a vector with value range from x to y and include y. The second
is ? operator, when a function call is returning an Result type object, the Ok type will contain the normal return result and the Error type will
contain the given error, if the operation fail, the ? operator will return the Error type object , this can save us many typings, we don't need to
use the match to check the result type.

Running the above code we will get the follwing result:
```js
the error message: No such file or directory (os error 2)
gray png file created successfully
```
And a png file is created on the same directory :

/Users/my/Documents/rust udemy/code/multi-thread/gray.png

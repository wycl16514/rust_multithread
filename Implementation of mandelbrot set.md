Mandelbrot set is a math concept, if we draw them on an image, they looks like following:
![image](https://github.com/wycl16514/rust_multithread/assets/7506958/75c984bf-2531-498e-880a-5a5ee7b9e0c7)

in this section we will try to draw the mandelbrot set as grayscale png file just like above image. Points in the set actually from complex plane and satisfy an
equation. Let's give it a definition:

Mandelbrot set = {x | loop: z = z * z + x ; and z is initialily 0 and x is a given complex number such that is not turn in to infinity}

in common language , if we assign the value of 0 to z, and pick a complex number for x, if no matter how many times the loop z = z*z+x goes, we can find a finite
number M, such that |M| >= |z| (|| means the norm of complex number), then we say x belongs to Mandelbrot set. Here we need the loop to be infinity, 
but when we write it in code, we can let it run indefinitly,and given a complex number, how can we check it meets the need of Mandelbrot set?

In program, we can never do thing that are endless otherwise the program will stuck.



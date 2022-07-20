# Shapes using ppm file format.
This code generates ppm files.
Ppm is an image file format that contains uncompressed data for RGB values,
which makes them easy to edit without needing any dependency.

[Resource to understand PPM](https://www.youtube.com/watch?v=12IbpyFiIYE)

basic syntax for PPM
```ppm
# This is a comment
# Line below specifies what kind of magic number we will use
P6
# Line below specifies the dimensions
120 120
# Specifies the max color value
255
# Magic number refer to the way you will add values of rgb per pixel.
# If we use P3 instead of P6 we can use ASCII as magic number but it takes more than double the size if we just use unsigned int.
# Watch the resource above to understand this completely.
# Magic number here
```

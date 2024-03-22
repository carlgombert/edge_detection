# Edge Detection  
Tdge detection implementation in rust.  
This is used to find the boundaries of objects within images.  

This implementation works by converting the image to a greyscale and then using the Sodel algorithm to calculate changes in gradient in 3x3 kernels throughout the image. A larger change in gradient would indicate the egde of an object.  

## Sample Images

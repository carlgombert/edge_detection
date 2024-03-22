use image::{DynamicImage, GenericImage, GenericImageView};

fn main() {
    let mut img = image::open("replace with filepath").unwrap();

    let mut imgData:Vec<Vec<f64>> = greyScale(img.clone());

    let mut gradientData = convolute(imgData);

    writeImage(img.clone(), gradientData, "replace with output file name");
}

// convert color image to black and white by averaging rgb values for each pixel
// return 2d array of greyscale for each pixel
fn greyScale(img: DynamicImage) -> Vec<Vec<f64>>{
    let mut imgData:Vec<Vec<f64>>= Vec::with_capacity(img.height() as usize);

    for _ in 0..img.height() {
        let row: Vec<f64> = vec![0.0; img.width() as usize];
        imgData.push(row);
    }

    for y in 0..img.height(){
        for x in 0..img.width() {
            let tempCol = img.get_pixel(x, y);

            let mut intensity = (tempCol[0] as f64 + tempCol[1] as f64 + tempCol[2] as f64) / 3.0;

            imgData[y as usize][x as usize] = intensity;
        }
    }
    return imgData;
}


// uses Sobel algorithm to detect changes in gradient
fn convolute(imgData : Vec<Vec<f64>>) -> Vec<Vec<f64>>{

    let sobelX: Vec<Vec<f64>> = vec![ // 1D gausian filter * x derivative
        vec![1.0, 0.0, -1.0],
        vec![2.0, 0.0, -2.0],
        vec![1.0, 0.0, -1.0],
    ];

    let sobelY: Vec<Vec<f64>> = vec![ // y derivative * 1D gausian filter
        vec![1.0, 2.0, 1.0],
        vec![0.0, 0.0, 0.0],
        vec![-1.0, -2.0, -1.0],
    ];

    // gradient data for image
    let mut gradData:Vec<Vec<f64>>= Vec::with_capacity(imgData.capacity() as usize);

    for _ in 0..imgData.capacity() {
        let row: Vec<f64> = vec![0.0; imgData[0].capacity() as usize];
        gradData.push(row);
    }

    // max gradient, used to normalize data later by dividing all gradient values by this
    let mut maxGrad = 0.0;

    // parse through image in 3x3 kernels
    for y in 0..imgData.capacity()-2{
        for x in 0..imgData[0].capacity()-2 {
            let mut Gx = 0.0;
            let mut Gy = 0.0;

            // use sobel algorithm to compute changes in gradient for kernel
            for y2 in 0..3{
                for x2 in 0..3{
                    Gx += imgData[y+y2][x+x2] * sobelX[y2][x2];
                    Gy += imgData[y+y2][x+x2] * sobelY[y2][x2];
                }
            }

            // change in gradient for kernel
            let gradient = (Gx * Gx + Gy * Gy).sqrt();

            let max = (maxGrad as f64).max(gradient as f64);

            maxGrad = max;

            gradData[y+1][x+1] = gradient;

        }
    }
    
    for y in 0..imgData.capacity(){
        for x in 0..imgData[0].capacity() {
            gradData[y][x] = gradData[y][x]/maxGrad;
        }
    }

    return gradData;
}

// writes image to output file
fn writeImage(mut img: DynamicImage, imgData : Vec<Vec<f64>>, name : &str){

    for y in 0..img.height() as usize{
        for x in 0..img.width() as usize{
            let r = (255.99 * imgData[y][x]) as u8;
            let g = (255.99 * imgData[y][x]) as u8;
            let b = (255.99  *imgData[y][x]) as u8;

            let mut tempCol = img.get_pixel(x as u32, y as u32);

            tempCol[0] = r;
            tempCol[1] = g;
            tempCol[2] = b;

            img.put_pixel(x as u32, y as u32, tempCol);
        }
    }

    if let Err(e) = img.save(format!("./assets/{}{}",name,".jpg")) {
        eprintln!("Error saving image: {}", e);
    } else {
        println!("Image saved successfully!");
    }
}

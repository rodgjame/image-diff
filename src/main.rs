// TODO remove later
#![allow(unused)]

extern crate image;

use clap::Parser;
use image::{GenericImageView, ImageFormat, RgbImage, Rgb};

/// Detect the difference between 2 images
#[derive(Parser)]
struct Cli {
    // The first image location
    #[clap(parse(from_os_str))]
    first_image_loc: std::path::PathBuf,
    
    // The second image location
    #[clap(parse(from_os_str))]
    second_image_loc: std::path::PathBuf,

    // The results image location
    #[clap(parse(from_os_str))]
    results_loc: std::path::PathBuf
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Hello, world!");
    let args = Cli::parse();
    let first_image = image::open(&args.first_image_loc).unwrap();
    let second_image = image::open(&args.second_image_loc).unwrap();
    
    // if the images are not the same size,
    // then we don't want to process them
    if first_image.dimensions() != second_image.dimensions() {
        return Err("The images have a different size. Can't continue".into());
    }

    // let's count the number of pixles
    let mut count = 0;
    for i in first_image.pixels() {
        count += 1;
    }
    println!("The first image has {} pixles", count);

    let first_image_width = first_image.width();
    let first_image_height = first_image.height();
    let mut diff_image = RgbImage::new(first_image_width, first_image_height);
    let mut x = 0;
    let mut y = 0;
    let mut different_pixles = 0;
    while x < first_image_width {
        while y < first_image_height {
            //println!("{}-{}", x, y);
            //diff_image.put_pixel(x, y, Rgb([255, 0, 255]));
            if (first_image.as_rgb8().unwrap().get_pixel(x, y) != second_image.as_rgb8().unwrap().get_pixel(x, y)) {
                println!("{:?} -- {:?}", first_image.as_rgb8().unwrap().get_pixel(x, y), second_image.as_rgb8().unwrap().get_pixel(x, y));
                different_pixles += 1;
                diff_image.put_pixel(x, y, Rgb([255, 0, 255]));
            }
            y += 1;
        }
        y = 0;
        x += 1;
    }

    println!("The number of different pixles is {}", different_pixles);
    println!("Saving diff image to {:?}", &args.results_loc);
    diff_image.save(&args.results_loc).unwrap();
    println!("diff image saved");
    
    Ok(())
}

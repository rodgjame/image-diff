// TODO remove later
#![allow(unused)]

extern crate image;

use clap::Parser;
use image::{GenericImageView, ImageFormat, RgbaImage, Rgba, Pixel};
use std::cmp::{min, max};

/// Detect the difference between 2 images
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // The first image location
    #[clap(parse(from_os_str), short, long)]
    first_image: std::path::PathBuf,
    
    // The second image location
    #[clap(parse(from_os_str), short, long)]
    second_image: std::path::PathBuf,

    // The results image location
    #[clap(parse(from_os_str), short, long)]
    results_image: std::path::PathBuf
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Cli::parse();
    let first_image = image::open(&args.first_image).unwrap();
    let second_image = image::open(&args.second_image).unwrap();
    
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
    let mut diff_image = RgbaImage::new(first_image_width, first_image_height);
    let mut x = 0;
    let mut y = 0;
    let mut different_pixles = 0;
    while x < first_image_width {
        while y < first_image_height {
            let should_mark = compare_pixles(&first_image.get_pixel(x, y), &second_image.get_pixel(x, y));
            if should_mark == true {
                diff_image.put_pixel(x, y, Rgba([255, 0, 255, 255]));
            }
            y += 1;
        }

        y = 0;
        x += 1;
    }
    
    println!("Saving diff image to {:?}", &args.results_image);
    diff_image.save(&args.results_image).unwrap();
    println!("diff image saved");
    
    Ok(())
}

fn compare_pixles(pix_1: &image::Rgba<u8>, pix_2: &image::Rgba<u8>) -> bool {
    let mut should_mark = false;
    for i in 0..4 {
        should_mark = compare_single_channel(pix_1[i], pix_2[i]);
        if should_mark == true {
            break;
        }
    }
    return should_mark;
}

fn compare_single_channel(channel_1: u8, channel_2: u8) -> bool {
    let number_diff = (channel_1 as i16 - channel_2 as i16) as i16;
    let avg_of_channels = (channel_1 as i16 + channel_2 as i16) as f32 / 2 as f32;
    let percent_diff = (number_diff.abs() as f32 / avg_of_channels) * 100 as f32;
    return percent_diff >= 45 as f32;
}

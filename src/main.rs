use std::env;
use std::path::Path;
use image::ImageReader;
use image::imageops::FilterType::{Nearest, Triangle, CatmullRom, Gaussian, Lanczos3};

const ESCAPE: u32 = 0x1B;
const TOP_HALF_CHAR: u32 = 0x2580;
const BOTTOM_HALF_CHAR: u32 = 0x2584;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;        
    }
    
    match args[1].as_str() {
        "--help" | "-h"=>help(),
        _=>render_image(args)
    }
}

fn render_image(args: Vec<String>) {
    let path = Path::new(&args[1]);

    let mut width: u32 = 64;
    let mut height: u32 = 64;

    if args.len() == 4 {
        width = args[2].parse::<u32>().expect("Invalid width");
        height = args[3].parse::<u32>().expect("Invalid height");
    } else if args.len() == 3 {
        width = args[2].parse::<u32>().expect("Invalid width/height");
        height = args[2].parse::<u32>().expect("Invalid width/height");
    }

    let escape = char::from_u32(ESCAPE).expect("Invalid Unicode code for ESCAPE");
    let top_half_char = char::from_u32(TOP_HALF_CHAR).expect("Invalid Unicode code for TOP_HALF_CHAR");
    let bottom_half_char = char::from_u32(BOTTOM_HALF_CHAR).expect("Invalid Unicode code for BOTTOM_HALF_CHAR");

    //actual logic
    let mut img = ImageReader::open(path).expect("Failed to open image").decode().expect("Failed to decode image");
    img = img.resize(width, height, CatmullRom);
    let rgba8 = img.to_rgba8();
    for pixel in rgba8.enumerate_pixels() {
        if pixel.1%2 == 0 {
            //bottom pixel does not exist
            if rgba8.get_pixel_checked(pixel.0, pixel.1+1) == None {
                //if transparent
                if pixel.2[3] < 250 {
                    print!("{}[0m ", escape);
                } else {
                    print!("{0}[38;2;{1};{2};{3}m", escape, pixel.2[0], pixel.2[1], pixel.2[2]);
                    print!("{}", top_half_char);
                }
            } else { //both pixels exist
                //if both are transparent
                if (rgba8.get_pixel(pixel.0, pixel.1+1)[3] < 250) && (pixel.2[3] < 250) {
                    print!("{}[0m ", escape);
                } else if rgba8.get_pixel(pixel.0, pixel.1+1)[3] < 250 { //if bottom is transparent
                    print!("{}[0m", escape);
                    print!("{0}[38;2;{1};{2};{3}m", escape, pixel.2[0], pixel.2[1], pixel.2[2]);
                    print!("{}", top_half_char);
                } else if pixel.2[3] < 250 { //if top is transparent
                    let bottom_pixel = rgba8.get_pixel(pixel.0, pixel.1+1);
                    print!("{}[0m", escape);
                    print!("{0}[38;2;{1};{2};{3}m", escape, bottom_pixel[0], bottom_pixel[1], bottom_pixel[2]);
                    print!("{}", bottom_half_char);
                } else { //normal case: neither are transparent   
                    let bottom_pixel = rgba8.get_pixel(pixel.0, pixel.1+1);
                    print!("{0}[38;2;{1};{2};{3}m", escape, pixel.2[0], pixel.2[1], pixel.2[2]);
                    print!("{0}[48;2;{1};{2};{3}m", escape, bottom_pixel[0], bottom_pixel[1], bottom_pixel[2]);
                    print!("{}", top_half_char);
                }
            }

            if pixel.0 == rgba8.width()-1 {
                println!("{}[0m", escape);
            }
        }
    }
    print!("{}[0m", escape);
}

fn help() {
    println!("\n\x1b[37;1mRenders an image in the console as ASCII characters\x1b[0m\n");
    println!("Main usage:");
    println!("consoleImageRender.exe [IMAGE] [WIDTH] [HEIGHT]");
    println!();
    println!("Available arguments:");
    println!("--help | -h : print this message")
}

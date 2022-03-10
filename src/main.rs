use indicatif::{ProgressBar, ProgressStyle};
use image::GenericImageView;

use std::time::Duration;
use std::vec::Vec;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Too many arguments passed into the application.");

    // TODO: Add support for an argument output path?
    const OUTPUT_PATH: &'static str = "output.jpg";
    let image_path = args[1].clone();

    let image = image::open(image_path.clone())
        .expect(format!("Cannot open an image from path {:?}.", image_path).as_str());
    
    if cfg!(debug_assertions) {
        println!("Image Path => {:?}", image_path);
        println!("Dimensions => {:?}", image.dimensions());
        println!("ColorType => {:?}", image.color());
    }

    // Progress Indicator
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(120).as_millis() as _);
    bar.set_message("Processing Greyscale");
    bar.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.bold.yellow} {msg:.yellow}")
            .tick_strings(&[
                "▏",
                "▎",
                "▍",
                "▌",
                "▋",
                "▊",
                "▉",
                "▊",
                "▋",
                "▌",
                "▍",
                "▎"
            ])
    );


    let image = image.grayscale();
    image.save(OUTPUT_PATH)
        .expect("Could properly save the image.");
    
    // Progress Finished Message
    bar.set_style( ProgressStyle::default_spinner().template("{msg:.bold.green}") );
    bar.finish_with_message(format!("▉ Finished greyscaling the image to {:?}.", OUTPUT_PATH));
}

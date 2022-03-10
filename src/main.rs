use image::GenericImageView;

use std::vec::Vec;
use std::env;



fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Too many arguments passed into the application.");

    let image_path = args[1].clone();

    let image = image::open(image_path.clone())
        .expect(format!("Cannot open an image from path {:?}.", image_path).as_str());
    
    if cfg!(debug_assertions) {
        println!("Image Path {:?}", image_path);
        println!("Dimensions {:?}", image.dimensions());
        println!("ColorType {:?}", image.color());
    }

    let image = image.grayscale();
    image.save("output.jpg")
        .expect("Could properly save the image.");
}

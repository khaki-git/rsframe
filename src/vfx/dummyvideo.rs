// THIS MODULE IS FOR TESTING THE RSFRAME CRATE
// DO NOT USE IT IN PRODUCTION

use image::{Rgb, RgbImage};

pub fn create_images(folder: String, frames: u8) {
    for f in 1..frames {
        let mut img = RgbImage::new(512, 512);

        for x in 1..512 {
            for y in 1..512 {
                img.put_pixel(x, y, Rgb([f, 0, 0]))
            }
        }
        img.save(format!("{}/image{}.bmp", folder, f)).expect("Could not save dummy video.");

        println!("Drew frame {}", f);
    }
}
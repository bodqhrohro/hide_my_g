extern crate image;
extern crate arguments;

use std::env;
use std::io;

fn sum_pix(base: u8, mixed: u8) -> u8 {
    base / 16 * 16 + mixed / 16
}

fn retrieve_pix(mix: u8) -> u8 {
    mix % 16 * 16
}

fn encode(args: arguments::Arguments) -> image::RgbaImage {
    if args.orphans.len() != 2 {
        panic!("There should be two filenames passed: the filename of the base image and of the mixed one");
    }

    //import
    let base_image = image::open(&args.orphans[0])
        .expect("Can't load the base image");
    let mixed_image = image::open(&args.orphans[1])
        .expect("Can't load the mixed image");

    let base_image = base_image.to_rgba();
    let mixed_image = mixed_image.to_rgba();

    if base_image.width() != mixed_image.width() || base_image.height() != mixed_image.height() {
        panic!("Image dimensions don't match");
    }

    // processing itself
    let result_buffer = image::RgbaImage::from_fn(base_image.width(), base_image.height(), |x, y| {
        let base_pixel = base_image.get_pixel(x, y).data;
        let mixed_pixel = mixed_image.get_pixel(x, y).data;

        image::Rgba {
            data: [
                sum_pix(base_pixel[0], mixed_pixel[0]),
                sum_pix(base_pixel[1], mixed_pixel[1]),
                sum_pix(base_pixel[2], mixed_pixel[2]),
                sum_pix(base_pixel[3], mixed_pixel[3]),
            ]
        }
    });

    result_buffer
}

fn decode(args: arguments::Arguments) -> image::RgbaImage {
    if args.orphans.len() != 1 {
        panic!("Pass the filename of the image containing the mixed image");
    }

    //import
    let mix_image = image::open(&args.orphans[0])
        .expect("Can't load the image");

    let mix_image = mix_image.to_rgba();

    // processing itself
    let result_buffer = image::RgbaImage::from_fn(mix_image.width(), mix_image.height(), |x, y| {
        let mix_pixel = mix_image.get_pixel(x, y).data;

        image::Rgba {
            data: [
                retrieve_pix(mix_pixel[0]),
                retrieve_pix(mix_pixel[1]),
                retrieve_pix(mix_pixel[2]),
                retrieve_pix(mix_pixel[3]),
            ]
        }
    });

    result_buffer
}

fn main() {
    let args = env::args();
    let args = arguments::parse(args).unwrap();

    let result_buffer: image::RgbaImage = match args.get::<String>("mode").as_ref().map(|s| &s[..]) {
        Some("encode") => encode(args),
        Some("decode") => decode(args),
        _ => panic!("Invalid mode"),
    };

    // export
    let mut stdout = io::stdout();
    let result_image = image::DynamicImage::ImageRgba8(result_buffer);
    result_image.save(&mut stdout, image::ImageFormat::PNG)
        .expect("Can't save the image");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_bw() {
        assert_eq!(15, sum_pix(0, 255));
    }

    #[test]
    fn encode_wb() {
        assert_eq!(240, sum_pix(255, 0));
    }

    #[test]
    fn encode_bb() {
        assert_eq!(0, sum_pix(0, 0));
    }

    #[test]
    fn encode_ww() {
        assert_eq!(255, sum_pix(255, 255));
    }

    #[test]
    fn encode_gg() {
        assert_eq!(119, sum_pix(127, 127));
    }

    #[test]
    fn encode_gg2() {
        assert_eq!(136, sum_pix(128, 128));
    }

    #[test]
    fn decode_bw() {
        assert_eq!(240, retrieve_pix(15));
    }

    #[test]
    fn decode_wb() {
        assert_eq!(0, retrieve_pix(240));
    }

    #[test]
    fn decode_bb() {
        assert_eq!(0, retrieve_pix(0));
    }

    #[test]
    fn decode_ww() {
        assert_eq!(240, retrieve_pix(255));
    }

    #[test]
    fn decode_gg() {
        assert_eq!(112, retrieve_pix(119));
    }

    #[test]
    fn decode_gg2() {
        assert_eq!(128, retrieve_pix(136));
    }
}

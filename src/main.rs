use image::{Rgb, RgbImage};
use julia_visualizer::JuliaFrame;
use num::complex::Complex;

fn main() {
    let mut frame = JuliaFrame::default();
    let center = Complex::new(0.0, 0.0);
    let julia_number = Complex::new(0.35, 0.35);

    frame = frame.into_recentered(center, julia_number);

    frame.iterate_escape_time(500);

    let width = frame.width();
    let height = frame.height();

    let mut img = RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let pixel = if frame.has_escaped(x, y).unwrap() {
                Rgb([255, 255, 255])
            } else {
                Rgb([0, 0, 0])
            };

            // let val = frame.value_at(x, y).unwrap().norm_sqr();
            // let color_val: u32 = (255.0 * val / 8.0) as u32;
            // let color_val: u8 = color_val.try_into().unwrap();
            // let pixel = Rgb([color_val, color_val, color_val]);

            // println!("{}", color_val);

            img.put_pixel(x, y, pixel)
        }
    }

    let res = img.save("output.png");
    match res {
        Ok(_) => println!("saved!"),
        Err(err) => println!("not saved :( because of error:\n{err}"),
    }
}

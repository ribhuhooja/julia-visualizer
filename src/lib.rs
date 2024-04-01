//! A library to visualize julia sets
//!
//! Provides functionality to interpolate smoothly between
//! julia sets defined on different coordinates, and provides
//! functionality to zoom

use num::{complex::Complex, range};

trait Frame {
    // get pixel rgb value
    // sliceablity
}

pub struct JuliaFrame {
    values: Vec<Complex<f32>>,
    coordinates: Vec<Complex<f32>>, // TODO: Make this a reference for optimizing space
    center: Complex<f32>,
    julia_number: Complex<f32>,
    width: u32,
    height: u32,
    cell_width: f32,
    cell_height: f32,
}

impl JuliaFrame {
    fn initial_values(
        center: Complex<f32>,
        width: u32,
        height: u32,
        cell_width: f32,
        cell_height: f32,
    ) -> Vec<Complex<f32>> {
        let mut result = Vec::new();

        for _ in range(0, width * height) {
            result.push(Complex::new(0.0, 0.0));
        }

        result
    }
}

impl Default for JuliaFrame {
    fn default() -> Self {
        let center = Complex { re: 0.0, im: 0.0 };
        let julia_number = Complex { re: 0.0, im: 0.0 };
        let (width, height) = (800, 600);
        let (cell_width, cell_height) = (0.005, 0.005);
        let coordinates =
            JuliaFrame::initial_values(center, width, height, cell_width, cell_height);
        let values = JuliaFrame::initial_values(center, width, height, cell_width, cell_height);

        Self {
            center,
            julia_number,
            width,
            height,
            cell_width,
            cell_height,
            coordinates,
            values,
        }
    }
}

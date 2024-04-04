//! A library to visualize julia sets
//!
//! Provides functionality to interpolate smoothly between
//! julia sets defined on different coordinates, and provides
//! functionality to zoom

use num::complex::Complex;

const ESCAPE_VALUE: f32 = 2.0 * 2.0;

pub trait Frame {
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
    pub fn new(
        center: Complex<f32>,
        width: u32,
        height: u32,
        cell_width: f32,
        cell_height: f32,
        julia_number: Complex<f32>,
    ) -> Self {
        let mut result = Self {
            center,
            width,
            height,
            cell_width,
            cell_height,
            julia_number,
            ..Default::default()
        };

        result.initialize();
        result
    }

    pub fn into_recentered(mut self, center: Complex<f32>, julia_number: Complex<f32>) -> Self {
        (self.center, self.julia_number) = (center, julia_number);
        self.initialize();
        self
    }

    fn initial_values(
        center: Complex<f32>,
        width: u32,
        height: u32,
        cell_width: f32,
        cell_height: f32,
    ) -> Vec<Complex<f32>> {
        let mut result = Vec::new();

        let top_left_corner = Complex::new(
            center.re - (width as f32) * cell_width / 2.0,
            center.im + (height as f32) * cell_height / 2.0,
        );

        for j in 0..height {
            for i in 0..width {
                let real_part = top_left_corner.re + (i as f32) * cell_width;
                let imaginary_part = top_left_corner.im - (j as f32) * cell_height;
                result.push(Complex::new(real_part, imaginary_part));
            }
        }

        result
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn initialize(&mut self) {
        let coordinates = Self::initial_values(
            self.center,
            self.width,
            self.height,
            self.cell_width,
            self.cell_height,
        );

        let values = coordinates.clone();

        self.values = values;
        self.coordinates = coordinates;
    }

    pub fn iterate_escape_time(&mut self, num_iters: u32) {
        for value_ref in &mut self.values {
            for _ in 0..num_iters {
                let value = *value_ref;
                if value.norm_sqr() > ESCAPE_VALUE {
                    break;
                }
                *value_ref = value * value + self.julia_number;
            }
        }
    }

    //TODO: remove this
    pub fn debug_iterate_escape_time(&mut self, num_iters: u32, x: u32, y: u32) {
        let i = Self::index_of(x, y, self.width);

        for _ in 0..num_iters {
            let value = self.values[i as usize];
            if value.norm_sqr() > ESCAPE_VALUE {
                break;
            }
            let value = value * value + self.julia_number;
            self.values[i as usize] = value;
            println!("{value}");
        }

        println!("{}", self.values[i as usize]);
    }

    pub fn value_at(&self, i: u32, j: u32) -> Option<&Complex<f32>> {
        self.values.get(Self::index_of(i, j, self.width) as usize)
    }

    pub fn has_escaped(&self, i: u32, j: u32) -> Option<bool> {
        let val = self.value_at(i, j)?;
        Some(val.norm_sqr() > ESCAPE_VALUE)
    }

    fn index_of(i: u32, j: u32, width: u32) -> u32 {
        width * j + i
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

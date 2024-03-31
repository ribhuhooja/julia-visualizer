//! A library to visualize julia sets
//!
//! Provides functionality to interpolate smoothly between
//! julia sets defined on different coordinates, and provides
//! functionality to zoom

use num::complex::Complex;

trait Frame {
    // get pixel rgb value
    // sliceablity
}

struct JuliaFrame<'a> {
    values: Vec<Complex<f32>>,
    coordinates: &'a Vec<Complex<f32>>,
}

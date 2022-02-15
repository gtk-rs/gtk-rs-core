// Take a look at the license at the top of the repository in the LICENSE file.

use crate::error::Error;
use crate::utils::status_to_result;

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq)]
#[doc(alias = "cairo_matrix_t")]
pub struct Matrix(ffi::cairo_matrix_t);

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

impl Matrix {
    pub(crate) fn ptr(&self) -> *const ffi::cairo_matrix_t {
        self as *const Matrix as _
    }

    pub(crate) fn mut_ptr(&mut self) -> *mut ffi::cairo_matrix_t {
        self as *mut Matrix as _
    }

    pub(crate) fn null() -> Self {
        Self(ffi::cairo_matrix_t {
            xx: 0.0,
            yx: 0.0,
            xy: 0.0,
            yy: 0.0,
            x0: 0.0,
            y0: 0.0,
        })
    }

    pub fn identity() -> Self {
        Self(ffi::cairo_matrix_t {
            xx: 1.0,
            yx: 0.0,
            xy: 0.0,
            yy: 1.0,
            x0: 0.0,
            y0: 0.0,
        })
    }

    pub fn new(xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64) -> Self {
        Self(ffi::cairo_matrix_t {
            xx,
            yx,
            xy,
            yy,
            x0,
            y0,
        })
    }

    pub fn xx(&self) -> f64 {
        self.0.xx
    }
    pub fn set_xx(&mut self, xx: f64) {
        self.0.xx = xx;
    }
    pub fn yx(&self) -> f64 {
        self.0.yx
    }
    pub fn set_yx(&mut self, yx: f64) {
        self.0.yx = yx;
    }
    pub fn xy(&self) -> f64 {
        self.0.xy
    }
    pub fn set_xy(&mut self, xy: f64) {
        self.0.xy = xy;
    }
    pub fn yy(&self) -> f64 {
        self.0.yy
    }
    pub fn set_yy(&mut self, yy: f64) {
        self.0.yy = yy;
    }
    pub fn x0(&self) -> f64 {
        self.0.x0
    }
    pub fn set_x0(&mut self, x0: f64) {
        self.0.x0 = x0;
    }
    pub fn y0(&self) -> f64 {
        self.0.y0
    }
    pub fn set_y0(&mut self, y0: f64) {
        self.0.y0 = y0;
    }

    #[doc(alias = "cairo_matrix_multiply")]
    pub fn multiply(left: &Matrix, right: &Matrix) -> Matrix {
        let mut matrix = Self::null();
        unsafe {
            ffi::cairo_matrix_multiply(matrix.mut_ptr(), left.ptr(), right.ptr());
        }
        matrix
    }

    #[doc(alias = "cairo_matrix_translate")]
    pub fn translate(&mut self, tx: f64, ty: f64) {
        unsafe { ffi::cairo_matrix_translate(self.mut_ptr(), tx, ty) }
    }

    #[doc(alias = "cairo_matrix_scale")]
    pub fn scale(&mut self, sx: f64, sy: f64) {
        unsafe { ffi::cairo_matrix_scale(self.mut_ptr(), sx, sy) }
    }

    #[doc(alias = "cairo_matrix_rotate")]
    pub fn rotate(&mut self, angle: f64) {
        unsafe { ffi::cairo_matrix_rotate(self.mut_ptr(), angle) }
    }

    #[doc(alias = "cairo_matrix_invert")]
    pub fn invert(&mut self) {
        let status = unsafe { ffi::cairo_matrix_invert(self.mut_ptr()) };
        status_to_result(status).expect("Failed to invert the matrix")
    }

    #[doc(alias = "cairo_matrix_invert")]
    pub fn try_invert(&self) -> Result<Matrix, Error> {
        let mut matrix = self.clone();

        let status = unsafe { ffi::cairo_matrix_invert(matrix.mut_ptr()) };
        status_to_result(status)?;
        Ok(matrix)
    }

    #[doc(alias = "cairo_matrix_transform_distance")]
    pub fn transform_distance(&self, _dx: f64, _dy: f64) -> (f64, f64) {
        let mut dx = _dx;
        let mut dy = _dy;

        unsafe {
            ffi::cairo_matrix_transform_distance(self.ptr(), &mut dx, &mut dy);
        }
        (dx, dy)
    }

    #[doc(alias = "cairo_matrix_transform_point")]
    pub fn transform_point(&self, _x: f64, _y: f64) -> (f64, f64) {
        let mut x = _x;
        let mut y = _y;

        unsafe {
            ffi::cairo_matrix_transform_point(self.ptr(), &mut x, &mut y);
        }
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_layout_is_ffi_equivalent() {
        let ffi_matrix = crate::ffi::cairo_matrix_t {
            xx: 1.0,
            yx: 2.0,
            xy: 3.0,
            yy: 4.0,
            x0: 5.0,
            y0: 6.0,
        };

        let transmuted: Matrix = unsafe { std::mem::transmute(ffi_matrix) };
        assert_eq!(
            transmuted,
            Matrix(crate::ffi::cairo_matrix_t {
                xx: 1.0,
                yx: 2.0,
                xy: 3.0,
                yy: 4.0,
                x0: 5.0,
                y0: 6.0,
            })
        );
    }

    #[test]
    fn invalid_matrix_does_not_invert() {
        let matrix = Matrix::null();
        assert!(matrix.try_invert().is_err());
    }

    #[test]
    #[should_panic]
    fn inverting_invalid_matrix_panics() {
        let mut matrix = Matrix::null();
        matrix.invert();
    }

    #[test]
    fn valid_matrix_try_invert() {
        let matrix = Matrix::identity();
        assert_eq!(matrix.try_invert().unwrap(), Matrix::identity());
    }

    #[test]
    fn valid_matrix_invert() {
        let mut matrix = Matrix::identity();
        matrix.invert();
        assert_eq!(matrix, Matrix::identity());
    }
}

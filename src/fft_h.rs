use super::*;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub(crate) struct FftComplex {
    pub(crate) real: f32,
    pub(crate) imag: f32,
}

/// replace 'struct fft_complex' with your type
pub(crate) type FftComplexT = FftComplex;

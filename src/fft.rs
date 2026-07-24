use super::*;
use crate::fft_h::FftComplexT;

#[inline]
#[allow(unused_doc_comments)]
extern "C" fn next_reversed_n(mut reversed_n: u64, shift: u64) -> u64 {
    reversed_n <<= shift;
    let count_leading_ones: u64 =
        unsafe { __builtin_clzl(!reversed_n) } as u64;
    reversed_n <<= count_leading_ones;

    /// remove leading ones
    (reversed_n |=
        (1 as u64) <<
            (core::mem::size_of::<u64>() as
                        u64).wrapping_mul(8 as u64).wrapping_sub(1 as u64));
    reversed_n >>= shift.wrapping_add(count_leading_ones);
    return reversed_n;
}

#[inline]
#[allow(unused_doc_comments)]
extern "C" fn rader(array: *const FftComplexT, target: *mut FftComplexT,
    logsize: u64) -> () {
    let size: u64 = (1 as u64) << logsize;
    /// how many bits should be shift to move the number to the most significant bit
    let shift: u64 =
        (core::mem::size_of::<u64>() as
                    u64).wrapping_mul(8 as u64).wrapping_sub(logsize);
    {
        let mut n: u64 = 0 as u64;
        let mut reversed_n: u64 = 0 as u64;
        '__b0: loop {
            if !(n < size) { break '__b0; }
            '__c0: loop {
                '__b1: loop {
                    '__c1: loop {
                        unsafe {
                            *target.add(reversed_n as usize) =
                                unsafe { *array.add(n as usize) }
                        };
                        break '__c1;
                    }
                    if !(0 != 0) { break '__b1; }
                }

                /// get next reversed_n
                (reversed_n = next_reversed_n(reversed_n, shift));
                break '__c0;
            }
            { let __p = &mut n; *__p = (*__p).wrapping_add(1); *__p };
        }
    }
}

extern "C" fn fft_raw(x: *mut FftComplexT, logsize: u64) -> () {
    if (!(logsize == 0 as u64) as i32 == 0) as i32 as i64 != 0 { return; }
    let begin: *mut FftComplexT = x;
    let end: *mut FftComplexT =
        unsafe { begin.add(((1 as u64) << logsize) as usize) };
    '__b2: loop {
        '__c2: loop {
            let mut unit: FftComplexT = unsafe { core::mem::zeroed() };
            '__b3: loop {
                '__c3: loop {
                    unit.real =
                        unsafe { cos(2 as f64 * 3.141592653589793 / 2 as f64) } as
                            f32;
                    unit.imag =
                        -unsafe { sin(2 as f64 * 3.141592653589793 / 2 as f64) } as
                            f32;
                    break '__c3;
                }
                if !(0 != 0) { break '__b3; }
            }
            let half: u64 = (2 / 2) as u64;
            {
                let mut p: *mut FftComplexT = begin;
                '__b4: loop {
                    if !(p != end) { break '__b4; }
                    '__c4: loop {
                        let mut t: FftComplexT = unsafe { core::mem::zeroed() };
                        let mut u: FftComplexT = unsafe { core::mem::zeroed() };
                        '__b5: loop {
                            '__c5: loop {
                                t = unsafe { *p.add(half as usize) };
                                break '__c5;
                            }
                            if !(0 != 0) { break '__b5; }
                        }
                        '__b6: loop {
                            '__c6: loop {
                                u = unsafe { *p.offset(0 as isize) };
                                break '__c6;
                            }
                            if !(0 != 0) { break '__b6; }
                        }
                        '__b7: loop {
                            '__c7: loop {
                                unsafe { (*p.offset(0 as isize)).real = u.real + t.real };
                                unsafe { (*p.offset(0 as isize)).imag = u.imag + t.imag };
                                break '__c7;
                            }
                            if !(0 != 0) { break '__b7; }
                        }
                        '__b8: loop {
                            '__c8: loop {
                                unsafe { (*p.add(half as usize)).real = u.real - t.real };
                                unsafe { (*p.add(half as usize)).imag = u.imag - t.imag };
                                break '__c8;
                            }
                            if !(0 != 0) { break '__b8; }
                        }
                        if half as u64 <= 1 as u64 { break '__c4; }
                        let mut root: FftComplexT = unsafe { core::mem::zeroed() };
                        '__b9: loop {
                            '__c9: loop { root = unit; break '__c9; }
                            if !(0 != 0) { break '__b9; }
                        }
                        '__b10: loop {
                            '__c10: loop {
                                t.real =
                                    root.real *
                                            unsafe {
                                                (*p.add(half.wrapping_add(1 as u64) as usize)).real
                                            } -
                                        root.imag *
                                            unsafe {
                                                (*p.add(half.wrapping_add(1 as u64) as usize)).imag
                                            };
                                t.imag =
                                    root.real *
                                            unsafe {
                                                (*p.add(half.wrapping_add(1 as u64) as usize)).imag
                                            } +
                                        root.imag *
                                            unsafe {
                                                (*p.add(half.wrapping_add(1 as u64) as usize)).real
                                            };
                                break '__c10;
                            }
                            if !(0 != 0) { break '__b10; }
                        }
                        '__b11: loop {
                            '__c11: loop {
                                u = unsafe { *p.offset(1 as isize) };
                                break '__c11;
                            }
                            if !(0 != 0) { break '__b11; }
                        }
                        '__b12: loop {
                            '__c12: loop {
                                unsafe { (*p.offset(1 as isize)).real = u.real + t.real };
                                unsafe { (*p.offset(1 as isize)).imag = u.imag + t.imag };
                                break '__c12;
                            }
                            if !(0 != 0) { break '__b12; }
                        }
                        '__b13: loop {
                            '__c13: loop {
                                unsafe {
                                    (*p.add(half.wrapping_add(1 as u64) as usize)).real =
                                        u.real - t.real
                                };
                                unsafe {
                                    (*p.add(half.wrapping_add(1 as u64) as usize)).imag =
                                        u.imag - t.imag
                                };
                                break '__c13;
                            }
                            if !(0 != 0) { break '__b13; }
                        }
                        {
                            let mut i: u64 = 2 as u64;
                            let mut j: u64 = half.wrapping_add(2 as u64);
                            '__b14: loop {
                                if !(i < half) { break '__b14; }
                                '__c14: loop {
                                    '__b15: loop {
                                        '__c15: loop {
                                            let r: f32 = root.real * unit.real - root.imag * unit.imag;
                                            let mut i: f32 =
                                                root.real * unit.imag + root.imag * unit.real;
                                            root.real = r;
                                            root.imag = i;
                                            break '__c15;
                                        }
                                        if !(0 != 0) { break '__b15; }
                                    }
                                    let mut t: FftComplexT = unsafe { core::mem::zeroed() };
                                    let mut u: FftComplexT = unsafe { core::mem::zeroed() };
                                    '__b16: loop {
                                        '__c16: loop {
                                            t.real =
                                                root.real * unsafe { (*p.add(j as usize)).real } -
                                                    root.imag * unsafe { (*p.add(j as usize)).imag };
                                            t.imag =
                                                root.real * unsafe { (*p.add(j as usize)).imag } +
                                                    root.imag * unsafe { (*p.add(j as usize)).real };
                                            break '__c16;
                                        }
                                        if !(0 != 0) { break '__b16; }
                                    }
                                    '__b17: loop {
                                        '__c17: loop {
                                            u = unsafe { *p.add(i as usize) };
                                            break '__c17;
                                        }
                                        if !(0 != 0) { break '__b17; }
                                    }
                                    '__b18: loop {
                                        '__c18: loop {
                                            unsafe { (*p.add(i as usize)).real = u.real + t.real };
                                            unsafe { (*p.add(i as usize)).imag = u.imag + t.imag };
                                            break '__c18;
                                        }
                                        if !(0 != 0) { break '__b18; }
                                    }
                                    '__b19: loop {
                                        '__c19: loop {
                                            unsafe { (*p.add(j as usize)).real = u.real - t.real };
                                            unsafe { (*p.add(j as usize)).imag = u.imag - t.imag };
                                            break '__c19;
                                        }
                                        if !(0 != 0) { break '__b19; }
                                    }
                                    break '__c14;
                                }
                                {
                                    { let __p = &mut i; *__p = (*__p).wrapping_add(1); *__p };
                                    { let __p = &mut j; *__p = (*__p).wrapping_add(1); *__p }
                                };
                            }
                        }
                        break '__c4;
                    }
                    {
                        let __n = 2;
                        let __p = &mut p;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                }
            }
            break '__c2;
        }
        if !(0 != 0) { break '__b2; }
    }
    if (!(logsize == 1 as u64) as i32 == 0) as i32 as i64 != 0 { return; }
    '__b20: loop {
        '__c20: loop {
            let mut unit: FftComplexT = unsafe { core::mem::zeroed() };
            '__b21: loop {
                '__c21: loop {
                    unit.real =
                        unsafe { cos(2 as f64 * 3.141592653589793 / 4 as f64) } as
                            f32;
                    unit.imag =
                        -unsafe { sin(2 as f64 * 3.141592653589793 / 4 as f64) } as
                            f32;
                    break '__c21;
                }
                if !(0 != 0) { break '__b21; }
            }
            let half: u64 = (4 / 2) as u64;
            {
                let mut p: *mut FftComplexT = begin;
                '__b22: loop {
                    if !(p != end) { break '__b22; }
                    '__c22: loop {
                        let mut t: FftComplexT = unsafe { core::mem::zeroed() };
                        let mut u: FftComplexT = unsafe { core::mem::zeroed() };
                        '__b23: loop {
                            '__c23: loop {
                                t = unsafe { *p.add(half as usize) };
                                break '__c23;
                            }
                            if !(0 != 0) { break '__b23; }
                        }
                        '__b24: loop {
                            '__c24: loop {
                                u = unsafe { *p.offset(0 as isize) };
                                break '__c24;
                            }
                            if !(0 != 0) { break '__b24; }
                        }
                        '__b25: loop {
                            '__c25: loop {
                                unsafe { (*p.offset(0 as isize)).real = u.real + t.real };
                                unsafe { (*p.offset(0 as isize)).imag = u.imag + t.imag };
                                break '__c25;
                            }
                            if !(0 != 0) { break '__b25; }
                        }
                        '__b26: loop {
                            '__c26: loop {
                                unsafe { (*p.add(half as usize)).real = u.real - t.real };
                                unsafe { (*p.add(half as usize)).imag = u.imag - t.imag };
                                break '__c26;
                            }
                            if !(0 != 0) { break '__b26; }
                        }
                        if half as u64 <= 1 as u64 { break '__c22; }
                        let mut root: FftComplexT = unsafe { core::mem::zeroed() };
                        '__b27: loop {
                            '__c27: loop { root = unit; break '__c27; }
                            if !(0 != 0) { break '__b27; }
                        }
                        '__b28: loop {
                            '__c28: loop {
                                t.real =
                                    root.real *
                                            unsafe {
                                                (*p.add(half.wrapping_add(1 as u64) as usize)).real
                                            } -
                                        root.imag *
                                            unsafe {
                                                (*p.add(half.wrapping_add(1 as u64) as usize)).imag
                                            };
                                t.imag =
                                    root.real *
                                            unsafe {
                                                (*p.add(half.wrapping_add(1 as u64) as usize)).imag
                                            } +
                                        root.imag *
                                            unsafe {
                                                (*p.add(half.wrapping_add(1 as u64) as usize)).real
                                            };
                                break '__c28;
                            }
                            if !(0 != 0) { break '__b28; }
                        }
                        '__b29: loop {
                            '__c29: loop {
                                u = unsafe { *p.offset(1 as isize) };
                                break '__c29;
                            }
                            if !(0 != 0) { break '__b29; }
                        }
                        '__b30: loop {
                            '__c30: loop {
                                unsafe { (*p.offset(1 as isize)).real = u.real + t.real };
                                unsafe { (*p.offset(1 as isize)).imag = u.imag + t.imag };
                                break '__c30;
                            }
                            if !(0 != 0) { break '__b30; }
                        }
                        '__b31: loop {
                            '__c31: loop {
                                unsafe {
                                    (*p.add(half.wrapping_add(1 as u64) as usize)).real =
                                        u.real - t.real
                                };
                                unsafe {
                                    (*p.add(half.wrapping_add(1 as u64) as usize)).imag =
                                        u.imag - t.imag
                                };
                                break '__c31;
                            }
                            if !(0 != 0) { break '__b31; }
                        }
                        {
                            let mut i: u64 = 2 as u64;
                            let mut j: u64 = half.wrapping_add(2 as u64);
                            '__b32: loop {
                                if !(i < half) { break '__b32; }
                                '__c32: loop {
                                    '__b33: loop {
                                        '__c33: loop {
                                            let r: f32 = root.real * unit.real - root.imag * unit.imag;
                                            let mut i: f32 =
                                                root.real * unit.imag + root.imag * unit.real;
                                            root.real = r;
                                            root.imag = i;
                                            break '__c33;
                                        }
                                        if !(0 != 0) { break '__b33; }
                                    }
                                    let mut t: FftComplexT = unsafe { core::mem::zeroed() };
                                    let mut u: FftComplexT = unsafe { core::mem::zeroed() };
                                    '__b34: loop {
                                        '__c34: loop {
                                            t.real =
                                                root.real * unsafe { (*p.add(j as usize)).real } -
                                                    root.imag * unsafe { (*p.add(j as usize)).imag };
                                            t.imag =
                                                root.real * unsafe { (*p.add(j as usize)).imag } +
                                                    root.imag * unsafe { (*p.add(j as usize)).real };
                                            break '__c34;
                                        }
                                        if !(0 != 0) { break '__b34; }
                                    }
                                    '__b35: loop {
                                        '__c35: loop {
                                            u = unsafe { *p.add(i as usize) };
                                            break '__c35;
                                        }
                                        if !(0 != 0) { break '__b35; }
                                    }
                                    '__b36: loop {
                                        '__c36: loop {
                                            unsafe { (*p.add(i as usize)).real = u.real + t.real };
                                            unsafe { (*p.add(i as usize)).imag = u.imag + t.imag };
                                            break '__c36;
                                        }
                                        if !(0 != 0) { break '__b36; }
                                    }
                                    '__b37: loop {
                                        '__c37: loop {
                                            unsafe { (*p.add(j as usize)).real = u.real - t.real };
                                            unsafe { (*p.add(j as usize)).imag = u.imag - t.imag };
                                            break '__c37;
                                        }
                                        if !(0 != 0) { break '__b37; }
                                    }
                                    break '__c32;
                                }
                                {
                                    { let __p = &mut i; *__p = (*__p).wrapping_add(1); *__p };
                                    { let __p = &mut j; *__p = (*__p).wrapping_add(1); *__p }
                                };
                            }
                        }
                        break '__c22;
                    }
                    {
                        let __n = 4;
                        let __p = &mut p;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                }
            }
            break '__c20;
        }
        if !(0 != 0) { break '__b20; }
    }
    if (!(logsize == 2 as u64) as i32 == 0) as i32 as i64 != 0 { return; }
    {
        let mut step: u64 = 8 as u64;
        '__b38: loop {
            if !(step <= (1 as u64) << logsize) { break '__b38; }
            '__c38: loop {
                '__b39: loop {
                    '__c39: loop {
                        let mut unit: FftComplexT = unsafe { core::mem::zeroed() };
                        '__b40: loop {
                            '__c40: loop {
                                unit.real =
                                    unsafe { cos(2 as f64 * 3.141592653589793 / step as f64) }
                                        as f32;
                                unit.imag =
                                    -unsafe { sin(2 as f64 * 3.141592653589793 / step as f64) }
                                        as f32;
                                break '__c40;
                            }
                            if !(0 != 0) { break '__b40; }
                        }
                        let half: u64 = (step / 2 as u64) as u64;
                        {
                            let mut p: *mut FftComplexT = begin;
                            '__b41: loop {
                                if !(p != end) { break '__b41; }
                                '__c41: loop {
                                    let mut t: FftComplexT = unsafe { core::mem::zeroed() };
                                    let mut u: FftComplexT = unsafe { core::mem::zeroed() };
                                    '__b42: loop {
                                        '__c42: loop {
                                            t = unsafe { *p.add(half as usize) };
                                            break '__c42;
                                        }
                                        if !(0 != 0) { break '__b42; }
                                    }
                                    '__b43: loop {
                                        '__c43: loop {
                                            u = unsafe { *p.offset(0 as isize) };
                                            break '__c43;
                                        }
                                        if !(0 != 0) { break '__b43; }
                                    }
                                    '__b44: loop {
                                        '__c44: loop {
                                            unsafe { (*p.offset(0 as isize)).real = u.real + t.real };
                                            unsafe { (*p.offset(0 as isize)).imag = u.imag + t.imag };
                                            break '__c44;
                                        }
                                        if !(0 != 0) { break '__b44; }
                                    }
                                    '__b45: loop {
                                        '__c45: loop {
                                            unsafe { (*p.add(half as usize)).real = u.real - t.real };
                                            unsafe { (*p.add(half as usize)).imag = u.imag - t.imag };
                                            break '__c45;
                                        }
                                        if !(0 != 0) { break '__b45; }
                                    }
                                    if half as u64 <= 1 as u64 { break '__c41; }
                                    let mut root: FftComplexT = unsafe { core::mem::zeroed() };
                                    '__b46: loop {
                                        '__c46: loop { root = unit; break '__c46; }
                                        if !(0 != 0) { break '__b46; }
                                    }
                                    '__b47: loop {
                                        '__c47: loop {
                                            t.real =
                                                root.real *
                                                        unsafe {
                                                            (*p.add(half.wrapping_add(1 as u64) as usize)).real
                                                        } -
                                                    root.imag *
                                                        unsafe {
                                                            (*p.add(half.wrapping_add(1 as u64) as usize)).imag
                                                        };
                                            t.imag =
                                                root.real *
                                                        unsafe {
                                                            (*p.add(half.wrapping_add(1 as u64) as usize)).imag
                                                        } +
                                                    root.imag *
                                                        unsafe {
                                                            (*p.add(half.wrapping_add(1 as u64) as usize)).real
                                                        };
                                            break '__c47;
                                        }
                                        if !(0 != 0) { break '__b47; }
                                    }
                                    '__b48: loop {
                                        '__c48: loop {
                                            u = unsafe { *p.offset(1 as isize) };
                                            break '__c48;
                                        }
                                        if !(0 != 0) { break '__b48; }
                                    }
                                    '__b49: loop {
                                        '__c49: loop {
                                            unsafe { (*p.offset(1 as isize)).real = u.real + t.real };
                                            unsafe { (*p.offset(1 as isize)).imag = u.imag + t.imag };
                                            break '__c49;
                                        }
                                        if !(0 != 0) { break '__b49; }
                                    }
                                    '__b50: loop {
                                        '__c50: loop {
                                            unsafe {
                                                (*p.add(half.wrapping_add(1 as u64) as usize)).real =
                                                    u.real - t.real
                                            };
                                            unsafe {
                                                (*p.add(half.wrapping_add(1 as u64) as usize)).imag =
                                                    u.imag - t.imag
                                            };
                                            break '__c50;
                                        }
                                        if !(0 != 0) { break '__b50; }
                                    }
                                    {
                                        let mut i: u64 = 2 as u64;
                                        let mut j: u64 = half.wrapping_add(2 as u64);
                                        '__b51: loop {
                                            if !(i < half) { break '__b51; }
                                            '__c51: loop {
                                                '__b52: loop {
                                                    '__c52: loop {
                                                        let r: f32 = root.real * unit.real - root.imag * unit.imag;
                                                        let mut i: f32 =
                                                            root.real * unit.imag + root.imag * unit.real;
                                                        root.real = r;
                                                        root.imag = i;
                                                        break '__c52;
                                                    }
                                                    if !(0 != 0) { break '__b52; }
                                                }
                                                let mut t: FftComplexT = unsafe { core::mem::zeroed() };
                                                let mut u: FftComplexT = unsafe { core::mem::zeroed() };
                                                '__b53: loop {
                                                    '__c53: loop {
                                                        t.real =
                                                            root.real * unsafe { (*p.add(j as usize)).real } -
                                                                root.imag * unsafe { (*p.add(j as usize)).imag };
                                                        t.imag =
                                                            root.real * unsafe { (*p.add(j as usize)).imag } +
                                                                root.imag * unsafe { (*p.add(j as usize)).real };
                                                        break '__c53;
                                                    }
                                                    if !(0 != 0) { break '__b53; }
                                                }
                                                '__b54: loop {
                                                    '__c54: loop {
                                                        u = unsafe { *p.add(i as usize) };
                                                        break '__c54;
                                                    }
                                                    if !(0 != 0) { break '__b54; }
                                                }
                                                '__b55: loop {
                                                    '__c55: loop {
                                                        unsafe { (*p.add(i as usize)).real = u.real + t.real };
                                                        unsafe { (*p.add(i as usize)).imag = u.imag + t.imag };
                                                        break '__c55;
                                                    }
                                                    if !(0 != 0) { break '__b55; }
                                                }
                                                '__b56: loop {
                                                    '__c56: loop {
                                                        unsafe { (*p.add(j as usize)).real = u.real - t.real };
                                                        unsafe { (*p.add(j as usize)).imag = u.imag - t.imag };
                                                        break '__c56;
                                                    }
                                                    if !(0 != 0) { break '__b56; }
                                                }
                                                break '__c51;
                                            }
                                            {
                                                { let __p = &mut i; *__p = (*__p).wrapping_add(1); *__p };
                                                { let __p = &mut j; *__p = (*__p).wrapping_add(1); *__p }
                                            };
                                        }
                                    }
                                    break '__c41;
                                }
                                {
                                    let __n = step;
                                    let __p = &mut p;
                                    *__p = unsafe { (*__p).add(__n as usize) };
                                };
                            }
                        }
                        break '__c39;
                    }
                    if !(0 != 0) { break '__b39; }
                }
                break '__c38;
            }
            step = step.wrapping_mul(2 as u64);
        }
    }
}

pub(crate) extern "C" fn fft(x: *const FftComplexT, x_1: *mut FftComplexT,
    logsize: u64) -> () {
    rader(x, x_1, logsize);
    fft_raw(x_1, logsize);
}

#[inline]
#[allow(unused_doc_comments)]
extern "C" fn rader_inplace(array: *mut FftComplexT, logsize: u64) -> () {
    let size: u64 = (1 as u64) << logsize;
    let shift: u64 =
        (core::mem::size_of::<u64>() as
                    u64).wrapping_mul(8 as u64).wrapping_sub(logsize);
    {
        let mut n: u64 = 1 as u64;
        let mut reversed_n: u64 = size >> 1;
        '__b57: loop {
            if !(n < size.wrapping_sub(1 as u64)) { break '__b57; }
            '__c57: loop {
                if n < reversed_n {
                    '__b58: loop {
                        '__c58: loop {
                            let tmp: FftComplexT = unsafe { *array.add(n as usize) };
                            unsafe {
                                *array.add(n as usize) =
                                    unsafe { *array.add(reversed_n as usize) }
                            };
                            unsafe { *array.add(reversed_n as usize) = tmp };
                            break '__c58;
                        }
                        if !(0 != 0) { break '__b58; }
                    }
                }

                /// get next reversed_n
                (reversed_n = next_reversed_n(reversed_n, shift));
                break '__c57;
            }
            { let __p = &mut n; *__p = (*__p).wrapping_add(1); *__p };
        }
    }
}

pub(crate) extern "C" fn fft_inplace(x: *mut FftComplexT, logsize: u64)
    -> () {
    rader_inplace(x, logsize);
    fft_raw(x, logsize);
}

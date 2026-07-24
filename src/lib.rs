#![allow(unused_imports, dead_code)]

mod fft;
mod fft_h;

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32)
    -> bool;
    fn __builtin_clzl(_: u64)
    -> i32;
    fn cos(_: f64)
    -> f64;
    fn sin(_: f64)
    -> f64;
    fn __builtin_clz(_: u32)
    -> i32;
    fn __builtin_clzll(_: u64)
    -> i32;
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}

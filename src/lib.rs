#![allow(unused_imports, dead_code)]

mod src;
mod test;
use crate::test::__main_inner;

pub(crate) type DarwinSizeT = u64;

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *const i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() {
        return 0;
    }
    return __r.unwrap_err();
}

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32) -> bool;
    fn malloc(__size: u64) -> *mut ();
    fn fmin(_: f64, _: f64) -> f64;
    fn fmax(_: f64, _: f64) -> f64;
    fn fabs(_: f64) -> f64;
    fn free(_: *mut ()) -> ();
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8) -> ();
    fn printf(_: *const i8, ...) -> i32;
    fn puts(_: *const i8) -> i32;
    fn __builtin_huge_valf() -> f32;
    fn __builtin_expect(_: i64, _: i64) -> i64;
}

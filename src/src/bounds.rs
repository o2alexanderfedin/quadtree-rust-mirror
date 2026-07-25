use super::*;
use crate::src::point::{quadtree_point_free, quadtree_point_new};
use crate::src::quadtree_h::QuadtreeBoundsT;
use crate::{fabs, fmax, fmin, free, malloc};

pub(crate) extern "C" fn quadtree_bounds_new() -> *mut QuadtreeBoundsT {
    let mut bounds: *mut QuadtreeBoundsT = core::ptr::null_mut();
    if {
        bounds = unsafe { malloc(core::mem::size_of::<QuadtreeBoundsT>() as u64) }
            as *mut QuadtreeBoundsT;
        bounds
    } as *mut ()
        == 0 as *mut ()
    {
        return 0 as *mut () as *mut QuadtreeBoundsT;
    }
    unsafe { (*bounds).nw = quadtree_point_new(f32::INFINITY as f64, -f32::INFINITY as f64) };
    unsafe { (*bounds).se = quadtree_point_new(-f32::INFINITY as f64, f32::INFINITY as f64) };
    unsafe { (*bounds).width = 0 as f64 };
    unsafe { (*bounds).height = 0 as f64 };
    return bounds;
}

pub(crate) extern "C" fn quadtree_bounds_extend(
    bounds: &mut QuadtreeBoundsT,
    x: f64,
    y: f64,
) -> () {
    unsafe { (*(*bounds).nw).x = unsafe { fmin(x, unsafe { (*(*bounds).nw).x }) } };
    unsafe { (*(*bounds).nw).y = unsafe { fmax(y, unsafe { (*(*bounds).nw).y }) } };
    unsafe { (*(*bounds).se).x = unsafe { fmax(x, unsafe { (*(*bounds).se).x }) } };
    unsafe { (*(*bounds).se).y = unsafe { fmin(y, unsafe { (*(*bounds).se).y }) } };
    (*bounds).width = unsafe { fabs(unsafe { (*(*bounds).nw).x } - unsafe { (*(*bounds).se).x }) };
    (*bounds).height = unsafe { fabs(unsafe { (*(*bounds).nw).y } - unsafe { (*(*bounds).se).y }) };
}

pub(crate) extern "C" fn quadtree_bounds_free(bounds: *mut QuadtreeBoundsT) -> () {
    quadtree_point_free(unsafe { (*bounds).nw });
    quadtree_point_free(unsafe { (*bounds).se });
    unsafe { free(bounds as *mut ()) };
}

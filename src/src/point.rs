use super::*;
use crate::{free, malloc};
use crate::src::quadtree_h::QuadtreePointT;

pub(crate) extern "C" fn quadtree_point_new(x: f64, y: f64)
    -> *mut QuadtreePointT {
    let mut point: *mut QuadtreePointT = core::ptr::null_mut();
    if ({
                            point =
                                unsafe {
                                        malloc(core::mem::size_of::<QuadtreePointT>() as u64)
                                    } as *mut QuadtreePointT;
                            point
                        }).is_null() as i32 != 0 {
        return 0 as *mut () as *mut QuadtreePointT;
    }
    unsafe { (*point).x = x };
    unsafe { (*point).y = y };
    return point;
}

pub(crate) extern "C" fn quadtree_point_free(point: *mut QuadtreePointT)
    -> () {
    unsafe { free(point as *mut ()) };
}

use super::*;
use crate::{free, malloc};
use crate::src::bounds::{
    quadtree_bounds_extend, quadtree_bounds_free, quadtree_bounds_new,
};
use crate::src::point::quadtree_point_free;
use crate::src::quadtree_h::{
    QuadtreeBoundsT, QuadtreeNode, QuadtreeNodeT, QuadtreePointT,
};

pub(crate) extern "C" fn quadtree_node_new() -> *mut QuadtreeNodeT {
    let mut node: *mut QuadtreeNodeT = core::ptr::null_mut();
    if ({
                            node =
                                unsafe {
                                        malloc(core::mem::size_of::<QuadtreeNodeT>() as u64)
                                    } as *mut QuadtreeNodeT;
                            node
                        }).is_null() as i32 != 0 {
        return 0 as *mut () as *mut QuadtreeNodeT;
    }
    unsafe { (*node).ne = 0 as *mut () as *mut QuadtreeNode };
    unsafe { (*node).nw = 0 as *mut () as *mut QuadtreeNode };
    unsafe { (*node).se = 0 as *mut () as *mut QuadtreeNode };
    unsafe { (*node).sw = 0 as *mut () as *mut QuadtreeNode };
    unsafe { (*node).point = 0 as *mut () as *mut QuadtreePointT };
    unsafe { (*node).bounds = 0 as *mut () as *mut QuadtreeBoundsT };
    unsafe { (*node).key = 0 as *mut () };
    return node;
}

pub(crate) extern "C" fn quadtree_node_reset(node: &QuadtreeNodeT,
    key_free: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> () {
    quadtree_point_free((*node).point);
    unsafe { key_free.unwrap()((*node).key) };
}

pub(crate) extern "C" fn quadtree_node_free(node: *mut QuadtreeNodeT,
    key_free: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> () {
    if unsafe { (*node).nw } as *mut () != 0 as *mut () {
        quadtree_node_free(unsafe { (*node).nw } as *mut QuadtreeNodeT,
            key_free);
    }
    if unsafe { (*node).ne } as *mut () != 0 as *mut () {
        quadtree_node_free(unsafe { (*node).ne } as *mut QuadtreeNodeT,
            key_free);
    }
    if unsafe { (*node).sw } as *mut () != 0 as *mut () {
        quadtree_node_free(unsafe { (*node).sw } as *mut QuadtreeNodeT,
            key_free);
    }
    if unsafe { (*node).se } as *mut () != 0 as *mut () {
        quadtree_node_free(unsafe { (*node).se } as *mut QuadtreeNodeT,
            key_free);
    }
    quadtree_bounds_free(unsafe { (*node).bounds });
    quadtree_node_reset(unsafe { &*node }, key_free);
    unsafe { free(node as *mut ()) };
}

pub(crate) extern "C" fn quadtree_node_isleaf(node: &QuadtreeNodeT) -> i32 {
    return ((*node).point as *mut () != 0 as *mut ()) as i32;
}

pub(crate) extern "C" fn quadtree_node_ispointer(node: *mut QuadtreeNodeT)
    -> i32 {
    return (unsafe { (*node).nw } as *mut () != 0 as *mut () &&
                            unsafe { (*node).ne } as *mut () != 0 as *mut () &&
                        unsafe { (*node).sw } as *mut () != 0 as *mut () &&
                    unsafe { (*node).se } as *mut () != 0 as *mut () &&
                (quadtree_node_isleaf(unsafe { &*node }) == 0) as i32 != 0) as
            i32;
}

pub(crate) extern "C" fn quadtree_node_isempty(node: *mut QuadtreeNodeT)
    -> i32 {
    return (unsafe { (*node).nw } as *mut () == 0 as *mut () &&
                            unsafe { (*node).ne } as *mut () == 0 as *mut () &&
                        unsafe { (*node).sw } as *mut () == 0 as *mut () &&
                    unsafe { (*node).se } as *mut () == 0 as *mut () &&
                (quadtree_node_isleaf(unsafe { &*node }) == 0) as i32 != 0) as
            i32;
}

pub(crate) extern "C" fn quadtree_node_with_bounds(minx: f64, miny: f64,
    maxx: f64, maxy: f64) -> *mut QuadtreeNodeT {
    let mut node: *mut QuadtreeNodeT = core::ptr::null_mut();
    if ({ node = quadtree_node_new(); node }).is_null() as i32 != 0 {
        return 0 as *mut () as *mut QuadtreeNodeT;
    }
    if ({
                            let __v = quadtree_bounds_new();
                            unsafe { (*node).bounds = __v };
                            __v
                        }).is_null() as i32 != 0 {
        return 0 as *mut () as *mut QuadtreeNodeT;
    }
    quadtree_bounds_extend(unsafe { &mut *unsafe { (*node).bounds } }, maxx,
        maxy);
    quadtree_bounds_extend(unsafe { &mut *unsafe { (*node).bounds } }, minx,
        miny);
    return node;
}

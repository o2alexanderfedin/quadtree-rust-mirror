use super::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct QuadtreePoint {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

pub(crate) type QuadtreePointT = QuadtreePoint;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct QuadtreeBounds {
    pub(crate) nw: *mut QuadtreePointT,
    pub(crate) se: *mut QuadtreePointT,
    pub(crate) width: f64,
    pub(crate) height: f64,
}

pub(crate) type QuadtreeBoundsT = QuadtreeBounds;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct QuadtreeNode {
    pub(crate) ne: *mut QuadtreeNode,
    pub(crate) nw: *mut QuadtreeNode,
    pub(crate) se: *mut QuadtreeNode,
    pub(crate) sw: *mut QuadtreeNode,
    pub(crate) bounds: *mut QuadtreeBoundsT,
    pub(crate) point: *mut QuadtreePointT,
    pub(crate) key: *mut (),
}

pub(crate) type QuadtreeNodeT = QuadtreeNode;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Quadtree {
    pub(crate) root: *mut QuadtreeNodeT,
    pub(crate) key_free: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    pub(crate) length: u32,
}

pub(crate) type QuadtreeT = Quadtree;

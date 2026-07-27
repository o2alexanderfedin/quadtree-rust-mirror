use super::*;
use crate::src::node::{
    quadtree_node_free, quadtree_node_isempty, quadtree_node_isleaf, quadtree_node_ispointer,
    quadtree_node_reset, quadtree_node_with_bounds,
};
use crate::src::point::{quadtree_point_free, quadtree_point_new};
use crate::src::quadtree_h::{QuadtreeNode, QuadtreeNodeT, QuadtreePointT, QuadtreeT};
use crate::test::{ascent, descent};
use crate::{free, malloc};

pub(crate) extern "C" fn quadtree_new(
    minx: f64,
    miny: f64,
    maxx: f64,
    maxy: f64,
) -> *mut QuadtreeT {
    let mut tree: *mut QuadtreeT = core::ptr::null_mut();
    if ({
        tree = unsafe { malloc(core::mem::size_of::<QuadtreeT>() as u64) } as *mut QuadtreeT;
        tree
    })
    .is_null() as i32
        != 0
    {
        return 0 as *mut () as *mut QuadtreeT;
    }
    unsafe { (*tree).root = quadtree_node_with_bounds(minx, miny, maxx, maxy) };
    if (unsafe { (*tree).root }).is_null() as i32 != 0 {
        return 0 as *mut () as *mut QuadtreeT;
    }
    unsafe { (*tree).key_free = None };
    unsafe { (*tree).length = 0 as u32 };
    return tree;
}

extern "C" fn elision_(key: *mut ()) -> () {}

pub(crate) extern "C" fn quadtree_free(tree: *mut QuadtreeT) -> () {
    if unsafe { (*tree).key_free.is_some() } {
        quadtree_node_free(unsafe { (*tree).root }, unsafe { (*tree).key_free });
    } else {
        quadtree_node_free(unsafe { (*tree).root }, Some(elision_));
    }
    unsafe { free(tree as *mut ()) };
}

extern "C" fn node_contains_(outer: &QuadtreeNodeT, it: &QuadtreePointT) -> i32 {
    return ((*outer).bounds as *mut () != 0 as *mut ()
        && unsafe { (*unsafe { (*(*outer).bounds).nw }).x } <= (*it).x
        && unsafe { (*unsafe { (*(*outer).bounds).nw }).y } >= (*it).y
        && unsafe { (*unsafe { (*(*outer).bounds).se }).x } >= (*it).x
        && unsafe { (*unsafe { (*(*outer).bounds).se }).y } <= (*it).y) as i32;
}

extern "C" fn get_quadrant_(
    root: &QuadtreeNodeT,
    point: *mut QuadtreePointT,
) -> *mut QuadtreeNodeT {
    if node_contains_(unsafe { &*(*root).nw }, unsafe { &*point }) != 0 {
        return (*root).nw as *mut QuadtreeNodeT;
    }
    if node_contains_(unsafe { &*(*root).ne }, unsafe { &*point }) != 0 {
        return (*root).ne as *mut QuadtreeNodeT;
    }
    if node_contains_(unsafe { &*(*root).sw }, unsafe { &*point }) != 0 {
        return (*root).sw as *mut QuadtreeNodeT;
    }
    if node_contains_(unsafe { &*(*root).se }, unsafe { &*point }) != 0 {
        return (*root).se as *mut QuadtreeNodeT;
    }
    return 0 as *mut () as *mut QuadtreeNodeT;
}

extern "C" fn find_(node: *mut QuadtreeNodeT, x: f64, y: f64) -> *mut QuadtreePointT {
    if (node).is_null() as i32 != 0 {
        return 0 as *mut () as *mut QuadtreePointT;
    }
    if quadtree_node_isleaf(unsafe { &*node }) != 0 {
        if unsafe { (*unsafe { (*node).point }).x } == x
            && unsafe { (*unsafe { (*node).point }).y } == y
        {
            return unsafe { (*node).point };
        }
    } else if quadtree_node_ispointer(node) != 0 {
        let mut test: QuadtreePointT = unsafe { core::mem::zeroed() };
        test.x = x;
        test.y = y;
        return find_(get_quadrant_(unsafe { &*node }, &mut test), x, y);
    }
    return 0 as *mut () as *mut QuadtreePointT;
}

pub(crate) extern "C" fn quadtree_search(tree: &QuadtreeT, x: f64, y: f64) -> *mut QuadtreePointT {
    return find_((*tree).root, x, y);
}

extern "C" fn reset_node_(tree: &QuadtreeT, node: *mut QuadtreeNodeT) -> () {
    if (*tree).key_free.is_some() {
        quadtree_node_reset(unsafe { &*node }, (*tree).key_free);
    } else {
        quadtree_node_reset(unsafe { &*node }, Some(elision_));
    }
}

extern "C" fn split_node_(tree: *mut QuadtreeT, node: *mut QuadtreeNodeT) -> i32 {
    let mut nw: *mut QuadtreeNodeT = core::ptr::null_mut();
    let mut ne: *mut QuadtreeNodeT = core::ptr::null_mut();
    let mut sw: *mut QuadtreeNodeT = core::ptr::null_mut();
    let mut se: *mut QuadtreeNodeT = core::ptr::null_mut();
    let mut old: *mut QuadtreePointT = core::ptr::null_mut();
    let mut key: *mut () = core::ptr::null_mut();
    let x: f64 = unsafe { (*unsafe { (*unsafe { (*node).bounds }).nw }).x };
    let y: f64 = unsafe { (*unsafe { (*unsafe { (*node).bounds }).nw }).y };
    let hw: f64 = unsafe { (*unsafe { (*node).bounds }).width } / 2 as f64;
    let hh: f64 = unsafe { (*unsafe { (*node).bounds }).height } / 2 as f64;
    if ({
        nw = quadtree_node_with_bounds(x, y - hh, x + hw, y);
        nw
    })
    .is_null() as i32
        != 0
    {
        return 0;
    }
    if ({
        ne = quadtree_node_with_bounds(x + hw, y - hh, x + hw * 2 as f64, y);
        ne
    })
    .is_null() as i32
        != 0
    {
        return 0;
    }
    if ({
        sw = quadtree_node_with_bounds(x, y - hh * 2 as f64, x + hw, y - hh);
        sw
    })
    .is_null() as i32
        != 0
    {
        return 0;
    }
    if ({
        se = quadtree_node_with_bounds(x + hw, y - hh * 2 as f64, x + hw * 2 as f64, y - hh);
        se
    })
    .is_null() as i32
        != 0
    {
        return 0;
    }
    unsafe { (*node).nw = nw as *mut QuadtreeNode };
    unsafe { (*node).ne = ne as *mut QuadtreeNode };
    unsafe { (*node).sw = sw as *mut QuadtreeNode };
    unsafe { (*node).se = se as *mut QuadtreeNode };
    old = unsafe { (*node).point };
    key = unsafe { (*node).key };
    unsafe { (*node).point = 0 as *mut () as *mut QuadtreePointT };
    unsafe { (*node).key = 0 as *mut () };
    return insert_(tree, node, old, key);
}

extern "C" fn insert_(
    tree: *mut QuadtreeT,
    root: *mut QuadtreeNodeT,
    point: *mut QuadtreePointT,
    key: *mut (),
) -> i32 {
    if quadtree_node_isempty(root) != 0 {
        unsafe { (*root).point = point };
        unsafe { (*root).key = key };
        return 1;
    } else if quadtree_node_isleaf(unsafe { &*root }) != 0 {
        if unsafe { (*unsafe { (*root).point }).x } == unsafe { (*point).x }
            && unsafe { (*unsafe { (*root).point }).y } == unsafe { (*point).y }
        {
            reset_node_(unsafe { &*tree }, root);
            unsafe { (*root).point = point };
            unsafe { (*root).key = key };
            return 2;
        } else {
            if (split_node_(tree, root) == 0) as i32 != 0 {
                return 0;
            }
            return insert_(tree, root, point, key);
        }
    } else if quadtree_node_ispointer(root) != 0 {
        let quadrant: *mut QuadtreeNodeT = get_quadrant_(unsafe { &*root }, point);
        return if quadrant as *mut () == 0 as *mut () {
            0
        } else {
            insert_(tree, quadrant, point, key)
        };
    }
    return 0;
}

pub(crate) extern "C" fn quadtree_insert(
    tree: *mut QuadtreeT,
    x: f64,
    y: f64,
    key: *mut (),
) -> i32 {
    let mut point: *mut QuadtreePointT = core::ptr::null_mut();
    let mut insert_status: i32 = 0;
    if ({
        point = quadtree_point_new(x, y);
        point
    })
    .is_null() as i32
        != 0
    {
        return 0;
    }
    if (node_contains_(unsafe { &*unsafe { (*tree).root } }, unsafe { &*point }) == 0) as i32 != 0 {
        quadtree_point_free(point);
        return 0;
    }
    if ({
        insert_status = insert_(tree, unsafe { (*tree).root }, point, key);
        insert_status
    } == 0) as i32
        != 0
    {
        quadtree_point_free(point);
        return 0;
    }
    if insert_status == 1 {
        {
            let __p = unsafe { &mut (*tree).length };
            let __t = *__p;
            *__p = (*__p).wrapping_add(1);
            __t
        };
    }
    return insert_status;
}

pub(crate) extern "C" fn quadtree_walk(
    root: *mut QuadtreeNodeT,
    descent: Option<unsafe extern "C" fn(*mut QuadtreeNode) -> ()>,
    ascent: Option<unsafe extern "C" fn(*mut QuadtreeNode) -> ()>,
) -> () {
    unsafe { descent.unwrap()(root as *mut QuadtreeNode) };
    if unsafe { (*root).nw } as *mut () != 0 as *mut () {
        quadtree_walk(unsafe { (*root).nw } as *mut QuadtreeNodeT, descent, ascent);
    }
    if unsafe { (*root).ne } as *mut () != 0 as *mut () {
        quadtree_walk(unsafe { (*root).ne } as *mut QuadtreeNodeT, descent, ascent);
    }
    if unsafe { (*root).sw } as *mut () != 0 as *mut () {
        quadtree_walk(unsafe { (*root).sw } as *mut QuadtreeNodeT, descent, ascent);
    }
    if unsafe { (*root).se } as *mut () != 0 as *mut () {
        quadtree_walk(unsafe { (*root).se } as *mut QuadtreeNodeT, descent, ascent);
    }
    unsafe { ascent.unwrap()(root as *mut QuadtreeNode) };
}

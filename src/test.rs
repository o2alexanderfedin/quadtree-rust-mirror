use super::*;
use crate::src::bounds::{
    quadtree_bounds_extend, quadtree_bounds_free, quadtree_bounds_new,
};
use crate::src::node::{
    quadtree_node_isempty, quadtree_node_isleaf, quadtree_node_ispointer,
    quadtree_node_new,
};
use crate::src::point::{quadtree_point_free, quadtree_point_new};
use crate::src::quadtree::{
    quadtree_free, quadtree_insert, quadtree_new, quadtree_search,
    quadtree_walk,
};
use crate::src::quadtree_h::{
    QuadtreeBoundsT, QuadtreeNodeT, QuadtreePointT, QuadtreeT,
};

pub(crate) extern "C" fn descent(node: *mut QuadtreeNodeT) -> () {}

pub(crate) extern "C" fn ascent(node: *mut QuadtreeNodeT) -> () {}

extern "C" fn test_node() -> () {
    let node: *mut QuadtreeNodeT = quadtree_node_new();
    if ((quadtree_node_isleaf(unsafe { &*node }) == 0) as i32 == 0) as i32 as
                i64 != 0 {
        unsafe {
            __assert_rtn(c"test_node".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 20,
                c"!quadtree_node_isleaf(node)".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if (quadtree_node_isempty(node) == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_node".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 21,
                c"quadtree_node_isempty(node)".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if ((quadtree_node_ispointer(node) == 0) as i32 == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_node".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 22,
                c"!quadtree_node_ispointer(node)".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    unsafe { free(node as *mut ()) };
}

extern "C" fn test_bounds() -> () {
    let bounds: *mut QuadtreeBoundsT = quadtree_bounds_new();
    if (bounds).is_null() as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_bounds".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 30,
                c"bounds".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*unsafe { (*bounds).nw }).x } == f32::INFINITY as f64) as
                    i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_bounds".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 31,
                c"bounds->nw->x == INFINITY".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*unsafe { (*bounds).se }).x } == -f32::INFINITY as f64) as
                    i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_bounds".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 32,
                c"bounds->se->x == -INFINITY".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    quadtree_bounds_extend(unsafe { &mut *bounds }, 5.0, 5.0);
    if !(unsafe { (*unsafe { (*bounds).nw }).x } == 5.0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_bounds".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 35,
                c"bounds->nw->x == 5.0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*unsafe { (*bounds).se }).x } == 5.0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_bounds".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 36,
                c"bounds->se->x == 5.0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    quadtree_bounds_extend(unsafe { &mut *bounds }, 10.0, 10.0);
    if !(unsafe { (*unsafe { (*bounds).nw }).y } == 10.0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_bounds".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 39,
                c"bounds->nw->y == 10.0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*unsafe { (*bounds).nw }).y } == 10.0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_bounds".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 40,
                c"bounds->nw->y == 10.0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*unsafe { (*bounds).se }).y } == 5.0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_bounds".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 41,
                c"bounds->se->y == 5.0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*unsafe { (*bounds).se }).y } == 5.0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_bounds".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 42,
                c"bounds->se->y == 5.0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*bounds).width } == 5.0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_bounds".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 44,
                c"bounds->width == 5.0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*bounds).height } == 5.0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_bounds".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 45,
                c"bounds->height == 5.0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    quadtree_bounds_free(bounds);
}

extern "C" fn test_tree() -> () {
    let mut val: i32 = 10;
    let tree: *mut QuadtreeT =
        quadtree_new(1 as f64, 1 as f64, 10 as f64, 10 as f64);
    if !(unsafe {
                                (*unsafe {
                                                (*unsafe { (*unsafe { (*tree).root }).bounds }).nw
                                            }).x
                            } == 1 as f64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 56,
                c"tree->root->bounds->nw->x == 1".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe {
                                (*unsafe {
                                                (*unsafe { (*unsafe { (*tree).root }).bounds }).nw
                                            }).y
                            } == 10.0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 57,
                c"tree->root->bounds->nw->y == 10.0".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe {
                                (*unsafe {
                                                (*unsafe { (*unsafe { (*tree).root }).bounds }).se
                                            }).x
                            } == 10.0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 58,
                c"tree->root->bounds->se->x == 10.0".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe {
                                (*unsafe {
                                                (*unsafe { (*unsafe { (*tree).root }).bounds }).se
                                            }).y
                            } == 1 as f64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 59,
                c"tree->root->bounds->se->y == 1".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if !(quadtree_insert(tree, 0 as f64, 0 as f64, &raw mut val as *mut ()) ==
                            0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 62,
                c"quadtree_insert(tree, 0, 0, &val) == 0".as_ptr() as *mut i8
                    as *const i8)
        }
    } else { { let _ = 0; } };
    if !(quadtree_insert(tree, 110.0, 110.0, &raw mut val as *mut ()) == 0) as
                    i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 63,
                c"quadtree_insert(tree, 110.0, 110.0, &val) == 0".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(quadtree_insert(tree, 8.0, 2.0, &raw mut val as *mut ()) != 0) as i32
                as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 65,
                c"quadtree_insert(tree, 8.0, 2.0, &val) != 0".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*tree).length } == 1 as u32) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 66,
                c"tree->length == 1".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*unsafe { (*unsafe { (*tree).root }).point }).x } == 8.0)
                    as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 67,
                c"tree->root->point->x == 8.0".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*unsafe { (*unsafe { (*tree).root }).point }).y } == 2.0)
                    as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 68,
                c"tree->root->point->y == 2.0".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if !(quadtree_insert(tree, 0.0, 1.0, &raw mut val as *mut ()) == 0) as i32
                as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 70,
                c"quadtree_insert(tree, 0.0, 1.0, &val) == 0".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(quadtree_insert(tree, 2.0, 3.0, &raw mut val as *mut ()) == 1) as i32
                as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 71,
                c"quadtree_insert(tree, 2.0, 3.0, &val) == 1".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(quadtree_insert(tree, 2.0, 3.0, &raw mut val as *mut ()) == 2) as i32
                as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 72,
                c"quadtree_insert(tree, 2.0, 3.0, &val) == 2".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*tree).length } == 2 as u32) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 73,
                c"tree->length == 2".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*unsafe { (*tree).root }).point } as *mut () ==
                            0 as *mut ()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 74,
                c"tree->root->point == NULL".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(quadtree_insert(tree, 3.0, 1.1, &raw mut val as *mut ()) == 1) as i32
                as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 76,
                c"quadtree_insert(tree, 3.0, 1.1, &val) == 1".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*tree).length } == 3 as u32) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 77,
                c"tree->length == 3".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*quadtree_search(unsafe { &*tree }, 3.0, 1.1)).x } == 3.0)
                    as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_tree".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 78,
                c"quadtree_search(tree, 3.0, 1.1)->x == 3.0".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    quadtree_walk(unsafe { (*tree).root }, Some(ascent), Some(descent));
    quadtree_free(tree);
}

extern "C" fn test_points() -> () {
    let point: *mut QuadtreePointT = quadtree_point_new(5 as f64, 6 as f64);
    if !(unsafe { (*point).x } == 5 as f64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_points".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 86,
                c"point->x == 5".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*point).y } == 6 as f64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"test_points".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8, 87,
                c"point->y == 6".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    quadtree_point_free(point);
}

pub(crate) extern "C" fn __main_inner(argc: i32, argv: *const *const i8)
    -> Result<(), i32> {
    unsafe {
        printf(c"\u{1b}[33mtree\u{1b}[0m ".as_ptr() as *mut i8 as *const i8)
    };
    test_tree();
    unsafe {
        puts(c"\u{1b}[1;32m\u{2713}\u{1b}[0m".as_ptr() as *mut i8 as
                *const i8)
    };
    unsafe {
        printf(c"\u{1b}[33mnode\u{1b}[0m ".as_ptr() as *mut i8 as *const i8)
    };
    test_node();
    unsafe {
        puts(c"\u{1b}[1;32m\u{2713}\u{1b}[0m".as_ptr() as *mut i8 as
                *const i8)
    };
    unsafe {
        printf(c"\u{1b}[33mbounds\u{1b}[0m ".as_ptr() as *mut i8 as *const i8)
    };
    test_bounds();
    unsafe {
        puts(c"\u{1b}[1;32m\u{2713}\u{1b}[0m".as_ptr() as *mut i8 as
                *const i8)
    };
    unsafe {
        printf(c"\u{1b}[33mpoints\u{1b}[0m ".as_ptr() as *mut i8 as *const i8)
    };
    test_points();
    unsafe {
        puts(c"\u{1b}[1;32m\u{2713}\u{1b}[0m".as_ptr() as *mut i8 as
                *const i8)
    };
    return Ok(());
}

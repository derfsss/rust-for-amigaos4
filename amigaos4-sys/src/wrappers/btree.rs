//! IBTree global(s) and convenience wrappers.
//!
//! Hand-written binding for btree.library — generic in-memory B-tree
//! container, used by various OS subsystems for keyed lookup.
//! Argument names taken from btree.library's AutoDocs where they
//! convey semantics; opaque cookie types are kept as `APTR`/`CONST_APTR`.

use crate::types::*;
use crate::interfaces::btree::*;

// ---- IBTree (BTreeIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IBTree: *mut BTreeIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IBTree: *mut BTreeIFace = core::ptr::null_mut();

// ── Tree lifecycle ───────────────────────────────────────────

#[inline]
pub unsafe fn btree_create_tree(node_count_hint: u32, args: *const BTArgArray) -> APTR {
    ((*IBTree).CreateTree)(IBTree, node_count_hint, args)
}

#[inline]
pub unsafe fn btree_delete_tree(tree: APTR) {
    ((*IBTree).DeleteTree)(IBTree, tree)
}

#[inline]
pub unsafe fn btree_flush_tree(tree: APTR) {
    ((*IBTree).FlushTree)(IBTree, tree)
}

// ── Node insertion / deletion ────────────────────────────────

#[inline]
pub unsafe fn btree_insert_tree_node(tree: APTR, key: CONST_APTR, data: CONST_APTR) -> APTR {
    ((*IBTree).InsertTreeNode)(IBTree, tree, key, data)
}

#[inline]
pub unsafe fn btree_delete_tree_node(tree: APTR, node: APTR) {
    ((*IBTree).DeleteTreeNode)(IBTree, tree, node)
}

#[inline]
pub unsafe fn btree_set_tree_node_data(tree: CONST_APTR, node: APTR, data: CONST_APTR) -> APTR {
    ((*IBTree).SetTreeNodeData)(IBTree, tree, node, data)
}

// ── Node accessors ───────────────────────────────────────────

#[inline]
pub unsafe fn btree_get_tree_node_key(tree: CONST_APTR, node: CONST_APTR) -> APTR {
    ((*IBTree).GetTreeNodeKey)(IBTree, tree, node)
}

#[inline]
pub unsafe fn btree_get_tree_node_data(tree: CONST_APTR, node: CONST_APTR) -> APTR {
    ((*IBTree).GetTreeNodeData)(IBTree, tree, node)
}

// ── Traversal (next / prev / min / max) ──────────────────────

#[inline]
pub unsafe fn btree_succ_tree_node(tree: CONST_APTR, node: CONST_APTR) -> APTR {
    ((*IBTree).SuccTreeNode)(IBTree, tree, node)
}

#[inline]
pub unsafe fn btree_pred_tree_node(tree: CONST_APTR, node: CONST_APTR) -> APTR {
    ((*IBTree).PredTreeNode)(IBTree, tree, node)
}

#[inline]
pub unsafe fn btree_min_tree_node(tree: CONST_APTR) -> APTR {
    ((*IBTree).MinTreeNode)(IBTree, tree)
}

#[inline]
pub unsafe fn btree_max_tree_node(tree: CONST_APTR) -> APTR {
    ((*IBTree).MaxTreeNode)(IBTree, tree)
}

// ── Lookup ───────────────────────────────────────────────────

#[inline]
pub unsafe fn btree_find_tree_node_by_key(tree: CONST_APTR, key: CONST_APTR) -> APTR {
    ((*IBTree).FindTreeNodeByKey)(IBTree, tree, key)
}

#[inline]
pub unsafe fn btree_find_tree_node_by_data(tree: CONST_APTR, data: CONST_APTR) -> APTR {
    ((*IBTree).FindTreeNodeByData)(IBTree, tree, data)
}

// ── Bulk traversal ───────────────────────────────────────────

#[inline]
pub unsafe fn btree_for_tree_nodes(
    tree: CONST_APTR, callback: APTR, user_data: CONST_APTR,
) -> u32 {
    ((*IBTree).ForTreeNodes)(IBTree, tree, callback, user_data)
}

#[inline]
pub unsafe fn btree_enum_tree_nodes(
    tree: CONST_APTR, low_key: CONST_APTR, high_key: CONST_APTR,
    callback: APTR, user_data: CONST_APTR,
) -> u32 {
    ((*IBTree).EnumTreeNodes)(IBTree, tree, low_key, high_key, callback, user_data)
}

// ── Tree statistics ──────────────────────────────────────────

#[inline]
pub unsafe fn btree_get_tree_size(tree: CONST_APTR) -> u32 {
    ((*IBTree).GetTreeSize)(IBTree, tree)
}

#[inline]
pub unsafe fn btree_get_tree_height(tree: CONST_APTR) -> u32 {
    ((*IBTree).GetTreeHeight)(IBTree, tree)
}

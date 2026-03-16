//! Safe tag list construction for AmigaOS API calls.
//!
//! Many AmigaOS functions accept a `*const TagItem` array terminated by
//! `TAG_DONE`. The [`TagListBuilder`] provides a safe builder pattern
//! for constructing these arrays.

use alloc::vec::Vec;
use amigaos4_sys::{Tag, TagItem, TAG_DONE, TAG_IGNORE};

/// Builder for constructing null-terminated TagItem arrays.
///
/// # Example
/// ```ignore
/// let tags = TagListBuilder::new()
///     .tag(AVT_TYPE, MEMF_SHARED)
///     .tag_if(clear, AVT_CLEAR_WITH_VALUE, 0xAA)
///     .build();
/// let ptr = tags.as_ptr();
/// // pass ptr to AllocVecTagList, NewObjectA, etc.
/// ```
pub struct TagListBuilder {
    tags: Vec<TagItem>,
}

impl TagListBuilder {
    /// Create a new empty builder.
    #[inline]
    pub fn new() -> Self {
        Self { tags: Vec::new() }
    }

    /// Append a tag/value pair.
    #[inline]
    pub fn tag(mut self, ti_Tag: Tag, ti_Data: u32) -> Self {
        self.tags.push(TagItem { ti_Tag, ti_Data });
        self
    }

    /// Append a tag/value pair only if `cond` is true; otherwise append TAG_IGNORE.
    #[inline]
    pub fn tag_if(mut self, cond: bool, ti_Tag: Tag, ti_Data: u32) -> Self {
        if cond {
            self.tags.push(TagItem { ti_Tag, ti_Data });
        } else {
            self.tags.push(TagItem { ti_Tag: TAG_IGNORE, ti_Data: 0 });
        }
        self
    }

    /// Finalize the list by appending TAG_DONE and returning the Vec.
    ///
    /// The caller must keep the returned Vec alive for as long as the OS
    /// may reference the pointer (i.e. across the OS call).
    #[inline]
    pub fn build(mut self) -> Vec<TagItem> {
        self.tags.push(TagItem { ti_Tag: TAG_DONE, ti_Data: 0 });
        self.tags
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amigaos4_sys::{TAG_DONE, TAG_IGNORE, TAG_USER};

    #[test]
    fn empty_build_has_tag_done() {
        let tags = TagListBuilder::new().build();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].ti_Tag, TAG_DONE);
        assert_eq!(tags[0].ti_Data, 0);
    }

    #[test]
    fn single_tag() {
        let tags = TagListBuilder::new()
            .tag(TAG_USER + 1, 42)
            .build();
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].ti_Tag, TAG_USER + 1);
        assert_eq!(tags[0].ti_Data, 42);
        assert_eq!(tags[1].ti_Tag, TAG_DONE);
    }

    #[test]
    fn multiple_tags() {
        let tags = TagListBuilder::new()
            .tag(TAG_USER + 1, 10)
            .tag(TAG_USER + 2, 20)
            .tag(TAG_USER + 3, 30)
            .build();
        assert_eq!(tags.len(), 4);
        assert_eq!(tags[0].ti_Tag, TAG_USER + 1);
        assert_eq!(tags[0].ti_Data, 10);
        assert_eq!(tags[1].ti_Tag, TAG_USER + 2);
        assert_eq!(tags[1].ti_Data, 20);
        assert_eq!(tags[2].ti_Tag, TAG_USER + 3);
        assert_eq!(tags[2].ti_Data, 30);
        assert_eq!(tags[3].ti_Tag, TAG_DONE);
    }

    #[test]
    fn tag_if_true() {
        let tags = TagListBuilder::new()
            .tag_if(true, TAG_USER + 5, 99)
            .build();
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].ti_Tag, TAG_USER + 5);
        assert_eq!(tags[0].ti_Data, 99);
    }

    #[test]
    fn tag_if_false() {
        let tags = TagListBuilder::new()
            .tag_if(false, TAG_USER + 5, 99)
            .build();
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].ti_Tag, TAG_IGNORE);
        assert_eq!(tags[0].ti_Data, 0);
    }

    #[test]
    fn mixed_tag_and_tag_if() {
        let tags = TagListBuilder::new()
            .tag(TAG_USER + 1, 100)
            .tag_if(true, TAG_USER + 2, 200)
            .tag_if(false, TAG_USER + 3, 300)
            .tag(TAG_USER + 4, 400)
            .build();
        assert_eq!(tags.len(), 5); // 4 entries + TAG_DONE
        assert_eq!(tags[0].ti_Tag, TAG_USER + 1);
        assert_eq!(tags[0].ti_Data, 100);
        assert_eq!(tags[1].ti_Tag, TAG_USER + 2);
        assert_eq!(tags[1].ti_Data, 200);
        assert_eq!(tags[2].ti_Tag, TAG_IGNORE); // false condition
        assert_eq!(tags[3].ti_Tag, TAG_USER + 4);
        assert_eq!(tags[3].ti_Data, 400);
        assert_eq!(tags[4].ti_Tag, TAG_DONE);
    }

    #[test]
    fn tag_data_zero() {
        let tags = TagListBuilder::new()
            .tag(TAG_USER + 1, 0)
            .build();
        assert_eq!(tags[0].ti_Data, 0);
    }

    #[test]
    fn tag_data_max_u32() {
        let tags = TagListBuilder::new()
            .tag(TAG_USER + 1, u32::MAX)
            .build();
        assert_eq!(tags[0].ti_Data, u32::MAX);
    }

    #[test]
    fn build_returns_owned_vec() {
        let tags = TagListBuilder::new()
            .tag(TAG_USER + 1, 1)
            .build();
        // Verify we can get a pointer (needed for OS calls)
        let _ptr = tags.as_ptr();
        assert!(!_ptr.is_null());
    }

    // ---- Sprint 3D: additional tests ----

    #[test]
    fn two_builds_independent() {
        let tags_a = TagListBuilder::new()
            .tag(TAG_USER + 1, 10)
            .build();
        let tags_b = TagListBuilder::new()
            .tag(TAG_USER + 2, 20)
            .build();
        // Different tags, independent Vecs
        assert_eq!(tags_a[0].ti_Tag, TAG_USER + 1);
        assert_eq!(tags_b[0].ti_Tag, TAG_USER + 2);
        assert_eq!(tags_a[0].ti_Data, 10);
        assert_eq!(tags_b[0].ti_Data, 20);
    }

    #[test]
    fn tag_item_size_is_8() {
        assert_eq!(core::mem::size_of::<TagItem>(), 8);
    }

    #[test]
    fn many_tags() {
        let mut builder = TagListBuilder::new();
        for i in 0..1000 {
            builder = builder.tag(TAG_USER + i, i);
        }
        let tags = builder.build();
        assert_eq!(tags.len(), 1001); // 1000 tags + TAG_DONE
        assert_eq!(tags[999].ti_Tag, TAG_USER + 999);
        assert_eq!(tags[1000].ti_Tag, TAG_DONE);
    }
}

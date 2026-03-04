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

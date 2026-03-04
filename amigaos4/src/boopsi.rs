use crate::error::{AmigaError, Result};
use amigaos4_sys::{APTR, TagItem};

/// RAII wrapper around NewObjectA / DisposeObject.
///
/// Automatically disposes the BOOPSI object on drop.
/// Use `into_raw()` to transfer ownership (e.g. to a parent layout).
pub struct AmigaObject {
    ptr: *mut APTR,
}

impl AmigaObject {
    /// Create a new BOOPSI object by class name.
    ///
    /// `class_id` must be a null-terminated byte string (e.g. `b"rootclass\0"`).
    /// `tags` should be a TAG_DONE-terminated array (use `TagListBuilder::build()`).
    pub fn new(class_id: &[u8], tags: &[TagItem]) -> Result<Self> {
        let ptr = unsafe {
            amigaos4_sys::intuition_new_object_a(
                core::ptr::null_mut(), // public class — no Class pointer
                class_id.as_ptr() as APTR,
                tags.as_ptr(),
            )
        };
        if ptr.is_null() {
            Err(AmigaError::AllocationFailed)
        } else {
            Ok(Self { ptr })
        }
    }

    /// Get an attribute value from the object.
    pub fn get_attr(&self, attr_id: u32) -> u32 {
        let mut value: u32 = 0;
        unsafe {
            amigaos4_sys::intuition_get_attr(
                attr_id,
                self.ptr,
                &mut value as *mut u32,
            );
        }
        value
    }

    /// Set attributes on the object using a TAG_DONE-terminated tag array.
    pub fn set_attrs(&self, tags: &[TagItem]) {
        unsafe {
            amigaos4_sys::intuition_set_attrs_a(self.ptr, tags.as_ptr());
        }
    }

    /// Dispatch a BOOPSI method with 0 extra arguments.
    pub fn do_method_0(&self, method_id: u32) -> u32 {
        unsafe { amigaos4_sys::amiga_do_method_0(self.ptr as APTR, method_id) }
    }

    /// Dispatch a BOOPSI method with 1 extra argument.
    pub fn do_method_1(&self, method_id: u32, a1: u32) -> u32 {
        unsafe { amigaos4_sys::amiga_do_method_1(self.ptr as APTR, method_id, a1) }
    }

    /// Dispatch a BOOPSI method with 2 extra arguments.
    pub fn do_method_2(&self, method_id: u32, a1: u32, a2: u32) -> u32 {
        unsafe { amigaos4_sys::amiga_do_method_2(self.ptr as APTR, method_id, a1, a2) }
    }

    /// Dispatch a BOOPSI method with 3 extra arguments.
    pub fn do_method_3(&self, method_id: u32, a1: u32, a2: u32, a3: u32) -> u32 {
        unsafe { amigaos4_sys::amiga_do_method_3(self.ptr as APTR, method_id, a1, a2, a3) }
    }

    /// Get the raw BOOPSI object pointer.
    #[inline]
    pub fn as_ptr(&self) -> *mut APTR {
        self.ptr
    }

    /// Consume the wrapper and return the raw pointer, preventing Drop from
    /// calling DisposeObject. Use when transferring ownership to a parent
    /// (e.g. LAYOUT_AddChild).
    #[inline]
    pub fn into_raw(self) -> *mut APTR {
        let ptr = self.ptr;
        core::mem::forget(self);
        ptr
    }
}

impl Drop for AmigaObject {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { amigaos4_sys::intuition_dispose_object(self.ptr) }
        }
    }
}

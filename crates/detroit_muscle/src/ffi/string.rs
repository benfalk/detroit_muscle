use super::*;
use std::ffi::CString;
use std::os::raw::c_char;

impl Compatibility for String {
    fn into_raw_pointer(self) -> RawPointer {
        let c_string = unsafe { CString::from_vec_unchecked(self.into_bytes()) };
        c_string.into_raw() as RawPointer
    }

    fn from_raw_pointer(ptr: RawPointer) -> Self {
        let c_string = unsafe { CString::from_raw(ptr as *mut c_char) };
        c_string.into_string().expect("A valid String structure")
    }

    fn free_raw_pointer(ptr: RawPointer) {
        unsafe { CString::from_raw(ptr as *mut c_char) };
    }
}

#[cfg(test)]
mod test {
    use crate::ffi::*;

    #[test]
    fn a_string_can_go_back_and_forth_to_a_raw_pointer() {
        let my_string = "hello world".to_owned();
        let ptr = my_string.into_raw_pointer();
        let returned = String::from_raw_pointer(ptr);
        assert_eq!(returned, "hello world");
    }

    #[test]
    fn a_raw_pointer_to_a_string_can_be_freed() {
        let ptr = "hello world".to_owned().into_raw_pointer();
        String::free_raw_pointer(ptr);
    }
}

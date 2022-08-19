use super::*;
use std::fmt::Display;
use std::marker::PhantomData;

#[derive(Eq, PartialEq, Debug)]
#[repr(u8)]
enum Status {
    Success = 0,
    Failure = 1,
}

///
/// FFI Result
///
/// The idea behind this is a way to convey a result
/// to the host caller.  Working with this type directly
/// probably should be avoided as it's intended use is
/// to ferry data from a result to the host language.
///
/// It will *NOT* free the success case pointer that it
/// generates; the host language is expected to call
/// for the success case to be freed.  So if you do plan
/// to use this ensure that the `ok` condition is properly
/// take care of.
///
#[repr(C)]
pub struct FfiResult<T: Compatibility> {
    status: Status,
    value: RawPointer,
    phantom: PhantomData<T>,
}

impl <T: Compatibility, S: Display> From<Result<T, S>> for FfiResult<T> {
    fn from(result: Result<T, S>) -> Self {
        match result {
            Ok(value) => FfiResult {
                status: Status::Success,
                value: value.into_raw_pointer(),
                phantom: PhantomData,
            },
            Err(err) => FfiResult {
                status: Status::Failure,
                value: err.to_string().into_raw_pointer(),
                phantom: PhantomData,
            }
        }
    }
}

impl <T: Compatibility> Compatibility for FfiResult<T> {
    fn into_raw_pointer(self) -> RawPointer {
        Box::into_raw(Box::new(self)) as RawPointer
    }

    fn from_raw_pointer(ptr: RawPointer) -> Self {
        let boxed = unsafe { Box::from_raw(ptr as *mut Self) };
        Box::into_inner(boxed)
    }

    fn free_raw_pointer(ptr: RawPointer) {
        let boxed = unsafe { Box::from_raw(ptr as *mut Self) };

        if boxed.status != Status::Success {
            String::free_raw_pointer(boxed.value);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn an_ok_result_can_be_converted_to_an_ffi_result() {
        let result: Result<String, String> = Ok("test".to_owned());
        let ffi = FfiResult::from(result);

        assert_eq!(ffi.status, Status::Success);
        assert_eq!(String::from_raw_pointer(ffi.value), "test");
    }

    #[test]
    fn an_error_result_can_be_converted_to_an_ffi_result() {
        let result: Result<String, u8> = Err(12);
        let ffi = FfiResult::from(result);

        assert_eq!(ffi.status, Status::Failure);
        assert_eq!(String::from_raw_pointer(ffi.value), "12");
    }
}

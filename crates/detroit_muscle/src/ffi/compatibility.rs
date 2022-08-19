use super::RawPointer;

pub trait Compatibility: Sized {
    ///
    /// Into Raw Pointer
    ///
    /// This is meant to convert a type to a C compatible
    /// structure, move it onto the heap if needed, and provide
    /// a raw pointer to it.
    ///
    fn into_raw_pointer(self) -> RawPointer;

    ///
    /// From Raw Pointer
    ///
    /// This is the reverse of `into_raw_pointer`.  It needs
    /// to reassemble to the original structure from which the
    /// pointer points to.
    ///
    fn from_raw_pointer(ptr: RawPointer) -> Self;

    //
    // Free Raw Pointer
    //
    // Ensures that the memory held by the pointer is released.
    // While this has a default implementation, it can be overriden
    // for more optimized implementations.
    //
    fn free_raw_pointer(ptr: RawPointer) {
        Self::from_raw_pointer(ptr);
    }
}

use super::ffi_common::*;

extern "C-unwind"{
    #[link_name = "from_cpp"]
    pub fn from_cpp() -> i32;
}

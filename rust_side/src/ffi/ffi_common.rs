// #[repr(C)]
// #[derive(Clone, Copy, PartialEq, Eq)]
// pub struct StringView{
//     pub size: usize,
//     pub ptr: *const u8,
// }

// impl StringView{
    
//     /// # Safety
//     /// The ptr MUST be pointing to valid data that is constant and size MUST be in bounds for the data
//     /// The data MUST be valud utf8
//     pub unsafe fn as_str(&self) -> &str{
//         std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.ptr, self.size))
//     }

//     pub fn from_str(str: &str) -> Self{
//         Self { size: str.len(), ptr: str.as_ptr() }
//     }
// }


#[repr(C)]
pub struct Array<T>{
    pub size: usize,
    pub ptr: *mut T,
}

impl<T> Array<T>{
    pub fn from_vec(mut vec: Vec<T>) -> Self{
        let tmp = Self { size: vec.len(), ptr: vec.as_mut_ptr() };
        std::mem::forget(vec);
        tmp
    }
}

pub type StringView = BasicStringView<std::os::raw::c_char>;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BasicStringView<T>(super::bindgen::root::std::basic_string_view<T>);

impl BasicStringView<std::os::raw::c_char>{
    
    /// # Safety
    /// The ptr MUST be pointing to valid data that is constant and size MUST be in bounds for the data
    /// The data MUST be valud utf8
    pub unsafe fn as_str(&self) -> &str{
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.0._M_str.cast(), self.0._M_len))
    }

    pub fn from_str(str: &str) -> Self{
        Self(super::bindgen::root::std::basic_string_view { _M_len: str.len(), _M_str: str.as_ptr().cast(), _phantom_0: Default::default() })
    }
}

impl<T> BasicStringView<T>{

}


fn test(){
    // Fn
}
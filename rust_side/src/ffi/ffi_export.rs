use super::ffi_common::*;

// #[export_name = "give_rust_slice"]
// pub extern "C-unwind" fn give_rust_slice(slice: StringView) -> Array<StringView>{
//     let str = unsafe{ slice.as_str() };
//     let vals = str.split(' ').map(StringView::from_str).collect::<Vec<_>>();
//     Array::from_vec(vals)
// }


#[export_name = "_ZN10ffi_import11get_integerEv"]
#[ignore = "always"]
pub extern "C-unwind" fn get_integer() -> i32{
    std::hint::black_box(666)
}

#[export_name = "_ZN10ffi_import13print_integerEi"]
pub extern "C-unwind" fn print_integer(val: i32){
    // println!("{val}");
}


// #[export_name = "_ZN10ffi_import15give_rust_sliceESt17basic_string_viewIcSt11char_traitsIcEE"]   
// pub extern "C-unwind" fn give_rust_slice(
//     slice: StringView,
// ) -> Array<StringView>{
//     let str = unsafe{ slice.as_str() };
//     let vals = str.split(' ').map(StringView::from_str).collect::<Vec<_>>();
//     Array::from_vec(vals)
// }

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Function<Args, R>{
    // nicenice lol
    whatever_cpp_wants: [*const(); 2],
    // this has something to do with the 'real' function being called?
    // cant be zero
    function_ptr: *const (),
    // call this number for a good time 
    // trampoline function?
    member_ptr: unsafe extern "C-unwind" fn (this: *const Function<Args, R>, args: *mut Args) -> R,
}

impl<Args, R> Function<Args, R>{
    pub fn verify(&self){
        if self.function_ptr.is_null(){
            unreachable!();
        }
    }
}

impl<Args, R> std::fmt::Debug for Function<Args, R>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function").field("whatever_cpp_wants", &self.whatever_cpp_wants).field("function_ptr", &self.function_ptr).field("member_ptr", &self.member_ptr).finish()
    }
}

#[export_name = "_ZN10ffi_import9callback_testESt8functionIFiSt17basic_string_viewIcSt11char_traitsIcEEEE"]   
pub unsafe extern "C-unwind" fn not_ok(
    v1: &Function<StringView, i32>,
) {
    v1.verify();
    let val = &mut StringView::from_str("asdasd");
    (v1.member_ptr)(v1, val as *mut StringView);
}

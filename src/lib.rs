extern crate octh;

use std::ffi::CString;

// https://thefullsnack.com/en/string-ffi-rust.html

#[no_mangle]
pub unsafe extern "C" fn Gadd (shl: *const octh::root::octave::dynamic_library, _relative: bool) -> *mut octh::root::octave_dld_function {

    let name = CString::new("add").unwrap();
    let pname = name.as_ptr() as *const octh::root::std::string;
    std::mem::forget(pname);

    let doc = CString::new("adds inputs and returns sum to all outputs").unwrap();
    let pdoc = doc.as_ptr() as *const octh::root::std::string;
    std::mem::forget(pdoc);
    
    let fcn = octh::root::octave_dld_function_create(Some(Fadd), shl, pname, pdoc);
    // if relative {
    //     fcn.mark_relative();
    // }
    return fcn;
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn Fadd (_args: *const octh::root::octave_value_list, nargout: i32) -> octh::root::octave_value_list {
    // let out = octh::root::octave_value_list::create();
    let out = octh::root::octave_value_list_create(nargout);
    return out;
}
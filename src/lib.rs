use std::ffi::CString;
use std::mem::MaybeUninit;
use core::ffi::c_void;

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

#[no_mangle]
pub unsafe extern "C" fn Fadd (_args: *const octh::root::octave_value_list, nargout: i32) -> octh::root::octave_value_list {
    // https://doc.rust-lang.org/core/mem/union.MaybeUninit.html
    // let out = octh::root::octave_value_list::create();
    // : octh::root::octave_value_list

    let mut list = MaybeUninit::<octh::root::octave_value_list>::uninit();
    let out = octh::root::octave_value_list_create(list.as_mut_ptr(), nargout);
    return list.assume_init();
    // error: out of memory or dimension too large for Octave's index type

    // let mut list = MaybeUninit::<octh::root::octave_value_list>::uninit();
    // let out = octh::root::octave_value_list::new(list.as_mut_ptr());
// 36 |     let out = octh::root::octave_value_list::new(list.as_mut_ptr());
//    |                                                  ^^^^^^^^^^^^^^^^^ expected array of 3 elements, found struct `octh::root::octave_value_list`
//    |
//    = note: expected type `*const [u64; 3]`
//               found type `*mut octh::root::octave_value_list`
    // return list.assume_init();

    // let mut list = MaybeUninit::<octh::root::octave_value_list>::uninit();
    // let mut v = octh::root::octave_value::new(1);
    // // https://stackoverflow.com/a/24191977/23059
    // let p_v: *mut c_void = &mut v as *mut _ as *mut c_void;
    // let list = octh::root::octave_base_value_list_value(p_v);
    // error: out of memory or dimension too large for Octave's index type

    return list;
}
use std::ffi::CString;
use std::ffi::CStr;
use std::mem::MaybeUninit;
use core::ffi::c_void;

// https://thefullsnack.com/en/string-ffi-rust.html

#[no_mangle]
pub unsafe extern "C" fn Gadd (shl: *const octh::root::octave::dynamic_library, _relative: bool) -> *mut octh::root::octave_dld_function {

    // let name = CString::new("add").unwrap();
    // let name = octh::root::stdstring_new("add");
    let name = CStr::from_bytes_with_nul(b"add\0").unwrap();
    let sname = octh::root::stdstring_new(name.as_ptr());
    let pname = sname as *const octh::root::std::string;
    // std::mem::forget(pname);

    // let doc = CString::new("adds inputs and returns sum to all outputs").unwrap();
    let doc = CStr::from_bytes_with_nul(b"adds inputs and returns sum to all outputs\0").unwrap();
    let sdoc = octh::root::stdstring_new(doc.as_ptr());
    let pdoc = sdoc as *const octh::root::std::string;
    // std::mem::forget(pdoc);

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

    let mut mlist = MaybeUninit::<octh::root::octave_value_list>::uninit();
    let plist = mlist.as_mut_ptr();
    octh::root::octave_value_list_new(plist, nargout);
    // let mut list = mlist.assume_init();

    let mut mvalue = MaybeUninit::<octh::root::octave_value>::uninit();
    let pvalue = mvalue.as_mut_ptr();
    octh::root::octave_value_octave_value(pvalue, 3);
    // let value = mvalue.assume_init();

    octh::root::octave_value_list_append(plist, pvalue);

    // return list;
    return mlist.assume_init();

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

    // return list;
}
use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::marker::PhantomData;

#[no_mangle]
pub unsafe extern "C" fn Gadd (shl: *const octh::root::octave::dynamic_library, _relative: bool) -> *mut octh::root::octave_dld_function {
    let name = CStr::from_bytes_with_nul(b"add\0").unwrap();
    let pmname = octh::root::stdstring_new(name.as_ptr());
    let sname = pmname as *const octh::root::std::string;

    let doc = CStr::from_bytes_with_nul(b"adds inputs and returns sum to all outputs\0").unwrap();
    let pmname = octh::root::stdstring_new(doc.as_ptr());
    let sdoc = pmname as *const octh::root::std::string;

    let fcn = octh::root::octave_dld_function_create(Some(Fadd), shl, sname, sdoc);
    // if relative {
    //     fcn.mark_relative();
    // }
    return fcn;
}

#[allow(non_snake_case)]
unsafe extern "C" fn Fadd (args: *const octh::root::octave_value_list, nargout: i32) -> octh::root::octave_value_list {
    let nargin = octh::root::octave_value_list_length(args);
    println!("nargin {}", nargin);
    println!("nargout {}", nargout);

    for i in 0..nargin {
        println!("\narg {}", i);
        let mut mvalue = MaybeUninit::<octh::root::octave_value>::uninit();
        let pvalue = mvalue.as_mut_ptr();
        octh::root::octave_value_list_get_value(pvalue, args, i);
        // let mut value = mvalue.assume_init();

        println!("is_scalar_type {}", octh::root::octave_value_is_scalar_type(pvalue));
        println!("is_string {}", octh::root::octave_value_is_string(pvalue));
        println!("is_true {}", octh::root::octave_value_is_true(pvalue));
        println!("is_uint16_type {}", octh::root::octave_value_is_uint16_type(pvalue));
        println!("is_uint32_type {}", octh::root::octave_value_is_uint32_type(pvalue));
        println!("is_uint64_type {}", octh::root::octave_value_is_uint64_type(pvalue));
        println!("is_uint8_type {}", octh::root::octave_value_is_uint8_type(pvalue));
        println!("isinteger {}", octh::root::octave_value_isinteger(pvalue));
        println!("isnull {}", octh::root::octave_value_isnull(pvalue));
        println!("isnumeric {}", octh::root::octave_value_isnumeric(pvalue));
        println!("isreal {}", octh::root::octave_value_isreal(pvalue));
    }

    let mut mlist = MaybeUninit::<octh::root::octave_value_list>::uninit();
    let plist = mlist.as_mut_ptr();
    octh::root::octave_value_list_new(plist, nargout);

    // let mut value = octh::root::octave_value::new63(&octh::root::octave_int32 {ival: 4, _phantom_0: PhantomData});
    // octh::root::octave_value_list_set_value(plist, 0, pvalue);

    return mlist.assume_init();
}
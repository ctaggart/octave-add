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
unsafe extern "C" fn Fadd (_args: *const octh::root::octave_value_list, nargout: i32) -> octh::root::octave_value_list {
    println!("nargout {}", nargout);

    let mut mlist = MaybeUninit::<octh::root::octave_value_list>::uninit();
    let plist = mlist.as_mut_ptr();
    octh::root::octave_value_list_new(plist, nargout);
    
    let mut value4 = octh::root::octave_value::new63(&octh::root::octave_int32 {ival: 4, _phantom_0: PhantomData});

    println!("is_scalar_type {}", octh::root::octave_value_is_scalar_type(&mut value4));
    println!("is_uint16_type {}", octh::root::octave_value_is_uint16_type(&mut value4));
    println!("is_uint32_type {}", octh::root::octave_value_is_uint32_type(&mut value4));
    println!("is_uint64_type {}", octh::root::octave_value_is_uint64_type(&mut value4));
    println!("is_uint8_type {}", octh::root::octave_value_is_uint8_type(&mut value4));
    println!("isinteger {}", octh::root::octave_value_isinteger(&mut value4));
    println!("isnull {}", octh::root::octave_value_isnull(&mut value4));
    println!("isnumeric {}", octh::root::octave_value_isnumeric(&mut value4));
    println!("isreal {}", octh::root::octave_value_isreal(&mut value4));

    octh::root::octave_value_list_set_value(plist, 0, &mut value4);

    return mlist.assume_init();
}
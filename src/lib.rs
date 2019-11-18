use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::marker::PhantomData;

#[no_mangle]
pub unsafe extern "C" fn Gadd (shl: *const octh::root::octave::dynamic_library, _relative: bool) -> *mut octh::root::octave_dld_function {

    // let name = CString::new("add").unwrap();
    // let name = octh::root::stdstring_new("add");
    let name = CStr::from_bytes_with_nul(b"add\0").unwrap();
    // let mut mname = MaybeUninit::<octh::root::stdstring>::uninit();
    // let pmname = mname.as_mut_ptr();
    // octh::root::stdstring_new(pmname, name.as_ptr());
    let pmname = octh::root::stdstring_new(name.as_ptr());
    let sname = pmname as *const octh::root::std::string;
    // std::mem::forget(pname);

    // let doc = CString::new("adds inputs and returns sum to all outputs").unwrap();
    let doc = CStr::from_bytes_with_nul(b"adds inputs and returns sum to all outputs\0").unwrap();
    // let mut mdoc = MaybeUninit::<octh::root::stdstring>::uninit();
    // let pmdoc = mdoc.as_mut_ptr();
    // octh::root::stdstring_new(pmdoc, doc.as_ptr());
    let pmname = octh::root::stdstring_new(doc.as_ptr());
    let sdoc = pmname as *const octh::root::std::string;
    // std::mem::forget(pdoc);

    let fcn = octh::root::octave_dld_function_create(Some(Fadd), shl, sname, sdoc);
    // if relative {
    //     fcn.mark_relative();
    // }
    return fcn;
}

#[allow(non_snake_case)]
unsafe extern "C" fn Fadd (_args: *const octh::root::octave_value_list, nargout: i32) -> octh::root::octave_value_list {
    // https://doc.rust-lang.org/core/mem/union.MaybeUninit.html
    // let out = octh::root::octave_value_list::create();
    // : octh::root::octave_value_list

    let mut mlist = MaybeUninit::<octh::root::octave_value_list>::uninit();
    let plist = mlist.as_mut_ptr();
    octh::root::octave_value_list_new(plist, nargout);
    // let mut list = mlist.assume_init();
    println!("nargout {}", nargout);

    // let mut mvalue = MaybeUninit::<octh::root::octave_value>::uninit();
    // let pvalue = mvalue.as_mut_ptr();
    // octh::root::octave_value_octave_value(pvalue, 2);
    // let value = octh::root::octave_value::new(2);
    
    // let mut mvalue4 = MaybeUninit::<octh::root::octave_value>::uninit();
    // let pvalue4 = mvalue4.as_mut_ptr();
    // octh::root::octave_value_octave_value(pvalue4, 4);
    // let mut value4 = octh::root::octave_value::new(4); // c_short
    // let mut value4 = octh::root::octave_value::new2(4); // c_int
    // let pvalue4: *mut octh::root::octave_value = &mut value4; // as *mut octh::root::octave_value;
    // let pvalue4 = &mut value4;

    // let mut value4 = octh::root::octave_value::new3(4); // c_uint
    let mut value4 = octh::root::octave_value::new63(&octh::root::octave_int32 {ival: 4, _phantom_0: PhantomData}); // octave_int32

    println!("is_scalar_type {}", octh::root::octave_value_is_scalar_type(&mut value4));
    println!("is_uint16_type {}", octh::root::octave_value_is_uint16_type(&mut value4));
    println!("is_uint32_type {}", octh::root::octave_value_is_uint32_type(&mut value4));
    println!("is_uint64_type {}", octh::root::octave_value_is_uint64_type(&mut value4));
    println!("is_uint8_type {}", octh::root::octave_value_is_uint8_type(&mut value4));
    println!("isinteger {}", octh::root::octave_value_isinteger(&mut value4));
    println!("isnull {}", octh::root::octave_value_isnull(&mut value4));
    println!("isnumeric {}", octh::root::octave_value_isnumeric(&mut value4));
    println!("isreal {}", octh::root::octave_value_isreal(&mut value4));

    // plist = octh::root::octave_value_list_append(plist, pvalue);
    // plist = octh::root::octave_value_list_append(plist, pvalue4);

    octh::root::octave_value_list_set_value(plist, 0, &mut value4);

    

    // return list;
    return mlist.assume_init();
}
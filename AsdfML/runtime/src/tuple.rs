use log::debug;

use crate::WithRaw;

// pub(crate) type Tuple = Vec<isize>;
pub(crate) type Tuple = safer_ffi::Vec<isize>; // Like regular Vec, but with #[repr(C)]

#[no_mangle]
pub extern "C" fn ml_create_tuple(size: usize) -> *mut Tuple {
    let tuple = Box::new(safer_ffi::c_vec![0isize; size]);
    debug!("Created tuple of size {} at {:?}", size, Box::as_ptr(&tuple));
    Box::into_raw(tuple)
}

#[no_mangle]
pub unsafe extern "C" fn ml_set_tuple_field(tuple_ptr: *mut Tuple, idx: usize, value: isize) {
    Box::with_raw_mut(tuple_ptr, |tuple| {
        tuple[idx] = value;
        debug!("Set [{}] = {} in {:?} at {:?}", idx, value, tuple, tuple_ptr);
    });
}

#[no_mangle]
pub unsafe extern "C" fn ml_get_tuple_field(tuple_ptr: *const Tuple, idx: usize) -> isize {
    Box::with_raw_ref(tuple_ptr, |tuple| {
        debug!("Getting {} of {:?} at {:?}", idx, tuple, tuple_ptr);
        *tuple
            .get(idx)
            .unwrap_or_else(|| panic!("Index {} is out of bounds for tuple {:?}", idx, tuple))
    })
}

#[no_mangle]
pub unsafe extern "C" fn ml_print_tuple(tuple_ptr: *const Tuple) -> isize {
    Box::with_raw_ref(tuple_ptr, |tuple| {
        println!(
            "({})",
            tuple.iter().map(isize::to_string).collect::<Vec<_>>().join(", ")
        );
        0
    })
}

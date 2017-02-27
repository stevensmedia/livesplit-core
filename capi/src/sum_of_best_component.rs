use livesplit_core::component::sum_of_best::Component as SumOfBestComponent;
use livesplit_core::Timer;
use super::{alloc, drop, acc, output_vec};
use libc::c_char;

pub type OwnedSumOfBestComponent = *mut SumOfBestComponent;

#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponent_new() -> OwnedSumOfBestComponent {
    alloc(SumOfBestComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponent_drop(this: OwnedSumOfBestComponent) {
    drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponent_state_as_json(this: *const SumOfBestComponent,
                                                          timer: *const Timer)
                                                          -> *const c_char {
    output_vec(|o| { acc(this).state(acc(timer)).write_json(o).unwrap(); })
}

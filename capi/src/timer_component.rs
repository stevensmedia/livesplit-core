use livesplit_core::component::timer::Component as TimerComponent;
use livesplit_core::Timer;
use super::{alloc, drop, acc, output_vec};
use libc::c_char;
use timer_component_state::OwnedTimerComponentState;

pub type OwnedTimerComponent = *mut TimerComponent;

#[no_mangle]
pub unsafe extern "C" fn TimerComponent_new() -> OwnedTimerComponent {
    alloc(TimerComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn TimerComponent_drop(this: OwnedTimerComponent) {
    drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn TimerComponent_state_as_json(this: *const TimerComponent,
                                                      timer: *const Timer)
                                                      -> *const c_char {
    output_vec(|o| { acc(this).state(acc(timer)).write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn TimerComponent_state(this: *const TimerComponent,
                                              timer: *const Timer)
                                              -> OwnedTimerComponentState {
    alloc(acc(this).state(acc(timer)))
}

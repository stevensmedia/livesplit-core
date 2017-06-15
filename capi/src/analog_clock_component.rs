use livesplit_core::component::analog_clock::Component as AnalogClockComponent;
use livesplit_core::Timer;
use super::{Json, alloc, own_drop, acc, output_vec};
use analog_clock_component_state::OwnedAnalogClockComponentState;

pub type OwnedAnalogClockComponent = *mut AnalogClockComponent;

#[no_mangle]
pub unsafe extern "C" fn AnalogClockComponent_new() -> OwnedAnalogClockComponent {
    alloc(AnalogClockComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn AnalogClockComponent_drop(this: OwnedAnalogClockComponent) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn AnalogClockComponent_state_as_json(
    this: *const AnalogClockComponent,
    timer: *const Timer,
) -> Json {
    output_vec(|o| { acc(this).state(acc(timer)).write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn AnalogClockComponent_state(
    this: *const AnalogClockComponent,
    timer: *const Timer,
) -> OwnedAnalogClockComponentState {
    alloc(acc(this).state(acc(timer)))
}

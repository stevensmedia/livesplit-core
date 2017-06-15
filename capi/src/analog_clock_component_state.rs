use livesplit_core::component::analog_clock::State as AnalogClockComponentState;
use super::{own_drop, acc};

pub type OwnedAnalogClockComponentState = *mut AnalogClockComponentState;

#[no_mangle]
pub unsafe extern "C" fn AnalogClockComponentState_drop(this: OwnedAnalogClockComponentState) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn AnalogClockComponentState_seconds_x(
    this: *const AnalogClockComponentState,
) -> f32 {
    acc(this).seconds.0
}

#[no_mangle]
pub unsafe extern "C" fn AnalogClockComponentState_seconds_y(
    this: *const AnalogClockComponentState,
) -> f32 {
    acc(this).seconds.1
}

#[no_mangle]
pub unsafe extern "C" fn AnalogClockComponentState_minutes_x(
    this: *const AnalogClockComponentState,
) -> f32 {
    acc(this).minutes.0
}

#[no_mangle]
pub unsafe extern "C" fn AnalogClockComponentState_minutes_y(
    this: *const AnalogClockComponentState,
) -> f32 {
    acc(this).minutes.1
}

#[no_mangle]
pub unsafe extern "C" fn AnalogClockComponentState_hours_x(
    this: *const AnalogClockComponentState,
) -> f32 {
    acc(this).hours.0
}

#[no_mangle]
pub unsafe extern "C" fn AnalogClockComponentState_hours_y(
    this: *const AnalogClockComponentState,
) -> f32 {
    acc(this).hours.1
}

use livesplit_core::Time;
use super::acc;

pub type SegmentHistoryElement = (&'static i32, &'static Time);
pub type OwnedSegmentHistoryElement = *mut SegmentHistoryElement;

#[no_mangle]
pub unsafe extern "C" fn SegmentHistoryElement_index(this: *const SegmentHistoryElement) -> i32 {
    *acc(this).0
}

#[no_mangle]
pub unsafe extern "C" fn SegmentHistoryElement_time(this: *const SegmentHistoryElement)
                                                    -> *const Time {
    acc(this).1
}

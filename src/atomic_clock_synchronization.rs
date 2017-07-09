use parking_lot::RwLock;

lazy_static! {
    static ref DRIFT: RwLock<f64> = RwLock::new(2.0);
}

pub fn drift() -> f64 {
    *DRIFT.read()
}

use crate::drv::io::mm::hpet::HPET as HPETTimer;
use core::time::Duration;
use spin::Mutex;

static HPET: Mutex<Option<HPETTimer>> = Mutex::new(None);

pub unsafe fn sleep_init() {
    *HPET.lock() = Some(HPETTimer::new());
}

pub unsafe fn sleep(dur: Duration) {
    if let Some(hpet) = HPET.lock().as_ref() {
        hpet.sleep(dur);
    }
}

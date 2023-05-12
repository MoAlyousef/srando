use std::sync::atomic::AtomicBool;

static SEEDED: AtomicBool = AtomicBool::new(false);

mod detail {
extern "C" {
    pub fn time( arg: *mut u32) -> u32;
    pub fn srand(seed: u32);
    pub fn rand() -> i32;
}
}

pub fn rand() -> i32 {
    unsafe {
        if !SEEDED.load(std::sync::atomic::Ordering::Relaxed) {
            detail::srand(std::mem::transmute(detail::time(std::ptr::null_mut())));
            SEEDED.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        detail::rand()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        dbg!(rand());
        dbg!(rand());
    }
}

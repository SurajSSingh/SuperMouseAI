/// A global state representing whether we can send with Sentry
pub static SEND_TO_SENTRY: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(cfg!(debug_assertions));

pub const ORDERING: std::sync::atomic::Ordering = std::sync::atomic::Ordering::SeqCst;

pub fn will_send_to_sentry() -> bool {
    SEND_TO_SENTRY.load(ORDERING)
}

pub fn change_send_to_sentry(val: bool) {
    SEND_TO_SENTRY.store(val, ORDERING);
}

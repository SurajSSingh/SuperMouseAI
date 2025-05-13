/// A global state representing whether we can send to Sentry
///
/// ### Note
///
/// Defaults to true when debug mode (`debug_assertions`) is true. On release, it defaults to false.
pub static SEND_TO_SENTRY: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(cfg!(debug_assertions));

/// Default Sequential Ordering
pub const ORDERING: std::sync::atomic::Ordering = std::sync::atomic::Ordering::SeqCst;

/// Get value of [`SEND_TO_SENTRY`]
pub fn will_send_to_sentry() -> bool {
    SEND_TO_SENTRY.load(ORDERING)
}

/// Set the [`SEND_TO_SENTRY`] static variables
pub fn change_send_to_sentry(val: bool) {
    SEND_TO_SENTRY.store(val, ORDERING);
}

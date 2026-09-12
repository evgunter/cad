//! The panic message an assertion produced, for the rows that must
//! drive their own guards across the boundary in BOTH directions.
//!
//! Test-only, and shared rather than copied: a module whose whole
//! content is assertions needs this, and every such module needs the
//! same one.

/// The panic message `f` produced, or `None` if it passed.
///
/// The message is taken from a **panic hook**, not by downcasting the
/// unwind payload: `downcast_ref` is a second bit channel and
/// `scripts/gates/bit-identity-punning.sh` forbids it outside
/// `geom-core/src/bit_identity.rs`.
///
/// `set_hook` / `take_hook` are process-global, so this installs the
/// hook **exactly once** and switches it per thread through
/// `INTERCEPTING` instead. Taking and restoring the hook around each
/// call races: two concurrent rows interleave, one restores the
/// original hook while the other is still inside `catch_unwind`, and
/// that row's message is printed by the default hook rather than
/// stashed. Installing once also keeps an unrelated thread's panic
/// during the window from being swallowed.
pub(crate) fn caught(f: impl FnOnce() + std::panic::UnwindSafe) -> Option<String> {
    thread_local! {
        /// The last panic seen on THIS thread while it was intercepting.
        /// The hook runs on the panicking thread, so a thread-local keeps
        /// two concurrent rows' messages apart.
        static LAST_PANIC: core::cell::RefCell<Option<String>> =
            const { core::cell::RefCell::new(None) };

        /// Whether THIS thread is inside [`caught`]. The hook is
        /// process-global and every thread in the binary runs through it,
        /// so this is what makes its behaviour per-thread.
        static INTERCEPTING: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    }
    static INSTALL: std::sync::Once = std::sync::Once::new();
    INSTALL.call_once(|| {
        let default = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if INTERCEPTING.with(core::cell::Cell::get) {
                let seen = info.to_string();
                LAST_PANIC.with(|c| *c.borrow_mut() = Some(seen));
            } else {
                default(info);
            }
        }));
    });
    LAST_PANIC.with(|c| c.borrow_mut().take());
    INTERCEPTING.with(|c| c.set(true));
    let out = std::panic::catch_unwind(f);
    INTERCEPTING.with(|c| c.set(false));
    match out {
        Ok(()) => None,
        Err(_) => Some(
            LAST_PANIC
                .with(|c| c.borrow_mut().take())
                .unwrap_or_default(),
        ),
    }
}

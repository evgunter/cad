//! **One subject, one thread, and its panic message kept.**
//!
//! A row whose subject can PANIC BY DESIGN — a guard being driven from
//! the failing side, a scalar whose contract says one lane aborts —
//! cannot simply call it: an unwind leaves whatever the subject
//! installed still installed on that thread, so the next case in the
//! same row runs against a dirty thread or refuses to start at all.
//! Running each case on a thread of its own is the fix, and it is the
//! same four lines wherever it is needed.
//!
//! **The message is kept**, through [`crate::panic_capture`]'s hook:
//! the payload is not downcast (a second bit channel
//! `scripts/gates/bit-identity-punning.sh` forbids outside
//! `geom-core/src/bit_identity.rs`), and an `Err` that says only "it
//! panicked" turns the row's diagnosis into a second debugging session.

/// Runs `f` on a thread of its own and returns what it produced, or the
/// message it panicked with.
///
/// # Errors
///
/// The panic message when `f` panicked, and a message of this module's
/// own when the thread could not be joined at all — the two are
/// distinguishable by the text, and a caller that only asks "did it
/// panic" reads the `Result`'s shape.
pub fn caught<R: Send + 'static>(f: impl FnOnce() -> R + Send + 'static) -> Result<R, String> {
    let joined = std::thread::spawn(move || {
        let mut out: Option<R> = None;
        let message = crate::panic_capture::caught(std::panic::AssertUnwindSafe(|| {
            out = Some(f());
        }));
        (message, out)
    })
    .join();
    match joined {
        Ok((None, Some(r))) => Ok(r),
        Ok((Some(message), _)) => Err(message),
        // Neither returned nor panicked: the only way out is a thread
        // that was killed under us, and saying so beats an `unwrap`
        // whose message is `None`.
        Ok((None, None)) => Err("the subject neither returned nor panicked".to_owned()),
        Err(_) => Err("the thread carrying the subject could not be joined".to_owned()),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::caught;

    #[test]
    fn a_value_comes_back_and_a_panic_comes_back_as_its_own_message() {
        assert_eq!(caught(|| 7_u32 + 1), Ok(8));
        let message = caught(|| panic!("the subject said this")).expect_err("it panicked");
        assert!(
            message.contains("the subject said this"),
            "the panic's OWN message is what a row diagnoses from, and this one is {message:?}"
        );
    }

    /// The thread is what makes a second case runnable after the first
    /// panicked: a thread-local the subject dirties is not this
    /// thread's.
    #[test]
    fn a_case_that_panicked_leaves_the_calling_thread_clean() {
        thread_local! {
            static INSTALLED: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
        }
        let _ = caught(|| {
            INSTALLED.with(|c| c.set(true));
            panic!("leaves it installed on ITS thread");
        });
        assert!(
            !INSTALLED.with(core::cell::Cell::get),
            "the subject's thread-local reached the caller's thread, which is the whole \
             thing this helper exists to prevent"
        );
    }
}

//! The viewer binary.
//!
//! Entry-point acts only: commit the run's ε, read the one optional
//! argument, hand both to the application. Everything below receives
//! the witness as a parameter (`scripts/gates/witness-not-ambient.sh`),
//! and the façade door is where a program inside this workspace mints
//! one.
//!
//! `viewer [document.pncad]` — the optional path is opened through the
//! same typed `Open` operation the dialog feeds (a CLI argument is a
//! way of choosing the `Path`, exactly as the dialog is), which is
//! also the only way to open a document where no desktop portal or
//! `zenity` exists for the dialog to fall back to.
//!
//! # The browser arm
//!
//! On wasm this same target is what `wasm-bindgen` post-processes:
//! `main` runs from the module's start section, so the page needs no
//! exported function to call — `local-scripts/serve-wasm.sh` builds it
//! and `crates/viewer/web/index.html` loads it. There are no arguments
//! to read (a URL is not an `argv`) and no terminal to fail into,
//! which is what the two arms below actually differ about: the native
//! arm RETURNS its error to a shell that will print it, and the browser
//! arm has to put it somewhere a person holding a phone can see.

#[cfg(not(target_family = "wasm"))]
fn main() -> eframe::Result<()> {
    viewer::app::run(
        pncad::tolerance::witness(),
        std::env::args().nth(1).map(std::path::PathBuf::from),
    )
}

/// The id the shell's `<canvas>` carries. Named here and in
/// `crates/viewer/web/index.html`, and nowhere else; those two lines
/// are one agreement.
#[cfg(target_family = "wasm")]
const CANVAS_ID: &str = "viewer_canvas";

/// The shell's status overlay — the loading message, and the only
/// place a startup refusal can be read on a phone.
#[cfg(target_family = "wasm")]
const STATUS_ID: &str = "status";

/// The overlay's detail paragraph, where the refusal's own sentence
/// goes.
#[cfg(target_family = "wasm")]
const STATUS_DETAIL_ID: &str = "status-detail";

/// The overlay's heading, which has to stop saying "loading" once
/// loading is what failed.
#[cfg(target_family = "wasm")]
const STATUS_TITLE_ID: &str = "status-title";

#[cfg(target_family = "wasm")]
fn main() {
    // `WebRunner::start` is async and `main` is not. Blocking here is
    // not an option a browser offers — the main thread is the one
    // thread the page has — so the future is handed to the event loop
    // and `main` returns immediately.
    wasm_bindgen_futures::spawn_local(async {
        if let Err(error) = viewer::app::run_web(pncad::tolerance::witness(), CANVAS_ID).await {
            // Both doors, deliberately. The console is where a
            // desktop browser's devtools look; the page is the only
            // one a phone has, and a phone is this arm's whole reason
            // for existing.
            let message = format!("viewer failed to start: {error}");
            eframe::web_sys::console::error_1(&message.as_str().into());
            // The page gets the refusal's own sentence, not the
            // prefixed line: the overlay's heading supplies the "failed
            // to start" half, and repeating it there would spend a
            // phone's narrow column saying the same thing twice.
            report_to_page(&error.to_string());
        }
    });
}

/// Put `message` into the page's status overlay, replacing whatever
/// the shell was showing while the module loaded.
///
/// **This drives the shell's own error presentation rather than
/// inventing one**: `crates/viewer/web/index.html` styles `.error` and
/// hides the overlay with the `hidden` attribute, so the two acts here
/// — set the class, clear `hidden` — are exactly what that page's own
/// `fail()` does from JS. The ids above are that agreement; changing
/// one without the other leaves a phone staring at "loading viewer…"
/// forever, which is the failure this whole path exists to prevent.
///
/// Best-effort ON PURPOSE, and the one place in this crate where a
/// failure is swallowed rather than reported: this runs only when
/// startup has ALREADY failed and is the last thing that will ever
/// run. Refusing loudly about a missing element would replace a
/// message a person can act on with no message at all — and the
/// console line at the call site is the belt to this brace.
#[cfg(target_family = "wasm")]
fn report_to_page(message: &str) {
    let Some(document) = eframe::web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    if let Some(detail) = document.get_element_by_id(STATUS_DETAIL_ID) {
        detail.set_text_content(Some(message));
    }
    if let Some(title) = document.get_element_by_id(STATUS_TITLE_ID) {
        title.set_text_content(Some("viewer failed to start"));
    }
    if let Some(status) = document.get_element_by_id(STATUS_ID) {
        status.set_class_name("error");
        // The shell hides the overlay once the canvas paints, so a
        // refusal arriving after that has to un-hide it or the message
        // lands invisibly.
        status.remove_attribute("hidden").ok();
    }
}

//! **What the chrome calls things.**
//!
//! Two of the names a user reads are pure functions of state rather
//! than pixels, so they are pinned here: the toolbar's name for the
//! open document, and the initial layout's shape. The rest of the
//! chrome's wording lives inside widget calls and is not testable
//! without a window; this suite claims only what it can see.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

/// **Loud skip.** Without `--features app` this suite is empty:
/// `viewer::app` does not exist in the build, so neither does anything
/// it names. Announce that rather than letting a run report the whole
/// suite as absent — a green lane over rows that were never compiled
/// says the same thing as a green lane over rows that passed.
///
/// The rows below gate under `cargo nextest run -p viewer --features
/// app` (`.github/workflows/ci.yml`) and nowhere else; the workspace
/// archive builds this crate at default features and so carries this
/// marker instead.
///
/// **This row closes no gate and cannot fail** — its payload is its
/// NAME in the PASS list, nothing more. It does not go red if the
/// rows it stands in for change, so the list it recites is kept by
/// hand and a stale one would read exactly like a current one.
#[cfg(not(feature = "app"))]
#[test]
fn app_lane_skipped_no_chrome_coverage_here() {
    println!(
        "SKIPPED (no --features app): chrome_labels.rs contributes NO chrome \
         coverage in this run - the toolbar's document name and the initial \
         layout's shape are pinned only where the `app` feature is built."
    );
}

#[cfg(feature = "app")]
mod chrome {
    use std::path::Path;

    use viewer::app::{Pane, document_name, initial_layout, model_stack};

    #[test]
    fn the_toolbar_names_the_open_document_not_the_program() {
        assert_eq!(
            document_name(Some(Path::new("/tmp/gallery/diefillet.pncad"))),
            "diefillet",
            "the file stem, without the directory or the extension"
        );
        assert_eq!(
            document_name(None),
            "untitled",
            "a document with no path of its own still has a name to show"
        );
        // A path that is all directory has no stem; the label must still
        // say something rather than come up empty.
        assert_eq!(document_name(Some(Path::new("/"))), "untitled");
    }

    #[test]
    fn the_starting_layout_stacks_features_over_properties() {
        let tree = initial_layout();
        for pane in [Pane::Viewport, Pane::Features, Pane::Properties, Pane::View] {
            assert!(tree.tiles.find_pane(&pane).is_some(), "{pane:?} has a tile");
        }
        // Features and Properties share one container — the tile the
        // `Model` tab title and the content-driven split both key off.
        assert!(
            model_stack(&tree.tiles).is_some(),
            "the starting layout stacks the two panes the user reads together"
        );
    }
}

//! **What the chrome calls things.**
//!
//! Two of the names a user reads are pure functions of state rather
//! than pixels, so they are pinned here: the toolbar's name for the
//! open document, and the initial layout's shape. The rest of the
//! chrome's wording lives inside widget calls and is not testable
//! without a window; this suite claims only what it can see.

#![cfg(feature = "app")]
// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

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

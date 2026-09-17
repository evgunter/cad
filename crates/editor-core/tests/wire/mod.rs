//! **Wire surgery** — a saved document, corrupted at one path of its
//! wire form, for the suites that measure what the LOAD door does with
//! a file the edit doors could not have produced.
//!
//! Its own helper tree rather than a block in [`crate::fixture`],
//! because `tests/fixture/` is SYMLINKED into `crates/viewer/tests/`
//! and is compiled as part of viewer's test binary: viewer's
//! dev-dependencies deliberately carry no serde edge (the comment on
//! them says so), and this helper's whole subject is
//! `serde_json::Value`. Nothing in `crates/viewer/tests/` mounts this
//! module, so the edge stays where it belongs.
#![allow(dead_code)] // one instance per binary; not every consumer uses all of it
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

/// **A saved document, corrupted at one path of its wire form.**
///
/// The load-door suites need files the edit doors could not have
/// produced, and the only honest way to get one is to save a good
/// document and edit the TEXT: the header passes through untouched,
/// the JSON body is parsed, `edit` moves what the row is about, and
/// the result is re-rendered.
///
/// **BY PATH, not by byte.** A byte substitution proves only that a
/// byte moved; going through the parsed value proves the INTENDED
/// field moved, because the path names it and the caller's `edit`
/// asserts what it found there — so a fixture or field-order change
/// breaks the surgery loudly instead of silently landing it on a
/// neighbour. The `assert_ne!` below is the same guard one level up: a
/// surgery whose path missed writes the file back unchanged, and a row
/// over it would pass while measuring nothing.
///
/// One body, read by every suite that doctors a save; the five
/// byte-identical copies it replaces are the reason it lives here.
pub fn doctored(text: &str, edit: impl FnOnce(&mut serde_json::Value)) -> String {
    let split = text.find('{').expect("the JSON body follows the id header");
    let (header, body) = text.split_at(split);
    let mut wire: serde_json::Value = serde_json::from_str(body).expect("the body parses");
    edit(&mut wire);
    let out = format!("{header}{wire}");
    assert_ne!(out, text, "the corruption really landed");
    out
}

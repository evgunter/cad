//! The demo-document gallery: every document-authored scene, saved as
//! a `.pncad` a person can open in the viewer.
//!
//! # What this mode is for
//!
//! The GUI's acceptance shape is "open the application and load one of
//! the existing demo documents through a file dialog". That needs the
//! documents to exist as files. This mode writes them, from the SAME
//! authoring functions the tour renders — never a second authoring,
//! which would make the gallery evidence about itself rather than
//! about the scenes (`memos`: `memories/demo-purpose.md`).
//!
//! # Which scenes are here, and which are not
//!
//! The document-authored scenes are the ones that build a `Doc` and
//! evaluate it: **checks, ring, diefillet, heatsink**, plus
//! **assembly**, whose documents are a workspace of several files and
//! are written by that scene's own store. The rest of the tour drives
//! the kernel API directly and has no document to save; they join the
//! gallery as they are re-authored, which is per-scene library work
//! and independent of the GUI.
//!
//! Everything here goes through the public doors: author, then
//! `pncad::document::save`, whose own validation is what decides
//! whether a scene's document is writable at all.

use std::path::{Path, PathBuf};

use pncad::document::{ProfileDoc, save};
use pncad::geom_core::Tol;

/// Write the gallery into `dir` (default `gallery/`).
pub fn run(dir: Option<String>, tol: Tol) {
    let dir = PathBuf::from(dir.unwrap_or_else(|| "gallery".to_owned()));
    std::fs::create_dir_all(&dir).expect("create the gallery directory");

    println!("demo-document gallery → {}", dir.display());
    let mut written = 0usize;
    for (name, doc) in [
        ("checks", crate::checks::gallery_document(tol)),
        ("ring", crate::ring::gallery_document(tol)),
        ("diefillet", crate::diefillet::gallery_document(tol)),
        ("heatsink", crate::heatsink::gallery_document(tol)),
    ] {
        write_one(&dir, name, &doc, tol);
        written += 1;
    }

    // The assembly scene's documents are a WORKSPACE — several files
    // that reference one another by content pin — so they are written
    // by that scene's own store rather than by the loop above. Running
    // its stops is what populates it, exactly as the ordinary tour run
    // does.
    let work = dir.join("assembly");
    let stops = crate::assembly::stops(&work, tol);
    let assembly_files = std::fs::read_dir(&work)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|e| e.path().extension().is_some_and(|x| x == "pncad"))
                .count()
        })
        .unwrap_or(0);
    println!(
        "   assembly/: {assembly_files} document(s) from {} stop(s)",
        stops.len()
    );
    println!(
        "gallery complete: {} standalone document(s) + {assembly_files} assembly document(s)",
        written
    );
    println!("   open one with: cargo run -p viewer --features app  (Open… in the toolbar)");
}

/// Save one document, loudly.
///
/// `save` validates before it writes — a document it refuses is a
/// finding about that scene, not something to skip past, so this
/// panics rather than logging and continuing.
fn write_one(dir: &Path, name: &str, doc: &ProfileDoc, tol: Tol) {
    // An empty edit log: the gallery ships the STATE, and a viewer
    // that opens it starts its own history from this snapshot.
    let text = save(doc, &[], tol)
        .unwrap_or_else(|error| panic!("the {name} document does not save: {error:?}"));
    let path = dir.join(format!("{name}.pncad"));
    std::fs::write(&path, &text)
        .unwrap_or_else(|error| panic!("cannot write {}: {error}", path.display()));
    println!(
        "   {name}.pncad — {} node(s), {} byte(s)",
        doc.len(),
        text.len()
    );
}

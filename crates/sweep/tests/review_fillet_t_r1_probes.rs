//! FILLET-T review probes (lane r1, PR 1943), widened at FILLET-SPLIT
//! (PR 1964) to the directory the carve now spans.
//!
//! **The one door.** `blend/surgery.rs` states that no site calls
//! `Body::kef` directly — every excision goes through `kef_minted`,
//! which refuses a half whose face is one of the running phase's source
//! faces. Nothing else holds that: a site rewritten to call `body.kef`
//! straight (the review's bypass mutant, planted at the edge-strip
//! site) leaves every `m6_*` row green and the bit dump identical,
//! because on a green carve the door is reached and never fires. This
//! row is the mechanical form of the header's sentence, taken over
//! EVERY `.rs` file under `crates/sweep/src/blend/` by walking the
//! directory ([`rust_sources`]) rather than reading a hand list: a
//! carve file that arrives later is inside the census the day it lands
//! (a planted fifth file under `blend/open/` carrying `body.kef(`
//! stayed green against the four-file list this replaced — measured by
//! both FILLET-SPLIT reviewers). Exactly one `.kef(` in the code of the
//! whole directory (comments and literals blanked), and it sits inside
//! `kef_minted`'s own body in `surgery.rs`. The second assertion pins
//! the site census PR 1943 states (eight `kef_minted` calls, summed over
//! the directory), so a ninth site is a deliberate re-count rather than
//! a silent addition.
//!
//! What this row does NOT pin — visibility across the `open/` boundary
//! — is `review_fillet_split_r2_probes`'s.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use std::path::PathBuf;

use test_utils::source::{ItemBody, code_only, crate_dir, rust_sources};

/// Every `.rs` under `src/blend/`, as `(path relative to `src/`, code-only text)`.
fn blend_sources() -> Vec<(String, String)> {
    let src = crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut out: Vec<(String, String)> = rust_sources(&src.join("blend"))
        .into_iter()
        .map(|p: PathBuf| {
            let rel = p
                .strip_prefix(&src)
                .expect("a blend source lies under src/")
                .to_string_lossy()
                .replace('\\', "/");
            let text = std::fs::read_to_string(&p)
                .unwrap_or_else(|e| panic!("readable {}: {e}", p.display()));
            (rel, code_only(&text))
        })
        .collect();
    out.sort();
    out
}

#[test]
fn every_kef_in_the_blend_surgery_goes_through_the_door() {
    let codes = blend_sources();
    assert!(
        codes.iter().any(|(rel, _)| rel.starts_with("blend/open/")),
        "the walk of src/blend/ found no file under blend/open/: {:?}",
        codes.iter().map(|(r, _)| r).collect::<Vec<_>>()
    );
    let (door_file, surgery) = codes
        .iter()
        .find(|(rel, _)| rel == "blend/surgery.rs")
        .expect("blend/surgery.rs is the seam and declares the door");
    let head = surgery
        .find("fn kef_minted")
        .expect("the door `kef_minted` is declared in blend/surgery.rs");
    let door = match test_utils::source::item_body(surgery, head) {
        ItemBody::Body(range) => range,
        other => panic!("`kef_minted` has no balanced body: {other:?}"),
    };
    let kefs: Vec<(&str, usize)> = codes
        .iter()
        .flat_map(|(rel, code)| {
            code.match_indices(".kef(")
                .map(move |(i, _)| (rel.as_str(), i))
        })
        .collect();
    assert_eq!(
        kefs.len(),
        1,
        "the files under blend/ call `Body::kef` at {} sites in code ({kefs:?}); the surgery \
         has one door, `kef_minted`, and every excision goes through it",
        kefs.len()
    );
    assert_eq!(
        kefs[0].0, *door_file,
        "the one `.kef(` call is not in the file that declares the door"
    );
    assert!(
        door.contains(&kefs[0].1),
        "the one `.kef(` call in blend/surgery.rs is outside `kef_minted`'s body \
         (byte {} vs door {:?})",
        kefs[0].1,
        door
    );
    let per_file: Vec<(&str, usize)> = codes
        .iter()
        .map(|(rel, code)| (rel.as_str(), code.matches("kef_minted(").count()))
        .filter(|(_, n)| *n > 0)
        .collect();
    let sites: usize = per_file.iter().map(|(_, n)| n).sum();
    assert_eq!(
        sites, 8,
        "the `kef` site census: PR 1943 states eight `kef_minted` calls (edge-strip, corner-strut, \
         rim, rim strut, annulus rim, annulus seam-crossing, ruled crease, cap sliver), summed \
         over every file under blend/ as {per_file:?}; a change in the count is a change to \
         re-take the census for, not to absorb"
    );
}

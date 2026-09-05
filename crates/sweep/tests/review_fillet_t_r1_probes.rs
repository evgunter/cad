//! FILLET-T review probes (lane r1, PR 1943).
//!
//! **The one door.** `blend/surgery.rs` states that no site calls
//! `Body::kef` directly — every excision goes through `kef_minted`,
//! which refuses a half whose face is one of the running phase's source
//! faces. Nothing else holds that: a site rewritten to call `body.kef`
//! straight (the review's bypass mutant, planted at the edge-strip
//! site) leaves every `m6_*` row green and the bit dump identical,
//! because on a green carve the door is reached and never fires. This
//! row is the mechanical form of the header's sentence, over every
//! file the carve spans — `surgery.rs` and the open bands under
//! `blend/open/`: exactly one `.kef(` in their CODE (comments and
//! literals blanked), and it sits inside `kef_minted`'s own body in
//! `surgery.rs`. The second assertion pins the site census the PR
//! states (eight, summed over those files), so a ninth site is a
//! deliberate re-count rather than a silent addition.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use test_utils::source::{ItemBody, code_only};

/// Every file the carve spans, `surgery.rs` first: the door is declared
/// there, and the census below is summed over all of them.
const CARVE_FILES: [(&str, &str); 4] = [
    ("blend/surgery.rs", include_str!("../src/blend/surgery.rs")),
    (
        "blend/open/mod.rs",
        include_str!("../src/blend/open/mod.rs"),
    ),
    (
        "blend/open/planar.rs",
        include_str!("../src/blend/open/planar.rs"),
    ),
    (
        "blend/open/ruled.rs",
        include_str!("../src/blend/open/ruled.rs"),
    ),
];

#[test]
fn every_kef_in_the_blend_surgery_goes_through_the_door() {
    let codes: Vec<(&str, String)> = CARVE_FILES
        .iter()
        .map(|(name, text)| (*name, code_only(text)))
        .collect();
    let (door_file, surgery) = &codes[0];
    let head = surgery
        .find("fn kef_minted")
        .expect("the door `kef_minted` is declared in blend/surgery.rs");
    let door = match test_utils::source::item_body(surgery, head) {
        ItemBody::Body(range) => range,
        other => panic!("`kef_minted` has no balanced body: {other:?}"),
    };
    let kefs: Vec<(&str, usize)> = codes
        .iter()
        .flat_map(|(name, code)| code.match_indices(".kef(").map(move |(i, _)| (*name, i)))
        .collect();
    assert_eq!(
        kefs.len(),
        1,
        "the carve's files call `Body::kef` at {} sites in code ({kefs:?}); the surgery has \
         one door, `kef_minted`, and every excision goes through it",
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
        .map(|(name, code)| (*name, code.matches("kef_minted(").count()))
        .collect();
    let sites: usize = per_file.iter().map(|(_, n)| n).sum();
    assert_eq!(
        sites, 8,
        "the `kef` site census: PR 1943 states eight `kef_minted` calls (edge-strip, corner-strut, \
         rim, rim strut, annulus rim, annulus seam-crossing, ruled crease, cap sliver), summed \
         over the carve's files as {per_file:?}; a change in the count is a change to re-take \
         the census for, not to absorb"
    );
}

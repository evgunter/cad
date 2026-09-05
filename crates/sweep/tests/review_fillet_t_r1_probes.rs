//! FILLET-T review probes (lane r1, PR 1943).
//!
//! **The one door.** `blend/surgery.rs` states that no site calls
//! `Body::kef` directly — every excision goes through `kef_minted`,
//! which refuses a half whose face is one of the running phase's source
//! faces. Nothing else holds that: a site rewritten to call `body.kef`
//! straight (the review's bypass mutant, planted at the edge-strip
//! site) leaves every `m6_*` row green and the bit dump identical,
//! because on a green carve the door is reached and never fires. This
//! row is the mechanical form of the header's sentence: exactly one
//! `.kef(` in the file's CODE (comments and literals blanked), and it
//! sits inside `kef_minted`'s own body. The second assertion pins the
//! site census the PR states (eight), so a ninth site is a deliberate
//! re-count rather than a silent addition.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use test_utils::source::{ItemBody, code_only};

const SURGERY: &str = include_str!("../src/blend/surgery.rs");

#[test]
fn every_kef_in_the_blend_surgery_goes_through_the_door() {
    let code = code_only(SURGERY);
    let head = code
        .find("fn kef_minted")
        .expect("the door `kef_minted` is declared in blend/surgery.rs");
    let door = match test_utils::source::item_body(&code, head) {
        ItemBody::Body(range) => range,
        other => panic!("`kef_minted` has no balanced body: {other:?}"),
    };
    let kefs: Vec<usize> = code.match_indices(".kef(").map(|(i, _)| i).collect();
    assert_eq!(
        kefs.len(),
        1,
        "blend/surgery.rs calls `Body::kef` at {} sites in code; the surgery has one door, \
         `kef_minted`, and every excision goes through it",
        kefs.len()
    );
    assert!(
        door.contains(&kefs[0]),
        "the one `.kef(` call in blend/surgery.rs is outside `kef_minted`'s body \
         (byte {} vs door {:?})",
        kefs[0],
        door
    );
    let sites = code.matches("kef_minted(").count();
    assert_eq!(
        sites, 8,
        "the `kef` site census: PR 1943 states eight `kef_minted` calls (edge-strip, corner-strut, \
         rim, rim strut, annulus rim, annulus seam-crossing, ruled crease, cap sliver); a change \
         in the count is a change to re-take the census for, not to absorb"
    );
}

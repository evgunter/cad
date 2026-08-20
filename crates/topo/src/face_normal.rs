//! The **one door** for a planar face's outward normal: the single
//! place the `sense` bit is folded into a stored chart normal.
//!
//! # Why one door, and why here
//!
//! Since S10 a face's outward normal is its surface's chart normal
//! times [`Face::sense_sign`](crate::entity::Face::sense_sign) — the
//! chart is the only place orientation was ever encoded, so on a
//! `sense: false` face the stored normal points INTO the material and
//! every consumer reading a material direction off it answers
//! backwards. One flip, in one function, is what keeps two flips from
//! drifting apart.
//!
//! The door used to live in `boolean::reduce`, whose invariant named
//! its five consumers — the boolean's `plane_of`, the reduction sweep,
//! the pierce lane, the REST lane, and `sector_face`. When
//! `sector_face` became [`crate::sector_face`], shared with the
//! splitting lane, that consumer moved OUT of `boolean/` and the door
//! could no longer be the one door from inside it: a crate-root module
//! importing from `boolean/` would be the same wrong-way edge, pointed
//! the other way. So the door moved to the crate root and
//! `boolean::reduce` re-exports it — the boolean's four remaining
//! consumers are unchanged, and the shared walk uses the same flip
//! rather than a second one. **Consumers here are lanes, not call
//! sites**: three of the four reach the door through
//! `boolean::reduce::face_plane` (`recl`'s `plane_of`, the reduction
//! sweep, the REST lane) and only the pierce lane (`vtxfac`) calls it
//! directly, so counting `face_outward_normal` call sites gives two
//! and does not falsify the four.
//!
//! **"One door" is true of these consumers, not of the workspace.**
//! `boolean::solid_contain::face_plane`, `chord_join`'s
//! `face_plane_normal` and `merge_faces.rs` each still carry their own
//! hand-multiply (smell-scan D6). This module is where they belong
//! when that is executed; naming them here rather than leaving the
//! claim unqualified is the point.
//!
//! **A better home exists and is not reachable from this lane.** The
//! function is a `Body` query, exactly like `Body::mate` or
//! `Body::loop_cycle`, and belongs as an inherent method on
//! [`Body`]. That is `body.rs`, outside the scope
//! this module was created under; it is recorded on issue #695 with
//! the rest of the placement questions.

use geom_brep::OutwardNormal;
use geom_core::Decide;

use crate::body::Body;
use crate::entity::FaceKey;

/// A planar face's OUTWARD normal — the chart normal with the face's
/// `sense` folded in, minted as an [`OutwardNormal`] so no consumer
/// can multiply again.
///
/// INVARIANT: this is where the planar lane's sense flip lives, and
/// the module docs list who goes through it. `None` for a non-planar
/// face (the caller falls through to its own curved arms, or has
/// none).
pub(crate) fn face_outward_normal<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Option<OutwardNormal<T>> {
    let f = body.get_face(face)?;
    match body.get_surface(f.surface) {
        Some(geom::Surface::Plane { normal, .. }) => {
            Some(OutwardNormal::from_chart(*normal, f.sense))
        }
        _ => None,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    /// **The anti-re-fork row for the planar sense flip.** The plane
    /// arm of this walk goes through [`crate::face_normal`], the one
    /// door, and no file under `topo/src` other than that one may both
    /// destructure a plane surface pattern for its normal and mint an
    /// [`OutwardNormal`] from a chart — which is what a second flip
    /// looks like textually. (The pattern is spelled ONLY in the check
    /// itself: writing it in this comment made the guard's home its own
    /// first counter-example, which is how this row first went red.)
    ///
    /// This row exists because the fix that created this module
    /// FALSIFIED that invariant once already: routing the boolean's
    /// `sector_face` through the shared body dropped its call to the
    /// door and re-derived `from_chart(*normal, sense)` here, which is
    /// exactly the drift `boolean::reduce`'s invariant forbids and
    /// nothing in the tree could see.
    ///
    /// **What it cannot match** — four shapes, said plainly:
    ///
    /// 1. **A flip written without `from_chart`** — `normal *
    ///    f.sense_sign()` reaching a material verdict. Three such sites
    ///    exist and are NAMED (smell-scan D6:
    ///    `solid_contain::face_plane`, `chord_join::face_plane_normal`,
    ///    `merge_faces.rs`); this row does not see them and is not a
    ///    claim that they are gone.
    /// 2. **A flip in another crate.** The walk is scoped to
    ///    `topo/src`.
    /// 3. **A caller that takes the door's answer and negates it.**
    ///    [`OutwardNormal`] is a wrapper, not a capability: `.vec()` is
    ///    public and a consumer can multiply what comes back.
    /// 4. **A file that reads the plane normal through a helper** —
    ///    `face_plane(..).normal` — and flips that. No plane-surface
    ///    pattern appears, so the textual pair never matches.
    #[test]
    fn the_planar_sense_flip_lives_in_one_place() {
        let home = crate::fixtures::src_root().join("face_normal.rs");
        let files = crate::fixtures::crate_sources();
        assert!(
            files.contains(&home),
            "the walk did not find face_normal.rs"
        );
        for path in &files {
            if path == &home {
                continue;
            }
            let text = std::fs::read_to_string(path).expect("a readable source file");
            assert!(
                !(text.contains("Surface::Plane {") && text.contains("from_chart")),
                "{} mints an OutwardNormal from a plane's chart normal — the planar \
                 sense flip has been re-forked out of face_normal.rs (smell scan S5 / \
                 S10). Call `face_outward_normal` instead.",
                path.display()
            );
        }
        let here = std::fs::read_to_string(&home).expect("the home module is readable");
        assert!(
            here.contains("Surface::Plane {") && here.contains("from_chart"),
            "the door no longer mints the flip, so this row guards nothing"
        );
    }
}

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
//! Other outward normals are still built by hand-multiplying a chart
//! normal by [`Face::sense_sign`](crate::entity::Face::sense_sign),
//! and this module is where they belong when smell-scan D6 is
//! executed. **Where they are is computed, not recited**: the guard
//! test below walks `topo/src`, counts every occurrence of that method
//! in code, and fails on one its dispositioned table does not carry.
//! The count is of the whole tree this crate can see; the rest of the
//! workspace is D6's scope and is not enumerated here.
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
    /// 1. **A flip written without `from_chart`** — a chart normal
    ///    multiplied by the face's own `±1` and reaching a material
    ///    verdict. This row does not see those; the inventory row
    ///    below does, and names each one.
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
        let home = crate::source_walk::src_root().join("face_normal.rs");
        let files = crate::source_walk::crate_sources();
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

    /// **The hand-multiply inventory** — smell-scan D6's `grep
    /// sense_sign` written as a gate rather than as a sentence. Every
    /// **occurrence** of [`Face::sense_sign`](crate::entity::Face::sense_sign)
    /// in the CODE of `topo/src` (comments and literal bodies removed
    /// by [`crate::fixtures::code_only`]) is counted per file and
    /// pinned below with its disposition, so a new hand-multiply
    /// cannot land without either going through the door or being
    /// argued for in the table:
    ///
    /// - `entity.rs` — the definition itself.
    /// - `boolean/solid_contain.rs` — `face_plane` and `face_geo`, two
    ///   plane arms whose consumers are ray-parity walks and therefore
    ///   blind to the sign; they thread it to keep the doors' stated
    ///   contract (an OUTWARD normal) honest for the next consumer.
    /// - `boolean/join.rs` — `ring_run_ccw`, which picks an island's
    ///   new outer boundary and needs exactly one of its two signs
    ///   threaded.
    /// - `boolean/rest.rs` — `face_carrier`, the curved generalization
    ///   of the door; it binds the `±1` to a local before multiplying,
    ///   which is the form a textual `* ….sense_sign` sweep misses.
    /// - `merge_faces.rs` — two sense-tuple reads in the coplanarity
    ///   gate and the survivor's plane hand-multiply.
    /// - `props.rs` — the `±1` handed to `curved_face`'s closed form.
    ///   Not a normal multiply, and the only read whose consumer is a
    ///   curved carrier.
    /// - `validate.rs` — check 6's outward normal, where the sense bit
    ///   is read as a claim to be falsified rather than honored.
    /// - `face_normal.rs` — **zero**: the door takes the sense BIT
    ///   through [`OutwardNormal::from_chart`], never the `±1`. That
    ///   is why the walk below reads the method name out of `concat!`
    ///   — spelled whole, this file would be its own first hit.
    ///
    /// **The pin is per FILE, and that is wider than the invariant.**
    /// Moving a read from one file to another changes nothing about
    /// the sense flip and still reds this row; so does adding a second
    /// occurrence to a line that already has one. The narrower pin — a
    /// total plus the dispositions — would red for fewer wrong
    /// reasons, and the per-file shape is chosen because *which* file
    /// carries a read is the only thing that makes the disposition
    /// list above checkable. The cost is a tripwire over all of
    /// `topo/src` in a tree several lanes are editing at once.
    ///
    /// **What this cannot match**, and it is a work order rather than
    /// a discharge:
    ///
    /// 1. A flip written through a helper that already returns an
    ///    outward normal — `face_plane(..).normal` multiplied again.
    /// 2. A [`crate::entity::Face::sense`] bool read and branched on
    ///    without the `±1` (`step-export`'s `same_sense` bit is such a
    ///    consumer, and a legitimate one).
    /// 3. An identifier a macro assembles, which no textual walk sees.
    /// 4. **Every crate but this one.** The walk is `topo/src` because
    ///    that is the tree this crate can see, and the workspace half
    ///    is D6's. **It is deliberately not enumerated here**: a roster
    ///    of other crates is exactly the artifact this row replaced,
    ///    and reciting one beside a computed inventory would mint the
    ///    same defect one level out. The out-of-crate readers are
    ///    inventoried once, in `docs/SMELL-SCAN-2026-08.md` at S67,
    ///    beside D6's work order — a list that is recited and says so,
    ///    in the document where a work order belongs.
    #[test]
    fn every_hand_multiply_of_the_face_sign_is_inventoried() {
        const PINNED: [(&str, usize); 8] = [
            ("boolean/join.rs", 1),
            ("boolean/rest.rs", 1),
            ("boolean/solid_contain.rs", 2),
            ("entity.rs", 1),
            ("face_normal.rs", 0),
            ("merge_faces.rs", 3),
            ("props.rs", 1),
            ("validate.rs", 1),
        ];
        let needle = concat!("sense", "_sign");
        let root = crate::source_walk::src_root();
        let mut found: Vec<(String, usize)> = Vec::new();
        for path in crate::source_walk::crate_sources() {
            let text = std::fs::read_to_string(&path).expect("a readable source file");
            let reads = crate::fixtures::code_only(&text).matches(needle).count();
            let rel = path
                .strip_prefix(&root)
                .expect("a walked file lies under topo/src")
                .to_string_lossy()
                .replace('\\', "/");
            if reads > 0 || PINNED.iter().any(|(pinned, _)| *pinned == rel) {
                found.push((rel, reads));
            }
        }
        found.sort();
        let pinned: Vec<(String, usize)> = PINNED
            .iter()
            .map(|(path, reads)| ((*path).to_string(), *reads))
            .collect();
        assert_eq!(
            found, pinned,
            "the inventory of hand-multiplies moved: an occurrence of the face's own ±1 was \
             added, removed or relocated. Route it through `face_outward_normal` if it wants \
             an outward normal; otherwise add it to this table with its disposition (D6)."
        );
    }
}

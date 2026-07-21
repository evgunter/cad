//! Reclassification rules (a) and (b) — every sign derived from
//! [`geom_brep::enters_material`] (F3), nothing sign-copied; rule (b)
//! adjudicated between the two contradicting witnesses (F4) — the
//! derivation is this module's docs (M3-LOG-ready).
//!
//! # Rule (a) — coplanar sectors, derived
//!
//! A sector whose face lies in the split plane belongs to the closure
//! of both sides; it must go **with the material it bounds** (it will
//! survive as a wall of that side's solid — Fig. 14.2's "artifact"
//! faces on top of Below). Where the material is, in F3 terms: take
//! `dir = +n_SP` (the Above direction). If
//! `enters_material(+n_SP, face) = Exits` (`dot(n_SP, n_face) > 0`),
//! going Above leaves the material — the material is Below, so both
//! bounding edges of the sector reclassify **Below**; `Enters` ⇒
//! **Above**. (`Tangent` is unreachable behind the parallelism gate —
//! it escalates as an internal inconsistency.) This reproduces the
//! book's printed `dot(feq, SP) > 0 ⇒ BELOW`, which TOG §2's
//! outward-normal convention already suggested is correct *for us* —
//! but the sign above is **derived**, and the mirror test pins it both
//! ways on a coplanar-face split.
//!
//! The coplanarity gate itself is the oriented-parallelism trilean
//! `split_sector_coplanar`: margin `‖n_face × n_SP‖ · extent` (sin of
//! the normal angle metered at the face's extent from the base vertex —
//! D4 ¶1's named lever arm), never the book's raw `EPS²` dot test. The
//! face's plane *offset* needs no second test: the sector's base vertex
//! is already ON the plane, so parallel ⇒ coincident (documented
//! consciously, as the ch. 14 notes demand).
//!
//! # Rule (b) — the F4 adjudication (tangent-edge & touching-wedge)
//!
//! After rule (a), a remaining ON entry is an edge in the plane whose
//! flanking sectors are not coplanar; its cyclic neighbor entries are
//! never ON. The witnesses **contradict** on the two symmetric cases
//! (book Program 14.6: AOA→BELOW, BOB→ABOVE; TOG §3: AOA→ABOVE,
//! BOB→BELOW; both: AOB→BELOW, BOA→BELOW). Derive from the stated
//! purpose — nonmanifold configurations must come out as DISCONNECTED
//! pieces, no dangling faces/edges:
//!
//! **Tangent-edge fixture** (V-notch cut into a block's top, notch tip
//! edge exactly in SP, material below): at a tip vertex the entries are
//! cyclically `[slantL: Above, tip: ON, slantR: Above, cap-bisector:
//! Below]` (the cap face's corner is reflex — its convex-subdivision
//! duplicate classifies Below). Above's material near the tip is two
//! wedges meeting only along the tip edge; a manifold representation
//! must give each wedge its own section face meeting its slant face in
//! its OWN edge — the tip edge may not be shared, or it would carry
//! four faces (slantL, sectionL, slantR, sectionR): non-manifold,
//! unrepresentable in the half-edge structure. So the tip edge must
//! leave the Above side: **AOA → BELOW** separates the runs
//! (`{slantL}`, `{slantR}` — two null edges, two vertex copies, the
//! wedges disconnect; the tip edge survives inside Below's coplanar
//! top as an artifact edge, manifold). The paper's AOA→ABOVE merges
//! one run `{slantL, tip, slantR}` — one copy, both wedges pinned to
//! one vertex/edge: the 4-face edge. The book is right; TOG §3's list
//! is the erratum.
//!
//! **Touching-wedge fixture** (the mirror: notch cut from below,
//! material above; entries `[slantL: Below, tip: ON, slantR: Below,
//! cap-bisector: Above]`): Below's material is two wedges meeting at
//! the tip edge. **BOB → ABOVE** isolates the tip edge into its own
//! run (its null-edge pair gives it Above-side copies; the Below
//! wedges separate); BOB→BELOW (paper) leaves the wedges joined
//! through it — the same 4-face edge in Below. Book right again, by
//! the mirror of the same argument.
//!
//! The mixed cases (AOB/BOA → BELOW, both witnesses agree) are a free
//! convention — either side yields manifold results where the pieces
//! do not touch; BELOW is kept for witness agreement and consistency
//! with rule (a)'s Fig. 14.8 coplanar-edge-goes-below choice.
//!
//! **Residue, stated honestly**: a solid tangent to SP *from one side
//! only* at an edge (nothing at all on the other side — no wide
//! BELOW-bisector in the neighborhood) still produces surgery under
//! AOA→BELOW (one wrapped run) whose Below piece degenerates to the
//! bare tangent edge; PR 3's joining must detect/refuse the degenerate
//! section polygon (flagged there). The paper's table would skip
//! surgery for that case but corrupts the embedded cases above —
//! representability outranks the degenerate-piece cosmetic.

use geom_brep::{EntersMaterial, enters_material};
use geom_core::{Band, Decide, Sign};

use super::neighborhood::sector_face;
use super::{PlaneSide, SectorEntry, SplitPlane, SplitReduceError};
use crate::body::Body;
use crate::entity::{FaceKey, VertexKey};
use crate::validate::decide;

/// Rule (a): reclassify both bounding entries of every
/// plane-coplanar sector (module docs for the derivation). Sweeps
/// entries in order; a later coplanar sector's verdict overwrites an
/// earlier one's on a shared entry (only reachable through adjacent
/// coplanar faces — a maximal-faces violation; deterministic
/// last-wins, as the book).
pub(super) fn apply_rule_a<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    vertex: VertexKey,
    entries: &mut [SectorEntry],
    band: Band,
) -> Result<(), SplitReduceError> {
    let n = entries.len();
    for k in 0..n {
        let (face, n_face) = sector_face(body, vertex, entries[k].he)?;
        let sliver = |diag| SplitReduceError::SliverSector { vertex, face, diag };
        let extent = face_extent(body, vertex, face)?;
        match decide("split_sector_arm", extent, band) {
            Ok(Sign::Positive) => {}
            Ok(_) => {
                return Err(sliver(geom_core::Indeterminate {
                    margin: geom_core::MarginDiag::Invalid,
                    band,
                    predicate: Some("split_sector_arm"),
                }));
            }
            Err(diag) => return Err(sliver(diag)),
        }
        // Oriented parallelism, part 1: are the normals parallel at
        // all? Margin ‖n_face × n_SP‖·extent (≥ 0; sin θ metered at
        // the face extent).
        let parallel_margin = n_face.cross(plane.normal).norm() * extent;
        match decide("split_sector_coplanar", parallel_margin, band) {
            Ok(Sign::Zero) => {}
            Ok(_) => continue, // definitely not coplanar: rule (a) silent
            Err(diag) => return Err(sliver(diag)),
        }
        // Part 2, the material sense — the F3 primitive with
        // dir = +n_SP (module docs): Exits ⇒ material Below.
        let class = match enters_material(plane.normal, n_face, extent, band) {
            Ok(EntersMaterial::Exits) => PlaneSide::Below,
            Ok(EntersMaterial::Enters) => PlaneSide::Above,
            // Tangent after the parallelism gate is contradictory —
            // escalate rather than guess.
            Ok(EntersMaterial::Tangent) => {
                return Err(sliver(geom_core::Indeterminate {
                    margin: geom_core::MarginDiag::Invalid,
                    band,
                    predicate: Some("enters_material"),
                }));
            }
            Err(diag) => return Err(sliver(diag)),
        };
        entries[k].class = class;
        entries[(k + 1) % n].class = class;
    }
    Ok(())
}

/// Rule (b): reclassify every remaining ON entry by its cyclic
/// neighbors — the **adjudicated** table (module docs):
/// `BELOW-ON-BELOW → ABOVE`, every other context `→ BELOW`.
/// Checks the no-consecutive-ONs invariant loudly first (the book
/// assumes it; we refuse if it fails).
pub(super) fn apply_rule_b(
    vertex: VertexKey,
    entries: &mut [SectorEntry],
) -> Result<(), SplitReduceError> {
    let n = entries.len();
    for k in 0..n {
        if entries[k].class == PlaneSide::On && entries[(k + 1) % n].class == PlaneSide::On {
            return Err(SplitReduceError::ConsecutiveOnSectors { vertex });
        }
    }
    for k in 0..n {
        if entries[k].class != PlaneSide::On {
            continue;
        }
        let prev = entries[(k + n - 1) % n].class;
        let next = entries[(k + 1) % n].class;
        entries[k].class = match (prev, next) {
            (PlaneSide::Below, PlaneSide::Below) => PlaneSide::Above,
            _ => PlaneSide::Below,
        };
    }
    Ok(())
}

/// The face-extent lever arm for the coplanarity/sense predicates: the
/// farthest distance from the base vertex to any vertex of the face's
/// loops — the largest displacement a normal-angle error can induce
/// across this face (D4 ¶1's "face extent" arm, computed, named).
fn face_extent<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    face: FaceKey,
) -> Result<T, SplitReduceError> {
    let corrupt = || SplitReduceError::CorruptOperand { vertex };
    let p_base = *body
        .get_point(body.get_vertex(vertex).ok_or_else(corrupt)?.point)
        .ok_or_else(corrupt)?;
    let face_data = body.get_face(face).ok_or_else(corrupt)?;
    let mut extent = T::zero();
    let loops = core::iter::once(face_data.outer).chain(face_data.rings.iter().copied());
    for loop_key in loops {
        let loop_data = body.get_loop(loop_key).ok_or_else(corrupt)?;
        let crate::entity::LoopBoundary::Cycle { first } = loop_data.boundary else {
            continue; // an empty loop contributes no extent
        };
        for he in body.loop_cycle(first).ok_or_else(corrupt)? {
            let start = body.get_half_edge(he).ok_or_else(corrupt)?.start;
            let p = *body
                .get_point(body.get_vertex(start).ok_or_else(corrupt)?.point)
                .ok_or_else(corrupt)?;
            extent = extent.max((p - p_base).norm());
        }
    }
    Ok(extent)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::entity::HalfEdgeKey;
    use crate::splitting::SectorEntryKind;

    fn entries(classes: &[PlaneSide]) -> Vec<SectorEntry> {
        classes
            .iter()
            .map(|&class| SectorEntry {
                he: HalfEdgeKey::default(),
                kind: SectorEntryKind::Edge,
                class,
            })
            .collect()
    }

    use PlaneSide::{Above as A, Below as B, On as O};

    /// The adjudicated rule (b) table (module docs): BOB → ABOVE,
    /// every other context → BELOW — pinned per row.
    #[test]
    fn rule_b_adjudicated_table() {
        let cases = [
            ([A, O, A], A, PlaneSide::Below), // AOA → BELOW (book, not paper)
            ([B, O, B], B, PlaneSide::Above), // BOB → ABOVE (book, not paper)
            ([A, O, B], A, PlaneSide::Below), // AOB → BELOW (both witnesses)
            ([B, O, A], B, PlaneSide::Below), // BOA → BELOW (both witnesses)
        ];
        for (classes, keep, expect) in cases {
            let mut e = entries(&classes);
            apply_rule_b(VertexKey::default(), &mut e).unwrap();
            assert_eq!(e[1].class, expect, "context {classes:?}");
            assert_eq!(e[0].class, keep); // neighbors untouched
        }
    }

    /// Rule (b) reads cyclic neighbors: an ON entry at the array seam
    /// wraps.
    #[test]
    fn rule_b_wraps_cyclically() {
        let mut e = entries(&[O, B, B]); // neighbors of entry 0: e[2]=B, e[1]=B
        apply_rule_b(VertexKey::default(), &mut e).unwrap();
        assert_eq!(e[0].class, PlaneSide::Above);
    }

    /// Consecutive ON entries are the loud invariant failure, never a
    /// silent walk-on.
    #[test]
    fn consecutive_on_refuses() {
        let mut e = entries(&[O, O, B]);
        let err = apply_rule_b(VertexKey::default(), &mut e).unwrap_err();
        assert!(matches!(err, SplitReduceError::ConsecutiveOnSectors { .. }));
    }
}

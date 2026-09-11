//! Reclassification rules (a) and (b) — every sign derived from
//! [`geom_brep::enters_material`] (F3), nothing sign-copied; rule (b)
//! adjudicated between the two contradicting witnesses (F4) — the
//! derivation is this module's docs (M3-LOG-ready).
//!
//! **[`face_extent`] is not one of those rules, and this is not its
//! home.** It is the `Margin::levered` lever arm for the
//! coplanarity/sense predicates, and two of its three callers are
//! outside this module (`chord_join`'s section chooser, twice) against
//! one inside it (`apply_rule_a`) — a shared core hosted inside the
//! minority consumer. **That cost has already shown once**: the
//! function's error contract was extended, documented on the function,
//! and discarded by both outside callers, which `map_err` it into
//! their own refusals. Named here rather than moved; moving it is a
//! placement decision with its own callers to re-audit.
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
//! never ON. (One more inhabitant: a `WideBisector` duplicate whose
//! bisector lies exactly in the plane also arrives here as On and is
//! reclassified by the same table as if it were an edge entry — this
//! matches the book, which stores duplicates indistinguishably in the
//! same array, and is harmless: a duplicate only relays its sector's
//! side into the run structure.) The witnesses **contradict** on the
//! two symmetric cases
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
//! wedges whose face fans at the tip vertex are **disjoint**, and a
//! half-edge vertex admits exactly one cyclic orbit — so one merged
//! run, producing one vertex copy, cannot host both fans, regardless
//! of how PR 3's joining later completes the section. That
//! representability-at-the-vertex fact is why TOG's AOA→ABOVE is
//! wrong: it merges one run `{slantL, tip, slantR}` ⇒ one copy pinned
//! to both fans. (The "4-face tip edge" sometimes cited here is only
//! one possible completion of that copy, not the forced one — with
//! distinct section faces the completion carries coincident distinct
//! edges instead, but the two-fan vertex remains either way.)
//! **AOA → BELOW** separates the runs (`{slantL}`, `{slantR}` — two
//! null edges, two vertex copies, one fan each; the tip edge survives
//! inside Below's coplanar top as an artifact edge). The book is
//! right; TOG §3's list is the erratum.
//!
//! **Touching-wedge fixture** (notch cut from below, material above;
//! entries `[slantL: Below, tip: ON, slantR: Below, cap-bisector:
//! Above]`): NOT settled by mirroring the argument above — copies are
//! minted only for ABOVE runs, so under either verdict both below
//! wedge fans stay on the single old vertex at PR 2 exit, and the fan
//! TOG's BOB→BELOW leaves there is contiguous and structurally
//! buildable (no 4-face edge follows from it). Two independent
//! arguments pick **BOB → ABOVE**: (i) **±n equivariance** — splitting
//! by `(o, −n)` reads the same physical configuration as the
//! tangent-edge AOA case, and the assignment of physical material to
//! pieces cannot depend on the plane's orientation, so the table must
//! pair BOB's verdict with AOA's: the book's BOB→ABOVE is the unique
//! companion of AOA→BELOW (executed witness:
//! `review_m3_pr2.rs::r1b_orientation_equivariance_pins_bob_from_aoa`).
//! (ii) **Distinct-entity 3′ representability** — BOB→ABOVE gives the
//! groove fin its own vertex copies, so the below piece's tip contact
//! happens through distinct entities (a legal 3′ touching); TOG's
//! BOB→BELOW instead leaves the fin sharing the old vertex with both
//! material wedges — a shared-entity pinch, unrepresentable per F2.
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
use geom_core::{Band, Decide, Margin, Sign};

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
        let (face, n_face, is_plane) = sector_face(body, vertex, entries[k].he)?;
        let sliver = |diag| SplitReduceError::SliverSector { vertex, face, diag };
        let extent = face_extent(body, vertex, face)?;
        // Distinct K name from the shared sector rungs' `sector_arm`
        // (the shorter-chord arm): this margin is the FACE extent.
        // Still lane-prefixed, and correctly so — the boolean lane has
        // no counterpart to it, so #652's pooling does not reach it.
        match decide("split_sector_extent", Margin::of(extent), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => {
                return Err(sliver(geom_core::Indeterminate {
                    margin: geom_core::MarginDiag::Invalid,
                    band,
                    predicate: Some("split_sector_extent"),
                }));
            }
            Err(diag) => return Err(sliver(diag)),
        }
        // Oriented parallelism, part 1: are the normals parallel at
        // all? Margin ‖n_face × n_SP‖·extent (≥ 0; sin θ metered at
        // the face extent). For curved faces `n_face` is the LOCAL
        // normal at the base vertex (M5 PR 5): a curved face is never
        // coplanar with the split plane, so a parallel local normal is
        // a **tangent contact** — C7 territory, refused typed (never
        // marched into); the arm for that pair moves at M5 PR 9.
        let parallel_margin = Margin::levered(n_face.vec().cross(plane.normal).norm(), extent);
        match decide("split_sector_coplanar", parallel_margin, band) {
            Ok(Sign::Zero) => {
                if !is_plane {
                    // A plane-parallel LOCAL normal on a curved face
                    // is a tangent CONTACT, not a coplanar sector.
                    // The C12.2 descent (M5 PR 9): if the surface
                    // definitely bends off the shared tangent plane
                    // (largest tangent-plane normal curvature, metered
                    // as its displacement at the face extent — D4 ¶1),
                    // the sector is NOT plane-like — rule (a) stays
                    // silent and the departure trileans own the edge
                    // classes (they carry the same second-order
                    // descent). A second-order tie keeps the typed
                    // refusal (the surfaces under-determine the
                    // contact — never guess); in-band escalates (F6:
                    // an osculating pair is a sliver at this ε).
                    let corrupt = || SplitReduceError::CorruptOperand { vertex };
                    let surface_key = body.get_face(face).ok_or_else(corrupt)?.surface;
                    let surface = body.get_surface(surface_key).ok_or_else(corrupt)?;
                    let p_base = *body
                        .get_point(body.get_vertex(vertex).ok_or_else(corrupt)?.point)
                        .ok_or_else(corrupt)?;
                    let kappa = geom_brep::implicit_max_normal_curvature(surface, p_base);
                    // Ledger row F11 (unchanged by the clause-(i)
                    // migration): the sagitta is metered at the
                    // WHOLE-FACE extent, over-refusal direction —
                    // arm-policy question, own unit.
                    let so_margin = Margin::sagitta(kappa, extent);
                    match decide("tangent_sector_osculation", so_margin, band) {
                        Ok(Sign::Positive) => continue,
                        Ok(Sign::Zero | Sign::Negative) => {
                            return Err(SplitReduceError::TangencyUnsupported { face, vertex });
                        }
                        Err(diag) => return Err(sliver(diag)),
                    }
                }
            }
            Ok(_) => continue, // definitely not coplanar: rule (a) silent
            Err(diag) => return Err(sliver(diag)),
        }
        // Part 2, the material sense — the F3 primitive with
        // dir = +n_SP (module docs): Exits ⇒ material Below.
        //
        // Exactly one of the two vectors carries an orientation (S10),
        // and the types say which. `n_face` is the FACE's
        // outward normal, an `OutwardNormal` minted from chart × sense
        // in this lane's `neighborhood::sector_face` — this is a
        // material-side verdict and
        // inverts on a reversed face read off the chart, so the
        // primitive refuses anything else in that slot. `plane.normal`
        // is the SPLIT PLANE's: an operation input that DEFINES the
        // Above/Below convention, belonging to no face and carrying no
        // sense to thread, which is why it travels as a bare vector in
        // the `dir` slot (likewise the parallelism margin above, which
        // is a magnitude in any case).
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
///
/// The arm must be an OVER-estimate of that displacement: it divides
/// out of the caller's angular residuals ([`Margin::levered`]), so an
/// under-claimed arm shrinks the margin and makes a face that is not
/// parallel to the split plane likelier to decide `Zero` — a wrong
/// answer, not a loud one. Two loop shapes have to be read with that
/// in mind rather than walked past:
///
/// - A [`LoopBoundary::Empty`] **ring** is an isolated vertex, and
///   that vertex is part of the face's boundary — it contributes its
///   own distance like any other boundary vertex.
/// - A [`LoopBoundary::Empty`] **outer** loop means the face has no
///   outer boundary at all, so its locus is unbounded and no finite
///   arm over-estimates anything. That is refused, not measured.
///   `validate_closed`'s tier-2 check 1 rejects every empty loop, so a
///   validated operand cannot carry one; the boolean's own operand
///   gates (`gate_operand_pairs`, `gate_maximal_faces`) do not run
///   that check, which is why the refusal is here rather than assumed.
///
/// The refusal is [`SplitReduceError::CorruptOperand`], whose own doc
/// is *"a traversal failed (broken orbit/loop or a **lone vertex**):
/// the operand is not a well-formed closed solid"* — which is what an
/// empty outer loop is, and what `validate_closed` calls
/// `ScaffoldingEmptyLoop`. It names the loop's **lone vertex**, not the
/// caller's base vertex, so the message points at the thing that is
/// wrong. It cannot also name the FACE: the variant carries a
/// `VertexKey` only, and widening it to an `EntityId` is public API,
/// filed as issue #695 (`splitting/neighborhood.rs`). Both outside
/// callers (`chord_join.rs:1088`, `:1289`) then `map_err` this into
/// their own corrupt-face / corrupt-vertex refusals, so at those two
/// the distinction is flattened on arrival — loud, but reported as a
/// body corruption for what is really unsupported inventory. Closing
/// that properly is #695's, not this arm's.
///
/// [`LoopBoundary::Empty`]: crate::entity::LoopBoundary::Empty
/// [`Margin::levered`]: geom_core::Margin::levered
pub(crate) fn face_extent<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    face: FaceKey,
) -> Result<T, SplitReduceError> {
    use crate::entity::LoopBoundary;
    let corrupt = || SplitReduceError::CorruptOperand { vertex };
    let point_of = |v: VertexKey| -> Result<geom_core::Point3<T>, SplitReduceError> {
        Ok(*body
            .get_point(body.get_vertex(v).ok_or_else(corrupt)?.point)
            .ok_or_else(corrupt)?)
    };
    let p_base = point_of(vertex)?;
    let face_data = body.get_face(face).ok_or_else(corrupt)?;
    let mut extent = T::zero();
    let outer = face_data.outer;
    let loops = core::iter::once(outer).chain(face_data.rings.iter().copied());
    for loop_key in loops {
        let loop_data = body.get_loop(loop_key).ok_or_else(corrupt)?;
        let first = match loop_data.boundary {
            LoopBoundary::Cycle { first } => first,
            // An unbounded face has no finite lever arm (docs above).
            // Named at the loop's own lone vertex, not the caller's
            // base vertex: that is the entity the refusal is about.
            LoopBoundary::Empty { vertex: lone } if loop_key == outer => {
                return Err(SplitReduceError::CorruptOperand { vertex: lone });
            }
            LoopBoundary::Empty { vertex: lone } => {
                extent = extent.max((point_of(lone)? - p_base).norm());
                continue;
            }
        };
        for he in body.loop_cycle(first).ok_or_else(corrupt)? {
            let start = body.get_half_edge(he).ok_or_else(corrupt)?.start;
            extent = extent.max((point_of(start)? - p_base).norm());
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
    use geom_core::Tol;

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

    // ============ The lever arm may not be under-claimed ============

    /// **An unbounded face has no lever arm.** `mvfs` seeds a face
    /// whose OUTER loop is a lone vertex, so the face's locus is the
    /// whole carrier: no finite distance over-estimates the
    /// displacement an angular error induces across it. `face_extent`
    /// used to walk past that loop and answer `0`, which is the
    /// under-claiming direction — a zero arm makes every angular
    /// residual decide `Zero`, i.e. coplanar.
    ///
    /// The zero answer was loud at ONE caller by accident (`apply_rule_a`
    /// gates on `split_sector_extent` being definitely positive) and
    /// silent at the other two, in `chord_join`, which pass the extent
    /// straight into `section_case`. The refusal is at the source.
    #[test]
    fn an_unbounded_face_has_no_lever_arm() {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(geom_core::Point3::new(0.0, 0.0, 0.0)).unwrap();
        assert!(matches!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            crate::entity::LoopBoundary::Empty { .. }
        ));
        // `face_extent` mints `CorruptOperand` from eleven arena
        // lookups as well as from the arm under test, so the variant
        // alone cannot tell the refusal from a broken fixture. Pin the
        // fixture first — every lookup the function makes resolves —
        // and then pin the vertex the refusal NAMES, which is the
        // loop's lone vertex and not the base vertex the eleven others
        // would report.
        assert!(body.get_face(seed.face).is_some(), "fixture: face resolves");
        assert!(
            body.get_vertex(seed.vertex)
                .and_then(|v| body.get_point(v.point))
                .is_some(),
            "fixture: the base vertex and its point resolve"
        );
        assert!(
            matches!(
                face_extent(&body, seed.vertex, seed.face),
                Err(SplitReduceError::CorruptOperand { vertex }) if vertex == seed.vertex
            ),
            "an empty OUTER loop refuses at its own lone vertex; it must not answer zero"
        );
    }

    /// **An isolated RING vertex is boundary, so it contributes.** The
    /// same `continue` also walked past a lone-vertex ring, whose
    /// vertex is a real point of the face's boundary and can be the
    /// farthest one. Planted on the cube's seed face, at the corner
    /// diagonally opposite the base vertex, it is strictly farther
    /// than every vertex of that face's own cycle — so the row is not
    /// vacuous, and it asserts that gap rather than just the maximum.
    #[test]
    fn an_isolated_ring_vertex_contributes_its_distance() {
        let mut cube = crate::fixtures::ops_cube(Tol::witness());
        // The BOTTOM face, whose own cycle stops at the far bottom
        // corner; the seed (top) face already reaches the body's
        // farthest vertex, which would make the row vacuous.
        let face = cube.mefs[0].face;
        let base = cube.seed.vertex;
        let before = face_extent(&cube.body, base, face).unwrap();
        // The farthest vertex in the whole body from `base`.
        let p_base = *cube
            .body
            .get_point(cube.body.get_vertex(base).unwrap().point)
            .unwrap();
        let (far, far_d) = cube
            .body
            .vertices()
            .map(|(k, v)| {
                let p = *cube.body.get_point(v.point).unwrap();
                (k, (p - p_base).norm())
            })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();
        assert!(
            far_d > before,
            "the fixture's precondition: the planted vertex must be farther \
             than the face's own cycle ({far_d} vs {before})"
        );
        let ring = cube.body.add_loop(
            crate::entity::Loop {
                boundary: crate::entity::LoopBoundary::Empty { vertex: far },
                face,
            },
            crate::provenance::Provenance::Primordial { op: "h14-row" },
        );
        cube.body.get_face_mut(face).unwrap().rings.push(ring);
        let after = face_extent(&cube.body, base, face).unwrap();
        assert!(
            (after - far_d).abs() < 1e-12,
            "the ring's lone vertex sets the arm: {after} vs {far_d}"
        );
    }
}

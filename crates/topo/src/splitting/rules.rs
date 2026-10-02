//! Reclassification rules (a) and (b) — every sign derived from
//! [`geom_brep::enters_material`] (F3), nothing sign-copied; the
//! derivation is this module's docs.
//!
//! **[`face_extent`] is not one of those rules, and this is not its
//! home.** It is the `Margin::levered` lever arm for the
//! coplanarity/sense predicates, and two of its four callers are
//! outside this module (`chord_join`'s section chooser, twice) — a
//! shared core hosted inside one of its consumers. **That cost has already shown once**: the
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
//! # Rule (b) — in-plane edges, derived (F4)
//!
//! After rule (a), a remaining ON entry is an edge in the plane whose
//! flanking sectors are not coplanar; its cyclic neighbour entries are
//! never ON. (A `WideBisector` duplicate whose bisector lies exactly in
//! the plane also arrives here as On, as the book stores duplicates
//! indistinguishably in the same array.) The entry goes where the
//! material at the edge goes, and two facts decide that: the side `S`
//! of its two neighbours, which are directions into the two flanking
//! faces, and the edge's wedge (`edge_wedge`): whether the two faces
//! are tangent-continuous, read by [`geom_brep::classify_dihedral`],
//! and otherwise whether the material wedge is under 180° (convex) or
//! over it (reflex), read by [`geom_brep::enters_material`].
//!
//! | neighbours | entry | verdict |
//! |---|---|---|
//! | `S`-ON-`S` | convex edge | `S` |
//! | `S`-ON-`S` | reflex edge | opposite `S` |
//! | `S`-ON-`S` | smooth edge or duplicate, convex graze | `S` |
//! | `S`-ON-`S` | smooth edge or duplicate, concave graze | opposite `S` |
//! | `S`-ON-`S` | smooth edge or duplicate, no graze | opposite `S` (a safety default) |
//! | `A`-ON-`B`, `B`-ON-`A` | any | `Below` |
//! | | in the band | refused, [`SplitReduceError::SliverSector`] |
//!
//! **Convex** — a one-sided tangency. Both faces leave the edge into
//! `S`, so every bit of material at the edge is on `S`. The edge stays
//! an ordinary edge of `S`'s piece: the vertex's runs do not split, no
//! null edge is minted, and the edge adds nothing to the section. That
//! is the answer the face contact (rule (a)) and the vertex contact (no
//! runs, `insert::above_runs`) already give.
//!
//! **Reflex** — the plane passes through the edge, with material on
//! both sides. Tangent-edge fixture: a V-notch cut into a block's top,
//! tip edge exactly in the plane, material below; at a tip vertex the
//! entries are cyclically `[slantL: Above, tip: ON, slantR: Above,
//! cap-bisector: Below]`. Above's material near the tip is two wedges
//! whose face fans at the tip vertex are **disjoint**, and a half-edge
//! vertex admits exactly one cyclic orbit, so one merged run, producing
//! one vertex copy, cannot host both fans. Sending the tip `Below`
//! separates the runs (`{slantL}`, `{slantR}` — two null edges, two
//! vertex copies, one fan each; the tip edge survives inside Below's
//! coplanar top as an artifact edge). TOG §3's AOA→ABOVE is the
//! erratum; the book's Program 14.6 is right. The touching-wedge
//! fixture (the notch cut from below, `[slantL: Below, tip: ON,
//! slantR: Below, cap-bisector: Above]`) reads the same physical
//! configuration under `−n`, and the assignment of material to pieces
//! cannot depend on the plane's orientation, so its tip goes `Above`
//! (executed witness:
//! `review_m3_pr2.rs::r1b_orientation_equivariance_pins_bob_from_aoa`).
//! That also gives the groove fin its own vertex copies, a legal 3′
//! touching through distinct entities rather than a shared-entity
//! pinch (F2).
//!
//! **Smooth edges and duplicates: the wall decides.** Neither carries
//! a corner to read. A smooth edge (a curved face's seam along the
//! plane) is unsigned: the seam, the cusp and the slit all classify
//! smooth. A duplicate stands inside a sector of one face of 180° or
//! more, which includes the straight sector a root inserted on a cap's
//! rim leaves where the plane grazes a cylinder's wall. Where the face
//! (both faces, at a smooth edge) is a curved wall tangent to the plane,
//! the plane grazes it, and the entry goes by the wall's material
//! convexity (`wall_graze`):
//!
//! - **Convex graze** — the material lies on `S` at first order, and
//!   the wall bends into its material
//!   ([`geom_brep::bends_into_material`], a second-order read): a
//!   cylinder or a cone touched from outside. Every bit of material at
//!   the vertex is on `S`, so the entry goes to `S` and the body lands
//!   whole on its material's side, as a planar edge contact does.
//! - **Concave graze** — the material lies across the plane at first
//!   order, and the wall bends away from it: a round hole or a conical
//!   socket touched from inside. Material lies on both sides, and the
//!   piece on `S` meets the cut face tangentially along the contact:
//!   two crescents vanishing to a knife edge that the split, having no
//!   declaration channel, refuses. The entry goes opposite `S`, which
//!   mints the null edge, and the graze refuses
//!   (`wedge_end_doors::a_split_tangent_to_a_hole_wall_refuses_the_knife_edge_it_would_mint`
//!   pins the knife edge's refusal). Sending it to `S` instead returns
//!   the true volumes with the undeclared knife edge in them.
//! - A wall's first- and second-order reads that disagree contradict
//!   the `S`-ON-`S` neighbours, which are curves on the same wall, and
//!   refuse as a sliver; a wall that osculates its tangent plane is
//!   [`SplitReduceError::TangencyUnsupported`].
//!
//! Anywhere else — a plane face, or a curved one not tangent to the
//! plane — opposite `S` stays a safety default, not a derivation: it
//! mints the null edge, so the configuration is cut or refuses, never
//! answered wrongly. The rows are
//! `sweep/tests/split_tangent_edge_curved.rs` (convex grazes of a
//! cylinder and a cone answered, concave ones guarded).
//!
//! **Mixed** neighbours are a free convention: either side yields
//! manifold results; `Below` is kept for both witnesses' agreement and
//! for rule (a)'s Fig. 14.8 coplanar-edge-goes-below choice.

use geom_brep::{EntersMaterial, WallBend, enters_material};
use geom_core::{Band, Decide, Margin, Sign};

use super::neighborhood::{chord, sector_face};
use super::{PlaneSide, SectorEntry, SectorEntryKind, SplitPlane, SplitReduceError};
use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, VertexKey};
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
                    margin: geom_core::MarginDiag::INVALID,
                    band,
                    predicate: Some("split_sector_extent"),
                    terminal_sliver: false,
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
        let parallel_margin =
            Margin::levered(n_face.vec().cross(plane.normal.get()).norm(), extent);
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
        let class = match enters_material(plane.normal.get(), n_face, extent, band) {
            Ok(EntersMaterial::Exits) => PlaneSide::Below,
            Ok(EntersMaterial::Enters) => PlaneSide::Above,
            // Tangent after the parallelism gate is contradictory —
            // escalate rather than guess.
            Ok(EntersMaterial::Tangent) => {
                return Err(sliver(geom_core::Indeterminate {
                    margin: geom_core::MarginDiag::INVALID,
                    band,
                    predicate: Some("enters_material"),
                    terminal_sliver: false,
                }));
            }
            Err(geom_brep::LeverEscalation { diag, .. }) => return Err(sliver(diag)),
        };
        entries[k].class = class;
        entries[(k + 1) % n].class = class;
    }
    Ok(())
}

/// Rule (b): reclassify every remaining ON entry by its cyclic
/// neighbours and, where both sit on one side, by the convexity of the
/// in-plane edge between its two flanking faces — the derived table
/// (module docs). Checks the no-consecutive-ONs invariant loudly first
/// (the book assumes it; we refuse if it fails).
pub(super) fn apply_rule_b<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    vertex: VertexKey,
    entries: &mut [SectorEntry],
    band: Band,
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
        let prev = entries[(k + n - 1) % n];
        let next = entries[(k + 1) % n].class;
        entries[k].class = match (prev.class, next) {
            (side @ (PlaneSide::Below | PlaneSide::Above), next) if next == side => {
                // A duplicate stands inside one face's sector of 180° or
                // more, so it has no edge to read: its face's wall
                // decides, as both faces' walls do at a smooth edge.
                match entries[k].kind {
                    SectorEntryKind::WideBisector => {
                        wall_graze(body, plane, vertex, &[entries[k].he], side, band)?
                    }
                    SectorEntryKind::Edge => {
                        match edge_wedge(body, vertex, prev.he, entries[k].he, band)? {
                            Some(true) => side,
                            Some(false) => side.opposite(),
                            None => wall_graze(
                                body,
                                plane,
                                vertex,
                                &[prev.he, entries[k].he],
                                side,
                                band,
                            )?,
                        }
                    }
                }
            }
            _ => PlaneSide::Below,
        };
    }
    Ok(())
}

/// The wedge of the edge of orbit half-edge `he` at `vertex`: `None`
/// where its two faces are tangent-continuous
/// ([`geom_brep::DihedralClass::Smooth`]), else whether the material
/// wedge is under 180°. `prev` is the orbit half-edge before `he`,
/// whose CW-after sector is `he`'s own face.
///
/// Smooth or not is the dihedral classifier's own question, read
/// through [`geom_brep::classify_dihedral`] with the edge's extent, as
/// the tier-3 validator reads it, so the two cannot disagree on one
/// edge. That classifier is unsigned, and which side of 180° a corner
/// is on is not its question: that sign is [`enters_material`] of the
/// direction into the mate's face, across the edge, against the own
/// face's outward normal, metered over the same folded lever arm.
fn edge_wedge<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    prev: HalfEdgeKey,
    he: HalfEdgeKey,
    band: Band,
) -> Result<Option<bool>, SplitReduceError> {
    let corrupt = || SplitReduceError::CorruptOperand { vertex };
    let (own_face, n_own, _) = sector_face(body, vertex, prev)?;
    let (mate_face, n_mate, _) = sector_face(body, vertex, he)?;
    let sliver = |diag| SplitReduceError::SliverSector {
        vertex,
        face: mate_face,
        diag,
    };
    let surface = |face: FaceKey| {
        body.get_face(face)
            .and_then(|f| body.get_surface(f.surface))
            .ok_or_else(corrupt)
    };
    let (s_own, s_mate) = (surface(own_face)?, surface(mate_face)?);
    let p = *body
        .get_point(body.get_vertex(vertex).ok_or_else(corrupt)?.point)
        .ok_or_else(corrupt)?;
    let (_, along, _) = chord(body, vertex, he)?;
    let extent = along.norm();
    match geom_brep::classify_dihedral(s_own, s_mate, p, extent, band) {
        Ok(geom_brep::DihedralClass::Smooth) => return Ok(None),
        Ok(geom_brep::DihedralClass::Transverse) => {}
        Err(geom_brep::LeverEscalation { diag, .. }) => return Err(sliver(diag)),
    }
    // The mate runs the edge backwards, so its face's interior lies at
    // `n_mate × (−along)` (interior-left, the orbit conventions in
    // `neighborhood`).
    let into_mate = along.cross(n_mate.vec());
    let arm = geom_brep::folded_lever_arm(s_own, s_mate, p, extent);
    match enters_material(into_mate, n_own, arm, band) {
        Ok(EntersMaterial::Enters) => Ok(Some(true)),
        Ok(EntersMaterial::Exits) => Ok(Some(false)),
        // The same wedge over the same arm read transverse just above,
        // so only the two normal reads' rounding can land here; it reads
        // as smooth.
        Ok(EntersMaterial::Tangent) => Ok(None),
        Err(geom_brep::LeverEscalation { diag, .. }) => Err(sliver(diag)),
    }
}

/// Where an `S`-ON-`S` entry with no corner to read goes (module docs,
/// "Smooth edges and duplicates"): a duplicate inside one face's wide
/// sector (`sectors` holds its orbit half-edge), or a smooth edge (the
/// orbit half-edges of both its faces' sectors). Each face is read over
/// its face extent, the arm rule (a) reads it over; every face must
/// give the same verdict, or the reading refuses.
fn wall_graze<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    vertex: VertexKey,
    sectors: &[HalfEdgeKey],
    side: PlaneSide,
    band: Band,
) -> Result<PlaneSide, SplitReduceError> {
    let corrupt = || SplitReduceError::CorruptOperand { vertex };
    let p = *body
        .get_point(body.get_vertex(vertex).ok_or_else(corrupt)?.point)
        .ok_or_else(corrupt)?;
    let toward_side = if side == PlaneSide::Above {
        plane.normal.get()
    } else {
        -plane.normal.get()
    };
    let mut verdict = None;
    for &he in sectors {
        let (face, n_face, is_plane) = sector_face(body, vertex, he)?;
        let sliver = |diag| SplitReduceError::SliverSector { vertex, face, diag };
        let contradiction = |predicate| {
            sliver(geom_core::Indeterminate {
                margin: geom_core::MarginDiag::INVALID,
                band,
                predicate: Some(predicate),
                terminal_sliver: false,
            })
        };
        let extent = face_extent(body, vertex, face)?;
        let tangent = !is_plane && {
            let parallel = Margin::levered(n_face.vec().cross(plane.normal.get()).norm(), extent);
            match decide("split_sector_coplanar", parallel, band) {
                Ok(Sign::Zero) => true,
                Ok(_) => false,
                Err(diag) => return Err(sliver(diag)),
            }
        };
        let this =
            if tangent {
                let surface = body
                    .get_face(face)
                    .and_then(|f| body.get_surface(f.surface))
                    .ok_or_else(corrupt)?;
                let material_on_side = match enters_material(toward_side, n_face, extent, band) {
                    Ok(EntersMaterial::Enters) => true,
                    Ok(EntersMaterial::Exits) => false,
                    Ok(EntersMaterial::Tangent) => return Err(contradiction("enters_material")),
                    Err(geom_brep::LeverEscalation { diag, .. }) => return Err(sliver(diag)),
                };
                let bend = geom_brep::bends_into_material(surface, p, n_face, extent, band)
                    .map_err(|e| match e {
                        geom_brep::WallBendError::Indefinite(kind) => {
                            SplitReduceError::CurvedBooleanUnsupported { face, kind }
                        }
                        geom_brep::WallBendError::Lever(geom_brep::LeverEscalation {
                            diag,
                            ..
                        }) => sliver(diag),
                    })?;
                match (material_on_side, bend) {
                    (true, WallBend::IntoMaterial) => side,
                    (false, WallBend::OutOfMaterial) => side.opposite(),
                    (_, WallBend::Flat) => {
                        return Err(SplitReduceError::TangencyUnsupported { face, vertex });
                    }
                    _ => return Err(contradiction("wall_bend_order2")),
                }
            } else {
                side.opposite()
            };
        match verdict {
            Some(v) if v != this => return Err(contradiction("wall_bend_order2")),
            _ => verdict = Some(this),
        }
    }
    verdict.ok_or_else(corrupt)
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

    /// One rule (b) row: a right prism over `profile` (x–y corners,
    /// z ∈ [0, 1]) whose corner `(apex, 1)` makes an edge along z in
    /// the split plane y = 1, taken with normal `normal · y`.
    struct Row {
        label: &'static str,
        profile: &'static [(f64, f64)],
        apex: f64,
        normal: f64,
        context: PlaneSide,
        convex: Option<bool>,
        verdict: PlaneSide,
    }

    /// Wedge touching y = 1 from above along its apex edge.
    const APEX_DOWN: &[(f64, f64)] = &[(3.0, 4.0), (6.0, 1.0), (9.0, 4.0)];
    /// Wedge touching y = 1 from below.
    const APEX_UP: &[(f64, f64)] = &[(3.0, -2.0), (9.0, -2.0), (6.0, 1.0)];
    /// A thin wedge leaning right: one flanking face's outward normal
    /// points up, the other's down, yet all its material is above.
    const LEANING: &[(f64, f64)] = &[(6.0, 1.0), (8.0, 4.0), (7.0, 4.0)];
    /// A block with a V-groove whose floor edge lies in y = 1.
    const GROOVE: &[(f64, f64)] = &[
        (0.0, 0.0),
        (8.0, 0.0),
        (8.0, 2.0),
        (5.0, 2.0),
        (4.0, 1.0),
        (3.0, 2.0),
        (0.0, 2.0),
    ];
    /// The same groove leaning right: both its walls slope the same way.
    const LEANING_GROOVE: &[(f64, f64)] = &[
        (0.0, 0.0),
        (8.0, 0.0),
        (8.0, 2.0),
        (5.0, 2.0),
        (4.0, 1.0),
        (4.5, 2.0),
        (0.0, 2.0),
    ];

    /// The ON entry at `(x, 1, 0)` whose edge runs to `(x, 1, 1)`, its
    /// neighbours' classes, its wedge, and its verdict.
    fn classify_apex(row: &Row) -> (PlaneSide, PlaneSide, Option<bool>, PlaneSide) {
        let tol = Tol::witness();
        let fx = crate::test_support_fixtures::prism::<f64>(row.profile, 1.0, tol);
        let plane = crate::test_support_fixtures::split_plane(
            geom_core::Point3::new(0.0, 1.0, 0.0),
            geom_core::Vec3::new(0.0, row.normal, 0.0),
            tol,
        );
        let at = |z: f64| {
            let i = row
                .profile
                .iter()
                .position(|&(x, y)| x == row.apex && y == 1.0);
            let i = i.expect("the profile has the apex corner");
            if z == 0.0 { fx.bottom[i] } else { fx.top[i] }
        };
        let (base, far) = (at(0.0), at(1.0));
        let band = Band::linear(tol).unwrap();
        let (sides, _) = crate::vertex_sides(&fx.body, &plane, tol).unwrap();
        let entries =
            super::super::classify_neighborhood(&fx.body, &plane, &sides, base, band).unwrap();
        let n = entries.len();
        let k = entries
            .iter()
            .position(|e| {
                e.kind == SectorEntryKind::Edge && fx.body.half_edge_end(e.he) == Some(far)
            })
            .expect("the in-plane edge is in the orbit");
        let (prev, next) = (entries[(k + n - 1) % n], entries[(k + 1) % n]);
        let convex = edge_wedge(&fx.body, base, prev.he, entries[k].he, band).unwrap();
        (prev.class, next.class, convex, entries[k].class)
    }

    /// The derived rule (b) table (module docs), one fixture per row,
    /// both plane orientations: a convex in-plane edge goes with its
    /// neighbours, a reflex one to the opposite side.
    #[test]
    fn rule_b_derived_table() {
        const CONVEX: Option<bool> = Some(true);
        const REFLEX: Option<bool> = Some(false);
        #[rustfmt::skip]
        let rows = [
            Row { label: "wedge from above, +y", profile: APEX_DOWN, apex: 6.0, normal: 1.0, context: A, convex: CONVEX, verdict: A },
            Row { label: "wedge from above, -y", profile: APEX_DOWN, apex: 6.0, normal: -1.0, context: B, convex: CONVEX, verdict: B },
            Row { label: "wedge from below, +y", profile: APEX_UP, apex: 6.0, normal: 1.0, context: B, convex: CONVEX, verdict: B },
            Row { label: "wedge from below, -y", profile: APEX_UP, apex: 6.0, normal: -1.0, context: A, convex: CONVEX, verdict: A },
            Row { label: "leaning wedge, +y", profile: LEANING, apex: 6.0, normal: 1.0, context: A, convex: CONVEX, verdict: A },
            Row { label: "groove floor, +y", profile: GROOVE, apex: 4.0, normal: 1.0, context: A, convex: REFLEX, verdict: B },
            Row { label: "groove floor, -y", profile: GROOVE, apex: 4.0, normal: -1.0, context: B, convex: REFLEX, verdict: A },
            Row { label: "leaning groove floor, +y", profile: LEANING_GROOVE, apex: 4.0, normal: 1.0, context: A, convex: REFLEX, verdict: B },
        ];
        for row in &rows {
            let (prev, next, convex, verdict) = classify_apex(row);
            assert_eq!(
                (prev, next),
                (row.context, row.context),
                "{}: context",
                row.label
            );
            assert_eq!(convex, row.convex, "{}: wedge", row.label);
            assert_eq!(verdict, row.verdict, "{}: verdict", row.label);
        }
    }

    /// Mixed neighbours take `Below` whatever the edge: a brick's edge
    /// through which the plane y = z cuts, in both orientations.
    #[test]
    fn rule_b_mixed_context_goes_below() {
        let tol = Tol::witness();
        let body =
            crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let band = Band::linear(tol).unwrap();
        let h = core::f64::consts::FRAC_1_SQRT_2;
        let point = |v: VertexKey| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        let at = |x: f64| {
            body.vertices()
                .find(|&(k, _)| {
                    let p = point(k);
                    (p.x, p.y, p.z) == (x, 0.0, 0.0)
                })
                .unwrap()
                .0
        };
        let (base, far) = (at(0.0), at(1.0));
        for s in [1.0, -1.0] {
            let plane = crate::test_support_fixtures::split_plane(
                geom_core::Point3::new(0.0, 0.0, 0.0),
                geom_core::Vec3::new(0.0, s * h, -s * h),
                tol,
            );
            let (sides, _) = crate::vertex_sides(&body, &plane, tol).unwrap();
            let entries =
                super::super::classify_neighborhood(&body, &plane, &sides, base, band).unwrap();
            let n = entries.len();
            let k = entries
                .iter()
                .position(|e| body.half_edge_end(e.he) == Some(far))
                .unwrap();
            let pair = (entries[(k + n - 1) % n].class, entries[(k + 1) % n].class);
            assert!(matches!(pair, (A, B) | (B, A)), "s = {s}: context {pair:?}");
            assert_eq!(entries[k].class, PlaneSide::Below, "s = {s}");
        }
    }

    /// Consecutive ON entries are the loud invariant failure, never a
    /// silent walk-on.
    #[test]
    fn consecutive_on_refuses() {
        let mut e = entries(&[O, O, B]);
        let body = Body::<f64>::new();
        let band = Band::linear(Tol::witness()).unwrap();
        let plane = crate::test_support_fixtures::split_plane(
            geom_core::Point3::origin(),
            geom_core::Vec3::unit_z(),
            Tol::witness(),
        );
        let err = apply_rule_b(&body, &plane, VertexKey::default(), &mut e, band).unwrap_err();
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
        let seed = body
            .mvfs(geom_core::Point3::new(0.0, 0.0, 0.0), true)
            .unwrap();
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
        let mut cube = crate::test_support_fixtures::declined_cube::<f64>(Tol::witness());
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

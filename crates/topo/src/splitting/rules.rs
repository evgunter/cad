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
//! | `S`-ON-`S` | smooth edge or duplicate, concave graze | refused by rule (a), [`SplitReduceError::KnifeEdge`] |
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
//! - **Concave graze** — the wall bends away from its material: a
//!   round hole or a conical socket touched from inside. Material lies
//!   on both sides of the plane, and the piece on the wall's side meets
//!   the cut face tangentially along the contact: two crescents
//!   vanishing to a knife edge that the split, having no declaration
//!   channel, refuses. Rule (a) refuses it where it finds the wall
//!   tangent ([`SplitReduceError::KnifeEdge`]), so rule (b) never holds
//!   one; it reads the same contact there whether the wall's neighbours
//!   are rim edges, a seam or a face in the plane
//!   (`split_tangent_edge_curved.rs`'s concave rows). Sending it to `S`
//!   instead returns the true volumes with the hole's wall touching the
//!   cut face's interior along the contact, with no edge for it, and
//!   tier 3 passes that
//!   (`work/cleave/tier-3-passes-a-curved-wall-touching-a-plane-face-interior-along-a-line.md`).
//! - A convex wall whose material, read at first order, lies across
//!   from its `S`-ON-`S` neighbours contradicts them (they are curves
//!   on the same wall) and refuses as a sliver; a wall that osculates
//!   its tangent plane is [`SplitReduceError::TangencyUnsupported`].
//!
//! Anywhere else — a plane face, or a curved one not tangent to the
//! plane — opposite `S` stays a safety default, not a derivation: it
//! mints the null edge, and the join cuts there. A contact with no
//! material across it never takes the default: a cusp's zero-width
//! sector is refused by the neighbourhood classifier before either
//! rule runs (`SliverSector` on `sector_straight`), and a curved wall
//! tangent to the plane is rule (a)'s. The rows are
//! `sweep/tests/split_tangent_edge_curved.rs` (convex grazes of a
//! cylinder and a cone answered, concave ones refused).
//!
//! **Mixed** neighbours are a free convention: either side yields
//! manifold results; `Below` is kept for both witnesses' agreement and
//! for rule (a)'s Fig. 14.8 coplanar-edge-goes-below choice.

use geom_brep::{EntersMaterial, WallBend, enters_material};
use geom_core::{Band, Decide, Margin, Point3, Sign};

use super::neighborhood::{chord, sector_face};
use super::{
    KnifeEdge, KnifeEdgeSite, PlaneSide, SectorEntry, SectorEntryKind, SplitPlane, SplitReduceError,
};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::live::{BoundaryMember, Proven, proven};
use crate::validate::decide;

/// Rule (a): reclassify both bounding entries of every
/// plane-coplanar sector (module docs for the derivation). Sweeps
/// entries in order; a later coplanar sector's verdict overwrites an
/// earlier one's on a shared entry (only reachable through adjacent
/// coplanar faces — a maximal-faces violation; deterministic
/// last-wins, as the book).
///
/// Returns, per entry, whether its sector's face is a curved wall the
/// plane grazes from outside (tangent at the vertex, definitely bending
/// into its material), which rule (b) reads rather than deciding the
/// tangency again. A wall that bends away from its material is a knife
/// edge, refused here ([`SplitReduceError::KnifeEdge`]).
///
/// A wall's rows read its face extent over its edges' spans alone,
/// while the split's chords through the same wall read the agreed
/// section (`chord_join::agreed_section`), so a sector read grazed here
/// can still escalate at its chord.
pub(super) fn apply_rule_a<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    vertex: VertexKey,
    entries: &mut [SectorEntry],
    band: Band,
) -> Result<Vec<bool>, SplitReduceError> {
    let n = entries.len();
    let mut grazes = vec![false; n];
    // The edges lying in the plane, as classified before this rule
    // rewrites any entry: where a knife edge's contact runs.
    let in_plane: Vec<bool> = entries
        .iter()
        .map(|e| e.class == PlaneSide::On && e.kind == SectorEntryKind::Edge)
        .collect();
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
                return Err(sliver(crate::invalid_margin::invalid(
                    band,
                    "split_sector_extent",
                )));
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
                    let surface = face_surface(body, face);
                    let p_base = body.resolve_vertex_point(vertex, Proven);
                    let kappa = geom_brep::implicit_max_normal_curvature(surface, p_base);
                    // Ledger row F11: the sagitta is metered at the
                    // WHOLE-FACE extent, which decides a bend more
                    // readily than the contact's own arm would —
                    // arm-policy question, own unit.
                    let so_margin = Margin::sagitta(kappa, extent);
                    match decide("tangent_sector_osculation", so_margin, band) {
                        Ok(Sign::Positive) => {
                            let bend = geom_brep::bends_into_material(
                                surface, p_base, n_face, extent, band,
                            )
                            .map_err(|e| match e {
                                geom_brep::WallBendError::Indefinite(kind) => unreachable!(
                                    "a {kind:?} wall reached rule (a): `sector_face` admits \
                                     only planes, cylinders and cones"
                                ),
                                geom_brep::WallBendError::Lever(geom_brep::LeverEscalation {
                                    diag,
                                    ..
                                }) => sliver(diag),
                            })?;
                            match bend {
                                WallBend::IntoMaterial => {
                                    grazes[k] = true;
                                    continue;
                                }
                                WallBend::OutOfMaterial => {
                                    let at = [k, (k + 1) % n]
                                        .into_iter()
                                        .filter(|&j| in_plane[j])
                                        .map(|j| {
                                            proven(
                                                &body.half_edges,
                                                entries[j].he,
                                                EntityId::HalfEdge,
                                            )
                                            .edge
                                        })
                                        .find(|&e| straight(body, e))
                                        .map_or(KnifeEdgeSite::Vertex(vertex), KnifeEdgeSite::Edge);
                                    return Err(SplitReduceError::KnifeEdge(KnifeEdge {
                                        wall: face,
                                        at,
                                    }));
                                }
                                // A cylinder or cone has one zero principal
                                // curvature, so this margin is half the
                                // osculation margin just decided ≥ Kε: it
                                // reads flat only when K ≤ 2, which
                                // `CAD_AMBIGUITY_K` admits (any K > 1).
                                WallBend::Flat => {
                                    return Err(SplitReduceError::TangencyUnsupported {
                                        face,
                                        vertex,
                                    });
                                }
                            }
                        }
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
                return Err(sliver(crate::invalid_margin::invalid(
                    band,
                    "enters_material",
                )));
            }
            Err(geom_brep::LeverEscalation { diag, .. }) => return Err(sliver(diag)),
        };
        entries[k].class = class;
        entries[(k + 1) % n].class = class;
    }
    Ok(grazes)
}

/// Whether `edge` is a straight line: the shape of the contact a plane
/// tangent to a cylinder or a cone makes, along a ruling.
fn straight<T: Decide>(body: &Body<T>, edge: EdgeKey) -> bool {
    let curve = proven(&body.edges, edge, EntityId::Edge).curve;
    matches!(
        body.get_curve_geom(curve).and_then(crate::null::CurveGeom::certified),
        Some(c) if matches!(c.carrier(), geom::Curve3::Line { .. })
    )
}

/// Rule (b): reclassify every remaining ON entry by its cyclic
/// neighbours and, where both sit on one side, by the convexity of the
/// in-plane edge between its two flanking faces — the derived table
/// (module docs). Checks the no-consecutive-ONs invariant loudly first
/// (the book assumes it; we refuse if it fails). `grazes` is rule (a)'s
/// per-entry report of the curved walls the plane grazes.
pub(super) fn apply_rule_b<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    vertex: VertexKey,
    entries: &mut [SectorEntry],
    grazes: &[bool],
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
        let k_prev = (k + n - 1) % n;
        let prev = entries[k_prev];
        let next = entries[(k + 1) % n].class;
        entries[k].class = match (prev.class, next) {
            (side @ (PlaneSide::Below | PlaneSide::Above), next) if next == side => {
                // A duplicate stands inside one face's sector of 180° or
                // more, so it has no edge to read: its face's wall
                // decides, as both faces' walls do at a smooth edge.
                match entries[k].kind {
                    SectorEntryKind::WideBisector => wall_graze(
                        body,
                        plane,
                        vertex,
                        &[(entries[k].he, grazes[k])],
                        side,
                        band,
                    )?,
                    SectorEntryKind::Edge => {
                        match edge_wedge(body, vertex, prev.he, entries[k].he, band)? {
                            Some(true) => side,
                            Some(false) => side.opposite(),
                            None => wall_graze(
                                body,
                                plane,
                                vertex,
                                &[(prev.he, grazes[k_prev]), (entries[k].he, grazes[k])],
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
    let (own_face, n_own, _) = sector_face(body, vertex, prev)?;
    let (mate_face, n_mate, _) = sector_face(body, vertex, he)?;
    let sliver = |diag| SplitReduceError::SliverSector {
        vertex,
        face: mate_face,
        diag,
    };
    let (s_own, s_mate) = (face_surface(body, own_face), face_surface(body, mate_face));
    let p = body.resolve_vertex_point(vertex, Proven);
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
/// orbit half-edges of both its faces' sectors), each with rule (a)'s
/// verdict on whether the plane grazes that face. Each face is read
/// over its face extent, the arm rule (a) reads it over (ledger F11);
/// every face must give the same verdict, or the reading refuses.
fn wall_graze<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    vertex: VertexKey,
    sectors: &[(HalfEdgeKey, bool)],
    side: PlaneSide,
    band: Band,
) -> Result<PlaneSide, SplitReduceError> {
    let toward_side = if side == PlaneSide::Above {
        plane.normal.get()
    } else {
        -plane.normal.get()
    };
    let mut verdict = None;
    for &(he, grazed) in sectors {
        let (face, n_face, _) = sector_face(body, vertex, he)?;
        let sliver = |diag| SplitReduceError::SliverSector { vertex, face, diag };
        let this = if grazed {
            // Rule (a) read this wall bending into its material, so its
            // material lies on the side it leaves the plane toward.
            let extent = face_extent(body, vertex, face)?;
            match enters_material(toward_side, n_face, extent, band) {
                Ok(EntersMaterial::Enters) => side,
                Ok(EntersMaterial::Exits | EntersMaterial::Tangent) => {
                    return Err(sliver(crate::invalid_margin::invalid(
                        band,
                        "wall_bend_order2",
                    )));
                }
                Err(geom_brep::LeverEscalation { diag, .. }) => return Err(sliver(diag)),
            }
        } else {
            side.opposite()
        };
        match verdict {
            Some(v) if v != this => {
                return Err(sliver(crate::invalid_margin::invalid(
                    band,
                    "wall_bend_order2",
                )));
            }
            _ => verdict = Some(this),
        }
    }
    Ok(verdict.unwrap_or_else(|| unreachable!("rule (b) hands wall_graze at least one sector")))
}

/// The face-extent lever arm for the coplanarity/sense predicates: the
/// farthest distance from the base vertex to any point of the face's
/// boundary — its vertices, and each curved edge's far reach
/// ([`face_reach_from`]), so a one-vertex face bounded by a closed edge has
/// the arm its edge spans, not zero — the largest displacement a
/// normal-angle error can induce across this face (D4 ¶1's "face
/// extent" arm, computed, named).
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
///   validated operand cannot carry one. The split's doors run tier 2
///   on an operand that carries no verdict, but this read does not
///   assume its caller's gate (a test-support door reaches it past
///   that gate), which is why the refusal is here rather than assumed.
///
/// The refusal is [`UnboundedFace`], naming the face
/// and the loop's lone vertex. Every caller resolves `vertex` and
/// `face` in the same `&Body` call, by a refusal of its own or as a
/// record it just read, so every hop past them is a link, and a miss
/// panics.
///
/// [`LoopBoundary::Empty`]: crate::entity::LoopBoundary::Empty
/// [`Margin::levered`]: geom_core::Margin::levered
pub(crate) fn face_extent<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    face: FaceKey,
) -> Result<T, UnboundedFace> {
    face_reach_from(body, face, body.resolve_vertex_point(vertex, Proven))
}

/// [`face_extent`] from any point `at`: the farthest `face`'s boundary
/// stands from it, each certified edge levered as its
/// [`geom_brep::Reach::Span`] is. The refusal is [`face_extent`]'s.
///
/// On a plane or a cylinder that is the face's own reach: the line or
/// ruling through an interior point meets the boundary both ways, and a
/// distance is convex along it. So an edge there is read over the span
/// it holds ([`geom_brep::Reach::span_reach_from`]: a circle arc
/// exactly, an ellipse arc by its quarters' chords and bulges, a
/// segment's ends). Elsewhere the face
/// can stand farthest inside its boundary (a sphere's or a torus's far
/// side, a cone's apex), and an edge is levered round its whole carrier
/// ([`geom_brep::Reach::lever_from`]).
pub(crate) fn face_reach_from<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    at: Point3<T>,
) -> Result<T, UnboundedFace> {
    let over_span = matches!(
        face_surface(body, face),
        geom::Surface::Plane { .. } | geom::Surface::Cylinder { .. }
    );
    boundary_reach(
        body,
        face,
        |p| (p - at).norm(),
        |curve| {
            let span = span_of(curve);
            if over_span {
                span.span_reach_from(at)
            } else {
                span.lever_from(at)
            }
        },
    )
}

/// [`face_reach_from`] with every edge levered round its whole carrier
/// ([`geom_brep::Reach::lever_from`]) whatever the face it bounds: the
/// measure the germ frame reads beside the span's, serving only where
/// the two agree. The refusal is [`face_extent`]'s.
pub(crate) fn face_reach_round_from<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    at: Point3<T>,
) -> Result<T, UnboundedFace> {
    boundary_reach(
        body,
        face,
        |p| (p - at).norm(),
        |curve| span_of(curve).lever_from(at),
    )
}

/// **How far a face reaches from `at` either way along `axis`** (unit):
/// `(below, above)`, the farthest `−(x − at)·axis` and `(x − at)·axis`
/// over `face`'s boundary, each at least zero: its vertices, and each
/// certified edge's carrier over the span it holds
/// ([`geom_brep::Reach::range_along`] of its [`geom_brep::Reach::Span`]),
/// so a curved edge's bulge past every vertex is reached. A coordinate
/// along an axis has no interior extremum on a plane or a cylinder about
/// that axis, so the boundary's bounds are the face's. They lever a tilt
/// of `axis` read at `at`'s foot on it, whose reading moves by the tilt
/// times the axial distance: never inside the face, never past
/// [`face_reach_from`] the same point, and exact wherever the boundary's
/// edges are segments and conic arcs. A spiric edge is read at its
/// torus's support and a spline at its whole control net, which can
/// reach past the span the edge holds.
///
/// The refusal is [`face_extent`]'s, on the same loop shapes.
pub(crate) fn face_axial_range<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    at: Point3<T>,
    axis: geom_core::Vec3<T>,
) -> Result<(T, T), UnboundedFace> {
    let along = |p: Point3<T>| (p - at).dot(axis);
    let above = boundary_reach(body, face, along, |curve| {
        span_of(curve).range_along(at, axis).1
    })?;
    let below = boundary_reach(
        body,
        face,
        |p| -along(p),
        |curve| -span_of(curve).range_along(at, axis).0,
    )?;
    Ok((below, above))
}

/// A certified edge as the reach of the span it holds.
fn span_of<T: Decide>(curve: &geom_brep::EdgeCurve<T>) -> geom_brep::Reach<T> {
    let (t0, t1) = curve.params();
    geom_brep::Reach::Span {
        carrier: curve.carrier().clone(),
        t0,
        t1,
    }
}

/// The farthest `face`'s boundary reaches by a measure: `point` of each
/// boundary vertex, `edge` of each certified edge, refusing a face whose
/// outer loop is a lone vertex ([`face_extent`]'s docs).
fn boundary_reach<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    point: impl Fn(Point3<T>) -> T,
    edge: impl Fn(&geom_brep::EdgeCurve<T>) -> T,
) -> Result<T, UnboundedFace> {
    let face_data = proven(&body.faces, face, EntityId::Face);
    let mut extent = T::zero();
    let outer = face_data.outer;
    for (loop_key, members) in body.face_boundary_by_loop(face, face_data) {
        for member in members {
            let p = match member {
                // An unbounded face has no finite lever arm.
                BoundaryMember::Isolated { vertex: lone, .. } if loop_key == outer => {
                    return Err(UnboundedFace { face, vertex: lone });
                }
                BoundaryMember::Isolated { point, .. } => point,
                BoundaryMember::Edge {
                    he,
                    half,
                    ek,
                    edge: data,
                } => {
                    if let Some(curve) = body.edge_curve_linked(ek, data).certified() {
                        extent = extent.max(edge(curve));
                    }
                    body.linked_vertex_point(half.start, EntityId::HalfEdge(he), "start")
                }
            };
            extent = extent.max(point(p));
        }
    }
    Ok(extent)
}

/// [`face_extent`]'s refusal: `face`'s outer loop is the lone `vertex`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct UnboundedFace {
    pub(crate) face: FaceKey,
    pub(crate) vertex: VertexKey,
}

impl From<UnboundedFace> for SplitReduceError {
    fn from(UnboundedFace { face, vertex }: UnboundedFace) -> Self {
        Self::UnboundedFace { face, vertex }
    }
}

/// `face`'s surface, for a face the caller resolved: its `surface` is a
/// link, and a miss panics.
fn face_surface<T: Decide>(body: &Body<T>, face: FaceKey) -> &geom::Surface<T> {
    body.face_surface_linked(face, proven(&body.faces, face, EntityId::Face))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Tol;

    /// **The face extent panics on a ring link that does not resolve**,
    /// where it stepped over it.
    #[test]
    fn the_face_extent_panics_on_a_torn_ring_link() {
        use crate::live::OPERATORS_KEEP_LINKS;
        use crate::review_d18::{ROW_FOUR, assert_torn_op_panics, tear_ring};
        let mut body = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let face = body.faces().next().map(|(k, _)| k).unwrap();
        let vertex = body.vertices().next().map(|(k, _)| k).unwrap();
        assert!(face_extent(&body, vertex, face).is_ok(), "a bounded face");
        let named = tear_ring(&mut body, face);
        assert_torn_op_panics(
            "face_extent",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| face_extent(b, vertex, face),
        );
    }

    /// **A face's axial range reaches its rim's bulge past every vertex,
    /// and not round the wall.** The wall about `z` trimmed at `φ`, its
    /// rim one closed ellipse on the seam vertex (`oblique_rim_wall`), the
    /// rim's highest point at `φ > 0` and its lowest at `φ < 0`: from that
    /// vertex the rim reaches `2·tan |φ|` down the axis (its low crest) or
    /// up it (its high one) and nothing the other way, which the vertex
    /// alone (zero) misses, and [`face_extent`]'s distance round the rim,
    /// `2/cos φ`, over-states.
    #[test]
    fn a_faces_axial_extent_reaches_its_rims_bulge() {
        for phi in [0.2, core::f64::consts::FRAC_PI_4, 1.2, -0.2, -1.2] {
            let (body, face, vertex) = crate::test_support_fixtures::oblique_rim_wall(phi);
            let at = body.resolve_vertex_point(vertex, Proven);
            let (below, above) =
                face_axial_range(&body, face, at, geom_core::Vec3::unit_z()).unwrap();
            let euclid = face_extent(&body, vertex, face).unwrap();
            let bulge = 2.0 * phi.tan().abs();
            let (far, near) = if phi > 0.0 {
                (below, above)
            } else {
                (above, below)
            };
            assert!(
                (far - bulge).abs() <= 1e-12 * bulge && near.abs() <= 1e-12,
                "φ = {phi}: the rim reaches {below} below and {above} above, not the bulge {bulge}"
            );
            assert!(
                (euclid - 2.0 / phi.cos()).abs() <= 1e-12 * euclid,
                "φ = {phi}: the face extent {euclid} is the distance round the rim"
            );
        }
    }

    /// **A face's reach reads an arc over its span only where the
    /// boundary bounds the face.** One face whose boundary is the arc of
    /// the unit circle about `z` from `(1, 0, 0)` a hundredth of a turn
    /// round, read from its start. On a cylinder about `z` (a rim arc)
    /// the face reaches the arc's far end, its chord. On the unit sphere
    /// (an equator arc) a face can stand farthest inside its boundary,
    /// out to the far side 2 away, so the arc is levered round its whole
    /// turn.
    #[test]
    fn a_faces_reach_reads_an_arc_over_its_span_only_on_a_ruled_face() {
        use geom_core::Vec3;
        let carrier = geom::Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let t1 = 0.01 * core::f64::consts::TAU;
        let chord = (carrier.eval(t1) - carrier.eval(0.0)).norm();
        for (surface, want) in [
            (
                geom::Surface::Cylinder {
                    origin: Point3::origin(),
                    axis: Vec3::unit_z(),
                    radius: 1.0,
                    u_ref: Vec3::unit_x(),
                },
                chord,
            ),
            (
                geom::Surface::Sphere {
                    center: Point3::origin(),
                    radius: 1.0,
                    u_ref: Vec3::unit_x(),
                    axis: Vec3::unit_z(),
                },
                2.0,
            ),
        ] {
            let mut body = Body::<f64>::new();
            let seed = body.mvfs(carrier.eval(0.0), true).unwrap();
            body.set_face_surface(
                seed.face,
                crate::FaceSurface::New {
                    surface: surface.clone(),
                    sense: true,
                },
            )
            .unwrap();
            let wall = body.get_face(seed.face).unwrap().surface;
            let rim_plane = body.add_surface(geom::Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            });
            body.mev(
                crate::MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                carrier.eval(t1),
                geom_brep::EdgeCurveSpec {
                    description: geom_brep::EdgeDescriptionSpec::Intersection {
                        s1: wall,
                        s2: rim_plane,
                        witness: carrier.mid_point(0.0, t1),
                    },
                    carrier: carrier.clone(),
                    param_start: 0.0,
                    param_end: t1,
                },
                Tol::witness(),
            )
            .unwrap();
            let got = face_extent(&body, seed.vertex, seed.face).unwrap();
            let slack = (1.0 - (t1 / 8.0).cos()) + 1e-15;
            assert!(
                got >= want - 1e-15 && got <= want + slack,
                "{surface:?}: the face reaches {got} from the arc's start, not {want}"
            );
        }
    }

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
        let operand = crate::test_support::finished("the fixture", fx.body.clone(), tol);
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
        let (sides, _) = crate::vertex_sides(&operand, &plane, tol).unwrap();
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
        let body = crate::test_support::finished("the body", body, tol);
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
        let grazes = vec![false; e.len()];
        let err =
            apply_rule_b(&body, &plane, VertexKey::default(), &mut e, &grazes, band).unwrap_err();
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
        assert!(
            matches!(
                face_extent(&body, seed.vertex, seed.face),
                Err(UnboundedFace { face, vertex }) if face == seed.face && vertex == seed.vertex
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
            .vertex_points()
            .map(|(k, p)| (k, (p - p_base).norm()))
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

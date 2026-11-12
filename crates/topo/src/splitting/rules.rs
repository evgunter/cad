//! Reclassification rules (a) and (b) — every sign derived from
//! [`geom_brep::enters_material`] (F3), nothing sign-copied; the
//! derivation is this module's docs.
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
//! **Above**. The parallelism gate's verdict is that read's input
//! ([`geom_brep::ParallelNormal::sense`]), so it has no `Tangent`: a
//! sense decided zero is a face too short to read it at this ε, and
//! escalates. This reproduces the
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
//! never ON. (A divider the star cut a corner at arrives here as On only
//! where its descended side rides the plane, and goes with its corner's
//! bounds.) The entry goes where the
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
//! | `S`-ON-`S` | smooth edge or duplicate, convex graze, material across from `S` | refused, [`SectorReading::GrazeAgainstNeighbours`] |
//! | `S`-ON-`S` | smooth edge or duplicate, concave graze | refused by rule (a), [`SplitReduceError::KnifeEdge`] |
//! | `S`-ON-`S` | smooth edge or duplicate, no graze | opposite `S` (a safety default) |
//! | `S`-ON-`S` | smooth edge, one face grazed and the other clear | refused, [`SectorReading::SmoothEdgeDisagrees`] |
//! | `A`-ON-`B`, `B`-ON-`A` | any | `Below` |
//! | | in the band | refused, [`SplitReduceError::SliverSector`] |
//!
//! **Convex** — a one-sided tangency. Both faces leave the edge into
//! `S`, so every bit of material at the edge is on `S`. The edge stays
//! an ordinary edge of `S`'s piece: the vertex's runs do not split, no
//! null edge is minted, and the edge adds nothing to the section. That
//! is the answer the face contact (rule (a)) and the vertex contact (no
//! runs, `star::runs`) already give.
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
//! Where Above's material joins round one end of the tip edge instead —
//! a valley in the plane ending at a vertex whose other edges all rise
//! — the runs are apart at the other end only. Above's piece touches
//! itself along the edge, and its section runs out along it and back;
//! the edge reaches into Below's section from its other end, and the
//! join kills it there (`join::Sweep::strip_spurs`).
//!
//! **Smooth edges and duplicates: the wall decides.** Neither carries
//! a corner to read. A smooth edge (a curved face's seam along the
//! plane) is unsigned: the seam, the cusp and the slit all classify
//! smooth. A duplicate stands inside a sector of one face (of 180° or
//! more, or at a cone's apex any sector its plane turns in), which
//! includes the straight sector a root inserted on a cap's
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
//!   whole on its material's side, as a planar edge contact does. At a
//!   cone's apex there is no tangent plane and no bend to read: the
//!   plane grazes the face along a ruling it holds, and the cone bends
//!   into its material wherever its sense keeps the material inside it
//!   (`apex_reading`).
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
//!   (`tier-3-passes-a-curved-wall-touching-a-plane-face-interior-along-a-line` (closed with CLEAVE: `docs/doc-ledger/cleave-leaves-the-tracker.md`)).
//! - A convex wall whose material, read at first order, lies across
//!   from its `S`-ON-`S` neighbours disagrees with them (they are curves
//!   on the same wall) and refuses as a sliver
//!   ([`SectorReading::GrazeAgainstNeighbours`]); a wall that osculates
//!   its tangent plane is [`SplitReduceError::TangencyUnsupported`].
//! - At a smooth edge each face's parallelism is read over its own face
//!   extent, so one tilt of the plane can read tangent over one face and
//!   clear over the other; the two decided readings disagree and refuse
//!   ([`SectorReading::SmoothEdgeDisagrees`]).
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

use geom_brep::{EntersMaterial, NormalAlignment, ParallelSense, WallBend, enters_material};
use geom_core::k_stats::{Magnitude, decide_magnitude_reported};
use geom_core::{Band, Decide, Indeterminate, Margin, MarginDiag, Real, Sign};

use super::{
    KnifeEdge, KnifeEdgeSite, PlaneSide, SectorReading, SplitPlane, SplitReduceError,
    TangencyReadings,
};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, HalfEdgeKey, VertexKey};
use crate::face_reach::{UnboundedFace, face_extent};
use crate::live::{Proven, proven};
use crate::sector_face::{ConeApex, SectorCarrier};
use crate::star::{Bound, Star, StarFace};
use crate::validate::decide;

/// Rule (a): reclassify both bounding entries of every
/// plane-coplanar sector (module docs for the derivation). Sweeps
/// entries in order; a later coplanar sector's verdict overwrites an
/// earlier one's on a shared entry (only reachable through adjacent
/// coplanar faces — a maximal-faces violation; deterministic
/// last-wins, as the book).
///
/// Returns, per entry, what it read of its sector's face
/// ([`WallReading`]): a curved wall the plane grazes from outside
/// (tangent at the vertex, definitely bending into its material) with its
/// parallelism verdict, which rule (b) reads rather than deciding the
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
    star: &Star<T>,
    classes: &mut [PlaneSide],
    band: Band,
) -> Result<Vec<WallReading<T>>, SplitReduceError> {
    let vertex = star.vertex;
    let n = classes.len();
    let mut readings = Vec::with_capacity(n);
    // The edges lying in the plane, as classified before this rule
    // rewrites any entry: where a knife edge's contact runs.
    let in_plane: Vec<bool> = classes
        .iter()
        .zip(&star.bounds)
        .map(|(&c, b)| c == PlaneSide::On && b.real)
        .collect();
    for k in 0..n {
        let sector = &star.sectors[k];
        let (face, is_plane) = (sector.face, sector.carrier == SectorCarrier::Plane);
        let refuse = |reading| SplitReduceError::SliverSector {
            vertex,
            face,
            reading,
        };
        let sliver = |diag| refuse(SectorReading::Escalated(diag));
        let extent = face_extent(body, vertex, face)?;
        // Distinct K name from the shared sector rungs' `sector_arm`
        // (the shorter-chord arm): this margin is the FACE extent.
        // Still lane-prefixed, and correctly so — the boolean lane has
        // no counterpart to it, so #652's pooling does not reach it.
        // `sector_shape` decided this sector's shorter chord past the band
        // before this rule runs, and the face's extent from the vertex is
        // at least that chord: a zero here is a kernel bug (D9 row 4).
        match decide_magnitude_reported("split_sector_extent", Margin::of(extent.length()), band) {
            Ok((Magnitude::Positive, _)) => {}
            Ok((Magnitude::Zero, margin)) => unreachable!(
                "face {face:?}'s extent from vertex {vertex:?} decided zero (margin {margin:e}, \
                 band ({:e}, {:e})), past which `sector_shape` decided the sector's shorter \
                 chord, an arm the extent bounds",
                band.zero(),
                band.escalate()
            ),
            Err(diag) => return Err(sliver(diag)),
        }
        // Where a knife edge at this sector touches: an in-plane straight
        // edge bounding it, else the vertex.
        let knife_at = || {
            [k, (k + 1) % n]
                .into_iter()
                .filter(|&j| in_plane[j])
                .map(|j| proven(&body.half_edges, star.bounds[j].he, EntityId::HalfEdge).edge)
                .find(|&e| straight(body, e))
                .map_or(KnifeEdgeSite::Vertex(vertex), KnifeEdgeSite::Edge)
        };
        let n_face = match sector.at {
            StarFace::Tangent(n_face) => n_face,
            StarFace::Apex(apex) => {
                let bounds = [&star.bounds[k], star.start(k)];
                let reading = apex_reading(plane, &apex, bounds, band).map_err(sliver)?;
                if let WallReading::Grazed(_) = reading
                    && !apex.material_inside()
                {
                    return Err(SplitReduceError::KnifeEdge(KnifeEdge {
                        wall: face,
                        at: knife_at(),
                    }));
                }
                readings.push(reading);
                continue;
            }
        };
        // Oriented parallelism, part 1: are the normals parallel at
        // all? Margin ‖n_face × n_SP‖·extent (≥ 0; sin θ metered at
        // the face extent). For curved faces `n_face` is the LOCAL
        // normal at the base vertex (M5 PR 5): a curved face is never
        // coplanar with the split plane, so a parallel local normal is
        // a **tangent contact** — C7 territory, refused typed (never
        // marched into); the arm for that pair moves at M5 PR 9.
        let alignment = geom_brep::normal_alignment(
            "split_sector_coplanar",
            plane.normal.get(),
            n_face,
            extent,
            band,
        )
        .map_err(sliver)?;
        let parallel = match alignment {
            NormalAlignment::Parallel(parallel) => {
                let parallel = ParallelFace(parallel);
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
                    let surface = body.proven_face(face).surface();
                    let p_base = body.resolve_vertex_point(vertex, Proven);
                    let kappa = geom_brep::implicit_max_normal_curvature(surface, p_base);
                    // Ledger row F11: the sagitta is metered at the
                    // WHOLE-FACE extent, which decides a bend more
                    // readily than the contact's own arm would —
                    // arm-policy question, own unit.
                    let so_margin = Margin::sagitta(kappa, extent.length());
                    match decide("tangent_sector_osculation", so_margin, band) {
                        Ok(Sign::Positive) => {
                            let bend = geom_brep::bends_into_material(
                                surface,
                                p_base,
                                n_face,
                                extent.length(),
                                band,
                            )
                            .map_err(|e| match e {
                                geom_brep::WallBendError::Indefinite(kind) => unreachable!(
                                    "a {kind:?} wall reached rule (a): `sector_face` admits \
                                     only planes, cylinders and cones"
                                ),
                                geom_brep::WallBendError::Lever(escalation) => {
                                    sliver(escalation.diag())
                                }
                            })?;
                            match bend {
                                WallBend::IntoMaterial => {
                                    readings.push(WallReading::Grazed(parallel));
                                    continue;
                                }
                                WallBend::OutOfMaterial => {
                                    return Err(SplitReduceError::KnifeEdge(KnifeEdge {
                                        wall: face,
                                        at: knife_at(),
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
                parallel
            }
            // Definitely not coplanar: rule (a) is silent.
            NormalAlignment::Oblique(sine) => {
                readings.push(WallReading::Clear(sine));
                continue;
            }
        };
        // Part 2, the material sense of the face part 1 decided
        // parallel ([`Parallel::material_side`]).
        let (class, _) = parallel.material_side().map_err(sliver)?;
        readings.push(WallReading::InPlane);
        classes[k] = class;
        classes[(k + 1) % n] = class;
    }
    Ok(readings)
}

/// Rule (a) at a cone's apex, for the sub-sector between the star's
/// `bounds`: an edge, or a ruling the star cut the corner at (its halving
/// or quarters, or an extremal ruling of the plane, `Star::refine`). A
/// cone is never coplanar with the plane, and the apex has no tangent
/// plane: the plane grazes the face along a ruling where the face's side
/// is extremal, and every such ruling is a bound. At an extremal ruling
/// `g` the plane's normal has no component round the axis, so the sine
/// between it and the outward normal along `g` is `|n·ĝ|`, the quantity
/// the split read the bound's side by, levered at the same reach (the
/// bound's). So the sub-sector is grazed where the outward
/// normal along either bounding ray is parallel to the plane's at that
/// ray's reach (`split_sector_coplanar`) — where the walk read that
/// entry On — and otherwise clear. The two are not one predicate: the
/// walk decides a signed side, and this decides a magnitude whose
/// parallel verdict carries the sense rule (b) reads the material side
/// by ([`geom_brep::ParallelNormal`]); they share the band, the lever
/// and, at an extremal ruling, the quantity. The cone is straight along a
/// ruling, so there is no bend to read: a graze's material side is the
/// parallel normal's sense, and whether the cone bends into its material
/// is its sense alone ([`ConeApex::material_inside`], the caller's).
fn apex_reading<T: Decide>(
    plane: &SplitPlane<T>,
    apex: &ConeApex<T>,
    bounds: [&Bound<T>; 2],
    band: Band,
) -> Result<WallReading<T>, Indeterminate> {
    let [first, second] = bounds.map(|bound| {
        geom_brep::normal_alignment(
            "split_sector_coplanar",
            plane.normal.get(),
            apex.normal_along(bound.dir),
            bound.reach.lever(),
            band,
        )
    });
    Ok(match (first?, second?) {
        (NormalAlignment::Parallel(parallel), _) | (_, NormalAlignment::Parallel(parallel)) => {
            WallReading::Grazed(ParallelFace(parallel))
        }
        (NormalAlignment::Oblique(sine), NormalAlignment::Oblique(_)) => WallReading::Clear(sine),
    })
}

/// What rule (a) read of an entry's face against the split plane, for
/// rule (b).
#[derive(Clone, Copy, Debug)]
pub(super) enum WallReading<T: Real> {
    /// A curved wall the plane grazes from outside: parallel to it at the
    /// vertex, and bending into its material.
    Grazed(ParallelFace<T>),
    /// A face read definitely not parallel to the plane, with the sine
    /// margin it was read on.
    Clear(MarginDiag),
    /// A plane face lying in the plane, both of whose sector's entries this
    /// rule reclassified, so neither reaches rule (b).
    InPlane,
}

/// A face whose outward normal rule (a) decided parallel to the split
/// plane's over its face extent (`split_sector_coplanar` decided zero),
/// carrying that verdict with the plane normal and band it was decided
/// against ([`geom_brep::ParallelNormal`]).
#[derive(Clone, Copy, Debug)]
pub(super) struct ParallelFace<T: Real>(geom_brep::ParallelNormal<T>);

impl<T: Decide> ParallelFace<T> {
    /// Which side of the plane the face's material lies on, with the sense
    /// margin it was decided on: the F3 sense of `dir = +n_SP` against the
    /// face (module docs), `Exits` ⇒ Below.
    ///
    /// Exactly one of the two vectors carries an orientation (S10), and
    /// the types say which. The face's normal is its outward normal,
    /// minted from chart × sense in `crate::sector_face::resolve`, so the
    /// verdict inverts on a reversed face. The split plane's normal
    /// DEFINES the Above/Below convention and carries no sense to thread,
    /// so it travels as a bare vector in the `dir` slot.
    fn material_side(self) -> Result<(PlaneSide, MarginDiag), Indeterminate> {
        let (sense, margin) = self.0.sense().map_err(|escalation| escalation.diag())?;
        let side = match sense {
            ParallelSense::Exits => PlaneSide::Below,
            ParallelSense::Enters => PlaneSide::Above,
        };
        Ok((side, margin))
    }
}

/// Whether `edge` is a straight line: the shape of the contact a plane
/// tangent to a cylinder or a cone makes, along a ruling.
fn straight<T: Decide>(body: &Body<T>, edge: EdgeKey) -> bool {
    let curve = proven(&body.edges, edge, EntityId::Edge).curve;
    body.get_curve_geom(curve)
        .and_then(crate::null::CurveGeom::certified)
        .is_some_and(|c| !c.carrier().is_curved())
}

/// Rule (b): reclassify every remaining ON entry by its cyclic
/// neighbours and, where both sit on one side, by the convexity of the
/// in-plane edge between its two flanking faces — the derived table
/// (module docs). Checks the no-consecutive-ONs invariant loudly first
/// (the book assumes it; we refuse if it fails). `readings` is rule
/// (a)'s per-entry report of what it read of each face.
pub(super) fn apply_rule_b<T: Decide>(
    body: &Body<T>,
    star: &Star<T>,
    (classes, unresolved): (&mut [PlaneSide], &[bool]),
    readings: &[WallReading<T>],
    band: Band,
) -> Result<(), SplitReduceError> {
    let vertex = star.vertex;
    let n = classes.len();
    for k in 0..n {
        if classes[k] == PlaneSide::On && classes[(k + 1) % n] == PlaneSide::On {
            return Err(SplitReduceError::ConsecutiveOnSectors { vertex });
        }
    }
    for k in 0..n {
        if classes[k] != PlaneSide::On {
            continue;
        }
        let k_prev = (k + n - 1) % n;
        let (prev, next) = (classes[k_prev], classes[(k + 1) % n]);
        classes[k] = match (prev, next) {
            (side @ (PlaneSide::Below | PlaneSide::Above), next) if next == side => {
                if star.bounds[k].real {
                    match edge_wedge(body, star, k, band)? {
                        Some(true) => side,
                        Some(false) => side.opposite(),
                        None => wall_graze(
                            body,
                            vertex,
                            &[
                                (star.bounds[k_prev].he, readings[k_prev]),
                                (star.bounds[k].he, readings[k]),
                            ],
                            side,
                        )?,
                    }
                } else if unresolved[k] {
                    // A divider whose descent read nothing past its tie
                    // has no second-order side: its wall decides, as for a
                    // smooth edge, and refuses where it disagrees.
                    wall_graze(body, vertex, &[(star.bounds[k].he, readings[k])], side)?
                } else {
                    // A divider still On after its descent rides the
                    // plane: its face lies on `S` at both of its
                    // sub-sectors' far bounds and in the plane along it at
                    // most, so no material across from `S` reaches it.
                    side
                }
            }
            _ => PlaneSide::Below,
        };
    }
    Ok(())
}

/// The wedge of the edge that is bound `k` of `star`: `None` where its
/// two faces are tangent-continuous ([`geom_brep::DihedralClass::Smooth`]),
/// else whether the material wedge is under 180°. The sector before the
/// bound is its own face's, the sector after it its mate's.
///
/// Smooth or not is the dihedral classifier's own question, read
/// through [`geom_brep::classify_dihedral`] with the edge's extent, as
/// the tier-3 validator reads it, so the two cannot disagree on one
/// edge. That classifier is unsigned, and which side of 180° a corner
/// is on is not its question: that sign is [`enters_material`] of the
/// direction into the mate's face, across the edge, against the own
/// face's outward normal, metered over the same folded lever arm.
/// Where either face is a cone at its apex, both are read along the
/// edge's ray ([`StarFace::normal_along`]).
fn edge_wedge<T: Decide>(
    body: &Body<T>,
    star: &Star<T>,
    k: usize,
    band: Band,
) -> Result<Option<bool>, SplitReduceError> {
    let vertex = star.vertex;
    let n = star.bounds.len();
    let (own, mate) = (&star.sectors[(k + n - 1) % n], &star.sectors[k]);
    let (own_face, own_at, mate_face, mate_at) = (own.face, own.at, mate.face, mate.at);
    let sliver = |diag| SplitReduceError::SliverSector {
        vertex,
        face: mate_face,
        reading: SectorReading::Escalated(diag),
    };
    let sector_surface = |f| body.proven_face(f).surface();
    let (s_own, s_mate) = (sector_surface(own_face), sector_surface(mate_face));
    let along = star.bounds[k].ray();
    let extent = star.bounds[k].reach.lever();
    // Both faces are read along the edge's ray: at the vertex where each
    // has a tangent plane, and out along the ray where either is a cone at
    // its apex, whose normal is constant along the ruling the ray lies on
    // and undefined at the apex itself.
    let p = body.resolve_vertex_point(vertex, Proven);
    let p = match (own_at, mate_at) {
        (StarFace::Tangent(_), StarFace::Tangent(_)) => p,
        _ => p + along,
    };
    let (n_own, n_mate) = (own_at.normal_along(along), mate_at.normal_along(along));
    match geom_brep::classify_dihedral(s_own, s_mate, p, extent, band) {
        Ok(geom_brep::Dihedral::Tangent(_)) => return Ok(None),
        Ok(geom_brep::Dihedral::Corner(_)) => {}
        Err(escalation) => return Err(sliver(escalation.diag())),
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
        Err(escalation) => Err(sliver(escalation.diag())),
    }
}

/// Where an `S`-ON-`S` entry with no corner to read goes (module docs,
/// "Smooth edges and duplicates"): a duplicate inside one face's wide
/// sector (`sectors` holds its orbit half-edge), or a smooth edge (the
/// orbit half-edges of both its faces' sectors), each with what rule (a)
/// read of that face. A grazed wall whose material lies across from `S`
/// refuses ([`SectorReading::GrazeAgainstNeighbours`]). At a smooth edge,
/// each face's parallelism is read over its own face extent (D4 ¶1's
/// parallelism arm), so one tilt of the plane can read tangent over one
/// face and clear over the other; the two decided readings then disagree
/// ([`SectorReading::SmoothEdgeDisagrees`]).
fn wall_graze<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    sectors: &[(HalfEdgeKey, WallReading<T>)],
    side: PlaneSide,
) -> Result<PlaneSide, SplitReduceError> {
    let mut verdict: Option<(PlaneSide, WallReading<T>)> = None;
    for &(he, reading) in sectors {
        let face = crate::sector_face::face_of(body, vertex, he);
        let refuse = |reading| SplitReduceError::SliverSector {
            vertex,
            face,
            reading,
        };
        let this = match reading {
            // Rule (a) read this wall bending into its material, so its
            // material lies on the side it leaves the plane toward.
            WallReading::Grazed(wall) => {
                let (material, margin) = wall
                    .material_side()
                    .map_err(|diag| refuse(SectorReading::Escalated(diag)))?;
                if material != side {
                    return Err(refuse(SectorReading::GrazeAgainstNeighbours { margin }));
                }
                side
            }
            WallReading::Clear(_) => side.opposite(),
            WallReading::InPlane => unreachable!(
                "face {face:?} lies in the plane at vertex {vertex:?}, so rule (a) took both \
                 entries of its sector off the plane and rule (b) does not read it"
            ),
        };
        match verdict {
            Some((v, first)) if v != this => {
                return Err(refuse(SectorReading::SmoothEdgeDisagrees(
                    tangency_readings(first, reading),
                )));
            }
            Some(_) => {}
            None => verdict = Some((this, reading)),
        }
    }
    Ok(verdict
        .unwrap_or_else(|| unreachable!("rule (b) hands wall_graze at least one sector"))
        .0)
}

/// The readings of a smooth edge's two faces whose verdicts differ: one
/// grazed, read tangent over its extent, and one clear over its own.
fn tangency_readings<T: Decide>(a: WallReading<T>, b: WallReading<T>) -> TangencyReadings {
    match (a, b) {
        (WallReading::Grazed(wall), WallReading::Clear(clear))
        | (WallReading::Clear(clear), WallReading::Grazed(wall)) => TangencyReadings {
            tangent: wall.0.margin(),
            clear,
            band: wall.0.band(),
        },
        (a, b) => unreachable!(
            "two faces' verdicts differ only where one is grazed and the other clear: {a:?}, {b:?}"
        ),
    }
}

impl From<UnboundedFace> for SplitReduceError {
    fn from(UnboundedFace { face, vertex }: UnboundedFace) -> Self {
        Self::UnboundedFace { face, vertex }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::entity::FaceKey;
    use geom_core::{Point3, Tol};

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
        let tips = crate::sector_shape::SlitTips::of(&fx.body);
        let hood =
            super::super::neighborhood::classify(&fx.body, &plane, &sides, base, &tips, band)
                .unwrap();
        let n = hood.classes.len();
        let k = hood
            .star
            .bounds
            .iter()
            .position(|b| b.real && fx.body.half_edge_end(b.he) == Some(far))
            .expect("the in-plane edge is in the orbit");
        let convex = edge_wedge(&fx.body, &hood.star, k, band).unwrap();
        let classes = &hood.classes;
        (
            classes[(k + n - 1) % n],
            classes[(k + 1) % n],
            convex,
            classes[k],
        )
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

    /// **A divider whose descent read nothing past its tie goes by its
    /// wall; one that rides the plane goes with its corner.** On the
    /// oblique rim's straight wall corner, with the divider On between two
    /// bounds Above and the wall read clear of a tilted plane, rule (b)
    /// sends the riding divider Above, with its bounds, and the unresolved
    /// one by its wall's reading (`wall_graze`), Below: the second order
    /// it lacks is not supplied by convention.
    #[test]
    fn an_unresolved_divider_goes_by_its_wall() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (body, wall, vertex) =
            crate::test_support_fixtures::oblique_rim_wall_from::<f64>(0.3, 1.0);
        let star = Star::walk(
            &body,
            vertex,
            &crate::sector_shape::SlitTips::default(),
            band,
        )
        .unwrap();
        let k = (0..star.bounds.len())
            .find(|&k| !star.bounds[k].real && star.sectors[k].face == wall)
            .expect("the wall's straight corner holds a divider");
        let StarFace::Tangent(n_wall) = star.sectors[k].at else {
            panic!("the wall has a tangent plane at the rim")
        };
        let plane = corner_plane((1.0, 2.0, 3.0));
        let readings: Vec<_> = (0..star.bounds.len())
            .map(|_| reading(&plane, n_wall, 1.0, band))
            .collect();
        assert!(
            matches!(readings[k], WallReading::Clear(_)),
            "the wall reads clear"
        );
        let read = |unresolved: bool| {
            let mut classes = vec![PlaneSide::Above; star.bounds.len()];
            classes[k] = PlaneSide::On;
            let mut ties = vec![false; star.bounds.len()];
            ties[k] = unresolved;
            apply_rule_b(&body, &star, (&mut classes, &ties), &readings, band).unwrap();
            classes[k]
        };
        assert_eq!(
            read(false),
            PlaneSide::Above,
            "a riding divider goes with its bounds"
        );
        assert_eq!(
            read(true),
            PlaneSide::Below,
            "an unresolved one goes by its wall"
        );
    }

    /// Consecutive ON entries are the loud invariant failure, never a
    /// silent walk-on.
    #[test]
    fn consecutive_on_refuses() {
        let mut e = [O, O, B];
        let (body, star) = corner_star();
        let band = Band::linear(Tol::witness()).unwrap();
        let readings = vec![WallReading::<f64>::InPlane; e.len()];
        let err = apply_rule_b(&body, &star, (&mut e, &[false; 3]), &readings, band).unwrap_err();
        assert!(matches!(err, SplitReduceError::ConsecutiveOnSectors { .. }));
    }

    // ============ The lever arm may not be under-claimed ============

    /// The unit cube's corner at the origin (faces x = 0, y = 0, z = 0,
    /// each reaching √2 from it) and its orbit.
    fn cube_corner() -> (Body<f64>, VertexKey, Vec<HalfEdgeKey>) {
        let body = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let vertex = body
            .vertices()
            .map(|(k, _)| k)
            .find(|&v| {
                body.resolve_vertex_point(v, Proven)
                    .equals_exactly(Point3::origin())
            })
            .expect("the cube has a corner at the origin");
        let orbit = body.vertex_orbit_linked(vertex);
        (body, vertex, orbit)
    }

    fn corner_plane(n: (f64, f64, f64)) -> SplitPlane<f64> {
        let l = (n.0 * n.0 + n.1 * n.1 + n.2 * n.2).sqrt();
        crate::test_support_fixtures::split_plane(
            Point3::origin(),
            geom_core::Vec3::new(n.0 / l, n.1 / l, n.2 / l),
            Tol::witness(),
        )
    }

    /// The cube corner's star: three convex sectors, no dividers.
    fn corner_star() -> (Body<f64>, Star<f64>) {
        let (body, vertex, _) = cube_corner();
        let star = Star::walk(
            &body,
            vertex,
            &crate::sector_shape::SlitTips::default(),
            Band::linear(Tol::witness()).unwrap(),
        )
        .unwrap();
        (body, star)
    }

    /// The orbit half-edge whose sector's face has outward normal `n`,
    /// with that normal.
    fn face_he(
        body: &Body<f64>,
        vertex: VertexKey,
        orbit: &[HalfEdgeKey],
        n: geom_core::Vec3<f64>,
    ) -> (HalfEdgeKey, geom_brep::OutwardNormal<f64>) {
        orbit
            .iter()
            .find_map(|&he| {
                let n_face = crate::sector_face::resolve(
                    body,
                    vertex,
                    he,
                    Band::linear(Tol::witness()).unwrap(),
                )
                .unwrap()
                .at_vertex;
                let crate::sector_face::VertexFace::Tangent(n_face) = n_face else {
                    panic!("a cube corner has a tangent plane on every face")
                };
                (n_face.vec().equals_exactly(n)).then_some((he, n_face))
            })
            .expect("a face at the corner has that normal")
    }

    /// What rule (a) reads of a face with outward normal `n` over `arm`
    /// against `plane` at `band`: grazed where parallel, clear otherwise.
    fn reading(
        plane: &SplitPlane<f64>,
        n: geom_brep::OutwardNormal<f64>,
        arm: f64,
        band: Band,
    ) -> WallReading<f64> {
        let arm = geom_core::test_support::fixture_lever(arm);
        match geom_brep::normal_alignment("split_sector_coplanar", plane.normal.get(), n, arm, band)
            .unwrap()
        {
            NormalAlignment::Parallel(p) => WallReading::Grazed(ParallelFace(p)),
            NormalAlignment::Oblique(sine) => WallReading::Clear(sine),
        }
    }

    /// A refusal's ending is the split's lever alone, after a payload
    /// that is not the poisoned-margin sentence.
    fn ends_in_the_lever(label: &str, e: &SplitReduceError) {
        let text = e.render(geom_brep::recourse::Reading::Build);
        assert!(
            text.ends_with(&format!(
                ". Recourse: {}",
                super::super::SPLIT_COINCIDENCE_RECOURSE
            )),
            "{label}: the lever alone: {text}"
        );
        assert!(
            !text.contains("invalid"),
            "{label}: a reading, not poison: {text}"
        );
    }

    /// **A face extent decided zero is a kernel bug.** `sector_shape`
    /// decides each sector's shorter chord past the band before rule (a),
    /// and the extent bounds it; handed a band whose zero holds the cube's
    /// √2 extent directly, rule (a) panics naming the face and margin.
    #[test]
    #[should_panic(expected = "decided zero (margin 1.414")]
    fn a_face_extent_decided_zero_panics() {
        let (body, star) = corner_star();
        let band = Band::new(2.0, 4.0).unwrap();
        let _ = apply_rule_a(
            &body,
            &corner_plane((0.0, 0.0, 1.0)),
            &star,
            &mut [PlaneSide::On; 3],
            band,
        );
    }

    /// The diagonal normals: each lies at 45° to two of the corner's faces
    /// and square to the third.
    const DIAGONALS: [(f64, f64, f64); 12] = [
        (1.0, 1.0, 0.0),
        (1.0, -1.0, 0.0),
        (-1.0, 1.0, 0.0),
        (-1.0, -1.0, 0.0),
        (1.0, 0.0, 1.0),
        (1.0, 0.0, -1.0),
        (-1.0, 0.0, 1.0),
        (-1.0, 0.0, -1.0),
        (0.0, 1.0, 1.0),
        (0.0, 1.0, -1.0),
        (0.0, -1.0, 1.0),
        (0.0, -1.0, -1.0),
    ];

    /// The sense escalation a face decided parallel over an arm too short
    /// to read it: the nonzero gate's rejection, carrying the margin
    /// `±√½·√2 = ±1` it decided zero.
    fn too_short_to_read(label: &str, e: &SplitReduceError) {
        let SplitReduceError::SliverSector {
            reading: SectorReading::Escalated(diag),
            ..
        } = e
        else {
            panic!("{label}: {e:?}");
        };
        assert_eq!(diag.predicate, Some("enters_material"), "{label}");
        assert_eq!(
            diag.margin.rejected_sign(),
            Some(Sign::Zero),
            "{label}: the gate's rejection of a decided zero"
        );
        let geom_core::ErrorTextReading::Value(m) = diag.margin.diagnostic_f64_for_error_text()
        else {
            panic!("{label}: a point margin: {diag:?}");
        };
        assert!((m.abs() - 1.0).abs() <= 1e-15, "{label}: the margin {m}");
        assert!(
            e.render(geom_brep::recourse::Reading::Build)
                .contains("within the zero band"),
            "{label}: {e_at}",
            e_at = e.render(geom_brep::recourse::Reading::Build)
        );
        ends_in_the_lever(label, e);
    }

    /// **A plane face decided parallel to the plane has no `Tangent`
    /// sense.** At K < √2 a face can read both parallel and square to the
    /// plane over one arm: the corner's faces reach √2, and at 45° the
    /// sine and the cosine there are both 1, inside a zero band of 1.1
    /// whose escalation threshold 1.3 the extent passes. Rule (a) reads
    /// the sense from the parallel verdict, so it refuses through the
    /// nonzero gate, keeping the margin, at every diagonal normal.
    #[test]
    fn a_plane_face_too_short_to_read_its_sense_escalates_its_zero() {
        let (body, star) = corner_star();
        let band = Band::new(1.1, 1.3).unwrap();
        for n in DIAGONALS {
            let label = format!("normal {n:?}");
            match apply_rule_a(
                &body,
                &corner_plane(n),
                &star,
                &mut [PlaneSide::On; 3],
                band,
            ) {
                Err(e) => too_short_to_read(&label, &e),
                Ok(_) => panic!("{label}: answered"),
            }
        }
    }

    /// **A grazed wall's material, its neighbours' side, and a smooth
    /// edge's two faces, each refused by how its reading stands.** Read
    /// through `wall_graze` with rule (a)'s readings handed in, at the
    /// cube corner, naming its faces `x = 0` (outward `−x`) and `y = 0`:
    ///
    /// - a wall too short to read its sense escalates through the nonzero
    ///   gate, at every diagonal normal that tilts it 45°;
    /// - a wall whose material lies on its neighbours' side answers that
    ///   side, and one whose material lies across refuses as
    ///   [`SectorReading::GrazeAgainstNeighbours`] with the sense margin
    ///   `±√2`, under either normal;
    /// - two faces of which only one is grazed refuse as
    ///   [`SectorReading::SmoothEdgeDisagrees`] with both sine margins, in
    ///   either order; two grazed faces (one over an arm of √2, the other
    ///   of 3) and two clear ones answer.
    #[test]
    fn a_grazed_walls_reading_refuses_by_how_it_stands() {
        use geom_core::Vec3;
        let (body, vertex, orbit) = cube_corner();
        let (hx, nx) = face_he(&body, vertex, &orbit, Vec3::new(-1.0, 0.0, 0.0));
        let (hy, ny) = face_he(&body, vertex, &orbit, Vec3::new(0.0, -1.0, 0.0));
        let root2 = 2.0f64.sqrt();
        let short = Band::new(1.1, 1.3).unwrap();
        for n in DIAGONALS.into_iter().filter(|n| n.0 != 0.0) {
            let plane = corner_plane(n);
            for side in [PlaneSide::Above, PlaneSide::Below] {
                let label = format!("normal {n:?}, {side:?}");
                let got = wall_graze(
                    &body,
                    vertex,
                    &[(hx, reading(&plane, nx, root2, short))],
                    side,
                );
                match got {
                    Err(e) => too_short_to_read(&label, &e),
                    Ok(v) => panic!("{label}: answered {v:?}"),
                }
            }
        }
        for zero in [1e-12, 1e-9, 1e-6] {
            let band = Band::new(zero, 10.0 * zero).unwrap();
            for s in [1.0, -1.0] {
                let plane = corner_plane((s, 0.0, 0.0));
                // Face x = 0 holds its material at +x.
                let material = if s > 0.0 {
                    PlaneSide::Above
                } else {
                    PlaneSide::Below
                };
                let label = format!("zero {zero:e}, normal {s}·x");
                let (gx, gx3, cy) = (
                    reading(&plane, nx, root2, band),
                    reading(&plane, nx, 3.0, band),
                    reading(&plane, ny, root2, band),
                );
                assert!(matches!(gx, WallReading::Grazed(_)), "{label}");
                assert!(matches!(cy, WallReading::Clear(_)), "{label}");
                let one = |side| wall_graze(&body, vertex, &[(hx, gx)], side);
                assert_eq!(
                    one(material).ok(),
                    Some(material),
                    "{label}: with its neighbours"
                );
                match one(material.opposite()) {
                    Err(
                        e @ SplitReduceError::SliverSector {
                            reading: SectorReading::GrazeAgainstNeighbours { margin },
                            ..
                        },
                    ) => {
                        assert_eq!(margin, MarginDiag::value(-s * root2), "{label}");
                        ends_in_the_lever(&label, &e);
                    }
                    other => panic!("{label}: against its neighbours: {other:?}"),
                }
                for sectors in [[(hx, gx), (hy, cy)], [(hy, cy), (hx, gx)]] {
                    match wall_graze(&body, vertex, &sectors, material) {
                        Err(
                            e @ SplitReduceError::SliverSector {
                                reading: SectorReading::SmoothEdgeDisagrees(readings),
                                ..
                            },
                        ) => {
                            assert_eq!(
                                readings,
                                TangencyReadings {
                                    tangent: MarginDiag::value(0.0),
                                    clear: MarginDiag::value(root2),
                                    band,
                                },
                                "{label}"
                            );
                            ends_in_the_lever(&label, &e);
                        }
                        other => panic!("{label}: one face grazed: {other:?}"),
                    }
                }
                assert_eq!(
                    wall_graze(&body, vertex, &[(hx, gx), (hy, gx3)], material).ok(),
                    Some(material),
                    "{label}: both grazed"
                );
                let cx = reading(&corner_plane((0.0, s, 0.0)), nx, root2, band);
                assert_eq!(
                    wall_graze(&body, vertex, &[(hx, cx), (hy, cy)], material).ok(),
                    Some(material.opposite()),
                    "{label}: neither grazed"
                );
            }
        }
    }

    /// **A sector reading whose margin could not be read ends in the
    /// lever and the unreadable-margin note**; every other reading ends in
    /// the lever alone. The poisoned margin is the funnel's own, from a
    /// NaN; the split reads it at the operation.
    #[test]
    fn a_poisoned_sector_reading_adds_its_note_to_the_lever() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let poisoned =
            geom_core::k_stats::decide("split_sector_coplanar", Margin::of(f64::NAN), band)
                .unwrap_err();
        assert!(poisoned.margin.is_invalid());
        let lever = format!("Recourse: {}", super::super::SPLIT_COINCIDENCE_RECOURSE);
        let noted = geom_core::lever_recourse(
            super::super::SPLIT_COINCIDENCE_RECOURSE,
            Some(geom_core::UNREADABLE_MARGIN_NOTE),
        );
        let read = geom_core::k_stats::decide("split_sector_coplanar", Margin::of(5e-9), band)
            .unwrap_err();
        for (label, reading, want) in [
            ("poisoned", SectorReading::Escalated(poisoned), &noted),
            ("in band", SectorReading::Escalated(read), &lever),
            (
                "against its neighbours",
                SectorReading::GrazeAgainstNeighbours {
                    margin: MarginDiag::value(-0.5),
                },
                &lever,
            ),
            (
                "disagreeing",
                SectorReading::SmoothEdgeDisagrees(TangencyReadings {
                    tangent: MarginDiag::value(5e-10),
                    clear: MarginDiag::value(2e-8),
                    band,
                }),
                &lever,
            ),
        ] {
            assert_eq!(&reading.ending(), want, "{label}");
            let text = SplitReduceError::SliverSector {
                vertex: VertexKey::default(),
                face: FaceKey::default(),
                reading,
            }
            .render(geom_brep::recourse::Reading::Build);
            assert!(text.ends_with(want.as_str()), "{label}: {text}");
        }
    }
}

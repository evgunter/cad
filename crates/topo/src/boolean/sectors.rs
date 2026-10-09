//! Vertex-vertex classification, part 1: the typed sector arrays and
//! the all-pairs intersecting-sector search (Programs 15.7–15.9
//! re-derived under OUR conventions — nothing sign-copied).
//!
//! # Sector representation (derived, PR 2's geometry reused)
//!
//! The orbit visits half-edges leaving the base vertex CW-from-outside;
//! the sector after orbit edge `he_i` (the corner of
//! `face(loop(mate(he_i)))`) sweeps CCW around that face's outward
//! normal **from `dir(he_{i+1})` to `dir(he_i)`**. We store each sector
//! with explicit `start`/`end` bound VECTORS (start = the CCW-first
//! bound = the next orbit chord), so `sectors[k].start ==
//! sectors[k+1].end` — the shared-bound chain the reclassification
//! neighbor propagation walks. Wide (≥ 180°) sectors are convexly
//! subdivided at an interior direction (PR 2's derivation: the cone
//! argument needs < 180°), pushed as two chained entries. The arm /
//! wideness / subdivision-direction rungs themselves are
//! [`crate::sector_shape`] — one implementation, shared with the
//! splitting lane, called here under this lane's K names.
//!
//! # Side codes (the F3 chain — the 15.7 sign resolution)
//!
//! A bound's code against the other sector's face is read as its
//! [`Reach`] allows ([`side_code`]): a line edge at its far vertex, in
//! metres; a curved edge and a bisector through
//! [`geom_brep::enters_material`]: `Enters ⇒ In`, `Exits ⇒ Out`,
//! `Tangent ⇒ On`. Program 15.7's printed `IN = +1` (positive dot ⇒ IN)
//! is coherent only for inward normals; under TOG §2's (and our)
//! outward normals the derived mapping is the opposite — mirror-pinned
//! by `mirror_check_side_codes`.
//!
//! # Intersection test (15.9 re-derived)
//!
//! Pair (a, b) intersects iff the face-plane intersection direction
//! `±(n_a × n_b)` lies within both (convex) sectors — `within` =
//! both boundary triples `(start × dir)·n`, `(dir × end)·n` not
//! definitely negative (a Zero graze counts as within: boundary hits
//! are exactly what must flow into the ON machinery). Coplanar pairs
//! (`n_a × n_b` ≈ 0) go to `sector_overlap` — the unprinted procedure,
//! designed here: overlap iff some bound of one lies strictly within
//! the other, or the two sectors' bounds are pairwise parallel (the
//! identical / identical-reversed region cases); touch-only sharing of
//! a single bound is NOT overlap.

use geom_brep::{EntersMaterial, OutwardNormal, enters_material};
use geom_core::k_stats::{Magnitude, NonzeroSign};
use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use super::{
    BooleanDecision, BooleanError, Coincide, DeclarationRead, LeverArm, Operand, SideCode,
};
use crate::body::Body;
use crate::chord_join::StaleSite;
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::live::{Proven, linked, proven};
use crate::sector_face::{SectorCarrier, SectorFaceError};
use crate::sector_shape::{SectorFault, SectorShape, sector_shape};
use crate::validate::decide;
use geom_brep::recourse::Refused;

/// What stands behind a sector bound, which decides how its side of a
/// plane is read ([`side_code`]).
#[derive(Clone, Copy, Debug)]
pub(super) enum Reach<T: geom_core::Real> {
    /// A line edge: its far vertex, read in metres off the base vertex.
    Chord {
        /// The sector's base vertex.
        base: Point3<T>,
        /// The edge's other end.
        far: Point3<T>,
    },
    /// A curved edge, levered at its own extent: a conic's departure
    /// tangent at the base vertex; a fitted (NURBS) edge's end-to-end
    /// chord, levered at the chord's length.
    Extent(T),
    /// A subdivision bisector: a direction with no point behind it,
    /// levered at its sector's arm.
    Bisector(T),
}

impl<T: geom_core::Real> Reach<T> {
    /// The length the bound reaches from its base vertex, in metres:
    /// a line edge's chord, a curved edge's extent, a bisector's arm.
    pub(super) fn length(self) -> T {
        match self {
            Self::Chord { base, far } => (far - base).norm(),
            Self::Extent(l) | Self::Bisector(l) => l,
        }
    }

    /// The bound's signed departure from the plane through its base
    /// vertex with unit normal `n`, as [`side_code`] reads it: a line
    /// edge's far vertex, in metres; any other bound's direction `dir`
    /// levered at its reach.
    pub(super) fn departure(self, dir: Vec3<T>, n: Vec3<T>) -> T {
        match self {
            Self::Chord { base, far } => crate::sector_shape::plane_offset(base, n, far),
            Self::Extent(l) | Self::Bisector(l) => dir.normalize().dot(n) * l,
        }
    }
}

/// One (convex) sector of a vertex neighborhood.
#[derive(Clone, Debug)]
pub(super) struct BoolSector<T: geom_core::Real> {
    /// The orbit half-edge the sector follows (CW-after `he`).
    pub he: HalfEdgeKey,
    /// CCW-first bound direction (the next orbit chord, or a bisector).
    pub start: Vec3<T>,
    /// CCW-last bound direction (this entry's own chord, or a bisector).
    pub end: Vec3<T>,
    /// What stands behind `start`: how its side of a plane is read.
    pub start_reach: Reach<T>,
    /// What stands behind `end`.
    pub end_reach: Reach<T>,
    /// The sector's face and outward normal.
    pub face: FaceKey,
    /// The face's outward unit normal at the base vertex, minted once
    /// in [`sector_face`] from the CHART normal and the face's `sense`
    /// bit (S10). Consumers pair it with the stored orbit order and
    /// must NOT re-apply the sense — the type says the sense is in.
    pub normal: OutwardNormal<T>,
    /// The metering arm (shorter bounding chord, in meters).
    pub arm: T,
}

impl<T: geom_core::Real> BoolSector<T> {
    /// Whether `start` is a real edge (false: a subdivision bisector).
    pub(super) fn start_edge(&self) -> bool {
        !matches!(self.start_reach, Reach::Bisector(_))
    }

    /// Whether `end` is a real edge.
    pub(super) fn end_edge(&self) -> bool {
        !matches!(self.end_reach, Reach::Bisector(_))
    }

    /// The farther of the sector's two bounds' reaches, in metres: how
    /// far from the base vertex the sector's own geometry is known.
    pub(super) fn span(&self) -> T {
        self.start_reach.length().max(self.end_reach.length())
    }
}

/// One corner of a vertex neighborhood as its orbit reads it, before
/// any subdivision: the corner after orbit half-edge `he`, sweeping CCW
/// around its face's outward normal from `start` (the next orbit
/// half-edge's chord) to `end` (`he`'s own), each unnormalized and as
/// [`Reach`] says it was read.
pub(super) struct OrbitCorner<T: geom_core::Real> {
    /// The orbit half-edge the corner follows.
    pub he: HalfEdgeKey,
    /// The next orbit half-edge's chord: the CCW-first bound.
    pub start: Vec3<T>,
    /// What stands behind `start`.
    pub start_reach: Reach<T>,
    /// `he`'s own chord: the CCW-last bound.
    pub end: Vec3<T>,
    /// What stands behind `end`.
    pub end_reach: Reach<T>,
    /// The corner's face.
    pub face: FaceKey,
    /// The face's outward unit normal at the vertex ([`sector_face`]).
    pub normal: OutwardNormal<T>,
}

/// Builds the sector array of `vertex`'s neighborhood (module docs).
pub(super) fn build_sectors<T: Decide>(
    body: &Body<T>,
    operand: Operand,
    vertex: VertexKey,
    band: Band,
) -> Result<Vec<BoolSector<T>>, BooleanError> {
    let corners = orbit_corners(body, operand, vertex);
    // A lone orbit half-edge's corner runs from its chord round to it.
    let alone = corners.len() == 1;
    let mut sectors = Vec::with_capacity(corners.len() + 2);
    for corner in corners {
        let OrbitCorner {
            he,
            start: dir_start,
            start_reach: reach_start,
            end: dir_end,
            end_reach: reach_end,
            face,
            normal,
        } = corner?;
        // The three sector-shape rungs — metering arm, wideness, and
        // the subdivision direction (PR 2's derivation: the cone
        // argument needs < 180°) — are [`crate::sector_shape`]: ONE
        // implementation, called from here and from the splitting
        // lane's neighborhood walk, under the one pooled set of K names
        // (pooled in #652). This is a call, not a copy.
        //
        // The sense-invariance argument for the `normal` passed here is
        // NOT restated: it is the contract of `sector_shape`'s `normal`
        // parameter, which is the one place a caller has to read it.
        // The value arrives typed, so it cannot be the wrong one.
        let SectorShape {
            arm,
            unit_own: u_end,
            unit_next: u_start,
            bisector: bisec,
        } = sector_shape(dir_end, dir_start, normal, alone, band).map_err(|fault| match fault {
            SectorFault::NonFiniteChord => BooleanError::NonFiniteSectorChord { vertex, face },
            SectorFault::UnderflowedChord => BooleanError::UnderflowedSectorChord { vertex, face },
            SectorFault::Rung { rung, diag } => BooleanError::Escalated {
                decision: BooleanDecision::Corner(rung),
                diag,
            },
        })?;
        match bisec {
            None => sectors.push(BoolSector {
                he,
                start: u_start,
                end: u_end,
                start_reach: reach_start,
                end_reach: reach_end,
                face,
                normal,
                arm,
            }),
            Some(b) => {
                // Chained order (module docs): the end-sharing half
                // first, then the start-sharing half.
                sectors.push(BoolSector {
                    he,
                    start: b,
                    end: u_end,
                    start_reach: Reach::Bisector(arm),
                    end_reach: reach_end,
                    face,
                    normal,
                    arm,
                });
                sectors.push(BoolSector {
                    he,
                    start: u_start,
                    end: b,
                    start_reach: reach_start,
                    end_reach: Reach::Bisector(arm),
                    face,
                    normal,
                    arm,
                });
            }
        }
    }
    Ok(sectors)
}

/// The corners of `vertex`'s neighborhood, in orbit order
/// ([`OrbitCorner`]).
pub(super) fn orbit_corners<T: Decide>(
    body: &Body<T>,
    operand: Operand,
    vertex: VertexKey,
) -> impl ExactSizeIterator<Item = Result<OrbitCorner<T>, BooleanError>> + '_ {
    let orbit = body.vertex_orbit_linked(vertex);
    if orbit.is_empty() {
        unreachable!(
            "{operand:?}'s vertex {vertex:?}, met through a contact, is a lone vertex: a gated \
             operand holds none"
        );
    }
    // The outgoing direction of an orbit half-edge, scaled to the
    // edge's honest extent — the M3 chord for `Line` carriers
    // (bit-identical), the carrier's outgoing TANGENT at the base
    // vertex scaled by `edge_extent` for conic carriers (M5 PR 9: the
    // ON-set machinery consumes curved carrier tangents instead of
    // assuming straight edges — the splitting lane's C12.2 idiom).
    let chord = move |he: HalfEdgeKey| -> (Vec3<T>, Reach<T>) {
        let end = body.proven_half_edge_end(he);
        let p_base = body.resolve_vertex_point(vertex, Proven);
        let p_end = body.resolve_vertex_point(end, Proven);
        let he_data = proven(&body.half_edges, he, EntityId::HalfEdge);
        let edge = linked(
            &body.edges,
            he_data.edge,
            EntityId::Edge,
            EntityId::HalfEdge(he),
            "edge",
        );
        let curve = body
            .edge_curve_linked(he_data.edge, edge)
            .certified()
            .unwrap_or_else(|| {
                unreachable!(
                    "{operand:?}'s vertex {vertex:?} has the null edge {:?} in its orbit: a \
                     gated operand holds none, and the boolean reads a vertex's sectors once, \
                     before it hangs one there (`vtxfac::pierced_and_paired` refuses a \
                     vertex that pierces two faces, and `classify_vertex_on_face` one that \
                     crosses a face it also pairs on)",
                    he_data.edge
                )
            });
        match curve.carrier() {
            geom::Curve3::Line { .. } => (
                p_end - p_base,
                Reach::Chord {
                    base: p_base,
                    far: p_end,
                },
            ),
            geom::Curve3::Nurbs(_) => {
                let d = p_end - p_base;
                (d, Reach::Extent(d.norm()))
            }
            geom::Curve3::Circle { .. }
            | geom::Curve3::Ellipse { .. }
            | geom::Curve3::Spiric { .. } => {
                let (t0, t1) = curve.params();
                let (tangent, _) = curve.walk_tangents(he == edge.he_plus);
                let extent =
                    geom_brep::edge_extent(curve.carrier(), t0, t1, p_end.distance(p_base));
                (tangent.normalize() * extent, Reach::Extent(extent))
            }
        }
    };
    let n = orbit.len();
    (0..n).map(move |i| {
        let he = orbit[i];
        let (end, end_reach) = chord(he); // this entry's own chord = CCW-last
        let (start, start_reach) = chord(orbit[(i + 1) % n]); // next chord = CCW-first
        let (face, normal) = sector_face(body, operand, vertex, he)?;
        Ok(OrbitCorner {
            he,
            start,
            start_reach,
            end,
            end_reach,
            face,
            normal,
        })
    })
}

/// The sector's face + outward normal at the base vertex.
///
/// The walk and the normals are [`crate::sector_face`] — ONE
/// implementation, called from here and from the splitting lane's
/// sector walk. What stays here is this lane's
/// adaptation of it, and only that: the boolean error type, whose
/// every arm carries the [`Operand`] the shared walk has no notion of.
/// All four wired arms — `Plane`, `Cylinder`, `Sphere`, `Torus` —
/// are live on this side; kinds without one refuse typed (C12.1, per
/// arm).
///
/// The normal arrives as an [`OutwardNormal`] with the face's `sense`
/// folded in (S10), minted at the shared chokepoint — which makes that
/// chokepoint the source for the whole vertex-vertex lane. Everything
/// downstream of [`BoolSector::normal`] — `within`, `side_code`,
/// `sector_overlap`, the wideness/bisector algebra above,
/// `insert::germ_dir`, `vtxfac::pierce_germ_dir` — is sense-invariant
/// GIVEN this source and must not multiply again: those sites pair the
/// normal with the STORED orbit/loop traversal, which `revert` flips in
/// the same breath as the sense bit, so a second factor would cancel
/// the first and re-break what this fixes.
pub(super) fn sector_face<T: Decide>(
    body: &Body<T>,
    operand: Operand,
    vertex: VertexKey,
    he: HalfEdgeKey,
) -> Result<(FaceKey, OutwardNormal<T>), BooleanError> {
    let resolved = crate::sector_face::resolve(body, vertex, he).map_err(|e| match e {
        SectorFaceError::Unsupported { face, kind } => BooleanError::CurvedBooleanUnsupported {
            operand,
            face,
            kind,
        },
    })?;
    // Exhaustive on purpose, exactly as the splitting wrapper is: a
    // fifth carrier arm added to the shared walk must be a compile
    // error in BOTH lanes, not silently accepted by the one whose
    // downstream algebra happens not to read the carrier.
    match resolved.carrier {
        SectorCarrier::Plane
        | SectorCarrier::Cylinder
        | SectorCarrier::Sphere
        | SectorCarrier::Torus => {}
        // The boolean's sector algebra has no cone arm: a cone-carried
        // sector refuses here, typed, as the split lane's does not.
        SectorCarrier::Cone => {
            return Err(BooleanError::CurvedBooleanUnsupported {
                operand,
                face: resolved.face,
                kind: geom::SurfaceKind::Cone,
            });
        }
    }
    Ok((resolved.face, resolved.normal))
}

/// The refusal for a bisector reading On between two bounds definitely
/// on one side (`vtxfac`'s on-edge resolution,
/// `recl::resolve_bisector_graze`): reachable only when K ≤ 2, where the
/// Zero is the band's and does not decide. The payload says what is
/// known: the reading lies within `±zero`.
pub(super) fn bisector_zero_refusal(band: Band) -> BooleanError {
    BooleanError::Escalated {
        decision: BooleanDecision::BisectorSide,
        diag: geom_core::Indeterminate {
            margin: geom_core::MarginDiag::enclosure(-band.zero(), band.zero()),
            band,
            predicate: Some("bool_sector_bisector_side"),
            terminal_sliver: false,
        },
    }
}

/// The lever a [`side_code`] call passes when its reference is a FLAT
/// datum — a sector's own face plane in the vertex-vertex lane, or the
/// reclassification's reference normal. Those verdicts are about a
/// plane, so there is no sagitta to charge and saying so at the call
/// site is the point of the name: an infinite radius of curvature makes
/// the charge vacuous, exactly as [`geom_brep::curvature_lever_arm`]
/// reports for a [`geom::Surface::Plane`].
#[allow(non_snake_case)]
pub(super) fn NO_CURVATURE<T: Decide>() -> T {
    T::from_f64(f64::MAX)
}

/// A bound's side code against a face — the F3 primitive applied
/// (module docs; the 15.7 sign resolution), read as its [`Reach`]
/// allows, and **charged for the pierced face's curvature at the
/// pierce point**.
///
/// # The reading, per reach
///
/// A `Zero` here is a verdict: it becomes [`SideCode::On`], "this
/// bound lies in the plane", and the classification builds on it. So a
/// Zero has to be a distance, not a direction times a lever that is
/// not the bound's own length.
/// - [`Reach::Chord`] (a line edge): its far vertex's signed distance
///   from the plane through the base vertex, in metres
///   ([`crate::sector_shape::plane_offset`]). The edge is its chord, so
///   every point of it lies between the base's distance (`≈ 0`, the
///   vertex is On) and the far vertex's: a Zero puts the whole edge
///   within the band of the plane. A direction levered at the SHORTER
///   chord of the sector would read Zero while a long edge's far end
///   stood thousands of bands off.
/// - [`Reach::Extent`] (a conic or fitted edge): the departure
///   direction levered at the edge's own extent — the displacement
///   the first-order datum implies at the far end. A definite sign is
///   exact (the sign of `t̂·n̂`). A Zero is a first-order tangency at
///   the vertex: the arc departs within the band of the plane over its
///   whole extent to first order, and where it goes at second order
///   is not read here — the declared-`Tangent` descent owns that
///   (`tangent_lump`), and an undeclared curved on-carrier sector
///   refuses typed (C8). What this reading does NOT certify is an arc
///   that departs tangentially and curves off; that residue is filed
///   (`work/contacthold/boolean-conic-side-code-zero-is-first-order`).
/// - [`Reach::Bisector`]: a subdivision direction has no point behind
///   it, so it is levered at its sector's arm. Its Zero is not read as
///   a verdict where it could decide anything: a bisector's code only
///   relays its sector's side between two twins of ONE physical
///   sector (the fan moves no edge across it), so with mixed
///   neighbours its code changes no topology, and with both
///   neighbours definitely on one side its reading is definite by the
///   argument at `vtxfac`'s on-edge resolution and
///   `recl::resolve_bisector_graze`, which refuse where it is not.
///
/// # Why a definite first-order verdict is charged on a curved face
///
/// The verdict is the bound's side NEAR THE VERTEX: a bound whose
/// first-order departure `s = |d̂·n̂|` from the face's tangent plane
/// is nonzero lies on that side of the face for every small enough
/// distance, whatever the face's curvature and whatever the bound's
/// own (a line, a conic arc, or a bisector with no point behind it).
/// What the charge asks is whether that side is RESOLVED at the band.
///
/// It reads the bound's tangent ray, not points of the bound. At
/// distance `l` along the ray, the face departs its tangent plane by
/// at most the sagitta `l²/lever`, so the ray sits at least
/// `g(l) = s·l − l²/lever` off the face on the verdict's side. That
/// lower bound grows faster than the first-order term, and far enough
/// out it flips sign. The witness is a hole wall (`r = 1`, material
/// outside, so the outward normal points at the axis): a direction
/// with `s ≈ 0.0995` is a definite `Exits`, and it does leave the
/// material, but by `l = 0.5` it has crossed back, and the point there
/// sits at `ρ = 1.0726`, INSIDE the material. So the charge is read
/// where `g` peaks, `l* = s·lever/2`, giving `s²·lever/4`, and capped
/// at the reach's own length: a separation witnessed only beyond the
/// bound's own extent is a direction levered at a lever that is not
/// the bound's length, the error the readings above refuse. The verdict
/// stands iff that separation clears the band. What refuses is a bound
/// leaving the face within about `2·sqrt(band/lever)` radians of
/// tangent (or a reach too short to witness its slope), whose side is
/// second order.
///
/// The charge is direction-agnostic on purpose, which is sound for
/// either bend. **`lever` is the pierced face's smallest radius of
/// curvature at the pierce point** ([`geom_brep::min_radius_of_curvature`]),
/// so a PLANE passes [`NO_CURVATURE`], the sagitta underflows to zero,
/// and the charge reduces to the reading just decided definite.
///
/// A `Zero` takes NO charge: `On` is not a side. A charge that does
/// not clear the band is a **typed refusal**, never a first-order
/// guess.
pub(super) fn side_code<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    face_normal: OutwardNormal<T>,
    lever: T,
    band: Band,
) -> Result<SideCode, BooleanError> {
    let n = face_normal.vec();
    let length = reach.length();
    let (verdict, displacement) = match reach {
        Reach::Chord { base, far } => {
            let offset = crate::sector_shape::plane_offset(base, n, far);
            let verdict = match decide("bool_chord_side", Margin::of(offset), band) {
                // Same mapping as `enters_material`: into the material
                // is against the outward normal.
                Ok(Sign::Negative) => SideCode::In,
                Ok(Sign::Positive) => SideCode::Out,
                Ok(Sign::Zero) => return Ok(SideCode::On),
                Err(diag) => {
                    return Err(BooleanError::coincidence(
                        Coincide::SectorSide,
                        DeclarationRead::Moot,
                        diag,
                    ));
                }
            };
            (verdict, offset.abs())
        }
        Reach::Extent(_) | Reach::Bisector(_) => {
            let verdict = match enters_material(dir, face_normal, length, band) {
                Ok(EntersMaterial::Enters) => SideCode::In,
                Ok(EntersMaterial::Exits) => SideCode::Out,
                Ok(EntersMaterial::Tangent) => return Ok(SideCode::On),
                Err(escalation) => {
                    return Err(BooleanError::of_lever(
                        LeverArm::SectorSide,
                        DeclarationRead::Moot,
                        at_departure(escalation, length, dir.normalize().dot(n) * length, band),
                    ));
                }
            };
            (verdict, (dir.normalize().dot(n) * length).abs())
        }
    };
    // **The sagitta bound, and why the textbook ½ is not in it.** At
    // lateral offset `l` a circle of radius `R` departs its tangent
    // line by exactly `R − sqrt(R² − l²) = l²/(R + sqrt(R² − l²))`. The
    // familiar `l²/2R` is that expression's small-`l` LIMIT and is a
    // LOWER bound on it, so charging `l²/2R` would under-charge exactly
    // where the distance is a large fraction of the radius. Dropping
    // the ½ gives `l²/R`, an upper bound unconditionally (`l* ≤ R/2`
    // here, inside the circle's reach).
    let slope = displacement / length;
    let l = (slope * T::from_f64(0.5) * lever).min(length);
    match crate::validate::decide_reported(
        "bool_pierce_sector_side_curved",
        Margin::of(slope * l - l.powi(2) / lever),
        band,
    ) {
        Ok(decided) => match Refused::of(decided, band) {
            None => Ok(verdict),
            Some(refused) => Err(BooleanError::CurvedSectorSideUnsupported { verdict: refused }),
        },
        Err(diag) => Err(BooleanError::Escalated {
            decision: BooleanDecision::PierceCurvature,
            diag,
        }),
    }
}

/// **A side reading's arm gate, quoted at the departure it reads**: the
/// arm is in band or decided zero, and the reading it meters is the
/// bound's departure from the face over that arm, `d̂·n̂·arm`, no longer
/// than the arm. A tolerance that decides the arm but leaves the
/// departure in band reads no side, so the escalation carries the
/// departure's own margin, through the arm gate's funnel, and the
/// tolerance it offers decides both, logged under its own name
/// (`"enters_material_rise"`). An exactly zero departure (a bound in the
/// face's plane) reads `On` at every tolerance that decides the arm, so
/// there the arm binds and keeps its own margin, the edge's length, as an
/// arm with no tolerance to offer does (poisoned, or no smaller tolerance
/// decides it positive).
fn at_departure<T: Decide>(
    escalation: geom_brep::LeverEscalation,
    arm: T,
    departure: T,
    band: Band,
) -> geom_brep::LeverEscalation {
    // `arm / departure` is finite unless the departure is exactly zero
    // (or poison).
    if escalation.rung() != geom_brep::LeverRung::Arm
        || !escalation.diag().offers_tolerance()
        || !geom_core::is_finite_length(arm / departure)
    {
        return escalation;
    }
    match geom_core::k_stats::decide_positive(
        "enters_material_rise",
        Margin::of(departure.abs()),
        band,
    ) {
        Err(diag) => escalation.with_diag(diag),
        // Unreachable: the departure is no longer than an arm that did
        // not read positive.
        Ok(()) => escalation,
    }
}

/// The **second-order lump** of a declared-`Tangent` sector pair
/// (CONTACT-DESIGN C7/C12.2 at the boolean lump sites): first-order
/// data ties along a tangency by definition, so the side a
/// geometrically-ON sector is treated on descends one order — which
/// side does the sector's face CURVE to, relative to the other face's
/// material?
///
/// The margin is the existing second-order sector trilean's
/// (`tangent_sector_order2{,_arm}`,
/// [`geom_brep::enters_material_order2`]): the
/// relative transverse curvature of the two carriers — the departing
/// transverse curve's acceleration on the sector's carrier measured
/// RELATIVE to the other carrier's own curving, signed against the
/// other face's outward normal — as the displacement it induces at
/// the sector's lever arm. Against a plane the partner curvature is
/// zero and the margin is the departure's own normal curvature (the
/// trilean's documented planar reading, reached bit-identically).
/// The transverse direction comes from the DEV-1 closed-form locus
/// ([`geom_brep::tangent_locus`], the same rows the door's witness
/// derivation runs): the descent exists exactly where the witness
/// lane reaches, and nowhere else.
///
/// Verdicts: definitely curving into the other body's material ⇒
/// `In`; definitely away ⇒ `Out`; an EXACT second-order zero is the
/// isolated osculating point whose residue the verified declaration
/// bridges (C4's #175 clause) — the pair is locally conformal to
/// every order the kernel measures, and the declaration's verified
/// opposed material sides make that the Eq. 15.3 ⁻ lump; an in-band
/// margin escalates (an osculating pair is a sliver at this ε — F6).
#[allow(clippy::too_many_arguments)]
pub(super) fn tangent_lump<T: Decide>(
    sector_surface: &geom::Surface<T>,
    other_surface: &geom::Surface<T>,
    reach: geom_brep::ExtentBall<T>,
    other_outward: OutwardNormal<T>,
    p: geom_core::Point3<T>,
    op: super::BooleanOp,
    on_side: Operand,
    sector_face: FaceKey,
    arm: T,
    read: DeclarationRead,
    band: Band,
) -> Result<SideCode, BooleanError> {
    use geom_brep::{TangentLocus, TangentLocusError, tangent_locus};
    let locus_dir = match tangent_locus(sector_surface, other_surface, reach, band) {
        Ok(TangentLocus::Line { dir, .. }) => dir,
        Err(TangentLocusError::Escalated(diag)) => {
            return Err(BooleanError::coincidence(
                Coincide::TangentLocus,
                read,
                diag,
            ));
        }
        // The sector pair read geometrically ON while the carriers are
        // definitely apart or crossing: the same self-contradiction
        // family as a coplanar sector with definitely-distinct planes.
        Err(TangentLocusError::NotTangent { .. }) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "declared-Tangent sector pair with definitely non-tangent carriers",
            });
        }
        // Outside the DEV-1 closed-form lane no witness exists and no
        // descent does either — the C5 typed refusal, same as every
        // unopened arm.
        Err(TangentLocusError::Unsupported { .. }) => {
            return Err(BooleanError::CurvedBooleanUnsupported {
                operand: on_side,
                face: sector_face,
                kind: sector_surface.kind(),
            });
        }
    };
    let n_ref = other_outward.vec();
    // Transverse in-tangent-plane direction (the jet family's d̂ =
    // n̂ × τ̂; quadratic consumption, so τ̂'s sign is immaterial).
    let d_hat = n_ref.cross(locus_dir).normalize();
    match tangent_relative_side(
        sector_surface,
        other_surface,
        other_outward,
        p,
        d_hat,
        arm,
        read,
        band,
    )? {
        SideCode::In => Ok(SideCode::In),
        SideCode::Out => Ok(SideCode::Out),
        // The exact-zero osculating residue, bridged by the verified
        // declaration (doc above): locally conformal with verified
        // opposed senses IS the Eq. 15.3 ⁻ posture.
        SideCode::On => Ok(super::tables::eq15_3_lump(
            op,
            on_side,
            super::plane_eq::PlaneRelation::SameOpposite,
        )),
    }
}

/// **The per-direction second-order side** of a declared-`Tangent`
/// sector pair: which side of the OTHER face's material does the
/// sector's carrier lie on along direction `d` from the tie point —
/// the relative graph-over-the-shared-tangent-plane acceleration
/// `z″ = −d̂ᵀ(∇²F)d̂ / (∇F·n̂_ref)` differenced across the two
/// carriers (the implicit-function graph over the plane normal to
/// `n̂_ref`, whose denominator carries the sign when a carrier's
/// gradient opposes `n̂_ref`, and which is a graph only where that
/// denominator is bounded away from zero — the declared tangency's
/// first-order tie), classified through the existing second-order
/// trilean (rows `tangent_sector_order2{,_arm}`). `On` is the honest
/// exact-zero: the direction rides the tangency locus (a curve on
/// either carrier along it separates at no order this kernel
/// measures) — the ON-direction machinery downstream adjudicates it,
/// exactly as a first-order On flows to the recl edge engine.
#[allow(clippy::too_many_arguments)]
pub(super) fn tangent_relative_side<T: Decide>(
    sector_surface: &geom::Surface<T>,
    other_surface: &geom::Surface<T>,
    other_outward: OutwardNormal<T>,
    p: geom_core::Point3<T>,
    d: Vec3<T>,
    arm: T,
    read: DeclarationRead,
    band: Band,
) -> Result<SideCode, BooleanError> {
    let n_ref = other_outward.vec();
    let d_hat = d.normalize();
    let graph_accel = |s: &geom::Surface<T>| {
        let g = geom_brep::implicit_gradient(s, p);
        T::zero() - geom_brep::implicit_hessian_form(s, p, d_hat) / g.dot(n_ref)
    };
    let rel_accel = graph_accel(sector_surface) - graph_accel(other_surface);
    match geom_brep::enters_material_order2(
        n_ref * rel_accel,
        T::one(),
        geom_brep::ReferenceNormal::of_face_outward(other_outward),
        arm,
        band,
    ) {
        Ok(EntersMaterial::Enters) => Ok(SideCode::In),
        Ok(EntersMaterial::Exits) => Ok(SideCode::Out),
        Ok(EntersMaterial::Tangent) => Ok(SideCode::On),
        Err(escalation) => Err(BooleanError::of_lever(
            LeverArm::SectorCurving,
            read,
            escalation,
        )),
    }
}

/// One potentially-intersecting sector pair with its four side codes
/// (the `sectors[]` record, typed).
#[derive(Clone, Copy, Debug)]
pub(super) struct PairRecord {
    /// Index into the A-side sector array.
    pub a: usize,
    /// Index into the B-side sector array.
    pub b: usize,
    /// A-sector (start, end) bound codes vs the B-sector's face.
    pub sa: (SideCode, SideCode),
    /// B-sector (start, end) bound codes vs the A-sector's face.
    pub sb: (SideCode, SideCode),
    /// Whether the pair still generates intersection geometry.
    pub intersect: bool,
}

impl PairRecord {
    /// Whether this record reaches null-edge insertion: the one test
    /// [`super::insert::plan_null_pairs`] filters its survivors by,
    /// and the reduction reads to tell a vertex pair that crosses from
    /// one that only touches.
    pub(super) const fn survives(&self) -> bool {
        self.intersect
    }
}

/// Whether `dir` lies within the convex sector (Zero grazes count —
/// module docs). `strict` demands definite interior. `read` is what the
/// calling door read of the pair's declaration: the primitive takes
/// none of its own, and no declaration settles a direction's membership.
///
/// Sense-invariant given the sector: `start`/`end` are traversal-
/// derived and `normal` already carries the sense, and `revert` flips
/// both together — a second sense fold here would cancel
/// [`sector_face`]'s and turn every membership test inside out.
pub(super) fn within<T: Decide>(
    s: &BoolSector<T>,
    dir: Vec3<T>,
    strict: bool,
    read: DeclarationRead,
    band: Band,
) -> Result<bool, BooleanError> {
    let c1 = Margin::levered(s.start.cross(dir).dot(s.normal.vec()), s.arm);
    let c2 = Margin::levered(dir.cross(s.end).dot(s.normal.vec()), s.arm);
    let escalate = |diag| BooleanError::coincidence(Coincide::Sectors, read, diag);
    let t1 = decide("bool_sector_within", c1, band).map_err(escalate)?;
    let t2 = decide("bool_sector_within", c2, band).map_err(escalate)?;
    Ok(if strict {
        t1 == Sign::Positive && t2 == Sign::Positive
    } else {
        t1 != Sign::Negative && t2 != Sign::Negative
    })
}

/// The sector's bounds that are edges of its face, with their unit
/// directions: `end` is `s.he`'s edge, `start` the next orbit
/// half-edge's. A subdivision bisector is no edge.
pub(super) fn bound_edges<T: Decide>(
    body: &Body<T>,
    s: &BoolSector<T>,
) -> Vec<(crate::entity::EdgeKey, Vec3<T>)> {
    let edge = |he| proven(&body.half_edges, he, EntityId::HalfEdge).edge;
    let mut out = Vec::new();
    if s.end_edge() {
        out.push((edge(s.he), s.end));
    }
    if s.start_edge() {
        out.push((edge(body.proven_orbit_step(s.he)), s.start));
    }
    out
}

/// **The one fold rule for an on-bound**: an edge read On against the
/// partner face, between bounds read `before` and `after`, joins the In
/// run unless both neighbours read Out. Both classifiers call it — the
/// vertex-on-face resolution of its entries and the vertex-vertex
/// attribution of an edge-sector crossing — so at every site, in every
/// op, a section segment along the edge is attributed to the flanking
/// sector on the Out side, and the two ends of the segment put their
/// null edges into the same face. The one exception is an edge-edge
/// germ along an edge that two crossing pairs at one vertex both cross
/// along: it folds Out in both, at both of the edge's ends, each of
/// which is a v-v site of both pairs once the reduction has split the
/// edges there ([`super::recl::Reversed`]).
pub(super) fn fold_on_bound(before: SideCode, after: SideCode) -> SideCode {
    match (before, after) {
        (SideCode::Out, SideCode::Out) => SideCode::Out,
        _ => SideCode::In,
    }
}

/// One of an on-bound's two flanking sectors, in orbit order: `Before`
/// holds the bound as its START, `After` as its END.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Flank {
    /// The flanker holding the on-bound as its start.
    Before,
    /// The flanker holding the on-bound as its end.
    After,
}

/// **Where [`fold_on_bound`] puts the crossing** at an on-bound whose
/// flankers' other bounds read `before` and `after`: on the flanker
/// whose key is not the fold, since the on-bound joins the run its
/// fold names and the transition lies across that flanker. `None` when
/// both keys read the fold: the run goes on through the bound and
/// nothing crosses there. Every attribution of a crossing along an
/// edge calls this: the vertex-vertex edge-sector and edge-edge
/// resolutions ([`super::recl`]).
pub(super) fn crossing_flank(before: SideCode, after: SideCode) -> Option<Flank> {
    let fold = fold_on_bound(before, after);
    match (before == fold, after == fold) {
        (true, false) => Some(Flank::After),
        (false, true) => Some(Flank::Before),
        _ => None,
    }
}

/// One operand's side of a germ: the sector the germ was attributed to
/// at `site` of `body`, and its bounds' readings against the partner
/// face as first read, before any rewrite.
#[derive(Clone, Copy)]
pub(super) struct GermSide<'a, T: geom_core::Real> {
    pub body: &'a Body<T>,
    pub operand: Operand,
    pub site: VertexKey,
    pub sector: &'a BoolSector<T>,
    pub read: (SideCode, SideCode),
}

/// What the reduction recorded at the far end of an edge leaving a
/// site, against the partner, in rank order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Touch {
    /// No contact: the edge leaves the partner.
    Apart,
    /// A vertex pair: the end lies on the partner's boundary.
    Boundary,
    /// The end lies inside a face of the partner.
    Face,
}

/// A bound of a germ's sector read On: the edge, and its far end.
#[derive(Clone, Copy)]
struct Along {
    edge: crate::entity::EdgeKey,
    far: VertexKey,
}

/// **The cell a germ lies in** ([`super::Locus`]) on one operand, where
/// the partner holds the germ inside a face. The germ ray is the
/// intersection of the sector's face with the partner face, so a bound
/// read On is tangent to it. That bound's edge is the germ's cell only
/// when the segment runs along it, and the reduction says when it does
/// not: it splits an edge at every crossing of the partner, so an edge
/// lying on the partner ends where it records a touch. An edge whose far
/// end it records none at leaves the partner, the germ is only tangent
/// to it, and it stays inside the sector's face, as it does under a
/// bisector or with no bound On. Called by every site kind, before that
/// site's surgery moves the orbit the bounds are read from.
pub(super) fn germ_locus<T: Decide>(
    side: GermSide<'_, T>,
    contacts: &super::ContactRecords,
) -> Result<super::Locus, BooleanError> {
    let in_face = super::Locus::InFace(side.sector.face);
    Ok(match along(side)? {
        Some(e) if touch(side, e.far, contacts)? != Touch::Apart => super::Locus::OnEdge(e.edge),
        _ => in_face,
    })
}

/// **Both operands' cells of a germ at a vertex pair** ([`germ_locus`]
/// on each). Where both sectors hold a bound read On, the two edges
/// leave the site together: they are one segment when the reduction
/// paired their far ends ([`one_segment`]), and otherwise they part,
/// and the segment runs along
/// at most one of them: the one whose far end lies deeper in the
/// partner by what the reduction recorded there ([`Touch`]), or, where
/// both are recorded alike, the one whose curve runs inside the
/// partner's face ([`runs_in`]). The other germ is only tangent to its
/// edge, and lies in the face its curve enters ([`tangent_face`]). Two
/// edges recorded alike that the face test does not separate stay
/// `OnEdge` both.
pub(super) fn germ_loci<T: Decide>(
    a: GermSide<'_, T>,
    b: GermSide<'_, T>,
    contacts: &super::ContactRecords,
    band: Band,
) -> Result<(super::Locus, super::Locus), BooleanError> {
    use super::Locus::{InFace, OnEdge};
    let (Some(ea), Some(eb)) = (along(a)?, along(b)?) else {
        return Ok((germ_locus(a, contacts)?, germ_locus(b, contacts)?));
    };
    let (far_a, far_b) = (site_of(a.body, ea.far)?, site_of(b.body, eb.far)?);
    let paired = contacts
        .vv
        .iter()
        .any(|c| far_a.contains(&c.a) && far_b.contains(&c.b));
    if paired {
        return one_segment(a, ea, b, eb);
    }
    let tangent = |side: GermSide<'_, T>, partner: GermSide<'_, T>, e: Along| {
        Ok::<_, BooleanError>(InFace(
            tangent_face(side, partner, e.edge, contacts)?.unwrap_or(side.sector.face),
        ))
    };
    let (ta, tb) = (touch(a, ea.far, contacts)?, touch(b, eb.far, contacts)?);
    Ok(match ta.cmp(&tb) {
        core::cmp::Ordering::Greater => (OnEdge(ea.edge), tangent(b, a, ea)?),
        core::cmp::Ordering::Less => (tangent(a, b, eb)?, OnEdge(eb.edge)),
        core::cmp::Ordering::Equal if ta == Touch::Apart => {
            (InFace(a.sector.face), InFace(b.sector.face))
        }
        core::cmp::Ordering::Equal => {
            match (runs_in(a, ea, b, eb, band)?, runs_in(b, eb, a, ea, band)?) {
                (Some(true), Some(false)) => (OnEdge(ea.edge), tangent(b, a, ea)?),
                (Some(false), Some(true)) => (tangent(a, b, eb)?, OnEdge(eb.edge)),
                _ => (OnEdge(ea.edge), OnEdge(eb.edge)),
            }
        }
    })
}

/// The certified curve of a germ edge.
fn germ_curve<T: Decide>(
    body: &Body<T>,
    edge: crate::entity::EdgeKey,
) -> Result<&geom_brep::EdgeCurve<T>, BooleanError> {
    body.edge_curve_linked(edge, proven(&body.edges, edge, EntityId::Edge))
        .certified()
        .ok_or(BooleanError::ClassificationInvariant {
            what: "a germ edge is null scaffolding",
        })
}

/// **Two edges leaving a site along the germ, with paired far ends,
/// are one segment** when both are lines or circles. Each is tangent to
/// the germ at the site, and a line or circle meets a line or circle
/// tangent to it there at no other point unless the two are one curve,
/// so the paired far ends make them one; the sweep splits an edge at
/// every crossing of the partner, so neither passes the other's far
/// end. This is structure, not a test: no reading of the two curves
/// could answer otherwise. A conic or spline can meet such a partner
/// again, and its pair is refused
/// ([`BooleanError::GermEdgeCarrierUnsupported`]).
fn one_segment<T: Decide>(
    a: GermSide<'_, T>,
    ea: Along,
    b: GermSide<'_, T>,
    eb: Along,
) -> Result<(super::Locus, super::Locus), BooleanError> {
    for (side, e) in [(a, ea), (b, eb)] {
        match germ_curve(side.body, e.edge)?.carrier() {
            geom::Curve3::Line { .. } | geom::Curve3::Circle { .. } => {}
            _ => {
                return Err(BooleanError::GermEdgeCarrierUnsupported {
                    operand: side.operand,
                    edge: e.edge,
                });
            }
        }
    }
    Ok((super::Locus::OnEdge(ea.edge), super::Locus::OnEdge(eb.edge)))
}

/// Whether the edge `e` of `side`, past the site, runs inside the face
/// of `partner` the germ runs along: the face across the partner's own
/// edge `pe` from its sector's face. Its midpoint is decided on that
/// face's plane and inside its trim
/// ([`super::solid_contain::point_in_face`]); the sweep splits an edge
/// at every crossing of the partner, so the midpoint speaks for the
/// whole edge. `None`: undecided — a midpoint on the face's boundary, a
/// face that is not a plane, or no single face across `pe`. A trim the
/// walk cannot read is refused.
fn runs_in<T: Decide>(
    side: GermSide<'_, T>,
    e: Along,
    partner: GermSide<'_, T>,
    pe: Along,
    band: Band,
) -> Result<Option<bool>, BooleanError> {
    let pb = partner.body;
    let edge = proven(&pb.edges, pe.edge, EntityId::Edge);
    let mut faces = Vec::new();
    for h in [edge.he_plus, edge.he_minus] {
        let f = pb.face_of_linked(h);
        if f != partner.sector.face {
            faces.push(f);
        }
    }
    let [face] = faces[..] else {
        return Ok(None);
    };
    let (origin, normal) = match super::solid_contain::face_plane(pb, face) {
        Ok(plane) => plane,
        Err(super::solid_contain::PointInSolidError::KindUnsupported { .. }) => return Ok(None),
        Err(e) => return Err(BooleanError::Containment(e)),
    };
    let p = germ_curve(side.body, e.edge)?.mid_point();
    let height = Margin::of((p - origin).dot(normal));
    match decide("bool_germ_tie_plane", height, band).map_err(|diag| {
        BooleanError::coincidence(Coincide::EdgeOnPlane, DeclarationRead::Moot, diag)
    })? {
        Sign::Zero => {}
        Sign::Positive | Sign::Negative => return Ok(Some(false)),
    }
    super::solid_contain::point_in_face(pb, face, normal, p, band)
        .map_err(BooleanError::Containment)
}

/// The bound of the germ's sector read On against the partner face, when
/// that bound is an edge of the face.
fn along<T: Decide>(side: GermSide<'_, T>) -> Result<Option<Along>, BooleanError> {
    let (body, s, read) = (side.body, side.sector, side.read);
    let on_start = read.0 == SideCode::On && s.start_edge();
    let on_end = read.1 == SideCode::On && s.end_edge();
    let he = match (on_start, on_end) {
        (false, false) => return Ok(None),
        (false, true) => s.he,
        (true, false) => body.proven_orbit_step(s.he),
        (true, true) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "a germ sector with both edge bounds on the partner face",
            });
        }
    };
    Ok(Some(Along {
        edge: proven(&body.half_edges, he, EntityId::HalfEdge).edge,
        far: body.proven_half_edge_end(he),
    }))
}

/// The site of `vertex` ([`crate::chord_join::null_site`]), a stale
/// site vertex refused ([`stale_site`]).
fn site_of<T: Decide>(body: &Body<T>, vertex: VertexKey) -> Result<Vec<VertexKey>, BooleanError> {
    crate::chord_join::null_site(body, &[vertex]).map_err(stale_site)
}

/// The boolean's refusal of a site vertex that no longer resolves
/// ([`StaleSite`]): the null edges are the classification's, so a stale
/// key or copy among them breaks its invariant, wherever the reduction
/// reads the site.
pub(super) fn stale_site(_: StaleSite) -> BooleanError {
    BooleanError::ClassificationInvariant {
        what: StaleSite::WHAT,
    }
}

/// What the reduction recorded at the site of `far`, a vertex of the
/// germ side's operand, against the partner.
fn touch<T: Decide>(
    side: GermSide<'_, T>,
    far: VertexKey,
    contacts: &super::ContactRecords,
) -> Result<Touch, BooleanError> {
    let site = site_of(side.body, far)?;
    Ok(
        if on_faces(contacts, side.operand)
            .iter()
            .any(|c| site.contains(&c.vertex))
        {
            Touch::Face
        } else if contacts
            .vv
            .iter()
            .any(|c| site.contains(&vv_sides(c, side.operand).0))
        {
            Touch::Boundary
        } else {
            Touch::Apart
        },
    )
}

/// The recorded contacts of `operand`'s vertices on the partner's faces.
fn on_faces(contacts: &super::ContactRecords, operand: Operand) -> &[super::VfContact] {
    match operand {
        Operand::A => &contacts.a_on_b,
        Operand::B => &contacts.b_on_a,
    }
}

/// A vertex pair's two vertices, `operand`'s first.
fn vv_sides(c: &super::VvContact, operand: Operand) -> (VertexKey, VertexKey) {
    match operand {
        Operand::A => (c.a, c.b),
        Operand::B => (c.b, c.a),
    }
}

/// **The face a tangent germ's curve enters**: the partner's germ runs
/// along its edge `along`, so the segment is that edge, and it lies in
/// the face of `side`'s body holding both of its ends, the site and
/// where the reduction put `along`'s far end. `None` when no single face
/// holds both.
fn tangent_face<T: Decide>(
    side: GermSide<'_, T>,
    partner: GermSide<'_, T>,
    along: crate::entity::EdgeKey,
    contacts: &super::ContactRecords,
) -> Result<Option<FaceKey>, BooleanError> {
    let edge = proven(&partner.body.edges, along, EntityId::Edge);
    let (u, v) = (
        proven(&partner.body.half_edges, edge.he_plus, EntityId::HalfEdge).start,
        proven(&partner.body.half_edges, edge.he_minus, EntityId::HalfEdge).start,
    );
    let near = site_of(partner.body, partner.site)?;
    let Some(far) = [u, v].into_iter().find(|w| !near.contains(w)) else {
        return Ok(None);
    };
    let far_site = site_of(partner.body, far)?;
    let mut far_faces: Vec<FaceKey> = on_faces(contacts, partner.operand)
        .iter()
        .filter(|c| far_site.contains(&c.vertex))
        .map(|c| c.face)
        .collect();
    for c in &contacts.vv {
        let (mine, theirs) = vv_sides(c, side.operand);
        if far_site.contains(&theirs) {
            far_faces.extend(faces_at(side.body, mine).map_err(stale_site)?);
        }
    }
    let at_site = faces_at(side.body, side.site).map_err(stale_site)?;
    Ok(super::fragments::sole_common_face(&at_site, &far_faces))
}

/// The faces around a site of `body` (every copy null edges tie
/// `vertex` to), deduplicated, null faces skipped. With no null edges
/// at the site, the faces around `vertex` itself. An isolated vertex
/// (a pierce-ring vertex joined to nothing) contributes none.
/// [`StaleSite`]: a site vertex that does not resolve
/// ([`crate::chord_join::null_site`]); every hop past a site vertex
/// that resolves is a link.
pub(super) fn faces_at<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
) -> Result<Vec<FaceKey>, StaleSite> {
    let mut out = Vec::new();
    for v in crate::chord_join::null_site(body, &[vertex])? {
        for he in body.vertex_orbit_linked(v) {
            let edge = proven(&body.half_edges, he, EntityId::HalfEdge).edge;
            let data = linked(
                &body.edges,
                edge,
                EntityId::Edge,
                EntityId::HalfEdge(he),
                "edge",
            );
            if body.edge_curve_linked(edge, data).null_scaffold().is_some() {
                continue;
            }
            let f = body.face_of_linked(he);
            if !out.contains(&f) {
                out.push(f);
            }
        }
    }
    Ok(out)
}

/// Whether `dir`, coplanar with `s`, runs into its face: within the
/// sector and along neither of its bounds that is an edge of the face.
pub(super) fn runs_into<T: Decide>(
    s: &BoolSector<T>,
    dir: Vec3<T>,
    arm: T,
    band: Band,
) -> Result<bool, BooleanError> {
    if !within(s, dir, false, DeclarationRead::Moot, band)? {
        return Ok(false);
    }
    Ok(!(s.start_edge() && parallel_same(dir, s.start, arm, band)?
        || s.end_edge() && parallel_same(dir, s.end, arm, band)?))
}

/// Same-direction parallelism of two bound directions (unit-ish).
fn parallel_same<T: Decide>(
    u: Vec3<T>,
    v: Vec3<T>,
    arm: T,
    band: Band,
) -> Result<bool, BooleanError> {
    let cross_margin = Margin::levered(u.cross(v).norm(), arm);
    match decide("bool_dir_parallel", cross_margin, band) {
        Ok(Sign::Zero) => {}
        Ok(_) => return Ok(false),
        Err(diag) => {
            return Err(BooleanError::coincidence(
                Coincide::Sectors,
                DeclarationRead::Moot,
                diag,
            ));
        }
    }
    direction_sense(u, v, arm, band)
}

/// Whether two directions read parallel point the same way (`true`)
/// or opposite ways, their cosine levered at `arm`
/// ([`BooleanDecision::DirectionSense`]). A decided zero refuses with
/// its decided margin, as the in-band arm does: both say the arm is
/// too short to tell.
pub(super) fn direction_sense<T: Decide>(
    u: Vec3<T>,
    v: Vec3<T>,
    arm: T,
    band: Band,
) -> Result<bool, BooleanError> {
    match crate::validate::decide_nonzero("bool_dir_same", Margin::levered(u.dot(v), arm), band) {
        Ok(NonzeroSign::Positive) => Ok(true),
        Ok(NonzeroSign::Negative) => Ok(false),
        Err(diag) => Err(BooleanError::Escalated {
            decision: BooleanDecision::DirectionSense,
            diag,
        }),
    }
}

/// `sectoroverlap` — coplanar sectors overlap test (module docs).
fn sector_overlap<T: Decide>(
    a: &BoolSector<T>,
    b: &BoolSector<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    for (s, dir) in [(a, b.start), (a, b.end), (b, a.start), (b, a.end)] {
        if within(s, dir, true, DeclarationRead::Moot, band)? {
            return Ok(true);
        }
    }
    let arm = a.arm.min(b.arm);
    // Identical region (same or crossed bound pairing).
    let straight =
        parallel_same(a.start, b.start, arm, band)? && parallel_same(a.end, b.end, arm, band)?;
    let crossed =
        parallel_same(a.start, b.end, arm, band)? && parallel_same(a.end, b.start, arm, band)?;
    Ok(straight || crossed)
}

/// A sector bound's side code against a face's PLANE ([`side_code`]
/// with [`NO_CURVATURE`]): what the sector passes read a bound against.
fn plane_side_code<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    face_normal: OutwardNormal<T>,
    band: Band,
) -> Result<SideCode, BooleanError> {
    side_code(dir, reach, face_normal, NO_CURVATURE(), band)
}

/// One sector's two side codes, `(start, end)`.
type SidePair = (SideCode, SideCode);

/// The four side codes of a sector pair: each sector's bounds against
/// the other's face.
fn pair_codes<T: Decide>(
    sa: &BoolSector<T>,
    sb: &BoolSector<T>,
    band: Band,
) -> Result<(SidePair, SidePair), BooleanError> {
    let code = |dir, reach, normal| plane_side_code(dir, reach, normal, band);
    Ok((
        (
            code(sa.start, sa.start_reach, sb.normal)?,
            code(sa.end, sa.end_reach, sb.normal)?,
        ),
        (
            code(sb.start, sb.start_reach, sa.normal)?,
            code(sb.end, sb.end_reach, sa.normal)?,
        ),
    ))
}

/// The all-pairs search (15.7): every intersecting (A-sector, B-sector)
/// pair becomes a [`PairRecord`] with its four side codes, in
/// deterministic A-major order.
pub(super) fn pair_search<T: Decide>(
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    band: Band,
) -> Result<Vec<PairRecord>, BooleanError> {
    let mut records = Vec::new();
    for (i, sa) in a_sectors.iter().enumerate() {
        for (j, sb) in b_sectors.iter().enumerate() {
            let int = sa.normal.vec().cross(sb.normal.vec());
            let arm = sa.arm.min(sb.arm);
            // Parallel normals at the shorter arm make the pair a
            // near-coincidence at this vertex: the bound that sets the
            // arm reads at most `arm·|n_a × n_b|` off the other plane, so
            // it is On (or in band, which escalates), whatever a farther
            // bound does. Such a pair goes to the carrier ladder as a
            // coincidence, every code On: undeclared, the ladder
            // refuses it; declared, the door has verified it. Its bounds'
            // readings are not consulted — a far bound reading off would
            // make the record half a crossing, which no germ pairing
            // closes.
            // A norm levered by a minimum of chord norms: a magnitude.
            let coplanar = match geom_core::k_stats::decide_magnitude(
                "bool_faces_parallel",
                Margin::levered(int.norm(), arm),
                band,
            ) {
                Ok(Magnitude::Zero) => true,
                Ok(Magnitude::Positive) => false,
                Err(diag) => {
                    return Err(BooleanError::coincidence(
                        Coincide::Sectors,
                        DeclarationRead::Moot,
                        diag,
                    ));
                }
            };
            let hit = if coplanar {
                sector_overlap(sa, sb, band)?
            } else {
                let d = int.normalize();
                let moot = DeclarationRead::Moot;
                (within(sa, d, false, moot, band)? && within(sb, d, false, moot, band)?)
                    || (within(sa, -d, false, moot, band)? && within(sb, -d, false, moot, band)?)
            };
            if !hit {
                continue;
            }
            let on = (SideCode::On, SideCode::On);
            let (sa_codes, sb_codes) = if coplanar {
                (on, on)
            } else {
                pair_codes(sa, sb, band)?
            };
            records.push(PairRecord {
                a: i,
                b: j,
                sa: sa_codes,
                sb: sb_codes,
                intersect: true,
            });
        }
    }
    Ok(records)
}

/// What [`wedge_classes`] read: each edge's class, and which side of
/// the other body's boundary there is its cone. The cone is the
/// material where `met` and the material's complement where not.
///
/// `pointed` says which side was read as the cone: the convex side of
/// a convex or hollow corner, and for a polygon cone the side `−c` does
/// not lie in ([`cone_read`]). It is not a half-space test: a saddle
/// whose mean bound has a decided length reads pointed too. Where the
/// link lies in an open half-space, the side so read is the one in it;
/// a touch's partner's link does, since `vtxfac::partner_side` refuses
/// any other first, which is why only a touch reads `pointed`. Where it
/// is false, the cone is the material.
#[derive(Clone, Debug)]
pub(super) struct WedgeRead {
    pub met: bool,
    pub pointed: bool,
    pub rows: Vec<(HalfEdgeKey, SideCode)>,
}

/// **Each edge of `own`'s orbit, classified against the other
/// operand's closed body at the same point** (`BooleanReduction::edge_classes`):
/// where its neighbourhood there is a wedge — `other`'s sectors lie on
/// two faces, so the point lies inside an edge of that body — or a
/// corner of three or more faces, and `None` where it is neither. A
/// corner neither convex nor hollow is read as a polygon cone
/// ([`cone_read`]).
///
/// A wedge is the two faces' inner half-spaces, met where the edge
/// between them is convex and joined where it is reflex. Which one is
/// read off one face's subdivision bisector, a direction inside it,
/// against the other face's plane; a bisector on that plane is two
/// faces tangent along the edge, whose half-spaces agree there and are
/// read as convex. A wedge with no bisector to read is not classified.
/// A corner of three or more faces is convex when every bound of every
/// sector lies behind or on every other face's plane, and is then the
/// faces' inner half-spaces met; it is hollow when every bound lies in
/// front of or on every other face's plane, its complement then convex,
/// and is the faces' inner half-spaces joined.
///
/// A direction's class is its side codes against the faces combined:
/// met, `Out` past any face, else `On` on any, else `In`; joined, `In`
/// behind any face, else `On` on any, else `Out`.
pub(super) fn wedge_classes<T: Decide>(
    own: &[BoolSector<T>],
    other: &[BoolSector<T>],
    band: Band,
) -> Result<Option<WedgeRead>, BooleanError> {
    let mut faces: Vec<(FaceKey, OutwardNormal<T>)> = Vec::new();
    for s in other {
        if !faces.iter().any(|&(f, _)| f == s.face) {
            faces.push((s.face, s.normal));
        }
    }
    let code = |dir, reach, normal| plane_side_code(dir, reach, normal, band);
    let (met, hollow) = match faces.as_slice() {
        [(f0, _), (_, n1)] => {
            let inside =
                other
                    .iter()
                    .find_map(|s| match (s.face == *f0, s.start_reach, s.end_reach) {
                        (false, _, _) => None,
                        (true, Reach::Bisector(_), _) => Some((s.start, s.start_reach)),
                        (true, _, Reach::Bisector(_)) => Some((s.end, s.end_reach)),
                        (true, _, _) => None,
                    });
            let Some((dir, reach)) = inside else {
                return Ok(None);
            };
            (code(dir, reach, *n1)? != SideCode::Out, false)
        }
        [_, _, _, ..] => {
            // Convex while no bound lies past another face's plane, and
            // its complement convex while none lies behind one. A
            // reading in band refuses only while the corner may still be
            // convex; past that the corner is read as a polygon cone.
            let (mut convex, mut reflex) = (true, true);
            for s in other {
                for &(f, n) in &faces {
                    if f == s.face {
                        continue;
                    }
                    for (dir, reach) in [(s.start, s.start_reach), (s.end, s.end_reach)] {
                        match code(dir, reach, n) {
                            Ok(SideCode::Out) => convex = false,
                            Ok(SideCode::In) => reflex = false,
                            Ok(SideCode::On) => {}
                            Err(e) if convex => return Err(e),
                            Err(_) => return cone_read(own, other, band),
                        }
                        if !convex && !reflex {
                            return cone_read(own, other, band);
                        }
                    }
                }
            }
            // A corner both ways at once (every bound on every plane)
            // reads as convex, as it did before hollow corners were read.
            match (convex, reflex) {
                (true, _) => (true, false),
                (false, true) => (false, true),
                (false, false) => return cone_read(own, other, band),
            }
        }
        _ => return Ok(None),
    };
    let (wins, loses) = if met {
        (SideCode::Out, SideCode::In)
    } else {
        (SideCode::In, SideCode::Out)
    };
    let mut rows = Vec::new();
    for s in own.iter().filter(|s| s.end_edge()) {
        let mut codes = Vec::with_capacity(faces.len());
        for &(_, n) in &faces {
            match code(s.end, s.end_reach, n) {
                Ok(c) => codes.push(c),
                Err(_) if hollow => return cone_read(own, other, band),
                Err(e) => return Err(e),
            }
        }
        let class = if codes.contains(&wins) {
            wins
        } else if codes.contains(&SideCode::On) {
            SideCode::On
        } else {
            loses
        };
        rows.push((s.he, class));
    }
    Ok(Some(WedgeRead {
        met,
        pointed: true,
        rows,
    }))
}

/// **Each edge of `own`'s orbit against `other`'s corner read as a
/// polygon cone**: the corner's faces bound its material whatever its
/// shape, a dart's apex (a reflex edge) or a saddle as much as a convex
/// corner, and each edge is read by [`cone_side`]. `None` where some
/// edge reads `None` there: no reference reads it and none escalated
/// (every reference's arc runs through the link, meets a face whose
/// crossing it cannot locate, or may cross a face inside its sector
/// without decidedly crossing its plane, and does not lie decidedly
/// apart from that face's sector).
///
/// Which side of the corner's link is its cone is read off the link's
/// mean direction `c` (the mean of its sectors' unit bounds): the side
/// `−c` does not lie in, the material where `−c` reads `Out` and its
/// complement where `−c` reads `In`. Where the link lies in an open
/// half-space, one side of it does too, and `−c` lies strictly outside
/// that side, since every bound leans into the half-space and `c` with
/// them, so the cone is that side. Where `c` is no decided length, or
/// `−c` reads on the link, refuses or reads nothing, the cone is the
/// material (`pointed` false); a touch refuses beside such a partner,
/// and pairs alone read the same under either labelling.
pub(super) fn cone_read<T: Decide>(
    own: &[BoolSector<T>],
    other: &[BoolSector<T>],
    band: Band,
) -> Result<Option<WedgeRead>, BooleanError> {
    let mut rows = Vec::new();
    for s in own.iter().filter(|s| s.end_edge()) {
        match cone_side(s.end, s.end_reach, other, band)? {
            Some(class) => rows.push((s.he, class)),
            None => return Ok(None),
        }
    }
    let Some(arm) = other.iter().map(|s| s.arm).reduce(|a, b| a.min(b)) else {
        return Ok(None);
    };
    let sum = other
        .iter()
        .fold(Vec3::zero(), |acc, s| acc + s.start + s.end);
    let mean = sum / T::from_f64(2.0 * other.len() as f64);
    let away = match decide("bool_cone_pointed", Margin::levered(mean.norm(), arm), band) {
        // An in-band `−c` labels nothing, as no side read does: the cone
        // is the material.
        Ok(Sign::Positive) => {
            cone_side(-mean, Reach::Bisector(arm), other, band).unwrap_or_default()
        }
        Ok(Sign::Zero | Sign::Negative) | Err(_) => None,
    };
    let (met, pointed) = match away {
        Some(SideCode::Out) => (true, true),
        Some(SideCode::In) => (false, true),
        _ => (true, false),
    };
    Ok(Some(WedgeRead { met, pointed, rows }))
}

/// **A direction's side of the material a polygon cone bounds**: `dir`,
/// read as `reach` allows, against `other`'s sectors, each a face's
/// corner of under 180°.
///
/// A direction on a face's plane within its sector ([`in_sector`]) is
/// `On`. Any other is read along the great arc to a reference `p`, the
/// direction halfway between one sector's bounds, where the direction
/// lies strictly to one side of that sector's plane: the side starts as
/// the direction's side of that face, and flips at each sector the arc
/// crosses. Each other face `S` is read by one rule, from the
/// direction's side of its plane and `p`'s:
/// - **Decidedly on one side**, both: `S` is not crossed.
/// - **Decidedly on opposite sides**: the arc crosses `S`'s plane once,
///   and crosses `S` iff `S`'s bounds lie on opposite sides of the plane
///   through the direction and `p`, the start on `p`'s side
///   ([`arc_side`]; two minor arcs `d→p` and `u→v` cross iff
///   `det(d,p,u) = −det(d,p,v) = −det(u,v,d) = det(u,v,p)`, all nonzero).
/// - **`p` in band of the plane**: the reference is passed over.
/// - **Otherwise**: the crossing, if any, is located ([`crossing`]), and
///   `S` is passed over only where [`in_sector`] decides it outside
///   `S`'s sector. Anywhere else, or where nothing is located, the
///   reference is passed over.
///
/// Where a rule would pass the reference over, `S` is read as not
/// crossed instead if the arc and `S`'s sector lie decidedly apart in
/// `S`'s plane ([`apart`]): wherever the arc meets that plane, the point
/// lies outside the sector.
///
/// `S` on `p`'s own face is not read: `p` lies on that plane in its own
/// sector, which no other sector of the face shares.
///
/// Every decision is a point deviation ([`least_lever`]). A reference
/// any of whose readings is zero or in band is passed over for the
/// next; where none reads, the first escalation refuses. `None` is
/// returned where no reference reads and none escalated: every
/// reference's arc runs through the link, or meets a face whose
/// crossing it cannot locate or that it may cross inside its sector
/// without decidedly crossing its plane, and from whose sector it does
/// not lie decidedly apart.
fn cone_side<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    other: &[BoolSector<T>],
    band: Band,
) -> Result<Option<SideCode>, BooleanError> {
    let d = dir.normalize();
    // A face's strict side, or `None` where the direction lies on or in
    // band of its plane outside its sector.
    let mut codes = Vec::with_capacity(other.len());
    for s in other {
        // On the plane, within the sector or on its bound, is on the face.
        let on_face = || in_sector(s, d, reach.length(), band);
        codes.push(match plane_side_code(dir, reach, s.normal, band) {
            Ok(SideCode::On) if on_face()? => return Ok(Some(SideCode::On)),
            Ok(SideCode::On) => None,
            Ok(code) => Some(code),
            Err(e) if on_face()? => return Err(e),
            Err(_) => None,
        });
    }
    let mut escalation = None;
    'reference: for (s0, base) in other.iter().zip(&codes) {
        let Some(base) = *base else { continue };
        let p = (s0.start + s0.end).normalize();
        let p_reach = Reach::Bisector(s0.arm);
        let mut held = base == SideCode::In;
        for (s, code) in other.iter().zip(&codes) {
            if s.face == s0.face {
                continue;
            }
            let arc = GreatArc {
                dir,
                reach,
                p,
                p_arm: s0.arm,
            };
            let side = match plane_side_code(p, p_reach, s.normal, band) {
                Ok(side) => side,
                Err(_) if apart(s, arc, band) => continue,
                Err(e) => {
                    escalation.get_or_insert(e);
                    continue 'reference;
                }
            };
            // Endpoints decidedly on one side: the arc stays there.
            // Decidedly on opposite sides: the crossing is read through
            // its bounds' determinants below. Otherwise the crossing, if
            // any, is located from the endpoints' heights over the plane,
            // and `S` is passed over only where that point is decidedly
            // outside its sector, read at how well it is located.
            let code = match (side, *code) {
                (SideCode::In | SideCode::Out, Some(code)) if code == side => continue,
                (SideCode::In | SideCode::Out, Some(code)) => code,
                _ => match crossing(d, reach, p, s0.arm, s.normal.vec())
                    .map(|(x, lever)| in_sector(s, x, lever, band))
                {
                    Some(Ok(false)) => continue,
                    _ if apart(s, arc, band) => continue,
                    Some(Err(e)) => {
                        escalation.get_or_insert(e);
                        continue 'reference;
                    }
                    Some(Ok(true)) | None => continue 'reference,
                },
            };
            let mut crosses = true;
            for (bound, bound_reach, want) in
                [(s.start, s.start_reach, side), (s.end, s.end_reach, code)]
            {
                match arc_side(arc, bound, bound_reach, s.arm, band) {
                    Ok(Some(got)) => crosses &= got == want,
                    _ if apart(s, arc, band) => {
                        crosses = false;
                        break;
                    }
                    Ok(None) => continue 'reference,
                    Err(e) => {
                        escalation.get_or_insert(e);
                        continue 'reference;
                    }
                }
            }
            held ^= crosses;
        }
        return Ok(Some(if held { SideCode::In } else { SideCode::Out }));
    }
    escalation.map_or(Ok(None), Err)
}

/// **The least deviation that flips a reading, as its lever.** A
/// reading's value is a sine, cosine or determinant of unit vectors,
/// each the direction to a point at some reach from the vertex. Moving
/// that point by `δ` turns its unit vector by at most `δ/L`, which
/// moves the value by at most `rate·δ/L`, `rate` the value's rate in
/// that vector's turn. Every point moved by `δ` at once moves it by at
/// most `δ·Σ(rate/L)`, so the least joint move that can flip the
/// value's sign is `|value|/Σ(rate/L)` (D4), and this returns that
/// `1/Σ(rate/L)` over `points`, each `(L, rate)`, `L` a positive reach
/// and `rate` a non-negative sine. A zero rate adds nothing: no move of
/// that point flips the reading.
///
/// `None` where every rate is zero: no move of any point flips the
/// reading. Its callers always read a point with a nonzero rate (the
/// read direction itself), so they do not meet it; they read it, were
/// it met, as a zero lever, which refuses rather than decides.
/// [`arc_side`], [`in_sector`] and [`crossing`] lever here.
fn least_lever<T: Decide>(points: impl IntoIterator<Item = (T, T)>) -> Option<T> {
    let one = T::from_f64(1.0);
    let rate = points
        .into_iter()
        .fold(T::from_f64(0.0), |sum, (length, rate)| sum + rate / length);
    geom_core::is_finite_length(one / rate).then(|| one / rate)
}

/// Where the arc from the unit direction `d`, read as `reach` allows,
/// to the unit reference `p`, levered at `arm`, meets the plane through
/// the vertex of normal `n`, if it does: `x = |p·n|·d + |d·n|·p`, the
/// point the ends' heights over the plane locate (on the arc, at the
/// ratio of the heights), and the lever it is read at.
///
/// The lever is the direction's reach or, if less, the joint least
/// deviation of the two points `x` is located from ([`least_lever`]):
/// moving `p`'s point at its arm by `δ` changes `|p·n|` by `δ/arm`, and
/// moving the direction's far point changes `|d·n|` by `δ/L_d`; either
/// slides `x` along the arc by that over `|x|`, a rate of `1/|x|` at
/// each, so `|x|·arm·L_d/(arm + L_d)`. `None` where `x` has no length:
/// both ends lie on the plane, and the arc locates no crossing.
fn crossing<T: Decide>(
    d: Vec3<T>,
    reach: Reach<T>,
    p: Vec3<T>,
    arm: T,
    n: Vec3<T>,
) -> Option<(Vec3<T>, T)> {
    let x = d * p.dot(n).abs() + p * d.dot(n).abs();
    let l_d = reach.length();
    let rate = T::from_f64(1.0) / x.norm();
    if !geom_core::is_finite_length(rate) {
        return None;
    }
    let located = least_lever([(arm, rate), (l_d, rate)])?;
    Some((x, l_d.min(located)))
}

/// Whether a direction on or beside `s`'s plane lies within its
/// sector, read as a point `lever` out along it: `false` where it lies
/// decidedly past one of the planes through each bound square to the
/// face (`"bool_cone_within"`, the sine of its angle past it) or
/// decidedly faces away from the sector's middle (`"bool_cone_facing"`,
/// the cosine), which a direction opposite a thin sector, between those
/// planes too, does; `true` where no reading decides it outside:
/// inside, or on a bound or square to the middle within the band.
/// Every caller reads a bound as the sector's.
///
/// Each reading is levered at the least deviation of the points it
/// reads ([`least_lever`]): the direction at `lever`, and each bound
/// it compares against at the bound's own reach. The middle turns by a
/// bound's turn over `|start + end|`. The facing reading's bound terms
/// buy refusals only: wherever it is near zero, a bound reading has
/// already decided the direction outside, so they never change a
/// class; and at a zero flip its answer can depend on which of two
/// sectors over one plane is read first, as an in-band reading may.
fn in_sector<T: Decide>(
    s: &BoolSector<T>,
    dir: Vec3<T>,
    lever: T,
    band: Band,
) -> Result<bool, BooleanError> {
    let escalate = |diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag);
    let (d, n) = (dir.normalize(), s.normal.vec());
    let sum = s.start + s.end;
    let middle = sum.normalize();
    let (one, l_start, l_end) = (
        T::from_f64(1.0),
        s.start_reach.length(),
        s.end_reach.length(),
    );
    let half = one / sum.norm();
    // Every reading here reads the direction at rate one, so a lever is
    // always found (`least_lever`'s `None` is not met).
    let zero = T::from_f64(0.0);
    let past = |x: T, l_bound: T| {
        Margin::levered(
            x,
            least_lever([(lever, one), (l_bound, one)]).unwrap_or(zero),
        )
    };
    let facing = least_lever([(lever, one), (l_start, half), (l_end, half)]).unwrap_or(zero);
    for (name, margin) in [
        ("bool_cone_within", past(s.start.cross(d).dot(n), l_start)),
        ("bool_cone_within", past(d.cross(s.end).dot(n), l_end)),
        ("bool_cone_facing", Margin::levered(d.dot(middle), facing)),
    ] {
        if decide(name, margin, band).map_err(escalate)? == Sign::Negative {
            return Ok(false);
        }
    }
    Ok(true)
}

/// **Whether the arc misses `s` where it meets `s`'s plane, read in
/// that plane**: wherever the arc from `d` to `p` meets the plane, it
/// meets it at `a·d + b·p` (`a, b ≥ 0`), a point of the wedge `d` and
/// `p` span projected into the plane, and the sector is the wedge its
/// bounds `u`, `v` span. Two such wedges, each under 180°, are apart
/// where a line through the vertex along one of the four has the other
/// strictly on its far side: `d` and `p` past `u`'s line or `v`'s, or
/// `u` and `v` past `d`'s line on the side away from `p`, or past
/// `p`'s away from `d`. No reading takes the ends' heights over the
/// plane, so it decides an arc lying in the plane, or meeting it at a
/// point it cannot locate.
///
/// Each reading is the sine `(a × b)·n` of two of the four
/// (`"bool_cone_apart"`), less its rounding, levered at the least joint
/// deviation of the two points it reads ([`least_lever`]), each at its
/// own reach: `d` at the direction's, `p` at its arm, a bound at the
/// bound's. `false` where no line decides it, a reading zero or in band
/// counting as no side.
fn apart<T: Decide>(s: &BoolSector<T>, arc: GreatArc<T>, band: Band) -> bool {
    let one = T::from_f64(1.0);
    let n = s.normal.vec();
    let u = (s.start, s.start_reach.length());
    let v = (s.end, s.end_reach.length());
    let d = (arc.dir.normalize(), arc.reach.length());
    let p = (arc.p, arc.p_arm);
    let (zero, rounding) = (T::from_f64(0.0), T::from_f64(8.0 * f64::EPSILON));
    // The decided sign of `(a × b)·n`, `None` where it is zero or in band.
    // The triple product of unit vectors rounds by a few ulp, which the
    // lever magnifies: what of it a rounding could account for is no
    // deviation of the points, so it is taken off first, as in
    // [`arc_side`].
    let sine = |(a, la): (Vec3<T>, T), (b, lb): (Vec3<T>, T)| {
        let lever = least_lever([(la, one), (lb, one)]).unwrap_or(zero);
        let x = a.cross(b).dot(n);
        let certain = (x - rounding).max(zero) + (x + rounding).min(zero);
        match decide("bool_cone_apart", Margin::levered(certain, lever), band) {
            Ok(Sign::Positive) => Some(1),
            Ok(Sign::Negative) => Some(-1),
            Ok(Sign::Zero) | Err(_) => None,
        }
    };
    let (ud, up) = (sine(u, d), sine(u, p));
    let (vd, vp) = (sine(v, d), sine(v, p));
    // The sector lies where `(u × x)·n ≥ 0` and `(x × v)·n ≥ 0`.
    let past_u = ud == Some(-1) && up == Some(-1);
    let past_v = vd == Some(1) && vp == Some(1);
    let past_ends = || {
        let pd = sine(p, d);
        let past_d = ud.is_some() && ud == vd && pd.is_some_and(|k| Some(-k) == ud);
        let past_p = up.is_some() && up == vp && pd.is_some() && pd == up;
        past_d || past_p
    };
    past_u || past_v || past_ends()
}

/// The arc [`cone_side`] reads along: from `dir`, read as `reach`
/// allows, to the unit reference `p`, a direction levered at `p_arm`.
#[derive(Clone, Copy)]
struct GreatArc<T: geom_core::Real> {
    dir: Vec3<T>,
    reach: Reach<T>,
    p: Vec3<T>,
    p_arm: T,
}

/// The sign of `det(dir, p, bound)` as a side code (`Out` positive):
/// which side of the plane through the arc's ends `bound` lies on.
/// `None` where the plane through `p` and `bound` has no decided normal
/// (`"bool_cone_arc_span"`, the sine between them levered at the
/// shorter of the two sectors' arms), or the determinant is decided
/// zero.
///
/// The margin (`"bool_cone_arc"`) is the least joint displacement of
/// the three points the determinant reads that flips its sign
/// ([`least_lever`]): turning one unit vector moves the determinant by
/// the turn times the sine between the other two, so the rates are
/// `|p̂×b̂|` for the direction's far point (at `L_d`), `|d̂×p̂|` for
/// `bound`'s (at its own reach) and `|d̂×b̂|` for `p` (at its arm). The
/// determinant's own rounding (eight ulp of three unit vectors) is
/// taken off before it is levered.
fn arc_side<T: Decide>(
    arc: GreatArc<T>,
    bound: Vec3<T>,
    bound_reach: Reach<T>,
    arm: T,
    band: Band,
) -> Result<Option<SideCode>, BooleanError> {
    let escalate = |diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag);
    let (d, b) = (arc.dir.normalize(), bound.normalize());
    let pb = arc.p.cross(b);
    let span = pb.norm();
    let gate = Margin::levered(span, arm.min(arc.p_arm));
    match decide("bool_cone_arc_span", gate, band).map_err(escalate)? {
        Sign::Positive => {}
        _ => return Ok(None),
    }
    let det = d.dot(pb);
    // The span is decided nonzero, so the direction's rate is, and a
    // lever is always found.
    let lever = least_lever([
        (arc.reach.length(), span),
        (bound_reach.length(), d.cross(arc.p).norm()),
        (arc.p_arm, d.cross(b).norm()),
    ])
    .unwrap_or(T::from_f64(0.0));
    // The determinant of three unit vectors rounds by a few ulp, which
    // the lever magnifies: what of it a rounding could account for is
    // no deviation of the points.
    let rounding = T::from_f64(8.0 * f64::EPSILON);
    let zero = T::from_f64(0.0);
    let certain = (det - rounding).max(zero) + (det + rounding).min(zero);
    Ok(
        match decide("bool_cone_arc", Margin::of(certain * lever), band).map_err(escalate)? {
            Sign::Positive => Some(SideCode::Out),
            Sign::Negative => Some(SideCode::In),
            Sign::Zero => None,
        },
    )
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::contact::BooleanCoincidence;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// **A flat corner reads as convex**: three faces on one plane, every
    /// bound on every other face's plane, read met, so an edge below
    /// the plane is In and one above it Out, as before hollow corners
    /// were read.
    #[test]
    fn a_flat_three_face_corner_reads_met() {
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let o = Point3::new(0.0, 0.0, 0.0);
        let dir = |deg: f64| {
            let (s, c) = deg.to_radians().sin_cos();
            Vec3::new(c, s, 0.0)
        };
        let chord = |d: Vec3<f64>| Reach::Chord {
            base: o,
            far: o + d,
        };
        let sector =
            |he: u64, face: u64, start: Vec3<f64>, end: Vec3<f64>, n: Vec3<f64>| BoolSector {
                he: HalfEdgeKey::from(key(he)),
                start,
                end,
                start_reach: chord(start),
                end_reach: chord(end),
                face: FaceKey::from(key(face)),
                normal: OutwardNormal::from_chart(n, true),
                arm: 1.0,
            };
        let up = Vec3::new(0.0, 0.0, 1.0);
        let flat: Vec<_> = [0.0, 120.0, 240.0]
            .iter()
            .enumerate()
            .map(|(k, &a)| sector(k as u64 + 1, k as u64 + 11, dir(a + 120.0), dir(a), up))
            .collect();
        let (below, above) = (Vec3::new(0.3, 0.2, -1.0), Vec3::new(-0.2, 0.3, 1.0));
        let own = [
            sector(21, 31, above, below, Vec3::new(1.0, 0.0, 0.0)),
            sector(22, 32, below, above, Vec3::new(-1.0, 0.0, 0.0)),
        ];
        let read = wedge_classes(&own, &flat, band()).unwrap().unwrap();
        assert!(read.met, "a flat corner reads met");
        assert_eq!(
            read.rows,
            vec![
                (HalfEdgeKey::from(key(21)), SideCode::In),
                (HalfEdgeKey::from(key(22)), SideCode::Out),
            ],
            "below the plane In, above it Out"
        );
    }

    /// The sectors of a cone's corner at the origin over the closed
    /// polygon `base` (counterclockwise from above): the material inside
    /// the cone, or outside it where `hollow`. Face `k` lies between
    /// corners `k` and `k + 1`, keyed `11 + k`.
    fn cone_sectors(base: &[Vec3<f64>], hollow: bool) -> Vec<BoolSector<f64>> {
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let o = Point3::new(0.0, 0.0, 0.0);
        let n = base.len();
        (0..n)
            .map(|k| {
                let (a, b) = (base[k], base[(k + 1) % n]);
                let (start, end, normal) = if hollow {
                    (a, b, a.cross(b))
                } else {
                    (b, a, b.cross(a))
                };
                BoolSector {
                    he: HalfEdgeKey::from(key(k as u64 + 1)),
                    start: start.normalize(),
                    end: end.normalize(),
                    start_reach: Reach::Chord {
                        base: o,
                        far: o + start,
                    },
                    end_reach: Reach::Chord {
                        base: o,
                        far: o + end,
                    },
                    face: FaceKey::from(key(k as u64 + 11)),
                    normal: OutwardNormal::from_chart(normal.normalize(), true),
                    arm: 1.0,
                }
            })
            .collect()
    }

    /// One own edge per direction, keyed `21 + k`, each a line edge.
    fn probes(dirs: &[Vec3<f64>]) -> Vec<BoolSector<f64>> {
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let o = Point3::new(0.0, 0.0, 0.0);
        dirs.iter()
            .enumerate()
            .map(|(k, &d)| BoolSector {
                he: HalfEdgeKey::from(key(k as u64 + 21)),
                start: Vec3::new(0.0, 0.0, 1.0),
                end: d.normalize(),
                start_reach: Reach::Bisector(1.0),
                end_reach: Reach::Chord {
                    base: o,
                    far: o + d,
                },
                face: FaceKey::from(key(99)),
                normal: OutwardNormal::from_chart(Vec3::new(1.0, 0.0, 0.0), true),
                arm: 1.0,
            })
            .collect()
    }

    fn classes(read: &WedgeRead) -> Vec<SideCode> {
        read.rows.iter().map(|&(_, c)| c).collect()
    }

    /// A dart over the square `(±1, 0)`, `(0, ±1)` at height 1, its
    /// corner `(−1, 0)` pulled to `(x, 0)`: a reflex edge there where
    /// `x > 0`.
    fn dart(x: f64) -> Vec<Vec3<f64>> {
        vec![
            Vec3::new(1.0, 0.0, 1.0),
            Vec3::new(0.0, 1.0, 1.0),
            Vec3::new(x, 0.0, 1.0),
            Vec3::new(0.0, -1.0, 1.0),
        ]
    }

    /// **A dart's apex reads every direction as its polygon cone holds
    /// it**, and its complement the other way round: In inside a lobe,
    /// Out in the notch the reflex edge cuts (which the convex hull would
    /// hold), past the tip and opposite, On a face within its sector and
    /// along an edge, and Out opposite an edge, where it lies on both of
    /// that edge's planes outside their sectors. The dart's cone lies in
    /// a half-space: read through it, met where it is the material and
    /// joined where it is the void.
    #[test]
    fn a_dart_corner_reads_as_its_polygon_cone() {
        let dirs = [
            Vec3::new(0.6, 0.05, 1.0),
            Vec3::new(0.1, 0.0, 1.0),
            Vec3::new(-0.5, 0.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.5, 0.5, 1.0),
            Vec3::new(0.3, 0.0, 1.0),
            Vec3::new(-1.0, 0.0, -1.0),
        ];
        use SideCode::{In, On, Out};
        let solid = [In, Out, Out, Out, On, On, Out];
        let read = wedge_classes(&probes(&dirs), &cone_sectors(&dart(0.3), false), band())
            .unwrap()
            .unwrap();
        assert_eq!(classes(&read), solid, "the dart");
        assert!(read.met && read.pointed, "the dart is its cone");
        let void = solid.map(|c| match c {
            In => Out,
            Out => In,
            On => On,
        });
        let read = wedge_classes(&probes(&dirs), &cone_sectors(&dart(0.3), true), band())
            .unwrap()
            .unwrap();
        assert_eq!(classes(&read), void, "the dart's void");
        assert!(!read.met && read.pointed, "the void is the dart's cone");
    }

    /// **A saddle reads as its polygon cone**, though neither it nor its
    /// complement lies in a half-space: below a fan of four planes
    /// through corners risen and fallen in turn, a direction under a
    /// ridge or a valley is In and over it Out. No side of its link is
    /// read as lying in a half-space, so its cone is its material.
    #[test]
    fn a_saddle_corner_reads_as_its_polygon_cone() {
        let ring: Vec<_> = [
            (1.0, 0.0, 0.4),
            (0.0, 1.0, -0.4),
            (-1.0, 0.0, 0.4),
            (0.0, -1.0, -0.4),
        ]
        .iter()
        .map(|&(x, y, z)| Vec3::new(x, y, z))
        .collect();
        // The material is below: each face's outward normal points up,
        // the opposite of a cone over a base above its apex.
        let saddle = cone_sectors(&ring, true);
        let dirs = [
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.2),
            Vec3::new(1.0, 0.0, 0.6),
            Vec3::new(0.0, 1.0, -0.6),
            Vec3::new(0.0, 1.0, -0.2),
            Vec3::new(1.0, 1.0, -0.1),
            Vec3::new(1.0, 1.0, 0.1),
        ];
        use SideCode::{In, Out};
        let read = wedge_classes(&probes(&dirs), &saddle, band())
            .unwrap()
            .unwrap();
        assert_eq!(
            classes(&read),
            [In, Out, In, Out, In, Out, In, Out],
            "below the saddle In"
        );
        assert!(read.met && !read.pointed, "a saddle's cone is its material");
        // A touch beside it would refuse rather than layer it.
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let partner = crate::entity::VertexKey::from(key(77));
        let first = read.rows[0].0;
        let pair = super::super::vtxfac::PairRead {
            partner,
            side: Some(SideCode::Out),
            read: Some(read),
            sectors: saddle,
        };
        assert_eq!(
            super::super::vtxfac::touch_classes(&[(first, SideCode::Out)], &[pair]),
            Err(Some(partner)),
            "a touch beside a saddle refuses"
        );
    }

    /// **An arc lying in a face's plane is read apart from its sector**:
    /// the saddle above read along `(0, −1, 0.4)`, opposite its valley
    /// bound, on the planes of the two faces beside that bound, outside
    /// their sectors (the review fuzz's `saddle0.4`, probe 34). Each
    /// other face's reference lies on the plane of the face opposite it,
    /// so the arc to it runs along that plane and locates no crossing;
    /// it lies decidedly apart from that face's sector, so the direction
    /// reads `Out`, as it exactly is.
    #[test]
    fn an_arc_in_a_faces_plane_is_read_apart_from_its_sector() {
        let ring: Vec<_> = [
            (1.0, 0.0, 0.4),
            (0.0, 1.0, -0.4),
            (-1.0, 0.0, 0.4),
            (0.0, -1.0, -0.4),
        ]
        .iter()
        .map(|&(x, y, z)| Vec3::new(x, y, z))
        .collect();
        let saddle = cone_sectors(&ring, true);
        let read = cone_read(&probes(&[Vec3::new(0.0, -1.0, 0.4)]), &saddle, band())
            .unwrap()
            .unwrap();
        assert_eq!(classes(&read), [SideCode::Out], "opposite the valley");
    }

    /// **A saddle whose mean bound has no decided length is not
    /// pointed**: the symmetric saddle with one corner risen 3e-12, so
    /// the mean of its unit bounds is a few 1e-13 long, nonzero but
    /// within the band. Its `−c` names no side, so the cone is labelled
    /// unpointed; read through `−c` regardless, the cone took whatever
    /// side that arbitrary direction read (PR 4289's fourth review,
    /// NoPointedGate). Read at ε = 1e-9, where the rise is in band.
    #[test]
    fn a_saddle_whose_mean_bound_is_in_band_is_not_pointed() {
        let ring: Vec<_> = [
            (1.0, 0.0, 0.4),
            (0.0, 1.0, -0.4),
            (-1.0, 0.0, 0.4),
            (0.0, -1.0, -0.4 + 3e-12),
        ]
        .iter()
        .map(|&(x, y, z)| Vec3::new(x, y, z))
        .collect();
        let saddle = cone_sectors(&ring, true);
        let read = cone_read(&probes(&[Vec3::new(0.0, 0.0, -1.0)]), &saddle, fuzz_band())
            .unwrap()
            .unwrap();
        assert!(
            read.met && !read.pointed,
            "an in-band mean bound labels nothing"
        );
    }

    /// **A direction in band of a face, within its sector, refuses**: a
    /// pyramid's face, read by a direction over its middle 5e-9 m off
    /// its plane at a 1 m reach, between the zero band and the
    /// escalation band at ε = 1e-9. It may be on the face, so it
    /// refuses, typed; passed over as off the face, the other faces'
    /// references read it, or nothing, and a pair beside it kept main's
    /// rows (PR 4289's fourth review, ErrOffFace).
    #[test]
    fn a_direction_in_band_of_a_face_within_its_sector_refuses() {
        let ring: Vec<_> = [
            (1.0, 0.0, 1.0),
            (0.0, 1.0, 1.0),
            (-1.0, 0.0, 1.0),
            (0.0, -1.0, 1.0),
        ]
        .iter()
        .map(|&(x, y, z)| Vec3::new(x, y, z))
        .collect();
        let cone = cone_sectors(&ring, false);
        let face = &cone[0];
        let d = (face.start + face.end).normalize() + face.normal.vec() * 5e-9;
        let o = Point3::new(0.0, 0.0, 0.0);
        let reach = Reach::Chord {
            base: o,
            far: o + d,
        };
        let read = cone_side(d.normalize(), reach, &cone, fuzz_band());
        assert!(
            read.is_err(),
            "in band of a face within its sector, read {read:?}"
        );
    }

    /// **An asymmetric saddle reads as its polygon cone too**: its mean
    /// bound has a decided length and `−c` reads decidedly, so it is
    /// labelled `pointed` though its link lies in no half-space (the
    /// label only a touch reads, and no touch's partner is a saddle);
    /// its rows are the cone's either way.
    #[test]
    fn an_asymmetric_saddle_reads_as_its_polygon_cone() {
        let ring: Vec<_> = [
            (1.0, 0.0, 0.4),
            (0.0, 1.0, -0.4),
            (-1.0, 0.0, 0.9),
            (0.0, -1.0, -0.1),
        ]
        .iter()
        .map(|&(x, y, z)| Vec3::new(x, y, z))
        .collect();
        let dirs = [
            Vec3::new(1.0, 0.0, 0.2),
            Vec3::new(1.0, 0.0, 0.6),
            Vec3::new(0.0, 1.0, -0.5),
            Vec3::new(0.0, 1.0, -0.3),
            Vec3::new(-1.0, 0.0, 0.8),
            Vec3::new(-1.0, 0.0, 1.0),
            Vec3::new(0.0, -1.0, -0.2),
            Vec3::new(0.0, -1.0, 0.0),
        ];
        use SideCode::{In, Out};
        let read = wedge_classes(&probes(&dirs), &cone_sectors(&ring, true), band())
            .unwrap()
            .unwrap();
        assert_eq!(
            classes(&read),
            [In, Out, In, Out, In, Out, In, Out],
            "below the saddle In"
        );
        assert!(
            read.pointed && !read.met,
            "`−c` points down into the material, so the cone read is the complement"
        );
    }

    /// **A reflex edge read in band refuses while the corner may still be
    /// convex**: a dart whose notch is an in-band offset deep reads its
    /// first bound in band before any reads past another face, and
    /// escalates as that side reading, rather than read as convex.
    #[test]
    fn an_in_band_reflex_edge_escalates_rather_than_read_convex() {
        let b = band();
        let mid = (b.zero() + b.escalate()) / 2.0;
        let dirs = [Vec3::new(0.6, 0.05, 1.0)];
        let got = wedge_classes(&probes(&dirs), &cone_sectors(&dart(mid / 2.0), false), b);
        match got {
            Err(BooleanError::Escalated { diag, .. }) => {
                assert_eq!(diag.predicate, Some("bool_chord_side"), "{diag:?}");
            }
            other => panic!("an in-band reflex edge escalates, got {other:?}"),
        }
    }

    /// A cone from fuzz rows, one per sector: its start, end and
    /// outward normal, whether each bound is an edge, each edge's far
    /// point, its arm, and its face.
    fn fuzz_cone(rows: &[[f64; 19]]) -> Vec<BoolSector<f64>> {
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let o = Point3::new(0.0, 0.0, 0.0);
        let v = |x: &[f64]| Vec3::new(x[0], x[1], x[2]);
        rows.iter()
            .enumerate()
            .map(|(k, f)| {
                let reach = |edge: f64, far: &[f64]| {
                    if edge > 0.5 {
                        Reach::Chord {
                            base: o,
                            far: o + v(far),
                        }
                    } else {
                        Reach::Bisector(f[17])
                    }
                };
                BoolSector {
                    he: HalfEdgeKey::from(key(k as u64 + 1)),
                    start: v(&f[0..3]),
                    end: v(&f[3..6]),
                    normal: OutwardNormal::from_chart(v(&f[6..9]), true),
                    start_reach: reach(f[9], &f[11..14]),
                    end_reach: reach(f[10], &f[14..17]),
                    face: FaceKey::from(key(1000 + f[18] as u64)),
                    arm: f[17],
                }
            })
            .collect()
    }

    /// The band the review's fuzz read its cones at, ε = 1e-9: the
    /// rows drawn from it are witnesses at that band, whatever ε the run
    /// reads.
    fn fuzz_band() -> Band {
        Band::linear_at(Tol::witness(), 1e-9).unwrap()
    }

    /// Each probe, a direction's far point, read against `cone` as a
    /// polygon cone ([`cone_read`]) at [`fuzz_band`], as its exact class.
    fn reads_exactly(what: &str, cone: &[BoolSector<f64>], want: &[(Vec3<f64>, SideCode)]) {
        for (k, &(far, class)) in want.iter().enumerate() {
            match cone_read(&probes(&[far]), cone, fuzz_band()) {
                Ok(Some(read)) => assert_eq!(classes(&read), [class], "{what}, probe {k}"),
                other => panic!("{what}, probe {k}: reads {class:?}, got {other:?}"),
            }
        }
    }

    /// Each probe read as in [`reads_exactly`], where a refusal or no
    /// reading is an answer too: only a class other than the exact one
    /// fails.
    fn never_contradicts(what: &str, cone: &[BoolSector<f64>], want: &[(Vec3<f64>, SideCode)]) {
        for (k, &(far, class)) in want.iter().enumerate() {
            if let Ok(Some(read)) = cone_read(&probes(&[far]), cone, fuzz_band()) {
                assert_eq!(classes(&read), [class], "{what}, probe {k}");
            }
        }
    }

    /// **An arc and a sector in one plane are apart across a line through
    /// a bound**: the sector from bearing 0° to 10° on `z = 0`, and arcs
    /// in that plane. Two lines part disjoint wedges, one at each end of
    /// the turn of lines that part them; each arc puts one end's two
    /// lines within a 1e-12 rad hair of touching, so the other end's line
    /// alone decides it. An arc through the sector is not apart, nor is
    /// one apart only within the band, read at its two points' joint
    /// lever: at a 1 mm reach, where the sine reads zero; at two 1 m
    /// reaches, where it lies between the zero band and the escalation
    /// band, and either reach alone would decide it; and at 1 km and
    /// 1 mm, where the longer would.
    #[test]
    fn an_arc_and_a_sector_in_one_plane_are_apart_across_a_bound_line() {
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let o = Point3::new(0.0, 0.0, 0.0);
        let at = |deg: f64| {
            let (s, c) = deg.to_radians().sin_cos();
            Vec3::new(c, s, 0.0)
        };
        let chord = |d: Vec3<f64>, l: f64| Reach::Chord {
            base: o,
            far: o + d * l,
        };
        let hair = 1e-12f64.to_degrees();
        let rad = f64::to_degrees;
        // Each arc: its ends' bearings, the direction's reach, and the
        // reach of the reference and the sector's bounds.
        for (what, d, p, (l_d, l), want) in [
            ("past u's line alone", 185.0, 190.0 + hair, (1.0, 1.0), true),
            ("past v's line alone", 185.0, 180.0 - hair, (1.0, 1.0), true),
            ("past d's line alone", 100.0, 190.0 + hair, (1.0, 1.0), true),
            ("past p's line alone", 190.0 + hair, 100.0, (1.0, 1.0), true),
            ("through the sector", -20.0, 30.0, (1.0, 1.0), false),
            (
                "apart within the band at a 1 mm reach",
                180.0 + rad(1e-7),
                190.0 + hair,
                (1e-3, 1e-3),
                false,
            ),
            (
                "apart within the band at the joint lever of two 1 m reaches",
                180.0 + rad(1.5e-8),
                190.0 + hair,
                (1.0, 1.0),
                false,
            ),
            (
                "apart within the band at the joint lever of 1 km and 1 mm",
                180.0 + rad(1e-7),
                190.0 + hair,
                (1e3, 1e-3),
                false,
            ),
        ] {
            let sector = BoolSector {
                he: HalfEdgeKey::from(key(1)),
                start: at(0.0),
                end: at(10.0),
                start_reach: chord(at(0.0), l),
                end_reach: chord(at(10.0), l),
                face: FaceKey::from(key(11)),
                normal: OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true),
                arm: l,
            };
            let arc = GreatArc {
                dir: at(d),
                reach: chord(at(d), l_d),
                p: at(p),
                p_arm: l,
            };
            assert_eq!(apart(&sector, arc, fuzz_band()), want, "{what}");
        }
    }

    /// **An arc and a sector are not parted by a sine's rounding**: a
    /// sector from `(2, 3, 0)` turned 10° about `z`, and an arc from
    /// `−(2, 3, 0)`, exactly on its start bound's line, to the end
    /// bound's opposite, every point a million metres out. The sine of
    /// the start bound and the direction is exactly zero; the unit
    /// vectors' product rounds to an ulp of it, which at that lever reads
    /// decided at ε = 1e-12 and parts them across one line or another,
    /// unless the rounding is taken off first. No line parts them
    /// strictly, so they are not apart.
    #[test]
    fn an_arc_and_a_sector_are_not_parted_by_a_sines_rounding() {
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let o = Point3::new(0.0, 0.0, 0.0);
        let band = Band::linear_at(Tol::witness(), 1e-12).unwrap();
        let far = 1e6;
        let u = Vec3::new(2.0, 3.0, 0.0).normalize();
        let (s, c) = 10f64.to_radians().sin_cos();
        let v = Vec3::new(c * u.x - s * u.y, s * u.x + c * u.y, 0.0);
        let d_far = Vec3::new(-2.0, -3.0, 0.0) * far;
        let chord = |far: Vec3<f64>| Reach::Chord {
            base: o,
            far: o + far,
        };
        let sector = BoolSector {
            he: HalfEdgeKey::from(key(1)),
            start: u,
            end: v,
            start_reach: chord(u * far),
            end_reach: chord(v * far),
            face: FaceKey::from(key(11)),
            normal: OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true),
            arm: far,
        };
        let arc = GreatArc {
            dir: d_far,
            reach: chord(d_far),
            p: -v,
            p_arm: far,
        };
        assert!(
            u.cross(d_far.normalize()).z != 0.0,
            "the sine rounds off zero"
        );
        assert!(!apart(&sector, arc, band), "parted by a rounding");
    }

    /// **An arc whose side of a bound reads nothing is read apart**: the
    /// long-probe family's dart void over 1 cm bounds, one edge reflex by
    /// 1e-5, read by a 1 km edge 1.1e-8 rad above the flat. Each
    /// reference's arc reads no side of some face's bound ([`arc_side`]),
    /// and where it lies decidedly apart from that face's sector, the
    /// face is not crossed, so the edge reads `Out`, as it exactly is,
    /// 1e-2 m from flipping (`sectors_cone_fuzz` seed 1, effort 10, cone
    /// 227, probe 10).
    #[test]
    fn an_arc_past_a_bound_it_reads_no_side_of_is_read_apart() {
        let cone = fuzz_cone(&[
            [
                1.0,
                0.0,
                0.0,
                0.0,
                0.9999999999500001,
                -9.9999999995e-6,
                -0.0,
                9.9999999995e-6,
                0.99999999995,
                1.0,
                1.0,
                0.01,
                0.0,
                0.0,
                0.0,
                0.0099999999995,
                -9.999999999500001e-8,
                0.01,
                0.0,
            ],
            [
                0.0,
                0.9999999999500001,
                -9.9999999995e-6,
                -1.0,
                0.0,
                0.0,
                0.0,
                9.9999999995e-6,
                0.99999999995,
                1.0,
                1.0,
                0.0,
                0.0099999999995,
                -9.999999999500001e-8,
                -0.01,
                0.0,
                0.0,
                0.01,
                1.0,
            ],
            [
                -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, -0.01, 0.0, 0.0, 0.0,
                -0.01, 0.0, 0.01, 2.0,
            ],
            [
                0.0, -1.0, 0.0, 1.0, 0.0, 0.0, -0.0, 0.0, 1.0, 1.0, 1.0, 0.0, -0.01, 0.0, 0.01,
                0.0, 0.0, 0.01, 3.0,
            ],
        ]);
        let far = Vec3::new(-43.62942360567253, 999.0477833396342, 1.1146525609545589e-5);
        reads_exactly("the dart void", &cone, &[(far, SideCode::Out)]);
    }

    /// **A thin fin reads as its polygon cone**: a crown whose two faces
    /// fold at a short edge (1 mm), corners 1e-5° apart, so a reference
    /// halfway along a fin face lies on the other fin face's plane
    /// within the band and inside its sector. Such a reference is passed
    /// over; read through it, the crossing beside it went uncounted and
    /// these read the other way. The exact classes are the review's
    /// rational oracle's (fuzz seed 1, cone 88).
    #[test]
    fn a_thin_fin_reads_as_its_polygon_cone() {
        use SideCode::{In, Out};
        let cone = fuzz_cone(&[
            [
                0.17364817766692978,
                1.515366220188009e-08,
                0.9848077530122081,
                1.0,
                0.0,
                0.0,
                0.0,
                0.9999999999999999,
                -1.5387431867316176e-08,
                1.0,
                1.0,
                0.00017364817766692977,
                1.5153662201880093e-11,
                0.0009848077530122082,
                0.5,
                0.0,
                0.0,
                0.001,
                0.0,
            ],
            [
                0.9999999999999848,
                1.745329251994321e-07,
                0.0,
                0.17364817766692978,
                1.515366220188009e-08,
                0.9848077530122081,
                1.745329251994321e-07,
                -0.9999999999999847,
                -1.5387431867316176e-08,
                1.0,
                1.0,
                0.4999999999999924,
                8.726646259971605e-08,
                0.0,
                0.00017364817766692977,
                1.5153662201880093e-11,
                0.0009848077530122082,
                0.001,
                1.0,
            ],
            [
                -0.49240387650610384,
                0.8528685319524434,
                -0.17364817766693036,
                0.9999999999999848,
                1.745329251994321e-07,
                0.0,
                3.482131941837936e-08,
                -0.19951146397500552,
                -0.9798954922554491,
                1.0,
                1.0,
                -0.24620193825305192,
                0.4264342659762217,
                -0.08682408883346518,
                0.4999999999999924,
                8.726646259971605e-08,
                0.0,
                0.5,
                2.0,
            ],
            [
                -0.4924038765061045,
                -0.852868531952443,
                0.17364817766693036,
                -0.49240387650610384,
                0.8528685319524434,
                -0.17364817766693036,
                -6.476292310140301e-17,
                -0.19951148327886362,
                -0.9798954883250905,
                1.0,
                1.0,
                -0.24620193825305225,
                -0.4264342659762215,
                0.08682408883346518,
                -0.24620193825305192,
                0.4264342659762217,
                -0.08682408883346518,
                0.5,
                3.0,
            ],
            [
                1.0,
                0.0,
                0.0,
                -0.4924038765061045,
                -0.852868531952443,
                0.17364817766693036,
                0.0,
                -0.19951148327886364,
                -0.9798954883250907,
                1.0,
                1.0,
                0.5,
                0.0,
                0.0,
                -0.24620193825305225,
                -0.4264342659762215,
                0.08682408883346518,
                0.5,
                4.0,
            ],
        ]);
        let probes = [
            // probe 0 (rand), 0.722 rad off the link: read Out before
            (
                Vec3::new(-0.7012307789035112, 0.04346165651255412, 0.745409981040932),
                In,
            ),
            // probe 2 (rand), 0.264 rad off the link: read Out before
            (
                Vec3::new(
                    -0.038538121019496815,
                    0.0893661290694972,
                    0.6785667375726702,
                ),
                In,
            ),
            // probe 9 (rand), 0.547 rad off the link: read In before
            (
                Vec3::new(-1.0832989362525558, 0.8839922290405432, -1.213883580710491),
                Out,
            ),
            // probe 30 (out1e-06), 1.02e-06 rad off the link: read In before
            (
                Vec3::new(
                    -0.055517245066364046,
                    1.8521729957714668,
                    -0.37711335246230726,
                ),
                Out,
            ),
            // probe 31 (out-1e-06), 9.81e-07 rad off the link: read Out before
            (
                Vec3::new(
                    -0.025275208515307724,
                    0.8432348847632849,
                    -0.17168584171430507,
                ),
                In,
            ),
        ];
        // Probe 30, 1.02e-6 rad off the link, is read through an arc
        // whose side of a bound the joint lever puts in band: it refuses
        // or reads `Out`, never `In`.
        let [p0, p2, p9, p30, p31] = probes;
        reads_exactly("the fin", &cone, &[p0, p2, p9, p31]);
        never_contradicts("the fin, probe 30", &cone, &[p30]);
    }

    /// **A direction beside a face's plane, outside its sector, reads
    /// where the arc meets that plane**: within the band of the plane
    /// the arc crosses it beside the direction, and whether that point
    /// lies in the sector decides, not the direction itself. Here that
    /// point lies in band of the sector's bound, so every reference is
    /// passed over and the reading escalates, typed, where it read `In`
    /// (exactly `Out`) on the direction alone (fuzz seed 1, cone 157,
    /// probe 66, 3.6e-8 rad off the link).
    #[test]
    fn a_direction_beside_a_face_reads_the_arcs_crossing_point() {
        let cone = fuzz_cone(&[
            [
                0.09849712864578242,
                0.31922617021914795,
                0.9425460030660316,
                0.7287911791219575,
                0.6774676576302496,
                0.09950371902099893,
                -0.6565251074459609,
                0.7326296871969556,
                -0.17952304790962156,
                1.0,
                1.0,
                0.1911579844210296,
                0.6195371592301719,
                1.8292431124378283,
                1.002102141786668,
                0.9315312947673083,
                0.13681956204082388,
                1.375019581046513,
                0.0,
            ],
            [
                -0.26698249053549805,
                -0.4499373456801624,
                0.8522187129544578,
                0.09849712864578242,
                0.31922617021914795,
                0.9425460030660316,
                -0.8995360624606991,
                0.43363616247784187,
                -0.052863512219601394,
                1.0,
                1.0,
                -0.3198402037772413,
                -0.5390168173227081,
                1.0209426328573807,
                0.1911579844210296,
                0.6195371592301719,
                1.8292431124378283,
                1.1979819468150301,
                1.0,
            ],
            [
                0.11177848716856556,
                -0.39914741906534357,
                0.910047750208626,
                -0.26698249053549805,
                -0.4499373456801624,
                0.8522187129544578,
                0.1827548372450386,
                -0.8919098333756266,
                -0.413639358126619,
                1.0,
                1.0,
                0.06237442965805327,
                -0.22273152235581717,
                0.5078232030037773,
                -0.3198402037772413,
                -0.5390168173227081,
                1.0209426328573807,
                0.5580181950753245,
                2.0,
            ],
            [
                0.7287911791219575,
                0.6774676576302496,
                0.09950371902099893,
                0.11177848716856556,
                -0.39914741906534357,
                0.910047750208626,
                0.6594444449145949,
                -0.6552921237288362,
                -0.36840908871827976,
                1.0,
                1.0,
                1.002102141786668,
                0.9315312947673083,
                0.13681956204082388,
                0.06237442965805327,
                -0.22273152235581717,
                0.5078232030037773,
                0.5580181950753245,
                3.0,
            ],
        ]);
        let far = Vec3::new(0.6272258333617353, 0.5830548321982457, 0.08563671260550405);
        match cone_read(&probes(&[far]), &cone, fuzz_band()) {
            Err(BooleanError::Escalated { diag, .. }) => {
                assert_eq!(diag.predicate, Some("bool_cone_within"), "{diag:?}");
            }
            other => panic!("the crossing point in band escalates, got {other:?}"),
        }
    }

    /// **An arc through a link vertex passes over its reference**: a fin
    /// 1e-3° wide whose short edge lies on the arc from a direction to
    /// its first reference, so the arc's side of that bound reads zero.
    /// The crossing there is neither counted nor dropped; the next
    /// reference reads (fuzz seed 1, cone 76).
    #[test]
    fn an_arc_through_a_link_vertex_passes_over_its_reference() {
        use SideCode::Out;
        let cone = fuzz_cone(&[
            [
                0.1736481776603184,
                1.5153662201687772e-06,
                0.9848077530122081,
                1.0,
                0.0,
                0.0,
                0.0,
                0.9999999999988161,
                -1.5387431867102676e-06,
                1.0,
                1.0,
                0.0001736481776603184,
                1.5153662201687773e-09,
                0.0009848077530122082,
                0.5,
                0.0,
                0.0,
                0.001,
                0.0,
            ],
            [
                0.9999999998476913,
                1.7453292519057202e-05,
                0.0,
                0.1736481776603184,
                1.5153662201687772e-06,
                0.9848077530122081,
                1.7453292519036538e-05,
                -0.9999999998465073,
                -1.5387431867102678e-06,
                1.0,
                1.0,
                0.49999999992384564,
                8.726646259528601e-06,
                0.0,
                0.0001736481776603184,
                1.5153662201687773e-09,
                0.0009848077530122082,
                0.001,
                1.0,
            ],
            [
                -0.49240387650610384,
                0.8528685319524434,
                -0.17364817766693036,
                0.9999999998476913,
                1.7453292519057202e-05,
                0.0,
                3.4820985878143478e-06,
                -0.19950955290996816,
                -0.9798958813494113,
                1.0,
                1.0,
                -0.24620193825305192,
                0.4264342659762217,
                -0.08682408883346518,
                0.49999999992384564,
                8.726646259528601e-06,
                0.0,
                0.5,
                2.0,
            ],
            [
                -0.4924038765061045,
                -0.852868531952443,
                0.17364817766693036,
                -0.49240387650610384,
                0.8528685319524434,
                -0.17364817766693036,
                -6.476292310140301e-17,
                -0.19951148327886362,
                -0.9798954883250905,
                1.0,
                1.0,
                -0.24620193825305225,
                -0.4264342659762215,
                0.08682408883346518,
                -0.24620193825305192,
                0.4264342659762217,
                -0.08682408883346518,
                0.5,
                3.0,
            ],
            [
                1.0,
                0.0,
                0.0,
                -0.4924038765061045,
                -0.852868531952443,
                0.17364817766693036,
                0.0,
                -0.19951148327886364,
                -0.9798954883250907,
                1.0,
                1.0,
                0.5,
                0.0,
                0.0,
                -0.24620193825305225,
                -0.4264342659762215,
                0.08682408883346518,
                0.5,
                4.0,
            ],
        ]);
        let probes = [
            // probe 29 (out-2e-08), 0.899 rad off the link
            (
                Vec3::new(
                    0.6927479537076511,
                    -2.489147207621945e-06,
                    -1.5950468525226076,
                ),
                Out,
            ),
            // probe 39 (nearantip), 0.63 rad off the link
            (
                Vec3::new(
                    -0.4386568833047421,
                    -6.017838887455884e-07,
                    -0.36807733841532486,
                ),
                Out,
            ),
        ];
        reads_exactly("the fin", &cone, &probes);
    }

    /// **A direction outside a sector is read at the bound it passes,
    /// not at the sector's shortest arm**: a hollow fin whose short edge
    /// is 1 cm, and a direction 7.9e-8 rad (1.5e-7 m at its 1.95 m
    /// reach) past the fin's base edge, which is 0.5 m long. The least
    /// deviation that flips that reading moves the base edge's far point
    /// or the direction's by 4e-8 m, forty zero bands, so it is decided:
    /// `In`, as the exact oracle reads it. Levered at the sector's arm
    /// (`within`'s, the short edge's 1 cm), it read on the bound (fuzz
    /// seed 1, cone 85; PR 4289's third review re-posed it).
    #[test]
    fn a_direction_off_a_short_armed_sector_is_off_it() {
        use SideCode::In;
        let cone = fuzz_cone(&[
            [
                1.0,
                0.0,
                0.0,
                0.17364817766686433,
                1.5153662201878184e-07,
                0.9848077530122081,
                0.0,
                -0.9999999999999881,
                1.5387431867314058e-07,
                1.0,
                1.0,
                0.5,
                0.0,
                0.0,
                0.0017364817766686432,
                1.5153662201878184e-09,
                0.009848077530122082,
                0.01,
                0.0,
            ],
            [
                0.17364817766686433,
                1.5153662201878184e-07,
                0.9848077530122081,
                0.999999999998477,
                1.7453292519934438e-06,
                0.0,
                -1.745329251993423e-06,
                0.999999999998465,
                1.5387431867314066e-07,
                1.0,
                1.0,
                0.0017364817766686432,
                1.5153662201878184e-09,
                0.009848077530122082,
                0.4999999999992385,
                8.726646259967219e-07,
                0.0,
                0.01,
                1.0,
            ],
            [
                0.999999999998477,
                1.7453292519934438e-06,
                0.0,
                -0.49240387650610384,
                0.8528685319524434,
                -0.17364817766693036,
                -3.4821289096011875e-07,
                0.1995112902404368,
                0.9798955276285707,
                1.0,
                1.0,
                0.4999999999992385,
                8.726646259967219e-07,
                0.0,
                -0.24620193825305192,
                0.4264342659762217,
                -0.08682408883346518,
                0.5,
                2.0,
            ],
            [
                -0.49240387650610384,
                0.8528685319524434,
                -0.17364817766693036,
                -0.4924038765061045,
                -0.852868531952443,
                0.17364817766693036,
                6.476292310140301e-17,
                0.19951148327886362,
                0.9798954883250905,
                1.0,
                1.0,
                -0.24620193825305192,
                0.4264342659762217,
                -0.08682408883346518,
                -0.24620193825305225,
                -0.4264342659762215,
                0.08682408883346518,
                0.5,
                3.0,
            ],
            [
                -0.4924038765061045,
                -0.852868531952443,
                0.17364817766693036,
                1.0,
                0.0,
                0.0,
                -0.0,
                0.19951148327886364,
                0.9798954883250907,
                1.0,
                1.0,
                -0.24620193825305225,
                -0.4264342659762215,
                0.08682408883346518,
                0.5,
                0.0,
                0.0,
                0.5,
                4.0,
            ],
        ]);
        let probes = [
            // probe 64 (nearbound), 7.94e-08 rad off the link
            (
                Vec3::new(
                    1.9466554485576073,
                    2.698130705263507e-10,
                    -1.546282621768826e-07,
                ),
                In,
            ),
        ];
        reads_exactly("the hollow fin", &cone, &probes);
    }

    /// **A reference read in band is passed over, not taken as clear**:
    /// a dart notched to within 1e-8 of its tip, two faces folded on two
    /// others, so every reference lies in band of a folded face. The
    /// reading escalates, typed, where taking the in-band face as not
    /// crossed read `In` (exactly `Out`, 0.42 rad off the link; fuzz
    /// seed 1, cone 12, probe 0).
    #[test]
    fn a_reference_read_in_band_is_passed_over() {
        let cone = fuzz_cone(&[
            [
                0.0,
                0.7071067811865475,
                0.7071067811865475,
                0.7071067811865475,
                0.0,
                0.7071067811865475,
                0.5773502691896257,
                0.5773502691896257,
                -0.5773502691896257,
                1.0,
                1.0,
                0.0,
                1.298868870099869,
                1.298868870099869,
                0.891259722645434,
                0.0,
                0.891259722645434,
                1.260431587362056,
                0.0,
            ],
            [
                0.7071067776510136,
                0.0,
                0.7071067847220814,
                0.0,
                0.7071067811865475,
                0.7071067811865475,
                -0.5773502730386275,
                -0.5773502672651248,
                0.5773502672651248,
                1.0,
                1.0,
                1.0814416594477672,
                0.0,
                1.0814416702621839,
                0.0,
                1.298868870099869,
                1.298868870099869,
                1.5293894693532457,
                1.0,
            ],
            [
                0.0,
                -0.7071067811865475,
                0.7071067811865475,
                0.7071067776510136,
                0.0,
                0.7071067847220814,
                -0.5773502730386275,
                0.5773502672651248,
                0.5773502672651248,
                1.0,
                1.0,
                0.0,
                -1.0428525624262577,
                1.0428525624262577,
                1.0814416594477672,
                0.0,
                1.0814416702621839,
                1.4748162373387486,
                2.0,
            ],
            [
                0.7071067811865475,
                0.0,
                0.7071067811865475,
                0.0,
                -0.7071067811865475,
                0.7071067811865475,
                0.5773502691896257,
                -0.5773502691896257,
                -0.5773502691896257,
                1.0,
                1.0,
                0.891259722645434,
                0.0,
                0.891259722645434,
                0.0,
                -1.0428525624262577,
                1.0428525624262577,
                1.260431587362056,
                3.0,
            ],
        ]);
        let far = Vec3::new(
            -0.21445403262747445,
            0.37346455774909554,
            0.8499197060265414,
        );
        match cone_read(&probes(&[far]), &cone, fuzz_band()) {
            Err(BooleanError::Escalated { .. }) => {}
            other => panic!("every reference in band escalates, got {other:?}"),
        }
    }

    /// **A direction opposite a thin sector is not within it**: the
    /// direction lies on the sector's plane between the planes through
    /// its two bounds square to it, as a direction inside does, but
    /// faces away from its middle. At ε = 1e-6 a sector under a band
    /// wide read it on the face, `On`, 0.38 rad off the link (fuzz seed
    /// 1, cone 421, probe 81).
    #[test]
    fn a_direction_opposite_a_thin_sector_is_not_within_it() {
        let cone = fuzz_cone(&[
            [
                -0.6614418098433579,
                0.7261333654521895,
                -0.18768342432996074,
                -0.6924786207552645,
                0.7153826686785647,
                -0.09327913566935259,
                0.6664551756483287,
                0.6838413138391651,
                0.2969824175574708,
                1.0,
                1.0,
                -0.5670774986384223,
                0.6225398613009403,
                -0.16090764935184948,
                -0.8092522502095967,
                0.8360186394746161,
                -0.10900892558343428,
                0.8573354302666731,
                0.0,
            ],
            [
                -0.5722075433895275,
                0.7764973150445121,
                -0.26387581741566607,
                -0.6614418098433579,
                0.7261333654521895,
                -0.18768342432996074,
                0.35999205060076545,
                0.526918713477677,
                0.7699106395493478,
                1.0,
                1.0,
                -0.6513101717614873,
                0.8838411962173216,
                -0.30035431365801496,
                -0.5670774986384223,
                0.6225398613009403,
                -0.16090764935184948,
                0.8573354302666731,
                1.0,
            ],
            [
                -0.2880801681515998,
                -0.207823551037667,
                -0.9347829632336269,
                -0.5722075433895275,
                0.7764973150445121,
                -0.26387581741566607,
                0.8063282199588498,
                0.47393837170939157,
                -0.35386045486807816,
                1.0,
                1.0,
                -0.2358595371029621,
                -0.1701511314067377,
                -0.7653337555815157,
                -0.6513101717614873,
                0.8838411962173216,
                -0.30035431365801496,
                0.8187288233560144,
                2.0,
            ],
            [
                0.23849063399021297,
                -0.8137918338720428,
                0.5299670448453598,
                -0.2880801681515998,
                -0.207823551037667,
                -0.9347829632336269,
                0.9479368002201074,
                0.07648295073682806,
                -0.3091378026625862,
                1.0,
                1.0,
                0.1490580396013985,
                -0.5086246506668751,
                0.33123249930744686,
                -0.2358595371029621,
                -0.1701511314067377,
                -0.7653337555815157,
                0.6250058423993097,
                3.0,
            ],
            [
                0.2384906346471053,
                -0.8137918298957801,
                0.5299670506555085,
                0.23849063399021297,
                -0.8137918338720428,
                0.5299670448453598,
                0.9666912988820815,
                0.14672957265931263,
                -0.20971019329753562,
                1.0,
                1.0,
                0.1890247090833439,
                -0.6450012770022163,
                0.4200452890829321,
                0.1490580396013985,
                -0.5086246506668751,
                0.33123249930744686,
                0.6250058423993097,
                4.0,
            ],
            [
                -0.6924786207552645,
                0.7153826686785647,
                -0.09327913566935259,
                0.2384906346471053,
                -0.8137918298957801,
                0.5299670506555085,
                0.5017699818753559,
                0.5704862301260616,
                0.650209463577207,
                1.0,
                1.0,
                -0.8092522502095967,
                0.8360186394746161,
                -0.10900892558343428,
                0.1890247090833439,
                -0.6450012770022163,
                0.4200452890829321,
                0.792587555327042,
                5.0,
            ],
        ]);
        let band = Band::linear_at(Tol::witness(), 1e-6).unwrap();
        let far = Vec3::new(-0.3057757217779084, 1.0433859762840192, -0.6794860436367299);
        let read = cone_read(&probes(&[far]), &cone, band).unwrap().unwrap();
        assert_eq!(classes(&read), [SideCode::Out], "opposite the thin sector");
    }

    /// **An arc's side of a bound is decided at its least deviation**:
    /// `det(d,p,b)` flips as soon as any of its three points moves off
    /// the plane through the other two. Read at the direction's far
    /// point alone it read decided where the bound's own far point, a
    /// 3e-9 jitter away, flips it, and this direction 1.6 rad from the
    /// link read `Out` (exactly `In`; the review's perturbation fuzz,
    /// seed 1 cone 742 jittered, probe 28).
    #[test]
    fn an_arcs_side_of_a_bound_is_decided_at_its_least_deviation() {
        let cone = fuzz_cone(&[
            [
                0.2944223632806079,
                0.11539269086588769,
                0.9486832974681232,
                0.3715470460896104,
                0.186857846147052,
                0.9094157123534701,
                -0.6454407765086191,
                0.756086979394373,
                0.1083452057556961,
                1.0,
                1.0,
                0.28440659311143385,
                0.11146721911150957,
                0.9164106339214383,
                0.4797224960355556,
                0.24126127041223222,
                1.1741909404359516,
                0.9659816275845734,
                0.0,
            ],
            [
                0.3715470460896104,
                0.186857846147052,
                0.9094157123534701,
                0.10380788962390015,
                0.32714454559678724,
                0.9392552200217488,
                -0.40639005025933034,
                -0.8479772651916408,
                0.34026707858434485,
                1.0,
                1.0,
                0.4797224960355556,
                0.24126127041223222,
                1.1741909404359516,
                0.18532528694754818,
                0.5840418970630067,
                1.6768257576391064,
                1.2911487291928618,
                1.0,
            ],
            [
                0.10380788962390015,
                0.32714454559678724,
                0.9392552200217488,
                0.07999406086409815,
                0.43752685769436445,
                0.8956401057470386,
                -0.976126507106398,
                -0.14764241799541128,
                0.15930712015704615,
                1.0,
                1.0,
                0.18532528694754818,
                0.5840418970630067,
                1.6768257576391064,
                0.15628581305765818,
                0.854803968578311,
                1.749827932495235,
                1.7852716948329268,
                2.0,
            ],
            [
                0.07999406086409815,
                0.43752685769436445,
                0.8956401057470386,
                -0.8784540743273178,
                0.29988474044443497,
                0.37200481414399605,
                -0.1151467464631772,
                -0.8884483860490664,
                0.4442979767067974,
                1.0,
                1.0,
                0.15628581305765818,
                0.854803968578311,
                1.749827932495235,
                -0.4456480797408382,
                0.15213437176551342,
                0.18872156885898547,
                0.5073094787309883,
                3.0,
            ],
            [
                -0.8784540743273178,
                0.29988474044443497,
                0.37200481414399605,
                -0.35833223316335583,
                -0.928276370693129,
                0.099503719975461,
                0.376178140715824,
                -0.0460160542549421,
                0.9254039816201312,
                1.0,
                1.0,
                -0.4456480797408382,
                0.15213437176551342,
                0.18872156885898547,
                -0.3250539754835754,
                -0.8420674913264363,
                0.09026282527780048,
                0.5073094787309883,
                4.0,
            ],
            [
                -0.35833223316335583,
                -0.928276370693129,
                0.099503719975461,
                0.14412110609971684,
                -0.3262473517414647,
                0.9342332536675597,
                -0.889078057441372,
                0.37182122291624414,
                0.26700034824948266,
                1.0,
                1.0,
                -0.3250539754835754,
                -0.8420674913264363,
                0.09026282527780048,
                0.179053944772201,
                -0.4053249165349545,
                1.16067766847977,
                0.9071301576064402,
                5.0,
            ],
            [
                0.14412110609971684,
                -0.3262473517414647,
                0.9342332536675597,
                0.2944223632806079,
                0.11539269086588769,
                0.9486832974681232,
                -0.9194840664623023,
                0.30479994594401605,
                0.24828621482972682,
                1.0,
                1.0,
                0.179053944772201,
                -0.4053249165349545,
                1.16067766847977,
                0.28440659311143385,
                0.11146721911150957,
                0.9164106339214383,
                0.9659816275845734,
                6.0,
            ],
        ]);
        let far = Vec3::new(0.5908476894119604, 0.612583177120514, -0.7550767842041842);
        let read = cone_read(&probes(&[far]), &cone, fuzz_band())
            .unwrap()
            .unwrap();
        assert_eq!(classes(&read), [SideCode::In], "1.6 rad inside");
    }

    /// **A reading is levered at the joint move of its points**: a hollow
    /// star with 1 cm and 1 mm bounds, read by a 1 cm direction 1.5 bands
    /// (by one point's move) past a 1 cm bound, at `K = 1.5`. Each point
    /// alone needs a move of 1.5 bands to flip it, so the least of them
    /// decided it at that `K`; moved together, the direction's far point
    /// and the bound's flip it within one, so it is not decided (PR 4289's
    /// fourth review: its adversarial fuzz, `s3e1e-9`, case 79, probe 13,
    /// exactly `Out`).
    #[test]
    fn a_reading_is_levered_at_the_joint_move_of_its_points() {
        let cone = fuzz_cone(&[
            [
                0.5662783278636936,
                0.004479865091332831,
                0.8242019086368719,
                0.5031402685791894,
                0.2475640549183358,
                -0.8279866598239687,
                -0.22629038364798074,
                0.9624028342998528,
                0.15024462319850146,
                1.0,
                1.0,
                0.2831391639318468,
                0.0022399325456664153,
                0.41210095431843596,
                0.05031402685791894,
                0.024756405491833583,
                -0.08279866598239688,
                0.1,
                0.0,
            ],
            [
                0.5031402685791894,
                0.2475640549183358,
                -0.8279866598239687,
                0.04225900435235078,
                0.5827980353915212,
                -0.811517483788816,
                0.5153764520068449,
                0.6831221127056741,
                0.5174275715975837,
                1.0,
                1.0,
                0.05031402685791894,
                0.024756405491833583,
                -0.08279866598239688,
                4.2259004352350785e-05,
                0.0005827980353915212,
                -0.0008115174837888161,
                0.001,
                1.0,
            ],
            [
                0.04225900435235078,
                0.5827980353915212,
                -0.811517483788816,
                -0.6675045382586478,
                0.06712531717404681,
                0.7415739229492856,
                0.603233869378344,
                0.6326012363251808,
                0.48572067552728665,
                1.0,
                1.0,
                4.2259004352350785e-05,
                0.0005827980353915212,
                -0.0008115174837888161,
                -0.006675045382586477,
                0.0006712531717404681,
                0.007415739229492856,
                0.001,
                2.0,
            ],
            [
                -0.6675045382586478,
                0.06712531717404681,
                0.7415739229492856,
                -0.46584302571477215,
                -0.12699063866727106,
                -0.8757075157173111,
                0.03773510138525747,
                -0.991599052426691,
                0.12372300250936683,
                1.0,
                1.0,
                -0.006675045382586477,
                0.0006712531717404681,
                0.007415739229492856,
                -0.23292151285738608,
                -0.06349531933363553,
                -0.43785375785865555,
                0.01,
                3.0,
            ],
            [
                -0.46584302571477215,
                -0.12699063866727106,
                -0.8757075157173111,
                0.29289245779873496,
                -0.5674103047224892,
                -0.7695840137758503,
                -0.5035061846332539,
                -0.7757703813394258,
                0.38034436695256246,
                1.0,
                1.0,
                -0.23292151285738608,
                -0.06349531933363553,
                -0.43785375785865555,
                0.0029289245779873497,
                -0.005674103047224892,
                -0.007695840137758503,
                0.01,
                4.0,
            ],
            [
                0.29289245779873496,
                -0.5674103047224892,
                -0.7695840137758503,
                0.2801514954778376,
                -0.5373358976682128,
                -0.7954780151950317,
                0.9079814127646484,
                0.4172932050401712,
                0.03789637319341752,
                1.0,
                1.0,
                0.0029289245779873497,
                -0.005674103047224892,
                -0.007695840137758503,
                0.02801514954778376,
                -0.05373358976682128,
                -0.07954780151950318,
                0.01,
                5.0,
            ],
            [
                0.2801514954778376,
                -0.5373358976682128,
                -0.7954780151950317,
                0.44136424345032577,
                -0.6658099107656776,
                0.6015769005951958,
                -0.8528895257880404,
                -0.5196303626005767,
                0.05063341845702857,
                1.0,
                1.0,
                0.02801514954778376,
                -0.05373358976682128,
                -0.07954780151950318,
                0.0004413642434503258,
                -0.0006658099107656777,
                0.0006015769005951958,
                0.001,
                6.0,
            ],
            [
                0.44136424345032577,
                -0.6658099107656776,
                0.6015769005951958,
                0.5662783278636936,
                0.004479865091332831,
                0.8242019086368719,
                -0.8236313417660174,
                -0.03452098346098773,
                0.5660739479622747,
                1.0,
                1.0,
                0.0004413642434503258,
                -0.0006658099107656777,
                0.0006015769005951958,
                0.2831391639318468,
                0.0022399325456664153,
                0.41210095431843596,
                0.001,
                7.0,
            ],
        ]);
        let band = Band::new(1e-9, 1.5e-9).unwrap();
        let far = Vec3::new(
            0.0029289253189041372,
            -0.005674104198342163,
            -0.00769583900706343,
        );
        let o = Point3::new(0.0, 0.0, 0.0);
        let reach = Reach::Chord {
            base: o,
            far: o + far,
        };
        let read = cone_side(far.normalize(), reach, &cone, band);
        assert!(
            !matches!(read, Ok(Some(SideCode::In | SideCode::Out))),
            "a joint move within the band flips it, read {read:?}"
        );
    }

    /// **A direction beside a short bound is read at the bound's own
    /// reach**: a crown whose face `S` (on `z = 0`) has a 1 mm edge `B`,
    /// read by directions 1e-7 rad past `B` and 1e-10, 4e-10 and 2e-9 m
    /// over the plane; and its twin, `B`'s far point moved 3e-10 m in
    /// the plane, a third of the zero band. The two cones are one cone
    /// within the band, but their exact classes differ (`Out`, then
    /// `In`). Levered at the direction's reach alone, the first read
    /// `Out` while its twin refused: a decided class its twin
    /// contradicts. Each now reads `On` or refuses (PR 4289's third
    /// review; `r3_bound.py`, case `bnd_zA0.2_s`).
    #[test]
    fn a_direction_beside_a_short_bound_is_read_at_the_bounds_reach() {
        let cone = fuzz_cone(&[
            [
                0.9987502603949663,
                0.04997916927067833,
                0.0,
                0.9367845000189781,
                -0.28978140392829665,
                0.19611613513818402,
                0.025180826917106023,
                -0.5031967879301025,
                -0.8638049077034687,
                1.0,
                1.0,
                0.0009987502603949663,
                4.997916927067833e-05,
                0.0,
                0.9367845000189781,
                -0.28978140392829665,
                0.19611613513818402,
                0.001,
                0.0,
            ],
            [
                0.8775825618903728,
                0.479425538604203,
                0.0,
                0.9987502603949663,
                0.04997916927067833,
                0.0,
                0.0,
                0.0,
                -1.0,
                1.0,
                1.0,
                0.8775825618903728,
                0.479425538604203,
                0.0,
                0.0009987502603949663,
                4.997916927067833e-05,
                0.0,
                0.001,
                1.0,
            ],
            [
                -0.39859637855648306,
                0.8709489764975944,
                0.2873478855663454,
                0.8775825618903728,
                0.479425538604203,
                0.0,
                -0.13807921244038343,
                0.25275230299584717,
                -0.9576274872945841,
                1.0,
                1.0,
                -0.39859637855648306,
                0.8709489764975944,
                0.2873478855663454,
                0.8775825618903728,
                0.479425538604203,
                0.0,
                1.0,
                2.0,
            ],
            [
                -0.8969628300582488,
                -0.3359893958958339,
                -0.2873478855663454,
                -0.39859637855648306,
                0.8709489764975944,
                0.2873478855663454,
                0.1537435140353627,
                0.3723343456140114,
                -0.9152759512672909,
                1.0,
                1.0,
                -0.8969628300582488,
                -0.3359893958958339,
                -0.2873478855663454,
                -0.39859637855648306,
                0.8709489764975944,
                0.2873478855663454,
                1.0,
                3.0,
            ],
            [
                0.2805238471386536,
                -0.9483150748622418,
                0.14834045293024462,
                -0.8969628300582488,
                -0.3359893958958339,
                -0.2873478855663454,
                0.3224329819611792,
                -0.05246353097466939,
                -0.9451373180979005,
                1.0,
                1.0,
                0.2805238471386536,
                -0.9483150748622418,
                0.14834045293024462,
                -0.8969628300582488,
                -0.3359893958958339,
                -0.2873478855663454,
                1.0,
                4.0,
            ],
            [
                0.9367845000189781,
                -0.28978140392829665,
                0.19611613513818402,
                0.2805238471386536,
                -0.9483150748622418,
                0.14834045293024462,
                0.17354989635680937,
                -0.10188659515906286,
                -0.9795404816552677,
                1.0,
                1.0,
                0.9367845000189781,
                -0.28978140392829665,
                0.19611613513818402,
                0.2805238471386536,
                -0.9483150748622418,
                0.14834045293024462,
                1.0,
                5.0,
            ],
        ]);
        let twin = fuzz_cone(&[
            [
                0.9987502753886721,
                0.04997886964559796,
                0.0,
                0.9367845000189781,
                -0.28978140392829665,
                0.19611613513818402,
                0.025180691399709475,
                -0.5031971040615982,
                -0.8638047274961244,
                1.0,
                1.0,
                0.0009987502753886722,
                4.997886964559796e-05,
                0.0,
                0.9367845000189781,
                -0.28978140392829665,
                0.19611613513818402,
                0.001,
                0.0,
            ],
            [
                0.8775825618903728,
                0.479425538604203,
                0.0,
                0.9987502753886721,
                0.04997886964559796,
                0.0,
                0.0,
                0.0,
                -1.0,
                1.0,
                1.0,
                0.8775825618903728,
                0.479425538604203,
                0.0,
                0.0009987502753886722,
                4.997886964559796e-05,
                0.0,
                0.001,
                1.0,
            ],
            [
                -0.39859637855648306,
                0.8709489764975944,
                0.2873478855663454,
                0.8775825618903728,
                0.479425538604203,
                0.0,
                -0.13807921244038343,
                0.25275230299584717,
                -0.9576274872945841,
                1.0,
                1.0,
                -0.39859637855648306,
                0.8709489764975944,
                0.2873478855663454,
                0.8775825618903728,
                0.479425538604203,
                0.0,
                1.0,
                2.0,
            ],
            [
                -0.8969628300582488,
                -0.3359893958958339,
                -0.2873478855663454,
                -0.39859637855648306,
                0.8709489764975944,
                0.2873478855663454,
                0.1537435140353627,
                0.3723343456140114,
                -0.9152759512672909,
                1.0,
                1.0,
                -0.8969628300582488,
                -0.3359893958958339,
                -0.2873478855663454,
                -0.39859637855648306,
                0.8709489764975944,
                0.2873478855663454,
                1.0,
                3.0,
            ],
            [
                0.2805238471386536,
                -0.9483150748622418,
                0.14834045293024462,
                -0.8969628300582488,
                -0.3359893958958339,
                -0.2873478855663454,
                0.3224329819611792,
                -0.05246353097466939,
                -0.9451373180979005,
                1.0,
                1.0,
                0.2805238471386536,
                -0.9483150748622418,
                0.14834045293024462,
                -0.8969628300582488,
                -0.3359893958958339,
                -0.2873478855663454,
                1.0,
                4.0,
            ],
            [
                0.9367845000189781,
                -0.28978140392829665,
                0.19611613513818402,
                0.2805238471386536,
                -0.9483150748622418,
                0.14834045293024462,
                0.17354989635680937,
                -0.10188659515906286,
                -0.9795404816552677,
                1.0,
                1.0,
                0.9367845000189781,
                -0.28978140392829665,
                0.19611613513818402,
                0.2805238471386536,
                -0.9483150748622418,
                0.14834045293024462,
                1.0,
                5.0,
            ],
        ]);
        for (what, cone) in [("the crown", &cone), ("its twin", &twin)] {
            for far in [1e-10, 4e-10, 2e-9] {
                let far = Vec3::new(0.9987502653928781, 0.04997906939565204, far);
                // A refusal or no reading is an answer; a class must be On.
                if let Ok(Some(read)) = cone_read(&probes(&[far]), cone, fuzz_band()) {
                    assert_eq!(classes(&read), [SideCode::On], "{what} at {far:?}");
                }
            }
        }
    }

    /// **A crossing point is read at how well it is located**: a
    /// reference on a plane at a 1 m arm, and a direction 1e-3 rad off
    /// it. The crossing is the reference itself, but a move of the
    /// reference's arm point by δ moves it toward the direction by
    /// δ/1e-3, so it is read at no more than 1e-3 of the arm, not at the
    /// arm (PR 4289's second review). Both ends on the plane locate
    /// nothing.
    ///
    /// It pins refusal behaviour, not a class: `p` is the reader's own point,
    /// not an input, and the crossing is located to rounding, so a
    /// longer lever reads no class the exact oracle contradicts. The
    /// fuzz, with a family built to locate crossings from heights within
    /// the zero band at ε = 1e-12, found none (PR 4289's third review).
    #[test]
    fn a_crossing_point_is_read_at_how_well_it_is_located() {
        let o = Point3::new(0.0, 0.0, 0.0);
        let chord = |far: Vec3<f64>| Reach::Chord {
            base: o,
            far: o + far,
        };
        let (n, p) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        let d = Vec3::new(0.0, 1.0, -1e-3).normalize();
        let (x, lever) = crossing(d, chord(d), p, 1.0, n).unwrap();
        assert!(
            x.normalize().dot(p) > 1.0 - 1e-12,
            "the crossing is the reference"
        );
        assert!(lever <= 1e-3, "read at {lever}, not at the arm");
        let flat = Vec3::new(0.0, 1.0, 0.0);
        assert!(
            crossing(flat, chord(flat), p, 1.0, n).is_none(),
            "both ends on the plane"
        );
    }

    /// **An arc's side of a short bound is read at the bound's reach**:
    /// a bound 1 mm long, 1e-7 rad off the plane through the direction
    /// and the reference, each 1 m out. Moving the bound's far point by
    /// 1e-10 m puts it on that plane, so the reading is in band; read
    /// at the direction's reach and the reference's arm alone, it was
    /// decided. It pins refusal behaviour, not a class: the faces either side
    /// of the bound read it alike, so the parity holds (PR 4289's third
    /// review).
    #[test]
    fn an_arcs_side_of_a_short_bound_is_read_at_the_bounds_reach() {
        let o = Point3::new(0.0, 0.0, 0.0);
        let chord = |far: Vec3<f64>| Reach::Chord {
            base: o,
            far: o + far,
        };
        let d = Vec3::new(1.0, 0.0, 0.0);
        let arc = GreatArc {
            dir: d,
            reach: chord(d),
            p: Vec3::new(0.0, 1.0, 0.0),
            p_arm: 1.0,
        };
        let b = Vec3::new(0.6, 0.6, 1e-7).normalize();
        let read = arc_side(arc, b, chord(b * 1e-3), 1.0, fuzz_band());
        assert!(!matches!(read, Ok(Some(_))), "a 1 mm bound, read {read:?}");
    }

    /// **An arc's side of a bound is not read off the determinant's
    /// rounding**: a direction, a reference and a bound whose far points
    /// are exactly coplanar (`b = 1e6·p + d`, integers), the bound far
    /// along the reference, so the plane through the reference and the
    /// bound is levered by millions. The determinant of the three unit
    /// vectors rounds to a few ulp; levered, that reads decided at
    /// ε = 1e-12 unless the rounding is taken off first (PR 4289's
    /// second review, n4).
    ///
    /// It pins refusal behaviour, not a class: the faces either side of a
    /// bound read its determinant alike, so a rounded sign counts the
    /// crossing at one face or the other and the parity holds. No class
    /// reads differently without the rounding (PR 4289's third review).
    #[test]
    fn an_arcs_side_of_a_bound_is_not_read_off_its_rounding() {
        let o = Point3::new(0.0, 0.0, 0.0);
        let chord = |far: Vec3<f64>| Reach::Chord {
            base: o,
            far: o + far,
        };
        let band = Band::linear_at(Tol::witness(), 1e-12).unwrap();
        let (d, p_far) = (Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 5.0));
        let b = p_far * 1e6 + d;
        let arc = GreatArc {
            dir: d,
            reach: chord(d),
            p: p_far.normalize(),
            p_arm: 1e6,
        };
        let read = arc_side(arc, b.normalize(), chord(b), 1.0, band);
        assert!(!matches!(read, Ok(Some(_))), "coplanar, read {read:?}");
    }

    /// A fin 1e-4° wide with a 1 mm edge, and a probe 1.9e-7 rad off its
    /// link, exactly `In` (fuzz seed 1, cone 82, probe 81).
    fn shorter_arm_fin() -> (Vec<BoolSector<f64>>, Vec3<f64>) {
        let cone = fuzz_cone(&[
            [
                0.17364817766686433,
                1.5153662201878184e-07,
                0.9848077530122081,
                1.0,
                0.0,
                0.0,
                0.0,
                0.9999999999999881,
                -1.5387431867314058e-07,
                1.0,
                1.0,
                0.00017364817766686434,
                1.5153662201878184e-10,
                0.0009848077530122082,
                0.5,
                0.0,
                0.0,
                0.001,
                0.0,
            ],
            [
                0.999999999998477,
                1.7453292519934438e-06,
                0.0,
                0.17364817766686433,
                1.5153662201878184e-07,
                0.9848077530122081,
                1.745329251993423e-06,
                -0.999999999998465,
                -1.5387431867314066e-07,
                1.0,
                1.0,
                0.4999999999992385,
                8.726646259967219e-07,
                0.0,
                0.00017364817766686434,
                1.5153662201878184e-10,
                0.0009848077530122082,
                0.001,
                1.0,
            ],
            [
                -0.49240387650610384,
                0.8528685319524434,
                -0.17364817766693036,
                0.999999999998477,
                1.7453292519934438e-06,
                0.0,
                3.4821289096011875e-07,
                -0.1995112902404368,
                -0.9798955276285707,
                1.0,
                1.0,
                -0.24620193825305192,
                0.4264342659762217,
                -0.08682408883346518,
                0.4999999999992385,
                8.726646259967219e-07,
                0.0,
                0.5,
                2.0,
            ],
            [
                -0.4924038765061045,
                -0.852868531952443,
                0.17364817766693036,
                -0.49240387650610384,
                0.8528685319524434,
                -0.17364817766693036,
                -6.476292310140301e-17,
                -0.19951148327886362,
                -0.9798954883250905,
                1.0,
                1.0,
                -0.24620193825305225,
                -0.4264342659762215,
                0.08682408883346518,
                -0.24620193825305192,
                0.4264342659762217,
                -0.08682408883346518,
                0.5,
                3.0,
            ],
            [
                1.0,
                0.0,
                0.0,
                -0.4924038765061045,
                -0.852868531952443,
                0.17364817766693036,
                0.0,
                -0.19951148327886364,
                -0.9798954883250907,
                1.0,
                1.0,
                0.5,
                0.0,
                0.0,
                -0.24620193825305225,
                -0.4264342659762215,
                0.08682408883346518,
                0.5,
                4.0,
            ],
        ]);
        let far = Vec3::new(1.2249034581828435, -0.7761906779756366, 0.15803647000763973);
        (cone, far)
    }

    /// **A plane through a reference and a bound is read at the shorter
    /// of the two sectors' arms**: the fin ([`shorter_arm_fin`]), whose
    /// references lie within that span's band of the fin's bounds at
    /// the 1 mm arm. The arc to the fourth face's reference escalates at
    /// the fin's bound: read at the longer arm, the plane's normal passes
    /// the gate, though at the shorter arm it is not resolved.
    #[test]
    fn a_plane_through_a_reference_and_a_bound_is_read_at_the_shorter_arm() {
        let (cone, far) = shorter_arm_fin();
        let o = Point3::new(0.0, 0.0, 0.0);
        let (s0, s) = (&cone[3], &cone[1]);
        let arc = GreatArc {
            dir: far,
            reach: Reach::Chord {
                base: o,
                far: o + far,
            },
            p: (s0.start + s0.end).normalize(),
            p_arm: s0.arm,
        };
        match arc_side(arc, s.start, s.start_reach, s.arm, fuzz_band()) {
            Err(BooleanError::Escalated { diag, .. }) => {
                assert_eq!(diag.predicate, Some("bool_cone_arc_span"), "{diag:?}");
            }
            other => panic!("the span at the shorter arm escalates, got {other:?}"),
        }
    }

    /// **A probe whose arcs meet unread faces apart from their sectors
    /// is decided**: the fin's probe ([`shorter_arm_fin`]), whose every
    /// reference leaves some face unread, reads `In`, as it exactly is,
    /// through a reference whose unread faces lie decidedly apart from
    /// its arc ([`apart`]).
    #[test]
    fn a_probe_whose_arcs_meet_unread_faces_apart_from_their_sectors_is_decided() {
        let (cone, far) = shorter_arm_fin();
        let read = cone_read(&probes(&[far]), &cone, fuzz_band())
            .unwrap()
            .unwrap();
        assert_eq!(classes(&read), [SideCode::In], "the probe, decided apart");
    }

    /// **A reference on a face's plane beside a crossing inside its
    /// sector passes over**: a flat corner, every face on `z = 0` but
    /// one dented 5e-7 m at its far corner beside a 1 mm edge. That
    /// face's reference reads on the flat faces' plane at the 1 mm arm,
    /// though it lies 1e-6 rad off it, so the arc from a direction 2e-7
    /// rad below the flat crosses a flat face far from the reference,
    /// inside its sector. Read as crossing at the reference, the parity
    /// was one short, `In`; exactly `Out` (PR 4289's second review).
    #[test]
    fn a_reference_on_a_plane_beside_a_crossing_inside_the_sector_passes_over() {
        let cone = fuzz_cone(&[
            [
                0.9987502603949663,
                0.04997916927067833,
                0.0,
                0.9553364891254865,
                -0.2955202066613026,
                4.999999999999375e-07,
                7.287764486087944e-08,
                -1.456338067317127e-06,
                -0.9999999999989369,
                1.0,
                1.0,
                0.0009987502603949663,
                4.997916927067833e-05,
                0.0,
                0.9553364891254865,
                -0.2955202066613026,
                4.999999999999375e-07,
                0.001,
                0.0,
            ],
            [
                0.8775825618903728,
                0.479425538604203,
                0.0,
                0.9987502603949663,
                0.04997916927067833,
                0.0,
                0.0,
                0.0,
                -1.0,
                1.0,
                1.0,
                0.8775825618903728,
                0.479425538604203,
                0.0,
                0.0009987502603949663,
                4.997916927067833e-05,
                0.0,
                0.001,
                1.0,
            ],
            [
                -0.4161468365471424,
                0.9092974268256817,
                0.0,
                0.8775825618903728,
                0.479425538604203,
                0.0,
                0.0,
                0.0,
                -1.0,
                1.0,
                1.0,
                -0.4161468365471424,
                0.9092974268256817,
                0.0,
                0.8775825618903728,
                0.479425538604203,
                0.0,
                1.0,
                2.0,
            ],
            [
                -0.9364566872907963,
                -0.35078322768961984,
                0.0,
                -0.4161468365471424,
                0.9092974268256817,
                0.0,
                0.0,
                0.0,
                -1.0,
                1.0,
                1.0,
                -0.9364566872907963,
                -0.35078322768961984,
                0.0,
                -0.4161468365471424,
                0.9092974268256817,
                0.0,
                1.0,
                3.0,
            ],
            [
                0.2836621854632263,
                -0.9589242746631386,
                0.0,
                -0.9364566872907963,
                -0.35078322768961984,
                0.0,
                0.0,
                0.0,
                -1.0,
                1.0,
                1.0,
                0.2836621854632263,
                -0.9589242746631386,
                0.0,
                -0.9364566872907963,
                -0.35078322768961984,
                0.0,
                1.0,
                4.0,
            ],
            [
                0.9553364891254865,
                -0.2955202066613026,
                4.999999999999375e-07,
                0.2836621854632263,
                -0.9589242746631386,
                0.0,
                5.760914256723924e-07,
                1.704152842415664e-07,
                -0.9999999999998195,
                1.0,
                1.0,
                0.9553364891254865,
                -0.2955202066613026,
                4.999999999999375e-07,
                0.2836621854632263,
                -0.9589242746631386,
                0.0,
                1.0,
                5.0,
            ],
        ]);
        let far = Vec3::new(0.8253356149096618, 0.564642473395024, -1.99999999999996e-07);
        let read = cone_read(&probes(&[far]), &cone, fuzz_band())
            .unwrap()
            .unwrap();
        assert_eq!(classes(&read), [SideCode::Out], "exactly Out");
        let read = wedge_classes(&probes(&[far]), &cone, fuzz_band())
            .unwrap()
            .unwrap();
        assert_eq!(classes(&read), [SideCode::Out], "through wedge_classes");
    }

    /// **A crossing far from a reference on a face's plane is located,
    /// not taken at the reference**: a crown flat but for one corner
    /// dented beside a 1 mm edge, its faces beyond risen 3e-7. The dented
    /// face's reference reads on the flat face's plane at the 1 mm arm;
    /// the arc from a direction 2e-7 or 5e-7 rad under the face beyond
    /// crosses that flat face near the direction. Read as crossing at
    /// the reference, the cone and its void read the other way (PR
    /// 4289's second review, `crisp3.py`, cones 0 and 1).
    #[test]
    fn a_crossing_far_from_a_reference_on_a_plane_is_located() {
        use SideCode::{In, Out};
        let cone = fuzz_cone(&[
            [
                0.9987502603949663,
                0.04997916927067833,
                0.0,
                0.9999500004162609,
                -0.00999983333416262,
                8.994600971913053e-07,
                7.496875389758351e-07,
                -1.4981253904239103e-05,
                -0.9999999998875,
                1.0,
                1.0,
                0.0009987502603949663,
                4.997916927067833e-05,
                0.0,
                0.39998000016650437,
                -0.0039999333336650485,
                3.5978403887652213e-07,
                0.001,
                0.0,
            ],
            [
                0.8775825618903728,
                0.479425538604203,
                0.0,
                0.9987502603949663,
                0.04997916927067833,
                0.0,
                0.0,
                0.0,
                -1.0,
                1.0,
                1.0,
                0.3510330247561491,
                0.1917702154416812,
                0.0,
                0.0009987502603949663,
                4.997916927067833e-05,
                0.0,
                0.001,
                1.0,
            ],
            [
                -0.41614683654712364,
                0.9092974268256407,
                2.999999999999865e-07,
                0.8775825618903728,
                0.479425538604203,
                0.0,
                -1.4418885659857992e-07,
                2.63935931611458e-07,
                -0.9999999999999548,
                1.0,
                1.0,
                -0.16645873461884947,
                0.36371897073025633,
                1.199999999999946e-07,
                0.3510330247561491,
                0.1917702154416812,
                0.0,
                0.4,
                2.0,
            ],
            [
                -0.9364566872907544,
                -0.3507832276896041,
                -2.9999999999998654e-07,
                -0.41614683654712364,
                0.9092974268256407,
                2.999999999999865e-07,
                1.679750394648462e-07,
                4.0680009684340706e-07,
                -0.9999999999999032,
                1.0,
                1.0,
                -0.37458267491630176,
                -0.14031329107584165,
                -1.1999999999999462e-07,
                -0.16645873461884947,
                0.36371897073025633,
                1.199999999999946e-07,
                0.4,
                3.0,
            ],
            [
                0.28366218546321353,
                -0.9589242746630955,
                2.9999999999998654e-07,
                -0.9364566872907544,
                -0.3507832276896041,
                -2.9999999999998654e-07,
                3.938989729095771e-07,
                -1.963301602296555e-07,
                -0.9999999999999032,
                1.0,
                1.0,
                0.11346487418528542,
                -0.3835697098652382,
                1.1999999999999462e-07,
                -0.37458267491630176,
                -0.14031329107584165,
                -1.1999999999999462e-07,
                0.4,
                4.0,
            ],
            [
                0.9999500004162609,
                -0.00999983333416262,
                8.994600971913053e-07,
                0.28366218546321353,
                -0.9589242746630955,
                2.9999999999998654e-07,
                8.990360154454561e-07,
                -4.6904098933432147e-08,
                -0.9999999999995948,
                1.0,
                1.0,
                0.39998000016650437,
                -0.0039999333336650485,
                3.5978403887652213e-07,
                0.11346487418528542,
                -0.3835697098652382,
                1.1999999999999462e-07,
                0.4,
                5.0,
            ],
        ]);
        let probes = [
            // probe 1 (a0.6h-2e-07), 2e-07 rad off the face: read In before
            (
                Vec3::new(
                    0.4126678074548332,
                    0.28232123669751363,
                    -8.4987380690496e-08,
                ),
                Out,
            ),
            // probe 5 (a0.8h-2e-07), 2e-07 rad off the face: read In before
            (
                Vec3::new(
                    0.34835335467358053,
                    0.3586780454497592,
                    -5.556064782830146e-08,
                ),
                Out,
            ),
        ];
        reads_exactly("the dented crown", &cone, &probes);
        let void = fuzz_cone(&[
            [
                0.9999500004162609,
                -0.00999983333416262,
                8.994600971913053e-07,
                0.9987502603949663,
                0.04997916927067833,
                0.0,
                -7.496875389758351e-07,
                1.4981253904239103e-05,
                0.9999999998875,
                1.0,
                1.0,
                0.39998000016650437,
                -0.0039999333336650485,
                3.5978403887652213e-07,
                0.0009987502603949663,
                4.997916927067833e-05,
                0.0,
                0.001,
                0.0,
            ],
            [
                0.9987502603949663,
                0.04997916927067833,
                0.0,
                0.8775825618903728,
                0.479425538604203,
                0.0,
                0.0,
                0.0,
                1.0,
                1.0,
                1.0,
                0.0009987502603949663,
                4.997916927067833e-05,
                0.0,
                0.3510330247561491,
                0.1917702154416812,
                0.0,
                0.001,
                1.0,
            ],
            [
                0.8775825618903728,
                0.479425538604203,
                0.0,
                -0.41614683654712364,
                0.9092974268256407,
                2.999999999999865e-07,
                1.4418885659857992e-07,
                -2.63935931611458e-07,
                0.9999999999999548,
                1.0,
                1.0,
                0.3510330247561491,
                0.1917702154416812,
                0.0,
                -0.16645873461884947,
                0.36371897073025633,
                1.199999999999946e-07,
                0.4,
                2.0,
            ],
            [
                -0.41614683654712364,
                0.9092974268256407,
                2.999999999999865e-07,
                -0.9364566872907544,
                -0.3507832276896041,
                -2.9999999999998654e-07,
                -1.679750394648462e-07,
                -4.0680009684340706e-07,
                0.9999999999999032,
                1.0,
                1.0,
                -0.16645873461884947,
                0.36371897073025633,
                1.199999999999946e-07,
                -0.37458267491630176,
                -0.14031329107584165,
                -1.1999999999999462e-07,
                0.4,
                3.0,
            ],
            [
                -0.9364566872907544,
                -0.3507832276896041,
                -2.9999999999998654e-07,
                0.28366218546321353,
                -0.9589242746630955,
                2.9999999999998654e-07,
                -3.938989729095771e-07,
                1.963301602296555e-07,
                0.9999999999999032,
                1.0,
                1.0,
                -0.37458267491630176,
                -0.14031329107584165,
                -1.1999999999999462e-07,
                0.11346487418528542,
                -0.3835697098652382,
                1.1999999999999462e-07,
                0.4,
                4.0,
            ],
            [
                0.28366218546321353,
                -0.9589242746630955,
                2.9999999999998654e-07,
                0.9999500004162609,
                -0.00999983333416262,
                8.994600971913053e-07,
                -8.990360154454561e-07,
                4.6904098933432147e-08,
                0.9999999999995948,
                1.0,
                1.0,
                0.11346487418528542,
                -0.3835697098652382,
                1.1999999999999462e-07,
                0.39998000016650437,
                -0.0039999333336650485,
                3.5978403887652213e-07,
                0.4,
                5.0,
            ],
        ]);
        let probes = [
            // probe 1 (a0.6h-2e-07), 2e-07 rad off the face: read Out before
            (
                Vec3::new(
                    0.4126678074548332,
                    0.28232123669751363,
                    -8.4987380690496e-08,
                ),
                In,
            ),
            // probe 5 (a0.8h-2e-07), 2e-07 rad off the face: read Out before
            (
                Vec3::new(
                    0.34835335467358053,
                    0.3586780454497592,
                    -5.556064782830146e-08,
                ),
                In,
            ),
        ];
        reads_exactly("its void", &void, &probes);
    }

    /// **A direction's membership in a sector names no declaration**: a
    /// direction an in-band angle past the sector's start bound, read
    /// by `within` ahead of any declaration (the sector primitives take
    /// none), escalates as the sectors' coincidence with nothing read,
    /// and ends in its lever and the tolerance the gap gives.
    #[test]
    fn a_direction_on_a_sector_bound_escalates_with_no_declaration_read() {
        let b = band();
        let mid = (b.zero() + b.escalate()) / 2.0;
        let o = Point3::new(0.0, 0.0, 0.0);
        let (x, y) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let s = BoolSector {
            he: HalfEdgeKey::default(),
            start: x,
            end: y,
            start_reach: Reach::Chord {
                base: o,
                far: o + x,
            },
            end_reach: Reach::Chord {
                base: o,
                far: o + y,
            },
            face: FaceKey::default(),
            normal: OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true),
            arm: 1.0,
        };
        let err = within(
            &s,
            Vec3::new(1.0, -mid, 0.0),
            false,
            DeclarationRead::Moot,
            b,
        )
        .expect_err("an in-band direction escalates");
        let BooleanError::Escalated { decision, diag } = &err else {
            panic!("an escalation: {err:?}");
        };
        assert_eq!(
            *decision,
            BooleanDecision::Coincidence(Coincide::Sectors, DeclarationRead::Moot)
        );
        assert_eq!(diag.predicate, Some("bool_sector_within"));
        let text = err.to_string();
        assert!(
            text.starts_with("how two corners of the two solids overlap where they meet is ")
                && text.contains(
                    "Recourse: move the parts so they clearly meet or clearly stand apart there, \
                     or, if this gap is intended, tighten the tolerance below "
                )
                && !text.contains("declare"),
            "{text}"
        );
    }

    /// **Two parallel directions' sense is its own decision, and its
    /// decided zero refuses as its in-band arm does**: read at an arm in
    /// the band, and at one in the zero band, the cosine of two equal
    /// directions escalates as [`BooleanDecision::DirectionSense`] with
    /// the margin the funnel read, and both end in the corner's lever
    /// and the tolerance that margin gives (the decision passes on
    /// either definite sign, so a zero-band margin is a size a smaller
    /// tolerance decides). A clear arm reads the sense.
    #[test]
    fn a_direction_sense_refuses_its_decided_zero_as_its_in_band_arm() {
        let b = band();
        let (z, e) = (b.zero(), b.escalate());
        let u = Vec3::new(1.0, 0.0, 0.0);
        assert!(direction_sense(u, u, 1.0, b).expect("a clear arm reads the sense"));
        assert!(!direction_sense(u, -u, 1.0, b).expect("a clear arm reads the sense"));
        for arm in [(z + e) / 2.0, 0.5 * z] {
            let err = direction_sense(u, u, arm, b).expect_err("a short arm refuses");
            let BooleanError::Escalated { decision, diag } = &err else {
                panic!("{arm:e}: an escalation: {err:?}");
            };
            assert_eq!(*decision, BooleanDecision::DirectionSense, "{arm:e}");
            assert_eq!(diag.predicate, Some("bool_dir_same"), "{arm:e}");
            assert_eq!(
                diag.margin.diagnostic_f64_for_error_text().value(),
                Some(arm),
                "{arm:e}: the margin the funnel read"
            );
            let text = err.to_string();
            assert!(
                text.starts_with(
                    "whether two parallel directions at a corner point the same way or opposite \
                     ways is undecided: "
                ) && text.ends_with(&format!(
                    "Recourse: make the edges at the corner where the two faces meet span \
                     clearly more than the tolerance, or, if this length of the corner's shorter edge \
                     is intended, tighten the tolerance below {:e} m",
                    arm / (e / z)
                )),
                "{arm:e}: {text}"
            );
        }
    }

    /// **A direction sense's refusal is on the frame's escalation log**,
    /// its decided zero as its in-band arm (the second review's probe
    /// P3, adopted): each refusal is the funnel's own escalation, beside
    /// the verdict it decided, as `kstats_escalation_channel` pins for the
    /// lever-arm gates, so the log sees every refusal the Boolean raises.
    #[test]
    fn a_direction_senses_refusal_is_on_the_escalation_log() {
        use geom_core::k_stats::Bracket;
        let b = band();
        let u = Vec3::new(1.0, 0.0, 0.0);
        for (arm, decided) in [
            (0.5 * b.zero(), true),
            ((b.zero() + b.escalate()) / 2.0, false),
        ] {
            let bracket = Bracket::open();
            let err = direction_sense(u, u, arm, b).expect_err("a short arm refuses");
            let log = bracket.finish();
            let BooleanError::Escalated { diag, .. } = err else {
                panic!("{arm:e}: an escalation: {err:?}");
            };
            assert_eq!(
                log.escalations.iter().map(|e| e.source).collect::<Vec<_>>(),
                vec![diag],
                "{arm:e}: the refusal is the log's escalation"
            );
            assert_eq!(
                log.verdicts
                    .iter()
                    .map(|v| (v.predicate, v.sign))
                    .collect::<Vec<_>>(),
                if decided {
                    vec![("bool_dir_same", Sign::Zero)]
                } else {
                    vec![]
                },
                "{arm:e}: a decided zero is the verdict beside it"
            );
        }
    }

    /// The 15.7 sign resolution, mirror-pinned (F3): against a face
    /// with outward normal +z (material below), a direction with
    /// negative z ENTERS material ⇒ In; positive z ⇒ Out; in-plane ⇒
    /// On. Program 15.7's printed IN=+1 would flip the first two — the
    /// suspect side of ch. 15 erratum 4, resolved by derivation.
    #[test]
    fn mirror_check_side_codes() {
        let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let b = band();
        assert_eq!(
            side_code(
                Vec3::new(0.3, 0.0, -1.0),
                Reach::Bisector(1.0),
                n,
                super::NO_CURVATURE(),
                b
            )
            .unwrap(),
            SideCode::In
        );
        assert_eq!(
            side_code(
                Vec3::new(0.3, 0.0, 1.0),
                Reach::Bisector(1.0),
                n,
                super::NO_CURVATURE(),
                b
            )
            .unwrap(),
            SideCode::Out
        );
        assert_eq!(
            side_code(
                Vec3::new(1.0, 2.0, 0.0),
                Reach::Bisector(1.0),
                n,
                super::NO_CURVATURE(),
                b
            )
            .unwrap(),
            SideCode::On
        );
    }

    fn sector(start: [f64; 3], end: [f64; 3], normal: [f64; 3]) -> BoolSector<f64> {
        BoolSector {
            he: HalfEdgeKey::default(),
            start: Vec3::from_array(start),
            end: Vec3::from_array(end),
            start_reach: Reach::Extent(1.0),
            end_reach: Reach::Extent(1.0),
            face: FaceKey::default(),
            normal: OutwardNormal::from_chart(Vec3::from_array(normal), true),
            arm: 1.0,
        }
    }

    /// `within`: interior yes, exterior no, boundary graze counts
    /// non-strictly and not strictly.
    #[test]
    fn within_convex_sector() {
        let s = sector([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        let b = band();
        let mid = Vec3::new(1.0, 1.0, 0.0).normalize();
        assert!(within(&s, mid, true, DeclarationRead::Moot, b).unwrap());
        assert!(
            !within(
                &s,
                Vec3::new(-1.0, -0.5, 0.0),
                false,
                DeclarationRead::Moot,
                b
            )
            .unwrap()
        );
        assert!(
            within(
                &s,
                Vec3::new(1.0, 0.0, 0.0),
                false,
                DeclarationRead::Moot,
                b
            )
            .unwrap()
        );
        assert!(!within(&s, Vec3::new(1.0, 0.0, 0.0), true, DeclarationRead::Moot, b).unwrap());
    }

    /// `sectoroverlap`: strict overlap yes; identical sectors yes;
    /// touch-only bound sharing NO (flows to the ON machinery, not to a
    /// fake coplanar pair).
    #[test]
    fn coplanar_overlap_cases() {
        let b = band();
        let s1 = sector([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        let deep = sector([0.5, 0.5, 0.0], [-0.5, 0.5, 0.0], [0.0, 0.0, 1.0]);
        assert!(sector_overlap(&s1, &deep, b).unwrap());
        assert!(sector_overlap(&s1, &s1.clone(), b).unwrap());
        let touch = sector([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert!(!sector_overlap(&s1, &touch, b).unwrap());
    }

    /// **The curvature charge, on the R1 review's witness.** A HOLE
    /// wall of radius 1 (material outside, so the outward normal points
    /// at the axis) and the direction `(-0.1, 1, 0)/|·|`, a definite
    /// first-order `Exits` (`d̂·n̂ = 0.0995`) that crosses back into the
    /// material before `l = 0.5`. The charge certifies `Out` at its
    /// peak, `0.0995/2`, where the ray is inside the hole.
    #[test]
    fn the_hole_wall_witness_certifies_its_side_near_the_vertex() {
        let b = band();
        let n = OutwardNormal::from_chart(Vec3::new(-1.0, 0.0, 0.0), true);
        let base = geom_core::Point3::new(1.0, 0.0, 0.0);
        let rho = |d: Vec3<f64>, l: f64| {
            let q = base + d.normalize() * l;
            q.x.hypot(q.y)
        };
        let d = Vec3::new(-0.1, 1.0, 0.0);
        assert_eq!(
            side_code(d, Reach::Bisector(0.5), n, 1.0, b).unwrap(),
            SideCode::Out,
            "a definite slope certifies at slope·lever/2"
        );
        let peak = d.normalize().dot(n.vec()) / 2.0;
        assert!(rho(d, peak) < 1.0, "the certified point is in the hole");
        assert!(rho(d, 0.5) > 1.0, "the arm point has crossed back");
    }

    /// **The charge's separation `Q = s²·R/4`, against the band, at
    /// radii far from 1.** Each row picks the slope `s` that puts `Q`
    /// at a stated multiple of the band, so a charge that reads a
    /// length where it should read `sqrt(band/R)` (or the reverse)
    /// lands on the wrong arm. The rows, and the slip each pins:
    ///
    /// - `Q = 0.3·zero`, a long reach: the near-tangent residue refuses
    ///   `CurvedSectorSideUnsupported`, although the first-order reading
    ///   is definite at the reach;
    /// - `Q = 0.7·escalate`: the separation is in the band and escalates;
    ///   a charge without the sagitta reads `2Q` and certifies;
    /// - `Q = 1.2·escalate`, a long reach: certifies; a charge read at
    ///   `l*/2` sees `0.75·Q` and escalates;
    /// - `Q = 1.2·escalate`, reach `l*/2`: the reach caps the reading
    ///   at `0.75·Q`, which escalates; an uncapped charge certifies.
    #[test]
    fn the_charge_resolves_the_slope_against_the_band_at_every_radius() {
        let charge_escalated = |r: Result<SideCode, BooleanError>| {
            matches!(r, Err(BooleanError::Escalated { diag, .. })
                if diag.predicate == Some("bool_pierce_sector_side_curved"))
        };
        let b = band();
        let n = OutwardNormal::from_chart(Vec3::new(-1.0, 0.0, 0.0), true);
        let (zero, esc) = (b.zero(), b.escalate());
        for r in [0.01_f64, 100.0] {
            // The direction with `d̂·n̂ = s` (an `Exits`), the peak `l*`.
            let at = |q: f64| {
                let s = (4.0 * q / r).sqrt();
                (Vec3::new(-s, (1.0 - s * s).sqrt(), 0.0), s * r / 2.0)
            };
            let code = |q: f64, reach_over_peak: f64| {
                let (d, peak) = at(q);
                side_code(d, Reach::Bisector(peak * reach_over_peak), n, r, b)
            };
            // The residue: the reach is long enough that the first-order
            // reading itself is definite (`s·reach ≥ escalate`).
            let (_, peak) = at(0.3 * zero);
            let long = 2.0 * esc / (4.0 * 0.3 * zero / r).sqrt() / peak;
            assert!(
                long > 1.0,
                "R = {r}: the residue row's reach is past the peak"
            );
            assert!(
                matches!(
                    code(0.3 * zero, long),
                    Err(BooleanError::CurvedSectorSideUnsupported { .. })
                ),
                "R = {r}: a slope inside 2·sqrt(zero/R) refuses"
            );
            assert!(
                charge_escalated(code(0.7 * esc, 4.0)),
                "R = {r}: a separation in the band escalates"
            );
            assert_eq!(
                code(1.2 * esc, 4.0).unwrap(),
                SideCode::Out,
                "R = {r}: a separation past the band certifies at the peak"
            );
            assert!(
                charge_escalated(code(1.2 * esc, 0.5)),
                "R = {r}: a reach short of the peak caps the reading"
            );
        }
        // A plane takes no charge: the shallowest of these is the truth.
        let (d, peak) = {
            let s = (4.0 * 0.3 * zero / 0.01).sqrt();
            (Vec3::new(-s, (1.0 - s * s).sqrt(), 0.0), s * 0.01 / 2.0)
        };
        assert_eq!(
            side_code(d, Reach::Bisector(peak * 1e6), n, NO_CURVATURE::<f64>(), b).unwrap(),
            SideCode::Out,
            "against a plane the first-order reading is the truth"
        );
    }

    /// **A declared-`Tangent` sector pair's second-order side escalates
    /// as the side it reads, with no declaration**, on a real raise: a
    /// unit sphere's sector against a plane it touches, read over an arm
    /// at which the relative curvature's sagitta lies in the band. The
    /// arm clears its gate, so the reading refuses; the pair is declared
    /// already, so the refusal names which way the face curves away from
    /// the other, the move that decides it and the tolerance, and no
    /// declaration.
    #[test]
    fn a_tangent_pair_side_escalates_as_the_side_it_reads() {
        let b = band();
        let mid = (b.zero() + b.escalate()) / 2.0;
        let (p, d) = (Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let ball = geom::Surface::Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let floor = crate::test_support_fixtures::plane(
            &[p, Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
            Tol::witness(),
        );
        let accel = |s: &geom::Surface<f64>| {
            -geom_brep::implicit_hessian_form(s, p, d)
                / geom_brep::implicit_gradient(s, p).dot(n.vec())
        };
        let arm = (2.0 * mid / (accel(&ball) - accel(&floor)).abs()).sqrt();
        let err = tangent_relative_side(
            &ball,
            &floor,
            n,
            p,
            d,
            arm,
            DeclarationRead::Spent(BooleanCoincidence::TANGENT),
            b,
        )
        .expect_err("an in-band sagitta escalates the reading");
        let BooleanError::Escalated { decision, diag } = err else {
            panic!("the reading escalates: {err:?}");
        };
        assert_eq!(
            decision,
            BooleanDecision::Coincidence(
                Coincide::TangentSide,
                DeclarationRead::Spent(BooleanCoincidence::TANGENT)
            )
        );
        assert_eq!(diag.predicate, Some("tangent_sector_order2"));
        let text = BooleanError::Escalated { decision, diag }.to_string();
        assert!(
            text.starts_with(
                "which way a face of one solid curves away from a face of the other that it \
                 touches is undecided: "
            ) && text.contains(
                "Recourse: make one face clearly curve away from the other where they touch, or \
                 make both curve alike there, or, if this difference in bend is intended, \
                 tighten the tolerance"
            ) && !text.contains("declare"),
            "{text}"
        );
    }

    /// **A lever arm gate escalates as its own decision, on a real
    /// raise**: a bound read over a curved edge whose extent lies in the
    /// band reaches `enters_material`'s arm rung before any side is
    /// read. The refusal names the arm's question and its lever, quotes
    /// the departure from the face the arm meters (the edge at 45°, so
    /// `extent/√2`), offers the tolerance that departure gives, which
    /// decides the side as well as the arm, and no declaration: no face
    /// pair names an edge's length. An extent that decides zero is the
    /// same decision's band-decided arm: it quotes its departure's
    /// decided margin and offers the tolerance it gives, not a kernel
    /// bug. A clear extent reads the side, and the same routing sends
    /// that reading's escalation to the coincidence it is, which no
    /// declaration is read ahead of.
    #[test]
    fn a_lever_arm_gate_escalates_as_its_own_decision() {
        use super::super::{Coincide, LeverArm};
        use test_utils::refusal::{recourse_markers, stage_prefixes, subjectless_escalations};
        let b = band();
        let (z, e) = (b.zero(), b.escalate());
        let mid = (z + e) / 2.0;
        let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let departure = |extent: f64| Vec3::new(1.0, 0.0, 1.0).normalize().dot(n.vec()) * extent;
        let err = side_code(
            Vec3::new(1.0, 0.0, 1.0),
            Reach::Extent(mid),
            n,
            NO_CURVATURE(),
            b,
        )
        .expect_err("an in-band extent escalates the arm gate");
        let BooleanError::Escalated { decision, diag } = err else {
            panic!("the arm gate escalates: {err:?}");
        };
        assert_eq!(decision, BooleanDecision::LeverArm(LeverArm::SectorSide));
        assert_eq!(diag.predicate, Some("enters_material_rise"));
        let text = BooleanError::Escalated { decision, diag }.to_string();
        assert_eq!(recourse_markers(&text), 1, "{text}");
        assert!(
            subjectless_escalations(&text).is_empty() && stage_prefixes(&text, &[]).is_empty(),
            "{text}"
        );
        assert!(
            text.starts_with(
                "whether an edge at a corner is long enough to read which side of a face it \
                 leaves on is undecided: "
            ) && text.ends_with(&format!(
                "Recourse: make the edges at the corner where the two faces meet span clearly \
                 more than the tolerance, or, if this edge's length or rise is intended, tighten \
                 the tolerance below {:e} m",
                departure(mid) / (e / z)
            )) && !text.contains("declare"),
            "{text}"
        );
        // The decided-zero arm: an extent inside the zero band.
        let short = 0.5 * z;
        let err = side_code(
            Vec3::new(1.0, 0.0, 1.0),
            Reach::Extent(short),
            n,
            NO_CURVATURE(),
            b,
        )
        .expect_err("a zero-band extent refuses the arm gate");
        let text = err.to_string();
        assert!(
            matches!(
                err,
                BooleanError::Escalated {
                    decision: BooleanDecision::LeverArm(LeverArm::SectorSide),
                    ..
                }
            ) && text.contains(&format!(
                "margin {:e} lies within the zero band",
                departure(short)
            )) && text.ends_with(&format!(
                "if this edge's length or rise is intended, tighten the tolerance below {:e} m",
                departure(short) / (e / z)
            )) && !text.contains("kernel bug"),
            "{text}"
        );
        // A bound in the face's plane departs by exactly zero, which reads
        // `On` at every tolerance that decides the arm: the arm binds, and
        // the refusal quotes the edge's length under the arm's own name
        // (the coincfr4 review's in-plane pose).
        let err = side_code(
            Vec3::new(1.0, 0.0, 0.0),
            Reach::Extent(mid),
            n,
            NO_CURVATURE(),
            b,
        )
        .expect_err("an in-band extent escalates the arm gate");
        let BooleanError::Escalated { decision, diag } = err else {
            panic!("the arm gate escalates: {err:?}");
        };
        assert_eq!(decision, BooleanDecision::LeverArm(LeverArm::SectorSide));
        assert_eq!(diag.predicate, Some("enters_material_arm"));
        let text = BooleanError::Escalated { decision, diag }.to_string();
        assert!(
            text.contains(&format!("margin {mid:e} lies inside the ambiguity band"))
                && text.ends_with(&format!(
                    "if this edge's length or rise is intended, tighten the \
                     tolerance below {:e} m",
                    mid / (e / z)
                )),
            "{text}"
        );
        // The reading's own escalation, over a clear extent: a direction
        // in band off the face's plane is the coincidence it names.
        let err = side_code(
            Vec3::new(1.0, 0.0, mid),
            Reach::Extent(1.0),
            n,
            NO_CURVATURE(),
            b,
        )
        .expect_err("an in-band elevation escalates the reading");
        assert!(
            matches!(
                err,
                BooleanError::Escalated {
                    decision: BooleanDecision::Coincidence(
                        Coincide::SectorSide,
                        DeclarationRead::Moot
                    ),
                    ..
                }
            ),
            "{err:?}"
        );
    }

    /// A pair whose normals agree at the shorter arm is a
    /// near-coincidence, and goes to the carrier ladder as one, every
    /// code On: a face with a 1 mm edge on the other face's plane and a
    /// 10 m edge dipping `500·ε` below it. Read at its far vertex, the
    /// long edge is In, and a record carrying that code beside the short
    /// edge's On is half a crossing that no germ pairing closes (the
    /// corner witness in `tests/contact9_side_codes.rs` refused as an
    /// invariant that way).
    #[test]
    fn a_pair_parallel_at_a_short_arm_goes_to_the_ladder_as_a_coincidence() {
        let dip = 500.0 * Tol::witness().eps();
        let o = Point3::new(0.0, 0.0, 0.0);
        let chord = |v: [f64; 3]| Reach::Chord {
            base: o,
            far: Point3::from_array(v),
        };
        let (a, b) = ([10.0, 1.0, -dip], [0.2e-3, 1e-3, 0.0]);
        let (va, vb) = (Vec3::from_array(a), Vec3::from_array(b));
        let tilted = BoolSector {
            start: vb.normalize(),
            end: va.normalize(),
            start_reach: chord(b),
            end_reach: chord(a),
            normal: OutwardNormal::from_chart(-va.cross(vb).normalize(), true),
            arm: vb.norm(),
            ..sector([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0])
        };
        let top = BoolSector {
            start_reach: chord([10.0, 0.0, 0.0]),
            end_reach: chord([0.0, 10.0, 0.0]),
            arm: 10.0,
            ..sector([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0])
        };
        let recs = pair_search(&[tilted], &[top], band()).unwrap();
        assert_eq!(recs.len(), 1, "the sectors overlap");
        let on = (SideCode::On, SideCode::On);
        assert_eq!((recs[0].sa, recs[0].sb), (on, on), "a coincidence record");
    }

    /// The generic pair search on two orthogonal quarter-sector fans:
    /// records carry transition codes.
    #[test]
    fn pair_search_generic_crossing() {
        // A-sector in the xy-plane (normal +z), sweeping +x → +y CCW
        // around +z; B-sector in the zx-plane (normal +y), sweeping
        // +z → +x CCW around +y. They share the boundary ray +x.
        let a = sector([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        let bsec = sector([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let recs = pair_search(&[a], &[bsec], band()).unwrap();
        assert_eq!(recs.len(), 1);
        let r = recs[0];
        // A's start (+x) lies in B's face plane: On; A's end (+y) has
        // dot(+y, n_b=+y) > 0: Exits ⇒ Out. B's start (+z): dot with
        // n_a=+z > 0 ⇒ Out; B's end (+x): On.
        assert_eq!(r.sa, (SideCode::On, SideCode::Out));
        assert_eq!(r.sb, (SideCode::Out, SideCode::On));
    }

    // -----------------------------------------------------------------
    // The second-order Tangent lump (M9-3 PR-A item 4): three-outcome
    // honest on the existing `tangent_sector_order2` rows, driven on
    // raw carriers through the DEV-1 locus.
    // -----------------------------------------------------------------

    fn plate_top() -> geom::Surface<f64> {
        crate::fixtures::plane_surface(
            geom_core::Point3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        )
    }

    /// A ball over the fixtures' plate and cylinders: their verdicts are
    /// exact, so the extent only has to enclose them.
    fn fixture_reach() -> geom_brep::ExtentBall<f64> {
        geom_brep::ExtentBall::new(geom_core::Point3::new(2.0, 0.5, 1.0), 4.0)
    }

    /// A y-axis cylinder at height `zc`, radius `r`.
    fn y_cyl(zc: f64, r: f64) -> geom::Surface<f64> {
        geom::Surface::Cylinder {
            origin: geom_core::Point3::new(2.0, 0.0, zc),
            axis: Vec3::new(0.0, 1.0, 0.0),
            radius: r,
            u_ref: Vec3::new(0.0, 0.0, 1.0),
        }
    }

    /// A cylinder resting ON a plate top (external tangency along the
    /// ruling): the wall sector definitely curves AWAY from the
    /// plate's material ⇒ Out; the mirrored question (the plate's
    /// sector against the cylinder's material) is Out too.
    #[test]
    fn tangent_lump_external_tangency_is_out_both_ways() {
        let b = band();
        let p = geom_core::Point3::new(2.0, 0.5, 1.0);
        // The plate top's outward normal at the touch: +z.
        let plate_out = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let lump = tangent_lump(
            &y_cyl(1.5, 0.5),
            &plate_top(),
            fixture_reach(),
            plate_out,
            p,
            super::super::BooleanOp::Union,
            Operand::B,
            FaceKey::default(),
            0.5,
            DeclarationRead::Spent(BooleanCoincidence::TANGENT),
            b,
        )
        .unwrap();
        assert_eq!(lump, SideCode::Out);
        // The cylinder wall's outward normal at the bottom ruling: -z
        // (radially away from the axis, solid wall).
        let wall_out = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, -1.0), true);
        let lump = tangent_lump(
            &plate_top(),
            &y_cyl(1.5, 0.5),
            fixture_reach(),
            wall_out,
            p,
            super::super::BooleanOp::Union,
            Operand::A,
            FaceKey::default(),
            0.5,
            DeclarationRead::Spent(BooleanCoincidence::TANGENT),
            b,
        )
        .unwrap();
        assert_eq!(lump, SideCode::Out);
    }

    /// Internal tangency with the sector INSIDE the other body's
    /// material (a thin solid cylinder internally tangent inside a
    /// fat one): the thin wall curves definitely INTO the fat one's
    /// material ⇒ In.
    #[test]
    fn tangent_lump_nested_internal_tangency_is_in() {
        let b = band();
        // Fat solid cylinder r=0.5 about (x=2, z=1.5); thin r=0.25
        // about (x=2, z=1.25); both touch z=1 at the shared ruling.
        let p = geom_core::Point3::new(2.0, 0.5, 1.0);
        let fat_out = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, -1.0), true);
        let lump = tangent_lump(
            &y_cyl(1.25, 0.25),
            &y_cyl(1.5, 0.5),
            fixture_reach(),
            fat_out,
            p,
            super::super::BooleanOp::Union,
            Operand::A,
            FaceKey::default(),
            0.25,
            DeclarationRead::Spent(BooleanCoincidence::TANGENT),
            b,
        )
        .unwrap();
        assert_eq!(lump, SideCode::In);
    }

    /// Three-outcome honesty on the metered row: the SAME external
    /// tangency at three lever arms — definite (Out), in-band
    /// (escalates, an osculating pair is a sliver at this ε), and
    /// exactly-zero displacement (the isolated osculating residue the
    /// verified declaration bridges: the Eq. 15.3 minus-lump, In for
    /// a Union A-sector).
    #[test]
    fn tangent_lump_is_three_outcome_honest_on_the_arm() {
        let b = band();
        let p = geom_core::Point3::new(2.0, 0.5, 1.0);
        let plate_out = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let run = |arm: f64| {
            tangent_lump(
                &y_cyl(1.5, 0.5),
                &plate_top(),
                fixture_reach(),
                plate_out,
                p,
                super::super::BooleanOp::Union,
                Operand::A,
                FaceKey::default(),
                arm,
                DeclarationRead::Spent(BooleanCoincidence::TANGENT),
                b,
            )
        };
        // sagitta = kappa_rel * arm^2 / 2 = arm^2 (kappa_rel = 2
        // here), so the arms are derived from the run's band and the
        // three rows hold at every sampled ε.
        assert_eq!(run(0.5).unwrap(), SideCode::Out);
        let inband_arm = (b.zero() * b.escalate()).sqrt().sqrt();
        match run(inband_arm) {
            Err(BooleanError::Escalated { diag, .. }) => {
                assert_eq!(diag.predicate, Some("tangent_sector_order2"));
            }
            other => panic!("an in-band sagitta must escalate: {other:?}"),
        }
        let zero_arm = (b.zero() * 0.5).sqrt();
        assert_eq!(run(zero_arm).unwrap(), SideCode::In);
    }

    /// The self-contradiction and out-of-lane arms stay typed: a
    /// definitely-apart pair at the lump site is the classification
    /// invariant family; a kind pair outside the DEV-1 lane keeps the
    /// C5 typed refusal.
    #[test]
    fn tangent_lump_refuses_typed_off_the_lane() {
        let b = band();
        let p = geom_core::Point3::new(2.0, 0.5, 1.0);
        let plate_out = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        match tangent_lump(
            &y_cyl(2.5, 0.5),
            &plate_top(),
            fixture_reach(),
            plate_out,
            p,
            super::super::BooleanOp::Union,
            Operand::A,
            FaceKey::default(),
            0.5,
            DeclarationRead::Spent(BooleanCoincidence::TANGENT),
            b,
        ) {
            Err(BooleanError::ClassificationInvariant { .. }) => {}
            other => panic!("definitely-apart carriers at a lump site: {other:?}"),
        }
        let sphere = geom::Surface::Sphere {
            center: geom_core::Point3::new(2.0, 0.5, 2.0),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        match tangent_lump(
            &sphere,
            &plate_top(),
            fixture_reach(),
            plate_out,
            p,
            super::super::BooleanOp::Union,
            Operand::A,
            FaceKey::default(),
            0.5,
            DeclarationRead::Spent(BooleanCoincidence::TANGENT),
            b,
        ) {
            Err(BooleanError::CurvedBooleanUnsupported { .. }) => {}
            other => panic!("sphere tangency is outside the DEV-1 lane: {other:?}"),
        }
    }
}

#[cfg(test)]
mod cone_fuzz;

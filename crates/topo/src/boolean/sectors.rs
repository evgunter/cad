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
//! bound = the next orbit half-edge's departure: a line's chord, a
//! curved edge's tangent at the vertex), so `sectors[k].start ==
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
//! `Tangent ⇒ On` in the sector passes; the vertex-on-face pass reads a
//! conic's `Tangent` against a plane face by its bend ([`pierce_side`]).
//! Program 15.7's printed `IN = +1` (positive dot ⇒ IN)
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
//! are exactly what must flow into the ON machinery) — and neither
//! sector's two side codes read one definite side of the other's plane
//! ([`one_side`]). `within` reads the line's side of each bound at that
//! bound's reach ([`within_reaching`]), the length its code reads the
//! bound at, so a line beside a long bound is decided as its code is
//! however short the sector's other bound; where the two still differ
//! (a curved bound's code reads the edge, `within` its tangent) a
//! crossing pair sides with the codes, because they are what the germ
//! machinery consumes. The coplanar test reads the normals at the
//! sector's arm, because a coplanar pair never reads its codes and goes
//! to the carrier ladder, which certifies the coincidence over the
//! whole sector. Coplanar pairs
//! (`n_a × n_b` ≈ 0) go to `sector_overlap` — the unprinted procedure,
//! designed here: overlap iff some bound of one lies strictly within
//! the other, or the two sectors' bounds are pairwise parallel (the
//! identical / identical-reversed region cases); touch-only sharing of
//! a single bound is NOT overlap. Where the shared bound is a conic, the
//! two can overlap one order down, in a crescent from the vertex, which
//! a union of two plane faces refuses ([`crescent_refusal`]).

use geom_brep::LeverBounds;
#[cfg(test)]
use geom_brep::enters_material;
use geom_brep::{
    ConicBend, ConicDeparture, EntersMaterial, OutwardNormal, enters_material_reported,
};
use geom_core::k_stats::{Magnitude, NonzeroSign};
use geom_core::{Band, Decide, Decided, Margin, Point3, Sign, Vec3};

use super::refusal_routes::{BOUNDS_PARALLEL, Parallelism};
use super::{
    BooleanDecision, BooleanError, Coincide, DeclarationRead, LeverArm, Operand, SideCode,
    UnreadNeighbourhood,
};
use crate::body::Body;
use crate::chord_join::StaleSite;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::live::{Proven, linked, proven};
use crate::sector_face::{SectorCarrier, SectorFaceError, VertexFace};
use crate::sector_shape::{
    CornerShape, FullTurn, SectorFault, SectorShape, SlitTips, TurnOrder, corner_shape,
};
use crate::validate::decide;
use geom_brep::recourse::Refused;

/// What stands behind a sector bound, which decides how its side of a
/// plane is read ([`side_code`]). A bound is its edge's departure from
/// the base vertex: a line's chord, a curved edge's tangent there.
#[derive(Clone, Copy, Debug)]
pub(super) enum Reach<T: geom_core::Real> {
    /// A line edge: its far vertex, read in metres off the base vertex.
    Chord {
        /// The sector's base vertex.
        base: Point3<T>,
        /// The edge's other end.
        far: Point3<T>,
    },
    /// A probe direction with no edge behind it, levered at a stated
    /// arm (`vtxfac`'s interior probes).
    Extent(T),
    /// A curved edge — a conic, a spiric or a spline: its departure
    /// tangent at the base vertex, levered at the edge's own extent,
    /// and how it bends off that tangent
    /// ([`ConicBend`]), which [`pierce_side`] reads where the tangent lies in
    /// a plane face.
    Conic {
        /// The edge's extent from below ([`geom_brep::edge_extent_inf`]):
        /// no longer than its arc, so a point at that arc length lies on
        /// the edge.
        extent: T,
        /// The edge's bend at the base vertex.
        bend: ConicBend<T>,
    },
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
            Self::Extent(l) | Self::Conic { extent: l, .. } | Self::Bisector(l) => l,
        }
    }
}

impl<T: Decide> Reach<T> {
    /// The bound's signed first-order departure from the plane through
    /// its base vertex with unit normal `n`, as [`side_code`] reads it: a
    /// line edge's far vertex, in metres; any other bound's direction
    /// `dir` levered at its reach ([`rise`]). The vertex-on-face pass
    /// quotes it for a sector whose bounds tie but whose tilt it cannot
    /// decide (`bool_sector_coplanar`): the steeper bound's first-order
    /// departure is what a smaller tolerance reads off the plane. A
    /// conic's bend is [`conic_side`]'s.
    pub(super) fn departure(self, dir: Vec3<T>, n: Vec3<T>) -> T {
        match self {
            Self::Chord { base, far } => crate::sector_shape::plane_offset(base, n, far),
            Self::Extent(l) | Self::Conic { extent: l, .. } | Self::Bisector(l) => rise(dir, n, l),
        }
    }
}

/// A direction's first-order rise off the plane with unit normal `n`,
/// levered at `arm`: `d̂·n̂·arm`, in metres.
fn rise<T: Decide>(dir: Vec3<T>, n: Vec3<T>, arm: T) -> T {
    Margin::levered(dir.normalize().dot(n), arm).value()
}

/// One (convex) sector of a vertex neighborhood.
#[derive(Clone, Debug)]
pub(super) struct BoolSector<T: geom_core::Real> {
    /// The orbit half-edge the sector follows (CW-after `he`).
    pub he: HalfEdgeKey,
    /// CCW-first bound direction (the next orbit half-edge's departure,
    /// or a bisector).
    pub start: Vec3<T>,
    /// CCW-last bound direction (this entry's own half-edge's departure,
    /// or a bisector).
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
    /// The metering arm: the shorter of its bounds' reaches, in metres.
    pub arm: T,
}

/// **Whether a face a bound's side is read against is a plane**, which
/// decides what a bound in its tangent plane at the vertex means
/// ([`pierce_side`]): on a plane, the bound lies in the face, and a
/// conic tangent to it is read one order down; on a curved face, a
/// bound in its tangent plane rides the face to first order, and is
/// `On`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum FaceShape {
    /// The face lies on a plane.
    Plane,
    /// The face lies on a curved surface.
    Curved,
}

impl FaceShape {
    /// The shape of a face on `surface`.
    pub(super) fn of<T: geom_core::Real>(surface: &geom::Surface<T>) -> Self {
        Self::of_kind(surface.kind())
    }

    /// [`Self::of`], from the surface's kind.
    pub(super) fn of_kind(kind: geom::SurfaceKind) -> Self {
        match kind {
            geom::SurfaceKind::Plane => Self::Plane,
            _ => Self::Curved,
        }
    }

    /// The shape of `face` of `body`. A face the classification reached
    /// has a carrier, so one without is the invariant `what` names.
    pub(super) fn of_face<T: geom_core::Real>(
        body: &Body<T>,
        face: FaceKey,
        what: &'static str,
    ) -> Result<Self, BooleanError> {
        crate::readback::face_carrier_kind(body, face)
            .map(Self::of_kind)
            .map_err(|_| BooleanError::ClassificationInvariant { what })
    }
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

    /// The two bounds, each with the length it reaches from the base
    /// vertex: what a per-bound reading levers at.
    pub(super) fn reached_bounds(&self) -> [(Vec3<T>, T); 2] {
        [
            (self.start, self.start_reach.length()),
            (self.end, self.end_reach.length()),
        ]
    }

    /// The sector's middle direction, unnormalized: its bounds' sum,
    /// inside the sector, which is convex.
    pub(super) fn middle(&self) -> Vec3<T> {
        self.start + self.end
    }

    /// The farther of the sector's two bounds' reaches, in metres: how
    /// far from the base vertex the sector's own geometry is known.
    pub(super) fn span(&self) -> T {
        self.start_reach.length().max(self.end_reach.length())
    }
}

impl<T: Decide> BoolSector<T> {
    /// **Whether the sector's bounds read off the plane through its
    /// vertex with unit normal `n`** (`bool_sector_coplanar`), on the
    /// steeper bound's first-order departure ([`Reach::departure`]):
    /// what a smaller tolerance reads off the plane first, and the
    /// sector with it. `Err` where it does not decide off, with that
    /// margin.
    pub(super) fn bounds_off(
        &self,
        n: Vec3<T>,
        band: Band,
    ) -> Result<NonzeroSign, geom_core::Indeterminate> {
        let steeper = self
            .start_reach
            .departure(self.start, n)
            .abs()
            .max(self.end_reach.departure(self.end, n).abs());
        crate::validate::decide_nonzero("bool_sector_coplanar", Margin::of(steeper), band)
    }
}

/// One corner of a vertex neighborhood as its orbit reads it, before
/// any subdivision: the corner after orbit half-edge `he`, sweeping CCW
/// around its face's outward normal from `start` (the next orbit
/// half-edge's departure) to `end` (`he`'s own), each unnormalized and
/// as [`Reach`] says it was read: a line's chord, a curved edge's
/// tangent at the vertex.
pub(super) struct OrbitCorner<T: geom_core::Real> {
    /// The orbit half-edge the corner follows.
    pub he: HalfEdgeKey,
    /// The next orbit half-edge's departure: the CCW-first bound.
    pub start: Vec3<T>,
    /// What stands behind `start`.
    pub start_reach: Reach<T>,
    /// `he`'s own departure: the CCW-last bound.
    pub end: Vec3<T>,
    /// What stands behind `end`.
    pub end_reach: Reach<T>,
    /// The corner's face.
    pub face: FaceKey,
    /// The face's outward unit normal at the vertex ([`sector_face`]).
    pub normal: OutwardNormal<T>,
}

/// Builds the sector array of `vertex`'s neighborhood (module docs).
///
/// A corner between a slit's or ring's two coincident edges at its tip
/// ([`SlitTips`], read off the operand at rest) is the carrying face's
/// full turn round the tip ([`corner_shape`]), pushed as four quarter
/// sectors, each convex.
pub(super) fn build_sectors<T: Decide>(
    body: &Body<T>,
    operand: Operand,
    vertex: VertexKey,
    tips: &SlitTips,
    band: Band,
) -> Result<Vec<BoolSector<T>>, BooleanError> {
    let corners = orbit_corners(body, operand, vertex, band);
    // A lone orbit half-edge's corner runs from its departure round to
    // it.
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
        // argument needs < 180°), and a slit tip's full turn, are
        // [`corner_shape`]: ONE implementation, called from here and from
        // the splitting lane's neighborhood walk, under the one pooled
        // set of K names (pooled in #652). This is a call, not a copy.
        //
        // The sense-invariance argument for the `normal` passed here is
        // NOT restated: it is the contract of `sector_shape`'s `normal`
        // parameter, which is the one place a caller has to read it.
        // The value arrives typed, so it cannot be the wrong one.
        let bounds = tips.bounds(vertex, face, alone);
        let shape = match corner_shape(dir_end, dir_start, normal, bounds, band) {
            Ok(CornerShape::Sector(shape)) => shape,
            Ok(CornerShape::FullTurn(turn)) => {
                push_full_turn(
                    &mut sectors,
                    he,
                    &turn,
                    (reach_start, reach_end),
                    face,
                    normal,
                );
                continue;
            }
            Err(fault) => {
                return Err(match fault {
                    SectorFault::NonFiniteChord => {
                        BooleanError::NonFiniteSectorChord { vertex, face }
                    }
                    SectorFault::UnderflowedChord => {
                        BooleanError::UnderflowedSectorChord { vertex, face }
                    }
                    SectorFault::Rung { rung, diag } => BooleanError::Escalated {
                        decision: BooleanDecision::Corner(rung),
                        diag,
                    },
                });
            }
        };
        let SectorShape {
            arm,
            unit_own: u_end,
            unit_next: u_start,
            bisector: bisec,
        } = shape;
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

/// Pushes the full turn of `face` round a slit's tip ([`FullTurn`]) as
/// four convex quarter sectors in the chained order [`build_sectors`]
/// pushes a subdivided corner in: the end-sharing quarter first. The
/// quarter bounds are levered at the turn's arm; `start_reach` and
/// `end_reach` stand behind the next half-edge's departure and the
/// corner's own.
fn push_full_turn<T: Decide>(
    sectors: &mut Vec<BoolSector<T>>,
    he: HalfEdgeKey,
    turn: &FullTurn<T>,
    (start_reach, end_reach): (Reach<T>, Reach<T>),
    face: FaceKey,
    normal: OutwardNormal<T>,
) {
    let [q1, q2, q3] = turn.quarters(TurnOrder::FromNext);
    let bounds = [turn.unit_next, q1, q2, q3, turn.unit_own];
    let inner = Reach::Bisector(turn.arm);
    let reaches = [start_reach, inner, inner, inner, end_reach];
    for k in (0..4).rev() {
        sectors.push(BoolSector {
            he,
            start: bounds[k],
            end: bounds[k + 1],
            start_reach: reaches[k],
            end_reach: reaches[k + 1],
            face,
            normal,
            arm: turn.arm,
        });
    }
}

/// The corners of `vertex`'s neighborhood, in orbit order
/// ([`OrbitCorner`]).
pub(super) fn orbit_corners<T: Decide>(
    body: &Body<T>,
    operand: Operand,
    vertex: VertexKey,
    band: Band,
) -> impl ExactSizeIterator<Item = Result<OrbitCorner<T>, BooleanError>> + '_ {
    let orbit = body.vertex_orbit_linked(vertex);
    if orbit.is_empty() {
        unreachable!(
            "{operand:?}'s vertex {vertex:?}, met through a contact, is a lone vertex: a gated \
             operand holds none"
        );
    }
    // The outgoing direction of an orbit half-edge, scaled to the
    // edge's extent from below: the chord for a `Line` carrier, the
    // carrier's outgoing tangent at the base vertex scaled by
    // `edge_extent_inf` for every curved one, a spline's included (its
    // chord is not where it leaves).
    let departure = move |he: HalfEdgeKey| -> (Vec3<T>, Reach<T>) {
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
            geom::Curve3::Circle { .. }
            | geom::Curve3::Ellipse { .. }
            | geom::Curve3::Spiric { .. }
            | geom::Curve3::Nurbs(_) => {
                let ConicDeparture { dir, extent, bend } =
                    ConicDeparture::of(curve, he == edge.he_plus, p_end.distance(p_base));
                (dir, Reach::Conic { extent, bend })
            }
        }
    };
    let n = orbit.len();
    (0..n).map(move |i| {
        let he = orbit[i];
        let (end, end_reach) = departure(he); // CCW-last
        let (start, start_reach) = departure(orbit[(i + 1) % n]); // CCW-first
        let (face, normal) = sector_face(body, operand, vertex, he, band)?;
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
/// adaptation of it: the boolean error type, whose every arm carries
/// the [`Operand`] the shared walk has no notion of, and the cone's
/// apex. All six wired arms — `Plane`, `Cylinder`, `Sphere`, `Torus`,
/// `Cone`, `Spline` — are live on this side; kinds without one refuse
/// typed (C12.1, per arm), and a spline face refuses
/// [`BooleanError::NormalAtSplineCrease`] at a vertex on a crease or a
/// pole of its chart, where it has no one normal.
///
/// **A sector based at a cone's apex refuses**
/// ([`BooleanError::NormalAtConeApex`]): the shared walk reads the face
/// there as its circle of rulings ([`VertexFace::Apex`]), each with a
/// normal of its own, while this lane's sector pair algebra reads one
/// normal per sector. What is missing is that algebra over a sector
/// whose interior is a fan of rulings; the split's walk has its own
/// (`splitting::neighborhood`). A vertex in the band of the axis refuses
/// the same way.
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
    band: Band,
) -> Result<(FaceKey, OutwardNormal<T>), BooleanError> {
    let resolved = crate::sector_face::resolve(body, vertex, he, band).map_err(|e| match e {
        SectorFaceError::Unsupported { face, kind } => BooleanError::CurvedBooleanUnsupported {
            operand,
            face,
            kind,
        },
        SectorFaceError::Crease { face, .. } => BooleanError::NormalAtSplineCrease {
            operand,
            face,
            band,
        },
        // A spline face whose rows are not owed (C4's exemption) stores
        // no chart image of the edge, which the arm reads the vertex at.
        SectorFaceError::Unplaced { face, kind } => BooleanError::CurvedBooleanUnsupported {
            operand,
            face,
            kind,
        },
        SectorFaceError::ApexUndecided { face, .. } => BooleanError::NormalAtConeApex {
            operand,
            face,
            band,
        },
    })?;
    // Exhaustive on purpose, exactly as the splitting wrapper is: a
    // new carrier arm added to the shared walk must be a compile
    // error in BOTH lanes, not silently accepted by the one whose
    // downstream algebra happens not to read the carrier.
    match resolved.carrier {
        SectorCarrier::Plane
        | SectorCarrier::Cylinder
        | SectorCarrier::Sphere
        | SectorCarrier::Torus
        | SectorCarrier::Cone
        | SectorCarrier::Spline(_) => {}
    }
    match resolved.at_vertex {
        VertexFace::Tangent(normal) => Ok((resolved.face, normal)),
        VertexFace::NearAxis(_) | VertexFace::Apex(_) => Err(BooleanError::NormalAtConeApex {
            operand,
            face: resolved.face,
            band,
        }),
    }
}

/// The refusal for a bisector reading On between two bounds definitely
/// on one side (`vtxfac`'s on-edge resolution,
/// `recl::resolve_bisector_graze`): reachable only when K ≤ 2, where the
/// Zero is the band's and does not decide. The side is read off a sign,
/// so `on`, the Zero the bisector's code was decided on, is the nonzero
/// gate's rejection, on the log under the predicate that decided it.
pub(super) fn bisector_zero_refusal(on: OnReading, band: Band) -> BooleanError {
    BooleanError::Escalated {
        decision: BooleanDecision::BisectorSide,
        diag: geom_core::k_stats::gate_rejection(
            on.predicate,
            on.decided,
            band,
            geom_core::SizedPass::NonZero,
        ),
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
/// - [`Reach::Conic`] (a curved edge): the departure tangent levered at
///   the edge's own extent. A definite sign is exact (the sign of
///   `t̂·n̂`). A Zero is a first-order tangency, and reads `On` here:
///   the sector passes that call this key their coincidence and
///   tangent machinery on first-order ties. The vertex-on-face pass
///   reads it one order down ([`pierce_side`], [`conic_side`]).
/// - [`Reach::Extent`] (a probe): the direction levered at its arm.
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
    read_side(dir, reach, face_normal, lever, None, band).map(|(side, _)| side.code)
}

/// **A bound's side of a pierced face, as the vertex-on-face pass reads
/// it** ([`super::vtxfac`]): [`side_code`]'s reading, except that a
/// conic whose tangent ties with a PLANE face is read one order down
/// ([`conic_side`]). `tied` says the first order tied, which is what the
/// pass's whole-sector lump keys on: a sector tangent to the face lumps
/// whole, and its second order is the lump's, read off the two carriers.
///
/// Against a curved face a tie stays `On`: a bound in the face's tangent
/// plane rides the face to first order, and which way it leaves the
/// face is the two carriers' relative curvature, which no reading of the
/// bound alone gives.
pub(super) fn pierce_side<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    face_normal: OutwardNormal<T>,
    shape: FaceShape,
    lever: T,
    band: Band,
) -> Result<PierceSide, BooleanError> {
    pierce_side_reported(dir, reach, face_normal, shape, lever, band).map(|(side, _)| side)
}

/// [`pierce_side`], keeping the first-order Zero a bound read `On` on:
/// what a caller that cannot use the `On` refuses through the gate
/// ([`bisector_zero_refusal`]). `None` beside a side, and beside an `On`
/// the second order read.
pub(super) fn pierce_side_reported<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    face_normal: OutwardNormal<T>,
    shape: FaceShape,
    lever: T,
    band: Band,
) -> Result<(PierceSide, Option<OnReading>), BooleanError> {
    read_side(dir, reach, face_normal, lever, Some(shape), band)
}

/// The Zero a bound's first-order side reading decided, and the
/// predicate that decided it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct OnReading {
    /// The predicate that decided the Zero.
    pub(super) predicate: &'static str,
    /// The Zero, with the margin it was decided on.
    pub(super) decided: Decided,
}

/// What [`pierce_side`] read of a bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct PierceSide {
    /// The bound's side code.
    pub code: SideCode,
    /// Whether its first-order reading tied (`Zero`).
    pub tied: bool,
}

/// [`side_code`] and [`pierce_side`]'s one body. `shape` is the face's
/// where the reading may descend (the vertex-on-face pass's), `None`
/// for the first-order reading; a conic's tie with a plane face is read
/// one order down ([`conic_side`]).
fn read_side<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    face_normal: OutwardNormal<T>,
    lever: T,
    shape: Option<FaceShape>,
    band: Band,
) -> Result<(PierceSide, Option<OnReading>), BooleanError> {
    let n = face_normal.vec();
    let length = reach.length();
    let tie = |code| PierceSide { code, tied: true };
    let on = |predicate, decided| (tie(SideCode::On), Some(OnReading { predicate, decided }));
    let (verdict, displacement) = match reach {
        Reach::Chord { base, far } => {
            let offset = crate::sector_shape::plane_offset(base, n, far);
            let verdict =
                match crate::validate::decide_reported("bool_chord_side", Margin::of(offset), band)
                {
                    // Same mapping as `enters_material`: into the material
                    // is against the outward normal.
                    Ok(Decided {
                        sign: Sign::Negative,
                        ..
                    }) => SideCode::In,
                    Ok(Decided {
                        sign: Sign::Positive,
                        ..
                    }) => SideCode::Out,
                    Ok(
                        zero @ Decided {
                            sign: Sign::Zero, ..
                        },
                    ) => return Ok(on("bool_chord_side", zero)),
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
        Reach::Extent(_) | Reach::Conic { .. } | Reach::Bisector(_) => {
            let verdict = match enters_material_reported(dir, face_normal, length, band) {
                Ok((EntersMaterial::Enters, _)) => SideCode::In,
                Ok((EntersMaterial::Exits, _)) => SideCode::Out,
                Ok((EntersMaterial::Tangent, margin)) => {
                    return match reach {
                        Reach::Conic { extent, bend } if shape == Some(FaceShape::Plane) => {
                            conic_side(dir, extent, bend, face_normal, band)
                                .map(|code| (tie(code), None))
                        }
                        _ => Ok(on(
                            "enters_material",
                            Decided {
                                sign: Sign::Zero,
                                margin,
                            },
                        )),
                    };
                }
                Err(escalation) => {
                    return Err(BooleanError::of_lever(
                        LeverArm::SectorSide,
                        DeclarationRead::Moot,
                        at_departure(escalation, length, rise(dir, n, length), band),
                    ));
                }
            };
            (verdict, rise(dir, n, length).abs())
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
    match crate::validate::decide_reported(
        "bool_pierce_sector_side_curved",
        Margin::of(sagitta_charge(slope, lever, length)),
        band,
    ) {
        Ok(decided) => match Refused::of(decided, band) {
            None => Ok((
                PierceSide {
                    code: verdict,
                    tied: false,
                },
                None,
            )),
            Some(refused) => Err(BooleanError::CurvedSectorSideUnsupported { verdict: refused }),
        },
        Err(diag) => Err(BooleanError::Escalated {
            decision: BooleanDecision::PierceCurvature,
            diag,
        }),
    }
}

/// **The curvature charge**: the least separation from a face that a
/// ray leaving it at `slope` (a sine) keeps past a bend of radius
/// `lever`, `slope·l − l²/lever`, read at `l = min(slope·lever/2,
/// reach)`, where it peaks within the reach. [`read_side`] carries the
/// argument and the witness; `vtxfac::faces_holding_pierce` charges a
/// held pierce's run with it.
pub(super) fn sagitta_charge<T: Decide>(slope: T, lever: T, reach: T) -> T {
    let l = (slope * T::from_f64(0.5) * lever).min(reach);
    slope * l - l.powi(2) / lever
}

/// **A conic bound's side of a plane face one order down**, where its
/// departure tangent read `Zero` against the plane ([`pierce_side`]):
/// [`geom_brep::conic_bend_side`] against the face's outward normal:
/// bending into the material is `In`, out of it `Out`, and a whole edge
/// within the band of the plane is `On` as a line edge's is. Anything
/// else refuses, typed ([`BooleanDecision::ConicBend`]).
fn conic_side<T: Decide>(
    dir: Vec3<T>,
    extent: T,
    bend: ConicBend<T>,
    face_normal: OutwardNormal<T>,
    band: Band,
) -> Result<SideCode, BooleanError> {
    match geom_brep::conic_bend_side(
        dir,
        extent,
        bend,
        geom_brep::ReferenceNormal::of_face_outward(face_normal),
        band,
    ) {
        Ok(EntersMaterial::Enters) => Ok(SideCode::In),
        Ok(EntersMaterial::Exits) => Ok(SideCode::Out),
        Ok(EntersMaterial::Tangent) => Ok(SideCode::On),
        Err(diag) => Err(BooleanError::Escalated {
            decision: BooleanDecision::ConicBend,
            diag,
        }),
    }
}

/// **The lever a pierced face's side verdicts charge** ([`side_code`]'s
/// `lever`): the smallest radius of curvature the face has where the
/// charge reads it, within a bound's reach of the pierce point.
///
/// [`geom_brep::min_radius_of_curvature`] bounds every bend of a plane,
/// sphere, cylinder or torus wherever a bound goes, and is the lever
/// for every reach. A cone's is `ρ` at the pierce point only: its
/// normal curvature `cos α·(d·φ̂)²/ρ` grows toward the axis, so a bound
/// reaching `l` toward it can meet a bend as tight as `ρ − l`.
///
/// The charge reads the bound no further than `l* = slope·lever/2`
/// (capped at the reach), and the slope is a sine, so a lever of `2ρ/3`
/// is never read past `ρ/3`, where the bend is no tighter than `2ρ/3`.
/// The cone's lever is therefore `max(ρ − reach, 2ρ/3)`: the bend
/// nearest the axis within the bound's reach, or within the stretch the
/// charge reads where that is shorter. It is positive wherever the
/// pierce normal is, since that door certified `ρ` definitely positive.
///
/// It bounds the bend of the pierced point's own nappe only. The charge
/// does not see the other sheet of the double cone, which on a wide
/// cone (`α` above about 78.7°) comes within `ρ/3` of the pierce.
/// What reads a crossing onto that sheet is the crossing layer's
/// far-nappe verdict, not this lever.
#[derive(Clone, Copy, Debug)]
pub(super) enum PierceLever<T: geom_core::Real> {
    /// A bound on every bend the face makes.
    Everywhere(T),
    /// A cone's `ρ` at the pierce point.
    ConeAt(T),
    /// A bound on every bend within this radius of the pierce point (a
    /// spline chart's certified arm, [`geom_brep::NurbsLane::chart_arm`]),
    /// which no charge reads past: it reads at most half its lever.
    Within(T),
}

impl<T: Decide> PierceLever<T> {
    /// The pierced `surface`'s lever at `p`.
    pub(super) fn of(surface: &geom::Surface<T>, p: Point3<T>) -> Self {
        let at_p = geom_brep::min_radius_of_curvature(surface, p);
        match surface {
            geom::Surface::Cone { .. } => Self::ConeAt(at_p),
            _ => Self::Everywhere(at_p),
        }
    }

    /// The lever for a bound reaching `reach` from the pierce point.
    pub(super) fn within(self, reach: T) -> T {
        match self {
            Self::Everywhere(lever) | Self::Within(lever) => lever,
            Self::ConeAt(rho) => (rho - reach).max(rho * T::from_f64(2.0 / 3.0)),
        }
    }
}

/// **A side reading's arm gate, quoted at the departure it reads**: the
/// arm is in band or decided zero, and the reading it meters is the
/// bound's departure from the face over that arm, `d̂·n̂·arm`, no longer
/// than the arm up to the in-band excess of the normal's length over one
/// (which [`geom_brep::LeverEscalation::quoting_reading`] guards). A
/// tolerance that decides the arm but leaves the
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
    if !escalation.re_quotes() || !geom_core::is_finite_length(arm / departure) {
        return escalation;
    }
    match geom_core::k_stats::decide_positive(
        "enters_material_rise",
        Margin::of(departure.abs()),
        band,
    ) {
        Err(diag) => escalation.quoting_reading(diag),
        // Only where the face normal's in-band excess over unit length
        // carries the departure past the band: the arm keeps its own
        // margin, whose tolerance decides that departure too.
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
/// The reading is [`tangent_relative_side`] along each of the
/// sector's two `bounds`, levered at that bound's own reach — the
/// length the first-order side code levers the same bound at. The
/// relative curvature of every pair the DEV-1 closed-form locus
/// ([`geom_brep::tangent_locus`], the same rows the door's witness
/// derivation runs) admits is transverse only, one-signed off the
/// locus, so every bound off the locus reads the same side, and the
/// one that reaches far enough for its departure to show decides it:
/// a bound along a short edge, too short for its height off the
/// tangent plane to show, is outvoted by any bound that does show it,
/// and a bound riding the locus reads an exact `On`. The locus gates
/// the descent: it exists exactly where the witness lane reaches, and
/// nowhere else.
///
/// The lump opens for every tangency the glue door decides, whichever
/// side the two materials lie on (C7): an internal tangency reads `In`
/// where an external one reads `Out`.
///
/// Verdicts: a bound definitely curving into the other body's
/// material ⇒ `In`; definitely away ⇒ `Out`; otherwise an in-band
/// bound escalates (an osculating pair is a sliver at this ε — F6),
/// and both bounds at an EXACT second-order zero are an osculating
/// sector, locally conformal to every order the kernel measures over
/// its reach, so it lumps as a one-carrier pair does under the pair's
/// sense bit (`sector_outward` against `other_outward`, levered at
/// `arm`: opposed the Eq. 15.3 ⁻ lump, aligned the ⁺).
#[allow(clippy::too_many_arguments)]
pub(super) fn tangent_lump<T: Decide>(
    sector_surface: &geom::Surface<T>,
    other_surface: &geom::Surface<T>,
    reach: geom_brep::ExtentBall<T>,
    other_outward: OutwardNormal<T>,
    sector_outward: OutwardNormal<T>,
    p: geom_core::Point3<T>,
    op: super::BooleanOp,
    on_side: Operand,
    sector_face: FaceKey,
    bounds: [(Vec3<T>, T); 2],
    arm: T,
    read: DeclarationRead,
    band: Band,
) -> Result<SideCode, BooleanError> {
    use geom_brep::{TangentLocus, TangentLocusError, tangent_locus};
    match tangent_locus(sector_surface, other_surface, reach, band) {
        Ok(TangentLocus::Line { .. }) => {}
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
    let mut shown: Option<SideCode> = None;
    let mut in_band = None;
    for (dir, reached) in bounds {
        match tangent_relative_side(
            sector_surface,
            other_surface,
            other_outward,
            p,
            dir,
            reached,
            read,
            band,
        ) {
            Ok(SideCode::On) => {}
            Ok(code) => match shown {
                Some(seen) if seen != code => {
                    return Err(BooleanError::ClassificationInvariant {
                        what: "a tangent sector's two bounds curve to opposite sides",
                    });
                }
                _ => shown = Some(code),
            },
            Err(escalation) => {
                in_band.get_or_insert(escalation);
            }
        }
    }
    match (shown, in_band) {
        (Some(code), _) => Ok(code),
        (None, Some(escalation)) => Err(escalation),
        // The exact-zero osculating residue (doc above): locally
        // conformal, so the pair's sense bit picks the Eq. 15.3 posture.
        (None, None) => {
            let relation = match crate::validate::decide(
                "tangent_lump_senses",
                Margin::levered(sector_outward.vec().dot(n_ref), arm),
                band,
            ) {
                Ok(Sign::Positive) => super::plane_eq::PlaneRelation::SameOriented,
                Ok(Sign::Negative) => super::plane_eq::PlaneRelation::SameOpposite,
                Ok(Sign::Zero) => {
                    return Err(BooleanError::ClassificationInvariant {
                        what: "a tangent sector pair's outward normals are perpendicular",
                    });
                }
                Err(diag) => {
                    return Err(BooleanError::coincidence(Coincide::TangentSide, read, diag));
                }
            };
            Ok(super::tables::eq15_3_lump(op, on_side, relation))
        }
    }
}

/// The carrier of a face glued `Tangent`, which the tangency's
/// second-order reads compare against its partner's.
pub(super) fn tangent_carrier<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<geom::Surface<T>, BooleanError> {
    body.get_face(face)
        .and_then(|f| body.get_surface(f.surface))
        .cloned()
        .ok_or(BooleanError::ClassificationInvariant {
            what: "a declared-Tangent face lost its surface",
        })
}

/// **The per-direction second-order side** of a declared-`Tangent`
/// sector pair: which side of the OTHER face's material does the
/// sector's carrier lie on along direction `d` from the tie point —
/// each carrier's graph acceleration over the shared tangent plane
/// toward `n̂_ref` ([`geom_brep::implicit_graph_acceleration`]; a graph
/// there by the declared tangency's first-order tie), differenced
/// across the two carriers and classified through the existing second-order
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
    let graph_accel = |s| geom_brep::implicit_graph_acceleration(s, p, d_hat, n_ref);
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
    /// Whether every code reads on: the pair lies in one plane.
    pub(super) fn coplanar(&self) -> bool {
        self.sa == (SideCode::On, SideCode::On) && self.sb == (SideCode::On, SideCode::On)
    }

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
/// `dir` is a line with no reach of its own (two faces' intersection
/// direction), so its side of each bound is read at that bound's reach
/// ([`within_reaching`]). It need not be unit: two faces' line is read
/// un-normalized ([`rays_within`]).
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
    within_reaching(s, (dir, None), strict, read, band)
}

/// [`within`] for a direction that reaches `own` metres from the vertex
/// where it has a length of its own (a bound of the other sector, an
/// edge), `None` where it has none.
///
/// **A direction's side of a bound is read at the shorter of the two's
/// reaches**: for a unit direction the reading is `sin θ` times the
/// bound's reach, how far its far end lies off the direction's line, at
/// the length the side codes read the bound at ([`side_code`]); a
/// direction with a reach of its own is known no farther than that
/// reach. Two faces' line, un-normalized, reads `sin θ · sin φ` times
/// the reach ([`rays_within`]). The sector's `arm`, the shorter chord of
/// its corner, is neither's: a short edge elsewhere at the corner would
/// leave in band a ray that the bound's own reach, and so its code,
/// reads definitely off.
pub(super) fn within_reaching<T: Decide>(
    s: &BoolSector<T>,
    (dir, own): (Vec3<T>, Option<T>),
    strict: bool,
    read: DeclarationRead,
    band: Band,
) -> Result<bool, BooleanError> {
    let (t1, t2) = within_signs(s, (dir, own), read, band)?;
    Ok(if strict {
        t1 == Sign::Positive && t2 == Sign::Positive
    } else {
        t1 != Sign::Negative && t2 != Sign::Negative
    })
}

/// **The length one direction's side of another is read at**: the
/// shorter of the two's own reaches, `bound`'s alone where the
/// direction read against it has none (`own` is `None`: a face–face
/// line, a bisector). Every reading of two directions against each
/// other in the sector passes and the fate cuts takes its lever here
/// ([`within_reaching`]); each reach may be known only from both sides,
/// and the lever then is too.
pub(super) fn reading_lever<T: geom_core::Real>(
    bound: LeverBounds<T>,
    own: Option<LeverBounds<T>>,
) -> LeverBounds<T> {
    own.map_or(bound, |o| bound.min(o))
}

/// [`reading_lever`] of two reaches known exactly.
pub(super) fn exact_lever<T: geom_core::Real>(bound: T, own: Option<T>) -> T {
    reading_lever(LeverBounds::Exact(bound), own.map(LeverBounds::Exact)).below()
}

/// [`within_reaching`]'s two readings of `dir` against `s`'s bounds:
/// which side of `start`, then of `end`, it lies on, `Positive` toward
/// the other.
fn within_signs<T: Decide>(
    s: &BoolSector<T>,
    (dir, own): (Vec3<T>, Option<T>),
    read: DeclarationRead,
    band: Band,
) -> Result<(Sign, Sign), BooleanError> {
    let lever = |r: Reach<T>| exact_lever(r.length(), own);
    let c1 = Margin::levered(s.start.cross(dir).dot(s.normal.vec()), lever(s.start_reach));
    let c2 = Margin::levered(dir.cross(s.end).dot(s.normal.vec()), lever(s.end_reach));
    let escalate = |diag| BooleanError::coincidence(Coincide::Sectors, read, diag);
    Ok((
        decide("bool_sector_within", c1, band).map_err(escalate)?,
        decide("bool_sector_within", c2, band).map_err(escalate)?,
    ))
}

/// **Which rays of `line` lie within every one of `sectors`**:
/// `(+line, −line)`, each `true` where [`within`] holds it in all.
///
/// Two faces' line is passed un-normalized, `m = n_a × n_b`. Read
/// against a bound `e` of `a`'s sector at `e`'s reach `L`, it measures
/// `L·(m × e)·n_a = L·((n_a·e) cos φ − n_b·e)`: the bound's own
/// departure from `a`'s plane, foreshortened by `cos φ`, less its far
/// end's offset from `b`'s plane. The two terms can add or cancel; the
/// first is within the zero wherever `a` holds its bound. For a straight
/// bound the second is what its side code reads at the chord's far point
/// ([`side_code`]); a curved bound's code reads the edge where this
/// reads its tangent. The unit line would divide the whole by
/// `|m| = sin φ`, so a far end within the zero of both planes would read
/// off their line wherever the two faces are near parallel. A line with
/// no such conditioning (a tangent locus) is passed unit.
pub(super) fn rays_within<T: Decide>(
    line: Vec3<T>,
    sectors: &[&BoolSector<T>],
    read: DeclarationRead,
    band: Band,
) -> Result<(bool, bool), BooleanError> {
    let holds = |d| -> Result<bool, BooleanError> {
        for s in sectors {
            if !within(s, d, false, read, band)? {
                return Ok(false);
            }
        }
        Ok(true)
    };
    Ok((holds(line)?, holds(-line)?))
}

/// A half-plane a sector's face leaves an edge in ([`half_planes`]): its
/// unit direction square to the edge, and the sector.
pub(super) type HalfPlane<'s, T> = (Vec3<T>, &'s BoolSector<T>);

/// **The half-planes `other`'s faces leave `dir` in**, where `dir` lies
/// on the boundary of `other`'s corner: for each sector whose face's
/// plane holds `dir` and whose closure holds it, the unit direction
/// square to `dir` into the face, on each side of `dir` the sector lies
/// — one side where `dir` runs along a bound, both where it runs inside
/// — each with the sector it leaves `dir` in. Around `dir`, the corner's
/// material changes side at exactly these.
pub(super) fn half_planes<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    other: &[BoolSector<T>],
    band: Band,
) -> Result<Vec<HalfPlane<'_, T>>, BooleanError> {
    let mut out = Vec::new();
    for s in other {
        if plane_side_code(dir, reach, s.normal, band)? != SideCode::On {
            continue;
        }
        // CCW about the normal from `start` to `end` (module docs), so
        // the face lies ahead of `start` and behind `end`.
        // A direction along a bound is read along it, sides and sense,
        // at the one lever (`reading_lever`).
        let ahead = s.normal.vec().cross(dir).normalize();
        let along = |r: Reach<T>| exact_lever(r.length(), Some(reach.length()));
        match within_signs(s, (dir, Some(reach.length())), DeclarationRead::Moot, band)? {
            (Sign::Positive, Sign::Positive) => out.extend([(ahead, s), (-ahead, s)]),
            (Sign::Zero, Sign::Positive)
                if direction_sense(s.start, dir, along(s.start_reach), band)? =>
            {
                out.push((ahead, s));
            }
            (Sign::Positive, Sign::Zero)
                if direction_sense(s.end, dir, along(s.end_reach), band)? =>
            {
                out.push((-ahead, s));
            }
            _ => {}
        }
    }
    Ok(out)
}

/// The sector's bounds that are edges of its face, with their unit
/// directions: `end` is `s.he`'s edge, `start` the next orbit
/// half-edge's. A subdivision bisector is no edge.
pub(super) fn bound_edges<T: Decide>(
    body: &Body<T>,
    s: &BoolSector<T>,
) -> Vec<(crate::entity::EdgeKey, Vec3<T>)> {
    bound_reaches(body, s)
        .into_iter()
        .map(|(e, dir, _)| (e, dir))
        .collect()
}

/// [`bound_edges`] with each bound's [`Reach`].
pub(super) fn bound_reaches<T: Decide>(
    body: &Body<T>,
    s: &BoolSector<T>,
) -> Vec<(crate::entity::EdgeKey, Vec3<T>, Reach<T>)> {
    let edge = |he| proven(&body.half_edges, he, EntityId::HalfEdge).edge;
    let mut out = Vec::new();
    if s.end_edge() {
        out.push((edge(s.he), s.end, s.end_reach));
    }
    if s.start_edge() {
        out.push((edge(body.proven_orbit_step(s.he)), s.start, s.start_reach));
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
/// null edges into the same face. There are two exceptions, each an
/// edge-edge germ placed on the flanker inside the other solid's wedge:
/// a germ along an edge that two crossing pairs at one vertex both cross
/// along, which folds Out in both, at both of the edge's ends, each of
/// which is a v-v site of both pairs once the reduction has split the
/// edges there ([`super::recl::Reversed`]); and a germ of a wedge that
/// crosses both wedges of a slit's or ring's line, so the line's two
/// germs lie apart on their own sides (`recl::onto_inside`).
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

/// Where an edge leaving a site lies against the partner past it, in
/// rank order: what the reduction recorded at its far end, or for a
/// closed edge where its own carrier runs ([`closed_reach`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Touch {
    /// No contact: the edge leaves the partner.
    Apart,
    /// On the partner's boundary: a far end in a vertex pair, or a
    /// closed edge's midpoint on a face's boundary.
    Boundary,
    /// Inside a face of the partner.
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
/// bisector or with no bound On. A closed edge's far end is the site
/// itself, whose record is the site's own contact, so whether it runs
/// along the partner's face `partner` (the one the germ runs in on the
/// other operand) is read from its own carrier instead ([`closed_reach`]).
/// There it cannot lie in the face's plane outside its trim: it leaves
/// the site into the face along the germ, and the sweep would split it
/// where it crossed out. Called by every site kind, before that site's
/// surgery moves the orbit the bounds are read from.
pub(super) fn germ_locus<T: Decide>(
    side: GermSide<'_, T>,
    partner: (&Body<T>, FaceKey),
    contacts: &super::ReductionContacts,
    band: Band,
) -> Result<super::Locus, BooleanError> {
    let Some(e) = along(side)? else {
        return Ok(super::Locus::InFace(side.sector.face));
    };
    let on = match closed_reach(side, e, (partner.0, &[partner.1]), band)? {
        None => touch(side, e.far, contacts)? != Touch::Apart,
        Some(ClosedReach::Off) => false,
        Some(ClosedReach::On(..)) => true,
        Some(ClosedReach::Beside) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "a closed edge along a germ lies in the plane of the face the germ runs \
                       in, outside its trim",
            });
        }
    };
    Ok(if on {
        super::Locus::OnEdge(e.edge)
    } else {
        super::Locus::InFace(side.sector.face)
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
/// `OnEdge` both. A closed edge's far end is the site itself, so where
/// it runs is read from its carrier in its far end's place
/// ([`closed_reach`]), against every face around the partner's site,
/// where an open edge's far end is read from the records. Two closed
/// edges both on the partner's boundary are one segment
/// ([`one_segment`]).
pub(super) fn germ_loci<T: Decide>(
    a: GermSide<'_, T>,
    b: GermSide<'_, T>,
    contacts: &super::ReductionContacts,
    declared: &super::GluedPairs<T>,
    band: Band,
) -> Result<(super::Locus, super::Locus), BooleanError> {
    use super::Locus::{InFace, OnEdge};
    let (Some(ea), Some(eb)) = (along(a)?, along(b)?) else {
        return Ok((
            germ_locus(a, (b.body, b.sector.face), contacts, band)?,
            germ_locus(b, (a.body, a.sector.face), contacts, band)?,
        ));
    };
    let reach = |side: GermSide<'_, T>, e: Along, partner: GermSide<'_, T>| {
        let faces = faces_at(partner.body, partner.site).map_err(stale_site)?;
        Ok::<_, BooleanError>(
            closed_reach(side, e, (partner.body, &faces), band)?.map(ClosedReach::touch),
        )
    };
    let (ca, cb) = (reach(a, ea, b)?, reach(b, eb, a)?);
    let (ta, tb) = match (ca, cb) {
        (Some((Touch::Boundary, _)), Some((Touch::Boundary, _))) => {
            return one_segment(a, ea, b, eb, declared);
        }
        (Some((ta, _)), Some((tb, _))) => (ta, tb),
        (ca, cb) => {
            let (far_a, far_b) = (site_of(a.body, ea.far)?, site_of(b.body, eb.far)?);
            if contacts
                .vv
                .iter()
                .any(|c| far_a.contains(&c.a) && far_b.contains(&c.b))
            {
                return one_segment(a, ea, b, eb, declared);
            }
            (
                ca.map_or_else(|| touch(a, ea.far, contacts), |c| Ok(c.0))?,
                cb.map_or_else(|| touch(b, eb.far, contacts), |c| Ok(c.0))?,
            )
        }
    };
    // A closed edge's segment lies in the face its midpoint was read in.
    let tangent = |side: GermSide<'_, T>, partner: GermSide<'_, T>, e: Along| {
        let held = match (side.operand, ca, cb) {
            (Operand::A, _, Some((_, f))) | (Operand::B, Some((_, f)), _) => f,
            _ => tangent_face(side, partner, e.edge, contacts)?,
        };
        Ok::<_, BooleanError>(InFace(held.unwrap_or(side.sector.face)))
    };
    Ok(match ta.cmp(&tb) {
        core::cmp::Ordering::Greater => (OnEdge(ea.edge), tangent(b, a, ea)?),
        core::cmp::Ordering::Less => (tangent(a, b, eb)?, OnEdge(eb.edge)),
        core::cmp::Ordering::Equal if ta == Touch::Apart => {
            (InFace(a.sector.face), InFace(b.sector.face))
        }
        core::cmp::Ordering::Equal => {
            match (
                runs_in((a, ea, ca), (b, eb), band)?,
                runs_in((b, eb, cb), (a, ea), band)?,
            ) {
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
/// are one segment** when both are lines or circles, or both are
/// sections of one plane and one cylinder or cone. Each is tangent to
/// the germ at the site, and the sweep splits an edge at every crossing
/// of the partner, so neither passes the other's far end.
///
/// - A line or circle meets a line or circle tangent to it at the site
///   at no other point unless the two are one curve, so the paired far
///   ends make them one.
/// - Where the glue door verified each edge's two faces one carrier
///   with the partner's two, crosswise, one a plane and the other a
///   cylinder or cone ([`same_construction`]), both edges lie on that
///   plane's section of the quadric. That section is one conic, which
///   has one branch through the site along the germ; or a line pair
///   (the plane through a cone's apex, or along a cylinder's rulings),
///   whose two lines cross at most at one point and there in two
///   directions, so the two edges leaving the site along one direction
///   are on one line. Either way they are one curve. Two curved
///   carriers can meet in two branches tangent at a point, so the
///   identity is read only where one of the two is a plane.
///
/// This is structure, not a test: no reading of the two curves could
/// answer otherwise. Any other conic or spline can meet such a partner
/// again, and its pair is refused
/// ([`BooleanError::GermEdgeCarrierUnsupported`]).
///
/// Two closed edges (each one vertex, both its ends) pair their far
/// ends, the site, always: [`germ_loci`] asks [`closed_reach`] first,
/// and comes here where both run along the partner's boundary.
fn one_segment<T: Decide>(
    a: GermSide<'_, T>,
    ea: Along,
    b: GermSide<'_, T>,
    eb: Along,
    declared: &super::GluedPairs<T>,
) -> Result<(super::Locus, super::Locus), BooleanError> {
    if same_construction(a, ea, b, eb, declared)? {
        return Ok((super::Locus::OnEdge(ea.edge), super::Locus::OnEdge(eb.edge)));
    }
    for (side, e) in [(a, ea), (b, eb)] {
        match germ_curve(side.body, e.edge)?.carrier() {
            geom::Curve3::Line { .. } | geom::Curve3::Circle { .. } => {}
            _ => {
                return Err(BooleanError::GermEdgeCarrierUnsupported {
                    operand: side.operand,
                    edge: side
                        .body
                        .split_root(e.edge, |_| false)
                        .map_err(BooleanError::SplitLineage)?,
                    closed: false,
                });
            }
        }
    }
    Ok((super::Locus::OnEdge(ea.edge), super::Locus::OnEdge(eb.edge)))
}

/// Whether the faces either side of `ea` and of `eb` pair crosswise
/// as carriers the glue door verified one
/// ([`super::GluedPairs::verified_one_carrier`]), one of the two a plane
/// and the other a cylinder or cone: the two edges are then sections of
/// one plane and one quadric ([`one_segment`]). A plane meets a sphere
/// in a circle and two faces on one plane are one face, so neither
/// reaches here past the kind rule. An edge bounding one face on both
/// sides (a wrap edge) is no section of two faces.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] where a germ edge's half
/// has no mate or no face, or a face's carrier does not read.
fn same_construction<T: Decide>(
    a: GermSide<'_, T>,
    ea: Along,
    b: GermSide<'_, T>,
    eb: Along,
    declared: &super::GluedPairs<T>,
) -> Result<bool, BooleanError> {
    use geom::SurfaceKind::{Cone, Cylinder, Plane};
    let lost = |what| BooleanError::ClassificationInvariant { what };
    let faces = |side: GermSide<'_, T>, e: EdgeKey| {
        let plus = proven(&side.body.edges, e, EntityId::Edge).he_plus;
        let minus = side
            .body
            .mate(plus)
            .ok_or(lost("a germ edge's half has no mate"))?;
        let face = |h| {
            side.body
                .face_of_half_edge(h)
                .ok_or(lost("a germ edge's half bounds no face"))
        };
        Ok::<_, BooleanError>([face(plus)?, face(minus)?])
    };
    let ([a0, a1], fb) = (faces(a, ea.edge)?, faces(b, eb.edge)?);
    if a0 == a1 || fb[0] == fb[1] {
        return Ok(false);
    }
    let kind = |f| {
        crate::readback::face_carrier_kind(a.body, f)
            .map_err(|_| lost("a germ edge's face carrier does not read"))
    };
    let section = matches!(
        (kind(a0)?, kind(a1)?),
        (Plane, Cylinder | Cone) | (Cylinder | Cone, Plane)
    );
    let one = |fa, fb| declared.verified_one_carrier(a.operand, fa, b.operand, fb);
    Ok(section
        && [fb, [fb[1], fb[0]]]
            .iter()
            .any(|&[b0, b1]| one(a0, b0) && one(a1, b1)))
}

/// Where a closed edge leaving the site runs against the partner
/// ([`closed_reach`]).
#[derive(Clone, Copy)]
enum ClosedReach {
    /// On the carrier of none of the faces read: it only touches the germ
    /// at the site.
    Off,
    /// In a planar face's plane, outside its trim.
    Beside,
    /// Along the partner: inside a planar face ([`Touch::Face`], with that
    /// face), or on one's boundary or on a curved face's carrier
    /// ([`Touch::Boundary`]).
    On(Touch, Option<FaceKey>),
}

/// A closed edge's reading as ranked ([`ClosedReach::touch`]): the
/// touch, and the face holding it where one does.
type ClosedTouch = (Touch, Option<FaceKey>);

impl ClosedReach {
    /// The reading ranked as an open edge's far end is ([`Touch`]).
    fn touch(self) -> ClosedTouch {
        match self {
            Self::Off | Self::Beside => (Touch::Apart, None),
            Self::On(touch, face) => (touch, face),
        }
    }
}

/// **Where a closed edge leaving the site runs against the partner's
/// `faces`**: its far end is the site itself, so what the reduction
/// recorded there says nothing of the edge, and its own carrier is read
/// instead: whether it lies on each face's surface
/// ([`super::slit::carrier_on_surface`], at points spread over the whole
/// turn, enough that a carrier merely meeting the surface cannot pass),
/// and on a plane, where its midpoint sits in the face's trim
/// ([`in_trim`]), where its two-arc form's far vertex would be. Inside
/// one face decides; else on a plane's trim boundary or a curved face's
/// carrier, whose trim is not placed, reads as running along the
/// partner's boundary there, as a far vertex paired with the partner's
/// would; else in a plane outside its trim; else off. `None` where `e`
/// is not closed at the site.
///
/// # Errors
///
/// [`BooleanError::GermEdgeCarrierUnsupported`] where no face reads it
/// inside and some face's surface is one the reading does not take
/// against the edge's carrier (a spline on either side); the reading's
/// own escalations, and a trim the walk cannot read.
fn closed_reach<T: Decide>(
    side: GermSide<'_, T>,
    e: Along,
    (body, faces): (&Body<T>, &[FaceKey]),
    band: Band,
) -> Result<Option<ClosedReach>, BooleanError> {
    if !site_of(side.body, e.far)?.contains(&side.site) {
        return Ok(None);
    }
    let curve = germ_curve(side.body, e.edge)?;
    let p = curve.mid_point();
    let (mut reach, mut unread) = (ClosedReach::Off, false);
    for &face in faces {
        match super::slit::carrier_on_surface(body, curve.carrier(), curve.params(), face, band)? {
            None => unread = true,
            Some(false) => {}
            Some(true) => match in_trim(body, face, p, band)? {
                Some(PlaneRead::Inside) => {
                    return Ok(Some(ClosedReach::On(Touch::Face, Some(face))));
                }
                Some(PlaneRead::Outside) => {
                    if matches!(reach, ClosedReach::Off) {
                        reach = ClosedReach::Beside;
                    }
                }
                _ => reach = ClosedReach::On(Touch::Boundary, None),
            },
        }
    }
    if unread {
        return Err(BooleanError::GermEdgeCarrierUnsupported {
            operand: side.operand,
            edge: side
                .body
                .split_root(e.edge, |_| false)
                .map_err(BooleanError::SplitLineage)?,
            closed: true,
        });
    }
    Ok(Some(reach))
}

/// Where a point lies against a face read as a plane
/// ([`on_planar_face`]).
#[derive(Clone, Copy, PartialEq, Eq)]
enum PlaneRead {
    /// The face is not a plane.
    NotPlane,
    /// Definitely off the face's plane.
    OffPlane,
    /// On the plane, inside the face's trim.
    Inside,
    /// On the plane, outside the trim.
    Outside,
    /// On the plane, on the trim's boundary within the band.
    OnBoundary,
}

/// **Where `p` lies against the planar `face` of `body`**: its height
/// over the face's plane decided first (`bool_germ_tie_plane`), then its
/// place in the face's trim ([`super::solid_contain::point_in_face`]).
/// A trim the walk cannot read is refused.
fn on_planar_face<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    p: Point3<T>,
    band: Band,
) -> Result<PlaneRead, BooleanError> {
    use super::solid_contain::{PointInSolidError, face_plane};
    let (origin, normal) = match face_plane(body, face) {
        Ok(plane) => plane,
        Err(PointInSolidError::KindUnsupported { .. }) => return Ok(PlaneRead::NotPlane),
        Err(e) => return Err(BooleanError::Containment(e)),
    };
    let height = Margin::of((p - origin).dot(normal));
    match decide("bool_germ_tie_plane", height, band).map_err(|diag| {
        BooleanError::coincidence(Coincide::EdgeOnPlane, DeclarationRead::Moot, diag)
    })? {
        Sign::Zero => {}
        Sign::Positive | Sign::Negative => return Ok(PlaneRead::OffPlane),
    }
    trim_read(body, face, normal, p, band)
}

/// **Where `p`, on the plane of `face`, sits in its trim**
/// ([`super::solid_contain::point_in_face`]); `None` where the face is
/// not a plane. A trim the walk cannot read is refused.
fn in_trim<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    p: Point3<T>,
    band: Band,
) -> Result<Option<PlaneRead>, BooleanError> {
    use super::solid_contain::{PointInSolidError, face_plane};
    match face_plane(body, face) {
        Ok((_, normal)) => trim_read(body, face, normal, p, band).map(Some),
        Err(PointInSolidError::KindUnsupported { .. }) => Ok(None),
        Err(e) => Err(BooleanError::Containment(e)),
    }
}

/// [`in_trim`] on the face's plane normal `normal`.
fn trim_read<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    normal: Vec3<T>,
    p: Point3<T>,
    band: Band,
) -> Result<PlaneRead, BooleanError> {
    Ok(
        match super::solid_contain::point_in_face(body, face, normal, p, band)
            .map_err(BooleanError::Containment)?
        {
            Some(true) => PlaneRead::Inside,
            Some(false) => PlaneRead::Outside,
            None => PlaneRead::OnBoundary,
        },
    )
}

/// Whether the edge `e` of `side`, past the site, runs inside the face
/// of `partner` the germ runs along: the face across the partner's own
/// edge `pe` from its sector's face. An open edge's midpoint is decided
/// on that face's plane and inside its trim ([`on_planar_face`]): the
/// sweep splits an edge where it crosses the partner's trim, so the
/// midpoint speaks for the whole edge. A closed edge, `closed` its
/// reading ([`closed_reach`]), answers by the face that reading holds it
/// in. `None`: undecided — a midpoint or a closed edge on the face's
/// boundary, a face that is not a plane, or no single face across `pe`.
/// A trim the walk cannot read is refused.
fn runs_in<T: Decide>(
    (side, e, closed): (GermSide<'_, T>, Along, Option<ClosedTouch>),
    (partner, pe): (GermSide<'_, T>, Along),
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
    if let Some((_, held)) = closed {
        return Ok(held.map(|held| held == face));
    }
    let p = germ_curve(side.body, e.edge)?.mid_point();
    Ok(match on_planar_face(pb, face, p, band)? {
        PlaneRead::Inside => Some(true),
        PlaneRead::Outside | PlaneRead::OffPlane => Some(false),
        PlaneRead::OnBoundary | PlaneRead::NotPlane => None,
    })
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
    contacts: &super::ReductionContacts,
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
fn on_faces(
    contacts: &super::ReductionContacts,
    operand: Operand,
) -> &[crate::Cited<super::VfContact>] {
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
    contacts: &super::ReductionContacts,
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

/// **How far a curve leaving a vertex stands past another it leaves
/// tangent to, one order down**, at arc length `l` along their shared
/// tangent: `(κ − κ_b)·û·l²/2`, with `κ` and `κ_b` the two curvature
/// vectors and `û` the unit in-plane direction square to the tangent
/// toward the side read. Positive where the curve with `κ` stands past
/// the one with `κ_b` toward `û`. Its sign does not depend on `l`, which
/// sets only the size the margin is decided at.
pub(super) fn bend_parting<T: geom_core::Real>(
    (k, kb): (Vec3<T>, Vec3<T>),
    u: Vec3<T>,
    l: T,
) -> Margin<T> {
    Margin::of((k - kb).dot(u) * l.powi(2) / (T::one() + T::one()))
}

/// **Whether an edge leaving the vertex along `dir`, coplanar with `s`,
/// runs into its face**: within the sector and along neither of its
/// bounds that is an edge of the face, or, along one of them, bending
/// into the face off it one order down: along a bound, the edge enters
/// the face where it stands past the bound toward `s`
/// ([`bend_parting`], decided at the shorter reach). A bound or an edge
/// with no bend to read (a fitted edge, a bisector) reads no entry.
pub(super) fn runs_into<T: Decide>(
    s: &BoolSector<T>,
    (dir, reach): (Vec3<T>, Reach<T>),
    band: Band,
) -> Result<bool, BooleanError> {
    let moot = DeclarationRead::Moot;
    if !within_reaching(s, (dir, Some(reach.length())), false, moot, band)? {
        return Ok(false);
    }
    for (bound, along, is_edge) in [
        (s.start, s.start_reach, s.start_edge()),
        (s.end, s.end_reach, s.end_edge()),
    ] {
        let lever = exact_lever(along.length(), Some(reach.length()));
        if !(is_edge && parallel_same(dir, bound, lever, BOUNDS_PARALLEL, band)?) {
            continue;
        }
        let (Some((k, l)), Some((kb, lb))) = (bound_bend(reach), bound_bend(along)) else {
            return Ok(false);
        };
        for r in [reach, along] {
            if let Reach::Conic { bend, .. } = r
                && matches!(bend.bounds, geom_brep::BendBounds::Unbounded)
            {
                return Ok(false);
            }
        }
        let d = bound.normalize();
        let u = {
            let m = s.middle();
            (m - d * m.dot(d)).normalize()
        };
        let entered = bend_parting((k, kb), u, l.min(lb));
        return Ok(
            decide("bool_sector_runs_into_bend", entered, band).map_err(|diag| {
                BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag)
            })? == Sign::Positive,
        );
    }
    Ok(true)
}

/// **Whether two directions are parallel and point the same way**: the
/// sine of their angle levered at `arm`, decided Zero, then their sense
/// ([`direction_sense`]). Both are read off the directions normalized,
/// so a margin is a length whatever the directions' norms. The caller's
/// `reading` names the parallelism margin and the coincidence an in-band
/// reading refuses as.
pub(super) fn parallel_same<T: Decide>(
    u: Vec3<T>,
    v: Vec3<T>,
    arm: T,
    reading: Parallelism,
    band: Band,
) -> Result<bool, BooleanError> {
    let (u, v) = (u.normalize(), v.normalize());
    match decide(
        reading.predicate,
        Margin::levered(u.cross(v).norm(), arm),
        band,
    ) {
        Ok(Sign::Zero) => {}
        Ok(_) => return Ok(false),
        Err(diag) => {
            return Err(BooleanError::coincidence(
                reading.coincide,
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
    Ok(
        crate::validate::decide_nonzero("bool_dir_same", Margin::levered(u.dot(v), arm), band)
            .map_err(|diag| BooleanError::Escalated {
                decision: BooleanDecision::DirectionSense,
                diag,
            })?
            == NonzeroSign::Positive,
    )
}

/// `sectoroverlap` — coplanar sectors overlap test (module docs).
fn sector_overlap<T: Decide>(
    a: &BoolSector<T>,
    b: &BoolSector<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    for (s, dir, reach) in [
        (a, b.start, b.start_reach),
        (a, b.end, b.end_reach),
        (b, a.start, a.start_reach),
        (b, a.end, a.end_reach),
    ] {
        if within_reaching(
            s,
            (dir, Some(reach.length())),
            true,
            DeclarationRead::Moot,
            band,
        )? {
            return Ok(true);
        }
    }
    let along = |(u, ru): (Vec3<T>, Reach<T>), (v, rv): (Vec3<T>, Reach<T>)| {
        let lever = exact_lever(rv.length(), Some(ru.length()));
        parallel_same(u, v, lever, BOUNDS_PARALLEL, band)
    };
    let (a_start, a_end) = ((a.start, a.start_reach), (a.end, a.end_reach));
    let (b_start, b_end) = ((b.start, b.start_reach), (b.end, b.end_reach));
    // Identical region (same or crossed bound pairing).
    let straight = along(a_start, b_start)? && along(a_end, b_end)?;
    let crossed = along(a_start, b_end)? && along(a_end, b_start)?;
    Ok(straight || crossed)
}

/// **A crescent the union cannot build.** Two plane faces, one of each
/// operand, lie on each other with opposed normals at a vertex pair,
/// and their sectors there share one bound direction from opposite
/// sides of it: at first order they only touch, so [`pair_search`]
/// records them as `touching`, while one order down they overlap
/// ([`crescent`]). The union glues the two faces over that crescent, a
/// region of the plane narrowing to the vertex between two curves
/// tangent there. Every germ the sector passes produce runs along a
/// bound or a face–face intersection direction of a pair they record,
/// and a region of zero angle at the vertex has neither, so the section
/// cannot reach the crescent and the union would keep both faces whole.
/// The intersection keeps neither face there and each difference one
/// face whole, so they build, and the reading runs under union alone:
/// its own decisions refuse no op that builds.
pub(super) fn crescent_refusal<T: Decide>(
    bodies: [&Body<T>; 2],
    sectors: [&[BoolSector<T>]; 2],
    touching: &[(usize, usize)],
    contacts: &super::ReductionContacts,
    band: Band,
) -> Result<(), BooleanError> {
    let what = "a sector's face has no carrier";
    for &(i, j) in touching {
        let (sa, sb) = (&sectors[0][i], &sectors[1][j]);
        if FaceShape::of_face(bodies[0], sa.face, what)? != FaceShape::Plane
            || FaceShape::of_face(bodies[1], sb.face, what)? != FaceShape::Plane
        {
            continue;
        }
        let arm = sa.arm.min(sb.arm);
        let corner = |k: usize, sector| Corner {
            body: bodies[k],
            sectors: sectors[k],
            sector,
        };
        if !direction_sense(sa.normal.vec(), sb.normal.vec(), arm, band)?
            && crescent(corner(0, sa), corner(1, sb), contacts, band)?
        {
            return Err(BooleanError::TangentCrescentUnsupported {
                a_face: sa.face,
                b_face: sb.face,
            });
        }
    }
    Ok(())
}

/// A bound's curvature vector at the base vertex and its reach, where
/// the bound is an edge whose bend the sector reads: a curved edge's
/// own, a line edge's zero; `None` for a bisector or a probe.
fn bound_bend<T: Decide>(reach: Reach<T>) -> Option<(Vec3<T>, T)> {
    match reach {
        Reach::Conic { extent, bend } => Some((bend.curvature, extent)),
        Reach::Chord { .. } => Some((Vec3::new(T::zero(), T::zero(), T::zero()), reach.length())),
        Reach::Extent(_) | Reach::Bisector(_) => None,
    }
}

/// **Whether two coplanar sectors that share one bound direction, from
/// opposite sides of it, overlap past it one order down**: the crescent
/// between two curves tangent at the vertex. With `a` on the side of
/// its bound toward `û`, the unit in-plane direction square to the
/// shared direction into `a`, and `b` on the other side of its own, the
/// two overlap from the vertex where `b`'s bound stands past `a`'s
/// toward `û` ([`bend_parting`]). A line edge bends by zero, so two line
/// edges read none; a fitted edge or a bisector reads no bend, and a
/// pair holding one reads no crescent. Equal bends (one circle bounding
/// both) read none.
///
/// The shared direction is read at the sector arm, no longer than the
/// bounds' own reaches [`pair_search`] read the pair touching at. Two
/// bounds that part at their own reach leave the site at an angle the
/// arm is too short to resolve; where the corners' other edges run on
/// together from the site
/// ([`coincident_run`]), the section closes that wedge and the pair
/// reads none.
///
/// The verdict is the second-order term's sign at the vertex, read off
/// the two carriers' point curvatures, which is exact as a claim that
/// the crescent exists: near enough the vertex that term dominates
/// whatever the carriers do further out. The size it is decided at,
/// the term at the shorter bound's reach with its own ½, answers only
/// whether the band resolves that sign there, which is why it is the
/// leading term and not a bound: `read_side`'s sagitta drops the ½
/// because it needs an upper bound on a departure, and a refusal gate
/// that misjudges a crescent's width at its reach refuses a sliver the
/// band would have read as a touch, never ships a body. A spiric
/// section certifies no bend, and refuses as
/// [`geom_brep::conic_bend_side`] does.
fn crescent<T: Decide>(
    ca: Corner<'_, T>,
    cb: Corner<'_, T>,
    contacts: &super::ReductionContacts,
    band: Band,
) -> Result<bool, BooleanError> {
    let (a, b) = (ca.sector, cb.sector);
    let arm = a.arm.min(b.arm);
    let escalate = |diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag);
    for (a_start, da, ra) in [(true, a.start, a.start_reach), (false, a.end, a.end_reach)] {
        for (b_start, db, rb) in [(true, b.start, b.start_reach), (false, b.end, b.end_reach)] {
            let (Some((ka, la)), Some((kb, lb))) = (bound_bend(ra), bound_bend(rb)) else {
                continue;
            };
            if !matches!(ra, Reach::Conic { .. }) && !matches!(rb, Reach::Conic { .. }) {
                continue;
            }
            if !parallel_same(da, db, arm, BOUNDS_PARALLEL, band)? {
                continue;
            }
            for reach in [ra, rb] {
                if let Reach::Conic { bend, .. } = reach
                    && matches!(bend.bounds, geom_brep::BendBounds::Unbounded)
                {
                    let diag = match crate::validate::decide_positive(
                        "bool_sector_crescent",
                        Margin::of(T::zero()),
                        band,
                    ) {
                        Err(diag) => diag,
                        Ok(()) => unreachable!("a zero margin is never decided positive"),
                    };
                    return Err(BooleanError::Escalated {
                        decision: BooleanDecision::ConicBend,
                        diag,
                    });
                }
            }
            let d = da.normalize();
            let square = |v: Vec3<T>| v - d * v.dot(d);
            let u = square(a.middle()).normalize();
            // `pair_search` reads two sectors sharing a bound from one
            // side as overlapping, so a touching pair lies across it;
            // this reads that premise where the claim rests on it.
            let across = Margin::levered(square(b.middle()).normalize().dot(u), arm);
            if decide("bool_sector_crescent_across", across, band).map_err(escalate)?
                != Sign::Negative
            {
                continue;
            }
            let l = la.min(lb);
            if coincident_run((ca, a_start), (cb, b_start), contacts, band)?
                && !parallel_same(da, db, l, BOUNDS_PARALLEL, band)?
            {
                continue;
            }
            let opened = bend_parting((kb, ka), u, l);
            if decide("bool_sector_crescent", opened, band).map_err(escalate)? == Sign::Positive {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// **Whether the section runs on from a vertex pair along the other
/// edges of the two corners whose bounds [`crescent`] reads**: the edge
/// across each corner from the bound it reads (`start`, or the end),
/// each a line or an arc of at most a half-turn ([`far_edge`]), their
/// far ends a vertex pair the reduction recorded, their departure
/// directions parallel levered at the shorter reach `L`.
///
/// Such edges lie on each other within the band. Two such arcs through
/// one pair of points `P`, `Q` (chord `c`, a line the arc of zero turn)
/// meeting at `P` at an angle `θ` turn by `α`, `α + θ` off the chord,
/// each `α ≤ π/2`, and the arc turning `α` off the chord stands
/// `(c/2)·tan(α/2)` from it at mid-chord, so they part by at most
/// `(c/4)·sec²(α/2)·θ ≤ c·θ/2` to first order. An arc's reach is no
/// shorter than its chord, so `L ≥ c`, and the parallel reading's
/// Zero, `θ·L` within the band, bounds that parting by half the band,
/// on top of the band the vertex pair stands within. Past a half-turn
/// the parting grows as `4R²θ/c`, which the reach does not bound, so a
/// longer arc carries no run.
///
/// The section carried along the run reaches the pair from both sides,
/// so where the two read bounds part at their own reach, the region
/// between them is a wedge the run's loops close, not a crescent
/// narrowing to the site. A closed edge, whose far end is the site,
/// carries nothing past it.
fn coincident_run<T: Decide>(
    a: (Corner<'_, T>, bool),
    b: (Corner<'_, T>, bool),
    contacts: &super::ReductionContacts,
    band: Band,
) -> Result<bool, BooleanError> {
    let [Some(a), Some(b)] = [far_edge(a.0, a.1, band)?, far_edge(b.0, b.1, band)?] else {
        return Ok(false);
    };
    Ok(contacts
        .vv
        .iter()
        .any(|c| a.far.contains(&c.a) && b.far.contains(&c.b))
        && parallel_same(
            a.dir,
            b.dir,
            exact_lever(b.reach, Some(a.reach)),
            BOUNDS_PARALLEL,
            band,
        )?)
}

/// A corner [`crescent`] reads: one operand's body, its sectors at the
/// site, and the sector.
#[derive(Clone, Copy)]
struct Corner<'s, T: geom_core::Real> {
    body: &'s Body<T>,
    sectors: &'s [BoolSector<T>],
    sector: &'s BoolSector<T>,
}

/// A line or an arc of at most a half-turn leaving a site
/// ([`far_edge`]).
struct RunEdge<T: geom_core::Real> {
    /// Its unit direction at the site.
    dir: Vec3<T>,
    /// Its reach from the site, in metres, no shorter than its chord.
    reach: T,
    /// Its far end's site.
    far: Vec<VertexKey>,
}

/// **The edge bound of `s`'s corner** that its sectors chain from:
/// with `arrival`, the bound the corner's first sector ends on (its own
/// half-edge's departure, the CCW-last bound); else the one its last
/// sector starts on (the next half-edge's departure). `None` where the
/// sectors hold no such bound of the corner.
pub(super) fn corner_edge_bound<T: geom_core::Real>(
    sectors: &[BoolSector<T>],
    s: &BoolSector<T>,
    arrival: bool,
) -> Option<(Vec3<T>, Reach<T>)> {
    let mut corner = sectors.iter().filter(|o| o.he == s.he && o.face == s.face);
    if arrival {
        corner.find(|o| o.end_edge()).map(|o| (o.end, o.end_reach))
    } else {
        corner
            .find(|o| o.start_edge())
            .map(|o| (o.start, o.start_reach))
    }
}

/// The edge across `corner` from its `start` bound (or its end), where
/// it is an open line or an arc of at most a half-turn
/// ([`within_half_turn`], [`coincident_run`]).
fn far_edge<T: Decide>(
    corner: Corner<'_, T>,
    start: bool,
    band: Band,
) -> Result<Option<RunEdge<T>>, BooleanError> {
    let (body, s) = (corner.body, corner.sector);
    let bound = corner_edge_bound(corner.sectors, s, start);
    let he = if start {
        s.he
    } else {
        body.proven_orbit_step(s.he)
    };
    let Some((dir, reach)) = bound else {
        return Ok(None);
    };
    let half = proven(&body.half_edges, he, EntityId::HalfEdge);
    let far = body.proven_half_edge_end(he);
    if far == half.start {
        return Ok(None);
    }
    match germ_curve(body, half.edge)?.carrier() {
        geom::Curve3::Line { .. } => {}
        geom::Curve3::Circle { .. } => {
            let chord = body.resolve_vertex_point(far, Proven)
                - body.resolve_vertex_point(half.start, Proven);
            if !within_half_turn(dir, chord, band)? {
                return Ok(None);
            }
        }
        _ => return Ok(None),
    }
    Ok(Some(RunEdge {
        dir,
        reach: reach.length(),
        far: site_of(body, far)?,
    }))
}

/// Whether an arc leaving its start along `dir` and ending `chord` off
/// it turns at most a half-turn: the chord leaves ahead of `dir`, the
/// angle between them being half the arc's turn. Decided as a length;
/// in band it escalates.
fn within_half_turn<T: Decide>(
    dir: Vec3<T>,
    chord: Vec3<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    let ahead = Margin::of(dir.normalize().dot(chord));
    Ok(
        decide("bool_sector_run_ahead", ahead, band).map_err(|diag| {
            BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag)
        })? == Sign::Positive,
    )
}

/// A sector bound's side code against a face's PLANE ([`side_code`]
/// with [`NO_CURVATURE`]): what the sector passes read a bound against.
pub(super) fn plane_side_code<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    face_normal: OutwardNormal<T>,
    band: Band,
) -> Result<SideCode, BooleanError> {
    side_code(dir, reach, face_normal, NO_CURVATURE(), band)
}

/// One sector's two side codes, `(start, end)`.
type SidePair = (SideCode, SideCode);

/// Whether a sector's two bounds lie definitely on one side of the
/// other's plane: the sector meets that plane only at the vertex.
pub(super) fn one_side(codes: SidePair) -> bool {
    codes.0 == codes.1 && codes.0 != SideCode::On
}

/// The four side codes of a sector pair: each sector's bounds against
/// the other's face.
pub(super) fn pair_codes<T: Decide>(
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

/// What [`pair_search`] read of a vertex pair's sectors.
pub(super) struct PairSearch {
    /// Every intersecting (A-sector, B-sector) pair, in A-major order.
    pub records: Vec<PairRecord>,
    /// The coplanar pairs read as not overlapping, by index: sectors
    /// that share at most a bound ([`crescent_refusal`] reads them).
    pub touching: Vec<(usize, usize)>,
}

/// The all-pairs search (15.7): every intersecting (A-sector, B-sector)
/// pair becomes a [`PairRecord`] with its four side codes, in
/// deterministic A-major order.
pub(super) fn pair_search<T: Decide>(
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    band: Band,
) -> Result<PairSearch, BooleanError> {
    let mut records = Vec::new();
    let mut touching = Vec::new();
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
                let (plus, minus) = rays_within(int, &[sa, sb], DeclarationRead::Moot, band)?;
                plus || minus
            };
            if !hit {
                if coplanar {
                    touching.push((i, j));
                }
                continue;
            }
            let on = (SideCode::On, SideCode::On);
            let (sa_codes, sb_codes) = if coplanar {
                (on, on)
            } else {
                pair_codes(sa, sb, band)?
            };
            // A curved bound's code reads the edge, which can lie
            // wholly off the plane where its tangent holds the line.
            if one_side(sa_codes) || one_side(sb_codes) {
                continue;
            }
            records.push(PairRecord {
                a: i,
                b: j,
                sa: sa_codes,
                sb: sb_codes,
                intersect: true,
            });
        }
    }
    Ok(PairSearch { records, touching })
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
/// against the other face's plane. A wedge with no bisector to read is
/// not classified.
/// A corner of three or more faces is convex when no bound of any
/// sector lies in front of the plane of a face it does not bound, and
/// is then the faces' inner half-spaces met; it is hollow when none
/// lies behind one, its complement then convex, and is the faces' inner
/// half-spaces joined. A bound bounds its own sector's face and the
/// face across it: for its start the next sector's along the
/// shared-bound chain (module docs), for its end the previous one's.
///
/// A bisector or bound on, or in band of, a plane it is read against
/// says nothing of the wedge or corner where that plane is its own
/// face's, or the face's across it, facing the same way, as far as the
/// reading reaches (the edges it classes and the corner's sectors,
/// `bool_corner_one_plane`): there met and joined read every direction
/// alike, and a wedge so read is met. Anywhere else the faces part by up
/// to the band at the bound's reach, which an edge reaching farther
/// reads as metres, so the wedge or corner is read as a polygon cone,
/// each edge at its own reach. A corner no bound reads decidedly off
/// any plane is met where all its faces lie on one plane facing one
/// way, and a polygon cone otherwise.
///
/// A direction's class is its side codes against the faces combined:
/// met, `Out` past any face, else `On` on any, else `In`; joined, `In`
/// behind any face, else `On` on any, else `Out`. A direction in band
/// of a face's plane reads the corner as a polygon cone, met or joined.
pub(super) fn wedge_classes<T: Decide>(
    own: &[BoolSector<T>],
    other: &[BoolSector<T>],
    band: Band,
) -> Result<Option<WedgeRead>, BooleanError> {
    wedge_read(own, other, band).map(Result::ok)
}

/// [`wedge_classes`], saying why it read nothing where it did not: the
/// other operand's sectors on fewer than two faces, two with no
/// bisector, or a polygon cone some edge of which no reference decides
/// ([`cone_read`]'s `None`).
pub(super) fn wedge_read<T: Decide>(
    own: &[BoolSector<T>],
    other: &[BoolSector<T>],
    band: Band,
) -> Result<Result<WedgeRead, UnreadNeighbourhood>, BooleanError> {
    let cone = |read: Result<Option<WedgeRead>, BooleanError>| {
        read.map(|r| r.ok_or(UnreadNeighbourhood::ConeUnread))
    };
    let mut faces: Vec<(FaceKey, OutwardNormal<T>)> = Vec::new();
    for s in other {
        if !faces.iter().any(|&(f, _)| f == s.face) {
            faces.push((s.face, s.normal));
        }
    }
    let code = |dir, reach, normal| plane_side_code(dir, reach, normal, band);
    // The farthest the reading reaches: the edges it classes, and the
    // corner's own sectors.
    let reach = own
        .iter()
        .filter(|s| s.end_edge())
        .map(|s| s.end_reach.length())
        .chain(other.iter().map(BoolSector::span))
        .reduce(|a, b| a.max(b))
        .unwrap_or_else(T::zero);
    // Two unit normals a chord apart part their planes by that chord at
    // the reach; opposed normals stand two apart.
    let one_plane = |a: OutwardNormal<T>, b: OutwardNormal<T>| {
        matches!(
            geom_core::k_stats::decide_magnitude(
                "bool_corner_one_plane",
                Margin::levered((a.vec() - b.vec()).norm(), reach),
                band,
            ),
            Ok(Magnitude::Zero)
        )
    };
    let met = match faces.as_slice() {
        [(f0, n0), (_, n1)] => {
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
                return Ok(Err(UnreadNeighbourhood::NoBisector));
            };
            match code(dir, reach, *n1) {
                Ok(SideCode::In) => true,
                Ok(SideCode::Out) => false,
                Ok(SideCode::On) | Err(BooleanError::Escalated { .. }) if one_plane(*n0, *n1) => {
                    true
                }
                Ok(SideCode::On) | Err(BooleanError::Escalated { .. }) => {
                    return cone(cone_read(own, other, band));
                }
                Err(e) => return Err(e),
            }
        }
        [(_, first), _, _, ..] => {
            let n = other.len();
            let (mut convex, mut reflex) = (true, true);
            for (k, s) in other.iter().enumerate() {
                let (next, prev) = (&other[(k + 1) % n], &other[(k + n - 1) % n]);
                for (dir, reach, across) in
                    [(s.start, s.start_reach, next), (s.end, s.end_reach, prev)]
                {
                    for &(f, normal) in &faces {
                        if f == s.face || f == across.face {
                            continue;
                        }
                        match code(dir, reach, normal) {
                            Ok(SideCode::Out) => convex = false,
                            Ok(SideCode::In) => reflex = false,
                            Ok(SideCode::On) | Err(BooleanError::Escalated { .. })
                                if one_plane(s.normal, normal)
                                    || one_plane(across.normal, normal) => {}
                            Ok(SideCode::On) | Err(BooleanError::Escalated { .. }) => {
                                return cone(cone_read(own, other, band));
                            }
                            Err(e) => return Err(e),
                        }
                    }
                }
            }
            match (convex, reflex) {
                (true, false) => true,
                (false, true) => false,
                (true, true) if faces.iter().all(|&(_, n)| one_plane(*first, n)) => true,
                _ => return cone(cone_read(own, other, band)),
            }
        }
        _ => return Ok(Err(UnreadNeighbourhood::OneFace)),
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
                Err(BooleanError::Escalated { .. }) => return cone(cone_read(own, other, band)),
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
    Ok(Ok(WedgeRead {
        met,
        pointed: true,
        rows,
    }))
}

/// **A corner's link mean** `c`: the mean of its sectors' unit bounds,
/// with its shortest arm, the lever a reading of `c` is metered over.
/// `None` for a corner with no sectors.
pub(super) fn link_mean<T: Decide>(sectors: &[BoolSector<T>]) -> Option<(Vec3<T>, T)> {
    let arm = sectors.iter().map(|s| s.arm).reduce(|a, b| a.min(b))?;
    let sum = sectors
        .iter()
        .fold(Vec3::zero(), |acc, s| acc + s.start + s.end);
    Some((sum / T::from_f64(2.0 * sectors.len() as f64), arm))
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
    let Some((mean, arm)) = link_mean(other) else {
        return Ok(None);
    };
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
        let p = s0.middle().normalize();
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
    let sum = s.middle();
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
    use crate::contact::GlueClass;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// **A flat corner reads met**: three faces on one plane, every bound
    /// on every other face's plane, which is its own face's as far as the
    /// reading reaches, so an edge below the plane is In and one above
    /// it Out.
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
    /// corners `k` and `k + 1`, keyed `11 + k`; the sectors run in orbit
    /// order, each sharing its start with the next one's end.
    fn cone_sectors(base: &[Vec3<f64>], hollow: bool) -> Vec<BoolSector<f64>> {
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let o = Point3::new(0.0, 0.0, 0.0);
        let n = base.len();
        let mut sectors: Vec<_> = (0..n)
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
            .collect();
        if hollow {
            sectors.reverse();
        }
        sectors
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
            read: Ok(read),
            sectors: saddle,
        };
        assert!(
            matches!(
                super::super::vtxfac::touch_classes(
                    &[],
                    &[(first, SideCode::Out)],
                    &[pair],
                    &|_| Ok(FaceShape::Plane),
                    band()
                ),
                Err(super::super::vtxfac::TouchUndecided::Beside(Some(p))) if p == partner
            ),
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

    /// **A reflex edge on or in band of a plane decides no side of a
    /// direction within the band of its faces**: a dart whose notch is
    /// half the zero band deep, or halfway between the bands, reads a
    /// direction inside a lobe In and its void Out, rather than read as
    /// convex or refuse. A direction into the notch reads On where the
    /// notch is within the zero band (its faces one plane at the
    /// reading's reach, the corner met), and refuses as its side of the
    /// face it lies in band of where the notch is in band.
    #[test]
    fn a_reflex_edge_on_or_in_band_reads_as_a_polygon_cone() {
        let b = band();
        for depth in [b.zero() / 2.0, (b.zero() + b.escalate()) / 4.0] {
            for (hollow, lobe) in [(false, SideCode::In), (true, SideCode::Out)] {
                let cone = cone_sectors(&dart(depth), hollow);
                let read = wedge_classes(&probes(&[Vec3::new(0.6, 0.05, 1.0)]), &cone, b);
                assert_eq!(
                    read.unwrap().map(|r| classes(&r)),
                    Some(vec![lobe]),
                    "a lobe at depth {depth:e}, hollow {hollow}"
                );
                let notch = Vec3::new(depth / 2.0, 0.0, 1.0);
                match wedge_classes(&probes(&[notch]), &cone, b) {
                    Ok(Some(read)) if depth < b.zero() => {
                        assert_eq!(classes(&read), [SideCode::On], "the notch, hollow {hollow}");
                    }
                    Err(BooleanError::Escalated { diag, .. }) if depth > b.zero() => {
                        assert_eq!(diag.predicate, Some("bool_chord_side"), "{diag:?}");
                    }
                    other => panic!("the notch at depth {depth:e}, hollow {hollow}: {other:?}"),
                }
            }
        }
    }

    /// The sectors of a wedge about the `x` axis, in orbit order, as
    /// [`build_sectors`] subdivides a vertex inside an edge: the
    /// material sweeps counterclockwise about `+x` from the half-plane
    /// at angle `a0` (from `+y` towards `+z`) to the one at `a1`, faces
    /// keyed 11 and 12, each split at its bisector, the half-plane's
    /// own direction off the axis.
    fn wedge_sectors(a0: f64, a1: f64) -> Vec<BoolSector<f64>> {
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let o = Point3::new(0.0, 0.0, 0.0);
        let x = Vec3::new(1.0, 0.0, 0.0);
        let off = |a: f64| Vec3::new(0.0, a.cos(), a.sin());
        let (u0, u1) = (off(a0), off(a1));
        let (n0, n1) = (-x.cross(u0), x.cross(u1));
        let reach = |d: Vec3<f64>, edge: bool| {
            if edge {
                Reach::Chord {
                    base: o,
                    far: o + d,
                }
            } else {
                Reach::Bisector(1.0)
            }
        };
        [
            (1, u0, false, x, true, n0, 11),
            (2, -x, true, u0, false, n0, 11),
            (3, u1, false, -x, true, n1, 12),
            (4, x, true, u1, false, n1, 12),
        ]
        .map(
            |(he, start, s_edge, end, e_edge, normal, face)| BoolSector {
                he: HalfEdgeKey::from(key(he)),
                start,
                end,
                start_reach: reach(start, s_edge),
                end_reach: reach(end, e_edge),
                face: FaceKey::from(key(face)),
                normal: OutwardNormal::from_chart(normal, true),
                arm: 1.0,
            },
        )
        .to_vec()
    }

    /// The exact class of `d` against [`wedge_sectors`]`(a0, a1)`, read
    /// off its angle about `x`: `d` well off both planes.
    fn wedge_truth(d: Vec3<f64>, a0: f64, a1: f64) -> SideCode {
        let tau = std::f64::consts::TAU;
        let rel = (d.z.atan2(d.y) - a0).rem_euclid(tau);
        if rel < (a1 - a0).rem_euclid(tau) {
            SideCode::In
        } else {
            SideCode::Out
        }
    }

    /// **A wedge reads each arm of its bisector against the half-planes'
    /// angles**: a quarter-turn's bisector reads In and the wedge met; a
    /// three-quarter turn's reads Out and the wedge joined, where an edge
    /// in band of one face's plane past its half-plane reads the wedge
    /// as a polygon cone, decidedly inside; a half-turn's
    /// reads On on one plane facing one way, met; and a half-turn bent
    /// by half the zero band at the bisector's arm, its planes 500 bands
    /// apart at a 1 km edge, is read as a polygon cone (no side of its
    /// link decided, so not pointed), which classes an edge in the
    /// sliver between the planes by the bend. Each row is the edge's
    /// exact class by its angle.
    #[test]
    fn a_wedge_reads_each_arm_of_its_bisector() {
        use std::f64::consts::PI;
        let b = band();
        let bend = b.zero() / 2.0;
        let dirs = [
            Vec3::new(0.3, 0.8, 0.5),
            Vec3::new(-0.2, -0.6, 0.7),
            Vec3::new(0.1, -0.7, -0.6),
            Vec3::new(0.4, 0.6, -0.8),
        ];
        let sliver = |a: f64| Some(Vec3::new(0.2, a.cos(), a.sin()).normalize() * 1e3);
        let in_band = Some(Vec3::new(0.2, -1.0, (b.zero() + b.escalate()) / 2.0));
        for (what, a1, met, pointed, extra) in [
            ("a quarter-turn", PI / 2.0, true, true, None),
            ("three quarters", 1.5 * PI, false, true, in_band),
            ("a half-turn", PI, true, true, None),
            (
                "a half-turn bent reflex",
                PI + bend,
                true,
                false,
                sliver(PI + bend / 2.0),
            ),
            (
                "a half-turn bent convex",
                PI - bend,
                true,
                false,
                sliver(PI - bend / 2.0),
            ),
        ] {
            let dirs: Vec<_> = dirs.iter().copied().chain(extra).collect();
            let read = wedge_classes(&probes(&dirs), &wedge_sectors(0.0, a1), b)
                .unwrap_or_else(|e| panic!("{what}: {e:?}"))
                .unwrap_or_else(|| panic!("{what}: read nothing"));
            let want: Vec<_> = dirs.iter().map(|&d| wedge_truth(d, 0.0, a1)).collect();
            assert_eq!(classes(&read), want, "{what}");
            assert_eq!((read.met, read.pointed), (met, pointed), "{what}");
        }
    }

    /// **A direction in band of a convex corner's face reads the corner
    /// as a polygon cone, met or joined**: a triangular cone, and its
    /// complement, read a direction in band of one face's plane, past
    /// the end of its sector and decidedly past the next face, as Out of
    /// the cone (In its complement), and refuse one as far off that
    /// plane within the sector as its side of that face.
    #[test]
    fn a_direction_in_band_of_a_corners_face_reads_it_as_a_polygon_cone() {
        let b = band();
        let h = (b.zero() + b.escalate()) / 2.0;
        let base = [
            Vec3::new(1.0, 0.0, 1.0),
            Vec3::new(-0.5, 0.75_f64.sqrt(), 1.0),
            Vec3::new(-0.5, -(0.75_f64.sqrt()), 1.0),
        ];
        let n0 = base[1].cross(base[0]).normalize();
        let past = base[0] + (base[0] - base[1]) * 0.5;
        let within = (base[0] + base[1]) * 0.5;
        // `past` lies beyond the base's edge from corner 2 to corner 0,
        // on the side away from the base's centre.
        let (e, q) = (base[0] - base[2], past - base[2]);
        assert!(e.x * q.y - e.y * q.x < -1.0, "past the next face");
        for (hollow, outside) in [(false, SideCode::Out), (true, SideCode::In)] {
            let cone = cone_sectors(&base, hollow);
            for side in [1.0, -1.0] {
                let probe = past + n0 * (side * h);
                let read = wedge_classes(&probes(&[probe]), &cone, b);
                assert_eq!(
                    read.unwrap().map(|r| classes(&r)),
                    Some(vec![outside]),
                    "hollow {hollow}, past the sector {probe:?}"
                );
                let probe = within + n0 * (side * h);
                match wedge_classes(&probes(&[probe]), &cone, b) {
                    Err(BooleanError::Escalated { diag, .. }) => {
                        assert_eq!(diag.predicate, Some("bool_chord_side"), "{diag:?}");
                    }
                    other => panic!("hollow {hollow}, within the sector {probe:?}: {other:?}"),
                }
            }
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

    /// **A direction's norm does not move its parallelism or its sense**:
    /// two directions `θ` apart read one verdict at any norms, and an
    /// in-band reading refuses as the caller's coincidence under the
    /// caller's predicate. At a millimetre norm, `sin θ` clear of the band
    /// read raw is in its zero band, and at a kilometre one, `sin θ` in
    /// the zero band read raw is clear of it; a cosine just clear at its
    /// arm read raw at a millimetre norm is in its zero band.
    #[test]
    fn parallelism_reads_the_directions_and_not_their_norms() {
        let b = band();
        let (z, e) = (b.zero(), b.escalate());
        let apart = |sin: f64| {
            (
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new((1.0 - sin * sin).sqrt(), sin, 0.0),
            )
        };
        for (sin, want) in [(2.0 * e, false), (0.5 * z, true)] {
            let (u, v) = apart(sin);
            for norm in [1e-3, 1.0, 1e3] {
                assert_eq!(
                    parallel_same(u * norm, v * norm, 1.0, BOUNDS_PARALLEL, b).unwrap(),
                    want,
                    "sin θ {sin:e} at norm {norm:e}"
                );
            }
        }
        // The sense, levered at an arm its cosine only just clears: read
        // raw at a millimetre norm it would be in its zero band.
        let (u, v) = apart(0.5 * z);
        for norm in [1e-3, 1.0, 1e3] {
            for (w, want) in [(v, true), (-v, false)] {
                assert_eq!(
                    parallel_same(u * norm, w * norm, 2.0 * e, BOUNDS_PARALLEL, b).unwrap(),
                    want,
                    "the sense of {w:?} at norm {norm:e}"
                );
            }
        }
        let (u, v) = apart((z + e) / 2.0);
        for (reading, which) in [
            (BOUNDS_PARALLEL, Coincide::Sectors),
            (
                super::super::refusal_routes::RAY_COLLINEAR,
                Coincide::EdgeOnEdge,
            ),
        ] {
            let err = parallel_same(u * 1e3, v * 1e3, 1.0, reading, b).expect_err("in band");
            let BooleanError::Escalated { decision, diag } = &err else {
                panic!("{which:?}: an escalation: {err:?}");
            };
            assert_eq!(
                *decision,
                BooleanDecision::Coincidence(which, DeclarationRead::Moot),
                "the caller's coincidence"
            );
            assert_eq!(
                diag.predicate,
                Some(reading.predicate),
                "the caller's predicate"
            );
        }
    }

    /// **A degenerate direction refuses, never decides** (PR 5983's
    /// review): normalizing a zero direction is NaN in every component,
    /// so the one check must not read it as parallel or apart. Before
    /// the normalization a zero bound's cross-norm was an exact zero and
    /// read parallel on to the sense; now it must come back an error.
    #[test]
    fn a_zero_direction_refuses_rather_than_reading_parallel_or_apart() {
        let b = band();
        let u = Vec3::new(1.0, 0.0, 0.0);
        for (p, q) in [
            (Vec3::zero(), u),
            (u, Vec3::zero()),
            (Vec3::zero(), Vec3::zero()),
            (Vec3::new(f64::NAN, 0.0, 0.0), u),
        ] {
            let got = parallel_same(p, q, 1.0, BOUNDS_PARALLEL, b);
            assert!(got.is_err(), "{p:?} against {q:?} decided {got:?}");
        }
        // A direction far below the squares' underflow band is still a
        // direction: normalize prescales it.
        let tiny = Vec3::new(1e-300, 0.0, 0.0);
        assert!(parallel_same(tiny, u, 1.0, BOUNDS_PARALLEL, b).unwrap());
        assert!(!parallel_same(-tiny, u, 1.0, BOUNDS_PARALLEL, b).unwrap());
    }

    /// **Two parallel directions' sense is its own decision, and its
    /// decided zero refuses as its in-band arm does**: read at an arm in
    /// the band, and at one in the zero band, the cosine of two equal
    /// directions escalates as [`BooleanDecision::DirectionSense`] with
    /// the margin the funnel read, and both end in the corner's lever
    /// and the tolerance that margin gives (the decision passes on
    /// either definite sign, so a zero-band margin is a size a smaller
    /// tolerance decides), the lever and the floor's sentence where that
    /// tolerance lies at or below `EPS_MIN` (D4 ¶4). A clear arm reads the
    /// sense.
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
            let ending = geom_core::MarginDiag::value(arm).sized_recourse(
                b,
                geom_core::SizedWords {
                    lever: "make the edges at the corner where the two faces meet span clearly \
                            more than the tolerance",
                    size: "length of the corner's shorter edge",
                    passes: geom_core::SizedPass::NonZero,
                    no_size: None,
                    unreadable: geom_core::UNREADABLE_MARGIN_NOTE,
                    refused: geom_core::RefusedReading::AsShown,
                },
            );
            assert!(
                text.starts_with(
                    "whether two parallel directions at a corner point the same way or opposite \
                     ways is undecided: "
                ) && text.ends_with(&ending),
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

    /// A sector whose long bound leaves the other's plane by an angle
    /// its short arm leaves in band: the planes' line lies outside it
    /// read at the long bound's reach, where the codes read the whole
    /// sector on one side of the other's plane, and the pair is no
    /// record (the boss inside a hole cut into short arcs either side of
    /// the touch, at the rims' shared vertex). At the arm the line would
    /// read within.
    #[test]
    fn a_sector_wholly_off_the_others_plane_is_no_record() {
        let b = band();
        let arm = 100.0 * b.escalate();
        let theta = b.zero() / (10.0 * arm);
        let reach = 100.0 * b.escalate() / theta;
        let floor = BoolSector {
            start: Vec3::new(-theta.sin(), -theta.cos(), 0.0),
            end: Vec3::new(-1.0, 0.0, 0.0),
            start_reach: Reach::Extent(reach),
            end_reach: Reach::Bisector(arm),
            arm,
            ..sector([0.0; 3], [0.0; 3], [0.0, 0.0, -1.0])
        };
        let wall = BoolSector {
            start: Vec3::new(0.0, 0.0, -1.0),
            end: Vec3::new(0.0, -1.0, 0.0),
            start_reach: Reach::Extent(reach),
            end_reach: Reach::Extent(reach),
            arm: reach,
            ..sector([0.0; 3], [0.0; 3], [-1.0, 0.0, 0.0])
        };
        let line = Vec3::new(0.0, -1.0, 0.0);
        assert!(
            !within(&floor, line, false, DeclarationRead::Moot, b).unwrap(),
            "the line reads outside the floor at its long bound's reach"
        );
        let at_the_arm = Margin::levered(floor.start.cross(line).dot(floor.normal.vec()), arm);
        assert_eq!(
            decide("arm", at_the_arm, b).unwrap(),
            Sign::Zero,
            "at the arm"
        );
        let (codes, _) = pair_codes(&floor, &wall, b).unwrap();
        assert_eq!(codes, (SideCode::Out, SideCode::Out), "the floor's codes");
        let found = pair_search(&[floor], &[wall], b).unwrap();
        assert!(found.records.is_empty(), "records: {:?}", found.records);
    }

    /// Two upright faces through the z axis 10° apart, as a tilted
    /// prism's side face meets its neighbour's (`three_solids_on_one_line`):
    /// `a`'s normal at 20°, tipped by `tilt` so its bound down the axis
    /// leaves its plane by `1.23·tilt` at its reach, against `b`'s at 30°.
    fn faces_ten_degrees_apart(tilt: f64) -> (BoolSector<f64>, BoolSector<f64>) {
        let at = |deg: f64| {
            let (s, c) = deg.to_radians().sin_cos();
            Vec3::new(c, s, 0.0)
        };
        let down = Vec3::new(0.0, 0.0, -1.0);
        let n_a = at(20.0) + Vec3::new(0.0, 0.0, -tilt);
        let a = BoolSector {
            start: at(290.0),
            end: down,
            start_reach: Reach::Extent(0.11),
            end_reach: Reach::Extent(1.23),
            arm: 0.11,
            ..sector([0.0; 3], [0.0; 3], n_a.normalize().to_array())
        };
        let b = BoolSector {
            start: down,
            end: at(120.0),
            start_reach: Reach::Extent(1.23),
            end_reach: Reach::Extent(0.4),
            arm: 0.4,
            ..sector([0.0; 3], [0.0; 3], at(30.0).to_array())
        };
        (a, b)
    }

    /// **A face tilted within the zero against a face near parallel to it
    /// decides as upright**: `a`'s bound down the axis leaves `a`'s plane
    /// by a quarter of the zero at its reach, and the faces' unit line,
    /// steeper by `1/sin 10° = 5.76`, would read in band against it at
    /// that reach; read off `b`'s plane, as its code reads it, it grazes.
    #[test]
    fn a_tilt_within_the_zero_between_near_parallel_faces_decides_as_upright() {
        let b = band();
        let tilt = 0.2 * b.zero();
        let (ua, ub) = faces_ten_degrees_apart(0.0);
        let (ta, tb) = faces_ten_degrees_apart(tilt);
        let unit = ta.normal.vec().cross(tb.normal.vec()).normalize();
        let steep = unit.cross(ta.end).dot(ta.normal.vec()) * 1.23;
        assert!(
            steep > b.zero(),
            "the unit line reads {steep:e} off the long bound, past the zero {:e}",
            b.zero()
        );
        let rays = |a: &BoolSector<f64>, b_: &BoolSector<f64>| {
            let m = a.normal.vec().cross(b_.normal.vec());
            rays_within(m, &[a, b_], DeclarationRead::Moot, b).unwrap()
        };
        let read = |a, b_: BoolSector<f64>| {
            let found = pair_search(&[a], &[b_], b).unwrap_or_else(|e| panic!("refused: {e:?}"));
            let records: Vec<_> = found.records.iter().map(|r| (r.sa, r.sb)).collect();
            (records, found.touching)
        };
        let upright = read(ua.clone(), ub.clone());
        assert_eq!(upright.0.len(), 1, "upright: one record, {upright:?}");
        assert_eq!(read(ta.clone(), tb.clone()), upright, "tilted as upright");
        assert_eq!(rays(&ta, &tb), rays(&ua, &ub), "the rays the germ reads");
    }

    /// Faces `φ` apart about the x axis — `b` the plane z = 0, `a` tipped
    /// about x — and a sector of `a` between the bounds at `θ0` and `θ1`
    /// from +x in `a`'s plane, every reach 1: a bound at `θ` lies
    /// `sin θ sin φ` off `b`'s plane at its far end.
    fn faces_phi_apart(phi: f64, (t0, t1): (f64, f64)) -> (BoolSector<f64>, BoolSector<f64>) {
        let (sp, cp) = phi.sin_cos();
        let w = Vec3::new(0.0, cp, sp);
        let x = Vec3::new(1.0, 0.0, 0.0);
        let at = |t: f64| x * t.cos() + w * t.sin();
        let a = BoolSector {
            start: at(t0),
            end: at(t1),
            ..sector([0.0; 3], [0.0; 3], [0.0, -sp, cp])
        };
        let (s, c) = 60f64.to_radians().sin_cos();
        let b = sector([c, -s, 0.0], [c, s, 0.0], [0.0, 0.0, 1.0]);
        (a, b)
    }

    /// **A bound in band off the other face's plane never reads a wrong
    /// pair**: a bound a small angle off the line of two faces near
    /// parallel, its far end three zeros off the other's plane, its code
    /// against that face in band. Where the line lies inside the sector
    /// the pair is a record or a typed refusal; where it lies outside, no
    /// record or a typed refusal. The plane-offset reading refuses both at
    /// the line today, where the unit line read the outside case as no
    /// record; a planar body's far vertex refuses first at the vertex–face
    /// pass, so a full Boolean refused it either way.
    #[test]
    fn a_bound_in_band_off_the_others_plane_reads_no_wrong_pair() {
        let b = band();
        let theta = 0.05f64;
        let phi = 3.0 * b.zero() / theta.sin();
        for (what, span, inside) in [
            ("the line inside", (-theta, theta), true),
            ("the line outside", (theta, 3.0 * theta), false),
        ] {
            let (sa, sb) = faces_phi_apart(phi, span);
            let code = plane_side_code(sa.start, sa.start_reach, sb.normal, b);
            assert!(
                code.is_err(),
                "{what}: the bound's code reads in band: {code:?}"
            );
            match pair_search(std::slice::from_ref(&sa), std::slice::from_ref(&sb), b) {
                Ok(found) => assert_eq!(
                    found.records.len(),
                    usize::from(inside),
                    "{what}: the pair's records"
                ),
                Err(BooleanError::Escalated {
                    decision:
                        BooleanDecision::Coincidence(Coincide::Sectors | Coincide::SectorSide, _),
                    ..
                }) => {}
                Err(e) => panic!("{what}: refused untyped: {e:?}"),
            }
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

    /// A unit-radius quarter arc at the origin, leaving along `+x` in
    /// the plane `z = 0` and bending toward `bend` (unit, normal to
    /// `+x`): its tangent, and its reach as `orbit_corners` mints it.
    fn quarter_arc(bend: Vec3<f64>) -> (Vec3<f64>, Reach<f64>) {
        let span = core::f64::consts::FRAC_PI_2;
        let carrier = geom::Curve3::Circle {
            center: geom_core::Point3::origin() + bend,
            axis: Vec3::new(1.0, 0.0, 0.0).cross(bend),
            radius: 1.0,
            u_ref: -bend,
        };
        let extent = geom_brep::edge_extent_inf(&carrier, 0.0, span, 2.0 * (span / 2.0).sin());
        let tangent = carrier.deriv(0.0);
        let bend = ConicBend::of(&carrier, tangent, carrier.deriv2(0.0), span);
        (tangent.normalize() * extent, Reach::Conic { extent, bend })
    }

    /// **A conic tangent to a face's plane at the vertex reads the side
    /// it bends to, where the first order reads it On.** The arc's far
    /// end stands a whole radius off the plane, on the side it bends
    /// to; its tangent lies in the plane exactly.
    #[test]
    fn a_tangent_arc_reads_the_side_it_bends_to() {
        let b = band();
        let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        for (bend, want) in [
            (Vec3::new(0.0, 0.0, -1.0), SideCode::In),
            (Vec3::new(0.0, 0.0, 1.0), SideCode::Out),
        ] {
            let (dir, reach) = quarter_arc(bend);
            assert_eq!(
                enters_material(dir, n, reach.length(), b).unwrap(),
                EntersMaterial::Tangent,
                "the first order ties: {bend:?}"
            );
            assert_eq!(
                side_code(dir, reach, n, NO_CURVATURE(), b).unwrap(),
                SideCode::On,
                "the sector passes read the first order: {bend:?}"
            );
            assert_eq!(
                pierce_side(dir, reach, n, FaceShape::Plane, NO_CURVATURE(), b).unwrap(),
                PierceSide {
                    code: want,
                    tied: true
                },
                "the arc bending toward {bend:?} reads its side"
            );
        }
        let (dir, reach) = quarter_arc(Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(
            pierce_side(dir, reach, n, FaceShape::Plane, NO_CURVATURE(), b)
                .unwrap()
                .code,
            SideCode::On,
            "an arc in the face's plane is On"
        );
    }

    /// **Every verdict `pierce_side` gives a conic tied with a plane is
    /// true of the arc, sampled** (review row, R1 of PR 4575). Circles
    /// and ellipses through the origin, leaving with a first-order
    /// residue inside the band (none, or most of the band either way)
    /// and bending off `z = 0` by a stated multiple of the band, at
    /// spans short of, at, and past the `π/(2·κ_max)` cap, and from both
    /// ends of an ellipse's axes: an `In` must put a point of the edge
    /// past the escalation band below the plane, an `Out` above it, and
    /// an `On` keep every point within the escalation band. `In` and
    /// `Out` are checked at the witness the derivation names
    /// ([`geom_brep::conic_bend_side`]), found here by sampling.
    #[test]
    fn a_conic_tie_verdict_is_true_of_the_sampled_arc() {
        let b = band();
        let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let base = geom_core::Point3::origin();
        // A margin that decides definite clears the escalation band
        // (less the sampling's own rounding).
        let cleared = b.escalate() * (1.0 - 1e-6);
        let (mut ins, mut outs, mut ons, mut refused) = (0, 0, 0, 0);
        // (semi-axis along the start's inward normal, the other, span);
        // equal semi-axes are a circle.
        let shapes: &[(f64, f64, f64)] = &[
            (1.0, 1.0, 0.3),
            (1.0, 1.0, core::f64::consts::FRAC_PI_2),
            (1.0, 1.0, 3.0),
            (1.0, 1.0, 6.0),
            (3.0, 1.0, 0.5),
            (3.0, 1.0, 1.5),
            (3.0, 1.0, 3.0),
            (1.0, 3.0, 0.5),
            (1.0, 3.0, 1.5),
            (1.0, 3.0, 3.0),
            (10.0, 0.1, 1.0),
            (0.1, 10.0, 1.0),
        ];
        for &(along_n, along_t, span) in shapes {
            for rho in [0.0, 0.6, -0.6] {
                for q in [0.3, 0.9, 1.5, 3.0, 6.0, 9.0, 10.3, 11.0, 15.0, 30.0, 1e3] {
                    for sense in [1.0, -1.0] {
                        // The arc's own frame: it leaves along `t`,
                        // bending toward `big_n`; `u` points from the
                        // centre to the start.
                        let mk = |beta: f64, cg: f64| {
                            let t = Vec3::new(beta.cos(), 0.0, beta.sin());
                            let sg = (1.0 - cg * cg).max(0.0).sqrt();
                            let big_n = Vec3::new(-beta.sin() * cg, sg, beta.cos() * cg);
                            let u = -big_n;
                            let carrier = if along_n == along_t {
                                geom::Curve3::Circle {
                                    center: base + big_n * along_n,
                                    axis: u.cross(t),
                                    radius: along_n,
                                    u_ref: u,
                                }
                            } else {
                                geom::Curve3::Ellipse {
                                    center: base + big_n * along_n,
                                    axis: u.cross(t),
                                    semi_u: along_n,
                                    semi_v: along_t,
                                    u_ref: u,
                                }
                            };
                            (carrier, big_n)
                        };
                        let (flat, _) = mk(0.0, 1.0);
                        assert!(flat.eval(0.0).distance(base) < 1e-12);
                        assert!(flat.deriv(0.0).normalize().dot(Vec3::new(1.0, 0.0, 0.0)) > 0.999);
                        let samples = 4000;
                        let param = |i: usize| span * i as f64 / samples as f64;
                        let chord = flat.eval(span).distance(base);
                        let extent = geom_brep::edge_extent_inf(&flat, 0.0, span, chord);
                        // The witness the derivation names, found
                        // independently: the sample at arc length
                        // `l = min(max(extent, b·span), cap)`, `b` the
                        // lesser semi-axis (the least speed) and `cap`
                        // a half turn on a circle, `π/(2·κ_max)` on an
                        // ellipse, with `κ_max` sampled over the whole
                        // carrier.
                        let kmax = (0..=samples)
                            .map(|i| {
                                let t = core::f64::consts::TAU * i as f64 / samples as f64;
                                let (d1, d2) = (flat.deriv(t), flat.deriv2(t));
                                d1.cross(d2).norm() / d1.norm().powi(3)
                            })
                            .fold(0.0f64, f64::max);
                        let cap = if along_n == along_t {
                            core::f64::consts::PI * along_n
                        } else {
                            core::f64::consts::FRAC_PI_2 / kmax
                        };
                        let l = extent.max(along_n.min(along_t) * span).min(cap);
                        // The arc length `l` is found to a step of
                        // `span / samples²`: coarse steps up to the one it
                        // falls in, fine ones within it, so the witness
                        // lands on `l` however short it is beside a step.
                        let walk = |t0: f64, dt: f64, steps: usize, walked: f64| {
                            let mut walked = walked;
                            for i in 1..=steps {
                                let (a, b) = (t0 + dt * (i - 1) as f64, t0 + dt * i as f64);
                                let step = flat.eval(b).distance(flat.eval(a));
                                if walked + step >= l {
                                    return Err((a, step, walked));
                                }
                                walked += step;
                            }
                            Ok(walked)
                        };
                        let dt = span / samples as f64;
                        let t_l = match walk(0.0, dt, samples, 0.0) {
                            Ok(_) => span,
                            Err((t0, _, walked)) => {
                                match walk(t0, dt / samples as f64, samples, walked) {
                                    Ok(_) => t0 + dt,
                                    Err((a, step, walked)) => {
                                        a + dt / samples as f64 * (l - walked) / step
                                    }
                                }
                            }
                        };
                        // The arc's excursion along its own normal at `l`.
                        let depth = (flat.eval(t_l) - base).z;
                        let beta = (rho * b.zero() / extent).asin();
                        let cg = sense * (q * b.zero() / depth).min(1.0);
                        let (carrier, _) = mk(beta, cg);
                        let tangent = carrier.deriv(0.0);
                        let bend = ConicBend::of(&carrier, tangent, carrier.deriv2(0.0), span);
                        let dir = tangent.normalize() * extent;
                        let reach = Reach::Conic { extent, bend };
                        if enters_material(dir, n, extent, b) != Ok(EntersMaterial::Tangent) {
                            continue;
                        }
                        let heights: Vec<f64> = (0..=samples)
                            .map(|i| (carrier.eval(param(i)) - base).z)
                            .collect();
                        let h_l = (carrier.eval(t_l) - base).z;
                        let lo = heights.iter().copied().fold(f64::INFINITY, f64::min);
                        let hi = heights.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        let what = format!(
                            "axes ({along_n}, {along_t}), span {span}, rho {rho}, q {q}, \
                             sense {sense}: heights [{lo:e}, {hi:e}], {h_l:e} at l = {l}"
                        );
                        match pierce_side(dir, reach, n, FaceShape::Plane, NO_CURVATURE(), b) {
                            Ok(PierceSide {
                                code: SideCode::In, ..
                            }) => {
                                ins += 1;
                                assert!(
                                    h_l < -cleared,
                                    "In, but the witness is not past the band below: {what}"
                                );
                            }
                            Ok(PierceSide {
                                code: SideCode::Out,
                                ..
                            }) => {
                                outs += 1;
                                assert!(
                                    h_l > cleared,
                                    "Out, but the witness is not past the band above: {what}"
                                );
                            }
                            Ok(PierceSide {
                                code: SideCode::On, ..
                            }) => {
                                ons += 1;
                                assert!(
                                    lo.abs().max(hi.abs()) <= b.escalate(),
                                    "On, but a point leaves the band: {what}"
                                );
                            }
                            Err(BooleanError::Escalated {
                                decision: BooleanDecision::ConicBend,
                                ..
                            }) => refused += 1,
                            other => panic!("{what}: {other:?}"),
                        }
                    }
                }
            }
        }
        eprintln!("conic tie sweep: In {ins}, Out {outs}, On {ons}, refused {refused}");
        assert!(ins > 0 && outs > 0 && ons > 0 && refused > 0);
    }

    /// **An arc long beside the band whose sagitta stays inside it is
    /// `On`** (review row, R2 of PR 4575): radius 1000, and a span whose
    /// sagitta is a tenth of the band, so the arc is over `10⁴` bands
    /// long. Bending toward the plane's normal it lies in the plane to
    /// the band; a height bound read at its reach refused.
    #[test]
    fn a_long_arc_whose_sagitta_is_in_the_band_is_on() {
        let b = band();
        let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let r = 1e3;
        let span = (0.2 * b.zero() / r).sqrt();
        let z = Vec3::new(0.0, 0.0, 1.0);
        let x = Vec3::new(1.0, 0.0, 0.0);
        let carrier = geom::Curve3::Circle {
            center: geom_core::Point3::origin() + z * r,
            axis: x.cross(z),
            radius: r,
            u_ref: -z,
        };
        let chord = carrier.eval(span).distance(geom_core::Point3::origin());
        let far = (carrier.eval(span) - geom_core::Point3::origin()).z;
        assert!(
            chord > 1e4 * b.zero() && far < b.zero(),
            "the arc is long beside the band (chord {chord:e}) and its far end in it ({far:e})"
        );
        let extent = geom_brep::edge_extent_inf(&carrier, 0.0, span, chord);
        let bend = ConicBend::of(&carrier, carrier.deriv(0.0), carrier.deriv2(0.0), span);
        let dir = carrier.deriv(0.0).normalize() * extent;
        assert_eq!(
            pierce_side(
                dir,
                Reach::Conic { extent, bend },
                n,
                FaceShape::Plane,
                NO_CURVATURE(),
                b
            )
            .unwrap()
            .code,
            SideCode::On,
            "the whole arc lies within the band of the plane"
        );
    }

    /// **A conic tie refuses typed where its bend does not certify a
    /// side** (an arc so short its lift stays in the band, and a spiric
    /// section, whose curvature carries no bound), **and rides a curved
    /// face `On`**: tangent to a cylinder of its own radius, it lies on
    /// the face to first order, which is all a tangent plane says there.
    #[test]
    fn a_conic_tie_refuses_where_its_bend_certifies_nothing() {
        let b = band();
        let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let conic_bend = |r: &Result<PierceSide, BooleanError>| {
            matches!(
                r,
                Err(BooleanError::Escalated {
                    decision: BooleanDecision::ConicBend,
                    ..
                })
            )
        };
        let (dir, reach) = quarter_arc(Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(
            pierce_side(dir, reach, n, FaceShape::Curved, 1.0, b)
                .unwrap()
                .code,
            SideCode::On,
            "a tie with a curved face rides it"
        );
        let Reach::Conic { extent, bend } = reach else {
            unreachable!("`quarter_arc` mints a conic reach")
        };
        // The arc's length and extent bounded loosely, but soundly, by
        // a length whose lift `2·sin²(short/2) ≈ short²/2` is a third
        // of the zero band.
        let short = (2.0 * b.zero() / 3.0).sqrt();
        let geom_brep::BendBounds::Planar {
            least,
            reach,
            rise,
            lifts_to,
            ..
        } = bend.bounds
        else {
            unreachable!("a circle's bend is planar")
        };
        let loose = ConicBend {
            bounds: geom_brep::BendBounds::Planar {
                least,
                reach,
                rise,
                arc: short,
                lifts_to,
            },
            ..bend
        };
        let got = pierce_side(
            dir,
            Reach::Conic {
                extent: short,
                bend: loose,
            },
            n,
            FaceShape::Plane,
            NO_CURVATURE(),
            b,
        );
        assert!(conic_bend(&got), "a lift inside the band: {got:?}");
        let spiric = Reach::Conic {
            extent,
            bend: ConicBend {
                bounds: geom_brep::BendBounds::Unbounded,
                ..bend
            },
        };
        let got = pierce_side(dir, spiric, n, FaceShape::Plane, NO_CURVATURE(), b);
        assert!(conic_bend(&got), "a spiric tie: {got:?}");
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
        let accel = |s| geom_brep::implicit_graph_acceleration(s, p, d, n.vec());
        let arm = (2.0 * mid / (accel(&ball) - accel(&floor)).abs()).sqrt();
        let err = tangent_relative_side(
            &ball,
            &floor,
            n,
            p,
            d,
            arm,
            DeclarationRead::Spent(GlueClass::Tangent),
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
                DeclarationRead::Spent(GlueClass::Tangent)
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
        // The arm re-quotes the rise it meters only where it offers a
        // tolerance to decide both (D4 ¶4's floor withholds one); else it
        // keeps its own margin. Either way the offer obeys the floor.
        let quoted = if b.offer_deciding(short).is_some() {
            departure(short)
        } else {
            short
        };
        let ending = geom_core::MarginDiag::value(quoted).sized_recourse(
            b,
            geom_core::SizedWords {
                lever: "make the edges at the corner where the two faces meet span clearly more \
                        than the tolerance",
                size: "edge's length or rise",
                passes: geom_core::SizedPass::Positive,
                no_size: None,
                unreadable: geom_core::UNREADABLE_MARGIN_NOTE,
                refused: geom_core::RefusedReading::AsShown,
            },
        );
        assert!(
            matches!(
                err,
                BooleanError::Escalated {
                    decision: BooleanDecision::LeverArm(LeverArm::SectorSide),
                    ..
                }
            ) && text.contains(&format!("margin {quoted:e} lies within the zero band"))
                && text.ends_with(&ending)
                && !text.contains("kernel bug"),
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
        let recs = pair_search(&[tilted], &[top], band()).unwrap().records;
        assert_eq!(recs.len(), 1, "the sectors overlap");
        let on = (SideCode::On, SideCode::On);
        assert_eq!((recs[0].sa, recs[0].sb), (on, on), "a coincidence record");
    }

    /// The generic pair search on two orthogonal quarter-sector fans:
    /// records carry transition codes.
    /// An arc short of a half-turn carries a run and a reflex one does
    /// not, at a chord as short as the crescent's short arcs: a reflex
    /// arc through both ends of another, tangent to it within the band
    /// at its reach, parts from it by up to `4R²θ/c` across the turn.
    #[test]
    fn only_an_arc_of_at_most_a_half_turn_carries_a_run() {
        use core::f64::consts::{PI, TAU};
        let r = 0.1;
        for (turn, carries) in [
            (0.3, true),
            (PI - 0.01, true),
            (PI + 0.01, false),
            (TAU - 1e-3, false),
        ] {
            let dir = Vec3::new(0.0, 1.0, 0.0);
            let far = Vec3::new(r - r * turn.cos(), r * turn.sin(), 0.0);
            assert_eq!(
                within_half_turn(dir, far, band()).unwrap(),
                carries,
                "an arc of turn {turn}"
            );
        }
    }

    #[test]
    fn pair_search_generic_crossing() {
        // A-sector in the xy-plane (normal +z), sweeping +x → +y CCW
        // around +z; B-sector in the zx-plane (normal +y), sweeping
        // +z → +x CCW around +y. They share the boundary ray +x.
        let a = sector([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        let bsec = sector([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let recs = pair_search(&[a], &[bsec], band()).unwrap().records;
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

    /// A sector's two bounds at the fixtures' touch, both reaching
    /// `reach`: one along the shared ruling (y), one across it (x).
    fn fixture_bounds(reach: f64) -> [(Vec3<f64>, f64); 2] {
        [
            (Vec3::new(0.0, 1.0, 0.0), reach),
            (Vec3::new(1.0, 0.0, 0.0), reach),
        ]
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
            OutwardNormal::from_chart(Vec3::new(0.0, 0.0, -1.0), true),
            p,
            super::super::BooleanOp::Union,
            Operand::B,
            FaceKey::default(),
            fixture_bounds(0.5),
            0.5,
            DeclarationRead::Spent(GlueClass::Tangent),
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
            OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true),
            p,
            super::super::BooleanOp::Union,
            Operand::A,
            FaceKey::default(),
            fixture_bounds(0.5),
            0.5,
            DeclarationRead::Spent(GlueClass::Tangent),
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
            OutwardNormal::from_chart(Vec3::new(0.0, 0.0, -1.0), true),
            p,
            super::super::BooleanOp::Union,
            Operand::A,
            FaceKey::default(),
            fixture_bounds(0.25),
            0.25,
            DeclarationRead::Spent(GlueClass::Tangent),
            b,
        )
        .unwrap();
        assert_eq!(lump, SideCode::In);
        // At an exact second-order zero the two outward normals AGREE
        // here, so the osculating residue lumps as a continuation does,
        // not as the opposed contact the external pair below lumps as.
        let zero_arm = (b.zero() * 0.5).sqrt();
        let at_zero = tangent_lump(
            &y_cyl(1.25, 0.25),
            &y_cyl(1.5, 0.5),
            fixture_reach(),
            fat_out,
            OutwardNormal::from_chart(Vec3::new(0.0, 0.0, -1.0), true),
            p,
            super::super::BooleanOp::Union,
            Operand::A,
            FaceKey::default(),
            fixture_bounds(zero_arm),
            zero_arm,
            DeclarationRead::Spent(GlueClass::Tangent),
            b,
        )
        .unwrap();
        let aligned = super::super::tables::eq15_3_lump(
            super::super::BooleanOp::Union,
            Operand::A,
            super::super::plane_eq::PlaneRelation::SameOriented,
        );
        let opposed = super::super::tables::eq15_3_lump(
            super::super::BooleanOp::Union,
            Operand::A,
            super::super::plane_eq::PlaneRelation::SameOpposite,
        );
        assert_ne!(
            aligned, opposed,
            "the fixture's op tells the two senses apart"
        );
        assert_eq!(
            at_zero, aligned,
            "an aligned exact zero lumps by its senses"
        );
    }

    /// Three-outcome honesty on the metered row: the SAME external
    /// tangency at three lever arms — definite (Out), in-band
    /// (escalates, an osculating pair is a sliver at this ε), and
    /// exactly-zero displacement (the isolated osculating residue, its
    /// opposed senses the Eq. 15.3 minus-lump, In for a Union A-sector).
    #[test]
    fn tangent_lump_is_three_outcome_honest_on_the_arm() {
        let b = band();
        let p = geom_core::Point3::new(2.0, 0.5, 1.0);
        let plate_out = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let lump = |bounds: [(Vec3<f64>, f64); 2], arm: f64| {
            tangent_lump(
                &y_cyl(1.5, 0.5),
                &plate_top(),
                fixture_reach(),
                plate_out,
                OutwardNormal::from_chart(Vec3::new(0.0, 0.0, -1.0), true),
                p,
                super::super::BooleanOp::Union,
                Operand::A,
                FaceKey::default(),
                bounds,
                arm,
                DeclarationRead::Spent(GlueClass::Tangent),
                b,
            )
        };
        let run = |arm: f64| lump(fixture_bounds(arm), arm);
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
        // A sector whose ruling bound is too short for any departure to
        // show still reads the side its long cross bound shows, not the
        // senses' lump: each bound is read over its own reach.
        let short_ruling = [
            (Vec3::new(0.0, 1.0, 0.0), zero_arm),
            (Vec3::new(1.0, 0.0, 0.0), 0.5),
        ];
        assert_eq!(lump(short_ruling, zero_arm).unwrap(), SideCode::Out);
        // A short cross bound in band beside a ruling escalates.
        let short_cross = [
            (Vec3::new(0.0, 1.0, 0.0), 0.5),
            (Vec3::new(1.0, 0.0, 0.0), inband_arm),
        ];
        assert!(
            matches!(lump(short_cross, 0.5), Err(BooleanError::Escalated { .. })),
            "an in-band cross bound beside a ruling escalates"
        );
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
            OutwardNormal::from_chart(Vec3::new(0.0, 0.0, -1.0), true),
            p,
            super::super::BooleanOp::Union,
            Operand::A,
            FaceKey::default(),
            fixture_bounds(0.5),
            0.5,
            DeclarationRead::Spent(GlueClass::Tangent),
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
            OutwardNormal::from_chart(Vec3::new(0.0, 0.0, -1.0), true),
            p,
            super::super::BooleanOp::Union,
            Operand::A,
            FaceKey::default(),
            fixture_bounds(0.5),
            0.5,
            DeclarationRead::Spent(GlueClass::Tangent),
            b,
        ) {
            Err(BooleanError::CurvedBooleanUnsupported { .. }) => {}
            other => panic!("sphere tangency is outside the DEV-1 lane: {other:?}"),
        }
    }

    /// **A cone sector's normal is the cone face's outward normal at
    /// its corner**, at every rim corner of a frustum wall and for both
    /// senses: the chart normal `ŵ cos α − ẑ sin α` there, negated where
    /// the face is reversed, as a subtracted cone's is.
    #[test]
    fn a_cone_sector_off_the_apex_is_the_faces_outward_normal() {
        let alpha = 0.5_f64;
        let (mut body, face) = super::super::boxes::tests::cone_wall(alpha, 0.0, 1.0, 0.5, 1.0);
        let vertices: Vec<VertexKey> = body.vertices.keys().collect();
        for sense in [true, false] {
            body.set_face_sense(face, sense).unwrap();
            let mut read = 0;
            for &v in &vertices {
                let p = body.resolve_vertex_point(v, Proven);
                let w = Vec3::new(p.x, p.y, 0.0).normalize();
                let chart = w * alpha.cos() - Vec3::new(0.0, 0.0, alpha.sin());
                let want = OutwardNormal::from_chart(chart, sense).vec();
                for he in body.vertex_orbit_linked(v) {
                    match sector_face(&body, Operand::B, v, he, band()) {
                        Ok((f, n)) if f == face => {
                            read += 1;
                            assert!(
                                (n.vec() - want).norm() < 1e-12,
                                "sense {sense} at {p:?}: {:?}, want {want:?}",
                                n.vec()
                            );
                        }
                        _ => {}
                    }
                }
            }
            assert_eq!(read, 4, "sense {sense}: the wall's four corners");
        }
    }

    /// **A sector based at a cone's apex refuses, named.** A prism's side
    /// face is carried by a cone whose apex is one of the face's corners:
    /// the sector there refuses with the apex refusal, naming the operand
    /// and the face, where the walk's gradient is `0/0`; the face's next
    /// corner, off the axis, reads its normal. Without the check the
    /// poisoned normal reaches the corner rung, which escalated it as
    /// `Corner(Straight)` on a plane through a full cone's apex.
    #[test]
    fn a_cone_sector_at_the_apex_refuses_naming_the_face() {
        let p = crate::fixtures::raw_prism(3, Tol::witness());
        let face = p.face_side[0];
        let mut body = p.body;
        let he = body
            .get_loop(body.get_face(face).unwrap().outer)
            .and_then(|l| match l.boundary {
                crate::entity::LoopBoundary::Cycle { first } => Some(first),
                crate::entity::LoopBoundary::Empty { .. } => None,
            })
            .unwrap();
        // The sector CW-after `mate(he)` is this face's corner at `he`'s
        // start; `next` is the corner after it.
        let corner = |body: &Body<f64>, he: HalfEdgeKey| {
            let orbit_he = body.mate(he).unwrap();
            (body.get_half_edge(orbit_he).unwrap().start, orbit_he)
        };
        let (apex_vertex, apex_he) = corner(&body, he);
        let next = body.get_half_edge(he).unwrap().next;
        let (off_vertex, off_he) = corner(&body, next);
        let apex = body.resolve_vertex_point(apex_vertex, Proven);
        let off = body.resolve_vertex_point(off_vertex, Proven);
        // An axis square to the edge between the two corners, so the
        // second stands the edge's length off it.
        let edge = off - apex;
        let axis = edge.cross(Vec3::new(1.0, 1.0, 1.0)).normalize();
        assert!(
            edge.cross(axis).norm() > 0.1,
            "the second corner stands off the axis"
        );
        body.set_face_surface(
            face,
            crate::construction::FaceSurface::New {
                surface: geom::Surface::Cone {
                    apex,
                    axis,
                    half_angle: 0.5,
                    u_ref: edge.normalize(),
                },
                sense: true,
            },
        )
        .unwrap();
        let b = band();
        assert!(
            matches!(
                sector_face(&body, Operand::A, apex_vertex, apex_he, b),
                Err(BooleanError::NormalAtConeApex { operand: Operand::A, face: f, band })
                    if f == face && band == b
            ),
            "the apex corner: {:?}",
            sector_face(&body, Operand::A, apex_vertex, apex_he, b)
        );
        let (f, n) = sector_face(&body, Operand::A, off_vertex, off_he, b)
            .expect("a corner off the axis reads its normal");
        assert!(f == face && n.vec().norm().is_finite(), "{n:?}");
    }

    /// **A cone's lever is the bend nearest the axis the charge can
    /// read.** A bound leaving a pierce at `ρ = 0.1` along the generator
    /// toward the apex, reaching 0.08, at a slope whose charge clears
    /// the band read at `ρ` and does not read at the lever, `2ρ/3`
    /// (`ρ − reach` is shorter, and the charge reads no further than
    /// `ρ/3`): the side refuses, where read at `ρ` it certifies.
    /// Measured, the reading at `ρ` never decides a wrong side on a cone
    /// (over 2·10⁵ poses the ray's true separation never fell below the
    /// charge's claim), so this row holds the lever against loosening,
    /// not a wrong answer. A short bound takes `ρ − reach`, and a
    /// sphere's lever is its own at every reach.
    #[test]
    fn a_cone_lever_is_the_bend_nearest_the_axis_within_reach() {
        let b = band();
        let alpha = 0.5_f64;
        let cone = geom::Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: alpha,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let (rho, reach) = (0.1, 0.08);
        let p = Point3::new(rho, 0.0, rho / alpha.tan());
        let chart = Vec3::new(alpha.cos(), 0.0, -alpha.sin());
        let normal = OutwardNormal::from_chart(chart, true);
        let toward_apex = -(p - Point3::origin()).normalize();
        // The charge at slope `s` is `s²·lever/4`: 1.2 escalations at
        // `ρ`, 0.8 of one at `2ρ/3`.
        let s = (4.8 * b.escalate() / rho).sqrt();
        let dir = toward_apex * (1.0 - s * s).sqrt() + chart * s;
        let bound = Reach::Chord {
            base: p,
            far: p + dir * reach,
        };
        let lever = PierceLever::of(&cone, p);
        let within = lever.within(reach);
        assert!(
            (within - 2.0 * rho / 3.0).abs() < 1e-15,
            "the lever a long bound reads: {within}"
        );
        assert!(
            (lever.within(0.01) - (rho - 0.01)).abs() < 1e-15,
            "the lever a short bound reads: {}",
            lever.within(0.01)
        );
        assert!(
            matches!(
                side_code(dir, bound, normal, within, b),
                Err(BooleanError::CurvedSectorSideUnsupported { .. }
                    | BooleanError::Escalated {
                        decision: BooleanDecision::PierceCurvature,
                        ..
                    })
            ),
            "read at the lever the side refuses: {:?}",
            side_code(dir, bound, normal, within, b)
        );
        assert_eq!(
            side_code(dir, bound, normal, rho, b).unwrap(),
            SideCode::Out,
            "read at ρ the same side certifies"
        );
        let sphere = geom::Surface::Sphere {
            center: Point3::origin(),
            radius: 2.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        assert_eq!(
            PierceLever::of(&sphere, p).within(1e9),
            2.0,
            "a sphere's lever does not depend on the reach"
        );
    }
}

#[cfg(test)]
mod cone_fuzz;

/// **A spline leaves a vertex along its tangent, not its chord.** The
/// sweep rows' Bézier sheet (`reduce::planar_lane_carrier_rows`): the
/// arc from `(0, 0, 0)` through the control point `(1, 2, 0)` to
/// `(2, 0, 0)` leaves its start along `(1, 2, 0)`, the first leg of its
/// control net, where its chord runs along `x̂`, in the plane `y = 0`
/// that holds both its ends.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod spline_departure_rows {
    use super::super::Operand;
    use super::{Reach, orbit_corners};
    use crate::boolean::reduce::planar_lane_carrier_rows::arc_sheet;
    use geom_core::{Band, Tol, Vec3};

    #[test]
    fn the_sector_walk_reads_a_splines_tangent() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (body, edge) = arc_sheet(1.0, 1.0);
        let he = body.get_edge(edge).unwrap().he_plus;
        let vertex = body.get_half_edge(he).unwrap().start;
        let corner = orbit_corners(&body, Operand::A, vertex, band)
            .map(Result::unwrap)
            .find(|c| c.he == he)
            .expect("the arc's half leaves its start vertex");
        let tangent = Vec3::new(1.0, 2.0, 0.0).normalize();
        assert!(
            (corner.end.normalize() - tangent).norm() < 1e-12,
            "the arc leaves along its tangent (1, 2, 0)/√5, got {:?}",
            corner.end
        );
        assert!(
            matches!(corner.end_reach, Reach::Conic { extent, .. } if (extent - 2.0).abs() < 1e-12),
            "levered at its extent, the chord's length 2: {:?}",
            corner.end_reach
        );
    }
}

/// The boolean's adaptation of the shared walk's spline refusals
/// ([`sector_face`]): no operand the boolean builds today reaches a
/// vertex on a crease of a spline face (a crossing on a corner's own
/// knot refuses first, at the crossing rung), so the mapping is pinned
/// here, on the shared walk's own fixture.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod spline_refusal_rows {
    use super::*;
    use crate::sector_face::tests::{planar_chart, sector_of, split_at_half};
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A vertex on a crease of a spline face refuses
    /// `NormalAtSplineCrease`, naming the operand, the face and the band.
    #[test]
    fn a_crease_vertex_refuses_normal_at_spline_crease() {
        let creased = planar_chart(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1, &[0.0, 0.5, 1.0]);
        let (body, top, mid) = split_at_half(creased);
        let got = sector_face(&body, Operand::B, mid, sector_of(&body, mid, top), band());
        assert!(
            matches!(
                got,
                Err(BooleanError::NormalAtSplineCrease { operand: Operand::B, face, band: b })
                    if face == top && b == band()
            ),
            "{got:?}"
        );
    }

    /// A spline face with no chart image of the edge into the vertex
    /// refuses `CurvedBooleanUnsupported` on its own kind.
    #[test]
    fn a_spline_face_with_no_pcurve_refuses_curved_boolean_unsupported() {
        let smooth = planar_chart(vec![0.0, 0.0, 1.0, 1.0], 1, &[0.0, 1.0]);
        let (mut body, top, mid) = split_at_half(smooth);
        let he = sector_of(&body, mid, top);
        let into = body.mate(he).unwrap();
        body.detach_pcurve(into)
            .expect("the half-edge stores its row");
        let got = sector_face(&body, Operand::A, mid, he, band());
        assert!(
            matches!(
                got,
                Err(BooleanError::CurvedBooleanUnsupported {
                    operand: Operand::A,
                    face,
                    kind: geom::SurfaceKind::Nurbs,
                }) if face == top
            ),
            "{got:?}"
        );
    }
}

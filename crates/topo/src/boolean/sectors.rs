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
//! A corner at a cone's APEX has no tangent plane and is no planar
//! wedge: it is a fan of the cone's rulings, from one bound's ruling
//! round the axis to the other's ([`BoolFace::Apex`]). Its bounds are
//! still rulings stored as `start`/`end` vectors, chained as above, but
//! "convex" is read round the axis, not in a plane: the corner is
//! subdivided at each extremal ruling of the plane it is read against
//! that lies inside it ([`ApexPlane`], [`crate::sector_shape::apex_corner_shape`]),
//! so that across each sub-sector the plane's side along its rulings is
//! monotone, and its middle is the ruling halfway round
//! ([`BoolSector::middle`]), not its bounds' sum. Only the pierce reads
//! such a sector; the vertex–vertex lane refuses it
//! ([`apex_pair_refusal`]).
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
use geom_brep::{ConicDeparture, EntersMaterial, OutwardNormal};
use geom_core::k_stats::{Magnitude, NonzeroSign};
use geom_core::{Band, Decide, Decided, Margin, Point3, Sign, Vec3};

use super::refusal_routes::{BOUNDS_PARALLEL, Parallelism};
use super::{
    BooleanDecision, BooleanError, Coincide, DeclarationRead, LeverArm, Operand, SideCode,
};
use crate::body::Body;
use crate::chord_join::StaleSite;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::live::{Proven, linked, proven};
use crate::sector_face::{ConeApex, SectorCarrier, SectorFaceError, VertexFace};

use crate::sector_shape::{
    ApexShape, CornerShape, FullTurn, SectorFault, SectorShape, SlitTips, TurnOrder,
    apex_corner_shape, corner_shape,
};
use crate::validate::decide;

use crate::bound_side::{BoundSide, BoundSideFault, FaceDatum, FirstOrder, Side};
pub(super) use crate::bound_side::{FaceShape, Reach, sagitta_charge};

/// One (convex) sector of a vertex neighborhood.
#[derive(Clone, Debug)]
pub(crate) struct BoolSector<T: geom_core::Real> {
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
    /// The sector's face.
    pub face: FaceKey,
    /// What the face is at the base vertex ([`sector_face`]): its
    /// tangent plane's outward unit normal, minted once from the CHART
    /// normal and the face's `sense` bit (S10), or a cone's apex, read
    /// as its rulings. Consumers pair a normal with the stored orbit
    /// order and must NOT re-apply the sense — the type says the sense
    /// is in.
    pub at_vertex: BoolFace<T>,
    /// The metering arm: the shorter of its bounds' reaches, in metres.
    pub arm: T,
}

impl FaceShape {
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

    /// The sector's middle direction. A planar sector's is unnormalized,
    /// its bounds' sum, inside the sector, which is convex. A sector at a
    /// cone's apex is a fan of rulings round the axis (module docs), whose
    /// bounds' sum points inside the cone, off every ruling, and for a fan
    /// over a half-turn wide away from it: its middle is the ruling
    /// halfway round from `start` to `end` in the face's sweep, unit.
    pub(super) fn middle(&self) -> Vec3<T>
    where
        T: Decide,
    {
        match self.apex() {
            Some(apex) => {
                let half = apex.azimuth_from(self.start, self.end) * T::from_f64(0.5);
                apex.ruling_from(self.start, half)
            }
            None => self.start + self.end,
        }
    }

    /// The farther of the sector's two bounds' reaches, in metres: how
    /// far from the base vertex the sector's own geometry is known.
    pub(super) fn span(&self) -> T {
        self.start_reach.length().max(self.end_reach.length())
    }
    /// The face's outward normal at the base vertex, its tangent plane's.
    /// Only a pierce reads a sector at a cone's apex, through its own
    /// arms ([`Self::apex`]); the vertex–vertex lane refuses an apex
    /// sector before it reads any ([`apex_pair_refusal`]).
    ///
    /// The panic at an apex rests on that order, not on the type: the
    /// readers of a tangent are some sixty sites across both lanes'
    /// pair algebra, and an apex sector reaching one is a lane that
    /// skipped its apex arm, a kernel defect a typed answer would only
    /// carry further.
    #[track_caller]
    pub(super) fn tangent(&self) -> OutwardNormal<T> {
        match self.at_vertex {
            BoolFace::Tangent(normal) => normal,
            BoolFace::Apex(_) => unreachable!(
                "a sector at a cone's apex has no tangent plane: the pierce reads it by its \
                 apex arms, and the vertex-vertex lane refuses it first"
            ),
        }
    }

    /// The cone the sector's face is at its apex, if the base vertex is
    /// one.
    pub(super) fn apex(&self) -> Option<&ConeApex<T>> {
        match &self.at_vertex {
            BoolFace::Apex(apex) => Some(apex),
            BoolFace::Tangent(_) => None,
        }
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
    /// What the face is at the vertex ([`sector_face`]).
    pub at_vertex: BoolFace<T>,
}

/// **The plane a corner at a cone's apex is read against**
/// ([`build_sectors`]). One at most: two planes' rulings are ordered
/// round the axis by a decision of their own, the vertex–vertex lane's
/// (`work/splitdoor/a-cone-with-its-apex-against-its-copy-refuses-normal-at-cone-apex.md`),
/// whose unit widens this.
#[derive(Clone, Copy, Debug)]
pub(super) enum ApexPlane<T: geom_core::Real> {
    /// No plane: the vertex–vertex lane, which refuses an apex sector
    /// ([`apex_pair_refusal`]), and every walk of a vertex no cone's
    /// apex is.
    None,
    /// The unit normal of the pierced face's plane through the apex.
    One(Vec3<T>),
}

/// **What a sector's face is at its base vertex, as the Boolean reads
/// it**: [`VertexFace`] less its near-axis arm, which the Boolean
/// refuses where it resolves the face ([`sector_face`]), so no reader
/// downstream has an arm to rule out.
#[derive(Clone, Copy, Debug)]
pub(crate) enum BoolFace<T: geom_core::Real> {
    /// The face's outward unit normal at the vertex
    /// ([`VertexFace::Tangent`]).
    Tangent(OutwardNormal<T>),
    /// The vertex is the apex of the face's cone
    /// ([`VertexFace::Apex`]).
    Apex(ConeApex<T>),
}

#[cfg(test)]
impl<T: geom_core::Real> BoolFace<T> {
    /// The tangent plane's normal; a row that reads an apex panics.
    #[allow(clippy::panic)]
    pub(super) fn tangent(&self) -> Vec3<T> {
        match self {
            Self::Tangent(normal) => normal.vec(),
            Self::Apex(apex) => panic!("a tangent plane, not {apex:?}"),
        }
    }
}

/// Builds the sector array of `vertex`'s neighborhood (module docs).
///
/// A corner between a slit's or ring's two coincident edges at its tip,
/// or round a wrap edge's tip, its one edge walked out and back
/// ([`SlitTips`], read off the operand at rest), is the carrying face's
/// full turn round the tip ([`corner_shape`]), pushed as four quarter
/// sectors, each convex.
///
/// A corner at a cone's apex has no tangent plane: its directions are
/// the cone's rulings between its bounds. Read against a plane through
/// the apex ([`ApexPlane::One`], the unit normal of the face it is read
/// against), it is subdivided at each extremal ruling of the plane that
/// lies strictly inside it ([`apex_corner_shape`]), so that between two
/// neighbouring bounds the plane's side is monotone; a ruling bound is a
/// bisector's. Against none ([`ApexPlane::None`]), the corner is one
/// sector, read by nothing but the refusal of the lane that has no apex
/// algebra ([`apex_pair_refusal`]).
pub(super) fn build_sectors<T: Decide>(
    body: &Body<T>,
    operand: Operand,
    vertex: VertexKey,
    tips: &SlitTips,
    plane: ApexPlane<T>,
    band: Band,
) -> Result<Vec<BoolSector<T>>, BooleanError> {
    let corners = orbit_corners(body, operand, vertex, band);
    // A lone orbit half-edge's corner runs from its departure round to
    // it.
    let alone = corners.len() == 1;
    let mut sectors = Vec::with_capacity(corners.len() + 2);
    for corner in corners {
        let corner = corner?;
        let bounds = tips.bounds(vertex, corner.face, alone);
        let normal = match corner.at_vertex {
            BoolFace::Tangent(normal) => normal,
            BoolFace::Apex(apex) => {
                let face = corner.face;
                push_apex_corner(&mut sectors, corner, &apex, bounds, plane, band)
                    .map_err(|fault| sector_fault(fault, vertex, face))?;
                continue;
            }
        };
        let OrbitCorner {
            he,
            start: dir_start,
            start_reach: reach_start,
            end: dir_end,
            end_reach: reach_end,
            face,
            at_vertex,
        } = corner;
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
        let shape = match corner_shape(dir_end, dir_start, normal, bounds, band) {
            Ok(CornerShape::Sector(shape)) => shape,
            Ok(CornerShape::FullTurn(turn)) => {
                push_full_turn(
                    &mut sectors,
                    he,
                    &turn,
                    (reach_start, reach_end),
                    face,
                    at_vertex,
                );
                continue;
            }
            Err(fault) => return Err(sector_fault(fault, vertex, face)),
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
                at_vertex,
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
                    at_vertex,
                    arm,
                });
                sectors.push(BoolSector {
                    he,
                    start: u_start,
                    end: b,
                    start_reach: reach_start,
                    end_reach: Reach::Bisector(arm),
                    face,
                    at_vertex,
                    arm,
                });
            }
        }
    }
    Ok(sectors)
}

/// A sector rung's fault at `vertex`'s corner in `face`, as the
/// Boolean refuses it.
fn sector_fault(fault: SectorFault, vertex: VertexKey, face: FaceKey) -> BooleanError {
    match fault {
        SectorFault::NonFiniteChord => BooleanError::NonFiniteSectorChord { vertex, face },
        SectorFault::UnderflowedChord => BooleanError::UnderflowedSectorChord { vertex, face },
        SectorFault::Rung { rung, diag } => BooleanError::Escalated {
            decision: BooleanDecision::Corner(rung),
            diag,
        },
    }
}

/// Pushes the corner at a cone's apex ([`build_sectors`]) in the
/// chained order a subdivided corner is pushed in: from its own bound,
/// one sector per stretch between neighbouring rulings, each extremal
/// ruling of `plane` strictly inside the corner ([`apex_corner_shape`])
/// a bound levered at the corner's arm.
fn push_apex_corner<T: Decide>(
    sectors: &mut Vec<BoolSector<T>>,
    corner: OrbitCorner<T>,
    apex: &ConeApex<T>,
    bounds: crate::sector_shape::CornerBounds,
    plane: ApexPlane<T>,
    band: Band,
) -> Result<(), SectorFault> {
    let OrbitCorner {
        he,
        start,
        start_reach,
        end,
        end_reach,
        face,
        at_vertex,
    } = corner;
    // With no plane, the axis's own: square to the axis, it holds no
    // extremal ruling, and the corner reads its arm and its window.
    let across = match plane {
        ApexPlane::None => apex.axis,
        ApexPlane::One(across) => across,
    };
    // In orbit order, from the own bound.
    let ApexShape { arm, rulings } = apex_corner_shape((end, start), apex, bounds, across, band)?;
    let (unit_end, unit_start) = (end.normalize(), start.normalize());
    let inner = Reach::Bisector(arm);
    let mut from = (unit_end, end_reach);
    for to in rulings
        .into_iter()
        .map(|r| (r, inner))
        .chain([(unit_start, start_reach)])
    {
        sectors.push(BoolSector {
            he,
            start: to.0,
            end: from.0,
            start_reach: to.1,
            end_reach: from.1,
            face,
            at_vertex,
            arm,
        });
        from = to;
    }
    Ok(())
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
    at_vertex: BoolFace<T>,
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
            at_vertex,
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
        let (face, at_vertex) = sector_face(body, operand, vertex, he, band)?;
        Ok(OrbitCorner {
            he,
            start,
            start_reach,
            end,
            end_reach,
            face,
            at_vertex,
        })
    })
}

/// The sector's face, and what it is at the base vertex.
///
/// The walk and the normals are [`crate::sector_face`] — ONE
/// implementation, called from here and from the splitting lane's
/// sector walk. What stays here is this lane's
/// adaptation of it: the boolean error type, whose every arm carries
/// the [`Operand`] the shared walk has no notion of. All six wired arms
/// — `Plane`, `Cylinder`, `Sphere`, `Torus`, `Cone`, `Spline` — are live
/// on this side; kinds without one refuse typed (C12.1, per arm), and a
/// spline face refuses [`BooleanError::NormalAtSplineCrease`] at a
/// vertex on a crease or a pole of its chart, where it has no one
/// normal.
///
/// A cone face at its apex is its circle of rulings
/// ([`VertexFace::Apex`]), which the pierce reads by its apex arms and
/// the vertex–vertex lane refuses ([`apex_pair_refusal`]). A vertex in
/// the band of a cone's axis, its tangent plane read ill-conditioned or
/// neither it nor the apex decided, refuses
/// [`BooleanError::VertexNearConeAxis`].
///
/// A tangent plane's normal arrives as an [`OutwardNormal`] with the
/// face's `sense` folded in (S10), minted at the shared chokepoint —
/// which makes that chokepoint the source for the whole vertex-vertex
/// lane. Everything downstream of [`BoolSector::tangent`] — `within`,
/// `side_code`, `sector_overlap`, the wideness/bisector algebra above,
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
) -> Result<(FaceKey, BoolFace<T>), BooleanError> {
    let near_axis = |face| BooleanError::VertexNearConeAxis {
        operand,
        face,
        band,
    };
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
        SectorFaceError::ApexUndecided { face, .. } => near_axis(face),
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
        VertexFace::NearAxis(_) => Err(near_axis(resolved.face)),
        VertexFace::Tangent(normal) => Ok((resolved.face, BoolFace::Tangent(normal))),
        VertexFace::Apex(apex) => Ok((resolved.face, BoolFace::Apex(apex))),
    }
}

/// **The vertex–vertex lane at a cone's apex**: a pair one of whose
/// vertices is the apex of a cone face of its operand refuses
/// [`BooleanError::VertexPairAtConeApex`], before any pair reading. A
/// sector there is a fan of rulings, and the pair algebra over two such
/// fans is unbuilt. The other vertex is in practice an apex too: a flat
/// corner at the apex has an edge through it, which refuses
/// [`BooleanError::CrossingAtConeApex`] before the pair is read
/// (`work/splitdoor/a-cone-with-its-apex-against-its-copy-refuses-normal-at-cone-apex.md`).
pub(super) fn apex_pair_refusal<T: geom_core::Real>(
    pair: [(Operand, &[BoolSector<T>]); 2],
) -> Result<(), BooleanError> {
    for (operand, sectors) in pair {
        if let Some(s) = sectors.iter().find(|s| s.apex().is_some()) {
            return Err(BooleanError::VertexPairAtConeApex {
                operand,
                face: s.face,
            });
        }
    }
    Ok(())
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

/// **A bound's side code against a face's tangent plane at the vertex**,
/// at first order: [`crate::bound_side::read`], charged for the face's
/// curvature at `lever` (a plane's [`NO_CURVATURE`]), a first-order tie
/// read [`SideCode::On`]. The sector passes that call this key their
/// coincidence and tangent machinery on the first-order tie
/// (`pair_codes`, recl's flank readings);
/// the vertex-on-face pass reads the tie one order down
/// ([`pierce_side`]).
///
/// A bisector's tie is not read as a verdict where it could decide
/// anything: a bisector's code only relays its sector's side between two
/// twins of ONE physical sector (the fan moves no edge across it), so
/// with mixed neighbours its code changes no topology, and with both
/// neighbours definitely on one side its reading is definite by the
/// argument at `vtxfac`'s on-edge resolution and
/// `recl::resolve_bisector_graze`, which refuse where it is not.
///
/// A charge that does not clear the band is a typed refusal, never a
/// first-order guess.
pub(super) fn side_code<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    face_normal: OutwardNormal<T>,
    lever: T,
    band: Band,
) -> Result<SideCode, BooleanError> {
    let face = FaceDatum::of_face(face_normal, FaceShape::Plane, lever);
    Ok(
        match crate::bound_side::read(dir, reach, face, band)
            .map_err(|fault| bound_refusal(fault, band))?
        {
            FirstOrder::Off(side) => code_of(side),
            FirstOrder::Tied(_) => SideCode::On,
        },
    )
}

/// **A bound's side of a pierced face, as the vertex-on-face pass reads
/// it** ([`super::vtxfac`]): [`crate::bound_side::read_side`], a first-order
/// tie descended ([`crate::bound_side::Tie::descend`]): a conic against a
/// plane face by its bend, a bound that rides the face, or one against a
/// curved face (which the descent does not read), `On`. `tied` says the
/// first order tied, which is what the pass's whole-sector lump keys
/// on: a sector tangent to the face lumps whole, and its second order is
/// the lump's, read off the two carriers.
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
/// a bend was read for.
pub(super) fn pierce_side_reported<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    face_normal: OutwardNormal<T>,
    shape: FaceShape,
    lever: T,
    band: Band,
) -> Result<(PierceSide, Option<OnReading>), BooleanError> {
    let face = FaceDatum::of_face(face_normal, shape, lever);
    let refused = |fault| bound_refusal(fault, band);
    match crate::bound_side::read(dir, reach, face, band).map_err(refused)? {
        FirstOrder::Off(side) => Ok((
            PierceSide {
                code: code_of(side),
                tied: false,
            },
            None,
        )),
        FirstOrder::Tied(tie) => {
            let on = (!tie.bends()).then_some(OnReading {
                predicate: tie.predicate,
                decided: tie.decided,
            });
            let code = match tie.descend(band).map_err(refused)? {
                BoundSide::Off(side) | BoundSide::Tangent(side) => code_of(side),
                BoundSide::Rides | BoundSide::Unread => SideCode::On,
            };
            Ok((PierceSide { code, tied: true }, on))
        }
    }
}

/// A side of a face as a sector code: into the material (against the
/// outward normal) `In`, out of it `Out`.
fn code_of(side: Side) -> SideCode {
    match side {
        Side::Below => SideCode::In,
        Side::Above => SideCode::Out,
    }
}

/// The boolean's refusal of a bound's side the reading did not decide.
fn bound_refusal<T: Decide>(fault: BoundSideFault<T>, band: Band) -> BooleanError {
    match fault {
        BoundSideFault::Chord(diag) => {
            BooleanError::coincidence(Coincide::SectorSide, DeclarationRead::Moot, diag)
        }
        BoundSideFault::Departure {
            escalation,
            arm,
            departure,
        } => BooleanError::of_lever(
            LeverArm::SectorSide,
            DeclarationRead::Moot,
            at_departure(escalation, arm, departure, band),
        ),
        BoundSideFault::Charge(verdict) => BooleanError::CurvedSectorSideUnsupported { verdict },
        BoundSideFault::ChargeEscalated(diag) => BooleanError::Escalated {
            decision: BooleanDecision::PierceCurvature,
            diag,
        },
        BoundSideFault::Bend(diag) => BooleanError::Escalated {
            decision: BooleanDecision::ConicBend,
            diag,
        },
    }
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
    match geom_brep::parting_side(
        n_ref * (graph_accel(sector_surface) - graph_accel(other_surface)),
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

/// A unit direction with its **lean**: `|n_a × n_b| = sin φ` for two
/// faces' line (a germ the faces' normals cross into), `1` for an edge,
/// a bound, a tangent locus or any direction read off one carrier.
/// Every reading of it against another direction levers by the lean
/// ([`leaned_lever`]); read against a sector's bounds ([`rays_within`])
/// the same lean scales the direction itself, its [`Leaned::line`].
#[derive(Clone, Copy, Debug)]
pub(super) struct Leaned<T: geom_core::Real> {
    /// The unit direction.
    pub dir: Vec3<T>,
    /// Its lean, in `(0, 1]`.
    pub lean: T,
}

impl<T: geom_core::Real> Leaned<T> {
    /// A direction read off one carrier: lean 1.
    pub(super) fn upright(dir: Vec3<T>) -> Self {
        Self {
            dir,
            lean: T::from_f64(1.0),
        }
    }

    /// Two faces' line from its un-normalized cross `m = n_a × n_b`.
    pub(super) fn of_line(m: Vec3<T>) -> Self {
        Self {
            dir: m.normalize(),
            lean: m.norm(),
        }
    }

    /// The direction scaled by its lean: two faces' line un-normalized.
    pub(super) fn line(self) -> Vec3<T> {
        self.dir * self.lean
    }

    /// The opposite ray, its lean unchanged.
    pub(super) fn reversed(self) -> Self {
        Self {
            dir: -self.dir,
            lean: self.lean,
        }
    }
}

/// **The lever a reading of two directions' sides carries, given their
/// leans**: `lever · min(lean₀, lean₁)` ([`Leaned`]).
///
/// A unit face line is steeper than its faces' own tilt: a tilt `δ` of
/// either face, a displacement `δ·L ≤ zero` at the reach `L` it governs,
/// turns their line in the reading's plane by up to `δ / sin φ`. A side
/// read at `lever` therefore carries noise up to
/// `lever · (δ₀ / s₀ + δ₁ / s₁)`, `sᵢ` each direction's lean (an edge's
/// `δ` its own departure, its lean 1). Levered at `lever · min(s₀, s₁)`
/// that noise is at most `lever · (δ₀ + δ₁)`: the two directions' own
/// tolerances, whatever the faces' angle. One direction tilted within
/// the zero therefore decides as the upright pose; two tilted the same
/// way can add past it, as they do between two edges. Against an edge
/// lying in the reading's plane, the reading is the edge's far end's
/// offset from the face line's other face; an edge off that plane adds
/// its own departure, foreshortened by `cos φ` ([`rays_within`]).
pub(super) fn leaned_lever<T: geom_core::Real>(lever: T, (l0, l1): (T, T)) -> T {
    lever * l0.min(l1)
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
    let c1 = Margin::levered(
        s.start.cross(dir).dot(s.tangent().vec()),
        lever(s.start_reach),
    );
    let c2 = Margin::levered(dir.cross(s.end).dot(s.tangent().vec()), lever(s.end_reach));
    let escalate = |diag| BooleanError::coincidence(Coincide::Sectors, read, diag);
    Ok((
        decide("bool_sector_within", c1, band).map_err(escalate)?,
        decide("bool_sector_within", c2, band).map_err(escalate)?,
    ))
}

/// **Which rays of `line` lie within every one of `sectors`**:
/// `(+line, −line)`, each `true` where [`within`] holds it in all.
///
/// The line is read scaled by its lean ([`Leaned::line`]): two faces'
/// line is read un-normalized, `m = n_a × n_b`, the bounds' reaches its
/// lever as [`leaned_lever`] would give them. Read against a bound `e`
/// of `a`'s sector at `e`'s reach `L`, it measures
/// `L·(m × e)·n_a = L·((n_a·e) cos φ − n_b·e)`: the bound's own
/// departure from `a`'s plane, foreshortened by `cos φ`, less its far
/// end's offset from `b`'s plane. The two terms can add or cancel; the
/// first is within the zero wherever `a` holds its bound. For a straight
/// bound the second is what its side code reads at the chord's far point
/// ([`side_code`]); a curved bound's code reads the edge where this
/// reads its tangent. The unit line would divide the whole by
/// `|m| = sin φ`, so a far end within the zero of both planes would read
/// off their line wherever the two faces are near parallel.
pub(super) fn rays_within<T: Decide>(
    line: Leaned<T>,
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
    let m = line.line();
    Ok((holds(m)?, holds(-m)?))
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

/// **Whether an edge leaving the vertex along `dir`, coplanar with `s`,
/// runs into its face**: within the sector and along neither of its
/// bounds that is an edge of the face, or, along one of them, bending
/// into the face off it one order down: along a bound, the edge enters
/// the face where it stands past the bound toward `s`
/// ([`geom_brep::parting`], decided at the shorter reach). A bound or an edge
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
        let entered = geom_brep::parting(k - kb, u, l.min(lb));
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
        if !direction_sense(sa.tangent().vec(), sb.tangent().vec(), arm, band)?
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
/// toward `û` ([`geom_brep::parting`]). A line edge bends by zero, so two line
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
/// leading term and not a bound: `crate::bound_side::read`'s sagitta drops the ½
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
            let opened = geom_brep::parting(kb - ka, u, l);
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
            code(sa.start, sa.start_reach, sb.tangent())?,
            code(sa.end, sa.end_reach, sb.tangent())?,
        ),
        (
            code(sb.start, sb.start_reach, sa.tangent())?,
            code(sb.end, sb.end_reach, sa.tangent())?,
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
            let int = sa.tangent().vec().cross(sb.tangent().vec());
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
                let line = Leaned::of_line(int);
                let (plus, minus) = rays_within(line, &[sa, sb], DeclarationRead::Moot, band)?;
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

/// **A vertex's sectors as the corners of a cone at its point**
/// ([`crate::link::Fan`]): each orbit corner one fan about `apex`, its
/// rays its bounds and any subdivision bisectors, each fixed at its
/// reach ([`Reach::point`]). [`build_sectors`] pushes a corner's pieces
/// one after another, the end-sharing piece first, so the fan's rays
/// run from the last piece's start to the first piece's end. Nothing is
/// decided here: the subdivision is [`build_sectors`]'.
pub(crate) fn fans<T: geom_core::Real>(
    sectors: &[BoolSector<T>],
    apex: Point3<T>,
) -> Vec<crate::link::Fan<T>> {
    corners(sectors)
        .map(|corner| {
            let mut rays = Vec::with_capacity(corner.len() + 1);
            for (k, s) in corner.iter().rev().enumerate() {
                if k == 0 {
                    rays.push(s.start_reach.point(apex, s.start));
                }
                rays.push(s.end_reach.point(apex, s.end));
            }
            crate::link::Fan {
                outward: corner[0].tangent().vec(),
                apex,
                rays,
            }
        })
        .collect()
}

/// **How a face of a solid's sectors at a point stands about a line
/// through it** ([`line_fans`]): a sector whose start bound lies along
/// the line, one whose end bound does, or one the line runs inside.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LineSheet {
    Start(usize),
    End(usize),
    Across(usize),
}

/// **A solid's faces holding a line through `apex`, as the half-planes
/// they span about it**: the line cone's corners
/// ([`crate::link::Cone::gathered`]), each tagged with the sector it is
/// read from. A sector with a bound along the line is the half-plane on
/// its far bound's side: rays the line's bound, the far bound, and the
/// line's opposite ray, in the orbit's turn — the line first where it
/// is the sector's start bound, last where it is its end. A sector the
/// line runs inside is its plane's two halves, its start bound's and its
/// end bound's, parted by the line at `line` and its opposite ray.
/// Every ray is fixed at its own bound's reach, the opposite ray at the
/// line's. Nothing is decided here: which bound lies along the line is
/// the sheet's ([`LineSheet`]), and the orbit's turn is the sectors'
/// order. Each wedge is under half a turn, as a [`crate::link::Sector`]'s
/// readers take it, because each sector is ([`build_sectors`] subdivides
/// a reflex corner at its bisector): its far bound lies strictly on one
/// side of the line, so the line's ray and its opposite each part from
/// it by less than half a turn.
pub(crate) fn line_fans<T: geom_core::Real>(
    sectors: &[BoolSector<T>],
    sheets: &[LineSheet],
    apex: Point3<T>,
    line: Point3<T>,
) -> Vec<(usize, crate::link::Fan<T>)> {
    let back = |l: Point3<T>| apex - (l - apex);
    let fan = |s: &BoolSector<T>, rays: Vec<Point3<T>>| crate::link::Fan {
        outward: s.tangent().vec(),
        apex,
        rays,
    };
    let mut out = Vec::with_capacity(sheets.len() + 1);
    for &sheet in sheets {
        match sheet {
            LineSheet::Start(i) => {
                let s = &sectors[i];
                let l = s.start_reach.point(apex, s.start);
                out.push((i, fan(s, vec![l, s.end_reach.point(apex, s.end), back(l)])));
            }
            LineSheet::End(i) => {
                let s = &sectors[i];
                let l = s.end_reach.point(apex, s.end);
                out.push((
                    i,
                    fan(s, vec![back(l), s.start_reach.point(apex, s.start), l]),
                ));
            }
            LineSheet::Across(i) => {
                let s = &sectors[i];
                let (start, end) = (
                    s.start_reach.point(apex, s.start),
                    s.end_reach.point(apex, s.end),
                );
                out.push((i, fan(s, vec![back(line), start, line])));
                out.push((i, fan(s, vec![line, end, back(line)])));
            }
        }
    }
    out
}

/// Each corner's face, in [`fans`]' order.
pub(crate) fn fan_faces<T: geom_core::Real>(
    sectors: &[BoolSector<T>],
) -> impl Iterator<Item = FaceKey> + '_ {
    corners(sectors).map(|corner| corner[0].face)
}

/// A vertex's sectors by orbit corner, each one face's pieces.
fn corners<T: geom_core::Real>(
    sectors: &[BoolSector<T>],
) -> impl Iterator<Item = &[BoolSector<T>]> {
    sectors.chunk_by(|a, b| a.he == b.he)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    use crate::contact::GlueClass;
    use geom_brep::ConicBend;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// **A sector at a cone's apex has its middle on the cone**: the
    /// ruling halfway round between its bounds, not their sum, which
    /// points inside the cone, and for a sector more than a half-turn
    /// wide away from it. Each sector of a 45° cone's apex, from the
    /// ruling at azimuth zero round a quarter, a half less a little, and
    /// three quarters of a turn: the middle is unit, on the cone, and
    /// halfway round from the start.
    #[test]
    fn an_apex_sectors_middle_is_the_ruling_halfway_round() {
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let o = Point3::new(0.0, 1.0, 0.0);
        let h = core::f64::consts::FRAC_PI_4;
        let apex = ConeApex {
            apex: o,
            axis: Vec3::new(0.0, 1.0, 0.0),
            half_angle: h,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            sense: true,
        };
        let start = Vec3::new(h.sin(), -h.cos(), 0.0);
        for turn in [0.25, 0.45, 0.75] {
            let azimuth = turn * core::f64::consts::TAU;
            let end = apex.ruling_from(start, azimuth);
            let s = BoolSector {
                he: HalfEdgeKey::from(key(1)),
                start,
                end,
                start_reach: Reach::Bisector(1.0),
                end_reach: Reach::Bisector(1.0),
                face: FaceKey::from(key(2)),
                at_vertex: BoolFace::Apex(apex),
                arm: 1.0,
            };
            let m = s.middle();
            assert!(
                (m.norm() - 1.0).abs() < 1e-12,
                "turn {turn}: |{m:?}| is not 1"
            );
            assert!(
                (m.dot(apex.axis) + h.cos()).abs() < 1e-12,
                "turn {turn}: {m:?} is off the cone's lower nappe"
            );
            assert!(
                (apex.azimuth_from(start, m) - azimuth / 2.0).abs() < 1e-12,
                "turn {turn}: {m:?} is not halfway round"
            );
        }
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
            at_vertex: BoolFace::Tangent(OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true)),
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
            at_vertex: BoolFace::Tangent(OutwardNormal::from_chart(Vec3::from_array(normal), true)),
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
        let at_the_arm = Margin::levered(floor.start.cross(line).dot(floor.tangent().vec()), arm);
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
        let unit = ta.tangent().vec().cross(tb.tangent().vec()).normalize();
        let steep = unit.cross(ta.end).dot(ta.tangent().vec()) * 1.23;
        assert!(
            steep > b.zero(),
            "the unit line reads {steep:e} off the long bound, past the zero {:e}",
            b.zero()
        );
        let rays = |a: &BoolSector<f64>, b_: &BoolSector<f64>| {
            let m = a.tangent().vec().cross(b_.tangent().vec());
            rays_within(Leaned::of_line(m), &[a, b_], DeclarationRead::Moot, b).unwrap()
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
            let code = plane_side_code(sa.start, sa.start_reach, sb.tangent(), b);
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

    /// **The split walk and the boolean read one edge's side of a plane
    /// alike**, each by its own path on one body: the split's
    /// [`crate::splitting::entry_side`] (the walk's departure read off the
    /// body) and the vertex-on-face pass's [`pierce_side`] of the bound
    /// [`build_sectors`] built. The rim is the oblique wall's closed
    /// ellipse ([`crate::test_support_fixtures::oblique_rim_wall`]), its
    /// two departures along `±y` at the vertex `(1, 0, tan φ)`. Planes
    /// through the vertex square to `y` tie at first order and are read
    /// by the side the rim bends to; tilted ones decide at first order.
    #[test]
    fn the_split_walk_and_the_boolean_read_one_edges_side_alike() {
        let b = band();
        let (body, _, vertex) = crate::test_support_fixtures::oblique_rim_wall(0.3);
        let sectors = build_sectors(
            &body,
            Operand::A,
            vertex,
            &crate::sector_shape::SlitTips::default(),
            ApexPlane::None,
            b,
        )
        .unwrap();
        let orbit = body.vertex_orbit_linked(vertex);
        let (mut read, mut descended) = (0, 0);
        for a in (0..12).map(|k| f64::from(k) * core::f64::consts::TAU / 12.0 + 0.05) {
            for lift in [0.0, 0.3, -0.3] {
                let n = Vec3::new(a.cos(), lift, a.sin()).normalize();
                for &he in &orbit {
                    let (dir, split) = crate::splitting::entry_side(
                        &body,
                        vertex,
                        he,
                        geom_brep::ReferenceNormal::of_split_plane(n),
                        |_| Err::<crate::splitting::PlaneSide, ()>(()),
                        b,
                    )
                    .unwrap();
                    let (bound, reach) = sectors
                        .iter()
                        .flat_map(|s| [(s.start, s.start_reach), (s.end, s.end_reach)])
                        .find(|(d, r)| {
                            matches!(r, Reach::Conic { .. })
                                && d.normalize().dot(dir.normalize()) > 1.0 - 1e-12
                        })
                        .expect("a sector bound along the walk's departure");
                    let boolean = pierce_side(
                        bound,
                        reach,
                        OutwardNormal::from_chart(n, true),
                        FaceShape::Plane,
                        NO_CURVATURE(),
                        b,
                    )
                    .unwrap();
                    let want = match split {
                        crate::splitting::PlaneSide::Below => SideCode::In,
                        crate::splitting::PlaneSide::Above => SideCode::Out,
                        crate::splitting::PlaneSide::On => SideCode::On,
                    };
                    assert_eq!(
                        boolean.code, want,
                        "n = {n:?}, departure {dir:?}: the boolean reads {boolean:?}, \
                         the split {split:?}"
                    );
                    read += 1;
                    if boolean.tied && boolean.code != SideCode::On {
                        descended += 1;
                    }
                }
            }
        }
        assert_eq!(read, 72, "every plane read at both departures");
        assert!(
            descended >= 8,
            "the planes square to the departures tie and are read one order \
             down ({descended} were)"
        );
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
            at_vertex: BoolFace::Tangent(OutwardNormal::from_chart(
                -va.cross(vb).normalize(),
                true,
            )),
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
                                (n.tangent() - want).norm() < 1e-12,
                                "sense {sense} at {p:?}: {:?}, want {want:?}",
                                n.tangent()
                            );
                        }
                        _ => {}
                    }
                }
            }
            assert_eq!(read, 4, "sense {sense}: the wall's four corners");
        }
    }

    /// **A sector based at a cone's apex reads the apex.** A prism's
    /// side face is carried by a cone whose apex is one of the face's
    /// corners: the walk reads the sector there as the cone's rulings
    /// ([`VertexFace::Apex`]), naming the face, where the gradient is
    /// `0/0`; the face's next corner, off the axis, reads its normal.
    /// Without the check the poisoned normal reaches the corner rung,
    /// which escalated it as `Corner(Straight)` on a plane through a full
    /// cone's apex.
    #[test]
    fn a_cone_sector_at_the_apex_reads_the_apex() {
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
        let got = sector_face(&body, Operand::A, apex_vertex, apex_he, b);
        assert!(
            matches!(got, Ok((f, BoolFace::Apex(at))) if f == face && at.apex.distance(apex) == 0.0),
            "the apex corner: {got:?}"
        );
        let (f, n) = sector_face(&body, Operand::A, off_vertex, off_he, b)
            .expect("a corner off the axis reads its normal");
        assert!(f == face && n.tangent().norm().is_finite(), "{n:?}");
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

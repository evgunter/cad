//! `offset_charts_together` — the SIMULTANEOUS offset door for bodies
//! whose junction corners mix PLANES with cylinder, cone and sphere
//! walls.
//!
//! # Why the planar door could not simply be widened
//!
//! [`crate::offset_planes_together`] solves a corner as `nᵢ·x = cᵢ`
//! over the distinct moved planes meeting it: three equations, three
//! unknowns, Cramer. **Measured on this unit's own corpus, a curved
//! junction corner does not have three equations.** A full revolve's
//! rim vertex is incident to exactly TWO distinct surfaces — the wall
//! and the cap it meets — and two surfaces determine a CURVE, not a
//! point. What pins the vertex on that curve is the revolve's own seam,
//! whose azimuth is conventional data (D2) carried from the operand,
//! exactly as the planar door carries a line's `t = 0` anchor.
//!
//! So this is not "the planar solve with more surface kinds". It is a
//! different reduction, and it exists in this shape because the corpus
//! was measured before it was written:
//!
//! ```text
//! sphere-zone vase   4 corners  2 [plane ∩ sphere]     + 2 axis poles
//! cone frustum       4 corners  2 [plane ∩ cone]       + 2 axis poles
//! drum               4 corners  2 [plane ∩ cylinder]   + 2 axis poles
//! bellied pot        2 corners  2 [plane ∩ sphere]
//!                    2 corners  2 [cylinder ∩ sphere]
//!                    2 corners  2 [plane ∩ cylinder]   + 2 axis poles
//! partial wedge      4 corners  3 [cylinder ∩ plane ∩ plane]
//!                    2 corners  3 [plane ∩ plane ∩ plane]
//! sphere lune        2 corners  3 [sphere ∩ meridian ∩ meridian]
//! klein elbow        4 corners  2 [torus ∩ meridian]
//! full torus         2 corners  1 [torus seam ∩ torus seam]
//! ```
//!
//! There is no `plane ∩ curved ∩ curved` corner anywhere in it, and no
//! corner with two planes CONTAINING the axis and a curved wall, so
//! neither is built: both refuse typed
//! ([`ReplaceFaceError::TogetherAxialCorner`]) rather than being
//! written on the presumption that something will need them.
//!
//! # The reduction
//!
//! Every surface this door accepts is a surface of revolution about ONE
//! axis `(o, a)`, or a plane normal to it, or a plane containing it. A
//! point `p` is read in axial coordinates
//!
//! ```text
//! h = (p − o)·a          the station along the axis
//! ρ = |p − o − a·h|      the distance from it
//! e = (p − o − a·h)/ρ    the azimuth direction (undefined at ρ = 0)
//! ```
//!
//! and in the `(ρ, h)` half-plane every accepted surface is a LINE or a
//! CIRCLE:
//!
//! | surface | in `(ρ, h)` |
//! |---|---|
//! | plane normal to `a` | the line `h = h₀` |
//! | cylinder | the line `ρ = r` |
//! | cone | the generator line through `(0, h_apex)` |
//! | sphere centred on `a` | the circle `ρ² + (h − h_c)² = R²` |
//! | torus coaxial with `a` | the meridian circle `(ρ − R)² + (h − h_c)² = r²` |
//! | plane containing `a` | not a profile constraint at all — it fixes the AZIMUTH |
//!
//! A corner is therefore solved in two independent steps, both closed
//! form, neither of them marching:
//!
//! 1. **the profile** — the first well-conditioned PAIR of profile
//!    constraints, solved as line∩line or line∩circle, with every
//!    further profile constraint VERIFIED against the answer. A corner
//!    with ONE profile circle is answered too, from the fact the wedge
//!    fixture taught this module: a partial revolve's meridian caps
//!    stop containing the axis when offset, so the corner they meet is
//!    displaced off it. TWO moved caps meet in a line parallel to the
//!    axis, which is the derived wall constraint `ρ = ρ_L` and the
//!    same line∩circle solve; ONE moved cap leaves the profile angle
//!    free, and it is CARRIED — the old corner's point moved
//!    concentrically with its circle, the same conventional datum the
//!    sphere-seam mint below trusts. A corner ALL of whose faces lie
//!    on ONE surface of revolution — a full torus's seam vertex, where
//!    the tube's two half-circle walls meet along their meridian seams
//!    and share their two equators — is a point OF that surface, and
//!    the offset of a surface moves each of its points along its own
//!    normal: so the corner is the old point's image under the one
//!    profile's own offset, the concentric move on a circle and the
//!    perpendicular foot on a line, with the azimuth carried as every
//!    seam's is;
//! 2. **the azimuth** — carried from the old vertex when no plane
//!    contains the axis at this corner (the seam's own conventional
//!    datum), solved as circle∩plane when exactly one does, or read
//!    off the moved caps' meeting line when a circle-profile corner
//!    stands on both.
//!
//! **The offsets themselves are [`geom_brep::offset_surface`]'s** — the
//! same analytic mint the per-chart door uses. One derivation, not a
//! second copy that could drift from it.
//!
//! # Every edge, and why its carrier is not routed
//!
//! A carrier here is NOT derived by routing its surface pair through
//! the C5 table. It is the OLD carrier's kind and conventional frame
//! with its position re-solved — a line keeps its direction and moves
//! perpendicular to itself, a latitude circle (a rim between two
//! charts, or a full tube's equator seam) keeps its normal and
//! `u_ref` and takes the corner's own station and radius, a sphere
//! seam's great circle keeps its centre and plane and takes the
//! chart's radius, a sphere-wall rim keeps the cap plane's normal and
//! `u_ref` and takes the moved cap's own plane∩sphere section — and
//! then it is **verified**: both endpoints are
//! read onto it and metered, and its midpoint is metered against BOTH
//! moved surfaces. The parameters are always re-read, because a
//! corner's motion slides an endpoint ALONG its own edge as readily as
//! it moves the edge.
//!
//! # Which refusals have a row, and what builds their operand
//!
//! A `what:` string with a row is one a test reaches; whether a DOOR
//! builds the operand that reaches it is a separate fact, and the two
//! lists keep them apart. Everything else is written for correctness
//! and said so at the end.
//!
//! **A door builds the operand, and `shell` reaches the refusal:** the
//! tangency arm of [`ReplaceFaceError::TogetherAxialCorner`] (the
//! bullet); the meridian-pair arm's parallel-caps refusal (the
//! half-turn lune) and its tangent-or-miss refusal (the narrow 20°
//! lune, whose moved caps' meeting line stands `t/sin 10° ≈ 0.288`
//! from the axis, past the shrunk circle's `r − t = 0.25`) — both
//! `torax_axial`; `TogetherNotAxial`'s oblique-plane arm;
//! `TogetherEdgeDisagreement` (`sf2b_r1_probes`, `sf2b_r2_probes`,
//! and `shell7_seam_corner`'s three-quarter-turn cone frustum); the
//! latitude posture's off-axis-centre refusal (the klein elbow's
//! spiric rim, `torax_axial`, `verbs_shell`).
//!
//! **A hand-made operand, or the door called directly:** the
//! no-profile-constraint refusal (a wedge's axis edge split by
//! `Body::split_edge`, `shell7_seam_corner`); the line-beside-a-
//! meridian refusal (a wedge's wall/cap generator split the same way,
//! `shell7_seam_corner`); the partial-set and chart-mixed gates and the
//! sphere lune's whole rim solve (`offset_charts_together` called
//! directly, `torax_axial` — `shell`'s closing tier 3 needs a volume
//! the sphere flux arm's `props_band_coplanar` premise cannot yet give
//! a lune).
//!
//! **The carried arms themselves have door-built rows**: a full tube's
//! seam vertex (torus circle), a drum's collinear wall vertex
//! (cylinder line), a frustum's collinear generator vertex (cone
//! line), a cap's collinear vertex (station line) and a two-arc
//! sphere's cocircular vertex (sphere circle) all shell through
//! `shell7_seam_corner`. **No row at all**, written for correctness:
//! the circle-beside-two-caps-still-on-the-axis refusal, the axis-pole
//! station arm and its off-axis-circle arm (a torus meridian cannot
//! contain a pole, `R − r > 0` keeps it clear), the seam arms'
//! refusing sides, the off-axis rim mint's four refusing predicates,
//! the re-author's out-of-plane refusal, and the over-determined-
//! azimuth arm — no constructible body here has more than one plane
//! through the axis at a corner that is not also all-planar.
//!
//! # What this door does not do
//!
//! - **No marching, no SSI, no crossing-pipeline entry.** Every solve
//!   above is a quadratic at worst.
//! - **It does not widen the C5 table** and does not call it. A body
//!   whose surfaces are not all coaxial about one axis — or whose kind
//!   the gate does not know — never reaches here and keeps the refusal
//!   it had. That includes a `plane × torus` rim on a PARTIAL revolve,
//!   whose moved cap has stopped containing the axis and whose section
//!   is a quartic; a FULL revolve's torus rim is a latitude circle and
//!   is this door's, without the table being asked.
//! - **It does not touch global clearance.** `shell`'s wall-clearance
//!   gate is the operand's and is unchanged; this door decides corners.
//!   A sliver WEDGE whose two moved meridian planes cross outside the
//!   shrunk wall has no cavity at all, and every one of its rim corners
//!   still solves locally — each meets only ONE meridian plane — so no
//!   meter here can see it. What catches it is the tier gate on the
//!   assembled body (`IntervalNotForward`), measured on
//!   `sf2b_r1_probes::r1p2`. That is a NET rather than a door-named
//!   refusal, and a meter that named the door would be better; it is
//!   future work, not a debt this unit is carrying, because the net is
//!   loud and no wrong body passes it.
//!
//! # Conditioning
//!
//! As in the planar door, a dimensionless quantity's lever is the
//! geometry being judged and never the request: the profile solve's
//! `|det|` — a sine between two unit 2-D normals — is levered by the
//! ARC LENGTHS of the corner's own incident edges, and the azimuth
//! solve's two roots
//! are separated by a LENGTH that is metered as one. A corner asked to
//! move nothing is answered before any meter runs. This is also what
//! refuses a TANGENT junction: a wall meeting a sphere with no angle
//! between them has no transversal corner to solve, and the meter says
//! so in the geometry's own terms rather than by a special case.

use geom::{Curve3, Surface};
use geom_brep::{EdgeAuthority, EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec, SurfaceKind};
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Tol, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::euler::FaceSurface;
use crate::geometry::SurfaceKey;
use crate::offset_together::ChartMove;
use crate::props::PropsQuadLane;
use crate::replace_face::ReplaceFaceError;

/// The revolution axis every accepted surface shares, with the scope's
/// own radial extent — the length that levers every direction test
/// here, so a verdict about alignment is a statement about the geometry
/// being judged.
#[derive(Clone, Copy)]
struct Frame<T: Real> {
    origin: Point3<T>,
    dir: Vec3<T>,
    extent: T,
}

impl<T: Real> Frame<T> {
    /// `p`'s station along the axis.
    fn station(&self, p: Point3<T>) -> T {
        (p - self.origin).dot(self.dir)
    }

    /// `p`'s radial offset vector from the axis.
    fn radial(&self, p: Point3<T>) -> Vec3<T> {
        let v = p - self.origin;
        v - self.dir * v.dot(self.dir)
    }

    /// `p` rebuilt from axial coordinates and an azimuth direction.
    fn place(&self, rho: T, h: T, e: Vec3<T>) -> Point3<T> {
        self.origin + self.dir * h + e * rho
    }
}

/// One moved chart, resolved once and read many times.
struct MovedChart<T: Real> {
    old_key: SurfaceKey,
    old: Surface<T>,
    /// The surface after [`geom_brep::offset_surface`].
    new: Surface<T>,
    /// The signed offset along the chart's stored outward direction —
    /// the caller's number.
    distance: T,
    /// The chart's constraint on a corner, in axial terms.
    constraint: Constraint<T>,
    /// The rigid displacement the chart underwent, when its offset IS a
    /// rigid translation — `None` when it is not.
    rigid: Option<Vec3<T>>,
}

/// A moved chart's constraint on a corner, in the axial frame.
///
/// The first four are PROFILE constraints — curves in the `(ρ, h)`
/// half-plane. The last is not: a plane containing the axis says
/// nothing about `(ρ, h)` and everything about the azimuth.
#[derive(Clone, Copy)]
enum Constraint<T: Real> {
    /// A plane normal to the axis, at station `h`.
    Station(T),
    /// A cylinder of radius `r`.
    Wall(T),
    /// A cone: `ρ·cos α = side·(h − h_apex)·sin α`.
    Generator { h_apex: T, sin_a: T, cos_a: T },
    /// A sphere centred on the axis at station `h_c`.
    Ball { h_c: T, r: T },
    /// A torus coaxial with the body: its meridian is the circle of
    /// radius `minor` centred `(major, h_c)` in the `(ρ, h)`
    /// half-plane. `major` is the only profile centre here that is not
    /// on the axis, and `major > minor > 0` is the standing
    /// construction invariant, netted upstream — no arm here re-decides
    /// it.
    Torus { major: T, h_c: T, minor: T },
    /// A plane CONTAINING the axis: `m̂·x = c`.
    Meridian { m: Vec3<T>, c: T },
}

/// A profile constraint as a line `n̂·(ρ, h) = c`, or a circle centred
/// `(ρ_c, h_c)` of radius `r`.
///
/// **The circle's centre carries a ρ, and every arithmetic site here
/// reads it.** A sphere's meridian is centred ON the axis and a torus's
/// is not — that one number is the whole difference between the two
/// kinds in this half-plane, so it lives in the datum rather than in a
/// fork per kind.
#[derive(Clone, Copy)]
enum Profile<T: Real> {
    Line { n: (T, T), c: T },
    Circle { rho_c: T, h_c: T, r: T },
}

impl<T: Real> Profile<T> {
    /// The signed residual of `(ρ, h)` against this curve, in meters: a
    /// projection onto a UNIT 2-D direction, or a Euclidean norm minus
    /// a radius. Both are lengths without a lever.
    fn residual(&self, rho: T, h: T) -> T {
        match *self {
            Self::Line { n, c } => n.0 * rho + n.1 * h - c,
            Self::Circle { rho_c, h_c, r } => Vec3::new(rho - rho_c, h - h_c, T::zero()).norm() - r,
        }
    }
}

impl<T: Decide> Profile<T> {
    /// The image of `(ρ, h)` under this curve's OWN offset — the point
    /// of the MOVED curve the old point's normal reaches: the
    /// perpendicular foot on a line, the concentric point on a circle.
    ///
    /// This is what a corner does when every surface meeting it is one
    /// surface of revolution, and it is ONE arithmetic whichever arm
    /// asks — a seam corner with nothing else at it, or a rim corner
    /// whose one moved cap fixes only the azimuth. `None` is a point
    /// standing at a circle's own centre, which fixes no direction to
    /// move along: decided here, named by the caller in its corner's
    /// own words.
    ///
    /// A point `δ` OFF its own profile is SNAPPED: its image lies on
    /// the moved curve exactly, `δ` from where the true surface point's
    /// image would be, never amplified — so a concurrence meter on the
    /// answer reads zero whatever `δ` was, and it is the edge layer
    /// (the endpoint and midpoint meters) that sees a corner that does
    /// not fit its edges.
    fn image_of(&self, rho: T, h: T, band: Band) -> Result<Option<(T, T)>, Indeterminate> {
        match *self {
            Self::Line { n, c } => {
                let gap = n.0 * rho + n.1 * h - c;
                Ok(Some((rho - gap * n.0, h - gap * n.1)))
            }
            Self::Circle { rho_c, h_c, r } => {
                let v = Vec3::new(rho - rho_c, h - h_c, T::zero());
                let len = v.norm();
                Ok(
                    match decide("offset_axial_datum_arm", Margin::of(len), band)? {
                        Sign::Positive => Some((rho_c + v.x / len * r, h_c + v.y / len * r)),
                        Sign::Zero | Sign::Negative => None,
                    },
                )
            }
        }
    }
}

// ---------------------------------------------------------------------
// The door
// ---------------------------------------------------------------------

/// **Offset every chart of an axial `body` at once** (module docs).
///
/// `moves` names each chart and its signed distance along the chart's
/// stored outward direction. Every face of every SOLID the moves touch
/// must appear exactly once across them, and no solid may be touched in
/// part; a solid the moves do not name is not offset and its geometry
/// is not written.
///
/// **What it READS is not as tight as what it writes**, and the whole
/// account — which two reads are scope-sized, which four are still
/// linear in the body, and what that costs — is [`crate::offset_together::Scope`]'s, stated
/// there once for both doors.
///
/// # Errors
///
/// [`ReplaceFaceError`], the body untouched on every one: the whole
/// plan is decided before anything is written, and the writes go to a
/// clone that replaces `body` only on success.
pub fn offset_charts_together<T: Decide + PropsQuadLane>(
    body: &mut Body<T>,
    moves: &[ChartMove<T>],
    band: Band,
    tol: Tol,
) -> Result<(), ReplaceFaceError<T>> {
    // ---- Decide: the chart moves are well formed. ----
    //
    // The planar door's own two preconditions, for the same reason: a
    // caller's mistake must be named here rather than surfacing
    // downstream as a refusal about something else.
    let mut seen: Vec<FaceKey> = Vec::new();
    for m in moves {
        let Some(&first) = m.faces.first() else {
            return Err(ReplaceFaceError::EmptyGroup);
        };
        let key = body
            .get_face(first)
            .ok_or(ReplaceFaceError::StaleFace { face: first })?
            .surface;
        for &face in &m.faces {
            let data = body
                .get_face(face)
                .ok_or(ReplaceFaceError::StaleFace { face })?;
            if data.surface != key {
                return Err(ReplaceFaceError::TogetherChartMixed { face, other: first });
            }
            if seen.contains(&face) {
                return Err(ReplaceFaceError::TogetherFaceRepeated { face });
            }
            seen.push(face);
        }
    }
    // The scope is the SOLIDS the moves touch, and every face of each
    // of them must be in the set: a corner belongs to one solid, so a
    // solid named in part has corners whose answer depends on faces
    // the door was not told about, while a solid named not at all has
    // no corner this call can disturb.
    let scope = crate::offset_together::scope_of_moves(body, moves)?;
    for (face, _) in body.faces() {
        if scope.holds_face(face) && !seen.contains(&face) {
            return Err(ReplaceFaceError::TogetherPartialSet { face });
        }
    }

    // ---- Decide: the axis, and every chart against it. ----
    //
    // The axis is the SCOPE's, not the body's: a box standing beside a
    // vessel has no axis of its own, and reading the seed or the
    // extent off it would answer a question about the vessel with the
    // box's geometry.
    let frame = axial_frame(body, &scope)?;
    let mut charts: Vec<(FaceKey, MovedChart<T>)> = Vec::new();
    for m in moves {
        // **The cone's mirror nappe is a CONSUMER obligation, and this
        // is where this door discharges it.**
        // [`geom_brep::ConeOffset`]'s action is the pushforward along
        // the continuous extension of the OPENING nappe's normal field
        // — `n₊` does not flip across the apex — so a mirror-nappe
        // face's material moves `−d` along its OWN chart normal. A
        // `ChartMove`'s distance is along the FACE's outward direction,
        // so below the apex the two conventions are opposite and the
        // caller's number is turned over before it reaches the mint.
        // Measured on the cone frustum: unturned, the cavity comes back
        // LARGER than its operand (0.001058 against 0.000895) — a
        // shrink that grew.
        //
        // The nappe is a fact only the face has, decided at its one
        // home; the MOVE names a chart, so every face of it is decided
        // and the answers are agreed before one number is turned for
        // all of them. Only a cone is asked: every other chart has one
        // sheet whose own normal IS the mint's stored field, so the
        // turn is the identity and a corner walk would answer a
        // question the surface already settles.
        let key = body
            .get_face(*m.faces.first().ok_or(ReplaceFaceError::EmptyGroup)?)
            .ok_or(ReplaceFaceError::Corrupt)?
            .surface;
        let d = match body.get_surface(key).ok_or(ReplaceFaceError::Corrupt)? {
            Surface::Cone { .. } => {
                crate::offset_nappe::group_nappe(body, &m.faces, band)?.turn(m.distance)
            }
            _ => m.distance,
        };
        for &face in &m.faces {
            let data = body
                .get_face(face)
                .ok_or(ReplaceFaceError::StaleFace { face })?;
            let old = body
                .get_surface(data.surface)
                .ok_or(ReplaceFaceError::Corrupt)?
                .clone();
            let new = geom_brep::offset_surface(&old, d, band)
                .map_err(|error| ReplaceFaceError::Offset { face, error })?;
            let constraint = classify(face, &old, &new, &frame, band)?;
            charts.push((
                face,
                MovedChart {
                    old_key: data.surface,
                    rigid: rigid_shift(&old, &new),
                    old,
                    new,
                    distance: m.distance,
                    constraint,
                },
            ));
        }
    }
    let chart_of = |face: FaceKey| charts.iter().find(|(k, _)| *k == face).map(|(_, c)| c);

    // ---- Decide: every corner, before anything is written. ----
    let mut moved: Vec<(VertexKey, Point3<T>)> = Vec::new();
    for (vertex, _) in body.vertices() {
        if !scope.holds_vertex(vertex) {
            continue;
        }
        let mut at: Vec<&MovedChart<T>> = Vec::new();
        for face in crate::offset_together::faces_at_vertex(body, vertex)? {
            let c = chart_of(face).ok_or(ReplaceFaceError::Corrupt)?;
            if !at.iter().any(|q| q.old_key == c.old_key) {
                at.push(c);
            }
        }
        let here = body
            .get_vertex(vertex)
            .and_then(|v| body.get_point(v.point).copied())
            .ok_or(ReplaceFaceError::Corrupt)?;
        let arms = corner_arms(body, vertex)?;
        moved.push((
            vertex,
            solve_corner(vertex, here, &at, &arms, &frame, band)?,
        ));
    }
    let point_at = |v: VertexKey| moved.iter().find(|(k, _)| *k == v).map(|(_, p)| *p);

    // ---- Decide: every edge's carrier and description. ----
    let mut specs: Vec<(EdgeKey, EdgeCurveSpec<T>)> = Vec::new();
    for (edge, edge_data) in body.edges() {
        if !scope.holds_edge(edge) {
            continue;
        }
        let (fa, fb) =
            crate::replace_face::edge_faces(body, edge).ok_or(ReplaceFaceError::Corrupt)?;
        let (ca, cb) = (
            chart_of(fa).ok_or(ReplaceFaceError::Corrupt)?,
            chart_of(fb).ok_or(ReplaceFaceError::Corrupt)?,
        );
        let start = body
            .get_half_edge(edge_data.he_plus)
            .ok_or(ReplaceFaceError::Corrupt)?
            .start;
        let end = body
            .half_edge_end(edge_data.he_plus)
            .ok_or(ReplaceFaceError::Corrupt)?;
        let (p_start, p_end) = (
            point_at(start).ok_or(ReplaceFaceError::Corrupt)?,
            point_at(end).ok_or(ReplaceFaceError::Corrupt)?,
        );
        let old_point = |v: VertexKey| {
            body.get_vertex(v)
                .and_then(|d| body.get_point(d.point).copied())
                .ok_or(ReplaceFaceError::Corrupt)
        };
        let (q_start, q_end) = (old_point(start)?, old_point(end)?);
        let curve = body
            .get_curve_geom(edge_data.curve)
            .and_then(crate::null::CurveGeom::certified)
            .ok_or(ReplaceFaceError::Corrupt)?;
        let old_carrier = curve.carrier().clone();
        let (t0_old, t1_old) = curve.params();
        let description = curve.description().clone();
        let authority = curve.authority();

        let carrier = mint_carrier(
            edge,
            &old_carrier,
            (t0_old, p_start),
            (ca, cb),
            &frame,
            band,
        )?;

        // Both endpoints are READ onto the new carrier and metered. Two
        // corner solves agreeing about the edge between them is the
        // claim this door makes about every edge, and it is checked.
        let t0 = param_on(&carrier, &old_carrier, t0_old, q_start, p_start, edge, band)?;
        let t1 = param_on(&carrier, &old_carrier, t1_old, q_end, p_end, edge, band)?;

        // The carrier is VERIFIED onto both moved surfaces at its own
        // midpoint: a re-derived edge's claim is that it lies on the two
        // surfaces it separates.
        let mid = carrier.eval((t0 + t1) * T::from_f64(0.5));
        for c in [ca, cb] {
            let gap = surface_residual(&c.new, mid, &frame);
            match decide("offset_axial_edge_on_surface", Margin::of(gap), band) {
                Ok(Sign::Zero) => {}
                Ok(_) => return Err(ReplaceFaceError::TogetherEdgeDisagreement { edge, gap }),
                Err(source) => return Err(ReplaceFaceError::Escalated { source }),
            }
        }

        specs.push((
            edge,
            EdgeCurveSpec {
                description: restate(
                    description,
                    authority,
                    mid,
                    &carrier,
                    (p_start, p_end),
                    edge,
                    band,
                )?,
                carrier,
                param_start: t0,
                param_end: t1,
            },
        ));
    }

    // ---- Mutation, on a clone (every decision is done). ----
    let mut work = body.clone();
    let mut minted: Vec<(SurfaceKey, SurfaceKey)> = Vec::new();
    for m in moves {
        let Some(&first) = m.faces.first() else {
            return Err(ReplaceFaceError::EmptyGroup);
        };
        let c = chart_of(first).ok_or(ReplaceFaceError::Corrupt)?;
        // **A chart asked to move nothing keeps its chart.** Re-minting
        // it would put a fresh key in the arena describing the same
        // surface — which is not a no-op to anything reading keys, and
        // this door is called with a mixed set (the rim LIFT moves ONE
        // chart of a body whose others must hold still).
        match decide("offset_axial_chart_motion", Margin::of(m.distance), band) {
            Ok(Sign::Zero) => continue,
            Ok(_) => {}
            Err(source) => return Err(ReplaceFaceError::Escalated { source }),
        }
        let new_key = work
            .set_face_surface(first, FaceSurface::New(c.new.clone()))
            .map_err(|error| ReplaceFaceError::Op { edge: None, error })?;
        for &member in &m.faces[1..] {
            work.set_face_surface(member, FaceSurface::Shared(new_key))
                .map_err(|error| ReplaceFaceError::Op { edge: None, error })?;
        }
        minted.push((c.old_key, new_key));
    }
    for (vertex, point) in &moved {
        let old_point = work
            .get_vertex(*vertex)
            .ok_or(ReplaceFaceError::Corrupt)?
            .point;
        let new_point = work.add_point(*point);
        work.get_vertex_mut(*vertex)
            .ok_or(ReplaceFaceError::Corrupt)?
            .point = new_point;
        work.remove_point_if_orphaned(old_point);
    }
    for (edge, mut spec) in specs {
        for (old, new) in &minted {
            spec.description = crate::replace_face::remap_description(spec.description, *old, *new);
        }
        work.set_edge_curve(edge, spec, tol)
            .map_err(|error| ReplaceFaceError::Op {
                edge: Some(edge),
                error,
            })?;
    }
    // Every edge OF THE SCOPE was re-described, and the charts here DO
    // mint pcurve rows (a cylinder, a cone and a sphere all do), so this
    // pass is load-bearing rather than the planar door's inert one. It
    // runs over the scope's faces alone: an out-of-scope row belongs to
    // an edge this door did not touch and stays exactly as it was found.
    let minting = scope.faces_in_scope();
    crate::pcurves::mint_pcurves_of(&mut work, &minting, tol)
        .map_err(|source| ReplaceFaceError::Pcurve { source })?;
    // Tier 2 over the WHOLE clone, deliberately, and one of the four
    // reads that stay linear in the body (`Scope`'s docs carry the
    // account and the reason for each).
    if let Err(errors) = crate::validate::validate_closed(&work) {
        return Err(ReplaceFaceError::ResultNotClosed { errors });
    }
    *body = work;
    Ok(())
}

// ---------------------------------------------------------------------
// The axis gate
// ---------------------------------------------------------------------

/// **Is this a body this door can take?** Structural and cheap: every
/// surface is a plane, cylinder, cone, sphere or TORUS, the curved ones
/// share one axis LINE, and every plane is normal to it or contains it.
///
/// `shell` reads this per SOLID to pick that solid's branch, so a solid
/// outside it keeps exactly the posture it had.
///
/// # Errors
///
/// **An ESCALATION is not a `false`.** The gate's own tests are
/// margined — a normal's misalignment levered by the scope's extent, a
/// centre's distance from the axis — and a margin that lands in the
/// ambiguity band means this body's kinds are not DECIDED either way
/// (D4 ¶3). Answering `false` there would turn "I cannot tell" into a
/// silent branch choice, and the branch it silently chooses is the
/// per-chart door — whose refusal would then name a carrier rather than
/// the undecided geometry that actually stopped it. So the escalation
/// is returned typed and the caller refuses with it. Every other
/// verdict — a NURBS or fitted wall, a skew cylinder, a torus whose own
/// axis is NOT the scope's, an all-planar scope with no axis at all — is
/// a definite `false` and stays one. A COAXIAL torus is no longer one
/// of them: it is inside the roster this door takes, and the table at
/// the top of this module carries its meridian circle.
///
/// **The band is not reachable from any operand this workspace's sweeps
/// build, and that is measured rather than assumed.** Every margin this
/// gate takes is EXACTLY zero on a revolve — the caps' normals, the
/// wall's axis and the frame's direction are minted from one
/// `AxisFrame`, so `n̂ × â` and `m̂ · â` are exact zeros, not small
/// numbers — and `revolve` refuses a 2-D axis that is not `±x`/`±y`
/// outright, so there is no tilted body to feed it either.
/// `sf2b_r1_probes::r1p5_the_axis_gates_third_outcome_is_unreachable_from_the_sweeps`
/// reads eighteen decades of band scale and reports no escalation
/// anywhere, and goes red the day one appears. The escalating arm is
/// therefore written for correctness rather than pinned by a fixture,
/// which is stated here rather than left to be discovered as a gap.
pub fn is_axial<T: Decide>(body: &Body<T>, band: Band) -> Result<bool, ReplaceFaceError<T>> {
    let scope = crate::offset_together::Scope::whole(body).ok_or(ReplaceFaceError::Corrupt)?;
    is_axial_in(body, &scope, band)
}

/// [`is_axial`] over the faces of the solids `scope` names, and nothing
/// else. Axiality is a property of a solid — a box beside a vessel is
/// not a body of revolution and each of the two is one — so the door
/// decision is read per solid and a solid's answer never depends on
/// what stands beside it.
pub(crate) fn is_axial_in<T: Decide>(
    body: &Body<T>,
    scope: &crate::offset_together::Scope,
    band: Band,
) -> Result<bool, ReplaceFaceError<T>> {
    let Ok(frame) = axial_frame(body, scope) else {
        return Ok(false);
    };
    for (face, f) in body.faces() {
        if !scope.holds_face(face) {
            continue;
        }
        let Some(surface) = body.get_surface(f.surface) else {
            return Ok(false);
        };
        match classify(face, surface, surface, &frame, band) {
            Ok(_) => {}
            // The gate's own definite verdicts: this body is not
            // axial, and that is an answer.
            Err(ReplaceFaceError::TogetherAxialUnsupported { .. })
            | Err(ReplaceFaceError::TogetherNotAxial { .. }) => return Ok(false),
            Err(source) => return Err(source),
        }
    }
    Ok(true)
}

/// The revolution axis of the solids `scope` names, and their radial
/// extent, read off the first curved chart in scope. An all-planar
/// scope has no axis and is not this door's.
fn axial_frame<T: Real>(
    body: &Body<T>,
    scope: &crate::offset_together::Scope,
) -> Result<Frame<T>, ReplaceFaceError<T>> {
    let first = body
        .faces()
        .find(|(k, _)| scope.holds_face(*k))
        .map(|(k, _)| k)
        .ok_or(ReplaceFaceError::Corrupt)?;
    let mut seed: Option<(Point3<T>, Vec3<T>)> = None;
    for (face, f) in body.faces() {
        if !scope.holds_face(face) {
            continue;
        }
        let Some(surface) = body.get_surface(f.surface) else {
            continue;
        };
        let found = match surface {
            Surface::Cylinder { origin, axis, .. } => Some((*origin, axis.normalize())),
            Surface::Cone { apex, axis, .. } => Some((*apex, axis.normalize())),
            Surface::Sphere { center, axis, .. } => Some((*center, axis.normalize())),
            // A torus carries `center` + `axis` exactly as a sphere
            // does: the centre is the tube midplane's own point on the
            // axis, and the axis is the revolution axis itself.
            Surface::Torus { center, axis, .. } => Some((*center, axis.normalize())),
            Surface::Plane { .. } => None,
            other => {
                return Err(ReplaceFaceError::TogetherAxialUnsupported {
                    face,
                    kind: SurfaceKind::of(other),
                });
            }
        };
        if found.is_some() {
            seed = found;
            break;
        }
    }
    let (seed_origin, dir) = seed.ok_or(ReplaceFaceError::TogetherAxialUnsupported {
        face: first,
        kind: SurfaceKind::Plane,
    })?;
    // **The axis point is CANONICALIZED to its own foot at the world
    // origin**, not left as whichever chart happened to seed it. A
    // station is then a world coordinate along the axis rather than a
    // difference from an arbitrary point, and rebuilding a corner from
    // it is exact where the arbitrary point's round trip was not: on a
    // vessel whose cylinder stores its origin at the far cap, a cavity
    // station of `0.2` came back `0.19999999999999996` purely from
    // subtracting and re-adding `2.0`. Measured on the byte-dump
    // harness, which now reports the curved fixtures unchanged.
    let origin = seed_origin - dir * (vec_of(seed_origin)).dot(dir);
    // The extent is the SCOPE's own furthest vertex from the axis
    // point: the length a direction error would move a corner by, which
    // is the geometry every alignment verdict here is about. A box
    // standing beside a vessel would lever the vessel's margins by its
    // own distance away, which is a fact about the assembly and not
    // about the vessel's charts.
    //
    // **The scoping of THIS walk is unpinnable, and that is stated
    // rather than left to be discovered.** `extent` feeds only
    // `Margin::levered(x, extent)`, and every margin that reads it is
    // EXACTLY zero on any body of revolution this workspace builds —
    // the caps' normals, the wall's axis and the frame's direction are
    // minted from one `AxisFrame`, so the sines and dot products are
    // exact zeros rather than small numbers. A zero margin decides the
    // same at every lever, so deleting the guard below leaves the whole
    // suite green. What would pin it is an operand whose own alignment
    // margin is NON-zero and small enough that the lever decides the
    // verdict — a tilted revolve, which `revolve` refuses to build (its
    // 2-D axis must be `±x`/`±y`) — or a hand-built body of that shape.
    // The same posture the axis gate's third outcome is documented
    // under: written for correctness rather than pinned by a fixture.
    let mut extent = T::zero();
    for (vertex, v) in body.vertices() {
        if !scope.holds_vertex(vertex) {
            continue;
        }
        if let Some(p) = body.get_point(v.point) {
            extent = extent.max((*p - origin).norm());
        }
    }
    Ok(Frame {
        origin,
        dir,
        extent,
    })
}

/// `moved`'s constraint on a corner, or the typed refusal that says the
/// chart is not a surface of revolution about this axis.
///
/// **The axis tests read `structural`, the operand's own surface; the
/// constraint's numbers read `moved`.** The distinction is load-bearing
/// and was found by measurement: a plane CONTAINING the axis stops
/// containing it the moment it is offset inward, so testing the moved
/// plane would refuse every partial revolve's meridian cap for having
/// done exactly what it was asked to do. What makes a chart axial is a
/// fact about the body; where it now sits is a fact about the offset.
fn classify<T: Decide>(
    face: FaceKey,
    structural: &Surface<T>,
    moved: &Surface<T>,
    frame: &Frame<T>,
    band: Band,
) -> Result<Constraint<T>, ReplaceFaceError<T>> {
    let not_axial = |what: &'static str| ReplaceFaceError::TogetherNotAxial { face, what };
    // A direction's misalignment is a SINE, levered by the body's own
    // extent — the length that misalignment would move a corner by.
    let sine = |x: T, what: &'static str| -> Result<bool, ReplaceFaceError<T>> {
        match decide(what, Margin::levered(x, frame.extent), band) {
            Ok(Sign::Zero) => Ok(true),
            Ok(_) => Ok(false),
            Err(source) => Err(ReplaceFaceError::Escalated { source }),
        }
    };
    let on_axis = |p: Point3<T>| {
        centre_on_axis(frame, p, band).map_err(|source| ReplaceFaceError::Escalated { source })
    };
    let parallel = |v: Vec3<T>| v.normalize().cross(frame.dir).norm();
    Ok(match (structural, moved) {
        (
            Surface::Plane { origin, normal, .. },
            Surface::Plane {
                origin: m_origin,
                normal: m_normal,
                ..
            },
        ) => {
            if sine(parallel(*normal), "offset_axial_alignment")? {
                Constraint::Station(frame.station(*m_origin))
            } else {
                // A plane whose normal is PERPENDICULAR to the axis
                // contains the axis exactly when the axis point is on
                // it. Anything else cuts the body obliquely to its own
                // axis of revolution — a corner this reduction has no
                // coordinates for.
                let m = normal.normalize();
                if !sine(m.dot(frame.dir).abs(), "offset_axial_meridian")? {
                    return Err(not_axial(
                        "a plane neither normal to the axis nor containing it",
                    ));
                }
                let through = (m.dot(vec_of(frame.origin)) - m.dot(vec_of(*origin))).abs();
                match decide("offset_axial_meridian_through", Margin::of(through), band) {
                    Ok(Sign::Zero) => Constraint::Meridian {
                        m: m_normal.normalize(),
                        c: m_normal.normalize().dot(vec_of(*m_origin)),
                    },
                    Ok(_) => {
                        return Err(not_axial("a plane parallel to the axis but not through it"));
                    }
                    Err(source) => return Err(ReplaceFaceError::Escalated { source }),
                }
            }
        }
        (
            Surface::Cylinder { origin, axis, .. },
            Surface::Cylinder {
                radius: m_radius, ..
            },
        ) => {
            if !sine(parallel(*axis), "offset_axial_alignment")? || !on_axis(*origin)? {
                return Err(not_axial("a cylinder that is not coaxial with the body"));
            }
            Constraint::Wall(*m_radius)
        }
        (
            Surface::Cone {
                apex,
                axis,
                half_angle,
                ..
            },
            Surface::Cone { apex: m_apex, .. },
        ) => {
            if !sine(parallel(*axis), "offset_axial_alignment")? || !on_axis(*apex)? {
                return Err(not_axial("a cone that is not coaxial with the body"));
            }
            let (sin_a, cos_a) = half_angle.sin_cos();
            Constraint::Generator {
                h_apex: frame.station(*m_apex),
                sin_a,
                cos_a,
            }
        }
        (
            Surface::Sphere { center, axis, .. },
            Surface::Sphere {
                center: m_center,
                radius: m_radius,
                ..
            },
        ) => {
            if !sine(parallel(*axis), "offset_axial_alignment")? || !on_axis(*center)? {
                return Err(not_axial("a sphere whose centre is off the body's axis"));
            }
            Constraint::Ball {
                h_c: frame.station(*m_center),
                r: *m_radius,
            }
        }
        (
            Surface::Torus { center, axis, .. },
            Surface::Torus {
                center: m_center,
                major_radius: m_major,
                minor_radius: m_minor,
                ..
            },
        ) => {
            if !sine(parallel(*axis), "offset_axial_alignment")? || !on_axis(*center)? {
                return Err(not_axial("a torus whose centre is off the body's axis"));
            }
            Constraint::Torus {
                major: *m_major,
                h_c: frame.station(*m_center),
                minor: *m_minor,
            }
        }
        (other, _) => {
            return Err(ReplaceFaceError::TogetherAxialUnsupported {
                face,
                kind: SurfaceKind::of(other),
            });
        }
    })
}

/// The ARC LENGTH of every edge ending at a vertex — the lengths this
/// door's conditioning is levered by.
///
/// The planar door levers by each edge's CHORD, which is the same
/// length there because a planar body's edges are straight. Here they
/// are not: a full revolve's rim arc runs half a turn and a chart with
/// ONE seam closes on itself, whose chord is exactly zero — and a zero
/// arm makes every meter read `Zero` and calls a perfectly transversal
/// corner degenerate. Measured on the revolved TUBE, which refused that
/// way before this. The arc length is the length that was always meant.
fn corner_arms<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
) -> Result<Vec<T>, ReplaceFaceError<T>> {
    let Some(emanating) = body
        .get_vertex(vertex)
        .ok_or(ReplaceFaceError::Corrupt)?
        .emanating
    else {
        return Ok(Vec::new());
    };
    let orbit = body
        .vertex_orbit(emanating)
        .ok_or(ReplaceFaceError::Corrupt)?;
    let mut out = Vec::new();
    for he in orbit {
        let edge = body
            .get_edge(
                body.get_half_edge(he)
                    .ok_or(ReplaceFaceError::Corrupt)?
                    .edge,
            )
            .ok_or(ReplaceFaceError::Corrupt)?;
        let curve = body
            .get_curve_geom(edge.curve)
            .and_then(crate::null::CurveGeom::certified)
            .ok_or(ReplaceFaceError::Corrupt)?;
        let (t0, t1) = curve.params();
        out.push(match curve.carrier() {
            // A line's parameter IS arc length; a circle's is an angle
            // levered by its own radius. Anything else falls back to
            // the endpoints' chord, which is what the carrier can say.
            Curve3::Line { .. } => (t1 - t0).abs(),
            Curve3::Circle { radius, .. } => (t1 - t0).abs() * *radius,
            other => other.eval(t1).distance(other.eval(t0)),
        });
    }
    Ok(out)
}

/// The chart's rigid displacement when its offset IS a translation.
///
/// A plane's is `n̂·d`. A cone's is the apex slide — the offset cone is
/// the same cone with its apex moved along the axis, so a generator
/// seam translates exactly. A cylinder's and a sphere's are radius
/// changes and are not translations of the surface at all, so they
/// report `None` and a mapped description on them refuses rather than
/// being shifted by a vector that does not exist.
fn rigid_shift<T: Real>(old: &Surface<T>, new: &Surface<T>) -> Option<Vec3<T>> {
    match (old, new) {
        (Surface::Plane { origin: a, .. }, Surface::Plane { origin: b, .. })
        | (Surface::Cone { apex: a, .. }, Surface::Cone { apex: b, .. }) => Some(*b - *a),
        _ => None,
    }
}

// ---------------------------------------------------------------------
// The corner
// ---------------------------------------------------------------------

/// The corner: the profile solve, then the azimuth (module docs).
fn solve_corner<T: Decide>(
    vertex: VertexKey,
    here: Point3<T>,
    at: &[&MovedChart<T>],
    arms: &[T],
    frame: &Frame<T>,
    band: Band,
) -> Result<Point3<T>, ReplaceFaceError<T>> {
    let refuse = |what: &'static str| ReplaceFaceError::TogetherAxialCorner {
        vertex,
        surfaces: at.len(),
        what,
    };
    // A corner asked to move nothing does not move, and is answered
    // before any meter runs: metering a motion of zero would call every
    // corner of a stationary body degenerate, and the refusals' own
    // words have to stay true.
    let requested = at.iter().fold(T::zero(), |acc, c| acc + c.distance.abs());
    match decide("offset_axial_request", Margin::of(requested), band) {
        Ok(Sign::Zero) => return Ok(here),
        Ok(_) => {}
        Err(source) => return Err(ReplaceFaceError::Escalated { source }),
    }

    // A corner where every chart is a PLANE has no axis in it, and the
    // planar door's own solve answers it — the same arithmetic, so an
    // all-planar corner of a mixed body reads exactly as it would on an
    // all-planar one.
    if at.len() >= 3 && at.iter().all(|c| plane_of(&c.new).is_some()) {
        let planes: Vec<(Vec3<T>, T)> = at
            .iter()
            .filter_map(|c| plane_of(&c.new))
            .map(|(n, o)| (n, n.dot(vec_of(o))))
            .collect();
        return crate::offset_together::solve_planar_corner(vertex, &planes, arms, band);
    }

    let h_old = frame.station(here);
    let radial_old = frame.radial(here);
    let rho_old = radial_old.norm();
    let mut profiles: Vec<Profile<T>> = Vec::new();
    let mut meridians: Vec<(Vec3<T>, T)> = Vec::new();
    for c in at {
        match c.constraint {
            Constraint::Meridian { m, c: k } => meridians.push((m, k)),
            Constraint::Station(h) => profiles.push(Profile::Line {
                n: (T::zero(), T::one()),
                c: h,
            }),
            Constraint::Wall(r) => profiles.push(Profile::Line {
                n: (T::one(), T::zero()),
                c: r,
            }),
            Constraint::Ball { h_c, r } => profiles.push(Profile::Circle {
                rho_c: T::zero(),
                h_c,
                r,
            }),
            Constraint::Torus { major, h_c, minor } => profiles.push(Profile::Circle {
                rho_c: major,
                h_c,
                r: minor,
            }),
            Constraint::Generator {
                h_apex,
                sin_a,
                cos_a,
            } => {
                // The generator LINE has two branches, one on each side
                // of the apex station. A cone FACE lives on one of them,
                // and which one is read from this corner's own side of
                // the apex rather than guessed.
                //
                // This is NOT the face's nappe read a second time, and
                // the two cannot be unified. The nappe is the face's
                // corners against the BASE cone's apex; this is one
                // corner against the MOVED one, and the two differ by
                // the slide `d/sin α` — which is the whole content of
                // the apex-window question, here at corner granularity.
                // A corner is also shared with its neighbouring faces,
                // so it has no one face's nappe to inherit; and the
                // same predicate answers the sphere's equator below,
                // where there is no nappe at all.
                let side = side_of(
                    h_old - h_apex,
                    vertex,
                    "stands at its cone's apex station, where the generator has no side to \
                     offset toward",
                    band,
                )?;
                profiles.push(Profile::Line {
                    n: (cos_a, -side * sin_a),
                    c: -side * h_apex * sin_a,
                });
            }
        }
    }

    // ---- The shapes a SINGLE profile constraint answers. A vertex on
    // the axis is the classical one: its `ρ = 0` is not a guess — it is
    // what makes it a revolve's pole — and the station comes from the
    // surface that meets it. A CIRCLE profile answers two more, both
    // born from one measured fact (the wedge's own module-doc law, now
    // met by a circle wall): a partial revolve's meridian caps stop
    // containing the axis the moment they are offset, so a corner they
    // meet is displaced OFF it. With BOTH moved caps here, the caps
    // jointly determine the corner's radial line and the circle
    // supplies its station ([`cap_pair_corner`]); with ONE, the corner
    // keeps its own angular position on the moved circle — the carried
    // datum, the same one the sphere-seam mint below trusts — and the
    // cap fixes the azimuth through the shared meridian solve. With
    // NO plane through the axis, the corner is a point of its one
    // surface — a full tube's seam vertex — and moves as that surface's
    // offset moves every point of it: the same carried arm, for a
    // LINE profile as for a circle, with the azimuth carried too. ----
    let mut carried: Option<(T, T)> = None;
    if profiles.len() < 2 {
        let pole = match decide("offset_axial_pole", Margin::of(rho_old), band) {
            Ok(Sign::Zero) => true,
            Ok(_) => false,
            Err(source) => return Err(ReplaceFaceError::Escalated { source }),
        };
        let [only] = profiles[..] else {
            return Err(refuse(
                "no profile constraint meets here, so no point in the meridian half-plane is \
                 determined",
            ));
        };
        if let (Profile::Circle { .. }, &[cap0, cap1]) = (&only, &meridians[..]) {
            // The meridian-pair arm. `None` says the moved caps still
            // hold the axis, and the pole arm below keeps its answer.
            if let Some(point) = cap_pair_corner(
                vertex,
                at.len(),
                (rho_old, h_old),
                &only,
                (cap0, cap1),
                arms,
                frame,
                band,
            )? {
                return Ok(point);
            }
        }
        if !pole {
            // The carried arm: the corner's `(ρ, h)` is the OLD corner's
            // profile point under the one profile's OWN offset —
            // concentric with its circle, centre fixed and radius the
            // moved circle's own, or the perpendicular foot on the
            // moved line. The azimuth is solved below exactly as every
            // corner's is: carried when no plane contains the axis
            // here (a seam corner, the seam's own datum), from the
            // moved cap when one does (a rim corner, the wedge's law).
            // Carrying a circle corner's angle about its centre beside
            // a moved cap, and NOT carrying a line corner's station
            // beside one, is a convention this door chooses rather
            // than a geometric necessity — both are datums the operand
            // fixed — and the line case refuses rather than guesses.
            match (only, &meridians[..]) {
                (_, []) | (Profile::Circle { .. }, [_]) => {}
                (Profile::Line { .. }, [_, ..]) => {
                    return Err(refuse(
                        "a line profile and a plane containing the axis meet here off the \
                         axis: the plane fixes an azimuth and the line one coordinate, and the \
                         corner's station along the line is a datum this door does not carry",
                    ));
                }
                (Profile::Circle { .. }, [_, _, ..]) => {
                    return Err(refuse(
                        "a circle profile meets more than one plane containing the axis here, \
                         and the moved caps still hold the axis, so the corner is neither \
                         the caps' meeting line nor a carried point",
                    ));
                }
            }
            carried = Some(
                only.image_of(rho_old, h_old, band)
                    .map_err(|source| ReplaceFaceError::Escalated { source })?
                    .ok_or_else(|| {
                        refuse(
                            "a corner standing at its own profile circle's centre, which fixes no \
                             direction to carry it along",
                        )
                    })?,
            );
        } else {
            let h = match only {
                Profile::Line { n, c } => {
                    match decide(
                        "offset_axial_pole_station",
                        Margin::levered(n.1.abs(), frame.extent),
                        band,
                    ) {
                        Ok(Sign::Positive) => {}
                        Ok(_) => {
                            return Err(refuse("an axis pole whose one surface fixes no station"));
                        }
                        Err(source) => return Err(ReplaceFaceError::Escalated { source }),
                    }
                    c / n.1
                }
                Profile::Circle { rho_c, h_c, r } => {
                    // **A profile circle centred OFF the axis contains no
                    // point of the axis.** Its nearest approach is
                    // `ρ_c − r`, which the torus's standing construction
                    // invariant `R > r > 0` keeps strictly positive — so a
                    // vertex read as a pole against one is a contradiction,
                    // not a station, and `h_c ± r` would answer it with a
                    // number that is on no surface here. The arm decides
                    // the centre rather than the kind: it is the circle's
                    // own geometry that makes the step below valid.
                    match decide("offset_axial_pole_centre", Margin::of(rho_c), band) {
                        Ok(Sign::Zero) => {}
                        Ok(_) => {
                            return Err(refuse(
                                "an axis pole whose one surface is a profile circle centred off the \
                             axis, which no point of the axis lies on",
                            ));
                        }
                        Err(source) => return Err(ReplaceFaceError::Escalated { source }),
                    }
                    h_c + side_of(
                        h_old - h_c,
                        vertex,
                        "is an axis pole at its sphere's own equator station, where the pole \
                         has no side to move to",
                        band,
                    )? * r
                }
            };
            return Ok(frame.origin + frame.dir * h);
        }
    }

    // ---- The profile solve: the first well-conditioned PAIR, in the
    // order the vertex's own fan is walked. The conditioning arm is the
    // corner's OWN edge chords — the solve amplifies each surface's ε by
    // 1/|det|, and the question is whether that stays below a length at
    // which this is still a corner. Levering by the offset instead would
    // make the verdict a statement about the request wearing the words
    // of a statement about the geometry. ----
    let mut solved: Option<(T, T)> = carried;
    if solved.is_none() {
        'pairs: for (i, a) in profiles.iter().enumerate() {
            for b in profiles.iter().skip(i + 1) {
                let Some(det) = transversality(a, b) else {
                    continue;
                };
                let mut resolvable = true;
                for &arm in arms {
                    match decide("offset_axial_corner", Margin::levered(det.abs(), arm), band) {
                        Ok(Sign::Positive) => {}
                        Ok(_) => {
                            resolvable = false;
                            break;
                        }
                        Err(source) => return Err(ReplaceFaceError::Escalated { source }),
                    }
                }
                if resolvable {
                    solved = Some(nearest(&roots(a, b, det), rho_old, h_old, vertex, band)?);
                    break 'pairs;
                }
            }
        }
    }
    let (rho, h) = solved.ok_or_else(|| {
        refuse(
            "no pair of the surfaces here meets transversally enough to resolve this corner \
             against the edges that end at it — they are tangent, parallel, or they miss",
        )
    })?;
    match decide("offset_axial_radius", Margin::of(rho), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => return Err(refuse("the solved corner is on or across the axis")),
        Err(source) => return Err(ReplaceFaceError::Escalated { source }),
    }
    // Every profile constraint is read against the answer. On a corner
    // solved from a PAIR this is where a THIRD surface is verified —
    // a corner placed off one of its own surfaces is a wrong body no
    // tier catches — and for the pair itself, and for a CARRIED corner
    // that [`Profile::image_of`] landed ON its one profile, the residual
    // is rounding by construction and this meter decides nothing. The
    // meter for those is the edge layer: every endpoint is read back
    // onto its carrier and every carrier's midpoint onto both moved
    // surfaces, which is where a corner that does not fit its edges
    // is caught.
    for p in &profiles {
        let gap = p.residual(rho, h);
        match decide("offset_axial_concurrence", Margin::of(gap), band) {
            Ok(Sign::Zero) => {}
            Ok(_) => {
                return Err(refuse(
                    "the surfaces meeting here do not concur after the offset, so this corner \
                     has no offset point",
                ));
            }
            Err(source) => return Err(ReplaceFaceError::Escalated { source }),
        }
    }

    // ---- The azimuth. ----
    match decide("offset_axial_azimuth_arm", Margin::of(rho_old), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => {
            return Err(refuse(
                "the corner stands on the axis, so it has no azimuth to carry or to solve",
            ));
        }
        Err(source) => return Err(ReplaceFaceError::Escalated { source }),
    }
    let e_old = radial_old / rho_old;
    match meridians[..] {
        // Carried: the seam's azimuth is the operand's own conventional
        // datum (D2). Carrying it is what makes an unmoved corner come
        // back unchanged, and it is the same law the planar door applies
        // to a line's `t = 0` anchor.
        [] => Ok(frame.place(rho, h, e_old)),
        [(m, c)] => {
            let w = frame.dir.cross(e_old);
            let a_co = rho * m.dot(e_old);
            let b_co = rho * m.dot(w);
            let rhs = c - m.dot(vec_of(frame.origin)) - m.dot(frame.dir) * h;
            let amp = Vec3::new(a_co, b_co, T::zero()).norm();
            // The two azimuths are `φ₀ ± Δ`, and the points they name are
            // `2ρ·sin Δ` apart — a LENGTH, which dies exactly as the
            // plane goes tangent to this corner's circle. Metered as the
            // length it is; no lever is needed and none is invented.
            // The plane's own reach across this corner's circle. A zero
            // amplitude means the plane is the axis itself as far as
            // this circle can tell, and there is no azimuth in it.
            match decide("offset_axial_azimuth_amp", Margin::of(amp), band) {
                Ok(Sign::Positive) => {}
                Ok(_) => {
                    return Err(refuse(
                        "the plane containing the axis has no reach across this corner's circle, \
                         so it fixes no azimuth",
                    ));
                }
                Err(source) => return Err(ReplaceFaceError::Escalated { source }),
            }
            let cos_d = clamp_unit(rhs / amp);
            let sin_d = (T::one() - cos_d.powi(2)).sqrt();
            let separation = T::from_f64(2.0) * rho * sin_d;
            match decide("offset_axial_azimuth", Margin::of(separation), band) {
                Ok(Sign::Positive) => {}
                Ok(_) => {
                    return Err(refuse(
                        "the plane containing the axis does not cut this corner's circle \
                         transversally, so its azimuth is not determined",
                    ));
                }
                Err(source) => return Err(ReplaceFaceError::Escalated { source }),
            }
            let base = b_co.atan2(a_co);
            let delta = cos_d.acos();
            // The two roots are `separation` apart and that length has
            // just been decided positive, so which of them the offset
            // keeps IS determined — and it is decided, not compared.
            let mut p = {
                let (s, cph) = (base + delta).sin_cos();
                frame.place(rho, h, e_old * cph + w * s)
            };
            let other = {
                let (s, cph) = (base - delta).sin_cos();
                frame.place(rho, h, e_old * cph + w * s)
            };
            match decide(
                "offset_axial_branch",
                Margin::of(other.distance(here) - p.distance(here)),
                band,
            ) {
                Ok(Sign::Negative) => p = other,
                Ok(Sign::Positive) => {}
                Ok(Sign::Zero) => {
                    return Err(refuse(
                        "the two azimuths stand the same distance from the corner being moved, \
                         so which one the offset keeps is not determined",
                    ));
                }
                Err(source) => return Err(ReplaceFaceError::Escalated { source }),
            }
            // The chosen root is VERIFIED onto the plane it came from.
            let gap = m.dot(vec_of(p)) - c;
            match decide("offset_axial_azimuth_residual", Margin::of(gap), band) {
                Ok(Sign::Zero) => Ok(p),
                Ok(_) => Err(refuse("the azimuth solve did not land on its own plane")),
                Err(source) => Err(ReplaceFaceError::Escalated { source }),
            }
        }
        _ => Err(refuse(
            "more than one plane containing the axis meets here, so the azimuth is \
             over-determined — a form this corpus has no fixture for and this door does not guess",
        )),
    }
}

/// **The rim corner a circle-profile wall shares with BOTH moved
/// meridian caps** — the sphere lune's pole corner once its caps are
/// offset.
///
/// A partial revolve's two meridian caps contain the axis at rest and
/// stop containing it when they are offset inward, so the corner that
/// stood at an axis pole is displaced off the axis. The two moved
/// planes both stay parallel to the axis and meet in a LINE parallel to
/// it, and that line is one radial fact plus one azimuth: `ρ = ρ_L` is
/// exactly a cylinder's profile constraint, so the caps' joint
/// constraint is DERIVED as that line and handed to the standard pair
/// machinery against the circle — [`transversality`] levered by the
/// corner's own arms, [`roots`]' closed form, the branch decided by
/// [`nearest`] against the old corner. The azimuth is the line's own;
/// no datum is carried, because the two caps leave nothing free.
///
/// The line is solved in the cross-section normal to the axis — a
/// meridian normal's axial component was decided zero at
/// classification — and the answer is then VERIFIED against both FULL
/// plane equations, so the projection is a solving convenience and not
/// a trusted convention.
///
/// Answers `None` when the moved caps still hold the axis (`ρ_L = 0`):
/// the corner is then the pole the pole arm already answers, and this
/// arm deliberately does not shadow it.
#[allow(clippy::too_many_arguments)]
fn cap_pair_corner<T: Decide>(
    vertex: VertexKey,
    surfaces: usize,
    old: (T, T),
    circle: &Profile<T>,
    caps: ((Vec3<T>, T), (Vec3<T>, T)),
    arms: &[T],
    frame: &Frame<T>,
    band: Band,
) -> Result<Option<Point3<T>>, ReplaceFaceError<T>> {
    let refuse = |what: &'static str| ReplaceFaceError::TogetherAxialCorner {
        vertex,
        surfaces,
        what,
    };
    let (rho_old, h_old) = old;
    let ((m0, c0), (m1, c1)) = caps;

    // The caps' meeting line, in the cross-section basis (e1, e2). The
    // pair's transversality is the sine between their cross-section
    // normals, levered by the corner's own arms exactly as the profile
    // pairs' is: the solve divides by it, so it is decided first.
    let mp0 = m0 - frame.dir * m0.dot(frame.dir);
    let mp1 = m1 - frame.dir * m1.dot(frame.dir);
    let (n0, n1) = (mp0.norm(), mp1.norm());
    // The divisors are certified UPSTREAM, not here: `m0`/`m1` are unit
    // meridian normals whose axial component `classify` decided Zero at
    // `offset_axial_meridian` (levered by the body's own extent), so
    // each cross-section projection keeps norm ~1 — a certified
    // distance from zero this division stands on, cited rather than
    // re-decided (one derivation, the module's own law).
    let sine = (mp0.cross(mp1) / (n0 * n1)).dot(frame.dir);
    for &arm in arms {
        match decide(
            "offset_axial_cap_pair",
            Margin::levered(sine.abs(), arm),
            band,
        ) {
            Ok(Sign::Positive) => {}
            Ok(_) => {
                return Err(refuse(
                    "the two moved meridian caps meet in no line this corner's own edges can \
                     resolve — their planes are parallel or nearly so",
                ));
            }
            Err(source) => return Err(ReplaceFaceError::Escalated { source }),
        }
    }
    let e1 = mp0 / n0;
    let e2 = frame.dir.cross(e1);
    let rhs0 = c0 - m0.dot(vec_of(frame.origin));
    let rhs1 = c1 - m1.dot(vec_of(frame.origin));
    let alpha = rhs0 / n0;
    let beta = (rhs1 - alpha * mp1.dot(e1)) / (n1 * sine);
    let rho_line = Vec3::new(alpha, beta, T::zero()).norm();
    match decide("offset_axial_cap_line", Margin::of(rho_line), band) {
        // The caps still hold the axis: the pole arm's territory.
        Ok(Sign::Zero) => return Ok(None),
        Ok(Sign::Positive) => {}
        // `rho_line` is a norm; a Negative verdict is not a geometric
        // posture but the margin machinery itself breaking — a kernel
        // bug, surfaced rather than solved on.
        Ok(Sign::Negative) => return Err(ReplaceFaceError::Corrupt),
        Err(source) => return Err(ReplaceFaceError::Escalated { source }),
    }
    let e = (e1 * alpha + e2 * beta) / rho_line;

    // The caps' joint constraint as the profile line `ρ = ρ_L`, against
    // the circle through the standard machinery.
    let wall = Profile::Line {
        n: (T::one(), T::zero()),
        c: rho_line,
    };
    let det = transversality(&wall, circle).ok_or(ReplaceFaceError::Corrupt)?;
    for &arm in arms {
        match decide("offset_axial_corner", Margin::levered(det.abs(), arm), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => {
                return Err(refuse(
                    "the moved caps' meeting line does not cross the profile circle \
                     transversally against the edges that end here — it is tangent, or it \
                     misses the circle",
                ));
            }
            Err(source) => return Err(ReplaceFaceError::Escalated { source }),
        }
    }
    let (rho, h) = nearest(&roots(&wall, circle, det), rho_old, h_old, vertex, band)?;
    match decide("offset_axial_radius", Margin::of(rho), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => return Err(refuse("the solved corner is on or across the axis")),
        Err(source) => return Err(ReplaceFaceError::Escalated { source }),
    }
    // The root read back onto the circle it came from: the same shape
    // as the corner solve's concurrence loop on its own pair, so it can
    // see only a root the quadratic lost to rounding, never a corner
    // off a surface — the full planes below and the edge layer are the
    // meters that can.
    let gap = circle.residual(rho, h);
    match decide("offset_axial_concurrence", Margin::of(gap), band) {
        Ok(Sign::Zero) => {}
        Ok(_) => {
            return Err(refuse(
                "the surfaces meeting here do not concur after the offset, so this corner has \
                 no offset point",
            ));
        }
        Err(source) => return Err(ReplaceFaceError::Escalated { source }),
    }

    // The answer, VERIFIED onto both full planes — the cross-section
    // projection above was a solving step, not a claim.
    let p = frame.place(rho, h, e);
    for (m, c) in [(m0, c0), (m1, c1)] {
        let gap = m.dot(vec_of(p)) - c;
        match decide("offset_axial_azimuth_residual", Margin::of(gap), band) {
            Ok(Sign::Zero) => {}
            Ok(_) => {
                return Err(refuse(
                    "the rim corner solve did not land on its own moved cap",
                ));
            }
            Err(source) => return Err(ReplaceFaceError::Escalated { source }),
        }
    }
    Ok(Some(p))
}

/// Two profile curves' TRANSVERSALITY — the sine of the angle at which
/// they cross, a pure number for the caller to lever by the corner's
/// own arms. `None` for circle∩circle: a form the corpus has no fixture
/// for, so it is not written.
///
/// Split from [`roots`] deliberately. The sine is what decides whether
/// the corner resolves at all, and the roots divide by it — so the
/// division happens only after a `decide` has certified the divisor,
/// rather than being computed and hoped over.
fn transversality<T: Real>(a: &Profile<T>, b: &Profile<T>) -> Option<T> {
    match (a, b) {
        (Profile::Line { n: na, .. }, Profile::Line { n: nb, .. }) => {
            // Two UNIT 2-D normals: this IS the sine of the crossing
            // angle.
            Some(na.0 * nb.1 - na.1 * nb.0)
        }
        (Profile::Line { n, c }, Profile::Circle { rho_c, h_c, r })
        | (Profile::Circle { rho_c, h_c, r }, Profile::Line { n, c }) => {
            // The half-chord over the radius is the sine of the angle
            // at which the line crosses the circle, and it dies exactly
            // at tangency. Clamped at zero because a line that MISSES
            // has no crossing at all, which is the same verdict.
            //
            // `d` is the SIGNED distance from the circle's centre to
            // the line, `n̂·(ρ_c, h_c) − c`: the centre's own ρ is part
            // of that projection, and a centre on the axis is the
            // `ρ_c = 0` case of it, not a different formula.
            let d = n.0 * *rho_c + n.1 * *h_c - *c;
            Some((r.powi(2) - d.powi(2)).max(T::zero()).sqrt() / *r)
        }
        (Profile::Circle { .. }, Profile::Circle { .. }) => None,
    }
}

/// The meeting points of two profile curves whose transversality the
/// caller has already certified positive.
fn roots<T: Real>(a: &Profile<T>, b: &Profile<T>, det: T) -> Vec<(T, T)> {
    match (a, b) {
        (Profile::Line { n: na, c: ca }, Profile::Line { n: nb, c: cb }) => vec![(
            (*ca * nb.1 - na.1 * *cb) / det,
            (na.0 * *cb - *ca * nb.0) / det,
        )],
        (Profile::Line { n, c }, Profile::Circle { rho_c, h_c, r })
        | (Profile::Circle { rho_c, h_c, r }, Profile::Line { n, c }) => {
            // The foot is the circle's centre stepped back along the
            // line's unit normal by that same signed distance, so the
            // centre's own ρ appears in both coordinates.
            let d = n.0 * *rho_c + n.1 * *h_c - *c;
            let half = det * *r;
            let foot = (*rho_c - n.0 * d, *h_c - n.1 * d);
            let dir = (-n.1, n.0);
            vec![
                (foot.0 + dir.0 * half, foot.1 + dir.1 * half),
                (foot.0 - dir.0 * half, foot.1 - dir.1 * half),
            ]
        }
        (Profile::Circle { .. }, Profile::Circle { .. }) => Vec::new(),
    }
}

/// The root nearest the old corner — the branch a small offset keeps.
///
/// The choice is DECIDED, not compared: two roots the same distance
/// from the old corner are two answers, and picking one of them would
/// be a guess. `Vertex` names the corner in the refusal.
fn nearest<T: Decide>(
    roots: &[(T, T)],
    rho: T,
    h: T,
    vertex: VertexKey,
    band: Band,
) -> Result<(T, T), ReplaceFaceError<T>> {
    let far = |r: (T, T)| Vec3::new(r.0 - rho, r.1 - h, T::zero()).norm();
    let mut best = *roots.first().ok_or(ReplaceFaceError::Corrupt)?;
    for &r in &roots[1..] {
        match decide("offset_axial_branch", Margin::of(far(r) - far(best)), band) {
            Ok(Sign::Negative) => best = r,
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero) => {
                return Err(ReplaceFaceError::TogetherAxialCorner {
                    vertex,
                    surfaces: 0,
                    what: "two solutions stand the same distance from the corner being moved, so \
                           which one the offset keeps is not determined",
                });
            }
            Err(source) => return Err(ReplaceFaceError::Escalated { source }),
        }
    }
    Ok(best)
}

// ---------------------------------------------------------------------
// The edge
// ---------------------------------------------------------------------

/// The moved edge's carrier: its KIND and conventional frame carried
/// from the operand, its position taken from the corner solves. The
/// caller reads both endpoints back onto it and meters the result, and
/// meters its midpoint against both moved surfaces.
fn mint_carrier<T: Decide>(
    edge: EdgeKey,
    old: &Curve3<T>,
    start: (T, Point3<T>),
    charts: (&MovedChart<T>, &MovedChart<T>),
    frame: &Frame<T>,
    band: Band,
) -> Result<Curve3<T>, ReplaceFaceError<T>> {
    let (t0_old, p_start) = start;
    let (ca, cb) = charts;
    let refuse = |what: &'static str| ReplaceFaceError::TogetherAxialEdge { edge, what };
    let translate = |delta: Vec3<T>| -> Result<Curve3<T>, ReplaceFaceError<T>> {
        crate::replace_face::translate_curve(old, delta)
            .map_err(|error| ReplaceFaceError::Structure { edge, error })
    };

    // A SEAM is not an intersection of two surfaces — the two are the
    // same surface — so it moves under that chart's OWN offset map.
    //
    // **Every same-surface circle in the LATITUDE posture — centred on
    // the axis, in a plane normal to it — is a latitude circle, and
    // takes the latitude rule whatever its surface is**: a cylinder's
    // or a cone's collinear-vertex ring, a sphere authored as two
    // cocircular arcs, a cap plane split by a collinear vertex, a full
    // tube's equator. The posture is decided FIRST, by the one helper
    // every centre-on-axis question here goes through; the arms below
    // are the seams that are NOT latitudes — a generator line, a
    // sphere's great circle, a torus's meridian circle — and each
    // certifies its own posture.
    if ca.old_key == cb.old_key {
        if let Curve3::Circle {
            center: cc,
            axis,
            u_ref,
            ..
        } = old
            && latitude_posture(frame, *cc, *axis, band)?.is_none()
        {
            return Ok(latitude_circle(frame, p_start, *axis, *u_ref));
        }
        return match (&ca.old, old) {
            // A plane's and a cone's offsets are rigid translations, so
            // their seams translate with them.
            (Surface::Plane { .. } | Surface::Cone { .. }, _) => {
                translate(ca.rigid.ok_or(ReplaceFaceError::Corrupt)?)
            }
            // A cylinder's seam is a generator LINE, and the radius
            // change moves it perpendicular to itself, radially — a
            // rigid translation OF THAT LINE even though the surface's
            // own motion is not one.
            (Surface::Cylinder { .. }, Curve3::Line { origin, .. }) => {
                let e = frame.radial(*origin);
                let n = e.norm();
                match decide("offset_axial_seam_radial", Margin::of(n), band) {
                    Ok(Sign::Positive) => {}
                    Ok(_) => {
                        return Err(refuse(
                            "a cylinder seam standing on the axis has no radial direction to \
                             move along",
                        ));
                    }
                    Err(source) => return Err(ReplaceFaceError::Escalated { source }),
                }
                translate(e / n * ca.distance)
            }
            // A sphere's seam is a GREAT circle about the sphere's own
            // centre, and the offset is concentric: same centre, same
            // plane, radius moved by the chart's distance. Not a
            // translation, and not pretended to be one.
            (
                Surface::Sphere { center, .. },
                Curve3::Circle {
                    center: cc,
                    axis,
                    radius,
                    u_ref,
                },
            ) => {
                let off = cc.distance(*center);
                match decide("offset_axial_seam_concentric", Margin::of(off), band) {
                    Ok(Sign::Zero) => Ok(Curve3::Circle {
                        center: *cc,
                        axis: *axis,
                        radius: *radius + ca.distance,
                        u_ref: *u_ref,
                    }),
                    Ok(_) => Err(refuse(
                        "a sphere seam that is not a great circle about the sphere's own centre",
                    )),
                    Err(source) => Err(ReplaceFaceError::Escalated { source }),
                }
            }
            // A torus's seam that is no latitude (its equators were
            // taken above) is a MERIDIAN circle, centred on the
            // tube-centre circle `(ρ, h) = (R, h_c)` and moved
            // concentrically about it for the same reason a sphere's
            // is about the sphere's: the mint keeps `center`, `axis`,
            // `major_radius` and `u_ref` and moves only the minor
            // radius, so the tube centre is the datum that does not
            // move. The certificate is one length in the meridian
            // half-plane.
            (
                Surface::Torus {
                    center,
                    major_radius,
                    ..
                },
                Curve3::Circle {
                    center: cc,
                    axis,
                    radius,
                    u_ref,
                },
            ) => {
                let off = Vec3::new(
                    frame.radial(*cc).norm() - *major_radius,
                    frame.station(*cc) - frame.station(*center),
                    T::zero(),
                )
                .norm();
                match decide("offset_axial_seam_meridian", Margin::of(off), band) {
                    Ok(Sign::Zero) => Ok(Curve3::Circle {
                        center: *cc,
                        axis: *axis,
                        radius: *radius + ca.distance,
                        u_ref: *u_ref,
                    }),
                    Ok(_) => Err(refuse(
                        "a torus seam that is neither a latitude circle nor a meridian circle \
                         about the tube's own centre",
                    )),
                    Err(source) => Err(ReplaceFaceError::Escalated { source }),
                }
            }
            _ => Err(refuse(
                "a seam whose carrier and chart are not one of the closed-form pairs",
            )),
        };
    }

    // ---- The off-axis rim circle: a sphere wall × meridian cap. ----
    //
    // A partial revolve's rim edge lies in the meridian cap's own
    // plane, and the MOVED cap is that plane translated off the axis by
    // the offset — it cuts the moved sphere in a circle whose centre is
    // the cap's own foot of the sphere centre and whose radius is the
    // section's: the `plane ∩ sphere` closed form, computed here in the
    // door's own arithmetic (the module's law stands — no marching, no
    // SSI, no section call). The latitude arm below cannot say this: a
    // meridian plane is never normal to the axis, so this is a SIBLING
    // arm gated on the chart pair, and its predicates verify the rest
    // of the posture in full — the carrier really is the wall × cap
    // great circle at rest — rather than only the binding relation.
    let ball_meridian = match (&ca.constraint, &cb.constraint) {
        (Constraint::Ball { .. }, Constraint::Meridian { m, c }) => Some((ca, *m, *c)),
        (Constraint::Meridian { m, c }, Constraint::Ball { .. }) => Some((cb, *m, *c)),
        _ => None,
    };
    if let Some((wall, m, c)) = ball_meridian {
        let Curve3::Circle {
            center: cc,
            axis,
            radius,
            u_ref,
        } = old
        else {
            return Err(refuse(
                "an edge between a sphere wall and a meridian cap whose carrier is not a \
                 circle",
            ));
        };
        let (
            Surface::Sphere {
                center: sc,
                radius: sr,
                ..
            },
            Surface::Sphere { radius: old_r, .. },
        ) = (&wall.new, &wall.old)
        else {
            return Err(ReplaceFaceError::Corrupt);
        };
        // The carrier is centred on the sphere's own centre — one
        // length covers the operand's posture and the mint's
        // concentricity at once, since the sphere offset keeps its
        // centre.
        let off = cc.distance(*sc);
        match decide("offset_axial_rim_concentric", Margin::of(off), band) {
            Ok(Sign::Zero) => {}
            Ok(_) => {
                return Err(refuse(
                    "a circular edge between a sphere wall and a meridian cap that is not \
                     centred on the sphere's own centre",
                ));
            }
            Err(source) => return Err(ReplaceFaceError::Escalated { source }),
        }
        // ... and it is GREAT: its radius is the operand sphere's own.
        let dr = *radius - *old_r;
        match decide("offset_axial_rim_great", Margin::of(dr), band) {
            Ok(Sign::Zero) => {}
            Ok(_) => {
                return Err(refuse(
                    "a circular edge between a sphere wall and a meridian cap that is not the \
                     wall's own great circle in the cap",
                ));
            }
            Err(source) => return Err(ReplaceFaceError::Escalated { source }),
        }
        // ... and its plane is the cap's — the moved cap keeps its
        // normal, so one sine covers the operand plane and the moved
        // one together.
        let mh = m.normalize();
        let tilt = axis.normalize().cross(mh).norm();
        match decide(
            "offset_axial_rim_plane",
            Margin::levered(tilt, frame.extent),
            band,
        ) {
            Ok(Sign::Zero) => {}
            Ok(_) => {
                return Err(refuse(
                    "a circular edge between a sphere wall and a meridian cap that does not \
                     lie in the cap's own plane",
                ));
            }
            Err(source) => return Err(ReplaceFaceError::Escalated { source }),
        }
        // The moved cap's signed stand-off from the sphere centre along
        // its own normal. The section circle dies as the cap reaches
        // the sphere, and that is a length, decided before the root.
        let t = c - mh.dot(vec_of(*sc));
        match decide("offset_axial_rim_reach", Margin::of(*sr - t.abs()), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => {
                return Err(refuse(
                    "a meridian cap standing tangent to or beyond its sphere wall, whose \
                     section leaves no rim circle",
                ));
            }
            Err(source) => return Err(ReplaceFaceError::Escalated { source }),
        }
        return Ok(Curve3::Circle {
            center: *sc + mh * t,
            axis: *axis,
            radius: (sr.powi(2) - t.powi(2)).sqrt(),
            u_ref: *u_ref,
        });
    }

    // Two distinct charts: the carrier keeps its KIND and its
    // conventional frame, and its position is re-solved from the corner
    // solves. The caller reads both endpoints back onto it and meters
    // the result, and meters its midpoint against both moved surfaces.
    match old {
        Curve3::Line { origin, dir } => {
            // Two moved surfaces that both contain a straight edge move
            // it perpendicular to itself: the direction survives, and
            // the old carrier's `t = 0` anchor is conventional data
            // whose carrying is what keeps an unmoved corner's edge
            // bit-identical.
            //
            // **That is a posture, not a proof, and it is VERIFIED like
            // every other one here.** A meridian plane meets a CONE in
            // a hyperbola, so a straight edge between those two is
            // straight only where the operand made it so, and the
            // moved pair need not carry a line at all. Nothing detects
            // that here — the caller's endpoint meters and the
            // midpoint-on-surface meter do, and they refuse. Measured
            // on a conical wedge: the two ends come back 0.64 mm apart
            // and the door says `TogetherEdgeDisagreement`
            // (`sf2b_r2_probes::r2_a_conical_wedge_meridian_edge`).
            let shift = p_start - old.eval(t0_old);
            let delta = shift - *dir * shift.dot(*dir);
            Ok(Curve3::Line {
                origin: *origin + delta,
                dir: *dir,
            })
        }
        Curve3::Circle {
            center,
            axis,
            u_ref,
            ..
        } => {
            // A LATITUDE circle: coaxial with the body, so its centre
            // stays on the axis and its radius is the corner's own.
            if let Some(what) = latitude_posture(frame, *center, *axis, band)? {
                return Err(refuse(what));
            }
            Ok(latitude_circle(frame, p_start, *axis, *u_ref))
        }
        _ => Err(refuse(
            "an edge between two distinct charts whose carrier is neither a line nor a circle",
        )),
    }
}

/// **Does `p` stand on the axis?** The one door for that question —
/// a surface's centre, origin or apex at classification, a circle's
/// centre at the carrier mint — so it has one predicate name and one
/// distribution. The quantity is `p`'s distance from the axis, a
/// LENGTH in metres, and it is metered as one: the frame's extent
/// levers the dimensionless sines here (a normal's misalignment), not
/// a length that already carries its own scale.
fn centre_on_axis<T: Decide>(
    frame: &Frame<T>,
    p: Point3<T>,
    band: Band,
) -> Result<bool, Indeterminate> {
    Ok(matches!(
        decide(
            "offset_axial_centre",
            Margin::of(frame.radial(p).norm()),
            band
        )?,
        Sign::Zero
    ))
}

/// **Is a circle in the LATITUDE posture** — centred on the axis, in a
/// plane normal to it? `None` when it is; otherwise the predicate that
/// failed, in the words a carrier refusal uses. A sphere's great-circle
/// seam is centred on the axis too, and it is the plane that tells
/// the two apart.
fn latitude_posture<T: Decide>(
    frame: &Frame<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    band: Band,
) -> Result<Option<&'static str>, ReplaceFaceError<T>> {
    let escalated = |source| ReplaceFaceError::Escalated { source };
    if !centre_on_axis(frame, center, band).map_err(escalated)? {
        return Ok(Some(
            "a circular edge between two charts whose centre is off the axis",
        ));
    }
    let tilt = axis.normalize().cross(frame.dir).norm();
    match decide(
        "offset_axial_latitude_tilt",
        Margin::levered(tilt, frame.extent),
        band,
    ) {
        Ok(Sign::Zero) => Ok(None),
        Ok(_) => Ok(Some(
            "a circle centred on the axis whose plane is not normal to it",
        )),
        Err(source) => Err(escalated(source)),
    }
}

/// **The latitude rule**: a circle in the latitude posture keeps its
/// normal's sign and its `u_ref` — conventional data, which keeps the
/// parameterization's sense — and takes the moved corner's own station
/// and radius. The posture is [`latitude_posture`]'s to decide; this
/// is only the mint.
fn latitude_circle<T: Real>(
    frame: &Frame<T>,
    p_start: Point3<T>,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
) -> Curve3<T> {
    Curve3::Circle {
        center: frame.origin + frame.dir * frame.station(p_start),
        axis,
        radius: frame.radial(p_start).norm(),
        u_ref,
    }
}

/// The moved endpoint's parameter on the moved carrier, with the point
/// VERIFIED onto it — the check that two independent corner solves
/// agree about the edge between them.
///
/// A LINE's parameter is arc length, so it is read by projection. A
/// CIRCLE's is an angle, and an angle read by `atan2` alone would lose
/// the turn the operand's own window took (a `(π, 2π)` arc would come
/// back as `(π, 0)`). So the angle is read as a DIFFERENCE — how far
/// the corner's own azimuth moved, which is small — and added to the
/// old parameter. The window's turn is conventional data, carried; only
/// the motion is re-derived.
fn param_on<T: Decide>(
    carrier: &Curve3<T>,
    old: &Curve3<T>,
    t_old: T,
    q: Point3<T>,
    p: Point3<T>,
    edge: EdgeKey,
    band: Band,
) -> Result<T, ReplaceFaceError<T>> {
    let t = match (carrier, old) {
        (Curve3::Line { origin, dir }, _) => (p - *origin).dot(*dir),
        (
            Curve3::Circle {
                center,
                axis,
                u_ref,
                ..
            },
            Curve3::Circle {
                center: old_center, ..
            },
        ) => {
            let _ = u_ref;
            let n = axis.normalize();
            let ray = |from: Point3<T>, at: Point3<T>| {
                let v = at - from;
                v - n * v.dot(n)
            };
            let (a, b) = (ray(*old_center, q), ray(*center, p));
            t_old + a.cross(b).dot(n).atan2(a.dot(b))
        }
        _ => {
            return Err(ReplaceFaceError::TogetherAxialEdge {
                edge,
                what: "a carrier kind this door does not parameterize",
            });
        }
    };
    let gap = carrier.eval(t).distance(p);
    match decide("offset_axial_edge_agreement", Margin::of(gap), band) {
        Ok(Sign::Zero) => Ok(t),
        Ok(_) => Err(ReplaceFaceError::TogetherEdgeDisagreement { edge, gap }),
        Err(source) => Err(ReplaceFaceError::Escalated { source }),
    }
}

/// A point's signed distance to a moved surface, in meters — the
/// residual every surface verification here reads. Each arm is a
/// projection of a metre vector onto a unit direction, or a Euclidean
/// norm minus a radius.
fn surface_residual<T: Real>(surface: &Surface<T>, p: Point3<T>, frame: &Frame<T>) -> T {
    match surface {
        Surface::Plane { origin, normal, .. } => normal.dot(p - *origin),
        Surface::Cylinder { radius, .. } => frame.radial(p).norm() - *radius,
        Surface::Sphere { center, radius, .. } => p.distance(*center) - *radius,
        Surface::Cone {
            apex, half_angle, ..
        } => {
            // The cone's own unit normal in the meridian half-plane is
            // `(cos α, −sin α)`, so this is the projection of the metre
            // vector `(ρ, |h|)` onto it.
            let (sin_a, cos_a) = half_angle.sin_cos();
            let v = p - *apex;
            let hh = v.dot(frame.dir);
            (v - frame.dir * hh).norm() * cos_a - hh.abs() * sin_a
        }
        // The torus's own meridian distance, in the same `(ρ, h)`
        // half-plane the corner solve works in. Without it the
        // edge-on-surface meter would read `zero` on every torus chart
        // and certify an edge it never measured.
        Surface::Torus {
            center,
            major_radius,
            minor_radius,
            ..
        } => {
            let rho = frame.radial(p).norm();
            let h = frame.station(p) - frame.station(*center);
            Vec3::new(rho - *major_radius, h, T::zero()).norm() - *minor_radius
        }
        _ => T::zero(),
    }
}

/// `description` re-stated for the moved edge.
///
/// - an **intrinsic** one keeps its (about to be remapped) surfaces
///   with the witness at the new mid-parameter;
/// - a **chart image** is asked for rather than carried: `image: None`
///   is the spec's documented REQUEST to derive it from the carrier.
///   This paragraph said "except on a CONE, whose offset slides `v` by
///   `d·cot α`" while the code did exactly that, and the cone frustum's
///   anti-seam refused at the attach layer's `ChartResidual` — a
///   constant shift describes a door that keeps its parameter WINDOW,
///   and this one re-solves both endpoints, so an edge shortens and
///   slides within its own chart;
/// - a **declaration** — the sketch entity under a sweep map that the
///   authority record keeps whole — is 3-space data and does owe the
///   transport. It is RE-AUTHORED in its own sketch plane rather than
///   translated: an offset moves the profile WITHIN the meridian plane
///   (a line perpendicular to itself, an arc concentrically) and the
///   placement is unchanged, so the sketch source's own points are
///   what move. That one rule covers a translated chart and a reshaped
///   one, which is why a sphere's seam needs no special case, and it
///   is what the per-face door cannot do — it has only a rigid delta,
///   so a reshaping chart makes it refuse.
///
/// Nothing authored here is trusted: the attach layer re-derives the
/// declaration against the carrier and refuses a mismatch.
fn restate<T: Decide>(
    description: EdgeDescription<T>,
    authority: EdgeAuthority<T>,
    mid: Point3<T>,
    carrier: &Curve3<T>,
    ends: (Point3<T>, Point3<T>),
    edge: EdgeKey,
    band: Band,
) -> Result<EdgeDescriptionSpec<T>, ReplaceFaceError<T>> {
    let carried = |mc: geom_brep::MappedCurve<T>| reauthor(mc, carrier, ends, edge, band);
    let declared = match authority {
        EdgeAuthority::Derived => None,
        EdgeAuthority::Declared(mc) => Some(carried(mc)?),
    };
    Ok(match description {
        EdgeDescription::Intersection { s1, s2, .. } => EdgeDescriptionSpec::Intersection {
            s1,
            s2,
            witness: mid,
        },
        EdgeDescription::TangentIntersection { s1, s2, .. } => {
            EdgeDescriptionSpec::TangentIntersection {
                s1,
                s2,
                witness: mid,
            }
        }
        EdgeDescription::Chart(c) => EdgeDescriptionSpec::Chart {
            surface: c.surface,
            // **`None` is the REQUEST to derive the image from the
            // carrier**, and it is the right one here for every chart
            // image, not only for a seam. The per-face door carries an
            // image forward under a constant `v` shift because it keeps
            // the edge's parameter WINDOW — it moves one chart and the
            // endpoints ride along. This door re-solves both endpoints
            // against every surface meeting them, so an edge SHORTENS
            // and slides within its own chart, and a constant shift
            // describes none of that. Measured: shifting it instead
            // refuses at the attach layer's `ChartResidual` on the cone
            // frustum's anti-seam, which is the gate doing its job.
            image: None,
            seam: c.seam,
            declared,
        },
        EdgeDescription::Scaffold(m) => EdgeDescriptionSpec::Scaffold(carried(m)?),
    })
}

/// A mapped description re-authored in its own sketch plane from the
/// endpoints the corner solves put it between.
///
/// A LINE takes the two points. An ARC takes them and the included
/// angle they subtend at the moved carrier's own centre — the offset of
/// a meridian arc is concentric, so the centre is the datum that does
/// not move and the sweep is what the endpoints say it is. A POINT's
/// trajectory — extruded along a vector, or revolved about an axis —
/// is the same trajectory of the moved point: the vector, the axis and
/// the angle are the operand's own conventional data and are carried,
/// and a revolved point's axis is this door's axis, so its rotation of
/// the moved corner IS the moved latitude circle — and that premise,
/// the moved corner standing IN the family's own sketch plane, is
/// decided rather than assumed: the pulled-back point's out-of-plane
/// coordinate is a length and is metered as one, and a corner the
/// azimuth solve moved out of the plane refuses typed. Refuses also
/// an arc whose moved carrier is no circle to subtend at.
fn reauthor<T: Decide>(
    mapped: geom_brep::MappedCurve<T>,
    carrier: &Curve3<T>,
    ends: (Point3<T>, Point3<T>),
    edge: EdgeKey,
    band: Band,
) -> Result<geom_brep::MappedCurve<T>, ReplaceFaceError<T>> {
    let refuse = |what: &'static str| ReplaceFaceError::TogetherAxialEdge { edge, what };
    let (p_start, p_end) = ends;
    Ok(match mapped {
        geom_brep::MappedCurve::PlacedSegment { segment, place } => {
            let inv = place.inverse();
            let flat = |p: Point3<T>| {
                let q = inv.transform_point(p);
                geom_core::Point2::new(q.x, q.y)
            };
            let (a, b) = (flat(p_start), flat(p_end));
            geom_brep::MappedCurve::PlacedSegment {
                segment: match segment {
                    geom_brep::SketchSegment::Line { .. } => {
                        geom_brep::SketchSegment::Line { a, b }
                    }
                    geom_brep::SketchSegment::Arc { .. } => {
                        let Curve3::Circle { center, .. } = carrier else {
                            return Err(refuse(
                                "a declaring pushforward whose sketch arc has no moved circle \
                                 to be re-authored about",
                            ));
                        };
                        let c = flat(*center);
                        let (u, v) = (a - c, b - c);
                        let theta = u.perp_dot(v).atan2(u.dot(v));
                        geom_brep::SketchSegment::Arc {
                            a,
                            b,
                            bulge: (theta / T::from_f64(4.0)).tan(),
                        }
                    }
                },
                place,
            }
        }
        geom_brep::MappedCurve::ExtrudedPoint { place, vec, .. } => {
            let q = place.inverse().transform_point(p_start);
            geom_brep::MappedCurve::ExtrudedPoint {
                point: geom_core::Point2::new(q.x, q.y),
                place,
                vec,
            }
        }
        geom_brep::MappedCurve::RevolvedPoint {
            place,
            axis_origin,
            axis_dir,
            angle,
            ..
        } => {
            let q = place.inverse().transform_point(p_start);
            match decide("offset_axial_reauthor_plane", Margin::of(q.z), band) {
                Ok(Sign::Zero) => {}
                Ok(_) => {
                    return Err(refuse(
                        "a revolved point's moved corner stands out of the family's own sketch \
                         plane, so the same rotation does not pass through it",
                    ));
                }
                Err(source) => return Err(ReplaceFaceError::Escalated { source }),
            }
            geom_brep::MappedCurve::RevolvedPoint {
                point: geom_core::Point2::new(q.x, q.y),
                place,
                axis_origin,
                axis_dir,
                angle,
            }
        }
    })
}

/// A point read as the vector from the origin — the form `n̂·x` needs.
fn vec_of<T: Real>(p: Point3<T>) -> Vec3<T> {
    Vec3::new(p.x, p.y, p.z)
}

/// A plane surface's stored normal and origin.
fn plane_of<T: Real>(s: &Surface<T>) -> Option<(Vec3<T>, Point3<T>)> {
    match s {
        Surface::Plane { origin, normal, .. } => Some((*normal, *origin)),
        _ => None,
    }
}

/// Which side of a station a corner stands on, DECIDED: a corner
/// exactly at a cone's apex station or a sphere's equator has no side,
/// and answering one anyway is how a branch gets guessed.
fn side_of<T: Decide>(
    x: T,
    vertex: VertexKey,
    what: &'static str,
    band: Band,
) -> Result<T, ReplaceFaceError<T>> {
    match decide("offset_axial_side", Margin::of(x), band) {
        Ok(Sign::Positive) => Ok(T::one()),
        Ok(Sign::Negative) => Ok(-T::one()),
        Ok(Sign::Zero) => Err(ReplaceFaceError::TogetherAxialCorner {
            vertex,
            surfaces: 0,
            what,
        }),
        Err(source) => Err(ReplaceFaceError::Escalated { source }),
    }
}

/// `x` clamped into `[-1, 1]` — the guard on a cosine the caller's own
/// separation meter has already certified.
fn clamp_unit<T: Real>(x: T) -> T {
    x.max(-T::one()).min(T::one())
}

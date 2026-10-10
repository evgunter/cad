//! `offset_charts_together` — the SIMULTANEOUS offset door for bodies
//! whose junction corners mix PLANES with cylinder, cone, sphere and
//! torus walls.
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
//! D-shaft            4 corners  3 [cylinder ∩ plane ∩ plane]
//! bored box          8 corners  3 [plane ∩ plane ∩ plane]
//!                    4 corners  2 [cylinder ∩ plane]
//! ```
//!
//! There is no `plane ∩ curved ∩ curved` corner anywhere in it, and no
//! corner with two axis-parallel planes holding the axis between them
//! and a curved wall, so neither is built: both refuse typed
//! ([`ReplaceFaceError::TogetherAxialCorner`]) rather than being
//! written on the presumption that something will need them.
//!
//! # Charts, not material
//!
//! Each move's distance is along its chart's stored normal, and no
//! face's sense decides the move: its argument is stated against charts
//! alone, so it takes construction state, a [`Body`] tier 2 in and
//! tier 2 out, where a door whose argument means something about
//! material takes an [`crate::AtRestBody`] (`crates/topo/README.md`,
//! "Shell and offset surgery"). Its result becomes finished only
//! through [`crate::AtRestBody::validate`]: on an inside-out solid the
//! charts move as they would on any other, and the result refuses
//! there as the operand would, `NegativeVolume`.
//!
//! # The reduction
//!
//! Every surface this door accepts is a surface of revolution about ONE
//! axis `(o, a)`, or a plane normal to it, or a plane PARALLEL to it —
//! through the axis or beside it, at any stand-off. That roster is what
//! "axial" means here: expressible in one axial frame. It is closed
//! under the door's own output (an offset moves each kind within
//! itself), so a body the door built passes its gate again — `shell`'s
//! rim lift is exactly that. Passing the gate is not building: a second
//! run meets the same corner and edge solves as the first, and an
//! offset large enough to change the body's topology is not seen by
//! them (`work/shelf/axial-door-builds-a-strict-subset-when-the-caps-meeting-line-enters-the-tube.md`).
//! A point `p` is read in axial coordinates
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
//! | plane parallel to `a` | not a profile constraint at all — through the axis it fixes the AZIMUTH; at stand-off `c` it fixes the azimuth given `ρ` (`ρ·cos(φ − φ_n) = c`, `φ_n` its normal's azimuth) |
//!
//! A corner is therefore solved in two steps, the profile first and
//! the azimuth from it, both closed form, neither of them marching:
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
//!    parallel to the axis meets this corner (the seam's own
//!    conventional datum), solved as circle∩plane when exactly one
//!    does, or read off the moved caps' meeting line when a
//!    circle-profile corner stands on two.
//!
//! **The offsets themselves are [`geom_brep::offset_surface`]'s** — the
//! same analytic mint the general door uses. One derivation, not a
//! second copy that could drift from it.
//!
//! # Every edge, and where its carrier comes from
//!
//! An edge whose two charts do not move keeps its carrier, and only
//! its window is re-read. A moved edge's carrier is one of two things.
//!
//! **A rim between a sphere or torus wall and a plane parallel to the
//! axis is the moved pair's own SECTION** —
//! [`geom_brep::plane_sphere_section`]'s circle, great or small, and
//! [`geom_brep::plane_torus_section`]'s two meridian circles when the
//! moved plane stands through the axis or its two spiric ovals when it
//! stands beside it. The kind is the section's, whatever the old
//! carrier's was: a cavity's cap offset off the axis turns a meridian
//! circle into a spiric, and the lift that puts it back turns the
//! spiric into a circle. The old carrier supplies only which of the
//! two torus curves the rim is (its midpoint's side of the cap's trace
//! of the axis) and the sense (its plane normal against the section's).
//!
//! **Every other carrier keeps the OLD carrier's kind and conventional
//! frame with its position re-solved** — a line keeps its direction
//! and moves perpendicular to itself, a latitude circle (a rim between
//! two charts, or a full tube's equator seam) keeps its normal and
//! `u_ref` and takes the corner's own station and radius, a sphere
//! seam's great circle keeps its centre and plane and takes the
//! chart's radius.
//!
//! Then the carrier is **verified**: both endpoints are read onto it
//! and metered, and its midpoint is metered against BOTH moved
//! surfaces. The parameters are always re-read, because a corner's
//! motion slides an endpoint ALONG its own edge as readily as it moves
//! the edge.
//!
//! # Which refusals have a row, and what builds their operand
//!
//! A `what:` string with a row is one a test reaches; whether a DOOR
//! builds the operand that reaches it is a separate fact, and the two
//! lists keep them apart. Everything else is written for correctness
//! and said so at the end.
//!
//! **A door builds the operand, and `shell` reaches the refusal:** the
//! two-root tie of [`ReplaceFaceError::TogetherAxialCorner`] ([`tie`];
//! the opened tangent dome's lift, `shell_curved_mouth`); the
//! meridian-pair arm's parallel-caps refusal (the half-turn lune) and
//! its tangent-or-miss refusal (the narrow 20° lune, whose moved caps'
//! meeting line stands `t/sin 10° ≈ 0.288` from the axis, past the
//! shrunk circle's `r − t = 0.25`) — both `torax_axial`;
//! `TogetherNotAxial`'s oblique-plane arm; `TogetherEdgeDisagreement` (`sf2b_r1_probes`, `sf2b_r2_probes`,
//! and `shell7_seam_corner`'s three-quarter-turn cone frustum); the
//! window's no-forward-window refusal (`sf2b_r1_probes::r1p2`'s sliver
//! wedge, whose moved meridian planes cross outside the shrunk wall,
//! and `sf2b_r2_probes`'s 1/12-turn conical wedge).
//!
//! **A hand-made operand, or the door called directly:** the
//! no-profile-constraint refusal (a wedge's axis edge split by
//! `Body::split_edge`, `shell7_seam_corner`); the line-beside-a-
//! meridian refusal (a wedge's wall/cap generator split the same way,
//! `shell7_seam_corner`); the partial-set and chart-mixed gates and the
//! sphere lune's whole rim solve (`offset_charts_together` called
//! directly, `torax_axial` — `shell`'s closing tier 3 needs a volume
//! the sphere flux arm cannot yet give the cavity's lens face, whose
//! rims are the moved caps' off-centre sections).
//!
//! **The re-author's turned ends have door-built rows**: a revolved
//! point whose ends the moved meridian caps turn about the axis — the
//! klein elbow's equator seams (`torax_axial`, `verbs_shell`,
//! `shell7_seam_corner`, `torax_interval`) and the two-arc lune's,
//! which certifies at the attach layer (`torax_axial`).
//!
//! **The near-tangent arms have door-built rows**, all on a dome over a
//! cylinder (`shell_curved_mouth`, `sf2b_axial`): [`branch`]'s side of
//! the foot (a cap short of tangent by `1e-12`, whose roots tie), and
//! [`tangent_foot`] (the tangent dome, whose roots tie on the foot
//! itself, and the tangent bullet, too ill-conditioned to solve). The
//! profile solve's own refusal for a pair that is nearly tangent outside
//! the band, parallel, or missing has none
//! (`work/shell/axial-corner-nearly-tangent-refusal-has-no-row.md`).
//!
//! **The carried arms themselves have door-built rows**: a full tube's
//! seam vertex (torus circle), a drum's collinear wall vertex
//! (cylinder line), a frustum's collinear generator vertex (cone
//! line) and a cap's collinear vertex (station line) all shell through
//! `shell7_seam_corner`. A sphere's same-surface latitude (sphere
//! circle) is door-built only by a partial revolve of cocircular arcs
//! (the two-arc lune, `torax_axial`); a full revolve builds the arc run
//! as one wall, so `shell7_seam_corner`'s two-arc sphere has its
//! latitude cut through the Euler door. **No row at all**, written for correctness:
//! the latitude posture's off-axis-centre refusal (no door-built
//! operand carries a circle between two distinct non-torus,
//! non-sphere charts with its centre off the axis; `torax_axial`
//! demonstrates it by mutation), the section rim mint's refusals (a cap
//! at or past its torus wall's inner equator or out of its reach, a
//! cap the section and the spiric constructor read on opposite sides
//! of the axis, a wall the section refuses as an operand, a midpoint
//! on the cap's trace of the axis, a sense that does not transfer, an
//! old carrier with no plane), the
//! circle-beside-three-caps refusal, the axis-pole station arm and its
//! off-axis-circle arm (a torus meridian cannot contain a pole,
//! `R − r > 0` keeps it clear), the seam arms' refusing sides, the
//! over-determined-azimuth arm — no constructible body here has more
//! than one plane parallel to the axis at a corner that is not also
//! all-planar or a cap pair — the re-author's turned-start refusal
//! (every door-built revolved point's sketch plane contains its axis; a
//! mutation that keeps the old azimuth reaches it on the klein elbow)
//! and its in-plane extrusion refusal (an extrude's vector never lies
//! in its own sketch plane), and the window's one-point refusal (a
//! door-built edge has two distinct ends), and the anchor's undecided
//! tie-break (a fresh carrier's frame would have to put the old point
//! within the band of a quarter turn from its old parameter).
//!
//! **The lift's rows**, the door run on a cavity it built: the klein
//! elbow opened at both caps and at one (`verbs_shell`), the lune
//! opened at one cap and, past the lifts, at both (`torax_axial`) —
//! the second of those moves a rim corner onto the axis, the pole arm
//! answering the cap pair that meets there. The extruded-point
//! re-author's moved stations are door-built by the bored hex prism
//! (`verbs_shell`).
//!
//! # What this door does not do
//!
//! - **No marching, no SSI, no crossing-pipeline entry.** Every solve
//!   above is a quadratic at worst.
//! - **It does not route through the C5 table.** A body with a surface
//!   that is neither of revolution about one axis nor a plane normal
//!   or parallel to it — or whose kind the gate does not know — never
//!   reaches here and keeps the refusal it had. The two section arms a
//!   rim calls are the table's own closed forms, called by name rather
//!   than through `route`: the one home of each pair's arithmetic. A
//!   FULL revolve's torus or sphere rim is a latitude circle and is
//!   this door's own.
//! - **It does not touch global clearance.** `shell`'s wall-clearance
//!   gate is the operand's and is unchanged; this door decides corners
//!   and edges. A sliver WEDGE whose two moved meridian planes cross
//!   outside the shrunk wall has no cavity at all, and every one of its
//!   rim corners still solves locally — each meets only ONE meridian
//!   plane — but the rim edge between two of them reads backwards, and
//!   the window's forward decision refuses it at the door
//!   (`offset_axial_edge_window`, measured on `sf2b_r1_probes::r1p2`).
//!   That is a local symptom of a global fact, not a clearance meter:
//!   an offset whose moved caps' meeting line enters the tube changes
//!   the body's topology while every corner and edge still solves, and
//!   the door returns a body with the operand's census that is a strict
//!   subset of the offset (filed:
//!   `work/shelf/axial-door-builds-a-strict-subset-when-the-caps-meeting-line-enters-the-tube.md`).
//!   `shell` does not reach that pose — a cavity moves its caps inward —
//!   but a direct call does.
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

use geom::SurfaceKind;
use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::k_stats::decide;
use geom_core::{Arc2, Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Tol, Vec3};

use crate::attach::Rechart;
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, VertexKey};
use crate::geometry::SurfaceKey;
use crate::live::{linked, proven};
use crate::offset_restate::chart_moves;
use crate::offset_together::{ChartMove, unmoved_in_scope, unplaced_in_scope};
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
    /// The signed offset along the chart's stored normal — the
    /// caller's number.
    distance: T,
    /// The chart's constraint on a corner, in axial terms.
    constraint: Constraint<T>,
    /// The rigid displacement the chart underwent, when its offset IS a
    /// rigid translation — `None` when it is not.
    rigid: Option<Vec3<T>>,
}

/// A moved chart's constraint on a corner, in the axial frame.
///
/// The first five are PROFILE constraints — curves in the `(ρ, h)`
/// half-plane. The last is not: a plane parallel to the axis says
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
    /// A plane PARALLEL to the axis, `m̂·x = c` with `m̂ ⊥ â`: through
    /// the axis (a partial revolve's meridian cap at rest) or standing
    /// beside it (that cap offset, a box's side, a shaft's flat). The
    /// stand-off is data — every corner and carrier arm reads `c`.
    AxisParallel { m: Vec3<T>, c: T },
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
/// stored normal. Every face of every SOLID the moves touch
/// must appear exactly once across them, and no solid may be touched in
/// part; a solid the moves do not name is not offset and its geometry
/// is not written.
///
/// **What it READS is not as tight as what it writes**, and the whole
/// account — which two reads are scope-sized, which four are still
/// linear in the body, and what that costs — is [`crate::offset_together::Scope`]'s, stated
/// there once for both doors.
///
/// The door **ends with the join** (`docs/DESIGN.md`, maximal edges):
/// the moved body is joined on the clone before it is adopted, and the
/// joins are returned ([`crate::replace_face::OffsetOutcome`]).
///
/// # Errors
///
/// [`ReplaceFaceError`], the body untouched on every one: the whole
/// plan is decided before anything is written, and the writes go to a
/// clone that replaces `body` only on success.
pub fn offset_charts_together<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    moves: &[ChartMove<T>],
    band: Band,
    tol: Tol,
) -> Result<crate::replace_face::OffsetOutcome, ReplaceFaceError<T>> {
    offset_charts_together_staged(body, moves, band, tol, true)
        .map(|joins| crate::replace_face::OffsetOutcome { joins })
}

/// [`offset_charts_together`], ending with the join where `join` is set. Unset, the
/// result is construction state a later step must join: the shell's
/// cavity and lift offsets, which key their naming rows by the moved
/// body's cells.
pub(crate) fn offset_charts_together_staged<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    moves: &[ChartMove<T>],
    band: Band,
    tol: Tol,
    join: bool,
) -> Result<Vec<crate::boolean::EdgeJoin>, ReplaceFaceError<T>> {
    // ---- Decide: the chart moves are well formed. ----
    let seen = crate::offset_together::well_formed(body, moves)?;
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
        // face's surface moves `−d` along its OWN chart normal. A
        // `ChartMove`'s distance is along that chart normal, so below
        // the apex it and `n₊` are opposite and the caller's number is
        // turned over before it reaches the mint.
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
        let first = *m.faces.first().ok_or(ReplaceFaceError::EmptyGroup)?;
        let first_data = proven(&body.faces, first, EntityId::Face);
        let d = match body.face_surface_linked(first, first_data) {
            Surface::Cone { .. } => {
                crate::offset_nappe::group_nappe(body, &m.faces, band)?.turn(m.distance)
            }
            _ => m.distance,
        };
        for &face in &m.faces {
            let data = body
                .get_face(face)
                .ok_or(ReplaceFaceError::StaleFace { face })?;
            let old = body.face_surface_linked(face, data).clone();
            let new = geom_brep::offset_surface(&old, d, band)
                .map_err(|error| ReplaceFaceError::Offset { face, error })?;
            let constraint = classify(face, &new, &frame, band)?;
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
    //
    // A corner whose charts the moves leave in place keeps its point;
    // the moved corners on one point are solved once, over every chart
    // meeting any of them, and move together (the planar door's rule,
    // `offset_planes_together`).
    let mut at_vertex = Vec::new();
    for (vertex, _) in body.vertices() {
        if !scope.holds_vertex(vertex) {
            continue;
        }
        let mut at: Vec<&MovedChart<T>> = Vec::new();
        for face in body.faces_of_vertex_linked(vertex) {
            let c = chart_of(face).unwrap_or_else(|| unmoved_in_scope(face));
            if !at.iter().any(|q| q.old_key == c.old_key) {
                at.push(c);
            }
        }
        let here = body.point_of(vertex, proven(&body.vertices, vertex, EntityId::Vertex));
        at_vertex.push((vertex, here, at));
    }
    let mut position: Vec<(VertexKey, Point3<T>)> = Vec::new();
    let mut asked: Vec<VertexKey> = Vec::new();
    for (vertex, here, at) in &at_vertex {
        if asked_to_move(at, band)? {
            asked.push(*vertex);
        } else {
            position.push((*vertex, *here));
        }
    }
    let mut moved: Vec<(Vec<VertexKey>, Point3<T>)> = Vec::new();
    for group in crate::replace_face::group_by_point(body, asked.clone()) {
        let mut at: Vec<&MovedChart<T>> = Vec::new();
        let mut arms: Vec<T> = Vec::new();
        let mut here = None;
        for (vertex, at_here, charts) in at_vertex.iter().filter(|(v, ..)| group.contains(v)) {
            for c in charts {
                if !at.iter().any(|q| q.old_key == c.old_key) {
                    at.push(c);
                }
            }
            arms.extend(corner_arms(body, *vertex)?);
            here.get_or_insert(*at_here);
        }
        let here = here.unwrap_or_else(|| {
            unreachable!("a group `group_by_point` answers holds the vertex it was asked about")
        });
        let point = solve_corner(group[0], here, &at, &arms, &frame, band)?;
        position.extend(group.iter().map(|&v| (v, point)));
        moved.push((group, point));
    }
    let point_at = |v: VertexKey| position.iter().find(|(k, _)| *k == v).map(|(_, p)| *p);

    // ---- Decide: every edge's carrier and description. ----
    let mut specs: Vec<(EdgeKey, EdgeCurveSpec<T>)> = Vec::new();
    for (edge, edge_data) in body.edges() {
        if !scope.holds_edge(edge) {
            continue;
        }
        let (fa, fb) = crate::readback::edge_sides_of(body, edge, edge_data).faces();
        let (ca, cb) = (
            chart_of(fa).unwrap_or_else(|| unmoved_in_scope(fa)),
            chart_of(fb).unwrap_or_else(|| unmoved_in_scope(fb)),
        );
        let start = linked(
            &body.half_edges,
            edge_data.he_plus,
            EntityId::HalfEdge,
            EntityId::Edge(edge),
            "he_plus",
        )
        .start;
        let end = body.proven_half_edge_end(edge_data.he_plus);
        let (p_start, p_end) = (
            point_at(start).unwrap_or_else(|| unplaced_in_scope(start)),
            point_at(end).unwrap_or_else(|| unplaced_in_scope(end)),
        );
        let old_point =
            |v: VertexKey| body.point_of(v, proven(&body.vertices, v, EntityId::Vertex));
        let (q_start, q_end) = (old_point(start), old_point(end));
        let Some(curve) = body.edge_curve_linked(edge, edge_data).certified() else {
            return Err(ReplaceFaceError::CarrierLaneUnsupported {
                edge,
                what: "it has no curve to move",
            });
        };
        let old_carrier = curve.carrier().clone();
        let (t0_old, t1_old) = curve.params();
        let description = curve.description().clone();
        let authority = curve.authority();

        // **An edge whose two charts do not move keeps its carrier**:
        // both surfaces it separates are where they were, so their
        // section is too, and only its window is re-read (an end can
        // still slide along it, turned by a moved neighbour).
        let held = !chart_moves(ca.distance, band)? && !chart_moves(cb.distance, band)?;
        let carrier = if held {
            old_carrier.clone()
        } else {
            mint_carrier(
                edge,
                &old_carrier,
                (t0_old, t1_old),
                p_start,
                (ca, cb),
                &frame,
                band,
            )?
        };

        // Both endpoints are READ onto the new carrier and metered. Two
        // corner solves agreeing about the edge between them is the
        // claim this door makes about every edge, and it is checked.
        let read = |t_old: T, v: VertexKey, q: Point3<T>, p: Point3<T>| {
            if held && !asked.contains(&v) {
                // Nothing moved under this end: its parameter stands.
                Ok(t_old)
            } else {
                param_on(&carrier, t_old, q, p, edge, band)
            }
        };
        let t0 = read(t0_old, start, q_start, p_start)?;
        let t1 = read(t1_old, end, q_end, p_end)?;
        let t1 = forward_window(&carrier, (t0_old, t1_old), t0, t1, edge, band)?;

        // The carrier is VERIFIED onto both moved surfaces at its own
        // midpoint: a re-derived edge's claim is that it lies on the two
        // surfaces it separates.
        let mid = carrier.mid_point(t0, t1);
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
                // Every image this door does not restate on a held
                // neighbour SLIDES within its own chart: the door
                // re-solves both endpoints against every surface meeting
                // them, so an edge shortens and moves along its chart,
                // and a constant shift of the old image describes none of
                // that (measured: it refuses `ChartResidual` on the cone
                // frustum's anti-seam). A declaration is RE-AUTHORED in
                // its own sketch plane ([`reauthor`]): the offset moves
                // the profile within the meridian plane and leaves the
                // placement alone, which covers a translated chart and a
                // reshaped one alike.
                description: crate::offset_restate::restate(
                    description,
                    authority,
                    [
                        (ca.old_key, chart_moves(ca.distance, band)?),
                        (cb.old_key, chart_moves(cb.distance, band)?),
                    ],
                    true,
                    mid,
                    |mc| reauthor(mc, &carrier, (p_start, p_end), edge, band),
                )?,
                carrier,
                param_start: t0,
                param_end: t1,
            },
        ));
    }

    // ---- Mutation, on a clone (every decision is done). ----
    //
    // Under a surgery scope for the whole of it: the setters below are
    // this door's operator sequence, and the tier-2 gate the clone is
    // adopted on is the door's own whole-body check.
    let mut staged = body.clone();
    let mut work = staged.begin_surgery();
    let mut charts: Vec<Rechart<T>> = Vec::new();
    for m in moves {
        let Some(&first) = m.faces.first() else {
            return Err(ReplaceFaceError::EmptyGroup);
        };
        let c = chart_of(first).unwrap_or_else(|| unmoved_in_scope(first));
        // **A chart asked to move nothing keeps its chart.** Re-minting
        // it would put a fresh key in the arena describing the same
        // surface — which is not a no-op to anything reading keys, and
        // this door is called with a mixed set (the rim LIFT moves ONE
        // chart of a body whose others must hold still).
        if !chart_moves(c.distance, band)? {
            continue;
        }
        charts.push(crate::replace_face::offset_rechart(
            &work,
            c.new.clone(),
            &m.faces,
        )?);
    }
    crate::replace_face::move_points_then_rechart(&mut work, &moved, charts, &specs, tol)?;
    // Every edge OF THE SCOPE was re-described, and the charts here DO
    // mint pcurve rows (a cylinder, a cone and a sphere all do), so this
    // pass is load-bearing rather than the planar door's inert one. It
    // runs over the scope's faces alone: an out-of-scope row belongs to
    // an edge this door did not touch and stays exactly as it was found.
    let minting = scope.faces_in_scope();
    crate::pcurves::mint_pcurves_of(&mut work, &minting, tol).map_err(|source| {
        ReplaceFaceError::Pcurve {
            source: source.for_driver(),
        }
    })?;
    // Tier 2 over the WHOLE clone, deliberately, and one of the four
    // reads that stay linear in the body (`Scope`'s docs carry the
    // account and the reason for each).
    work.sweep_and_close();
    if let Err(errors) = crate::validate::validate_closed(&staged) {
        return Err(ReplaceFaceError::ResultNotClosed { errors });
    }
    let joins =
        crate::replace_face::staged_join(&mut staged, join, tol, &|v| scope.holds_vertex(v))?;
    body.adopt(staged);
    Ok(joins)
}

// ---------------------------------------------------------------------
// The axis gate
// ---------------------------------------------------------------------

/// **Is this a body this door can take?** Structural and cheap: every
/// surface is a plane, cylinder, cone, sphere or TORUS, the curved ones
/// share one axis LINE, and every plane is normal to it or parallel to
/// it, through it or beside it — "axial" is "expressible in one axial
/// frame", not "a body of revolution". A box with a coaxial bore, a
/// flatted shaft and a hollow the door itself built are all axial, so
/// a lift reads the same verdict the cavity did.
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
/// general door — whose refusal would then name a carrier rather than
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
///
/// # Panics
///
/// On a torn body — a face's surface or a vertex's point that does not
/// resolve, a shell or loop walk that does not close — naming the
/// record (D2 row 4): only a kernel bug reaches one.
pub fn is_axial<T: Decide>(body: &Body<T>, band: Band) -> Result<bool, ReplaceFaceError<T>> {
    let scope = crate::offset_together::Scope::whole(body);
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
    let frame = match axial_frame(body, scope) {
        Ok(frame) => frame,
        // No curved chart, or no face at all: nothing revolves.
        Err(ReplaceFaceError::TogetherAxialUnsupported { .. } | ReplaceFaceError::EmptyGroup) => {
            return Ok(false);
        }
        Err(source) => return Err(source),
    };
    for (face, f) in body.faces() {
        if !scope.holds_face(face) {
            continue;
        }
        let surface = body.face_surface_linked(face, f);
        match classify(face, surface, &frame, band) {
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
/// scope has no axis and is not this door's; a scope holding no face
/// (an empty move set, a faceless body) names nothing to revolve and
/// answers [`ReplaceFaceError::EmptyGroup`].
fn axial_frame<T: Real>(
    body: &Body<T>,
    scope: &crate::offset_together::Scope,
) -> Result<Frame<T>, ReplaceFaceError<T>> {
    let first = body
        .faces()
        .find(|(k, _)| scope.holds_face(*k))
        .map(|(k, _)| k)
        .ok_or(ReplaceFaceError::EmptyGroup)?;
    let mut seed: Option<(Point3<T>, Vec3<T>)> = None;
    for (face, f) in body.faces() {
        if !scope.holds_face(face) {
            continue;
        }
        let found = match body.face_surface_linked(face, f) {
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
                    kind: other.kind(),
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
    for (key, vertex) in body.vertices().filter(|&(k, _)| scope.holds_vertex(k)) {
        extent = extent.max((body.point_of(key, vertex) - origin).norm());
    }
    Ok(Frame {
        origin,
        dir,
        extent,
    })
}

/// `surface`'s constraint on a corner, or the typed refusal that says
/// the chart is not expressible in this axial frame.
///
/// **One surface, read as it stands.** The gate reads a body's charts
/// and the door reads them moved, and the answer is the same kind
/// either way: an offset keeps a coaxial surface coaxial, a plane
/// normal to the axis normal to it, and a plane parallel to the axis
/// parallel to it — at whatever stand-off the offset leaves it. So the
/// roster is closed under the door's own output, and a lift (the door
/// run on a cavity it built) is classified by the same predicates as
/// the cavity's operand.
fn classify<T: Decide>(
    face: FaceKey,
    surface: &Surface<T>,
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
    Ok(match surface {
        Surface::Plane { origin, normal, .. } => {
            let n = normal.normalize();
            if sine(parallel(n), "offset_axial_alignment")? {
                Constraint::Station(frame.station(*origin))
            } else {
                // A plane whose normal is PERPENDICULAR to the axis is
                // parallel to it, through it or beside it. Anything
                // else cuts the body obliquely to its own axis of
                // revolution — a corner this reduction has no
                // coordinates for.
                if !sine(n.dot(frame.dir).abs(), "offset_axial_axis_parallel")? {
                    return Err(not_axial(
                        "a plane neither normal to the axis nor parallel to it",
                    ));
                }
                Constraint::AxisParallel {
                    m: n,
                    c: n.dot(vec_of(*origin)),
                }
            }
        }
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            if !sine(parallel(*axis), "offset_axial_alignment")? || !on_axis(*origin)? {
                return Err(not_axial("a cylinder that is not coaxial with the body"));
            }
            Constraint::Wall(*radius)
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            if !sine(parallel(*axis), "offset_axial_alignment")? || !on_axis(*apex)? {
                return Err(not_axial("a cone that is not coaxial with the body"));
            }
            let (sin_a, cos_a) = half_angle.sin_cos();
            Constraint::Generator {
                h_apex: frame.station(*apex),
                sin_a,
                cos_a,
            }
        }
        Surface::Sphere {
            center,
            axis,
            radius,
            ..
        } => {
            if !sine(parallel(*axis), "offset_axial_alignment")? || !on_axis(*center)? {
                return Err(not_axial("a sphere whose centre is off the body's axis"));
            }
            Constraint::Ball {
                h_c: frame.station(*center),
                r: *radius,
            }
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            if !sine(parallel(*axis), "offset_axial_alignment")? || !on_axis(*center)? {
                return Err(not_axial("a torus whose centre is off the body's axis"));
            }
            Constraint::Torus {
                major: *major_radius,
                h_c: frame.station(*center),
                minor: *minor_radius,
            }
        }
        other => {
            return Err(ReplaceFaceError::TogetherAxialUnsupported {
                face,
                kind: other.kind(),
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
    let mut out = Vec::new();
    for he in body.vertex_orbit_linked(vertex) {
        let key = proven(&body.half_edges, he, EntityId::HalfEdge).edge;
        let edge = linked(
            &body.edges,
            key,
            EntityId::Edge,
            EntityId::HalfEdge(he),
            "edge",
        );
        let Some(curve) = body.edge_curve_linked(key, edge).certified() else {
            return Err(ReplaceFaceError::CarrierLaneUnsupported {
                edge: key,
                what: "it has no curve to move",
            });
        };
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

/// Whether the moves ask the corner on `at`'s charts to move —
/// decided from the request (how far its charts are offset), before
/// any meter runs on the corner. Metering a motion of zero would call
/// every corner of a stationary body degenerate, and the refusals' own
/// words have to stay true.
fn asked_to_move<T: Decide>(
    at: &[&MovedChart<T>],
    band: Band,
) -> Result<bool, ReplaceFaceError<T>> {
    let requested = at.iter().fold(T::zero(), |acc, c| acc + c.distance.abs());
    match decide("offset_axial_request", Margin::of(requested), band) {
        Ok(Sign::Zero) => Ok(false),
        Ok(_) => Ok(true),
        Err(source) => Err(ReplaceFaceError::Escalated { source }),
    }
}

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
            Constraint::AxisParallel { m, c: k } => meridians.push((m, k)),
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
    // NO plane parallel to the axis, the corner is a point of its one
    // surface — a full tube's seam vertex — and moves as that surface's
    // offset moves every point of it: the same carried arm, for a
    // LINE profile as for a circle, with the azimuth carried too. ----
    let mut carried: Option<(T, T)> = None;
    if profiles.len() < 2 {
        let mut pole = match decide("offset_axial_pole", Margin::of(rho_old), band) {
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
            // The meridian-pair arm. `None` says the moved caps meet ON
            // the axis, so the corner is its pole — the one it already
            // was, or the one a lift putting both caps back through the
            // axis moves it onto — and the pole arm below answers it.
            match cap_pair_corner(
                vertex,
                at.len(),
                (rho_old, h_old),
                &only,
                (cap0, cap1),
                arms,
                frame,
                band,
            )? {
                Some(point) => return Ok(point),
                None => pole = true,
            }
        }
        if !pole {
            // The carried arm: the corner's `(ρ, h)` is the OLD corner's
            // profile point under the one profile's OWN offset —
            // concentric with its circle, centre fixed and radius the
            // moved circle's own, or the perpendicular foot on the
            // moved line. The azimuth is solved below exactly as every
            // corner's is: carried when no plane parallel to the axis
            // meets it (a seam corner, the seam's own datum), from the
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
                        "a line profile and a plane parallel to the axis meet here off the axis: \
                         the plane fixes an azimuth and the line one coordinate, and nothing \
                         records where along the line the corner stands",
                    ));
                }
                (Profile::Circle { .. }, [_, _, ..]) => {
                    return Err(refuse(
                        "a circle profile meets more than two planes parallel to the axis \
                         here, so the corner is neither the caps' meeting line nor a carried \
                         point",
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
                            return Err(refuse(
                                "a corner on the axis whose one surface fixes no \
                                               height along it",
                            ));
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
                                "a corner on the axis whose one surface is a profile circle \
                                 centred off the axis, which no point of the axis lies on",
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
    // order the vertex's own fan is walked, whose branch is determined
    // ([`branch`]: the nearest root, or on a tie the old corner's side of
    // the pair's foot). The conditioning arm is the corner's OWN edge
    // chords — the solve amplifies each surface's ε by 1/|det|, and the
    // question is whether that stays below a length at which this is
    // still a corner. Levering by the offset instead would make the
    // verdict a statement about the request wearing the words of a
    // statement about the geometry. A tangent pair whose roots the solve
    // cannot tell apart — too ill-conditioned to resolve, or tied on the
    // foot itself — names its foot ([`tangent_foot`]), the answer only
    // when no pair resolves; a tangency the band cannot decide escalates
    // only then too, so the outcome does not hang on the order the fan
    // is walked. ----
    let mut solved: Option<(T, T)> = carried;
    if solved.is_none() {
        let mut tangent: Option<Result<(T, T), ReplaceFaceError<T>>> = None;
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
                if resolvable && let Some(root) = branch(a, b, det, rho_old, h_old, band)? {
                    solved = Some(root);
                    break 'pairs;
                }
                match tangent_foot(a, b, band) {
                    Ok(Some(foot)) => {
                        tangent.get_or_insert(Ok(foot));
                    }
                    Ok(None) if resolvable => return Err(tie(vertex)),
                    Ok(None) => {}
                    Err(e) => {
                        tangent.get_or_insert(Err(e));
                    }
                }
            }
        }
        if solved.is_none() {
            solved = tangent.transpose()?;
        }
    }
    let (rho, h) = solved.ok_or_else(|| {
        refuse(
            "no pair of the surfaces here meets transversally enough to resolve this corner \
             against the edges that end at it — they are nearly tangent, parallel, or they \
             miss",
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
                        "the plane parallel to the axis has no reach across this corner's circle, \
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
                        "the plane parallel to the axis does not cut this corner's circle \
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
            "more than one plane parallel to the axis meets here, so the azimuth is \
             over-determined",
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
/// Answers `None` when the moved caps meet on the axis (`ρ_L = 0`):
/// the corner is then an axis pole — one it already stood on, or one a
/// lift that puts both caps back through the axis moves it onto — and
/// the pole arm answers it.
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
    // `offset_axial_axis_parallel` (levered by the body's own extent), so
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
        Ok(Sign::Negative) => {
            unreachable!("{vertex:?}: `rho_line` is a norm, which no margin decides negative")
        }
        Err(source) => return Err(ReplaceFaceError::Escalated { source }),
    }
    let e = (e1 * alpha + e2 * beta) / rho_line;

    // The caps' joint constraint as the profile line `ρ = ρ_L`, against
    // the circle through the standard machinery.
    let wall = Profile::Line {
        n: (T::one(), T::zero()),
        c: rho_line,
    };
    let det = transversality(&wall, circle)
        .unwrap_or_else(|| unreachable!("a line and a circle always have a transversality"));
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
    let nearest_root = if resolvable {
        branch(&wall, circle, det, rho_old, h_old, band)?
    } else {
        None
    };
    // A tangent meeting the solve cannot resolve, or whose roots tie,
    // is answered by its foot ([`tangent_foot`]).
    let (rho, h) = match (nearest_root, tangent_foot(&wall, circle, band)?) {
        (Some(root), _) => root,
        (None, Some(foot)) => foot,
        (None, None) if resolvable => return Err(tie(vertex)),
        (None, None) => {
            return Err(refuse(
                "the moved caps' meeting line does not cross the profile circle \
                 transversally against the edges that end here — it is nearly tangent, or it \
                 misses the circle",
            ));
        }
    };
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
        (Profile::Line { .. }, Profile::Circle { .. })
        | (Profile::Circle { .. }, Profile::Line { .. }) => {
            // The half-chord over the radius is the sine of the angle
            // at which the line crosses the circle, and it dies exactly
            // at tangency. Clamped at zero because a line that MISSES
            // has no crossing at all, which is the same verdict.
            let (r, d, _) = line_circle(a, b)?;
            Some((r.powi(2) - d.powi(2)).max(T::zero()).sqrt() / r)
        }
        (Profile::Circle { .. }, Profile::Circle { .. }) => None,
    }
}

/// A line–circle pair's circle radius `r`, the signed distance
/// `d = n̂·(ρ_c, h_c) − c` from the circle's centre to the line, and the
/// centre's foot on the line (the centre stepped back along the line's
/// unit normal by `d`). The
/// centre's own ρ is part of the projection, and a centre on the axis
/// is its `ρ_c = 0` case, not a different formula. `None` for any other
/// pair.
fn line_circle<T: Real>(a: &Profile<T>, b: &Profile<T>) -> Option<(T, T, (T, T))> {
    let ((Profile::Line { n, c }, Profile::Circle { rho_c, h_c, r })
    | (Profile::Circle { rho_c, h_c, r }, Profile::Line { n, c })) = (a, b)
    else {
        return None;
    };
    let d = n.0 * *rho_c + n.1 * *h_c - *c;
    Some((*r, d, (*rho_c - n.0 * d, *h_c - n.1 * d)))
}

/// A line–circle pair the band calls TANGENT, and its one meeting
/// point: the foot of the circle's centre on the line. `None` for a
/// pair that is not a line and a circle, and for one whose gap
/// `r − |d|` (a length) is not Zero.
///
/// The foot is a FALLBACK, taken only where the solve cannot tell the
/// pair's two roots apart: the conditioning meter cannot resolve the
/// pair, or [`branch`] finds them tied with the old corner on the foot
/// itself. A gap inside the band still leaves the roots `2√(2r·gap)`
/// apart, which can be far outside it, and wherever a root is
/// determined it is the corner the operand's crossing edge describes;
/// the foot would sit `√(2r·gap)` off it, where the moved surfaces are
/// tangent and that description does not certify. Where no root is,
/// the foot is the corner the exact data name: the double root, and the
/// midpoint of a split pair.
fn tangent_foot<T: Decide>(
    a: &Profile<T>,
    b: &Profile<T>,
    band: Band,
) -> Result<Option<(T, T)>, ReplaceFaceError<T>> {
    let Some((r, d, foot)) = line_circle(a, b) else {
        return Ok(None);
    };
    match decide("offset_axial_tangency", Margin::of(r - d.abs()), band) {
        Ok(Sign::Zero) => Ok(Some(foot)),
        Ok(_) => Ok(None),
        Err(source) => Err(ReplaceFaceError::Escalated { source }),
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
        (Profile::Line { n, .. }, Profile::Circle { .. })
        | (Profile::Circle { .. }, Profile::Line { n, .. }) => {
            let Some((r, _, foot)) = line_circle(a, b) else {
                unreachable!("a line and a circle have a foot")
            };
            let half = det * r;
            let dir = (-n.1, n.0);
            vec![
                (foot.0 + dir.0 * half, foot.1 + dir.1 * half),
                (foot.0 - dir.0 * half, foot.1 - dir.1 * half),
            ]
        }
        (Profile::Circle { .. }, Profile::Circle { .. }) => Vec::new(),
    }
}

/// The root nearest the old corner — the branch a small offset keeps —
/// or `None` where two roots tie.
///
/// The choice is DECIDED, not compared: two roots the same distance
/// from the old corner are two answers, and picking one of them would
/// be a guess. [`branch`] decides what a tie means.
fn nearest<T: Decide>(
    roots: &[(T, T)],
    rho: T,
    h: T,
    band: Band,
) -> Result<Option<(T, T)>, ReplaceFaceError<T>> {
    let far = |r: (T, T)| Vec3::new(r.0 - rho, r.1 - h, T::zero()).norm();
    // Both callers hand over a line pair's one root or a line–circle
    // pair's two, whose transversality they certified first.
    let Some(&first) = roots.first() else {
        unreachable!("a certified-transversal profile pair has a root")
    };
    let mut best = first;
    for &r in &roots[1..] {
        match decide("offset_axial_branch", Margin::of(far(r) - far(best)), band) {
            Ok(Sign::Negative) => best = r,
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero) => return Ok(None),
            Err(source) => return Err(ReplaceFaceError::Escalated { source }),
        }
    }
    Ok(Some(best))
}

/// The branch a small offset keeps on a profile pair whose
/// transversality the caller has certified: the [`nearest`] root, and
/// where two roots tie, the one on the old corner's side of the pair's
/// foot along the line. `None` where that side decides Zero too.
///
/// A tie is the shape of a pair near tangency, not of an ambiguous one:
/// the two roots stand symmetric about the foot, and every point near
/// the foot is nearly equidistant from them, so nearness cannot say
/// which root continues the corner. The side can: the old corner is a
/// point of the unmoved pair, which crossed on the same side of its own
/// foot, and that side is a length decided like any other.
fn branch<T: Decide>(
    a: &Profile<T>,
    b: &Profile<T>,
    det: T,
    rho: T,
    h: T,
    band: Band,
) -> Result<Option<(T, T)>, ReplaceFaceError<T>> {
    let roots = roots(a, b, det);
    if let Some(root) = nearest(&roots, rho, h, band)? {
        return Ok(Some(root));
    }
    let (Some((_, _, foot)), &[up, down]) = (line_circle(a, b), &roots[..]) else {
        unreachable!("only a line–circle pair has two roots to tie")
    };
    let along = (up.0 - foot.0) * (rho - foot.0) + (up.1 - foot.1) * (h - foot.1);
    // `det` is certified positive, so the roots stand apart and `reach`
    // divides.
    let reach = Vec3::new(up.0 - foot.0, up.1 - foot.1, T::zero()).norm();
    match decide("offset_axial_branch_side", Margin::of(along / reach), band) {
        Ok(Sign::Positive) => Ok(Some(up)),
        Ok(Sign::Negative) => Ok(Some(down)),
        Ok(Sign::Zero) => Ok(None),
        Err(source) => Err(ReplaceFaceError::Escalated { source }),
    }
}

/// Two distinct roots the same distance from the corner being moved.
fn tie<T: Decide>(vertex: VertexKey) -> ReplaceFaceError<T> {
    ReplaceFaceError::TogetherAxialCorner {
        vertex,
        surfaces: 0,
        what: "two solutions stand the same distance from the corner being moved, so which one \
               the offset keeps is not determined",
    }
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
    old_span: (T, T),
    p_start: Point3<T>,
    charts: (&MovedChart<T>, &MovedChart<T>),
    frame: &Frame<T>,
    band: Band,
) -> Result<Curve3<T>, ReplaceFaceError<T>> {
    let (t0_old, _) = old_span;
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
    // or a cone's collinear-vertex ring, a sphere's latitude between
    // two cocircular arcs' walls, a cap plane split by a collinear
    // vertex, a full
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
                translate(ca.rigid.unwrap_or_else(|| {
                    unreachable!("a plane's or a cone's offset is a translation of its own kind")
                }))
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
                let off = tube_centre_offset(frame, *cc, *center, *major_radius);
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
                "a seam whose curve and surface have no exact offset together",
            )),
        };
    }

    // ---- A rim between a sphere or torus wall and a plane parallel
    // to the axis: the moved pair's own SECTION. ----
    //
    // The cap's stand-off is whatever the offset left it — zero on a
    // partial revolve's meridian cap at rest, or on a cavity cap
    // lifted back onto it, nonzero once a cap is offset off the axis
    // — and the section names the kind: a plane cuts a sphere in a
    // circle, great or small, and a torus in its two meridian circles
    // through the axis or its two spiric ovals beside it. The old
    // carrier supplies only what the section cannot: WHICH of the two
    // torus curves (the side of the plane's trace its midpoint stands
    // on) and the SENSE (its plane normal against the section's).
    let wall_and_cap = match (&ca.constraint, &cb.constraint) {
        (Constraint::Ball { .. } | Constraint::Torus { .. }, Constraint::AxisParallel { .. }) => {
            Some((ca, cb))
        }
        (Constraint::AxisParallel { .. }, Constraint::Ball { .. } | Constraint::Torus { .. }) => {
            Some((cb, ca))
        }
        _ => None,
    };
    if let Some((wall, cap)) = wall_and_cap {
        // Each refusal the section arm can return names its own cause.
        let section_refused = |error: geom_brep::SectionError| match error {
            geom_brep::SectionError::Escalated(source)
            | geom_brep::SectionError::Spiric(geom::SpiricInvalid::Escalated(source)) => {
                ReplaceFaceError::Escalated { source }
            }
            geom_brep::SectionError::RoutesToGeneralRung { .. }
            | geom_brep::SectionError::Spiric(geom::SpiricInvalid::NotTwoOvals) => refuse(
                "a rim whose moved cap stands at or past the torus wall's inner equator, or \
                 out of its reach — the section there is a node, one folded loop or \
                 nothing, not two ovals",
            ),
            geom_brep::SectionError::Spiric(geom::SpiricInvalid::ThroughAxis) => refuse(
                "a rim whose moved cap the kernel reads both off the torus wall's axis and \
                 through it",
            ),
            _ => refuse(
                "a rim whose moved wall is no longer a ring torus or a sphere a cap can be cut \
                 against",
            ),
        };
        // The section's curves, with the torus's two named by the side
        // of the cap's trace of the axis each stands on: `plus` on the
        // `+a × n` side, for the torus's own axis `a` and the cap's
        // normal `n` (the section's documented placement).
        let (plus, minus) = match &wall.new {
            Surface::Sphere { .. } => {
                match geom_brep::plane_sphere_section(&cap.new, &wall.new, band)
                    .map_err(section_refused)?
                {
                    geom_brep::PlaneSphereSection::Circle(circle) => (circle, None),
                    geom_brep::PlaneSphereSection::TangentPoint(_)
                    | geom_brep::PlaneSphereSection::Empty => {
                        return Err(refuse(
                            "a cap standing tangent to or beyond its sphere wall, whose \
                             section leaves no rim circle",
                        ));
                    }
                }
            }
            _ => match geom_brep::plane_torus_section(&cap.new, &wall.new, frame.extent, band)
                .map_err(section_refused)?
            {
                geom_brep::PlaneTorusSection::MeridianCircles { c1, c2 } => (c2, Some(c1)),
                geom_brep::PlaneTorusSection::SpiricOvals { s1, s2 } => (s1, Some(s2)),
                geom_brep::PlaneTorusSection::ConcentricCircles { .. }
                | geom_brep::PlaneTorusSection::TangentCircle(_)
                | geom_brep::PlaneTorusSection::Empty => {
                    unreachable!(
                        "{edge:?}: a plane classified parallel to the axis is not normal to it"
                    )
                }
            },
        };
        // Which curve: a sphere's one circle straddles the trace and
        // needs no choosing; of a torus's two, the one the OLD rim's
        // midpoint stands beside — its offset along `a × n` from the
        // torus centre, a metre length, no lever (`±ρ` of the midpoint
        // on a meridian rim, and never less than `R − r` in size).
        let chosen = match (minus, &wall.new, &cap.new) {
            (None, ..) => plus,
            (Some(minus), Surface::Torus { center, axis, .. }, Surface::Plane { normal, .. }) => {
                let q_mid = old.mid_point(old_span.0, old_span.1);
                let side = axis.cross(*normal).dot(q_mid - *center);
                match decide("offset_axial_rim_side", Margin::of(side), band) {
                    Ok(Sign::Positive) => plus,
                    Ok(Sign::Negative) => minus,
                    Ok(Sign::Zero) => {
                        return Err(refuse(
                            "a torus rim whose old midpoint stands on the cap's own trace of \
                             the axis, so neither section curve is named",
                        ));
                    }
                    Err(source) => return Err(ReplaceFaceError::Escalated { source }),
                }
            }
            (Some(_), ..) => {
                unreachable!("{edge:?}: two section curves are a plane × torus pair's")
            }
        };
        // The sense: both curves lie in planes parallel to the cap, and
        // each runs counterclockwise about its own plane normal (a
        // circle's `axis`, a spiric's `u_ref`), so the cosine between
        // the two normals says whether the parameter runs the old way.
        // A cosine of unit vectors, levered at the body's extent.
        let normal_of = |c: &Curve3<T>| match c {
            Curve3::Circle { axis, .. } | Curve3::Ellipse { axis, .. } => Some(axis.normalize()),
            Curve3::Spiric { u_ref, .. } => Some(u_ref.normalize()),
            Curve3::Line { .. } | Curve3::Nurbs(_) => None,
        };
        let Some(old_normal) = normal_of(old) else {
            return Err(refuse(
                "a rim between a curved wall and a plane parallel to the axis whose curve is not \
                 a flat curve with a direction to carry",
            ));
        };
        let new_normal = normal_of(&chosen)
            .unwrap_or_else(|| unreachable!("{edge:?}: a section curve is a circle or a spiric"));
        return match decide(
            "offset_axial_rim_sense",
            Margin::levered(old_normal.dot(new_normal), frame.extent),
            band,
        ) {
            Ok(Sign::Positive) => Ok(chosen),
            Ok(Sign::Negative) => Ok(chosen
                .reversed()
                .unwrap_or_else(|| unreachable!("{edge:?}: a closed-form carrier reverses"))),
            Ok(Sign::Zero) => Err(refuse(
                "a rim whose old curve's plane is not parallel to its cap's, so its direction \
                 does not carry over",
            )),
            Err(source) => Err(ReplaceFaceError::Escalated { source }),
        };
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
            // on a quarter-turn conical wedge: the two ends come back
            // 0.64 mm apart and the door says `TogetherEdgeDisagreement`;
            // at a 1/12 turn the ends read backwards first and the
            // window refuses (`TogetherAxialEdge`, no forward window)
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
            "an edge between two different surfaces whose curve is neither a line nor a circle",
        )),
    }
}

/// A circle centre's distance from the tube-centre circle `(ρ, h) =
/// (R, h_c)` in the meridian half-plane — the comparand a torus seam's
/// meridian posture is decided on (`offset_axial_seam_meridian`).
fn tube_centre_offset<T: Real>(
    frame: &Frame<T>,
    centre: Point3<T>,
    tube_centre: Point3<T>,
    major_radius: T,
) -> T {
    Vec3::new(
        frame.radial(centre).norm() - major_radius,
        frame.station(centre) - frame.station(tube_centre),
        T::zero(),
    )
    .norm()
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
/// **One rule for every carrier kind**, keyed on the NEW carrier alone.
/// The old point `q` is read onto the new carrier first, near its old
/// parameter — on a carrier whose frame the door carried (a line, a
/// latitude circle, a seam) that is the old parameter itself, turn and
/// all, and on one minted afresh (a section curve, which may change
/// kind) it is the old point's own place in the new parameter. The
/// moved point `p` is then read from that anchor — on a circle as the
/// angle between the two points' rays about its centre, on a spiric
/// within a half turn of it — so what is re-derived is only the
/// motion; which way round the two ends make a window is
/// [`forward_window`]'s.
///
/// **The first read starts on the near half of the turn.** A periodic
/// read is an `atan2` about a guess, cut opposite it, and a fresh
/// carrier's frame can put the old parameter's point opposite `q` —
/// where an interval read straddles the cut and is poison. So the
/// guess is the old parameter or the one a half turn on, whichever
/// names the point nearer `q` (`offset_axial_edge_anchor`, a difference
/// of metre distances). An equal pair keeps the old parameter, as `q`
/// then stands a quarter turn from both and neither read is near its
/// cut; an undecided one escalates.
fn param_on<T: Decide>(
    carrier: &Curve3<T>,
    t_old: T,
    q: Point3<T>,
    p: Point3<T>,
    edge: EdgeKey,
    band: Band,
) -> Result<T, ReplaceFaceError<T>> {
    let unread = || ReplaceFaceError::TogetherAxialEdge {
        edge,
        what: "its kind of curve has no exact offset here",
    };
    let guess = match carrier {
        Curve3::Circle { .. } | Curve3::Spiric { .. } => {
            let across = t_old + T::pi();
            let lead = carrier.eval(across).distance(q) - carrier.eval(t_old).distance(q);
            match decide("offset_axial_edge_anchor", Margin::of(lead), band) {
                Ok(Sign::Negative) => across,
                Ok(Sign::Zero | Sign::Positive) => t_old,
                Err(source) => return Err(ReplaceFaceError::Escalated { source }),
            }
        }
        Curve3::Line { .. } | Curve3::Ellipse { .. } | Curve3::Nurbs(_) => t_old,
    };
    let read = carrier.param_near(q, guess).ok_or_else(unread)?;
    // On a carried frame the read IS the old parameter, and a second
    // evaluation at it would only add the carrier's own rounding (a
    // circle's `sin π`), so a read decided equal to it is it — levered
    // at the speed's ceiling, so the two are one point in metres.
    let anchor = match param_speeds(carrier) {
        Some((_, ceiling)) => match decide(
            "offset_axial_edge_carried",
            Margin::levered(read - t_old, ceiling),
            band,
        ) {
            Ok(Sign::Zero) => t_old,
            Ok(_) => read,
            Err(source) => return Err(ReplaceFaceError::Escalated { source }),
        },
        None => read,
    };
    let t = match carrier {
        // A circle's motion is the angle between the two points' rays
        // about its own centre, read off the points themselves.
        Curve3::Circle { center, axis, .. } => {
            let n = axis.normalize();
            let ray = |x: Point3<T>| {
                let v = x - *center;
                v - n * v.dot(n)
            };
            let (a, b) = (ray(q), ray(p));
            anchor + a.cross(b).dot(n).atan2(a.dot(b))
        }
        _ => carrier.param_near(p, anchor).ok_or_else(unread)?,
    };
    let gap = carrier.eval(t).distance(p);
    match decide("offset_axial_edge_agreement", Margin::of(gap), band) {
        Ok(Sign::Zero) => Ok(t),
        Ok(_) => Err(ReplaceFaceError::TogetherEdgeDisagreement { edge, gap }),
        Err(source) => Err(ReplaceFaceError::Escalated { source }),
    }
}

/// The end of a moved edge's window, on the turn nearest the old one.
///
/// [`param_on`] reads each end within a half turn of its own anchor, so
/// on a periodic carrier the two reads can land a period apart — a rim
/// between a torus's two equators has one end on the inner equator,
/// where a spiric's minor angle is `±π` by the sign of a zero. The
/// carrier runs the old one's way (the door carries the frame, or the
/// rim mint decides the sense), so the window keeps the old one's turn:
/// its span is taken a whole number of periods from the read one,
/// nearest the old span ([`keep_turn`], the rule an arc's re-authored
/// sweep keeps in [`reauthor`]). A read span already nearest the old
/// one is returned as read. The span is then DECIDED forward — an arc
/// of the carrier in metres, levered at the speed's floor, so a span
/// decided forward is one in metres — and one that cannot be told from
/// zero is no edge.
fn forward_window<T: Decide>(
    carrier: &Curve3<T>,
    old_span: (T, T),
    t0: T,
    t1: T,
    edge: EdgeKey,
    band: Band,
) -> Result<T, ReplaceFaceError<T>> {
    let speeds = param_speeds(carrier);
    let t1 = match speeds {
        Some((_, ceiling)) => {
            let old = old_span.1 - old_span.0;
            let kept = keep_turn(old, t1 - t0);
            // Zero or a whole number of turns, so never near the band.
            match decide(
                "offset_axial_edge_turn",
                Margin::levered(kept - (t1 - t0), ceiling),
                band,
            ) {
                Ok(Sign::Zero) => t1,
                Ok(_) => t0 + kept,
                Err(source) => return Err(ReplaceFaceError::Escalated { source }),
            }
        }
        None => t1,
    };
    let span = t1 - t0;
    let margin = match speeds {
        Some((floor, _)) => Margin::levered(span, floor),
        None => Margin::of(span),
    };
    match decide("offset_axial_edge_window", margin, band) {
        Ok(Sign::Positive) => Ok(t1),
        Ok(Sign::Zero | Sign::Negative) => Err(ReplaceFaceError::TogetherAxialEdge {
            edge,
            what: "an edge whose moved ends read as one point of its curve, or in reverse, so it \
                   has no forward span left",
        }),
        Err(source) => Err(ReplaceFaceError::Escalated { source }),
    }
}

/// A periodic carrier's speed bounds `(floor, ceiling)` in metres per
/// radian — the one spelling of the parameter-to-metre lever per kind.
/// A circle's is its radius both ways; a spiric's runs from its minor
/// radius to `r(R − r)/√((R − r)² − d²)` at the inner equator
/// ([`geom::spiric_rate_bounds`] over a whole period). A decision whose
/// Zero admits — two reads are one point — levers at the ceiling; one
/// whose Positive admits — a span is forward — at the floor, so each
/// errs conservative. `None` for a carrier whose parameter is a length.
fn param_speeds<T: Real>(carrier: &Curve3<T>) -> Option<(T, T)> {
    match carrier {
        Curve3::Circle { radius, .. } => Some((*radius, *radius)),
        Curve3::Spiric {
            major_radius,
            minor_radius,
            offset,
            ..
        } => {
            let (ceiling, _) = geom::spiric_rate_bounds(
                *minor_radius,
                *offset,
                (*major_radius - *minor_radius, *major_radius + *minor_radius),
                T::one(),
            );
            Some((*minor_radius, ceiling))
        }
        Curve3::Line { .. } | Curve3::Ellipse { .. } | Curve3::Nurbs(_) => None,
    }
}

/// **The turn-keeping rule**: a re-read sweep `read` of a periodic
/// carrier, taken the whole number of turns from it that lands nearest
/// the `old` sweep it replaces — the moved edge keeps the old one's
/// side of every cut. Shared by [`forward_window`] and an arc's
/// re-authored sweep in [`reauthor`].
fn keep_turn<T: Real>(old: T, read: T) -> T {
    old + (read - old).reduce_periodic_centred(T::tau())
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
        // The double cone's elevation about the frame's own axis line
        // (`frame.dir` is the cone's axis up to sign, and the
        // double-cone reading is symmetric in it).
        Surface::Cone {
            apex, half_angle, ..
        } => geom_brep::cone_elevation(*apex, frame.dir, *half_angle, None, p),
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

/// A mapped description re-authored in its own sketch plane from the
/// endpoints the corner solves put it between.
///
/// A SEGMENT is written afresh, whole, between the two points: a LINE
/// takes them, and an ARC takes them, the moved carrier itself (its
/// centre and radius), and the included angle the points subtend at
/// that centre — the offset of a meridian arc is concentric, so the
/// centre is the datum that does not move and the sweep is what the
/// endpoints say it is, on the turn of the edge it replaces: the
/// subtended angle is read nearest the turn that edge covers (the
/// segment's sweep times its range's span), so a half turn (a
/// pole-to-pole meridian) keeps its side of the atan2 cut. A POINT's
/// trajectory — extruded along a vector, or revolved about an axis —
/// is the same trajectory of the moved point: the vector and the axis
/// are the operand's own conventional data and are carried. A revolved
/// point's axis is this door's axis, so its rotation of the moved
/// corner IS the moved latitude circle; what an offset can move is
/// where on that circle each end stands. A moved meridian cap stops
/// containing the axis and turns the corner on it out of its old
/// sketch plane, so each end's out-of-plane coordinate — a length — is
/// decided: an end still in its plane keeps its azimuth, and an end
/// turned out of it moves its end of the sweep range by the turn over
/// the angle, the placement staying as built. A start turned onto its
/// own azimuth and still out of the plane — a sketch plane that does
/// not contain the axis — refuses typed. Refuses also an arc
/// whose moved carrier is no circle to subtend at.
fn reauthor<T: Decide>(
    mapped: geom_brep::MappedCurve<T>,
    carrier: &Curve3<T>,
    ends: (Point3<T>, Point3<T>),
    edge: EdgeKey,
    band: Band,
) -> Result<geom_brep::MappedCurve<T>, ReplaceFaceError<T>> {
    let refuse = |what: &'static str| ReplaceFaceError::TogetherAxialEdge { edge, what };
    let (p_start, p_end) = ends;
    let range = mapped.range;
    Ok(match mapped.source {
        geom_brep::MappedSource::PlacedSegment { segment, place } => {
            let inv = place.inverse();
            let flat = |p: Point3<T>| {
                let q = inv.transform_point(p);
                geom_core::Point2::new(q.x, q.y)
            };
            let (a, b) = (flat(p_start), flat(p_end));
            geom_brep::MappedCurve::whole(geom_brep::MappedSource::PlacedSegment {
                segment: match segment {
                    geom_brep::SketchSegment::Line { .. } => {
                        geom_brep::SketchSegment::Line { a, b }
                    }
                    geom_brep::SketchSegment::Arc { arc: was, .. } => {
                        let Curve3::Circle { center, radius, .. } = carrier else {
                            return Err(refuse(
                                "a sketch arc with no moved circle to be redrawn about",
                            ));
                        };
                        let centre = flat(*center);
                        let (u, v) = (a - centre, b - centre);
                        let covered = range.span().map_or(was.sweep, |span| was.sweep * span);
                        geom_brep::SketchSegment::Arc {
                            a,
                            b,
                            arc: Arc2 {
                                centre,
                                radius: *radius,
                                sweep: keep_turn(covered, u.perp_dot(v).atan2(u.dot(v))),
                            },
                        }
                    }
                },
                place,
            })
        }
        geom_brep::MappedSource::ExtrudedPoint { place, vec, .. } => {
            // Each moved end's station `u` along the whole strut, `p =
            // place(point) + vec·u`: its height off the sketch plane
            // over the vector's own. An end still at its station keeps
            // it, decided on the height it would be off by — a length;
            // a moved one shifts its end of the range by that height
            // over the rise.
            let inv = place.inverse();
            let rise = inv.transform_vec(vec).z;
            match decide("offset_axial_reauthor_rise", Margin::of(rise), band) {
                Ok(Sign::Positive | Sign::Negative) => {}
                Ok(Sign::Zero) => {
                    return Err(refuse(
                        "an extruded point whose extrusion vector lies in its own sketch \
                         plane, so no station along it is read",
                    ));
                }
                Err(source) => return Err(ReplaceFaceError::Escalated { source }),
            }
            let shift = |name: &'static str, p: Point3<T>, station: Option<T>| {
                let height = inv.transform_point(p).z;
                let off = match station {
                    None => height,
                    Some(u) => height - rise * u,
                };
                match decide(name, Margin::of(off), band) {
                    Ok(Sign::Zero) => Ok(None),
                    Ok(_) => Ok(Some(off / rise)),
                    Err(source) => Err(ReplaceFaceError::Escalated { source }),
                }
            };
            let range = range.moved(
                shift(
                    "offset_axial_reauthor_extrude_start",
                    p_start,
                    range.start(),
                )?,
                shift(
                    "offset_axial_reauthor_extrude_end",
                    p_end,
                    Some(range.at(T::one())),
                )?,
            );
            let q = inv.transform_point(match range.start() {
                None => p_start,
                Some(u) => p_start - vec * u,
            });
            geom_brep::MappedCurve {
                source: geom_brep::MappedSource::ExtrudedPoint {
                    point: geom_core::Point2::new(q.x, q.y),
                    place,
                    vec,
                },
                range,
            }
        }
        geom_brep::MappedSource::RevolvedPoint {
            place,
            axis_origin,
            axis_dir,
            angle,
            ..
        } => {
            // The sketch plane turned `theta` about the axis: `place`
            // itself at a whole sweep's exact start. A moved start is
            // read on its own plane turned by the measured turn, and
            // its range moves by that turn over the angle.
            let plane = |theta: Option<T>| match theta {
                None => place,
                Some(theta) => {
                    geom_core::Affine3::rotation_about_axis(axis_origin, axis_dir, theta) * place
                }
            };
            let turn = |name, theta: Option<T>, s: T, moved: Point3<T>| {
                let old = mapped.eval(s);
                azimuth_turn(
                    name,
                    plane(theta),
                    (axis_origin, axis_dir),
                    old,
                    moved,
                    band,
                )
            };
            let start = range.start().map(|u| u * angle);
            let start_turn = turn("offset_axial_reauthor_plane", start, T::zero(), p_start)?;
            let end_turn = turn(
                "offset_axial_reauthor_end",
                Some(range.at(T::one()) * angle),
                T::one(),
                p_end,
            )?;
            let read_at = match (start, start_turn) {
                (start, None) => start,
                (None, Some(phi)) => Some(phi),
                (Some(start), Some(phi)) => Some(start + phi),
            };
            let range = range.moved(
                start_turn.map(|phi| phi / angle),
                end_turn.map(|phi| phi / angle),
            );
            let q = plane(read_at).inverse().transform_point(p_start);
            if start_turn.is_some() {
                match decide("offset_axial_reauthor_azimuth", Margin::of(q.z), band) {
                    Ok(Sign::Zero) => {}
                    Ok(_) => {
                        return Err(refuse(
                            "a revolved point's moved corner stands out of its family's sketch \
                             plane even turned to its own azimuth, so no rotation of the sketch \
                             point passes through it",
                        ));
                    }
                    Err(source) => return Err(ReplaceFaceError::Escalated { source }),
                }
            }
            geom_brep::MappedCurve {
                source: geom_brep::MappedSource::RevolvedPoint {
                    point: geom_core::Point2::new(q.x, q.y),
                    place,
                    axis_origin,
                    axis_dir,
                    angle,
                },
                range,
            }
        }
    })
}

/// How far about the axis a revolved point's end moved: `None` when the
/// moved corner still stands in that end's own sketch plane `plane`
/// (its out-of-plane coordinate, a length, decided under `name`), and
/// otherwise the signed turn about `axis` from the old corner's
/// azimuth to the moved one's.
fn azimuth_turn<T: Decide>(
    name: &'static str,
    plane: geom_core::Affine3<T>,
    (origin, dir): (Point3<T>, Vec3<T>),
    old: Point3<T>,
    moved: Point3<T>,
    band: Band,
) -> Result<Option<T>, ReplaceFaceError<T>> {
    let off = plane.inverse().transform_point(moved).z;
    match decide(name, Margin::of(off), band) {
        Ok(Sign::Zero) => Ok(None),
        Ok(_) => {
            let a = dir.normalize();
            let radial = |p: Point3<T>| {
                let v = p - origin;
                v - a * v.dot(a)
            };
            let (from, to) = (radial(old), radial(moved));
            Ok(Some(a.dot(from.cross(to)).atan2(from.dot(to))))
        }
        Err(source) => Err(ReplaceFaceError::Escalated { source }),
    }
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::test_support_fixtures::geometric_cube;

    /// The unit cube re-charted as a quarter-revolve wedge about `z`:
    /// `x = 0` and `y = 0` contain the axis, the caps are normal to it,
    /// and the `x = 1` and `y = 1` walls sit on one coaxial cylinder.
    fn quarter_wedge() -> Body<f64> {
        let cube = geometric_cube::<f64>(Tol::witness());
        let mut body = cube.body;
        let wall = body.add_surface(Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        });
        for side in [cube.mefs[2].face, cube.mefs[3].face] {
            body.get_face_mut(side).expect("a cube wall").surface = wall;
        }
        body
    }

    /// A torn record is not a verdict: a dangling point or surface on a
    /// face in scope panics naming the record, rather than reading as a
    /// silent `false` that would pick `shell`'s general branch.
    #[test]
    fn is_axial_panics_on_a_torn_record_rather_than_answering_not_axial() {
        use crate::entity::GeomRef;
        let band = Band::new(1e-9, 1e-8).unwrap();
        let wedge = quarter_wedge();
        assert!(
            matches!(is_axial(&wedge, band), Ok(true)),
            "the untorn wedge is axial"
        );

        let mut body = wedge.clone();
        let (vertex, _) = body.vertices().next().expect("a vertex");
        let dead = body.add_point(Point3::new(0.0, 0.0, 0.0));
        body.points.remove(dead);
        body.get_vertex_mut(vertex).unwrap().point = dead;
        let report = crate::surgery::tests::panic_message(std::panic::AssertUnwindSafe(|| {
            let _ = is_axial(&body, band);
        }));
        let premise = format!(
            "{}'s point names {}, which does not resolve",
            EntityId::Vertex(vertex),
            GeomRef::Point(dead)
        );
        assert!(report.contains(&premise), "a torn point: {report}");

        // The first curved chart is the seed, so tearing it is the case
        // a skip would answer from the next chart or as all-planar.
        let mut body = wedge;
        let (face, f) = body
            .faces()
            .find(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Cylinder { .. })))
            .map(|(k, f)| (k, f.surface))
            .expect("a wall on the cylinder");
        let dead = body.add_surface(Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        });
        body.surfaces.remove(dead);
        body.get_face_mut(face).unwrap().surface = dead;
        assert_ne!(f, dead);
        let report = crate::surgery::tests::panic_message(std::panic::AssertUnwindSafe(|| {
            let _ = is_axial(&body, band);
        }));
        let premise = format!(
            "{}'s surface names {}, which does not resolve",
            EntityId::Face(face),
            GeomRef::Surface(dead)
        );
        assert!(
            report.contains(&premise) && report.contains(crate::live::NAMES_ONLY_LIVE),
            "a torn surface: {report}"
        );
    }

    /// A body with no face names nothing that revolves: a definite
    /// `false`, not a refusal.
    #[test]
    fn is_axial_answers_false_on_a_faceless_body() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        assert!(matches!(is_axial(&Body::<f64>::new(), band), Ok(false)));
    }

    use geom_core::{Affine3, Bounds, Interval};

    /// A revolve rim at `at` over a whole turn — the placed point
    /// `at + (2, 2, 0)`, one metre off a `+z` axis through `at + (1, 2, 0)`
    /// — at the scalar `lift` builds.
    fn rim<T: Decide>(at: [f64; 3], lift: impl Fn(f64) -> T) -> geom_brep::MappedCurve<T> {
        let [x, y, z] = at;
        geom_brep::MappedCurve::whole(geom_brep::MappedSource::RevolvedPoint {
            point: geom_core::Point2::new(lift(2.0), lift(2.0)),
            place: Affine3::translation(Vec3::new(lift(x), lift(y), lift(z))),
            axis_origin: Point3::new(lift(1.0 + x), lift(2.0 + y), lift(z)),
            axis_dir: Vec3::new(lift(0.0), lift(0.0), lift(1.0)),
            angle: lift(core::f64::consts::TAU),
        })
    }

    /// A rim on a tilted placement: the sketch plane shifted to `at`
    /// and then turned 0.7 rad about `(0.2, 1, −0.4)` through the
    /// origin, so its frame is no coordinate frame; the axis runs
    /// through the sketch point `(0.5, 1)` along the in-plane `(1, 0.4)`,
    /// and the rim sweeps 1.9 rad about it.
    fn tilted_rim<T: Decide>(at: [f64; 3], lift: impl Fn(f64) -> T) -> geom_brep::MappedCurve<T> {
        let place = Affine3::rotation_about_axis(
            Point3::new(lift(0.0), lift(0.0), lift(0.0)),
            Vec3::new(lift(0.2), lift(1.0), lift(-0.4)),
            lift(0.7),
        ) * Affine3::translation(Vec3::new(lift(at[0]), lift(at[1]), lift(at[2])));
        geom_brep::MappedCurve::whole(geom_brep::MappedSource::RevolvedPoint {
            point: geom_core::Point2::new(lift(2.0), lift(2.0)),
            place,
            axis_origin: place.transform_point(Point3::new(lift(0.5), lift(1.0), lift(0.0))),
            axis_dir: place.transform_vec(Vec3::new(lift(1.0), lift(0.4), lift(0.0))),
            angle: lift(1.9),
        })
    }

    /// An `f64` rim lifted to `Interval` scalar by scalar, so every
    /// input is an exact point and the widths a row reads are the
    /// arithmetic's own.
    fn lifted(mapped: geom_brep::MappedCurve<f64>) -> geom_brep::MappedCurve<Interval> {
        let geom_brep::MappedCurve {
            source:
                geom_brep::MappedSource::RevolvedPoint {
                    point,
                    place,
                    axis_origin,
                    axis_dir,
                    angle,
                    ..
                },
            ..
        } = mapped
        else {
            panic!("a revolved point");
        };
        let iv = Interval::from_f64;
        let p3 = |p: Point3<f64>| Point3::new(iv(p.x), iv(p.y), iv(p.z));
        let v3 = |v: Vec3<f64>| Vec3::new(iv(v.x), iv(v.y), iv(v.z));
        geom_brep::MappedCurve::whole(geom_brep::MappedSource::RevolvedPoint {
            point: geom_core::Point2::new(iv(point.x), iv(point.y)),
            place: Affine3::from_parts(
                geom_core::Mat3::from_cols(
                    v3(place.linear.c0),
                    v3(place.linear.c1),
                    v3(place.linear.c2),
                ),
                v3(place.translation),
            ),
            axis_origin: p3(axis_origin),
            axis_dir: v3(axis_dir),
            angle: iv(angle),
        })
    }

    /// The parts of a revolved point, for the rows to rebuild main's
    /// composed reading from.
    struct Rim<T: Real> {
        point: geom_core::Point2<T>,
        place: Affine3<T>,
        axis_origin: Point3<T>,
        axis_dir: Vec3<T>,
        angle: T,
    }

    fn parts<T: Decide>(mapped: geom_brep::MappedCurve<T>) -> Rim<T> {
        let geom_brep::MappedCurve {
            source:
                geom_brep::MappedSource::RevolvedPoint {
                    point,
                    place,
                    axis_origin,
                    axis_dir,
                    angle,
                    ..
                },
            ..
        } = mapped
        else {
            panic!("a revolved point");
        };
        Rim {
            point,
            place,
            axis_origin,
            axis_dir,
            angle,
        }
    }

    /// The placed sketch point, read straight off the placement.
    fn placed<T: Decide>(rim: &Rim<T>) -> Point3<T> {
        rim.place
            .transform_point(Point3::new(rim.point.x, rim.point.y, T::zero()))
    }

    /// `mapped` re-authored between `p_start` and its own unmoved end
    /// sample.
    fn reauthored<T: Decide>(
        mapped: geom_brep::MappedCurve<T>,
        p_start: Point3<T>,
        band: Band,
    ) -> geom_brep::MappedCurve<T> {
        let carrier = Curve3::Line {
            origin: Point3::new(T::zero(), T::zero(), T::zero()),
            dir: Vec3::new(T::one(), T::zero(), T::zero()),
        };
        reauthor(
            mapped,
            &carrier,
            (p_start, mapped.eval(T::one())),
            EdgeKey::default(),
            band,
        )
        .expect("the rim re-authors")
    }

    /// `rim`'s placed point turned `theta` about its own axis — a start
    /// corner an offset moved round the axis.
    fn turned<T: Decide>(rim: &Rim<T>, theta: T) -> Point3<T> {
        Affine3::rotation_about_axis(rim.axis_origin, rim.axis_dir, theta)
            .transform_point(placed(rim))
    }

    /// The turn main read off a moved start: the azimuth about the axis
    /// from the radial of its start sample — the placed point under the
    /// rotation by `0·angle` — to the corner's.
    fn main_turn<T: Decide>(rim: &Rim<T>, corner: Point3<T>) -> T {
        let a = rim.axis_dir.normalize();
        let radial = |p: Point3<T>| {
            let v = p - rim.axis_origin;
            v - a * v.dot(a)
        };
        let start =
            Affine3::rotation_about_axis(rim.axis_origin, rim.axis_dir, T::zero() * rim.angle)
                .transform_point(placed(rim));
        let (from, to) = (radial(start), radial(corner));
        a.dot(from.cross(to)).atan2(from.dot(to))
    }

    /// Main's reading of a start turned by `phi`: the turn composed into
    /// the placement and the corner read back through that composite.
    fn composed_reading<T: Decide>(rim: &Rim<T>, phi: T, p: Point3<T>) -> (Point3<T>, Affine3<T>) {
        let plane = Affine3::rotation_about_axis(rim.axis_origin, rim.axis_dir, phi) * rim.place;
        (plane.inverse().transform_point(p), plane)
    }

    fn width(e: Interval) -> f64 {
        e.hi() - e.lo()
    }

    /// The band the f64 and exact-corner rows decide at.
    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }
    /// A band the tilted far placement's turned corner fits inside at
    /// `Interval`: read back through the composed placement a thousand
    /// metres out, its out-of-plane coordinate is some 1e-6 wide, on
    /// main's composite as on this one.
    fn wide() -> Band {
        Band::new(1e-5, 1e-4).unwrap()
    }

    const NEAR: [f64; 3] = [0.0, 0.0, 3.0];
    const FAR: [f64; 3] = [1000.0, -700.0, 300.0];
    const FARTHER: [f64; 3] = [1.0e5, 3.0e4, -2.0e4];

    /// **An unmoved start stores what its placement reads, and nothing
    /// else.** A rim whose start corner did not move keeps its whole
    /// range, and its sketch point is `place⁻¹` of the corner — at
    /// `Interval` bit for bit what that one map gives, so an exact
    /// corner on a translated placement stores width 0, near and a
    /// thousand metres out, and a tilted placement's corner pays only
    /// its own inverse. Turning the corner back through `I − R(−0)`
    /// first stored an ulp of the coordinates each side (6.7e-16 near,
    /// 2.3e-13 far), and through `R(0)·place` the rotation's diagonal
    /// enclosure (3.1e-14 near, 1.1e-11 far).
    #[test]
    fn an_unmoved_revolved_point_stores_what_its_placement_reads() {
        for at in [NEAR, FAR] {
            for (shape, mapped) in [
                ("translated", rim(at, Interval::from_f64)),
                ("tilted", tilted_rim(at, Interval::from_f64)),
            ] {
                let rim = parts(mapped);
                let corner = placed(&rim);
                let geom_brep::MappedCurve {
                    source: geom_brep::MappedSource::RevolvedPoint { point, angle, .. },
                    range,
                } = reauthored(mapped, corner, band())
                else {
                    panic!("a revolved point re-authors as one");
                };
                let read = rim.place.inverse().transform_point(corner);
                let bits = |e: Interval| (e.lo().to_bits(), e.hi().to_bits());
                assert_eq!(
                    (bits(point.x), bits(point.y)),
                    (bits(read.x), bits(read.y)),
                    "{shape} at {at:?}: the unmoved start stored {point:?}, not place⁻¹ of \
                     its corner {read:?}"
                );
                assert!(
                    range.start().is_none() && range.span().is_none(),
                    "{shape} at {at:?}: an unmoved rim's range moved: {range:?}"
                );
                assert_eq!(
                    bits(angle),
                    bits(Interval::from_f64(if shape == "tilted" {
                        1.9
                    } else {
                        core::f64::consts::TAU
                    })),
                    "{shape} at {at:?}: an unmoved rim's angle moved"
                );
                if shape == "translated" {
                    let stored = width(point.x).max(width(point.y));
                    assert!(
                        stored == 0.0,
                        "at {at:?} the exact corner re-authored to a sketch point {stored:e} wide"
                    );
                }
            }
        }
    }

    /// At `f64` an unmoved rim re-authors to its own data bit for bit:
    /// the whole range, the angle, and the point read straight off the
    /// placement.
    #[test]
    fn an_unmoved_revolved_point_reauthors_bit_for_bit_at_f64() {
        for at in [NEAR, FAR] {
            let mapped = rim(at, |x| x);
            let geom_brep::MappedCurve {
                source: geom_brep::MappedSource::RevolvedPoint { point, angle, .. },
                range,
            } = reauthored(mapped, placed(&parts(mapped)), band())
            else {
                panic!("a revolved point re-authors as one");
            };
            assert_eq!(
                (point.x.to_bits(), point.y.to_bits()),
                (2.0f64.to_bits(), 2.0f64.to_bits()),
                "at {at:?} the re-authored point moved: {point:?}"
            );
            assert_eq!(
                (angle.to_bits(), range.start(), range.span()),
                (core::f64::consts::TAU.to_bits(), None, None),
                "at {at:?} the re-authored sweep moved: {angle:e} over {range:?}"
            );
        }
    }

    /// **A turned start on a tilted far placement lands as close to its
    /// corner as composing the turn into the placement did.** At `f64`,
    /// over 200 turns in `[−3, 3]`, the re-authored description's start
    /// sample sits within 1.25× of main's worst distance, plus one ulp
    /// of the coordinates: main read the corner back through `(R(φ)·place)⁻¹`
    /// and placed it again through the same composite, rebuilt here from
    /// `Affine3` alone. The ulp is the start sample's own form, which
    /// applies `place` and then the rotation rather than their
    /// composite; the distances are a few ulps of the coordinates, so
    /// one ulp is a ratio of 1.5 at 1e5. Turning the corner back through
    /// `I − R(−θ)` and reading it through `place⁻¹` landed 2.5–3×
    /// farther.
    #[test]
    fn a_turned_start_on_a_tilted_far_placement_lands_as_close_as_composing() {
        let mut worst = Vec::new();
        for at in [FAR, FARTHER] {
            let mapped = tilted_rim(at, |x| x);
            let rim = parts(mapped);
            let (mut ours, mut main) = (0.0f64, 0.0f64);
            for k in 0..200 {
                let theta = -3.0 + 6.0 * f64::from(k) / 199.0;
                let corner = turned(&rim, theta);
                let start = reauthored(mapped, corner, band()).eval(0.0);
                ours = ours.max((start - corner).norm_inf());
                let (q, plane) = composed_reading(&rim, main_turn(&rim, corner), corner);
                main = main
                    .max((plane.transform_point(Point3::new(q.x, q.y, 0.0)) - corner).norm_inf());
            }
            println!("at {at:?}: worst start-sample distance {ours:e}, composed {main:e}");
            worst.push((at, ours, main));
        }
        for (at, ours, main) in worst {
            let scale = at.iter().fold(0.0f64, |m, c| m.max(c.abs()));
            let ulp = scale.next_up() - scale;
            assert!(
                ours <= 1.25 * main + ulp,
                "at {at:?} a turned start re-authors {ours:e} from its corner, over 1.25× \
                 the composed placement's {main:e} plus one ulp of the coordinates ({ulp:e})"
            );
        }
    }

    /// **At `Interval` a turned start stores no wider than composing the
    /// turn into the placement did.** On the tilted far placement, at
    /// three turns, the stored sketch point is no wider than main's
    /// composed reading of the same corner, with main's turn rebuilt
    /// here as main measured it. Every input is an exact point; at 1e5
    /// the composite's reading is some 7e-3 wide out of the plane, past
    /// any band a decision there could take, on main's spelling as on
    /// this one.
    #[test]
    fn a_turned_start_on_a_tilted_far_placement_stores_no_wider_than_composing() {
        let iv = Interval::from_f64;
        for at in [FAR] {
            let mapped = lifted(tilted_rim(at, |x| x));
            let rim = parts(mapped);
            for theta in [0.3, 1.7, -1.1, -2.9] {
                let corner = turned(&rim, iv(theta));
                let geom_brep::MappedCurve {
                    source: geom_brep::MappedSource::RevolvedPoint { point, .. },
                    ..
                } = reauthored(mapped, corner, wide())
                else {
                    panic!("a revolved point re-authors as one");
                };
                let (q, _) = composed_reading(&rim, main_turn(&rim, corner), corner);
                let ours = width(point.x).max(width(point.y));
                let main = width(q.x).max(width(q.y));
                println!("at {at:?}, turn {theta}: stored width {ours:e}, composed {main:e}");
                assert!(
                    ours <= main,
                    "at {at:?} a start turned {theta} stored a point {ours:e} wide, over the \
                     composed placement's {main:e}"
                );
            }
        }
    }

    /// **A restricted arc re-authors on the turn its edge covers.** A
    /// 1.9π sketch arc restricted to `[0.1, 0.35]` covers 0.475π; offset
    /// concentrically, its moved ends subtend that same 0.475π at the
    /// centre. The re-authored segment is whole and turns 0.475π, the
    /// subtended angle read nearest the covered turn. Read nearest the
    /// whole 1.9π instead, it would turn 2.475π — more than a full turn,
    /// and off the moved end.
    #[test]
    fn a_restricted_arc_re_authors_on_its_covered_turn() {
        use core::f64::consts::PI;
        let on = |r: f64, theta: f64| geom_core::Point2::new(r * theta.cos(), r * theta.sin());
        let sweep = 1.9 * PI;
        let whole = geom_brep::MappedCurve::whole(geom_brep::MappedSource::PlacedSegment {
            segment: geom_brep::SketchSegment::Arc {
                a: on(1.0, 0.0),
                b: on(1.0, sweep),
                arc: Arc2 {
                    centre: geom_core::Point2::new(0.0, 0.0),
                    radius: 1.0,
                    sweep,
                },
            },
            place: Affine3::identity(),
        });
        let edge = whole.restrict(0.1, 0.35);
        let (t0, t1) = (0.1 * sweep, 0.35 * sweep);
        let lift = |p: geom_core::Point2<f64>| Point3::new(p.x, p.y, 0.0);
        let (p_start, p_end) = (lift(on(1.5, t0)), lift(on(1.5, t1)));
        let carrier = Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.5,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let moved = reauthor(edge, &carrier, (p_start, p_end), EdgeKey::default(), band())
            .expect("a concentric offset of a sketch arc re-authors");
        assert!(moved.range.is_whole(), "the re-authored segment is whole");
        let geom_brep::MappedSource::PlacedSegment {
            segment: geom_brep::SketchSegment::Arc { arc, .. },
            ..
        } = moved.source
        else {
            panic!("a placed arc re-authors as one");
        };
        let covered = 0.25 * sweep;
        assert!(
            (arc.sweep - covered).abs() < 1e-12,
            "the re-authored sweep is {} rad, not the covered {covered} rad",
            arc.sweep
        );
        for (s, want) in [
            (0.0, p_start),
            (0.5, lift(on(1.5, 0.5 * (t0 + t1)))),
            (1.0, p_end),
        ] {
            let got = moved.eval(s);
            assert!(
                got.distance(want) < 1e-12,
                "the re-authored arc at s = {s} is {got:?}, off the moved edge's {want:?}"
            );
        }
    }
}

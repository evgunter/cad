//! The revolve operation: a validated profile swept about an
//! in-sketch-plane axis into a closed solid (M2 PR 5).
//!
//! # Conventions this module owns (normative, stated once)
//!
//! - **Axis.** [`RevolveAxis`] is a line **in sketch coordinates**:
//!   `origin + dir·t`. With `d̂ = dir/|dir|` and `ê_r = (d̂.y, −d̂.x)`,
//!   every sketch point gets a signed **radial coordinate**
//!   `r(p) = (p − origin)·ê_r = (p − origin).perp_dot(d̂)` and an axial
//!   coordinate `z(p) = (p − origin)·d̂`. The profile must lie in the
//!   closed half-plane `r ≥ 0` (the half-plane check: a definite `r < 0`
//!   anywhere is a typed error; the sliver band is a typed error too —
//!   micro-radius revolve is a genuine sliver).
//! - **Angle.** [`Revolution::Partial`] carries a **signed** angle θ in
//!   radians, right-hand rule about the world axis direction
//!   `a₃ = place·d̂`; `|θ|` must classify definitely inside `(0, 2π)`
//!   (zero/sliver ⇒ typed error; `|θ| ≥ 2π` at tolerance ⇒ typed error —
//!   an exactly-full revolve must say [`Revolution::Full`], which sweeps
//!   `+2π`). Angular margins are metered in meters through the profile's
//!   maximum radial extent (D4 ¶1 — the lever arm is named).
//! - **Winding.** For θ > 0 the rotation carries the profile toward the
//!   `−n` side of the sketch plane (velocity `a₃ × radial` at angle 0),
//!   so the **start cap** (on the sketch plane) is outward-`+n` and
//!   carries the profile's canonical winding; the sweep therefore
//!   traverses the chains **reversed** for θ > 0, forward for θ < 0.
//!   `θ > 0` is this verb's answer to the question extrude answers
//!   with `w·n < 0`; both feed the one `swept::swept_segments`, which
//!   is where the involution itself lives. [`Revolution::Full`] sweeps +2π and reverses
//!   likewise.
//! - **The shared azimuthal frame.** Every revolution surface minted by
//!   one revolve call uses `axis = +a₃` and `u_ref = u₃ = place·ê_r`,
//!   anchored on the placed axis line — so the `u = 0` iso-curve of
//!   every wall is the **angle-0 meridian half-plane**, which is where
//!   the profile sits. A full revolve's surviving meridian edges are
//!   therefore exactly the `u = 0` iso-curves: they re-describe as
//!   [`geom_brep::EdgeGeometry::Seam`] `{ surface }` — except meridians
//!   of **plane** walls (a segment ⊥ axis sweeps a plane annulus; a
//!   plane chart is not periodic, so `Seam` is malformed on it and the
//!   edge honestly keeps its conventional `MappedCurve` description —
//!   the same-surface split is definitely smooth, which tier 3's
//!   prefer-intrinsic enforcement permits by the D2 conventional split).
//!   Latitude rim **carriers** take the θ-signed axis (forward
//!   intervals) with `u_ref` from the rim's own start point.
//! - **Full period is definitionally the identity.** For
//!   [`Revolution::Full`], rotated-copy coordinates are the original
//!   coordinates **bitwise** and the copied chain's placement is the
//!   sketch placement itself (a 2π rotation is the identity by
//!   definition, not by trigonometry) — seam coincidences are exact,
//!   and the seam zip pairs entities by construction-record keys, never
//!   by geometric matching.
//! - **Axis contact (the ratified case split).** Radial classification
//!   is a named trilean per vertex: definitely zero ⇒ *on-axis*
//!   (special class), definitely negative ⇒ typed error, sliver band ⇒
//!   typed error. Partial revolves treat on-axis vertices/edges as
//!   ordinary boundary entities shared by the wedge caps (no strut, no
//!   wall). Full revolves OMIT on-axis edges (they sweep to nothing):
//!   a profile whose axis contact is one contiguous run of on-axis
//!   line segments opens into a **wire** whose tips become poles/
//!   apexes; contact at an isolated vertex, or in two or more runs,
//!   revolves to a non-manifold solid and is refused (D1). Holes are
//!   supported for partial revolves (extrude-shaped) AND for full
//!   revolves: a full revolve of a holed profile is DEFINED as
//!   `revolve(outer) − revolve(hole-as-outer)` (OFFSET-DESIGN O4) and
//!   executed as the degenerate no-crossing arm — each hole revolves
//!   as its own solid of revolution and its reversed boundary is
//!   inserted as a cavity through the shared void-insertion door
//!   ([`topo::insert_void`], DESIGN's M2 cavity invariant), with
//!   strict containment certified FROM the profile's own validation:
//!   a hole is strictly inside the outer loop with decided clearance
//!   and containment margins, and revolution about the shared axis
//!   carries strict 2-D containment to strict 3-D containment
//!   verbatim. No SSI, no crossing census, no 3-D box test runs on
//!   that path (a torus-walled cavity could not even enter the
//!   crossing pipeline — its operand gate refuses tori).
//! - **Cone charts.** A cone wall is minted with `axis = +a₃`
//!   regardless of which nappe the face occupies: the implicit
//!   residual/gradient machinery is nappe-symmetric, so the axis sign
//!   is pure chart convention.
//!
//! # What a revolve stores (the D2 story, applied)
//!
//! Meridian chain edges are `MappedCurve::PlacedSegment` (start chain at
//! the sketch placement, end chain at the rotated placement); latitude
//! edges are `MappedCurve::RevolvedPoint`. After all surfaces exist:
//! wedge-cap meridians upgrade to `Intersection { cap, wall, witness }`,
//! definitely-transverse latitude rims upgrade to
//! `Intersection { wall₁, wall₂, witness }` (witness = carrier
//! mid-parameter point — the S2 witness contract; for a full-period rim
//! that is the start point's antipode), a partial revolve's on-axis
//! edges upgrade to `Intersection { start cap, end cap }` when the caps
//! are definitely transverse (θ ≠ π), and a full revolve's meridians
//! become `Seam` on periodic walls. Cosurface runs (collinear segments,
//! same-carrier tangent arcs) share one surface key, decided for the
//! whole loop — including the wrap pair — before any wall is minted
//! (the PR 4 SHOULD-1 lesson).
//!
//! # K-telemetry
//!
//! Every topology-determining comparison goes through the named
//! [`decide`] funnel, which delegates to the unified recorder funnel
//! `geom_core::k_stats::decide` (M2 PR 7).

mod axis;
mod chain;
mod full;
mod partial;
mod surfaces;
pub mod tube;
mod upgrade;

use core::fmt;

use geom_brep::NewellError;
use geom_core::{Band, BandError, Decide, Indeterminate, Margin, Point2, Real, Sign, Tol, Vec2};
use profile::ValidatedProfile;
use topo::readback::{Pose, ReadbackError, face_pose};
use topo::{Body, EdgeKey, EulerOpError, FaceKey, ShellKey, SolidKey, VertexKey};

use crate::swept::decide;

/// The predicate names a revolve's cosurface decision reports under
/// (the revolution walls: planes, cylinders and cones from lines,
/// spheres and tori from arcs).
pub(super) const WALL_COSURFACE: crate::swept::CosurfaceNames = crate::swept::CosurfaceNames {
    lines: "wall_lines_cosurface",
    arcs: "wall_arcs_cosurface",
};

/// The revolve axis: a line in **sketch coordinates** (module docs).
/// The profile must lie in the closed half-plane `r ≥ 0`, where `r` is
/// the signed radial coordinate `(p − origin).perp_dot(dir/|dir|)`.
#[derive(Clone, Copy, Debug)]
pub struct RevolveAxis<T: Real> {
    /// A point on the axis, sketch-plane meters.
    pub origin: Point2<T>,
    /// The axis direction (any definitely nonzero vector; normalized
    /// internally — a zero/sliver direction is a typed error).
    pub dir: Vec2<T>,
}

/// How far to revolve — the operation's third input (module docs:
/// angle conventions, including why exactly-full must say `Full`).
#[derive(Clone, Copy, Debug)]
pub enum Revolution<T: Real> {
    /// The full revolution: sweeps exactly +2π (no wedge caps). A
    /// closed off-axis profile closes its seam through same-shell
    /// `kfmrh` plus the loopglue zip; an axis-touching profile sweeps
    /// as a two-band wire (see [`RevolvedKind::Full`]).
    Full,
    /// A partial revolution by the **signed** angle θ (radians,
    /// right-hand rule about the placed axis direction);
    /// `|θ|` must classify definitely inside `(0, 2π)`.
    Partial(T),
}

/// Everything [`revolve`] built, keyed (the `Extruded` key-bundle
/// shape): the body plus the handles downstream passes address it by.
///
/// Indexing convention: outer Vecs are per **canonical** loop (loop 0
/// the outer, then holes); inner Vecs are per canonical segment
/// (`walls`) or canonical vertex (`rims`).
/// `None` marks the axis-contact special classes: an on-axis segment
/// has no wall (partial: the edge is shared by the wedge caps; full:
/// omitted entirely), an on-axis vertex has no latitude edge (partial:
/// fixed point; full: pole/apex).
#[derive(Debug)]
pub struct Revolved<T: Real> {
    /// The built body — a closed solid passing tiers 1–3.
    pub body: Body<T>,
    /// The solid.
    pub solid: SolidKey,
    /// The primary (outer-boundary) shell.
    pub shell: ShellKey,
    /// A full revolve of a holed profile only: the cavity shell each
    /// hole's revolved boundary was inserted as, one per hole loop
    /// (index 0 ↔ canonical loop 1). Empty for partial revolves (holes
    /// there are extrude-shaped tunnels in the one shell) and for
    /// unholed profiles.
    pub cavities: Vec<ShellKey>,
    /// Wall faces, per loop, per canonical segment (`None`: on-axis).
    pub walls: Vec<Vec<Option<FaceKey>>>,
    /// Latitude edges (partial: wedge arcs; full: full-period rims,
    /// self-loops at the surviving meridian vertices), per loop, per
    /// canonical vertex (`None`: on-axis vertex).
    pub rims: Vec<Vec<Option<EdgeKey>>>,
    /// Pole vertices, per loop, per canonical vertex: the ONE body
    /// vertex an on-axis profile vertex revolves to (the rotation
    /// fixes it, so every meridian chain meets there). `None` at
    /// off-axis vertices — those have one copy per chain, addressed
    /// through `rims` and the meridian chains — and at vertices
    /// strictly INTERIOR to a full revolve's omitted axis run, which
    /// that case deletes outright (no body entity exists to name).
    /// That last case is reachable through THIS API only — a
    /// multi-segment axis run needs collinear same-carrier joins,
    /// which the recipe layer's program validation refuses (#101).
    pub poles: Vec<Vec<Option<VertexKey>>>,
    /// The wedge caps and meridian edges — shaped by the case split.
    pub kind: RevolvedKind,
}

/// The per-case keys of a [`Revolved`] (see the ratified case split in
/// the module docs).
#[derive(Debug)]
pub enum RevolvedKind {
    /// θ < 2π: wedge caps plus both meridian chains.
    Partial {
        /// The start cap — on the sketch plane; carries the profile's
        /// canonical winding for θ > 0 (module docs).
        start_cap: FaceKey,
        /// The end cap — on the sketch plane rotated by θ.
        end_cap: FaceKey,
        /// Start-chain meridian edges, per loop, per canonical segment.
        /// For an on-axis segment this is the shared axis edge (the
        /// same key appears in `end_meridians`).
        start_meridians: Vec<Vec<EdgeKey>>,
        /// End-chain meridian edges, per loop, per canonical segment.
        end_meridians: Vec<Vec<EdgeKey>>,
    },
    /// θ = 2π: no caps. The **lamina** case (no axis contact) sweeps
    /// one full-period band: `walls`/`rims` carry the complete
    /// revolution patches and full-period rim self-loops, `meridians`
    /// the single seam chain, and every `pi_*` field is `None`. The
    /// **wire** case (an on-axis run, omitted — the OUTER loop only;
    /// holes are strictly off-axis by validated containment) sweeps
    /// two π-bands so poles/apexes keep valence 2 (tier 2's strut
    /// ban): `walls`/`rims` are the angle-0…π band, the `pi_*` fields
    /// the π…2π band. Hole loops are always lamina-shaped: their
    /// `meridians` entries are their cavity seam chains, and the
    /// `pi_*` fields (outer-loop shaped) never name hole entities.
    Full {
        /// Angle-0 meridian edges (the `u = 0` seam chain), per
        /// canonical loop, per canonical segment (`None`: omitted
        /// on-axis segment of the outer loop).
        meridians: Vec<Vec<Option<EdgeKey>>>,
        /// Wire case: the π…2π band's wall faces, per canonical
        /// segment of the OUTER loop.
        pi_walls: Vec<Option<FaceKey>>,
        /// Wire case: the angle-π meridian copies (conventional
        /// `MappedCurve` — the π half-plane is not the seam), per
        /// canonical segment of the OUTER loop.
        pi_meridians: Vec<Option<EdgeKey>>,
        /// Wire case: the π…2π latitude half-rims, per canonical
        /// vertex of the OUTER loop.
        pi_rims: Vec<Option<EdgeKey>>,
    },
}

/// The two wedge-cap frames of a PARTIAL revolve, in the operation's
/// own vocabulary.
#[derive(Clone, Copy, Debug)]
pub struct WedgeFrames<T: Real> {
    /// The start cap's carrier frame (the sketch plane).
    pub start: Pose<T>,
    /// The end cap's carrier frame (the sketch plane rotated by θ).
    pub end: Pose<T>,
}

/// Typed refusal of [`revolved_caps`] (closed enum, D4 ¶3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WedgeCapsError {
    /// The revolve has no caps to read: [`RevolvedKind::Full`] closes
    /// on itself, so there is no start or end plane. A fact about the
    /// operation, not an empty answer.
    NoCaps,
    /// A cap face's frame could not be read.
    Read(ReadbackError),
}

impl From<ReadbackError> for WedgeCapsError {
    fn from(e: ReadbackError) -> Self {
        Self::Read(e)
    }
}

/// **Where did the partial revolve's wedge caps land?** — the joint
/// frames of a tube: each cap plane's origin and normal, which is
/// exactly the tube's end tangent there when the profile is a
/// cross-section.
///
/// The op-specific half of the read: the cap faces live inside
/// [`RevolvedKind::Partial`], so the case analysis is the door's
/// content and [`topo::readback::face_pose`] does the reading.
///
/// # Errors
///
/// [`WedgeCapsError::NoCaps`] for a full revolve; every
/// [`topo::readback::face_pose`] refusal.
///
/// ```
/// use geom_core::{Point2, Tol, Vec2};
/// use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
/// use sweep::{Revolution, RevolveAxis, revolve, revolved_caps};
///
/// let tol = Tol::witness();
/// // A quarter tube: a small circle a distance 5 from the axis,
/// // revolved a quarter turn about the sketch frame's +v.
/// let circle = profile::circle(Point2::new(5.0, 0.0), 0.5, tol).expect("a positive radius");
/// // The complete-loop primitives answer with a `ClosedLoop` (the
/// // lowered loop plus its program); `Profile` takes the loop.
/// let sketch = Profile::new(SketchPlane::xy(), vec![circle.into()])
///     .validate(tol)
///     .expect("the circle validates");
/// let axis = RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) };
/// let quarter = revolve::<f64>(
///     &sketch,
///     axis,
///     Revolution::Partial(std::f64::consts::FRAC_PI_2),
///     tol,
/// )
/// .expect("the tube revolves");
///
/// let caps = revolved_caps(&quarter).expect("a partial revolve has caps");
/// // The start cap IS the sketch plane, so its normal is the sketch
/// // normal (+z) — and that is the tube's end tangent there.
/// assert!(caps.start.axis.z.abs() > 0.99);
/// // A quarter turn about +y later, the end cap's normal has turned
/// // with it, onto x.
/// assert!(caps.end.axis.x.abs() > 0.99);
/// ```
pub fn revolved_caps<T: Real>(r: &Revolved<T>) -> Result<WedgeFrames<T>, WedgeCapsError> {
    let RevolvedKind::Partial {
        start_cap, end_cap, ..
    } = r.kind
    else {
        return Err(WedgeCapsError::NoCaps);
    };
    Ok(WedgeFrames {
        start: face_pose(&r.body, start_cap)?,
        end: face_pose(&r.body, end_cap)?,
    })
}

/// Typed failure of [`revolve`] (closed enum, D4 ¶3). Loop indices
/// reference the **canonical** (validated) profile — loop 0 the outer,
/// then holes; vertex/segment indices reference the canonical chain.
#[derive(Clone, Debug, PartialEq)]
pub enum RevolveError {
    /// The run's tolerance could not form a classification band.
    Band(BandError),
    /// The axis direction has no definite length (zero or sliver).
    DegenerateAxis,
    /// The axis-direction classification escalated or was poisoned.
    AxisEscalated {
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// The revolve angle is definitely degenerate: zero (or sliver) at
    /// tolerance, metered at the profile's maximum radial extent.
    DegenerateAngle,
    /// A partial angle reached (or exceeded) the full period at
    /// tolerance: an exactly-full revolve must say [`Revolution::Full`].
    FullRangeAngle,
    /// The angle classification escalated or was poisoned.
    AngleEscalated {
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// A profile vertex lies definitely on the negative-`r` side of the
    /// axis (the half-plane check).
    VertexCrossesAxis {
        /// Canonical index of the loop.
        loop_index: usize,
        /// Canonical index of the vertex.
        vertex_index: usize,
    },
    /// A profile vertex's radial distance landed in the sliver band —
    /// a micro-radius revolve is a genuine sliver (ratified case
    /// split), surfaced as this typed error rather than escalated.
    SliverRadius {
        /// Canonical index of the loop.
        loop_index: usize,
        /// Canonical index of the vertex.
        vertex_index: usize,
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// An arc segment's interior definitely dips into `r < 0` (span
    /// beyond the half-period around an on-axis center, or an apex on
    /// the negative side — the half-plane check for arc interiors).
    ArcCrossesAxis {
        /// Canonical index of the loop.
        loop_index: usize,
        /// Canonical index of the segment.
        segment_index: usize,
    },
    /// A segment's axis-clearance classification escalated (a line's
    /// radial/axial delta, or an arc's span/apex/center-radius margin,
    /// in the sliver band or poisoned).
    SliverAxisClearance {
        /// Canonical index of the loop.
        loop_index: usize,
        /// Canonical index of the segment.
        segment_index: usize,
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// An off-axis-centered arc whose carrier reaches (or crosses) the
    /// axis: the swept torus would be horn/spindle, outside D3's ring
    /// torus convention (`R > r > 0`).
    UnsupportedToroid {
        /// Canonical index of the loop.
        loop_index: usize,
        /// Canonical index of the segment.
        segment_index: usize,
    },
    /// Full revolve of a profile whose axis contact is not a single
    /// contiguous run of on-axis segments: an isolated on-axis vertex
    /// (or a run-detached one) revolves to a non-manifold solid (D1).
    NonManifoldAxisContact {
        /// Canonical index of the loop.
        loop_index: usize,
        /// Canonical index of the offending vertex.
        vertex_index: usize,
    },
    /// Full revolve of a profile whose OUTER loop touches the axis in
    /// two or more disjoint runs: the revolved boundary closes an
    /// inner shell — a cavity — that this construction has no
    /// certified containment evidence for (the two-run profile's
    /// "hole" is not a validated hole loop; it exists only in the
    /// revolved solid). Refused under the ratified cavity invariant
    /// (DESIGN's M2 bullet: every cavity is born through the shared
    /// void-insertion door, with caller-certified containment) — the
    /// recourse is the explicit composition, whose boolean derives
    /// the evidence itself.
    MultipleAxisRuns {
        /// Canonical index of the loop.
        loop_index: usize,
    },
    /// Full revolve of a holed profile met a HOLE loop with axis
    /// contact (an on-axis vertex or segment). Unreachable for a
    /// profile validated at this run's tolerance — a hole is strictly
    /// interior to the outer region, interior points are strictly
    /// off-axis under the half-plane gate, and near-axis holes refuse
    /// as [`RevolveError::SliverRadius`] first — so this arm exists
    /// for the tolerance-disagreement case (profile validated at a
    /// different ε than the revolve runs at), surfaced typed rather
    /// than trusted.
    HoleTouchesAxis {
        /// Canonical index of the hole loop.
        loop_index: usize,
    },
    /// The void-insertion door refused a hole cavity's insertion
    /// ([`topo::insert_void`]). The evidence arms are unreachable from
    /// this construction (every hole shell is certified from the
    /// profile's own validation before the call); the revert/graft
    /// arms surface kernel-level corruption typed.
    VoidInsertion {
        /// Canonical index of the hole loop whose insertion refused.
        loop_index: usize,
        /// The door's refusal.
        source: topo::VoidInsertError,
    },
    /// A cosurface-sharing predicate escalated at a join (the extrude
    /// `CosurfaceEscalated` posture: defense-in-depth, believed
    /// unreachable from validated profiles).
    CosurfaceEscalated {
        /// Canonical index of the loop.
        loop_index: usize,
        /// Canonical index of the join vertex.
        vertex_index: usize,
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// The dihedral classification at a latitude (wall–wall) join
    /// escalated: a sliver dihedral, certifiable as neither a corner
    /// nor a smooth join (D2's ratified text).
    SliverJoin {
        /// Canonical index of the loop.
        loop_index: usize,
        /// Canonical index of the join vertex.
        vertex_index: usize,
        /// The classifier's diagnostic.
        source: Indeterminate,
    },
    /// The dihedral classification at a cap–wall meridian rim (or a
    /// partial revolve's cap–cap axis edge) escalated during the
    /// upgrade pass.
    SliverRim {
        /// Canonical index of the loop.
        loop_index: usize,
        /// Canonical index of the rim's segment.
        segment_index: usize,
        /// The classifier's diagnostic.
        source: Indeterminate,
    },
    /// A cap plane failed Newell certification (unreachable for
    /// validated profiles — surfaced rather than trusted).
    CapPlane {
        /// The Newell failure.
        source: NewellError,
    },
    /// An Euler operator or attachment gate refused — including every
    /// D4 ¶2 certification failure
    /// ([`EulerOpError::Certification`]).
    Op {
        /// The operator-layer failure.
        source: EulerOpError,
    },
    /// The final whole-body pcurve mint pass refused (M6-3, walk row
    /// 4: revolve outputs carry stored certified pcurves at rest,
    /// exactly as boolean/split/loft outputs do).
    Pcurve(topo::PcurveMintError),
}

impl fmt::Display for RevolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Band(e) => write!(f, "revolve could not form a band: {e}"),
            Self::DegenerateAxis => write!(
                f,
                "revolve axis direction has no definite length (zero or sliver) — {}",
                geom_core::COINCIDENCE_RECOURSE
            ),
            Self::AxisEscalated { source } => {
                write!(f, "revolve axis classification escalated: {source}")
            }
            Self::DegenerateAngle => write!(
                f,
                "revolve angle is coincident with zero at tolerance (metered at the profile's \
                 maximum radial extent) — {}",
                geom_core::COINCIDENCE_RECOURSE
            ),
            Self::FullRangeAngle => f.write_str(
                "partial revolve angle reaches the full period at tolerance: an exactly-full \
                 revolve must use Revolution::Full",
            ),
            Self::AngleEscalated { source } => {
                write!(f, "revolve angle classification escalated: {source}")
            }
            // Definite at ANY magnitude (r = -0.5 fires this same arm),
            // so the coincidence levers are offered conditionally — the
            // unconditional fix is to move the profile off the negative
            // side (S6 review, MINOR-2).
            Self::VertexCrossesAxis {
                loop_index,
                vertex_index,
            } => write!(
                f,
                "profile vertex at loop {loop_index} vertex {vertex_index} lies definitely on \
                 the negative side of the revolve axis (a revolve never carries material \
                 through the axis) — move the profile to the non-negative side; if the vertex \
                 was meant to sit exactly on the axis, {}",
                geom_core::COINCIDENCE_RECOURSE
            ),
            Self::SliverRadius {
                loop_index,
                vertex_index,
                source,
            } => write!(
                f,
                "profile vertex at loop {loop_index} vertex {vertex_index} sits a sliver away \
                 from the revolve axis (micro-radius revolve): {source}"
            ),
            Self::ArcCrossesAxis {
                loop_index,
                segment_index,
            } => write!(
                f,
                "arc segment at loop {loop_index} segment {segment_index} definitely dips into \
                 the negative side of the revolve axis"
            ),
            Self::SliverAxisClearance {
                loop_index,
                segment_index,
                source,
            } => write!(
                f,
                "axis-clearance classification at loop {loop_index} segment {segment_index} \
                 escalated: {source}"
            ),
            Self::UnsupportedToroid {
                loop_index,
                segment_index,
            } => write!(
                f,
                "arc at loop {loop_index} segment {segment_index} sweeps a horn/spindle torus \
                 (carrier reaches the axis): outside D3's ring-torus convention"
            ),
            Self::NonManifoldAxisContact {
                loop_index,
                vertex_index,
            } => write!(
                f,
                "full revolve: axis contact at loop {loop_index} vertex {vertex_index} is not \
                 part of a single on-axis segment run — the solid would be non-manifold"
            ),
            Self::MultipleAxisRuns { loop_index } => write!(
                f,
                "full revolve: loop {loop_index} touches the axis in two or more disjoint \
                 segment runs, so the revolved boundary would close an inner cavity shell \
                 this construction holds no containment evidence for (every cavity is born \
                 through the void-insertion door with caller-certified containment; a \
                 two-run profile's enclosure is not a validated hole loop) — revolve the \
                 solid profile and subtract the enclosed body (topo::subtract), or use a \
                 partial revolve"
            ),
            Self::HoleTouchesAxis { loop_index } => write!(
                f,
                "full revolve: hole loop {loop_index} touches the revolve axis, which \
                 contradicts its validated strict containment inside the outer loop — the \
                 profile was validated at a different tolerance than this revolve runs at; \
                 re-validate the profile at the run's tolerance"
            ),
            Self::VoidInsertion { loop_index, source } => write!(
                f,
                "full revolve: inserting hole loop {loop_index}'s revolved cavity through \
                 the void-insertion door refused: {source}"
            ),
            Self::CosurfaceEscalated {
                loop_index,
                vertex_index,
                source,
            } => write!(
                f,
                "cosurface sharing at loop {loop_index} vertex {vertex_index} escalated: \
                 {source}"
            ),
            Self::SliverJoin {
                loop_index,
                vertex_index,
                source,
            } => write!(
                f,
                "sliver dihedral at loop {loop_index} vertex {vertex_index}: the latitude join \
                 is neither a definite corner nor definitely smooth: {source}"
            ),
            Self::SliverRim {
                loop_index,
                segment_index,
                source,
            } => write!(
                f,
                "sliver dihedral at loop {loop_index} segment {segment_index}'s cap rim: \
                 {source}"
            ),
            Self::CapPlane { source } => write!(f, "revolve cap plane: {source}"),
            Self::Op { source } => write!(f, "revolve operator step failed: {source}"),
            Self::Pcurve(source) => write!(f, "revolve pcurve mint pass: {source}"),
        }
    }
}

impl std::error::Error for RevolveError {}

impl From<EulerOpError> for RevolveError {
    fn from(source: EulerOpError) -> Self {
        Self::Op { source }
    }
}

/// The swept traversal a revolve sweeps: the shared record and the
/// shared builder, re-exported under this module's names.
///
/// Reversed for θ > 0 (module docs). A revolve's wall orientation is
/// decided per wall class (`axis::WallKind`, `axis::classify_segment`)
/// and not per segment, so this verb needs nothing beyond the shared
/// record — unlike `extrude`, which wraps it to add an orientation bit.
pub(super) use crate::swept::{SweptSeg, swept_segments};

/// Revolves a validated profile about an in-sketch-plane axis into a
/// closed solid.
///
/// The sketch placement is the profile's own; the axis and angle
/// classify per the module docs' conventions (named trilean predicates
/// throughout). On success the returned body passes tiers 1–3 — the
/// caller re-validates at rest per the workspace convention.
///
/// # Errors
///
/// [`RevolveError`] — closed and typed: degenerate/sliver axis and
/// angle, half-plane violations, sliver radii, unsupported toroids,
/// non-manifold axis contact, sliver dihedrals, Newell failures, and
/// every operator/certification refusal.
pub fn revolve<T: Decide>(
    profile: &ValidatedProfile<T>,
    axis: RevolveAxis<T>,
    revolution: Revolution<T>,
    tol: Tol,
) -> Result<Revolved<T>, RevolveError> {
    let band = Band::linear(tol).map_err(RevolveError::Band)?;
    let place = profile.plane().placement;
    let frame = axis::AxisFrame::build(place, &axis, band)?;

    // ---- Angle classification (module docs), metered at the
    // profile's maximum radial extent (the named lever arm). ----
    let r_max = axis::radial_extent(profile, &frame);
    let (theta, reverse, full) = match revolution {
        Revolution::Full => (T::tau(), true, true),
        Revolution::Partial(t) => {
            let sign = decide("revolve_angle", Margin::levered(t, r_max), band)
                .map_err(|source| RevolveError::AngleEscalated { source })?;
            let reverse = match sign {
                Sign::Zero => return Err(RevolveError::DegenerateAngle),
                Sign::Positive => true,
                Sign::Negative => false,
            };
            let headroom = Margin::levered(T::tau() - t.abs(), r_max);
            match decide("revolve_angle_headroom", headroom, band)
                .map_err(|source| RevolveError::AngleEscalated { source })?
            {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => return Err(RevolveError::FullRangeAngle),
            }
            (t, reverse, false)
        }
    };

    // ---- Swept traversals + axis-contact classification (canonical
    // error indices carried through the reversal). A FULL revolve's
    // hole loops traverse FORWARD: the stored hole chain is the
    // canonical clockwise traversal, i.e. exactly the reversed chain
    // of the counterclockwise hole-as-outer loop — which is what the
    // +2π sweep expects (module docs: winding). `classify_loop` still
    // receives `reverse = true` for them: its sign un-flip then lands
    // on the hole-as-outer (counterclockwise) direction, whose
    // material-left side is the hole's interior — the cavity solid's
    // own material, which is what each hole builds as before the door
    // reverses it.
    let loops: Vec<Vec<SweptSeg<T>>> = profile
        .loops()
        .iter()
        .enumerate()
        .map(|(li, lp)| swept_segments(lp, if full && li > 0 { false } else { reverse }))
        .collect();
    let mut classes = Vec::with_capacity(loops.len());
    for (li, segs) in loops.iter().enumerate() {
        classes.push(axis::classify_loop(segs, &frame, li, reverse, band)?);
    }

    let mut out = if full {
        full::build_full(&frame, &loops, &classes, theta, band, tol)
    } else {
        partial::build_partial(&frame, &loops, &classes, theta, reverse, band, tol)
    }?;
    // Final pass (M6-3, walk row 4): every revolve face's chart now
    // mints — cone/sphere/torus walls exactly as cylinder ones — so a
    // revolve output carries its stored certified pcurves at rest,
    // the same posture as boolean/split/loft outputs.
    topo::mint_pcurves(&mut out.body, tol).map_err(RevolveError::Pcurve)?;
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// S6 (two-tolerance, D4 ¶1 addendum): the three revolve pairs —
    /// axis length, angle, and vertex radius — each describe one user
    /// situation across their definite and in-band arms; every arm
    /// carries the shared recourse fragment.
    #[test]
    fn revolve_pairs_carry_the_shared_recourse() {
        let diag = |name| Indeterminate {
            margin: geom_core::MarginDiag::Value(5e-9),
            band: Band::new(1e-9, 1e-8).unwrap(),
            predicate: Some(name),
        };
        let errors = [
            RevolveError::DegenerateAxis,
            RevolveError::AxisEscalated {
                source: diag("revolve_axis_direction"),
            },
            RevolveError::DegenerateAngle,
            RevolveError::AngleEscalated {
                source: diag("revolve_angle"),
            },
            RevolveError::VertexCrossesAxis {
                loop_index: 0,
                vertex_index: 1,
            },
            RevolveError::SliverRadius {
                loop_index: 0,
                vertex_index: 1,
                source: diag("axis_vertex_radius"),
            },
        ];
        for e in errors {
            let msg = e.to_string();
            assert_eq!(
                msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
                1,
                "{msg}"
            );
        }
    }
}

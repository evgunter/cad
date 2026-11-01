//! Trilean **point-in-solid** containment (F8): the ray design of
//! profile's 2-D machinery and the planar loop walk
//! ([`crate::splitting::containment::point_in_loop`]) promoted to 3-D.
//! Its consumers: the boolean's containment fallback for operands
//! whose boundaries do not intersect (the case §15.9 names), the
//! split-join's role resolution for a region its section cannot place
//! (`join.rs`), and the census's material test. Its crossing walk
//! ([`ray_crossings`]) is also the ray the side-at-infinity reader
//! ([`crate::side_reader`]) aims, and through it every reader of a
//! shell's role or a solid's orientation (that module's header names
//! them).
//!
//! # Method: a counted ray with the fixed schedule
//!
//! Cast a ray from `q` along a direction of the fixed schedule — the
//! same 16-member table of dyadic directions as the planar loop walk, and
//! literally the same const (`SCHEDULE`, read from
//! `splitting::containment`), used here as space directions
//! **directly**: this module normalizes the raw triple, where
//! the loop walk projects it into the loop's plane and skips the
//! near-parallel members. One table, two different sweeps — the
//! shared const buys the absence of drift between copies, not
//! agreement on a direction, and determinism is per site (a `const`
//! swept in a fixed order every run). The ray is crossed edge by edge
//! (D10, Booleans; [`ray_crossings`]): each line edge between two planar
//! faces whose planes it crosses is decided once, for both faces, and an
//! edge it passes within the band of meets it, half a crossing for each
//! face the edge bounds; a planar face of lines and circle arcs clear of
//! its edges is crossed where the turn of its loops says, and another
//! tests the hit against its loops on their edges' own carriers
//! (`point_in_loop_projected`); the curved kinds' arms below offer their
//! roots, and a cylinder face of rulings and circles clear of its edges
//! holds a root where the turn of its loops on an image of its wall
//! says. Each crossing counts `+1` leaving material
//! (`d·n > 0`) and `−1` entering it, crossings in band of each other are
//! summed, and the sign of the first nonzero running count is the side
//! of `q`: rising ⇒ the ray *exits* material first ⇒ `In`; falling ⇒
//! `Out`.
//!
//! **Why a signed count, not bare parity** (a documented deviation from
//! the plan's "ray parity" wording): parity is orientation-blind — on
//! a reverted (complement) operand it reports containment in the
//! *enclosed* region rather than in the body's actual material, and
//! the `A∖B ≡ A∩revert(B)` oracle (Problem 15.9) feeds reverted
//! operands through this very fallback. The signed count
//! answers "is `q` in the material?" uniformly for ordinary bodies,
//! multi-shell bodies with voids, and complements; for an ordinary
//! embedded boundary it agrees with parity.
//!
//! # Grazing and the retry schedule
//!
//! Ray choices are never allowed to decide borderline geometry. A
//! boundary pre-pass reports `q` ON the solid's boundary as
//! [`SolidContainment::OnBoundary`] before any ray is cast, and refuses
//! where `q` is in band of a face. Past it, `q` is off every FACE by
//! more than the band's zero — but it may still be in band of a face's
//! CARRIER, where its foot lies outside the face. The schedule is walked
//! by [`crate::ray_walk::walk`], and each ray counts its crossings
//! ([`crate::ray_walk::Crossings::first_count`]): a crossing landing ON
//! a curved face's boundary or meeting its face tangentially before
//! the count turns, a tie between two faces that do not meet, a tangent
//! root, or a crossing at `q` grazes the ray; a ray within the band of a
//! vertex, or of a face at two of its edges (a wedge), reads nothing. A
//! ray that meets a cylinder or cone face whose region cannot be read
//! (`FaceGeo::PartialWall`, `FaceGeo::PartialCone`) ahead of `q`, within
//! the ball that holds the face, is blocked. On the plane arm, and at every crossing's advance and order,
//! an in-band margin is about the ray — how it runs against a carrier,
//! where it meets one, a hit near a face's loop however far from `q` —
//! and sets it aside. A ray parallel to a carrier within the band skips
//! that face only where `q` is definitely off the carrier, or (on a
//! plane) the face lies definitely beside the ray; otherwise it is set
//! aside too. When no ray decides, the refusal is the walk's: the first
//! limit that blocked a ray, else the first in-band reading, else
//! [`PointInSolidError::RayExhausted`].
//!
//! # Predicates (all K-tagged through the Q1 funnel, meters)
//!
//! - **`bool_point_in_solid_plane`**: signed elevation of `q` off a
//!   face's carrier (boundary pre-pass; Zero ⇒ on the carrier ⇒ trim
//!   test). On a plane, an in-band elevation refuses only where `q`'s
//!   foot on the plane is not definitely outside the face: a point in
//!   band of a carrier far from its face is off that face.
//! - **`bool_point_in_solid_sphere`**: the same pre-pass question on a
//!   sphere face, the linearized radial residual `(|q − c|² − r²)/2r`
//!   (metres).
//! - **`bool_point_in_solid_denom`**: the parallel-ray gate, a length:
//!   `d·n` per planar face, and a wall's `|d⊥|`, each levered by how far
//!   from `q` the selection reaches (`selection_reach`), so the margin is
//!   how far the ray rises off the plane, or drifts off its distance
//!   from the axis, over every length at which it could meet the face.
//!   Zero ⇒ the ray runs along the carrier: it skips the face where `q`
//!   is definitely off the carrier (`bool_point_in_solid_clearance`) or,
//!   on a plane, the face lies definitely beside the ray
//!   (`bool_point_in_solid_beside`); otherwise the ray is abandoned. The
//!   wall arm's outward sign at a root is read off the decided
//!   discriminant, never re-decided.
//! - **`bool_point_in_solid_clearance`**: `|elev|`, the unsigned
//!   distance of `q` off a carrier a ray runs along within the band (a
//!   plane's elevation, a wall's linearized residual). Positive ⇒ the
//!   ray cannot reach the face; Zero ⇒ graze; in band ⇒ the ray is
//!   abandoned. Unsigned because the side is not the question: which
//!   side `q` is on decides nothing here.
//! - **`bool_point_in_solid_beside`**: for a ray running along a planar
//!   face's carrier, the face's clearance off the plane through the ray
//!   normal to that carrier (the larger of its two sides'), over the
//!   hull of its outer loop. Positive ⇒ the ray passes beside the face.
//! - **`bool_point_in_solid_advance`**: the crossing's advance `t`
//!   along the ray (Zero ⇒ crossing at `q` — graze, retry).
//! - **`bool_cone_partial_reach`**: a point's distance from a cone's
//!   apex against the ball an untrimmable cone face lies in (Positive ⇒
//!   that face cannot hold the point; anything else refuses or sets the
//!   ray aside).
//! - The in-face walk's rows are its own module's (`point_in_loop_*`
//!   for a loop of lines, `point_in_arc_loop_*` for a loop with arcs —
//!   [`crate::splitting::containment::point_in_loop`] lists them).
//! - **`bool_point_in_solid_order`**: two crossings' advances, the
//!   least found and every other asked against it (Zero ⇒ a tie with
//!   the closest ⇒ graze, retry). The winning crossing's
//!   already-decided `denom` sign is the In/Out verdict — no second
//!   decision on the same margin.
//! - **`bool_ray_sphere_disc`**: the ray/sphere discriminant, metered
//!   as a length (`disc / 2r`; `√disc` is the half-chord in metres).
//!   Zero ⇒ a tangent ray — graze, retry; Negative ⇒ definite miss.
//!   The outward sign at each root is read off the SAME decided
//!   discriminant (`d·(p − c)/r = ±√disc/r`), never re-decided.
//! - **`bool_ray_cone_lead`**: the ray×cone quadratic's leading
//!   coefficient `(d·â)² − cos²α`, levered by the face's slant extent.
//!   Zero ⇒ the ray runs parallel to a generator, where the quadratic
//!   degenerates and no certified crossing PAIR exists — graze, retry.
//! - **`bool_ray_cone_disc`**: that quadratic's discriminant over the
//!   same extent (a length). Zero ⇒ the ray grazes a generator —
//!   graze, retry; Negative ⇒ definite miss.
//! - **`bool_ray_cone_nappe`**: the hit's height over the apex along
//!   the axis, which says the nappe whose outward normal the hit is read
//!   by. Zero ⇒ the hit is at the apex, where the SURFACE (not merely
//!   the chart) is singular and no outward normal exists: the crossing
//!   is offered sideless, and grazes where it counts.
//! - **`bool_ruled_region_*`**: whether a cylinder or cone hit lies in
//!   the face, its region read from its edges ([`super::ruled_region`]).
//! - **`bool_ray_cone_incidence`**: `d·n̂` at a cone hit, levered by the
//!   local radius — the cone's analogue of the plane arm's `denom`.
//! - **`bool_ray_torus_disc`**: the ray×torus QUARTIC's discriminant,
//!   metered over `(R + r)¹¹` (a length: for a monic quartic the
//!   discriminant is the squared product of root gaps). Zero ⇒ a
//!   repeated root — graze, retry. Its two companions
//!   **`bool_ray_torus_shape`** and **`bool_ray_torus_depth`** split the
//!   positive-discriminant case into four real roots and none; a Zero on
//!   either leaves the count uncertain, which this door never answers
//!   on. **`bool_ray_torus_odd`** selects the biquadratic closed form,
//!   **`bool_ray_torus_split_lead`** and **`bool_ray_torus_split`** are
//!   the factorization's own certifications. A constructed root set
//!   that disagrees with the certified count
//!   ([`Unsettled::CountDisagrees`]) blocks the ray, carrying both
//!   counts ([`PointInSolidError::RootCountDisagrees`]).
//! - **`bool_ray_torus_incidence`**: `d·n̂` at a torus hit, levered by
//!   the tube radius.
//! - A torus face outside a closed group is read from its boundary
//!   edges by [`super::torus_region`], under its own rows
//!   (`bool_torus_region_*`, `bool_torus_frame_radius`).
//! - A cylinder or cone face is read from its boundary edges by
//!   [`super::ruled_region`], under its own rows (`bool_ruled_region_*`),
//!   except a cylinder face held by the turn of its loops
//!   ([`wall_crosses_by_turn`], under `_winding`).
//!   One whose region cannot be read refuses only within a ball holding
//!   it: **`bool_wall_outline_reach`** on a wall, **`bool_cone_partial_reach`**
//!   on a cone (a distance less a radius, metres).
//! - **The full-turn route** ([`wrap_rims`], the sphere's chart):
//!   **`bool_wrap_rim`** decides whether an unmated boundary circle is an
//!   iso-line of the coordinate the face does not wrap — coaxial (the
//!   sine between the axes levered by the circle's radius; the centre's
//!   distance off the axis, metres).
//! - **`bool_point_in_solid_vertex`**: a ray's distance from a vertex of
//!   an edge it is read against (metres); in band ⇒ a corner, Zero ⇒ a
//!   graze. **`bool_point_in_solid_edge`**: the lower end of the
//!   enclosed gap between the ray and a line edge's line, read by the
//!   census's closest read (`census::line_passes`); not Positive ⇒ the
//!   ray meets the edge where **`bool_point_in_solid_edge_span`** puts
//!   the closest point definitely inside the edge.
//!   **`bool_point_in_solid_winding`**: the turn of a face's loops about
//!   the ray's hit (a planar face's, or a cylinder face's on an image of
//!   its wall), less `π`, levered by the reach; Positive ⇒ the hit is
//!   inside the face. Read through the unlogged door, as it is definite
//!   by construction.
//! - **`side_at_infinity_advance`**, **`_denom`**, **`_clearance`**,
//!   **`_beside`**, **`_order`**, **`_vertex`**, **`_edge`**,
//!   **`_edge_span`** and **`_winding`**: the planar rows above, as a ray aimed
//!   from beyond the selection decides them
//!   ([`crate::side_reader::AIMED_ROWS`]); the curved
//!   arms keep their own names.
//! - **The side at infinity** ([`crate::side_reader`], the one reader of
//!   a shell's role and of a solid's orientation) — consulted here only
//!   when a ray crosses NO boundary at all: `q` then sits on the
//!   at-infinity side, which is `Out` for a positively-oriented body
//!   and `In` for a complement ([`missed_side`]). A ray aimed from beyond
//!   the selection at a face, 10° off its normal, reads it off its count
//!   on the rows above; only where no
//!   face offers an aim that reads, **`bool_point_in_solid_infinity`**,
//!   the selection's signed volume scaled to a length (`V/A`), does.
//! - **`bool_point_past_every_reach`**: where the ray test has no arm for
//!   a face of the selection, how far `q` stands past the hull of every
//!   face's reach along a direction that turns with the body, less the
//!   sweep pad (metres); Positive ⇒ the side at infinity
//!   ([`past_every_reach`]). **`bool_point_axis`**: the offset from the
//!   faces' anchor to `q`, before it is read as one of those directions.

use geom_core::period::{Exact, Extent, Period, TurnRows, Within};
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use crate::body::Body;
use crate::chart_groups::ChartGroups;
use crate::entity::{EntityId, FaceKey, LoopBoundary, SolidKey};
use crate::face_normal::plane_outward_normal;
use crate::live::linked;
use crate::ray_walk::{self, Crossings, RayFault};
use crate::side_reader::{SideUnread, side_at_infinity};
use crate::splitting::containment::{
    Escalation, LoopContainment, PointInLoopError, SCHEDULE, loop_extent_from,
    point_in_loop_projected,
};
use crate::validate::decide;
use geom_brep::quartic::{QuarticRoots, QuarticRows, depressed_quartic_roots, quadratic_roots};
use geom_brep::recourse::Reading;
use geom_brep::roots::{Found, RootRefusal, Roots, Unsettled};

use super::region::RegionRefusal;
use super::rim_wedge::Rim;
use super::ruled_region::{RuledFaceRegion, ruled_face_region};
use super::sphere_region::{SphereFaceRegion, sphere_face_region};
use super::surface_group::{WrapRims, coaxial_margins, surface_group, wrap_rims};
use super::torus_region::{TorusFaceRegion, torus_face_region};
use geom::Surface;
use geom_core::Tol;

/// The trilean answer: is `q` in the solid's **material**?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolidContainment {
    /// Strictly inside the material.
    In,
    /// Strictly outside.
    Out,
    /// On the boundary (within the band).
    OnBoundary,
}

/// The note a refusal ends on where the kernel's own arithmetic, not
/// the geometry, came apart ([`PointInSolidError::RootCountDisagrees`]):
/// every door that renders it ends on this.
pub(crate) const KERNEL_BUG_NOTE: &str = "this may indicate a kernel bug worth reporting";

/// Typed failure of [`point_in_solid`].
#[derive(Clone, Debug, PartialEq)]
// The variant roster the sample-coverage row reads (test builds only).
#[cfg_attr(
    test,
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(PointInSolidErrorKind), vis(pub(crate)), derive(strum::EnumIter))
)]
pub enum PointInSolidError {
    /// A predicate escalated (in-band margin).
    Escalated {
        /// The face being tested.
        face: FaceKey,
        /// The escalation diagnostics (named predicate inside).
        diag: Indeterminate,
    },
    /// No schedule ray settled — each grazed or gave nothing to read —
    /// for a point the boundary pre-pass placed off every face
    /// ([`crate::ray_walk::NoRaySettled`]).
    RayExhausted,
    /// The at-infinity orientation probe found a (near-)zero signed
    /// volume — the body bounds no material to be inside of.
    ZeroVolumeBody,
    /// A body read as given ([`SolidFaces::read_as_given`]) holds the side at
    /// infinity in its material: a ray that met none of its boundary
    /// left the point in it. A body a caller hands in bounds its
    /// material from outside, so it is inside-out. A complement the
    /// kernel builds for itself is not read as given, and answers `In`
    /// there.
    InsideOut,
    /// An in-plane loop test failed (nested typed error).
    Loop(PointInLoopError),
    /// A face is not walkable, or an entity it names is lost — an
    /// arena claim about a body that is BROKEN.
    ///
    /// A healthy face whose surface KIND this door has no arm for is
    /// [`Self::KindUnsupported`], never this: the two answers are
    /// about different things, and reporting a cone as corruption
    /// sends a reader to look for damage that is not there.
    CorruptFace {
        /// The face.
        face: FaceKey,
    },
    /// A healthy face whose surface KIND has no arm in the
    /// CONTAINMENT door.
    ///
    /// `point_in_solid` is ray-parity against each face of the body
    /// being classified against, so it needs a ray×surface crossing
    /// count per kind: `Plane` (its in-face walk), `Cylinder` and `Cone`
    /// (a face's region, [`super::ruled_region`]), `Sphere` (a closed
    /// group, or a trimmed face whose every edge is a certified circle
    /// arc, [`super::sphere_region`]) and `Torus` (a closed group, or a
    /// face's region, [`super::torus_region`]) have one; `Nurbs` and
    /// `Approx` do not.
    ///
    /// **Why the kind still matters here.** The operand gate is
    /// pair-scoped — a face whose box clears the other operand cannot
    /// enter a crossing, so it does not gate the operation — but
    /// containment asks a different question: a ray from the query
    /// point crosses the WHOLE boundary, and a face out of reach of the
    /// cut is not out of reach of the ray. The one box this door reads
    /// is the hull of every face's reach: a point definitely past it
    /// takes the side at infinity without crossing a face. Any other
    /// point meets this refusal, so an operation the gate admits can
    /// still meet it downstream, and that boundary is the honest one
    /// until the kind has a containment arm. Issue 1011 has landed
    /// whole — the ray×cone quadratic and the ray×torus quartic with
    /// its certified root count, each root read against the face's
    /// region — and what this variant now carries is the spline kinds.
    KindUnsupported {
        /// The face.
        face: FaceKey,
        /// Its kind — the row the arm is missing for.
        kind: geom::SurfaceKind,
    },
    /// The at-infinity side could be read neither off the sign of the
    /// body's signed volume, which the props lane did not certify, nor
    /// off a ray aimed at one of its planar faces.
    ///
    /// Consulted only when a schedule ray crosses NO boundary at all —
    /// `q` then sits on the at-infinity side, and which side that IS
    /// depends on the body's orientation. A body whose volume's sign the
    /// props lane cannot certify (a face outside the closed-form inventory
    /// at a scalar with no quadrature lane, a quadrature that ran out of
    /// budget, or a bracket that still straddles zero at the reporting
    /// target), and none of whose aimed rays decides
    /// without reading in band, leaves that question unanswerable along
    /// that ray; where no ray of the schedule decides, this says so
    /// rather than reporting a HEALTHY body as broken. A body with no
    /// planar face offers no ray to aim.
    VolumeUncertified,
    /// A `Sphere` face that is not closed on its own surface and has a
    /// boundary edge that is not a circle arc.
    ///
    /// A trimmed sphere face is read from its boundary arcs
    /// ([`super::sphere_region`]): every crossing of a great circle with
    /// a circle arc is the root of a first harmonic. An edge on any other
    /// carrier — a spline, or the quartic a torus, a cylinder or a cone
    /// whose axis misses the sphere's centre leaves on it — has no such
    /// crossing here, and neither has an edge with no certified carrier.
    PartialSphereFace {
        /// The sphere face whose outline the region reading cannot cross.
        face: FaceKey,
    },
    /// A `Cone` face whose region cannot be read
    /// ([`super::ruled_region::ruled_face_region`]'s `None`): a boundary
    /// edge that is not a line, a circle or an ellipse — a spline, or the
    /// parabola or hyperbola a plane parallel to a generator leaves — or
    /// one with no certified carrier between two points.
    ///
    /// The point-in-solid door refuses it only where such a face could
    /// decide the answer: the point on that cone within reach of the
    /// face, or a ray of the schedule meeting the cone definitely ahead
    /// of the point and within reach of the face, where no other ray
    /// settled.
    PartialConeFace {
        /// The cone face neither class expresses.
        face: FaceKey,
    },
    /// A `Torus` face outside a closed group with a boundary edge that
    /// lies in no plane — a spline — or with no certified carrier between
    /// two points.
    ///
    /// A torus face is read from its boundary edges
    /// ([`super::torus_region`]): every edge a torus carries at rest (a
    /// parallel, a meridian, a Villarceau circle, a spiric oval's arc)
    /// lies in a plane, and every crossing of a path with it is the root
    /// of a first harmonic. An edge in no plane has no such crossing here.
    PartialTorusFace {
        /// The torus face whose boundary the region cannot read.
        face: FaceKey,
    },
    /// A ray's hit on a `Plane` face could land inside it, and the face
    /// is bounded by a spiric (a plane's section of a torus) or spline
    /// edge the in-face walk could not read at the hit.
    ///
    /// The walk crosses every boundary edge on its own carrier: a line
    /// where its end vertices straddle the ray, a circle or an ellipse
    /// at the roots of its quadratic inside the arc's window, a spiric or
    /// a spline piece by piece. This is where a spiric's or spline's
    /// pieces ran out of depth or budget before placing the hit.
    ///
    /// Its own arm rather than [`Self::Loop`], because this door names
    /// the FACE a probe met, as its curved-face siblings do, and the
    /// census and the shell witness read it with them as a face kind
    /// the door does not serve rather than as a point too close to call.
    EdgeCarrierUnsupported {
        /// The face whose outline the walk cannot read: a planar face's
        /// loop walk, or a torus face's boundary reading of an oval arc.
        face: FaceKey,
        /// The loop, and the edge whose reading ran out.
        cause: crate::splitting::Uncrossable,
    },
    /// A ray's hit on a `Cylinder` wall face could land inside it, and
    /// the face's region cannot be read
    /// ([`super::ruled_region::ruled_face_region`]'s `None`): a boundary
    /// edge that is not a line, a circle or an ellipse, or one with no
    /// certified carrier between two points.
    ///
    /// Confined the way [`Self::PartialConeFace`] is: a hit definitely
    /// outside a ball holding the face is a miss ([`FaceGeo::PartialWall`]),
    /// and only a hit the face could actually contain refuses.
    WallOutlineUnsupported {
        /// The cylinder wall face whose outline the walk cannot read.
        face: FaceKey,
    },
    /// A test ray met a `Torus` face where the crossings built from its
    /// quartic disagree in number with the count the quartic's signs
    /// certify ([`Unsettled::CountDisagrees`]). The certified count is
    /// the wrong one: the sign ladder's discriminant carries `f64`
    /// rounding its band does not charge
    /// (`work/inside/ray-torus-count-certifies-on-an-uncharged-discriminant.md`),
    /// a kernel defect. Another ray's quartic is another computation, so
    /// it blocks this ray only.
    RootCountDisagrees {
        /// The torus face.
        face: FaceKey,
        /// The count the sign ladder certified.
        certified: usize,
        /// The count the factorization built.
        constructed: usize,
    },
    /// [`point_in_solid_of`] was asked about a solid the body does not
    /// hold — an arena claim, like [`Self::CorruptFace`].
    NoSuchSolid {
        /// The key that did not resolve.
        solid: SolidKey,
    },
}

impl PointInSolidError {
    /// Is this refusal **inconclusive about the one point asked**, so
    /// that another point can still decide: a reading about where the
    /// point stands ([`Self::in_band`]), a limit the door met near it
    /// ([`Self::confined`]), or a side that neither the volume nor an
    /// aimed ray could read for the point's rays that met nothing
    /// ([`Self::sideless`]) — another point's rays may meet the boundary. Every other refusal is about a body — a
    /// face kind with no arm, a corrupt face — and answers the same at
    /// any point.
    pub(crate) fn inconclusive(&self) -> bool {
        self.in_band() || self.confined() || self.sideless()
    }

    /// A ray of the point's that met nothing, whose side neither the
    /// solid's volume nor a ray aimed at its boundary could read
    /// (`VolumeUncertified`, `ZeroVolumeBody`).
    /// The walk reaches it only through such a ray, and refuses on it
    /// only where no ray settled; an empty selection's `ZeroVolumeBody`
    /// answers the same at every point, which a ladder trying the next
    /// witness only reads again.
    pub(crate) fn sideless(&self) -> bool {
        matches!(self, Self::VolumeUncertified | Self::ZeroVolumeBody)
    }

    /// An in-band reading (`Escalated`, `Loop(Escalated)`), or a
    /// schedule no ray of which settled (`RayExhausted`,
    /// `Loop(RayExhausted)`).
    pub(crate) fn in_band(&self) -> bool {
        matches!(
            self,
            Self::Escalated { .. }
                | Self::RayExhausted
                | Self::Loop(
                    PointInLoopError::Escalated { .. } | PointInLoopError::RayExhausted { .. }
                )
        )
    }

    /// A face or edge the door cannot read, refused only where it could
    /// decide the answer: the point on or near it, or a test ray that
    /// definitely met it where no ray settled; or a ray whose quartic
    /// against a torus face the kernel read wrong.
    pub(crate) fn confined(&self) -> bool {
        matches!(
            self,
            Self::PartialConeFace { .. }
                | Self::EdgeCarrierUnsupported { .. }
                | Self::WallOutlineUnsupported { .. }
                | Self::RootCountDisagrees { .. }
        )
    }

    /// The refusal in one clause — the short form of the sentence
    /// `Display` renders with its recourse, and the one vocabulary a
    /// consumer that carries a `&'static str` reads (the census's
    /// containment arm names the witness it could not decide with it).
    pub fn summary(&self) -> &'static str {
        match self {
            Self::Escalated { .. } => {
                "the material witness escalated in band at the instance's boundary — \
                 undecided at this ε"
            }
            Self::RayExhausted => {
                "no schedule ray from the material witness settled its side of the \
                 instance's boundary: each grazed it or could not be read — undecided"
            }
            Self::ZeroVolumeBody => {
                "the instance's signed volume is (near-)zero — no material side at \
                 infinity to classify against"
            }
            Self::InsideOut => {
                "the instance is inside-out: its material holds the side at infinity"
            }
            Self::Loop(_) => "the in-plane region walk at the instance's boundary refused",
            Self::CorruptFace { .. } => {
                "a face of the instance is not walkable, or an entity it names is lost"
            }
            Self::KindUnsupported { .. } => {
                "the instance carries a face kind the point-in-solid door does not serve \
                 (a spline surface), so its material cannot be probed"
            }
            Self::VolumeUncertified => {
                "the instance's at-infinity side could not be read — the sign of its \
                 signed volume is uncertified, and no ray aimed at a flat face of it \
                 decided"
            }
            Self::PartialSphereFace { .. }
            | Self::PartialConeFace { .. }
            | Self::PartialTorusFace { .. } => {
                "the instance carries a curved face outside the point-in-solid door's \
                 chart classes, so its material cannot be probed"
            }
            Self::EdgeCarrierUnsupported { .. } => {
                "a probe ray from the material witness met a flat face bounded by a \
                 spline or torus-section edge, whose outline cannot be crossed exactly"
            }
            Self::WallOutlineUnsupported { .. } => {
                "a probe ray from the material witness met a cylinder wall edged by a \
                 curve that is not a flat cut across it, so the hit cannot be placed on it \
                 exactly"
            }
            Self::RootCountDisagrees { .. } => {
                "a probe ray from the material witness met a torus face where the kernel's \
                 count of its crossings came apart"
            }
            Self::NoSuchSolid { .. } => "the instance's solid key does not resolve",
        }
    }
}

impl From<PointInLoopError> for PointInSolidError {
    fn from(e: PointInLoopError) -> Self {
        Self::Loop(e)
    }
}

impl core::fmt::Display for PointInSolidError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            // The escalation names no decision, so it ends as the
            // containment door ends the same escalation
            // (`ContainError::Escalated` with no decision).
            Self::Escalated { diag, .. } => write!(
                f,
                "cannot tell what is inside the solid: {}",
                diag.undecided(
                    super::placement_subject(None),
                    super::placement_ending(None, Escalation::Margin, diag, Reading::Build),
                )
            ),
            Self::RayExhausted => write!(
                f,
                "cannot tell what is inside the solid: {}",
                ray_walk::NoRaySettled
            ),
            Self::Loop(e) => write!(f, "cannot tell what is inside the solid: {e}"),
            Self::ZeroVolumeBody => write!(
                f,
                "cannot tell what is inside the solid: it encloses no measurable volume"
            ),
            Self::InsideOut => write!(
                f,
                "the solid is inside-out: a test ray from the point met none of its \
                 boundary and left the point in its material, so its faces point into \
                 it. Recourse: build it with its faces pointing outward, or revert it"
            ),
            Self::CorruptFace { .. } => write!(
                f,
                "cannot tell what is inside the solid: one of its faces is broken (it \
                 cannot be walked, or names something that is gone)"
            ),
            Self::KindUnsupported { kind, .. } => write!(
                f,
                "cannot tell what is inside the solid: one of its faces is a {} \
                 surface, which the inside/outside test has no way yet to cross. The \
                 solid itself is fine. Recourse: build the solid so no spline (NURBS) \
                 face bounds it",
                super::kind_word(*kind)
            ),
            Self::VolumeUncertified => write!(
                f,
                "cannot tell what is inside the solid: a test ray from the point met \
                 none of its boundary, and neither its volume, whose sign could not be \
                 certified, nor a ray aimed at a flat face said which side that leaves; no \
                 other ray settled it. The solid itself is fine. Recourse: test a point \
                 whose rays meet the boundary"
            ),
            Self::PartialSphereFace { .. } => write!(
                f,
                "cannot tell what is inside the solid: one of its sphere faces has an \
                 outline the inside/outside test cannot read. The solid itself is fine. \
                 Recourse: cut the sphere face only where its edges are circles — with \
                 planes, other spheres, or cylinders and cones whose axis passes \
                 through its centre"
            ),
            Self::PartialConeFace { .. } => write!(
                f,
                "cannot tell what is inside the solid: one of its cone faces has an \
                 outline the inside/outside test cannot read. The solid itself is fine. \
                 Recourse: bound the cone face with straight lines through its apex, \
                 circles about its axis, or flat cuts that cross it in an ellipse, \
                 rather than a spline"
            ),
            Self::PartialTorusFace { .. } => write!(
                f,
                "cannot tell what is inside the solid: one of its torus faces has an \
                 outline the inside/outside test cannot read. The solid itself is fine. \
                 Recourse: bound the torus face with plane sections of the tube \
                 (circles, or the ovals a plane parallel to its axis cuts) rather than \
                 a spline, or let its faces together cover the whole torus"
            ),
            Self::EdgeCarrierUnsupported { cause, .. } => write!(
                f,
                "cannot tell what is inside the solid: the point, or where a test ray \
                 from it met the boundary, lies on a flat face bounded by a {} edge, \
                 within reach of that edge, which cannot be crossed exactly. The solid \
                 itself is fine. Recourse: test a point farther from that face",
                cause.carrier.word()
            ),
            Self::WallOutlineUnsupported { .. } => write!(
                f,
                "cannot tell what is inside the solid: the point, or where a test ray \
                 from it met the boundary, lies where a cylinder wall with an edge that \
                 is not a flat cut across it could hold it, and that outline cannot be \
                 read. The solid itself is fine. Recourse: test a point farther from that \
                 wall"
            ),
            Self::RootCountDisagrees {
                certified,
                constructed,
                ..
            } => write!(
                f,
                "cannot tell what is inside the solid: a test ray met one of its torus faces \
                 where the crossings built from the ray's equation ({constructed}) disagree \
                 with the count its signs certify ({certified}), and no other ray settled. \
                 The solid itself is fine. Recourse: test a point farther from that face; \
                 {KERNEL_BUG_NOTE}"
            ),
            Self::NoSuchSolid { .. } => write!(
                f,
                "cannot tell what is inside the solid: the body holds no such solid"
            ),
        }
    }
}

impl std::error::Error for PointInSolidError {}

/// The face's plane, F5-gated: origin and **outward** normal (the
/// chart normal with the face's sense folded in through
/// [`plane_outward_normal`] — the callers of this door are handed a
/// material direction, not a chart datum).
///
/// Every consumer reads the normal's sign-blind content: each feeds it,
/// directly or through a probe, to [`point_in_face`], whose answer is
/// ray-crossing parity, and the join's tie (`sectors::runs_in`) also
/// asks only whether a point's height over the plane is zero. Threading
/// the sense here keeps the door's CONTRACT honest for a consumer that
/// reads the sign.
pub(crate) fn face_plane<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<(Point3<T>, Vec3<T>), PointInSolidError> {
    let f = body
        .get_face(face)
        .ok_or(PointInSolidError::CorruptFace { face })?;
    match body.get_surface(f.surface) {
        Some(Surface::Plane { origin, normal, .. }) => {
            Ok((*origin, plane_outward_normal(f, *normal).vec()))
        }
        // A resolved non-plane is a CAPABILITY answer; only a surface
        // key that does not resolve is corruption.
        Some(s) => Err(PointInSolidError::KindUnsupported {
            face,
            kind: s.kind(),
        }),
        None => Err(PointInSolidError::CorruptFace { face }),
    }
}

/// A face's resolved query geometry: the planar datum, or a curved
/// face's carrier with what reads a point on it — a closed group's
/// representative, or the face's own region read from its boundary.
///
/// **Orientation (S10)**: every arm's outward direction is the chart's
/// with the face's sense folded in. The plane arm carries it in the
/// normal itself (there is a vector to fold it into); the curved arms have
/// no stored normal — their outward sign is taken at each ray hit — so
/// they carry the face's `sense` bit and the doors apply
/// it to the sign they derive. Only the material-side signs need it:
/// the boundary pre-pass compares residuals against Zero, orientation-free,
/// and a region reads the sense itself.
pub(crate) enum FaceGeo<T: geom_core::Real> {
    /// A planar face: a point on it and its OUTWARD normal.
    Plane(Point3<T>, Vec3<T>),
    /// A cylinder wall face: the ray/wall quadratic's roots, each tested
    /// against the face's own region, read from its boundary edges
    /// ([`super::ruled_region`]). Per face, like [`FaceGeo::SpherePatch`].
    Cylinder {
        origin: Point3<T>,
        axis: Vec3<T>,
        radius: T,
        /// The face's region on the wall.
        region: RuledFaceRegion<T>,
        /// The face's orientation sense: `false` means the material is
        /// INSIDE the wall, so the radial gradient at a hit points
        /// into material and the crossing sign flips.
        sense: bool,
    },
    /// A CLOSED sphere wall (M5 PR 9c): the faces sharing this sphere
    /// surface together cover the whole chart, so there is no chart
    /// trim to carry — membership on the surface IS membership in the
    /// face group. Closure is decided structurally at resolution time
    /// ([`closed_sphere_group`]), and every arm acts only for the
    /// group's REPRESENTATIVE face so one sphere contributes exactly
    /// one crossing pair per ray (a per-face arm would fold the same
    /// root twice and tie itself into a permanent graze).
    Sphere {
        /// The sphere's centre.
        center: Point3<T>,
        /// Its radius (positive by convention).
        radius: T,
        /// The group's representative — the first face of the
        /// selection, in the selection's order, carrying this surface.
        /// Arms no-op on every other member.
        representative: FaceKey,
        /// The REPRESENTATIVE's orientation sense (S10). The group is
        /// closed and bounds one material region, so its members share
        /// a sense; asking the representative is asking the group, and
        /// it is the representative that the arms act for. (Agreement
        /// across the group is a tier-3 obligation, not re-checked
        /// here.) `false` swaps the near/far crossing pair below.
        sense: bool,
    },
    /// A cone face: the ray/double-cone quadratic's roots, each tested
    /// against the face's own region, read from its boundary edges
    /// ([`super::ruled_region`]), which places a root on the mirror nappe
    /// outside it. Per face, like [`FaceGeo::SpherePatch`].
    Cone {
        /// The apex, where the surface has no tangent plane.
        apex: Point3<T>,
        /// The unit axis; the opening nappe opens along it.
        axis: Vec3<T>,
        /// The half-angle α ∈ (0, π/2) between axis and generators.
        half_angle: T,
        /// The face's region on the cone.
        region: RuledFaceRegion<T>,
        /// The face's orientation sense (S10), applied to the sign
        /// derived from the chart normal.
        sense: bool,
    },
    /// A TRIMMED sphere face: the same ray/sphere quadratic as
    /// [`FaceGeo::Sphere`], with each root tested against the face's own
    /// region, read from its boundary arcs
    /// ([`super::sphere_region`]). Unlike the closed group this arm IS
    /// per face: a trimmed face carries its own region, so two faces of
    /// one sphere surface contribute different crossings and no
    /// representative stands in for the rest.
    SpherePatch {
        /// The sphere's centre.
        center: Point3<T>,
        /// Its radius (positive by convention).
        radius: T,
        /// The face's region on the sphere.
        region: SphereFaceRegion<T>,
        /// The face's orientation sense (S10) — `false` swaps the
        /// near/far crossing pair, as for the closed group.
        sense: bool,
    },
    /// A CLOSED torus group: the faces sharing this torus surface
    /// together cover the whole torus, as [`FaceGeo::Sphere`]'s do the
    /// sphere, and the arms act for the representative alone so one torus
    /// contributes one crossing per root ([`torus_closed_group`]).
    Torus {
        /// The torus centre, on the axis in the tube's midplane.
        center: Point3<T>,
        /// The unit axis of revolution.
        axis: Vec3<T>,
        /// The major radius `R` (centre to tube centre).
        major_radius: T,
        /// The minor radius `r` (the tube), `R > r` by the ring-torus
        /// convention tier-3 check 1 enforces at rest.
        minor_radius: T,
        /// The group's first face in the selection's order. Arms no-op
        /// on every other member.
        representative: FaceKey,
        /// The representative's orientation sense (S10), applied to the
        /// sign derived from the chart normal.
        sense: bool,
    },
    /// A torus face outside a closed group: the same ray/torus quartic as
    /// [`FaceGeo::Torus`], with each root tested against the face's own
    /// region, read from its boundary edges ([`super::torus_region`]).
    /// Like [`FaceGeo::SpherePatch`] this arm is per face.
    TorusPatch {
        /// The torus centre.
        center: Point3<T>,
        /// The unit axis of revolution.
        axis: Vec3<T>,
        /// The major radius.
        major_radius: T,
        /// The minor radius.
        minor_radius: T,
        /// The face's region on the torus.
        region: TorusFaceRegion<T>,
        /// The face's orientation sense (S10).
        sense: bool,
    },
    /// A cone face whose region this door cannot read
    /// ([`super::ruled_region::ruled_face_region`]'s `None`: an edge that
    /// is not a line, a circle or an ellipse). A query refuses only where
    /// that region could matter: `q` on the double cone within the
    /// face's apex ball, or a ray that may meet the cone ahead within it
    /// — the sweep then tries the next ray, and the query refuses
    /// `PartialConeFace` only when every ray does ([`partial_reach`]).
    PartialCone {
        /// The apex.
        apex: Point3<T>,
        /// The unit axis.
        axis: Vec3<T>,
        /// The half-angle.
        half_angle: T,
        /// A radius about the apex the face lies within, which is also
        /// the lever of the ray margins.
        reach: T,
    },
    /// A cylinder wall face whose region this door cannot read, as
    /// [`FaceGeo::PartialCone`] is on a cone: a query refuses
    /// `WallOutlineUnsupported` only where a point on the wall within the
    /// ball `(anchor, reach)` holding the face could decide it.
    PartialWall {
        origin: Point3<T>,
        axis: Vec3<T>,
        radius: T,
        /// The centre of a ball holding the face: a point of its
        /// boundary.
        anchor: Point3<T>,
        /// The ball's radius ([`partial_reach`]).
        reach: T,
    },
}

/// The representative of `face`'s sphere-surface face group when that
/// group is CLOSED — every edge on its boundary is shared by two faces
/// of the SAME sphere surface, so the group's union has no boundary
/// against any other surface and therefore covers the whole sphere.
///
/// This is the shape the M5 inventory actually mints: a full revolve of
/// a pole-to-pole arc yields TWO half-sphere bands on ONE sphere key,
/// joined along the seam meridian and its angle-π copy (the `ball`
/// acceptance's V2 E2 F2 structure) — not one full-chart face. Asking
/// the question of the GROUP rather than the face is what lets the
/// whole-sphere class through without inventing a per-face chart trim.
///
/// The scan is [`super::surface_group`]'s, with nothing exempt: it reads
/// arena keys and mate adjacency only, never a margin, so it has no
/// in-band twin and does not move with ε. Rings on a sphere face make
/// the answer `None` (a ringed sphere face is a trimmed one), and so
/// does a body this walk cannot complete — a sphere face whose arena is
/// broken is not a CLOSED sphere face, and the trimmed class it falls
/// through to raises the corruption itself.
pub(super) fn closed_sphere_group<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    charts: &ChartGroups,
) -> Option<FaceKey> {
    surface_group(body, face, charts).ok().flatten()
}

/// Resolves [`FaceGeo`]; kinds outside
/// {Plane, Cylinder, Cone, Sphere, Torus} refuse as
/// [`PointInSolidError::KindUnsupported`] (per-arm, C12.1 — the spline
/// kinds have no ray-crossing arm), and a TRIMMED sphere face with a
/// boundary edge that is not a circle arc as
/// [`PointInSolidError::PartialSphereFace`]. Only a surface key that
/// does not RESOLVE is corruption.
pub(crate) fn face_geo<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    charts: &ChartGroups,
) -> Result<FaceGeo<T>, PointInSolidError> {
    // The kinds this dispatch has an arm for are [`ray_crosses`]'s, and
    // it refuses the others through it, as the reader's pre-filter does
    // ([`crate::side_reader::side_at_infinity`]): one predicate, which
    // `face_geo_arms_are_the_kinds_rays_cross` holds to the arms.
    if let Some(e) = uncrossable(body, face)? {
        return Err(e);
    }
    face_geo_arms(body, face, charts)
}

/// [`face_geo`]'s dispatch, past the predicate: an arm for each kind
/// [`ray_crosses`] lists, and [`PointInSolidError::KindUnsupported`] for
/// each it does not, named variant by variant.
fn face_geo_arms<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    charts: &ChartGroups,
) -> Result<FaceGeo<T>, PointInSolidError> {
    let f = body
        .get_face(face)
        .ok_or(PointInSolidError::CorruptFace { face })?;
    match body.get_surface(f.surface) {
        Some(Surface::Plane { origin, normal, .. }) => Ok(FaceGeo::Plane(
            *origin,
            plane_outward_normal(f, *normal).vec(),
        )),
        Some(&Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        }) => match ruled_face_region(body, face)? {
            Some(region) => Ok(FaceGeo::Cylinder {
                origin,
                axis,
                radius,
                region,
                sense: f.sense,
            }),
            None => {
                let anchor = boundary_point(body, face)?;
                Ok(FaceGeo::PartialWall {
                    origin,
                    axis,
                    radius,
                    anchor,
                    reach: partial_reach(body, face, anchor)?,
                })
            }
        },
        Some(&Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        }) => match ruled_face_region(body, face)? {
            Some(region) => Ok(FaceGeo::Cone {
                apex,
                axis,
                half_angle,
                region,
                sense: f.sense,
            }),
            None => Ok(FaceGeo::PartialCone {
                apex,
                axis,
                half_angle,
                reach: partial_reach(body, face, apex)?,
            }),
        },
        Some(&Surface::Sphere { center, radius, .. }) => {
            match closed_sphere_group(body, face, charts) {
                Some(representative) => Ok(FaceGeo::Sphere {
                    center,
                    radius,
                    representative,
                    sense: body
                        .get_face(representative)
                        .ok_or(PointInSolidError::CorruptFace { face })?
                        .sense,
                }),
                None => match sphere_face_region(body, face, center, radius)? {
                    Some(region) => Ok(FaceGeo::SpherePatch {
                        center,
                        radius,
                        region,
                        sense: f.sense,
                    }),
                    None => Err(PointInSolidError::PartialSphereFace { face }),
                },
            }
        }
        Some(&Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        }) => {
            if let Some(representative) = torus_closed_group(body, face, charts)? {
                return Ok(FaceGeo::Torus {
                    center,
                    axis,
                    major_radius,
                    minor_radius,
                    representative,
                    sense: body
                        .get_face(representative)
                        .ok_or(PointInSolidError::CorruptFace { face })?
                        .sense,
                });
            }
            match torus_face_region(body, face)? {
                Some(region) => Ok(FaceGeo::TorusPatch {
                    center,
                    axis,
                    major_radius,
                    minor_radius,
                    region,
                    sense: f.sense,
                }),
                None => Err(PointInSolidError::PartialTorusFace { face }),
            }
        }
        // Refused in `face_geo` through `uncrossable`: only this
        // dispatch read alone reaches here, and it refuses the same way.
        Some(s @ (Surface::Nurbs(_) | Surface::Approx(_))) => {
            Err(PointInSolidError::KindUnsupported {
                face,
                kind: s.kind(),
            })
        }
        None => Err(PointInSolidError::CorruptFace { face }),
    }
}

/// K name: how far a point stands past the hull of a selection's face
/// reaches, along one candidate direction (metres).
const PAST_EVERY_REACH: &str = "bool_point_past_every_reach";

/// K name: the offset from a selection's anchor to a point (metres),
/// before it is read as a unit axis.
pub(crate) const POINT_AXIS: &str = "bool_point_axis";

/// Whether `q` lies definitely outside the hull of every face's
/// certified reach (`census::face_reach_in`, the box rules' one map),
/// along some direction that turns with the body: a planar face's
/// outward normal, or the axis from the faces' anchor to `q`
/// ([`super::separating`]'s candidates, each read both ways; this
/// scalar carries no bracket to merge a normal with its negation by).
/// World axes are not read: a turned body's world hull widens with the
/// turn, so a verdict read on them would be the pose's. The hull is
/// padded by the sweep pad, as [`super::separating`] pads each item it
/// reads; `q` is exact and takes none. A face with no claim to make
/// answers `false`.
fn past_every_reach<T: Decide>(
    body: &Body<T>,
    faces: &[FaceKey],
    q: Point3<T>,
    band: Band,
) -> bool {
    use super::separating::{self, Item};
    let items = || faces.iter().map(|&f| Item::Face(f));
    let mut dirs = separating::face_axes(faces.iter().map(|&f| (body, f)), band, None);
    let anchor = separating::mean(items().filter_map(|i| separating::anchor(body, i)));
    dirs.extend(anchor.and_then(|a| geom_core::UnitVec3::new(q - a, POINT_AXIS, band).ok()));
    let pad = T::from_f64(super::boxes::sweep_pad(band));
    dirs.into_iter().any(|dir| {
        let Some((lo, hi)) = separating::support_along(body, items(), dir, band) else {
            return false;
        };
        let at = dir.get().dot(Vec3::new(q.x, q.y, q.z));
        let past = |m: T| {
            matches!(
                decide(PAST_EVERY_REACH, Margin::of(m - pad), band),
                Ok(Sign::Positive)
            )
        };
        past(at - hi) || past(lo - at)
    })
}

/// **Whether the ray test has an arm for a surface kind** — the one
/// list of them: [`face_geo`] refuses every other kind through
/// [`uncrossable`] before its dispatch, and the side-at-infinity reader
/// casts no ray at a selection holding one. Wildcard-free, so a new kind
/// is decided here.
pub(crate) const fn ray_crosses(kind: geom::SurfaceKind) -> bool {
    use geom::SurfaceKind;
    match kind {
        SurfaceKind::Plane
        | SurfaceKind::Cylinder
        | SurfaceKind::Cone
        | SurfaceKind::Sphere
        | SurfaceKind::Torus => true,
        SurfaceKind::Nurbs | SurfaceKind::Approx => false,
    }
}

/// `face`'s refusal where the ray test has no arm for its surface
/// ([`ray_crosses`]): [`face_geo`] raises it on every ray, before any
/// geometry, so it is a property of the selection and not of a ray.
///
/// # Errors
///
/// A face or surface that does not resolve.
pub(crate) fn uncrossable<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Option<PointInSolidError>, PointInSolidError> {
    let f = body
        .get_face(face)
        .ok_or(PointInSolidError::CorruptFace { face })?;
    let kind = body
        .get_surface(f.surface)
        .ok_or(PointInSolidError::CorruptFace { face })?
        .kind();
    Ok((!ray_crosses(kind)).then_some(PointInSolidError::KindUnsupported { face, kind }))
}

/// **A radius about `anchor` that a cylinder or cone face lies within**
/// — the reach [`FaceGeo::PartialWall`] and [`FaceGeo::PartialCone`]
/// refuse inside, about a point of the wall face's boundary or the
/// cone's apex.
///
/// Every point of a wall face lies on a ruling segment whose two ends
/// are points of its boundary, and every point of a cone face on a
/// generator segment from a point of its boundary toward the apex, so a
/// ball about such an anchor holding the boundary holds the face; each
/// loop's ball is the containment walk's own ([`loop_extent_from`]:
/// vertices, and each curved edge's carrier ball).
///
/// # Errors
///
/// [`PointInSolidError::CorruptFace`] for a missing face, and
/// [`loop_extent_from`]'s refusal of a loop that does not walk.
fn partial_reach<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    anchor: Point3<T>,
) -> Result<T, PointInSolidError> {
    let f = body
        .get_face(face)
        .ok_or(PointInSolidError::CorruptFace { face })?;
    let mut reach = T::zero();
    for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        reach = reach.max(loop_extent_from(body, lk, anchor)?);
    }
    Ok(reach)
}

/// The point of `face`'s outer loop's first vertex.
///
/// # Errors
///
/// [`PointInSolidError::CorruptFace`] for a face whose outer loop has no
/// cycle or does not resolve.
fn boundary_point<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Point3<T>, PointInSolidError> {
    let corrupt = PointInSolidError::CorruptFace { face };
    let outer = body.get_face(face).ok_or(corrupt.clone())?.outer;
    let LoopBoundary::Cycle { first } = body.get_loop(outer).ok_or(corrupt.clone())?.boundary
    else {
        return Err(corrupt);
    };
    let start = body.get_half_edge(first).ok_or(corrupt.clone())?.start;
    body.get_vertex(start)
        .and_then(|v| body.get_point(v.point))
        .copied()
        .ok_or(corrupt)
}

/// **A torus face's closed group**: its representative when every edge
/// of the group's boundary is shared by two faces of the same torus
/// surface ([`super::surface_group`] with nothing exempt), so the union
/// has no boundary against the rest of the body and covers the whole
/// (compact, connected) torus. Such a group has no boundary to read a
/// region from, and its members must not each fold the same root: the
/// arms act for the representative alone. This is the shape a full
/// revolve of a closed profile mints. `None` for every other face, whose
/// region its own boundary states ([`super::torus_region`]).
///
/// # Errors
///
/// [`PointInSolidError::CorruptFace`] for a body the scan cannot walk.
fn torus_closed_group<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    charts: &ChartGroups,
) -> Result<Option<FaceKey>, PointInSolidError> {
    surface_group(body, face, charts).map_err(|face| PointInSolidError::CorruptFace { face })
}

/// A point's linearized residual off a cylinder wall,
/// `(ρ² − r²)/2r` in metres: the form the certification layer
/// classifies.
fn wall_elevation<T: Decide>(origin: Point3<T>, axis: Vec3<T>, radius: T, q: Point3<T>) -> T {
    let w = q - origin;
    let radial = w - axis * w.dot(axis);
    (radial.norm_squared() - radius.powi(2)) / (T::from_f64(2.0) * radius)
}

/// **A point's elevation off a ring torus's tube**: `√((ρ − R)² + h²) − r`,
/// the exact signed distance to the surface (negative inside the tube).
/// Every door that asks "is this point ON the torus" reads this one
/// expression, so the solid door, the face door and their rows cannot
/// disagree about which points lie on the tube.
pub(super) fn torus_elevation<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    major_radius: T,
    minor_radius: T,
    p: Point3<T>,
) -> T {
    let w = p - center;
    let h = w.dot(axis);
    let rho = (w - axis * h).norm();
    ((rho - major_radius).powi(2) + h.powi(2)).sqrt() - minor_radius
}

/// The unit radial direction of the chart ruling at azimuth `u`:
/// `u_ref·cos u + (â × u_ref)·sin u`. The one spelling of the chart's
/// azimuth frame, shared by the window construction and the junction
/// test.
pub(super) fn chart_dir<T: Decide>(axis: Vec3<T>, u_ref: Vec3<T>, u: T) -> Vec3<T> {
    let (s, c) = u.sin_cos();
    u_ref * c + axis.cross(u_ref) * s
}

/// Whether a face's azimuth window `raw` on a sphere of `radius` is
/// definitely short of one turn (`bool_sphere_trim_period`, the turn
/// rule at the radius).
pub(crate) fn trim_window_short<T: Decide>(
    raw: (T, T),
    radius: T,
    band: Band,
) -> Result<bool, Indeterminate> {
    let rows = TurnRows::exact("bool_sphere_trim_period");
    Period::new(T::tau())
        .strictly_partial(
            rows,
            Extent::between(raw.0, raw.1),
            Exact::new(radius),
            band,
        )
        .map_err(|undecided| undecided.diag)
}

/// **A sphere face's exact chart rectangle**, for the face class the
/// sphere chart rectangle can actually express — the rectangle the
/// sphere arm of the face boxes bounds a face by
/// (`boxes::sphere_window`). Point containment does not read it: a
/// trimmed sphere face's region is read from its boundary arcs
/// ([`super::sphere_region`]).
///
/// # The class
///
/// No rings, and every boundary edge a chart iso-line of the sphere —
/// a **latitude rim** (circle whose axis is the polar axis, centred on
/// it, at the radius the sphere's own geometry fixes) or a **meridian
/// great circle** (centred at the sphere centre, at the sphere's own
/// radius, axis perpendicular to the polar axis). Those are the two
/// classes the analytic sphere chart mints, and together they make the
/// face exactly the rectangle `[azimuth] × [latitude]` its boundary
/// pins. A face outside the class answers `None` — the honest
/// remainder, never a rectangle that misstates it.
///
/// # The latitude window is NOT an axial one, and that is the point
///
/// A sphere's latitude extremes are carried here as the exact
/// `(axial, radial)` PAIR of each extreme boundary latitude —
/// `(w·â, |w − â(w·â)|)`, the unnormalized point on the chart's
/// meridian half-plane — and never as the axial offset alone, nor as
/// the latitude SINE.
///
/// The reason is a lever, not a convenience. An axial separation is
/// `R·|Δ cos v|`, which collapses like `sin v̄` as either pole is
/// approached: two latitudes a millimetre apart across a pole differ
/// axially by micrometres, so an axial margin calls them the same
/// level and a rectangle that is not one passes. The pair form instead
/// meters the separation of two latitudes by the 2-D cross product
/// `ρ₁h₂ − h₁ρ₂ = R² sin(v₂ − v₁)` — the SINE of the latitude
/// difference, levered by `R`, which is the arc-length convention and
/// is bounded below by nothing at a pole: at `v₁ = 0` it is exactly
/// `R sin v₂`, the geodesic distance from the pole. Both components of
/// each pair come straight from geometry (a rim's own radius, a
/// vertex's own radial norm), so neither is recovered from the other
/// by a subtraction that cancels near a pole.
///
/// # The pole-in-edge-interior invariant
///
/// A meridian arc that runs THROUGH a pole breaks the rectangle in both
/// coordinates at once, and the azimuth half is the sharper of the two.
///
/// * **Azimuth.** The window comes from each boundary edge's closed-form
///   chart image, and a meridian's image is a constant-azimuth iso-line.
///   An arc through a pole is not one: its azimuth jumps by π there. The
///   loop walk carries a pole junction exactly — it reads the loop's own
///   orientation to pin the branch — but it carries it at a VERTEX,
///   which a pole interior to an edge is not.
/// * **Latitude.** The extreme is then interior to the edge, where a
///   fold over boundary levels never looks. (The props lane's own
///   version of this is now folded from the stored span rather than from
///   endpoints, so that half could in principle be lifted the same way.
///   The azimuth half cannot, so lifting it alone would buy nothing.)
///
/// The premise is therefore CHECKED rather than assumed: a meridian edge
/// with a pole strictly inside its span takes the face out of the class,
/// and the caller's typed refusal stands.
///
/// # Errors
///
/// [`PointInSolidError`] — escalations from the class predicates.
///
/// # Panics
///
/// Where a boundary half-edge's edge or that edge's curve does not
/// resolve (D2 row 4): a torn curve is not null scaffolding. The links
/// hold at rest and, on the reduction's working copies, by
/// [`crate::live::OPERATORS_KEEP_LINKS`].
pub(crate) fn sphere_chart_trim<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    band: Band,
) -> Result<Option<SphereChartTrim<T>>, PointInSolidError> {
    let escalate = |diag| PointInSolidError::Escalated { face, diag };
    let f = body
        .get_face(face)
        .ok_or(PointInSolidError::CorruptFace { face })?;
    if !f.rings.is_empty() {
        return Ok(None);
    }
    let surf = body
        .get_surface(f.surface)
        .cloned()
        .ok_or(PointInSolidError::CorruptFace { face })?;
    let LoopBoundary::Cycle { first } = body
        .get_loop(f.outer)
        .ok_or(PointInSolidError::CorruptFace { face })?
        .boundary
    else {
        return Ok(None);
    };
    // A definite class miss is `None` (the honest remainder); an
    // in-band one escalates — the two-tolerance pair.
    let zero = |name: &'static str, m: Margin<T>| -> Result<bool, PointInSolidError> {
        match decide(name, m, band).map_err(escalate)? {
            Sign::Zero => Ok(true),
            Sign::Positive | Sign::Negative => Ok(false),
        }
    };
    // A record miss past `face` answers `CorruptFace` and a torn edge
    // or curve panics: the walk's `CorruptFace` raises are
    // `torn-body-refusal-families-beyond-the-six-doors`' to split.
    let mut levels: Vec<(T, T)> = Vec::new();
    // A point's own exact meridian-half-plane pair.
    let pair = |p: Point3<T>| -> (T, T) {
        let w = p - center;
        let h = w.dot(axis);
        (h, (w - axis * h).norm())
    };
    for he in body
        .loop_cycle(first)
        .ok_or(PointInSolidError::CorruptFace { face })?
    {
        let he_data = body
            .get_half_edge(he)
            .ok_or(PointInSolidError::CorruptFace { face })?;
        // Every boundary VERTEX is a latitude the face attains.
        let v = *body
            .get_vertex(he_data.start)
            .and_then(|v| body.get_point(v.point))
            .ok_or(PointInSolidError::CorruptFace { face })?;
        levels.push(pair(v));
        let edge = linked(
            &body.edges,
            he_data.edge,
            EntityId::Edge,
            EntityId::HalfEdge(he),
            "edge",
        );
        let Some(curve) = body.edge_curve_linked(he_data.edge, edge).certified() else {
            // Null scaffolding: no carrier, so no class claim and no
            // latitude of its own beyond the vertex above.
            continue;
        };
        let (t0, t1) = curve.params();
        let geom::Curve3::Circle {
            center: c_c,
            axis: n_c,
            radius: r_c,
            u_ref: u_c,
        } = *curve.carrier()
        else {
            return Ok(None);
        };
        let w = c_c - center;
        // A unit-vector cross/dot is a SINE or COSINE (dimensionless)
        // and is levered by the radius; a length difference is already
        // metres.
        if zero(
            "bool_sphere_iso_meridian",
            Margin::levered(n_c.dot(axis), r_c),
        )? {
            // A meridian GREAT circle: centred at the sphere centre,
            // at the sphere's own radius.
            if !zero("bool_sphere_iso_meridian", Margin::of(w.norm()))?
                || !zero("bool_sphere_iso_meridian", Margin::of(r_c - radius))?
            {
                return Ok(None);
            }
            // The pole invariant (header). On this carrier the two
            // poles are the points whose radial direction is ±â, so
            // the in-span test is THE cosine-window construction with
            // `r̂ = ±â` — no vertex lookup and no `atan2`.
            let half = T::from_f64(0.5);
            let width = t1 - t0;
            // The edge's own turn, as its certificate decided it.
            let Some(turn) = curve.turn().copied() else {
                unreachable!("check 2 mints a turn for every circle")
            };
            match turn.turn() {
                Within::Short => {}
                // A full-period meridian edge runs through BOTH poles
                // and has no angular gate to test: out of the class.
                Within::Whole => return Ok(None),
            }
            let m_hat = chart_dir(n_c, u_c, geom::mid_param(t0, t1));
            let (_, c_h) = (width * half).sin_cos();
            for pole in [axis, axis * (T::zero() - T::one())] {
                match decide(
                    "bool_sphere_trim_pole_interior",
                    Margin::levered(pole.dot(m_hat) - c_h, radius),
                    band,
                )
                .map_err(escalate)?
                {
                    // The pole is strictly inside this edge: the
                    // latitude extreme is interior and no fold over
                    // boundary levels can see it.
                    Sign::Positive => return Ok(None),
                    // At an endpoint (already folded above) or off
                    // the span: latitude is monotone along the edge.
                    Sign::Zero | Sign::Negative => {}
                }
            }
        } else {
            // A latitude RIM: coaxial with the polar axis, centred on
            // it, and seated on the sphere (`|w|² + r_c² = R²`).
            let [tilt, off] = coaxial_margins(
                center,
                axis,
                &Rim {
                    center: c_c,
                    axis: n_c,
                    radius: r_c,
                    u_ref: u_c,
                },
            );
            if !zero("bool_sphere_iso_rim", tilt)?
                || !zero("bool_sphere_iso_rim", off)?
                || !zero(
                    "bool_sphere_iso_rim",
                    Margin::of((w.norm_squared() + r_c.powi(2)).sqrt() - radius),
                )?
            {
                return Ok(None);
            }
            // The rim's own exact pair — its radius IS the radial
            // component, with no square root anywhere near a pole.
            levels.push((w.dot(axis), r_c));
        }
    }
    // A window the loop walk cannot derive is a face this chart cannot
    // express, NOT a broken body: the walk needs a closed-form chart
    // image for every boundary edge and a branch it can pin across each
    // junction, and a face that denies it either is out of the class.
    // Only the in-band arm is an escalation.
    let raw = match crate::chord_join::face_azimuth_window(body, &surf, face, band) {
        Ok(Some(w)) => w,
        Ok(None) => return Ok(None),
        Err(crate::chord_join::SplitJoinError::Escalated { diag, .. })
        | Err(crate::chord_join::SplitJoinError::OrderEscalated { diag }) => {
            return Err(escalate(diag));
        }
        // A CORRUPTION-shaped refusal is not a class statement and must
        // not wear one: a key the walk could not resolve, or a boundary
        // that does not close, keeps the arena claim's own door — the
        // same distinction `bool_planar_chord_spec` draws between a key
        // that does not resolve and a key of the wrong kind.
        Err(crate::chord_join::SplitJoinError::Corrupt { .. })
        | Err(crate::chord_join::SplitJoinError::UnpairedLooseEnds { .. }) => {
            return Err(PointInSolidError::CorruptFace { face });
        }
        // The honest remainder: a walk this chart cannot express.
        Err(_) => return Ok(None),
    };
    // A face that ALONE wraps the azimuth ([`wrap_rims`], asked first)
    // attains every azimuth of its latitude range, so its azimuth window
    // is the whole turn (`None`). Any other face trims by its window, which must then be
    // definitely narrower than a period: a window that reads a whole
    // turn on a face that does not wrap alone has a gap the window
    // cannot see (a zone merged with half a cap), and one wider than a
    // period is a walk that wrapped more than once. Both are out of the
    // class.
    //
    // Latitude's only critical points are the poles, so the levels of
    // the edges that BOUND the face bound it unless it encloses a pole
    // none of them reaches; the levels of an edge the face holds on both
    // sides (a strut, its own seam) are points of the face and bound
    // nothing. On a face that does not wrap alone the period gate is
    // what excludes the enclosed pole: a loop that steps around one is a
    // walk reading a whole turn. On a face that wraps alone the bounding
    // edges are exactly its rims, so only their levels enter the fold,
    // with any pole the face's vertices reach: a single rim level is a
    // cap, whose far pole no rim sees, and keeps no window unless a
    // vertex sits on that pole.
    let (az, levels) = match wrap_rims(
        body,
        face,
        WrapRims {
            origin: center,
            axis,
        },
        band,
    )? {
        Some(rims) => {
            let mut bounding: Vec<(T, T)> = rims
                .iter()
                .map(|rim| ((rim.center - center).dot(axis), rim.radius))
                .collect();
            for level in levels {
                if zero("bool_sphere_trim_pole_end", Margin::of(level.1))? {
                    bounding.push(level);
                }
            }
            (None, bounding)
        }
        None => {
            if !trim_window_short(raw, radius, band).map_err(escalate)? {
                return Ok(None);
            }
            (Some(raw), levels)
        }
    };
    // Only a face that wraps alone can leave no level: one with no rim
    // and no vertex on a pole has no unmated boundary at all, so it is
    // the whole sphere (a slit closed on itself), which nothing trims.
    let Some((&first, rest)) = levels.split_first() else {
        return Ok(Some(SphereChartTrim {
            az: None,
            north: None,
            south: None,
        }));
    };
    let Some((north, south)) = latitude_extremes(first, rest, radius, band).map_err(escalate)?
    else {
        return Ok(None);
    };
    Ok(Some(SphereChartTrim { az, north, south }))
}

/// A sphere face's chart rectangle: the azimuth window its boundary
/// walk pins, and its two extreme latitudes, each as its exact
/// `(axial, radial)` pair on the meridian half-plane.
///
/// A `None` END is a constraint the face does not have, not a missing
/// datum: a face that wraps the azimuth alone attains every azimuth of
/// its latitudes, and a latitude window that reaches a POLE cannot be
/// excluded on that side — every latitude is at least the north pole's
/// and at most the south pole's. Carrying those as `None` rather than as
/// a margin against the pole is what keeps the margins honest:
/// `sin(v - v_pole)` degenerates to `sin v`, which is Zero at BOTH poles
/// and would call the far pole a graze.
pub(crate) struct SphereChartTrim<T> {
    /// The azimuth window on the surface's chart (`u_ref` at zero,
    /// increasing about the axis), definitely narrower than a turn, or
    /// `None` for a face that wraps alone.
    pub az: Option<(T, T)>,
    /// The extreme latitude nearest the `+axis` pole, or `None` when
    /// the face reaches that pole.
    pub north: Option<(T, T)>,
    /// The extreme latitude nearest the `−axis` pole, or `None` when
    /// the face reaches that pole.
    pub south: Option<(T, T)>,
}

/// The dimensionless sine of the latitude difference `v_b − v_a`
/// between two meridian-half-plane pairs on a radius-`r` sphere:
/// `(h_a ρ_b − ρ_a h_b)/r²`. Positive exactly when `b` lies further
/// from the `+axis` pole than `a`, and its levered magnitude is the
/// arc-length separation — the lever that does not collapse at a pole.
fn latitude_sine<T: geom_core::Real>(a: (T, T), b: (T, T), r: T) -> T {
    (a.0 * b.1 - a.1 * b.0) / r.powi(2)
}

/// Is `b` further from the `+axis` pole than `a`? A total order on the
/// meridian half-plane, in two exact steps.
///
/// The primary key is the non-collapsing sine of the latitude
/// difference. It answers everywhere except one place: it is
/// `sin(v_b − v_a)`, so it is Zero both when the two latitudes are
/// EQUAL and when they are ANTIPODAL — and on a half-plane whose
/// latitudes run `[0, π]` the antipodal case is exactly the two poles,
/// which is the ordinary shape of a lune's boundary, not an exotic one.
/// The tie-break is therefore the axial difference, and it is exact in
/// precisely the case it is asked: a Zero sine means the axial gap is
/// either ~0 (the same latitude, either order is right) or ~2R (the two
/// poles, no cancellation anywhere near it). The collapsing regime an
/// axial comparison has — two nearby latitudes at a pole — is the one
/// regime the sine has already decided.
fn latitude_after<T: Decide>(
    a: (T, T),
    b: (T, T),
    radius: T,
    band: Band,
) -> Result<bool, Indeterminate> {
    match decide(
        "bool_sphere_trim_latitude",
        Margin::levered(latitude_sine(a, b, radius), radius),
        band,
    )? {
        Sign::Positive => Ok(true),
        Sign::Negative => Ok(false),
        Sign::Zero => Ok(matches!(
            decide("bool_sphere_trim_antipode", Margin::of(a.0 - b.0), band)?,
            Sign::Positive
        )),
    }
}

/// Folds the boundary latitudes (`first`, then `rest`: a set with at
/// least one level) to the window's two extremes, and resolves each end
/// against its pole.
///
/// The fold's own comparison is [`latitude_after`], whose primary key
/// is the same non-collapsing sine the membership margins use, so its
/// in-band arm is a **deterministic tie-break** (D9) and not a verdict:
/// two candidates it cannot separate are within a band-width of ARC
/// LENGTH of each other, and the membership margins downstream are arc
/// lengths against that same band — so a query the tie-break could move
/// is a query already within a band of the window edge, whose verdict
/// is a graze either way. The tie-break can turn a definite verdict
/// into a graze; it can never turn `In` into `Out`.
///
/// An end that IS a pole becomes `None` — no constraint on that side.
/// `Ok(None)` overall is a face the window cannot describe: both
/// extremes at one pole is a face with no latitude extent at all.
#[allow(clippy::type_complexity)] // the two window ends, each optional
fn latitude_extremes<T: Decide>(
    first: (T, T),
    rest: &[(T, T)],
    radius: T,
    band: Band,
) -> Result<Option<(Option<(T, T)>, Option<(T, T)>)>, Indeterminate> {
    let (mut lo, mut hi) = (first, first);
    for &e in rest {
        if latitude_after(e, lo, radius, band)? {
            lo = e;
        }
        if latitude_after(hi, e, radius, band)? {
            hi = e;
        }
    }
    // A window END at a pole is a constraint the face does not have.
    // Which pole it is comes from the axial sign, whose margin at
    // `ρ = 0` is the full radius — the one place an axial comparison is
    // unambiguous.
    let resolve = |end: (T, T), want: Sign| -> Result<Option<Option<(T, T)>>, Indeterminate> {
        match decide("bool_sphere_trim_pole_end", Margin::of(end.1), band)? {
            Sign::Positive => Ok(Some(Some(end))),
            Sign::Zero | Sign::Negative => {
                match decide("bool_sphere_trim_pole_end", Margin::of(end.0), band)? {
                    s if s == want => Ok(Some(None)),
                    // The window's low end sits at the SOUTH pole (or
                    // its high end at the north): the face has no
                    // latitude extent, and no rectangle describes it.
                    _ => Ok(None),
                }
            }
        }
    };
    // A boundary with only ONE distinct latitude and no pole on it
    // does not pin a rectangle: the face's own latitude extreme is then
    // INTERIOR to it — a full cap, whose pole no boundary edge reaches
    // — and a window folded from the boundary would report the face as
    // its own rim. The pole-in-edge-interior invariant's face-level
    // twin, and the same reason: an extreme no boundary walk can see.
    if matches!(
        decide(
            "bool_sphere_trim_latitude",
            Margin::levered(latitude_sine(lo, hi, radius), radius),
            band,
        )?,
        Sign::Zero
    ) && matches!(
        decide("bool_sphere_trim_antipode", Margin::of(lo.0 - hi.0), band)?,
        Sign::Zero
    ) {
        return Ok(None);
    }
    let (Some(lo), Some(hi)) = (resolve(lo, Sign::Positive)?, resolve(hi, Sign::Negative)?) else {
        return Ok(None);
    };
    Ok(Some((lo, hi)))
}

/// Is `p` (already in the face's plane) within the face's region —
/// inside the outer loop and outside every ring? `OnBoundary` from any
/// loop is reported as `None` (graze). Each loop is read on its edges'
/// own carriers ([`point_in_loop_projected`]); a loop the walk can only
/// answer outside its reach is [`PointInSolidError::EdgeCarrierUnsupported`]
/// where `p` could land in it.
///
/// **Not the census's read, on purpose.** A vertex-on-face record and a
/// boolean's carry place a point through [`super::face_placement`],
/// which decides the point on the plane first and walks the face's
/// boundary before its interior, escalating a point in the band of a
/// vertex. This test is the ray walk's: `p` is a ray's hit on the face's
/// own plane, read against the loops' projection (an admitted operand's
/// face may hold a vertex off that plane by more than a tightened band),
/// and any boundary is `None`, a graze the walk retries on another ray.
/// Both walk each loop on the one core
/// (`splitting::containment::carrier_walk`); they differ around it, on
/// whether the point is decided on the plane and on how a boundary is
/// read.
///
/// # Panics
///
/// Where a loop `face` lists does not resolve (D2 row 4): a torn outer
/// loop is not an empty one. The links hold at rest and, on the
/// reduction's working copies, by [`crate::live::OPERATORS_KEEP_LINKS`].
pub(crate) fn point_in_face<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    normal: Vec3<T>,
    p: Point3<T>,
    band: Band,
) -> Result<Option<bool>, PointInSolidError> {
    let f = body
        .get_face(face)
        .ok_or(PointInSolidError::CorruptFace { face })?;
    // An empty outer loop bounds no region (mid-op scaffolding never
    // reaches this query; treat as no-hit).
    let boundary =
        |lk, field| linked(&body.loops, lk, EntityId::Loop, EntityId::Face(face), field).boundary;
    if matches!(boundary(f.outer, "outer"), LoopBoundary::Empty { .. }) {
        return Ok(Some(false));
    }
    let region = |lk| -> Result<LoopContainment, PointInSolidError> {
        point_in_loop_projected(body, lk, normal, p, band).map_err(|e| match e {
            PointInLoopError::Uncrossable(cause) => {
                PointInSolidError::EdgeCarrierUnsupported { face, cause }
            }
            e => PointInSolidError::Loop(e),
        })
    };
    match region(f.outer)? {
        LoopContainment::Out => return Ok(Some(false)),
        LoopContainment::OnBoundary => return Ok(None),
        LoopContainment::In => {}
    }
    for &ring in &f.rings {
        if matches!(boundary(ring, "rings"), LoopBoundary::Empty { .. }) {
            continue; // a lone ring vertex excludes no area
        }
        match region(ring)? {
            LoopContainment::In => return Ok(Some(false)),
            LoopContainment::OnBoundary => return Ok(None),
            LoopContainment::Out => {}
        }
    }
    Ok(Some(true))
}

/// Trilean containment of `q` in `body`'s material (module docs).
/// Every face of every shell participates (multi-shell bodies and
/// complements answer correctly by the signed count).
///
/// One of the door's two entries over the one counting core
/// ([`point_in_faces`]): this one hands the core every face of the
/// body in arena order; [`point_in_solid_of`] hands it one solid's. A
/// caller probing one body at many points holds
/// [`SolidFaces::of_body`] and asks [`point_in_solid_faces`], so the
/// body's side at infinity is read once.
///
/// # Errors
///
/// [`PointInSolidError`] — escalation, ray exhaustion, or a
/// non-planar/corrupt face.
pub fn point_in_solid<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    q: Point3<T>,
    band: Band,
    tol: Tol,
) -> Result<SolidContainment, PointInSolidError> {
    point_in_faces(body, &SolidFaces::of_body(body)?, q, band, tol)
}

/// [`point_in_solid`]'s selection read as given ([`SolidFaces::read_as_given`]):
/// an operand a caller handed in, whose side at infinity reading `In`
/// refuses [`PointInSolidError::InsideOut`]. Selected once and probed at
/// every point through [`point_in_solid_faces`].
///
/// # Errors
///
/// [`PointInSolidError::CorruptFace`] for a face whose chart does not
/// group.
pub(crate) fn operand_faces<T: Decide>(body: &Body<T>) -> Result<SolidFaces, PointInSolidError> {
    Ok(SolidFaces::of_body(body)?.read_as_given())
}

/// Trilean containment of `q` in the material of ONE solid of a
/// multi-solid body: the faces of `solid`'s shells, in face-arena
/// order, through the same counting core as [`point_in_solid`] —
/// the census's instance-containment arm asks this of a container
/// while the other instances of the arena are not part of the
/// question. The at-infinity side is read off the sign of THIS solid's
/// signed volume (the selection's faces), not the body's.
///
/// # Errors
///
/// [`PointInSolidError`] — the core's, plus
/// [`PointInSolidError::NoSuchSolid`] for a key the body does not hold
/// and [`PointInSolidError::ZeroVolumeBody`] for a solid with no faces.
pub fn point_in_solid_of<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    solid: SolidKey,
    q: Point3<T>,
    band: Band,
    tol: Tol,
) -> Result<SolidContainment, PointInSolidError> {
    let sel = SolidFaces::of(body, solid)?;
    point_in_solid_faces(body, &sel, q, band, tol)
}

/// One solid's faces, selected once and probed many times — what the
/// census's containment arm holds per ordering, so the selection and
/// its chart grouping are paid once per pair rather than once per
/// vertex. [`point_in_solid_of`] is this selection followed by one
/// [`point_in_solid_faces`].
///
/// **The selection is the probe's scope.** The arms that read a surface
/// GROUP — the cone, the closed sphere, the torus — group the
/// SELECTION's faces by key and act for a representative among them: a
/// chart is body-wide, and a wearer outside the selection bounds
/// material the query is not about.
#[derive(Clone, Debug)]
pub struct SolidFaces {
    faces: Vec<FaceKey>,
    pub(crate) charts: ChartGroups,
    /// The side a ray that crosses nothing reads, when the selection's
    /// role is already decided ([`SolidFaces::with_role`]); `None`
    /// reads it ([`crate::side_reader::side_at_infinity`]).
    at_infinity: Option<SolidContainment>,
    /// Whether the selection is read as given ([`SolidFaces::read_as_given`]):
    /// its side at infinity reading `In` refuses.
    as_given: bool,
    /// The side at infinity off the selection's signed volume, read once.
    volume: VolumeReads,
}

impl SolidFaces {
    /// One solid's faces in face-arena order — selected by the faces'
    /// own back-pointers through [`Body::faces_of_solid`], not by a walk
    /// of [`crate::Solid::shells`] (that door's doc carries why the two
    /// orders differ).
    ///
    /// # Errors
    ///
    /// [`PointInSolidError::NoSuchSolid`] for a key the body does not
    /// hold. An empty selection is not refused here: the probe answers
    /// [`PointInSolidError::ZeroVolumeBody`] for it.
    pub fn of<T: Decide>(body: &Body<T>, solid: SolidKey) -> Result<Self, PointInSolidError> {
        let faces = body
            .faces_of_solid(solid)
            .ok_or(PointInSolidError::NoSuchSolid { solid })?;
        Self::select(body, faces)
    }

    /// Every face of the body in face-arena order: [`point_in_solid`]'s
    /// selection, for a caller probing the whole body at many points
    /// through [`point_in_solid_faces`], which reads the body's side at
    /// infinity once for all of them.
    ///
    /// # Errors
    ///
    /// [`PointInSolidError::CorruptFace`] for a face whose chart does not
    /// group.
    pub fn of_body<T: Decide>(body: &Body<T>) -> Result<Self, PointInSolidError> {
        Self::select(body, body.faces().map(|(k, _)| k).collect())
    }

    /// One SHELL's faces in face-arena order — selected by the faces' own `shell`
    /// back-pointers, so the probe reads the material that shell ALONE
    /// bounds: for a cavity wall, whose faces point into the cavity,
    /// that is everything outside the cavity (the complement, read
    /// through the probe's at-infinity side exactly as a reverted
    /// operand is).
    ///
    /// A key the body does not hold selects no face, which the probe
    /// answers [`PointInSolidError::ZeroVolumeBody`].
    pub(crate) fn of_shell<T: Decide>(body: &Body<T>, shell: crate::entity::ShellKey) -> Self {
        let live: Vec<_> = body.faces().filter(|(_, d)| d.shell == shell).collect();
        Self {
            faces: live.iter().map(|&(k, _)| k).collect(),
            charts: ChartGroups::of_live(live),
            at_infinity: None,
            as_given: false,
            volume: VolumeReads::default(),
        }
    }

    /// `faces` as a selection, grouped by chart within itself.
    pub(crate) fn select<T: Decide>(
        body: &Body<T>,
        faces: Vec<FaceKey>,
    ) -> Result<Self, PointInSolidError> {
        let charts = ChartGroups::within(body, faces.iter().copied())
            .map_err(|face| PointInSolidError::CorruptFace { face })?;
        Ok(Self {
            faces,
            charts,
            at_infinity: None,
            as_given: false,
            volume: VolumeReads::default(),
        })
    }

    /// This selection with its role already decided — an `Outer`
    /// boundary leaves infinity outside its material, a `Void` inside —
    /// so a ray that crosses nothing reads that side instead of reading
    /// it again ([`crate::side_reader::side_at_infinity`]). The role is the same fact
    /// either way; the reader that decided it (`crate::stands::ShellRead`)
    /// hands it over.
    pub(crate) fn with_role(self, role: crate::props::ShellRole) -> Self {
        let side = match role {
            crate::props::ShellRole::Outer => SolidContainment::Out,
            crate::props::ShellRole::Void => SolidContainment::In,
        };
        Self {
            at_infinity: Some(side),
            ..self
        }
    }

    /// This selection **read as given**: the material of a body a caller
    /// handed in, which a closed outward boundary bounds from outside, so
    /// the side at infinity is outside it. A ray that crosses nothing
    /// reading `In` then refuses [`PointInSolidError::InsideOut`] rather
    /// than answering for the complement. A complement the kernel builds
    /// for itself (∖'s transient `revert(B)`) is not read as given.
    pub(crate) fn read_as_given(self) -> Self {
        Self {
            as_given: true,
            ..self
        }
    }

    /// The selected faces, in face-arena order.
    pub fn faces(&self) -> &[FaceKey] {
        &self.faces
    }
}

/// Trilean containment of `q` in the material the selection's faces
/// bound — [`point_in_solid_of`] with the selection made once.
///
/// # Errors
///
/// The core's ([`point_in_solid`]'s), plus
/// [`PointInSolidError::ZeroVolumeBody`] for an empty selection.
pub fn point_in_solid_faces<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    sel: &SolidFaces,
    q: Point3<T>,
    band: Band,
    tol: Tol,
) -> Result<SolidContainment, PointInSolidError> {
    point_in_faces(body, sel, q, band, tol)
}

/// Each face of a selection with its resolved query geometry, built once
/// per query by the boundary pre-pass, in the selection's order: every
/// ray of the schedule and every aimed ray read the same list, so a
/// face's region is read off its boundary once rather than at every ray.
pub(crate) type FaceGeos<T> = Vec<(FaceKey, FaceGeo<T>)>;

/// The one counting core behind both entries: the boundary
/// pre-pass and the schedule sweep over exactly `faces` (module docs).
/// An empty selection answers [`PointInSolidError::ZeroVolumeBody`]
/// before any predicate runs, and it is the same state that variant
/// names: no face means no enclosure, and the at-infinity fold would
/// read the volume of nothing as exactly zero — there is no material
/// side to classify against.
fn point_in_faces<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    sel: &SolidFaces,
    q: Point3<T>,
    band: Band,
    tol: Tol,
) -> Result<SolidContainment, PointInSolidError> {
    let faces = sel.faces();
    if faces.is_empty() {
        return Err(PointInSolidError::ZeroVolumeBody);
    }
    // ---- Boundary pre-pass: q on any face ⇒ OnBoundary. ----
    // Each face's geometry is built as the pass reaches it, so a point on
    // an earlier face answers before a later face is resolved.
    let mut geos: FaceGeos<T> = Vec::with_capacity(faces.len());
    for &face in faces {
        let escalate = |diag| PointInSolidError::Escalated { face, diag };
        let geo = match face_geo(body, face, &sel.charts) {
            Ok(geo) => geo,
            // **A point past the whole selection's reach**, read only
            // where the ray test cannot cross a face of it: definitely
            // outside the hull of every face's certified reach, it
            // leaves along a straight path that meets no face, and takes
            // the side at infinity. Outside each face's own reach is not
            // enough — the centre of a box solid lies outside every
            // face's slab and inside the solid.
            Err(refusal @ PointInSolidError::KindUnsupported { .. }) => {
                if !past_every_reach(body, faces, q, band) {
                    return Err(refusal);
                }
                return match missed_side(body, sel, band, tol, &MissedRead::new()) {
                    Ok(side) => Ok(side),
                    Err(RayFault::InBand(e) | RayFault::Blocked(e) | RayFault::Fatal(e)) => Err(e),
                    Err(RayFault::Graze | RayFault::Unread) => Err(refusal),
                };
            }
            Err(other) => return Err(other),
        };
        geos.push((face, geo));
        let geo = &geos[geos.len() - 1].1;
        match *geo {
            FaceGeo::Plane(origin, normal) => {
                // Orientation-free: a residual compared against Zero
                // answers the same whichever way the normal points,
                // and `point_in_face` below is ray parity (ditto).
                let elev = (q - origin).dot(normal);
                match decide("bool_point_in_solid_plane", Margin::of(elev), band) {
                    // In-plane: ON the boundary iff within the face
                    // region (a loop-boundary graze is also ON).
                    Ok(Sign::Zero) => match point_in_face(body, face, normal, q, band)? {
                        Some(true) | None => return Ok(SolidContainment::OnBoundary),
                        Some(false) => {}
                    },
                    Ok(_) => {}
                    // In band of the carrier, which is a question about
                    // this face only where q's foot on the plane is in
                    // the face or on its boundary. A foot definitely
                    // outside the face puts q off it whatever its
                    // elevation; otherwise the elevation is the answer.
                    Err(diag) => match point_in_face(body, face, normal, q - normal * elev, band) {
                        Ok(Some(false)) => {}
                        Ok(Some(true) | None) => return Err(escalate(diag)),
                        Err(e) if e.in_band() => return Err(escalate(diag)),
                        Err(e) => return Err(e),
                    },
                }
            }
            // The wall arms: the linearized residual (metres) — the same
            // form the certification layer classifies — then, on the
            // carrier, the face's own region says whether `q` is on THIS
            // face, as on the trimmed sphere.
            FaceGeo::Cylinder {
                origin,
                axis,
                radius,
                ref region,
                ..
            } => {
                if decide(
                    "bool_point_in_solid_plane",
                    Margin::of(wall_elevation(origin, axis, radius, q)),
                    band,
                )
                .map_err(escalate)?
                    == Sign::Zero
                {
                    match region.contains(q, band).map_err(|e| e.of_face(face))? {
                        Some(true) | None => return Ok(SolidContainment::OnBoundary),
                        Some(false) => {}
                    }
                }
            }
            // The cone arm: the residual is the point's EXACT distance to
            // the double cone ([`geom_brep::cone_elevation`]), a signed
            // length outright that needs no linearization, unlike the
            // cylinder and sphere residuals. A point on the mirror nappe
            // is on the carrier, and the region puts it off the face.
            FaceGeo::Cone {
                apex,
                axis,
                half_angle,
                ref region,
                ..
            } => {
                let elev = geom_brep::cone_elevation(apex, axis, half_angle, None, q);
                if decide("bool_point_in_solid_plane", Margin::of(elev), band).map_err(escalate)?
                    == Sign::Zero
                {
                    match region.contains(q, band).map_err(|e| e.of_face(face))? {
                        Some(true) | None => return Ok(SolidContainment::OnBoundary),
                        Some(false) => {}
                    }
                }
            }
            // On the carrier, `q` may be on the face or off it, and which
            // is the region this face has none of.
            FaceGeo::PartialCone {
                apex,
                axis,
                half_angle,
                reach,
            } => {
                let elev = geom_brep::cone_elevation(apex, axis, half_angle, None, q);
                if decide("bool_point_in_solid_plane", Margin::of(elev), band).map_err(escalate)?
                    == Sign::Zero
                    && decide(
                        "bool_cone_partial_reach",
                        Margin::of((q - apex).norm() - reach),
                        band,
                    )
                    .map_err(escalate)?
                        != Sign::Positive
                {
                    return Err(PointInSolidError::PartialConeFace { face });
                }
            }
            FaceGeo::PartialWall {
                origin,
                axis,
                radius,
                anchor,
                reach,
            } => {
                if decide(
                    "bool_point_in_solid_plane",
                    Margin::of(wall_elevation(origin, axis, radius, q)),
                    band,
                )
                .map_err(escalate)?
                    == Sign::Zero
                    && decide(
                        "bool_wall_outline_reach",
                        Margin::of((q - anchor).norm() - reach),
                        band,
                    )
                    .map_err(escalate)?
                        != Sign::Positive
                {
                    return Err(PointInSolidError::WallOutlineUnsupported { face });
                }
            }
            // The full-sphere arm (M5 PR 9c): the linearized radial
            // residual, the same metre-valued form the cylinder arm
            // and the certification layer classify. A full chart
            // carries no trim, so a Zero residual IS boundary — there
            // is no second containment question to ask.
            FaceGeo::Sphere {
                center,
                radius,
                representative,
                sense: _, // a residual against Zero: orientation-free
            } => {
                if face != representative {
                    continue;
                }
                let elev =
                    ((q - center).norm_squared() - radius.powi(2)) / (T::from_f64(2.0) * radius);
                if decide("bool_point_in_solid_sphere", Margin::of(elev), band).map_err(escalate)?
                    == Sign::Zero
                {
                    return Ok(SolidContainment::OnBoundary);
                }
            }
            // The TRIMMED sphere arm: a Zero radial residual puts `q`
            // on the CARRIER, and the face's own region then says
            // whether it is on THIS face (a point on the region's
            // boundary counts as ON, as it does for the cylinder wall).
            FaceGeo::SpherePatch {
                center,
                radius,
                ref region,
                sense: _, // a residual against Zero: orientation-free
            } => {
                let elev =
                    ((q - center).norm_squared() - radius.powi(2)) / (T::from_f64(2.0) * radius);
                if decide("bool_point_in_solid_sphere", Margin::of(elev), band).map_err(escalate)?
                    == Sign::Zero
                {
                    match region.contains(q, band).map_err(|e| e.of_face(face))? {
                        Some(true) | None => return Ok(SolidContainment::OnBoundary),
                        Some(false) => {}
                    }
                }
            }
            // The torus wall arms: the residual is the point's EXACT
            // signed distance to the tube. A ring torus is the set of
            // points whose meridian-plane offset `(ρ − R, h)` has norm
            // `r`, and that offset is the point's own displacement from
            // the spine circle, so `|(ρ − R, h)| − r` is a signed length
            // outright — like the cone's residual and unlike the
            // cylinder's and sphere's, which are `/2r` linearizations.
            // A closed group carries no region, so a Zero residual IS
            // boundary, as on the full sphere.
            FaceGeo::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                representative,
                ..
            } => {
                if face != representative {
                    continue;
                }
                let elev = torus_elevation(center, axis, major_radius, minor_radius, q);
                if decide("bool_point_in_solid_plane", Margin::of(elev), band).map_err(escalate)?
                    == Sign::Zero
                {
                    return Ok(SolidContainment::OnBoundary);
                }
            }
            // The face's own region then says whether a point on the
            // carrier is on THIS face, as on the trimmed sphere.
            FaceGeo::TorusPatch {
                center,
                axis,
                major_radius,
                minor_radius,
                ref region,
                ..
            } => {
                let elev = torus_elevation(center, axis, major_radius, minor_radius, q);
                if decide("bool_point_in_solid_plane", Margin::of(elev), band).map_err(escalate)?
                    == Sign::Zero
                {
                    match region.contains(q, band).map_err(|e| e.of_face(face))? {
                        Some(true) | None => return Ok(SolidContainment::OnBoundary),
                        Some(false) => {}
                    }
                }
            }
        }
    }

    // ---- Closest-hit ray sweep over the fixed schedule. ----
    let reach = selection_reach(body, faces, q)?;
    // A ray that crosses nothing reads the at-infinity side
    // (`missed_side`), which says how the ray's reading ranks.
    let read = MissedRead::new();
    ray_walk::walk(
        &SCHEDULE,
        |r| {
            let d = r.map(T::from_f64).normalize();
            match cast_ray(body, &geos, q, d, reach, band)? {
                Some(side) => Ok(side),
                None => missed_side(body, sel, band, tol, &read),
            }
        },
        || PointInSolidError::RayExhausted,
    )
}

/// A ray's hit as a face's region reads it: `Some` its side, `None` on
/// the face's boundary or where the region's paths could not place it,
/// so the fold grazes on it only where it is closest.
///
/// # Errors
///
/// An in-band reading sets the ray aside; an edge whose reading ran out
/// at this hit blocks it (another ray may miss it); an arc wound past a
/// turn refuses the query.
fn region_hit(
    face: FaceKey,
    reading: Result<Option<bool>, RegionRefusal>,
) -> Result<Option<bool>, RayFault<PointInSolidError>> {
    match reading {
        Ok(side) => Ok(side),
        Err(RegionRefusal::RayExhausted { .. }) => Ok(None),
        Err(e @ RegionRefusal::Escalated(_)) => Err(RayFault::InBand(e.of_face(face))),
        Err(e @ RegionRefusal::WoundPastPeriod) => Err(RayFault::Fatal(e.of_face(face))),
        Err(e @ RegionRefusal::Unreadable(_)) => Err(RayFault::Blocked(e.of_face(face))),
    }
}

/// A refusal from a trim test at a ray's hit point, as a reading of
/// that ray: in band of the face's boundary there, or a walk there that
/// no ray of its own settled, is about where this ray lands; a face or
/// edge the test cannot read there blocks this ray, and another may
/// miss it. A face or carrier the door has no arm for is about the
/// body.
///
/// A confined refusal reaches here only on definite decisions: each
/// test raises it where the hit is definitely within reach of what it
/// cannot read, and places a hit that only the band puts there on the
/// face's boundary (a `Zero`, which the count grazes on where it
/// reaches it) — so a block is one no tolerance moves
/// ([`RayFault::Blocked`]). [`PointInSolidError::RootCountDisagrees`],
/// the other confined refusal, is a reading of the ray itself and blocks
/// it where it is raised ([`torus_crossings`]), never here.
fn at_hit(e: PointInSolidError) -> RayFault<PointInSolidError> {
    if e.in_band() {
        RayFault::InBand(e)
    } else if e.confined() {
        RayFault::Blocked(e)
    } else {
        RayFault::Fatal(e)
    }
}

/// **How far from `q` any loop of the selection reaches**: the radius of
/// a ball about `q` holding every boundary loop of every face in
/// `faces` ([`loop_extent_from`]). It asks no decision. It is the lever
/// the ray's two skip questions are metered over (the plane arm's
/// `d·n̂`, the wall arm's axis-parallel rung): a ray that drifts off a
/// face's plane, or off a wall's axis, by no more than the band's zero
/// over this whole length meets no point of the face when `q` starts
/// definitely further off the carrier than that ([`clear_of_carrier`]),
/// so skipping it there is sound. A planar face lies in its outer loop's hull, and a wall face
/// in the hull of its loops (each of its points is on a ruling segment
/// between two boundary points), so each lies in this ball. A sphere or
/// a torus face bulges past its loops' hull, by up to its radius (the
/// tube's, for a torus), so this ball need not hold it: its arms meter
/// their own crossings over their own extents, and the side-at-infinity
/// reader, which needs a start beyond every face, widens this reach by
/// each such face's whole ball (`side_reader`'s `beyond`).
pub(crate) fn selection_reach<T: Decide>(
    body: &Body<T>,
    faces: &[FaceKey],
    q: Point3<T>,
) -> Result<T, PointInSolidError> {
    let mut reach = T::zero();
    for &face in faces {
        let f = body
            .get_face(face)
            .ok_or(PointInSolidError::CorruptFace { face })?;
        for l in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
            reach = reach.max(loop_extent_from(body, l, q)?);
        }
    }
    Ok(reach)
}

/// The crossing sign implied by a CHART-outward direction on a face
/// whose orientation sense may reverse it (S10): `d·n̂_outward` has the
/// opposite sign to `d·n̂_chart` exactly when the face's `sense` is
/// `false`. Used by the curved doors, which take their outward sign at
/// each hit and so have no stored normal to fold it into (the plane door gets it from [`face_geo`]).
///
/// Exact structure, not a numeric decision: a `bool` selects between
/// two enum values — no comparison, no tolerance, nothing for the
/// k-lint. `Zero` is fixed: a grazing incidence grazes from either
/// side, so the retry it triggers is sense-independent.
const fn oriented(sign: Sign, sense: bool) -> Sign {
    if sense {
        sign
    } else {
        match sign {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
            Sign::Zero => Sign::Zero,
        }
    }
}

/// What [`line_wall_roots`] found. The three non-root answers are kept
/// APART rather than collapsed into one "no roots": each caller owes a
/// different thing to each of them — a ray retries a tangent and skips
/// a miss, an edge sweep refuses a tangent and clears a miss — and a
/// merged variant would let one caller silently inherit the other's
/// posture.
#[derive(Debug, Clone, Copy)]
pub(super) enum WallRoots<T> {
    /// The line is parallel to the axis: its residual is constant, so
    /// the quadratic degenerates and there is no root to report.
    AxisParallel,
    /// The discriminant is in the zero band — the line is TANGENT to
    /// the wall, and a tangency is not a crossing at any order this
    /// function can see.
    Tangent,
    /// A definitely negative discriminant: no real root.
    Miss,
    /// Two definite roots of the carrier's own parameter, ascending.
    Two([T; 2]),
}

/// The certified roots of the LINE `q + d·t` against the INFINITE
/// cylinder wall `(origin, axis, radius)` — the quadratic in metres
/// that the ray lane has solved since M5 PR 9, factored out so the
/// edge-sweep lane solves the same one.
///
/// **Roots, not hits.** Trimming to a face, ordering by advance and
/// counting them are the RAY's concerns and stay in
/// [`cast_ray`]; a line-edge span needs both roots, unordered by sign,
/// and folding them here would have made this the ray's function with
/// a second caller rather than a shared primitive.
///
/// **Both trileans are lengths.**
///
/// - **`bool_point_in_solid_denom`**, the axis-parallel rung: `|d⊥|`,
///   the line's speed off the axis, levered by `span`, the run of the
///   line's own parameter over which the caller reads roots. Their
///   product is how far the line drifts off its distance from the axis
///   over that run. Zero ⇒ it drifts less than the band, so the residual
///   is constant there and no root lands in the run.
/// - **`bool_ray_cylinder_disc`**: the discriminant over `|d⊥|²` is
///   the squared half-chord in the plane across the axis, `r² − p²` for
///   a line passing `p` from it, and over `2r` it is the depth the line
///   reaches inside the circle, the sphere arm's form
///   ([`line_sphere_roots`]). Positive ⇒ two definite roots; Zero ⇒
///   tangent; Negative ⇒ definite miss.
///
/// # Errors
///
/// [`WallRootFault`] — an in-band axis-parallel test or an in-band
/// discriminant, with the rung that escalated. The caller wraps it in
/// its own error type.
#[allow(clippy::too_many_arguments)] // the line, the wall, the run and the band, each named
pub(super) fn line_wall_roots<T: Decide>(
    q: Point3<T>,
    d: Vec3<T>,
    origin: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    span: T,
    band: Band,
) -> Result<WallRoots<T>, WallRootFault> {
    let w0 = q - origin;
    let w0p = w0 - axis * w0.dot(axis);
    let dp = d - axis * d.dot(axis);
    let a2 = dp.norm_squared();
    let two_r = T::from_f64(2.0) * radius;
    match decide(
        "bool_point_in_solid_denom",
        Margin::levered(dp.norm(), span),
        band,
    )
    .map_err(|diag| WallRootFault {
        rung: WallRung::AxisParallel,
        diag,
    })? {
        Sign::Positive => {}
        _ => return Ok(WallRoots::AxisParallel),
    }
    let b2 = w0p.dot(dp);
    let c2 = w0p.norm_squared() - radius.powi(2);
    let disc = b2.powi(2) - a2 * c2;
    match decide(
        "bool_ray_cylinder_disc",
        Margin::over_lever(disc / a2, two_r),
        band,
    )
    .map_err(|diag| WallRootFault {
        rung: WallRung::Discriminant,
        diag,
    })? {
        Sign::Positive => {}
        Sign::Zero => return Ok(WallRoots::Tangent),
        Sign::Negative => return Ok(WallRoots::Miss),
    }
    Ok(WallRoots::Two(quadratic_roots(a2, b2, c2, disc)))
}

/// **Which rung of [`line_wall_roots`] escalated**: the two questions
/// the line × wall quadratic asks before it has roots. Both margins are
/// lengths ([`line_wall_roots`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum WallRung {
    /// Whether the line runs parallel to the wall's axis
    /// (`bool_point_in_solid_denom`, `|d⊥|` levered by the run the
    /// caller reads): a positive margin has roots to find, a zero one a
    /// residual constant over that run.
    AxisParallel,
    /// Whether the line crosses the wall, grazes it or misses it
    /// (`bool_ray_cylinder_disc`, `disc/|d⊥|²` over `2r`).
    Discriminant,
}

/// An escalation of [`line_wall_roots`], with the rung that raised it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallRootFault {
    /// The rung that could not decide.
    pub rung: WallRung,
    /// Its diagnostics.
    pub diag: Indeterminate,
}

/// **The line × cone quadratic**: the cone's quadric form along the
/// line `q + d·t`, negated, `(w·â)² − |w|²·cos²α = A t² + 2B t + C` with
/// `w = q + d·t − apex` — `A` in `d`'s units squared, `B` in metres
/// times them, `C` in m². Its zero set is the DOUBLE cone; which nappe
/// a root lands on is the caller's question. `A = (d·â)² − |d|²cos²α`
/// vanishes exactly when the line runs parallel to a generator. Each
/// square straddling zero is a `powi(2)`, the tight square.
pub(super) fn line_cone_quadratic<T: Decide>(
    q: Point3<T>,
    d: Vec3<T>,
    apex: Point3<T>,
    axis: Vec3<T>,
    half_angle: T,
) -> [T; 3] {
    let cos2 = half_angle.cos().powi(2);
    let w0 = q - apex;
    let (da, wa) = (d.dot(axis), w0.dot(axis));
    [
        da.powi(2) - d.norm_squared() * cos2,
        da * wa - w0.dot(d) * cos2,
        wa.powi(2) - w0.norm_squared() * cos2,
    ]
}

/// The certified roots of the LINE `q + d·t` against the sphere
/// `(center, radius)`: `|d|²t² + 2(w·d)t + (|w|² − r²) = 0`, `w = q − c`.
/// `d` need not be unit — a ray's is, and a `Line` carrier's is by a
/// convention nothing checks — so the roots are in `d`'s own parameter.
///
/// The discriminant is metered as a LENGTH: `disc/|d|²` is the squared
/// half-chord in metres, so `disc/(|d|²·2r)` is the D4 ¶1-honest
/// margin, the form [`line_wall_roots`] takes across the axis too.
///
/// A line is never parallel to a sphere in the sense a wall's ruling
/// is, so [`WallRoots::AxisParallel`] is never answered, and no line
/// lies on a sphere: two distinct roots certify the line is off it.
///
/// # Errors
///
/// [`geom_core::Indeterminate`] — an in-band discriminant decision the
/// band cannot call, or a zero-length `d` (no line at all). The caller
/// wraps it in its own error type.
pub(super) fn line_sphere_roots<T: Decide>(
    q: Point3<T>,
    d: Vec3<T>,
    center: Point3<T>,
    radius: T,
    band: Band,
) -> Result<WallRoots<T>, geom_core::Indeterminate> {
    let w0 = q - center;
    let a2 = d.norm_squared();
    let b2 = w0.dot(d);
    let c2 = w0.norm_squared() - radius.powi(2);
    let disc = b2.powi(2) - a2 * c2;
    let two_r = T::from_f64(2.0) * radius;
    match decide(
        "bool_ray_sphere_disc",
        Margin::over_lever(disc / a2, two_r),
        band,
    )? {
        Sign::Positive => {}
        Sign::Zero => return Ok(WallRoots::Tangent),
        Sign::Negative => return Ok(WallRoots::Miss),
    }
    Ok(WallRoots::Two(quadratic_roots(a2, b2, c2, disc)))
}

/// The certified real roots of the LINE `q + d·t` (with `d` a UNIT
/// direction) against the torus `(center, axis, R, r)`.
///
/// # The quartic, and why its leading coefficient is not a posture
///
/// With `w = q − c + d·t`, `h = w·â` and `ρ² = |w|² − h²`, the torus is
/// the zero set of `F = (|w|² + R² − r²)² − 4R²ρ²`. Substituting the ray
/// and depressing by `t → y − b` (with `b = (q−c)·d`) gives the DEPRESSED
/// MONIC quartic `y⁴ + p y² + q̂ y + s` with, writing `u` for the
/// component of `q − c` perpendicular to `d`, `m = |u|²`, `n = u·â`,
/// `e = d·â` and `M = m + R² − r²`:
///
/// ```text
/// p = 2M − 4R²(1 − e²)      q̂ = 8R²·e·n      s = M² − 4R²(m − n²)
/// ```
///
/// **The ladder that solves it is [`depressed_quartic_roots`]**, whose
/// docs state its count, its band and its rungs; what is the RAY's is
/// stated here. A reader arriving from the cone arm will look for the
/// vanishing leading coefficient that arm makes a posture of, and there
/// is none to find: `d` is a unit direction, so the leading coefficient
/// is `|d|⁴` and the degree cannot drop. At `f64` that is exactly 1; at
/// the `Interval` scalar it is an enclosure of 1, a few ulps wide, since
/// `normalize` cannot round to a unit vector exactly — **bounded away
/// from zero either way**, which is all the monic form needs. The lever
/// is `ext = R + r`.
///
/// What the ray's symmetry alignments degrade is the root structure,
/// and they land on the ladder's rungs (rows `bool_ray_torus_*`):
///
/// * a ray along the axis (`e = ±1`, where `1 − e² = 0` and the `t²`
///   coefficient loses its `R²` term) and a ray in the midplane
///   (`n = 0`, `q̂ = 0`) each merge roots for the poses that touch the
///   tube: rung 1, `bool_ray_torus_disc` Zero. Uncertain count — graze,
///   retry.
/// * every axis-parallel ray (`n = 0`) and every midplane ray (`e = 0`)
///   has `q̂ = 8R²·e·n` zero, the biquadratic arm (rung 2,
///   `bool_ray_torus_odd`); the four-root ray through the hole is in it.
/// * a ray all but perpendicular to the axis has a resolvent root
///   `≈ q̂²/c1` that can fall in rung 3's band (`bool_ray_torus_split_lead`).
/// * an [`Unsettled::CountDisagrees`] blocks that ray
///   ([`PointInSolidError::RootCountDisagrees`]) and another may settle.
///
/// **So this door refuses over a BAND around a tangency, and the band is
/// wide.** A caller reading "a tangent ray grazes" will assume a
/// measure-zero nuisance; it is not, for the same reason the cone arm's
/// apex refusal is not. Where the band surrounds a feature of the BODY —
/// the tube's top and bottom circles, whose tangent plane is
/// perpendicular to the axis — a near-horizontal ray from any point
/// within a few tenths of a millimetre of that circle on a unit-sized
/// torus reads in it at the default ε. `point_in_solid` sets such a ray
/// aside and another answers, so the band leaves no shell about the body
/// (`bool3_torus_doors::the_clamp_floor_clears_the_shell_about_the_top_circle`).
///
/// # Errors
///
/// [`RootRefusal::Escalated`] — an in-band classifying sign, which the
/// caller wraps in its own error type — and [`RootRefusal::Unsettled`],
/// a count the ladder does not certify.
pub(super) fn line_torus_roots<T: Decide>(
    q: Point3<T>,
    d: Vec3<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    major_radius: T,
    minor_radius: T,
    band: Band,
) -> Result<QuarticRoots<T>, RootRefusal> {
    let two = T::from_f64(2.0);
    let four = T::from_f64(4.0);
    let ext = major_radius + minor_radius;
    let w0 = q - center;
    let e = d.dot(axis);
    let b = w0.dot(d);
    // The component of `q − c` perpendicular to the ray: its norm is the
    // line's distance from the centre and its axial part is the height
    // at the line's closest approach.
    let perp = w0 - d * b;
    let m = perp.norm_squared();
    let n = perp.dot(axis);
    let rr = major_radius.powi(2);
    let big_m = m + rr - minor_radius.powi(2);
    let p = two * big_m - four * rr * (T::one() - e.powi(2));
    let q_hat = T::from_f64(8.0) * rr * e * n;
    let s = big_m.powi(2) - four * rr * (m - n.powi(2));
    match depressed_quartic_roots(p, q_hat, s, ext, &RAY_TORUS_ROWS, band)? {
        // Undo the depression: `t = y − b`.
        Roots::Certified(ys) => Ok(Roots::Certified(ys.map(|y| y - b))),
        Roots::Miss => Ok(Roots::Miss),
    }
}

/// The ray × torus lane's rows ([`line_torus_roots`]).
const RAY_TORUS_ROWS: QuarticRows = QuarticRows {
    disc: "bool_ray_torus_disc",
    shape: "bool_ray_torus_shape",
    depth: "bool_ray_torus_depth",
    odd: "bool_ray_torus_odd",
    split: "bool_ray_torus_split",
    split_lead: "bool_ray_torus_split_lead",
};

/// **What the torus arm reads off a ray's quartic** ([`line_torus_roots`]):
/// `None` a certified miss, `Some` the certified crossings. An uncertain
/// count grazes; a constructed count that disagrees with the certified
/// one blocks this ray ([`PointInSolidError::RootCountDisagrees`]): its
/// quartic is read wrong, and another ray's is another computation.
///
/// # Errors
///
/// [`RayFault::Graze`] and [`RayFault::Blocked`], as above.
fn torus_crossings<T: Decide>(
    face: FaceKey,
    roots: Result<QuarticRoots<T>, Unsettled>,
) -> Result<Option<Found<T, 4>>, RayFault<PointInSolidError>> {
    match roots {
        Ok(Roots::Miss) => Ok(None),
        Ok(Roots::Certified(found)) => Ok(Some(found)),
        Err(Unsettled::CountDisagrees {
            certified,
            constructed,
        }) => Err(RayFault::Blocked(PointInSolidError::RootCountDisagrees {
            face,
            certified,
            constructed,
        })),
        Err(_) => Err(RayFault::Graze),
    }
}

/// Is `q`, at elevation `elev` off `face`'s carrier, definitely further
/// from it than the band — so a ray running along the carrier within the
/// band's zero cannot reach it? `false` where the elevation is in the
/// zero band; an in-band elevation abandons the ray. An interval
/// enclosure that straddles a band edge decides nothing, and abandons it
/// too.
fn clear_of_carrier<T: Decide>(
    elev: T,
    face: FaceKey,
    band: Band,
    rows: RayRows,
) -> Result<bool, RayFault<PointInSolidError>> {
    match decide(rows.clearance, Margin::of(elev.abs()), band) {
        Ok(Sign::Zero) => Ok(false),
        Ok(_) => Ok(true),
        Err(diag) => Err(RayFault::InBand(PointInSolidError::Escalated {
            face,
            diag,
        })),
    }
}

/// Does the ray from `q` along `d`, which runs along planar `face`'s
/// carrier (normal `normal`) within the band, pass definitely beside the
/// face? The ray lies in the plane through `q` spanned by `d` and
/// `normal`, whose normal is `m = normal × d`; the face lies in the hull
/// of its outer loop's vertices and its curved edges' balls
/// ([`loop_hull`](crate::splitting::containment::loop_hull)), so when
/// every one of them is definitely on one side of that plane, no point
/// of the face is on the ray. `bool_point_in_solid_beside` decides the
/// larger of the two sides' clearances; anything but Positive abandons
/// the ray, and a loop that does not read is the body's refusal.
fn ray_passes_beside<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    q: Point3<T>,
    d: Vec3<T>,
    normal: Vec3<T>,
    band: Band,
    rows: RayRows,
) -> Result<bool, RayFault<PointInSolidError>> {
    let f = body
        .get_face(face)
        .ok_or(RayFault::Fatal(PointInSolidError::CorruptFace { face }))?;
    let (verts, balls) = crate::splitting::containment::loop_hull(body, f.outer)
        .map_err(|e| RayFault::Fatal(PointInSolidError::Loop(e)))?;
    let m = normal.cross(d);
    let m = m / m.norm();
    let (mut above, mut below) = (None::<T>, None::<T>);
    for (c, r) in verts.into_iter().map(|v| (v, T::zero())).chain(balls) {
        let h = (c - q).dot(m);
        let (a, b) = (h - r, T::zero() - h - r);
        above = Some(above.map_or(a, |x| x.min(a)));
        below = Some(below.map_or(b, |x| x.min(b)));
    }
    let (Some(above), Some(below)) = (above, below) else {
        return Ok(true); // no vertex: the loop bounds no region
    };
    Ok(decide(rows.beside, Margin::of(above.max(below)), band)
        .map_err(|diag| RayFault::InBand(PointInSolidError::Escalated { face, diag }))?
        == Sign::Positive)
}

/// **The names a ray's planar crossings are decided under**: a schedule
/// ray's ([`SCHEDULE_ROWS`]) or a ray aimed at the boundary from beyond
/// it ([`crate::side_reader::AIMED_ROWS`]), so the reader of the side at infinity, which
/// every at-rest gate runs, logs apart from point containment.
#[derive(Clone, Copy)]
pub(crate) struct RayRows {
    pub(crate) advance: &'static str,
    pub(crate) denom: &'static str,
    pub(crate) clearance: &'static str,
    pub(crate) beside: &'static str,
    pub(crate) order: &'static str,
    pub(crate) vertex: &'static str,
    pub(crate) edge: &'static str,
    pub(crate) edge_span: &'static str,
    pub(crate) winding: &'static str,
    pub(crate) represents: &'static str,
    pub(crate) reaches: &'static str,
}

/// A schedule ray's rows (module docs).
const SCHEDULE_ROWS: RayRows = RayRows {
    advance: "bool_point_in_solid_advance",
    denom: "bool_point_in_solid_denom",
    clearance: "bool_point_in_solid_clearance",
    beside: "bool_point_in_solid_beside",
    order: "bool_point_in_solid_order",
    vertex: "bool_point_in_solid_vertex",
    edge: "bool_point_in_solid_edge",
    edge_span: "bool_point_in_solid_edge_span",
    winding: "bool_point_in_solid_winding",
    represents: "bool_point_in_solid_met_edge_represents",
    reaches: "bool_point_in_solid_met_edge_reaches",
};

/// One ray of the sweep: the side its running count reads
/// ([`Crossings::first_count`]), or `None` where it crosses nothing
/// that counts.
fn cast_ray<T: Decide>(
    body: &Body<T>,
    geos: &[(FaceKey, FaceGeo<T>)],
    q: Point3<T>,
    d: Vec3<T>,
    reach: T,
    band: Band,
) -> Result<Option<SolidContainment>, RayFault<PointInSolidError>> {
    let side = ray_crossings(body, geos, q, d, reach, band, SCHEDULE_ROWS)?.first_count(
        SCHEDULE_ROWS.order,
        band,
        RayHit::count,
        |a, b| {
            a.pair(body, b, SCHEDULE_ROWS.order, band)
                .map_err(|f| f.map(|(face, diag, _)| PointInSolidError::Escalated { face, diag }))
        },
        |hits| {
            RayHit::unread(hits, |_, faces, diag| PointInSolidError::Escalated {
                face: faces[1],
                diag,
            })
        },
    )?;
    Ok(side.map(side_before))
}

/// The side of a ray's start read off the sign of its first nonzero
/// running count: a count that rises leaves material (`d·n > 0`), so
/// the start is `In`.
pub(crate) const fn side_before(outward: Sign) -> SolidContainment {
    match outward {
        Sign::Positive => SolidContainment::In,
        _ => SolidContainment::Out,
    }
}

/// **One crossing of a ray**: a face it crosses, or an edge it meets,
/// half a crossing for each face the edge bounds (D10, Booleans).
#[derive(Clone, Copy, Debug)]
pub(crate) struct RayHit<T> {
    /// `+1` leaving material, `−1` entering it, `0` an edge whose two
    /// faces the ray crosses opposite ways (a fold it passes by or
    /// through, either way crossing nothing net).
    pub(crate) count: i32,
    /// The face crossed, or the edge's first face.
    pub(crate) face: FaceKey,
    /// The edge's other face, for an edge met.
    pub(crate) other: Option<FaceKey>,
    /// Its advance along the ray, where it lies on its faces: a face's
    /// crossing, or where the ray passes an edge, within the band of
    /// both its faces.
    pub(crate) at: T,
    /// What the ray reads nothing at here ([`edge_meets`]), if anything:
    /// a mark counts nothing and is sideless, so it matters only where
    /// the count reaches it.
    pub(crate) mark: Option<Mark>,
}

/// What a ray meets that it reads nothing at ([`RayHit::mark`]).
#[derive(Clone, Copy, Debug)]
pub(crate) enum Mark {
    /// A vertex within the band of the ray, `gap` its distance.
    Corner(Indeterminate),
    /// A face the ray passes within the band of at two of its edges,
    /// narrower than the band there, `gap` the nearer edge's distance.
    Wedge(Indeterminate),
    /// A vertex on the ray, a wedge whose edges it passes through, or a
    /// crossing no reading here places.
    Graze,
}

impl<T: Copy> RayHit<T> {
    /// The hit's count.
    pub(crate) const fn count(&self) -> i32 {
        self.count
    }

    /// A face's crossing at `at`, counting `count`.
    const fn crossing(face: FaceKey, at: T, count: i32) -> Self {
        Self {
            count,
            face,
            other: None,
            at,
            mark: None,
        }
    }

    /// A mark at `at` ([`Mark`]) against `faces`.
    const fn marked(faces: [FaceKey; 2], at: T, mark: Mark) -> Self {
        Self {
            count: 0,
            face: faces[0],
            other: Some(faces[1]),
            at,
            mark: Some(mark),
        }
    }

    /// The faces it lies on.
    fn faces(&self) -> impl Iterator<Item = FaceKey> {
        core::iter::once(self.face).chain(self.other)
    }

    /// **Whether two hits of one cluster are a pair** (D10: the ray
    /// refuses only on a pair). A hit that counts nothing (a mark, or an
    /// edge the ray passes by or through without crossing) is no party.
    /// Two parties lie on their faces where the ray meets them (`at`: a
    /// face's crossing, or where it passes an edge, in band of both its
    /// faces), so two faces of theirs `t` apart are met about `t` apart
    /// along the ray, less what the band lets an edge's passage slide
    /// along it. Not so where the ray crosses their carriers: on a ray
    /// shallow to a face, its carrier crossing lands far off the face,
    /// beside another face (PR 5422's review R2). Two
    /// parties met in band of each other along the ray (decided on
    /// `order` as a magnitude) are a pair, or a tie that grazes, on any
    /// two of their faces that do not meet ([`crate::side_reader::meet`]).
    ///
    /// # Errors
    ///
    /// [`RayFault::InBand`] with the pair's first face, its gap and the
    /// other face; [`RayFault::Graze`] at a tie.
    pub(crate) fn pair<U: Decide>(
        &self,
        body: &Body<U>,
        other: &Self,
        order: &'static str,
        band: Band,
    ) -> Result<(), RayFault<(FaceKey, Indeterminate, FaceKey)>>
    where
        T: Decide,
    {
        if self.count == 0 || other.count == 0 {
            return Ok(());
        }
        let along = geom_core::k_stats::decide_magnitude(
            order,
            Margin::of((self.at - other.at).abs()),
            band,
        );
        if along == Ok(geom_core::k_stats::Magnitude::Positive) {
            return Ok(());
        }
        for a in self.faces() {
            for b in other.faces() {
                if crate::side_reader::meet(body, a, b) {
                    continue;
                }
                return Err(match along {
                    Err(diag) => RayFault::InBand((a, diag, b)),
                    Ok(_) => RayFault::Graze,
                });
            }
        }
        Ok(())
    }

    /// The fault a sideless hit the count reached reads, `unread` a
    /// corner's or a wedge's (`true` for a wedge): a corner or wedge in
    /// band of the ray, else a graze.
    pub(crate) fn unread<E>(
        hits: &[&Self],
        unread: impl FnOnce(bool, [FaceKey; 2], Indeterminate) -> E,
    ) -> RayFault<E> {
        match hits.iter().find_map(|h| {
            let faces = [h.face, h.other.unwrap_or(h.face)];
            match h.mark {
                Some(Mark::Corner(gap)) => Some((false, faces, gap)),
                Some(Mark::Wedge(gap)) => Some((true, faces, gap)),
                _ => None,
            }
        }) {
            Some((wedge, faces, gap)) => RayFault::InBand(unread(wedge, faces, gap)),
            None => RayFault::Graze,
        }
    }
}

/// A ray's crossings, each a [`RayHit`].
pub(crate) type RayCrossings<T> = Crossings<T, RayHit<T>>;

/// A planar face's plane as one ray crosses it: the advance, the
/// crossing point and the face's outward sign there.
struct PlaneCross<T: geom_core::Real> {
    face: FaceKey,
    normal: Vec3<T>,
    t: T,
    p: Point3<T>,
    outward: Sign,
}

/// **The edges a ray meets**, each decided once, for both faces it
/// bounds: a line edge between two crossed planes by the census's closest
/// read of two lines ([`edge_meets`]), any other by where the ray passes
/// it ([`curved_edges`]). `met` holds each edge it passes within the band
/// of; `lines` and `read` the edges decided, met or not; `clear` those a
/// face's crossing may be read past by the turn of its loops; `marks`
/// where along the ray it meets what it reads nothing at
/// ([`RayHit::mark`]). A seam inside a face bounds nothing and is none of
/// these.
struct EdgeMeets<T> {
    met: Vec<Met<T>>,
    /// The line edges between two planar faces decided, met or not.
    lines: std::collections::BTreeSet<crate::entity::EdgeKey>,
    /// Every edge decided, met or not: `lines` and the curved reading's
    /// ([`curved_edges`]).
    read: std::collections::BTreeSet<crate::entity::EdgeKey>,
    /// The edges a face's turn may be read past: every one of `lines`,
    /// met or not, as [`edge_meets`] inserts each pass before deciding
    /// its gap, and each curved edge read clear from every crossing of
    /// its faces, or whose face's carrier the ray definitely misses
    /// ([`curved_edges`]). A curved edge read past its span's end from
    /// one is not in it, and neither is a curved edge met. A met line
    /// here is harmless: the turn is read only for a face no edge is met
    /// at, so a face with a met line never takes it.
    clear: std::collections::BTreeSet<crate::entity::EdgeKey>,
    marks: Vec<RayHit<T>>,
    /// The vertices read so far, each once.
    vertices: Vec<crate::entity::VertexKey>,
}

/// How far the ray passes a met edge, as a wedge's mark reads it.
#[derive(Clone, Copy)]
enum Gap {
    /// Decided Zero.
    Zero,
    /// In the band's gap.
    InBand(Indeterminate),
}

impl Gap {
    /// The mark two met edges of one face read: a wedge where either gap
    /// is in band, else a graze.
    const fn mark(self, other: Self) -> Mark {
        match (self, other) {
            (Self::InBand(gap), _) | (Self::Zero, Self::InBand(gap)) => Mark::Wedge(gap),
            (Self::Zero, Self::Zero) => Mark::Graze,
        }
    }
}

/// **An edge the ray meets**, half a crossing for each of its faces: the
/// edge, its faces, the advance where the ray passes it, the span its
/// passage lies in along the ray, which carrier crossing of each face it
/// stands for, and how far the ray passes it.
#[derive(Clone, Copy)]
struct Met<T> {
    edge: crate::entity::EdgeKey,
    faces: [FaceKey; 2],
    at: T,
    span: (T, T),
    cross: [Cross; 2],
    gap: Gap,
}

/// Which carrier crossing of a face a met edge stands for: a planar
/// face's one plane crossing, or one root of a curved face's carrier
/// (an index into the ray's [`RootCross`]es).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cross {
    Plane,
    Root(usize),
}

/// **A root of a curved face's carrier** on the ray: the face, its
/// advance, and how its outward sign is read there ([`RootSide`]).
#[derive(Clone, Copy)]
struct RootCross<T> {
    face: FaceKey,
    t: T,
    side: RootSide,
    /// Its place among its face's roots: two faces on one carrier read
    /// the same roots in the same order.
    ordinal: usize,
}

/// How a root's outward sign is read: off the decided root order (a
/// wall's near root enters its chart, the far one leaves it; a sphere's
/// likewise), folded with the face's sense; or at the root, where it
/// counts (a cone's nappe and incidence, a torus's incidence).
#[derive(Clone, Copy)]
enum RootSide {
    Ordered(Sign),
    Cone,
    Torus,
}

/// The edges and vertices the ray `q + s·d` passes ([`EdgeMeets`]).
///
/// - **A vertex** of a decided edge within the band of the ray is a
///   corner, and one on it a graze, marked where the ray comes closest
///   to it. Its distance from the ray is enclosed by its rounding
///   charge ([`geom_core::rounding_charge`] over the lengths it is read
///   from), and the enclosure's lower end is decided, as an edge's gap
///   is.
/// - **An edge** whose lines' gap is not definitely past the band and
///   whose closest point lies definitely inside it is met. One whose
///   closest point lies definitely past an end is nearest the ray at
///   that end, which is read as its vertex. One whose closest point is
///   in band of an end is marked a graze there, which no reading here
///   places.
/// - **A wedge** is a face two met edges bound: it is narrower than the
///   band where the ray passes it, and both marks stand. The half rule
///   would count that face once at each edge, half a crossing each,
///   where the ray crosses it once or not at all, and read `In` outside
///   a turned needle (`r1s_witnesses_outside_a_turned_needle_read_outside`).
///   Two met edges at one vertex of different faces are no wedge: no
///   face is met twice.
///
/// A mark matters only where the count reaches it
/// ([`Crossings::first_count`]).
#[allow(clippy::too_many_arguments)] // the ray, the planes and the rows, no less
fn edge_meets<T: Decide>(
    body: &Body<T>,
    planes: &[PlaneCross<T>],
    q: Point3<T>,
    d: Vec3<T>,
    reach: T,
    band: Band,
    rows: RayRows,
) -> Result<EdgeMeets<T>, RayFault<PointInSolidError>> {
    let crossed = |f: FaceKey| planes.iter().any(|c| c.face == f);
    let far = q + d * (reach * T::from_f64(2.0));
    let passes = crate::census::line_passes(body, (q, far), |_, a, b| crossed(a) && crossed(b));
    let mut meets = EdgeMeets {
        met: Vec::new(),
        lines: std::collections::BTreeSet::new(),
        read: std::collections::BTreeSet::new(),
        clear: std::collections::BTreeSet::new(),
        marks: Vec::new(),
        vertices: Vec::new(),
    };
    for pass in &passes {
        for v in pass.ends {
            meets.vertex(
                body,
                v,
                [pass.faces.0, pass.faces.1],
                (q, far),
                d,
                band,
                rows,
            )?;
        }
    }
    for pass in passes {
        meets.lines.insert(pass.edge);
        meets.read.insert(pass.edge);
        meets.clear.insert(pass.edge);
        let gap =
            match geom_core::k_stats::decide_magnitude(rows.edge, Margin::of(pass.gap[0]), band) {
                Ok(geom_core::k_stats::Magnitude::Positive) => continue,
                Ok(geom_core::k_stats::Magnitude::Zero) => Gap::Zero,
                Err(gap) => Gap::InBand(gap),
            };
        let faces = [pass.faces.0, pass.faces.1];
        let [lo, hi] = pass.along;
        let at = geom::mid_param(lo, hi);
        // The ray starts at `q`. Where its line comes closest to the
        // edge's line behind `q`, the ray itself is nearest the edge at
        // `q`, and the boundary pre-pass holds `q` clear of both the
        // edge's faces, so of the edge.
        if decide(rows.advance, Margin::of(at), band) == Ok(Sign::Negative) {
            continue;
        }
        // Where the lines come closest, against each end of the edge:
        // inside it, past it (the edge's nearest point to the ray is that
        // end, its vertex read above), or in band of it.
        let mut inside = true;
        for [lo, hi] in pass.reach {
            if decide(rows.edge_span, Margin::of(lo), band) == Ok(Sign::Positive) {
                continue;
            }
            inside = false;
            if decide(rows.edge_span, Margin::of(hi), band) != Ok(Sign::Negative) {
                meets.marks.push(RayHit::marked(faces, at, Mark::Graze));
            }
            break;
        }
        if !inside {
            continue;
        }
        meets.meet(Met {
            edge: pass.edge,
            faces,
            at,
            span: (at, at),
            cross: [Cross::Plane; 2],
            gap,
        });
    }
    Ok(meets)
}

impl<T: Decide> EdgeMeets<T> {
    /// **Reads vertex `v`** of an edge between `faces` against the ray
    /// segment `ray` along `d`, once per vertex: within the band of the
    /// ray it is a corner, on it a graze, marked where the ray comes
    /// closest to it. Its distance from the ray is enclosed by its
    /// rounding charge ([`geom_core::rounding_charge`] over the lengths
    /// it is read from), and the enclosure's lower end is decided, as an
    /// edge's gap is.
    #[allow(clippy::too_many_arguments)] // the vertex, its faces, the ray and the rows
    fn vertex(
        &mut self,
        body: &Body<T>,
        v: crate::entity::VertexKey,
        faces: [FaceKey; 2],
        (q, far): (Point3<T>, Point3<T>),
        d: Vec3<T>,
        band: Band,
        rows: RayRows,
    ) -> Result<(), RayFault<PointInSolidError>> {
        if self.vertices.contains(&v) {
            return Ok(());
        }
        self.vertices.push(v);
        let at = body
            .get_vertex(v)
            .and_then(|vx| body.get_point(vx.point))
            .copied()
            .ok_or(RayFault::Fatal(PointInSolidError::CorruptFace {
                face: faces[0],
            }))?;
        let gap = crate::sector_shape::point_segment_distance(q, far, at);
        // The distance is a short chain (differences, a projection, a
        // norm), each rounding under half an ulp of the lengths it is
        // read from: the kernel's one charge for such a chain.
        let low = gap - geom_core::rounding_charge((at - q).norm() + gap);
        let along = (at - q).dot(d);
        match geom_core::k_stats::decide_magnitude(
            rows.vertex,
            Margin::of(low.max(T::zero())),
            band,
        ) {
            Ok(geom_core::k_stats::Magnitude::Positive) => {}
            Ok(geom_core::k_stats::Magnitude::Zero) => {
                self.marks.push(RayHit::marked(faces, along, Mark::Graze));
            }
            Err(gap) => self
                .marks
                .push(RayHit::marked(faces, along, Mark::Corner(gap))),
        }
        Ok(())
    }

    /// **Records `met`**. A face it shares with an edge already met is a
    /// wedge: both are marked ([`Gap::mark`]).
    fn meet(&mut self, met: Met<T>) {
        if let Some(other) = self
            .met
            .iter()
            .find(|w| w.faces.iter().any(|f| met.faces.contains(f)))
            .copied()
        {
            let mark = met.gap.mark(other.gap);
            self.marks.push(RayHit::marked(met.faces, met.at, mark));
            self.marks.push(RayHit::marked(other.faces, other.at, mark));
        }
        self.met.push(met);
    }
}

/// What a turn of a face's loops is read under: the band and the ray's
/// rows.
#[derive(Clone, Copy)]
struct TurnRay {
    band: Band,
    rows: RayRows,
}

/// **Whether a ray crosses planar `face` at `p`**, off the turn of its
/// loops about `p` ([`loop_turn`]); `None` where an edge of it is neither
/// a line nor a circle arc. The caller reads it only where every edge
/// bounding `face` is one the ray was decided clear of
/// ([`EdgeMeets::clear`]), so `p`, a point of the ray, lies off every one,
/// and a seam inside the face, which bounds nothing and was read nowhere,
/// is left out of the turn ([`loop_turn`]). The turn is `±2π` inside the
/// face and `0` outside it, to rounding: `|turn| − π` is `±π`, definite
/// by construction ([`turned_inside`]). No auxiliary direction is cast,
/// so no tie the face's geometry does not hold is ever decided.
fn crosses_by_turn<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    (normal, p): (Vec3<T>, Point3<T>),
    reach: T,
    ray: TurnRay,
) -> Result<Option<bool>, RayFault<PointInSolidError>> {
    match loop_turn(body, face, normal, p, |x| x)? {
        Some(turn) => turned_inside(face, turn, reach, ray).map(Some),
        None => Ok(None),
    }
}

/// **Whether a root `p` of a cylinder face's wall lies in the face**, off
/// the turn of its loops about `p` on an image of the wall in a plane
/// ([`loop_turn`]); `None` where an edge of it is neither a ruling nor a
/// circle (an oblique section). The wall `(origin, axis, radius)` minus
/// nothing is an annulus: a point at height `h` along the axis and
/// radial direction `s` goes to `ρ(h − h_p)·s`, `s` read in the frame of
/// `p`'s own radial and `ρ(Δ) = 1 + Δ/(|Δ| + radius)`, which rises from
/// `0` to `2` and is `1` at `p`. It is one to one and continuous, carries
/// every ruling to a segment of a ray from the origin and every circle on
/// the wall (a parallel) to a circle about it, so the face's image is a
/// region of the plane bounded by its loops' images, and `p`'s image lies
/// in it exactly when `p` lies in the face; the face's own seam, a ruling
/// its image walks out and back, is left out of the turn. The caller reads
/// it only where the face is met at no edge and every edge bounding it is
/// one the ray was decided clear of ([`EdgeMeets::clear`]), so `p` lies
/// off every one; `None` too where `p` lies nearer an edge than twice how
/// far the edge is off its ruling or parallel (`bool_wall_turn_true`,
/// below). The turn is read as a planar face's is ([`turned_inside`]): no
/// path is walked.
fn wall_crosses_by_turn<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    (origin, axis, radius): (Point3<T>, Vec3<T>, T),
    (p, reach): (Point3<T>, T),
    ray: TurnRay,
) -> Result<Option<bool>, RayFault<PointInSolidError>> {
    let band = ray.band;
    let axis = axis / axis.norm();
    let radius = radius.abs();
    let split = |x: Point3<T>| {
        let w = x - origin;
        let h = w.dot(axis);
        let radial = w - axis * h;
        (h, radial / radial.norm())
    };
    let (h_p, e1) = split(p);
    let e2 = axis.cross(e1);
    let image = |x: Point3<T>| {
        let (h, s) = split(x);
        let dh = h - h_p;
        let rho = T::one() + dh / (dh.abs() + radius);
        Point3::new(s.dot(e1) * rho, s.dot(e2) * rho, T::zero())
    };
    // The image carries a ruling to a segment, and a circle to a circle,
    // exactly where the ruling lies along the axis and the circle is a
    // parallel. Tier 3 admits an edge off that by its band's zero
    // (`props_cylinder_circle_tilt`, `props_meridian_axial`), so off it by
    // up to `√(2εr)` along the axis: where `p` lies nearer the edge than
    // twice how far the edge is off, the image may misplace it, and the
    // face is not read so here. For an edge on its line or parallel the
    // margin is `p`'s clearance of it, which every crossing read definite.
    //
    // The factor 2 is argued, not demonstrated: no fixture reads wrong
    // under factor 1, and none is pinned. Off-line numerics (not kept)
    // put the image's misplacement zone, as a share of `off`, below 1
    // for a ruling drifting linearly round the wall (about 0.25 at a
    // length of twice the radius, approaching 1 only as length over
    // radius grows without bound), and at about `off/(2r)` for a tilted
    // circle, whose image the two three-point arcs carry to first order;
    // a circle's centre off the axis moves no image at all. So factor 1
    // can be made red by no pose, and factor 0 only by a ruling drifting
    // by far more than the band, which a valid body bounds by a plane
    // through that ruling: not built.
    for e in face_edges(body, face).map_err(RayFault::Fatal)? {
        if e.across == face {
            continue;
        }
        let Some(curve) = body
            .get_edge(e.edge)
            .and_then(|ed| body.get_curve_geom(ed.curve))
            .and_then(crate::null::CurveGeom::certified)
        else {
            return Ok(None);
        };
        let off = match *curve.carrier() {
            geom::Curve3::Line { dir, .. } => {
                let (t0, t1) = curve.params();
                (dir / dir.norm()).cross(axis).norm() * ((t1 - t0) * dir.norm()).abs()
            }
            geom::Curve3::Circle {
                center,
                axis: normal,
                radius: r,
                ..
            } => {
                let w = center - origin;
                r.abs() * (normal / normal.norm()).cross(axis).norm()
                    + (w - axis * w.dot(axis)).norm()
            }
            _ => return Ok(None),
        };
        let Some(clear) = edge_clearance(body, &e, p) else {
            return Ok(None);
        };
        if decide(
            "bool_wall_turn_true",
            Margin::of(clear - off * T::from_f64(2.0)),
            band,
        ) != Ok(Sign::Positive)
        {
            return Ok(None);
        }
    }
    let up = Vec3::new(T::zero(), T::zero(), T::one());
    match loop_turn(body, face, up, image(p), image)? {
        Some(turn) => turned_inside(face, turn, reach, ray).map(Some),
        None => Ok(None),
    }
}

/// Whether a loop turn `turn` about a point says it lies inside: the
/// turn is `±2π` inside and `0` outside, to rounding, so `|turn| − π` is
/// `±π`, definite by construction, and is read under `winding` through
/// the unlogged door (`k_stats::check_unlogged`, ledger row F23 of
/// `docs/predicate-dimension-audit.md`), so no verdict log, flip lane or
/// tie census counts it as a margin.
fn turned_inside<T: Decide>(
    face: FaceKey,
    turn: T,
    reach: T,
    ray: TurnRay,
) -> Result<bool, RayFault<PointInSolidError>> {
    Ok(geom_core::k_stats::check_unlogged(
        ray.rows.winding,
        (turn.abs() - T::pi()) * reach,
        ray.band,
        "F23",
    )
    .map_err(|diag| RayFault::InBand(PointInSolidError::Escalated { face, diag }))?
        == Sign::Positive)
}

/// **The turn of `face`'s loops about `p`**, about `normal`, every point
/// of them first carried by `map` (the identity for a planar face, which
/// `p` lies in; an image of a curved face in the plane `p`'s image lies
/// in): a line edge, carried to a segment, turns by the angle its ends
/// make at `p`, and a circle arc, carried to a circle arc, by [`arc_turn`]
/// over its two halves. `None` where an edge is neither.
fn loop_turn<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    normal: Vec3<T>,
    p: Point3<T>,
    map: impl Fn(Point3<T>) -> Point3<T>,
) -> Result<Option<T>, RayFault<PointInSolidError>> {
    let corrupt = || RayFault::Fatal(PointInSolidError::CorruptFace { face });
    let at = |v| {
        body.get_vertex(v)
            .and_then(|vx| body.get_point(vx.point))
            .map(|&x| map(x))
            .ok_or_else(corrupt)
    };
    let angle = |a: Point3<T>, b: Point3<T>| {
        let (u, w) = (a - p, b - p);
        normal.dot(u.cross(w)).atan2(u.dot(w))
    };
    let mut turn = T::zero();
    for e in face_edges(body, face).map_err(RayFault::Fatal)? {
        // A seam inside the face is walked once each way, and its two
        // turns cancel: it is left out, not summed, since in floats two
        // passes whose products vanish both read `atan2(+0, x < 0) = π`.
        if e.across == face {
            continue;
        }
        let (a, b) = (at(e.ends[0])?, at(e.ends[1])?);
        let curve = body
            .get_edge(e.edge)
            .and_then(|ed| body.get_curve_geom(ed.curve))
            .and_then(crate::null::CurveGeom::certified)
            .ok_or_else(corrupt)?;
        match *curve.carrier() {
            geom::Curve3::Line { .. } => turn = turn + angle(a, b),
            geom::Curve3::Circle { .. } => {
                let on = |f: f64| {
                    along_half(body, e.half, f)
                        .map(|(x, _)| map(x))
                        .ok_or_else(corrupt)
                };
                let m = on(0.5)?;
                turn = turn
                    + arc_turn(normal, p, (a, on(0.25)?, m))
                    + arc_turn(normal, p, (m, on(0.75)?, b));
            }
            _ => return Ok(None),
        }
    }
    Ok(Some(turn))
}

/// **The turn about `p` of a circle arc** from `a` through `m` to `b`,
/// at most half a turn, about `normal`, in the plane of `p`: the change
/// in the direction from `p` to the arc's point as it runs from `a` to
/// `b`. As complex numbers in that plane, `M(x) = (x − a)/(x − b)` carries
/// the arc's circle to a line through the origin and the arc to the
/// half of it `M(m)` lies on, so the turn is
/// `Arg(−conj M(m)) − Arg(−M(p) conj M(m))`, whose one cut is the arc
/// itself: it asks no question, as a chord's turn and a test of which
/// side of the chord `p` lies would, and `p` lies clear of the arc. Each
/// `M` is read by its numerator `(x − a)·conj(x − b)`, the positive
/// scale `|x − b|²` turning no argument.
fn arc_turn<T: Decide>(
    normal: Vec3<T>,
    p: Point3<T>,
    (a, m, b): (Point3<T>, Point3<T>, Point3<T>),
) -> T {
    let numerator = |x: Point3<T>| {
        let (u, w) = (x - a, x - b);
        (u.dot(w), normal.dot(w.cross(u)))
    };
    let ((pr, pi), (mr, mi)) = (numerator(p), numerator(m));
    let zero = T::zero();
    mi.atan2(zero - mr) - (zero - (pi * mr - pr * mi)).atan2(zero - (pr * mr + pi * mi))
}

/// **An edge of a face's loops**, as one of its half-edges walks it: the
/// half-edge, the edge, its two ends in the half-edge's direction, and the
/// face across it (the face itself across a seam inside it).
#[derive(Clone, Copy)]
pub(crate) struct FaceEdge {
    // Read by the probes' frames, which orient the edge by it.
    #[cfg_attr(not(any(test, feature = "test-support")), allow(dead_code))]
    pub(crate) half: crate::entity::HalfEdgeKey,
    pub(crate) edge: crate::entity::EdgeKey,
    pub(crate) ends: [crate::entity::VertexKey; 2],
    pub(crate) across: FaceKey,
}

/// Every edge of `face`'s loops, rings included ([`FaceEdge`]). A loop
/// with no edge (a lone ring vertex) bounds no area and has none.
///
/// # Errors
///
/// [`PointInSolidError::CorruptFace`] where a loop, a half-edge, its mate
/// or the mate's face does not resolve.
pub(crate) fn face_edges<T: geom_core::Real>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Vec<FaceEdge>, PointInSolidError> {
    let corrupt = PointInSolidError::CorruptFace { face };
    let f = body.get_face(face).ok_or(corrupt.clone())?;
    let mut out = Vec::new();
    for l in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let first = match body.get_loop(l).map(|lp| lp.boundary) {
            Some(LoopBoundary::Cycle { first }) => first,
            Some(LoopBoundary::Empty { .. }) => continue,
            None => return Err(corrupt),
        };
        for half in body.loop_walk(first).closed("loop", first) {
            let he = body.get_half_edge(half).ok_or(corrupt.clone())?;
            let across = body
                .edges
                .get(he.edge)
                .and_then(|e| e.claim(half))
                .and_then(|c| body.get_half_edge(c.mate))
                .and_then(|m| body.get_loop(m.parent_loop))
                .ok_or(corrupt.clone())?
                .face;
            let end = body.half_edge_end(half).ok_or(corrupt.clone())?;
            out.push(FaceEdge {
                half,
                edge: he.edge,
                ends: [he.start, end],
                across,
            });
        }
    }
    Ok(out)
}

/// **The point of half-edge `half`'s curve at fraction `f` of its span,
/// as the half walks it**, and the curve's derivative there in that
/// direction: the one reading of a half-edge's walk along its carrier.
/// `None` where the half, its edge or its certified curve does not
/// resolve.
pub(crate) fn along_half<T: Decide>(
    body: &Body<T>,
    half: crate::entity::HalfEdgeKey,
    f: f64,
) -> Option<(Point3<T>, Vec3<T>)> {
    let edge = body.get_half_edge(half)?.edge;
    let plus = body.edges.get(edge)?.claim(half)?.plus;
    let curve = body
        .get_edge(edge)
        .and_then(|ed| body.get_curve_geom(ed.curve))
        .and_then(crate::null::CurveGeom::certified)?;
    let (t0, t1) = curve.params();
    let t = t0 + (t1 - t0) * T::from_f64(if plus { f } else { 1.0 - f });
    let d = curve.carrier().deriv(t);
    Some((curve.carrier().eval(t), if plus { d } else { -d }))
}

/// How many even steps of an arc's span [`edge_clearance`] reads.
const CLEARANCE_SAMPLES: u32 = 32;

/// **A lower bound on the distance from `m` to the edge of `e`**, a line
/// or a circle arc, by evaluation alone: a segment's exact distance, or
/// for an arc the greater of two, each never above the true distance —
/// `m`'s distance from the arc's whole circle (its carrier's closed-form
/// clearance, [`geom::Curve3::clearance`]), and the least distance from
/// `m` to the arc's points at [`CLEARANCE_SAMPLES`] even steps of its span
/// less half the arc length between two of them. `None` for another edge.
pub(crate) fn edge_clearance<T: Decide>(body: &Body<T>, e: &FaceEdge, m: Point3<T>) -> Option<T> {
    let at = |v| Some(*body.get_point(body.get_vertex(v)?.point)?);
    let curve = body
        .get_edge(e.edge)
        .and_then(|ed| body.get_curve_geom(ed.curve))
        .and_then(crate::null::CurveGeom::certified)?;
    match *curve.carrier() {
        geom::Curve3::Line { .. } => Some(crate::sector_shape::point_segment_distance(
            at(e.ends[0])?,
            at(e.ends[1])?,
            m,
        )),
        geom::Curve3::Circle { radius, .. } => {
            let (t0, t1) = curve.params();
            let n = T::from_f64(f64::from(CLEARANCE_SAMPLES));
            let mut least: Option<T> = None;
            for k in 0..=CLEARANCE_SAMPLES {
                let p = curve
                    .carrier()
                    .eval(t0 + (t1 - t0) * T::from_f64(f64::from(k)) / n);
                let d = (p - m).norm();
                least = Some(least.map_or(d, |l: T| l.min(d)));
            }
            let gap = radius.abs() * (t1 - t0).abs() / (n * T::from_f64(2.0));
            Some(curve.carrier().clearance(m)?.max(least? - gap))
        }
        _ => None,
    }
}

/// Whether every edge bounding `face` is one of `read`: a seam inside it
/// (an edge both of whose sides are the face) bounds nothing, owes no
/// reading, and is never one of `read`.
fn every_bound_read<T: geom_core::Real>(
    body: &Body<T>,
    face: FaceKey,
    read: &std::collections::BTreeSet<crate::entity::EdgeKey>,
) -> bool {
    face_edges(body, face).is_ok_and(|edges| {
        edges
            .iter()
            .all(|e| e.across == face || read.contains(&e.edge))
    })
}

/// Whether every edge of `face`, a seam inside it included, is one of
/// `read`.
fn every_edge_read<T: geom_core::Real>(
    body: &Body<T>,
    face: FaceKey,
    read: &std::collections::BTreeSet<crate::entity::EdgeKey>,
) -> bool {
    face_edges(body, face).is_ok_and(|edges| edges.iter().all(|e| read.contains(&e.edge)))
}

/// The roots of a ray's trimmed curved carriers, and the faces whose
/// carriers it definitely misses ([`carrier_roots`]).
type CarrierRoots<T> = (Vec<RootCross<T>>, Vec<FaceKey>);

/// The higher-ranked of a kept fault and `fault`, raised at `face`
/// ([`RayFault::rank`]); of two of one rank, the lesser face's, so the
/// ray's fault does not turn on the order its faces are read in.
fn worse<E>(
    kept: Option<(FaceKey, RayFault<E>)>,
    face: FaceKey,
    fault: RayFault<E>,
) -> Option<(FaceKey, RayFault<E>)> {
    match kept {
        Some((at, k))
            if (k.rank(), core::cmp::Reverse(at)) >= (fault.rank(), core::cmp::Reverse(face)) =>
        {
            Some((at, k))
        }
        _ => Some((face, fault)),
    }
}

/// **The roots of each trimmed curved face's carrier** on the ray `q +
/// s·d` (a wall's, a cone's, a sphere patch's, a torus patch's), before
/// any region reads them, and the faces whose carriers the ray
/// definitely misses. The edge reading ([`curved_edges`]) stands an edge
/// for one root of each curved face it bounds; [`ray_crossings`] tests
/// the rest against their faces' regions.
///
/// The wall arm (M5 PR 9): the ray meets the infinite wall at the roots
/// of a quadratic in metres; a tangent ray (discriminant in the zero
/// band) grazes, and an axis-parallel one misses only where `q`'s
/// residual is definitely past the escalate threshold. The near root
/// enters the wall's chart and the far one leaves it, read off the
/// decided discriminant (`d·rad = ∓√disc` at the roots).
///
/// The cone arm (issue 1011): the roots of `G(t) = (w·â)² − |w|²cos²α`
/// on the double cone. A ray parallel to a generator (`A` in band,
/// levered by the face's slant extent) leaves no certified pair and
/// grazes, and so does a tangent one (the discriminant `B² − AC`, metered
/// by that extent); `bool2_r1_probes::probe_generator_parallel_schedule_rays_graze_and_the_door_recovers`
/// is the generator-parallel row.
///
/// The sphere patch: [`line_sphere_roots`], the near root entering the
/// ball and the far one leaving it. The torus patch: [`line_torus_roots`]
/// on a certified count ([`torus_crossings`]).
///
/// # Errors
///
/// A graze, or a decision in band.
fn carrier_roots<T: Decide>(
    geos: &[(FaceKey, FaceGeo<T>)],
    q: Point3<T>,
    d: Vec3<T>,
    reach: T,
    band: Band,
    rows: RayRows,
) -> Result<CarrierRoots<T>, RayFault<PointInSolidError>> {
    let (mut roots, mut missed) = (Vec::new(), Vec::new());
    // Each face's fault, the highest kept: the ray's, whatever order its
    // faces are read in.
    let mut worst: Option<(FaceKey, RayFault<PointInSolidError>)> = None;
    for &(face, ref geo) in geos {
        let read = (|| {
            let escalate = |diag| RayFault::InBand(PointInSolidError::Escalated { face, diag });
            let first = roots.len();
            let mut push = |t: T, side: RootSide| {
                let ordinal = roots.len() - first;
                roots.push(RootCross {
                    face,
                    t,
                    side,
                    ordinal,
                });
            };
            match *geo {
                FaceGeo::Cylinder {
                    origin,
                    axis,
                    radius,
                    sense,
                    ..
                } => match line_wall_roots(q, d, origin, axis, radius, reach, band)
                    .map_err(|fault| escalate(fault.diag))?
                {
                    WallRoots::Miss => missed.push(face),
                    WallRoots::AxisParallel => {
                        if !clear_of_carrier(
                            wall_elevation(origin, axis, radius, q),
                            face,
                            band,
                            rows,
                        )? {
                            return Err(RayFault::Graze);
                        }
                    }
                    WallRoots::Tangent => return Err(RayFault::Graze),
                    WallRoots::Two(ts) => {
                        for (t, chart) in ts.into_iter().zip([Sign::Negative, Sign::Positive]) {
                            push(t, RootSide::Ordered(oriented(chart, sense)));
                        }
                    }
                },
                FaceGeo::Cone {
                    apex,
                    axis,
                    half_angle,
                    ref region,
                    ..
                } => {
                    let [a2, b2, c2] = line_cone_quadratic(q, d, apex, axis, half_angle);
                    // A bound on the face's reach from its apex, never under
                    // it — the lever both margins below are metered by.
                    let v_ext = region.reach();
                    if decide("bool_ray_cone_lead", Margin::levered(a2, v_ext), band)
                        .map_err(escalate)?
                        == Sign::Zero
                    {
                        return Err(RayFault::Graze); // generator-parallel
                    }
                    let disc = b2.powi(2) - a2 * c2;
                    match decide("bool_ray_cone_disc", Margin::over_lever(disc, v_ext), band)
                        .map_err(escalate)?
                    {
                        Sign::Positive => {
                            // Unordered: `A` may be negative, and the count
                            // orders by advance anyway.
                            for t in quadratic_roots(a2, b2, c2, disc) {
                                push(t, RootSide::Cone);
                            }
                        }
                        Sign::Zero => return Err(RayFault::Graze), // tangent ray
                        Sign::Negative => missed.push(face),
                    }
                }
                FaceGeo::SpherePatch {
                    center,
                    radius,
                    sense,
                    ..
                } => match line_sphere_roots(q, d, center, radius, band).map_err(escalate)? {
                    WallRoots::Two([near, far]) => {
                        push(near, RootSide::Ordered(oriented(Sign::Negative, sense)));
                        push(far, RootSide::Ordered(oriented(Sign::Positive, sense)));
                    }
                    WallRoots::Tangent => return Err(RayFault::Graze),
                    WallRoots::Miss | WallRoots::AxisParallel => missed.push(face),
                },
                FaceGeo::TorusPatch {
                    center,
                    axis,
                    major_radius,
                    minor_radius,
                    ..
                } => {
                    let found = match line_torus_roots(
                        q,
                        d,
                        center,
                        axis,
                        major_radius,
                        minor_radius,
                        band,
                    ) {
                        Ok(roots) => Ok(roots),
                        Err(RootRefusal::Unsettled(why)) => Err(why),
                        Err(RootRefusal::Escalated(diag)) => return Err(escalate(diag)),
                        // A touch of the ray is this ray's; a fault grazes too
                        // (`work/splitdoor/ray-lanes-graze-on-a-root-finders-fault.md`).
                        Err(RootRefusal::Touch(_) | RootRefusal::Fault(_)) => {
                            return Err(RayFault::Graze);
                        }
                    };
                    match torus_crossings(face, found)? {
                        Some(found) => {
                            for &t in found.roots() {
                                push(t, RootSide::Torus);
                            }
                        }
                        None => missed.push(face),
                    }
                }
                FaceGeo::Plane(..)
                | FaceGeo::Sphere { .. }
                | FaceGeo::Torus { .. }
                | FaceGeo::PartialCone { .. }
                | FaceGeo::PartialWall { .. } => {}
            }
            Ok(())
        })();
        if let Err(fault) = read {
            worst = worse(worst, face, fault);
        }
    }
    if let Some((_, fault)) = worst {
        return Err(fault);
    }
    Ok((roots, missed))
}

/// **The outward sign of curved `face` at `root`**: its decided root
/// order's, or, on a cone, the nappe the root lies on and the ray's
/// incidence against the chart normal there, and on a torus the
/// incidence against the tube's chart normal; the face's sense folded in
/// (S10). `Zero` where no side is read: at a cone's apex, or at an
/// incidence in band.
///
/// On a cone, the chart normal is `r̂·cos α − σ·axis·sin α`, `σ` the
/// nappe's sign (`S` places the `v < 0` nappe at azimuth `u + π`, so its
/// chart radial is the physical one negated; what carries the nappe is
/// the axial term alone), a unit vector, so the dot is a cosine levered
/// by the local radius (D4's θ·r), which the nappe has put definitely
/// clear of the apex. An exactly tangent ray never reaches the
/// incidence: it is a double root, which the discriminant grazed on; an
/// in-band incidence with a definite discriminant is a fixture tuned to
/// one ε row, and reads no side. On a torus, the chart normal is the
/// hit's meridian-plane offset over `r`, levered by the tube's radius.
///
/// # Errors
///
/// A decision in band.
fn root_outward<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    geo: &FaceGeo<T>,
    root: RootCross<T>,
    q: Point3<T>,
    d: Vec3<T>,
    band: Band,
) -> Result<Sign, RayFault<PointInSolidError>> {
    let escalate = |diag| RayFault::InBand(PointInSolidError::Escalated { face, diag });
    let p = q + d * root.t;
    match (root.side, geo) {
        (RootSide::Ordered(sign), _) => Ok(sign),
        (
            RootSide::Cone,
            &FaceGeo::Cone {
                apex, axis, sense, ..
            },
        ) => {
            let wp = p - apex;
            let hp = wp.dot(axis);
            if decide("bool_ray_cone_nappe", Margin::of(hp), band).map_err(escalate)? == Sign::Zero
            {
                return Ok(Sign::Zero);
            }
            let rho = (wp - axis * hp).norm();
            Ok(oriented(
                decide(
                    "bool_ray_cone_incidence",
                    Margin::levered(d.dot(chart_normal(body, face, p)?), rho),
                    band,
                )
                .map_err(escalate)?,
                sense,
            ))
        }
        (
            RootSide::Torus,
            &(FaceGeo::Torus {
                minor_radius,
                sense,
                ..
            }
            | FaceGeo::TorusPatch {
                minor_radius,
                sense,
                ..
            }),
        ) => Ok(oriented(
            decide(
                "bool_ray_torus_incidence",
                Margin::levered(d.dot(chart_normal(body, face, p)?), minor_radius),
                band,
            )
            .map_err(escalate)?,
            sense,
        )),
        // A root is made only by its face's own arm.
        _ => Err(RayFault::Fatal(PointInSolidError::CorruptFace { face })),
    }
}

/// **A curved face's carrier at `face`**, the surface its roots and
/// incidences are read on: a face with none is corrupt.
fn carrier<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<&Surface<T>, RayFault<PointInSolidError>> {
    body.get_face(face)
        .and_then(|f| body.get_surface(f.surface))
        .ok_or(RayFault::Fatal(PointInSolidError::CorruptFace { face }))
}

/// **The chart normal of `face`'s carrier at `p`**, from its one home
/// ([`geom_brep::implicit_gradient`]): unit on the carrier, and poison
/// where the carrier is singular (a cone's apex or axis, a torus's
/// axis). Every incidence the ray reads against a curved face is read
/// against it.
fn chart_normal<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    p: Point3<T>,
) -> Result<Vec3<T>, RayFault<PointInSolidError>> {
    Ok(geom_brep::implicit_gradient(carrier(body, face)?, p))
}

/// **The margin by which the ray, from where it passes the met edge
/// `m`, overshoots reaching a root of `face`'s carrier at advance `root`
/// in band of the carrier** ([`reaches_root_in_band`]): the rise off the
/// carrier over the root's distance `off` from the passage's span, at
/// its incidence there (`levered(|d·n̂|, off)`), less the passage's
/// clearance (the carrier's residual there). It is charged for the
/// root's point lying off the carrier (the normal's tilt, the residual
/// over the least radius of curvature) and for rounding. That rounding is
/// a short chain's over the advances, the points' coordinates and the
/// carrier's scale.
///
/// # Errors
///
/// A face with no carrier.
fn reach_margin<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    (q, d): (Point3<T>, Vec3<T>),
    m: &Met<T>,
    root: T,
) -> Result<T, RayFault<PointInSolidError>> {
    let surface = carrier(body, face)?;
    let (at, passage) = (q + d * root, q + d * m.at);
    let off = (m.span.0 - root).max(root - m.span.1).max(T::zero());
    let n = chart_normal(body, face, at)?;
    let tilt = geom_brep::implicit_residual(surface, at).abs()
        / geom_brep::min_radius_of_curvature(surface, at);
    let incidence = d.dot(n).abs() / n.norm() + tilt + geom_core::rounding_charge(T::one());
    let rise = Margin::levered(incidence, off).value();
    let clearance = geom_brep::implicit_residual(surface, passage).abs();
    let origin = Point3::origin();
    let terms = m.at.abs()
        + off
        + (passage - origin).norm()
        + (at - origin).norm()
        + geom_brep::curvature_lever_arm(surface, passage);
    // Lengths all.
    Ok(rise - clearance + geom_core::rounding_charge(terms))
}

/// **Whether the ray reaches a root of `face`'s carrier at advance
/// `root` from where it passes the met edge `m`, in band of the
/// carrier**: whether [`reach_margin`] decides Negative or Zero.
///
/// A ray's distance from a curved carrier need not be monotone along
/// it. A root far from the passage can be the ray's crossing of the face
/// elsewhere, past a graze at the edge that crosses the carrier nowhere
/// (a ray grazing a partial torus's rim, crossing the tube across the
/// axis). Where the distance `f` is convex (a cylinder, a cone's nappe,
/// a sphere) and the ray reaches the root from outside,
/// `|f'| · off ≤ f(passage)` at the root: at its incidence there, the
/// ray rises off the carrier over `off` by no more than it stands off it
/// at the passage. A margin decided Positive is a root the ray does not
/// reach so. A margin in band, of either sign, decides nothing, and its
/// root is set aside, even below zero where the argument would let it
/// stand: at ε 1e-6, 204 roots of the curved-edge corpus's rays have a
/// margin in band below zero, 192 of them reached from inside a convex
/// carrier (short inside chords, which can rise up to twice the chord's
/// depth) and 12 from outside. On a torus the bound is necessary and not
/// shown sufficient.
///
/// # Errors
///
/// A comparison that no subdivision of a box can leave in band (an
/// enclosure straddling it), or one a poisoned normal makes invalid (a
/// root at a singular point of the carrier); a face with no carrier.
fn reaches_root_in_band<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    ray: (Point3<T>, Vec3<T>),
    m: &Met<T>,
    root: T,
    band: Band,
    rows: RayRows,
) -> Result<bool, RayFault<PointInSolidError>> {
    let margin = reach_margin(body, face, ray, m, root)?;
    match decide(rows.reaches, Margin::of(margin), band) {
        Ok(sign) => Ok(sign != Sign::Positive),
        Err(diag) if crate::validate::in_band(&diag) => Ok(false),
        Err(diag) => Err(RayFault::InBand(PointInSolidError::Escalated {
            face,
            diag,
        })),
    }
}

/// **Whether trimmed curved `face` holds the point `p` of its carrier**,
/// read off its own region: `None` on its boundary, or where its region
/// cannot place `p` (a walk no ray of its own settled), which reads no
/// side; a face the region cannot read there blocks the ray.
///
/// # Errors
///
/// A decision in band, a limit of the region's reader, or a broken face.
fn root_held<T: Decide>(
    face: FaceKey,
    geo: &FaceGeo<T>,
    p: Point3<T>,
    band: Band,
) -> Result<Option<bool>, RayFault<PointInSolidError>> {
    let reading = match geo {
        FaceGeo::Cylinder { region, .. } | FaceGeo::Cone { region, .. } => {
            return region_hit(face, region.contains(p, band));
        }
        FaceGeo::SpherePatch { region, .. } => region.contains(p, band),
        FaceGeo::TorusPatch { region, .. } => region.contains(p, band),
        _ => return Err(RayFault::Fatal(PointInSolidError::CorruptFace { face })),
    };
    match reading {
        Ok(side) => Ok(side),
        Err(RegionRefusal::RayExhausted { .. }) => Ok(None),
        Err(e @ RegionRefusal::Escalated(_)) => Err(RayFault::InBand(e.of_face(face))),
        Err(e @ RegionRefusal::WoundPastPeriod) => Err(RayFault::Fatal(e.of_face(face))),
        // An edge whose reading ran out at this hit: a limit of the
        // reader this ray met, which another ray may miss.
        Err(e @ RegionRefusal::Unreadable(_)) => Err(RayFault::Blocked(e.of_face(face))),
    }
}

/// Every crossing of the ray `q + s·d` ahead of `q` with the faces of
/// `geos` (a selection's, each with its resolved query geometry), each
/// offered as a [`RayHit`] at its advance, and as on an edge where it
/// lands on a curved face's boundary or meets it tangentially.
///
/// **Every edge is decided once, for both faces it bounds** (D10,
/// Booleans): a line edge between two planar faces whose planes the ray
/// crosses through the lines' closest point ([`edge_meets`]), and every
/// other edge between two faces that read their own carrier crossings
/// (a crossed plane, a trimmed curved face's roots, [`carrier_roots`])
/// through where the ray passes it, read on its span
/// ([`curved_edges`]). An edge the ray passes within the band of meets
/// it, half a crossing for each face it bounds, at the carrier crossing
/// of each face it stands for; a planar face with such an edge offers no
/// crossing of its own, and a curved face none at that root.
///
/// A met edge stands for a crossing only where the edge represents it,
/// and reads nothing there otherwise. It represents a crossing that:
/// - no other met edge stands for;
/// - belongs to a face whose every edge is decided;
/// - on a face not every edge of which is a line edge read exactly, lies
///   near the passage, or is definitely held by the face and reached
///   from the passage in band of its carrier.
///
/// A planar face whose edges are all lines and circle arcs, each decided
/// clear of the ray, is crossed where the turn of its loops says
/// ([`crosses_by_turn`]); another takes its in-face walk. Each root of a
/// curved face no edge stands for is tested against its face's trim,
/// except a cylinder face's whose edges are all rulings and circles, each
/// decided clear of the ray, which is held where the turn of its loops on
/// an image of its wall says ([`wall_crosses_by_turn`]). A ray within the band of a vertex,
/// or a wedge, is marked there and reads nothing where its count reaches
/// the mark ([`RayHit::mark`]).
pub(crate) fn ray_crossings<T: Decide>(
    body: &Body<T>,
    geos: &[(FaceKey, FaceGeo<T>)],
    q: Point3<T>,
    d: Vec3<T>,
    reach: T,
    band: Band,
    rows: RayRows,
) -> Result<RayCrossings<T>, RayFault<PointInSolidError>> {
    let mut crossings = Crossings::new();
    let turn_ray = TurnRay { band, rows };
    // A crossing at advance `t`, somewhere in `span`; behind `q` it is
    // none.
    let mut offer = |hit: RayHit<T>, span: (T, T), sideless: bool| {
        let escalate = |diag| PointInSolidError::Escalated {
            face: hit.face,
            diag,
        };
        let ahead = ray_walk::advance(rows.advance, Margin::of(hit.at), band)
            .map_err(|f| f.map(escalate))?;
        if ahead == Sign::Positive {
            crossings.push_span(hit.at, span, hit, sideless);
        }
        Ok::<(), RayFault<PointInSolidError>>(())
    };
    // A crossing's sign as its count.
    let count = |outward: Sign| match outward {
        Sign::Positive => 1,
        Sign::Negative => -1,
        Sign::Zero => 0,
    };
    // ---- Each planar face's plane, crossed once. ----
    let mut planes: Vec<PlaneCross<T>> = Vec::new();
    for &(face, ref geo) in geos {
        let FaceGeo::Plane(origin, normal) = *geo else {
            continue;
        };
        let escalate = |diag| RayFault::InBand(PointInSolidError::Escalated { face, diag });
        // `normal` is the face's OUTWARD normal (S10, folded in by
        // `face_geo`), so this sign IS the material-side verdict the
        // count consumes; the advance `t` below is a ratio of two such
        // dots and cannot see the orientation at all.
        let denom = d.dot(normal);
        // The cosine levered by the selection's reach: how far the ray
        // rises off the plane over every length at which it could meet
        // the face ([`selection_reach`]).
        let outward = decide(rows.denom, Margin::levered(denom, reach), band).map_err(escalate)?;
        if outward == Sign::Zero {
            // A ray parallel to the plane within the band rises off it
            // by at most the band's zero over the reach. The pre-pass
            // passes q in band of the carrier wherever its foot is
            // outside the face, so a miss is asked, not assumed: q
            // definitely off the plane, or the face definitely to one
            // side of the ray.
            let clear = clear_of_carrier((q - origin).dot(normal), face, band, rows);
            if matches!(clear, Ok(true)) || ray_passes_beside(body, face, q, d, normal, band, rows)?
            {
                continue;
            }
            return Err(match clear {
                Err(in_band) => in_band,
                Ok(_) => RayFault::Graze,
            });
        }
        let t = (origin - q).dot(normal) / denom;
        planes.push(PlaneCross {
            face,
            normal,
            t,
            p: q + d * t,
            outward,
        });
    }
    let (roots, missed) = carrier_roots(geos, q, d, reach, band, rows)?;
    let mut meets = edge_meets(body, &planes, q, d, reach, band, rows)?;
    curved_edges::read(
        body,
        geos,
        &planes,
        &roots,
        &missed,
        (q, d, reach),
        band,
        rows,
        &mut meets,
    )?;
    let geo_of = |f: FaceKey| geos.iter().find(|g| g.0 == f).map(|g| &g.1);
    // A carrier crossing as one point of its carrier: faces on one carrier
    // (a seam split) cross it at the same crossings, read in the same
    // order, so the crossing is the carrier and its place among them.
    let surface = |f: FaceKey| body.get_face(f).map(|fc| fc.surface);
    let key = |f: FaceKey, c: Cross| {
        (
            surface(f),
            match c {
                Cross::Plane => 0,
                Cross::Root(k) => roots[k].ordinal + 1,
            },
        )
    };
    // Each carrier crossing, with every met edge that stands for it: one
    // standing for both its faces' one crossing (a seam met) stands once.
    let mut claims = Vec::new();
    for m in &meets.met {
        for (&f, &c) in m.faces.iter().zip(&m.cross) {
            let claim = (key(f, c), m.edge);
            if !claims.contains(&claim) {
                claims.push(claim);
            }
        }
    }
    // ---- Each met edge, half a crossing for each face at the crossing it
    // stands for, or nothing. ----
    for m in &meets.met {
        let [a, b] = m.faces;
        let mut ends = [(m.at, 0); 2];
        let mut stands = true;
        for (end, (&f, &c)) in ends.iter_mut().zip(m.faces.iter().zip(&m.cross)) {
            let plane = planes.iter().find(|p| p.face == f);
            let (t, side) = match c {
                Cross::Plane => plane.map_or((m.at, Sign::Zero), |p| (p.t, p.outward)),
                Cross::Root(k) => (
                    roots[k].t,
                    root_outward(
                        body,
                        f,
                        geo_of(f)
                            .ok_or(RayFault::Fatal(PointInSolidError::CorruptFace { face: f }))?,
                        roots[k],
                        q,
                        d,
                        band,
                    )?,
                ),
            };
            *end = (t, count(side));
            // A crossing two met edges stand for would be counted at each.
            let k = key(f, c);
            if claims.iter().filter(|(claimed, _)| *claimed == k).count() > 1 {
                stands = false;
            }
            // An edge of the face no reading here decides could lie in band
            // of the ray too.
            if !every_bound_read(body, f, &meets.read) {
                stands = false;
            }
            // A face whose edges are all read exactly is met at each edge
            // the ray passes in band of, so its crossing lies between them.
            // Another's crossing stands for the edge only where it lies near
            // the passage, or where the face definitely holds it and the ray
            // reaches it from the passage in band of the carrier (a plane's
            // crossing always is; a curved face's root is where
            // [`reaches_root_in_band`] says): a ray shallow to the face can
            // pass an edge of it read clear at the crossings, and cross its
            // carrier past that edge, outside the face.
            if stands && !every_edge_read(body, f, &meets.lines) {
                let off = (m.span.0 - t).max(t - m.span.1).max(T::zero());
                let near =
                    geom_core::k_stats::decide_magnitude(rows.represents, Margin::of(off), band)
                        != Ok(geom_core::k_stats::Magnitude::Positive);
                let held = match (c, plane) {
                    _ if near => Ok(Some(true)),
                    (Cross::Plane, Some(p)) => {
                        point_in_face(body, f, p.normal, p.p, band).map_err(at_hit)
                    }
                    (Cross::Root(_), _) => {
                        let geo = geo_of(f)
                            .ok_or(RayFault::Fatal(PointInSolidError::CorruptFace { face: f }))?;
                        if reaches_root_in_band(body, f, (q, d), m, t, band, rows)? {
                            root_held(f, geo, q + d * t, band)
                        } else {
                            Ok(None)
                        }
                    }
                    _ => Ok(None),
                };
                stands = match held {
                    Ok(held) => held == Some(true),
                    Err(fault @ RayFault::Fatal(_)) => return Err(fault),
                    Err(_) => false,
                };
            }
        }
        let [(ta, sa), (tb, sb)] = ends;
        // An edge's half crossings are its faces' crossings of their
        // carriers, which the ray makes through the edge only where both
        // lie ahead of `q`: from a `q` in band of a face's carrier, the
        // ray can pass the face's edge ahead and its carrier behind.
        let ahead = [ta, tb]
            .iter()
            .all(|&t| decide(rows.advance, Margin::of(t), band) == Ok(Sign::Positive));
        if !stands || !ahead || sa == 0 || sb == 0 {
            for at in [m.at, ta, tb] {
                offer(RayHit::marked([a, b], at, Mark::Graze), (at, at), true)?;
            }
            continue;
        }
        // The ray crosses the edge's faces somewhere between where it
        // passes the edge and where it crosses their carriers, which on a
        // face it meets obliquely lie bands apart: the crossing spans them.
        offer(
            RayHit {
                count: (sa + sb) / 2,
                face: a,
                other: Some(b),
                at: m.at,
                mark: None,
            },
            (m.span.0.min(ta).min(tb), m.span.1.max(ta).max(tb)),
            false,
        )?;
    }
    // ---- Each planar face no edge is met at, crossed where it lies. ----
    for c in &planes {
        let face = c.face;
        if meets.met.iter().any(|m| m.faces.contains(&face)) {
            continue;
        }
        // In-face test FIRST: a plane hit outside the face region is no
        // crossing at all — in particular a `t = 0` hit on a face plane
        // through `q` (a corner-aligned query) must be skipped, not
        // grazed, when the face itself is elsewhere.
        let turned = match every_bound_read(body, face, &meets.clear) {
            true => crosses_by_turn(body, face, (c.normal, c.p), reach, turn_ray)?,
            false => None,
        };
        let inside = match turned {
            Some(inside) => Some(inside),
            None => point_in_face(body, face, c.normal, c.p, band).map_err(at_hit)?,
        };
        let point = (c.t, c.t);
        match inside {
            Some(false) => {}
            Some(true) => offer(RayHit::crossing(face, c.t, count(c.outward)), point, false)?,
            None => offer(RayHit::crossing(face, c.t, 0), point, true)?,
        }
    }
    for m in meets.marks {
        offer(m, (m.at, m.at), true)?;
    }
    // A curved face's crossing at `t`, `outward` its sign there.
    let mut offer_root = |face: FaceKey, t: T, outward: Sign, edge: bool| {
        offer(
            RayHit::crossing(face, t, count(outward)),
            (t, t),
            edge || outward == Sign::Zero,
        )
    };
    // ---- Each root of a trimmed curved face no edge stands for, tested
    // against the face's own region: a root outside it is no crossing of
    // this face, and one on its boundary, or one the region cannot place,
    // reads no side. Faces on one carrier (a seam split) share its roots:
    // a face that definitely holds one holds it alone, so the others'
    // readings of it, in band, faulted or not, decide nothing; a fault
    // stands only where no face holds it. ----
    let mut read = vec![false; roots.len()];
    for k in 0..roots.len() {
        if read[k] {
            continue;
        }
        let root_key = key(roots[k].face, Cross::Root(k));
        let shared: Vec<usize> = (k..roots.len())
            .filter(|&j| key(roots[j].face, Cross::Root(j)) == root_key)
            .collect();
        for &j in &shared {
            read[j] = true;
        }
        if claims.iter().any(|(claimed, _)| *claimed == root_key) {
            continue;
        }
        let (mut held, mut fault, mut boundary) = (None, None, None);
        for &j in &shared {
            let r = roots[j];
            let Some(geo) = geo_of(r.face) else {
                continue;
            };
            // A wall face met at no edge, every edge of which the ray was
            // decided clear of, is held where the turn of its loops says.
            let p = q + d * r.t;
            let turned = match *geo {
                FaceGeo::Cylinder {
                    origin,
                    axis,
                    radius,
                    ..
                } if !meets.met.iter().any(|m| m.faces.contains(&r.face))
                    && every_bound_read(body, r.face, &meets.clear) =>
                {
                    wall_crosses_by_turn(body, r.face, (origin, axis, radius), (p, reach), turn_ray)
                }
                _ => Ok(None),
            };
            let reading = match turned {
                Ok(Some(inside)) => Ok(Some(inside)),
                Ok(None) => root_held(r.face, geo, p, band),
                Err(fault) => Err(fault),
            };
            match reading {
                Ok(Some(true)) => held = held.or(Some((r, geo))),
                Ok(Some(false)) => {}
                Ok(None) => boundary = boundary.or(Some(r)),
                Err(f) => fault = worse(fault, r.face, f),
            }
        }
        // A broken face is the query's refusal, whatever another face holds.
        if let Some((_, f @ RayFault::Fatal(_))) = fault {
            return Err(f);
        }
        let fault = fault.map(|(_, f)| f);
        match (held, fault, boundary) {
            (Some((r, geo)), ..) => {
                let outward = root_outward(body, r.face, geo, r, q, d, band)?;
                offer_root(r.face, r.t, outward, false)?;
            }
            (None, Some(fault), _) => return Err(fault),
            (None, None, Some(r)) => offer_root(r.face, r.t, Sign::Zero, true)?,
            (None, None, None) => {}
        }
    }
    for &(face, ref geo) in geos {
        let escalate = |diag| RayFault::InBand(PointInSolidError::Escalated { face, diag });
        match *geo {
            FaceGeo::Plane(..)
            | FaceGeo::Cylinder { .. }
            | FaceGeo::Cone { .. }
            | FaceGeo::SpherePatch { .. }
            | FaceGeo::TorusPatch { .. } => {}
            // A ray that may meet the untrimmable face ahead cannot be
            // answered; one that definitely misses the double cone, or
            // meets it only behind `q`, takes no crossing from it. The
            // quadratic and its margins are the cone arm's above.
            FaceGeo::PartialCone {
                apex,
                axis,
                half_angle,
                reach,
            } => {
                // Only a root definitely ahead and definitely within the
                // ball holding the face blocks the ray; a `Zero` on any
                // of these rows grazes, as on the trimmable arm, since a
                // tighter tolerance could decide it either way.
                let [a2, b2, c2] = line_cone_quadratic(q, d, apex, axis, half_angle);
                if decide("bool_ray_cone_lead", Margin::levered(a2, reach), band)
                    .map_err(escalate)?
                    == Sign::Zero
                {
                    return Err(RayFault::Graze); // generator-parallel
                }
                let disc = b2.powi(2) - a2 * c2;
                match decide("bool_ray_cone_disc", Margin::over_lever(disc, reach), band)
                    .map_err(escalate)?
                {
                    Sign::Negative => continue,
                    Sign::Zero => return Err(RayFault::Graze), // tangent ray
                    Sign::Positive => {}
                }
                for t in quadratic_roots(a2, b2, c2, disc) {
                    let ahead = ray_walk::advance(rows.advance, Margin::of(t), band)
                        .map_err(|f| f.map(|diag| PointInSolidError::Escalated { face, diag }))?;
                    if ahead == Sign::Negative {
                        continue;
                    }
                    let p = q + d * t;
                    match decide(
                        "bool_cone_partial_reach",
                        Margin::of((p - apex).norm() - reach),
                        band,
                    )
                    .map_err(escalate)?
                    {
                        Sign::Positive => {}
                        Sign::Zero => return Err(RayFault::Graze),
                        Sign::Negative => {
                            return Err(RayFault::Blocked(PointInSolidError::PartialConeFace {
                                face,
                            }));
                        }
                    }
                }
            }
            // The cylinder's own: a root definitely ahead and definitely
            // within the ball holding the face blocks the ray.
            FaceGeo::PartialWall {
                origin,
                axis,
                radius,
                anchor,
                reach: ball,
            } => {
                let ts = match line_wall_roots(q, d, origin, axis, radius, reach, band)
                    .map_err(|fault| escalate(fault.diag))?
                {
                    WallRoots::Miss => continue,
                    WallRoots::AxisParallel => {
                        if clear_of_carrier(
                            wall_elevation(origin, axis, radius, q),
                            face,
                            band,
                            rows,
                        )? {
                            continue;
                        }
                        return Err(RayFault::Graze);
                    }
                    WallRoots::Tangent => return Err(RayFault::Graze),
                    WallRoots::Two(ts) => ts,
                };
                for t in ts {
                    let ahead = ray_walk::advance(rows.advance, Margin::of(t), band)
                        .map_err(|f| f.map(|diag| PointInSolidError::Escalated { face, diag }))?;
                    if ahead == Sign::Negative {
                        continue;
                    }
                    match decide(
                        "bool_wall_outline_reach",
                        Margin::of((q + d * t - anchor).norm() - ball),
                        band,
                    )
                    .map_err(escalate)?
                    {
                        Sign::Positive => {}
                        Sign::Zero => return Err(RayFault::Graze),
                        Sign::Negative => {
                            return Err(RayFault::Blocked(
                                PointInSolidError::WallOutlineUnsupported { face },
                            ));
                        }
                    }
                }
            }
            // The full-sphere pierce arm: the quadratic of
            // [`line_sphere_roots`], whose metering that function states.
            //
            // Zero ⇒ the ray is tangent: a graze, retried on
            // the next schedule member, never a parity guess.
            //
            // The outward sign at a root needs NO second predicate:
            // `d·(p − c)/r = (w·d + t)/r = ±√disc/r`, so the near root
            // enters material and the far root exits — read off the
            // discriminant that was already decided definite. A
            // grazing-incidence hit is exactly the Zero-discriminant
            // case, already handled above.
            FaceGeo::Sphere {
                center,
                radius,
                representative,
                sense,
            } => {
                if face != representative {
                    continue;
                }
                let [near, far] =
                    match line_sphere_roots(q, d, center, radius, band).map_err(escalate)? {
                        WallRoots::Two(ts) => ts,
                        WallRoots::Tangent => return Err(RayFault::Graze),
                        WallRoots::Miss | WallRoots::AxisParallel => continue, // definite miss
                    };
                // The near/far outward pair is read off the geometry
                // (`d·(p − c)/r = ±√disc/r`) and is therefore a CHART
                // statement: it says the near root enters the BALL and
                // the far root leaves it. Whether entering the ball
                // means entering material is the face's sense (S10) —
                // a reversed sphere face bounds the material OUTSIDE
                // it — so the pair is swapped rather than recomputed.
                for (t, outward) in [
                    (near, oriented(Sign::Negative, sense)),
                    (far, oriented(Sign::Positive, sense)),
                ] {
                    offer_root(face, t, outward, false)?;
                }
            }
            // The closed torus group: every root of the tube's quartic,
            // for its representative ([`carrier_roots`] reads a trimmed
            // torus face's).
            FaceGeo::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                representative,
                ..
            } => {
                if face != representative {
                    continue;
                }
                let roots =
                    match line_torus_roots(q, d, center, axis, major_radius, minor_radius, band) {
                        Ok(roots) => Ok(roots),
                        Err(RootRefusal::Unsettled(why)) => Err(why),
                        Err(RootRefusal::Escalated(diag)) => return Err(escalate(diag)),
                        // A touch of the ray is this ray's; a fault grazes too
                        // (`work/splitdoor/ray-lanes-graze-on-a-root-finders-fault.md`).
                        Err(RootRefusal::Touch(_) | RootRefusal::Fault(_)) => {
                            return Err(RayFault::Graze);
                        }
                    };
                let Some(found) = torus_crossings(face, roots)? else {
                    continue;
                };
                for &t in found.roots() {
                    let root = RootCross {
                        face,
                        t,
                        side: RootSide::Torus,
                        ordinal: 0,
                    };
                    let outward = root_outward(body, face, geo, root, q, d, band)?;
                    offer_root(face, t, outward, false)?;
                }
            }
        }
    }
    Ok(crossings)
}

/// A query's reading of its selection's side at infinity, taken once
/// whichever of its rays first crosses nothing ([`missed_side`]).
type MissedRead = core::cell::OnceCell<
    Result<Result<SolidContainment, SideUnread<PointInSolidError>>, PointInSolidError>,
>;

/// How the selection's signed volume is measured for its side at
/// infinity ([`at_infinity_side`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Measure {
    /// Closed forms only: a face with none refuses `VolumeUncertified`.
    ClosedForm,
    /// Through the scalar's quadrature lane, refined only until the sign
    /// decides.
    Quadrature,
}

/// One reading of the selection's side at infinity, with the band and
/// tolerance it was read at.
type VolumeRead = ((Band, Tol), Result<SolidContainment, PointInSolidError>);

/// **The selection's side at infinity off its signed volume, one cell
/// per [`Measure`]**: a fact of the selection's faces, read at most once
/// per selection at a band and tolerance and shared by every query of
/// it ([`point_in_solid_faces`] over one [`SolidFaces`], probed against
/// the body it was selected from, as its face keys already are). A
/// reading at another band or tolerance is taken afresh and not kept.
#[derive(Clone, Debug, Default)]
struct VolumeReads {
    closed_form: std::sync::OnceLock<VolumeRead>,
    quadrature: std::sync::OnceLock<VolumeRead>,
}

impl VolumeReads {
    /// The reading by `measure` at `band` and `tol`, read through `read`
    /// where this selection holds none at them.
    fn get(
        &self,
        measure: Measure,
        (band, tol): (Band, Tol),
        read: impl FnOnce() -> Result<SolidContainment, PointInSolidError>,
    ) -> Result<SolidContainment, PointInSolidError> {
        let cell = match measure {
            Measure::ClosedForm => &self.closed_form,
            Measure::Quadrature => &self.quadrature,
        };
        let key = (band, tol);
        let mut read = Some(read);
        let (at, got) = cell.get_or_init(|| {
            let fresh = read
                .take()
                .unwrap_or_else(|| unreachable!("the cell runs one init"));
            (key, fresh())
        });
        // Another thread's reading, at its own band, may have filled the
        // cell first.
        match read {
            Some(read) if *at != key => read(),
            _ => got.clone(),
        }
    }
}

/// How a refusal about the body ranks as one ray's reading: in band is
/// the ray's to set aside, a side no reading could give blocks it, and
/// the rest refuses the query.
fn as_ray_fault(e: PointInSolidError) -> RayFault<PointInSolidError> {
    if e.in_band() {
        RayFault::InBand(e)
    } else if e.sideless() {
        RayFault::Blocked(e)
    } else {
        RayFault::Fatal(e)
    }
}

/// **The side of `q` that a ray crossing no face leaves to read**: the
/// at-infinity side, since that ray, `q + s·d` for `s ≥ 0`, joins `q`
/// to infinity without crossing the boundary (module docs).
///
/// A selection whose role is already decided reads it
/// ([`SolidFaces::with_role`]). Otherwise the selection's side at
/// infinity is read by the one reader of it ([`crate::side_reader::side_at_infinity`]), with
/// the sign of the selection's signed volume ([`at_infinity_side`]) as
/// its fallback: in closed form, and, where the closed form had no value
/// for a face, measured through the scalar's quadrature lane, refined
/// only until it decides. Each reading is the selection's, taken once
/// ([`VolumeReads`]); the quadrature's refusal supersedes the closed
/// form's. An aimed reading in band refuses as the far face's
/// escalation. Where neither the rays nor the volume read, the refusal
/// is ranked as a walk ranks ([`ray_walk::Evidence`]): the first limit
/// an aimed ray met, or the volume's own refusal.
///
/// A selection read as given ([`SolidFaces::read_as_given`]) whose side at
/// infinity reads `In` refuses the query as
/// [`PointInSolidError::InsideOut`].
fn missed_side<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    sel: &SolidFaces,
    band: Band,
    tol: Tol,
    read: &MissedRead,
) -> Result<SolidContainment, RayFault<PointInSolidError>> {
    if let Some(side) = sel.at_infinity {
        return Ok(side);
    }
    let read = read.get_or_init(|| {
        side_at_infinity(body, sel, band, || {
            let volume = |measure| {
                sel.volume.get(measure, (band, tol), || {
                    at_infinity_side(body, sel.faces(), band, tol, measure)
                })
            };
            match volume(Measure::ClosedForm) {
                Err(PointInSolidError::VolumeUncertified) if T::quad_lane().is_some() => {
                    volume(Measure::Quadrature)
                }
                closed => closed,
            }
        })
    });
    let side = match read {
        Err(fatal) => return Err(RayFault::Fatal(fatal.clone())),
        Ok(Ok(side)) => *side,
        Ok(Err(SideUnread::InBand(pair))) => {
            return Err(RayFault::InBand(PointInSolidError::Escalated {
                face: pair.far,
                diag: pair.gap,
            }));
        }
        Ok(Err(
            unread @ SideUnread::Fallback {
                volume, set_aside, ..
            },
        )) => {
            let mut kept = ray_walk::Evidence::default();
            match as_ray_fault(volume.clone()) {
                RayFault::Fatal(e) => return Err(RayFault::Fatal(e)),
                RayFault::InBand(e) => kept.in_band(e),
                RayFault::Blocked(e) => kept.blocked(e),
                RayFault::Graze | RayFault::Unread => {}
            }
            if let Some(e) = unread.limit() {
                kept.blocked(e.clone());
            }
            // A ray set aside at a corner or a wedge read in band there:
            // it is this point's evidence, not a pair.
            for s in set_aside {
                if let crate::side_reader::SetAsideWhy::Corner { faces, gap }
                | crate::side_reader::SetAsideWhy::Wedge { faces, gap } = &s.why
                {
                    kept.in_band(PointInSolidError::Escalated {
                        face: faces[1],
                        diag: *gap,
                    });
                }
            }
            return Err(match kept.ranked() {
                ray_walk::Ranked::Blocked(e) => RayFault::Blocked(e),
                ray_walk::Ranked::InBand(e) => RayFault::InBand(e),
                ray_walk::Ranked::Neither => RayFault::Unread,
            });
        }
    };
    sided(sel, side)
}

/// `side` as the side at infinity of `sel`: a selection read as given
/// whose material holds infinity is inside-out.
fn sided(
    sel: &SolidFaces,
    side: SolidContainment,
) -> Result<SolidContainment, RayFault<PointInSolidError>> {
    if sel.as_given && side == SolidContainment::In {
        return Err(RayFault::Fatal(PointInSolidError::InsideOut));
    }
    Ok(side)
}

/// The at-infinity material side, from the sign of the signed volume
/// the entry's own `faces` enclose: the role walk
/// ([`crate::props::role_walk`]) measured by `measure` and certified
/// through the scalar's lane ([`crate::props::AtRestPolicy::quad_lane`]),
/// so a volume below its own rounding reads no side rather than a wrong
/// one. Margin is scaled to a length (V / surface area, meters).
///
/// The volume is the SELECTION's, not the body's: for
/// [`point_in_solid`] the two coincide (the arena list), and for
/// [`point_in_solid_of`] a no-hit ray's side is the queried solid's
/// orientation — the body's total would read the other instances'
/// volumes into this solid's side, and would refuse this solid for a
/// neighbour's uncertifiable face.
fn at_infinity_side<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    faces: &[FaceKey],
    band: Band,
    tol: Tol,
    measure: Measure,
) -> Result<SolidContainment, PointInSolidError> {
    use crate::props::ShellRole;
    let lane = match measure {
        Measure::ClosedForm => None,
        Measure::Quadrature => T::quad_lane(),
    };
    let degenerate = |face| {
        matches!(
            crate::props::mass_properties_closed_form_of(body, &[face], band, tol),
            Err(crate::props::MassPropsError::Face {
                source: geom_brep::props::PropsError::DegenerateFace,
                ..
            })
        )
    };
    let refused = |e| volume_refusal(e, faces[0], measure, degenerate);
    let (role, _) = crate::props::role_walk(
        body,
        faces,
        band,
        tol,
        lane,
        T::quad_lane(),
        crate::props::SignNames {
            walk: AT_INFINITY,
            certified: AT_INFINITY_ENCLOSURE,
        },
    )
    .map_err(refused)?;
    let role = role.map_err(|refusal| match refusal {
        crate::props::RoleWalkRefusal::Refused(source) => refused(source),
        // A sign left open by a limit of the measurement is the capability
        // gap `VolumeUncertified` names, never the body's.
        crate::props::RoleWalkRefusal::Unread { limit: Some(_), .. } => {
            PointInSolidError::VolumeUncertified
        }
        crate::props::RoleWalkRefusal::Poisoned(last)
        | crate::props::RoleWalkRefusal::Unread { last, .. } => unread_refusal(&last, faces[0]),
    });
    // An `Outer` boundary leaves infinity outside its material, a `Void`
    // one inside.
    match role? {
        ShellRole::Outer => Ok(SolidContainment::Out),
        ShellRole::Void => Ok(SolidContainment::In),
    }
}

/// **A props refusal of the selection's volume, as the probe's**, `first`
/// the selection's first face. It is read, not flattened:
/// `VolumeUncertified`'s own message says the solid is fine and only its
/// volume could not be measured, and two kinds of refusal make that
/// false. An escalation is an ill-conditioned operand at this ε (with a
/// predicate name and a band the caller can act on). The corruption arms
/// are arena claims about a BROKEN body: a key the walk could not
/// resolve, null scaffolding at rest, or a face whose exact area will
/// not certify a positive extent. Read through the quadrature, a face
/// refused as degenerate is corrupt only where its closed form says so
/// too (`degenerate_in_closed_form`); otherwise it is a limit of the
/// quadrature's enclosure of a face the closed form had no value for,
/// so it refuses this reading and the walk goes on. The rest is the
/// capability gap the variant describes.
fn volume_refusal(
    e: crate::props::MassPropsError,
    first: FaceKey,
    measure: Measure,
    degenerate_in_closed_form: impl Fn(FaceKey) -> bool,
) -> PointInSolidError {
    use crate::props::MassPropsError as M;
    use geom_brep::props::PropsError as P;
    match e {
        M::Face {
            face,
            source: P::Escalated { cause, .. },
        } => PointInSolidError::Escalated { face, diag: cause },
        M::Face {
            face,
            source: P::DegenerateFace,
        } if measure == Measure::ClosedForm || degenerate_in_closed_form(face) => {
            PointInSolidError::CorruptFace { face }
        }
        M::Corrupt { .. } | M::NullScaffoldEdge { .. } => {
            PointInSolidError::CorruptFace { face: first }
        }
        M::Band { .. }
        | M::RingOnCurvedFace { .. }
        | M::Face { .. }
        | M::ResolutionFinerThanEps { .. }
        | M::EmptyContains
        | M::Poisoned { .. }
        | M::Undecided { .. } => PointInSolidError::VolumeUncertified,
    }
}

/// **A role read that decided no role, as the probe's refusal**, `first`
/// the selection's first face: an end's escalation; a zero volume where
/// the reading that decides it reads zero at both ends
/// ([`crate::props::RoleUnread::reads_zero`]); and otherwise an unmeasured
/// volume, since a bracket with an end off zero says nothing of zero.
fn unread_refusal(unread: &crate::props::RoleUnread, first: FaceKey) -> PointInSolidError {
    match unread.escalation() {
        Some(diag) => PointInSolidError::Escalated { face: first, diag },
        None if unread.reads_zero() => PointInSolidError::ZeroVolumeBody,
        None => PointInSolidError::VolumeUncertified,
    }
}

/// The side-at-infinity decision's names on the walk's sums.
const AT_INFINITY: crate::props::RoleNames =
    crate::props::RoleNames::one("bool_point_in_solid_infinity");

/// The side-at-infinity decision's name on the interval re-derivation.
const AT_INFINITY_ENCLOSURE: crate::props::RoleNames =
    crate::props::RoleNames::one("bool_point_in_solid_infinity_enclosure");

#[cfg(test)]
mod volume_refusal_rows {
    //! How the probe types the volume's refusals: a read that decided no
    //! role ([`unread_refusal`]) and a props refusal ([`volume_refusal`]).

    use super::*;
    use crate::props::MassPropsError;
    use crate::props::role_unread_rows::{Read, rows, unread};
    use geom_brep::props::PropsError;

    #[test]
    fn an_undecided_read_is_a_zero_volume_only_where_its_reading_says_zero() {
        let first = FaceKey::default();
        for (what, walk, certified, read) in rows() {
            let got = unread_refusal(&unread(walk, certified), first);
            match read {
                Read::Zero => assert_eq!(got, PointInSolidError::ZeroVolumeBody, "{what}"),
                Read::HoldsZero => assert!(
                    matches!(got, PointInSolidError::Escalated { face, .. } if face == first),
                    "{what}: {got:?}"
                ),
                Read::Unmeasured => {
                    assert_eq!(got, PointInSolidError::VolumeUncertified, "{what}");
                }
            }
        }
    }

    #[test]
    fn a_degenerate_face_is_corrupt_by_its_closed_form_and_else_a_limit_of_the_quadrature() {
        let face = FaceKey::default();
        let degenerate = || MassPropsError::Face {
            face,
            source: PropsError::DegenerateFace,
        };
        assert_eq!(
            volume_refusal(degenerate(), face, Measure::ClosedForm, |_| false),
            PointInSolidError::CorruptFace { face },
            "in closed form"
        );
        assert_eq!(
            volume_refusal(degenerate(), face, Measure::Quadrature, |_| true),
            PointInSolidError::CorruptFace { face },
            "through the quadrature, on a face whose closed form is degenerate too"
        );
        let measured = volume_refusal(degenerate(), face, Measure::Quadrature, |_| false);
        assert_eq!(
            measured,
            PointInSolidError::VolumeUncertified,
            "through the quadrature"
        );
        assert!(
            matches!(as_ray_fault(measured), RayFault::Blocked(_)),
            "the quadrature's refusal blocks the ray and the walk goes on"
        );
    }
}

#[cfg(test)]
mod volume_read_key_rows {
    //! A selection's volume reading serves only the band and tolerance
    //! it was read at, whichever thread read it first.

    #![allow(clippy::unwrap_used)]
    use super::*;

    fn bands() -> (Band, Band) {
        (
            Band::new(1e-9, 1e-6).unwrap(),
            Band::new(1e-8, 1e-5).unwrap(),
        )
    }

    #[test]
    fn a_second_band_is_read_afresh() {
        let (b1, b2) = bands();
        let tol = Tol::witness();
        let r = VolumeReads::default();
        assert_eq!(
            r.get(Measure::ClosedForm, (b1, tol), || Ok(SolidContainment::Out)),
            Ok(SolidContainment::Out)
        );
        assert_eq!(
            r.get(Measure::ClosedForm, (b2, tol), || Err(
                PointInSolidError::VolumeUncertified
            )),
            Err(PointInSolidError::VolumeUncertified),
            "band 2 served band 1's reading"
        );
    }

    #[test]
    fn a_second_band_racing_the_first_is_read_afresh() {
        let (b1, b2) = bands();
        let tol = Tol::witness();
        let r = VolumeReads::default();
        std::thread::scope(|s| {
            s.spawn(|| {
                r.get(Measure::ClosedForm, (b1, tol), || {
                    std::thread::sleep(std::time::Duration::from_millis(300));
                    Ok(SolidContainment::Out)
                })
            });
            std::thread::sleep(std::time::Duration::from_millis(50));
            let got = r.get(Measure::ClosedForm, (b2, tol), || {
                Err(PointInSolidError::VolumeUncertified)
            });
            assert_eq!(
                got,
                Err(PointInSolidError::VolumeUncertified),
                "band 2 served band 1's reading under a race"
            );
        });
    }
}

#[cfg(test)]
#[path = "r1_probes.rs"]
mod r1_probes;

mod curved_edges;

#[cfg(test)]
mod r1_generic_poses;

#[cfg(test)]
#[path = "torus_predicate_rows.rs"]
mod torus_predicate_rows;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod per_solid_entry_tests {
    //! The per-solid entry against its whole-body twin, and a key
    //! shared across the selection boundary written by an in-crate hand.

    use super::*;
    use crate::splitting::reassembly::quad_prism;

    fn two_cubes() -> Body<f64> {
        let tol = Tol::witness();
        let mut body = quad_prism(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 1.0, tol);
        let other = quad_prism(&[(3.0, 0.0), (4.0, 0.0), (4.0, 1.0), (3.0, 1.0)], 1.0, tol);
        crate::instance::graft_disjoint(&mut body, &other).unwrap();
        body
    }

    /// **A ray reads its count where it first turns, and a hit on an
    /// edge past it does not matter** (the far-edge rule). From inside the
    /// first cube the ray leaves through its face `x = 1`, then meets the
    /// second cube exactly on its top edge (`x = 3, z = 1`). From between
    /// the two cubes that edge is the first thing the ray meets, and the
    /// ray grazes there.
    /// **Two edges of faces the census reads definite apart are no
    /// pair** (PR 5422's review R2). Two unit cubes `g` apart, `g` 12ε to
    /// 19ε, their facing faces definite at `Kε`; a ray crosses the gap's
    /// mouth into the second cube near the first's top edge. The ray can
    /// pass the two cubes' edges in band of each other along it, while
    /// their faces' planes it crosses at least `g` apart: it reads, or is
    /// set aside, and never refuses on a pair (`bool_point_in_solid_order`
    /// in band).
    #[test]
    fn two_edges_of_faces_definite_apart_are_no_pair() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        for m in [12.0, 15.0, 19.0] {
            let g = m * tol.eps();
            let mut body = quad_prism(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 1.0, tol);
            let other = quad_prism(
                &[
                    (1.0 + g, 0.0),
                    (2.0 + g, 0.0),
                    (2.0 + g, 1.0),
                    (1.0 + g, 1.0),
                ],
                1.0,
                tol,
            );
            crate::instance::graft_disjoint(&mut body, &other).unwrap();
            let sel = SolidFaces::select(&body, body.faces().map(|(k, _)| k).collect()).unwrap();
            let geos: Vec<_> = sel
                .faces()
                .iter()
                .map(|&face| (face, face_geo(&body, face, &sel.charts).unwrap()))
                .collect();
            let d = Vec3::new(0.5, 0.3, -1.0).normalize();
            let q = Point3::new(1.0 + 0.5 * g, 0.5, 1.0) - d * 2.0;
            let reach = selection_reach(&body, sel.faces(), q).unwrap();
            let got = cast_ray(&body, &geos, q, d, reach, band);
            assert!(
                !matches!(
                    &got,
                    Err(RayFault::InBand(PointInSolidError::Escalated { diag, .. }))
                        if diag.predicate == Some(SCHEDULE_ROWS.order)
                ),
                "cubes {m}ε apart: the ray refuses on a pair, {got:?}"
            );
        }
    }

    /// **A carrier crossing beside another face is no pair** (PR 5422's
    /// review R2, re-check of `2be7c1ad14`). A wall `1.5` tall, a gap of
    /// 3Kε to 8Kε, then a unit box. A shallow ray leaves the wall near
    /// `z = 1` and passes the box's top edge in band; the top's carrier it
    /// crosses a few ε from the wall's, far off the top face itself. The
    /// census reads the wall and the top definite apart, so the ray reads
    /// `In` or is set aside, and never refuses on them.
    #[test]
    fn an_oblique_carrier_crossing_beside_another_face_is_no_pair() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        for m in [3.0, 5.0, 8.0] {
            for slope in [0.05, 0.1, 0.15] {
                let l = m * tol.k() * tol.eps();
                let mut body =
                    quad_prism(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 1.5, tol);
                let other = quad_prism(
                    &[
                        (1.0 + l, 0.0),
                        (2.0 + l, 0.0),
                        (2.0 + l, 1.0),
                        (1.0 + l, 1.0),
                    ],
                    1.0,
                    tol,
                );
                crate::instance::graft_disjoint(&mut body, &other).unwrap();
                let sel =
                    SolidFaces::select(&body, body.faces().map(|(k, _)| k).collect()).unwrap();
                let geos: Vec<_> = sel
                    .faces()
                    .iter()
                    .map(|&face| (face, face_geo(&body, face, &sel.charts).unwrap()))
                    .collect();
                let d = Vec3::new(1.0, 0.05, -slope).normalize();
                let q = Point3::new(1.0, 0.5, 1.0 + 3.0 * tol.eps() * slope) - d * 0.3;
                let reach = selection_reach(&body, sel.faces(), q).unwrap();
                let got = cast_ray(&body, &geos, q, d, reach, band);
                let pair = matches!(
                    &got,
                    Err(RayFault::InBand(PointInSolidError::Escalated { diag, .. }))
                        if diag.predicate == Some(SCHEDULE_ROWS.order)
                );
                assert!(
                    !pair && !matches!(got, Ok(Some(SolidContainment::Out))),
                    "a gap of {m}Kε, slope {slope}: {got:?}"
                );
            }
        }
    }

    #[test]
    fn an_edge_hit_past_the_closest_crossing_is_passed_over() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let body = two_cubes();
        let sel = SolidFaces::select(&body, body.faces().map(|(k, _)| k).collect()).unwrap();
        let geos: Vec<_> = sel
            .faces()
            .iter()
            .map(|&face| (face, face_geo(&body, face, &sel.charts).unwrap()))
            .collect();
        let cast = |q: Point3<f64>, d: Vec3<f64>| {
            let reach = selection_reach(&body, sel.faces(), q).unwrap();
            cast_ray(&body, &geos, q, d.normalize(), reach, band)
        };
        let inside = Point3::new(0.5, 0.5, 0.5);
        assert!(matches!(
            cast(inside, Vec3::new(2.5, 0.0, 0.5)),
            Ok(Some(SolidContainment::In))
        ));
        let between = Point3::new(2.5, 0.5, 0.5);
        assert!(matches!(
            cast(between, Vec3::new(0.5, 0.0, 0.5)),
            Err(RayFault::Graze)
        ));
    }

    fn solids(body: &Body<f64>) -> (SolidKey, SolidKey) {
        let v: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
        (v[0], v[1])
    }

    /// A key shared across the selection boundary is served: the query
    /// decides on the selection's own faces.
    #[test]
    fn a_shared_plane_key_is_served() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut body = two_cubes();
        let (a, b) = solids(&body);
        let fa = body.faces_of_solid(a).unwrap()[0];
        let fb = body.faces_of_solid(b).unwrap()[0];
        let key = body.get_face(fa).unwrap().surface;
        // Re-point a face of `b` at `a`'s plane key: `b` is no longer a
        // sound body, but `a`'s query never reads `b`'s faces.
        body.faces[fb].surface = key;
        assert_eq!(
            point_in_solid_of(&body, a, Point3::new(0.5, 0.5, 0.5), band, tol).unwrap(),
            SolidContainment::In
        );
    }

    /// The entries agree with each other where the body IS one solid,
    /// and the per-solid entry answers for one solid where it is two.
    #[test]
    fn the_two_entries_agree_on_a_single_solid_and_split_a_pair() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let body = two_cubes();
        let (a, b) = solids(&body);
        for (q, in_a, in_b) in [
            (
                Point3::new(0.5, 0.5, 0.5),
                SolidContainment::In,
                SolidContainment::Out,
            ),
            (
                Point3::new(3.5, 0.5, 0.5),
                SolidContainment::Out,
                SolidContainment::In,
            ),
            (
                Point3::new(2.0, 0.5, 0.5),
                SolidContainment::Out,
                SolidContainment::Out,
            ),
            (
                Point3::new(1.0, 0.5, 0.5),
                SolidContainment::OnBoundary,
                SolidContainment::Out,
            ),
        ] {
            assert_eq!(
                point_in_solid_of(&body, a, q, band, tol).unwrap(),
                in_a,
                "{q:?}"
            );
            assert_eq!(
                point_in_solid_of(&body, b, q, band, tol).unwrap(),
                in_b,
                "{q:?}"
            );
            // The whole-body door sees both.
            let whole = point_in_solid(&body, q, band, tol).unwrap();
            let expected = match (in_a, in_b) {
                (SolidContainment::In, _) | (_, SolidContainment::In) => SolidContainment::In,
                (SolidContainment::OnBoundary, _) | (_, SolidContainment::OnBoundary) => {
                    SolidContainment::OnBoundary
                }
                _ => SolidContainment::Out,
            };
            assert_eq!(whole, expected, "{q:?}");
        }
    }
}

#[cfg(test)]
#[allow(clippy::panic)]
mod wall_root_rows {
    //! [`line_wall_roots`]' two rungs are lengths: the axis-parallel
    //! rung reads the line's drift off its distance from the axis over
    //! the run its caller reads, so a wall's size does not decide it.

    use super::*;
    use geom_core::{Band, Tol};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap_or_else(|e| panic!("the witness band: {e:?}"))
    }

    /// The roots of the line `q + d·t` on the wall of radius `r` about
    /// the `z` axis, over a run of `span`.
    fn roots(q: Point3<f64>, d: Vec3<f64>, r: f64, span: f64) -> WallRoots<f64> {
        let z = Vec3::new(0.0, 0.0, 1.0);
        line_wall_roots(q, d, Point3::new(0.0, 0.0, 0.0), z, r, span, band())
            .unwrap_or_else(|f| panic!("the rungs decide: {f:?}"))
    }

    /// **A ray straight across a wide wall's axis has its two roots.**
    /// The radius is `1/ε`, so the retired `|d⊥|²/2r` read `ε/2` here,
    /// in the zero band, and the wall arm skipped every ray of the
    /// schedule: a point on the axis of a rod that wide read `Out`
    /// (a scaled oblique rod at ε = 1e-6).
    #[test]
    fn a_ray_across_a_wide_walls_axis_has_both_roots() {
        let r = 1.0 / Tol::witness().eps();
        let got = roots(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            r,
            4.0 * r,
        );
        let WallRoots::Two([t0, t1]) = got else {
            panic!("both roots: {got:?}");
        };
        assert!(
            (t0 + r).abs() <= r * 1e-15 && (t1 - r).abs() <= r * 1e-15,
            "the roots are the radius either side: {t0}, {t1}"
        );
    }

    /// **A near-axis ray's in-span exit root is right to its row's ε.**
    /// Each line runs all but parallel to the axis, so the lead
    /// `|d⊥|²` is tiny, and its exit root's numerator `−b + √disc`
    /// cancels: spelled that way, the root is off by `1.1e-11` (against
    /// ε = 1e-12) and `8.4e-8` (against ε = 1e-9), where
    /// [`quadratic_roots`]' `c/(−b − √disc)` is off by `4.5e-14` and
    /// `4.5e-11`. Each expected root is the exact root of the row's own
    /// `f64` data, rounded once.
    #[test]
    fn a_near_axis_rays_exit_root_is_right_to_the_band() {
        let tol = Tol::witness();
        let z = Vec3::new(0.0, 0.0, 1.0);
        let rows = [
            // ε, radius, q, d⊥, span, the exit root
            (
                1e-12,
                0.5,
                (0.484_375, 0.0625),
                (2f64.powi(-18), 2f64.powi(-19)),
                4096.0,
                2_878.535_214_595_601,
            ),
            (1e-9, 1e3, (999.0, 0.0), (1e-6, 0.0), 2e6, 1e6),
        ];
        let mut off = Vec::new();
        for (eps, r, (qx, qy), (dx, dy), span, exit) in rows {
            let band =
                Band::linear_at(tol, eps).unwrap_or_else(|e| panic!("the band at {eps}: {e:?}"));
            let d = Vec3::new(dx, dy, (1.0 - dx * dx - dy * dy).sqrt());
            let got = line_wall_roots(
                Point3::new(qx, qy, 0.0),
                d,
                Point3::new(0.0, 0.0, 0.0),
                z,
                r,
                span,
                band,
            )
            .unwrap_or_else(|f| panic!("the rungs decide at {eps}: {f:?}"));
            let WallRoots::Two([_, t1]) = got else {
                panic!("both roots at {eps}: {got:?}");
            };
            if (t1 - exit).abs() > eps {
                off.push(format!(
                    "at ε = {eps}: {t1} against {exit}, off by {:e}",
                    (t1 - exit).abs()
                ));
            }
        }
        assert!(off.is_empty(), "the exit roots: {off:?}");
    }

    /// **The axis-parallel rung reads the drift over the run.** A ray
    /// from the axis of a unit wall, tilted off it so that over a unit
    /// run it drifts `ε/4` from the axis, stays inside the band there and
    /// is axis-parallel; tilted to drift `4Kε`, it is not, and has its
    /// roots far beyond the run.
    #[test]
    fn the_axis_parallel_rung_reads_the_drift_over_the_run() {
        let tol = Tol::witness();
        let ray = |drift: f64| Vec3::new(drift, 0.0, 1.0).normalize();
        let q = Point3::new(0.0, 0.0, 0.0);
        let near = roots(q, ray(tol.eps() / 4.0), 1.0, 1.0);
        assert!(
            matches!(near, WallRoots::AxisParallel),
            "a drift of ε/4 over the run: {near:?}"
        );
        let clear = roots(q, ray(4.0 * tol.k() * tol.eps()), 1.0, 1.0);
        let WallRoots::Two([t0, t1]) = clear else {
            panic!("a drift of 4Kε over the run has roots: {clear:?}");
        };
        assert!(
            t0 < -1.0 && t1 > 1.0,
            "the roots lie beyond the unit run: {t0}, {t1}"
        );
    }
}

/// **The boundary walks panic on a torn boundary curve**, where a region
/// would step over it as null scaffolding.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torn_hop_rows {
    use super::*;
    use crate::entity::GeomRef;
    use crate::live::OPERATORS_KEEP_LINKS;
    use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};

    /// Drops the curve of the first edge of `face`'s outer walk.
    fn drop_first_curve(body: &mut Body<f64>, face: FaceKey) -> String {
        let outer = body.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("the fixture's loop is a cycle");
        };
        let edge = body.get_half_edge(first).unwrap().edge;
        let curve = body.get_edge(edge).unwrap().curve;
        body.curves.remove(curve);
        format!(
            "{}'s curve names {}",
            EntityId::Edge(edge),
            GeomRef::Curve(curve)
        )
    }

    #[test]
    fn the_torus_region_panics_on_a_torn_curve() {
        let (mut body, face) = crate::boolean::boxes::tests::torus_wall(
            Point3::origin(),
            Vec3::unit_z(),
            Vec3::unit_x(),
            2.0,
            0.5,
            (0.3, 1.9),
            (-0.7, 0.8),
        );
        assert!(
            torus_face_region(&body, face).unwrap().is_some(),
            "the sound wall's region"
        );
        let named = drop_first_curve(&mut body, face);
        assert_torn_op_panics(
            "torus_face_region",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| torus_face_region(b, face).map(|r| r.is_some()),
        );
    }

    #[test]
    fn the_ruled_region_panics_on_a_torn_curve() {
        let tol = geom_core::Tol::witness();
        let mut body = Body::<f64>::new();
        let face = crate::test_support_fixtures::cyl_wall_sheet(
            &mut body,
            crate::test_support_fixtures::CylFrame::canonical(1.0),
            (0.2, 1.4),
            (0.0, 1.0),
            tol,
        );
        assert!(
            ruled_face_region(&body, face).unwrap().is_some(),
            "the sound wall's region"
        );
        let named = drop_first_curve(&mut body, face);
        assert_torn_op_panics(
            "ruled_face_region",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| ruled_face_region(b, face).map(|r| r.is_some()),
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod escalated_ending_rows {
    use geom_core::{MarginDiag, UNREADABLE_MARGIN_NOTE};

    use super::*;
    use crate::boolean::ContainError;

    /// **A solid door's escalation ends as the unnamed placement does**
    /// (D4 ¶1 (i): one recourse per decision): the containment door
    /// carries the same escalation — the sphere region's, on the same face
    /// — as `ContainError::Escalated` naming no decision, and both end in
    /// the placement's own lever. A point has no coincidence to declare,
    /// and a poisoned margin keeps the lever with the build's
    /// unreadable-margin note: another point is a way through, as
    /// `PointInSolidError::in_band` says.
    #[test]
    fn a_solid_escalation_ends_as_the_unnamed_placement_does() {
        let face = FaceKey::default();
        for margin in [MarginDiag::INVALID, MarginDiag::value(5e-9)] {
            let diag = Indeterminate {
                margin,
                band: Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("bool_ruled_region_cross"),
                terminal_sliver: false,
            };
            let contfp = ContainError::Escalated {
                decision: None,
                escalation: Escalation::Margin,
                diag,
            }
            .to_string();
            let placement = contfp.strip_prefix("contfp: ").unwrap();
            for (label, e) in [
                ("solid door", PointInSolidError::Escalated { face, diag }),
                (
                    "sphere region",
                    RegionRefusal::Escalated(diag).of_face(face),
                ),
            ] {
                let text = e.to_string();
                assert_eq!(
                    text.strip_prefix("cannot tell what is inside the solid: "),
                    Some(placement),
                    "{label} on {margin}"
                );
                assert!(!text.contains("declare"), "{label}: {text}");
                assert_eq!(
                    text.ends_with(UNREADABLE_MARGIN_NOTE),
                    margin.is_invalid(),
                    "{label}: {text}"
                );
            }
        }
    }
}

/// **A torus wall whose carrier is stated whole periods off reads as its
/// twin.** The bottom parallel's carrier parameter runs 5 and 10 periods
/// below the azimuth it covers; the region reads each root against the
/// span it was stated in, so every point of a lattice over the wall's
/// chart and past it reads as the principal wall does: in the patch. The
/// same loop under the chart's own normal runs clockwise, and its face
/// is the patch's complement.
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod stated_periods_rows {
    use super::*;

    #[test]
    fn a_torus_wall_stated_periods_off_reads_as_its_twin() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (us, vs) = ((0.3, 1.9), (-0.7, 0.8));
        let on = |u: f64, v: f64| {
            Point3::new(
                (2.0 + 0.5 * v.cos()) * u.cos(),
                (2.0 + 0.5 * v.cos()) * u.sin(),
                0.5 * v.sin(),
            )
        };
        for periods in [0.0, 5.0, 10.0] {
            let (body, face) = crate::boolean::boxes::tests::torus_wall_below(
                periods,
                Point3::origin(),
                Vec3::unit_z(),
                Vec3::unit_x(),
                2.0,
                0.5,
                us,
                vs,
            );
            let region = torus_face_region(&body, face).unwrap().unwrap();
            for i in 0..9 {
                for j in 0..9 {
                    let (u, v) = (-0.4 + 0.33 * f64::from(i), -1.3 + 0.31 * f64::from(j));
                    let inside = us.0 < u && u < us.1 && vs.0 < v && v < vs.1;
                    assert_eq!(
                        region.contains(on(u, v), band),
                        Ok(Some(inside)),
                        "{periods} periods: (u, v) = ({u}, {v})"
                    );
                }
            }
        }
        let (body, face) = crate::boolean::boxes::tests::torus_wall_clockwise(
            Point3::origin(),
            Vec3::unit_z(),
            Vec3::unit_x(),
            2.0,
            0.5,
            us,
            vs,
        );
        let region = torus_face_region(&body, face).unwrap().unwrap();
        for (u, v) in [(1.1, 0.05), (-0.2, 0.05), (1.1, 1.5), (2.5, -1.0)] {
            let inside = us.0 < u && u < us.1 && vs.0 < v && v < vs.1;
            assert_eq!(
                region.contains(on(u, v), band),
                Ok(Some(!inside)),
                "clockwise: (u, v) = ({u}, {v})"
            );
        }
    }

    /// **A wall and its clockwise twin read complementary sides at random
    /// points** (a counterexample search: the seed varies and is logged,
    /// the count rides `CAD_FUZZ_EFFORT`). Every point of the torus a
    /// micron clear of the patch's boundary reads `In` on the wall exactly
    /// where it is in the patch, and on the clockwise twin exactly where it
    /// is not.
    #[test]
    fn a_wall_and_its_clockwise_twin_read_complementary_sides() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (us, vs) = ((0.3, 1.9), (-0.7, 0.8));
        let frame = (Point3::origin(), Vec3::unit_z(), Vec3::unit_x(), 2.0, 0.5);
        let (wall, wf) = crate::boolean::boxes::tests::torus_wall(
            frame.0, frame.1, frame.2, frame.3, frame.4, us, vs,
        );
        let (twin, tf) = crate::boolean::boxes::tests::torus_wall_clockwise(
            frame.0, frame.1, frame.2, frame.3, frame.4, us, vs,
        );
        let (wall, twin) = (
            torus_face_region(&wall, wf).unwrap().unwrap(),
            torus_face_region(&twin, tf).unwrap().unwrap(),
        );
        let mut rng = test_utils::fuzz::start("torus_wall_clockwise_twin");
        let tau = core::f64::consts::TAU;
        let mut read = 0;
        for _ in 0..test_utils::fuzz::scaled(200) {
            let (u, v) = (
                rng.range(0.0, tau),
                rng.range(-core::f64::consts::PI, core::f64::consts::PI),
            );
            let clear = |x: f64, (a, b): (f64, f64)| (x - a).abs().min((x - b).abs()) > 1e-6;
            if !clear(u, us) || !clear(v, vs) {
                continue;
            }
            let inside = us.0 < u && u < us.1 && vs.0 < v && v < vs.1;
            let p = Point3::new(
                (2.0 + 0.5 * v.cos()) * u.cos(),
                (2.0 + 0.5 * v.cos()) * u.sin(),
                0.5 * v.sin(),
            );
            let replay = test_utils::fuzz::replay();
            assert_eq!(
                wall.contains(p, band),
                Ok(Some(inside)),
                "({u}, {v}), {replay}"
            );
            assert_eq!(
                twin.contains(p, band),
                Ok(Some(!inside)),
                "({u}, {v}), {replay}"
            );
            read += 1;
        }
        assert!(read > 0, "no point clear of the boundary");
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod kind_arm_tests {
    //! [`ray_crosses`] is [`face_geo`]'s arms, kind by kind.

    use super::*;

    /// **The predicate is the dispatch's arms**: a brick's face given a
    /// surface of each kind, read by the dispatch alone (past
    /// [`uncrossable`]), refuses as `KindUnsupported` exactly where
    /// [`ray_crosses`] says no ray crosses it. An arm added for a kind the
    /// predicate refuses would be unreachable, and a kind the predicate
    /// admits with no arm would fault every ray; either turns this red.
    #[test]
    fn face_geo_arms_are_the_kinds_rays_cross() {
        use geom::SurfaceKind;
        let tol = Tol::witness();
        let (o, z, x) = (
            Point3::new(0.5, 0.5, 0.5),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        for kind in SurfaceKind::ALL {
            // Every kind, named: a new one is a compile error here.
            let surface: Surface<f64> =
                match kind {
                    SurfaceKind::Plane => Surface::Plane {
                        origin: o,
                        normal: z,
                        u_ref: x,
                    },
                    SurfaceKind::Cylinder => Surface::Cylinder {
                        origin: o,
                        axis: z,
                        radius: 0.5,
                        u_ref: x,
                    },
                    SurfaceKind::Cone => Surface::Cone {
                        apex: o,
                        axis: z,
                        half_angle: 0.5,
                        u_ref: x,
                    },
                    SurfaceKind::Sphere => Surface::Sphere {
                        center: o,
                        radius: 0.5,
                        axis: z,
                        u_ref: x,
                    },
                    SurfaceKind::Torus => Surface::Torus {
                        center: o,
                        axis: z,
                        major_radius: 0.4,
                        minor_radius: 0.1,
                        u_ref: x,
                    },
                    SurfaceKind::Nurbs => Surface::nurbs_placeholder(),
                    SurfaceKind::Approx => Surface::Approx(std::sync::Arc::new(
                        crate::fixtures::bowed_offset_approx::<f64>(),
                    )),
                };
            let mut body =
                crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
            let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
            let face = faces[0];
            let key = body.get_face(face).unwrap().surface;
            *body.surfaces.get_mut(key).unwrap() = surface;
            let sel = SolidFaces::select(&body, faces).unwrap();
            let read = face_geo_arms(&body, face, &sel.charts);
            let refused = matches!(read, Err(PointInSolidError::KindUnsupported { .. }));
            assert_eq!(
                refused,
                !ray_crosses(kind),
                "{kind:?}: the dispatch has an arm iff rays cross it"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod arc_turn_rows {
    //! [`arc_turn`] against the turn a fine polyline along the arc makes,
    //! and, nearer the arc than a polyline's chords lie, against the turn's
    //! closed form.

    use super::*;

    /// **Near the arc, and at the spans the turn is fed at its ends**: arcs
    /// of exactly half a turn either way (the halves of a closed circle),
    /// of a millionth of a radian, and between, about points `10⁻⁹` and
    /// `10⁻¹²` inside and outside the circle at the arc's middle and its
    /// quarter, against the closed form: the angle from `a − p` to
    /// `b − p`, taken the arc's way round past the cut where `p` lies
    /// inside the circle. A point that near reads on the right side of the
    /// arc's one cut, or the turn is `2π` off.
    #[test]
    fn an_arcs_turn_is_its_closed_form_within_rounding_of_it() {
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let on = |r: f64, t: f64| Point3::new(r * t.cos(), r * t.sin(), 0.0);
        let pi = core::f64::consts::PI;
        for span in [pi, -pi, 1e-6, -1e-6, 0.5, -2.5] {
            for t0 in [0.0, 1.0, -2.5] {
                let t1 = t0 + span;
                let (a, m, b) = (on(1.0, t0), on(1.0, t0 + span / 2.0), on(1.0, t1));
                for at in [t0 + span / 2.0, t0 + span / 4.0] {
                    for (r, inside) in [
                        (1.0 - 1e-9, true),
                        (1.0 + 1e-9, false),
                        (1.0 - 1e-12, true),
                        (1.0 + 1e-12, false),
                    ] {
                        let p = on(r, at);
                        let (u, w) = (a - p, b - p);
                        let principal = normal.dot(u.cross(w)).atan2(u.dot(w));
                        let want = match (inside, span > 0.0) {
                            (true, true) if principal <= 0.0 => principal + 2.0 * pi,
                            (true, false) if principal >= 0.0 => principal - 2.0 * pi,
                            _ => principal,
                        };
                        let got = arc_turn(normal, p, (a, m, b));
                        assert!(
                            (got - want).abs() < 1e-6,
                            "span {span}, t0 {t0}, r {r} at {at}: {got} against {want}"
                        );
                    }
                }
            }
        }
    }

    /// **An arc's turn about a point is its polyline's**, for arcs of a
    /// tenth to nine tenths of a half turn either way round, about points
    /// inside and outside the circle, on the chord and on its line past
    /// the arc, on the circle past the arc and on the axis through the
    /// arc's middle (the poses where
    /// a chord's turn, or a test of which side of the chord the point
    /// lies, ties): a cut anywhere but on the arc reads `2π` off.
    #[test]
    fn an_arcs_turn_is_its_polylines_wherever_the_point_is_off_it() {
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let on = |r: f64, t: f64| Point3::new(r * t.cos(), r * t.sin(), 0.0);
        let polyline = |p: Point3<f64>, t0: f64, t1: f64| {
            let n = 4096;
            (0..n)
                .map(|k| {
                    let (a, b) = (
                        on(1.0, t0 + (t1 - t0) * f64::from(k) / f64::from(n)),
                        on(1.0, t0 + (t1 - t0) * f64::from(k + 1) / f64::from(n)),
                    );
                    let (u, w) = (a - p, b - p);
                    normal.dot(u.cross(w)).atan2(u.dot(w))
                })
                .sum::<f64>()
        };
        for span in [0.1, 0.5, 0.9, -0.1, -0.5, -0.9].map(|f| f * core::f64::consts::PI) {
            for t0 in [0.0, 1.0, -2.5] {
                let t1 = t0 + span;
                let mid = t0 + span / 2.0;
                let (a, m, b) = (on(1.0, t0), on(1.0, mid), on(1.0, t1));
                let chord = Point3::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0, 0.0);
                let points = [
                    Point3::new(0.0, 0.0, 0.0),
                    on(0.5, mid),
                    on(0.97, mid),
                    on(1.03, mid),
                    on(3.0, mid),
                    on(0.5, mid + core::f64::consts::PI),
                    on(1.0, mid + core::f64::consts::PI),
                    on(1.0, t1 + span.signum() * 0.3),
                    chord,
                    Point3::new(b.x * 2.0 - a.x, b.y * 2.0 - a.y, 0.0),
                ];
                for p in points {
                    let (got, want) = (arc_turn(normal, p, (a, m, b)), polyline(p, t0, t1));
                    assert!(
                        (got - want).abs() < 1e-6,
                        "span {span}, t0 {t0}, p {p:?}: {got} against {want}"
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod reach_tests {
    //! [`reaches_root_in_band`]'s fold: a margin in band sets its root
    //! aside, whatever its sign.

    use super::*;

    /// **A root whose reach margin lies in band is set aside**, where a
    /// fold of the band toward standing would let it stand. A brick's
    /// face is restated as a unit cylinder about `z`. A ray in the plane
    /// `z = 0.5`, along `y = 0.6`, enters the cylinder at `x = −0.8`.
    /// From a passage `off` before that root, outside, the margin is
    /// about `−f''·off²/2`, so a sweep of `off` crosses the band: a
    /// near passage decides and reaches the root, and a passage whose
    /// margin lies in band reads it not reached.
    #[test]
    fn a_root_whose_reach_margin_lies_in_band_is_set_aside() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the run's band");
        let mut body =
            crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let face = body.faces().next().expect("a face").0;
        let key = body.get_face(face).expect("the face").surface;
        *body.surfaces.get_mut(key).expect("its surface") = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let (q, d) = (Point3::new(-3.0, 0.6, 0.5), Vec3::new(1.0, 0.0, 0.0));
        let root = 2.2;
        let met = |off: f64| Met {
            edge: crate::entity::EdgeKey::default(),
            faces: [face, face],
            at: root - off,
            span: (root - off, root - off),
            cross: [Cross::Root(0); 2],
            gap: Gap::Zero,
        };
        let (mut decided, mut in_band) = (0, 0);
        for i in 0..400 {
            let off = 10f64.powf(-8.0 + f64::from(i) * 0.02);
            let m = met(off);
            let margin = reach_margin(&body, face, (q, d), &m, root).expect("a carrier");
            let reached =
                reaches_root_in_band(&body, face, (q, d), &m, root, band, SCHEDULE_ROWS).ok();
            match decide(SCHEDULE_ROWS.reaches, Margin::of(margin), band) {
                Ok(Sign::Positive) => assert_eq!(reached, Some(false), "off {off:e}"),
                Ok(_) => {
                    decided += 1;
                    assert_eq!(reached, Some(true), "off {off:e}: decided, reached");
                }
                Err(diag) if crate::validate::in_band(&diag) => {
                    in_band += 1;
                    assert_eq!(
                        reached,
                        Some(false),
                        "off {off:e}: margin {margin:e} in band, set aside"
                    );
                }
                Err(diag) => panic!("off {off:e}: a margin no band reads: {diag:?}"),
            }
        }
        assert!(
            decided > 0 && in_band > 0,
            "at ε {:e}: {decided} decided, {in_band} in band",
            tol.eps()
        );
    }
}

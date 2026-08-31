//! Trilean **point-in-solid** containment (F8): the ray design of
//! profile's 2-D machinery and PR 3's [`point_in_loop`] promoted to
//! 3-D — the boolean containment fallback for operands whose
//! boundaries do not intersect (§15.9: "the ONLY place a
//! point-in-solid test is ever needed").
//!
//! # Method: closest-hit ray test with the fixed schedule
//!
//! Cast a ray from `q` along a direction of the fixed schedule — the
//! same 16-member golden-angle table as [`point_in_loop`], and
//! literally the same const (`SCHEDULE`, read from
//! `splitting::containment`), used here as space directions
//! **directly**: this module normalizes the raw triple, where
//! `point_in_loop` projects it into the loop's plane and skips the
//! near-parallel members. One table, two different sweeps — the
//! shared const buys the absence of drift between copies, not
//! agreement on a direction, and determinism is per site (a `const`
//! swept in a fixed order every run). For each face
//! (planar — the F5 regime):
//! intersect the ray with the face plane, test the hit point against
//! the face's loops (outer minus rings) via [`point_in_loop`], and
//! keep the **closest** crossing. The verdict reads the material side
//! from that crossing's outward normal: `d·n > 0` at the closest hit
//! ⇒ the ray *exits* material there ⇒ `In`; `d·n < 0` ⇒ `Out`.
//!
//! **Why closest-hit, not bare parity** (a documented deviation from
//! the plan's "ray parity" wording): parity is orientation-blind — on
//! a reverted (complement) operand it reports containment in the
//! *enclosed* region rather than in the body's actual material, and
//! the `A∖B ≡ A∩revert(B)` oracle (Problem 15.9) feeds reverted
//! operands through this very fallback. The closest-hit normal test
//! answers "is `q` in the material?" uniformly for ordinary bodies,
//! multi-shell bodies with voids, and complements; for an ordinary
//! embedded boundary it agrees with parity.
//!
//! # Grazing and the retry schedule
//!
//! Ray choices are never allowed to decide borderline geometry: a hit
//! landing ON a loop boundary (edge/vertex hit), a tangent crossing
//! (`d·n` in band), a tie between two crossings' advances, or an
//! in-band advance sign all abandon the ray and retry with the next
//! schedule member; exhaustion is the typed
//! [`PointInSolidError::RayExhausted`]. A boundary pre-pass reports
//! `q` ON the solid's boundary as [`SolidContainment::OnBoundary`]
//! before any ray is cast.
//!
//! # Predicates (all K-tagged through the Q1 funnel, meters)
//!
//! - **`bool_point_in_solid_plane`**: signed elevation of `q` off a
//!   face plane (boundary pre-pass; Zero ⇒ in-plane ⇒ loop test).
//! - **`bool_point_in_solid_denom`**: `d·n` per face (parallel-ray
//!   gate; Zero with `q` off-plane ⇒ the ray misses the plane —
//!   skipped, not grazed).
//! - **`bool_point_in_solid_advance`**: the crossing's advance `t`
//!   along the ray (Zero ⇒ crossing at `q` — graze, retry).
//! - **`bool_point_in_solid_order`**: `t − t_best` (closest-hit
//!   selection; Zero ⇒ tie ⇒ graze, retry). The winning crossing's
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
//! - **`bool_ray_cone_apex`**: the hit's distance from the apex. Zero ⇒
//!   the hit IS the apex, where the SURFACE (not merely the chart) is
//!   singular and no outward normal exists — graze, retry, decided
//!   before any direction is read off the surface.
//! - **`bool_ray_cone_nappe`**: `(p − apex)·axis` signed by the face's
//!   own nappe — Negative is the mirror nappe, which this face does not
//!   bound.
//! - **`bool_cone_trim`** / **`bool_cone_trim_nappe`** /
//!   **`bool_cone_trim_side`**: the cone face's azimuth+slant window,
//!   and the two premises that make the window single-nappe.
//! - **`bool_ray_cone_incidence`**: `d·n̂` at a cone hit, levered by the
//!   local radius — the cone's analogue of the plane arm's `denom`.
//! - **`bool_point_in_solid_infinity`**: the signed volume (scaled to
//!   a mean-thickness margin in meters) — consulted only when a ray
//!   crosses NO boundary at all: `q` then sits on the at-infinity
//!   side, which is `Out` for a positively-oriented body and `In` for
//!   a complement (negative signed volume — PR 1's revert posture).
//!   Without this, a no-hit ray on a reverted operand would misreport
//!   complement material as `Out`.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use crate::body::Body;
use crate::entity::{FaceKey, LoopBoundary};
use crate::splitting::containment::{LoopContainment, PointInLoopError, SCHEDULE, point_in_loop};
use crate::validate::decide;
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

/// Typed failure of [`point_in_solid`].
#[derive(Debug)]
pub enum PointInSolidError {
    /// A predicate escalated (in-band margin).
    Escalated {
        /// The face being tested.
        face: FaceKey,
        /// The escalation diagnostics (named predicate inside).
        diag: Indeterminate,
    },
    /// Every schedule ray grazed — the query is ill-conditioned at
    /// this ε.
    RayExhausted,
    /// The at-infinity orientation probe found a (near-)zero signed
    /// volume — the body bounds no material to be inside of.
    ZeroVolumeBody,
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
    /// count per kind: `Plane`, `Cylinder` and `Cone` (each with its
    /// exact chart trim) and `Sphere` (a closed group, or a face its
    /// chart rectangle expresses) have one; `Torus` and `Nurbs` do
    /// not.
    ///
    /// **This door is BOX-BLIND on purpose and that is why the kind
    /// still matters here.** The operand gate is pair-scoped — a face
    /// whose box clears the other operand cannot enter a crossing, so
    /// it does not gate the operation — but containment asks a
    /// different question: a ray from the query point crosses the
    /// WHOLE boundary, and a face out of reach of the cut is not out
    /// of reach of the ray. So an operation the gate admits can still
    /// meet this refusal downstream, and that boundary is the honest
    /// one until the kind has a containment arm. The cone half of
    /// issue 1011 has landed — the ray×cone quadratic and its chart
    /// trim — and what this variant now carries is the ray×torus
    /// quartic, its two windows, and NURBS.
    KindUnsupported {
        /// The face.
        face: FaceKey,
        /// Its kind — the row the arm is missing for.
        kind: geom_brep::SurfaceKind,
    },
    /// The at-infinity orientation probe needs the body's signed
    /// volume and the closed-form props lane refused to certify it.
    ///
    /// Consulted only when a schedule ray crosses NO boundary at all —
    /// `q` then sits on the at-infinity side, and which side that IS
    /// depends on the body's orientation. A body whose volume the props
    /// lane cannot certify (a rimless sphere band whose meridians lie
    /// on two DIFFERENT great circles is the standing one: that arm
    /// hardcodes `Δu = π`) leaves that question unanswerable, and this
    /// says so rather than reporting a HEALTHY body as broken.
    VolumeUncertified,
    /// A `Sphere` face that is neither closed on its own surface nor
    /// expressible as a chart RECTANGLE.
    ///
    /// Two classes answer: a face group closed against itself covers
    /// the whole chart, so membership on the surface is membership in
    /// the group; and a trimmed face whose every boundary edge is a
    /// latitude rim or a meridian great circle is exactly the rectangle
    /// `[azimuth] × [latitude]` its boundary pins, which
    /// [`sphere_chart_trim`] reads. What reaches here is the
    /// remainder — a ringed face, a boundary edge that is neither chart
    /// class, an azimuth walk that wraps past a period, or a meridian
    /// edge with a POLE strictly inside it, where latitude stops being
    /// monotone along the edge and no fold over boundary levels can see
    /// the face's own extreme.
    PartialSphereFace {
        /// The sphere face the chart rectangle cannot express.
        face: FaceKey,
    },
    /// A `Cone` face in neither cone class: its surface group does not
    /// wrap the azimuth, and its own azimuth window is not definitely
    /// narrower than a period.
    ///
    /// The cone chart's apex is a junction no azimuth walk crosses —
    /// every azimuth maps to the tip — so a face bounded by two
    /// meridians that MEET there has no closed-form window: the walk
    /// continues in the direction it was going and reports the whole
    /// period. Two classes answer around that. A face group with no
    /// azimuth boundary of its own covers every azimuth of its slant
    /// window, so the window alone trims it exactly; and a face whose
    /// azimuth window IS definitely narrower than a period is the case
    /// the walk gets right, trimmed per face. What reaches here is the
    /// remainder — a ringed cone face, a group whose members disagree
    /// on their slant window (two bands stacked on one cone, with
    /// another surface's face between them), or a wrapped window on a
    /// face whose group does not wrap.
    PartialConeFace {
        /// The cone face neither class expresses.
        face: FaceKey,
    },
}

impl From<PointInLoopError> for PointInSolidError {
    fn from(e: PointInLoopError) -> Self {
        Self::Loop(e)
    }
}

impl core::fmt::Display for PointInSolidError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Escalated { face, diag } => {
                write!(f, "point_in_solid: escalated at face {face:?}: {diag}")
            }
            Self::RayExhausted => write!(
                f,
                "point_in_solid: every schedule ray grazed — ill-conditioned query at this \
                 tolerance"
            ),
            Self::Loop(e) => write!(f, "point_in_solid: {e}"),
            Self::ZeroVolumeBody => write!(
                f,
                "point_in_solid: the body's signed volume is (near-)zero — no material side \
                 at infinity to classify against"
            ),
            Self::CorruptFace { face } => {
                write!(
                    f,
                    "point_in_solid: face {face:?} is not walkable, or an entity it \
                     names is lost — this is an arena claim about a BROKEN body, not \
                     about a surface kind"
                )
            }
            Self::KindUnsupported { face, kind } => {
                write!(
                    f,
                    "point_in_solid: face {face:?} is a {} and the containment door \
                     has no ray-crossing arm for the kind. The body is HEALTHY; what \
                     is missing is a capability. This door is deliberately box-blind \
                     — a ray from the query point crosses the whole boundary, so a \
                     face out of reach of the CUT is still in reach of the RAY, which \
                     is why the pair-scoped operand gate admitting the operation does \
                     not settle this. Recourse: express the operation so no torus or \
                     NURBS face bounds the solid being classified against, or wait on \
                     the containment arm for the kind",
                    kind.name()
                )
            }
            Self::VolumeUncertified => write!(
                f,
                "point_in_solid: a schedule ray crossed no boundary at all, so the verdict is \
                 the AT-INFINITY side — and that side is read off the body's signed volume, \
                 which the closed-form props lane refused to certify. The body is HEALTHY and \
                 this door's own arms answered; what is missing is a volume. The standing case \
                 is a rimless sphere band whose two meridian boundaries lie on DIFFERENT great \
                 circles (a lune narrower or wider than a hemisphere): that props arm hardcodes \
                 the azimuthal width at π. Recourse: pose the query where a ray meets the \
                 boundary, or wait on the props arm that reads the width from the boundary"
            ),
            Self::PartialSphereFace { face } => {
                write!(
                    f,
                    "point_in_solid: sphere face {face:?} is neither closed on its own \
                     surface nor expressible as a chart rectangle. A trimmed sphere face \
                     IS served when every boundary edge is a latitude rim or a meridian \
                     great circle — that face is exactly the [azimuth] × [latitude] window \
                     its boundary pins. This one is not: it carries a ring, a boundary edge \
                     in neither chart class, an azimuth walk wrapping past a period, or a \
                     meridian edge with a POLE strictly inside it. That last one breaks the \
                     rectangle in both coordinates: a meridian's chart image is a \
                     constant-azimuth iso-line and an arc through a pole is not one (its \
                     azimuth jumps by π there, and the loop walk carries a pole junction \
                     only at a VERTEX), while its latitude extreme is interior to the edge, \
                     where a fold over boundary levels never looks. Recourse: bound the \
                     sphere face with rims and meridians meeting AT the poles, keep it \
                     whole, or trim it with the cylinder/plane arms"
                )
            }
            Self::PartialConeFace { face } => {
                write!(
                    f,
                    "point_in_solid: cone face {face:?} is in neither cone class. The body \
                     is HEALTHY and the containment door HAS a ray-crossing arm for the \
                     kind; what it cannot pin is this face's azimuth. A cone chart's apex \
                     is a junction no azimuth walk crosses — every azimuth maps to the tip \
                     — so a face whose two bounding meridians MEET there has no closed-form \
                     window, and the walk reports a whole period for a face that covers part \
                     of one. Two classes answer around that: a face group with no azimuth \
                     boundary of its own (the full revolve's two bands, which together cover \
                     every azimuth of their shared slant window), and a face whose window is \
                     definitely NARROWER than a period. This one is neither: it carries a \
                     ring, its group's members disagree on their slant window, or its window \
                     wrapped without its group wrapping. Recourse: bound the cone face so its \
                     azimuth window closes short of a period, or let its group cover the \
                     chart"
                )
            }
        }
    }
}

impl std::error::Error for PointInSolidError {}

/// The face's plane, F5-gated: origin and **outward** normal (chart
/// normal times the face's `sense_sign`, S10 — the callers of this
/// door are handed a material direction, not a chart datum).
///
/// Its one external consumer feeds the normal to [`point_in_face`],
/// whose answer is ray-crossing parity and therefore blind to the
/// normal's sign either way; threading here is what keeps the door's
/// CONTRACT honest for the next consumer.
pub(super) fn face_plane<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<(Point3<T>, Vec3<T>), PointInSolidError> {
    let f = body
        .get_face(face)
        .ok_or(PointInSolidError::CorruptFace { face })?;
    match body.get_surface(f.surface) {
        Some(Surface::Plane { origin, normal, .. }) => Ok((*origin, *normal * f.sense_sign::<T>())),
        // A resolved non-plane is a CAPABILITY answer; only a surface
        // key that does not resolve is corruption.
        Some(s) => Err(PointInSolidError::KindUnsupported {
            face,
            kind: geom_brep::SurfaceKind::of(s),
        }),
        None => Err(PointInSolidError::CorruptFace { face }),
    }
}

/// A face's resolved query geometry (M5 PR 9): the planar datum, or a
/// cylinder wall with its exact chart trim — azimuth window from the
/// outer cycle's closed-form chart images (the S9 machinery), height
/// range from the boundary vertices (exact for the iso-bounded wall
/// class every M5 operand mints: rims are height iso-lines, meridians
/// azimuth iso-lines, and the boolean's own seam arcs are height
/// iso-lines too).
///
/// **Orientation (S10)**: every arm's outward direction is the chart's
/// times the face's `sense_sign`. The plane arm carries it in the
/// normal itself (there is a vector to multiply); the curved arms have
/// no stored normal — their outward direction is recomputed at each
/// ray hit — so they carry the face's `sense` bit and the doors apply
/// it to the sign they derive. Only the material-side signs need it:
/// the boundary pre-pass compares residuals against Zero and the
/// chart trims are parameter-domain work, both orientation-free.
enum FaceGeo<T: geom_core::Real> {
    /// A planar face: a point on it and its OUTWARD normal.
    Plane(Point3<T>, Vec3<T>),
    Cylinder {
        origin: Point3<T>,
        axis: Vec3<T>,
        radius: T,
        u_ref: Vec3<T>,
        az: (T, T),
        h: (T, T),
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
        /// The group's representative — the lowest face key in
        /// face-arena order carrying this surface. Arms no-op on every
        /// other member.
        representative: FaceKey,
        /// The REPRESENTATIVE's orientation sense (S10). The group is
        /// closed and bounds one material region, so its members share
        /// a sense; asking the representative is asking the group, and
        /// it is the representative that the arms act for. (Agreement
        /// across the group is a tier-3 obligation, not re-checked
        /// here.) `false` swaps the near/far crossing pair below.
        sense: bool,
    },
    /// A cone wall face with its exact chart trim — the cylinder arm's
    /// shape on the cone chart: an azimuth window from the outer
    /// cycle's closed-form chart images, and a SLANT window `v` in
    /// metres along the generator (`v = (p − apex)·axis / cos α`, the
    /// chart's own second coordinate, exact for a point on the cone).
    ///
    /// **The slant window carries the nappe.** `v` is signed and zero
    /// exactly at the apex, so a face's window pins which nappe it lies
    /// on; `nappe` is that side, read once at resolution time. A window
    /// that STRADDLES the apex describes no manifold patch — the two
    /// halves meet only at a point where no tangent plane exists — and
    /// is refused there rather than admitted here.
    Cone {
        /// The apex (`v = 0`), where the surface has no tangent plane.
        apex: Point3<T>,
        /// The unit axis; the `v > 0` nappe opens along it.
        axis: Vec3<T>,
        /// The half-angle α ∈ (0, π/2) between axis and generators.
        half_angle: T,
        /// The chart's seam direction.
        u_ref: Vec3<T>,
        /// The face's azimuth window, or `None` when the face's cone
        /// surface group WRAPS the azimuth and the slant window alone
        /// is the exact trim (see [`cone_chart_trim`]).
        az: Option<(T, T)>,
        /// The face the arms act for: the wrapped group's lowest face
        /// key in arena order, or the face itself when the azimuth
        /// trims it. Arms no-op on every other member, so one cone
        /// contributes one crossing per root.
        representative: FaceKey,
        /// The slant window, metres along the generator — the
        /// representative's, which the whole group shares.
        v: (T, T),
        /// Which nappe the face lies on: `true` is the `v > 0` one,
        /// which opens along `+axis`. It fixes the chart normal's sign
        /// (`radial·cos α − axis·sin α` for `v > 0`, its negation for
        /// `v < 0`) and the sign of the boundary residual. Exact
        /// structure once resolved, like [`FaceGeo::Sphere`]'s
        /// representative.
        nappe: bool,
        /// The face's orientation sense (S10), applied to the sign
        /// derived from the chart normal.
        sense: bool,
    },
    /// A TRIMMED sphere face whose chart rectangle expresses it: the
    /// same ray/sphere quadratic as [`FaceGeo::Sphere`], with each root
    /// tested against the face's own `[azimuth] × [latitude]` window —
    /// the cylinder arm's shape, on the sphere chart. Unlike the closed
    /// group this arm IS per face: a trimmed face carries its own trim,
    /// so two faces of one sphere surface contribute different
    /// crossings and no representative stands in for the rest.
    SpherePatch {
        /// The sphere's centre.
        center: Point3<T>,
        /// Its radius (positive by convention).
        radius: T,
        /// The chart's polar axis.
        axis: Vec3<T>,
        /// The chart's seam direction.
        u_ref: Vec3<T>,
        /// The face's exact chart rectangle.
        trim: SphereChartTrim<T>,
        /// The face's orientation sense (S10) — `false` swaps the
        /// near/far crossing pair, as for the closed group.
        sense: bool,
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
/// The scan is exact-`f64` structure selection (C6): it reads arena
/// keys and mate adjacency only, never a margin, so it has no in-band
/// twin and does not move with ε. Rings on a sphere face make the
/// answer `None` (a ringed sphere face is a trimmed one).
pub(super) fn closed_sphere_group<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<FaceKey> {
    let surface = body.get_face(face)?.surface;
    let group: Vec<FaceKey> = body
        .faces()
        .filter(|(_, f)| f.surface == surface)
        .map(|(k, _)| k)
        .collect();
    for &member in &group {
        let f = body.get_face(member)?;
        if !f.rings.is_empty() {
            return None;
        }
        let LoopBoundary::Cycle { first } = body.get_loop(f.outer)?.boundary else {
            return None;
        };
        for he in body.loop_cycle(first)? {
            let mate = body.mate(he)?;
            let neighbour = body.get_loop(body.get_half_edge(mate)?.parent_loop)?.face;
            if !group.contains(&neighbour) {
                return None;
            }
        }
    }
    group.first().copied()
}

/// Resolves [`FaceGeo`]; kinds outside {Plane, Cylinder, Cone, Sphere}
/// refuse as [`PointInSolidError::KindUnsupported`] (per-arm, C12.1 —
/// torus walls have no ray-crossing arm), and a TRIMMED sphere face
/// the sphere chart cannot express as
/// [`PointInSolidError::PartialSphereFace`]. Only a surface key that
/// does not RESOLVE is corruption.
fn face_geo<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<FaceGeo<T>, PointInSolidError> {
    let f = body
        .get_face(face)
        .ok_or(PointInSolidError::CorruptFace { face })?;
    match body.get_surface(f.surface) {
        Some(Surface::Plane { origin, normal, .. }) => {
            Ok(FaceGeo::Plane(*origin, *normal * f.sense_sign::<T>()))
        }
        Some(&Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        }) => {
            let (az, h) = cylinder_chart_trim(body, face, origin, axis, band)?;
            Ok(FaceGeo::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
                az,
                h,
                sense: f.sense,
            })
        }
        Some(&Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        }) => {
            let (az, representative, v, nappe) =
                cone_chart_trim(body, face, apex, axis, half_angle, band)?;
            Ok(FaceGeo::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
                az,
                representative,
                v,
                nappe,
                sense: body
                    .get_face(representative)
                    .ok_or(PointInSolidError::CorruptFace { face })?
                    .sense,
            })
        }
        Some(&Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        }) => match closed_sphere_group(body, face) {
            Some(representative) => Ok(FaceGeo::Sphere {
                center,
                radius,
                representative,
                sense: body
                    .get_face(representative)
                    .ok_or(PointInSolidError::CorruptFace { face })?
                    .sense,
            }),
            // A TRIMMED sphere face is served through its own chart
            // rectangle when the chart can express it, exactly as a
            // trimmed cylinder wall is. Only a face outside that class
            // keeps the refusal.
            None => match sphere_chart_trim(body, face, center, radius, axis, band)? {
                Some(trim) => Ok(FaceGeo::SpherePatch {
                    center,
                    radius,
                    axis,
                    u_ref,
                    trim,
                    sense: f.sense,
                }),
                None => Err(PointInSolidError::PartialSphereFace { face }),
            },
        },
        Some(s) => Err(PointInSolidError::KindUnsupported {
            face,
            kind: geom_brep::SurfaceKind::of(s),
        }),
        None => Err(PointInSolidError::CorruptFace { face }),
    }
}

/// **A cylinder wall face's exact chart trim**: the azimuth window
/// from the outer cycle's closed-form chart images (the S9 machinery)
/// and the height range from its boundary vertices.
///
/// The height fold takes NO infinity seed: an `Interval` scalar's ±∞
/// singleton is `Ill` and poisons every min/max through it (the
/// MAJOR-1 root cause — the whole Interval boolean lane died on a NaN
/// height range).
///
/// **The premise, stated once for both readers**: the range is the
/// face's own only for the ISO-BOUNDED wall class — rims are height
/// iso-lines and meridians azimuth iso-lines, so the extremes of both
/// chart coordinates lie on the boundary VERTICES. A wall bounded by a
/// tilted section takes its height extreme in an edge's interior, and
/// this rectangle then under-covers it. The face-level containment door
/// ([`super::contain::curved_face_containment`]) checks the class
/// before it reads this; the ray lane premises it from construction.
///
/// # Errors
///
/// [`PointInSolidError::CorruptFace`] for a face whose window or
/// boundary cannot be resolved.
#[allow(clippy::type_complexity)] // (azimuth window, height range) — one chart trim
pub(super) fn cylinder_chart_trim<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    origin: Point3<T>,
    axis: Vec3<T>,
    band: Band,
) -> Result<((T, T), (T, T)), PointInSolidError> {
    let f = body
        .get_face(face)
        .ok_or(PointInSolidError::CorruptFace { face })?;
    let surf = body
        .get_surface(f.surface)
        .cloned()
        .ok_or(PointInSolidError::CorruptFace { face })?;
    let az = crate::chord_join::face_azimuth_window(body, &surf, face, band)
        .ok()
        .flatten()
        .ok_or(PointInSolidError::CorruptFace { face })?;
    let mut h_range: Option<(T, T)> = None;
    let LoopBoundary::Cycle { first } = body
        .get_loop(f.outer)
        .ok_or(PointInSolidError::CorruptFace { face })?
        .boundary
    else {
        return Err(PointInSolidError::CorruptFace { face });
    };
    for he in body
        .loop_cycle(first)
        .ok_or(PointInSolidError::CorruptFace { face })?
    {
        let v = body
            .get_half_edge(he)
            .ok_or(PointInSolidError::CorruptFace { face })?
            .start;
        let p = *body
            .get_vertex(v)
            .and_then(|v| body.get_point(v.point))
            .ok_or(PointInSolidError::CorruptFace { face })?;
        let h = (p - origin).dot(axis);
        h_range = Some(match h_range {
            None => (h, h),
            Some((lo, hi)) => (lo.min(h), hi.max(h)),
        });
    }
    let h = h_range.ok_or(PointInSolidError::CorruptFace { face })?;
    Ok((az, h))
}

/// **A cone wall face's chart trim, and which of the two cone classes
/// it belongs to.** Answers `(azimuth window, representative, slant
/// window, nappe)`.
///
/// The slant coordinate is the cone chart's own `v` — metres along the
/// generator, `v = (p − apex)·axis / cos α` — so the slant window is
/// the cylinder height range's analogue and shares its premise: the
/// range is the face's own for the ISO-BOUNDED wall class, where rims
/// are slant iso-lines and meridians azimuth iso-lines and both
/// extremes therefore sit on boundary VERTICES. The fold takes no
/// infinity seed, for the reason [`cylinder_chart_trim`] states.
///
/// # The two classes, and why a cone needs both
///
/// A cone's apex is a junction the azimuth walk cannot cross. The
/// closed-form window ([`crate::chord_join::face_azimuth_window`])
/// pins each boundary edge's branch by nearest-branch continuity, and
/// at an apex-closed face's TIP the two bounding meridians meet at a
/// point every azimuth maps to — so the walk continues in the
/// direction it was going and reports a FULL PERIOD for a face that
/// covers half of one. That is the same singular junction the sphere
/// chart has at its poles, which is why the sphere arm carries a
/// closed-GROUP class beside its per-face rectangle. The cone carries
/// the same pair:
///
/// - **the azimuth-WRAPPED group** ([`wrapped_cone_group`]): the faces
///   on this cone surface have no azimuth boundary between them and
///   the rest of the body, so their union covers every azimuth of the
///   slant window and there is nothing for an azimuth trim to do. The
///   answer is `None` for the window and the group's REPRESENTATIVE,
///   for which alone the arms act — a per-face arm here would fold one
///   root once per member and tie itself into a permanent graze, the
///   defect [`FaceGeo::Sphere`] documents.
/// - **the azimuth-TRIMMED face**: a face whose own window is
///   definitely narrower than a period, which is exactly the case the
///   walk gets right. It is served per face, like
///   [`FaceGeo::SpherePatch`], and is its own representative.
///
/// A face in neither class is [`PointInSolidError::PartialConeFace`]:
/// the honest remainder, never a window that misstates the face.
///
/// # The nappe, decided here rather than at every hit
///
/// `v` is signed, and zero exactly at the apex, so its sign IS the
/// nappe. Two predicates settle the face's posture once:
///
/// - `bool_cone_trim_nappe` on `max(v_lo, −v_hi)`: **Positive** ⇒ the
///   window is clear of the apex on one nappe; **Zero** ⇒ a bound sits
///   AT the apex — the apex-closed cone face a full revolve of a
///   pole-touching profile mints, which this arm serves; **Negative** ⇒
///   the window straddles the apex. A straddling window is not a
///   trimmable patch: the two nappes meet only at the singular point,
///   the outward normal is opposite across it, and a rectangle spanning
///   both would count a mirror-nappe crossing as this face's. That is a
///   premise VIOLATION, so it escalates `Invalid` — the shape
///   [`point_on_wall_in_face`]'s period guard uses for the same reason.
/// - `bool_cone_trim_side` on `v_lo + v_hi`: the nappe itself.
///   **Zero** ⇒ a window that is a single point at the apex, which
///   bounds no surface, and escalates the same way.
///
/// # Errors
///
/// [`PointInSolidError::CorruptFace`] for a face whose window or
/// boundary cannot be resolved; [`PointInSolidError::PartialConeFace`]
/// for a face in neither class; [`PointInSolidError::Escalated`] for an
/// in-band nappe posture or a window the premises above reject.
#[allow(clippy::type_complexity)] // one chart trim, plus the class it selected
pub(super) fn cone_chart_trim<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    apex: Point3<T>,
    axis: Vec3<T>,
    half_angle: T,
    band: Band,
) -> Result<(Option<(T, T)>, FaceKey, (T, T), bool), PointInSolidError> {
    let cos_a = half_angle.cos();
    if let Some(representative) = wrapped_cone_group(body, face, apex, axis, cos_a, band)? {
        let v = cone_slant_window(body, representative, apex, axis, cos_a)?;
        let nappe = cone_nappe(face, v, band)?;
        return Ok((None, representative, v, nappe));
    }
    let v = cone_slant_window(body, face, apex, axis, cos_a)?;
    let nappe = cone_nappe(face, v, band)?;
    let f = body
        .get_face(face)
        .ok_or(PointInSolidError::CorruptFace { face })?;
    let surf = body
        .get_surface(f.surface)
        .cloned()
        .ok_or(PointInSolidError::CorruptFace { face })?;
    let az = crate::chord_join::face_azimuth_window(body, &surf, face, band)
        .ok()
        .flatten()
        .ok_or(PointInSolidError::CorruptFace { face })?;
    // The trimmed class is exactly the window the walk gets right. A
    // window a period wide or wider is the apex junction's wrap, not a
    // face that covers the chart — and the refusal says so rather than
    // trimming by an angle that means nothing.
    match decide(
        "bool_cone_trim_period",
        Margin::levered(T::tau() - (az.1 - az.0), v.0.abs().max(v.1.abs())),
        band,
    )
    .map_err(|diag| PointInSolidError::Escalated { face, diag })?
    {
        Sign::Positive => Ok((Some(az), face, v, nappe)),
        Sign::Zero | Sign::Negative => Err(PointInSolidError::PartialConeFace { face }),
    }
}

/// The face's slant window, folded over its outer cycle's vertices.
///
/// # Errors
///
/// [`PointInSolidError::CorruptFace`] — an unwalkable face.
fn cone_slant_window<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    apex: Point3<T>,
    axis: Vec3<T>,
    cos_a: T,
) -> Result<(T, T), PointInSolidError> {
    let f = body
        .get_face(face)
        .ok_or(PointInSolidError::CorruptFace { face })?;
    let LoopBoundary::Cycle { first } = body
        .get_loop(f.outer)
        .ok_or(PointInSolidError::CorruptFace { face })?
        .boundary
    else {
        return Err(PointInSolidError::CorruptFace { face });
    };
    let mut range: Option<(T, T)> = None;
    for he in body
        .loop_cycle(first)
        .ok_or(PointInSolidError::CorruptFace { face })?
    {
        let vtx = body
            .get_half_edge(he)
            .ok_or(PointInSolidError::CorruptFace { face })?
            .start;
        let p = *body
            .get_vertex(vtx)
            .and_then(|v| body.get_point(v.point))
            .ok_or(PointInSolidError::CorruptFace { face })?;
        // The chart's slant coordinate. `cos α > 0` on the whole
        // conventional domain (0, π/2), so the quotient is a length
        // with no small denominator of its own.
        let v = (p - apex).dot(axis) / cos_a;
        range = Some(match range {
            None => (v, v),
            Some((lo, hi)) => (lo.min(v), hi.max(v)),
        });
    }
    range.ok_or(PointInSolidError::CorruptFace { face })
}

/// Which nappe a slant window lies on, and the premise that it lies on
/// one at all (the two predicates [`cone_chart_trim`] documents).
///
/// # Errors
///
/// [`PointInSolidError::Escalated`] — in-band, or a window the premises
/// reject.
fn cone_nappe<T: Decide>(face: FaceKey, v: (T, T), band: Band) -> Result<bool, PointInSolidError> {
    let escalate = |diag| PointInSolidError::Escalated { face, diag };
    let invalid = |predicate| {
        escalate(geom_core::Indeterminate {
            margin: geom_core::MarginDiag::Invalid,
            band,
            predicate: Some(predicate),
        })
    };
    match decide(
        "bool_cone_trim_nappe",
        Margin::of(v.0.max(T::zero() - v.1)),
        band,
    )
    .map_err(escalate)?
    {
        // Clear of the apex, or touching it — both single-nappe.
        Sign::Positive | Sign::Zero => {}
        Sign::Negative => return Err(invalid("bool_cone_trim_nappe")),
    }
    match decide("bool_cone_trim_side", Margin::of(v.0 + v.1), band).map_err(escalate)? {
        Sign::Positive => Ok(true),
        Sign::Negative => Ok(false),
        Sign::Zero => Err(invalid("bool_cone_trim_side")),
    }
}

/// The representative of `face`'s cone-surface face group when that
/// group WRAPS THE FULL AZIMUTH — when no member has a boundary edge
/// that could trim the azimuth at all, so the union of the group covers
/// every azimuth of its slant window and the slant window alone is the
/// exact trim.
///
/// # The rule, and why it is structure rather than a margin
///
/// Only an edge that CROSSES the slant coordinate can bound a face in
/// azimuth; a slant ISO-line — a rim — bounds it in `v`, which the
/// slant window already carries. On a cone the two are told apart by
/// the carrier's VARIANT: a circle on a cone is centred on the axis
/// (an oblique plane cuts an ellipse, not a circle), so it is a rim,
/// and every other carrier crosses `v`. So the rule is: **every
/// boundary edge that is not a circle must be shared with another
/// member of the group.** That is [`closed_sphere_group`]'s scan with
/// rims exempted — arena keys, mate adjacency and curve variants only,
/// never a margin, so it has no in-band twin and does not move with ε.
///
/// The members must also agree on their slant window, decided against
/// the band. A group whose members' windows DIFFER is two bands stacked
/// along one cone, and the fold over both would cover whatever lies
/// between them — which is another surface's face, not this group's. A
/// disagreeing group answers `None` and takes the typed refusal.
///
/// Rings on a cone face make the answer `None` (a ringed face is a
/// trimmed one, and a ring is an azimuth boundary the scan cannot see).
///
/// # Errors
///
/// [`PointInSolidError`] — a corrupt member, or an in-band slant-window
/// comparison between members.
fn wrapped_cone_group<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    apex: Point3<T>,
    axis: Vec3<T>,
    cos_a: T,
    band: Band,
) -> Result<Option<FaceKey>, PointInSolidError> {
    let Some(surface) = body.get_face(face).map(|f| f.surface) else {
        return Err(PointInSolidError::CorruptFace { face });
    };
    let group: Vec<FaceKey> = body
        .faces()
        .filter(|(_, f)| f.surface == surface)
        .map(|(k, _)| k)
        .collect();
    for &member in &group {
        let Some(f) = body.get_face(member) else {
            return Err(PointInSolidError::CorruptFace { face: member });
        };
        if !f.rings.is_empty() {
            return Ok(None);
        }
        let Some(LoopBoundary::Cycle { first }) = body.get_loop(f.outer).map(|l| l.boundary) else {
            return Ok(None);
        };
        let Some(cycle) = body.loop_cycle(first) else {
            return Err(PointInSolidError::CorruptFace { face: member });
        };
        for he in cycle {
            let Some(edge) = body
                .get_half_edge(he)
                .and_then(|h| body.get_edge(h.edge))
                .map(|e| e.curve)
            else {
                return Err(PointInSolidError::CorruptFace { face: member });
            };
            // A rim bounds the slant window, not the azimuth.
            if let Some(crate::null::CurveGeom::Certified(c)) = body.get_curve_geom(edge)
                && matches!(c.carrier(), geom::Curve3::Circle { .. })
            {
                continue;
            }
            let Some(neighbour) = body
                .mate(he)
                .and_then(|m| body.get_half_edge(m))
                .and_then(|h| body.get_loop(h.parent_loop))
                .map(|l| l.face)
            else {
                return Err(PointInSolidError::CorruptFace { face: member });
            };
            if !group.contains(&neighbour) {
                return Ok(None);
            }
        }
    }
    let Some(&representative) = group.first() else {
        return Err(PointInSolidError::CorruptFace { face });
    };
    let reference = cone_slant_window(body, representative, apex, axis, cos_a)?;
    for &member in &group {
        let w = cone_slant_window(body, member, apex, axis, cos_a)?;
        for margin in [w.0 - reference.0, w.1 - reference.1] {
            if decide("bool_cone_group_slant", Margin::of(margin), band)
                .map_err(|diag| PointInSolidError::Escalated { face: member, diag })?
                != Sign::Zero
            {
                return Ok(None);
            }
        }
    }
    Ok(Some(representative))
}

/// Is the ON-WALL point `p` within the wall face's chart trim?
/// `Some(true/false)` definite, `None` a boundary graze.
///
/// The angular test is **branch-cut-free** (M5 PR 9 fix pass,
/// MAJOR-1): the azimuth-window membership `|θ − mid| ≤ w/2` is
/// decided as the exact cosine comparison `r̂·m̂ ≥ cos(w/2)` (cosine is
/// monotone on [0, π], so the equivalence is exact for every window
/// narrower than a period — guarded). No `atan2`, no periodic
/// reduction: under the Interval scalar an `atan2` enclosure near the
/// chart seam is honest poison, and the pre-fix trim escalated
/// `Invalid` on probe points every f64 run decides cleanly — the
/// whole Interval boolean lane died on it. The cone margin is metered
/// `· radius` (its displacement scale at the window edge is
/// `sin(w/2)·δθ·r ≤ δθ·r` — conservative relative to the arc-length
/// convention, escalating MORE near degenerate windows, never less).
/// Height margins are metres directly, unchanged.
///
/// **THE cosine-window construction, and its sites.** This argument —
/// the guard that the window is narrower than a period, the
/// `r̂·m̂ ≥ cos(w/2)` comparison, the lever metering, and ledger row
/// F8's deferred narrow-window fix — is one construction, so a change
/// to any of it is a change to all of them. The ray lane's two windowed
/// arms (this one and [`point_on_cone_in_face`]) share the one body,
/// [`chart_azimuth_margin`], and cannot drift; the two boundary-walk
/// sites restate it — [`super::contain::point_on_arc`] (a rim ARC's own
/// angular span) and [`super::contain::curved_face_containment`] (the
/// same period guard asked as a chart-form question, which is why its
/// answer is `None` where this one escalates).
#[allow(clippy::too_many_arguments)] // one internal lane, each a named datum
pub(super) fn point_on_wall_in_face<T: Decide>(
    face: FaceKey,
    origin: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
    az: (T, T),
    h: (T, T),
    p: Point3<T>,
    band: Band,
) -> Result<Option<bool>, PointInSolidError> {
    let escalate = |diag| PointInSolidError::Escalated { face, diag };
    let w = p - origin;
    let height = w.dot(axis);
    let radial = w - axis * height;
    let azimuth = chart_azimuth_margin(face, axis, u_ref, az, radial, radius, band)?;
    let mut verdict = Some(true);
    for margin in [azimuth, Margin::of(height - h.0), Margin::of(h.1 - height)] {
        match decide("bool_wall_trim", margin, band).map_err(escalate)? {
            Sign::Positive => {}
            Sign::Negative => return Ok(Some(false)),
            Sign::Zero => verdict = None, // boundary graze
        }
    }
    Ok(verdict)
}

/// The azimuth-window membership margin for an ON-CHART point whose
/// radial component off the axis is `radial`, levered by `lever` — the
/// **branch-cut-free cosine construction** [`point_on_wall_in_face`]
/// documents, in one body so the ray lane's two windowed arms cannot
/// drift apart. Positive is strictly inside the window, Zero on its
/// edge, Negative outside; the caller decides it with its own predicate
/// name alongside the rest of its trim.
///
/// `lever` is the length that converts the angular displacement to the
/// point deviation it implies (D4's θ·r form): the wall's radius for a
/// cylinder, the LOCAL radius at the hit for a cone, whose radius grows
/// with the slant. Both are the distance the window's edge moves per
/// radian at the point being tested, which is what the margin has to
/// mean.
///
/// # Errors
///
/// [`PointInSolidError::Escalated`] — an in-band period guard, or a
/// window a period wide or wider, which cannot trim by angle at all.
fn chart_azimuth_margin<T: Decide>(
    face: FaceKey,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    az: (T, T),
    radial: Vec3<T>,
    lever: T,
    band: Band,
) -> Result<Margin<T>, PointInSolidError> {
    let escalate = |diag| PointInSolidError::Escalated { face, diag };
    let (w_min, w_max) = az;
    let width = w_max - w_min;
    // The cosine equivalence needs width < τ, decided loudly (a
    // full-period window cannot trim by angle at all).
    match decide(
        "bool_wall_trim_period",
        Margin::levered(T::tau() - width, lever),
        band,
    )
    .map_err(escalate)?
    {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => {
            return Err(escalate(geom_core::Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band,
                predicate: Some("bool_wall_trim_period"),
            }));
        }
    }
    let r_hat = radial / radial.norm();
    let half = T::from_f64(0.5);
    let mid = (w_min + w_max) * half;
    let (s_m, c_m) = mid.sin_cos();
    let v_ref = axis.cross(u_ref);
    let m_hat = u_ref * c_m + v_ref * s_m;
    let (s_h, c_h) = (width * half).sin_cos();
    let _ = s_h; // the cosine comparison carries the whole test
    // Ledger row F8: the cone term's (cosΔ − cos h)·r collapses
    // quadratically for narrow windows — conservative direction, the
    // row's deferred fix; the margin is a length (levered) today.
    Ok(Margin::levered(r_hat.dot(m_hat) - c_h, lever))
}

/// Is the ON-CONE point `p` within the cone wall face's chart trim?
/// `Some(true/false)` definite, `None` a graze the ray schedule
/// retries. [`point_on_wall_in_face`]'s contract, on the cone chart.
///
/// # The apex, decided here
///
/// **The apex never yields a crossing.** The cone has no tangent plane
/// there — the singularity is the SURFACE's, not merely the chart's (a
/// sphere's poles are the other case) — so no outward normal exists and
/// the closest-hit rule has no material side to consume; a definite
/// answer would have to fabricate the tangent plane the geometry does
/// not have. `bool_ray_cone_apex` therefore answers a hit at the apex
/// with `None`: the ray abandons and the schedule retries, exactly as a
/// tangency does. Deciding it before the azimuth arm is what keeps that
/// arm honest, since the radial component vanishes at the apex and its
/// normalization would be poison.
///
/// It is decided AFTER the slant window, not before, because the apex
/// of a face the window excludes is not this face's apex to graze on —
/// see the note at the site.
///
/// Away from the apex the two nappes are told apart by
/// `bool_ray_cone_nappe`, the issue's `(p − apex)·axis` sign taken
/// against the face's own nappe: Negative is the MIRROR nappe, which is
/// not this face at all; Zero is the apex plane, which for a point on
/// the cone means the apex again, and grazes. The slant window then
/// excludes the mirror nappe a second time and independently — its
/// bounds are signed — which is deliberate: the nappe posture does not
/// rest on the trim being tight.
#[allow(clippy::too_many_arguments)] // one internal lane, each a named datum
pub(super) fn point_on_cone_in_face<T: Decide>(
    face: FaceKey,
    apex: Point3<T>,
    axis: Vec3<T>,
    half_angle: T,
    u_ref: Vec3<T>,
    az: Option<(T, T)>,
    v_win: (T, T),
    nappe: bool,
    p: Point3<T>,
    band: Band,
) -> Result<Option<bool>, PointInSolidError> {
    let escalate = |diag| PointInSolidError::Escalated { face, diag };
    let w = p - apex;
    let h = w.dot(axis);
    let v = h / half_angle.cos();
    // **The slant window is asked FIRST, and that order is
    // load-bearing.** It is the one test that still means something AT
    // the apex, where the azimuth is undefined; and a face whose window
    // is clear of the apex — every frustum — must answer a query at its
    // VIRTUAL apex `Some(false)`, not the graze below. Grazing there
    // would hand the pre-pass a point of free space, arbitrarily far
    // outside the solid, as a point ON its boundary.
    let mut verdict = Some(true);
    for margin in [Margin::of(v - v_win.0), Margin::of(v_win.1 - v)] {
        match decide("bool_cone_trim", margin, band).map_err(escalate)? {
            Sign::Positive => {}
            Sign::Negative => return Ok(Some(false)),
            Sign::Zero => verdict = None, // boundary graze
        }
    }
    if decide("bool_ray_cone_apex", Margin::norm3(w), band).map_err(escalate)? == Sign::Zero {
        // The apex of a face whose window REACHES it (an apex-closed
        // cone face): no tangent plane, so no material side to read —
        // the ray abandons and the schedule retries. The boundary
        // pre-pass reads the same `None` as ON the boundary, which the
        // apex of such a face is: it is the tip of the solid.
        return Ok(None);
    }
    let nappe_h = if nappe { h } else { T::zero() - h };
    match decide("bool_ray_cone_nappe", Margin::of(nappe_h), band).map_err(escalate)? {
        Sign::Positive => {}
        Sign::Negative => return Ok(Some(false)), // the mirror nappe
        Sign::Zero => return Ok(None),
    }
    // A WRAPPED group has no azimuth boundary to test: every azimuth of
    // the slant window is the group's, and the arms act for one member.
    let Some(az) = az else { return Ok(verdict) };
    let radial = w - axis * h;
    // The window is stated in CHART azimuth, and on the `v < 0` nappe
    // the chart's radial is the negation of the physical one (`S` puts
    // that nappe at azimuth `u + π`). The direction compared against
    // the window must therefore be the chart's, or the test reads the
    // window a half-period away from the face it belongs to.
    let chart_radial = if nappe {
        radial
    } else {
        radial * (T::zero() - T::one())
    };
    // The local radius IS the lever here: a cone's azimuth window edge
    // moves `ρ` per radian at the point being tested, and `ρ` runs from
    // zero at the apex to the face's widest rim.
    let azimuth = chart_azimuth_margin(face, axis, u_ref, az, chart_radial, radial.norm(), band)?;
    match decide("bool_cone_trim", azimuth, band).map_err(escalate)? {
        Sign::Positive => {}
        Sign::Negative => return Ok(Some(false)),
        Sign::Zero => verdict = None, // boundary graze
    }
    Ok(verdict)
}

/// **A sphere face's exact chart trim**: the azimuth window and the
/// LATITUDE window, for the face class the sphere chart rectangle can
/// actually express.
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
pub(super) fn sphere_chart_trim<T: Decide>(
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
    // in-band one escalates — the two-tolerance pair, as
    // `iso_bounded_wall` runs it for the cylinder.
    let zero = |name: &'static str, m: Margin<T>| -> Result<bool, PointInSolidError> {
        match decide(name, m, band).map_err(escalate)? {
            Sign::Zero => Ok(true),
            Sign::Positive | Sign::Negative => Ok(false),
        }
    };
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
        let certified = body
            .get_edge(he_data.edge)
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(crate::null::CurveGeom::certified);
        let Some(curve) = certified else {
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
        // metres. The same dimension convention `iso_bounded_wall`
        // states for the cylinder.
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
            match decide(
                "bool_sphere_trim_meridian_span",
                Margin::levered(T::tau() - width, radius),
                band,
            )
            .map_err(escalate)?
            {
                Sign::Positive => {}
                // A full-period meridian edge runs through BOTH poles
                // and has no angular gate to test: out of the class.
                Sign::Zero | Sign::Negative => return Ok(None),
            }
            let mid = (t0 + t1) * half;
            let (s_m, c_m) = mid.sin_cos();
            let m_hat = u_c * c_m + n_c.cross(u_c) * s_m;
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
            if !zero(
                "bool_sphere_iso_rim",
                Margin::levered(n_c.cross(axis).norm(), r_c),
            )? || !zero(
                "bool_sphere_iso_rim",
                Margin::of((w - axis * w.dot(axis)).norm()),
            )? || !zero(
                "bool_sphere_iso_rim",
                Margin::of((w.norm_squared() + r_c.powi(2)).sqrt() - radius),
            )? {
                return Ok(None);
            }
            // The rim's own exact pair — its radius IS the radial
            // component, with no square root anywhere near a pole.
            levels.push((w.dot(axis), r_c));
        }
    }
    let Some((lat_lo, lat_hi)) = latitude_extremes(&levels, radius, band).map_err(escalate)? else {
        return Ok(None);
    };
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
        // not wear one: `PartialSphereFace`'s message says the body is
        // healthy and merely outside the served class, and a key the
        // walk could not resolve, or a boundary that does not close,
        // would be wearing that sentence falsely. The arena claim keeps
        // its own door — the same distinction `bool_planar_chord_spec`
        // draws between a key that does not resolve and a key of the
        // wrong kind.
        Err(crate::chord_join::SplitJoinError::Corrupt { .. })
        | Err(crate::chord_join::SplitJoinError::UnpairedLooseEnds { .. }) => {
            return Err(PointInSolidError::CorruptFace { face });
        }
        // The honest remainder: a walk this chart cannot express.
        Err(_) => return Ok(None),
    };
    // A FULL-PERIOD azimuth window is not an ill-conditioned window
    // here, it is a face that attains every azimuth — a cap, or a
    // latitude band — and the honest membership answer for it is
    // "yes, at every azimuth". The cylinder's ray arm escalates on the
    // same reading because there a full turn means its cosine
    // comparison stopped being an equivalence and the rectangle stopped
    // describing the face; on a sphere the LATITUDE window still
    // describes it exactly, so there is a rectangle and no equivalence
    // is needed. A window WIDER than a period is a walk that wrapped
    // more than once — out of the class.
    let az = match decide(
        "bool_sphere_trim_period",
        Margin::levered(T::tau() - (raw.1 - raw.0), radius),
        band,
    )
    .map_err(escalate)?
    {
        Sign::Positive => Some(raw),
        Sign::Zero => None,
        Sign::Negative => return Ok(None),
    };
    Ok(Some(SphereChartTrim { az, lat_lo, lat_hi }))
}

/// A sphere face's chart rectangle: the azimuth window and the two
/// extreme latitudes, each as its exact `(axial, radial)` pair on the
/// meridian half-plane.
///
/// A `None` window END is a constraint the face does not have, not a
/// missing datum: a face attaining every azimuth cannot be excluded by
/// an azimuth, and a latitude window that reaches a POLE cannot be
/// excluded on that side — every latitude is at least the north pole's
/// and at most the south pole's. Carrying those as `None` rather than
/// as a margin against the pole is what keeps the margins honest:
/// `sin(v - v_pole)` degenerates to `sin v`, which is Zero at BOTH
/// poles and would call the far pole a graze.
pub(super) struct SphereChartTrim<T> {
    /// The azimuth window, or `None` for a full period.
    pub az: Option<(T, T)>,
    /// The extreme latitude nearest the `+axis` pole, or `None` when
    /// the face reaches that pole.
    pub lat_lo: Option<(T, T)>,
    /// The extreme latitude nearest the `−axis` pole, or `None` when
    /// the face reaches that pole.
    pub lat_hi: Option<(T, T)>,
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

/// Folds the boundary latitudes to the window's two extremes, and
/// resolves each end against its pole.
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
    levels: &[(T, T)],
    radius: T,
    band: Band,
) -> Result<Option<(Option<(T, T)>, Option<(T, T)>)>, Indeterminate> {
    let mut it = levels.iter().copied();
    let first = it.next().ok_or(Indeterminate {
        margin: geom_core::MarginDiag::Invalid,
        band,
        predicate: Some("bool_sphere_trim_latitude"),
    })?;
    let (mut lo, mut hi) = (first, first);
    for e in it {
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

/// Is the ON-SPHERE point `p` within the sphere face's chart trim?
/// `Some(true/false)` definite, `None` a boundary graze.
///
/// The azimuth half is THE cosine-window construction verbatim — the
/// argument (period guard, `r̂·m̂ ≥ cos(w/2)`, `· radius` metering,
/// ledger row F8) lives at [`point_on_wall_in_face`] and is shared,
/// so a change to any of it is a change to all of its sites. A face
/// with no azimuth window attains every azimuth and skips it.
///
/// The latitude half is the sine margin of [`sphere_chart_trim`]'s
/// header: `R sin(v − v_lo)` and `R sin(v_hi − v)`, both arc lengths,
/// both bounded below by the geodesic distance to the window edge even
/// when that edge is a pole.
#[allow(clippy::too_many_arguments)] // one chart datum, each argument named
pub(super) fn point_on_sphere_in_face<T: Decide>(
    face: FaceKey,
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    trim: &SphereChartTrim<T>,
    p: Point3<T>,
    band: Band,
) -> Result<Option<bool>, PointInSolidError> {
    let escalate = |diag| PointInSolidError::Escalated { face, diag };
    let half = T::from_f64(0.5);
    let w = p - center;
    let height = w.dot(axis);
    let radial = w - axis * height;
    let here = (height, radial.norm());
    let mut margins: Vec<Margin<T>> = [
        trim.lat_lo
            .map(|lo| Margin::levered(latitude_sine(lo, here, radius), radius)),
        trim.lat_hi
            .map(|hi| Margin::levered(latitude_sine(here, hi, radius), radius)),
    ]
    .into_iter()
    .flatten()
    .collect();
    if let Some((w_min, w_max)) = trim.az {
        let width = w_max - w_min;
        let mid = (w_min + w_max) * half;
        let (s_m, c_m) = mid.sin_cos();
        let v_ref = axis.cross(u_ref);
        let m_hat = u_ref * c_m + v_ref * s_m;
        let (_, c_h) = (width * half).sin_cos();
        // The azimuth direction is the radial one, and at a POLE there
        // is none: every azimuth meets there, so the window cannot
        // exclude the point and the test is skipped rather than run on
        // a direction that does not exist. The latitude margins above
        // still decide the pole, exactly.
        match decide("bool_sphere_trim_pole", Margin::of(radial.norm()), band).map_err(escalate)? {
            Sign::Positive => {
                let r_hat = radial / radial.norm();
                margins.push(Margin::levered(r_hat.dot(m_hat) - c_h, radius));
            }
            Sign::Zero | Sign::Negative => {}
        }
    }
    let mut verdict = Some(true);
    for margin in margins {
        match decide("bool_sphere_trim", margin, band).map_err(escalate)? {
            Sign::Positive => {}
            Sign::Negative => return Ok(Some(false)),
            Sign::Zero => verdict = None, // boundary graze
        }
    }
    Ok(verdict)
}

/// Is `p` (already in the face's plane) within the face's region —
/// inside the outer loop and outside every ring? `OnBoundary` from any
/// loop is reported as `None` (graze).
pub(super) fn point_in_face<T: Decide>(
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
    if matches!(
        body.get_loop(f.outer).map(|l| l.boundary),
        Some(LoopBoundary::Empty { .. }) | None
    ) {
        return Ok(Some(false));
    }
    match point_in_loop(body, f.outer, normal, p, band)? {
        LoopContainment::Out => return Ok(Some(false)),
        LoopContainment::OnBoundary => return Ok(None),
        LoopContainment::In => {}
    }
    for &ring in &f.rings {
        if matches!(
            body.get_loop(ring).map(|l| l.boundary),
            Some(LoopBoundary::Empty { .. })
        ) {
            continue; // a lone ring vertex excludes no area
        }
        match point_in_loop(body, ring, normal, p, band)? {
            LoopContainment::In => return Ok(Some(false)),
            LoopContainment::OnBoundary => return Ok(None),
            LoopContainment::Out => {}
        }
    }
    Ok(Some(true))
}

/// Trilean containment of `q` in `body`'s material (module docs).
/// Every face of every shell participates (multi-shell bodies and
/// complements answer correctly by the closest-hit rule).
///
/// # Errors
///
/// [`PointInSolidError`] — escalation, ray exhaustion, or a
/// non-planar/corrupt face.
pub fn point_in_solid<T: Decide>(
    body: &Body<T>,
    q: Point3<T>,
    band: Band,
    tol: Tol,
) -> Result<SolidContainment, PointInSolidError> {
    // Deterministic face sweep order (arena order).
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();

    // ---- Boundary pre-pass: q on any face ⇒ OnBoundary. ----
    for &face in &faces {
        let escalate = |diag| PointInSolidError::Escalated { face, diag };
        match face_geo(body, face, band)? {
            FaceGeo::Plane(origin, normal) => {
                // Orientation-free: a residual compared against Zero
                // answers the same whichever way the normal points,
                // and `point_in_face` below is ray parity (ditto).
                let elev = (q - origin).dot(normal);
                if decide("bool_point_in_solid_plane", Margin::of(elev), band).map_err(escalate)?
                    == Sign::Zero
                {
                    // In-plane: ON the boundary iff within the face
                    // region (a loop-boundary graze is also ON).
                    match point_in_face(body, face, normal, q, band)? {
                        Some(true) | None => return Ok(SolidContainment::OnBoundary),
                        Some(false) => {}
                    }
                }
            }
            FaceGeo::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
                az,
                h,
                sense: _, // residual-vs-Zero and chart trim: orientation-free
            } => {
                let w = q - origin;
                let radial = w - axis * w.dot(axis);
                // The linearized residual (metres) — the same form the
                // certification layer classifies.
                let elev = (radial.norm_squared() - radius.powi(2)) / (T::from_f64(2.0) * radius);
                if decide("bool_point_in_solid_plane", Margin::of(elev), band).map_err(escalate)?
                    == Sign::Zero
                {
                    match point_on_wall_in_face(face, origin, axis, radius, u_ref, az, h, q, band)?
                    {
                        Some(true) | None => return Ok(SolidContainment::OnBoundary),
                        Some(false) => {}
                    }
                }
            }
            // The cone wall arm: the residual is the point's EXACT
            // perpendicular distance to the face's own nappe — the
            // generator line through the same azimuth sits at
            // `ρ cos α = σ·h sin α`, so `ρ cos α − σ·h sin α` is a
            // signed length outright (positive OUTSIDE the nappe) and
            // needs no linearization, unlike the cylinder and sphere
            // residuals above. `σ` is the face's nappe, which is what
            // keeps a point on the MIRROR nappe from reading as
            // on-boundary: there the same expression is `2ρ cos α`
            // away from zero.
            FaceGeo::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
                az,
                representative,
                v,
                nappe,
                sense: _, // residual-vs-Zero and chart trim: orientation-free
            } => {
                if face != representative {
                    continue;
                }
                let w = q - apex;
                let h = w.dot(axis);
                let nappe_h = if nappe { h } else { T::zero() - h };
                let (sin_a, cos_a) = half_angle.sin_cos();
                let radial = w - axis * h;
                let elev = radial.norm() * cos_a - nappe_h * sin_a;
                if decide("bool_point_in_solid_plane", Margin::of(elev), band).map_err(escalate)?
                    == Sign::Zero
                {
                    match point_on_cone_in_face(
                        face, apex, axis, half_angle, u_ref, az, v, nappe, q, band,
                    )? {
                        Some(true) | None => return Ok(SolidContainment::OnBoundary),
                        Some(false) => {}
                    }
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
                if decide("bool_point_in_solid_plane", Margin::of(elev), band).map_err(escalate)?
                    == Sign::Zero
                {
                    return Ok(SolidContainment::OnBoundary);
                }
            }
            // The TRIMMED sphere arm: a Zero radial residual puts `q`
            // on the CARRIER, and the face's own chart rectangle then
            // says whether it is on THIS face (a trim graze counts as
            // ON, as it does for the cylinder wall).
            FaceGeo::SpherePatch {
                center,
                radius,
                axis,
                u_ref,
                ref trim,
                sense: _, // a residual against Zero: orientation-free
            } => {
                let elev =
                    ((q - center).norm_squared() - radius.powi(2)) / (T::from_f64(2.0) * radius);
                if decide("bool_point_in_solid_plane", Margin::of(elev), band).map_err(escalate)?
                    == Sign::Zero
                {
                    match point_on_sphere_in_face(face, center, radius, axis, u_ref, trim, q, band)?
                    {
                        Some(true) | None => return Ok(SolidContainment::OnBoundary),
                        Some(false) => {}
                    }
                }
            }
        }
    }

    // ---- Closest-hit ray sweep over the fixed schedule. ----
    for r in &SCHEDULE {
        let d = Vec3::new(T::from_f64(r[0]), T::from_f64(r[1]), T::from_f64(r[2])).normalize();
        if let Some(verdict) = cast_ray(body, &faces, q, d, band, tol)? {
            return Ok(verdict);
        }
        // graze: next schedule member
    }
    Err(PointInSolidError::RayExhausted)
}

/// The crossing sign implied by a CHART-outward direction on a face
/// whose orientation sense may reverse it (S10): `d·n̂_outward` has the
/// opposite sign to `d·n̂_chart` exactly when the face's `sense` is
/// `false`. Used by the curved doors, which recompute their outward
/// direction from the surface at each hit and so have no stored normal
/// to fold the sign into (the plane door gets it from [`face_geo`]).
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
/// folding to a closest hit are the RAY's concerns and stay in
/// [`cast_ray`]; a line-edge span needs both roots, unordered by sign,
/// and folding them here would have made this the ray's function with
/// a second caller rather than a shared primitive.
///
/// **The two trileans keep their names and their metering**, because
/// both are pinned: `bool_ray_cylinder_disc`'s dimensionless
/// `disc/(2r)²` is a deliberate non-normalization flagged at the sphere
/// arm below, and re-metering either one would move every acceptance
/// margin that quotes them.
///
/// # Errors
///
/// [`geom_core::Indeterminate`] — an in-band discriminant or an in-band
/// axis-parallel test. The caller wraps it in its own error type.
pub(super) fn line_wall_roots<T: Decide>(
    q: Point3<T>,
    d: Vec3<T>,
    origin: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    band: Band,
) -> Result<WallRoots<T>, geom_core::Indeterminate> {
    let w0 = q - origin;
    let w0p = w0 - axis * w0.dot(axis);
    let dp = d - axis * d.dot(axis);
    let a2 = dp.norm_squared();
    let two_r = T::from_f64(2.0) * radius;
    // Ledger row F2: sin²/2r is 1/m — flagged, not cast.
    match geom_core::k_stats::decide_flagged("bool_point_in_solid_denom", a2 / two_r, band, "F2")? {
        Sign::Positive => {}
        _ => return Ok(WallRoots::AxisParallel),
    }
    let b2 = w0p.dot(dp);
    let c2 = w0p.norm_squared() - radius.powi(2);
    let disc = b2.powi(2) - a2 * c2;
    // Metre-scaled discriminant: Positive ⇒ two definite roots; Zero ⇒
    // tangent; Negative ⇒ definite miss; in-band escalates.
    // Ledger row F2: disc/(2r)² is dimensionless (its own in-tree
    // admission, PR 9c review F3) — flagged.
    match geom_core::k_stats::decide_flagged(
        "bool_ray_cylinder_disc",
        disc / two_r.powi(2),
        band,
        "F2",
    )? {
        Sign::Positive => {}
        Sign::Zero => return Ok(WallRoots::Tangent),
        Sign::Negative => return Ok(WallRoots::Miss),
    }
    let root = disc.max(T::zero()).sqrt();
    Ok(WallRoots::Two([
        (T::zero() - b2 - root) / a2,
        (T::zero() - b2 + root) / a2,
    ]))
}

/// One ray of the sweep: `Some(verdict)` or `None` for a graze.
fn cast_ray<T: Decide>(
    body: &Body<T>,
    faces: &[FaceKey],
    q: Point3<T>,
    d: Vec3<T>,
    band: Band,
    tol: Tol,
) -> Result<Option<SolidContainment>, PointInSolidError> {
    let mut best: Option<(T, Sign)> = None; // (advance, sign of d·n)
    // A candidate crossing (advance, outward sign), or a graze.
    let fold = |best: &mut Option<(T, Sign)>,
                face: FaceKey,
                t: T,
                outward: Sign|
     -> Result<Option<()>, PointInSolidError> {
        let escalate = |diag| PointInSolidError::Escalated { face, diag };
        match decide("bool_point_in_solid_advance", Margin::of(t), band).map_err(escalate)? {
            Sign::Positive => {}
            Sign::Negative => return Ok(Some(())),
            // A genuine crossing at q contradicts the boundary
            // pre-pass — graze, retry.
            Sign::Zero => return Ok(None),
        }
        *best = match *best {
            None => Some((t, outward)),
            Some((tb, sb)) => {
                match decide("bool_point_in_solid_order", Margin::of(t - tb), band)
                    .map_err(escalate)?
                {
                    Sign::Negative => Some((t, outward)),
                    Sign::Positive => Some((tb, sb)),
                    Sign::Zero => return Ok(None), // tie: graze
                }
            }
        };
        Ok(Some(()))
    };
    for &face in faces {
        let escalate = |diag| PointInSolidError::Escalated { face, diag };
        match face_geo(body, face, band)? {
            FaceGeo::Plane(origin, normal) => {
                // `normal` is the face's OUTWARD normal (S10, folded in
                // by `face_geo`), so this sign IS the material-side
                // verdict the closest-hit rule consumes; the advance
                // `t` below is a ratio of two such dots and cannot see
                // the orientation at all.
                let denom = d.dot(normal);
                // Ledger row F2: a unit·unit cosine against the metre
                // band — dimensionless; the coordinated ray-caster
                // re-pin unit owns the fix. Flagged, not cast.
                let denom_sign = geom_core::k_stats::decide_flagged(
                    "bool_point_in_solid_denom",
                    denom,
                    band,
                    "F2",
                )
                .map_err(escalate)?;
                if denom_sign == Sign::Zero {
                    // Parallel ray: q is definitely off this plane (the
                    // pre-pass returned), so the ray misses it entirely.
                    continue;
                }
                let t = (origin - q).dot(normal) / denom;
                // In-face test FIRST: a plane hit outside the face
                // region is no crossing at all — in particular a
                // `t = 0` hit on a face plane through `q` (a
                // corner-aligned query) must be skipped, not grazed,
                // when the face itself is elsewhere.
                let p = q + d * t;
                match point_in_face(body, face, normal, p, band)? {
                    Some(false) => continue,
                    None => return Ok(None), // edge/vertex hit: graze
                    Some(true) => {}
                }
                if fold(&mut best, face, t, denom_sign)?.is_none() {
                    return Ok(None);
                }
            }
            // The cylinder wall arm (M5 PR 9): the ray meets the
            // infinite wall at the roots of a quadratic in metres
            // (the linearized residual along the ray); each definite
            // root inside the face's chart trim folds like a planar
            // hit, with the outward sign read from the radial
            // gradient at the hit. A tangent ray (discriminant in the
            // zero band) grazes and retries — never a parity guess.
            FaceGeo::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
                az,
                h,
                sense,
            } => {
                let roots = line_wall_roots(q, d, origin, axis, radius, band).map_err(escalate)?;
                let ts = match roots {
                    // Axis-parallel ray: constant residual; the pre-pass
                    // said q is off the wall, so it misses entirely.
                    WallRoots::AxisParallel | WallRoots::Miss => continue,
                    WallRoots::Tangent => return Ok(None), // tangent ray: graze
                    WallRoots::Two(ts) => ts,
                };
                for t in ts {
                    let p = q + d * t;
                    match point_on_wall_in_face(face, origin, axis, radius, u_ref, az, h, p, band)?
                    {
                        Some(false) => continue,
                        None => return Ok(None), // trim-boundary hit: graze
                        Some(true) => {}
                    }
                    // Outward sign: d · (radial gradient) at the hit —
                    // a CHART direction, so the face's sense decides
                    // whether it is the material-outward one (S10).
                    let wp = p - origin;
                    let rad = wp - axis * wp.dot(axis);
                    let outward = oriented(
                        // Ledger row F2: (unit·radial)/radius is
                        // dimensionless — flagged, not cast.
                        geom_core::k_stats::decide_flagged(
                            "bool_point_in_solid_denom",
                            d.dot(rad) / radius,
                            band,
                            "F2",
                        )
                        .map_err(escalate)?,
                        sense,
                    );
                    if outward == Sign::Zero {
                        return Ok(None); // grazing incidence at the hit
                    }
                    if fold(&mut best, face, t, outward)?.is_none() {
                        return Ok(None);
                    }
                }
            }
            // The cone wall arm (issue 1011, the cone half): the ray
            // meets the infinite DOUBLE cone at the roots of
            // `G(t) = (w·â)² − |w|²cos²α`, `w = q − apex + d·t` — the
            // implicit form whose zero set is both nappes, expanded to
            // `A t² + 2B t + C` with `A` dimensionless, `B` metres and
            // `C` m². Which nappe a root landed on, and whether it
            // landed on THIS face, are the chart trim's questions, not
            // the quadratic's.
            //
            // **The leading coefficient is a posture, not a
            // convenience.** `A = (d·â)² − cos²α` vanishes exactly when
            // the ray runs PARALLEL to a generator, where the quadratic
            // degenerates to a line: the far root has left for infinity
            // and the crossing count this arm can certify is no longer
            // two. That is not a miss and not a tangency, so it is
            // neither skipped nor answered — the ray abandons and the
            // schedule retries, and exhaustion is `RayExhausted` like
            // every other ill-conditioned direction. The margin is
            // `A` levered by the face's own slant extent (D4's θ·r
            // form: `A` is a difference of squared cosines and the
            // extent is the length over which the near-parallel ray
            // drifts off the generator).
            //
            // **The discriminant** `B² − AC` is m² and is metered by
            // that same extent, so `disc / v_ext` is a length. It is
            // the tangency margin: `−disc/A` is `G` at the ray's
            // closest approach, which factors as the product of the
            // distances to the two nappes' generators, so a zero
            // discriminant IS a ray grazing a generator — a graze,
            // retried, never a parity guess. The metering is
            // conservative wherever the near-tangency is near the face
            // (both `|A| ≤ 1` and the mirror-nappe distance at a hit on
            // the face are bounded by the extent), and its exact zero
            // is the true tangency either way.
            FaceGeo::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
                az,
                representative,
                v,
                nappe,
                sense,
            } => {
                if face != representative {
                    continue;
                }
                let (sin_a, cos_a) = half_angle.sin_cos();
                let cos2 = cos_a.powi(2);
                let w0 = q - apex;
                let da = d.dot(axis);
                let wa = w0.dot(axis);
                let a2 = da.powi(2) - cos2;
                let b2 = da * wa - w0.dot(d) * cos2;
                let c2 = wa.powi(2) - w0.norm_squared() * cos2;
                // The face's own slant extent — the lever both margins
                // below are metered by. Single-nappe by construction
                // ([`cone_chart_trim`]), so this is the far bound.
                let v_ext = v.0.abs().max(v.1.abs());
                match decide("bool_ray_cone_lead", Margin::levered(a2, v_ext), band)
                    .map_err(escalate)?
                {
                    Sign::Positive | Sign::Negative => {}
                    // Generator-parallel: a certified pair is gone.
                    Sign::Zero => return Ok(None),
                }
                let disc = b2.powi(2) - a2 * c2;
                match decide("bool_ray_cone_disc", Margin::over_lever(disc, v_ext), band)
                    .map_err(escalate)?
                {
                    Sign::Positive => {}
                    Sign::Zero => return Ok(None), // tangent ray: graze
                    Sign::Negative => continue,    // definite miss
                }
                let root = disc.max(T::zero()).sqrt();
                // Unordered: `A` may be negative, and the closest-hit
                // fold orders by advance anyway.
                for t in [(T::zero() - b2 - root) / a2, (T::zero() - b2 + root) / a2] {
                    let p = q + d * t;
                    match point_on_cone_in_face(
                        face, apex, axis, half_angle, u_ref, az, v, nappe, p, band,
                    )? {
                        Some(false) => continue,
                        None => return Ok(None), // trim boundary or apex: graze
                        Some(true) => {}
                    }
                    // Outward sign: `d` against the CHART normal
                    // `radial̂·cos α − axis·sin α` on the `v > 0` nappe
                    // and its negation on the mirror one — a unit
                    // vector, so the dot is a cosine and takes the
                    // local radius as its lever (D4's θ·r form; the
                    // trim above has already put the hit definitely
                    // clear of the apex, where that radius vanishes).
                    // The face's sense then says whether chart-outward
                    // is material-outward (S10).
                    let wp = p - apex;
                    let hp = wp.dot(axis);
                    let rad = wp - axis * hp;
                    let rho = rad.norm();
                    // `radial(u)` in the chart normal is the CHART's
                    // radial, and on the `v < 0` nappe it is the
                    // negation of the physical one — `S` places that
                    // nappe at azimuth `u + π`. Folding the nappe's
                    // sign into the whole expression would cancel
                    // twice and point the normal INTO the material;
                    // what carries it is the axial term alone:
                    // `r̂·cos α − σ·axis·sin α`.
                    let axial = if nappe { sin_a } else { T::zero() - sin_a };
                    let n_chart = rad / rho * cos_a - axis * axial;
                    let outward = oriented(
                        decide(
                            "bool_ray_cone_incidence",
                            Margin::levered(d.dot(n_chart), rho),
                            band,
                        )
                        .map_err(escalate)?,
                        sense,
                    );
                    if outward == Sign::Zero {
                        return Ok(None); // grazing incidence at the hit
                    }
                    if fold(&mut best, face, t, outward)?.is_none() {
                        return Ok(None);
                    }
                }
            }
            // The full-sphere pierce arm (M5 PR 9c). With `d` a unit
            // direction the ray/sphere system is monic in `t`:
            // `t² + 2(w·d)t + (|w|² − r²) = 0`, `w = q − c`. The
            // discriminant is metered as a LENGTH: `√disc` is the
            // half-chord in metres, so `disc` is m² and `disc / 2r` is
            // the D4 ¶1-honest margin.
            //
            // This is NOT what the cylinder arm above does: it divides
            // its own discriminant by `(2r)²`, which is dimensionless.
            // The length-dimensioned form here is the correct one.
            // Normalizing the cylinder arm to match is deliberately NOT
            // done in passing — its margins are pinned by the PR 9
            // acceptance rows and a metering change moves every one of
            // them, so it is flagged for a unit that can re-pin them
            // (PR 9c review, F3).
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
                let w0 = q - center;
                let b2 = w0.dot(d);
                let c2 = w0.norm_squared() - radius.powi(2);
                let disc = b2.powi(2) - c2;
                let two_r = T::from_f64(2.0) * radius;
                match decide(
                    "bool_ray_sphere_disc",
                    Margin::over_lever(disc, two_r),
                    band,
                )
                .map_err(escalate)?
                {
                    Sign::Positive => {}
                    Sign::Zero => return Ok(None), // tangent ray: graze
                    Sign::Negative => continue,    // definite miss
                }
                let root = disc.max(T::zero()).sqrt();
                // The near/far outward pair is read off the geometry
                // (`d·(p − c)/r = ±√disc/r`) and is therefore a CHART
                // statement: it says the near root enters the BALL and
                // the far root leaves it. Whether entering the ball
                // means entering material is the face's sense (S10) —
                // a reversed sphere face bounds the material OUTSIDE
                // it — so the pair is swapped rather than recomputed.
                for (t, outward) in [
                    (T::zero() - b2 - root, oriented(Sign::Negative, sense)),
                    (T::zero() - b2 + root, oriented(Sign::Positive, sense)),
                ] {
                    if fold(&mut best, face, t, outward)?.is_none() {
                        return Ok(None);
                    }
                }
            }
            // The TRIMMED sphere face: the same quadratic, the same
            // read-off outward pair, with each root filtered through
            // the face's own chart rectangle — the cylinder arm's
            // shape. A root outside the trim is not a crossing of THIS
            // face; a root ON its boundary is a graze the schedule
            // retries, never a parity guess.
            FaceGeo::SpherePatch {
                center,
                radius,
                axis,
                u_ref,
                ref trim,
                sense,
            } => {
                let w0 = q - center;
                let b2 = w0.dot(d);
                let c2 = w0.norm_squared() - radius.powi(2);
                let disc = b2.powi(2) - c2;
                let two_r = T::from_f64(2.0) * radius;
                match decide(
                    "bool_ray_sphere_disc",
                    Margin::over_lever(disc, two_r),
                    band,
                )
                .map_err(escalate)?
                {
                    Sign::Positive => {}
                    Sign::Zero => return Ok(None), // tangent ray: graze
                    Sign::Negative => continue,    // definite miss
                }
                let root = disc.max(T::zero()).sqrt();
                for (t, outward) in [
                    (T::zero() - b2 - root, oriented(Sign::Negative, sense)),
                    (T::zero() - b2 + root, oriented(Sign::Positive, sense)),
                ] {
                    let p = q + d * t;
                    match point_on_sphere_in_face(face, center, radius, axis, u_ref, trim, p, band)?
                    {
                        Some(false) => continue,
                        None => return Ok(None), // trim-boundary hit: graze
                        Some(true) => {}
                    }
                    if fold(&mut best, face, t, outward)?.is_none() {
                        return Ok(None);
                    }
                }
            }
        }
    }
    match best {
        // Closest crossing exits material (d·n > 0) ⇒ q is In.
        Some((_, Sign::Positive)) => Ok(Some(SolidContainment::In)),
        Some((_, _)) => Ok(Some(SolidContainment::Out)),
        // No crossing: q is on the at-infinity side (module docs).
        None => Ok(Some(at_infinity_side(body, faces, band, tol)?)),
    }
}

/// The at-infinity material side, from the body's EXACT signed
/// volume (the divergence-theorem props — carrier-aware since the
/// M5 PR 9 fix pass: the old vertex-fan triangulation read a two-arc
/// disc cylinder as (near-)zero volume, a structural degeneracy of
/// the chord approximation, and refused `ZeroVolumeBody` on a
/// perfectly solid operand). Margin is scaled to a mean thickness
/// (V / surface area, meters), as before.
fn at_infinity_side<T: Decide>(
    body: &Body<T>,
    faces: &[FaceKey],
    band: Band,
    tol: Tol,
) -> Result<SolidContainment, PointInSolidError> {
    // Closed-form lane (M5 PR 11 lane split) — see `volume_backstop`.
    //
    // The props refusal is READ, not flattened. `VolumeUncertified`'s
    // own message asserts "the body is HEALTHY and this door's arms
    // answered; what is missing is a volume", and two of the props
    // lane's refusals make that sentence false: an escalation is an
    // ill-conditioned operand at this ε (with a predicate name and a
    // band the caller can act on), and the corruption-shaped arms are
    // arena claims about a BROKEN body. Each keeps its own door.
    let props = crate::props::mass_properties_closed_form(body, band, tol).map_err(|e| {
        match e {
            // An escalation stays an escalation, carrying its
            // diagnostics and the face it happened on.
            crate::props::MassPropsError::Face {
                face,
                source: geom_brep::props::PropsError::Escalated { cause },
            } => PointInSolidError::Escalated { face, diag: cause },
            // Corruption-shaped: a face whose area enclosure will not
            // certify a positive extent, a key the props walk could not
            // resolve, or null scaffolding in a body being classified
            // AT REST. None of these is "healthy body, missing
            // capability".
            crate::props::MassPropsError::Face {
                face,
                source: geom_brep::props::PropsError::DegenerateFace,
            } => PointInSolidError::CorruptFace { face },
            crate::props::MassPropsError::Corrupt { .. }
            | crate::props::MassPropsError::NullScaffoldEdge { .. } => {
                PointInSolidError::CorruptFace { face: faces[0] }
            }
            // The remainder IS the capability gap the variant
            // describes: a boundary outside the iso-rectangle
            // inventory (the standing rimless-lune case), a ring on a
            // curved face, an unimplemented kind, a quadrature that
            // would not converge inside its budget, a band that would
            // not construct. A HEALTHY body, and a missing volume.
            crate::props::MassPropsError::Band { .. }
            | crate::props::MassPropsError::RingOnCurvedFace { .. }
            | crate::props::MassPropsError::Face { .. } => PointInSolidError::VolumeUncertified,
        }
    })?;
    let margin = Margin::over_lever(props.volume, props.surface_area);
    match decide("bool_point_in_solid_infinity", margin, band).map_err(|diag| {
        PointInSolidError::Escalated {
            face: faces[0],
            diag,
        }
    })? {
        Sign::Positive => Ok(SolidContainment::Out),
        Sign::Negative => Ok(SolidContainment::In),
        Sign::Zero => Err(PointInSolidError::ZeroVolumeBody),
    }
}

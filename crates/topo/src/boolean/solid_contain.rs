//! Trilean **point-in-solid** containment (F8): the ray design of
//! profile's 2-D machinery and PR 3's [`point_in_loop`] promoted to
//! 3-D — the boolean containment fallback for operands whose
//! boundaries do not intersect (§15.9: "the ONLY place a
//! point-in-solid test is ever needed").
//!
//! # Method: closest-hit ray test with the fixed schedule
//!
//! Cast a ray from `q` along a direction of the fixed schedule (the
//! same 16-member golden-angle table as [`point_in_loop`], used as
//! space directions directly). For each face (planar — the F5 regime):
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
//! - **`bool_point_in_solid_infinity`**: the signed volume (scaled to
//!   a mean-thickness margin in meters) — consulted only when a ray
//!   crosses NO boundary at all: `q` then sits on the at-infinity
//!   side, which is `Out` for a positively-oriented body and `In` for
//!   a complement (negative signed volume — PR 1's revert posture).
//!   Without this, a no-hit ray on a reverted operand would misreport
//!   complement material as `Out`.

use geom_core::{Band, Decide, Indeterminate, Point3, Sign, Vec3};

use crate::body::Body;
use crate::entity::{FaceKey, LoopBoundary};
use crate::splitting::containment::{LoopContainment, PointInLoopError, point_in_loop};
use crate::validate::decide;
use geom_surfaces::Surface;

/// The 16-member fixed direction schedule (shared table with
/// `point_in_loop` — re-declared here to keep the module boundaries
/// thin; the values are the ratified schedule).
const SCHEDULE: [[f64; 3]; 16] = [
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.5, 0.25, 1.0],
    [1.0, 0.5, 0.25],
    [0.25, 1.0, 0.5],
    [-0.5, 1.0, 0.125],
    [0.125, -0.5, 1.0],
    [1.0, 0.125, -0.5],
    [0.75, -1.0, 0.375],
    [0.375, 0.75, -1.0],
    [-1.0, 0.375, 0.75],
    [0.625, 0.9375, 0.3125],
    [0.3125, -0.625, 0.9375],
    [0.9375, 0.3125, -0.625],
    [-0.75, -0.25, 1.0],
];

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
    /// A face is not planar (F5) or not walkable.
    CorruptFace {
        /// The face.
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
                    "point_in_solid: face {face:?} is not a walkable planar face"
                )
            }
        }
    }
}

impl std::error::Error for PointInSolidError {}

/// The face's plane, F5-gated.
pub(super) fn face_plane<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<(Point3<T>, Vec3<T>), PointInSolidError> {
    let f = body
        .get_face(face)
        .ok_or(PointInSolidError::CorruptFace { face })?;
    match body.get_surface(f.surface) {
        Some(Surface::Plane { origin, normal, .. }) => Ok((*origin, *normal)),
        _ => Err(PointInSolidError::CorruptFace { face }),
    }
}

/// A face's resolved query geometry (M5 PR 9): the planar datum, or a
/// cylinder wall with its exact chart trim — azimuth window from the
/// outer cycle's closed-form chart images (the S9 machinery), height
/// range from the boundary vertices (exact for the iso-bounded wall
/// class every M5 operand mints: rims are height iso-lines, meridians
/// azimuth iso-lines, and the boolean's own seam arcs are height
/// iso-lines too).
enum FaceGeo<T: geom_core::Real> {
    Plane(Point3<T>, Vec3<T>),
    Cylinder {
        origin: Point3<T>,
        axis: Vec3<T>,
        radius: T,
        u_ref: Vec3<T>,
        az: (T, T),
        h: (T, T),
    },
}

/// Resolves [`FaceGeo`]; kinds outside {Plane, Cylinder} refuse as
/// [`PointInSolidError::CorruptFace`] (per-arm, C12.1 — the sphere
/// wall arm arrives with the rung-3 zip).
fn face_geo<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<FaceGeo<T>, PointInSolidError> {
    let f = body
        .get_face(face)
        .ok_or(PointInSolidError::CorruptFace { face })?;
    match body.get_surface(f.surface) {
        Some(Surface::Plane { origin, normal, .. }) => Ok(FaceGeo::Plane(*origin, *normal)),
        Some(&Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        }) => {
            let surf = body
                .get_surface(f.surface)
                .cloned()
                .ok_or(PointInSolidError::CorruptFace { face })?;
            let az = crate::splitting::join::face_azimuth_window(body, &surf, face, band)
                .ok()
                .flatten()
                .ok_or(PointInSolidError::CorruptFace { face })?;
            // Height range from the outer cycle's vertices (exact for
            // the iso-bounded wall class). Folded WITHOUT infinity
            // seeds: an Interval scalar's ±∞ singleton is Ill and
            // poisons every min/max through it (the MAJOR-1 root
            // cause — the whole Interval boolean lane died on a NaN
            // height range).
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
            Ok(FaceGeo::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
                az,
                h,
            })
        }
        _ => Err(PointInSolidError::CorruptFace { face }),
    }
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
#[allow(clippy::too_many_arguments)] // one internal lane, each a named datum
fn point_on_wall_in_face<T: Decide>(
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
    let (w_min, w_max) = az;
    let width = w_max - w_min;
    // The cosine equivalence needs width < τ, decided loudly (a
    // full-period window cannot trim by angle at all).
    match decide("bool_wall_trim_period", (T::tau() - width) * radius, band).map_err(escalate)? {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => {
            return Err(escalate(geom_core::Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band,
                predicate: Some("bool_wall_trim_period"),
            }));
        }
    }
    let w = p - origin;
    let height = w.dot(axis);
    let radial = w - axis * height;
    let r_hat = radial / radial.norm();
    let half = T::from_f64(0.5);
    let mid = (w_min + w_max) * half;
    let (s_m, c_m) = mid.sin_cos();
    let v_ref = axis.cross(u_ref);
    let m_hat = u_ref * c_m + v_ref * s_m;
    let (s_h, c_h) = (width * half).sin_cos();
    let _ = s_h; // the cosine comparison carries the whole test
    let cone = (r_hat.dot(m_hat) - c_h) * radius;
    let mut verdict = Some(true);
    for margin in [cone, height - h.0, h.1 - height] {
        match decide("bool_wall_trim", margin, band).map_err(escalate)? {
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
) -> Result<SolidContainment, PointInSolidError> {
    // Deterministic face sweep order (arena order).
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();

    // ---- Boundary pre-pass: q on any face ⇒ OnBoundary. ----
    for &face in &faces {
        let escalate = |diag| PointInSolidError::Escalated { face, diag };
        match face_geo(body, face, band)? {
            FaceGeo::Plane(origin, normal) => {
                let elev = (q - origin).dot(normal);
                if decide("bool_point_in_solid_plane", elev, band).map_err(escalate)? == Sign::Zero
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
            } => {
                let w = q - origin;
                let radial = w - axis * w.dot(axis);
                // The linearized residual (metres) — the same form the
                // certification layer classifies.
                let elev = (radial.norm_squared() - radius.powi(2)) / (T::from_f64(2.0) * radius);
                if decide("bool_point_in_solid_plane", elev, band).map_err(escalate)? == Sign::Zero
                {
                    match point_on_wall_in_face(face, origin, axis, radius, u_ref, az, h, q, band)?
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
        if let Some(verdict) = cast_ray(body, &faces, q, d, band)? {
            return Ok(verdict);
        }
        // graze: next schedule member
    }
    Err(PointInSolidError::RayExhausted)
}

/// One ray of the sweep: `Some(verdict)` or `None` for a graze.
fn cast_ray<T: Decide>(
    body: &Body<T>,
    faces: &[FaceKey],
    q: Point3<T>,
    d: Vec3<T>,
    band: Band,
) -> Result<Option<SolidContainment>, PointInSolidError> {
    let mut best: Option<(T, Sign)> = None; // (advance, sign of d·n)
    // A candidate crossing (advance, outward sign), or a graze.
    let fold = |best: &mut Option<(T, Sign)>,
                face: FaceKey,
                t: T,
                outward: Sign|
     -> Result<Option<()>, PointInSolidError> {
        let escalate = |diag| PointInSolidError::Escalated { face, diag };
        match decide("bool_point_in_solid_advance", t, band).map_err(escalate)? {
            Sign::Positive => {}
            Sign::Negative => return Ok(Some(())),
            // A genuine crossing at q contradicts the boundary
            // pre-pass — graze, retry.
            Sign::Zero => return Ok(None),
        }
        *best = match *best {
            None => Some((t, outward)),
            Some((tb, sb)) => {
                match decide("bool_point_in_solid_order", t - tb, band).map_err(escalate)? {
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
                let denom = d.dot(normal);
                let denom_sign =
                    decide("bool_point_in_solid_denom", denom, band).map_err(escalate)?;
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
            } => {
                let w0 = q - origin;
                let w0p = w0 - axis * w0.dot(axis);
                let dp = d - axis * d.dot(axis);
                let a2 = dp.norm_squared();
                let two_r = T::from_f64(2.0) * radius;
                // Axis-parallel ray: constant residual; the pre-pass
                // said q is off the wall, so it misses entirely.
                match decide("bool_point_in_solid_denom", a2 / two_r, band).map_err(escalate)? {
                    Sign::Positive => {}
                    _ => continue,
                }
                let b2 = w0p.dot(dp);
                let c2 = w0p.norm_squared() - radius.powi(2);
                let disc = b2.powi(2) - a2 * c2;
                // Metre-scaled discriminant: Positive ⇒ two definite
                // roots; Zero ⇒ a tangent ray (graze, retry the next
                // schedule member); Negative ⇒ definite miss; in-band
                // escalates through the funnel.
                match decide("bool_ray_cylinder_disc", disc / two_r.powi(2), band)
                    .map_err(escalate)?
                {
                    Sign::Positive => {}
                    Sign::Zero => return Ok(None), // tangent ray: graze
                    Sign::Negative => continue,    // definite miss
                }
                let root = disc.max(T::zero()).sqrt();
                for t in [(T::zero() - b2 - root) / a2, (T::zero() - b2 + root) / a2] {
                    let p = q + d * t;
                    match point_on_wall_in_face(face, origin, axis, radius, u_ref, az, h, p, band)?
                    {
                        Some(false) => continue,
                        None => return Ok(None), // trim-boundary hit: graze
                        Some(true) => {}
                    }
                    // Outward sign: d · (radial gradient) at the hit.
                    let wp = p - origin;
                    let rad = wp - axis * wp.dot(axis);
                    let outward = decide("bool_point_in_solid_denom", d.dot(rad) / radius, band)
                        .map_err(escalate)?;
                    if outward == Sign::Zero {
                        return Ok(None); // grazing incidence at the hit
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
        None => Ok(Some(at_infinity_side(body, faces, band)?)),
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
) -> Result<SolidContainment, PointInSolidError> {
    let props = crate::props::mass_properties_with(body, band).map_err(|_| {
        PointInSolidError::CorruptFace {
            face: faces.first().copied().unwrap_or_default(),
        }
    })?;
    let margin = props.volume / props.surface_area;
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

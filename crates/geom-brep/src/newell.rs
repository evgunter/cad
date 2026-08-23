//! Newell face equations: certified planes from loop vertex data
//! (Mäntylä ch. 13.1, with the Problem 13.1 translate-to-origin
//! accuracy fix as the **default**, not an exercise).
//!
//! [`newell_plane`] turns an ordered loop of vertex positions into a
//! certified [`Surface::Plane`]. Plane equations are *derived caches* —
//! Euler operators never fill them (M2-PLAN's source grounding); the
//! sweeps call this when they create planar caps and side faces from
//! profile data (they know planarity by construction — Newell turns
//! vertex data into the certified surface), and the tier-3 validator
//! re-checks the stored plane against the vertices at rest.
//!
//! # The method, and why translate-to-origin
//!
//! Newell's normal is the cross-product sum `Σ pᵢ × pᵢ₊₁` (cyclic).
//! Evaluated raw, each term has magnitude ~|p|² and the sum cancels to
//! ~(loop area): for a feature of extent 1 m at offset 1e8 m, the
//! terms are ~1e16 and f64 keeps ~none of the ~1e0 result — the normal
//! is noise. Subtracting the loop centroid first makes each term
//! ~extent², eliminating the cancellation entirely:
//!
//! `c = (Σ pᵢ)/n`, `N = Σ (pᵢ − c) × (pᵢ₊₁ − c)`, normal = `N/|N|`.
//!
//! The plane's origin is the centroid `c` (on the plane whenever the
//! vertices are — the mean of points within ε of a plane is within ε of
//! it); `u_ref` comes from the branchless orthonormal basis (Duff 2017,
//! the PR 1 machinery), so the whole construction is comparison-free.
//! The far-from-origin accuracy is pinned by the 1e8-offset rectangle
//! test at ε = 1e-9.
//!
//! # Orientation contract
//!
//! The returned normal is the **right-hand normal of the vertex order**:
//! walk the loop in the given order and the normal points toward the
//! viewer who sees that walk counterclockwise. Callers feeding a face's
//! outer loop in `next` order therefore get the *outward* normal (the
//! ratified interior-left convention) — orientation is the caller's
//! loop order, never a hidden choice here.
//!
//! # Certification
//!
//! Every vertex's signed distance to the constructed plane is
//! classified against the run's linear band: |dᵢ| ≤ ε required
//! (`newell_plane_residual`). A definite excess is
//! [`NewellError::NotPlanar`]; an in-band or poisoned residual (e.g. a
//! degenerate loop whose normal normalizes to poison) escalates
//! (D4 ¶3) — total, never a panic.
//!
//! # What certification does NOT pin: the normal's direction on
//! near-collinear input
//!
//! The residual contract is *positional*: every vertex within ε of the
//! returned plane. For a **near-collinear** loop — all vertices within
//! ε of a line — infinitely many planes satisfy that contract, and the
//! returned normal's *direction* is determined by the sub-ε lateral
//! noise: two inputs differing by a fraction of ε can receive
//! near-opposite certified normals. This is inherent to
//! under-determined data, not a defect of the method (exact
//! collinearity escalates via the poison normal; the near-collinear
//! band cannot, because the residuals honestly certify). Consumers
//! reading the normal as an *orientation* (outward-normal contracts)
//! must not feed near-collinear loops expecting a stable direction —
//! the sweeps never do (validated profiles have definite area), and
//! anything that might should treat the direction as data-quality-
//! limited. (M2 PR 3 review, N3 — pinned by the promoted
//! `fixed_n3_*` test.)

use geom::Surface;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use crate::dihedral::decide;

/// Typed failure of [`newell_plane`] (closed enum, D4 ¶3).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NewellError {
    /// Fewer than three vertices: no plane is determined.
    TooFewVertices {
        /// The vertex count given.
        got: usize,
    },
    /// A vertex is definitely off the constructed plane (its residual
    /// exceeds the escalation threshold): the loop is not planar at
    /// tolerance.
    NotPlanar {
        /// The index (in the given order) of the offending vertex.
        vertex: usize,
    },
    /// A residual classification escalated: in the sliver band, or
    /// poisoned (degenerate loops — collinear or coincident vertices —
    /// surface here through the poison normal).
    Escalated {
        /// The index of the vertex whose residual escalated.
        vertex: usize,
        /// The classifier's diagnostic.
        cause: Indeterminate,
    },
}

impl core::fmt::Display for NewellError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooFewVertices { got } => {
                write!(
                    f,
                    "newell: {got} vertices cannot determine a plane (need ≥ 3)"
                )
            }
            Self::NotPlanar { vertex } => write!(
                f,
                "newell: vertex {vertex} lies definitely off the fitted plane — the loop \
                 is not planar at tolerance (D4)"
            ),
            Self::Escalated { vertex, cause } => {
                write!(f, "newell: residual at vertex {vertex} escalated: {cause}")
            }
        }
    }
}

impl std::error::Error for NewellError {}

/// Computes the certified [`Surface::Plane`] of an ordered planar loop
/// by translate-to-origin Newell (module docs: method, orientation
/// contract, certification). `band` is the run's linear band.
///
/// Evaluation order (fixed, D9): centroid as the left-to-right sum of
/// the points' coordinates divided by n; the cross-sum left-to-right
/// over consecutive pairs, wrapping at the end; normalize; residuals in
/// vertex order.
///
/// # Errors
///
/// [`NewellError`] — arity, definite non-planarity, or escalation (the
/// first failing vertex, in order).
pub fn newell_plane<T: Decide>(
    points: &[Point3<T>],
    band: Band,
) -> Result<Surface<T>, NewellError> {
    if points.len() < 3 {
        return Err(NewellError::TooFewVertices { got: points.len() });
    }
    // Centroid (translate-to-origin's translation).
    let n = T::from_f64(points.len() as f64);
    let mut sum = Vec3::zero();
    for p in points {
        sum = sum + (*p - Point3::origin());
    }
    let centroid = Point3::origin() + sum / n;
    // The translated cross-sum.
    let mut normal_sum = Vec3::zero();
    for (i, p) in points.iter().enumerate() {
        let next = points[(i + 1) % points.len()];
        normal_sum = normal_sum + (*p - centroid).cross(next - centroid);
    }
    let normal = normal_sum.normalize();
    let (u_ref, _) = normal.orthonormal_basis();
    let plane = Surface::Plane {
        origin: centroid,
        normal,
        u_ref,
    };
    // Certification: every vertex within ε of the plane.
    for (i, p) in points.iter().enumerate() {
        let residual = (*p - centroid).dot(normal);
        match decide("newell_plane_residual", Margin::of(residual), band) {
            Ok(Sign::Zero) => {}
            Ok(Sign::Positive | Sign::Negative) => {
                return Err(NewellError::NotPlanar { vertex: i });
            }
            Err(cause) => return Err(NewellError::Escalated { vertex: i, cause }),
        }
    }
    Ok(plane)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;

    use super::*;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
        Point3::new(x, y, z)
    }

    #[test]
    fn unit_square_ccw_gives_plus_z() {
        // CCW in the xy-plane viewed from +z ⇒ normal = +z (the
        // orientation contract).
        let pts = [
            pt(0.0, 0.0, 0.0),
            pt(1.0, 0.0, 0.0),
            pt(1.0, 1.0, 0.0),
            pt(0.0, 1.0, 0.0),
        ];
        let plane = newell_plane(&pts, band()).unwrap();
        let Surface::Plane {
            origin,
            normal,
            u_ref,
        } = plane
        else {
            panic!("newell returns a plane");
        };
        assert_eq!((normal.x, normal.y, normal.z), (0.0, 0.0, 1.0));
        assert_eq!((origin.x, origin.y), (0.5, 0.5));
        // u_ref is unit and perpendicular to the normal (branchless
        // basis).
        assert!((u_ref.norm() - 1.0).abs() < 1e-15);
        assert!(u_ref.dot(normal).abs() < 1e-15);
        // Reversed order flips the normal.
        let rev: Vec<_> = pts.iter().rev().copied().collect();
        let Surface::Plane { normal: n2, .. } = newell_plane(&rev, band()).unwrap() else {
            panic!("plane");
        };
        assert_eq!((n2.x, n2.y, n2.z), (0.0, 0.0, -1.0));
    }

    /// The translate-to-origin pin (Problem 13.1 as default): a 1 m × 2 m
    /// rectangle offset 1e8 m from the origin, certified at ε = 1e-9
    /// and below (all CI ε rows). The corner coordinates are exact
    /// dyadics — 1e8 + small integers/halves are exactly representable
    /// — so the *data* is exactly planar and the certification residual
    /// isolates the **method's** error: translated Newell is exact
    /// here; the naive cross-sum's normal is garbage (demonstrated
    /// inline — its terms are ~1e16, past 2⁵³, and the cancellation
    /// leaves rounding noise of the same order as the true area
    /// vector).
    #[test]
    fn far_from_origin_rectangle_is_accurate() {
        // A tilted 1 × 2 rectangle: spanned by dyadic (non-unit) frame
        // vectors, so the corner coordinates are exact dyadics — the
        // data is *exactly* planar in ℝ, and the certification residual
        // isolates the method's own error.
        let off = pt(1.0e8 + 0.25, 1.0e8 + 0.5, 1.0e8 + 0.125);
        let u = Vec3::new(1.0, 0.0, 0.5);
        let v = Vec3::new(0.0, 1.0, -0.25);
        let true_n = u.cross(v).normalize();
        let corner = |a: f64, b: f64| off + u * a + v * b;
        let pts = [
            corner(0.0, 0.0),
            corner(1.0, 0.0),
            corner(1.0, 2.0),
            corner(0.0, 2.0),
        ];
        let plane = newell_plane(&pts, band()).unwrap();
        let Surface::Plane { normal, .. } = plane else {
            panic!("plane");
        };
        // Translated Newell is exact on this data (centroid,
        // differences, and cross products are all exact dyadics): the
        // normal matches the frame's true normal to rounding and every
        // residual certified ≤ ε at ε = 1e-9 and below.
        assert!(normal.cross(true_n).norm() < 1e-15);
        assert!(normal.dot(true_n) > 0.0);

        // The naive (untranslated) cross-sum, for contrast: at 1e8
        // offsets its ~1e16 products round (past 2⁵³) and the
        // cancellation leaves a normal ~0.3 radians off — the
        // justification for the translate-to-origin default.
        let mut naive = Vec3::zero();
        for (i, p) in pts.iter().enumerate() {
            let q = pts[(i + 1) % pts.len()];
            naive = naive + (*p - Point3::origin()).cross(q - Point3::origin());
        }
        let naive_n = naive.normalize();
        let err = naive_n.cross(true_n).norm();
        assert!(
            err > 1e-2,
            "naive Newell unexpectedly accurate: {naive_n:?} (err {err:e})"
        );
    }

    #[test]
    fn non_planar_loop_is_rejected() {
        let lift = 1000.0 * Tol::witness().get().eps; // ≥ K·ε at every CI row
        let pts = [
            pt(0.0, 0.0, 0.0),
            pt(1.0, 0.0, 0.0),
            pt(1.0, 1.0, lift),
            pt(0.0, 1.0, 0.0),
        ];
        let err = newell_plane(&pts, band()).unwrap_err();
        assert!(matches!(err, NewellError::NotPlanar { .. }), "{err:?}");
        // An in-band lift escalates instead. The fitted plane absorbs
        // 3/4 of a single-vertex lift (the Newell normal tilts toward
        // the best fit), so the worst residual is lift/4 — put THAT in
        // the band: lift = 12ε ⇒ residual ≈ 3ε ∈ (ε, 10ε).
        let lift = 12.0 * Tol::witness().get().eps;
        let pts = [
            pt(0.0, 0.0, 0.0),
            pt(1.0, 0.0, 0.0),
            pt(1.0, 1.0, lift),
            pt(0.0, 1.0, 0.0),
        ];
        let err = newell_plane(&pts, band()).unwrap_err();
        assert!(matches!(err, NewellError::Escalated { .. }), "{err:?}");
    }

    #[test]
    fn degenerate_loops_escalate_not_panic() {
        // Collinear: zero normal ⇒ poison residuals ⇒ escalation.
        let pts = [pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0), pt(2.0, 0.0, 0.0)];
        let err = newell_plane(&pts, band()).unwrap_err();
        assert!(matches!(err, NewellError::Escalated { .. }), "{err:?}");
        // Arity floor.
        assert_eq!(
            newell_plane(&[pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0)], band()).unwrap_err(),
            NewellError::TooFewVertices { got: 2 }
        );
    }
}

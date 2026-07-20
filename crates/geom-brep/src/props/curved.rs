//! Closed-form flux/area for the curved M2 surfaces (cylinder, cone,
//! sphere, torus) over structurally verified iso-parameter rectangles
//! (see [`super`] module docs for the formulation and the stored-data
//! discipline).
//!
//! Classification vocabulary: a boundary edge of a revolution surface
//! is a **rim** (iso-`v` circle about the surface axis — carrier axis
//! parallel or antiparallel to the surface axis) or a **meridian**
//! (iso-`u`: an axial/generator line, a great circle through the
//! sphere's poles, or a torus minor circle — carrier axis
//! perpendicular to the surface axis). Every decision goes through the
//! crate's recording funnel with a `props_*` predicate name; a
//! definite-nonzero consistency residual or an out-of-inventory shape
//! is a typed [`PropsError`].

use geom_core::{Band, Decide, Point3, Real, Sign, Vec3};
use geom_curves::Curve3;
use geom_surfaces::Surface;

use super::{FaceContribution, LoopEdge, PropsError, loop_vector_area};
use crate::dihedral::decide;

/// The flux and area of a curved face from its **outer** loop (curved
/// M2 faces carry no rings — the owning body refuses ringed curved
/// faces before calling). Dispatches on the surface kind; `band` is
/// the run's linear band, built once at operation entry.
///
/// # Errors
///
/// [`PropsError`] — unimplemented kinds, out-of-inventory boundary
/// shapes, definite consistency failures, escalated classifications.
pub fn curved_face<T: Decide>(
    surface: &Surface<T>,
    outer: &[LoopEdge<T>],
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    match *surface {
        Surface::Plane { .. } => Err(PropsError::NotIsoRectangle {
            what: "curved_face called on a plane (planar faces take the loop route)",
        }),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => cylinder(origin, axis, radius, outer, band),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => cone(apex, axis, half_angle, outer, band),
        Surface::Sphere {
            center,
            radius,
            axis,
            ..
        } => sphere(center, radius, axis, outer, band),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => torus(center, axis, major_radius, minor_radius, outer, band),
        Surface::Nurbs => Err(PropsError::Unimplemented),
    }
}

// ---------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------

/// ±1 as a scalar from a **definite** sign (`Zero` maps to 0 — callers
/// only reach this with definite outcomes).
fn t_sign<T: Real>(s: Sign) -> T {
    match s {
        Sign::Positive => T::one(),
        Sign::Negative => -T::one(),
        Sign::Zero => T::zero(),
    }
}

/// ±1 for the loop's traversal direction of an edge.
fn trav<T: Real>(forward: bool) -> T {
    if forward { T::one() } else { -T::one() }
}

/// Funnel wrapper: classify, mapping an escalation to the typed
/// [`PropsError::Escalated`].
fn classify<T: Decide>(name: &'static str, margin: T, band: Band) -> Result<Sign, PropsError> {
    decide(name, margin, band).map_err(|cause| PropsError::Escalated { cause })
}

/// Require a consistency residual to be coincident with zero.
fn require_zero<T: Decide>(name: &'static str, margin: T, band: Band) -> Result<(), PropsError> {
    match classify(name, margin, band)? {
        Sign::Zero => Ok(()),
        Sign::Positive | Sign::Negative => Err(PropsError::NotIsoRectangle { what: name }),
    }
}

/// Require a definitely-positive extent (degenerate ⇒ typed error).
fn require_extent<T: Decide>(margin: T, band: Band) -> Result<(), PropsError> {
    match classify("props_face_extent", margin, band)? {
        Sign::Positive => Ok(()),
        Sign::Zero | Sign::Negative => Err(PropsError::DegenerateFace),
    }
}

/// A classified rim: signed `u`-traversal direction (`d_u`), parameter
/// span (`dt`, the face's `Δu` candidate), and its iso-level payload.
struct Rim<T: Real> {
    /// ±1: traversal direction in `u` = sign(carrier axis · surface
    /// axis) × traversal direction.
    d_u: T,
    /// Carrier parameter span `t1 − t0` (angle-true; `Δu`).
    dt: T,
    /// The rim's iso-level: `v` for cylinder/cone, `sin v` for the
    /// sphere, `(sin v, cos v)` packed for the torus (see call sites).
    level: (T, T),
    /// Traversal-order endpoint tags.
    tags: (u32, u32),
}

/// Check all rims agree on `Δu`, metered by `arm` (meters); returns
/// the face's `Δu`.
fn du_of_rims<T: Decide>(rims: &[Rim<T>], arm: T, band: Band) -> Result<T, PropsError> {
    let Some(first) = rims.first() else {
        return Err(PropsError::NotIsoRectangle {
            what: "curved face without a rim (non-sphere)",
        });
    };
    for rim in &rims[1..] {
        require_zero("props_du_consistent", (rim.dt - first.dt) * arm, band)?;
    }
    Ok(first.dt)
}

/// `s_f` for a linearly-leveled surface (cylinder/cone/sphere): from
/// `rim`'s level and the face's level range, metered by `arm`.
/// Interior lies toward the opposite extreme; a rim strictly inside
/// the range cannot happen on an iso-rectangle (the residual would be
/// the full extent, definite).
fn s_f_from_rim<T: Decide>(
    rim: &Rim<T>,
    lo: T,
    hi: T,
    arm: T,
    band: Band,
) -> Result<T, PropsError> {
    let other = lo + hi - rim.level.0;
    match classify("props_rim_side", (other - rim.level.0) * arm, band)? {
        Sign::Positive => Ok(rim.d_u),
        Sign::Negative => Ok(-rim.d_u),
        Sign::Zero => Err(PropsError::DegenerateFace),
    }
}

// ---------------------------------------------------------------------
// Cylinder
// ---------------------------------------------------------------------

/// Cylinder face: rims are circles of the cylinder's radius centered
/// on the axis; meridians are axial lines. `v = (p − origin)·axis`
/// (meters), `Area = r·Δu·(v_hi − v_lo)`,
/// `∮(p−o)·n_chart dA = r·Area` ⇒ flux `= s_f·r·Area + o·A⃗`.
fn cylinder<T: Decide>(
    origin: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    let mut rims: Vec<Rim<T>> = Vec::new();
    let mut levels: Vec<T> = Vec::new();
    for e in edges {
        match e.carrier {
            Curve3::Line { dir, .. } => {
                require_zero(
                    "props_meridian_axial",
                    dir.cross(axis).norm() * (e.t1 - e.t0),
                    band,
                )?;
                levels.push((e.p0() - origin).dot(axis));
                levels.push((e.p1() - origin).dot(axis));
            }
            Curve3::Circle {
                center,
                axis: n_c,
                radius: r_c,
                ..
            } => {
                let s = classify("props_circle_axis_class", n_c.dot(axis) * r_c, band)?;
                if s == Sign::Zero {
                    return Err(PropsError::NotIsoRectangle {
                        what: "cylinder boundary circle is not a rim",
                    });
                }
                require_zero("props_rim_fit", r_c - radius, band)?;
                let v = (center - origin).dot(axis);
                rims.push(Rim {
                    d_u: t_sign::<T>(s) * trav(e.forward),
                    dt: e.t1 - e.t0,
                    level: (v, T::zero()),
                    tags: (e.start, e.end),
                });
                levels.push(v);
            }
            Curve3::Nurbs => return Err(PropsError::Unimplemented),
        }
    }
    let du = du_of_rims(&rims, radius, band)?;
    let (lo, hi) = min_max(&levels)?;
    require_extent(hi - lo, band)?;
    let s_f = s_f_from_rim(&rims[0], lo, hi, T::one(), band)?;
    let area = radius * du * (hi - lo);
    let va = loop_vector_area(edges, origin)?;
    let flux = s_f * (radius * area) + (origin - Point3::origin()).dot(va);
    Ok(FaceContribution { flux, area })
}

/// Fold a nonempty level list to (min, max).
fn min_max<T: Real>(levels: &[T]) -> Result<(T, T), PropsError> {
    let Some((&first, rest)) = levels.split_first() else {
        return Err(PropsError::NotIsoRectangle {
            what: "curved face with an empty boundary",
        });
    };
    let mut lo = first;
    let mut hi = first;
    for &l in rest {
        lo = lo.min(l);
        hi = hi.max(l);
    }
    Ok((lo, hi))
}

// ---------------------------------------------------------------------
// Cone
// ---------------------------------------------------------------------

/// Cone face: rims are axis-centered circles of radius `|v|·sin α`;
/// meridians are generator lines (`|dir·axis| = cos α`). `v` is the
/// signed slant parameter `((p − apex)·axis)/cos α`; the face must not
/// definitely span both nappes. `Area = sin α·Δu·|v_hi² − v_lo²|/2`;
/// `(p − apex)·n_chart = 0` along generators, so the anchored term
/// vanishes and flux `= apex·A⃗` — no orientation sign is needed.
fn cone<T: Decide>(
    apex: Point3<T>,
    axis: Vec3<T>,
    half_angle: T,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    let (sin_a, cos_a) = half_angle.sin_cos();
    let mut rims: Vec<Rim<T>> = Vec::new();
    let mut levels: Vec<T> = Vec::new();
    for e in edges {
        match e.carrier {
            Curve3::Line { dir, .. } => {
                require_zero(
                    "props_meridian_generator",
                    (dir.dot(axis).abs() - cos_a) * (e.t1 - e.t0),
                    band,
                )?;
                levels.push((e.p0() - apex).dot(axis) / cos_a);
                levels.push((e.p1() - apex).dot(axis) / cos_a);
            }
            Curve3::Circle {
                center,
                axis: n_c,
                radius: r_c,
                ..
            } => {
                let s = classify("props_circle_axis_class", n_c.dot(axis) * r_c, band)?;
                if s == Sign::Zero {
                    return Err(PropsError::NotIsoRectangle {
                        what: "cone boundary circle is not a rim",
                    });
                }
                let v = (center - apex).dot(axis) / cos_a;
                require_zero("props_rim_fit", r_c - v.abs() * sin_a, band)?;
                rims.push(Rim {
                    d_u: t_sign::<T>(s) * trav(e.forward),
                    dt: e.t1 - e.t0,
                    level: (v, T::zero()),
                    tags: (e.start, e.end),
                });
                levels.push(v);
            }
            Curve3::Nurbs => return Err(PropsError::Unimplemented),
        }
    }
    let arm = rims
        .first()
        .map(|r| r.level.0.abs() * sin_a)
        .unwrap_or(T::one());
    let du = du_of_rims(&rims, arm, band)?;
    let (lo, hi) = min_max(&levels)?;
    require_extent(hi - lo, band)?;
    // Single-nappe check: definitely-negative low AND definitely-positive
    // high would straddle the apex through both nappes.
    let s_lo = classify("props_cone_nappe", lo, band)?;
    let s_hi = classify("props_cone_nappe", hi, band)?;
    if s_lo == Sign::Negative && s_hi == Sign::Positive {
        return Err(PropsError::NappeSpanning);
    }
    let half = T::from_f64(0.5);
    let area = sin_a * du * ((hi * hi - lo * lo) * half).abs();
    let va = loop_vector_area(edges, apex)?;
    let flux = (apex - Point3::origin()).dot(va);
    Ok(FaceContribution { flux, area })
}

// ---------------------------------------------------------------------
// Sphere
// ---------------------------------------------------------------------

/// Sphere face: rims are circles with axis ∥ the sphere axis
/// (`sin v = ((C − c)·axis)/R`); meridians are great circles through
/// the poles (center = sphere center, radius = R, axis ⊥ sphere axis).
/// Levels are `sin v` (latitude sines — all formulas need only these).
/// `Area = R²·Δu·(sin v_hi − sin v_lo)`,
/// `(p − c)·n_chart = R` ⇒ flux `= s_f·R·Area + c·A⃗`.
///
/// A face with **no rims** is the two-band construction (M2 PR 5's
/// axis-touching full revolve): its meridians are verified coplanar
/// and `Δu = π` **by construction**; `s_f = +1` because M2 sweeps emit
/// single outward shells only (voids arrive with booleans, M3) — a
/// rimless band with inward orientation is unrepresentable at rest.
fn sphere<T: Decide>(
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    let mut rims: Vec<Rim<T>> = Vec::new();
    let mut meridian_axes: Vec<Vec3<T>> = Vec::new();
    let mut levels: Vec<T> = Vec::new();
    for e in edges {
        let Curve3::Circle {
            center: c_c,
            axis: n_c,
            radius: r_c,
            ..
        } = e.carrier
        else {
            return Err(match e.carrier {
                Curve3::Nurbs => PropsError::Unimplemented,
                _ => PropsError::NotIsoRectangle {
                    what: "sphere boundary edge is not a circle",
                },
            });
        };
        let s = classify("props_circle_axis_class", n_c.dot(axis) * r_c, band)?;
        match s {
            Sign::Positive | Sign::Negative => {
                let w = c_c - center;
                require_zero(
                    "props_rim_fit",
                    (w.norm_squared() + r_c * r_c).sqrt() - radius,
                    band,
                )?;
                let sin_v = w.dot(axis) / radius;
                rims.push(Rim {
                    d_u: t_sign::<T>(s) * trav(e.forward),
                    dt: e.t1 - e.t0,
                    level: (sin_v, T::zero()),
                    tags: (e.start, e.end),
                });
                levels.push(sin_v);
            }
            Sign::Zero => {
                // Meridian great circle: centered at the sphere center
                // with the sphere's radius.
                require_zero(
                    "props_meridian_great",
                    (c_c - center).norm().max((r_c - radius).abs()),
                    band,
                )?;
                meridian_axes.push(n_c);
                levels.push((e.p0() - center).dot(axis) / radius);
                levels.push((e.p1() - center).dot(axis) / radius);
            }
        }
    }
    let (du, s_f);
    let (lo, hi) = min_max(&levels)?;
    require_extent((hi - lo) * radius, band)?;
    if rims.is_empty() {
        // Two-band face (module docs above): meridians coplanar, Δu = π.
        let Some((&first, rest)) = meridian_axes.split_first() else {
            return Err(PropsError::NotIsoRectangle {
                what: "sphere face with an empty boundary",
            });
        };
        for &n in rest {
            require_zero("props_band_coplanar", n.cross(first).norm() * radius, band)?;
        }
        du = T::pi();
        s_f = T::one();
    } else {
        du = du_of_rims(&rims, radius, band)?;
        s_f = s_f_from_rim(&rims[0], lo, hi, radius, band)?;
    }
    let area = radius * radius * du * (hi - lo);
    let va = loop_vector_area(edges, center)?;
    let flux = s_f * (radius * area) + (center - Point3::origin()).dot(va);
    Ok(FaceContribution { flux, area })
}

// ---------------------------------------------------------------------
// Torus
// ---------------------------------------------------------------------

/// Torus face: rims are circles with axis ∥ the torus axis at minor
/// angle `v` (`sin v = ((C − c)·axis)/r`, `cos v = (r_c − R)/r`);
/// meridians are minor circles (radius `r`, center on the tube center
/// circle, carrier axis ⊥ the torus axis). The minor angle is
/// periodic, so the face's `v`-interval comes from a meridian's
/// **stored parameter span** plus its orientation relative to the
/// chart (`dv/dt = −sign(n_c·τ̂)` with `τ̂ = axis × ρ̂` at the minor
/// center) — never from endpoint `atan2`. With `[v0, v1]` the
/// increasing interval (`Δv = v1 − v0`, `s_i = sin v_i`,
/// `c_i = cos v_i`):
///
/// ```text
/// Area = r·Δu·[R·Δv + r·(s1 − s0)]
/// ∮(p−c)·n_chart dA
///   = r·Δu·[(R²+r²)(s1−s0) + R·r·Δv + (R·r/2)(Δv + s1·c1 − s0·c0)]
/// ```
///
/// (from `(p−c)·n_chart = R·cos v + r` and the Jacobian
/// `r·(R + r·cos v)`). `s_f` uses the rim topologically adjacent to
/// the anchor meridian's `t0` endpoint: the interior lies from that
/// rim in the direction `dv/dt` sweeps.
fn torus<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    major: T,
    minor: T,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    struct Meridian<T: Real> {
        n_c: Vec3<T>,
        c_c: Point3<T>,
        dt: T,
        anchor: Point3<T>,
        anchor_tag: u32,
    }
    let mut rims: Vec<Rim<T>> = Vec::new();
    let mut meridians: Vec<Meridian<T>> = Vec::new();
    for e in edges {
        let Curve3::Circle {
            center: c_c,
            axis: n_c,
            radius: r_c,
            ..
        } = e.carrier
        else {
            return Err(match e.carrier {
                Curve3::Nurbs => PropsError::Unimplemented,
                _ => PropsError::NotIsoRectangle {
                    what: "torus boundary edge is not a circle",
                },
            });
        };
        let s = classify("props_circle_axis_class", n_c.dot(axis) * r_c, band)?;
        match s {
            Sign::Positive | Sign::Negative => {
                let h = (c_c - center).dot(axis);
                let sin_v = h / minor;
                let cos_v = (r_c - major) / minor;
                require_zero(
                    "props_rim_fit",
                    ((sin_v * sin_v + cos_v * cos_v).sqrt() - T::one()) * minor,
                    band,
                )?;
                rims.push(Rim {
                    d_u: t_sign::<T>(s) * trav(e.forward),
                    dt: e.t1 - e.t0,
                    level: (sin_v, cos_v),
                    tags: (e.start, e.end),
                });
            }
            Sign::Zero => {
                let w = c_c - center;
                let h = w.dot(axis);
                let rho = (w - axis * h).norm();
                require_zero(
                    "props_meridian_fit",
                    (rho - major).abs().max(h.abs()).max((r_c - minor).abs()),
                    band,
                )?;
                meridians.push(Meridian {
                    n_c,
                    c_c,
                    dt: e.t1 - e.t0,
                    anchor: e.p0(),
                    anchor_tag: e.tag_at_t0(),
                });
            }
        }
    }
    let du = du_of_rims(&rims, major, band)?;
    let Some(m0) = meridians.first() else {
        return Err(PropsError::NotIsoRectangle {
            what: "torus face without a meridian",
        });
    };
    // Chart orientation of the anchor meridian: v winds right-handed
    // about −τ̂ at its minor center.
    let w = m0.c_c - center;
    let rho_hat = (w - axis * w.dot(axis)).normalize();
    let tau = axis.cross(rho_hat);
    let orient = classify("props_meridian_orient", m0.n_c.dot(tau) * minor, band)?;
    if orient == Sign::Zero {
        return Err(PropsError::NotIsoRectangle {
            what: "torus meridian orientation degenerate",
        });
    }
    let dv_sign = -t_sign::<T>(orient);
    // Anchor latitude from the meridian's t0 endpoint.
    let wa = m0.anchor - center;
    let ha = wa.dot(axis);
    let rho_a = (wa - axis * ha).norm();
    let (sin_a, cos_a) = (ha / minor, (rho_a - major) / minor);
    require_extent(m0.dt * minor, band)?;
    let dv = m0.dt;
    // Normalize to the increasing interval [v0, v1]: rotate the anchor
    // latitude by the signed span where needed.
    let (sd, cd) = dv.sin_cos();
    let (s0, c0, s1, c1) = match orient {
        // orient Negative ⇒ dv_sign = +1: anchor is v0.
        Sign::Negative => {
            let s1 = sin_a * cd + cos_a * sd;
            let c1 = cos_a * cd - sin_a * sd;
            (sin_a, cos_a, s1, c1)
        }
        // orient Positive ⇒ dv_sign = −1: anchor is v1.
        Sign::Positive => {
            let s0 = sin_a * cd - cos_a * sd;
            let c0 = cos_a * cd + sin_a * sd;
            (s0, c0, sin_a, cos_a)
        }
        Sign::Zero => unreachable_zero(),
    };
    // Rim levels must sit at the interval ends.
    for rim in &rims {
        let (rs, rc) = rim.level;
        let d0 = ((rs - s0) * (rs - s0) + (rc - c0) * (rc - c0)).sqrt();
        let d1 = ((rs - s1) * (rs - s1) + (rc - c1) * (rc - c1)).sqrt();
        require_zero("props_rim_level", d0.min(d1) * minor, band)?;
    }
    // s_f: the rim topologically adjacent to the anchor endpoint; the
    // interior sweeps from it in the dv_sign direction.
    let Some(rim_a) = rims
        .iter()
        .find(|r| r.tags.0 == m0.anchor_tag || r.tags.1 == m0.anchor_tag)
    else {
        return Err(PropsError::NotIsoRectangle {
            what: "torus meridian anchor not on a rim",
        });
    };
    let s_f = rim_a.d_u * dv_sign;
    let half = T::from_f64(0.5);
    let area = minor * du * (major * dv + minor * (s1 - s0));
    let k = minor
        * du
        * ((major * major + minor * minor) * (s1 - s0)
            + major * minor * dv
            + (major * minor * half) * (dv + s1 * c1 - s0 * c0));
    let va = loop_vector_area(edges, center)?;
    let flux = s_f * k + (center - Point3::origin()).dot(va);
    Ok(FaceContribution { flux, area })
}

/// Documented-unreachable arm (the caller matched a definite sign);
/// returns poison values rather than panicking (D9).
fn unreachable_zero<T: Real>() -> (T, T, T, T) {
    let nan = T::from_f64(f64::NAN);
    (nan, nan, nan, nan)
}

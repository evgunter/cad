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

use geom_core::{Band, Decide, Length, Point3, Real, Sign, Vec3};
use geom_curves::Curve3;
use geom_surfaces::Surface;

use super::{FaceContribution, LoopEdge, PropsError, loop_vector_area};
use crate::dihedral::decide;

/// The flux and area of a curved face from its **outer** loop (curved
/// M2 faces carry no rings — the owning body refuses ringed curved
/// faces before calling). Dispatches on the surface kind; `band` is
/// the run's linear band, built once at operation entry.
///
/// `sense_sign` is the face's `±1` orientation sense (M5 S10,
/// `topo::Face::sense_sign`). It is **deliberately not applied to
/// every term**: `A⃗` and the rim-derived `s_f` are recovered from the
/// face's STORED LOOP TRAVERSAL, which the interior-left rule already
/// ties to the outward normal — `revert` reverses loops and flips
/// `sense` together, so multiplying those terms by the sense would
/// double-count and negate the volume twice. `sense_sign` is consumed
/// at exactly one place: the **rimless** sphere band, whose boundary
/// carries no rim to derive `s_f` from and which previously hardcoded
/// `s_f = +1` on the assumption that sweeps emit outward shells only.
/// That is the one orientation fact in this module the boundary does
/// not encode, so it is the one the bit must supply.
///
/// # Errors
///
/// [`PropsError`] — unimplemented kinds, out-of-inventory boundary
/// shapes, definite consistency failures, escalated classifications.
pub fn curved_face<T: Decide>(
    surface: &Surface<T>,
    outer: &[LoopEdge<T>],
    sense_sign: T,
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
        } => sphere(center, radius, axis, outer, sense_sign, band),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => torus(center, axis, major_radius, minor_radius, outer, band),
        Surface::Nurbs(_) => Err(PropsError::Unimplemented),
    }
}

/// The material side a curved face's **boundary traversal** encodes —
/// [`boundary_material_sign`]'s answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialSign {
    /// The boundary encodes the side: `Sign::Positive` ⇔ the stored
    /// outer-loop traversal places the material where a `sense: true`
    /// face's outward normal (`+chart_normal`) claims. Definite
    /// (`Positive`/`Negative`) by construction: every factor is a
    /// definite `classify` outcome or a stored traversal bool — the
    /// `Zero` arms refuse typed before a sign is minted.
    Encoded(Sign),
    /// The boundary does not encode the side: the **rimless sphere
    /// band** (M2 PR 5's axis-touching full revolve), the one analytic
    /// face whose flux sign the boundary cannot supply — its `s_f` IS
    /// `Face::sense`, a single encoding with nothing to cross-check
    /// against (the documented residual of the curved sense gate).
    Unencoded,
}

/// The flux derivations' material-side sign, factored public (M6-6):
/// which side of the chart normal the face's **stored outer-loop
/// traversal** says the material lies on — the boundary's own encoding
/// of the orientation fact `Face::sense` (M5 S10) also encodes. Tier
/// 3's curved check-6 arm compares the two encodings; this fn re-runs
/// the exact sub-derivations the flux lanes consume ([`s_f_from_rim`]
/// for cylinder/cone/rim-bearing sphere; anchor-rim traversal × chart
/// orientation for the torus), through the same already-length-metered
/// named decides (`props_rim_side`, `props_circle_axis_class`,
/// `props_meridian_orient`, …) — no new comparand, no new margin.
///
/// The derivation is chart-generic: with `n_chart = ∂u × ∂v`, the
/// interior-left rule makes "traversal direction on the extreme rim"
/// determine the material side on ANY nappe/latitude — which is why
/// the cone arm needs no nappe correction (the `v < 0` chart normal
/// negation and the physical-azimuth reversal cancel).
///
/// # Errors
///
/// [`PropsError`] exactly as the flux lanes: unimplemented kinds
/// (NURBS), out-of-inventory boundary shapes (including conic-trimmed
/// faces the quadrature lane owns), escalated classifications,
/// degenerate faces. Gating callers MUST treat an error as exempt,
/// never as disagreement (the check-7 posture).
pub fn boundary_material_sign<T: Decide>(
    surface: &Surface<T>,
    outer: &[LoopEdge<T>],
    band: Band,
) -> Result<MaterialSign, PropsError> {
    let no_rim = || PropsError::NotIsoRectangle {
        what: "curved face without a rim (non-sphere)",
    };
    match *surface {
        Surface::Plane { .. } => Err(PropsError::NotIsoRectangle {
            what: "boundary_material_sign called on a plane (planar orientation is the \
                   check-6 Newell winding, not a props derivation)",
        }),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let (rims, levels) = cylinder_boundary(origin, axis, radius, outer, band)?;
            let (lo, hi) = min_max(&levels)?;
            let rim = rims.first().ok_or_else(no_rim)?;
            Ok(MaterialSign::Encoded(s_f_from_rim(
                rim, lo, hi, radius, band,
            )?))
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let (sin_a, cos_a) = half_angle.sin_cos();
            let (rims, levels) = cone_boundary(apex, axis, sin_a, cos_a, outer, band)?;
            let (lo, hi) = min_max(&levels)?;
            let arm = cone_arm(&rims, sin_a);
            let rim = rims.first().ok_or_else(no_rim)?;
            Ok(MaterialSign::Encoded(s_f_from_rim(rim, lo, hi, arm, band)?))
        }
        Surface::Sphere {
            center,
            radius,
            axis,
            ..
        } => {
            let (rims, _meridian_axes, levels) =
                sphere_boundary(center, radius, axis, outer, band)?;
            let Some(rim) = rims.first() else {
                return Ok(MaterialSign::Unencoded);
            };
            let (lo, hi) = min_max(&levels)?;
            Ok(MaterialSign::Encoded(s_f_from_rim(
                rim, lo, hi, radius, band,
            )?))
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let (rims, meridians) =
                torus_boundary(center, axis, major_radius, minor_radius, outer, band)?;
            let m0 = meridians.first().ok_or(PropsError::NotIsoRectangle {
                what: "torus face without a meridian",
            })?;
            let orient = torus_meridian_orient(m0, center, axis, minor_radius, band)?;
            let rim_a = torus_anchor_rim(&rims, m0)?;
            Ok(MaterialSign::Encoded(sign_mul(
                rim_a.d_u_sign,
                orient.flip(),
            )))
        }
        Surface::Nurbs(_) => Err(PropsError::Unimplemented),
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

/// The rim's discrete `u`-traversal direction: a definite circle
/// axis class, reversed when the loop traverses the edge backward.
/// Definite in ⇒ definite out (`flip` fixes only `Zero`).
fn rim_dir(s: Sign, forward: bool) -> Sign {
    if forward { s } else { s.flip() }
}

/// The product of two discrete signs (`Zero` absorbs — unreachable
/// here: every producer feeding this is definite, but the arm stays
/// total rather than panicking, D9).
fn sign_mul(a: Sign, b: Sign) -> Sign {
    match b {
        Sign::Positive => a,
        Sign::Negative => a.flip(),
        Sign::Zero => Sign::Zero,
    }
}

/// Funnel wrapper: classify, mapping an escalation to the typed
/// [`PropsError::Escalated`].
fn classify<T: Decide>(
    name: &'static str,
    margin: Length<T>,
    band: Band,
) -> Result<Sign, PropsError> {
    decide(name, margin, band).map_err(|cause| PropsError::Escalated { cause })
}

/// Require a consistency residual to be coincident with zero.
fn require_zero<T: Decide>(
    name: &'static str,
    margin: Length<T>,
    band: Band,
) -> Result<(), PropsError> {
    match classify(name, margin, band)? {
        Sign::Zero => Ok(()),
        Sign::Positive | Sign::Negative => Err(PropsError::NotIsoRectangle { what: name }),
    }
}

/// Require a definitely-positive extent (degenerate ⇒ typed error).
fn require_extent<T: Decide>(margin: Length<T>, band: Band) -> Result<(), PropsError> {
    match classify("props_face_extent", margin, band)? {
        Sign::Positive => Ok(()),
        Sign::Zero | Sign::Negative => Err(PropsError::DegenerateFace),
    }
}

/// Rim incidence on its surface of revolution (M2 PR 7 review F1:
/// shape checks alone — radius fit, definite axis class — accept
/// circles nowhere on the surface). Certifies, with `w` the rim center
/// minus the surface's axis origin:
///
/// - carrier axis **parallel** to the surface axis (the definite axis
///   class only pins `n_c·â ≠ 0`): margin `‖n_c × â‖·r_c` — the
///   tilt angle metered at the rim radius (lever arm `r_c`);
/// - rim center **on the axis line**: margin `‖w − â(w·â)‖`, the
///   center's perpendicular offset from the axis (already meters).
///
/// Together with the per-surface radius/level fit these pin the rim
/// circle pointwise onto the surface.
fn require_rim_incidence<T: Decide>(
    w: Vec3<T>,
    n_c: Vec3<T>,
    r_c: T,
    axis: Vec3<T>,
    band: Band,
) -> Result<(), PropsError> {
    require_zero(
        "props_rim_axis_parallel",
        Length::levered(n_c.cross(axis).norm(), r_c),
        band,
    )?;
    require_zero(
        "props_rim_center_on_axis",
        Length::norm3(w - axis * w.dot(axis)),
        band,
    )
}

/// A rim's iso-level, carrying its own DIMENSION so every grouping
/// comparand is metered per kind rather than by one uniform
/// expression. The ratified ε semantics (D4) make every `classify`
/// comparand a LENGTH — the point deviation the difference induces —
/// and the level payload's dimension varies by surface kind, so the
/// metering choice is forced here at the constructor, not left to
/// convention at the comparison site. (The uniform `× arm` this
/// replaces turned a cone's already-length level difference into an
/// AREA — two lengths multiplied — and shrank the mm-scale
/// `cone_trunc` rim separation into the ε = 1e-7 ambiguity band: the
/// project's first in-band K landing, #89.)
#[derive(Clone, Copy)]
enum RimLevel<T: Real> {
    /// Cylinder/cone: the level is the axial/slant arc length `v`
    /// itself, in meters — a difference is ALREADY the point
    /// deviation and reaches `classify` bare.
    Length(T),
    /// Sphere/torus: a dimensionless direction pair — `(sin v, 0)`
    /// for the sphere, `(sin v, cos v)` for the torus — whose
    /// componentwise differences need `× arm` (the lever arm, meters)
    /// to become point deviations.
    Unit(T, T),
}

/// A classified rim: signed `u`-traversal direction (`d_u`), parameter
/// span (`dt`, the face's `Δu` candidate), and its iso-level payload.
struct Rim<T: Real> {
    /// ±1: traversal direction in `u` = sign(carrier axis · surface
    /// axis) × traversal direction. The scalar image of `d_u_sign`
    /// (`t_sign`), kept as `T` for the margin arithmetic it feeds
    /// (the `du_of_rims` group keys).
    d_u: T,
    /// The same traversal direction as a **discrete** definite sign —
    /// the combinatorial channel [`boundary_material_sign`] reads, so
    /// the material-side cross-check compares two exact ±1s without
    /// comparing scalars (both factors are discrete at origin: a
    /// definite `classify` outcome × the stored traversal bool).
    d_u_sign: Sign,
    /// Carrier parameter span `t1 − t0` (angle-true; `Δu`).
    dt: T,
    /// The rim's iso-level, dimension carried by the variant (see
    /// [`RimLevel`] and the per-surface call sites).
    level: RimLevel<T>,
    /// Traversal-order endpoint tags.
    tags: (u32, u32),
}

/// Whether two rim levels are coincident, through the funnel: each
/// margin is the point deviation in meters — bare for [`RimLevel::
/// Length`] (the difference IS a length), `× arm` for
/// [`RimLevel::Unit`] (dimensionless components at the lever arm).
fn same_level<T: Decide>(
    a: RimLevel<T>,
    b: RimLevel<T>,
    arm: T,
    band: Band,
) -> Result<bool, PropsError> {
    match (a, b) {
        (RimLevel::Length(la), RimLevel::Length(lb)) => {
            Ok(classify("props_rim_level_group", Length::of(la - lb), band)? == Sign::Zero)
        }
        (RimLevel::Unit(sa, ca), RimLevel::Unit(sb, cb)) => {
            let d0 = classify("props_rim_level_group", Length::levered(sa - sb, arm), band)?;
            let d1 = classify("props_rim_level_group", Length::levered(ca - cb, arm), band)?;
            Ok(d0 == Sign::Zero && d1 == Sign::Zero)
        }
        // One surface builds every rim of a face, so mixed kinds are
        // structurally impossible; a poisoned margin turns it into a
        // typed escalation rather than a panic (D9).
        _ => {
            classify(
                "props_rim_level_group",
                Length::of(T::from_f64(f64::NAN)),
                band,
            )?;
            Ok(false)
        }
    }
}

/// Check all rims agree on `Δu`; returns the face's `Δu`. `arm` is
/// the azimuthal lever arm (meters), metering the DIMENSIONLESS
/// margins here (`Unit` level components, the ±1 traversal-direction
/// difference, the `Δu` angle difference); `Length` level margins are
/// already meters and never touch it.
fn du_of_rims<T: Decide>(rims: &[Rim<T>], arm: T, band: Band) -> Result<T, PropsError> {
    if rims.is_empty() {
        return Err(PropsError::NotIsoRectangle {
            what: "curved face without a rim (non-sphere)",
        });
    }
    // Group arcs by (rim LEVEL, traversal direction) first (M5 PR 9):
    // a re-merged or boolean-cut wall's rim legitimately arrives as
    // SEVERAL arcs per level (a rim run of two arcs after a strut
    // kev), so the face's Δu is the per-group SUM of spans, required
    // consistent ACROSS groups — the pre-PR-9 first-arc rule silently
    // undercounted multi-arc rims. Direction joins the key so the
    // degenerate zero-extent patch (both rims one level, opposite
    // traversal) keeps its M2 verdict downstream.
    let mut groups: Vec<(RimLevel<T>, T, T)> = Vec::new(); // (level, d_u, dt sum)
    for rim in rims {
        let mut placed = false;
        for g in &mut groups {
            let same = same_level(rim.level, g.0, arm, band)?;
            let same_dir = classify(
                "props_rim_dir_group",
                Length::levered(rim.d_u - g.1, arm),
                band,
            )? == Sign::Zero;
            if same && same_dir {
                g.2 = g.2 + rim.dt;
                placed = true;
                break;
            }
        }
        if !placed {
            groups.push((rim.level, rim.d_u, rim.dt));
        }
    }
    let total = groups[0].2;
    for g in &groups[1..] {
        require_zero(
            "props_du_consistent",
            Length::levered(g.2 - total, arm),
            band,
        )?;
    }
    Ok(total)
}

/// `s_f` for a linearly-leveled surface (cylinder/cone/sphere): from
/// `rim`'s level and the face's level range. Interior lies toward the
/// opposite extreme; a rim strictly inside the range cannot happen on
/// an iso-rectangle (the residual would be the full extent, definite).
/// The margin is metered by the level's own dimension ([`RimLevel`]):
/// bare for `Length` levels (meters already), `× arm` for the
/// dimensionless `Unit` primary component.
///
/// Returns the **discrete** sign (definite by construction: a definite
/// `props_rim_side` outcome × the rim's definite traversal direction);
/// flux callers scalarize through `t_sign`, the material-side gate
/// ([`boundary_material_sign`]) consumes it combinatorially.
fn s_f_from_rim<T: Decide>(
    rim: &Rim<T>,
    lo: T,
    hi: T,
    arm: T,
    band: Band,
) -> Result<Sign, PropsError> {
    let margin = match rim.level {
        RimLevel::Length(v) => Length::of(lo + hi - v - v),
        RimLevel::Unit(s, _) => Length::levered(lo + hi - s - s, arm),
    };
    match classify("props_rim_side", margin, band)? {
        Sign::Positive => Ok(rim.d_u_sign),
        Sign::Negative => Ok(rim.d_u_sign.flip()),
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
    let (rims, levels) = cylinder_boundary(origin, axis, radius, edges, band)?;
    let du = du_of_rims(&rims, radius, band)?;
    let (lo, hi) = min_max(&levels)?;
    require_extent(Length::of(hi - lo), band)?;
    // `radius` is the azimuthal lever arm; the rim-side margin itself
    // is Length-leveled (meters) and never touches it.
    let s_f = t_sign::<T>(s_f_from_rim(&rims[0], lo, hi, radius, band)?);
    let area = radius * du * (hi - lo);
    let va = loop_vector_area(edges, origin)?;
    let flux = s_f * (radius * area) + (origin - Point3::origin()).dot(va);
    Ok(FaceContribution { flux, area })
}

/// Classify a cylinder face's boundary into (rims, iso-levels) — the
/// shared parse consumed by both the flux closed form and
/// [`boundary_material_sign`].
fn cylinder_boundary<T: Decide>(
    origin: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<(Vec<Rim<T>>, Vec<T>), PropsError> {
    let mut rims: Vec<Rim<T>> = Vec::new();
    let mut levels: Vec<T> = Vec::new();
    for e in edges {
        match e.carrier {
            Curve3::Line { dir, .. } => {
                require_zero(
                    "props_meridian_axial",
                    Length::levered(dir.cross(axis).norm(), e.t1 - e.t0),
                    band,
                )?;
                // Incidence: the (certified-axial) line lies on the
                // cylinder iff one of its points does — radial distance
                // of the interval start from the axis vs the radius
                // (meters).
                let w0 = e.p0() - origin;
                require_zero(
                    "props_meridian_on_surface",
                    Length::of((w0 - axis * w0.dot(axis)).norm() - radius),
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
                let s = classify(
                    "props_circle_axis_class",
                    Length::levered(n_c.dot(axis), r_c),
                    band,
                )?;
                if s == Sign::Zero {
                    return Err(PropsError::NotIsoRectangle {
                        what: "cylinder boundary circle is not a rim",
                    });
                }
                require_zero("props_rim_fit", Length::of(r_c - radius), band)?;
                require_rim_incidence(center - origin, n_c, r_c, axis, band)?;
                let v = (center - origin).dot(axis);
                rims.push(Rim {
                    d_u: t_sign::<T>(rim_dir(s, e.forward)),
                    d_u_sign: rim_dir(s, e.forward),
                    dt: e.t1 - e.t0,
                    // The axial arc length itself — meters.
                    level: RimLevel::Length(v),
                    tags: (e.start, e.end),
                });
                levels.push(v);
            }
            // An ellipse arc on a wall boundary (a curved cut, M5
            // PR 5) breaks the iso-rectangle patch shape THIS pass
            // requires. The PR 11 quadrature lane handles it — but it
            // needs the body's stored pcurves, so it lives one layer
            // up (`topo::mass_properties` routes conic-trimmed
            // cylinder faces there BEFORE this closed form runs); a
            // direct key-free call keeps the typed refusal.
            Curve3::Ellipse { .. } => {
                return Err(PropsError::NotIsoRectangle {
                    what: "cylinder boundary carries an ellipse arc (curved cut) — route                            through topo::mass_properties, whose PR 11 quadrature lane                            consumes the stored pcurves this key-free pass cannot see",
                });
            }
            Curve3::Nurbs(_) => return Err(PropsError::Unimplemented),
        }
    }
    Ok((rims, levels))
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
    let (rims, levels) = cone_boundary(apex, axis, sin_a, cos_a, edges, band)?;
    let arm = cone_arm(&rims, sin_a);
    let du = du_of_rims(&rims, arm, band)?;
    let (lo, hi) = min_max(&levels)?;
    require_extent(Length::of(hi - lo), band)?;
    // Single-nappe check: definitely-negative low AND definitely-positive
    // high would straddle the apex through both nappes.
    let s_lo = classify("props_cone_nappe", Length::of(lo), band)?;
    let s_hi = classify("props_cone_nappe", Length::of(hi), band)?;
    if s_lo == Sign::Negative && s_hi == Sign::Positive {
        return Err(PropsError::NappeSpanning);
    }
    let half = T::from_f64(0.5);
    let area = sin_a * du * ((hi.powi(2) - lo.powi(2)) * half).abs();
    let va = loop_vector_area(edges, apex)?;
    let flux = (apex - Point3::origin()).dot(va);
    Ok(FaceContribution { flux, area })
}

/// Classify a cone face's boundary into (rims, signed slant levels) —
/// the shared parse consumed by both the flux closed form and
/// [`boundary_material_sign`].
fn cone_boundary<T: Decide>(
    apex: Point3<T>,
    axis: Vec3<T>,
    sin_a: T,
    cos_a: T,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<(Vec<Rim<T>>, Vec<T>), PropsError> {
    let mut rims: Vec<Rim<T>> = Vec::new();
    let mut levels: Vec<T> = Vec::new();
    for e in edges {
        match e.carrier {
            Curve3::Line { dir, .. } => {
                require_zero(
                    "props_meridian_generator",
                    Length::levered(dir.dot(axis).abs() - cos_a, e.t1 - e.t0),
                    band,
                )?;
                // Incidence: a line at the generator angle is a
                // generator iff it passes through the apex — the
                // apex-to-line distance `‖(apex − p) × dir‖` (`dir`
                // unit, so meters directly).
                require_zero(
                    "props_meridian_apex",
                    Length::norm3((apex - e.p0()).cross(dir)),
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
                let s = classify(
                    "props_circle_axis_class",
                    Length::levered(n_c.dot(axis), r_c),
                    band,
                )?;
                if s == Sign::Zero {
                    return Err(PropsError::NotIsoRectangle {
                        what: "cone boundary circle is not a rim",
                    });
                }
                let v = (center - apex).dot(axis) / cos_a;
                require_zero("props_rim_fit", Length::of(r_c - v.abs() * sin_a), band)?;
                require_rim_incidence(center - apex, n_c, r_c, axis, band)?;
                rims.push(Rim {
                    d_u: t_sign::<T>(rim_dir(s, e.forward)),
                    d_u_sign: rim_dir(s, e.forward),
                    dt: e.t1 - e.t0,
                    // The signed slant arc length itself — meters.
                    level: RimLevel::Length(v),
                    tags: (e.start, e.end),
                });
                levels.push(v);
            }
            Curve3::Ellipse { .. } => {
                return Err(PropsError::NotIsoRectangle {
                    what: "cone boundary carries an ellipse arc (a tilted-section cut) — the \
                           class has no cone-chart image at all (azimuth-non-harmonic, \
                           M6-3, and no ring-computable fitted certificate either), so \
                           the quadrature lane has nothing to consume",
                });
            }
            Curve3::Nurbs(_) => return Err(PropsError::Unimplemented),
        }
    }
    Ok((rims, levels))
}

/// The cone's azimuthal lever arm: the first rim's own radius
/// `|v|·sin α` (cone rims are `Length`-leveled, so the arm meters only
/// the dimensionless direction/Δu margins in `du_of_rims`). The
/// `T::one()` fallback covers the no-rim case, where `du_of_rims`
/// refuses before any margin is metered.
fn cone_arm<T: Real>(rims: &[Rim<T>], sin_a: T) -> T {
    match rims.first() {
        Some(Rim {
            level: RimLevel::Length(v),
            ..
        }) => v.abs() * sin_a,
        _ => T::one(),
    }
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
/// and `Δu = π` **by construction**. Its `s_f` is the **only** flux
/// sign in this module that the boundary does not encode — with no rim
/// there is no traversal to read it off, and a rimless band's two
/// meridians are traversed the same way whichever side is material.
/// Before M5 S10 it was hardcoded `+1`, justified by "M2 sweeps emit
/// single outward shells only, so a rimless band with inward
/// orientation is unrepresentable at rest". S10 makes that
/// representable — `Face::sense` is exactly the missing bit — so the
/// hardcode becomes `s_f = sense_sign`. Identical for every face this
/// build mints (all `sense: true`); the difference is that an inward
/// rimless band is no longer silently metered as outward.
fn sphere<T: Decide>(
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    edges: &[LoopEdge<T>],
    sense_sign: T,
    band: Band,
) -> Result<FaceContribution<T>, PropsError> {
    let (rims, meridian_axes, levels) = sphere_boundary(center, radius, axis, edges, band)?;
    let (du, s_f);
    let (lo, hi) = min_max(&levels)?;
    require_extent(Length::levered(hi - lo, radius), band)?;
    if rims.is_empty() {
        // Two-band face (module docs above): meridians coplanar, Δu = π.
        let Some((&first, rest)) = meridian_axes.split_first() else {
            return Err(PropsError::NotIsoRectangle {
                what: "sphere face with an empty boundary",
            });
        };
        for &n in rest {
            require_zero(
                "props_band_coplanar",
                Length::levered(n.cross(first).norm(), radius),
                band,
            )?;
        }
        du = T::pi();
        // The one orientation fact no rim encodes (see the fn docs):
        // the face's sense IS `s_f` here, not a cross-check of it.
        s_f = sense_sign;
    } else {
        du = du_of_rims(&rims, radius, band)?;
        s_f = t_sign::<T>(s_f_from_rim(&rims[0], lo, hi, radius, band)?);
    }
    let area = radius.powi(2) * du * (hi - lo);
    let va = loop_vector_area(edges, center)?;
    let flux = s_f * (radius * area) + (center - Point3::origin()).dot(va);
    Ok(FaceContribution { flux, area })
}

/// Classify a sphere face's boundary into (rims, meridian great-circle
/// axes, latitude-sine levels) — the shared parse consumed by both the
/// flux closed form and [`boundary_material_sign`].
#[allow(clippy::type_complexity)]
fn sphere_boundary<T: Decide>(
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<(Vec<Rim<T>>, Vec<Vec3<T>>, Vec<T>), PropsError> {
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
                Curve3::Nurbs(_) => PropsError::Unimplemented,
                _ => PropsError::NotIsoRectangle {
                    what: "sphere boundary edge is not a circle",
                },
            });
        };
        let s = classify(
            "props_circle_axis_class",
            Length::levered(n_c.dot(axis), r_c),
            band,
        )?;
        match s {
            Sign::Positive | Sign::Negative => {
                let w = c_c - center;
                require_zero(
                    "props_rim_fit",
                    Length::of((w.norm_squared() + r_c.powi(2)).sqrt() - radius),
                    band,
                )?;
                // Incidence: the fit above only fixes ‖w‖; the offset
                // must also point ALONG the axis (w ∥ â) with the
                // carrier axis parallel — together they place the
                // circle on the sphere as the iso-v rim.
                require_rim_incidence(w, n_c, r_c, axis, band)?;
                let sin_v = w.dot(axis) / radius;
                rims.push(Rim {
                    d_u: t_sign::<T>(rim_dir(s, e.forward)),
                    d_u_sign: rim_dir(s, e.forward),
                    dt: e.t1 - e.t0,
                    // Dimensionless latitude sine — levered by R.
                    level: RimLevel::Unit(sin_v, T::zero()),
                    tags: (e.start, e.end),
                });
                levels.push(sin_v);
            }
            Sign::Zero => {
                // Meridian great circle: centered at the sphere center
                // with the sphere's radius.
                require_zero(
                    "props_meridian_great",
                    Length::of((c_c - center).norm().max((r_c - radius).abs())),
                    band,
                )?;
                meridian_axes.push(n_c);
                levels.push((e.p0() - center).dot(axis) / radius);
                levels.push((e.p1() - center).dot(axis) / radius);
            }
        }
    }
    Ok((rims, meridian_axes, levels))
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
    let (rims, meridians) = torus_boundary(center, axis, major, minor, edges, band)?;
    let du = du_of_rims(&rims, major, band)?;
    let Some(m0) = meridians.first() else {
        return Err(PropsError::NotIsoRectangle {
            what: "torus face without a meridian",
        });
    };
    let orient = torus_meridian_orient(m0, center, axis, minor, band)?;
    // Anchor latitude from the meridian's t0 endpoint.
    let wa = m0.anchor - center;
    let ha = wa.dot(axis);
    let rho_a = (wa - axis * ha).norm();
    let (sin_a, cos_a) = (ha / minor, (rho_a - major) / minor);
    require_extent(Length::levered(m0.dt, minor), band)?;
    let dv = m0.dt;
    // Normalize to the increasing interval [v0, v1]: rotate the anchor
    // latitude by the signed span where needed.
    let (sd, cd) = dv.sin_cos();
    let (s0, c0, s1, c1) = match orient {
        // orient Negative ⇒ dv/dt = +1: anchor is v0.
        Sign::Negative => {
            let s1 = sin_a * cd + cos_a * sd;
            let c1 = cos_a * cd - sin_a * sd;
            (sin_a, cos_a, s1, c1)
        }
        // orient Positive ⇒ dv/dt = −1: anchor is v1.
        Sign::Positive => {
            let s0 = sin_a * cd - cos_a * sd;
            let c0 = cos_a * cd + sin_a * sd;
            (s0, c0, sin_a, cos_a)
        }
        Sign::Zero => unreachable_zero(),
    };
    // Rim levels must sit at the interval ends.
    for rim in &rims {
        // Torus rims are minted `Unit` a page up; the refusal arm is
        // structurally unreachable but stays typed, never a panic (D9).
        let RimLevel::Unit(rs, rc) = rim.level else {
            return Err(PropsError::NotIsoRectangle {
                what: "torus rim carries a non-angular level",
            });
        };
        // powi(2), not x*x: the interval square is tight and
        // nonnegative, so the sqrt stays fully in-domain even when the
        // difference encloses zero (an x*x interval product has a
        // negative lower bound there, and the domain-clamped sqrt's
        // decoration would poison the margin — found live on the
        // interval-lane donut).
        let d0 = ((rs - s0).powi(2) + (rc - c0).powi(2)).sqrt();
        let d1 = ((rs - s1).powi(2) + (rc - c1).powi(2)).sqrt();
        require_zero("props_rim_level", Length::levered(d0.min(d1), minor), band)?;
    }
    // s_f: the rim topologically adjacent to the anchor endpoint; the
    // interior sweeps from it in the `dv/dt = −orient` direction.
    let rim_a = torus_anchor_rim(&rims, m0)?;
    let s_f = t_sign::<T>(sign_mul(rim_a.d_u_sign, orient.flip()));
    let half = T::from_f64(0.5);
    let area = minor * du * (major * dv + minor * (s1 - s0));
    let k = minor
        * du
        * ((major.powi(2) + minor.powi(2)) * (s1 - s0)
            + major * minor * dv
            + (major * minor * half) * (dv + s1 * c1 - s0 * c0));
    let va = loop_vector_area(edges, center)?;
    let flux = s_f * k + (center - Point3::origin()).dot(va);
    Ok(FaceContribution { flux, area })
}

/// A torus minor-circle boundary edge (iso-`u` meridian): its carrier
/// frame, stored parameter span, and traversal-start anchor.
struct TorusMeridian<T: Real> {
    n_c: Vec3<T>,
    c_c: Point3<T>,
    dt: T,
    anchor: Point3<T>,
    anchor_tag: u32,
}

/// Classify a torus face's boundary into (rims, meridians) — the
/// shared parse consumed by both the flux closed form and
/// [`boundary_material_sign`].
#[allow(clippy::type_complexity)]
fn torus_boundary<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    major: T,
    minor: T,
    edges: &[LoopEdge<T>],
    band: Band,
) -> Result<(Vec<Rim<T>>, Vec<TorusMeridian<T>>), PropsError> {
    let mut rims: Vec<Rim<T>> = Vec::new();
    let mut meridians: Vec<TorusMeridian<T>> = Vec::new();
    for e in edges {
        let Curve3::Circle {
            center: c_c,
            axis: n_c,
            radius: r_c,
            ..
        } = e.carrier
        else {
            return Err(match e.carrier {
                Curve3::Nurbs(_) => PropsError::Unimplemented,
                _ => PropsError::NotIsoRectangle {
                    what: "torus boundary edge is not a circle",
                },
            });
        };
        let s = classify(
            "props_circle_axis_class",
            Length::levered(n_c.dot(axis), r_c),
            band,
        )?;
        match s {
            Sign::Positive | Sign::Negative => {
                let h = (c_c - center).dot(axis);
                let sin_v = h / minor;
                let cos_v = (r_c - major) / minor;
                require_zero(
                    "props_rim_fit",
                    Length::levered((sin_v.powi(2) + cos_v.powi(2)).sqrt() - T::one(), minor),
                    band,
                )?;
                require_rim_incidence(c_c - center, n_c, r_c, axis, band)?;
                rims.push(Rim {
                    d_u: t_sign::<T>(rim_dir(s, e.forward)),
                    d_u_sign: rim_dir(s, e.forward),
                    dt: e.t1 - e.t0,
                    // Dimensionless minor-angle direction pair.
                    level: RimLevel::Unit(sin_v, cos_v),
                    tags: (e.start, e.end),
                });
            }
            Sign::Zero => {
                let w = c_c - center;
                let h = w.dot(axis);
                let rho = (w - axis * h).norm();
                require_zero(
                    "props_meridian_fit",
                    Length::of((rho - major).abs().max(h.abs()).max((r_c - minor).abs())),
                    band,
                )?;
                // Incidence: the minor circle's plane must CONTAIN the
                // torus axis direction — its normal `n_c` has no radial
                // component (the definite `n_c·τ̂` orientation check
                // below only excludes n_c ⊥ τ̂). Margin
                // `n_c·(w − âh) = (n_c·ρ̂)·ρ`: the tilt metered at the
                // tube-center distance (lever arm ρ ≈ R, meters).
                require_zero(
                    "props_meridian_plane",
                    Length::of(n_c.dot(w - axis * h)),
                    band,
                )?;
                meridians.push(TorusMeridian {
                    n_c,
                    c_c,
                    dt: e.t1 - e.t0,
                    anchor: e.p0(),
                    anchor_tag: e.tag_at_t0(),
                });
            }
        }
    }
    Ok((rims, meridians))
}

/// Chart orientation of the anchor meridian: `v` winds right-handed
/// about `−τ̂` at its minor center, so `dv/dt = −orient`. Definite by
/// construction (the `Zero` arm refuses typed).
fn torus_meridian_orient<T: Decide>(
    m0: &TorusMeridian<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    minor: T,
    band: Band,
) -> Result<Sign, PropsError> {
    let w = m0.c_c - center;
    let rho_hat = (w - axis * w.dot(axis)).normalize();
    let tau = axis.cross(rho_hat);
    let orient = classify(
        "props_meridian_orient",
        Length::levered(m0.n_c.dot(tau), minor),
        band,
    )?;
    if orient == Sign::Zero {
        return Err(PropsError::NotIsoRectangle {
            what: "torus meridian orientation degenerate",
        });
    }
    Ok(orient)
}

/// The rim topologically adjacent to the anchor meridian's `t0`
/// endpoint — the rim the torus `s_f` derivation reads (the interior
/// sweeps from it in the `dv/dt` direction).
fn torus_anchor_rim<'a, T: Real>(
    rims: &'a [Rim<T>],
    m0: &TorusMeridian<T>,
) -> Result<&'a Rim<T>, PropsError> {
    rims.iter()
        .find(|r| r.tags.0 == m0.anchor_tag || r.tags.1 == m0.anchor_tag)
        .ok_or(PropsError::NotIsoRectangle {
            what: "torus meridian anchor not on a rim",
        })
}

/// Documented-unreachable arm (the caller matched a definite sign);
/// returns poison values rather than panicking (D9).
fn unreachable_zero<T: Real>() -> (T, T, T, T) {
    let nan = T::from_f64(f64::NAN);
    (nan, nan, nan, nan)
}

// ADVERSARIAL REVIEW PROBE (authored on branch review/rim-dim,
// adopted by merge — authorship kept).
#[cfg(test)]
#[allow(clippy::expect_used)]
mod rim_level_review_probe {
    use super::*;

    /// The structurally-impossible mixed-kind arm must escalate typed
    /// (poisoned classify), never panic and never answer false.
    #[test]
    fn mixed_kind_levels_escalate_typed() {
        let band = Band::linear().expect("band");
        let got = same_level(
            RimLevel::Length(1.0_f64),
            RimLevel::Unit(0.5, 0.5),
            1.0,
            band,
        );
        assert!(got.is_err(), "mixed kinds must poison typed: {got:?}");
    }
}

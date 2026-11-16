//! Per-triangle closed-form deviation certificates — the
//! certified-conservative half of the chordal promise (crate docs).
//!
//! Each bound is a statement in exact arithmetic about the triangle's
//! vertex data; every emitted triangle must certify ≤ δ or
//! tessellation fails loudly. Derivations (also in the PR log):
//!
//! - **Cylinder** (radius r): for p = Σλᵢxᵢ with |xᵢ⊥| = r, convexity
//!   gives |p⊥| ≤ r (no outward deviation) and
//!   dist(p, cyl) = r − |p⊥| ≤ r − dist(axis, T). Exact line–triangle
//!   distance ⇒ certificate `r − dist(axis, T)`.
//! - **Sphere** (center c, radius r): vertices on the sphere put the
//!   triangle inside the ball; dist(p, sphere) = r − |p − c| ≤
//!   r − dist(c, T). Certificate `r − dist(c, T)` (exact
//!   point–triangle distance). Handles pole-fan triangles with no UV
//!   involvement.
//! - **Cone** (half-angle α): the flat triangle's own deviation, in
//!   closed form: the largest sag of its three chords, or its height
//!   over the axis where the axis passes through it ([`cert_cone`]
//!   carries the derivation). It reads each chord at its own two radii
//!   and its own azimuth difference, so a chord from a small radius to a
//!   large one is charged what it sags, which goes as the smaller.
//! - **Torus** (R, r): the doubly-curved interpolation bound
//!   `crate::sizing::torus_grid_steps` derives and inverts (its (★)):
//!   with `Δu`, `Δv` the triangle's UV extents and the second-partial
//!   sups over the triangle's own φ range — `A = max |R + r·cos φ|`
//!   (the absolute value is what keeps it a sup on a spindle torus,
//!   `R < r`, where `R + r·cos φ` crosses zero), `B = r·max |sin φ|`,
//!   `C = r` — Taylor with the integral remainder
//!   at each point, Popoviciu's variance bound and Cauchy–Schwarz give
//!   `‖S − ΠS‖ ≤ (A·Δu² + 2B·Δu·Δv + C·Δv²)/8`. Per direction, and
//!   attained: on the outer equator (φ = 0, where `P_θθ ∥ P_φφ` and
//!   `P_θφ = 0`) a cell-half's hypotenuse midpoint deviates by exactly
//!   this to second order, measured below. Certificate
//!   `(A·Δu² + 2B·Δu·Δv + C·Δv²)/8`.
//! - **Spindle sheet**: the torus's, with an apex corner's `u` read as
//!   another corner's ([`cert_spindle`]): the apex is one point for
//!   every `u`, so the triangle is the same in space, and its
//!   parametric patch spans only the other corners' `u`.
//! - **Plane**: affine — deviation 0, no certificate needed.
//!
//! Two additive slacks sit outside all bounds, documented in the crate
//! docs: boundary vertices carry the kernel's ≤ ε carrier residual
//! (they lie on certified carriers, not exactly on the surface), and
//! f64 rounding of the evaluations. Sizing takes no margin for them
//! (`crates/mesh/README.md` `[mesh.sizing]`), so a lane's worst
//! triangle can certify at δ to the last bit. The verdict therefore
//! charges a certificate's own rounding to the promise's "+ rounding"
//! rather than to δ: [`verdict`] compares a bound against δ plus
//! [`rounding_allowance`] of the magnitude the bound was computed
//! from ([`position_scale`], [`uv_scale`], [`cone_scale`]), and refuses
//! to absorb an allowance that is not small against δ and ε.

use crate::sizing::Eps;
use geom_core::{Point3, Vec3};

/// **The distance from `p` to the triangle `abc`, for every triangle**:
/// one of no area included, where it is the distance to the nearest
/// side. It is a function of the three corners as a set (they are put in
/// one order first), so no corner order answers differently.
///
/// The nearest point of a triangle is the foot of `p` on its plane where
/// that lies inside, and on a side otherwise. With `s` the distance to
/// the nearest side and `ρ = 2·area / perimeter` the inradius, the
/// distance is in `[s − ρ, s]` either way, since no point of a triangle
/// is further than `ρ` from its sides, and the foot's distance is read
/// inside that bracket. So a normal that rounding has left no direction
/// (a triangle thin against the rounding of its own corners) cannot move
/// the answer by more than the triangle's own width, and a triangle of no
/// area answers `s` exactly. The normal is the cross product of the two
/// shorter sides, each a difference of stored corners: on a needle those
/// are near square to each other, and the product is exact to its own
/// rounding.
///
/// A coordinate that is not finite, or corners so far from `p` that the
/// fourth power of that distance is past `f64`'s range, answers NaN.
pub fn dist_point_triangle(p: Point3<f64>, a: Point3<f64>, b: Point3<f64>, c: Point3<f64>) -> f64 {
    let mut corners = [a, b, c];
    corners.sort_by(|s, t| {
        s.x.total_cmp(&t.x)
            .then(s.y.total_cmp(&t.y))
            .then(s.z.total_cmp(&t.z))
    });
    let [a, b, c] = corners;
    let (u, v, w) = (a - p, b - p, c - p);
    // Every product below is at most `192·m⁴`, `m` the largest
    // coordinate of a corner from `p`.
    let reach = u.dot(u) + v.dot(v) + w.dot(w);
    if !(16.0 * reach).powi(2).is_finite() {
        return f64::NAN;
    }
    let (ab, bc, ca) = (b - a, c - b, a - c);
    let sides = dist_origin_segment(u, v, ab)
        .min(dist_origin_segment(v, w, bc))
        .min(dist_origin_segment(w, u, ca));
    let (lab, lbc, lca) = (ab.dot(ab), bc.dot(bc), ca.dot(ca));
    let n = if lab >= lbc && lab >= lca {
        bc.cross(ca)
    } else if lbc >= lca {
        ca.cross(ab)
    } else {
        ab.cross(bc)
    };
    // The foot's barycentric weights, each times `|n|²`.
    let inside = n.dot(v.cross(bc)) > 0.0 && n.dot(w.cross(ca)) > 0.0 && n.dot(u.cross(ab)) > 0.0;
    if !inside {
        return sides;
    }
    let area2 = n.norm();
    let inradius = area2 / (lab.sqrt() + lbc.sqrt() + lca.sqrt());
    // `f64::max` drops a NaN: a foot that underflow left `0/0` reads the
    // bracket's floor.
    (u.dot(n).abs() / area2).max(sides - inradius).min(sides)
}

/// The distance from the origin to the segment from `u` to `v`, with
/// `e = v − u`; an endpoint where the origin's foot on the line is past
/// it, and where the segment has no length.
fn dist_origin_segment(u: Vec3<f64>, v: Vec3<f64>, e: Vec3<f64>) -> f64 {
    let (along, length2) = (-u.dot(e), e.dot(e));
    if along <= 0.0 {
        u.norm()
    } else if along >= length2 {
        v.norm()
    } else {
        (u + e * (along / length2)).norm()
    }
}

/// The distance from the line through `o` along `d` to the triangle
/// `abc`: the triangle's shadow on the plane across the line is a
/// triangle, or a segment where a side is along `d` or the triangle is
/// seen edge-on, and the line is one point of that plane, so it is a
/// point–triangle distance ([`dist_point_triangle`], which takes a
/// shadow of no area). `d` need not be unit; a zero `d` answers NaN.
pub fn dist_line_triangle(
    o: Point3<f64>,
    d: Vec3<f64>,
    a: Point3<f64>,
    b: Point3<f64>,
    c: Point3<f64>,
) -> f64 {
    let shadow = |p: Point3<f64>| -> Point3<f64> {
        Point3::origin() + geom::Line { origin: o, dir: d }.offset(p)
    };
    dist_point_triangle(Point3::origin(), shadow(a), shadow(b), shadow(c))
}

/// Cylinder certificate: `r − dist(axis, T)`, at least 0 (a corner
/// outside the cylinder by its carrier residual reads a distance past
/// `r`). NaN where the distance or `r` is not finite.
pub fn cert_cylinder(origin: Point3<f64>, axis: Vec3<f64>, r: f64, tri: [Point3<f64>; 3]) -> f64 {
    inside_bound(r, dist_line_triangle(origin, axis, tri[0], tri[1], tri[2]))
}

/// Sphere certificate: `r − dist(center, T)`, at least 0 and NaN as
/// [`cert_cylinder`]'s.
pub fn cert_sphere(center: Point3<f64>, r: f64, tri: [Point3<f64>; 3]) -> f64 {
    inside_bound(r, dist_point_triangle(center, tri[0], tri[1], tri[2]))
}

/// `r − dist`, at least 0, and NaN unless both are finite: `f64::max`
/// would read a NaN as the 0.
fn inside_bound(r: f64, dist: f64) -> f64 {
    if r.is_finite() && dist.is_finite() {
        (r - dist).max(0.0)
    } else {
        f64::NAN
    }
}

/// **Cone certificate: the deviation of a flat triangle with its corners
/// on one nappe**, corner `i` at azimuth `uᵢ` and slant `vᵢ` (radius
/// `ρᵢ = |vᵢ|·sin α`, height `hᵢ = |vᵢ|·cos α`), an apex corner (`pole`)
/// at `ρ = h = 0` whatever its `u`:
///
/// ```text
/// max( D₁₂, D₂₃, D₃₁, A ),
/// Dᵢⱼ = cos α · 2ρᵢρⱼ·sin²(Δuᵢⱼ/2) / (ρᵢ + ρⱼ + 2√(ρᵢρⱼ)·|cos(Δuᵢⱼ/2)|)
/// A   = sin α · (the height at which the axis passes through the triangle)
/// ```
///
/// # Derivation
///
/// Take the nappe as `h ≥ 0`, and `f(p) = sin α·h(p) − cos α·ρ(p)`.
///
/// 1. **`dist(p, cone) ≤ f(p)` on the triangle.** The solid nappe
///    `{f ≥ 0, h ≥ 0}` is convex and holds the corners, so it holds the
///    triangle; there `f(p)` is the distance from `p` to its meridian's
///    generator, whose foot `ρ·sin α + h·cos α ≥ 0` is on the ray.
/// 2. **`f` peaks on an edge, or on the axis.** From the point `O` where
///    the triangle's plane meets the axis, `h − h(O)` and `ρ` are both
///    homogeneous of degree one along a ray, so `f` is affine on it, and
///    the ray's segment in the triangle ends on the boundary or at `O`.
///    A plane parallel to the axis: `f` is affine along the axis's
///    direction, and that segment ends on the boundary both ways.
/// 3. **On the chord `i → j`**, `f = cos α·(ρ̄ − ρ)`: `ρ̄` the radii's
///    linear interpolant, `ρ = √(b² + s²)` along the chord's shadow
///    across the axis (length `L`, at distance `b` from the axis). It is
///    concave, stationary where `s/ρ = κ = (ρⱼ − ρᵢ)/L`, and there
///    `f = cos α·(ρᵢ − κ·sᵢ − b·√(1 − κ²))`, which with
///    `L² = (ρᵢ + ρⱼ)² − 4ρᵢρⱼ·cos²(θ/2)`, `b·L = ρᵢρⱼ·sin θ` and
///    `sᵢ·L = ρᵢρⱼ·cos θ − ρᵢ²` is `Dᵢⱼ`, `θ ∈ [0, π]` the angle between
///    the two azimuths (`sin²(θ/2) = sin²(Δu/2)`, `cos(θ/2) =
///    |cos(Δu/2)|` for any `Δu`). A chord at the apex is a ruling.
/// 4. **On the axis**, `f = sin α·h`. The corners' shadows `qᵢ` hold the
///    origin where `qⱼ × qₖ = ρⱼρₖ·Sᵢ`, `Sᵢ = sin(uₖ − uⱼ)`, has one sign
///    for all three, and those are its barycentric weights:
///    `A = sin α·cos α·v₁v₂v₃·ΣSᵢ / Σ vⱼvₖSᵢ`.
///
/// So the bound is the triangle's largest `f`, attained, and by (1) at
/// least its distance from the cone. It reads no chart box.
///
/// # Rounding
///
/// Every term is a quotient of sums of terms of one sign, so nothing
/// cancels after the azimuth differences are taken. The axis term reads
/// each `Sᵢ` to the rounding of its own value at any difference, which a
/// bare `sin(uₖ − uⱼ)` is not near a multiple of π ([`sin_between`]).
/// The chord term reads the rounded `Δu`, exact to half an ulp of
/// itself, about `10⁻¹⁶·|Δu|` absolute. That is relative to the chord's
/// own sag while `Δu` is not within that of a whole turn: at corners a
/// whole number of turns apart to `10⁻⁸` rad the sag (of order `10⁻³²` of
/// the radius) is read 87 % short, and across a hundred turns the loss
/// passes the verdict's allowance. So the chord term holds to its own
/// rounding for `|Δu|` up to a turn less a few `10⁻⁸`, and the lanes keep
/// a side under that: the curved lane's sides span one column step, and
/// a trimmed face's sides join points of one chart box, which is within
/// a turn (a loop that winds about the axis refuses at the domain door,
/// `TessellateError::DomainUnread`, and a seam walked twice refuses
/// `SeamTwice`), so a side that near a whole turn would have to join
/// that box's two ends. Whether the axis term enters is the
/// signs of the `Sᵢ`; where one is within rounding of zero the axis is
/// within rounding of a side, whose `D` is `A`'s limit there.
///
/// # What it assumes
///
/// The corners are the chart's points `S(uᵢ, vᵢ)` (a boundary corner's
/// carrier residual is the crate's stated slack, module docs), on one
/// nappe: non-apex corners with `v` of both signs are no triangle of one
/// nappe, and certify nothing (`+∞`). An input that is not finite, or
/// slants whose product is past `f64`'s range, answers NaN.
pub fn cert_cone(half_angle: f64, uv: [[f64; 2]; 3], pole: [bool; 3]) -> f64 {
    if !half_angle.is_finite() || uv.iter().flatten().any(|x| !x.is_finite()) {
        return f64::NAN;
    }
    let off = |i: usize| !pole[i];
    if (0..3).any(|i| off(i) && uv[i][1] > 0.0) && (0..3).any(|i| off(i) && uv[i][1] < 0.0) {
        return f64::INFINITY;
    }
    let k = half_angle.sin() * half_angle.cos();
    // Slants off the apex; an apex corner's is zero.
    let v = [0, 1, 2].map(|i| if off(i) { uv[i][1].abs() } else { 0.0 });
    let mut bound: f64 = 0.0;
    for (i, j) in [(0, 1), (1, 2), (2, 0)] {
        bound = worse(bound, cone_chord_sag(k, v[i], v[j], uv[j][0] - uv[i][0]));
    }
    // With a corner at the apex the axis meets the triangle there, where
    // `f = 0`.
    if pole == [false; 3] {
        let s = [(2, 1), (0, 2), (1, 0)].map(|(a, b)| sin_between(uv[a][0], uv[b][0]));
        if s.iter().all(|&x| x >= 0.0) || s.iter().all(|&x| x <= 0.0) {
            let weights = (v[1] * v[2] * s[0] + v[2] * v[0] * s[1] + v[0] * v[1] * s[2]).abs();
            if weights > 0.0 {
                let over_axis = v[0] * v[1] * v[2] * (s[0] + s[1] + s[2]).abs() / weights;
                bound = worse(bound, k * over_axis);
            }
        }
    }
    bound
}

/// The larger of two bounds, a NaN if either is one: `f64::max` drops a
/// NaN, and "replace where the kept one is NaN" lets the next finite
/// bound overwrite it, so a face refused for a NaN would report a finite
/// bound under its δ.
pub(crate) fn worse(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.max(b)
    }
}

/// **`sin(a − b)` to the rounding of its own value.** Where `a − b` is
/// near a multiple of π the sine is small, and the subtraction's rounding
/// (half an ulp of the difference, about `10⁻¹⁶` near π) is not small
/// against it: two azimuths `10⁻⁸` short of opposite lose seven digits.
/// The difference is taken exactly instead, as its rounded value `d` and
/// what rounding dropped, `e` (the error-free difference: `e` is exact in
/// `f64`), and `sin(d + e) = sin d + e·cos d` to `e²`. `sin d` is the
/// platform-shared `libm`'s (D9), within an ulp of its value at any `d`.
fn sin_between(a: f64, b: f64) -> f64 {
    let d = a - b;
    let from_a = d + b;
    let e = (a - from_a) - (b - (from_a - d));
    let (sin, cos) = <f64 as geom_core::Real>::sin_cos(d);
    sin + e * cos
}

/// **The largest sag of a chord of the cone** between slants `va`, `vb ≥
/// 0` at azimuth difference `du` ([`cert_cone`], step 3), with
/// `k = sin α·cos α`:
/// `k · 2·va·vb·sin²(du/2) / (va + vb + 2√(va·vb)·|cos(du/2)|)`.
/// At one slant it is `k·v·(1 − cos(du/2))`, the rim chord's sagitta
/// `sizing::cone_grid_step` solves; from a small slant to a large one it
/// tends to `k·2·v_small·sin²(du/2)`. Zero at the apex, and NaN for an
/// input that is not finite or a product past `f64`'s range.
pub(crate) fn cone_chord_sag(k: f64, va: f64, vb: f64, du: f64) -> f64 {
    let (sin, cos) = (0.5 * du).sin_cos();
    let span = va + vb + 2.0 * (va * vb).sqrt() * cos.abs();
    if span == 0.0 {
        0.0
    } else {
        k * 2.0 * va * vb * sin.powi(2) / span
    }
}

/// Torus certificate: `(A·Δu² + 2B·Δu·Δv + C·Δv²)/8` over the
/// triangle's UV extents, with the second-partial sups taken over the
/// triangle's own φ range (module docs; the derivation is
/// `crate::sizing::torus_grid_steps`'). Total for every `R, r > 0`:
/// `A` is `max |R + r·cos φ|`, so a spindle torus's window across
/// `R + r·cos φ = 0` reads the larger lobe rather than a signed value.
/// NaN-sticky: a poisoned corner poisons the certificate rather than
/// being dropped by a `max`.
pub fn cert_torus(major: f64, minor: f64, uv: [[f64; 2]; 3]) -> f64 {
    if uv.iter().flatten().any(|x| x.is_nan()) {
        return f64::NAN;
    }
    let (mut u0, mut u1, mut v0, mut v1) = (
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
    );
    for [u, v] in uv {
        u0 = u0.min(u);
        u1 = u1.max(u);
        v0 = v0.min(v);
        v1 = v1.max(v);
    }
    torus_chord_bound(
        abs_radial_max(major, minor, v0, v1),
        minor * abs_sin_max(v0, v1),
        minor,
        u1 - u0,
        v1 - v0,
    )
}

/// Spindle-sheet certificate: [`cert_torus`] over the triangle with
/// each apex corner's `u` read as a non-apex corner's. Every `u` at an
/// apex (`v = ±v*`) maps to the one point there, so that triangle has
/// the same three corners in space and a parametric patch on the sheet
/// whose `u` extent is the non-apex corners' alone, as the cone's
/// certificate drops its apex from `Δu`. Its `v` extent still reaches
/// the apex. A triangle with fewer than two corners off an apex is no
/// fan triangle, and certifies nothing (+∞).
pub fn cert_spindle(major: f64, minor: f64, uv: [[f64; 2]; 3], apex: [bool; 3]) -> f64 {
    let Some(u) = (0..3).find(|&i| !apex[i]).map(|i| uv[i][0]) else {
        return f64::INFINITY;
    };
    if apex.iter().filter(|&&a| a).count() > 1 {
        return f64::INFINITY;
    }
    let mut read = uv;
    for (corner, &at) in read.iter_mut().zip(&apex) {
        if at {
            corner[0] = u;
        }
    }
    cert_torus(major, minor, read)
}

/// The doubly-curved chord bound `(A·Δu² + 2B·Δu·Δv + C·Δv²)/8` on
/// the second-partial sups `A, B, C` and the UV extents `Δu, Δv` —
/// the one arithmetic [`cert_torus`] certifies with and
/// `crate::sizing::torus_grid_steps` inverts, so the two cannot drift
/// apart by a constant (`sizing`'s
/// `torus_grid_steps_and_cert_torus_are_an_inverse_pair`).
pub(crate) fn torus_chord_bound(a: f64, b: f64, c: f64, du: f64, dv: f64) -> f64 {
    (a * du.powi(2) + 2.0 * b * du * dv + c * dv.powi(2)) / 8.0
}

/// `max |R + r·cos φ|` over `[v0, v1]`: the endpoints, plus `R + r`
/// where the interval holds a multiple of 2π and `|R − r|` where it
/// holds an odd multiple of π (every interior extremum of
/// `R + r·cos φ` is one of those, and `|·|` peaks at an extremum or
/// an endpoint).
///
/// "Holds" is [`geom::periodic_window_may_hold`]'s: an extremum is
/// left out only where the interval certainly misses it.
fn abs_radial_max(major: f64, minor: f64, v0: f64, v1: f64) -> f64 {
    use core::f64::consts::{PI, TAU};
    let mut m = (major + minor * v0.cos())
        .abs()
        .max((major + minor * v1.cos()).abs());
    if geom::periodic_window_may_hold((0.0, 0.0), (v0, v1), TAU) {
        m = m.max(major + minor);
    }
    if geom::periodic_window_may_hold((PI, PI), (v0, v1), TAU) {
        m = m.max((major - minor).abs());
    }
    m
}

/// `max |sin φ|` over `[v0, v1]`: 1 if the interval holds an odd
/// multiple of π/2 (π/2 at period π), else at an endpoint.
fn abs_sin_max(v0: f64, v1: f64) -> f64 {
    use core::f64::consts::{FRAC_PI_2, PI};
    if geom::periodic_window_may_hold((FRAC_PI_2, FRAC_PI_2), (v0, v1), PI) {
        1.0
    } else {
        v0.sin().abs().max(v1.sin().abs())
    }
}

/// The rounding a certificate is allowed beyond δ, in units of the
/// f64 epsilon of the magnitude it was computed from. **Certificate
/// resolution.**
///
/// A certificate is a few dozen f64 operations, each exact to half an
/// ulp of its own result. The ones that matter are the subtractions
/// that cancel: `r − dist(axis, T)` cancels two lengths of the size of
/// the vertex coordinates, `1 − cos(Δu/2)` cancels at 1, and a UV
/// extent `u₁ − u₀` cancels at `|u|`. Each such step carries at most an
/// ulp or two of the cancelled magnitude, and 8 bounds their sum with
/// room; the worst overshoot measured at a δ chosen to land a
/// cylinder's worst triangle on δ exactly (an n-gon rim) is half an
/// ulp. It is the promise's "+ rounding", not a margin on δ: no step
/// reads it.
pub(crate) const ROUNDING_ULPS: f64 = 8.0;

/// How small the allowance must stay against both δ and the kernel ε
/// for the verdict to apply it: a quarter of the smaller. **Certificate
/// resolution.**
///
/// Applying the allowance says "this triangle is within δ, and f64
/// cannot tell the last bits"; that is honest only while those bits are
/// far below anything the promise or the model resolves. A quarter of
/// the smaller of δ and ε keeps the absorbed rounding a minor term of
/// both, while leaving every body of ordinary extent (coordinates up to
/// ~1e2 m at the ε = 1e-12 row: 8·2.2e-16·100 ≈ 1.8e-13 = 0.18 ε) free
/// to certify at the edge. Past it the coordinates are too large, or δ
/// or ε too small, for f64 to certify the triangle at all, and the face
/// refuses typed ([`TessellateError::RoundingUnresolved`]) rather than
/// absorbing a rounding term that rivals what it certifies.
///
/// [`TessellateError::RoundingUnresolved`]: crate::types::TessellateError::RoundingUnresolved
pub(crate) const ALLOWANCE_SHARE: f64 = 0.25;

/// [`ROUNDING_ULPS`] epsilons of `scale`.
pub(crate) fn rounding_allowance(scale: f64) -> f64 {
    ROUNDING_ULPS * f64::EPSILON * scale
}

/// What the certificate verdict found for one triangle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Verdict {
    /// Within δ, or past it only by its own rounding while that rounding
    /// is small ([`ALLOWANCE_SHARE`]).
    Certified,
    /// Past δ by more than its rounding, or NaN.
    Exceeded,
    /// Past δ only by its rounding, but that rounding is not small
    /// against δ or ε: f64 cannot certify this triangle honestly.
    Unresolved {
        /// The rounding allowance the verdict would have had to absorb.
        allowance: f64,
    },
}

/// **The one certificate verdict**: whether `bound`, computed at the
/// magnitude `scale`, certifies the triangle within `delta`. The
/// allowance is consulted only when the bound is past δ — a triangle
/// within δ needs none — and it is applied only while it is small
/// against both δ and `eps`; otherwise the verdict says so
/// ([`Verdict::Unresolved`]) instead of absorbing it.
pub(crate) fn verdict(bound: f64, delta: f64, scale: f64, eps: Eps) -> Verdict {
    if bound.is_nan() {
        return Verdict::Exceeded;
    }
    if bound <= delta {
        return Verdict::Certified;
    }
    let allowance = rounding_allowance(scale);
    if bound > delta + allowance {
        return Verdict::Exceeded;
    }
    let measured = allowance / ALLOWANCE_SHARE;
    if measured <= delta && eps.dominates(measured) {
        Verdict::Certified
    } else {
        Verdict::Unresolved { allowance }
    }
}

/// The magnitude a position certificate ([`cert_cylinder`],
/// [`cert_sphere`]) rounds at: the largest coordinate norm among the
/// chart's anchor and the triangle's corners, since every distance it
/// takes is a difference of those.
pub(crate) fn position_scale(anchor: Point3<f64>, tri: [Point3<f64>; 3]) -> f64 {
    let norm = |p: Point3<f64>| (p - Point3::origin()).norm();
    norm(anchor)
        .max(norm(tri[0]))
        .max(norm(tri[1]))
        .max(norm(tri[2]))
}

/// The magnitude a UV-extent certificate ([`cert_torus`], the NURBS
/// lane's) rounds at. The bound is a sum of non-negative products of
/// the extents, so its own arithmetic is relative to it; what is not is
/// each extent `Δ = x₁ − x₀`, exact only to an ulp of `|x|`, which moves
/// a term quadratic in it by `2·|x|/Δ` of itself.
pub(crate) fn uv_scale(bound: f64, uv: [[f64; 2]; 3]) -> f64 {
    let mut scale = 1.0;
    for axis in 0..2 {
        let (lo, hi, big) = uv.iter().fold(
            (f64::INFINITY, f64::NEG_INFINITY, 0.0_f64),
            |(lo, hi, big), p| (lo.min(p[axis]), hi.max(p[axis]), big.max(p[axis].abs())),
        );
        if hi > lo {
            scale += 2.0 * big / (hi - lo);
        }
    }
    bound * scale
}

/// The magnitude a cone triangle's verdict rounds at: `cos α·sin α·
/// v_maxᵀ`, times `1 + π`. [`cert_cone`] is exact to the rounding of its
/// own value; what rounds at this magnitude is the triangle it is handed:
///
/// - a lattice step is solved from `1 − cos(Δu/2)`, which cancels at 1
///   (`sizing::cone_grid_step`), so the worst triangle's true sag sits
///   within an ulp or two of `cos α·sin α·v_max` of δ, either side: the
///   `1`;
/// - a lattice azimuth is exact to an ulp of itself, which moves a
///   step-wide chord's sag by `cos α·sin α·v·|u|·Δu·ε/4`: the `π`, which
///   with the verdict's eight ulps covers `|u|·Δu ≤ 32π`, a chart of
///   twenty turns at the angular cap and more at a finer step.
///
/// Past that a triangle landing on δ reads as exceeded, a refusal and
/// never a pass.
pub(crate) fn cone_scale(half_angle: f64, uv: [[f64; 2]; 3]) -> f64 {
    let v_max = uv.iter().fold(0.0_f64, |m, p| m.max(p[1].abs()));
    half_angle.cos() * half_angle.sin() * v_max * (1.0 + core::f64::consts::PI)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// **The verdict's edges** ([`verdict`]): δ itself certifies; a
    /// bound within [`ROUNDING_ULPS`] epsilons of its magnitude past δ
    /// certifies while that allowance is small against δ and ε; one past
    /// the allowance is exceeded; NaN and infinity never certify. And the
    /// flag: at a magnitude whose allowance rivals ε (a body ~1e4 m from
    /// the origin at ε = 1e-12), or rivals δ (δ = 1e-15 at unit scale),
    /// the same overshoot is [`Verdict::Unresolved`], naming the
    /// allowance, not certified.
    #[test]
    fn the_verdict_admits_its_own_rounding_only_while_it_is_small() {
        let unit = f64::EPSILON;
        let eps = Eps::exactly(1e-9);
        assert_eq!(verdict(0.5, 1.0, 1.0, eps), Verdict::Certified);
        assert_eq!(
            verdict(1.0, 1.0, 1.0, eps),
            Verdict::Certified,
            "δ certifies"
        );
        assert_eq!(
            verdict(1e-3 * (1.0 + 2.0 * unit), 1e-3, 1.0, eps),
            Verdict::Certified,
            "rounding past δ certifies while the allowance is small"
        );
        assert_eq!(
            verdict(1e-3 + 16.0 * unit, 1e-3, 1.0, eps),
            Verdict::Exceeded,
            "past the allowance is exceeded"
        );
        assert_eq!(verdict(f64::NAN, 1.0, 1.0, eps), Verdict::Exceeded);
        assert_eq!(verdict(f64::INFINITY, 1.0, 1.0, eps), Verdict::Exceeded);
        // Far from the origin at a fine ε: the allowance (8·ε·1e4 ≈ 1.8e-11)
        // is past a quarter of ε = 1e-12.
        let fine = Eps::exactly(1e-12);
        let far = 1e4;
        assert_eq!(
            verdict(1e-3 + 1e-12, 1e-3, far, fine),
            Verdict::Unresolved {
                allowance: rounding_allowance(far)
            },
            "a large allowance is flagged, not absorbed"
        );
        // A δ near the f64 floor at unit scale: the allowance rivals δ.
        assert!(matches!(
            verdict(1e-15 * (1.0 + 1e-3), 1e-15, 1.0, eps),
            Verdict::Unresolved { .. }
        ));
        // The same overshoot at an ordinary magnitude certifies.
        assert_eq!(verdict(1e-3 + 1e-18, 1e-3, 1.0, fine), Verdict::Certified);
    }
    use core::f64::consts::{PI, TAU};

    /// A torus on the z axis with `u_ref = x`, in the chart the
    /// certificate is stated in: `(R + r·cos v)·(cos u, sin u, 0) +
    /// (0, 0, r·sin v)`.
    fn torus_point(major: f64, minor: f64, u: f64, v: f64) -> Point3<f64> {
        let rho = major + minor * v.cos();
        Point3::new(rho * u.cos(), rho * u.sin(), minor * v.sin())
    }

    /// Exact distance from `p` to that torus: in the `(ρ, z)` half-plane
    /// the tube is the circle of radius `r` about `(R, 0)`.
    fn torus_dist(major: f64, minor: f64, p: Point3<f64>) -> f64 {
        let rho = p.x.hypot(p.y);
        (((rho - major).powi(2) + p.z * p.z).sqrt() - minor).abs()
    }

    /// The largest sampled `‖Π − P‖` over the triangle `uv` — the
    /// affine interpolant against the parametric surface at the same
    /// barycentric point (barycentric grid, `n = 8` ⇒ 45 points). This
    /// is the quantity the certificate bounds, and on a spindle torus
    /// (`R < r`) it is the only honest one: the inner lobe of the
    /// parametrisation lies INSIDE the solid, so a distance to the
    /// locus "r from the major circle" is not a distance to the chart.
    fn parametric_deviation(major: f64, minor: f64, uv: [[f64; 2]; 3]) -> f64 {
        let c = uv.map(|[u, v]| torus_point(major, minor, u, v));
        let n = 8u32;
        let mut worst: f64 = 0.0;
        for i in 0..=n {
            for j in 0..=(n - i) {
                let (li, lj) = (f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
                let lk = 1.0 - li - lj;
                let u = uv[0][0] * li + uv[1][0] * lj + uv[2][0] * lk;
                let v = uv[0][1] * li + uv[1][1] * lj + uv[2][1] * lk;
                let p = torus_point(major, minor, u, v);
                let q = Point3::new(
                    c[0].x * li + c[1].x * lj + c[2].x * lk,
                    c[0].y * li + c[1].y * lj + c[2].y * lk,
                    c[0].z * li + c[1].z * lj + c[2].z * lk,
                );
                worst = worst.max((q - p).norm());
            }
        }
        worst
    }

    /// The largest sampled distance from the affine triangle over `uv`
    /// to the ring torus's locus (barycentric grid, `n = 8` ⇒ 45
    /// points) — the chordal promise's own quantity, `≤` the parametric
    /// deviation; ring tori only (`R ≥ r`), where locus and chart agree.
    fn sampled_deviation(major: f64, minor: f64, uv: [[f64; 2]; 3]) -> f64 {
        let c = uv.map(|[u, v]| torus_point(major, minor, u, v));
        let n = 8u32;
        let mut worst: f64 = 0.0;
        for i in 0..=n {
            for j in 0..=(n - i) {
                let (li, lj) = (f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
                let lk = 1.0 - li - lj;
                let p = Point3::new(
                    c[0].x * li + c[1].x * lj + c[2].x * lk,
                    c[0].y * li + c[1].y * lj + c[2].y * lk,
                    c[0].z * li + c[1].z * lj + c[2].z * lk,
                );
                worst = worst.max(torus_dist(major, minor, p));
            }
        }
        worst
    }

    /// **Soundness of [`cert_torus`] on triangles of every shape**, not
    /// only the axis-aligned cell-halves the grid emits: random UV
    /// triangles inside random boxes up to 0.6 rad a side, anywhere on
    /// tori across `R/r` from 1.1 to 100, densely sampled against the
    /// exact torus distance. Popoviciu and Cauchy–Schwarz are what
    /// make the bound hold off the lattice, and this is their row.
    #[test]
    fn cert_torus_bounds_the_sampled_deviation_on_random_triangles() {
        let mut rng = test_utils::fuzz::start("cert_torus");
        for _ in 0..test_utils::fuzz::scaled(3000) {
            let minor = rng.range(0.05, 2.0);
            let major = minor * rng.range(1.1, 100.0);
            let (u0, v0) = (rng.range(-PI, PI), rng.range(-PI, PI));
            let (du, dv) = (rng.range(0.0, 0.6), rng.range(0.0, 0.6));
            let mut uv = [[0.0; 2]; 3];
            for corner in &mut uv {
                *corner = [u0 + rng.unit() * du, v0 + rng.unit() * dv];
            }
            let cert = cert_torus(major, minor, uv);
            let dev = sampled_deviation(major, minor, uv);
            assert!(
                dev <= cert * (1.0 + 1e-9) + 1e-12,
                "R {major} r {minor} uv {uv:?}: sampled deviation {dev} exceeds the certificate \
                 {cert} ({})",
                test_utils::fuzz::replay()
            );
        }
    }

    /// **The certificate's constant is not slack.** On the outer
    /// equator `P_θθ` and `P_φφ` are parallel and `P_θφ` vanishes, so a
    /// cell-half's hypotenuse midpoint deviates by exactly
    /// `(A·h_u² + C·h_v²)/8` to second order — the certificate, less
    /// the `O(h³)` mixed term the window's `max |sin|` still charges.
    /// The measured-to-certified ratio approaches 1 from below as the
    /// cell shrinks; a fudge factor on either side of the certificate
    /// moves it off 1 and reds this row.
    #[test]
    fn certificate_is_attained_on_the_outer_equator() {
        for (major, minor) in [(0.30, 0.07), (2.0, 0.5), (1.2, 1.0), (50.0, 1.0)] {
            let mut gaps = Vec::new();
            for h in [0.04, 0.02, 0.01] {
                let (hu, hv) = (h * 0.5, h * 1.5);
                let uv = [[0.0, -hv / 2.0], [hu, -hv / 2.0], [0.0, hv / 2.0]];
                let cert = cert_torus(major, minor, uv);
                let c = uv.map(|[u, v]| torus_point(major, minor, u, v));
                let mid = Point3::new(
                    (c[1].x + c[2].x) / 2.0,
                    (c[1].y + c[2].y) / 2.0,
                    (c[1].z + c[2].z) / 2.0,
                );
                let dev = torus_dist(major, minor, mid);
                let ratio = dev / cert;
                assert!(
                    ratio <= 1.0 && ratio > 0.95,
                    "R {major} r {minor} h {h}: hypotenuse-midpoint deviation is {ratio} of the \
                     certificate"
                );
                gaps.push(1.0 - ratio);
            }
            assert!(
                gaps[0] > gaps[1] && gaps[1] > gaps[2],
                "R {major} r {minor}: the gap to the certificate does not shrink with the cell: \
                 {gaps:?}"
            );
        }
    }

    /// **A spindle torus (`R < r`) certifies soundly.** `R + r·cos φ`
    /// changes sign across the inner lobe, and a certificate reading
    /// the SIGNED radial as `sup ‖P_θθ‖` under-reports there: on this
    /// window `R + r·cos_max = 0.2 − 0.989 < 0` would certify 0.0057
    /// against a sampled parametric deviation of 0.0201. The absolute
    /// sup makes the bound total for every `R, r > 0`. (The random
    /// sweep above draws `R/r ≥ 1.1`, ring tori, because its oracle is
    /// the locus distance; this row's is the parametric deviation.)
    #[test]
    fn cert_torus_is_sound_on_a_spindle_torus_across_the_inner_lobe() {
        let (major, minor) = (0.2, 1.0);
        let uv = [[0.0, PI - 0.15], [0.3, PI - 0.15], [0.0, PI + 0.15]];
        let cert = cert_torus(major, minor, uv);
        let dev = parametric_deviation(major, minor, uv);
        assert!(
            dev <= cert * (1.0 + 1e-9) + 1e-12,
            "spindle: sampled deviation {dev} exceeds certificate {cert}"
        );
        assert!(dev > 0.02, "the probe window is the one that bites ({dev})");
        let signed = (major + minor * (PI - 0.15).cos()).max(major + minor * (PI + 0.15).cos());
        let b = minor * abs_sin_max(PI - 0.15, PI + 0.15);
        let under = torus_chord_bound(signed, b, minor, 0.3, 0.3);
        assert!(
            signed < 0.0 && under < 0.5 * dev,
            "the signed radial ({under}) is no longer the under-reporting expression this \
             row exists for ({dev})"
        );
    }

    /// The window sups at their edge cases: an interval holding 2πk
    /// reads `A = R + r`, one holding an odd multiple of π reads
    /// `|R − r|` where that is the larger, and one holding π/2 + kπ
    /// reads `abs_sin_max = 1`, however far from the origin; otherwise
    /// an endpoint decides. A poisoned corner poisons the certificate.
    #[test]
    fn torus_window_sups_and_nan_stickiness() {
        let (r_big, r_small) = (2.0, 0.5);
        assert!(abs_radial_max(r_big, r_small, -0.1, 0.1) == 2.5);
        assert!(abs_radial_max(r_big, r_small, 6.0, 6.5) == 2.5);
        assert!(abs_radial_max(r_big, r_small, -13.0, -12.5) == 2.5);
        assert!(abs_radial_max(r_big, r_small, 0.1, 0.3) == 2.0 + 0.5 * 0.1_f64.cos());
        assert!(abs_radial_max(r_big, r_small, 3.0, 3.2) == 2.0 + 0.5 * 3.0_f64.cos());
        // Spindle (R < r): across φ = π the signed radial crosses zero
        // and the sup is `|R − r|` when that lies inside, else the
        // larger endpoint magnitude — never a signed value.
        assert!(abs_radial_max(0.2, 1.0, 3.0, 3.3) == 0.8);
        assert!(abs_radial_max(0.2, 1.0, 2.5, 2.8) == (0.2 + 2.8_f64.cos()).abs());
        assert!(abs_radial_max(0.2, 1.0, -0.2, 0.2) == 1.2);
        assert!(abs_sin_max(1.5, 1.6) == 1.0);
        assert!(abs_sin_max(-1.6, -1.5) == 1.0);
        assert!(abs_sin_max(4.6, 4.8) == 1.0);
        assert!((abs_sin_max(0.1, 0.3) - 0.3_f64.sin()).abs() == 0.0);
        assert!((abs_sin_max(3.0, 3.1) - 3.0_f64.sin().abs()).abs() == 0.0);
        assert!(cert_torus(1.0, 0.2, [[0.0, 0.0], [f64::NAN, 0.0], [0.0, 0.1]]).is_nan());
        assert!(cert_torus(1.0, 0.2, [[0.0, 0.0], [0.1, 0.0], [0.0, 0.1]]).is_finite());
    }

    /// **[`cert_torus`] is sound on spindle sheets**, the datum a sheet
    /// face meshes through: signed `R ∈ (−r, r)`, lemons and apples,
    /// random triangles inside the sheet's window `|v| ≤ v*` (up to 0.6
    /// rad a side), against the parametric deviation.
    #[test]
    fn cert_torus_bounds_the_deviation_on_lemon_and_apple_sheets() {
        let mut rng = test_utils::fuzz::start("cert_torus_sheet");
        for _ in 0..test_utils::fuzz::scaled(20_000) {
            let minor = rng.range(0.05, 2.0);
            let major = minor * rng.range(-0.98, 0.98);
            let vs = (-major / minor).acos();
            let (du, dv) = (rng.range(0.0, 0.6), rng.range(0.0, 0.6_f64.min(2.0 * vs)));
            let (u0, v0) = (rng.range(-PI, PI), rng.range(-vs, vs - dv));
            let mut uv = [[0.0; 2]; 3];
            for corner in &mut uv {
                *corner = [u0 + rng.unit() * du, v0 + rng.unit() * dv];
            }
            let cert = cert_torus(major, minor, uv);
            let dev = parametric_deviation(major, minor, uv);
            assert!(
                dev <= cert * (1.0 + 1e-9) + 1e-12,
                "R {major} r {minor} uv {uv:?}: deviation {dev} exceeds {cert} ({})",
                test_utils::fuzz::replay()
            );
        }
    }

    /// The exact distance from `p` to the sheet `(R, r)` about `z`: in
    /// `p`'s meridian half-plane, to the sheet's arc `|v| ≤ v*` of the
    /// circle of radius `r` about `(R, 0)`, or to the nearer apex where
    /// the circle's nearest point is off the arc.
    fn sheet_dist(major: f64, minor: f64, p: Point3<f64>) -> f64 {
        let (rho, z) = (p.x.hypot(p.y), p.z);
        let v_star = (-major / minor).clamp(-1.0, 1.0).acos();
        let (x, y) = (rho - major, z);
        if y.atan2(x).abs() <= v_star {
            return (x.hypot(y) - minor).abs();
        }
        let h = minor * v_star.sin();
        rho.hypot(z - h).min(rho.hypot(z + h))
    }

    /// **`cert_spindle` bounds a fan triangle's distance from its sheet**
    /// (a seeded fuzz). Each triangle has one corner, in any slot, at an
    /// apex with an arbitrary `u`, and two near it: every point of the
    /// flat triangle is within the certificate of the sheet, measured
    /// exactly ([`sheet_dist`], blind to the chart). The sheets reach the
    /// tightest the bound gets: a fifth horns (`R = r`, the needle's
    /// cusp), a tenth a hair inside one, a tenth needle-thin lemons, the
    /// rest across `|R| ≤ 0.999r`, with fans to `Δu ≤ 1.5`. The apex's
    /// `u` is dropped from the certificate's `Δu` and its `v` kept; with
    /// the `v` dropped as well this goes red.
    #[test]
    fn cert_spindle_bounds_a_fan_triangle_on_every_sheet() {
        let mut rng = test_utils::fuzz::start("cert_spindle_fan");
        for _ in 0..test_utils::fuzz::scaled(20_000) {
            let minor = rng.range(0.05, 2.0);
            let pick = rng.unit();
            let major = if pick < 0.2 {
                minor
            } else if pick < 0.3 {
                minor * (1.0 - 1e-9)
            } else if pick < 0.4 {
                -minor * (1.0 - 1e-6)
            } else {
                minor * rng.range(-0.999, 0.999)
            };
            let vs = (-major / minor).clamp(-1.0, 1.0).acos();
            let up = rng.unit() < 0.5;
            let at = if up { vs } else { -vs };
            let (du, dv) = (rng.range(0.0, 1.5), rng.range(0.0, vs.min(1.5)));
            let u0 = rng.range(-PI, PI);
            let mut uv = [[rng.range(-PI, PI), at]; 3];
            for corner in &mut uv[1..] {
                let off = rng.unit() * dv;
                *corner = [u0 + rng.unit() * du, if up { at - off } else { at + off }];
            }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let slot = ((rng.unit() * 3.0) as usize).min(2);
            uv.swap(0, slot);
            let mut apex = [false; 3];
            apex[slot] = true;
            let cert = cert_spindle(major, minor, uv, apex);
            let c = uv.map(|[u, v]| torus_point(major, minor, u, v));
            let n = 12u32;
            let mut worst: f64 = 0.0;
            for i in 0..=n {
                for j in 0..=(n - i) {
                    let (li, lj) = (f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
                    let lk = 1.0 - li - lj;
                    let q = Point3::new(
                        c[0].x * li + c[1].x * lj + c[2].x * lk,
                        c[0].y * li + c[1].y * lj + c[2].y * lk,
                        c[0].z * li + c[1].z * lj + c[2].z * lk,
                    );
                    worst = worst.max(sheet_dist(major, minor, q));
                }
            }
            assert!(
                worst <= cert * (1.0 + 1e-9) + 1e-12,
                "R {major} r {minor} uv {uv:?} apex {apex:?}: distance {worst} exceeds {cert} ({})",
                test_utils::fuzz::replay()
            );
        }
    }

    /// A unit-frame cone, apex at the origin about `z`, in the chart the
    /// certificate is stated in: `v·(sin α·cos u, sin α·sin u, cos α)`.
    fn cone_point(alpha: f64, u: f64, v: f64) -> Point3<f64> {
        let rho = v * alpha.sin();
        Point3::new(rho * u.cos(), rho * u.sin(), v * alpha.cos())
    }

    /// Exact distance from `p` to that double cone: in `p`'s meridian
    /// half-plane, to the nearer of the two generator rays `t·(sin α,
    /// ±cos α)`, `t ≥ 0`. Blind to the chart.
    fn cone_dist(alpha: f64, p: Point3<f64>) -> f64 {
        let (rho, h) = (p.x.hypot(p.y), p.z);
        let (s, c) = alpha.sin_cos();
        [1.0, -1.0]
            .map(|sign| {
                let t = (rho * s + h * sign * c).max(0.0);
                (rho - t * s).hypot(h - t * sign * c)
            })
            .into_iter()
            .fold(f64::INFINITY, f64::min)
    }

    /// The largest [`cone_dist`] over a barycentric grid of `n` steps a
    /// side on the flat triangle over the chart corners `uv`.
    fn cone_sampled(alpha: f64, uv: [[f64; 2]; 3], n: u32) -> f64 {
        let c = uv.map(|[u, v]| cone_point(alpha, u, v));
        let mut worst: f64 = 0.0;
        for i in 0..=n {
            for j in 0..=(n - i) {
                let (li, lj) = (f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
                let lk = 1.0 - li - lj;
                let q = Point3::new(
                    c[0].x * li + c[1].x * lj + c[2].x * lk,
                    c[0].y * li + c[1].y * lj + c[2].y * lk,
                    c[0].z * li + c[1].z * lj + c[2].z * lk,
                );
                worst = worst.max(cone_dist(alpha, q));
            }
        }
        worst
    }

    /// **[`cert_cone`] bounds the true deviation of every triangle built
    /// to break it** (a seeded fuzz): cones from a needle (`α = 0.002`)
    /// to nearly flat (`α = 1.55`), either nappe, and the flat triangle's
    /// distance from the cone sampled exactly ([`cone_dist`], blind to
    /// the chart) on a 325-point grid. The shapes, a sixth of the draws
    /// each:
    ///
    /// - the filed table's: two corners on one row and one at a slant
    ///   down to `10⁻⁶` of it, up to 1.5 rad round the axis;
    /// - a needle: a sliver in azimuth across a long slant range;
    /// - a corner at the apex, in any slot, at an arbitrary `u`;
    /// - about the axis: three azimuths near thirds of a turn, so the
    ///   axis passes through the triangle;
    /// - a whole-turn face's: any three azimuths over a turn and a half;
    /// - any three chart points in a box.
    ///
    /// Halving a chord's sag, or dropping the axis term, reds this row.
    #[test]
    fn cert_cone_bounds_the_true_deviation_on_triangles_built_to_break_it() {
        let mut rng = test_utils::fuzz::start("cert_cone_break");
        let mut measurable = 0usize;
        let draws = test_utils::fuzz::scaled(20_000);
        for k in 0..draws {
            let alpha = match k % 5 {
                0 => rng.range(0.002, 0.05),
                1 => rng.range(1.4, 1.55),
                _ => rng.range(0.05, 1.4),
            };
            let u0 = rng.range(-7.0, 7.0);
            let big = rng.range(0.01, 30.0);
            let mut pole = [false; 3];
            let mut uv = match k % 6 {
                0 => {
                    let small = big * 10f64.powf(rng.range(-6.0, 0.0));
                    let step = rng.range(0.0, 0.2);
                    [
                        [u0, big],
                        [u0 + step, big],
                        [u0 + rng.range(-1.5, 1.5), small],
                    ]
                }
                1 => {
                    let du = rng.range(0.0, 1e-3);
                    [
                        [u0, big * rng.unit()],
                        [u0 + du * rng.unit(), big],
                        [u0 + du, big * rng.unit()],
                    ]
                }
                2 => {
                    pole[0] = true;
                    let du = rng.range(0.0, 3.0);
                    [
                        [rng.range(-7.0, 7.0), 0.0],
                        [u0, big * rng.unit()],
                        [u0 + du, big * rng.unit()],
                    ]
                }
                3 => {
                    let third = core::f64::consts::TAU / 3.0;
                    [0.0, 1.0, 2.0]
                        .map(|i| [u0 + i * third + rng.range(-0.9, 0.9), big * rng.unit()])
                }
                4 => [0.0; 3].map(|_| [u0 + rng.range(0.0, 9.5), big * rng.unit()]),
                _ => {
                    let (du, v0) = (rng.range(0.0, 2.0), big * rng.unit());
                    [0.0; 3].map(|_| [u0 + du * rng.unit(), v0 + (big - v0) * rng.unit()])
                }
            };
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let slot = ((rng.unit() * 3.0) as usize).min(2);
            uv.swap(0, slot);
            pole.swap(0, slot);
            if rng.unit() < 0.5 {
                for corner in &mut uv {
                    corner[1] = -corner[1];
                }
            }
            let cert = cert_cone(alpha, uv, pole);
            let dev = cone_sampled(alpha, uv, 24);
            assert!(
                dev <= cert * (1.0 + 1e-9) + 1e-13 * big,
                "α {alpha} uv {uv:?} apex {pole:?}: true deviation {dev:e} past its                  certificate {cert:e} ({})",
                test_utils::fuzz::replay()
            );
            measurable += usize::from(dev > 1e-6 * big);
        }
        assert!(
            measurable > draws / 3,
            "the triangles deviate measurably ({measurable} of {draws}), so an              under-reporting certificate reds"
        );
    }

    /// **[`cert_cone`] is attained, on these poses.** Each term is the
    /// triangle's deviation where it peaks, not a bound above it:
    ///
    /// - a chord's sag ([`cone_chord_sag`]) is the largest distance from
    ///   the cone along the chord, found by a ternary search of that
    ///   concave function, to `10⁻¹²` of the chord's scale: at one radius
    ///   and at radii `10⁻³` apart, across `0.4`, `2.5`, `π` and `4` rad
    ///   (past half a turn, where the chart's `Δu` is not the angle
    ///   between the corners), on `α = 0.3` and `1.2`;
    /// - a triangle about the axis (azimuths `0, 2.2, 4.3`, slants `1,
    ///   0.5, 2`, `α = 0.4`) certifies at its distance from the cone
    ///   where its plane meets the axis, to `10⁻¹²`; no chord of it sags
    ///   past nine tenths of that, and no point of a 20 301-point grid of
    ///   it lies farther.
    ///
    /// A bound that only dominates (the triangle's whole azimuth span at
    /// its widest slant) reds here.
    #[test]
    fn cert_cone_is_attained_on_a_chord_and_over_the_axis() {
        for alpha in [0.3_f64, 1.2] {
            for (va, vb) in [(1.0, 1.0), (1e-3, 1.0), (0.4, 2.5)] {
                for du in [0.4, 2.5, PI, 4.0] {
                    let (a, b) = (cone_point(alpha, 0.2, va), cone_point(alpha, 0.2 + du, vb));
                    let along = |t: f64| cone_dist(alpha, a + (b - a) * t);
                    let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
                    for _ in 0..200 {
                        let (m1, m2) = (lo + (hi - lo) / 3.0, hi - (hi - lo) / 3.0);
                        if along(m1) < along(m2) {
                            lo = m1;
                        } else {
                            hi = m2;
                        }
                    }
                    let sag = cone_chord_sag(alpha.sin() * alpha.cos(), va, vb, du);
                    let found = along(0.5 * (lo + hi));
                    assert!(
                        (sag - found).abs() <= 1e-12 * vb,
                        "α {alpha} slants {va}, {vb} across {du}: the chord's sag {sag:e} is                          not its largest distance {found:e}"
                    );
                    // The apex in the third slot adds two rulings.
                    let uv = [[0.2, va], [0.2 + du, vb], [1.0, 0.0]];
                    let fan = cert_cone(alpha, uv, [false, false, true]);
                    assert!(
                        (fan - sag).abs() <= 1e-12 * vb,
                        "with the apex its third corner the triangle certifies at {fan:e}, \
                         not its one chord's {sag:e}"
                    );
                }
            }
        }
        let (alpha, uv) = (0.4_f64, [[0.0, 1.0], [2.2, 0.5], [4.3, 2.0]]);
        let cert = cert_cone(alpha, uv, [false; 3]);
        // Where the triangle's plane meets the axis, by its own linear
        // solve: the barycentric point whose `x` and `y` vanish.
        let c = uv.map(|[u, v]| cone_point(alpha, u, v));
        let cross = |i: usize, j: usize| c[i].x * c[j].y - c[j].x * c[i].y;
        let w = [cross(1, 2), cross(2, 0), cross(0, 1)];
        let sum = w[0] + w[1] + w[2];
        assert!(w.iter().all(|&x| x * sum > 0.0), "the axis is inside it");
        let on_axis = Point3::new(
            0.0,
            0.0,
            (w[0] * c[0].z + w[1] * c[1].z + w[2] * c[2].z) / sum,
        );
        let found = cone_dist(alpha, on_axis);
        let k = alpha.sin() * alpha.cos();
        let chords = [(0, 1), (1, 2), (2, 0)]
            .map(|(i, j)| cone_chord_sag(k, uv[i][1], uv[j][1], uv[j][0] - uv[i][0]))
            .into_iter()
            .fold(0.0, f64::max);
        assert!(
            (found - cert).abs() <= 1e-12 && chords < 0.9 * cert,
            "about the axis: certificate {cert:e}, on the axis {found:e}, largest chord \
             {chords:e}"
        );
        let sampled = cone_sampled(alpha, uv, 200);
        assert!(
            sampled <= cert * (1.0 + 1e-12) && sampled >= 0.99 * cert,
            "about the axis: certificate {cert:e}, sampled {sampled:e}"
        );
    }

    /// `a − b` exactly, as its rounded value and the rest, by integer
    /// arithmetic on the two mantissas (no floating subtraction): the
    /// oracle's own reading of an azimuth difference.
    fn exact_difference(a: f64, b: f64) -> (f64, f64) {
        let parts = |x: f64| -> (i128, i32) {
            let bits = x.to_bits();
            let exponent = ((bits >> 52) & 0x7ff) as i32;
            let fraction = (bits & ((1u64 << 52) - 1)) as i128;
            let (mantissa, e) = if exponent == 0 {
                (fraction, -1074)
            } else {
                (fraction | (1i128 << 52), exponent - 1075)
            };
            (if x < 0.0 { -mantissa } else { mantissa }, e)
        };
        let ((ma, ea), (mb, eb)) = (parts(a), parts(b));
        let e = ea.min(eb);
        assert!(
            ea - e < 70 && eb - e < 70,
            "{a} and {b} are too far apart in scale"
        );
        let d = (ma << (ea - e)) - (mb << (eb - e));
        #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
        let (hi, rest) = {
            let hi = d as f64;
            (hi, (d - hi as i128) as f64)
        };
        let unit = 2f64.powi(e);
        (hi * unit, rest * unit)
    }

    /// The triangle's deviation by geometry, in a frame turned so its
    /// first corner is at azimuth zero and the others at their exact
    /// azimuths from it ([`exact_difference`]): the largest distance from
    /// the cone along each side (a ternary search of that concave
    /// function) and, where the triangle's plane meets the axis inside
    /// it, the distance there. It shares the premise that the largest
    /// distance is on a side or on the axis ([`cert_cone`], step 2; the
    /// grid row holds that) and none of its closed forms.
    fn cone_deviation_by_geometry(alpha: f64, uv: [[f64; 2]; 3]) -> f64 {
        let corner = |i: usize| {
            let (d, rest) = exact_difference(uv[i][0], uv[0][0]);
            let (sin, cos) = (d.sin() + rest * d.cos(), d.cos() - rest * d.sin());
            let v = uv[i][1].abs();
            Point3::new(
                v * alpha.sin() * cos,
                v * alpha.sin() * sin,
                v * alpha.cos(),
            )
        };
        let c = [corner(0), corner(1), corner(2)];
        let mut worst: f64 = 0.0;
        for (i, j) in [(0, 1), (1, 2), (2, 0)] {
            let along = |t: f64| cone_dist(alpha, c[i] + (c[j] - c[i]) * t);
            let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
            for _ in 0..200 {
                let (m1, m2) = (lo + (hi - lo) / 3.0, hi - (hi - lo) / 3.0);
                if along(m1) < along(m2) {
                    lo = m1;
                } else {
                    hi = m2;
                }
            }
            worst = worst.max(along(0.5 * (lo + hi)));
        }
        let cross = |i: usize, j: usize| c[i].x * c[j].y - c[j].x * c[i].y;
        let w = [cross(1, 2), cross(2, 0), cross(0, 1)];
        let sum = w[0] + w[1] + w[2];
        if sum != 0.0 && w.iter().all(|&x| x * sum >= 0.0) {
            let height = (w[0] * c[0].z + w[1] * c[1].z + w[2] * c[2].z) / sum;
            worst = worst.max(height * alpha.sin());
        }
        worst
    }

    /// **The axis term holds to the rounding of its own value where two
    /// corners are nearly one azimuth and the third nearly opposite.**
    /// There `sin(u_k − u_j)` is small against the rounding of the
    /// subtraction, and read off the rounded difference the term
    /// under-reports ([`sin_between`]). Three poses pinned against the
    /// closed form evaluated at 300 bits on the same `f64` inputs, each to
    /// `10⁻¹²`: the bare sine reads the first `1.1·10⁻⁸` short, the second
    /// `5·10⁻⁹` short and the third (an ulp-thin sliver, whose answer is a
    /// chord's) 14 % over. Then the family, a seeded fuzz: two corners
    /// `10⁻¹² … 10⁻⁴` rad apart at one radius and the third within a few
    /// of that of opposite, at azimuths to `±700` and radii `10⁻³ … 10⁶`,
    /// against [`cone_deviation_by_geometry`], to `10⁻⁹` of it either way.
    #[test]
    fn cert_cone_reads_the_axis_term_to_its_own_rounding() {
        let pinned = [
            (
                2.412_418_747_912_457_4e-4,
                [
                    [-1.990_817_571_118_756, -188_258.199_953_055_88],
                    [-1.990_817_578_359_271_2, -188_258.199_953_055_88],
                    [-5.132_410_229_531_247, -376_139.883_506_205_63],
                ],
                60.534_147_538_493_464,
            ),
            (
                0.253_354_387_621_294_53,
                [
                    [-2.097_144_693_703_569_4, 1.0],
                    [-2.097_144_663_224_8, 1.0],
                    [1.044_447_974_075_501_3, 0.176_507_381_390_651_03],
                ],
                0.072_808_231_871_773_27,
            ),
            (
                0.381_931_956_698_053_13,
                [
                    [-3.411_604_776_571_973_5, 0.045_294_931_643_037_516],
                    [-0.270_012_122_982_180_75, 0.763_176_745_809_110_2],
                    [-0.270_012_122_982_180_1, 0.813_800_464_264_726_7],
                ],
                0.029_679_364_596_004_516,
            ),
        ];
        for (alpha, uv, exact) in pinned {
            let cert = cert_cone(alpha, uv, [false; 3]);
            assert!(
                ((cert - exact) / exact).abs() <= 1e-12,
                "α {alpha} uv {uv:?}: {cert:e} against the 300-bit value {exact:e}"
            );
            let found = cone_deviation_by_geometry(alpha, uv);
            assert!(
                ((found - exact) / exact).abs() <= 1e-9,
                "α {alpha} uv {uv:?}: the oracle reads {found:e} against {exact:e}"
            );
        }
        let mut rng = test_utils::fuzz::start("cert_cone_axis");
        let mut about_axis = 0usize;
        let draws = test_utils::fuzz::scaled(2_500);
        for k in 0..draws {
            let alpha = if k % 4 == 0 {
                10f64.powf(rng.range(-3.7, -1.0))
            } else {
                rng.range(0.05, 1.5)
            };
            let u0 = rng.range(-700.0, 700.0);
            let v = 10f64.powf(rng.range(-3.0, 6.0));
            let x = 10f64.powf(rng.range(-12.0, -4.0));
            let third = v * 10f64.powf(rng.range(-3.0, 0.3));
            let mut uv = [
                [u0, v],
                [u0 + x, v],
                [u0 + PI + x * rng.range(-0.5, 1.5), third],
            ];
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let slot = ((rng.unit() * 3.0) as usize).min(2);
            uv.swap(0, slot);
            if rng.unit() < 0.5 {
                for corner in &mut uv {
                    corner[1] = -corner[1];
                }
            }
            let cert = cert_cone(alpha, uv, [false; 3]);
            let found = cone_deviation_by_geometry(alpha, uv);
            assert!(
                (cert - found).abs() <= 1e-9 * found,
                "α {alpha} uv {uv:?}: certificate {cert:e} against the geometry's {found:e} ({})",
                test_utils::fuzz::replay()
            );
            let k_ = alpha.sin() * alpha.cos();
            let chords = [(0, 1), (1, 2), (2, 0)]
                .map(|(i, j)| {
                    cone_chord_sag(k_, uv[i][1].abs(), uv[j][1].abs(), uv[j][0] - uv[i][0])
                })
                .into_iter()
                .fold(0.0, f64::max);
            about_axis += usize::from(cert > chords);
        }
        assert!(
            about_axis > draws / 4,
            "the axis term decides ({about_axis} of {draws})"
        );
    }

    /// **An input that is not finite poisons a certificate.** The cone's:
    /// a NaN or an infinity in the half-angle or in any corner, an apex
    /// corner's unread azimuth included, answers NaN, and slants whose
    /// product is past `f64`'s range answer NaN too; the verdict reads NaN
    /// as exceeded. The torus's and the spindle's answer NaN or `+∞`. A
    /// face's worst bound keeps a NaN once met ([`worse`]). The
    /// cylinder's and the sphere's answer NaN for a corner, anchor, axis
    /// or radius that is not finite, for a zero axis, and for corners
    /// past the range their distance holds products in
    /// ([`dist_point_triangle`]); a clamp `(r − dist).max(0.0)` alone
    /// reads each of those as 0 and reds here.
    #[test]
    fn the_certificates_poison_on_an_input_that_is_not_finite() {
        let (inf, nan) = (f64::INFINITY, f64::NAN);
        let off = [false; 3];
        let cases = [
            (0.5, [[inf, 1.0], [0.0, 1.0], [0.1, 1.0]], off),
            (0.5, [[-inf, 1.0], [0.0, 1.0], [0.1, 1.0]], off),
            (0.5, [[0.3, inf], [0.0, 1.0], [0.1, 1.0]], off),
            (0.5, [[0.0, inf], [0.0, 1.0], [0.1, 1.0]], off),
            (0.5, [[0.0, inf], [0.2, inf], [0.1, inf]], off),
            (nan, [[0.0, 1.0], [0.2, 1.0], [0.1, 0.5]], off),
            (inf, [[0.0, 1.0], [0.2, 1.0], [0.1, 0.5]], off),
            (0.5, [[0.0, 1e200], [0.2, 1e200], [0.1, 1e200]], off),
            (
                0.5,
                [[nan, 0.0], [0.2, 1.0], [0.1, 0.5]],
                [true, false, false],
            ),
            (
                0.5,
                [[inf, 0.0], [0.2, 1.0], [0.1, 0.5]],
                [true, false, false],
            ),
        ];
        for (alpha, uv, pole) in cases {
            let cert = cert_cone(alpha, uv, pole);
            assert!(cert.is_nan(), "α {alpha} uv {uv:?} apex {pole:?}: {cert}");
            assert_eq!(
                verdict(cert, 1.0, 1.0, Eps::exactly(1e-9)),
                Verdict::Exceeded
            );
        }
        let o = Point3::new(0.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);
        let on = [
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
        ];
        let poisoned = |cert: f64, what: &str| {
            assert!(cert.is_nan(), "{what}: {cert}");
            assert_eq!(
                verdict(cert, 1.0, 1.0, Eps::exactly(1e-9)),
                Verdict::Exceeded
            );
        };
        for bad in [nan, inf, -inf, 1e80, -1e200] {
            for slot in 0..3 {
                for axis in 0..3 {
                    let mut tri = on;
                    match axis {
                        0 => tri[slot].x = bad,
                        1 => tri[slot].y = bad,
                        _ => tri[slot].z = bad,
                    }
                    let what = format!("corner {slot}, coordinate {axis} = {bad}");
                    poisoned(cert_sphere(o, 1.0, tri), &what);
                    // A height past the range does not reach the shadow.
                    if axis != 1 || !bad.is_finite() {
                        poisoned(cert_cylinder(o, y, 1.0, tri), &what);
                    }
                }
            }
            poisoned(cert_sphere(Point3::new(bad, 0.0, 0.0), 1.0, on), "centre");
            poisoned(
                cert_cylinder(Point3::new(bad, 0.0, 0.0), y, 1.0, on),
                "anchor",
            );
        }
        for bad in [nan, inf, -inf] {
            poisoned(cert_sphere(o, bad, on), "sphere radius");
            poisoned(cert_cylinder(o, y, bad, on), "cylinder radius");
            poisoned(cert_cylinder(o, Vec3::new(0.0, bad, 0.0), 1.0, on), "axis");
        }
        poisoned(
            cert_cylinder(o, Vec3::new(0.0, 0.0, 0.0), 1.0, on),
            "a zero axis",
        );
        // The torus's and the spindle's answer no finite bound either.
        let flat = [[0.0, 0.0], [0.1, 0.0], [0.0, 0.1]];
        for bad in [nan, inf, -inf] {
            for slot in 0..2 {
                let mut uv = flat;
                uv[1][slot] = bad;
                assert!(!cert_torus(1.0, 0.2, uv).is_finite(), "torus, {uv:?}");
                assert!(
                    !cert_spindle(0.2, 1.0, uv, [false; 3]).is_finite(),
                    "spindle, {uv:?}"
                );
            }
            assert!(!cert_torus(bad, 0.2, flat).is_finite(), "torus, R = {bad}");
            assert!(!cert_torus(1.0, bad, flat).is_finite(), "torus, r = {bad}");
        }
        // A face's worst bound keeps a NaN whichever side it comes from.
        assert!(worse(f64::NAN, 1.0).is_nan() && worse(1.0, f64::NAN).is_nan());
        assert!(worse(worse(0.0, f64::NAN), 0.5).is_nan());
        assert_eq!(worse(0.25, 2.0), 2.0);
        assert!(cone_chord_sag(0.3, f64::NAN, 1.0, 0.1).is_nan());
        assert!(cone_chord_sag(0.3, 1.0, 1.0, f64::NAN).is_nan());
        assert_eq!(cone_chord_sag(0.3, 0.0, 0.0, 0.1), 0.0);
    }

    /// The six orders of a triangle's corners.
    fn orders(t: [Point3<f64>; 3]) -> [[Point3<f64>; 3]; 6] {
        [
            [0, 1, 2],
            [1, 2, 0],
            [2, 0, 1],
            [0, 2, 1],
            [2, 1, 0],
            [1, 0, 2],
        ]
        .map(|[i, j, k]| [t[i], t[j], t[k]])
    }

    /// **A triangle whose shadow has no area certifies at its true
    /// deviation, bit for bit the same in every corner order.** On a
    /// cylinder of radius 1 about `y`, columns a twelfth of a turn apart:
    ///
    /// - both halves of a lattice cell, each with a side on a ruling (the
    ///   shadow is the column's chord): the chord's sagitta
    ///   `1 − cos(π/12)`, and the two halves agree to the bit;
    /// - three corners on one ruling (the shadow is a point): 0;
    /// - a side on a ruling and a corner across the axis (the shadow is a
    ///   diameter): the radius;
    /// - a corner on the axis: the radius;
    /// - a triangle seen edge-on with its middle corner nearest the axis,
    ///   and one with an end corner nearest: the radius less that
    ///   corner's distance.
    ///
    /// On a sphere of radius 1: two corners at one point (a chord), three
    /// at one point (0), three in line with the middle one nearest, a
    /// triangle through the centre with the centre on a side and with it
    /// inside, and a corner at the centre (the radius, all three).
    ///
    /// The distance answering one fixed side for a triangle of no area
    /// reds the first case in four of its six orders.
    #[test]
    fn a_shadow_of_no_area_certifies_at_its_true_deviation_in_every_corner_order() {
        let o = Point3::new(0.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);
        let p = Point3::new;
        let (c, s) = (0.866_025_403_784_438_6, -0.5);
        let sagitta = 1.0 - (PI / 12.0).cos();
        let every_order =
            |cert: &dyn Fn([Point3<f64>; 3]) -> f64, tri, expected: f64, what: &str| {
                let first = cert(tri);
                assert!(
                    (first - expected).abs() <= 1e-15,
                    "{what}: {first:e}, not {expected:e}"
                );
                for order in orders(tri) {
                    let got = cert(order);
                    assert_eq!(got.to_bits(), first.to_bits(), "{what}, {order:?}: {got:e}");
                }
                first
            };
        let cylinder = |tri: [Point3<f64>; 3]| cert_cylinder(o, y, 1.0, tri);
        let lower = every_order(
            &cylinder,
            [p(1.0, 1.0, 0.0), p(1.0, 0.0, 0.0), p(c, 0.0, s)],
            sagitta,
            "the cell's lower half",
        );
        let upper = every_order(
            &cylinder,
            [p(1.0, 1.0, 0.0), p(c, 0.0, s), p(c, 1.0, s)],
            sagitta,
            "the cell's upper half",
        );
        assert_eq!(lower.to_bits(), upper.to_bits(), "a cell's two halves");
        let rows: [([Point3<f64>; 3], f64, &str); 5] = [
            (
                [p(1.0, 0.0, 0.0), p(1.0, 1.0, 0.0), p(1.0, 3.0, 0.0)],
                0.0,
                "three corners on one ruling",
            ),
            (
                [p(1.0, 0.0, 0.0), p(1.0, 1.0, 0.0), p(-1.0, 0.5, 0.0)],
                1.0,
                "a ruling and a corner across the axis",
            ),
            (
                [p(1.0, 0.0, 0.0), p(c, 0.0, s), p(0.0, 2.0, 0.0)],
                1.0,
                "a corner on the axis",
            ),
            (
                [p(0.5, 0.0, -0.75), p(0.5, 1.0, 0.0), p(0.5, 2.0, 0.5)],
                0.5,
                "edge-on, the middle corner nearest",
            ),
            (
                [p(0.5, 0.0, 0.5), p(0.5, 1.0, 0.75), p(0.5, 2.0, 0.625)],
                1.0 - 0.5_f64.sqrt(),
                "edge-on, an end corner nearest",
            ),
        ];
        for (tri, expected, what) in rows {
            every_order(&cylinder, tri, expected, what);
        }
        let sphere = |tri: [Point3<f64>; 3]| cert_sphere(o, 1.0, tri);
        let third = TAU / 3.0;
        let rows: [([Point3<f64>; 3], f64, &str); 6] = [
            (
                [p(1.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(c, 0.0, s)],
                sagitta,
                "two corners at one point",
            ),
            (
                [p(0.0, 0.0, 1.0), p(0.0, 0.0, 1.0), p(0.0, 0.0, 1.0)],
                0.0,
                "three corners at one point",
            ),
            (
                [p(0.5, -0.75, 0.0), p(0.5, 0.0, 0.0), p(0.5, 0.5, 0.0)],
                0.5,
                "three corners in line",
            ),
            (
                [p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), p(-1.0, 0.0, 0.0)],
                1.0,
                "the centre on a side",
            ),
            (
                [0.0, 1.0, 2.0].map(|i| p((i * third).cos(), (i * third).sin(), 0.0)),
                1.0,
                "the centre inside",
            ),
            (
                [p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), p(0.0, 0.0, 0.0)],
                1.0,
                "a corner at the centre",
            ),
        ];
        for (tri, expected, what) in rows {
            every_order(&sphere, tri, expected, what);
        }
    }

    /// A point and three orthonormal directions.
    struct Frame {
        o: Point3<f64>,
        e: [Vec3<f64>; 3],
    }

    impl Frame {
        fn identity() -> Self {
            Self {
                o: Point3::new(0.0, 0.0, 0.0),
                e: [
                    Vec3::new(1.0, 0.0, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(0.0, 0.0, 1.0),
                ],
            }
        }

        /// Any directions, about a point up to `reach` from the origin.
        fn random(rng: &mut test_utils::fuzz::Rng, reach: f64) -> Self {
            let mut any = || {
                Vec3::new(
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                )
            };
            // Two directions far enough apart that what is left of the
            // second across the first keeps its digits.
            let (e0, e1) = loop {
                let (a, b) = (any(), any());
                let e0 = a * (1.0 / a.norm());
                let b = b - e0 * b.dot(e0);
                if a.norm() > 0.5 && b.norm() > 0.5 {
                    break (e0, b * (1.0 / b.norm()));
                }
            };
            let o = Point3::new(0.0, 0.0, 0.0) + any() * reach;
            Self {
                o,
                e: [e0, e1, e0.cross(e1)],
            }
        }

        fn at(&self, x: f64, y: f64, z: f64) -> Point3<f64> {
            self.o + self.e[0] * x + self.e[1] * y + self.e[2] * z
        }
    }

    /// **[`dist_point_triangle`] and [`dist_line_triangle`] answer the
    /// distance, on triangles built to be degenerate** (a seeded fuzz).
    /// The nearest point is built first, so the distance is known: a
    /// point of the triangle (inside it, on a side, at a corner), and the
    /// point or line placed off it along a direction no corner lies
    /// ahead of, which makes that point the nearest. The triangles, a
    /// sixth of the draws each: any three points; a needle, two corners
    /// `10⁻¹⁷` to `10⁻⁵` of its length apart; three corners exactly in
    /// line; a sliver, one corner that close to the opposite side; two
    /// corners at one point; three at one point. Half the draws are then
    /// moved to any pose up to `10⁴` lengths from the origin, where the
    /// corners round. For the line the triangle is the shadow, its
    /// corners at any heights, and the direction is not unit.
    ///
    /// The answer is held to 64 epsilons of the pose's largest coordinate,
    /// plus what the foot's plane is uncertain by where the two shorter
    /// sides are near parallel (their length over the sine of their
    /// angle, in epsilons), never more than the inradius.
    ///
    /// Reading the foot's distance outside the bracket reds this row on
    /// the slivers; a fixed side for a triangle of no area reds it on the
    /// corners in line.
    #[test]
    fn the_two_distances_are_the_distance_on_triangles_built_to_be_degenerate() {
        let mut rng = test_utils::fuzz::start("dist_triangle_degenerate");
        let draws = test_utils::fuzz::scaled(24_000);
        let mut kept = [0usize; 3];
        for k in 0..draws {
            let l = rng.log_uniform(1e-3, 1e3);
            let thin = l * rng.log_uniform(1e-17, 1e-5);
            let a = [rng.range(-l, l), rng.range(-l, l)];
            let turn = rng.range(0.0, TAU);
            let (dx, dy) = (turn.cos(), turn.sin());
            let b = [a[0] + l * dx, a[1] + l * dy];
            let t = rng.unit();
            let mut tri: [[f64; 2]; 3] = match k % 6 {
                0 => [
                    a,
                    [rng.range(-l, l), rng.range(-l, l)],
                    [rng.range(-l, l), rng.range(-l, l)],
                ],
                1 => [a, [a[0] - thin * dy, a[1] + thin * dx], b],
                2 => [[a[0], 0.0], [a[0] + l * t, 0.0], [a[0] + l, 0.0]],
                3 => [
                    a,
                    b,
                    [a[0] + t * l * dx - thin * dy, a[1] + t * l * dy + thin * dx],
                ],
                4 => [a, a, b],
                _ => [a, a, a],
            };
            tri.rotate_left(rng.below(3));
            if rng.unit() < 0.5 {
                tri.swap(0, 1);
            }
            let sub = |p: [f64; 2], q: [f64; 2]| [p[0] - q[0], p[1] - q[1]];
            let dot = |p: [f64; 2], q: [f64; 2]| p[0] * q[0] + p[1] * q[1];
            let len = |p: [f64; 2]| dot(p, p).sqrt();
            let edges = [
                sub(tri[1], tri[0]),
                sub(tri[2], tri[1]),
                sub(tri[0], tri[2]),
            ];
            let area2 = (edges[0][0] * edges[1][1] - edges[0][1] * edges[1][0]).abs();
            // The nearest point `q`, and the unit direction `out` in the
            // triangle's plane the point is moved off it along.
            let mut feature = rng.below(3);
            if feature == 0 && area2 == 0.0 {
                feature = 1;
            }
            let side = rng.below(3);
            if feature == 1 && len(edges[side]) == 0.0 {
                feature = 2;
            }
            let ahead = |q: [f64; 2], out: [f64; 2]| {
                tri.iter().fold(0.0_f64, |m, &x| m.max(dot(out, sub(x, q))))
            };
            let unit = |p: [f64; 2]| {
                let n = len(p);
                if n > 0.0 {
                    [p[0] / n, p[1] / n]
                } else {
                    [0.0, 0.0]
                }
            };
            let (q, out) = match feature {
                0 => {
                    let w = [0; 3].map(|_| rng.unit() + 0.05);
                    let sum = w[0] + w[1] + w[2];
                    let q = [0, 1]
                        .map(|i| (w[0] * tri[0][i] + w[1] * tri[1][i] + w[2] * tri[2][i]) / sum);
                    (q, [0.0, 0.0])
                }
                1 => {
                    let (e, s) = (edges[side], rng.unit());
                    let q = [tri[side][0] + s * e[0], tri[side][1] + s * e[1]];
                    let out = unit([e[1], -e[0]]);
                    let back = [-out[0], -out[1]];
                    if ahead(q, out) <= ahead(q, back) {
                        (q, out)
                    } else {
                        (q, back)
                    }
                }
                _ => {
                    let q = tri[side];
                    let mut tries: Vec<[f64; 2]> = (0..6)
                        .map(|_| {
                            let t = rng.range(0.0, TAU);
                            [t.cos(), t.sin()]
                        })
                        .collect();
                    for e in [edges[side], edges[(side + 2) % 3]] {
                        let n = unit([e[1], -e[0]]);
                        tries.extend([n, [-n[0], -n[1]]]);
                    }
                    let best = tries
                        .into_iter()
                        .filter(|&d| len(d) > 0.0)
                        .min_by(|&x, &y| ahead(q, x).total_cmp(&ahead(q, y)))
                        .unwrap();
                    (q, best)
                }
            };
            // What a corner lies ahead of `q` by, which moves the distance
            // by at most twice that: rounding where `q` is the nearest
            // point, and a draw whose corner had no direction free is
            // dropped.
            let slack = ahead(q, out);
            if slack > 8.0 * f64::EPSILON * l {
                continue;
            }
            kept[feature] += 1;
            let longest = edges.iter().fold(0.0_f64, |m, &e| m.max(len(e)));
            let mut by_length = edges;
            by_length.sort_by(|&x, &y| len(x).total_cmp(&len(y)));
            let shorter = len(by_length[0]) * len(by_length[1]);
            let inradius = if area2 > 0.0 {
                area2 / (len(edges[0]) + len(edges[1]) + len(edges[2]))
            } else {
                0.0
            };
            let plane = if area2 > 0.0 {
                inradius.min(64.0 * f64::EPSILON * longest * shorter / area2)
            } else {
                0.0
            };
            let off = if feature == 0 {
                0.0
            } else {
                l * rng.log_uniform(1e-6, 1e2)
            };
            let up = if k % 4 == 0 {
                0.0
            } else {
                l * rng.log_uniform(1e-6, 1e2) * if rng.unit() < 0.5 { -1.0 } else { 1.0 }
            };
            let reach = l * rng.log_uniform(1e-3, 1e4);
            let frame = if rng.unit() < 0.5 {
                Frame::identity()
            } else {
                Frame::random(&mut rng, reach)
            };
            let from = [q[0] + off * out[0], q[1] + off * out[1]];
            let norm = |p: Point3<f64>| (p - Point3::new(0.0, 0.0, 0.0)).norm();

            let flat = tri.map(|x| frame.at(x[0], x[1], 0.0));
            let p = frame.at(from[0], from[1], up);
            let scale = flat.iter().fold(norm(p), |m, &x| m.max(norm(x)));
            let tol = 64.0 * f64::EPSILON * scale + 2.0 * slack + plane;
            let expected = off.hypot(up);
            let got = dist_point_triangle(p, flat[0], flat[1], flat[2]);
            assert!(
                (got - expected).abs() <= tol,
                "point: shape {} feature {feature}, {flat:?} from {p:?}: {got:e}, not \
                 {expected:e} within {tol:e} ({})",
                k % 6,
                test_utils::fuzz::replay()
            );

            for [a, b, c] in orders(flat) {
                assert_eq!(
                    dist_point_triangle(p, a, b, c).to_bits(),
                    got.to_bits(),
                    "point: {flat:?} from {p:?} answers by its corners' order ({})",
                    test_utils::fuzz::replay()
                );
            }

            let lifted = tri.map(|x| frame.at(x[0], x[1], l * rng.range(-10.0, 10.0)));
            let o = frame.at(from[0], from[1], l * rng.range(-10.0, 10.0));
            let d = frame.e[2] * rng.log_uniform(0.1, 10.0);
            let scale = lifted.iter().fold(norm(o), |m, &x| m.max(norm(x)));
            let tol = 64.0 * f64::EPSILON * scale + 2.0 * slack + plane;
            let got = dist_line_triangle(o, d, lifted[0], lifted[1], lifted[2]);
            assert!(
                (got - off).abs() <= tol,
                "line: shape {} feature {feature}, {lifted:?} from {o:?} along {d:?}: {got:e}, \
                 not {off:e} within {tol:e} ({})",
                k % 6,
                test_utils::fuzz::replay()
            );
            for [a, b, c] in orders(lifted) {
                assert_eq!(
                    dist_line_triangle(o, d, a, b, c).to_bits(),
                    got.to_bits(),
                    "line: {lifted:?} from {o:?} along {d:?} answers by its corners' order ({})",
                    test_utils::fuzz::replay()
                );
            }
        }
        assert!(
            kept.iter().all(|&n| n > draws / 12),
            "draws kept inside, on a side and at a corner: {kept:?} of {draws}"
        );
    }

    /// The largest `depth` over a flat triangle: a 325-point grid, then a
    /// pattern search from its best point (`depth` is concave on the
    /// triangle for a cylinder's and a sphere's inside).
    fn deepest(tri: [Point3<f64>; 3], depth: &dyn Fn(Point3<f64>) -> f64) -> f64 {
        let at = |s: f64, t: f64| {
            let a = Point3::new(0.0, 0.0, 0.0);
            depth(a + (tri[0] - a) * (1.0 - s - t) + (tri[1] - a) * s + (tri[2] - a) * t)
        };
        let n = 24_u32;
        let mut best = (f64::NEG_INFINITY, 0.0, 0.0);
        for i in 0..=n {
            for j in 0..=(n - i) {
                let (s, t) = (f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
                let d = at(s, t);
                if d > best.0 {
                    best = (d, s, t);
                }
            }
        }
        let mut step = 1.0 / f64::from(n);
        while step > 1e-13 {
            let (_, s, t) = best;
            let mut moved = false;
            for (ds, dt) in [
                (1.0, 0.0),
                (-1.0, 0.0),
                (0.0, 1.0),
                (0.0, -1.0),
                (1.0, -1.0),
                (-1.0, 1.0),
            ] {
                let (s1, t1) = (s + ds * step, t + dt * step);
                if s1 >= 0.0 && t1 >= 0.0 && s1 + t1 <= 1.0 {
                    let d = at(s1, t1);
                    if d > best.0 {
                        best = (d, s1, t1);
                        moved = true;
                    }
                }
            }
            if !moved {
                step *= 0.5;
            }
        }
        best.0
    }

    /// **[`cert_cylinder`] and [`cert_sphere`] are the true deviation of
    /// triangles built to be degenerate** (a seeded fuzz): corners on the
    /// surface, radius `10⁻³` to `10³`, a third of the draws about the
    /// origin along `z` and the rest in any pose up to `10³` radii off.
    /// The deviation is found on the triangle, blind to the certificate
    /// ([`deepest`]), and the certificate is neither under it nor over it.
    ///
    /// Cylinder triangles, by their corners' azimuths, at any heights: a
    /// side on a ruling; three corners on one ruling; a side `10⁻¹⁷` to
    /// `10⁻⁶` rad off a ruling; a side on or beside a ruling with the
    /// third corner across the axis to that much; three corners about the
    /// axis; any three. The cylinder's deviation is also `2r·sin²(s/4)`,
    /// `s` the arc the three azimuths span, or `r` where they span no
    /// half turn, and the certificate is held to that in 64 epsilons of
    /// the pose's largest coordinate.
    ///
    /// Sphere triangles: two corners at one point; two that close; three
    /// at one point; three on a great circle within half a turn; three
    /// about the centre on one; three in a cap `10⁻⁶` to 1 rad wide; any
    /// three.
    ///
    /// The clamp reading a NaN as 0 does not red this row (the distance
    /// answers none); a fixed side for a shadow of no area does.
    #[test]
    fn the_cylinder_and_sphere_certificates_are_the_deviation_on_triangles_built_to_be_degenerate()
    {
        let mut rng = test_utils::fuzz::start("cert_cylinder_sphere_degenerate");
        let draws = test_utils::fuzz::scaled(4_200);
        let mut measurable = 0usize;
        let third = TAU / 3.0;
        for k in 0..draws {
            let r = rng.log_uniform(1e-3, 1e3);
            let reach = r * rng.log_uniform(1e-2, 1e3);
            let frame = if k % 3 == 0 {
                Frame::identity()
            } else {
                Frame::random(&mut rng, reach)
            };
            let tiny = rng.log_uniform(1e-17, 1e-6);
            let u0 = rng.range(-PI, PI);
            let az = match k % 6 {
                0 => [u0, u0, u0 + rng.range(0.0, 2.0)],
                1 => [u0, u0, u0],
                2 => [u0, u0 + tiny, u0 + rng.range(0.0, 2.0)],
                3 => [
                    u0,
                    u0 + tiny * rng.below(2) as f64,
                    u0 + PI + tiny * rng.range(-1.0, 1.0),
                ],
                4 => [0.0, 1.0, 2.0].map(|i| u0 + i * third + rng.range(-0.9, 0.9)),
                _ => [0.0; 3].map(|_| u0 + rng.range(0.0, 3.0)),
            };
            let mut tri =
                az.map(|u| frame.at(r * u.cos(), r * u.sin(), r * rng.range(-10.0, 10.0)));
            tri.rotate_left(rng.below(3));
            if rng.unit() < 0.5 {
                tri.swap(0, 1);
            }
            let axis = frame.e[2];
            let cert = cert_cylinder(frame.o, axis, r, tri);
            let tol = 64.0 * f64::EPSILON * position_scale(frame.o, tri);
            let mut turn = az.map(|u| u.rem_euclid(TAU));
            turn.sort_by(f64::total_cmp);
            let gap = (turn[1] - turn[0])
                .max(turn[2] - turn[1])
                .max(turn[0] + TAU - turn[2]);
            let closed = if gap >= PI {
                2.0 * r * ((TAU - gap) / 4.0).sin().powi(2)
            } else {
                r
            };
            let line = geom::Line {
                origin: frame.o,
                dir: axis,
            };
            let dev = deepest(tri, &|q| r - line.offset(q).norm());
            assert!(
                (cert - closed).abs() <= tol && dev <= cert + tol && cert <= dev + tol + 1e-9 * r,
                "cylinder, shape {}: r {r} about {:?} along {axis:?}, {tri:?}: certificate \
                 {cert:e}, closed form {closed:e}, found {dev:e} ({})",
                k % 6,
                frame.o,
                test_utils::fuzz::replay()
            );
            measurable += usize::from(dev > 1e-6 * r);

            let mut any = || {
                let v = Vec3::new(
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                );
                v * (1.0 / v.norm())
            };
            let (a, b, c) = (any(), any(), any());
            let near = |a: Vec3<f64>, b: Vec3<f64>, by: f64| {
                let v = a + b * by;
                v * (1.0 / v.norm())
            };
            let on_circle = |t: f64| Vec3::new(t.cos(), t.sin(), 0.0);
            let cap = rng.log_uniform(1e-6, 1.0);
            let units = match k % 7 {
                0 => [a, a, b],
                1 => [a, near(a, b, tiny), c],
                2 => [a, a, a],
                3 => [0.0; 3].map(|_| on_circle(u0 + rng.range(0.0, 3.0))),
                4 => [0.0, 1.0, 2.0].map(|i| on_circle(u0 + i * third + rng.range(-0.9, 0.9))),
                5 => [a, near(a, b, cap), near(a, c, cap)],
                _ => [a, b, c],
            };
            let mut tri = units.map(|v| frame.at(r * v.x, r * v.y, r * v.z));
            tri.rotate_left(rng.below(3));
            if rng.unit() < 0.5 {
                tri.swap(0, 1);
            }
            let cert = cert_sphere(frame.o, r, tri);
            let tol = 64.0 * f64::EPSILON * position_scale(frame.o, tri);
            let dev = deepest(tri, &|q| r - (q - frame.o).norm());
            assert!(
                dev <= cert + tol && cert <= dev + tol + 1e-9 * r,
                "sphere, shape {}: r {r} about {:?}, {tri:?}: certificate {cert:e}, found \
                 {dev:e} ({})",
                k % 7,
                frame.o,
                test_utils::fuzz::replay()
            );
            measurable += usize::from(dev > 1e-6 * r);
        }
        assert!(
            measurable > draws,
            "the triangles deviate measurably ({measurable} of {} certificates), so an \
             under-reporting certificate reds",
            2 * draws
        );
    }

    /// **A chord's sag is at most its larger radius's sagitta across the
    /// same `Δu`**, with equality at one radius: the statement
    /// `sizing::cone_grid_step`'s theorem and the curved lane's
    /// no-rows argument read ([`cone_chord_sag`]). And the certificate's
    /// edge cases: corners on both nappes certify nothing, a poisoned
    /// corner poisons, three corners of one ruling or at the apex deviate
    /// by nothing.
    #[test]
    fn a_cone_chord_sags_within_its_larger_radius_sagitta() {
        let k = 0.3;
        for du in [1e-4, 0.05, 0.7, 2.0, PI] {
            // The sagitta `k·v·(1 − cos(du/2))` in the form that does not
            // cancel, `2·sin²(du/4)`.
            let sagitta = |v: f64| k * v * 2.0 * (0.25 * du).sin().powi(2);
            for (va, vb) in [
                (1.0, 1.0),
                (0.999, 1.0),
                (0.5, 1.0),
                (1e-6, 1.0),
                (0.0, 1.0),
            ] {
                let sag = cone_chord_sag(k, va, vb, du);
                assert!(
                    sag <= sagitta(vb) * (1.0 + 1e-12),
                    "slants {va}, {vb} across {du}: {sag:e} past {:e}",
                    sagitta(vb)
                );
                assert_eq!(cone_chord_sag(k, vb, va, -du), sag, "symmetric");
            }
            let (one, same) = (cone_chord_sag(k, 2.0, 2.0, du), sagitta(2.0));
            assert!(
                (one - same).abs() <= 1e-12 * same,
                "one radius across {du}: {one:e} against its sagitta {same:e}"
            );
        }
        let across = [[0.0, 1.0], [0.3, -1.0], [0.1, 0.5]];
        assert_eq!(cert_cone(0.5, across, [false; 3]), f64::INFINITY);
        assert!(cert_cone(0.5, across, [false, true, false]).is_finite());
        assert!(cert_cone(0.5, [[0.0, 1.0], [f64::NAN, 1.0], [0.1, 0.5]], [false; 3]).is_nan());
        assert!(
            cert_cone(
                0.5,
                [[0.0, 1.0], [0.1, f64::NAN], [0.1, 0.5]],
                [false, true, false]
            )
            .is_nan()
        );
        assert_eq!(
            cert_cone(0.5, [[1.0, 0.2], [1.0, 3.0], [1.0, 1.0]], [false; 3]),
            0.0
        );
        assert_eq!(
            cert_cone(0.5, [[1.0, 0.0], [2.0, 0.0], [3.0, 0.0]], [true; 3]),
            0.0
        );
    }

    /// **A triangle with fewer than two corners off an apex certifies
    /// nothing**: both apexes and a third corner span the sheet's whole
    /// `v` window, which no fan triangle does.
    #[test]
    fn cert_spindle_refuses_a_triangle_on_two_apexes() {
        let vs = (0.5_f64).acos();
        let uv = [[0.0, vs], [0.3, -vs], [0.1, 0.0]];
        assert_eq!(
            cert_spindle(-0.5, 1.0, uv, [true, true, false]),
            f64::INFINITY
        );
        assert!(cert_spindle(-0.5, 1.0, uv, [true, false, false]).is_finite());
    }
}

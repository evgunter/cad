//! M2 PR 7 e2e review — unit-level falsification of the per-face
//! closed forms in `geom_brep::props` against an INDEPENDENT numeric
//! oracle (dense Simpson quadrature over the chart, derived from the
//! surface parameterizations alone — never from the implementer's
//! formulas). Attacks: both interior sides (`s_f = ±1`) on every
//! surface, the cone "no `s_f` needed" claim on BOTH nappes, sphere
//! bands crossing the equator and touching a pole, arbitrary torus
//! minor intervals (incl. inner-half tube patches), reflex/major arcs
//! in the planar vector-area form, multi-hole caps, antiparallel rim
//! carriers, and out-of-inventory boundaries (typed refusal, never a
//! silent number).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_3, PI, TAU};

use geom::Curve3;
use geom::Surface;
use geom_brep::props::{FaceContribution, LoopEdge, PropsError, curved_face, planar_face};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

// ---------------------------------------------------------------------
// Independent oracle: Simpson quadrature of area and outward flux over
// the chart rectangle [u0,u1]×[v0,v1]. `sigma` is the outward normal's
// sign against the chart normal — MY construction's ground truth.
// ---------------------------------------------------------------------

/// Composite Simpson over the rectangle; `f(u, v)` integrand.
fn simpson2(mut f: impl FnMut(f64, f64) -> f64, r: [f64; 4], n: usize) -> f64 {
    let [u0, u1, v0, v1] = r;
    let (hu, hv) = ((u1 - u0) / n as f64, (v1 - v0) / n as f64);
    let w = |i: usize| {
        if i == 0 || i == n {
            1.0
        } else if i % 2 == 1 {
            4.0
        } else {
            2.0
        }
    };
    let mut acc = 0.0;
    for i in 0..=n {
        let u = u0 + hu * i as f64;
        for j in 0..=n {
            let v = v0 + hv * j as f64;
            acc += w(i) * w(j) * f(u, v);
        }
    }
    acc * hu * hv / 9.0
}

/// Point, unit chart normal, and Jacobian of the surface at (u, v),
/// derived from the documented parameterizations (`geom`'s `surfaces` module).
fn chart(s: &Surface<f64>, u: f64, v: f64) -> (Point3<f64>, Vec3<f64>, f64) {
    let frame = |axis: Vec3<f64>, u_ref: Vec3<f64>, u: f64| {
        let v_ref = axis.cross(u_ref);
        let radial = u_ref * u.cos() + v_ref * u.sin();
        let tangential = axis.cross(radial);
        (radial, tangential)
    };
    match *s {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => {
            let (radial, _) = frame(axis, u_ref, u);
            (origin + radial * radius + axis * v, radial, radius)
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            let (sa, ca) = half_angle.sin_cos();
            let (radial, _) = frame(axis, u_ref, u);
            let p = apex + axis * (v * ca) + radial * (v * sa);
            // ∂u×∂v = v·sinα·(radial·cosα − axis·sinα): normal flips
            // with the nappe, |J| = |v|·sinα.
            let n = (radial * ca - axis * sa) * v.signum();
            (p, n, v.abs() * sa)
        }
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => {
            let (sv, cv) = v.sin_cos();
            let (radial, _) = frame(axis, u_ref, u);
            let n = radial * cv + axis * sv;
            (center + n * radius, n, radius * radius * cv)
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            let (sv, cv) = v.sin_cos();
            let (radial, _) = frame(axis, u_ref, u);
            let n = radial * cv + axis * sv;
            let p =
                center + radial * (major_radius + minor_radius * cv) + axis * (minor_radius * sv);
            (p, n, minor_radius * (major_radius + minor_radius * cv))
        }
        _ => panic!("oracle: unsupported surface"),
    }
}

/// Oracle (area, flux) for the patch with outward normal `sigma`×chart.
fn oracle(s: &Surface<f64>, rect: [f64; 4], sigma: f64) -> (f64, f64) {
    let area = simpson2(|u, v| chart(s, u, v).2, rect, 256);
    let flux = simpson2(
        |u, v| {
            let (p, n, j) = chart(s, u, v);
            sigma * (p - Point3::origin()).dot(n) * j
        },
        rect,
        256,
    );
    (area, flux)
}

fn assert_rel(what: &str, got: f64, want: f64, tol: f64) {
    let scale = want.abs().max(1.0);
    assert!(
        (got - want).abs() <= tol * scale,
        "{what}: got {got}, oracle {want} (rel {})",
        ((got - want) / scale).abs()
    );
}

// ---------------------------------------------------------------------
// Boundary construction: the chart-CCW iso-rectangle loop, reversed
// wholesale for sigma = −1. `flip_rim_carriers` stores each rim with an
// ANTIPARALLEL carrier axis instead (same locus, reversed traversal
// flag) — a legal alternative encoding the classifier must also read.
// ---------------------------------------------------------------------

struct PatchLoop {
    edges: Vec<LoopEdge<f64>>,
}

/// Build the boundary loop of `s` over `rect` with outward = sigma ×
/// chart normal. Tags: corners 0..4 = (u0,v0), (u1,v0), (u1,v1),
/// (u0,v1).
fn patch_loop(s: &Surface<f64>, rect: [f64; 4], sigma: f64, flip_rims: bool) -> PatchLoop {
    let [u0, u1, v0, v1] = rect;
    // Chart-CCW cycle: rim v0 (+u), meridian u1 (+v), rim v1 (−u),
    // meridian u0 (−v).
    let mut edges = vec![
        rim_edge(s, v0, u0, u1, true, (0, 1), flip_rims),
        meridian_edge(s, u1, v0, v1, true, (1, 2)),
        rim_edge(s, v1, u0, u1, false, (2, 3), flip_rims),
        meridian_edge(s, u0, v0, v1, false, (3, 0)),
    ];
    if sigma < 0.0 {
        edges.reverse();
        for e in &mut edges {
            e.forward = !e.forward;
            core::mem::swap(&mut e.start, &mut e.end);
        }
    }
    PatchLoop { edges }
}

/// The iso-v rim over [u0,u1]; `up` = traversed in +u. Carrier axis =
/// surface axis (or antiparallel when `flip`).
fn rim_edge(
    s: &Surface<f64>,
    v: f64,
    u0: f64,
    u1: f64,
    up: bool,
    tags: (u32, u32),
    flip: bool,
) -> LoopEdge<f64> {
    let (center, axis, radius, u_ref) = match *s {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => (origin + axis * v, axis, radius, u_ref),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => (
            apex + axis * (v * half_angle.cos()),
            axis,
            (v * half_angle.sin()).abs(),
            // Lower nappe: the chart point at azimuth u lies along
            // −radial(u) from the axis (v·sinα < 0), so the carrier's
            // seam frame is negated to keep carrier param t ≡ chart u.
            if v < 0.0 { -u_ref } else { u_ref },
        ),
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => (
            center + axis * (radius * v.sin()),
            axis,
            radius * v.cos(),
            u_ref,
        ),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => (
            center + axis * (minor_radius * v.sin()),
            axis,
            major_radius + minor_radius * v.cos(),
            u_ref,
        ),
        _ => panic!("rim_edge: unsupported"),
    };
    if flip {
        // Antiparallel carrier: eval(t) sits at azimuth −t, so the
        // carrier interval [−u1, −u0] covers the same arc; traversal
        // +u is the DECREASING-t direction (forward = !up).
        LoopEdge::hand_built(
            Curve3::Circle {
                center,
                axis: -axis,
                radius,
                u_ref,
            },
            -u1,
            -u0,
            !up,
            tags.0,
            tags.1,
        )
    } else {
        LoopEdge::hand_built(
            Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            },
            u0,
            u1,
            up,
            tags.0,
            tags.1,
        )
    }
}

/// The iso-u meridian over [v0,v1]; `up` = traversed in +v.
fn meridian_edge(
    s: &Surface<f64>,
    u: f64,
    v0: f64,
    v1: f64,
    up: bool,
    tags: (u32, u32),
) -> LoopEdge<f64> {
    let frame = |axis: Vec3<f64>, u_ref: Vec3<f64>| {
        let v_ref = axis.cross(u_ref);
        let radial = u_ref * u.cos() + v_ref * u.sin();
        (radial, axis.cross(radial))
    };
    let (carrier, t0, t1) = match *s {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => {
            let (radial, _) = frame(axis, u_ref);
            (
                Curve3::Line {
                    origin: origin + radial * radius,
                    dir: axis,
                },
                v0,
                v1,
            )
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            let (sa, ca) = half_angle.sin_cos();
            let (radial, _) = frame(axis, u_ref);
            let dir = axis * ca + radial * sa; // unit generator; p(v) = apex + dir·v
            (Curve3::Line { origin: apex, dir }, v0, v1)
        }
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => {
            // Great circle through the poles in the plane of (radial,
            // axis): eval(t) = center + (radial·cos t + axis·sin t)·R —
            // carrier u_ref = radial, carrier axis = radial × axis ...
            // must satisfy carrier_axis × carrier_uref = second frame
            // vector = axis. radial × axis = −tangential; (−tangential)
            // × radial = ... use n = radial.cross(axis).neg? Pick
            // n = radial.cross(axis): n × radial should equal axis.
            let (radial, tangential) = frame(axis, u_ref);
            let n = radial.cross(axis).normalize();
            // n = -tangential (since axis×radial = tangential ⇒
            // radial×axis = −tangential). (−tangential)×radial = −axis?
            // tangential×radial = −axis ⇒ n×radial = +axis. Good.
            let _ = tangential;
            (
                Curve3::Circle {
                    center,
                    axis: n,
                    radius,
                    u_ref: radial,
                },
                v0,
                v1,
            )
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            let (radial, _) = frame(axis, u_ref);
            let n = radial.cross(axis).normalize(); // n×radial = axis: minor angle = t
            (
                Curve3::Circle {
                    center: center + radial * major_radius,
                    axis: n,
                    radius: minor_radius,
                    u_ref: radial,
                },
                v0,
                v1,
            )
        }
        _ => panic!("meridian_edge: unsupported"),
    };
    // `tags` are already traversal-order (start, end).
    LoopEdge::hand_built(carrier, t0, t1, up, tags.0, tags.1)
}

fn check_patch(what: &str, s: &Surface<f64>, rect: [f64; 4], sigma: f64, flip_rims: bool) {
    let lp = patch_loop(s, rect, sigma, flip_rims);
    let got: FaceContribution<f64> = curved_face(s, &lp.edges, 1.0, band())
        .unwrap_or_else(|e| panic!("{what}: closed form refused a legal iso-rectangle: {e:?}"));
    let (area, flux) = oracle(s, rect, sigma);
    assert_rel(&format!("{what} area"), got.area, area, 1e-9);
    assert_rel(&format!("{what} flux"), got.flux, flux, 1e-9);
}

// ---------------------------------------------------------------------
// The surfaces under test (deliberately off-origin, tilted axes).
// ---------------------------------------------------------------------

fn tilted_frame() -> (Vec3<f64>, Vec3<f64>) {
    let axis = Vec3::new(1.0, 2.0, 2.0).normalize();
    let u_ref = {
        let raw = Vec3::new(0.0, 1.0, 0.0);
        (raw - axis * raw.dot(axis)).normalize()
    };
    (axis, u_ref)
}

fn cyl() -> Surface<f64> {
    let (axis, u_ref) = tilted_frame();
    Surface::Cylinder {
        origin: Point3::new(0.3, -0.7, 1.1),
        axis,
        radius: 0.8,
        u_ref,
    }
}

fn cone_s() -> Surface<f64> {
    let (axis, u_ref) = tilted_frame();
    Surface::Cone {
        apex: Point3::new(-0.4, 0.9, 0.2),
        axis,
        half_angle: FRAC_PI_3 * 0.7,
        u_ref,
    }
}

fn sph() -> Surface<f64> {
    let (axis, u_ref) = tilted_frame();
    Surface::Sphere {
        center: Point3::new(1.2, 0.4, -0.6),
        radius: 1.3,
        axis,
        u_ref,
    }
}

fn tor() -> Surface<f64> {
    let (axis, u_ref) = tilted_frame();
    Surface::Torus {
        center: Point3::new(0.5, -1.1, 0.7),
        axis,
        major_radius: 2.0,
        minor_radius: 0.6,
        u_ref,
    }
}

// ---------------------------------------------------------------------
// Assignment 1/2: closed forms vs the oracle, both interior sides.
// ---------------------------------------------------------------------

#[test]
fn cylinder_patch_matches_oracle_both_sides_and_flipped_carriers() {
    let s = cyl();
    for &rect in &[
        [0.2, 1.9, -0.5, 1.4],
        [-1.0, 5.0, 0.0, 0.3], // reflex u-span (> π)
    ] {
        for &sigma in &[1.0, -1.0] {
            for &flip in &[false, true] {
                check_patch(
                    &format!("cylinder {rect:?} sigma={sigma} flip={flip}"),
                    &s,
                    rect,
                    sigma,
                    flip,
                );
            }
        }
    }
}

#[test]
fn cone_patch_matches_oracle_on_both_nappes_and_sides() {
    let s = cone_s();
    // Upper nappe (v > 0), lower nappe (v < 0): the "flux = apex·A⃗,
    // no s_f" claim must hold for all four (nappe × side) cases.
    for &rect in &[[0.3, 2.4, 0.5, 1.7], [0.3, 2.4, -1.7, -0.5]] {
        for &sigma in &[1.0, -1.0] {
            check_patch(
                &format!("cone {rect:?} sigma={sigma}"),
                &s,
                rect,
                sigma,
                false,
            );
        }
    }
}

#[test]
fn cone_nappe_spanning_is_refused_typed() {
    let s = cone_s();
    let lp = patch_loop(&s, [0.3, 2.4, -1.0, 1.0], 1.0, false);
    // v-range definitely straddles the apex: refuse, never integrate.
    // (The rims are honest circles of |v|·sinα on both nappes.)
    match curved_face(&s, &lp.edges, 1.0, band()) {
        Err(PropsError::NappeSpanning) => {}
        other => panic!("expected NappeSpanning, got {other:?}"),
    }
}

#[test]
fn sphere_band_matches_oracle_equator_crossing_and_polar() {
    let s = sph();
    for &rect in &[
        [0.4, 2.9, -0.6, 0.9],          // equator-crossing band
        [0.0, FRAC_PI_2, 1.0, 1.570_6], // near-pole wedge (v_hi just shy of π/2)
        [-2.0, 2.0, -1.5, -0.2],        // southern band, reflex u-span
    ] {
        for &sigma in &[1.0, -1.0] {
            check_patch(
                &format!("sphere {rect:?} sigma={sigma}"),
                &s,
                rect,
                sigma,
                false,
            );
        }
    }
}

#[test]
fn torus_patch_matches_oracle_arbitrary_minor_intervals() {
    let s = tor();
    for &rect in &[
        [0.1, 1.8, -0.7, 0.9],      // outer-equator patch
        [0.5, 2.0, 2.0, 4.4],       // inner-half tube (cos v < 0)
        [-1.0, 4.0, -2.5, 1.0],     // reflex u AND wide v (crosses π)
        [0.0, TAU - 0.2, 0.3, 1.1], // near-full major span
    ] {
        for &sigma in &[1.0, -1.0] {
            for &flip in &[false, true] {
                check_patch(
                    &format!("torus {rect:?} sigma={sigma} flip={flip}"),
                    &s,
                    rect,
                    sigma,
                    flip,
                );
            }
        }
    }
}

// ---------------------------------------------------------------------
// Assignment 1: planar forms — reflex/major arcs, multi-hole caps.
// ---------------------------------------------------------------------

fn line_edge(a: Point3<f64>, b: Point3<f64>) -> LoopEdge<f64> {
    let d = b - a;
    let len = d.norm();
    LoopEdge::hand_built(
        Curve3::Line {
            origin: a,
            dir: d * (1.0 / len),
        },
        0.0,
        len,
        true,
        0,
        0,
    )
}

/// Full circle in the z = h plane as a single self-loop edge, CCW
/// (+z) when `ccw`.
fn circle_loop(cx: f64, cy: f64, h: f64, r: f64, ccw: bool) -> Vec<LoopEdge<f64>> {
    vec![LoopEdge::hand_built(
        Curve3::Circle {
            center: Point3::new(cx, cy, h),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: r,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        0.0,
        TAU,
        ccw,
        0,
        0,
    )]
}

#[test]
fn planar_cap_with_two_holes_subtracts_ring_areas() {
    // Square cap [0,4]² at z = 2, outward +z, two circular holes.
    let z = 2.0;
    let p = |x: f64, y: f64| Point3::new(x, y, z);
    let outer = vec![
        line_edge(p(0.0, 0.0), p(4.0, 0.0)),
        line_edge(p(4.0, 0.0), p(4.0, 4.0)),
        line_edge(p(4.0, 4.0), p(0.0, 4.0)),
        line_edge(p(0.0, 4.0), p(0.0, 0.0)),
    ];
    let hole1 = circle_loop(1.0, 1.0, z, 0.5, false); // CW: subtracts
    let hole2 = circle_loop(3.0, 2.5, z, 0.8, false);
    let c = planar_face(p(0.0, 0.0), &[outer, hole1, hole2]).unwrap();
    let area = 16.0 - PI * (0.25 + 0.64);
    assert_rel("two-hole cap area", c.area, area, 1e-12);
    // flux = z · A⃗_z (outward +z at height z).
    assert_rel("two-hole cap flux", c.flux, z * area, 1e-12);
}

#[test]
fn reflex_major_arc_vector_area_matches_dense_polyline() {
    // A planar loop: chord + major (reflex) arc spanning 3π/2, in the
    // z = 0 plane. Closed form vs a 20000-segment polyline shoelace.
    let r = 1.5;
    let (t0, t1) = (0.25, 0.25 + 1.5 * PI);
    let c = Point3::new(0.4, -0.2, 0.0);
    let arc = LoopEdge::hand_built(
        Curve3::Circle {
            center: c,
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: r,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        t0,
        t1,
        true,
        0,
        1,
    );
    let a = arc.carrier.eval(t1);
    let b = arc.carrier.eval(t0);
    let closing = line_edge(a, b);
    let got =
        geom_brep::props::loop_vector_area(&[arc.clone(), closing], Point3::new(9.0, 9.0, 9.0))
            .unwrap();
    // Dense polyline oracle.
    let mut acc = Vec3::zero();
    let n = 20000;
    let refp = Point3::new(9.0, 9.0, 9.0);
    let mut prev = arc.carrier.eval(t0) - refp;
    for i in 1..=n {
        let t = t0 + (t1 - t0) * i as f64 / n as f64;
        let cur = arc.carrier.eval(t) - refp;
        acc = acc + prev.cross(cur) * 0.5;
        prev = cur;
    }
    let a_ = a - refp;
    let b_ = b - refp;
    acc = acc + a_.cross(b_) * 0.5;
    assert_rel("reflex arc va.z", got.z, acc.z, 1e-6);
    assert_rel("reflex arc va.x", got.x, acc.x, 1e-6);
    assert_rel("reflex arc va.y", got.y, acc.y, 1e-6);
}

// ---------------------------------------------------------------------
// Assignment 3 (unit level): out-of-inventory boundaries refuse typed.
// ---------------------------------------------------------------------

#[test]
fn out_of_inventory_boundaries_refuse_typed() {
    let s = cyl();
    let Surface::Cylinder { origin, u_ref, .. } = s else {
        panic!("cylinder expected")
    };
    let b = band();
    let _ = origin;
    // (b) wrong-radius rim.
    let mut lp = patch_loop(&s, [0.0, 1.0, 0.0, 1.0], 1.0, false);
    if let Curve3::Circle { radius, .. } = &mut lp.edges[0].carrier {
        *radius += 0.05;
    }
    assert!(
        matches!(
            curved_face(&s, &lp.edges, 1.0, b),
            Err(PropsError::NotIsoRectangle { .. })
        ),
        "wrong-radius rim must refuse"
    );
    // (c) non-axial line on a cylinder.
    let mut lp = patch_loop(&s, [0.0, 1.0, 0.0, 1.0], 1.0, false);
    if let Curve3::Line { dir, .. } = &mut lp.edges[1].carrier {
        *dir = (*dir + u_ref * 0.2).normalize();
    }
    assert!(
        matches!(
            curved_face(&s, &lp.edges, 1.0, b),
            Err(PropsError::NotIsoRectangle { .. })
        ),
        "tilted meridian must refuse"
    );
    // (d) line edge on a sphere.
    let sp = sph();
    let e = line_edge(Point3::origin(), Point3::new(1.0, 0.0, 0.0));
    assert!(
        matches!(
            curved_face(&sp, &[e], 1.0, b),
            Err(PropsError::NotIsoRectangle { .. })
        ),
        "line on sphere must refuse"
    );
    // (e) torus face without a meridian (two full rims only — a
    // full-period tube band: outside the M2 inventory reachable here).
    let t = tor();
    let rims = vec![
        rim_edge(&t, 0.2, 0.0, TAU, true, (0, 0), false),
        rim_edge(&t, 1.0, 0.0, TAU, false, (1, 1), false),
    ];
    assert!(
        matches!(
            curved_face(&t, &rims, 1.0, b),
            Err(PropsError::NotIsoRectangle { .. })
        ),
        "torus without meridian must refuse"
    );
    // (f) degenerate extent (zero-height cylinder patch).
    let lp = patch_loop(&s, [0.0, 1.0, 0.5, 0.5], 1.0, false);
    assert!(
        matches!(
            curved_face(&s, &lp.edges, 1.0, b),
            Err(PropsError::DegenerateFace)
        ),
        "zero-extent face must refuse"
    );
    // (g) Nurbs carrier.
    let e = LoopEdge::hand_built(Curve3::nurbs_placeholder(), 0.0, 1.0, true, 0, 1);
    assert!(matches!(
        curved_face(&s, &[e], 1.0, b),
        Err(PropsError::Unimplemented)
    ));
}

/// REVIEW FINDING (M2 PR 7, fixed in the fix pass): the structural
/// iso-rectangle gate checked carrier SHAPE consistency (radius fits,
/// axis classes) but not INCIDENCE on the surface — a boundary circle
/// with the right radius and an axis-parallel carrier axis but an
/// off-axis center (a curve nowhere on the cylinder) was accepted
/// silently with a fabricated contribution. The fix adds the
/// `props_rim_center_on_axis` / `props_rim_axis_parallel` /
/// per-surface meridian incidence residuals; the off-axis rim now
/// refuses typed (`NotIsoRectangle`), matching the module docs.
#[test]
fn off_surface_boundaries_must_refuse_typed() {
    let axis = Vec3::new(0.0, 0.0, 1.0);
    let u_ref = Vec3::new(1.0, 0.0, 0.0);
    let s = Surface::Cylinder {
        origin: Point3::origin(),
        axis,
        radius: 1.0,
        u_ref,
    };
    // Full-period rims at v = 0 and v = 1, the top rim's center
    // off-axis by 0.4 — that circle is nowhere on the cylinder.
    let rim = |z: f64, off: f64, fwd: bool, tag: u32| {
        LoopEdge::hand_built(
            Curve3::Circle {
                center: Point3::new(off, 0.0, z),
                axis,
                radius: 1.0,
                u_ref,
            },
            0.0,
            TAU,
            fwd,
            tag,
            tag,
        )
    };
    let edges = vec![rim(0.0, 0.0, true, 0), rim(1.0, 0.4, false, 1)];
    assert!(
        matches!(
            curved_face(&s, &edges, 1.0, band()),
            Err(PropsError::NotIsoRectangle {
                what: "props_rim_center_on_axis"
            })
        ),
        "off-surface rim (center off the axis) must be a typed refusal, \
         not a silently fabricated contribution"
    );
}

/// The torus `s_f` inference trusts the loop-local vertex tags: a
/// caller that LIES (tags the meridian anchor onto the far rim) gets a
/// silently flipped sign. This is a documented caller contract (tags
/// come from `topo`'s flattening, first-seen order), not a public
/// surface — pinned here so the trust boundary is explicit.
#[test]
fn torus_tag_contract_is_load_bearing() {
    let s = tor();
    let rect = [0.1, 1.8, -0.7, 0.9];
    let mut lp = patch_loop(&s, rect, 1.0, false);
    let honest = curved_face(&s, &lp.edges, 1.0, band()).unwrap();
    // Swap the two rims' tag pairs (a lie no topo flattening produces:
    // the loop is otherwise untouched).
    let (r0, r1) = (lp.edges[0].clone(), lp.edges[2].clone());
    lp.edges[0].start = r1.start;
    lp.edges[0].end = r1.end;
    lp.edges[2].start = r0.start;
    lp.edges[2].end = r0.end;
    let lied = curved_face(&s, &lp.edges, 1.0, band()).unwrap();
    let (_, flux) = oracle(&s, rect, 1.0);
    assert_rel("honest torus flux", honest.flux, flux, 1e-9);
    assert!(
        (lied.flux - honest.flux).abs() > 1e-6,
        "tag lie must actually change the anchored term (proves the tags are load-bearing)"
    );
}

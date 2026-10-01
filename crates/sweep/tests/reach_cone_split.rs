//! The plane × cone split: a cone frustum cut by a plane meets its cone
//! in the exact `Ellipse` — the axis-normal `Circle` at its boundary —
//! and the halves are valid at every tier with the closed-form volumes.
//!
//! Two frusta, one per nappe of the stored cone: `revolve` stores the
//! NARROWING frustum's cone with its apex above it and its axis `+y`, so
//! that face sits on the mirror nappe (`v < 0`), and the WIDENING
//! frustum's apex below it, on the `v > 0` nappe. Every row runs both,
//! so both signs of the cone chart's nappe convention run — in the chart
//! image and in the chord's arc-side frame.
//!
//! Two volume oracles, independent of each other and of the kernel's
//! flux. A cut that stays inside the lateral face leaves an apex-side
//! piece that is the oblique cone over the section ellipse, minus the
//! small cone beyond the far cap: `(π/3)·A·b·|δ| − π/12`, with the
//! semi-axes in closed form. Every cut is also integrated slice by slice
//! (a disk segment per height, Simpson), which needs no ellipse at all
//! and covers the cuts that cross a cap.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use crate::revolve_common::{axis_y, validated};
use geom::Curve3;
use geom_brep::{EdgeDescription, Pcurve, SectionError};
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::{Revolution, revolve};
use topo::splitting::{SplitError, SplitJoinError, SplitPart, SplitPlane, SplitResult, split};
use topo::{Body, validate, validate_closed, validate_geometric};

/// The cone's half-angle: the radius changes by 1/2 per unit height.
fn alpha() -> f64 {
    0.5f64.atan()
}

/// One frustum of height 1 between radii 1 and 1/2 about `+y`.
struct Frustum {
    body: Body<f64>,
    /// The radius at height `y`.
    r: fn(f64) -> f64,
    /// The cone's apex height.
    apex_y: f64,
}

fn revolved(pts: &[(f64, f64)]) -> Body<f64> {
    let lp = ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// Narrowing upward: apex at `y = 2`, the face on the mirror nappe.
fn narrowing() -> Frustum {
    Frustum {
        body: revolved(&[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0), (0.0, 1.0)]),
        r: |y| 1.0 - y / 2.0,
        apex_y: 2.0,
    }
}

/// Widening upward: apex at `y = −1`, the face on the `v > 0` nappe.
fn widening() -> Frustum {
    Frustum {
        body: revolved(&[(0.0, 0.0), (0.5, 0.0), (1.0, 1.0), (0.0, 1.0)]),
        r: |y| 0.5 + y / 2.0,
        apex_y: -1.0,
    }
}

/// `π·h/3·(R² + R·r + r²)` at `h = 1`, `R = 1`, `r = 1/2`.
const FRUSTUM_VOLUME: f64 = 7.0 * PI / 12.0;

/// The plane through `(0, qy, 0)` with normal `(sin φ, cos φ, 0)`.
fn plane(phi: f64, qy: f64) -> SplitPlane<f64> {
    SplitPlane {
        origin: Point3::new(0.0, qy, 0.0),
        normal: Vec3::new(phi.sin(), phi.cos(), 0.0),
    }
}

fn vol(b: &Body<f64>) -> f64 {
    topo::props::mass_properties(b, Tol::witness())
        .unwrap()
        .volume
}

/// The volume of the solid of revolution `r(y)`, `y ∈ [0, 1]`, on the
/// positive side of `plane(phi, qy)` — integrated slice by slice: at
/// height `y` the slice is a disk of radius `r` cut by the line
/// `x = d`, and the side kept is `x > d` (or the whole disk / nothing).
fn slice_oracle(r_of: fn(f64) -> f64, phi: f64, qy: f64) -> f64 {
    let n = 200_000;
    let h = 1.0 / f64::from(n);
    let segment = |y: f64| {
        let r = r_of(y);
        let d = (qy - y) * phi.cos() / phi.sin();
        if d >= r {
            0.0
        } else if d <= -r {
            PI * r * r
        } else {
            r * r * (d / r).acos() - d * (r * r - d * d).sqrt()
        }
    };
    let mut s = segment(0.0) + segment(1.0);
    for i in 1..n {
        s += if i % 2 == 1 { 4.0 } else { 2.0 } * segment(h * f64::from(i));
    }
    s * h / 3.0
}

/// The apex-side piece of a cut that stays inside the lateral face: the
/// oblique cone over the section ellipse, minus the small cone of
/// radius 1/2 and height 1 beyond the far cap. With `c = cos φ` (the
/// axis against the normal), `δ = (apex − q)·n` and `K = c² − sin²α`:
/// `A = |δ|·sin α·cos α / K`, `b = |δ|·sin α / √K`.
fn apex_side_closed_form(f: &Frustum, phi: f64, qy: f64) -> f64 {
    let (s, c) = alpha().sin_cos();
    let k = phi.cos().powi(2) - s * s;
    let delta = ((f.apex_y - qy) * phi.cos()).abs();
    let major = delta * s * c / k;
    let minor = delta * s / k.sqrt();
    PI / 3.0 * major * minor * delta - PI / 12.0
}

/// Both halves exist and are valid at every tier.
fn halves(result: &SplitResult<f64>, what: &str) -> (Body<f64>, Body<f64>) {
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("{what}: both sides carry material");
    };
    for part in [above, below] {
        assert_eq!(validate(part), Ok(()), "{what}: tier 1");
        assert_eq!(validate_closed(part), Ok(()), "{what}: tier 2");
        if let Err(errs) = validate_geometric(part, Tol::witness()) {
            panic!("{what}: tier 3: {errs:?}");
        }
    }
    (above.clone(), below.clone())
}

/// The cut's section on a half: its conic edges are wall × plane
/// `Intersection`s certified to rounding scale, their spans close the
/// conic once, and — for a tilted cut — the cone faces store the exact
/// `ConeSection` chart image of each.
fn assert_section(part: &Body<f64>, tilted: bool, what: &str) {
    let mut span = 0.0;
    for (_, edge) in part.edges() {
        let Some(c) = part.get_curve_geom(edge.curve).and_then(|g| g.certified()) else {
            continue;
        };
        let conic = match c.carrier() {
            Curve3::Ellipse { .. } => tilted,
            Curve3::Circle { center, .. } => !tilted && center.y == 0.5,
            _ => false,
        };
        if !conic {
            continue;
        }
        assert!(
            matches!(c.description(), EdgeDescription::Intersection { .. }),
            "{what}: a section edge is a described intersection"
        );
        assert!(
            c.certificate().max_residual < 1e-12,
            "{what}: residual {:e}",
            c.certificate().max_residual
        );
        let (t0, t1) = c.params();
        span += t1 - t0;
    }
    assert!((span - TAU).abs() < 1e-9, "{what}: section span {span}");
    if tilted {
        let images = part
            .pcurves()
            .filter(|(_, cache)| matches!(cache.pcurve(), Pcurve::ConeSection { .. }))
            .count();
        assert!(images >= 2, "{what}: {images} cone-section images");
    }
}

/// Tilts across the elliptic range that keep the cut inside the lateral
/// face, the axis-normal circle (`φ = 0`) at its boundary: valid halves,
/// volumes that close, the apex-side piece at the closed form and every
/// piece at the slice oracle.
#[test]
fn tilted_cuts_across_the_elliptic_range_are_exact_on_both_nappes() {
    for (name, f) in [("narrowing", narrowing()), ("widening", widening())] {
        assert!(
            (vol(&f.body) - FRUSTUM_VOLUME).abs() < 1e-12,
            "{name}: the uncut frustum"
        );
        for phi in [0.0, 0.15, 0.3, 0.45] {
            let what = format!("{name}, phi {phi}");
            let result = split(&f.body, &plane(phi, 0.5), Tol::witness())
                .unwrap_or_else(|e| panic!("{what}: {e}"));
            let (above, below) = halves(&result, &what);
            assert_section(&above, phi > 0.0, &what);
            assert_section(&below, phi > 0.0, &what);
            let (va, vb) = (vol(&above), vol(&below));
            assert!(
                (va + vb - FRUSTUM_VOLUME).abs() < 1e-12,
                "{what}: the halves sum to the frustum"
            );
            let apex_side = if f.apex_y > 0.5 { va } else { vb };
            let closed = apex_side_closed_form(&f, phi, 0.5);
            assert!(
                (apex_side - closed).abs() < 1e-10,
                "{what}: apex side {apex_side} vs closed form {closed}"
            );
            if phi > 0.0 {
                let oracle = slice_oracle(f.r, phi, 0.5);
                assert!((va - oracle).abs() < 1e-10, "{what}: {va} vs {oracle}");
            }
        }
    }
}

/// Cuts the closed form does not cover, against the slice oracle: tilts
/// that leave through a cap, one just short of the parabola (the long
/// ellipse), one through the empty region where the frustum's apex would
/// be, and one through that apex itself (the apex lane: two generators,
/// straight chords).
#[test]
fn cuts_through_a_cap_and_the_apex_region() {
    let f = narrowing();
    for (phi, qy) in [
        (0.6, 0.5),
        (0.9, 0.5),
        (1.0, 0.5),
        (0.9, 1.5),
        (10.0f64.atan(), 2.0),
    ] {
        let what = format!("phi {phi}, through y = {qy}");
        let result = split(&f.body, &plane(phi, qy), Tol::witness())
            .unwrap_or_else(|e| panic!("{what}: {e}"));
        let (above, below) = halves(&result, &what);
        let (va, vb) = (vol(&above), vol(&below));
        assert!((va + vb - FRUSTUM_VOLUME).abs() < 1e-12, "{what}: sum");
        let oracle = slice_oracle(f.r, phi, qy);
        assert!((va - oracle).abs() < 1e-10, "{what}: {va} vs {oracle}");
    }
}

/// Past the ellipse: a plane parallel to a generator cuts a parabola, a
/// steeper one a hyperbola, and each refuses naming its conic (R1). A
/// tilt within a few ε of the parabola escalates on the conic-type
/// trilean, from either side, rather than being snapped to a conic.
#[test]
fn the_parabola_and_hyperbola_refuse_naming_their_conic() {
    let f = narrowing();
    let parabola = FRAC_PI_2 - alpha();
    for (phi, conic) in [(parabola, "PARABOLA"), (1.3, "HYPERBOLA")] {
        let err = split(&f.body, &plane(phi, 0.5), Tol::witness()).unwrap_err();
        let SplitError::Join(SplitJoinError::Section {
            source: SectionError::RoutesToGeneralRung { why, .. },
            ..
        }) = &err
        else {
            panic!("{conic}: expected the section refusal, got {err:?}");
        };
        assert!(why.contains(conic), "{conic}: {why}");
        assert!(err.to_string().contains(conic), "{conic}: {err}");
    }
    let eps = Tol::witness().get().eps;
    for k in [-3.0, 3.0] {
        let err = split(&f.body, &plane(parabola + k * eps, 0.5), Tol::witness()).unwrap_err();
        let SplitError::Join(SplitJoinError::Escalated { diag, .. }) = &err else {
            panic!("{k}ε: expected the in-band escalation, got {err:?}");
        };
        assert_eq!(diag.predicate, Some("pn_conic_type"), "{k}ε");
    }
}

/// A body with a cone face splits where the plane misses the cone: a
/// cylinder under a frustum, cut through the cylinder. The half below
/// is the half-cylinder `π·r²·h/2` the plane through the axis's
/// mid-height leaves, whatever the tilt — within the certified bracket
/// of the tilted cylinder face's quadrature.
#[test]
fn a_plane_missing_the_cone_splits_the_body() {
    let tower = revolved(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.5, 2.0), (0.0, 2.0)]);
    let total = PI + FRUSTUM_VOLUME;
    for phi in [0.0, 0.3] {
        let what = format!("phi {phi}");
        let result = split(&tower, &plane(phi, 0.5), Tol::witness())
            .unwrap_or_else(|e| panic!("{what}: {e}"));
        let (above, below) = halves(&result, &what);
        let props = |b: &Body<f64>| topo::props::mass_properties(b, Tol::witness()).unwrap();
        let (pa, pb) = (props(&above), props(&below));
        let slack = pa.volume_pad + pb.volume_pad + 1e-12;
        assert!(
            (pb.volume - PI / 2.0).abs() <= pb.volume_pad + 1e-12,
            "{what}: below {} ± {}",
            pb.volume,
            pb.volume_pad
        );
        assert!(
            (pa.volume + pb.volume - total).abs() <= slack,
            "{what}: sum"
        );
    }
}

/// The halves of a cut tilted about `x` — a section whose height peaks
/// inside its arcs, not at their ends on the seams — answer the solid
/// containment door correctly or refuse `PartialConeFace`, never a
/// wrong verdict. The three points are outside the half they are asked
/// of, between the section's true height and the window its vertices
/// fold; a cone trim reading that window answered `In` for each.
#[test]
fn a_tilted_cone_cut_is_never_misread_by_containment() {
    use topo::{PointInSolidError, SolidContainment, point_in_solid};
    let f = narrowing();
    let n = Vec3::new(0.0, 0.4f64.cos(), 0.4f64.sin());
    let cut = SplitPlane {
        origin: Point3::new(0.0, 0.5, 0.0),
        normal: n,
    };
    let result = split(&f.body, &cut, Tol::witness()).unwrap();
    let (above, below) = halves(&result, "x-tilted cut");
    let band = crate::common::approx::band();
    for q in [
        Point3::new(-0.212, 0.507, -0.439),
        Point3::new(0.013, 0.3945, 0.686),
        Point3::new(-0.437, 0.282, 0.686),
    ] {
        let up = n.dot(q - cut.origin) > 0.0;
        for (part, inside) in [(&above, up), (&below, !up)] {
            match point_in_solid(part, q, band, Tol::witness()) {
                Ok(SolidContainment::In) if inside => {}
                Ok(SolidContainment::Out) if !inside => {}
                Err(PointInSolidError::PartialConeFace { .. }) => {}
                other => panic!("{q:?} (inside: {inside}): {other:?}"),
            }
        }
    }
}

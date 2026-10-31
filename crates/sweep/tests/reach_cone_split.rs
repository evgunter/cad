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
use geom_brep::{EdgeDescription, Pcurve};
use geom_core::{Band, Point2, Point3, Tol, UnitVec3, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::{Revolution, revolve};
use topo::splitting::{
    SplitError, SplitFinishError, SplitJoinError, SplitPart, SplitPlane, SplitResult, split,
};
use topo::{Body, POSE_UNIT_NORM, ValidationError, validate, validate_closed, validate_geometric};

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

/// A cut normal minted the way a caller holding a direction mints one.
fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
    let band = Band::linear(Tol::witness()).expect("the witness tolerance forms a band");
    UnitVec3::new(v, POSE_UNIT_NORM, band).expect("a cut normal has a length")
}

/// The plane through `(0, qy, 0)` with normal `(sin φ, cos φ, 0)`.
fn plane(phi: f64, qy: f64) -> SplitPlane<f64> {
    SplitPlane {
        origin: Point3::new(0.0, qy, 0.0),
        normal: unit(Vec3::new(phi.sin(), phi.cos(), 0.0)),
    }
}

fn vol(b: &Body<f64>) -> f64 {
    topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished("the measured body", topo::Body::clone(b), Tol::witness()),
        Tol::witness(),
    )
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
/// axis against the normal), `δ = (apex − q)·n` and
/// `K = c² − sin²α = (cos α·c − sin α·sin φ)·(cos α·c + sin α·sin φ)`,
/// read in the product form:
/// `A = |δ|·sin α·cos α / K`, `b = |δ|·sin α / √K`.
fn apex_side_closed_form(f: &Frustum, phi: f64, qy: f64) -> f64 {
    let (s, c) = alpha().sin_cos();
    let (sin_phi, cos_phi) = phi.sin_cos();
    let k = (c * cos_phi - s * sin_phi) * (c * cos_phi + s * sin_phi);
    let delta = ((f.apex_y - qy) * cos_phi).abs();
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
    (topo::Body::clone(above), topo::Body::clone(below))
}

/// The cut's section on a half: its conic edges are wall × plane
/// `Intersection`s certified to rounding scale, their spans close the
/// conic once, and — for a tilted cut — the cone faces store the exact
/// `FocalSection` chart image of each.
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
            .filter(|(_, cache)| matches!(cache.pcurve(), Pcurve::FocalSection(_)))
            .count();
        assert!(images >= 2, "{what}: {images} focal-section images");
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
            let result = split(
                &sweep::test_support::finished("the operand", f.body.clone(), Tol::witness()),
                &plane(phi, 0.5),
                Tol::witness(),
            )
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
        let result = split(
            &sweep::test_support::finished("the operand", f.body.clone(), Tol::witness()),
            &plane(phi, qy),
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("{what}: {e}"));
        let (above, below) = halves(&result, &what);
        let (va, vb) = (vol(&above), vol(&below));
        assert!((va + vb - FRUSTUM_VOLUME).abs() < 1e-12, "{what}: sum");
        let oracle = slice_oracle(f.r, phi, qy);
        assert!((va - oracle).abs() < 1e-10, "{what}: {va} vs {oracle}");
    }
}

/// Past the ellipse: a plane parallel to a generator cuts a parabola, a
/// steeper one a hyperbola, and the split refuses each naming its conic:
/// the arc is a NURBS chord with no certificate on a cone yet. A tilt
/// within a few ε of the parabola escalates on the conic-type trilean,
/// from either side, rather than being snapped to a conic.
#[test]
fn the_parabola_and_hyperbola_refuse_naming_their_conic() {
    let f = narrowing();
    let parabola = FRAC_PI_2 - alpha();
    for (phi, conic) in [
        (parabola, geom_brep::OpenConic::Parabola),
        (1.3, geom_brep::OpenConic::Hyperbola),
    ] {
        let err = split(
            &sweep::test_support::finished("the operand", f.body.clone(), Tol::witness()),
            &plane(phi, 0.5),
            Tol::witness(),
        )
        .unwrap_err();
        let SplitError::Join(SplitJoinError::OpenConicChord { conic: named, .. }) = &err else {
            panic!("{conic:?}: expected the open-conic refusal, got {err:?}");
        };
        assert_eq!(*named, conic);
        assert!(err.to_string().contains(conic.named()), "{conic:?}: {err}");
    }
    let eps = Tol::witness().get().eps;
    for k in [-3.0, 3.0] {
        let err = split(
            &sweep::test_support::finished("the operand", f.body.clone(), Tol::witness()),
            &plane(parabola + k * eps, 0.5),
            Tol::witness(),
        )
        .unwrap_err();
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
    let tower = sweep::test_support::finished("the tower", tower, Tol::witness());
    let total = PI + FRUSTUM_VOLUME;
    for phi in [0.0, 0.3] {
        let what = format!("phi {phi}");
        let result = split(&tower, &plane(phi, 0.5), Tol::witness())
            .unwrap_or_else(|e| panic!("{what}: {e}"));
        let (above, below) = halves(&result, &what);
        let props = |b: &Body<f64>| {
            topo::mass_properties(
                &topo::test_support::finished(
                    "the measured body",
                    topo::Body::clone(b),
                    Tol::witness(),
                ),
                Tol::witness(),
                topo::test_support::pinned_resolution(Tol::witness()),
            )
            .unwrap()
        };
        let (pa, pb) = (props(&above), props(&below));
        let slack = pa.enclosure.volume_pad + pb.enclosure.volume_pad + 1e-12;
        assert!(
            (pb.enclosure.volume - PI / 2.0).abs() <= pb.enclosure.volume_pad + 1e-12,
            "{what}: below {} ± {}",
            pb.enclosure.volume,
            pb.enclosure.volume_pad
        );
        assert!(
            (pa.enclosure.volume + pb.enclosure.volume - total).abs() <= slack,
            "{what}: sum"
        );
    }
}

/// The halves of a cut tilted about `x` — a section whose height peaks
/// inside its arcs, not at their ends on the seams — answer the solid
/// containment door correctly. The three points are outside the half they are asked
/// of, between the section's true height and the window its vertices
/// fold; a cone trim reading that window answered `In` for each.
#[test]
fn a_tilted_cone_cut_is_never_misread_by_containment() {
    use topo::{SolidContainment, point_in_solid};
    let f = narrowing();
    let n = Vec3::new(0.0, 0.4f64.cos(), 0.4f64.sin());
    let cut = SplitPlane {
        origin: Point3::new(0.0, 0.5, 0.0),
        normal: unit(n),
    };
    let result = split(
        &sweep::test_support::finished("the operand", f.body.clone(), Tol::witness()),
        &cut,
        Tol::witness(),
    )
    .unwrap();
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
                other => panic!("{q:?} (inside: {inside}): {other:?}"),
            }
        }
    }
    // Far from the body, a ray that misses the cone answers: the
    // untrimmable face refuses only a query it could decide.
    for q in [Point3::new(10.0, 10.0, 10.0), Point3::new(-3.0, -8.0, 2.0)] {
        for part in [&above, &below] {
            assert_eq!(
                point_in_solid(part, q, band, Tol::witness()).unwrap(),
                SolidContainment::Out,
                "{q:?}"
            );
        }
    }
}

/// The upright cone, `revolve` of the triangle `(0,0), (1,0), (0,1)`:
/// its lateral faces run from the base rim to the APEX at `y = 1`, on
/// the stored cone's mirror nappe. A cut's chord on such a face co-bounds
/// a run that crosses the apex, where the chart has no azimuth — the
/// window is the face's apex-closed lift's, never a branch pinned across
/// it. Every tilt × height × azimuth of the plane (the azimuth moves the
/// cut against the faces' seams), plus the axis-normal circle: valid
/// halves, volumes that close on the cone's `π/3`, and the slice oracle.
#[test]
fn an_upright_cone_splits_at_every_pose_through_its_apex_faces() {
    let cone = revolved(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]);
    let cone = sweep::test_support::finished("the cone", cone, Tol::witness());
    let total = PI / 3.0;
    assert!((vol(&cone) - total).abs() < 1e-12, "the uncut cone");
    for qy in [0.15, 0.4, 0.75] {
        let what = format!("axis-normal circle at y = {qy}");
        let result =
            split(&cone, &plane(0.0, qy), Tol::witness()).unwrap_or_else(|e| panic!("{what}: {e}"));
        let (above, below) = halves(&result, &what);
        let tip = PI * (1.0 - qy).powi(3) / 3.0;
        assert!((vol(&above) - tip).abs() < 1e-12, "{what}: the tip cone");
        assert!(
            (vol(&above) + vol(&below) - total).abs() < 1e-12,
            "{what}: sum"
        );
        for psi in (0..12).map(|i| f64::from(i) * 0.53) {
            for phi in [0.05f64, 0.2, 0.5, 0.7] {
                let what = format!("psi {psi:.2}, phi {phi}, through y = {qy}");
                let cut = SplitPlane {
                    origin: Point3::new(0.0, qy, 0.0),
                    normal: unit(Vec3::new(
                        phi.sin() * psi.cos(),
                        phi.cos(),
                        phi.sin() * psi.sin(),
                    )),
                };
                let result =
                    split(&cone, &cut, Tol::witness()).unwrap_or_else(|e| panic!("{what}: {e}"));
                let (above, below) = halves(&result, &what);
                let (va, vb) = (vol(&above), vol(&below));
                assert!((va + vb - total).abs() < 1e-12, "{what}: sum {}", va + vb);
                let oracle = slice_oracle(|y| 1.0 - y, phi, qy);
                assert!((va - oracle).abs() < 1e-8, "{what}: {va} vs {oracle}");
            }
        }
    }
}

/// Steep frusta (top radius `S/2`, height `S`, half-angle `α`) in a
/// generic rigid pose, cut through `(0, S/2, 0)` by a plane standing
/// `φ = f·(π/2 − α)` off the axis: a well-conditioned ellipse on a
/// near-flat cone, whose carrier must close on the vertices where the
/// plane crosses the caps' rims at a shallow dihedral angle. The split
/// succeeds and its halves are valid at tiers 1 to 3, with volumes that
/// sum to the whole and to its closed form within the certified pads and
/// a relative `1e-12`, wherever the props lane can read them; where it
/// cannot, the split's at-rest gate refuses the halves, and every finding
/// is the volume the props lane could not compute.
///
/// Two ε floors are not this cut's: the `(1.55, 100)` operand does not
/// revolve below ε = 1e-9, its far cap rim's return half missing its
/// start vertex
/// (`a-full-revolves-return-half-rim-misses-its-start-by-two-rounded-half-turns`), and at
/// scale 100 the props lane's bound on the section ellipse's elevation
/// escalates at ε = 1e-12
/// (`props-cone-ellipse-elevation-over-reads-a-section-ellipse-by-twenty-fold`),
/// so the `(1.3, 100)` split refuses there.
#[test]
fn steep_frusta_in_a_generic_pose_split_at_a_shallow_tilt() {
    use geom_core::{Affine3, test_support::rigid};
    let tol = Tol::witness();
    let eps = tol.get().eps;
    let motion = Affine3::rotation_about_axis(
        Point3::new(0.37, -0.21, 0.13),
        Vec3::new(0.3, 0.5, 0.8).normalize(),
        0.7,
    );
    // (α, S, f, the least ε the operand revolves at, the least ε tier 3
    // reads the halves at)
    for (alpha, scale, f, builds_from, measured_from) in [
        (1.3, 100.0, 0.5, 0.0, 1e-9),
        (1.55, 1.0, 0.3, 0.0, 0.0),
        (1.55, 1.0, 0.5, 0.0, 0.0),
        (1.55, 1.0, 0.7, 0.0, 0.0),
        (1.55, 100.0, 0.5, 1e-9, 1e-9),
    ] {
        let what = format!("alpha {alpha}, S {scale}, f {f}");
        if eps < builds_from {
            println!(
                "SKIPPED at ε {eps:e}: {what}: the operand does not revolve below ε {builds_from:e}"
            );
            continue;
        }
        let (h, r1) = (scale, 0.5 * scale);
        let r0 = r1 + h * f64::tan(alpha);
        let body = sweep::test_support::revolved_about_y(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(r0, 0.0), 0.0),
                (Point2::new(r1, h), 0.0),
                (Point2::new(0.0, h), 0.0),
            ],
            Revolution::Full,
            tol,
        );
        // The 0.9 km steep frustum's turn re-certifies its rims past the
        // band at ε = 1e-12, at its own extent's rounding: limb 2's
        // kernel limit
        // (`work/offset/eps-min-leaves-no-certification-headroom-at-the-box-edge.md`).
        let pinned: &[f64] = if alpha == 1.3 { &[1e-12] } else { &[] };
        let turned = topo::transform_rigid(&body, &rigid(&motion, tol), tol);
        let Some(body) = test_utils::kernel_limit::pinned(turned, eps, pinned, &what) else {
            continue;
        };
        let phi = f * (FRAC_PI_2 - alpha);
        let cut = topo::test_support::split_plane(
            motion.transform_point(Point3::new(0.0, 0.5 * h, 0.0)),
            motion.transform_vec(Vec3::new(phi.sin(), phi.cos(), 0.0)),
            tol,
        );
        let operand = sweep::test_support::finished("the frustum", body, tol);
        let answer = split(&operand, &cut, tol);
        if eps < measured_from {
            match answer {
                Err(SplitError::Finish(SplitFinishError::ResultNotAtRest {
                    ref errors, ..
                })) if !errors.is_empty()
                    && errors
                        .iter()
                        .all(|e| matches!(e, ValidationError::VolumeUncomputable { .. })) =>
                {
                    println!(
                        "SKIPPED at ε {eps:e}: {what}: the halves' volumes, below ε \
                         {measured_from:e}; the split's at-rest gate refuses them"
                    );
                    continue;
                }
                other => panic!(
                    "{what}: below ε {measured_from:e} the at-rest gate refuses on the \
                     props lane's volume alone, not {other:?}"
                ),
            }
        }
        let result = answer.unwrap_or_else(|e| panic!("{what}: {e:?}"));
        let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below)
        else {
            panic!("{what}: both sides carry material");
        };
        for part in [above, below] {
            assert_eq!(validate(part), Ok(()), "{what}: tier 1");
            assert_eq!(validate_closed(part), Ok(()), "{what}: tier 2");
        }
        halves(&result, &what);
        let props = |b: &Body<f64>| {
            topo::mass_properties(
                &topo::test_support::finished("the measured body", topo::Body::clone(b), tol),
                tol,
                topo::test_support::pinned_resolution(tol),
            )
            .unwrap()
        };
        let (pw, pa, pb) = (props(&operand), props(above), props(below));
        let whole = PI * h / 3.0 * (r0 * r0 + r0 * r1 + r1 * r1);
        let slack = 1e-12 * whole;
        assert!(
            (pw.enclosure.volume - whole).abs() <= pw.enclosure.volume_pad + slack,
            "{what}: the uncut frustum {} ± {}, want {whole}",
            pw.enclosure.volume,
            pw.enclosure.volume_pad
        );
        let sum = pa.enclosure.volume + pb.enclosure.volume;
        assert!(
            (sum - whole).abs() <= pa.enclosure.volume_pad + pb.enclosure.volume_pad + slack,
            "{what}: the halves sum to {sum} ± {}, want {whole}",
            pa.enclosure.volume_pad + pb.enclosure.volume_pad
        );
    }
}

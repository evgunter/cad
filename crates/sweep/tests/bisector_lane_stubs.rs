//! **The bisector lane on branch stubs**: two equal cylinders whose
//! axes meet, posed so that neither node `p ± r·n̂` lies in both
//! operands' faces. The walls then meet in two disjoint arcs, one of
//! each bisector-plane ellipse, and every boolean of the pair builds
//! through the join's bisector lane: each germ rides the ellipse its
//! branch names, and each side's chord is its own wall cut by that
//! branch's plane.
//!
//! The stub is a cylinder whose end cap stands inside the other wall,
//! clear of the axes' meeting point: the nodes sit at the stub's axial
//! station of that point, outside its face. At right angles the
//! intersection has a closed form (below); at a slant the four
//! operations are held to each other's volumes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::germ_pair::{cyl, repose, spin};
use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::test_support::finished;
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{Body, BooleanResult};

/// A cylinder of radius `r` about `z`, `z ∈ [z0, z1]`, its seam turned
/// `phi` about its own axis.
fn stub_z(r: f64, (z0, z1): (f64, f64), phi: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: z1 - z0,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    spin(&body, Vec3::new(0.0, 0.0, 1.0), phi)
}

/// The stub [`stub_z`] turned so its axis runs along `(0, sin θ, cos θ)`
/// through the origin: its stations along that axis are `[s0, s1]`.
fn stub_at(theta: f64, stations: (f64, f64), phi: f64) -> Body<f64> {
    spin(
        &stub_z(1.0, stations, phi),
        Vec3::new(1.0, 0.0, 0.0),
        -theta,
    )
}

/// The long cylinder about `z`, `z ∈ [−2, 2]`, its seam turned `phi`.
fn long(phi: f64) -> Body<f64> {
    spin(&cyl(1.0, 2.0), Vec3::new(0.0, 0.0, 1.0), phi)
}

/// A volume with its certified bracket's half-width: the quadrature's
/// own, on faces with no closed form.
type Measured = (f64, f64);

fn volume(b: &Body<f64>) -> Measured {
    let m = topo::mass_properties(b, Tol::witness()).unwrap();
    (m.volume, m.volume_pad)
}

/// The four operations' volumes, `[∪, ∩, A∖B, B∖A]`; each must build.
fn four(a: &Body<f64>, b: &Body<f64>, what: &str) -> [Measured; 4] {
    let tol = Tol::witness();
    let (a, b) = (
        finished("operand A", a.clone(), tol),
        finished("operand B", b.clone(), tol),
    );
    let built = |r: Result<BooleanResult<f64>, topo::BooleanError>, op: &str| match r {
        Ok(BooleanResult::Body(body)) => volume(&body.body),
        other => panic!("{what}, {op}: the bisector lane builds, got {other:?}"),
    };
    [
        built(topo::union(&a, &b, tol), "∪"),
        built(topo::intersect(&a, &b, tol), "∩"),
        built(topo::subtract(&a, &b, tol), "A∖B"),
        built(topo::subtract(&b, &a, tol), "B∖A"),
    ]
}

/// Asserts the four volumes against the operands' and `cap`, the
/// intersection's: `∪ = A + B − cap`, `A∖B = A − cap`, `B∖A = B − cap`,
/// each within its certified bracket and `cap`'s.
fn assert_four(got: [Measured; 4], (va, vb, cap): (f64, f64, Measured), what: &str) {
    let want = [va + vb - cap.0, cap.0, va - cap.0, vb - cap.0];
    for (op, ((g, pad), w)) in ["∪", "∩", "A∖B", "B∖A"]
        .into_iter()
        .zip(got.into_iter().zip(want))
    {
        assert!(
            (g - w).abs() <= pad + cap.1 + 1e-12,
            "{what}, {op}: volume {g} ± {pad}, want {w} ± {} (off by {:e})",
            cap.1,
            g - w
        );
    }
}

/// **The right-angle stub's intersection, in closed form.** The long
/// wall `x² + y² ≤ 1` against the stub `x² + z² ≤ 1`, `y ∈ [a, b]` with
/// `0 < a < 1 ≤ b`: at abscissa `x`, with `s = √(1 − x²)`, the section
/// is `|z| ≤ s` by `a ≤ y ≤ s`, so the volume is `∫ 2s(s − a) dx` over
/// `|x| ≤ x₀ = √(1 − a²)`: `4(x₀ − x₀³/3) − 2a(a·x₀ + asin x₀)`.
fn right_angle_cap(a: f64) -> f64 {
    let x0 = (1.0 - a * a).sqrt();
    4.0 * (x0 - x0.powi(3) / 3.0) - 2.0 * a * (a * x0 + x0.asin())
}

/// The closed form against a midpoint sum of its own integrand, so a
/// slip in the algebra cannot pass as the kernel's error.
#[test]
fn the_right_angle_cap_is_its_integral() {
    for a in [0.3_f64, 0.5, 0.7] {
        let x0 = (1.0 - a * a).sqrt();
        let n = 200_000;
        let h = 2.0 * x0 / f64::from(n);
        let sum: f64 = (0..n)
            .map(|i| {
                let x = -x0 + h * (f64::from(i) + 0.5);
                let s = (1.0 - x * x).sqrt();
                2.0 * s * (s - a) * h
            })
            .sum();
        assert!(
            (sum - right_angle_cap(a)).abs() < 1e-8,
            "a = {a}: the closed form {} against its integral {sum}",
            right_angle_cap(a)
        );
    }
}

/// **A stub at right angles builds every operation at the closed form**,
/// in both operand orders and with the stub's cap at several depths,
/// each pose beside its re-posed twin.
#[test]
fn a_right_angle_stub_builds_every_operation_at_its_closed_form() {
    let phi = 0.4;
    for a in [0.3_f64, 0.5, 0.7] {
        let (big, stub) = (long(phi), stub_at(PI / 2.0, (a, 2.0), phi));
        let (va, vb) = (4.0 * PI, PI * (2.0 - a));
        let cap = (right_angle_cap(a), 0.0);
        let what = format!("a = {a}");
        assert_four(four(&big, &stub, &what), (va, vb, cap), &what);
        assert_four(
            four(&stub, &big, &format!("{what}, swapped")),
            (vb, va, cap),
            &format!("{what}, swapped"),
        );
        assert_four(
            four(&repose(&big), &repose(&stub), &format!("{what}, re-posed")),
            (va, vb, cap),
            &format!("{what}, re-posed"),
        );
    }
}

/// **A slanted stub builds every operation**, the four volumes agreeing
/// with each other through the intersection's: its axes meet at 60°,
/// so the bisector planes are not the right-angle pair's.
#[test]
fn a_slanted_stub_builds_every_operation_consistently() {
    let phi = 0.4;
    let (big, stub) = (long(phi), stub_at(PI / 3.0, (0.6, 2.5), phi));
    let (va, vb) = (4.0 * PI, PI * 1.9);
    let got = four(&big, &stub, "60°");
    assert_four(got, (va, vb, got[1]), "60°");
    assert!(
        got[1].0 > 0.0 && got[1].0 < vb,
        "60°: the stub enters the long wall and leaves it"
    );
    let twin = four(&repose(&big), &repose(&stub), "60°, re-posed");
    assert_four(twin, (va, vb, got[1]), "60°, re-posed");
}

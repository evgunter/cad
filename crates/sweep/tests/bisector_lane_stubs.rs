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
use geom_core::{Point2, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::test_support::finished;
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{Body, BooleanResult};

/// A cylinder of radius `r` about `z`, `z ∈ [z0, z1]`, its seam turned
/// `phi` about its own axis.
fn stub_z(r: f64, (z0, z1): (f64, f64), phi: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol).unwrap();
    let plane = SketchPlane::xy_at(geom_core::Point3::new(0.0, 0.0, z0));
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
    let m = topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(b), Tol::witness()),
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap();
    (m.enclosure.volume, m.enclosure.volume_pad)
}

/// The four operations' volumes, `[∪, ∩, A∖B, B∖A]`; each must build.
fn four(a: &Body<f64>, b: &Body<f64>, what: &str) -> [Measured; 4] {
    let tol = Tol::witness();
    let (a, b) = (
        finished("operand A", a.clone(), tol),
        finished("operand B", b.clone(), tol),
    );
    let built = |r: Result<BooleanResult<f64>, topo::BooleanError>,
                 operands: (&Body<f64>, &Body<f64>),
                 op: &str| match r {
        Ok(BooleanResult::Body(body)) => {
            assert_crossing_rows(&body.coincidences, operands, &format!("{what}, {op}"));
            volume(&body.body)
        }
        other => panic!("{what}, {op}: the bisector lane builds, got {other:?}"),
    };
    [
        built(topo::union(&a, &b, tol), (&a, &b), "∪"),
        built(topo::intersect(&a, &b, tol), (&a, &b), "∩"),
        built(topo::subtract(&a, &b, tol), (&a, &b), "A∖B"),
        built(topo::subtract(&b, &a, tol), (&b, &a), "B∖A"),
    ]
}

/// **The two coincidences the lane is decided from are recorded**: the
/// walls' radii decided equal and their axes decided meeting, each a
/// row between a wall face of A and one of B, for the
/// `unproven-coincidence` lint.
fn assert_crossing_rows(rows: &[topo::Coincidence], (a, b): (&Body<f64>, &Body<f64>), what: &str) {
    let crossing: Vec<_> = rows
        .iter()
        .filter(|r| r.site == topo::DecisionSite::CrossingCylinders)
        .collect();
    for relation in [topo::Relation::SameRadius, topo::Relation::AxesMeet] {
        assert!(
            crossing.iter().any(|r| r.relation == relation),
            "{what}: a {relation:?} row is recorded, got {crossing:?}"
        );
    }
    assert!(
        crossing.iter().all(|r| r.face_pair().is_some()
            && matches!(
                r.relation,
                topo::Relation::SameRadius | topo::Relation::AxesMeet
            )),
        "{what}: every crossing row relates a face of A to one of B: {crossing:?}"
    );
    // What the rows cite: each face pair is two unit cylinder walls,
    // one of each operand, and both relations are recorded for each.
    let wall = |body: &Body<f64>, f| {
        body.get_face(f)
            .and_then(|face| body.get_surface(face.surface))
            .is_some_and(|s| matches!(s, topo::Surface::Cylinder { radius, .. } if *radius == 1.0))
    };
    for r in &crossing {
        let (fa, fb) = r.face_pair().expect("checked above");
        assert!(
            wall(a, fa) && wall(b, fb),
            "{what}: a crossing row cites {fa:?} and {fb:?}, which are not two unit walls"
        );
        for relation in [topo::Relation::SameRadius, topo::Relation::AxesMeet] {
            assert!(
                crossing
                    .iter()
                    .any(|o| o.face_pair() == Some((fa, fb)) && o.relation == relation),
                "{what}: the pair {fa:?}, {fb:?} carries a {relation:?} row"
            );
        }
    }
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

/// **The intersection of the long wall and a stub at angle `theta`,
/// by quadrature**: the stub of unit radius about `u = (0, sin θ, cos θ)`
/// over stations `[s0, s1]`, each line of it along `u` through a point
/// `ξ·x̂ + η·(0, cos θ, −sin θ)` of its unit disc meeting the long wall
/// `x² + y² ≤ 1`, `|z| ≤ 2` in one interval of stations, whose length is
/// in closed form; integrated over the disc with `ξ = sin t`,
/// `η = cos t · v`, by the midpoint rule in `t` and `v`.
fn stub_cap_by_quadrature(theta: f64, (s0, s1): (f64, f64), n: u32) -> f64 {
    let (sin, cos) = theta.sin_cos();
    let h = 1.0 / f64::from(n);
    let mut sum = 0.0;
    for i in 0..n {
        let t = -PI / 2.0 + PI * (f64::from(i) + 0.5) * h;
        // `ξ = sin t` enters through `w = √(1 − ξ²) = cos t` alone.
        let w = t.cos();
        for j in 0..n {
            let v = -1.0 + 2.0 * (f64::from(j) + 0.5) * h;
            let eta = w * v;
            // `|y| ≤ w`, `y = η cos θ + s sin θ`.
            let (mut lo, mut hi) = ((-w - eta * cos) / sin, (w - eta * cos) / sin);
            lo = lo.max(s0);
            hi = hi.min(s1);
            // `|z| ≤ 2`, `z = −η sin θ + s cos θ`.
            if cos.abs() > 1e-12 {
                let (a, b) = ((-2.0 + eta * sin) / cos, (2.0 + eta * sin) / cos);
                lo = lo.max(a.min(b));
                hi = hi.min(a.max(b));
            }
            // dξ dη = (cos t · π h)(cos t · 2h).
            sum += (hi - lo).max(0.0) * w * w * PI * h * 2.0 * h;
        }
    }
    sum
}

/// The quadrature against the right angle's closed form, so the oracle
/// the slanted row is held to is itself held to one.
#[test]
fn the_stub_quadrature_is_the_right_angle_closed_form() {
    for a in [0.3_f64, 0.6] {
        let got = stub_cap_by_quadrature(PI / 2.0, (a, 2.0), 2000);
        assert!(
            (got - right_angle_cap(a)).abs() < 1e-5,
            "a = {a}: the quadrature {got} against the closed form {}",
            right_angle_cap(a)
        );
    }
}

/// **A slanted stub builds every operation at the quadrature's
/// intersection**: its axes meet at 60°, so the bisector planes are not
/// the right-angle pair's; the four volumes are held to the intersection
/// [`stub_cap_by_quadrature`] measures, and the re-posed twin to the
/// same.
#[test]
fn a_slanted_stub_builds_every_operation_at_the_quadratures_intersection() {
    let phi = 0.4;
    let (big, stub) = (long(phi), stub_at(PI / 3.0, (0.6, 2.5), phi));
    let (va, vb) = (4.0 * PI, PI * 1.9);
    let cap = (stub_cap_by_quadrature(PI / 3.0, (0.6, 2.5), 2000), 1e-5);
    assert!(
        cap.0 > 0.0 && cap.0 < vb,
        "60°: the stub enters the long wall and leaves it"
    );
    assert_four(four(&big, &stub, "60°"), (va, vb, cap), "60°");
    let twin = four(&repose(&big), &repose(&stub), "60°, re-posed");
    assert_four(twin, (va, vb, cap), "60°, re-posed");
}

/// **The nodes are read off the section table, wherever a wall's origin
/// is stored.** The long wall cut from a rod `far` long, so its stored
/// origin sits `far` off its faces, against the right-angle stub: every
/// operation builds at the closed form, as the wall stored at its own
/// centre does. The axes' meeting point is the table's, read between
/// the axes' feet across the pair's reach, not off the stored origins.
///
/// Past D4 ¶4's size range at the run's ε the answer is a typed
/// escalation, not a body: the rod the wall is cut from reaches
/// `|z| = far`, where f64 spaces coordinates `far·2⁻⁵²` apart, so past
/// `far ≈ 2⁵²·ε` the cut itself cannot certify. At `far = 1e4` and
/// ε = 1e-12 the cut's crossing insertion escalates on
/// `carrier_matches_mapped_source`, which the row asserts.
#[test]
fn a_wall_stored_far_off_its_faces_builds_at_the_closed_form() {
    let tol = Tol::witness();
    let zero = geom_core::Band::linear(tol).unwrap().zero();
    let phi = 0.4;
    let a = 0.5;
    for far in [1e2_f64, 1e4] {
        let rod = spin(&cyl(1.0, far), Vec3::new(0.0, 0.0, 1.0), phi);
        let cutter = finished(
            "the cutter",
            sweep::test_support::brick((-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0), tol),
            tol,
        );
        let cut = topo::intersect(&finished("the rod", rod, tol), &cutter, tol);
        if far * f64::EPSILON > zero {
            assert!(
                matches!(
                    &cut,
                    Err(topo::BooleanError::CrossingInsertion {
                        source: topo::EulerOpError::Certification {
                            error: geom_brep::CertifyError::Escalated { cause, .. },
                        },
                        ..
                    }) if cause.predicate == Some("carrier_matches_mapped_source")
                ),
                "far = {far:e}, past the size range at zero band {zero:e}: the cut escalates \
                 its crossing's certification, got {:?}",
                cut.as_ref().map(|_| "a body")
            );
            continue;
        }
        let cut = cut
            .expect("the cut rod")
            .body()
            .expect("the cut is a body")
            .body
            .clone();
        let stub = stub_at(PI / 2.0, (a, 2.0), phi);
        let (va, vb) = (4.0 * PI, PI * (2.0 - a));
        let cap = (right_angle_cap(a), 0.0);
        let what = format!("far = {far:e}");
        assert_four(four(&cut, &stub, &what), (va, vb, cap), &what);
    }
}

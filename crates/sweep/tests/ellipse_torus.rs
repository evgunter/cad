//! **An ellipse rim against a torus face**: the exact tilted cut of a drum
//! (a radius-0.5 cylinder of height 1, split by the plane through
//! `(0, 0, 0.5)` with normal `(sin 0.3, 0, cos 0.3)`), its lower part,
//! against a ring torus whose tube comes near the cut's `Ellipse` rim.
//! Along an ellipse the torus's implicit is a trigonometric polynomial of
//! degree four (`geom_brep::curve_face::ellipse_torus`, "Against a torus"), and
//! the crossing layer must decide the rim against the torus face by its
//! certified roots wherever the rim's arc enclosure cannot clear it.
//!
//! The rows read the reduction sweep (`sweep_traces`): whether it
//! refuses, and which (rim, torus face) pairs the exact predicates
//! accepted, then every boolean op: a torus just clear of the rim builds
//! the two solids apart, and one that crosses it meets the cut plane
//! obliquely, a plane × torus pair the section value marches, and builds
//! the crossing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Band, Point2, Point3, Tol, UnitVec3, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanError, EdgeKey, FaceKey, POSE_UNIT_NORM, SweepStrategy, sweep_traces};

const DRUM_RADIUS: f64 = 0.5;
const TILT: f64 = 0.3;

/// The drum's lower part: its rim is the cut face's two `Ellipse` arcs.
fn drum_lower() -> Body<f64> {
    let tol = Tol::witness();
    let r = DRUM_RADIUS;
    let cylinder = sweep::test_support::prism(
        vec![(Point2::new(-r, 0.0), 1.0), (Point2::new(r, 0.0), 1.0)],
        1.0,
        tol,
    );
    let cylinder = sweep::test_support::finished("the cylinder", cylinder, tol);
    let plane = topo::splitting::SplitPlane {
        origin: Point3::new(0.0, 0.0, 0.5),
        normal: UnitVec3::new(
            Vec3::new(TILT.sin(), 0.0, TILT.cos()),
            POSE_UNIT_NORM,
            Band::linear(tol).expect("the witness tolerance forms a band"),
        )
        .expect("a cut normal has a length"),
    };
    let result = topo::splitting::split(&cylinder, &plane, tol).expect("the tilted cut splits");
    let topo::splitting::SplitPart::Body(below) = result.below else {
        panic!("the cut leaves material below");
    };
    assert_eq!(
        ellipse_edges(&below).len(),
        2,
        "the rim is two ellipse arcs"
    );
    below.into_body()
}

/// A ring torus `(big, small)` about the axis `axis` through `c`: a full
/// circle profile revolved about `y`, turned onto `axis` and moved.
fn donut(big: f64, small: f64, c: Vec3<f64>, axis: Vec3<f64>) -> Body<f64> {
    let at_origin = revolved_about_y(
        vec![
            (Point2::new(big, -small), 1.0),
            (Point2::new(big, small), 1.0),
        ],
        Revolution::Full,
        Tol::witness(),
    );
    let y = Vec3::new(0.0, 1.0, 0.0);
    let a = axis.normalize();
    let turned = if y.cross(a).norm() < 1e-15 {
        at_origin
    } else {
        let turn = Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            y.cross(a).normalize(),
            y.dot(a).acos(),
        );
        topo::transform_rigid(
            &at_origin,
            &geom_core::test_support::rigid(&turn, Tol::witness()),
        )
        .expect("a rotation is rigid")
    };
    topo::transform_rigid(&turned, &geom_core::RigidMotion::translation(c).unwrap())
        .expect("a translation is rigid")
}

/// The torus of `(big, small)` whose tube comes within `gap` of the rim at
/// azimuth `phi`, from OUTSIDE the drum's convex corner there: its core
/// circle passes `small + gap` from the rim point along the bisector `d̂`
/// of the wall's and the cut plane's outward normals, square to the
/// rim's tangent `t̂`, its centre `big` away along `d̂ × t̂` on the wall's
/// outward side. The tube's least distance from the rim is `gap`, at that
/// point (the rim bends into the drum and the core circle away from it),
/// and from the wall and the cut plane, which the corner keeps farther
/// than the rim, more. Negative `gap` crosses the rim.
fn corner_torus(big: f64, small: f64, phi: f64, gap: f64) -> Body<f64> {
    let (st, ct) = TILT.sin_cos();
    let (sp, cp) = phi.sin_cos();
    let x = DRUM_RADIUS * cp;
    let rim = Vec3::new(x, DRUM_RADIUS * sp, 0.5 - x * TILT.tan());
    let wall = Vec3::new(cp, sp, 0.0);
    let cut = Vec3::new(st, 0.0, ct);
    let d = (wall + cut).normalize();
    let t = cut.cross(wall).normalize();
    let e = d.cross(t).normalize();
    let e = if e.dot(wall) < 0.0 { -e } else { e };
    donut(big, small, rim + d * (small + gap) + e * big, d)
}

fn ellipse_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    body.edges()
        .filter(|(_, e)| {
            body.get_curve_geom(e.curve)
                .and_then(topo::CurveGeom::certified)
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Ellipse(..)))
        })
        .map(|(k, _)| k)
        .collect()
}

fn torus_faces(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(fk, _)| matches!(body.face_surface(*fk), Some(geom::Surface::Torus { .. })))
        .map(|(k, _)| k)
        .collect()
}

/// The rim's pairs with `b`'s torus faces in the A → B sweep:
/// `(examined, accepted)`.
fn rim_pairs(label: &str, a: &Body<f64>, b: &Body<f64>) -> (usize, usize) {
    let (ab, _) = sweep_traces(a, b, SweepStrategy::Realized, None, Tol::witness())
        .unwrap_or_else(|e| panic!("{label}: the reduction sweep refused: {e:?}"));
    let (rim, tori) = (ellipse_edges(a), torus_faces(b));
    let count = |v: &[(EdgeKey, FaceKey)]| {
        v.iter()
            .filter(|(e, f)| rim.contains(e) && tori.contains(f))
            .count()
    };
    (count(ab.examined()), count(ab.accepted()))
}

/// **A torus just clear of the rim, from outside the corner.** The tube
/// passes 1e-4 m down to 100 zero bands from the rim, nowhere nearer: the rim's arc
/// enclosure cannot clear it, its certified roots answer `Miss`, the
/// pairs are examined and none accepted, and the crossing layer passes.
/// The section certificate then clears the oblique torus × wall pair on
/// its march's components, and every op builds the two solids apart:
/// ∪ both (an assembly), ∩ nothing, each difference its first operand,
/// held to tier 3 and to the closed-form volumes (the drum's lower part
/// `π/8`, the torus `2π²Rr²`).
#[test]
fn a_torus_just_clear_of_the_rim_is_decided_by_its_roots() {
    let a = drum_lower();
    let eps = Tol::witness().eps();
    // Every gap the band reads as definite (100 zero bands or more), from
    // the widest the rim's enclosure cannot clear down.
    let gaps = [1e-4, 1e-5, 1e-6, 1e-7, 1e-8, 1e-10];
    for gap in gaps.into_iter().filter(|&g| g >= 100.0 * eps) {
        let label = format!("gap {gap:e}");
        let b = corner_torus(0.3, 0.05, 1.05, gap);
        let (examined, accepted) = rim_pairs(&label, &a, &b);
        assert!(
            examined > 0,
            "{label}: the rim is examined against the torus"
        );
        assert_eq!(accepted, 0, "{label}: and cleared");
        let (drum, ring) = (
            core::f64::consts::PI / 8.0,
            0.0015 * core::f64::consts::PI.powi(2),
        );
        for ((op, r), want) in
            ops(&a, &b)
                .into_iter()
                .zip([Some(drum + ring), None, Some(drum), Some(ring)])
        {
            // At ε 1e-6 the wall pair's march straddles its tube
            // (`work/secttorus/the-torus-doors-tube-straddles-transverse-poses-at-a-coarse-band.md`),
            // and the certificate refuses R-march at the extent scan.
            let r = match r {
                Err(BooleanError::FallbackExtentUnsupported { .. }) if eps == 1e-6 => continue,
                r => r.unwrap_or_else(|e| panic!("{label}, {op}: {e:?}")),
            };
            let got = r.body().map(|b| {
                assert_eq!(
                    topo::validate_geometric(&b.body, Tol::witness()),
                    Ok(()),
                    "{label}, {op}: tier 3"
                );
                topo::test_support::pinned_volume(op, &b.body, Tol::witness()).volume
            });
            match (got, want) {
                (Some(v), Some(w)) => {
                    assert!(
                        (v - w).abs() < 1e-9 * w,
                        "{label}, {op}: volume {v} against {w}"
                    );
                }
                (None, None) => {}
                _ => panic!("{label}, {op}: {got:?} against {want:?}"),
            }
        }
    }
}

/// Every op's result, `A ∖ B` and `B ∖ A` both.
fn ops(
    a: &Body<f64>,
    b: &Body<f64>,
) -> Vec<(&'static str, Result<topo::BooleanResult<f64>, BooleanError>)> {
    let tol = Tol::witness();
    let (a, b) = (
        &sweep::test_support::finished("operand A", a.clone(), tol),
        &sweep::test_support::finished("operand B", b.clone(), tol),
    );
    vec![
        ("A ∪ B", topo::boolean::union(a, b, tol)),
        ("A ∩ B", topo::boolean::intersect(a, b, tol)),
        ("A ∖ B", topo::boolean::subtract(a, b, tol)),
        ("B ∖ A", topo::boolean::subtract(b, a, tol)),
    ]
}

/// **A torus crossing the rim.** A tube through the corner by a
/// millimetre, and a ring of radius 0.45 about the drum's axis at the
/// cut's centre whose tube the rim enters and leaves: the rim's pairs are
/// accepted at their certified roots, the oblique plane × torus pair is
/// marched (one null loop through the corner, two loops round the
/// ring's tube), and every op builds, held to tier 3. The four volumes
/// close on the operands' closed forms (`π/8`, `2π²Rr²`): `∪ + ∩` is
/// their sum, each difference its operand less `∩`, which is positive;
/// the ring's `∩` is also held to a quadrature of the tube below the
/// plane and inside the wall. At ε 1e-6 a march's tube straddles
/// (`work/secttorus/the-torus-doors-tube-straddles-transverse-poses-at-a-coarse-band.md`).
#[test]
fn a_torus_crossing_the_rim_builds_every_op() {
    let a = drum_lower();
    let coarse = Tol::witness().eps() == 1e-6;
    let (ring_r, ring_small) = (0.45, 0.1);
    for (label, b, (big, small)) in [
        (
            "corner, 1 mm deep",
            corner_torus(0.3, 0.05, 1.05, -1e-3),
            (0.3, 0.05),
        ),
        (
            "ring about the axis",
            donut(
                ring_r,
                ring_small,
                Vec3::new(0.0, 0.0, 0.5),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            (ring_r, ring_small),
        ),
    ] {
        let (_, accepted) = rim_pairs(label, &a, &b);
        assert!(accepted > 0, "{label}: the rim's crossings are accepted");
        let mut volumes = Vec::new();
        for (op, r) in ops(&a, &b) {
            let r = match r {
                Err(BooleanError::GermSectionUnbuilt {
                    why:
                        topo::SectionUnbuilt::Carry(geom_brep::CarryRefusal::March(
                            geom_brep::SsiError::TubeStraddles { .. },
                        )),
                    ..
                }) if coarse => break,
                r => r.unwrap_or_else(|e| panic!("{label}, {op}: {e:?}")),
            };
            let b = r
                .body()
                .unwrap_or_else(|| panic!("{label}, {op}: the operands cross, so a body"));
            assert_eq!(
                topo::validate_geometric(&b.body, Tol::witness()),
                Ok(()),
                "{label}, {op}: tier 3"
            );
            volumes.push(topo::test_support::pinned_volume(op, &b.body, Tol::witness()).volume);
        }
        let [union, inter, a_less, b_less] = volumes[..] else {
            assert!(
                coarse,
                "{label}: every op builds at {}",
                Tol::witness().eps()
            );
            test_utils::vacuity::stood_down(label, "every op's body and the volumes, at ε 1e-6");
            continue;
        };
        let pi = core::f64::consts::PI;
        let (drum, torus) = (pi / 8.0, 2.0 * pi * pi * big * small * small);
        // A marched edge stands within the band of the section, so each
        // volume moves by at most the cut's area (under 1 m² here) times
        // that: 10 ε holds it.
        let close = |got: f64, want: f64, what: &str| {
            assert!(
                (got - want).abs() < 10.0 * Tol::witness().eps(),
                "{label}: {what} {got} against {want}"
            );
        };
        assert!(inter > 0.0, "{label}: ∩ holds the crossing, {inter}");
        close(union + inter, drum + torus, "∪ + ∩");
        close(a_less, drum - inter, "A ∖ B");
        close(b_less, torus - inter, "B ∖ A");
        if label == "ring about the axis" {
            let want = ring_below_the_cut(ring_r, ring_small);
            assert!(
                (inter - want).abs() < 2e-5 * want,
                "{label}: ∩ {inter} against the quadrature's {want}"
            );
        }
    }
}

/// **The ring's tube below the cut and inside the wall**, by the
/// midpoint rule in the azimuth `φ` and the distance `ρ` from the axis:
/// at `(ρ, φ)` the tube spans `z = 0.5 ± w`, `w = √(r² − (ρ − R)²)`,
/// and the cut keeps `z < 0.5 − ρ·cos φ·tan 0.3`; the wall keeps
/// `ρ < 0.5`. The integrand is a square root at the tube's inner rim, so
/// the rule converges as `h^1.5`: at 720 × 600 cells it is within `1e-5`
/// of the limit.
fn ring_below_the_cut(big: f64, small: f64) -> f64 {
    let (n_phi, n_rho) = (720_u32, 600_u32);
    let (lo, hi) = (big - small, DRUM_RADIUS);
    let (d_phi, d_rho) = (
        core::f64::consts::TAU / f64::from(n_phi),
        (hi - lo) / f64::from(n_rho),
    );
    let lean = TILT.tan();
    let mut sum = 0.0;
    for i in 0..n_phi {
        let c = ((f64::from(i) + 0.5) * d_phi).cos();
        for j in 0..n_rho {
            let rho = lo + (f64::from(j) + 0.5) * d_rho;
            let w = (small * small - (rho - big).powi(2)).max(0.0).sqrt();
            let top = (0.5 + w).min(0.5 - rho * c * lean);
            sum += rho * (top - (0.5 - w)).max(0.0);
        }
    }
    sum * d_phi * d_rho
}

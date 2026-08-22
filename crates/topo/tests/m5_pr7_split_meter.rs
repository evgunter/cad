//! M5 PR 7 → PR 9: the rung-3 arm of `split_edge`'s
//! parameter-interiority meter (C12.3), and the end-to-end `Nurbs`
//! split row PR 7 named this PR as the trigger for.
//!
//! `split_edge` states its interiority margin in metres by multiplying
//! a parameter distance by a per-carrier meter. PR 7 gave the `Nurbs`
//! arm a real one — `NurbsCurve3::speed_lower_bound`, a certified lower
//! bound on `‖C′(t)‖` from the derivative net's convex hull — and PR 9
//! flipped `EdgeCurve::certify`'s carrier gate for the rung-3 class
//! (a `Nurbs` carrier under an `Intersection` description of two
//! analytic surfaces), so the arm is now reachable from an at-rest
//! body and the honest row is the end-to-end one: an SSI-fitted
//! carrier certified into a body, split, both children re-certified
//! through the ordinary gate. The old refusal narrows rather than
//! disappears — a `Nurbs` carrier under a CONVENTIONAL description
//! still refuses, and that boundary is pinned below too.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom::{Curve3, NurbsCurve3};
use geom_brep::ssi::{self, SsiDomain, SsiError};
use geom_brep::{EdgeCurve, EdgeCurveSpec};
use geom_core::Tol;
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Vec3};

/// A gentle cubic advancing steadily in `+x` — the shape a fitted SSI
/// carrier has.
fn nurbs_carrier() -> NurbsCurve3<f64> {
    let pts: Vec<Point3<f64>> = (0..=30)
        .map(|i| {
            let t = f64::from(i) / 30.0;
            Point3::new(t, 0.1 * (t * 4.0).sin(), 0.02 * t)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

#[test]
fn a_nurbs_carrier_under_a_conventional_description_still_refuses() {
    // PR 9 narrowed the carrier gate rather than removing it: the
    // rung-3 class is `Intersection` of two analytic surfaces (the
    // shape the zip mints). A fitted carrier smuggled under a
    // conventional description has no certified residual story and
    // keeps the typed refusal — the boundary, pinned from the
    // refusing side.
    let n = nurbs_carrier();
    let (p0, p1) = (n.eval(0.0), n.eval(1.0));
    let chord = EdgeCurveSpec::line_between(p0, p1);
    let spec = EdgeCurveSpec {
        description: chord.description,
        carrier: Curve3::Nurbs(std::sync::Arc::new(n)),
        param_start: 0.0,
        param_end: 1.0,
    };
    let err = EdgeCurve::certify(
        spec,
        p0,
        p1,
        |_| None,
        Band::linear(Tol::witness()).unwrap(),
    )
    .expect_err("conventional-description Nurbs carriers do not certify");
    let msg = format!("{err}");
    assert!(
        msg.contains("Nurbs") && msg.contains("Intersection"),
        "the refusal must name the carrier kind and the certifiable class: {msg}"
    );
}

/// One certified rung-3 branch of the PR 7 planted fixture (the offset
/// cylinder threading the unit sphere), or `None` when this ε's sample
/// demand exceeds the named fit budget — the same typed-refusal gate
/// the SSI suite stands down on (never an ε literal).
fn ssi_branch_or_budget() -> Option<ssi::SsiBranch> {
    let sph = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let cyl = Surface::Cylinder {
        origin: Point3::new(0.03, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.08,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let slab = SsiDomain {
        center: Point3::new(0.0, 0.0, 0.0),
        half_extent: 1.5,
        extent: 2.0,
        floor_scale: 1.0,
    };
    match ssi::cylinder_sphere_ssi(&cyl, &sph, slab, Band::linear(Tol::witness()).unwrap()) {
        Ok(out) => Some(out.branches.into_iter().next().expect("two loops")),
        Err(SsiError::FitSampleBudget { .. }) => None,
        Err(e) => panic!("the planted fixture: {e}"),
    }
}

/// The scaffold: a body, its rung-3 edge, the fitted carrier, and the
/// carrier's parameter window.
type Rung3Scaffold = (
    topo::Body<f64>,
    topo::EdgeKey,
    std::sync::Arc<NurbsCurve3<f64>>,
    (f64, f64),
);

/// A scaffold body holding one certified rung-3 edge (the open half of
/// the fixture's small loop, `Intersection { cylinder, sphere }`), or
/// `None` on the budget refusal. The second `mvfs` seed exists only to
/// anchor the cylinder surface so both description keys resolve
/// through the public attach door.
fn body_with_rung3_edge() -> Option<Rung3Scaffold> {
    let branch = ssi_branch_or_budget()?;
    let Curve3::Nurbs(ref loop_carrier) = branch.carrier else {
        panic!("a rung-3 carrier is a NURBS curve")
    };
    // The branch is a closed loop; an edge wants distinct endpoints.
    // Split the carrier by knot insertion (the zip's own idiom,
    // C12.3) and take the first QUARTER: an open sub-arc whose
    // turning stays well under a half turn, so the chord-direction
    // speed meter is definitely positive (a half loop's meter
    // honestly collapses — its endpoint tangents run perpendicular
    // to the chord — and the span gate would rightly escalate).
    // That collapse is a property of the INTEGRAL arm's single global
    // chord, which this fitted (unit-weight) carrier takes; the
    // rational arm projects per span and would not collapse here.
    // The fixture keeps the quarter either way — it is pinning the
    // split, not the meter's conservatism.
    let (d0, d1) = loop_carrier.domain();
    let (quarter, _) = loop_carrier.split_at(d0 + (d1 - d0) * 0.25).unwrap();
    let carrier = std::sync::Arc::new(quarter);
    let (h0, h1) = carrier.domain();
    let (p0, p1) = (carrier.eval(h0), carrier.eval(h1));

    let mut body = topo::Body::<f64>::new();
    let seed = body.mvfs(p0).unwrap();
    let sph = body
        .set_face_surface(
            seed.face,
            topo::FaceSurface::New(Surface::Sphere {
                center: Point3::new(0.0, 0.0, 0.0),
                radius: 1.0,
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }),
        )
        .unwrap();
    let anchor = body.mvfs(p1).unwrap();
    let cyl = body
        .set_face_surface(
            anchor.face,
            topo::FaceSurface::New(Surface::Cylinder {
                origin: Point3::new(0.03, 0.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: 0.08,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }),
        )
        .unwrap();
    let mid = h0 + (h1 - h0) * 0.5;
    let made = body
        .mev(
            topo::MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            EdgeCurveSpec {
                description: geom_brep::EdgeGeometry::Intersection {
                    s1: cyl,
                    s2: sph,
                    // The witness contract: carrier(mid), bitwise the
                    // certification schedule's middle sample.
                    witness: carrier.eval(mid),
                },
                carrier: Curve3::Nurbs(carrier.clone()),
                param_start: h0,
                param_end: h1,
            },
            Tol::witness(),
        )
        .expect("the kernel's first rung-3 edge at rest certifies");
    Some((body, made.edge, carrier, (h0, h1)))
}

#[test]
fn the_end_to_end_nurbs_split_row() {
    // The row PR 7 banked (fix-pass item m8): an SSI-fitted carrier
    // certified into a body through the ordinary gate, split by
    // `split_edge`, both children re-certified — the kernel's first
    // rung-3 edge at rest, split at rest.
    let Some((mut body, edge, carrier, (h0, h1))) = body_with_rung3_edge() else {
        return; // typed FitSampleBudget refusal at this ε — its own row pins it
    };

    // Split at an interior parameter: both children re-certify
    // (their witnesses re-minted at their own mid-parameters), the
    // meter having stated the interiority margin in metres.
    let t_split = h0 + (h1 - h0) * 0.375;
    let created = body
        .split_edge(edge, t_split, Tol::witness())
        .expect("the split row");
    let c1 = body
        .get_curve_geom(created.first_curve)
        .and_then(|g| g.certified())
        .unwrap();
    let c2 = body
        .get_curve_geom(created.second_curve)
        .and_then(|g| g.certified())
        .unwrap();
    assert_eq!(c1.params(), (h0, t_split));
    assert_eq!(c2.params(), (t_split, h1));
    assert!(matches!(c1.carrier(), Curve3::Nurbs(_)));
    let p_split = *body
        .get_point(body.get_vertex(created.vertex).unwrap().point)
        .unwrap();
    assert!(
        p_split.distance(carrier.eval(t_split)) == 0.0,
        "the split vertex is the carrier point exactly (D9)"
    );
}

#[test]
fn a_split_at_the_endpoint_band_escalates_or_refuses_in_metres() {
    // The meter's other face: a parameter distance whose metred span
    // is NOT definitely positive must refuse/escalate, scaled from
    // the resolved band (multi-ε honesty) — never accepted.
    let Some((mut body, edge, carrier, (h0, _h1))) = body_with_rung3_edge() else {
        return;
    };
    // A split parameter whose distance-to-endpoint, METERED through
    // the certified speed bound, sits inside the band: place it from
    // the band the run resolved, through the carrier's own meter.
    let meter = carrier.speed_lower_bound();
    assert!(meter > 0.0);
    let band = Band::linear(Tol::witness()).unwrap();
    let t_bad = h0 + 0.5 * (band.zero() + band.escalate()) / meter;
    assert!(
        body.split_edge(edge, t_bad, Tol::witness()).is_err(),
        "an in-band interiority margin must never split"
    );
}

#[test]
fn the_meter_the_split_arm_will_use_is_a_real_lower_bound() {
    // The arm itself: `split_edge` computes `(t − t₀) · meter` in
    // metres, so the meter must never exceed the true speed — a meter
    // that overstated it would accept a split that is NOT clear of the
    // endpoints, which is the failure the conservative posture exists
    // to prevent (`Circle` ⇒ radius, `Ellipse` ⇒ the MINOR semi-axis).
    let n = nurbs_carrier();
    let m = n.speed_lower_bound();
    assert!(m > 0.0, "a monotone carrier meters positively: {m}");
    for i in 0..=400 {
        let t = f64::from(i) / 400.0;
        assert!(
            n.deriv(t).norm() >= m - 1e-12,
            "meter {m} exceeds the real speed at t = {t}"
        );
    }
    // And the interiority margin the split gate would compute is a
    // genuine length: a parameter distance of 0.25 on this carrier is
    // at least `0.25 · m` metres of arc.
    let span = 0.25;
    let arc: f64 = (0..1000)
        .map(|i| {
            let a = span * f64::from(i) / 1000.0;
            let b = span * f64::from(i + 1) / 1000.0;
            (n.eval(b) - n.eval(a)).norm()
        })
        .sum();
    assert!(
        arc >= span * m - 1e-9,
        "the metered margin {} overstates the arc {arc}",
        span * m
    );
}

/// An exact quarter of the unit circle in `z = 0`, as the rational
/// quadratic it is — the carrier shape a sweep's arc profile mints.
/// It lies EXACTLY on the unit sphere and on the plane `z = 0`, which
/// is what lets it certify as a rung-3 `Intersection` edge.
fn rational_quarter_arc() -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let w = 0.5f64.sqrt();
    NurbsCurve3::new(
        kv,
        vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ],
        vec![1.0, w, 1.0],
    )
    .unwrap()
}

#[test]
fn a_rational_carrier_splits_with_a_metered_interiority() {
    // FLIPPED from `a_rational_carrier_would_poison_the_meter_and_
    // escalate` (M5 PR 7 → M7). The convex-hull argument still needs a
    // non-rational derivative net, but `speed_lower_bound` no longer
    // stops there: its rational arm assembles the bound from the
    // HOMOGENEOUS net through the quotient rule (derivation on the
    // method). So the `split_edge_param_interior` trilean now has a
    // real metre-per-parameter for a rational carrier, and the arm
    // runs end to end instead of escalating.
    //
    // The fixture is the plainest rational carrier there is: an exact
    // quarter circle, which lies on the unit sphere AND on `z = 0`, so
    // it certifies as an `Intersection` of two analytic surfaces with
    // zero residual — a rung-3 edge whose carrier is rational on its
    // own merits, not by any manufactured weight channel.
    let carrier = std::sync::Arc::new(rational_quarter_arc());
    let (h0, h1) = carrier.domain();
    let (p0, p1) = (carrier.eval(h0), carrier.eval(h1));

    let mut body = topo::Body::<f64>::new();
    let seed = body.mvfs(p0).unwrap();
    let sph = body
        .set_face_surface(
            seed.face,
            topo::FaceSurface::New(Surface::Sphere {
                center: Point3::new(0.0, 0.0, 0.0),
                radius: 1.0,
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }),
        )
        .unwrap();
    let anchor = body.mvfs(p1).unwrap();
    let plane = body
        .set_face_surface(
            anchor.face,
            topo::FaceSurface::New(Surface::Plane {
                origin: Point3::new(0.0, 0.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }),
        )
        .unwrap();
    let mid = h0 + (h1 - h0) * 0.5;
    let made = body
        .mev(
            topo::MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            EdgeCurveSpec {
                description: geom_brep::EdgeGeometry::Intersection {
                    s1: sph,
                    s2: plane,
                    witness: carrier.eval(mid),
                },
                carrier: Curve3::Nurbs(carrier.clone()),
                param_start: h0,
                param_end: h1,
            },
            Tol::witness(),
        )
        .expect("a rational rung-3 edge certifies — the span meter is real now");

    // The margins are honest: the meter is a genuine lower bound on
    // the carrier's speed, and the metered interiority it hands the
    // split gate never overstates the arc it stands for.
    let m = carrier.speed_lower_bound();
    assert!(m > 0.0, "a rational carrier meters positively: {m}");
    for i in 0..=400 {
        let t = h0 + (h1 - h0) * f64::from(i) / 400.0;
        assert!(
            carrier.deriv(t).norm() >= m - 1e-12,
            "meter {m} exceeds the real speed at t = {t}"
        );
    }
    let span = (h1 - h0) * 0.25;
    let arc: f64 = (0..1000)
        .map(|i| {
            let a = h0 + span * f64::from(i) / 1000.0;
            let b = h0 + span * f64::from(i + 1) / 1000.0;
            (carrier.eval(b) - carrier.eval(a)).norm()
        })
        .sum();
    assert!(
        arc >= span * m - 1e-9,
        "the metered margin {} overstates the arc {arc}",
        span * m
    );

    // And the split itself runs, both children re-certifying through
    // the ordinary gate.
    let t_split = h0 + (h1 - h0) * 0.375;
    let created = body
        .split_edge(made.edge, t_split, Tol::witness())
        .expect("the rational split row");
    let c1 = body
        .get_curve_geom(created.first_curve)
        .and_then(|g| g.certified())
        .unwrap();
    let c2 = body
        .get_curve_geom(created.second_curve)
        .and_then(|g| g.certified())
        .unwrap();
    assert_eq!(c1.params(), (h0, t_split));
    assert_eq!(c2.params(), (t_split, h1));
    let p_split = *body
        .get_point(body.get_vertex(created.vertex).unwrap().point)
        .unwrap();
    assert!(
        p_split.distance(carrier.eval(t_split)) == 0.0,
        "the split vertex is the carrier point exactly (D9)"
    );

    // The other face of the meter is unchanged: a split parameter
    // whose METERED distance-to-endpoint sits inside the band still
    // refuses, on the rational arm exactly as on the integral one.
    let mut body2 = topo::Body::<f64>::new();
    let seed2 = body2.mvfs(p0).unwrap();
    let sph2 = body2
        .set_face_surface(
            seed2.face,
            topo::FaceSurface::New(Surface::Sphere {
                center: Point3::new(0.0, 0.0, 0.0),
                radius: 1.0,
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }),
        )
        .unwrap();
    let anchor2 = body2.mvfs(p1).unwrap();
    let plane2 = body2
        .set_face_surface(
            anchor2.face,
            topo::FaceSurface::New(Surface::Plane {
                origin: Point3::new(0.0, 0.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }),
        )
        .unwrap();
    let made2 = body2
        .mev(
            topo::MevSite::Lone {
                r#loop: seed2.r#loop,
            },
            p1,
            EdgeCurveSpec {
                description: geom_brep::EdgeGeometry::Intersection {
                    s1: sph2,
                    s2: plane2,
                    witness: carrier.eval(mid),
                },
                carrier: Curve3::Nurbs(carrier.clone()),
                param_start: h0,
                param_end: h1,
            },
            Tol::witness(),
        )
        .unwrap();
    let band = Band::linear(Tol::witness()).unwrap();
    let t_bad = h0 + 0.5 * (band.zero() + band.escalate()) / m;
    assert!(
        body2.split_edge(made2.edge, t_bad, Tol::witness()).is_err(),
        "an in-band interiority margin must never split, rational or not"
    );
}

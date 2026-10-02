//! PR 3759 dual review, R2's probes: whether tier 3's pcurve pass
//! (`validate_pcurves`) reports every rowless curved face C4 says is
//! invalid, and whether its verdict depends on where in the loop the
//! walk meets each edge.
//!
//! The fixture is a quarter revolve of `dome_profile`, so the sphere wall
//! is minted. Two struts are added to that wall:
//!
//! - a GENERAL circle that lies on the sphere (neither polar nor
//!   meridian), which `chart_pcurve` refuses as uncovered
//!   (`UncoveredClass::SphereGeneralCircle`), the one refusal
//!   `uncovered()` excuses;
//! - a small circle in the sphere's tangent plane at its vertex, which
//!   is OFF the sphere (`CarrierOffChart`, a body defect).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{dome_profile, revolved_about_y};
use topo::pcurves::validate_pcurves;
use topo::{Body, FaceKey, HalfEdgeKey, MevSite, PcurveMintError};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

fn dome_quarter() -> Body<f64> {
    revolved_about_y(
        dome_profile(1.0),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        tol(),
    )
}

/// The minted sphere wall, its centre and radius, and its outer cycle.
fn sphere_wall(body: &Body<f64>) -> (FaceKey, Point3<f64>, f64, Vec<HalfEdgeKey>) {
    body.faces()
        .find_map(|(fk, f)| {
            let Surface::Sphere { center, radius, .. } = *body.get_surface(f.surface).unwrap()
            else {
                return None;
            };
            let topo::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary
            else {
                return None;
            };
            assert!(
                body.pcurve(first).is_some(),
                "the dome's sphere wall is minted"
            );
            Some((fk, center, radius, body.loop_cycle(first).unwrap()))
        })
        .expect("the dome has a sphere wall")
}

fn start_point(body: &Body<f64>, he: HalfEdgeKey) -> Point3<f64> {
    let v = body.get_half_edge(he).unwrap().start;
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

fn unit(v: Vec3<f64>) -> Vec3<f64> {
    v * (1.0 / v.dot(v).sqrt())
}

/// A circle on the sphere `(c, r)` through `p`, in a plane tilted off
/// both polar (⊥ Y) and meridian (∋ Y).
fn general_circle_through(c: Point3<f64>, r: f64, p: Point3<f64>) -> Curve3<f64> {
    let n = unit(Vec3::new(0.7, 1.0, 0.4));
    let d = (p - c).dot(n);
    let center = c + n * d;
    Curve3::Circle {
        center,
        axis: n,
        radius: (r * r - d * d).sqrt(),
        u_ref: unit(p - center),
    }
}

/// A small circle in the sphere's tangent plane at `p`: it meets the
/// sphere only at `p`.
fn tangent_circle_at(c: Point3<f64>, p: Point3<f64>) -> Curve3<f64> {
    let normal = unit(p - c);
    let helper = if normal.x.abs() < 0.9 {
        Vec3::unit_x()
    } else {
        Vec3::unit_z()
    };
    let t = unit(helper - normal * helper.dot(normal));
    let radius = 0.1;
    let center = p + t * radius;
    Curve3::Circle {
        center,
        axis: normal,
        radius,
        u_ref: unit(p - center),
    }
}

fn strut(body: &mut Body<f64>, he: HalfEdgeKey, carrier: Curve3<f64>, span: f64) {
    let end = carrier.eval(span);
    let spec = EdgeCurveSpec::arc_of_circle(carrier, 0.0, span).unwrap();
    body.mev(MevSite::Fan { he1: he, he2: he }, end, spec, tol())
        .unwrap();
}

fn off_chart(e: &PcurveMintError) -> bool {
    matches!(
        e,
        PcurveMintError::Certify {
            error: geom_brep::PcurveCertifyError::CarrierOffChart { .. },
            ..
        }
    )
}

/// The verdicts of tier 3 and of the mint on the sphere wall, with the
/// general strut at cycle position `g` and the off-chart strut at `o`
/// (`None` leaves that strut out).
fn verdicts(
    g: Option<usize>,
    o: Option<usize>,
) -> (
    Vec<PcurveMintError>,
    Result<usize, PcurveMintError>,
    Body<f64>,
) {
    let mut body = dome_quarter();
    let (wall, c, r, cycle) = sphere_wall(&body);
    if let Some(i) = g {
        let p = start_point(&body, cycle[i]);
        strut(&mut body, cycle[i], general_circle_through(c, r, p), 0.2);
    }
    if let Some(i) = o {
        let p = start_point(&body, cycle[i]);
        strut(&mut body, cycle[i], tangent_circle_at(c, p), 0.5);
    }
    let findings = validate_pcurves(&body, band());
    let mut minted = body.clone();
    let mint = topo::mint_pcurves_of(&mut minted, &[wall], tol());
    (findings, mint, body)
}

/// Controls: each strut alone. The general circle is uncovered and
/// excused (tier 3 clean, mint stores nothing); the off-chart circle is
/// loud in both.
#[test]
fn r2_controls_each_strut_alone() {
    let (f, m, _) = verdicts(Some(0), None);
    eprintln!("general alone: tier3={f:?} mint={m:?}");
    assert!(f.is_empty(), "the uncovered strut is excused: {f:?}");
    assert_eq!(m, Ok(0), "the mint excuses the wall, storing nothing");
    for i in 0..2 {
        let (f, m, _) = verdicts(None, Some(i));
        eprintln!("off-chart alone at {i}: tier3={f:?} mint={m:?}");
        assert!(matches!(f.as_slice(), [e] if off_chart(e)), "{f:?}");
        assert!(matches!(&m, Err(e) if off_chart(e)), "{m:?}");
    }
}

/// **The probe.** The same wall carries the same off-chart strut,
/// with an uncovered strut beside it, in either cycle order. C4: "a
/// face no route covers refuses at the producer"; and the off-chart
/// strut is a defect whatever else the face carries. Tier 3's verdict
/// here is whichever refusal the walk meets FIRST.
#[test]
fn r2_an_uncovered_strut_masks_an_off_chart_strut_by_cycle_order() {
    let mut silent = 0;
    for (g, o) in [(0, 1), (1, 0), (0, 2), (2, 0)] {
        let (f, m, body) = verdicts(Some(g), Some(o));
        eprintln!("general at {g}, off-chart at {o}: tier3={f:?} mint={m:?}");
        if f.is_empty() {
            silent += 1;
            assert_eq!(m, Ok(0), "the mint excuses the wall too (one predicate)");
            // The same body, reversed: the walk meets the edges in the
            // other order.
            let reverted = body.revert().unwrap();
            let fr = validate_pcurves(&reverted, band());
            eprintln!("   reverted: tier3={fr:?}");
        }
    }
    assert_eq!(
        silent, 0,
        "an off-chart strut passed tier 3 in {silent} of 4 orderings because an uncovered \
         strut came first in the walk"
    );
}

/// What the whole at-rest gate says about the masked body and about the
/// off-chart-alone body (informational: prints, asserts nothing).
#[test]
fn r2_whole_gate_on_the_masked_body() {
    for (label, g, o) in [
        ("masked", Some(1), Some(0)),
        ("off-chart alone", None, Some(0)),
        ("general alone", Some(1), None),
    ] {
        let (_, _, body) = verdicts(g, o);
        let t2 = topo::validate(&body);
        let t3 = topo::validate_geometric(&body, tol());
        eprintln!("{label}: tier2={t2:?}\n   geometric={t3:?}");
    }
}

/// **Claim 1's window.** On a minted cylinder wall, one row is
/// re-certified over MORE of its carrier than the edge spans (a stale
/// row from before a split, say) and a DIFFERENT half-edge's row is
/// detached, so the face is half-minted. The PR: "the window comes from
/// the derivation (the face), not from the rows under check, so a stored
/// row stated over more of the chart than the face reaches fails
/// `TrimEscape`". Prints what tier 3 reports for each choice of the
/// gap, extending each end of the row.
#[test]
fn r2_a_stale_wide_row_on_a_half_minted_face() {
    let base =
        crate::common::operands::n_arc_boss::<f64>(geom_core::Point2::new(0.0, 0.0), 3, 0.0, 1.0);
    let (wall, cycle) = base
        .faces()
        .find_map(|(fk, f)| {
            if !matches!(
                base.get_surface(f.surface).unwrap(),
                Surface::Cylinder { .. }
            ) {
                return None;
            }
            let topo::LoopBoundary::Cycle { first } = base.get_loop(f.outer).unwrap().boundary
            else {
                return None;
            };
            Some((fk, base.loop_cycle(first).unwrap()))
        })
        .unwrap();
    let surface = {
        let f = base.get_face(wall).unwrap();
        base.get_surface(f.surface).unwrap().clone()
    };
    let mut any_silent = false;
    for (i, &h1) in cycle.iter().enumerate() {
        let edge = base.get_edge(base.get_half_edge(h1).unwrap().edge).unwrap();
        let carrier = base
            .get_curve_geom(edge.curve)
            .unwrap()
            .certified()
            .unwrap()
            .carrier()
            .clone();
        if !matches!(carrier, Curve3::Circle { .. }) {
            continue;
        }
        let cache = base.pcurve(h1).unwrap().clone();
        let (t0, t1) = cache.params();
        for (lo, hi, end) in [(t0, t1 + 0.4, "t1"), (t0 - 0.4, t1, "t0")] {
            for (j, &h2) in cycle.iter().enumerate() {
                if j == i && std::env::var("R2_COMPLETE").is_err() {
                    continue;
                }
                let mut body = base.clone();
                let wide = cache.pcurve().clone();
                let window = wide.chart_box(lo, hi);
                let row = geom_brep::PcurveCache::certify(
                    wide,
                    lo,
                    hi,
                    &carrier,
                    &surface,
                    window,
                    band(),
                )
                .expect("the carrier's own image certifies over a longer span");
                body.attach_pcurve(h1, row);
                if j != i {
                    body.detach_pcurve(h2);
                }
                let f = validate_pcurves(&body, band());
                let escapes = f.iter().any(|e| {
                    matches!(
                        e,
                        PcurveMintError::Certify {
                            half_edge,
                            error: geom_brep::PcurveCertifyError::TrimEscape { .. },
                        } if *half_edge == h1
                    )
                });
                let about_h1 = f
                    .iter()
                    .any(|e| format!("{e:?}").contains(&format!("{h1:?}")));
                eprintln!(
                    "row {i} wide at {end}, gap {j}: TrimEscape(h1)={escapes} any-finding-on-h1={about_h1} {f:?}"
                );
                any_silent |= !about_h1;
            }
        }
    }
    assert!(
        !any_silent,
        "a stale wide row on a half-minted face passed unmeasured"
    );
}

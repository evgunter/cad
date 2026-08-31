//! BOOL-1 fix-pass rows (issue 1152, PR 1378): the coverage the two
//! blinded reviews found missing.
//!
//! 1. **Keep-vs-restate is observable** — the tangent-tip edge sits
//!    between TWO section faces, so it is visited twice and the chart
//!    it ends with is the FIRST visit's restate, kept by the second
//!    visit's coherence rule. Pinning WHICH adjacent chart it carries
//!    kills the mutant that deletes the keep rule (always-restate ends
//!    with the second visit's chart) and the mutant that drops the
//!    `s_other` clause from it.
//! 2. **Restate-vs-rebuild differs on a conic** — a boss-on-plate
//!    solid of revolution split at its own annulus plane puts a full
//!    CIRCLE on the section boundary between flush planes. The
//!    restated edge must keep the circular carrier bitwise; a rebuild
//!    through `line_between` cannot represent it at all (the committed
//!    reproduction's carriers are axis-aligned lines, where a rebuild
//!    is bit-identical and mutation-invisible).
//! 3. **The band posture, pinned per band** — the near-flush ladder's
//!    four regimes at band-relative displacements, asserted (not
//!    printed) at whatever ε the run committed, both signs: flush
//!    enough certifies, in the residual band the split REFUSES typed
//!    through certification's escalation lane (`ChartResidual` /
//!    `pcurve_map_residual` — the refusal reachable on this branch and
//!    not on main), in the vertex band it refuses `SliverVertex` as
//!    ever, and definitely-off runs the generic split.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, FaceKey, SplitError, SplitFinishError, SplitReduceError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn extruded(loops: Vec<ProfileLoop<f64>>, h: f64) -> Body<f64> {
    let prof = Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .expect("a valid profile");
    extrude(&prof, Extrusion::Distance(h), Tol::witness())
        .expect("the profile extrudes")
        .body
}

/// The issue-1152 notched block with the notch floor at `1 + dy`.
fn notched(dy: f64) -> ProfileLoop<f64> {
    ProfileLoop::polygon(
        [
            (0.0, 0.0),
            (8.0, 0.0),
            (8.0, 2.0),
            (7.0, 1.0 + dy),
            (6.0, 1.0 + dy),
            (5.0, 2.0),
            (4.0, 1.0 + dy),
            (3.0, 2.0),
            (0.0, 2.0),
        ]
        .map(|(x, y)| p2(x, y)),
    )
}

fn plane_y1() -> topo::SplitPlane<f64> {
    topo::SplitPlane {
        origin: Point3::new(0.0, 1.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
    }
}

/// The edge's two faces, ordered by arena key.
fn faces_of(body: &Body<f64>, e: topo::EdgeKey) -> (FaceKey, FaceKey) {
    let ed = body.get_edge(e).expect("edge");
    let face_of = |he| {
        let l = body.get_half_edge(he).expect("he").parent_loop;
        body.get_loop(l).expect("loop").face
    };
    let (a, b) = (face_of(ed.he_plus), face_of(ed.he_minus));
    if a <= b { (a, b) } else { (b, a) }
}

/// Row 1: the tangent-tip edge (between two section faces) carries the
/// FIRST-visited section face's chart — the keep rule pinned where
/// keep and restate genuinely differ.
#[test]
fn tip_edge_keeps_the_first_visits_section_chart() {
    let body = extruded(vec![notched(0.0)], 1.0);
    let result = topo::split(&body, &plane_y1(), Tol::witness()).expect("the coplanar split runs");
    let below = result.below.body().expect("below has material");
    // The tip edge: the vertical at (4, 1, ·).
    let tip = below
        .edges()
        .filter(|(_, e)| {
            let at_tip = |he| {
                let s = below.get_half_edge(he).unwrap().start;
                let p = *below.get_point(below.get_vertex(s).unwrap().point).unwrap();
                p.x == 4.0 && p.y == 1.0
            };
            at_tip(e.he_plus) && at_tip(e.he_minus)
        })
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    assert_eq!(tip.len(), 1, "one tip edge in below");
    let (f_first, f_second) = faces_of(below, tip[0]);
    let surf = |f: FaceKey| below.get_face(f).unwrap().surface;
    assert_ne!(
        surf(f_first),
        surf(f_second),
        "the tip edge sits between two DISTINCT section charts"
    );
    let c = below
        .get_curve_geom(below.get_edge(tip[0]).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .expect("certified");
    let geom_brep::EdgeDescription::Chart(ch) = c.description() else {
        panic!(
            "the tip edge is conventional after the restate: {:?}",
            c.description()
        );
    };
    // Section faces are visited in arena key order, the first visit
    // restates, and the second KEEPS — so the chart is the lower
    // key's. An always-restate arm would leave the higher key's chart
    // here instead.
    assert_eq!(
        ch.surface,
        surf(f_first),
        "the kept chart is the first visit's, not the last's"
    );
}

/// Row 2: a CONIC section-boundary edge — the boss-on-plate solid of
/// revolution split at its own annulus plane — restates on its own
/// circular carrier, bitwise.
#[test]
fn conic_section_boundary_restates_on_its_own_carrier() {
    // Plate r=2 for y in [0,1], boss r=1 for y in [1,2], as one solid
    // of revolution about the y axis.
    let profile = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 1.0),
        p2(1.0, 1.0),
        p2(1.0, 2.0),
        p2(0.0, 2.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![profile])
        .validate(Tol::witness())
        .expect("a valid revolve profile");
    let body = revolve(
        &vp,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: geom_core::Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the boss-on-plate revolves")
    .body;
    // The operand's circle carriers at the boss joint (r = 1, y = 1),
    // as bit-printed (carrier, interval) records.
    let joint_circles = |b: &Body<f64>| -> Vec<String> {
        let mut v: Vec<String> = b
            .edges()
            .filter_map(|(_, e)| {
                let c = b.get_curve_geom(e.curve)?.certified()?;
                let geom::Curve3::Circle { center, radius, .. } = *c.carrier() else {
                    return None;
                };
                (center.y == 1.0 && radius == 1.0).then(|| {
                    let (t0, t1) = c.params();
                    format!("{:?} [{:x},{:x}]", c.carrier(), t0.to_bits(), t1.to_bits())
                })
            })
            .collect();
        v.sort();
        v
    };
    let before = joint_circles(&body);
    assert!(
        !before.is_empty(),
        "the operand has circle carriers at the boss joint"
    );
    let result = topo::split(&body, &plane_y1(), Tol::witness()).expect("the coplanar split runs");
    let below = result.below.body().expect("below has material");
    assert_eq!(
        topo::validate_geometric(below, Tol::witness()),
        Ok(()),
        "below at tier 3"
    );
    assert_eq!(
        topo::validate_geometric(result.above.body().expect("above"), Tol::witness()),
        Ok(()),
        "above at tier 3"
    );
    // The joint circles surviving in below kept their carriers and
    // intervals bitwise, and every one is conventionally described in
    // an adjacent chart (the flush annulus/section pair).
    let after = joint_circles(below);
    assert!(
        !after.is_empty(),
        "the joint circle lands on below's section boundary"
    );
    for rec in &after {
        assert!(
            before.contains(rec),
            "a joint circle's carrier or interval moved across the restatement:\n\
             after:  {rec}\n\
             before: {before:?}"
        );
    }
    for (k, e) in below.edges() {
        let Some(c) = below
            .get_curve_geom(e.curve)
            .and_then(topo::CurveGeom::certified)
        else {
            continue;
        };
        let geom::Curve3::Circle { center, radius, .. } = *c.carrier() else {
            continue;
        };
        if !(center.y == 1.0 && radius == 1.0) {
            continue;
        }
        let geom_brep::EdgeDescription::Chart(ch) = c.description() else {
            panic!("joint circle {k:?} is conventional after the restate");
        };
        let (fa, fb) = faces_of(below, k);
        let surf = |f: FaceKey| below.get_face(f).unwrap().surface;
        assert!(
            ch.surface == surf(fa) || ch.surface == surf(fb),
            "joint circle {k:?}'s chart names an adjacent surface"
        );
    }
}

/// Row 3: the near-flush ladder's four regimes, asserted per band at
/// band-relative displacements (both signs). The `ChartResidual`
/// regime is the refusal this branch makes reachable out of `split`;
/// its pin is what keeps the band posture honest at every ε the
/// matrix draws.
#[test]
fn near_flush_regimes_pin_per_band() {
    let t = Tol::witness().get();
    let (eps, k) = (t.eps, t.k);
    // The section-boundary containment residual of a floor displaced
    // by dy meters is 8·dy (the notch floor's own lever); the vertex
    // margin is dy. Regime boundaries follow from band zero = ε,
    // escalate = K·ε.
    let dy_flush = eps / 16.0; // vertex ON, residual ε/2: certifies
    let dy_resid = eps * (0.125 + 1.0_f64.min(k / 8.0)) / 2.0; // vertex ON, residual in band
    let dy_sliver = eps * (1.0 + k) / 2.0; // vertex in the escalation band
    let dy_clear = eps * k * 8.0; // definitely off: the generic split
    for sign in [1.0, -1.0] {
        for (dy, want) in [
            (sign * dy_flush, "ok"),
            (sign * dy_resid, "chart-residual"),
            (sign * dy_sliver, "sliver-vertex"),
            (sign * dy_clear, "ok"),
        ] {
            let body = extruded(vec![notched(dy)], 1.0);
            let got = topo::split(&body, &plane_y1(), Tol::witness());
            match (want, got) {
                ("ok", Ok(r)) => {
                    for (name, part) in [("above", &r.above), ("below", &r.below)] {
                        let b = part.body().expect("side has material");
                        assert_eq!(
                            topo::validate_geometric(b, Tol::witness()),
                            Ok(()),
                            "dy={dy:e}: {name} at tier 3"
                        );
                    }
                }
                (
                    "chart-residual",
                    Err(SplitError::Finish(SplitFinishError::Euler(
                        topo::EulerOpError::Certification {
                            error:
                                geom_brep::CertifyError::Escalated {
                                    check: geom_brep::CertCheck::ChartResidual,
                                    ..
                                },
                        },
                    ))),
                ) => {}
                (
                    "sliver-vertex",
                    Err(SplitError::Reduce(SplitReduceError::SliverVertex { .. })),
                ) => {}
                (want, got) => {
                    panic!("dy={dy:e} (eps={eps:e}, k={k}): expected {want}, got {got:?}")
                }
            }
        }
    }
}

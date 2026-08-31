//! BOOL-1 R2 review probes (issue 1152, PR 1378): attack the fix's
//! keep-vs-restate rule and band posture on shapes the unit did not
//! draw — a second coplanar split over descriptions the FIRST split
//! minted, a transverse re-split of a restated product, and a
//! bitwise carrier comparison across the restatement.

use geom_brep::EdgeDescription;
use geom_core::{Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, ValidationError};

fn p2(x: f64, y: f64) -> geom_core::Point2<f64> {
    geom_core::Point2::new(x, y)
}

fn extruded(loops: Vec<ProfileLoop<f64>>, h: f64) -> Body<f64> {
    let prof = Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .expect("valid probe profile");
    extrude(&prof, Extrusion::Distance(h), Tol::witness())
        .expect("the probe profile extrudes")
        .body
}

fn split_at_y(body: &Body<f64>, y: f64) -> topo::SplitResult<f64> {
    topo::split(
        body,
        &topo::SplitPlane {
            origin: Point3::new(0.0, y, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
        },
        Tol::witness(),
    )
    .expect("the face-coplanar split runs")
}

fn tier3(body: &Body<f64>, ctx: &str) {
    assert_eq!(
        topo::validate_geometric(body, Tol::witness()),
        Ok(()),
        "{ctx}: tier 3 must pass"
    );
    let scaffolds: Vec<EdgeKey> = body
        .edges()
        .filter(|(_, e)| {
            body.get_curve_geom(e.curve)
                .and_then(topo::CurveGeom::certified)
                .is_some_and(|c| matches!(c.description(), EdgeDescription::Scaffold(_)))
        })
        .map(|(k, _)| k)
        .collect();
    assert!(scaffolds.is_empty(), "{ctx}: scaffolds at rest {scaffolds:?}");
}

/// Two notches at DIFFERENT depths (floors y=1 and y=0.5). Split #1 at
/// y=1 (coplanar with the shallow floor) exercises the fixed arm; the
/// below product then carries descriptions the first split minted.
/// Split #2 at y=0.5 (coplanar with the deep floor) runs the arm AGAIN
/// over that product — the class re-entering over its own output.
#[test]
fn two_successive_coplanar_splits_stay_tier3() {
    let profile = ProfileLoop::polygon(
        [
            (0.0, 0.0),
            (8.0, 0.0),
            (8.0, 2.0),
            (6.0, 2.0),
            (6.0, 1.0),
            (5.0, 1.0),
            (5.0, 2.0),
            (3.0, 2.0),
            (3.0, 0.5),
            (2.0, 0.5),
            (2.0, 2.0),
            (0.0, 2.0),
        ]
        .map(|(x, y)| p2(x, y)),
    );
    let body = extruded(vec![profile], 1.0);
    let first = split_at_y(&body, 1.0);
    let below1 = first.below.body().expect("below of split #1");
    tier3(first.above.body().expect("above of split #1"), "split#1 above");
    tier3(below1, "split#1 below");

    let second = split_at_y(below1, 0.5);
    tier3(second.above.body().expect("above of split #2"), "split#2 above");
    tier3(second.below.body().expect("below of split #2"), "split#2 below");
}

/// A transverse re-split of a product whose section-boundary edges the
/// first (coplanar) split restated as chart images: the restated
/// descriptions must survive the second split's describe pass wherever
/// they remain smooth, and both second-split products must hold tier 3.
#[test]
fn transverse_resplit_of_a_restated_product_stays_tier3() {
    let notched = ProfileLoop::polygon(
        [
            (0.0, 0.0),
            (8.0, 0.0),
            (8.0, 2.0),
            (7.0, 1.0),
            (6.0, 1.0),
            (5.0, 2.0),
            (4.0, 1.0),
            (3.0, 2.0),
            (0.0, 2.0),
        ]
        .map(|(x, y)| p2(x, y)),
    );
    let body = extruded(vec![notched], 1.0);
    let first = split_at_y(&body, 1.0);
    let below1 = first.below.body().expect("below of split #1");
    tier3(below1, "coplanar split below");

    // Transverse second cut straight through the restated edges' span.
    let second = topo::split(
        below1,
        &topo::SplitPlane {
            origin: Point3::new(6.5, 0.0, 0.0),
            normal: Vec3::new(1.0, 0.0, 0.0),
        },
        Tol::witness(),
    )
    .expect("the transverse re-split runs");
    tier3(second.above.body().expect("above"), "re-split above");
    tier3(second.below.body().expect("below"), "re-split below");
}

/// The restatement moves NO carrier bits (the rebuild-vs-restate 1-ULP
/// class): the three operand edges the coplanar split restates keep
/// their extrude-minted line origin/dir and parameter interval bitwise.
#[test]
fn restated_edges_keep_carrier_bits() {
    let notched = ProfileLoop::polygon(
        [
            (0.0, 0.0),
            (8.0, 0.0),
            (8.0, 2.0),
            (7.0, 1.0),
            (6.0, 1.0),
            (5.0, 2.0),
            (4.0, 1.0),
            (3.0, 2.0),
            (0.0, 2.0),
        ]
        .map(|(x, y)| p2(x, y)),
    );
    let body = extruded(vec![notched], 1.0);
    let before: Vec<(EdgeKey, geom::Curve3<f64>, f64, f64)> = body
        .edges()
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            let (t0, t1) = c.params();
            Some((k, c.carrier().clone(), t0, t1))
        })
        .collect();
    let result = split_at_y(&body, 1.0);
    let below = result.below.body().expect("below");
    let mut checked = 0usize;
    for (k, e) in below.edges() {
        let Some(c) = below.get_curve_geom(e.curve).and_then(topo::CurveGeom::certified) else {
            continue;
        };
        // Only edges restated to a chart image by the split's smooth arm.
        if !matches!(c.description(), EdgeDescription::Chart(_)) {
            continue;
        }
        let Some((_, carrier0, t0, t1)) = before.iter().find(|(bk, ..)| *bk == k) else {
            continue; // split-minted edge, no operand twin
        };
        let (u0, u1) = c.params();
        assert_eq!(
            format!("{:?}", c.carrier()),
            format!("{carrier0:?}"),
            "edge {k:?}: carrier moved across restatement"
        );
        assert!(
            u0.to_bits() == t0.to_bits() && u1.to_bits() == t1.to_bits(),
            "edge {k:?}: interval moved across restatement"
        );
        checked += 1;
    }
    assert_eq!(checked, 3, "expected exactly the three restated operand edges");
}

/// The site's unreachability argument, attacked with the input it
/// names: a split plane TANGENT to a curved (cylinder) wall must not
/// reach the smooth arm with a determinate jet — the verb refuses
/// typed (or legitimately produces no cut), never hands back a body
/// tier 3 rejects.
#[test]
fn tangent_plane_split_of_a_cylinder_never_reaches_the_smooth_arm() {
    // Full circle profile (two semicircular arcs) of radius 1 at the
    // origin, extruded: a cylinder barrel with planar caps.
    let circle = ProfileLoop::new(vec![
        profile::ProfileVertex::new(p2(-1.0, 0.0), 1.0),
        profile::ProfileVertex::new(p2(1.0, 0.0), 1.0),
    ]);
    let body = extruded(vec![circle], 1.0);
    // Plane y = 1 is tangent to the barrel along the line (0,1,z).
    let attempt = topo::split(
        &body,
        &topo::SplitPlane {
            origin: Point3::new(0.0, 1.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
        },
        Tol::witness(),
    );
    match attempt {
        Ok(r) => {
            // A no-cut (whole body one side) is legitimate; a cut that
            // passed through the tangency must still meter tier 3.
            for (part, ctx) in [(&r.above, "above"), (&r.below, "below")] {
                if let Some(b) = part.body() {
                    let verdict = topo::validate_geometric(b, Tol::witness());
                    assert!(
                        !matches!(
                            &verdict,
                            Err(errs) if errs.iter().any(|e| matches!(
                                e,
                                ValidationError::DescriptionNotAdjacent { .. }
                            ))
                        ),
                        "{ctx}: tangent split produced non-adjacent descriptions: {verdict:?}"
                    );
                }
            }
        }
        Err(e) => {
            // Typed refusal is the documented outcome; assert it is a
            // refusal, not a panic (reaching here at all is the pass).
            let _ = e;
        }
    }
}

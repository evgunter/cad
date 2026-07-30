//! M5 PR 5 acceptance shape (i): the tilted-plane×cylinder cut end to
//! end — an extruded disc split by a tilted plane, with the exact
//! `Ellipse` carrier on the minted section edges
//! (zero-residual-by-construction), plus the rim (perpendicular) and
//! ruling (axis-parallel) cut lanes and the determinism row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::EdgeGeometry;
use geom_core::{Point2, Point3, Tolerance, Vec3};
use geom_curves::Curve3;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{Body, validate, validate_closed, validate_geometric};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The disc profile: two half-circle arcs (bulge 1), radius 0.5 about
/// the sketch origin — extrudes to a cylinder of height 1 whose two
/// wall faces share ONE cylinder surface (the cosurface run).
fn disc() -> ValidatedProfile<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(-0.5, 0.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: p2(0.5, 0.0),
            bulge: 1.0,
        },
    ]);
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap()
}

fn cylinder_body() -> Body<f64> {
    extrude(&disc(), Extrusion::Distance(1.0)).unwrap().body
}

/// Every certified `Ellipse` edge of a body, with its curve.
fn ellipse_edges(body: &Body<f64>) -> Vec<(topo::EdgeKey, geom_brep::EdgeCurve<f64>)> {
    let mut out = Vec::new();
    for (ek, edge) in body.edges() {
        if let Some(c) = body.get_curve_geom(edge.curve).and_then(|g| g.certified())
            && matches!(c.carrier(), Curve3::Ellipse { .. })
        {
            out.push((ek, c.clone()));
        }
    }
    out
}

/// Shape (i): the tilted cut succeeds end to end; both parts are
/// closed, tier-3 valid solids; the section edges carry the exact
/// `Ellipse` (a = r/cos φ, b = r) described as the wall×plane
/// `Intersection`, and the residual against BOTH surfaces is
/// rounding-scale (identically zero in ℝ — the D4 ¶2 claim; the
/// certificate's ε-bound is the certified form).
#[test]
fn tilted_cut_mints_exact_ellipse_carriers() {
    let body = cylinder_body();
    let phi = 0.3f64;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 0.5),
        normal: Vec3::new(phi.sin(), 0.0, phi.cos()),
    };
    let result = split(&body, &plane).unwrap();
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("both sides carry material");
    };
    for part in [above, below] {
        assert_eq!(validate(part), Ok(()));
        assert_eq!(validate_closed(part), Ok(()));
        // Tier 3: every certification/dihedral row passes; the ONE
        // residue is the volume row's NotIsoRectangle on the cut wall
        // pieces (the split.rs review-F2 posture, documented: props'
        // curved quadrature lane is M5 PR 11's) — pinned exactly so
        // nothing else hides behind it.
        let errs = validate_geometric(part).unwrap_err();
        assert_eq!(errs.len(), 1, "{errs:?}");
        let msg = format!("{:?}", errs[0]);
        assert!(
            msg.contains("VolumeUncomputable") && msg.contains("NotIsoRectangle"),
            "{msg}"
        );
        // The section boundary: exactly two ellipse arcs (one per wall
        // piece), each the wall×plane Intersection.
        let ellipses = ellipse_edges(part);
        assert_eq!(ellipses.len(), 2, "two ellipse arcs bound the section");
        for (_, curve) in &ellipses {
            let Curve3::Ellipse { major, minor, .. } = *curve.carrier() else {
                panic!("ellipse filter guarantees the variant");
            };
            assert!((minor - 0.5).abs() < 1e-12, "b = r");
            assert!((major - 0.5 / phi.cos()).abs() < 1e-12, "a = r/cos φ");
            assert!(
                matches!(curve.description(), EdgeGeometry::Intersection { .. }),
                "section edges are described intersections"
            );
            // Zero-residual-by-construction: the certificate's worst
            // distance residual is rounding-scale, orders below ε.
            assert!(
                curve.certificate().max_residual < 1e-12,
                "residual {:e}",
                curve.certificate().max_residual
            );
        }
    }
    // The two parts' ellipse arcs cover the full period between them:
    // each part carries ~half the ellipse per wall piece — spans sum
    // to 2π per side pair (the two arcs of one full section ellipse).
    let span_sum: f64 = ellipse_edges(above)
        .iter()
        .map(|(_, c)| {
            let (t0, t1) = c.params();
            t1 - t0
        })
        .sum();
    assert!(
        (span_sum - core::f64::consts::TAU).abs() < 1e-9,
        "above section arcs span one full period, got {span_sum}"
    );
}

/// The rim lane: a perpendicular mid-height cut stays rung 1 — the
/// section edges carry `Circle` carriers on the cylinder axis.
#[test]
fn perpendicular_cut_stays_rung_1_circles() {
    let body = cylinder_body();
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 0.5),
        normal: Vec3::unit_z(),
    };
    let result = split(&body, &plane).unwrap();
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("both sides carry material");
    };
    for part in [above, below] {
        assert_eq!(validate_closed(part), Ok(()));
        assert_eq!(validate_geometric(part), Ok(()));
        assert!(ellipse_edges(part).is_empty(), "no ellipse minted");
        // The section boundary circles sit at z = 0.5 with radius 0.5.
        let mut section_circles = 0;
        for (_, edge) in part.edges() {
            let Some(c) = part.get_curve_geom(edge.curve).and_then(|g| g.certified()) else {
                continue;
            };
            if let Curve3::Circle { center, radius, .. } = *c.carrier()
                && (center.z - 0.5).abs() < 1e-12
            {
                assert!((radius - 0.5).abs() < 1e-12);
                section_circles += 1;
            }
        }
        assert_eq!(section_circles, 2, "two rim arcs bound the section");
    }
}

/// The ruling lane: an axis-parallel cut (x = 0, clear of the profile
/// seams) crosses the four rim arcs (the conic crossing-root lane on
/// circle carriers) and sections the walls along rulings — straight
/// chords, no conic carrier minted.
#[test]
fn axis_parallel_cut_splits_through_rim_crossings() {
    let body = cylinder_body();
    let plane = SplitPlane {
        origin: Point3::origin(),
        normal: Vec3::unit_x(),
    };
    let result = split(&body, &plane).unwrap();
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("both sides carry material");
    };
    for part in [above, below] {
        assert_eq!(validate_closed(part), Ok(()));
        assert_eq!(validate_geometric(part), Ok(()));
        assert!(ellipse_edges(part).is_empty(), "ruling sections are lines");
    }
}

/// A cut whose plane CONTAINS the seam rulings (y = 0 through both
/// profile corners): every operand vertex is ON and the rim arcs
/// leave their ON endpoints with in-plane chords-of-record — a
/// configuration the M5 PR 5 lane refuses TYPED (the curved ON-edge
/// chain is PR 9's second-order sector lane), never builds garbage.
#[test]
fn seam_coincident_cut_refuses_typed() {
    let body = cylinder_body();
    let plane = SplitPlane {
        origin: Point3::origin(),
        normal: Vec3::unit_y(),
    };
    let err = split(&body, &plane).unwrap_err();
    // The refusal class is the reduction's typed surface — pinned
    // loosely (the exact door may sharpen at PR 9, the refusing never).
    let msg = format!("{err}");
    assert!(
        msg.contains("split"),
        "typed reduction refusal expected, got: {msg}"
    );
}

/// Determinism (D9): identical histories mint bit-identical section
/// geometry — the ellipse fields and intervals replay byte-equal.
#[test]
fn tilted_cut_replays_bit_identically() {
    let run = || {
        let body = cylinder_body();
        let phi = 0.3f64;
        let plane = SplitPlane {
            origin: Point3::new(0.0, 0.0, 0.5),
            normal: Vec3::new(phi.sin(), 0.0, phi.cos()),
        };
        let result = split(&body, &plane).unwrap();
        let SplitPart::Body(above) = result.above else {
            panic!("above side carries material");
        };
        let mut rows: Vec<String> = ellipse_edges(&above)
            .iter()
            .map(|(_, c)| format!("{:?} {:?}", c.carrier(), c.params()))
            .collect();
        rows.sort();
        rows
    };
    assert_eq!(run(), run());
}

/// The tangent lane refuses typed (C7): a plane grazing the wall at
/// exactly one ruling is a tangency, never marched into.
#[test]
fn tangent_plane_refuses_typed() {
    let body = cylinder_body();
    // x = 0.5 exactly touches the wall along the (0.5, 0, z) ruling…
    // but that ruling IS the seam through the profile start vertex, so
    // the vertex sweep classifies the seam endpoints ON and the sector
    // machinery refuses on the tangent contact (rule (a)'s curved
    // lane) or the join's tangency door — either way typed, never a
    // marched tangency.
    let plane = SplitPlane {
        origin: Point3::new(0.5, 0.0, 0.0),
        normal: Vec3::unit_x(),
    };
    let err = split(&body, &plane).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("tangent") || msg.contains("Tangen") || msg.contains("degenerate"),
        "tangency-class refusal expected, got: {msg}"
    );
}

/// The interval lane: the tilted cut replays at `T = Interval` and the
/// section ellipses' residual enclosures contain zero against both
/// surfaces (the exact-in-ℝ claim, certified).
#[cfg(feature = "interval")]
mod interval {
    use super::*;
    use geom_core::{Bounds, Interval, Real};

    #[test]
    fn tilted_cut_at_interval_encloses_zero_residuals() {
        let iv = <Interval as Real>::from_f64;
        let lp = ProfileLoop::new(vec![
            ProfileVertex {
                pos: Point2::new(iv(-0.5), iv(0.0)),
                bulge: iv(1.0),
            },
            ProfileVertex {
                pos: Point2::new(iv(0.5), iv(0.0)),
                bulge: iv(1.0),
            },
        ]);
        let vp = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
            .validate(Tolerance::get())
            .unwrap();
        let body = extrude(&vp, Extrusion::Distance(iv(1.0))).unwrap().body;
        let phi = 0.3f64;
        let plane = SplitPlane {
            origin: Point3::new(iv(0.0), iv(0.0), iv(0.5)),
            normal: Vec3::new(iv(phi.sin()), iv(0.0), iv(phi.cos())),
        };
        let result = split(&body, &plane).unwrap();
        let SplitPart::Body(above) = result.above else {
            panic!("above side carries material");
        };
        assert_eq!(validate_closed(&above), Ok(()));
        let mut count = 0;
        for (_, edge) in above.edges() {
            let Some(c) = above.get_curve_geom(edge.curve).and_then(|g| g.certified()) else {
                continue;
            };
            if !matches!(c.carrier(), Curve3::Ellipse { .. }) {
                continue;
            }
            count += 1;
            let EdgeGeometry::Intersection { s1, s2, .. } = *c.description() else {
                panic!("section edge described as Intersection");
            };
            let (t0, t1) = c.params();
            for i in 0..=8u32 {
                let t = t0 + (t1 - t0) * iv(f64::from(i) / 8.0);
                let p = c.carrier().eval(t);
                for key in [s1, s2] {
                    let s = above.get_surface(key).unwrap();
                    let r = geom_brep::implicit_residual(s, p);
                    assert!(
                        r.lo() <= 0.0 && 0.0 <= r.hi(),
                        "residual enclosure [{}, {}]",
                        r.lo(),
                        r.hi()
                    );
                }
            }
        }
        assert_eq!(count, 2);
    }
}

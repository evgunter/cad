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
/// profile corners): every operand vertex is ON and the seams are
/// in-plane ON edges. Since the M1 fix pass, conic orbit entries
/// classify by their DEPARTURE tangent (definite here — the rim arcs
/// leave the plane transversally), so the split succeeds along the
/// existing seams: two half-cylinder bodies, no conic section edges
/// (the section rectangles are seam rulings + cap chords).
#[test]
fn seam_coincident_cut_splits_along_the_seams() {
    let body = cylinder_body();
    let plane = SplitPlane {
        origin: Point3::origin(),
        normal: Vec3::unit_y(),
    };
    let result = split(&body, &plane).unwrap();
    let (above, below) = assert_two_sided(&result);
    for part in [&above, &below] {
        assert_eq!(part.faces().count(), 4, "wall + two half caps + section");
        assert!(ellipse_edges(part).is_empty());
    }
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

// ---------------------------------------------------------------------
// M1 fix rows (adversarial review): even-crossing completeness
// ---------------------------------------------------------------------

/// Shared pins for a both-sides split: closed + tier-3 clean except
/// the (exactly pinned) curved-wall volume residue when the cut
/// touches the walls off-rim.
fn assert_two_sided(result: &topo::splitting::SplitResult<f64>) -> (Body<f64>, Body<f64>) {
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("both sides must carry material");
    };
    for part in [above, below] {
        assert_eq!(validate(part), Ok(()));
        assert_eq!(validate_closed(part), Ok(()));
        if let Err(errs) = validate_geometric(part) {
            for e in &errs {
                let msg = format!("{e:?}");
                assert!(
                    msg.contains("VolumeUncomputable") && msg.contains("NotIsoRectangle"),
                    "only the pinned props residue may remain: {msg}"
                );
            }
        }
    }
    (above.clone(), below.clone())
}

/// THE reviewer probe (review_pr5_probe::even_crossing, adopted): the
/// plane y = 0.25 crosses the upper rim arcs TWICE between same-side
/// endpoints. Before the fix the sliver 0.25 < y ≤ 0.5 was silently
/// LOST (above = Empty, below = the whole body); the root-based
/// crossing lane now recovers it — a genuine two-sided split.
#[test]
fn even_crossing_recovers_the_sliver() {
    let body = cylinder_body();
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.25, 0.0),
        normal: Vec3::unit_y(),
    };
    let result = split(&body, &plane).unwrap();
    let (above, below) = assert_two_sided(&result);
    // The above body IS the recovered sliver: every vertex at
    // y ≥ 0.25 − ε, at least one strictly beyond (the rim apex band).
    let mut max_y = f64::NEG_INFINITY;
    for (_, v) in above.vertices() {
        let p = above.get_point(v.point).unwrap();
        assert!(p.y >= 0.25 - 1e-9, "sliver vertex below the plane: {p:?}");
        max_y = max_y.max(p.y);
    }
    assert!(
        (max_y - 0.25).abs() < 1e-9,
        "crossing vertices sit on the plane"
    );
    // The rim apex band survives as the belly ARCS' interiors: some
    // above-side edge's carrier midpoint reaches y ≈ 0.5.
    let mut apex = f64::NEG_INFINITY;
    for (_, edge) in above.edges() {
        if let Some(c) = above.get_curve_geom(edge.curve).and_then(|g| g.certified()) {
            let (t0, t1) = c.params();
            apex = apex.max(c.carrier().eval(t0 + (t1 - t0) * 0.5).y);
        }
    }
    assert!(
        (apex - 0.5).abs() < 1e-9,
        "the belly arc reaches the rim apex"
    );
    // The below body keeps the far side: its lower rim arcs' interiors
    // reach y ≈ −0.5 (the extreme lives mid-arc, not at a vertex).
    let mut far = f64::INFINITY;
    for (_, edge) in below.edges() {
        if let Some(c) = below.get_curve_geom(edge.curve).and_then(|g| g.certified()) {
            let (t0, t1) = c.params();
            far = far.min(c.carrier().eval(t0 + (t1 - t0) * 0.5).y);
        }
    }
    assert!((far + 0.5).abs() < 1e-9, "the main body keeps the far side");
    // The sections are ruling/chord rectangles — no conic carrier
    // minted (plane ∥ axis), and the belly arcs stay real cap/wall
    // boundary on the ABOVE side.
    assert!(ellipse_edges(&above).is_empty());
    assert!(ellipse_edges(&below).is_empty());
}

/// The tilted belly (both rims crossed twice, seams crossed once): a
/// full two-sided split whose section is the mixed hexagon — four
/// exact ellipse arcs (walls) + two cap chords — and every section
/// edge lies in the plane.
#[test]
fn tilted_belly_cut_splits_with_mixed_section() {
    let body = cylinder_body();
    let n = 1.0 / 5.0f64.sqrt();
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.1, 0.5),
        normal: Vec3::new(0.0, 2.0 * n, n),
    };
    let result = split(&body, &plane).unwrap();
    let (above, below) = assert_two_sided(&result);
    for part in [&above, &below] {
        let ellipses = ellipse_edges(part);
        assert_eq!(ellipses.len(), 4, "four wall arcs bound the section");
        for (_, c) in &ellipses {
            assert!(
                matches!(c.description(), EdgeGeometry::Intersection { .. }),
                "section arcs are described intersections"
            );
            assert!(c.certificate().max_residual < 1e-12);
            // Every sampled point lies in the split plane.
            let (t0, t1) = c.params();
            for i in 0..=8 {
                let t = t0 + (t1 - t0) * f64::from(i) / 8.0;
                let p = c.carrier().eval(t);
                let d = (p - plane.origin).dot(plane.normal);
                assert!(d.abs() < 1e-12, "section arc off-plane: {d:e}");
            }
        }
    }
}

/// The ON-endpoint belly variant (audit row): the plane passes through
/// one bottom corner and the arcs leave it with one MORE interior
/// crossing each — the root lane inserts the interior crossings, the
/// endpoint root lands on the existing ON vertex, and the split is
/// two-sided.
#[test]
fn on_endpoint_belly_cut_splits() {
    let body = cylinder_body();
    let n = Vec3::new(0.3, 1.0, 0.2).normalize();
    let plane = SplitPlane {
        origin: Point3::new(0.5, 0.0, 0.0),
        normal: n,
    };
    let result = split(&body, &plane).unwrap();
    let (above, below) = assert_two_sided(&result);
    // The mixed section carries ellipse arcs on both sides.
    assert!(!ellipse_edges(&above).is_empty());
    assert!(!ellipse_edges(&below).is_empty());
}

/// Even-count crossings at `T = Interval` (both lanes, per the fix
/// order). The belly class currently ESCALATES TYPED at the interval
/// scalar inside the join's arc-side selector: the azimuth-difference
/// enclosures of coincident-copy chords straddle a period boundary,
/// and `reduce_periodic`'s containment-honest floor widens them to a
/// full period — `split_conic_arc_side` then refuses (the Q1
/// subdivision posture: a value-channel refusal, never wrong
/// geometry; the f64 lane splits the same document correctly, and
/// the non-belly tilted cut replays two-sided at Interval in
/// `interval::tilted_cut_at_interval_encloses_zero_residuals`).
/// Wrap-free angular enclosures are C9-hull territory (PR 6/7) — the
/// row pins today's typed posture so any sharpening is a deliberate
/// repin.
#[cfg(feature = "interval")]
#[test]
fn even_crossing_belly_cut_at_interval() {
    use geom_core::{Interval, Real};
    let iv = <Interval as Real>::from_f64;
    let lp = profile::ProfileLoop::new(vec![
        profile::ProfileVertex {
            pos: Point2::new(iv(-0.5), iv(0.0)),
            bulge: iv(1.0),
        },
        profile::ProfileVertex {
            pos: Point2::new(iv(0.5), iv(0.0)),
            bulge: iv(1.0),
        },
    ]);
    let vp = profile::Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    let body = extrude(&vp, Extrusion::Distance(iv(1.0))).unwrap().body;
    // The tilted-belly even-crossing configuration (both rims crossed
    // twice + both seams once). Axis-parallel even-crossing planes put
    // crossing-vertex PAIRS at equal in-plane u (vertically aligned),
    // and at Interval the join's exact-band order comparator then
    // escalates typed (`split_join_order_u` — the honest
    // subdivision-refusal posture for enclosure-width ties), so the
    // interval row uses the tilted variant, the same belly class with
    // all six section vertices order-distinct.
    let nv = Vec3::new(0.3, 2.0, 1.0).normalize();
    let plane = SplitPlane {
        origin: Point3::new(iv(0.03), iv(0.11), iv(0.47)),
        normal: Vec3::new(iv(nv.x), iv(nv.y), iv(nv.z)),
    };
    let err = split(&body, &plane).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("split_conic_arc_side"),
        "the pinned interval posture is the arc-side escalation, got: {msg}"
    );
}

// ---------------------------------------------------------------------
// M3 fix rows: end-to-end trilean arms of the conic crossing lane
// ---------------------------------------------------------------------

/// `split_conic_crossing_root` Zero arm, end to end: the y = 0 plane's
/// rim roots land EXACTLY on the profile corners — nothing is
/// inserted (the vertex sweep owns them), and the ON set is exactly
/// the four original corners.
#[test]
fn root_at_endpoint_inserts_nothing() {
    let body = cylinder_body();
    let originals: std::collections::BTreeSet<_> = body.vertices().map(|(k, _)| k).collect();
    let plane = SplitPlane {
        origin: Point3::origin(),
        normal: Vec3::unit_y(),
    };
    let red = topo::splitting::split_reduce(&body, &plane).unwrap();
    assert_eq!(red.on_vertices.len(), 4);
    for v in &red.on_vertices {
        assert!(originals.contains(v), "no crossing vertex minted");
    }
}

/// The definite arm, end to end: the reviewer probe's plane mints
/// exactly four NEW crossing vertices (two per doubly-crossed rim).
#[test]
fn definite_roots_mint_four_crossings() {
    let body = cylinder_body();
    let originals: std::collections::BTreeSet<_> = body.vertices().map(|(k, _)| k).collect();
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.25, 0.0),
        normal: Vec3::unit_y(),
    };
    let red = topo::splitting::split_reduce(&body, &plane).unwrap();
    assert_eq!(red.on_vertices.len(), 4);
    for v in &red.on_vertices {
        assert!(!originals.contains(v), "all four are minted crossings");
    }
}

/// `split_conic_belly_graze` in-band, end to end: a plane 3ε shy of
/// the rim apex is an ill-conditioned graze — CrossingEscalated, with
/// the predicate named and the shared recourse riding the payload.
#[test]
fn near_graze_escalates_typed() {
    let body = cylinder_body();
    let eps = Tolerance::get().eps;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.5 + 3.0 * eps, 0.0),
        normal: Vec3::unit_y(),
    };
    let err = split(&body, &plane).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("split_conic_belly_graze"), "{msg}");
    assert_eq!(msg.matches(geom_core::COINCIDENCE_RECOURSE).count(), 1);
}

/// The exact graze (plane through the rim apexes): the double root
/// inserts a single ON contact per rim and the pipeline resolves the
/// one-sided tangency through its established net — a typed refusal,
/// never a degenerate body.
#[test]
fn exact_graze_refuses_typed() {
    let body = cylinder_body();
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.5, 0.0),
        normal: Vec3::unit_y(),
    };
    match split(&body, &plane) {
        Ok(r) => panic!(
            "a tangent graze must not produce a two-sided split: above={:?} below={:?}",
            matches!(r.above, SplitPart::Body(_)),
            matches!(r.below, SplitPart::Body(_))
        ),
        Err(e) => {
            let msg = format!("{e}");
            assert!(msg.contains("split"), "typed refusal expected: {msg}");
        }
    }
}

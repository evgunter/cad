//! M5 PR 5 acceptance shape (i): the tilted-plane×cylinder cut end to
//! end — an extruded disc split by a tilted plane, with the exact
//! `Ellipse` carrier on the minted section edges
//! (zero-residual-by-construction), plus the rim (perpendicular) and
//! ruling (axis-parallel) cut lanes and the determinism row.
//!
//! The **belly rows** at the end of the M1 fix block are the arc-side
//! rule's end-to-end acceptance: since M5 S9 the rule is azimuth-window
//! containment, and those rows pin what it selects (the short arcs, on
//! the finite wall), its seam-placement independence, and the PR 6
//! pcurve certification closing the loop on the repaired bodies.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom_brep::EdgeDescription;
use geom_core::Tol;
use geom_core::{Point2, Point3, Vec3};
use profile::RawLoop;
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
        ProfileVertex::new(p2(-0.5, 0.0), 1.0),
        ProfileVertex::new(p2(0.5, 0.0), 1.0),
    ]);
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap()
}

fn cylinder_body() -> Body<f64> {
    extrude(&disc(), Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
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
    let result = split(&body, &plane, Tol::witness()).unwrap();
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("both sides carry material");
    };
    for part in [above, below] {
        assert_eq!(validate(part), Ok(()));
        assert_eq!(validate_closed(part), Ok(()));
        // Tier 3 in FULL (M5 PR 11): the volume row's staged
        // NotIsoRectangle residue retired when the certified
        // quadrature lane landed — check 7 now consumes the
        // quadrature enclosure's bounds (construction row, the S9
        // flip pattern; the enclosure itself is pinned in
        // `m5_pr11_quad_props.rs`).
        if let Err(errs) = validate_geometric(part, Tol::witness()) {
            panic!("tier 3 passes end to end since PR 11: {errs:?}");
        }
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
                matches!(curve.description(), EdgeDescription::Intersection { .. }),
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
    let result = split(&body, &plane, Tol::witness()).unwrap();
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("both sides carry material");
    };
    for part in [above, below] {
        assert_eq!(validate_closed(part), Ok(()));
        assert_eq!(validate_geometric(part, Tol::witness()), Ok(()));
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
    let result = split(&body, &plane, Tol::witness()).unwrap();
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("both sides carry material");
    };
    for part in [above, below] {
        assert_eq!(validate_closed(part), Ok(()));
        assert_eq!(validate_geometric(part, Tol::witness()), Ok(()));
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
    let result = split(&body, &plane, Tol::witness()).unwrap();
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
        let result = split(&body, &plane, Tol::witness()).unwrap();
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
    let err = split(&body, &plane, Tol::witness()).unwrap_err();
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
    use geom_core::Tol;
    use geom_core::{Bounds, Interval, Real};

    #[test]
    fn tilted_cut_at_interval_encloses_zero_residuals() {
        let iv = <Interval as Real>::from_f64;
        let lp = ProfileLoop::new(vec![
            ProfileVertex::new(Point2::new(iv(-0.5), iv(0.0)), iv(1.0)),
            ProfileVertex::new(Point2::new(iv(0.5), iv(0.0)), iv(1.0)),
        ]);
        let vp = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
            .validate(Tol::witness())
            .unwrap();
        let body = extrude(&vp, Extrusion::Distance(iv(1.0)), Tol::witness())
            .unwrap()
            .body;
        let phi = 0.3f64;
        let plane = SplitPlane {
            origin: Point3::new(iv(0.0), iv(0.0), iv(0.5)),
            normal: Vec3::new(iv(phi.sin()), iv(0.0), iv(phi.cos())),
        };
        let result = split(&body, &plane, Tol::witness()).unwrap();
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
            let EdgeDescription::Intersection { s1, s2, .. } = *c.description() else {
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

/// Shared pins for a both-sides split: closed + tier-3 clean IN FULL
/// (M5 PR 11 retired the curved-wall volume residue this helper used
/// to exempt — the quadrature lane computes those faces now).
fn assert_two_sided(result: &topo::splitting::SplitResult<f64>) -> (Body<f64>, Body<f64>) {
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("both sides must carry material");
    };
    for part in [above, below] {
        assert_eq!(validate(part), Ok(()));
        assert_eq!(validate_closed(part), Ok(()));
        if let Err(errs) = validate_geometric(part, Tol::witness()) {
            panic!("tier 3 passes end to end since PR 11: {errs:?}");
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
    let result = split(&body, &plane, Tol::witness()).unwrap();
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

/// Every certified ellipse arc's carrier stays on the FINITE wall over
/// its own stored interval: the domain-validity statement the arc-side
/// defect violated. Returns the (sorted) spans so callers can pin them.
fn wall_contained_ellipse_spans(part: &Body<f64>, height: f64) -> Vec<f64> {
    let mut spans = Vec::new();
    for (_, curve) in ellipse_edges(part) {
        let (t0, t1) = curve.params();
        for i in 0..=64u32 {
            let t = t0 + (t1 - t0) * f64::from(i) / 64.0;
            let z = curve.carrier().eval(t).z;
            assert!(
                (-1e-12..=height + 1e-12).contains(&z),
                "section arc leaves the finite wall: z = {z} at t = {t} \
                 over [{t0}, {t1}]"
            );
        }
        spans.push(t1 - t0);
    }
    spans.sort_by(|a, b| a.partial_cmp(b).expect("finite spans"));
    spans
}

/// The tilted belly (both rims crossed twice, seams crossed once) —
/// **the certified-pass row since M5 S9**, and the row that pins the
/// repaired arc-side rule end to end.
///
/// PR 5's arc-side rule (`topo::chord_join`'s `chord_spec`,
/// `split_conic_arc_side`) picked which of the section conic's two arcs
/// bounds the divided face from a single azimuth sample taken on the
/// RUN the chord co-bounds, premised on that sample's azimuth "lying
/// strictly inside the azimuth interval the chord must span". On this
/// configuration the premise is FALSE: the above-side wall piece is
/// bounded by two short ellipse arcs (≈17.5° and ≈44.4°) AND by a 91°
/// bottom-rim arc whose midpoint the rule sampled — so the sample
/// landed outside the chord's own interval and the rule selected the
/// COMPLEMENT arc (≈342.5°, ≈315.6°) every time.
///
/// S9 replaced the premise with **containment in the divided face's own
/// azimuth window** (`split_arc_window`), derived from the run through
/// the same closed-form chart machinery PR 6 certifies pcurves with.
/// This row now asserts what the repair produces: a two-sided split
/// whose eight section arcs are the SHORT ones, each staying on the
/// finite wall over its whole stored interval.
///
/// ---- The defect's evidence, kept as a history note ----
///
/// It cannot be asserted at HEAD (the wrong bodies are no longer
/// reachable). The measurement was taken at the PR 6 merge base, twice
/// and independently — once during implementation (by making the mint
/// pass non-fatal for one run) and once by the adversarial review (its
/// own probe against an unmodified merge-base checkout), with agreeing
/// numbers:
///
///   * each of the EIGHT section-arc edges across the two sides stored
///     a parameter interval of τ − 0.305 or τ − 0.775 rad — bitwise the
///     complement of the correct ≈17.5° / ≈44.4° arc this row now pins;
///   * those carriers swept z ∈ [−0.297, 1.689] on a wall of height 1,
///     i.e. they left the finite wall on both ends;
///   * `validate_geometric` on those bodies returned exactly one error,
///     `VolumeUncomputable{NotIsoRectangle}` — the same one the CORRECT
///     simple tilted cut returned AT THAT TIME (pre-PR-11, when the
///     volume row was staged), so tier 3 was blind to the defect
///     rather than merely quiet about it.
///
/// Between PR 6 and S9 the configuration REFUSED typed instead
/// (`LoopNotClosed`: PR 6's per-face one-branch chart walk measured the
/// face's azimuth advance as −2τ), which is the posture that row
/// pinned. Both are now history: the split succeeds with the right
/// arcs.
#[test]
fn tilted_belly_cut_mints_wall_contained_section_arcs() {
    let body = cylinder_body();
    let n = 1.0 / 5.0f64.sqrt();
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.1, 0.5),
        normal: Vec3::new(0.0, 2.0 * n, n),
    };
    let result = split(&body, &plane, Tol::witness()).unwrap();
    let (above, below) = assert_two_sided(&result);
    for part in [&above, &below] {
        // Four section arcs per side: the two short ones per wall
        // piece, NOT their ≈342.5°/≈315.6° complements.
        let spans = wall_contained_ellipse_spans(part, 1.0);
        assert_eq!(spans.len(), 4, "{spans:?}");
        for (got, want) in spans.iter().zip([0.304693, 0.304693, 0.775397, 0.775397]) {
            assert!((got - want).abs() < 1e-6, "{spans:?}");
        }
        // The two wall pieces' arcs sum to the section's total sweep on
        // this side — the same total on both sides (one section).
        let sum: f64 = spans.iter().sum();
        assert!((sum - 2.160180).abs() < 1e-6, "{sum}");
    }
    // Determinism (D9): the repaired selection replays bit-identically.
    let rows = |part: &Body<f64>| {
        let mut rows: Vec<String> = ellipse_edges(part)
            .iter()
            .map(|(_, c)| format!("{:?} {:?}", c.carrier(), c.params()))
            .collect();
        rows.sort();
        rows
    };
    let again = split(&body, &plane, Tol::witness()).unwrap();
    let (above2, below2) = assert_two_sided(&again);
    assert_eq!(rows(&above), rows(&above2));
    assert_eq!(rows(&below), rows(&below2));
}

/// Seam-placement independence of the window computation (S9): the
/// belly cut, rotated about the cylinder axis by 0.7 rad. The chart's
/// `u_ref` does NOT rotate with it, so the run's azimuth window, the
/// chord endpoints and the section arcs all land on a different part of
/// the chart — including across the chart seam. The window rule
/// compares only differences of azimuths, so the answer must not care:
/// both sides split, every arc stays on the wall, and the section's
/// TOTAL sweep is the rotation-invariant it must be (the individual
/// arcs differ because the profile seams cut the section elsewhere).
#[test]
fn rotated_belly_cut_is_seam_placement_independent() {
    let body = cylinder_body();
    let a = 0.7f64;
    let (s, c) = (a.sin(), a.cos());
    let n0 = Vec3::new(0.0, 2.0, 1.0).normalize();
    let o0 = Point3::new(0.0, 0.1, 0.5);
    let plane = SplitPlane {
        origin: Point3::new(c * o0.x - s * o0.y, s * o0.x + c * o0.y, o0.z),
        normal: Vec3::new(c * n0.x - s * n0.y, s * n0.x + c * n0.y, n0.z),
    };
    let result = split(&body, &plane, Tol::witness()).unwrap();
    let (above, below) = assert_two_sided(&result);
    for part in [&above, &below] {
        let spans = wall_contained_ellipse_spans(part, 1.0);
        assert_eq!(spans.len(), 3, "{spans:?}");
        let sum: f64 = spans.iter().sum();
        assert!(
            (sum - 2.160180).abs() < 1e-6,
            "the section's total sweep is rotation-invariant, got {sum}"
        );
    }
}

/// The ON-endpoint belly variant: the plane passes through one bottom
/// corner and the arcs leave it with one MORE interior crossing each —
/// the root lane inserts the interior crossings, the endpoint root
/// lands on the existing ON vertex, and the split is two-sided.
///
/// **This row is the second member of the belly class, and S9 repaired
/// it too.** It has passed since PR 5, but only because it asserted
/// two-sidedness and the presence of ellipse arcs — never their extent.
/// At the merge base its four section arcs each stored a span of
/// 5.895799 rad (τ − 0.387386, the complement) sweeping
/// z ∈ [−1.860, 3.360] on a wall of height 1, and — unlike the tilted
/// belly cut — PR 6's pcurve mint pass ACCEPTED those bodies (the
/// wrong-arc loops still closed on the chart). So this configuration
/// was shipping a wrong body silently, with nothing in the kernel able
/// to see it. The window rule selects 0.387386 rad here, on the wall;
/// the extent assertions below are what makes that visible.
///
/// ---- The merge-base measurement, as a history note (probe F2) ----
///
/// It cannot be asserted at HEAD — the wrong bodies are unreachable —
/// so the probe is recorded rather than committed as a row. It was run
/// twice on unmodified merge-base checkouts (implementation and
/// adversarial review, independently) with agreeing numbers. To re-run
/// it: check out the PR 6 merge base, drop a test into
/// `crates/sweep/tests/` that builds this exact body and plane, splits,
/// and for each side prints every certified `Curve3::Ellipse` edge's
/// `params()` span, the min/max `z` of `carrier().eval` over a dense
/// sweep of that interval, `topo::pcurves::validate_pcurves(part,
/// Band::linear(Tol::witness()))`, and `topo::mint_pcurves(&mut part.clone())`.
/// It reports, per side:
///
///   * two section-arc edges, each span 5.8957988 rad
///     (= τ − 0.387386, bitwise the complement of the arc pinned
///     below);
///   * those carriers sweeping z ∈ [−1.860, 3.360] on a wall of
///     height 1 — off the wall by nearly two heights at each end;
///   * `validate_pcurves` returning **zero** findings and the fresh
///     re-mint returning `Ok` — the PR 6 machinery that caught the
///     tilted belly cut does not catch this one.
///
/// That last line is the point of keeping the note: the pcurve
/// loop-closure limb is not a general detector of wrong-arc selection,
/// it happened to fire on one member of the class. Containment at
/// selection time is what actually covers it.
#[test]
fn on_endpoint_belly_cut_splits() {
    let body = cylinder_body();
    let n = Vec3::new(0.3, 1.0, 0.2).normalize();
    let plane = SplitPlane {
        origin: Point3::new(0.5, 0.0, 0.0),
        normal: n,
    };
    let result = split(&body, &plane, Tol::witness()).unwrap();
    let (above, below) = assert_two_sided(&result);
    // The mixed section carries ellipse arcs on both sides, and every
    // one of them stays on the finite wall over its stored interval.
    for part in [&above, &below] {
        let spans = wall_contained_ellipse_spans(part, 1.0);
        assert_eq!(spans.len(), 2, "{spans:?}");
        for got in &spans {
            assert!(
                (got - 0.387386).abs() < 1e-6,
                "the short arc, not its τ − 0.387386 complement: {spans:?}"
            );
        }
    }
}

/// M5 PR 6 acceptance on the repaired bodies (S9 §2): the belly-cut
/// sides now mint and certify pcurve caches — the very machinery whose
/// mint pass found the defect closes the loop on the repair. (The split
/// pipeline already runs `mint_pcurves` on each side it produces, so
/// reaching a `SplitPart::Body` at all is one certification; this row
/// re-mints from scratch and re-validates at rest, so the caches are
/// pinned as *re-derivable* and *re-certifiable*, not merely as
/// once-accepted.)
#[test]
fn repaired_belly_bodies_mint_certified_pcurves() {
    let body = cylinder_body();
    let n = 1.0 / 5.0f64.sqrt();
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.1, 0.5),
        normal: Vec3::new(0.0, 2.0 * n, n),
    };
    let result = split(&body, &plane, Tol::witness()).unwrap();
    let (above, below) = assert_two_sided(&result);
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    for part in [above, below] {
        // The split's own caches re-validate at rest.
        assert!(topo::pcurves::validate_pcurves(&part, band).is_empty());
        // And a fresh mint over the same body is accepted and clean.
        let mut again = part.clone();
        topo::mint_pcurves(&mut again, Tol::witness()).unwrap();
        assert!(topo::pcurves::validate_pcurves(&again, band).is_empty());
        // The curved wall pieces really do carry caches (the row would
        // be vacuous on an all-planar body).
        assert!(
            again.half_edges().any(|(he, _)| again.pcurve(he).is_some()),
            "the repaired belly sides carry pcurve caches"
        );
    }
}

/// Even-count crossings at `T = Interval` (the belly row's second
/// lane). Before S9 this configuration ESCALATED TYPED inside the
/// arc-side selector: that rule reduced azimuth differences taken *at*
/// the coincident-copy chord endpoints, whose enclosures straddle a
/// period boundary, and `reduce_periodic`'s containment-honest floor
/// widened them to a full period. The window rule reduces against the
/// window's CENTRE instead — half a window away from the boundary by
/// construction — so the enclosures stay narrow and the interval lane
/// now splits the belly document two-sided, with every section arc on
/// the finite wall.
#[cfg(feature = "interval")]
#[test]
fn even_crossing_belly_cut_at_interval() {
    use geom_core::{Bounds, Interval, Real};
    let iv = <Interval as Real>::from_f64;
    let lp = profile::ProfileLoop::new(vec![
        profile::ProfileVertex::new(Point2::new(iv(-0.5), iv(0.0)), iv(1.0)),
        profile::ProfileVertex::new(Point2::new(iv(0.5), iv(0.0)), iv(1.0)),
    ]);
    let vp = profile::Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let body = extrude(&vp, Extrusion::Distance(iv(1.0)), Tol::witness())
        .unwrap()
        .body;
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
    // The same §2 assertions the f64 belly row carries, at Interval:
    // wall containment, the spans summing per part, and bit-identical
    // replay.
    let section_rows = |part: &Body<Interval>| -> (Vec<String>, Interval) {
        let mut rows = Vec::new();
        let mut sum = iv(0.0);
        for (_, edge) in part.edges() {
            let Some(c) = part.get_curve_geom(edge.curve).and_then(|g| g.certified()) else {
                continue;
            };
            if !matches!(c.carrier(), Curve3::Ellipse { .. }) {
                continue;
            }
            let (t0, t1) = c.params();
            // Every section arc's carrier stays on the finite wall over
            // its stored interval — the enclosure form of the
            // domain-validity statement the defect violated.
            for i in 0..=16u32 {
                let t = t0 + (t1 - t0) * iv(f64::from(i) / 16.0);
                let z = c.carrier().eval(t).z;
                assert!(
                    z.lo() >= -1e-9 && z.hi() <= 1.0 + 1e-9,
                    "section arc leaves the finite wall: z in [{}, {}]",
                    z.lo(),
                    z.hi()
                );
            }
            sum = sum + (t1 - t0);
            rows.push(format!("{:?} {:?}", c.carrier(), c.params()));
        }
        rows.sort();
        (rows, sum)
    };
    let run = || {
        let result = split(&body, &plane, Tol::witness()).unwrap();
        let (SplitPart::Body(above), SplitPart::Body(below)) = (result.above, result.below) else {
            panic!("both sides carry material at Interval");
        };
        assert_eq!(validate_closed(&above), Ok(()));
        assert_eq!(validate_closed(&below), Ok(()));
        (section_rows(&above), section_rows(&below))
    };
    let ((above_rows, above_sum), (below_rows, below_sum)) = run();
    assert!(!above_rows.is_empty(), "the belly section carries arcs");
    assert!(!below_rows.is_empty(), "the belly section carries arcs");
    // One section, so the two sides' arcs sum to the same total sweep —
    // enclosures that overlap, the interval form of "spans summing per
    // part" (and both well under a full period: the belly cut's section
    // is cut short by the caps).
    assert!(
        above_sum.lo() <= below_sum.hi() && below_sum.lo() <= above_sum.hi(),
        "per-part span sums must agree: [{}, {}] vs [{}, {}]",
        above_sum.lo(),
        above_sum.hi(),
        below_sum.lo(),
        below_sum.hi()
    );
    assert!(
        above_sum.hi() < core::f64::consts::TAU,
        "a belly section sweeps less than a full period, got {}",
        above_sum.hi()
    );
    // Determinism (D9) at Interval: identical histories, identical bits.
    let ((above2, _), (below2, _)) = run();
    assert_eq!(above_rows, above2);
    assert_eq!(below_rows, below2);
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
    let red = topo::splitting::split_reduce(&body, &plane, Tol::witness()).unwrap();
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
    let red = topo::splitting::split_reduce(&body, &plane, Tol::witness()).unwrap();
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
    let eps = Tol::witness().get().eps;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.5 + 3.0 * eps, 0.0),
        normal: Vec3::unit_y(),
    };
    let err = split(&body, &plane, Tol::witness()).unwrap_err();
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
    match split(&body, &plane, Tol::witness()) {
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

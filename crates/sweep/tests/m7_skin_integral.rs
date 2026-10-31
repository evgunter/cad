//! **#207: an integral skin fit never synthesizes a rational wall.**
//!
//! Before this suite the §10.3 skin solved the weight channel even when
//! every input section was non-rational, and the LU round-trip landed
//! that channel an ulp off `1.0` for most parameterizations. A wall
//! with a non-unit weight is bitwise RATIONAL, and when this suite was
//! written `speed_lower_bound` poisoned for ANY rational carrier (the
//! derivative of a rational B-spline is not a convex combination of any
//! control net), so the `nurbs_span_meter` margin came back `Invalid`
//! and the body refused at assembly. The measured blast radius was
//! everything the uniform lane hid: `sweep_body` with ANY curved path
//! had zero successful callers in the tree, and `loft_body` refused ANY
//! non-uniform section spacing.
//!
//! The pins here are the three halves of that statement: the drift is
//! gone at the source (`skin_on` interpolates integral sections in
//! Cartesian ℝ³ and emits exactly `1.0`), the two shapes that could
//! not be built now build and pass the tier ladder with derived
//! measures, and the uniform case that always worked did not move a
//! bit.
//!
//! **M7 (rational span meter): Pin 4 has flipped.** The meter grew a
//! rational arm — a quotient-rule assembly over the homogeneous control
//! net — so a rational wall no longer refuses at `nurbs_span_meter` on
//! account of being rational. Pin 4 below now asserts the positive
//! statement its own retirement condition specified. The drift fix the
//! first three pins hold is untouched and still the right fix: a
//! manufactured weight channel is a lie about the geometry whether or
//! not the meter can cope with it.

// Panicking is a test's failure mechanism (workspace lint policy).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};
use profile::SketchPlane;
use profile::test_support::bulge_loop;

use geom::NurbsCurve3;
use geom::curves::fit::interpolate_columns;
use geom_core::{Point2, Point3};
use sweep::skin::{LoftGeometry, Section, loft_geometry, loft_parameters, sweep_geometry};
use sweep::test_support::{
    ELBOW_H, ELBOW_R, ELBOW_STATIONS, ELBOW_V_DEGREE, PRISM_Z, elbow_path, elbow_section,
    loft_prism_sections, stacked_at,
};
use sweep::{loft_body, sweep_body};

use crate::common;
use common::orient::{
    LevelIndex, along_v, assert_caps_face_out, assert_walls_face_out, chart_at, min_roll_turn,
};
use common::quad;
use geom_core::Tol;

/// A closed square section of half-width `h`, centred on the sketch
/// origin — the plainest possible INTEGRAL profile (four lines, unit
/// weights, no arc anywhere).
fn square(h: f64) -> Section {
    quad([(-h, -h), (h, -h), (h, h), (-h, h)])
}

/// A loft's walls at the loft's own parameters (v-degree 2), as
/// `loft_body` builds them.
fn lofted_geometry(sections: &[Section], places: &[SketchPlane<f64>]) -> LoftGeometry {
    let params = loft_parameters(sections, places, 2, Tol::witness()).expect("parameters");
    loft_geometry(sections, places, 2, &params, Tol::witness()).expect("geometry")
}

/// Every weight of every wall, bit-exactly `1.0` — the integral lane's
/// output, asserted with `==` because an ulp here is a change of KIND,
/// not an error.
fn walls_are_integral(g: &LoftGeometry, what: &str) {
    for (l, loop_walls) in g.walls.iter().enumerate() {
        for (j, wall) in loop_walls.iter().enumerate() {
            for (i, w) in wall.weights().iter().enumerate() {
                assert_eq!(
                    *w, 1.0,
                    "{what}: wall [{l}][{j}] weight {i} is {w}, not bit-exactly 1.0 — \
                     an integral input skinned to a RATIONAL wall"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------
// The defect, and its absence
// ---------------------------------------------------------------------

/// The OLD lane, reproduced verbatim from the pre-#207 `skin_on`:
/// homogeneous `(w·x, w·y, w·z, w)` rows through the same collocation
/// solve, then the projective divide. Returns the control points and
/// weights it would have produced.
fn homogeneous_lane(
    sections: &[NurbsCurve3<f64>],
    v_degree: usize,
    params: &[f64],
) -> (Vec<Point3<f64>>, Vec<f64>) {
    let n = sections[0].control().len();
    let rows: Vec<Vec<f64>> = sections
        .iter()
        .map(|c| {
            let mut row = Vec::with_capacity(4 * n);
            for (p, w) in c.control().iter().zip(c.weights()) {
                row.extend([p.x * w, p.y * w, p.z * w, *w]);
            }
            row
        })
        .collect();
    let (_, solved) = interpolate_columns(params, v_degree, &rows).expect("old lane solves");
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for i in 0..n {
        for row in &solved {
            let (x, y, z, w) = (row[4 * i], row[4 * i + 1], row[4 * i + 2], row[4 * i + 3]);
            control.push(Point3::new(x / w, y / w, z / w));
            weights.push(w);
        }
    }
    (control, weights)
}

/// **The defect is real and is exactly where the fix is.** On the
/// non-uniform spacing the old homogeneous lane still returns weights
/// off `1.0`; the shipped lane returns `1.0`. Same sections, same
/// parameters, same solve — the only difference is that the shipped
/// lane never manufactures the weight column.
#[test]
fn the_homogeneous_lane_still_drifts_where_the_shipped_lane_does_not() {
    let g = lofted_geometry(
        &[square(1.0), square(1.0), square(1.0)],
        &stacked_at(&[0.0, 1.0, 3.0]),
    );
    let (_, old_w) = homogeneous_lane(&g.sections[0][0], 2, &g.section_params);
    assert!(
        old_w.iter().any(|w| *w != 1.0),
        "the pre-#207 lane must still be shown drifting here (weights {old_w:?}); \
         if it no longer does, this suite has stopped testing anything"
    );
    walls_are_integral(&g, "non-uniform loft");
}

/// The Cartesian solve of every row: `(x, y, z)` triples through the
/// same collocation, no weight anywhere. What a constant-weight row's
/// control points must be, bit for bit.
fn cartesian_lane(
    sections: &[NurbsCurve3<f64>],
    v_degree: usize,
    params: &[f64],
) -> Vec<Point3<f64>> {
    let n = sections[0].control().len();
    let rows: Vec<Vec<f64>> = sections
        .iter()
        .map(|c| c.control().iter().flat_map(|p| p.to_array()).collect())
        .collect();
    let (_, solved) = interpolate_columns(params, v_degree, &rows).expect("cartesian lane solves");
    let mut control = Vec::new();
    for i in 0..n {
        for row in &solved {
            control.push(Point3::new(row[3 * i], row[3 * i + 1], row[3 * i + 2]));
        }
    }
    control
}

fn bits(p: Point3<f64>) -> [u64; 3] {
    p.to_array().map(f64::to_bits)
}

/// Asserts the skinned row `i` (its `nv` stations) is the Cartesian
/// lane's control points at weight exactly `c`.
fn assert_cartesian_row(
    what: &str,
    (control, weights): (&[Point3<f64>], &[f64]),
    cartesian: &[Point3<f64>],
    (i, nv): (usize, usize),
    c: f64,
) {
    for at in i * nv..(i + 1) * nv {
        assert_eq!(
            weights[at].to_bits(),
            c.to_bits(),
            "{what}: row {i}, net index {at}: weight {} is not bit-exactly {c}",
            weights[at]
        );
        assert_eq!(
            bits(control[at]),
            bits(cartesian[at]),
            "{what}: row {i}, net index {at}: {:?} is not the Cartesian lane's {:?}",
            control[at],
            cartesian[at]
        );
    }
}

/// Asserts the skinned row `i` is the homogeneous lane's, bit for bit.
fn assert_homogeneous_row(
    what: &str,
    (control, weights): (&[Point3<f64>], &[f64]),
    (old_c, old_w): (&[Point3<f64>], &[f64]),
    (i, nv): (usize, usize),
) {
    for at in i * nv..(i + 1) * nv {
        assert!(
            weights[at].to_bits() == old_w[at].to_bits() && bits(control[at]) == bits(old_c[at]),
            "{what}: row {i}, net index {at} moved off the homogeneous lane: {:?} w {} vs {:?} w {}",
            control[at],
            weights[at],
            old_c[at],
            old_w[at]
        );
    }
}

/// Whether the homogeneous lane lands row `i` anywhere off the Cartesian
/// lane at weight `c` — what makes a constant-weight row's pin bite.
fn homogeneous_drifts(
    (old_c, old_w): (&[Point3<f64>], &[f64]),
    cartesian: &[Point3<f64>],
    (i, nv): (usize, usize),
    c: f64,
) -> (bool, bool) {
    let range = i * nv..(i + 1) * nv;
    (
        range.clone().any(|at| old_w[at] != c),
        range
            .into_iter()
            .any(|at| bits(old_c[at]) != bits(cartesian[at])),
    )
}

/// **An arc wall's rows are each constant-weight.** `arc_section`'s arc
/// is one rational quadratic: its two end rows are the corners it
/// shares with the line walls, at weight `1.0`, and its middle row
/// carries the arc's weight `cos 45°`. The sections are identical
/// copies, so every row is constant along the stack by construction,
/// and every row skins in the Cartesian lane at its own weight — the
/// corner rows at exactly `1.0`, which is what the line neighbour's
/// boundary row holds. At z = 0, 1, 3 the homogeneous lane lands the
/// corner rows' weights AND every row's control points off that, so
/// neither a whole-wall homogeneous solve nor one whose weights are
/// snapped back afterwards passes.
#[test]
fn an_arc_walls_rows_each_skin_cartesian_at_their_own_weight() {
    let g = lofted_geometry(
        &[
            common::arc_section(1.0),
            common::arc_section(1.0),
            common::arc_section(1.0),
        ],
        &stacked_at(&[0.0, 1.0, 3.0]),
    );
    let sections = &g.sections[0][1];
    let wall = &g.walls[0][1];
    let params = &g.section_params;
    let nv = params.len();
    let (old_c, old_w) = homogeneous_lane(sections, 2, params);
    let cartesian = cartesian_lane(sections, 2, params);
    // The arc's own middle weight, `cos 45°` as the profile door
    // rounds it.
    let arc_w = sections[0].weights()[1];
    let want = [1.0, arc_w, 1.0];
    assert!(
        sections[0].weights() == want.as_slice() && (arc_w - 0.5f64.sqrt()).abs() < 1e-15,
        "the fixture's arc must be one rational quadratic, ends at 1 and middle at cos 45° \
         (weights {:?})",
        sections[0].weights()
    );
    let skinned = (wall.control(), wall.weights());
    let old = (old_c.as_slice(), old_w.as_slice());
    let corner_weight_drifts = homogeneous_drifts(old, &cartesian, (0, nv), 1.0).0;
    let points_drift = (0..3).all(|i| homogeneous_drifts(old, &cartesian, (i, nv), want[i]).1);
    assert!(
        corner_weight_drifts && points_drift,
        "the homogeneous lane must still land the corner row's weights and every row's \
         points off the Cartesian lane here; if it no longer does, this row tests nothing"
    );
    for (i, c) in want.into_iter().enumerate() {
        assert_cartesian_row("arc wall", skinned, &cartesian, (i, nv), c);
    }
}

/// **Each row takes its own lane, decided over every section.** A
/// static three-row net over four sections, its rows by construction:
/// row 0 at weight `1.0` in every section (integral), row 1 at `1.0` in
/// section 0 only (rational — a lane decided on one section, or on
/// any section, would take it for integral), row 2 at `0.75` in every
/// section (constant-weight). Rows 0 and 2 must be the Cartesian lane's
/// at their constant; row 1 the homogeneous lane's, bit for bit.
#[test]
fn each_row_takes_its_own_lane() {
    const INTEGRAL: usize = 0;
    const MIXED: usize = 1;
    const CONSTANT: usize = 2;
    let weights = [
        [1.0, 1.0, 0.75],
        [1.0, 0.8, 0.75],
        [1.0, 0.9, 0.75],
        [1.0, 0.85, 0.75],
    ];
    let heights = [0.0, 0.7, 1.9, 3.0];
    let sections: Vec<NurbsCurve3<f64>> = heights
        .iter()
        .zip(weights)
        .enumerate()
        .map(|(k, (z, w))| {
            let k = f64::from(u8::try_from(k).expect("four sections"));
            NurbsCurve3::new(
                geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2)
                    .expect("one quadratic span"),
                vec![
                    Point3::new(0.0, 0.1 * k, *z),
                    Point3::new(1.0 + 0.3 * k, 1.1, *z),
                    Point3::new(2.0, 0.2 - 0.05 * k, *z),
                ],
                w.to_vec(),
            )
            .expect("a section")
        })
        .collect();
    let params = [0.0, 0.3, 0.65, 1.0];
    let nv = params.len();
    let wall = sweep::skin::skin_on(&sections, 2, &params).expect("the net skins");
    let (old_c, old_w) = homogeneous_lane(&sections, 2, &params);
    let cartesian = cartesian_lane(&sections, 2, &params);
    let old = (old_c.as_slice(), old_w.as_slice());
    let (integral_w, integral_p) = homogeneous_drifts(old, &cartesian, (INTEGRAL, nv), 1.0);
    let (constant_w, constant_p) = homogeneous_drifts(old, &cartesian, (CONSTANT, nv), 0.75);
    assert!(
        integral_w && integral_p && constant_w && constant_p,
        "the homogeneous lane must land the integral and constant rows' weights and points \
         off the Cartesian lane here ({integral_w} {integral_p} {constant_w} {constant_p}); \
         if it no longer does, this row tests nothing"
    );
    let skinned = (wall.control(), wall.weights());
    assert_cartesian_row("integral row", skinned, &cartesian, (INTEGRAL, nv), 1.0);
    assert_cartesian_row("constant row", skinned, &cartesian, (CONSTANT, nv), 0.75);
    assert_homogeneous_row("mixed row", skinned, old, (MIXED, nv));
}

/// **Pin 3: the uniform case did not move a bit.** For every wall of
/// the `loft_prism` corpus loft, the shipped control points are
/// BITWISE the ones the old homogeneous lane produced. (The other half
/// of this pin is external: `step-export`'s committed
/// `fixtures/loft_prism.expect` holds the pre-fix bytes and is
/// compared byte-for-byte by the golden-file suite.)
#[test]
fn the_uniform_loft_is_bitwise_unchanged() {
    let g = lofted_geometry(&loft_prism_sections(), &stacked_at(&PRISM_Z));
    for (l, loop_walls) in g.walls.iter().enumerate() {
        for (j, wall) in loop_walls.iter().enumerate() {
            let (old_c, old_w) = homogeneous_lane(&g.sections[l][j], 2, &g.section_params);
            assert!(
                old_w.iter().all(|w| *w == 1.0),
                "the uniform loft's old weights were exact, so the lanes must agree"
            );
            for (i, (got, want)) in wall.control().iter().zip(&old_c).enumerate() {
                assert!(
                    got.x == want.x && got.y == want.y && got.z == want.z,
                    "wall [{l}][{j}] control {i} moved: {got:?} vs {want:?}"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------
// Pin 2: a non-uniformly-spaced loft body
// ---------------------------------------------------------------------

/// **Pin 2a, with a derived volume.** Identical unit squares (side 2)
/// at z = 0, 1, 3 — spacing 1 : 2, so the chord-length
/// v-parameterization is `[0, 1/3, 1]`, which is exactly what used to
/// poison. The sections are congruent, so the solid is the prism of
/// area 4 and height 3: **V = 12 m³**, derived, not measured.
#[test]
fn nonuniform_prism_loft_body_matches_the_derived_volume() {
    let sections = [square(1.0), square(1.0), square(1.0)];
    let places = stacked_at(&[0.0, 1.0, 3.0]);
    let lofted = loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("the non-uniform loft builds");
    assert_eq!(topo::validate(&lofted.body), Ok(()), "tier 1");
    assert_eq!(
        topo::validate_closed(&lofted.body),
        Ok(()),
        "tier 2 (watertight)"
    );
    let body = topo::AtRestBody::validate(topo::Body::clone(&lofted.body), Tol::witness())
        .expect("tier 3 at rest");
    let m = topo::mass_properties(
        &body,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .expect("mass properties");
    assert!(
        (m.enclosure.volume - 12.0).abs() <= m.enclosure.volume_pad + 1e-9,
        "volume {} ± {} must bracket the derived 12",
        m.enclosure.volume,
        m.enclosure.volume_pad
    );
}

/// **Pin 2b: the same non-uniform spacing under the corpus's
/// non-affine sections**, where the walls are genuinely curved in v
/// and no closed form is claimed — the pin is that it builds at all,
/// passes the whole ladder, and skins integral.
#[test]
fn nonuniform_trapezoid_loft_body_is_tier3_valid() {
    let places = stacked_at(&[0.0, 1.0, 3.0]);
    walls_are_integral(
        &lofted_geometry(&loft_prism_sections(), &places),
        "non-uniform trapezoid loft",
    );
    let lofted =
        loft_body::<f64>(&loft_prism_sections(), &places, 2, Tol::witness()).expect("builds");
    assert_eq!(topo::validate_closed(&lofted.body), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&lofted.body, Tol::witness()),
        Ok(()),
        "tier 3"
    );
}

// ---------------------------------------------------------------------
// Pin 1: the first successful `sweep_body` caller in the tree
// ---------------------------------------------------------------------

// The elbow — path, section and constants — is `sweep::test_support`'s:
// this suite derives the Pappus bracket over it, and the tessellation
// and STEP suites one crate over meter the same solid. The rational
// rows below reuse the PATH under a different section, which is why
// the halves are named separately there.

/// **Pin 1: a curved-path `sweep_body` builds, passes the tier ladder,
/// and measures.** This is the first successful `sweep_body` caller in
/// the tree — before #207 every curved path refused at assembly.
///
/// # The measure is a BRACKET, and why it has to be
///
/// The frame `sweep_places` carries along a planar arc turns only
/// about the arc's own axis (`crates/sweep/README.md` `[sweep.frame]`),
/// and the path starts at the frame origin, so every station is the
/// start frame rotated about the arc centre: the STATIONS lie exactly
/// on the quarter torus of square cross-section whose Pappus volume is
/// `A·R̄·θ = (2h)²·R·(π/2)`. The WALLS, though, are the degree-3
/// v-interpolation through nine such stations — a polynomial
/// approximation of circular motion, not the torus. So the honest
/// claim is convergence to Pappus, not equality with it: measured
/// 8.6e-4 at 5 stations, 1.03e-5 at 9 and −2.1e-7 at 17.
///
/// The stations sit at the path's parameter, and this path is a
/// rational quarter arc, whose parameter is not its arc length, so the
/// cubic fits the circular motion about 2.7× worse than stations at
/// their chord lengths did (3.8e-6 at 9). The cap is that measurement
/// plus a tenth. The certified enclosure `pad` is ~1e-14 relative here,
/// eight orders TIGHTER than that gap — the bracket is a
/// discretization statement, and the assertion below pins the pad
/// separately so the tolerance can never be silently absorbed by a
/// loosening enclosure.
#[test]
fn curved_path_sweep_body_builds_validates_and_brackets_pappus() {
    let path = elbow_path();
    let profile = elbow_section();
    let stations = ELBOW_STATIONS;
    let v_degree = ELBOW_V_DEGREE;

    let geometry = sweep_geometry(
        &profile,
        SketchPlane::xy(),
        &path,
        stations,
        v_degree,
        Tol::witness(),
    )
    .expect("the elbow skins");
    walls_are_integral(&geometry, "curved-path sweep");

    let swept = sweep_body::<f64>(
        &profile,
        SketchPlane::xy(),
        &path,
        stations,
        v_degree,
        Tol::witness(),
    )
    .expect("the curved-path sweep body builds");
    assert_eq!(topo::validate(&swept.body), Ok(()), "tier 1");
    assert_eq!(
        topo::validate_closed(&swept.body),
        Ok(()),
        "tier 2 (watertight)"
    );
    let body = topo::AtRestBody::validate(topo::Body::clone(&swept.body), Tol::witness())
        .expect("tier 3 at rest");

    let m = topo::mass_properties(
        &body,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .expect("mass properties");
    let pappus = (2.0 * ELBOW_H) * (2.0 * ELBOW_H) * ELBOW_R * FRAC_PI_2;
    let rel = ((m.enclosure.volume - pappus) / pappus).abs();
    assert!(
        rel <= 1.1e-5,
        "swept volume {} vs Pappus {pappus} (rel {rel}) — the 9-station \
         discretization bracket",
        m.enclosure.volume
    );
    assert!(
        m.enclosure.volume_pad < 1e-9,
        "the certified enclosure must stay far tighter than the discretization \
         bracket, else the bracket is measuring the wrong thing (pad {})",
        m.enclosure.volume_pad
    );
}

/// The seam carriers of that swept body are the thing that used to
/// poison: NURBS iso-curves off the wall. Pinned directly — every one
/// of them yields a POSITIVE speed lower bound, which is exactly the
/// `nurbs_span_meter` input that came back `Invalid` before.
#[test]
fn the_swept_bodys_seam_carriers_meter_positively() {
    let swept = sweep_body::<f64>(
        &elbow_section(),
        SketchPlane::xy(),
        &elbow_path(),
        ELBOW_STATIONS,
        ELBOW_V_DEGREE,
        Tol::witness(),
    )
    .expect("the curved-path sweep body builds");
    let body = &swept.body;
    let mut seen = 0usize;
    for (_, edge) in body.edges() {
        let topo::CurveGeom::Certified(curve) =
            body.get_curve_geom(edge.curve).expect("curve key resolves")
        else {
            panic!("a finished body has no null scaffolding");
        };
        if let geom::Curve3::Nurbs(c) = curve.carrier() {
            let s = c.speed_lower_bound().get();
            assert!(
                s > 0.0,
                "a seam carrier's speed lower bound is {s} (poison or non-positive) — \
                 the #207 refusal is back"
            );
            seen += 1;
        }
    }
    assert_eq!(seen, 4, "the elbow has four wall–wall seams");
}

// ---------------------------------------------------------------------
// Pin 4: the frontier the rational span meter DID move (M7) — a
// RATIONAL section
// ---------------------------------------------------------------------

/// A closed CIRCLE of radius `r` about the sketch origin: the
/// two-vertex bulge-1 chain, i.e. two semicircular arcs. The plainest
/// possible RATIONAL profile — an arc's exact NURBS form carries
/// non-unit weights, and there is no parameterization of a circle that
/// does not.
fn circle_section(r: f64) -> Section {
    vec![bulge_loop(vec![
        (Point2::new(-r, 0.0), 1.0),
        (Point2::new(r, 0.0), 1.0),
    ])]
}

/// **A rational section swept along a curved path builds, and its seam
/// carriers meter positively.**
///
/// FLIPPED per this pin's own retirement condition (M7, the rational
/// span meter). It used to assert a refusal: a section that is rational
/// on its own merits — an arc's weights are not an artifact to be
/// removed, they *are* the arc — skinned to a genuinely rational wall,
/// whose seam carrier's `speed_lower_bound` returned the documented
/// poison, so `nurbs_span_meter` had no metre-per-parameter and
/// escalated at `CertCheck::ParamSpan`.
///
/// The span meter carries a rational arm (a quotient-rule assembly
/// over the HOMOGENEOUS control net, derivation on
/// `NurbsCurve3::speed_lower_bound`), so the refusal is gone and the
/// positive statement takes its place, exactly as
/// [`the_swept_bodys_seam_carriers_meter_positively`] states it for the
/// integral lane: the body builds, and every NURBS carrier on it
/// reports a real, positive metre-per-parameter.
///
/// (The QUADRATURE half is a different frontier and is not what this
/// row states: it has its own pins, and its remaining limit is a fixed
/// round budget rather than a missing lane.)
#[test]
fn a_rational_section_on_a_curved_path_meters_at_the_span_meter() {
    let swept = sweep_body::<f64>(
        &circle_section(ELBOW_H),
        SketchPlane::xy(),
        &elbow_path(),
        ELBOW_STATIONS,
        ELBOW_V_DEGREE,
        Tol::witness(),
    )
    .expect(
        "a rational section skins to a body the certifier accepts — if this refuses, \
         the rational span meter has regressed and #207's frontier moved BACK",
    );
    let body = &swept.body;
    let mut seen = 0usize;
    for (_, edge) in body.edges() {
        let topo::CurveGeom::Certified(curve) =
            body.get_curve_geom(edge.curve).expect("curve key resolves")
        else {
            panic!("a finished body has no null scaffolding");
        };
        if let geom::Curve3::Nurbs(c) = curve.carrier() {
            let s = c.speed_lower_bound().get();
            assert!(
                s > 0.0,
                "a rational wall's carrier meters {s} (poison or non-positive) — the \
                 span-meter refusal is back"
            );
            seen += 1;
        }
    }
    assert!(
        seen > 0,
        "the rational sweep produced no NURBS carrier at all — the fixture stopped \
         exercising the thing it pins"
    );
}

/// **The rational elbow answers a bracket, never a refusal**: the
/// circle-section elbow above, whose rational walls stop short of
/// `1024·ε` at the default ε, measured at the suites' pinned resolution
/// (`1024·ε`, which is 1.024e-6 m at the default ε). It answers its
/// enclosure with the stop that says why, at the wall or met — never
/// the floor, at any ε.
///
/// Its walls interpolate the torus through nine stations rather than
/// being it, so Pappus is not its volume: the two differ by the fit,
/// measured at 1.03e-5 of Pappus, and the row holds the bracket to
/// Pappus within 1.1e-5 of it beyond the pad. The fit is the square
/// elbow's above: stations at the rational arc's parameter, which is
/// not its arc length (3.9e-6 when they sat at chord lengths). Acceptance 5's exact
/// oracle is the conic quarter torus, whose walls ARE the torus
/// (`step-import`'s
/// `a_conic_quarter_torus_measures_a_bracket_at_the_wall`).
///
/// Through the raw door, because the subject is the measurement and not
/// the gate.
#[test]
fn a_rational_quarter_torus_answers_a_bracket_at_the_wall_not_a_refusal() {
    let tol = Tol::witness();
    let swept = sweep_body::<f64>(
        &circle_section(ELBOW_H),
        SketchPlane::xy(),
        &elbow_path(),
        ELBOW_STATIONS,
        ELBOW_V_DEGREE,
        tol,
    )
    .expect("the rational elbow builds");
    // Raw door: the measurement, not tier 3's sign read, is the subject, and tier 3 may leave this body's sign open at its walls' wall (VolumeSignUnresolved).
    let measured = topo::test_support::measure_unvalidated(
        &swept.body,
        tol,
        topo::test_support::pinned_resolution(tol),
    )
    .unwrap_or_else(|e| panic!("the quarter torus measures, never refuses: {e:?}"));
    assert!(
        matches!(measured.stop, topo::Stop::Wall | topo::Stop::Met),
        "the stop is the wall or the resolution met: {measured:?}"
    );
    let e = measured.enclosure;
    let pappus = PI * ELBOW_H * ELBOW_H * ELBOW_R * FRAC_PI_2;
    let gap = 1.1e-5 * pappus;
    assert!(
        (e.volume - pappus).abs() <= e.volume_pad + gap,
        "the bracket {} ± {:e} holds Pappus's {pappus} within the fit's {gap:e}",
        e.volume,
        e.volume_pad
    );
    eprintln!(
        "rational elbow @ ε = {:e}, resolution 1024·ε: stop {:?}, V = {} ± {:e}",
        tol.eps(),
        measured.stop,
        e.volume,
        e.volume_pad
    );
}

/// The step off a wall, both ways, that the material-side probe takes.
/// The section is a circle of radius [`ELBOW_H`] = 0.25, so an inward
/// step lands at radius 0.19 and an outward one at 0.31 — each a fifth
/// of the radius clear of the wall, and three orders above the level
/// polyline's chord error at 64 samples per half-circle (7.5e-5).
const PROBE_DELTA: f64 = 0.06;

/// **The rational swept chart's walls face out of the material.**
///
/// The orientation half of the fixture above. It is here rather than in
/// the orientation suite next door because this is the tree's only
/// RATIONAL swept chart, and the alternative to putting the row beside
/// its fixture is typing that fixture a third time.
///
/// `m5_s11_concave_sense`'s elbow row makes this claim for an INTEGRAL
/// square section on this very path. What is unpinned until here is the
/// rational arm: `S_u × S_v` off a chart whose weight channel is not
/// `1`, which is a different jet through different code even though the
/// `sense` it is signed by is minted by the same traversal argument.
///
/// ANTI-VACUITY on both halves of that. The chart must genuinely turn
/// along `v` — the level planes are the path's normal planes, so a
/// quarter-turn arc turns them by `π/2` and a straight path by nothing.
/// The bar is [`min_roll_turn`]'s, the same law the helix rows use, at
/// this path's `k = 0`. And the walls must genuinely be rational, or
/// the row is the integral elbow next door retyped.
///
/// The CAPS are probed for the reason the helix rows probe them: a body
/// whose two cap bits are inverted leaves every wall bit honest and
/// passes every wall probe.
///
/// Tiers 1 and 2 only, deliberately, and not because the caps are
/// covered by them. This fixture's rational walls miss the certified
/// quadrature's `1024·ε` target at this scale — the same honest
/// out-of-budget posture `m5_s11_concave_sense`'s rational hole-wall
/// row records — so `validate_geometric` refuses here with
/// `VolumeUncomputable`, and pinning that refusal costs 61 s against
/// this row's 0.04 s for a statement about the quadrature schedule
/// rather than about orientation. The cap probe above is what pins the
/// cap bits on this body; the helix rows run the full ladder as well
/// because there it is free.
#[test]
fn a_rational_section_on_a_curved_path_faces_out_along_the_turn() {
    let swept = sweep_body::<f64>(
        &circle_section(ELBOW_H),
        SketchPlane::xy(),
        &elbow_path(),
        ELBOW_STATIONS,
        ELBOW_V_DEGREE,
        Tol::witness(),
    )
    .expect("the rational elbow sweeps");
    assert_eq!(topo::validate(&swept.body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&swept.body), Ok(()), "tier 2");

    let index = LevelIndex::build(&swept);
    let turned = index.total_turn();
    let want = min_roll_turn(0.25, 1.0, 0.0);
    assert!(
        turned >= want,
        "the elbow must turn the level planes a quarter turn along v, not \
         {turned} rad against {want} — a straight path would hold them parallel"
    );
    let (surface, _, _) = chart_at(&swept.body, swept.side_faces[0][0][0], 0.5, 0.5);
    assert!(
        surface.weights().iter().any(|w| *w != 1.0),
        "the fixture stopped exercising a RATIONAL chart: every weight is 1.0, so \
         this row now restates the integral elbow next door"
    );

    let oracle = |q| index.contains(q);
    assert_walls_face_out(&swept, &oracle, &along_v(), PROBE_DELTA, 2);
    assert_caps_face_out(&swept, &oracle, PROBE_DELTA);
}

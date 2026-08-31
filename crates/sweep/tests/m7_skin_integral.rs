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
use profile::RawLoop;

use geom::NurbsCurve3;
use geom::curves::fit::interpolate_columns;
use geom_core::{Affine3, Point2, Point3, Vec3};
use sweep::skin::{LoftGeometry, Section, loft_geometry, segment_curve, sweep_geometry};
use sweep::{SketchSegment, loft_body, sweep_body};

mod common;
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

/// The `loft_prism` corpus sections: squares at the ends, a NON-AFFINE
/// trapezoid in the middle (so the walls are genuinely curved in v).
fn prism_sections() -> Vec<Section> {
    vec![
        quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        quad([(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
    ]
}

fn at_z(zs: &[f64]) -> Vec<Affine3<f64>> {
    zs.iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect()
}

/// Every weight of every wall, bit-exactly `1.0` — the kernel's own
/// definition of non-rational (`speed_lower_bound`'s test), asserted
/// with `==` because an ulp here is a change of KIND, not an error.
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
    let g = loft_geometry(
        &[square(1.0), square(1.0), square(1.0)],
        &at_z(&[0.0, 1.0, 3.0]),
        2,
        Tol::witness(),
    )
    .expect("geometry");
    let (_, old_w) = homogeneous_lane(&g.sections[0][0], 2, &g.section_params);
    assert!(
        old_w.iter().any(|w| *w != 1.0),
        "the pre-#207 lane must still be shown drifting here (weights {old_w:?}); \
         if it no longer does, this suite has stopped testing anything"
    );
    walls_are_integral(&g, "non-uniform loft");
}

/// **Pin 3: the uniform case did not move a bit.** For every wall of
/// the `loft_prism` corpus loft, the shipped control points are
/// BITWISE the ones the old homogeneous lane produced. (The other half
/// of this pin is external: `step-export`'s committed
/// `fixtures/loft_prism.expect` holds the pre-fix bytes and is
/// compared byte-for-byte by the golden-file suite.)
#[test]
fn the_uniform_loft_is_bitwise_unchanged() {
    let g = loft_geometry(
        &prism_sections(),
        &at_z(&[0.0, 1.0, 2.0]),
        2,
        Tol::witness(),
    )
    .expect("geometry");
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
    let places = at_z(&[0.0, 1.0, 3.0]);
    let lofted = loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("the non-uniform loft builds");
    let body = &lofted.body;
    assert_eq!(topo::validate(body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "tier 2 (watertight)");
    assert_eq!(
        topo::validate_geometric(body, Tol::witness()),
        Ok(()),
        "tier 3 at rest"
    );
    let m = topo::props::mass_properties(body, Tol::witness()).expect("mass properties");
    assert!(
        (m.volume - 12.0).abs() <= m.volume_pad + 1e-9,
        "volume {} ± {} must bracket the derived 12",
        m.volume,
        m.volume_pad
    );
}

/// **Pin 2b: the same non-uniform spacing under the corpus's
/// non-affine sections**, where the walls are genuinely curved in v
/// and no closed form is claimed — the pin is that it builds at all,
/// passes the whole ladder, and skins integral.
#[test]
fn nonuniform_trapezoid_loft_body_is_tier3_valid() {
    let places = at_z(&[0.0, 1.0, 3.0]);
    walls_are_integral(
        &loft_geometry(&prism_sections(), &places, 2, Tol::witness()).expect("geometry"),
        "non-uniform trapezoid loft",
    );
    let lofted = loft_body::<f64>(&prism_sections(), &places, 2, Tol::witness()).expect("builds");
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

/// The elbow's path: a quarter circle of radius [`ELBOW_R`] in the
/// world YZ plane, starting at the origin with tangent `+z` (so the
/// identity-placed profile, which lies in the world XY plane, is
/// already perpendicular to the path) and ending at `(0, R, R)`. Its
/// centre is `(0, R, 0)`; the axis of revolution is the world-x
/// direction through that centre.
///
/// The sketch arc runs `(0,0) → (R,R)` with `bulge = tan(θ/4) =
/// tan(π/8)`, i.e. a 90° turn, and the placement rotates the sketch
/// plane by −π/2 about the world y-axis, which sends sketch `(x, y)`
/// to world `(0, y, x)`.
const ELBOW_R: f64 = 3.0;
/// The elbow's square cross-section half-width.
const ELBOW_H: f64 = 0.25;

fn elbow_path() -> NurbsCurve3<f64> {
    let place = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        -FRAC_PI_2,
    );
    segment_curve(
        0,
        SketchSegment::Arc {
            a: Point2::new(0.0, 0.0),
            b: Point2::new(ELBOW_R, ELBOW_R),
            bulge: (PI / 8.0).tan(),
        },
        place,
    )
    .expect("the elbow path is a well-formed quarter arc")
}

/// **Pin 1: a curved-path `sweep_body` builds, passes the tier ladder,
/// and measures.** This is the first successful `sweep_body` caller in
/// the tree — before #207 every curved path refused at assembly.
///
/// # The measure is a BRACKET, and why it has to be
///
/// The frame `sweep_places` builds for a planar arc is exactly the
/// rigid rotation about the arc's own axis (minimal rotation from the
/// base tangent, then translation onto the path — for a planar arc
/// starting at the frame origin those compose to rotation about the
/// arc centre), so the STATIONS lie exactly on the quarter torus of
/// square cross-section whose Pappus volume is `A·R̄·θ =
/// (2h)²·R·(π/2)`. The WALLS, though, are the degree-3 v-interpolation
/// through nine such stations — a polynomial approximation of circular
/// motion, not the torus. So the honest claim is convergence to
/// Pappus, not equality with it: at 9 stations the relative gap is
/// ≈ 4e-6, and it fell by ~1.8 decades when the station count went
/// 5 → 9 (measured 2.2e-4 → 3.8e-6). The certified enclosure `pad` is
/// ~1e-13 here, four orders TIGHTER than that gap — the bracket is a
/// discretization statement, and the assertion below pins the pad
/// separately so the tolerance can never be silently absorbed by a
/// loosening enclosure.
#[test]
fn curved_path_sweep_body_builds_validates_and_brackets_pappus() {
    let path = elbow_path();
    let profile = square(ELBOW_H);
    let stations = 9;
    let v_degree = 3;

    let geometry = sweep_geometry(
        &profile,
        Affine3::identity(),
        &path,
        stations,
        v_degree,
        Tol::witness(),
    )
    .expect("the elbow skins");
    walls_are_integral(&geometry, "curved-path sweep");

    let swept = sweep_body::<f64>(
        &profile,
        Affine3::identity(),
        &path,
        stations,
        v_degree,
        Tol::witness(),
    )
    .expect("the curved-path sweep body builds");
    let body = &swept.body;
    assert_eq!(topo::validate(body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "tier 2 (watertight)");
    assert_eq!(
        topo::validate_geometric(body, Tol::witness()),
        Ok(()),
        "tier 3 at rest"
    );

    let m = topo::props::mass_properties(body, Tol::witness()).expect("mass properties");
    let pappus = (2.0 * ELBOW_H) * (2.0 * ELBOW_H) * ELBOW_R * FRAC_PI_2;
    let rel = ((m.volume - pappus) / pappus).abs();
    assert!(
        rel <= 1e-5,
        "swept volume {} vs Pappus {pappus} (rel {rel}) — the 9-station \
         discretization bracket",
        m.volume
    );
    assert!(
        m.volume_pad < 1e-9,
        "the certified enclosure must stay far tighter than the discretization \
         bracket, else the bracket is measuring the wrong thing (pad {})",
        m.volume_pad
    );
}

/// The seam carriers of that swept body are the thing that used to
/// poison: NURBS iso-curves off the wall. Pinned directly — every one
/// of them yields a POSITIVE speed lower bound, which is exactly the
/// `nurbs_span_meter` input that came back `Invalid` before.
#[test]
fn the_swept_bodys_seam_carriers_meter_positively() {
    let swept = sweep_body::<f64>(
        &square(ELBOW_H),
        Affine3::identity(),
        &elbow_path(),
        9,
        3,
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
            let s = c.speed_lower_bound();
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
    vec![sweep::ProfileLoop::new(vec![
        sweep::ProfileVertex::new(Point2::new(-r, 0.0), 1.0),
        sweep::ProfileVertex::new(Point2::new(r, 0.0), 1.0),
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
        Affine3::identity(),
        &elbow_path(),
        9,
        3,
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
            let s = c.speed_lower_bound();
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
        Affine3::identity(),
        &elbow_path(),
        9,
        3,
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
    let (surface, _, _) = chart_at(&swept.body, swept.side_faces[0][0], 0.5, 0.5);
    assert!(
        surface.weights().iter().any(|w| *w != 1.0),
        "the fixture stopped exercising a RATIONAL chart: every weight is 1.0, so \
         this row now restates the integral elbow next door"
    );

    let oracle = |q| index.contains(q);
    assert_walls_face_out(&swept, &oracle, &along_v(), PROBE_DELTA, 2);
    assert_caps_face_out(&swept, &oracle, PROBE_DELTA);
}

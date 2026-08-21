//! M5 PR 10 — the blinded review's independent skinning-math probes,
//! ADOPTED into the PR (charter A). Authored against the
//! implementation without sight of it: arc exactness over adversarial
//! bulges, compatibility over mixed degrees and rational sections, a
//! closed-form rational loft checked BETWEEN sections, and Eq. 10.8
//! re-derived independently.
//!
//! The half-turn row is the fix pass's F3 evidence: it pins what the
//! frame ACTUALLY does at an anti-parallel tangent, not what the
//! docs would like it to do.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsCurve3;
use geom_brep::SketchSegment;
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Band, Point2, Point3};
use profile::RawLoop;
use sweep::skin::{SkinError, make_compatible, segment_curve, skin_on, skin_parameters};
use geom_core::Tol;

fn ring() -> f64 {
    Band::linear(Tol::witness()).expect("band").zero()
}

/// A1: arc → NURBS exactness, densely, over adversarial bulges
/// (tiny, quarter, just-over-quarter, 3/4 turn, near-full, negative).
/// Locus on the carrier circle + endpoints + total signed turn
/// = 4·atan(bulge).
#[test]
fn review_arc_exactness_dense_and_signed_turn() {
    let cases = [
        0.01,
        0.4142135623730951,
        0.45,
        1.0,
        core::f64::consts::SQRT_2 + 1.0, // tan(3pi/8): a 3/4 turn
        5.0,
        -0.7,
        -3.0,
    ];
    for bulge in cases {
        let a = Point2::new(1.0, 0.0);
        // Place b on the unit circle so the carrier is the unit circle:
        // chord endpoint at angle theta from a.
        let theta = 4.0 * f64::atan(bulge);
        let b = Point2::new(theta.cos(), theta.sin());
        let c = segment_curve(0, SketchSegment::Arc { a, b, bulge }, Affine3::identity())
            .expect("converts");
        let n = 4096usize;
        let mut prev = f64::atan2(0.0, 1.0);
        let mut turn = 0.0f64;
        for i in 0..=n {
            let t = i as f64 / n as f64;
            let p = c.eval(t);
            let r = p.x.hypot(p.y);
            assert!(
                (r - 1.0).abs() <= 4.0 * ring(),
                "bulge {bulge}: sample {i} off carrier by {}",
                (r - 1.0).abs()
            );
            assert!(p.z.abs() <= 4.0 * ring());
            let ang = p.y.atan2(p.x);
            if i > 0 {
                let mut d = ang - prev;
                while d > core::f64::consts::PI {
                    d -= 2.0 * core::f64::consts::PI;
                }
                while d < -core::f64::consts::PI {
                    d += 2.0 * core::f64::consts::PI;
                }
                turn += d;
            }
            prev = ang;
        }
        assert!(
            (turn - theta).abs() <= 1e-6,
            "bulge {bulge}: swept {turn}, want {theta}"
        );
        assert!(c.eval(0.0).distance(Point3::new(a.x, a.y, 0.0)) <= 4.0 * ring());
        assert!(c.eval(1.0).distance(Point3::new(b.x, b.y, 0.0)) <= 4.0 * ring());
    }
}

/// A2: adversarial compatibility — a degree-1 line (3 elevations
/// needed), a degree-4 Bezier, and a RATIONAL degree-2 curve with
/// dense repeated interior knots. Every section's locus must be
/// preserved through the doors, densely sampled.
#[test]
fn review_compat_mixed_degrees_dense_vs_sparse_knots_rational() {
    let line = NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(3.0, 1.0, 0.0)],
        vec![1.0, 1.0],
    )
    .unwrap();
    let quartic = NurbsCurve3::new(
        KnotVector::clamped(vec![0.0; 5].into_iter().chain([1.0; 5]).collect(), 4).unwrap(),
        vec![
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(1.0, 2.0, 1.0),
            Point3::new(2.0, -1.0, 1.5),
            Point3::new(2.5, 0.5, 1.0),
            Point3::new(3.0, 1.0, 1.0),
        ],
        vec![1.0; 5],
    )
    .unwrap();
    // degree 2, interior knots {0.2 (x2), 0.6}, rational weights.
    let dense = NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.2, 0.2, 0.6, 1.0, 1.0, 1.0], 2).unwrap(),
        vec![
            Point3::new(0.0, 0.0, 2.0),
            Point3::new(0.5, 1.0, 2.0),
            Point3::new(1.0, 0.0, 2.0),
            Point3::new(2.0, 1.5, 2.0),
            Point3::new(2.5, -0.5, 2.0),
            Point3::new(3.0, 1.0, 2.0),
        ],
        vec![1.0, core::f64::consts::FRAC_1_SQRT_2, 1.0, 2.0, 0.5, 1.0],
    )
    .unwrap();
    let raw = vec![line, quartic, dense];
    let compat = make_compatible(&raw).expect("compatible");
    let d = compat[0].degree();
    let kn = compat[0].knots().knots().to_vec();
    for (i, c) in compat.iter().enumerate() {
        assert_eq!(c.degree(), d, "section {i}");
        assert_eq!(c.knots().knots(), &kn[..], "section {i}");
    }
    assert_eq!(d, 4, "common degree is the max");
    for (i, (before, after)) in raw.iter().zip(&compat).enumerate() {
        for s in 0..=2048 {
            let t = f64::from(s) / 2048.0;
            let dist = before.eval(t).distance(after.eval(t));
            assert!(
                dist <= 16.0 * ring(),
                "section {i} locus moved {dist} at t={t}"
            );
        }
    }
    // And the trio skins.
    let params = skin_parameters(&compat).expect("params");
    assert!(skin_on(&compat, 2, &params).is_ok());
}

/// Lagrange quadratic through (0, y0), (vm, ym), (1, y1).
fn quad(v: f64, vm: f64, y0: f64, ym: f64, y1: f64) -> f64 {
    y0 * ((v - vm) * (v - 1.0)) / ((0.0 - vm) * (0.0 - 1.0))
        + ym * ((v - 0.0) * (v - 1.0)) / ((vm - 0.0) * (vm - 1.0))
        + y1 * ((v - 0.0) * (v - vm)) / ((1.0 - 0.0) * (1.0 - vm))
}

/// A3: closed-form RATIONAL loft. Sections are exact quarter arcs of
/// radius r_k at height z_k, identical weights; on a deliberately
/// skewed parameterization [0, 0.3, 1] with v-degree 2 the skinned
/// surface must equal the closed form BETWEEN sections:
/// |xy(u,v)| = R(v), z(u,v) = Z(v), with R and Z the unique quadratics
/// through the section data at the section parameters.
#[test]
fn review_loft_matches_closed_form_between_sections() {
    let w = core::f64::consts::FRAC_1_SQRT_2;
    let arc = |r: f64, z: f64| {
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
            vec![
                Point3::new(r, 0.0, z),
                Point3::new(r, r, z),
                Point3::new(0.0, r, z),
            ],
            vec![1.0, w, 1.0],
        )
        .unwrap()
    };
    let (rs, zs) = ([1.0, 1.7, 0.8], [0.0, 0.5, 2.0]);
    let sections: Vec<_> = rs.iter().zip(&zs).map(|(&r, &z)| arc(r, z)).collect();
    let params = [0.0, 0.3, 1.0];
    let surf = skin_on(&sections, 2, &params).expect("skins");
    for i in 0..=64 {
        let u = f64::from(i) / 64.0;
        for j in 0..=64 {
            let v = f64::from(j) / 64.0;
            let p = surf.eval(u, v);
            let want_r = quad(v, 0.3, rs[0], rs[1], rs[2]);
            let want_z = quad(v, 0.3, zs[0], zs[1], zs[2]);
            let got_r = p.x.hypot(p.y);
            assert!(
                (got_r - want_r).abs() <= 64.0 * ring(),
                "(u={u}, v={v}): radius {got_r} vs closed form {want_r}"
            );
            assert!(
                (p.z - want_z).abs() <= 64.0 * ring(),
                "(u={u}, v={v}): z {} vs closed form {want_z}",
                p.z
            );
        }
    }
}

/// A4a: Eq 10.8 v-parameters, independently derived. Four translated
/// copies at z = 0, 1, 1.5, 4: every control row's chords are
/// (1, 0.5, 2.5), total 4, so the params are EXACTLY
/// [0, 0.25, 0.375, 1] — skewed, and NOT renormalized to uniform.
/// Ends must be bit-exact 0 and 1.
#[test]
fn review_eq_10_8_skewed_spacing_is_preserved_bitwise() {
    let line_at = |z: f64| {
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![Point3::new(0.0, 0.0, z), Point3::new(1.0, 0.0, z)],
            vec![1.0, 1.0],
        )
        .unwrap()
    };
    let sections: Vec<_> = [0.0, 1.0, 1.5, 4.0].iter().map(|&z| line_at(z)).collect();
    let p = skin_parameters(&sections).expect("params");
    assert_eq!(p[0].to_bits(), 0.0f64.to_bits());
    assert_eq!(p[3].to_bits(), 1.0f64.to_bits());
    assert_eq!(p[1], 0.25, "got {p:?}");
    assert_eq!(p[2], 0.375, "got {p:?}");
}

/// A4b: the cross-row AVERAGE of Eq 10.8: row 0 accumulates (0, .5, 1),
/// row 1 (0, .75, 1) — the parameter is the average, 0.625.
#[test]
fn review_eq_10_8_averages_across_control_rows() {
    let sec = |a: f64, b: f64, z: f64| {
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![Point3::new(a, 0.0, z), Point3::new(b, 10.0, z)],
            vec![1.0, 1.0],
        )
        .unwrap()
    };
    // Row 0 x: 0, 1, 2 (chords 1, 1); row 1 x: 0, 3, 4 (chords 3, 1).
    let sections = vec![sec(0.0, 0.0, 0.0), sec(1.0, 3.0, 0.0), sec(2.0, 4.0, 0.0)];
    let p = skin_parameters(&sections).expect("params");
    assert_eq!(p[1], 0.625, "got {p:?}");
}

/// The pinned-row abstention: a control row that never moves must not
/// drag the average toward zero (it abstains — skin.rs's stated rule).
#[test]
fn review_pinned_row_abstains_from_the_average() {
    let sec = |b: f64, z: f64| {
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![Point3::new(0.0, 0.0, 0.0), Point3::new(b, 10.0, z)],
            vec![1.0, 1.0],
        )
        .unwrap()
    };
    // Row 0 pinned at the origin in every section; row 1 chords 1, 3.
    let sections = vec![sec(0.0, 0.0), sec(1.0, 0.0), sec(4.0, 0.0)];
    let p = skin_parameters(&sections).expect("params");
    assert_eq!(p[1], 0.25, "got {p:?}");
}

/// skin_on rejects a renormalization-shaped lie: params not clamped
/// 0→1 refuse typed rather than being silently rescaled.
#[test]
fn review_skin_on_refuses_unclamped_params() {
    let compat = {
        let line_at = |z: f64| {
            NurbsCurve3::new(
                KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                vec![Point3::new(0.0, 0.0, z), Point3::new(1.0, 0.0, z)],
                vec![1.0, 1.0],
            )
            .unwrap()
        };
        vec![line_at(0.0), line_at(1.0), line_at(2.0)]
    };
    assert!(matches!(
        skin_on(&compat, 2, &[0.0, 0.5, 2.0]),
        Err(SkinError::Fit(_))
    ));
    assert!(matches!(
        skin_on(&compat, 2, &[0.1, 0.5, 1.0]),
        Err(SkinError::Fit(_))
    ));
}

/// **F3 evidence, pinned as executed.** At an EXACTLY half-turn path
/// the tangents are anti-parallel in ℝ — but not in `f64`:
/// `|t₀ × t₁|` evaluates to ≈ `1.2e-16`, not `0`, so
/// `PathTangentReversal`'s anti-parallel arm does NOT fire and the
/// frame is built from an ill-conditioned axis.
///
/// This row asserts that truth rather than the tidier claim, so the
/// docs and the code cannot drift apart. If a future PR gives the
/// frame choice a named angular predicate with a real margin (see
/// `sweep_geometry`'s C6 note for why this fix pass declined to
/// invent one), this row flips to the refusal and says so loudly.
#[test]
fn review_half_turn_path_builds_on_the_float_knife_edge() {
    use sweep::skin::sweep_geometry;
    // Half-turn arc: bulge = tan(pi/4) = 1.
    let path = segment_curve(
        0,
        SketchSegment::Arc {
            a: Point2::new(0.0, 0.0),
            b: Point2::new(0.0, 2.0),
            bulge: 1.0,
        },
        Affine3::identity(),
    )
    .expect("path");
    let profile: sweep::Section = vec![profile::ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(0.3, 0.0),
        Point2::new(0.3, 0.2),
        Point2::new(0.0, 0.2),
    ])];
    match sweep_geometry(&profile, Affine3::identity(), &path, 3, 2) {
        Ok(_) => { /* the executed truth: sin(pi) != 0 in f64 */ }
        Err(SkinError::PathTangentReversal { station }) => panic!(
            "the anti-parallel arm now fires at station {station} — the frame gained a \
             real margin: update `sweep_geometry`'s C6 note and this row together"
        ),
        Err(e) => panic!("half-turn: unexpected refusal {e:?}"),
    }
}

/// A VANISHING tangent is the case the arm genuinely catches: a path
/// curve whose derivative is zero at a station has no frame at all.
#[test]
fn review_vanishing_tangent_refuses_typed() {
    use geom_core::spline::KnotVector;
    use sweep::skin::sweep_geometry;
    // A degree-3 curve with a triple control point at the start: the
    // derivative there is exactly the zero vector.
    let path = NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
        ],
        vec![1.0; 4],
    )
    .unwrap();
    let profile: sweep::Section = vec![profile::ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(0.3, 0.0),
        Point2::new(0.3, 0.2),
        Point2::new(0.0, 0.2),
    ])];
    match sweep_geometry(&profile, Affine3::identity(), &path, 3, 2) {
        Err(SkinError::PathTangentReversal { station: 0 }) => {}
        other => panic!("a vanishing tangent must refuse at station 0, got {other:?}"),
    }
}

/// **F2 evidence, REWORKED at LIB-U3.** The original probe fed the
/// door an OPEN chain beside a closed one and pinned the
/// `OpenClosedMixed` refusal. That refusal is retired WITH its input
/// vocabulary: a section is now a list of `ProfileLoop`s, and a
/// `ProfileLoop` closes by construction (the last vertex's segment
/// returns to the first), so a strip/tube mix is UNREPRESENTABLE at
/// the door — the signature is the proof, and the removed enum
/// variant means a resurrected open-chain door cannot come back
/// silently (this file would fail to compile).
///
/// What remains probe-able is what the old open chain's data BECOMES:
/// its dangling endpoint is just a fourth vertex, the loop closes
/// through it, and the pair lofts to a (closed) tube.
#[test]
fn review_open_chains_are_unrepresentable_and_close_by_construction() {
    // The old probe's open chain ran (0,0) -> (1,0) -> (1,1) -> (0.5,2)
    // and STOPPED. In the profile vocabulary the same points author a
    // closed quad — there is no open case left to mix.
    let was_open: sweep::Section = vec![profile::ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(0.5, 2.0),
    ])];
    let places = [
        Affine3::identity(),
        Affine3::translation(geom_core::Vec3::new(0.0, 0.0, 1.0)),
    ];
    let g = sweep::skin::loft_geometry(&[was_open.clone(), was_open], &places, 1)
        .expect("the closed-by-construction pair skins to a tube");
    assert_eq!(g.walls[0].len(), 4, "four vertices, four walls — closed");
}

/// **F5 evidence**: ragged column rows get a SHAPED refusal naming the
/// widths, not a parameterization sentence about counts that are not
/// counts.
#[test]
fn review_ragged_column_rows_get_a_shaped_refusal() {
    use geom::curves::fit::{FitError, interpolate_columns};
    let rows = vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![2.0]];
    match interpolate_columns(&[0.0, 0.5, 1.0], 2, &rows) {
        Err(FitError::RaggedRows {
            row: 2,
            width: 2,
            found: 1,
        }) => {}
        other => panic!("ragged rows must refuse RaggedRows, got {other:?}"),
    }
    let msg = FitError::RaggedRows {
        row: 2,
        width: 2,
        found: 1,
    }
    .to_string();
    assert!(msg.contains("wide"), "{msg}");
    assert!(!msg.contains("parameter"), "{msg}");
}

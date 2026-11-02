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
use geom_core::Tol;
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Band, Point2, Point3, Vec3};
use profile::SketchPlane;
use profile::{RawLoop, test_support::bulge_loop};
use sweep::skin::{SkinError, make_compatible, segment_curve, skin_on, skin_parameters};
use sweep::test_support::bulge_arc;

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
        let c = segment_curve(0, bulge_arc(a, b, bulge), Affine3::identity()).expect("converts");
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
    let kn = compat[0].knots_f64_interim().knots().to_vec();
    for (i, c) in compat.iter().enumerate() {
        assert_eq!(c.degree(), d, "section {i}");
        assert_eq!(c.knots_f64_interim().knots(), &kn[..], "section {i}");
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

/// The frame `path_start_frame` mints at `path`'s start.
fn start_plane(path: &NurbsCurve3<f64>) -> SketchPlane<f64> {
    let (lo, _) = path.domain();
    let (p, d) = path.ders1(lo);
    SketchPlane::from_frame(
        geom_core::linalg::frame::path_start_frame(p, d, Tol::witness()).expect("a start frame"),
    )
}

fn small_rectangle() -> sweep::Section {
    vec![profile::ProfileLoop::polygon([
        Point2::new(-0.15, -0.1),
        Point2::new(0.15, -0.1),
        Point2::new(0.15, 0.1),
        Point2::new(-0.15, 0.1),
    ])]
}

/// **A half-turn arc sweeps, its end frame the start frame turned a
/// half turn about the arc's axis.** The semicircle's end tangent is
/// its start tangent reversed, exactly. The frame is carried station
/// to station, so no station reads the start tangent against a distant
/// one, and on a planar path the carried frame turns about the plane's
/// normal by the tangent's own turn: at the last station `u` and `v`
/// are the start's negated in x and y, and the normal is unmoved. At
/// two stations the chord is normal to both tangents, the case where
/// the first reflection leaves the tangent where it was.
#[test]
fn a_half_turn_arc_sweeps_its_exact_tangents() {
    use sweep::skin::sweep_geometry;
    let path = segment_curve(
        0,
        bulge_arc(Point2::new(0.0, 0.0), Point2::new(0.0, 2.0), 1.0),
        Affine3::identity(),
    )
    .expect("path");
    let plane = start_plane(&path);
    let turned = |v: geom_core::Vec3<f64>| geom_core::Vec3::new(-v.x, -v.y, v.z);
    for stations in [2, 3, 5] {
        let places = sweep::sweep_places(plane, &path, stations, Tol::witness())
            .unwrap_or_else(|e| panic!("{stations} stations: the half turn carries: {e}"));
        let (start, end) = (plane.frame(), places[stations - 1].frame());
        for (axis, want, got) in [
            ("u", turned(start.u().get()), end.u().get()),
            ("v", turned(start.v().get()), end.v().get()),
            ("w", turned(start.w().get()), end.w().get()),
        ] {
            let off = (got - want).norm();
            assert!(
                off < 1e-14,
                "{stations} stations: the end frame's {axis} is the start's turned a half \
                 turn about z, off by {off:e}"
            );
        }
        let o = end.origin();
        assert!(
            o.x.abs() < 1e-15 && (o.y - 2.0).abs() < 1e-15 && o.z == 0.0,
            "{stations} stations: the end frame sits on the path's end point, at {o:?}"
        );
    }
    sweep_geometry(&small_rectangle(), plane, &path, 5, 2, Tol::witness())
        .expect("the half turn skins");
    sweep::sweep_body::<f64>(&small_rectangle(), plane, &path, 5, 2, Tol::witness())
        .expect("the half turn sweeps a body");
}

/// **A path that folds back across its chord between two stations
/// refuses there, and more stations sweep it.** The U-turn is a 270° arc over the top
/// and a 90° arc back down onto the line it left, as four rational
/// quarter arcs, so its start tangent is exactly `+z`, its end tangent
/// exactly `−z`, and its chord exactly along `−z`. At two stations the
/// start tangent mirrored in that chord is `−z`, the arrival itself,
/// so the step's second reflection has no plane. Sampled at nine
/// stations no step turns that far.
#[test]
fn a_half_turn_between_two_stations_refuses_and_more_stations_sweep_it() {
    let r = 1.0;
    let on = [(0.0, 0.0), (r, r), (2.0 * r, 0.0), (r, -r), (0.0, -2.0 * r)];
    let centres = [(r, 0.0), (r, 0.0), (r, 0.0), (r, -2.0 * r)];
    let mut ctrl = vec![on[0]];
    for k in 0..4 {
        let (a, b, c) = (on[k], on[k + 1], centres[k]);
        ctrl.push((a.0 + b.0 - c.0, a.1 + b.1 - c.1));
        ctrl.push(b);
    }
    let w = core::f64::consts::FRAC_1_SQRT_2;
    let path = NurbsCurve3::new(
        KnotVector::clamped(vec![0., 0., 0., 1., 1., 2., 2., 3., 3., 4., 4., 4.], 2).unwrap(),
        ctrl.into_iter()
            .map(|(x, z)| Point3::new(x, 0.0, z))
            .collect(),
        vec![1., w, 1., w, 1., w, 1., w, 1.],
    )
    .unwrap();
    let plane = start_plane(&path);
    let err =
        sweep::sweep_places(plane, &path, 2, Tol::witness()).expect_err("the step folds back");
    assert_eq!(err, SkinError::PathFoldsBack { station: 1 });
    let text = err.to_string();
    assert!(
        text.contains("between stations 0 and 1")
            && text.contains("Recourse: smooth the path's kink there, or add stations"),
        "the fold-back names its stations and both cures: {text}"
    );
    sweep::sweep_body::<f64>(&small_rectangle(), plane, &path, 9, 2, Tol::witness())
        .expect("at nine stations no step folds back");
}

/// **A hairpin kink folds back at every station count.** The degree-1
/// path runs out along `+x` to `(1, 0, 0)` and straight back to
/// `(0.3, 0, 0)`: whichever two stations straddle the corner, the one
/// before it leaves along the chord and the one after arrives against
/// it, which is the fold-back exactly. More stations only move the
/// step that straddles it, so the refusal is at the corner every time,
/// and the recourse names smoothing first.
#[test]
fn a_hairpin_kink_folds_back_at_every_station_count() {
    let path = NurbsCurve3::new(
        KnotVector::clamped(vec![0., 0., 1., 2., 2.], 1).unwrap(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.3, 0.0, 0.0),
        ],
        vec![1.0; 3],
    )
    .unwrap();
    let plane = start_plane(&path);
    for stations in [3, 4, 5, 8, 9, 16, 33, 64] {
        match sweep::sweep_places(plane, &path, stations, Tol::witness()) {
            Err(SkinError::PathFoldsBack { station }) => {
                #[allow(clippy::cast_precision_loss)]
                let (before, after) = (
                    2.0 * (station - 1) as f64 / (stations - 1) as f64,
                    2.0 * station as f64 / (stations - 1) as f64,
                );
                assert!(
                    before <= 1.0 && 1.0 <= after,
                    "{stations} stations: the fold-back is the step across the corner, \
                     not station {station}"
                );
            }
            other => panic!("{stations} stations: the hairpin folds back, got {other:?}"),
        }
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
    match sweep_geometry(&profile, SketchPlane::xy(), &path, 3, 2, Tol::witness()) {
        Err(SkinError::PathTangentVanishes { station: 0 }) => {}
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
        SketchPlane::xy(),
        SketchPlane::xy_at(Point3::new(0.0, 0.0, 1.0)),
    ];
    let sections = [was_open.clone(), was_open];
    let params = sweep::loft_parameters(&sections, &places, 1, Tol::witness())
        .expect("the closed-by-construction pair parameterizes");
    let g = sweep::skin::loft_geometry(&sections, &places, 1, &params, Tol::witness())
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

/// **A sub-tolerance arc lofts as the line validation classified it.**
/// A section whose first side carries the bulge 1e-13 — a sagitta of
/// 5e-14 on a unit chord, far inside the band — is a validated `Line`,
/// and the wall's section curve is that line: degree 1, its end
/// control points the placed vertices exactly. Built from the bulge
/// instead, the curve is the degree-2 arc of a carrier whose centre is
/// 2.5e12 away, whose end control points miss the vertices by 2.4e-6
/// and 2.5e-4.
#[test]
fn a_sub_tolerance_arc_lofts_as_a_line() {
    let section = || {
        vec![bulge_loop(vec![
            (Point2::new(0.0, 0.0), 1e-13),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ])]
    };
    let places = [
        SketchPlane::xy(),
        SketchPlane::xy_at(Point3::new(0.0, 0.0, 1.0)),
    ];
    let sections = [section(), section()];
    let params = sweep::loft_parameters(&sections, &places, 1, Tol::witness())
        .expect("the square parameterizes");
    let geometry = sweep::skin::loft_geometry(&sections, &places, 1, &params, Tol::witness())
        .expect("the square lofts");
    for (i, place) in places.iter().enumerate() {
        let curve = &geometry.sections[0][0][i];
        assert_eq!(curve.degree(), 1, "section {i}: {curve:?}");
        let control = curve.control();
        let want = [
            place.to_world(Point2::new(0.0, 0.0)),
            place.to_world(Point2::new(1.0, 0.0)),
        ];
        for (got, want) in [(control[0], want[0]), (control[control.len() - 1], want[1])] {
            assert_eq!(
                got.to_array().map(f64::to_bits),
                want.to_array().map(f64::to_bits),
                "section {i}: {got:?} vs {want:?}"
            );
        }
    }
}

/// **Each of a path sweep's three decisions has an in-band twin a path
/// reaches.** Every path is sized off the run's band `(ε, K·ε)`, so the
/// margin it hands its decision lands at `ε·√K`, inside the band at
/// every ε row:
///
/// - the tangent: a straight segment that long, whose derivative is its
///   length;
/// - the chord: a straight hairpin out to `(1, 0, 0)` and back to that
///   distance from its start, whose tangents are a unit apart but whose
///   two stations nearly coincide;
/// - the fold-back: a quadratic that rises along `+z` and comes back
///   down past its start, its middle pole tilted off the line by `δ` so
///   the arrival tangent misses the departure tangent's mirror by
///   `θ ≈ 1.5·δ`, a margin `1 − cos θ` of `ε·√K`. At `δ = 0` the same
///   path folds back exactly, and refuses as decided.
#[test]
fn each_path_decision_has_an_in_band_twin() {
    use sweep::PathDecision;
    let tol = Tol::witness();
    let in_band = tol.eps() * tol.k().sqrt();
    let line = |pts: &[Point3<f64>]| {
        let n = pts.len();
        let mut knots = vec![0.0];
        #[allow(clippy::cast_precision_loss)]
        knots.extend((0..n).map(|i| i as f64));
        #[allow(clippy::cast_precision_loss)]
        knots.push((n - 1) as f64);
        NurbsCurve3::new(
            KnotVector::clamped(knots, 1).unwrap(),
            pts.to_vec(),
            vec![1.0; n],
        )
        .unwrap()
    };
    let plane = SketchPlane::<f64>::xy();
    let which = |path: &NurbsCurve3<f64>| match sweep::sweep_places(plane, path, 2, tol) {
        Err(SkinError::PathUndecided { which, station, .. }) => Some((which, station)),
        other => panic!("an in-band decision escalates, got {other:?}"),
    };
    let tangent = line(&[Point3::origin(), Point3::new(in_band, 0.0, 0.0)]);
    assert_eq!(
        which(&tangent),
        Some((PathDecision::Tangent, 0)),
        "the tangent"
    );
    let chord = line(&[
        Point3::origin(),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(in_band, 0.0, 0.0),
    ]);
    assert_eq!(which(&chord), Some((PathDecision::Chord, 1)), "the chord");
    let fold = |delta: f64| {
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
            vec![
                Point3::origin(),
                Point3::new(delta, 0.0, 1.0),
                Point3::new(0.0, 0.0, -1.0),
            ],
            vec![1.0; 3],
        )
        .unwrap()
    };
    assert_eq!(
        sweep::sweep_places(plane, &fold(0.0), 2, tol).map(|_| ()),
        Err(SkinError::PathFoldsBack { station: 1 }),
        "the untilted hairpin folds back exactly"
    );
    let delta = (2.0 * in_band).sqrt() / 1.5;
    assert_eq!(
        which(&fold(delta)),
        Some((PathDecision::FoldBack, 1)),
        "the fold-back"
    );
    assert!(
        sweep::sweep_places(plane, &fold(1e3 * delta), 2, tol).is_ok(),
        "a tilt well past the band carries"
    );
}

/// A helix of `turns` revolutions about `z`, radius `r`, rising `pitch`
/// a turn, interpolated at degree 3 through `per_turn` exact points a
/// turn.
fn helix(turns: f64, per_turn: usize, r: f64, pitch: f64) -> NurbsCurve3<f64> {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    let n = (turns * per_turn as f64) as usize;
    let pts: Vec<Point3<f64>> = (0..=n)
        .map(|k| {
            #[allow(clippy::cast_precision_loss)]
            let th = core::f64::consts::TAU * k as f64 / per_turn as f64;
            Point3::new(
                r * th.cos(),
                r * th.sin(),
                pitch * th / core::f64::consts::TAU,
            )
        })
        .collect();
    NurbsCurve3::interpolate(&pts, 3).unwrap()
}

/// Wang, Jüttler, Zheng and Liu's double reflection (ACM TOG 2008,
/// Algorithm 1) as the paper writes it, two reflections of vectors and
/// nothing of the kernel's: `r0` carried to each of `n` uniform shares
/// of `path`'s parameter.
fn wang_double_reflection(path: &NurbsCurve3<f64>, n: usize, r0: Vec3<f64>) -> Vec<Vec3<f64>> {
    let (lo, hi) = path.domain();
    let at = |i: usize| {
        #[allow(clippy::cast_precision_loss)]
        let (p, d) = path.ders1((hi - lo).mul_add(i as f64 / (n - 1) as f64, lo));
        (p, d * (1.0 / d.norm()))
    };
    let reflect = |n: Vec3<f64>, x: Vec3<f64>| x - n * (2.0 * n.dot(x) / n.dot(n));
    let mut r = vec![r0];
    for i in 0..n - 1 {
        let ((p, t), (q, t1)) = (at(i), at(i + 1));
        let v1 = q - p;
        let v2 = t1 - reflect(v1, t);
        r.push(reflect(v2, reflect(v1, r[i])));
    }
    r
}

/// **The carry is the paper's double reflection**: every axis of the
/// start frame, carried along two turns of a helix at 64 stations,
/// agrees with Wang et al.'s algorithm run on that axis alone to
/// `1e-12` (measured 5.7e-15), and station 0 is the start plane's bits.
#[test]
fn the_carry_is_wangs_double_reflection_on_a_helix() {
    let path = helix(2.0, 512, 0.14, 0.08);
    let n = 64;
    let plane = start_plane(&path);
    let places = sweep::sweep_places(plane, &path, n, Tol::witness()).unwrap();
    let start = plane.frame();
    for (axis, k) in [("u", 0), ("v", 1), ("w", 2)] {
        let want = wang_double_reflection(&path, n, [start.u(), start.v(), start.w()][k].get());
        let worst = places
            .iter()
            .zip(&want)
            .map(|(pl, w)| {
                let f = pl.frame();
                ([f.u(), f.v(), f.w()][k].get() - *w).norm()
            })
            .fold(0.0, f64::max);
        assert!(
            worst < 1e-12,
            "axis {axis}: {worst:e} off the paper's carry"
        );
    }
    assert_eq!(
        places[0].placement().components().map(f64::to_bits),
        plane.placement().components().map(f64::to_bits),
        "station 0 is the start plane"
    );
}

/// **The sweep does not depend on where the sketch origin sits.** The
/// same square, written about a start frame moved `(0.37, −0.21)` in its
/// own plane and shifted back by that much, places every profile point
/// at the same world point at every station (to `1e-14`; measured
/// 2.5e-16), and the walls' poles agree (to `1e-13`; measured 6.9e-16):
/// the stations turn about the path's start point, not the sketch's.
#[test]
fn the_sweep_does_not_depend_on_where_the_sketch_origin_sits() {
    let path = helix(1.5, 64, 0.5, 0.9);
    let plane = start_plane(&path);
    let f = plane.frame();
    let (a, b) = (0.37, -0.21);
    let moved = SketchPlane::from_frame(f.translated(f.u().get() * a + f.v().get() * b).unwrap());
    let n = 40;
    let tol = Tol::witness();
    let here = sweep::sweep_places(plane, &path, n, tol).unwrap();
    let there = sweep::sweep_places(moved, &path, n, tol).unwrap();
    let mut worst = 0.0f64;
    for (x, y) in here.iter().zip(&there) {
        for q in [
            Point3::new(0.1, 0.05, 0.0),
            Point3::new(-0.2, 0.3, 0.0),
            Point3::origin(),
        ] {
            let w0 = x.frame().transform_point(q);
            let w1 = y
                .frame()
                .transform_point(Point3::new(q.x - a, q.y - b, q.z));
            worst = worst.max((w0 - w1).norm());
        }
    }
    assert!(
        worst < 1e-14,
        "a profile point moves {worst:e} with the sketch origin"
    );
    let square = |dx: f64, dy: f64| -> sweep::Section {
        vec![profile::ProfileLoop::polygon([
            Point2::new(-0.05 + dx, -0.04 + dy),
            Point2::new(0.05 + dx, -0.04 + dy),
            Point2::new(0.05 + dx, 0.04 + dy),
            Point2::new(-0.05 + dx, 0.04 + dy),
        ])]
    };
    let g0 = sweep::skin::sweep_geometry(&square(0.0, 0.0), plane, &path, n, 3, tol).unwrap();
    let g1 = sweep::skin::sweep_geometry(&square(-a, -b), moved, &path, n, 3, tol).unwrap();
    let poles = g0.walls[0]
        .iter()
        .zip(&g1.walls[0])
        .flat_map(|(w0, w1)| {
            w0.control()
                .iter()
                .zip(w1.control())
                .map(|(c0, c1)| (*c0 - *c1).norm())
                .collect::<Vec<_>>()
        })
        .fold(0.0, f64::max);
    assert!(
        poles < 1e-13,
        "a wall pole moves {poles:e} with the sketch origin"
    );
}

/// **A helix rolls by its torsion.** Against the helix's Frenet normal
/// the carried frame turns at `−τ` per unit of arc, so over two turns of
/// radius 0.14 and pitch 0.08 its end roll from the Frenet frame, plus
/// `τ·s`, is zero modulo a turn (to `1e-5`; measured 7.3e-11 at 257
/// stations).
#[test]
fn a_helix_rolls_by_its_torsion() {
    let (r, pitch, turns) = (0.14, 0.08, 2.0);
    let path = helix(turns, 1024, r, pitch);
    let n = 257;
    let plane = start_plane(&path);
    let places = sweep::sweep_places(plane, &path, n, Tol::witness()).unwrap();
    let c = pitch / core::f64::consts::TAU;
    let tangent = |th: f64| {
        let t = Vec3::new(-r * th.sin(), r * th.cos(), c);
        t * (1.0 / t.norm())
    };
    let normal = |th: f64| Vec3::new(-th.cos(), -th.sin(), 0.0);
    let angle = |a: Vec3<f64>, b: Vec3<f64>, t: Vec3<f64>| a.cross(b).dot(t).atan2(a.dot(b));
    let end = core::f64::consts::TAU * turns;
    let from_frenet_at_start = angle(normal(0.0), plane.frame().u().get(), tangent(0.0));
    let from_frenet_at_end = angle(normal(end), places[n - 1].frame().u().get(), tangent(end));
    let torsion = c / (r * r + c * c);
    let arc = end * (r * r + c * c).sqrt();
    let roll = (from_frenet_at_end - from_frenet_at_start + torsion * arc)
        .rem_euclid(core::f64::consts::TAU);
    let off = roll.min(core::f64::consts::TAU - roll);
    assert!(
        off < 1e-5,
        "the end frame is {off:e} rad off the torsion's roll"
    );
}

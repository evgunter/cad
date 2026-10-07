//! **One door for two loft sections that are not apart.** The loft's
//! stacking fold decides it, once, under the run's band, before the
//! skin parameterizes anything: a sliver slab and an exactly
//! coincident pair both refuse as `LoftError::DegenerateStacking`
//! naming the slab. The skin's own `SkinError::NoParameterStep` is a
//! different fact — the chord-length parameterization has no `f64`
//! step — and through `loft_body` it is reachable only for sections
//! the fold found definitely apart, so its text never says the
//! sections coincide.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsCurve3;
use geom_core::linalg::frame::path_start_frame;
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Interval, Point2, Point3, RANGE_RECOURSE, Tol, Vec3};
use profile::RawLoop;
use sweep::test_support::{loft_prism_sections, stacked_at};
use sweep::{
    LoftError, ProfileLoop, SkinError, loft_body, loft_geometry, loft_parameters, sweep_body,
    sweep_geometry,
};

fn squares(n: usize) -> Vec<sweep::Section> {
    vec![loft_prism_sections()[0].clone(); n]
}

/// Four squares whose middle slab steps by exactly `step`: its base
/// sits at z = 0, so the fold's margin for slab 1 is `step` bit for
/// bit, while the slab's NORMALISED chord step is `step / 2`.
fn middle_step(step: f64) -> Vec<Affine3<f64>> {
    stacked_at(&[-1.0, 0.0, step, 1.0])
}

/// The chord steps BOOL-6 measured (5e-10, 1e-12, 1e-15, and 1e-16,
/// where the normalised accumulation underflows), then the sub-ulp
/// tail down to exact coincidence. Each rung is the fold's, with the
/// verdict the run's band gives its margin: at the default ε every
/// rung refuses; at ε = 1e-12 the 5e-10 rung is definitely apart and
/// builds.
#[test]
fn a_slab_short_of_the_band_refuses_at_the_fold_at_every_scale() {
    let tol = Tol::witness();
    let (eps, k) = (tol.eps(), tol.k());
    for step in [5e-10, 1e-12, 1e-15, 1e-16, 1e-17, 1e-300, 0.0] {
        let out = loft_body::<f64>(&squares(4), &middle_step(step), 2, tol).map(|_| ());
        if step <= eps {
            match out {
                Err(e @ LoftError::DegenerateStacking { slab: 1 }) => {
                    let msg = e.to_string();
                    assert!(
                        msg.contains(
                            "sections 1 and 2 are not apart along section 1's normal at tolerance"
                        ),
                        "step {step:e}: the refusal names the pair and what it decided: {msg}"
                    );
                }
                other => panic!(
                    "step {step:e} (ε = {eps:e}): expected the fold's DegenerateStacking \
                     naming slab 1, got {other:?}"
                ),
            }
        } else if step >= k * eps {
            assert!(
                out.is_ok(),
                "step {step:e} (ε = {eps:e}): definitely apart, expected a build, got {out:?}"
            );
        } else {
            match out {
                Err(LoftError::StackingEscalated { slab: 1, source }) => {
                    assert_eq!(source.predicate, Some("loft_stacking"), "step {step:e}");
                }
                other => panic!(
                    "step {step:e} (ε = {eps:e}): expected StackingEscalated naming slab 1, \
                     got {other:?}"
                ),
            }
        }
    }
}

/// **The underflow rungs are real.** From 1e-16 down, the skin's
/// chord-length parameters for the same placements do not ascend —
/// so the row above sees the fold's refusal on those rungs because the
/// fold runs first, not because the skin happened to pass them. At
/// 1e-15 the parameters still step, which is the measured hand-off.
#[test]
fn below_1e_16_the_parameterization_has_no_step_on_the_same_placements() {
    let tol = Tol::witness();
    assert!(
        loft_parameters(&squares(4), &middle_step(1e-15), 2, tol).is_ok(),
        "a 1e-15 chord step still ascends in f64"
    );
    for step in [1e-16, 1e-17, 0.0] {
        match loft_parameters(&squares(4), &middle_step(step), 2, tol) {
            Err(SkinError::NoParameterStep { section: 2 }) => {}
            other => panic!("step {step:e}: expected NoParameterStep at section 2, got {other:?}"),
        }
    }
}

/// **Apart, but stepless: the skin's refusal, told truly.** Two shapes
/// the fold accepts and the chord-length parameterization cannot step:
/// a middle slab 1 mm thick (above K·ε at every ε row) under a first
/// slab 1e14 m tall, whose accumulated chord swallows it, and a
/// section hinged on the edge the parameterization is measured on.
/// Both refuse as the skin's `NoParameterStep` naming the section, and
/// neither text claims the sections coincide — they do not.
#[test]
fn sections_apart_whose_parameterization_cannot_step_refuse_without_claiming_coincidence() {
    let tol = Tol::witness();
    let thin = 1e-3;
    assert!(
        thin > tol.k() * tol.eps(),
        "the thin slab is definitely apart"
    );
    let tall = stacked_at(&[-1e14, 0.0, thin, 1.0]);
    let hinged = vec![
        Affine3::identity(),
        Affine3::rotation_about_axis(Point3::new(0.0, -1.0, 0.0), Vec3::new(1.0, 0.0, 0.0), 0.3),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    for (what, sections, places, at) in [
        ("a 1 mm slab under 1e14 m", squares(4), tall, 2),
        ("hinged on the first strip", squares(3), hinged, 1),
    ] {
        match loft_body::<f64>(&sections, &places, 2, tol) {
            Err(LoftError::Skin(e @ SkinError::NoParameterStep { section })) => {
                assert_eq!(section, at, "{what}: the stepless section is named");
                let msg = e.to_string();
                assert!(
                    msg.contains(&format!(
                        "Recourse: move section {at} away from section {}",
                        at - 1
                    )),
                    "{what}: the one lever is moving the named section: {msg}"
                );
                assert!(
                    !msg.contains(RANGE_RECOURSE),
                    "{what}: no uniform scale changes the step's ratio to the travel: {msg}"
                );
                assert!(
                    !msg.contains("coincide"),
                    "{what}: the sections are apart, and the text must not say otherwise: {msg}"
                );
            }
            other => panic!("{what}: expected the skin's NoParameterStep, got {other:?}"),
        }
    }
}

/// **A sweep whose two stations coincide refuses at the fold**, at
/// `f64` and at `Interval`. The path is closed, C¹ and non-planar, so
/// with two stations its start and end frames are bit-equal; the
/// geometry door, which has no fold, says only what the skin can:
/// `NoParameterStep` at section 1.
#[test]
fn a_sweep_whose_two_stations_coincide_refuses_at_the_fold() {
    let tol = Tol::witness();
    let path = NurbsCurve3::new(
        KnotVector::clamped(
            vec![0.0, 0.0, 0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0, 1.0, 1.0],
            3,
        )
        .unwrap(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(2.0, 2.0, 1.0),
            Point3::new(-1.0, 2.0, -1.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
        ],
        vec![1.0; 6],
    )
    .unwrap();
    let place = path_start_frame(path.eval(0.0), path.deriv(0.0), tol).unwrap();
    let profile = vec![ProfileLoop::polygon(
        [(-0.1, -0.1), (0.1, -0.1), (0.1, 0.1), (-0.1, 0.1)]
            .iter()
            .map(|&(x, y)| Point2::new(x, y)),
    )];
    let stations = sweep::sweep_places(place, &path, 2).unwrap();
    assert_eq!(
        stations[0].components().map(f64::to_bits),
        stations[1].components().map(f64::to_bits),
        "the two stations are the same frame, bit for bit"
    );
    match sweep_body::<f64>(&profile, place, &path, 2, 1, tol) {
        Err(LoftError::DegenerateStacking { slab: 0 }) => {}
        other => panic!("f64: expected DegenerateStacking at slab 0, got {other:?}"),
    }
    match sweep_body::<Interval>(&profile, place, &path, 2, 1, tol) {
        Err(LoftError::DegenerateStacking { slab: 0 }) => {}
        other => panic!("Interval: expected DegenerateStacking at slab 0, got {other:?}"),
    }
    match sweep_geometry(&profile, place, &path, 2, 1, tol) {
        Err(SkinError::NoParameterStep { section: 1 }) => {}
        other => panic!("sweep_geometry: expected NoParameterStep at section 1, got {other:?}"),
    }
}

/// **A hinge pinned only up to rounding passes the exact ascent check**
/// — the measured state of
/// `work/carve/a-wall-pinned-between-two-loft-sections-refuses-at-the-wrong-door.md`,
/// pinned as it is. Under a generic placement the strip-0 hinge moves
/// by rounding alone, so the parameters ascend by about 1e-16,
/// `loft_geometry` answers `Ok` with control coordinates past 1e12 for
/// a 2-unit section, and `loft_body` refuses at the attach gate. This
/// row goes red when that item is fixed; rewrite it to the fix's
/// refusal then.
#[test]
fn a_hinge_pinned_only_up_to_rounding_passes_the_exact_ascent_check() {
    let tol = Tol::witness();
    let sections = squares(3);
    let pp = Affine3::translation(Vec3::new(0.37, -0.21, 0.13))
        * Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.3, 0.4, 0.866), 0.7);
    let (a, b) = (
        pp.transform_point(Point3::new(-1.0, -1.0, 0.0)),
        pp.transform_point(Point3::new(1.0, -1.0, 0.0)),
    );
    let n = pp.linear.c2;
    let places = vec![
        pp,
        Affine3::rotation_about_axis(a, b - a, 0.3) * pp,
        Affine3::translation(Vec3::new(2.0 * n.x, 2.0 * n.y, 2.0 * n.z)) * pp,
    ];
    let g = loft_geometry(&sections, &places, 2, tol).expect("the exact ascent check passes");
    assert!(
        g.section_params[1] < 1e-15,
        "the hinge steps by rounding alone: {:?}",
        g.section_params
    );
    let worst = g
        .walls
        .iter()
        .flatten()
        .flat_map(|w| {
            w.control()
                .iter()
                .map(|q| q.x.abs().max(q.y.abs()).max(q.z.abs()))
        })
        .fold(0.0f64, f64::max);
    assert!(
        worst > 1e12,
        "the walls blow up: largest control coordinate {worst:e}"
    );
    match loft_body::<f64>(&sections, &places, 2, tol) {
        Err(LoftError::Euler(_)) => {}
        other => panic!("expected the attach gate's refusal, got {other:?}"),
    }
}

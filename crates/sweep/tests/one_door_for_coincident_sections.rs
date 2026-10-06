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

use geom_core::{Affine3, Point3, RANGE_RECOURSE, Tol, Vec3};
use sweep::test_support::{loft_prism_sections, stacked_at};
use sweep::{LoftError, SkinError, loft_body, loft_parameters};

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
                        msg.contains("sections 1 and 2 are not apart at tolerance"),
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
                    msg.contains(RANGE_RECOURSE),
                    "{what}: the range lever is offered: {msg}"
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

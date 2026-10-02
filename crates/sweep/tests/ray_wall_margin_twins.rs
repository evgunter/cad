//! **The ray caster's wall arm scales with the model.** The same pipe
//! at a millimetre and at a metre, the same probes scaled with it:
//! every predicate `point_in_solid` decides must fire the same number of
//! times with the same verdicts, and every decisive margin must be the
//! metre twin's over a thousand. The wall arm's two rungs are what this
//! suite exists for — `bool_point_in_solid_denom`'s axis-parallel rung
//! (`|d⊥|` levered by the selection's reach) and
//! `bool_ray_cylinder_disc` (the half-chord's depth) — and both are
//! asserted to FIRE, so the pin cannot go vacuous. A planar-only body
//! (`topo`'s `rim_dim_boolean_twins`) never reaches either, nor the
//! face boxes' cylinder axis read (`bool_box_cylinder_axis`), which the
//! same pipe boring a plate pins beside them.
//!
//! **CI EXECUTES THIS SUITE**: it is rostered in
//! `scripts/gates/probe-suite-census.sh` (`RUN_FLOOR`). By hand:
//! `cargo test -p sweep --features probe --test all -- ray_wall_margin_twins::`.

#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use geom_core::k_stats::{self, Probe, SampleOutcome};
use geom_core::{Affine3, Band, Point2, Point3, Sign, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::point_in_solid;

/// The predicates whose firing makes this pin a claim about the wall.
const WALL_RUNGS: [&str; 2] = ["bool_point_in_solid_denom", "bool_ray_cylinder_disc"];

/// A pipe of radius `scale` about `z`, `z ∈ [−2, 2]·scale`, and probes
/// inside, outside and beyond its caps, all scaled with it: the
/// `(predicate, (outcome, |margin|))` stream `point_in_solid` records
/// over them, and their verdicts.
#[allow(clippy::type_complexity)] // the stream, then the verdicts
fn margins_at(
    scale: f64,
) -> (
    BTreeMap<&'static str, Vec<(SampleOutcome, f64)>>,
    Vec<topo::SolidContainment>,
) {
    let tol = Tol::witness();
    let s = |v: f64| Probe(v * scale);
    let lp = profile::circle(Point2::new(s(0.0), s(0.0)), s(1.0), tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(s(0.0), s(0.0), s(-2.0))));
    let vp = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    let body = extrude(&vp, Extrusion::Distance(s(4.0)), tol).unwrap().body;
    let band = Band::linear(tol).unwrap();
    let probes = [
        (0.25, 0.125, 0.5),
        (-0.5, 0.375, -1.25),
        (3.0, 0.5, 0.25),
        (0.5, -2.5, 1.5),
        (0.25, 0.25, 3.0),
    ];
    k_stats::start_recording();
    let verdicts = probes
        .iter()
        .map(|&(x, y, z)| {
            point_in_solid(&body, Point3::new(s(x), s(y), s(z)), band, tol)
                .unwrap_or_else(|e| panic!("({x}, {y}, {z}) at scale {scale:e}: {e:?}"))
        })
        .collect();
    let mut out: BTreeMap<&'static str, Vec<(SampleOutcome, f64)>> = BTreeMap::new();
    for sample in k_stats::take_samples() {
        out.entry(sample.predicate)
            .or_default()
            .push((sample.outcome, sample.margin.abs()));
    }
    (out, verdicts)
}

#[test]
fn the_wall_arms_margins_scale_linearly_with_the_model() {
    let ((mm, mm_verdicts), (m, m_verdicts)) = (margins_at(1e-3), margins_at(1.0));
    assert_eq!(mm_verdicts, m_verdicts, "the twins' verdicts");
    for rung in WALL_RUNGS {
        assert!(
            m.contains_key(rung),
            "{rung} must fire on the pipe, or this pin is vacuous: {:?}",
            m.keys().collect::<Vec<_>>()
        );
    }
    assert_eq!(
        mm.keys().collect::<Vec<_>>(),
        m.keys().collect::<Vec<_>>(),
        "the twins must fire the same predicate set"
    );
    let mut nonlinear = Vec::new();
    for (pred, mm_list) in &mm {
        let m_list = &m[pred];
        let outcomes = |l: &[(SampleOutcome, f64)]| l.iter().map(|s| s.0).collect::<Vec<_>>();
        assert_eq!(
            outcomes(mm_list),
            outcomes(m_list),
            "{pred}: the twins must produce identical verdict streams"
        );
        // Decisive margins only, sorted: a coincident residual is
        // rounding noise, and recording order pairs unrelated sites.
        let decisive = |l: &[(SampleOutcome, f64)]| {
            let mut v: Vec<f64> = l
                .iter()
                .filter(|s| {
                    matches!(
                        s.0,
                        SampleOutcome::Definite(Sign::Positive | Sign::Negative)
                    )
                })
                .map(|s| s.1)
                .collect();
            v.sort_by(f64::total_cmp);
            v
        };
        // Not `dev > 1e-9`: a NaN ratio is a deviation too.
        let off: Vec<f64> = decisive(mm_list)
            .iter()
            .zip(&decisive(m_list))
            .map(|(a, b)| (b / a / 1e3 - 1.0).abs())
            .filter(|dev| dev.is_nan() || *dev > 1e-9)
            .collect();
        if !off.is_empty() {
            nonlinear.push(format!(
                "{pred}: {} pairs off, e.g. {:.3e}",
                off.len(),
                off[0]
            ));
        }
    }
    assert!(
        nonlinear.is_empty(),
        "predicates whose margins do not scale with the model: {nonlinear:?}"
    );
}

/// The `bool_box_cylinder_axis` margins a subtract records when a pipe
/// of radius `scale` (`z ∈ [−2, 2]·scale`) bores a `4 × 4 × 1` plate,
/// everything scaled with it.
fn box_axis_margins(scale: f64) -> Vec<f64> {
    let tol = Tol::witness();
    let s = |v: f64| Probe(v * scale);
    let at = |z: f64| SketchPlane::new(Affine3::translation(Vec3::new(s(0.0), s(0.0), s(z))));
    let pipe = Profile::new(
        at(-2.0),
        vec![
            profile::circle(Point2::new(s(0.0), s(0.0)), s(1.0), tol)
                .unwrap()
                .into(),
        ],
    )
    .validate(tol)
    .unwrap();
    let pipe = extrude(&pipe, Extrusion::Distance(s(4.0)), tol)
        .unwrap()
        .body;
    let corners = [(-2.0, -2.0), (2.0, -2.0), (2.0, 2.0), (-2.0, 2.0)]
        .map(|(x, y)| (Point2::new(s(x), s(y)), Probe(0.0)));
    let plate = Profile::new(
        at(-0.5),
        vec![profile::test_support::bulge_loop(corners.to_vec())],
    )
    .validate(tol)
    .unwrap();
    let plate = extrude(&plate, Extrusion::Distance(s(1.0)), tol)
        .unwrap()
        .body;
    k_stats::start_recording();
    topo::subtract(&plate, &pipe, tol).expect("the pipe bores the plate");
    k_stats::take_samples()
        .iter()
        .filter(|s| s.predicate == "bool_box_cylinder_axis")
        .map(|s| s.margin)
        .collect()
}

/// **A cylinder face's axis length is decided at the model's scale.**
/// The axis is unit at rest, a pure number; levered by the radius its
/// slab swings it by, every margin is `scale`, and so `1e3` apart
/// between the twins. The bare norm reads `1` at both.
#[test]
fn the_box_cylinder_axis_margin_is_the_radius_at_both_scales() {
    for scale in [1e-3, 1.0] {
        let margins = box_axis_margins(scale);
        assert!(
            !margins.is_empty(),
            "the bore boxes the pipe's wall at {scale:e}"
        );
        for m in &margins {
            assert!(
                ((m - scale) / scale).abs() < 1e-12,
                "at {scale:e} an axis margin {m:e} is not the radius: {margins:?}"
            );
        }
    }
}

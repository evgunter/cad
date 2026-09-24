//! Review probes for the per-slab stacking fold (BOOL-6, issue 368).
//!
//! Additive rows written against the frozen head. Each row names the
//! claim it attacks; none is a spec row.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats::Bracket;
use geom_core::{Affine3, Mat3, Point2, Tol, Vec3};
use profile::RawLoop;
use sweep::ProfileLoop;
use sweep::test_support::{loft_prism_sections, stacked_at};
use sweep::{LoftError, SkinError, loft_body};

use crate::common;
use common::quad;

fn four_squares() -> Vec<sweep::Section> {
    vec![loft_prism_sections()[0].clone(); 4]
}

/// A 4-section stack whose middle slab steps by EXACTLY `step`: the
/// slab's base sits at z = 0 so `(0 + step) - 0` is `step` bit for
/// bit, the four identical displacements sum to `4·step` exactly and
/// divide back to `step` exactly. The fold's margin IS `step`.
fn middle_step(step: f64) -> Result<(), LoftError> {
    let places = stacked_at(&[-1.0, 0.0, step, 1.0]);
    loft_body::<f64>(&four_squares(), &places, 2, Tol::witness()).map(|_| ())
}

fn next_up(x: f64) -> f64 {
    f64::from_bits(x.to_bits() + 1)
}
fn next_down(x: f64) -> f64 {
    f64::from_bits(x.to_bits() - 1)
}

/// **Claim 2 attack: one slab exactly at each band edge.** The band is
/// `|m| ≤ ε ⇒ Zero`, `|m| ≥ K·ε ⇒ definite`, strictly between ⇒
/// escalate. Every edge and both its neighbours, at the run's own ε.
#[test]
fn r2_middle_slab_at_every_band_edge_at_the_run_band() {
    let tol = Tol::witness();
    let (eps, k) = (tol.eps(), tol.k());
    let zero_edge = eps;
    let escalate_edge = k * eps;
    let degenerate = |s: f64| match middle_step(s) {
        Err(LoftError::DegenerateStacking { slab }) => assert_eq!(slab, 1, "step {s:e}"),
        other => panic!("step {s:e}: expected DegenerateStacking slab 1, got {other:?}"),
    };
    let escalates = |s: f64| match middle_step(s) {
        Err(LoftError::StackingEscalated { slab, source }) => {
            assert_eq!(slab, 1, "step {s:e}");
            assert_eq!(source.predicate, Some("loft_stacking"), "step {s:e}");
        }
        other => panic!("step {s:e}: expected StackingEscalated slab 1, got {other:?}"),
    };
    let builds = |s: f64| match middle_step(s) {
        Ok(()) => {}
        other => panic!("step {s:e}: expected a build, got {other:?}"),
    };
    degenerate(0.5 * zero_edge);
    degenerate(next_down(zero_edge));
    degenerate(zero_edge);
    escalates(next_up(zero_edge));
    escalates(0.5 * (zero_edge + escalate_edge));
    escalates(next_down(escalate_edge));
    builds(escalate_edge);
    builds(next_up(escalate_edge));
    builds(2.0 * escalate_edge);
    // The same edges reversed: the sign is read off the margin, the
    // magnitude off the band.
    match middle_step(-escalate_edge) {
        Err(LoftError::ReversedStacking { slab }) => assert_eq!(slab, 1),
        other => panic!("expected ReversedStacking slab 1, got {other:?}"),
    }
    escalates(-next_down(escalate_edge));
    degenerate(-zero_edge);
}

/// The same edges at the certified scalar. The margin is not an exact
/// interval there (the lift and the world transform round outward),
/// so the exact edges are RECORDED rather than pinned, and only the
/// off-edge steps are asserted to agree with the f64 lane.
#[test]
fn r2_middle_slab_band_edges_at_interval() {
    use geom_core::Interval;
    let tol = Tol::witness();
    let (eps, k) = (tol.eps(), tol.k());
    let run = |s: f64| {
        loft_body::<Interval>(&four_squares(), &stacked_at(&[-1.0, 0.0, s, 1.0]), 2, tol)
            .map(|_| ())
    };
    for (name, s) in [
        ("0.5eps", 0.5 * eps),
        ("eps", eps),
        ("next_up(eps)", next_up(eps)),
        ("mid", 0.5 * (eps + k * eps)),
        ("next_down(K eps)", next_down(k * eps)),
        ("K eps", k * eps),
        ("2 K eps", 2.0 * k * eps),
        ("-K eps", -(k * eps)),
    ] {
        println!("R2 interval step {name} = {s:e}: {:?}", run(s));
    }
    assert!(matches!(
        run(0.5 * eps),
        Err(LoftError::DegenerateStacking { slab: 1 })
    ));
    assert!(matches!(
        run(0.5 * (eps + k * eps)),
        Err(LoftError::StackingEscalated { slab: 1, .. })
    ));
    assert!(run(2.0 * k * eps).is_ok());
    assert!(matches!(
        run(-2.0 * k * eps),
        Err(LoftError::ReversedStacking { slab: 1 })
    ));
}

/// **Claim 2 attack: the top loop's vertex ORDER rotated relative to
/// the base's.** A 2×1 rectangle lofted onto the same rectangle turned
/// a quarter turn in its own plane: the canonical lex-min start is a
/// DIFFERENT geometric corner, so the by-index pairing shifts by one
/// vertex. The mean displacement is the centroid displacement whatever
/// the pairing (a sum of differences telescopes), so the verdict is
/// the centroid's: builds at 1, sliver at ε/2, reversed at −1 — the
/// same three verdicts the un-rotated stack gives.
#[test]
fn r2_a_rotated_top_loop_still_measures_the_centroid() {
    let rect = quad([(-1.0, -0.5), (1.0, -0.5), (1.0, 0.5), (-1.0, 0.5)]);
    let turned = quad([(0.5, -1.0), (0.5, 1.0), (-0.5, 1.0), (-0.5, -1.0)]);
    let sections = vec![rect, turned];
    let at = |z: f64| stacked_at(&[0.0, z]);
    let tol = Tol::witness();
    let a = loft_body::<f64>(&sections, &at(1.0), 1, tol);
    assert!(a.is_ok(), "turned quarter: {:?}", a.err());
    assert!(matches!(
        loft_body::<f64>(&sections, &at(0.5 * tol.eps()), 1, tol),
        Err(LoftError::DegenerateStacking { slab: 0 })
    ));
    assert!(matches!(
        loft_body::<f64>(&sections, &at(-1.0), 1, tol),
        Err(LoftError::ReversedStacking { slab: 0 })
    ));
    // And a 45° turn of a square (a twisted loft), three sections.
    let h = core::f64::consts::FRAC_1_SQRT_2;
    let sq = quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]);
    let diamond = quad([
        (0.0, -h * 2.0),
        (h * 2.0, 0.0),
        (0.0, h * 2.0),
        (-h * 2.0, 0.0),
    ]);
    let twisted = vec![sq.clone(), diamond, sq];
    let t = loft_body::<f64>(&twisted, &stacked_at(&[0.0, 1.0, 2.0]), 2, tol);
    assert!(t.is_ok(), "twisted: {:?}", t.err());
}

/// **Claim 2 attack: sections with different vertex counts.** The
/// skin's door refuses them by index before the fold can pair
/// anything; the fold's own `next.len() != base.len()` arm is not
/// reachable through `loft_body`.
#[test]
fn r2_differing_vertex_counts_are_the_skins_refusal() {
    let pent = vec![ProfileLoop::polygon(
        [
            (1.0, 0.0),
            (0.3, 0.95),
            (-0.8, 0.59),
            (-0.8, -0.59),
            (0.3, -0.95),
        ]
        .iter()
        .map(|&(x, y)| Point2::new(x, y)),
    )];
    let sections = vec![loft_prism_sections()[0].clone(), pent];
    match loft_body::<f64>(&sections, &stacked_at(&[0.0, 1.0]), 1, Tol::witness()) {
        Err(LoftError::Skin(SkinError::SectionShapeMismatch { section, .. })) => {
            assert_eq!(section, 1);
        }
        other => panic!("expected the skin's shape refusal, got {other:?}"),
    }
}

/// **Claim 8: the skin's unbanded `params[j-1] < params[j]`.** A
/// sub-ulp step (1e-17 beside chords of 1) collapses to equal
/// parameters and the SKIN refuses; a step below ε but above the ulp
/// reaches the fold, which names the sliver slab. So the PR's
/// measurement holds: exactly-coincident-in-parameter is the skin's,
/// the band-scale sliver is the fold's.
#[test]
fn r2_sub_ulp_slivers_are_the_skins_and_band_slivers_are_the_folds() {
    match middle_step(1e-17) {
        Err(LoftError::Skin(SkinError::DegenerateSection { section, .. })) => {
            assert_eq!(section, 2);
        }
        other => panic!("expected the skin's DegenerateSection, got {other:?}"),
    }
    let eps = Tol::witness().eps();
    for step in [1e-14, 1e-13, 0.1 * eps] {
        match middle_step(step) {
            Err(LoftError::DegenerateStacking { slab }) => assert_eq!(slab, 1),
            other => panic!("step {step:e}: expected DegenerateStacking slab 1, got {other:?}"),
        }
    }
}

/// **Claim 6: the fold mints `k − 1` `loft_stacking` samples per
/// loft** — and stops at the first non-positive slab, so a refusal at
/// slab 1 of four sections leaves TWO verdicts, and an escalation
/// leaves one verdict plus one escalation.
#[test]
fn r2_the_fold_mints_k_minus_one_samples_and_stops_at_the_first_refusal() {
    let count = |run: &dyn Fn()| {
        let b = Bracket::open();
        run();
        let rec = b.finish();
        let v = rec
            .verdicts
            .iter()
            .filter(|v| v.predicate == "loft_stacking")
            .count();
        let e = rec
            .escalations
            .iter()
            .filter(|e| e.source.predicate == Some("loft_stacking"))
            .count();
        (v, e)
    };
    let tol = Tol::witness();
    let five = vec![loft_prism_sections()[0].clone(); 5];
    assert_eq!(
        count(&|| {
            loft_body::<f64>(&five, &stacked_at(&[0.0, 1.0, 2.0, 3.0, 4.0]), 2, tol).unwrap();
        }),
        (4, 0)
    );
    assert_eq!(
        count(&|| {
            loft_body::<f64>(&five[..2], &stacked_at(&[0.0, 1.0]), 1, tol).unwrap();
        }),
        (1, 0)
    );
    assert_eq!(
        count(&|| {
            let _ = loft_body::<f64>(&five[..4], &stacked_at(&[0.0, 1.0, 0.5, 2.0]), 2, tol);
        }),
        (2, 0)
    );
    let mid = 0.5 * (1.0 + tol.k()) * tol.eps();
    assert_eq!(
        count(&|| {
            let _ = loft_body::<f64>(&five[..4], &stacked_at(&[-1.0, 0.0, mid, 1.0]), 2, tol);
        }),
        (1, 1)
    );
}

/// The suite's arc spine, re-spelled here so this file stands alone.
fn arc_spine_places(curl: f64, stations: usize) -> Vec<Affine3<f64>> {
    (0..stations)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let theta = curl * (i as f64) / ((stations - 1) as f64);
            let (s, c) = theta.sin_cos();
            let linear = Mat3::from_cols(
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(-s, 0.0, c),
                Vec3::new(c, 0.0, s),
            );
            Affine3::from_parts(linear, Vec3::new(s, 0.0, 1.0 - c))
        })
        .collect()
}

/// **What the fold newly admits.** A unit-radius circular spine past a
/// full turn returns through its own start: the suite's own
/// `2.2π / 17 stations` row lofts a tube that passes through itself.
/// This row records what the tiers say about that body — the fold is
/// local by ruling, so this is not the fold's to refuse, but the PR
/// presents 9.0 and 13.0 rad as plain wins and nothing downstream
/// says otherwise.
#[test]
fn r2_a_spine_past_a_full_turn_builds_a_self_overlapping_body() {
    let sections = vec![quad([(-0.05, -0.05), (0.05, -0.05), (0.05, 0.05), (-0.05, 0.05)]); 17];
    let tol = Tol::witness();
    let curl = 2.2 * core::f64::consts::PI;
    let built = loft_body::<f64>(&sections, &arc_spine_places(curl, 17), 3, tol)
        .expect("the suite's own row says this builds");
    let g = topo::validate_geometric(&built.body, tol);
    println!("R2 self-overlap: tier-3 says {g:?}");
    assert!(topo::validate(&built.body).is_ok());
    assert!(topo::validate_closed(&built.body).is_ok());
}

/// **Claim 3 attack: the LAST section's normal is what the top cap
/// reads, and nothing in the fold checks it agrees with the
/// stacking.** A two-section loft whose top section sits above the
/// base but is placed with its plane normal pointing DOWN: the fold
/// reads `d · n_0 = +1` and builds. What the body then is, the tiers
/// say below; the row records it rather than pinning it.
#[test]
fn r2_a_flipped_last_normal_is_not_the_folds_to_see() {
    let sections = vec![loft_prism_sections()[0].clone(); 2];
    let flipped = Affine3::from_parts(
        Mat3::from_cols(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
        ),
        Vec3::new(0.0, 0.0, 1.0),
    );
    let tol = Tol::witness();
    let out = loft_body::<f64>(&sections, &[Affine3::identity(), flipped], 1, tol);
    match out {
        Ok(l) => {
            println!(
                "R2 flipped top normal: BUILT; tier-3 says {:?}",
                topo::validate_geometric(&l.body, tol)
            );
        }
        Err(e) => println!("R2 flipped top normal: refused {e:?}"),
    }
}

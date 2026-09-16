//! **BOOL-6 review probes (R1)** — executed checks of the per-slab
//! stacking fold, written by a reviewer and not by the implementing
//! lane.
//!
//! What these rows add to `bool6_per_slab_stacking.rs` is the
//! POPULATION and the EDGES: how many `loft_stacking` samples one loft
//! now mints, whether the slab margin is the step itself at the band's
//! two edges, whether the fold's vertex PAIRING carries any
//! information, what a pair of sections with different vertex counts
//! reaches, and where the sliver hands off between the skin's unbanded
//! coincidence check and the loft's banded degenerate arm.
//!
//! Two of these rows are not here: `a_spine_curled_past_pi_still_faces_out_everywhere`
//! and `a_curl_past_a_full_turn_builds_a_spine_that_revisits_itself`
//! were taken into the unit's own suite at the fix pass, on the
//! ruling that the accept side's orientation and the self-overlap the
//! fold newly reaches are acceptance facts and not review probes.
//! They are marked as adopted from here at their new home.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats::Bracket;
use geom_core::predicate::{Band, Margin, Sign};
use geom_core::{Affine3, Mat3, Point2, Tol, Vec3};
use profile::RawLoop;
use sweep::test_support::{loft_prism_sections, stacked_at};
use sweep::{LoftError, SkinError, loft_body};

use crate::common;
use common::quad;

/// A small square section, far smaller than any spine radius here.
fn sq() -> sweep::Section {
    quad([(-0.05, -0.05), (0.05, -0.05), (0.05, 0.05), (-0.05, 0.05)])
}

/// The same square, authored starting one corner later — the SAME
/// point set and the same winding, a rotated traversal order.
fn sq_rotated() -> sweep::Section {
    quad([(0.05, -0.05), (0.05, 0.05), (-0.05, 0.05), (-0.05, -0.05)])
}

/// `stations` sections along a planar unit-radius arc turning through
/// `curl` toward `+z`, each section's `c2` the spine tangent — the
/// issue's signature, re-spelled here so the probes do not lean on the
/// unit's own helper.
fn arc_spine_places(curl: f64, stations: usize) -> Vec<Affine3<f64>> {
    (0..stations)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let theta = curl * (i as f64) / ((stations - 1) as f64);
            let (s, c) = theta.sin_cos();
            Affine3::from_parts(
                Mat3::from_cols(
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(-s, 0.0, c),
                    Vec3::new(c, 0.0, s),
                ),
                Vec3::new(s, 0.0, 1.0 - c),
            )
        })
        .collect()
}

fn arc_spine_loft(curl: f64, stations: usize) -> Result<(), LoftError> {
    let sections = vec![sq(); stations];
    let v_degree = stations.min(4) - 1;
    loft_body::<f64>(
        &sections,
        &arc_spine_places(curl, stations),
        v_degree,
        Tol::witness(),
    )
    .map(|_| ())
}

/// A forward stack of `k` equal sections one metre apart.
fn forward_stack(k: usize) -> (Vec<sweep::Section>, Vec<Affine3<f64>>) {
    #[allow(clippy::cast_precision_loss)]
    let zs: Vec<f64> = (0..k).map(|i| i as f64).collect();
    (vec![sq(); k], stacked_at(&zs))
}

/// Counts the `loft_stacking` verdicts one closure mints, and returns
/// them in decision order.
fn stacking_verdicts(run: impl FnOnce()) -> Vec<Sign> {
    let bracket = Bracket::open();
    run();
    bracket
        .finish()
        .verdicts
        .iter()
        .filter(|v| v.predicate == "loft_stacking")
        .map(|v| v.sign)
        .collect()
}

/// **The population: one `loft_stacking` sample per SLAB.** The old
/// statement minted exactly one per loft whatever `k` was; the fold
/// mints `k − 1`, all Positive on a forward stack. This is the claim
/// the ε-posture paragraph makes about the predicate's distribution,
/// executed rather than asserted.
#[test]
fn the_fold_mints_one_loft_stacking_verdict_per_slab() {
    for k in [2usize, 3, 4, 5, 9, 17] {
        let (sections, places) = forward_stack(k);
        let v_degree = k.min(4) - 1;
        let signs = stacking_verdicts(|| {
            let out = loft_body::<f64>(&sections, &places, v_degree, Tol::witness());
            assert!(out.is_ok(), "the forward stack of {k} sections builds");
        });
        assert_eq!(
            signs.len(),
            k - 1,
            "{k} sections must mint {} loft_stacking samples, got {signs:?}",
            k - 1
        );
        assert!(
            signs.iter().all(|s| *s == Sign::Positive),
            "every slab of a forward stack is Positive, got {signs:?}"
        );
    }
}

/// **The two-section slab margin IS the step**, at both band edges.
/// A pure `+z` stack of a 4-vertex square sums four copies of
/// `(0, 0, step)` and divides by four, so the margin the fold hands
/// `loft_stacking` must be `step` to the bit — and the loft's verdict
/// must therefore agree with `decide` called on `Margin::of(step)`
/// directly, at ε, at `K·ε`, and either side of both. If the fold had
/// rescaled, re-normalised or re-ordered anything, these rows are
/// where it would show.
#[test]
fn the_two_section_slab_margin_is_the_step_itself() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let (eps, k) = (tol.eps(), tol.k());
    for step in [
        0.5 * eps,
        eps,
        1.0001 * eps,
        0.5 * (1.0 + k) * eps,
        k * eps,
        1.0001 * k * eps,
        1.0,
    ] {
        let sections = vec![sq(), sq()];
        let got = loft_body::<f64>(&sections, &stacked_at(&[0.0, step]), 1, tol).map(|_| ());
        let want = geom_core::k_stats::decide("loft_stacking", Margin::of(step), band);
        match (want, &got) {
            (Ok(Sign::Positive), Ok(())) => {}
            (Ok(Sign::Zero), Err(LoftError::DegenerateStacking { slab: 0 })) => {}
            (Ok(Sign::Negative), Err(LoftError::ReversedStacking { slab: 0 })) => {}
            (Err(_), Err(LoftError::StackingEscalated { slab: 0, .. })) => {}
            _ => panic!("step {step:e}: decide said {want:?}, the loft said {got:?}"),
        }
    }
}

/// **A reversed two-section loft still names slab 0.**
#[test]
fn a_reversed_two_section_loft_names_slab_zero() {
    let sections = vec![sq(), sq()];
    match loft_body::<f64>(&sections, &stacked_at(&[0.0, -1.0]), 1, Tol::witness()) {
        Err(LoftError::ReversedStacking { slab: 0 }) => {}
        other => panic!("expected ReversedStacking naming slab 0, got {other:?}"),
    }
}

/// **The band edge inside a forward stack, at whatever ε the run
/// commits.** Three interior steps — one below ε, one at the band's
/// midpoint, one above `K·ε` — inside an otherwise honest stack, each
/// asserted against `decide` on the same number. Run at three ε rows
/// this pins that the fold's band tracks the run's tolerance rather
/// than a digit written into the fold.
#[test]
fn an_interior_slab_at_the_band_edges_tracks_the_run_band() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let (eps, k) = (tol.eps(), tol.k());
    for step in [0.5 * eps, 0.5 * (1.0 + k) * eps, 2.0 * k * eps] {
        let sections = vec![sq(), sq(), sq(), sq()];
        let places = stacked_at(&[0.0, 1.0, 1.0 + step, 2.0]);
        let got = loft_body::<f64>(&sections, &places, 2, tol).map(|_| ());
        let want = geom_core::k_stats::decide("loft_stacking", Margin::of(step), band);
        match (want, &got) {
            (Ok(Sign::Positive), Ok(())) => {}
            (Ok(Sign::Zero), Err(LoftError::DegenerateStacking { slab: 1 })) => {}
            (Ok(Sign::Negative), Err(LoftError::ReversedStacking { slab: 1 })) => {}
            (Err(_), Err(LoftError::StackingEscalated { slab: 1, .. })) => {}
            _ => panic!("interior step {step:e}: decide said {want:?}, loft said {got:?}"),
        }
    }
}

/// **The fold's vertex PAIRING carries no information** — and this is
/// an algebraic identity, not a defeated attack. A sum of differences
/// over a full zip telescopes to the difference of the two sums, so
/// the margin is the vertex CENTROID's displacement and no
/// correspondence can change it. The row executes the identity so the
/// docs' phrase "mean top-vertex displacement" is not read as a claim
/// about pairing.
///
/// The same two stacks, one with the top square authored from a
/// different corner, decide identically.
#[test]
fn a_rotated_top_traversal_decides_identically() {
    let tol = Tol::witness();
    for (zs, tag) in [(vec![0.0, 1.0], "forward"), (vec![0.0, -1.0], "reversed")] {
        let places = stacked_at(&zs);
        let straight = loft_body::<f64>(&[sq(), sq()], &places, 1, tol).map(|_| ());
        let rotated = loft_body::<f64>(&[sq(), sq_rotated()], &places, 1, tol).map(|_| ());
        let (a, b) = (
            stacking_verdicts(|| {
                let _ = loft_body::<f64>(&[sq(), sq()], &places, 1, tol);
            }),
            stacking_verdicts(|| {
                let _ = loft_body::<f64>(&[sq(), sq_rotated()], &places, 1, tol);
            }),
        );
        assert_eq!(
            a, b,
            "{tag}: rotating the top traversal changed the stacking verdicts \
             ({a:?} vs {b:?}); straight={straight:?} rotated={rotated:?}"
        );
    }
}

/// **Sections with different vertex counts never reach the fold.** The
/// fold's own guard is `next.len() != base.len()` →
/// `SectionStructure`; this row records which door actually answers a
/// triangle stacked under a square, so a reader knows whether that
/// guard is reachable through the public surface at all.
#[test]
fn a_triangle_under_a_square_is_refused_before_the_fold() {
    let tri = vec![sweep::ProfileLoop::polygon(
        [
            Point2::new(-0.05, -0.05),
            Point2::new(0.05, -0.05),
            Point2::new(0.0, 0.05),
        ]
        .into_iter(),
    )];
    let out = loft_body::<f64>(&[tri, sq()], &stacked_at(&[0.0, 1.0]), 1, Tol::witness());
    assert!(
        out.is_err(),
        "a 3-vertex section under a 4-vertex one must refuse, got {out:?}"
    );
    println!("   triangle-under-square refuses: {:?}", out.err());
}

/// **Where the sliver hands off.** The skin refuses coincident
/// sections by a BARE `params[j-1] < params[j]` (no band), so the
/// question the PR's finding raises is which door answers a pair that
/// is within ε but not exactly coincident. This row walks the step
/// down from ε to zero and prints the door at each — the sliver band
/// belongs to the loft's `DegenerateStacking`, and the skin takes over
/// only once the NORMALISED chord step underflows to zero.
#[test]
fn the_sliver_hands_off_from_the_loft_to_the_skin() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let mut doors: Vec<(f64, &'static str)> = Vec::new();
    for step in [
        0.5 * eps,
        1e-3 * eps,
        1e-9 * eps,
        1e-15,
        1e-16,
        1e-17,
        1e-300,
        0.0,
    ] {
        let sections = vec![sq(), sq(), sq(), sq()];
        let places = stacked_at(&[0.0, 1.0, 1.0 + step, 2.0]);
        let door = match loft_body::<f64>(&sections, &places, 2, tol) {
            Ok(_) => "built",
            Err(LoftError::DegenerateStacking { .. }) => "loft: DegenerateStacking",
            Err(LoftError::StackingEscalated { .. }) => "loft: StackingEscalated",
            Err(LoftError::ReversedStacking { .. }) => "loft: ReversedStacking",
            Err(LoftError::Skin(SkinError::DegenerateSection { .. })) => "skin: DegenerateSection",
            Err(_) => "other",
        };
        println!("   sliver {step:e} -> {door}");
        doors.push((step, door));
    }
    assert_eq!(
        doors[0].1, "loft: DegenerateStacking",
        "half of eps is the loft's sliver, not the skin's coincidence"
    );
    assert_eq!(
        doors.last().expect("rows").1,
        "skin: DegenerateSection",
        "exactly coincident sections are the skin's, before the fold"
    );
}

/// **The straddle set the PR's table names, executed.** Curls 3.5,
/// 4.7, 6.0, 9.0 and 13.0 at 17 stations — the last two past two full
/// turns — all build, and the same total curl refuses once the
/// sampling puts a slab past π.
#[test]
fn the_straddle_set_builds_and_the_sampling_wall_refuses() {
    for curl in [3.5, 4.7, 6.0, 9.0, 13.0] {
        let out = arc_spine_loft(curl, 17);
        assert!(
            out.is_ok(),
            "curl {curl} rad over 17 stations must build, got {out:?}"
        );
    }
    match arc_spine_loft(10.0, 4) {
        Err(LoftError::ReversedStacking { slab }) => {
            assert_eq!(slab, 0, "the first slab past the wall is the one named");
        }
        other => panic!("10.0 rad over 4 stations is 3.33 rad per slab: got {other:?}"),
    }
}

/// **The fold reads no third section, measured by SUBSTITUTION.** Two
/// four-section stacks that agree on sections 1 and 2 and disagree
/// everywhere else decide slab 1 the same way — and the whole verdict
/// SEQUENCE up to the refusal is the same, which the unit's own row
/// (which reads only the refusal) does not say.
#[test]
fn the_verdict_sequence_up_to_a_refusal_is_slab_local() {
    let tol = Tol::witness();
    let sections = vec![
        loft_prism_sections()[0].clone(),
        loft_prism_sections()[0].clone(),
        loft_prism_sections()[0].clone(),
        loft_prism_sections()[0].clone(),
    ];
    let a = stacking_verdicts(|| {
        let _ = loft_body::<f64>(&sections, &stacked_at(&[0.0, 1.0, 0.5, 2.0]), 2, tol);
    });
    let b = stacking_verdicts(|| {
        let _ = loft_body::<f64>(&sections, &stacked_at(&[0.0, 1.0, 0.5, 40.0]), 2, tol);
    });
    assert_eq!(
        a,
        vec![Sign::Positive, Sign::Negative],
        "slab 0 decides Positive, slab 1 Negative, and the fold stops there \
         — slab 2 is never sampled"
    );
    assert_eq!(a, b, "the last section cannot reach slab 1's verdict");
}

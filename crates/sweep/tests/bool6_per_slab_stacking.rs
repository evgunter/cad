//! **The per-slab stacking fold** (issue 368, ruled 2026-09-01).
//!
//! The loft's stacking statement is a FOLD over adjacent section
//! pairs, each pair decided against ITS OWN base section's plane
//! normal, and not a single end-to-end summary of the last section
//! against the first. These rows pin both directions of that change.
//!
//! What moves: a planar spine that turns past π stacks honestly at
//! every slab and now BUILDS, where the end-to-end statement —
//! `cos(curl/2)` for a planar arc spine — walled it at exactly a half
//! turn regardless of station count. The wall that remains is the
//! per-slab one, at per-slab turn π (total curl `(k−1)·π` for `k`
//! sections), so it is a statement about how coarsely the spine was
//! sampled rather than about how far it goes.
//!
//! What ALSO moves, in the strict direction: a stack that walks
//! BACKWARDS in the middle and recovers — `z = 0, 1, 0.5, 2` — used to
//! pass on the strength of its positive end-to-end sum. Every such
//! slab is now decided on its own and refuses NAMING the pair, because
//! the reorder recourse the refusal offers is only sound for a wholly
//! reversed list.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Mat3, Tol, Vec3};
use sweep::test_support::{loft_prism_sections, stacked_at};
use sweep::{LoftError, loft_body};

use crate::common;
use common::quad;

/// The section every arc-spine row lofts: a small square, kept far
/// smaller than the spine's radius of curvature so the walls never
/// fold through the spine's own centre and the row decides the
/// STACKING rather than the geometry.
fn spine_section() -> sweep::Section {
    quad([(-0.05, -0.05), (0.05, -0.05), (0.05, 0.05), (-0.05, 0.05)])
}

/// **The issue's executed signature, in the sweep crate's own
/// vocabulary**: `stations` sections placed along a planar circular
/// arc of unit radius that turns through `curl` toward `+z`, each
/// section's plane normal the spine's own tangent there.
///
/// This is lily's leaf-A spine with a constant section — the same
/// placement family (`demos/tour/src/lily.rs::lofted_blade`), reduced
/// to the part the stacking statement reads. The end-to-end
/// displacement of the last section against the FIRST section's normal
/// is `R·sin(curl)·cos(0) …` in closed form `∝ cos(curl/2)`; each
/// slab's is `∝ cos(curl/(2(stations−1)))`.
fn arc_spine_places(curl: f64, stations: usize) -> Vec<Affine3<f64>> {
    (0..stations)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let theta = curl * (i as f64) / ((stations - 1) as f64);
            let (s, c) = theta.sin_cos();
            // c2 is the section's plane normal — the spine tangent.
            let linear = Mat3::from_cols(
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(-s, 0.0, c),
                Vec3::new(c, 0.0, s),
            );
            Affine3::from_parts(linear, Vec3::new(s, 0.0, 1.0 - c))
        })
        .collect()
}

/// Lofts the arc-spine family at one curl.
fn arc_spine_loft(curl: f64, stations: usize) -> Result<(), LoftError> {
    let sections = vec![spine_section(); stations];
    // The v-degree the skin can interpolate `stations` sections at —
    // 3 wherever there is room for it, as lily lofts its blade.
    let v_degree = stations.min(4) - 1;
    loft_body::<f64>(
        &sections,
        &arc_spine_places(curl, stations),
        v_degree,
        Tol::witness(),
    )
    .map(|_| ())
}

/// **Curls past π build.** The issue's executed signature: 0.45 … 3.0
/// built before (through the end-to-end wall at exactly π), and 3.5
/// and 4.0 refused `ReversedStacking` although every slab — 1/16 of
/// the turn at 17 stations — advances honestly.
#[test]
fn a_planar_arc_spine_lofts_past_pi() {
    for curl in [0.45, 1.0, 2.0, 2.5, 2.8, 3.0, 3.5, 4.0] {
        let out = arc_spine_loft(curl, 17);
        assert!(
            out.is_ok(),
            "curl {curl} rad over 17 stations refused ({:?}): every slab turns \
             {:.4} rad, far short of the per-slab wall at π",
            out.err(),
            curl / 16.0,
        );
    }
}

/// **The wall that remains is PER-SLAB, and it is about sampling.**
/// The same total curl builds at 17 stations and refuses at 3, because
/// at 3 stations one slab carries 1.1π of it and its displacement
/// genuinely runs against its own base normal.
#[test]
fn the_remaining_wall_is_per_slab_turn_pi() {
    let curl = 2.2 * core::f64::consts::PI;
    assert!(
        arc_spine_loft(curl, 17).is_ok(),
        "{curl:.3} rad over 17 stations is 0.138π per slab and must build"
    );
    match arc_spine_loft(curl, 3) {
        Err(LoftError::ReversedStacking { slab }) => assert_eq!(
            slab, 0,
            "the FIRST slab past the wall is the one named, not a later one"
        ),
        other => panic!(
            "{curl:.3} rad over 3 stations is 1.1π per slab and must refuse \
             reversed, got {other:?}"
        ),
    }
}

/// **A reversed MIDDLE slab refuses, naming its pair.** `z = 0, 1,
/// 0.5, 2` sums to a positive end-to-end displacement and passed the
/// ends-only statement; slab 1 walks backwards and is an authoring
/// fault the builder must not orient past.
#[test]
fn a_reversed_middle_slab_refuses_naming_its_pair() {
    let sections = vec![
        loft_prism_sections()[0].clone(),
        loft_prism_sections()[0].clone(),
        loft_prism_sections()[0].clone(),
        loft_prism_sections()[0].clone(),
    ];
    let places = stacked_at(&[0.0, 1.0, 0.5, 2.0]);
    match loft_body::<f64>(&sections, &places, 2, Tol::witness()) {
        Err(LoftError::ReversedStacking { slab }) => assert_eq!(
            slab, 1,
            "the refusal must name the pair that reverses (sections 1 and 2)"
        ),
        other => panic!("expected ReversedStacking naming slab 1, got {other:?}"),
    }
}

/// **A sliver MIDDLE slab refuses, naming its pair.** A stack that
/// barely moves at one pair and walks on: the end-to-end sum is +2 and
/// said nothing about it.
///
/// The step is read off the RUN's band rather than written as a digit,
/// because what the row pins is a band verdict — half of ε is
/// coincident at every ε row, and a fixed digit would pin the verdict
/// at one row and something else at the others. Exactly coincident
/// sections cannot reach this door at all: the skin refuses them first
/// as `SkinError::DegenerateSection` (it needs a chord step to
/// parameterize at all), so the reachable degenerate slab is the
/// sliver, which is what the arm's own docs call it.
#[test]
fn a_sliver_middle_slab_refuses_naming_its_pair() {
    let sections = vec![
        loft_prism_sections()[0].clone(),
        loft_prism_sections()[0].clone(),
        loft_prism_sections()[0].clone(),
        loft_prism_sections()[0].clone(),
    ];
    let sliver = 0.5 * Tol::witness().eps();
    let places = stacked_at(&[0.0, 1.0, 1.0 + sliver, 2.0]);
    match loft_body::<f64>(&sections, &places, 2, Tol::witness()) {
        Err(LoftError::DegenerateStacking { slab }) => assert_eq!(
            slab, 1,
            "the refusal must name the sliver pair (sections 1 and 2)"
        ),
        other => panic!("expected DegenerateStacking naming slab 1, got {other:?}"),
    }
}

/// **An AMBIGUOUS middle slab escalates, carrying its pair.** The step
/// is placed in the middle of the run's ambiguity band `(ε, K·ε)`, so
/// the fold can neither accept nor refuse it — and this is the case
/// that decides the fold's shape: deciding the MINIMUM slab margin
/// instead would let the two honest slabs' neighbours say nothing
/// while a definitely-reversed slab elsewhere answered for this one.
#[test]
fn an_ambiguous_middle_slab_escalates_carrying_its_pair() {
    let sections = vec![
        loft_prism_sections()[0].clone(),
        loft_prism_sections()[0].clone(),
        loft_prism_sections()[0].clone(),
        loft_prism_sections()[0].clone(),
    ];
    let tol = Tol::witness();
    let inside_the_band = 0.5 * (1.0 + tol.k()) * tol.eps();
    let places = stacked_at(&[0.0, 1.0, 1.0 + inside_the_band, 2.0]);
    match loft_body::<f64>(&sections, &places, 2, tol) {
        Err(LoftError::StackingEscalated { slab, source }) => {
            assert_eq!(slab, 1, "the escalation must carry the ambiguous pair");
            assert_eq!(
                source.predicate,
                Some("loft_stacking"),
                "and name the predicate that could not decide"
            );
        }
        other => panic!("expected StackingEscalated carrying slab 1, got {other:?}"),
    }
}

/// **A wholly reversed stack names its FIRST slab** — the fold refuses
/// at the first slab that is not definitely positive, in index order,
/// so the message points at the earliest authoring fault rather than
/// the worst one. This is the case the reorder recourse is sound for.
#[test]
fn a_wholly_reversed_stack_names_its_first_slab() {
    let places = stacked_at(&[0.0, -1.0, -2.0]);
    match loft_body::<f64>(&loft_prism_sections(), &places, 2, Tol::witness()) {
        Err(LoftError::ReversedStacking { slab }) => assert_eq!(slab, 0, "the first slab"),
        other => panic!("expected ReversedStacking naming slab 0, got {other:?}"),
    }
}

/// **The two-section loft is the degenerate case of the fold.** One
/// slab, whose base section IS the first section — so the fold
/// evaluates the very expression the end-to-end statement did, over
/// the same vertices in the same traversal order, and decides it at
/// the same `loft_stacking` band. Both signs pinned.
#[test]
fn the_two_section_loft_is_one_slab() {
    let sections = vec![
        loft_prism_sections()[0].clone(),
        loft_prism_sections()[0].clone(),
    ];
    assert!(
        loft_body::<f64>(&sections, &stacked_at(&[0.0, 1.0]), 1, Tol::witness()).is_ok(),
        "the forward two-section loft builds"
    );
    match loft_body::<f64>(&sections, &stacked_at(&[0.0, -1.0]), 1, Tol::witness()) {
        Err(LoftError::ReversedStacking { slab }) => assert_eq!(slab, 0, "its one slab is slab 0"),
        other => panic!("expected ReversedStacking naming slab 0, got {other:?}"),
    }
}

/// **The fold reads nothing but the two adjacent sections.** Moving a
/// section the slab does not touch cannot change that slab's verdict:
/// the `z = 0, 1, 0.5, 2` stack refuses at slab 1 whatever the LAST
/// section does, including when the end-to-end sum is driven far
/// positive or made negative outright.
#[test]
fn a_slab_verdict_ignores_the_sections_it_does_not_touch() {
    let sections = vec![
        loft_prism_sections()[0].clone(),
        loft_prism_sections()[0].clone(),
        loft_prism_sections()[0].clone(),
        loft_prism_sections()[0].clone(),
    ];
    for last in [2.0, 40.0] {
        let places = stacked_at(&[0.0, 1.0, 0.5, last]);
        match loft_body::<f64>(&sections, &places, 2, Tol::witness()) {
            Err(LoftError::ReversedStacking { slab }) => assert_eq!(
                slab, 1,
                "slab 1's verdict is its own; the last section sat at {last}"
            ),
            other => panic!("expected ReversedStacking naming slab 1, got {other:?}"),
        }
    }
}

/// The spine helper is a spine: consecutive stations advance along the
/// arc, and the frame is right-handed with `c2` the tangent. A silent
/// error here would make every arc-spine row above decide something
/// other than what it claims.
#[test]
fn the_arc_spine_helper_places_a_right_handed_tangent_frame() {
    let places = arc_spine_places(2.2, 5);
    for (i, p) in places.iter().enumerate() {
        let n = p.linear.c2;
        assert!(
            (n.dot(n) - 1.0).abs() < 1e-14,
            "station {i}'s normal is a unit tangent"
        );
        assert!(
            (p.linear.c0.cross(p.linear.c1).dot(n) - 1.0).abs() < 1e-14,
            "station {i}'s frame is right-handed"
        );
    }
    let step: Vec3<f64> = places[1].translation - places[0].translation;
    assert!(
        step.dot(places[0].linear.c2) > 0.0,
        "the first slab advances along the first station's own normal"
    );
    let ends: Vec3<f64> = places[4].translation - places[0].translation;
    assert!(
        ends.dot(places[0].linear.c2) > 0.0,
        "2.2 rad is short of pi, so the END-TO-END displacement is still positive \
         here - the rows past pi are the ones that separate the two statements"
    );
}

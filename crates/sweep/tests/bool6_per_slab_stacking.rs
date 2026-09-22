//! **The per-slab stacking fold** (issue 368, ruled 2026-09-01).
//!
//! The loft's stacking statement is a fold over adjacent section
//! pairs, each pair decided against its own base section's plane
//! normal (`sweep::loft`'s `stacking_fold`, where the statement is
//! made and stated). These rows pin what that admits, what it refuses,
//! and what it names when it refuses.
//!
//! The admitted family reaches past a half turn, and the ORACLE that
//! can answer there is the turning one. The loft corpus's default,
//! `common::orient::loft_contains`, measures against a fixed reference
//! chord and refuses a stack this curled outright — `cos = 0.0698` at
//! v-fraction `0.03125` against its `FIXED_AXIS_GUARD_COS` of `0.1`,
//! measured on the 3.5-rad spine — so [`common::orient::LevelIndex`],
//! the index the long-turn sweep suite uses, is the instrument for
//! every orientation row here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Mat3, Tol, Vec3};
use sweep::test_support::{loft_prism_sections, stacked_at};
use sweep::{LoftError, Lofted, loft_body};

use crate::common;
use common::{band_midpoint, quad};

/// The section every arc-spine row lofts: a small square, kept far
/// smaller than the spine's radius of curvature so the walls never
/// fold through the spine's own centre and the row decides the
/// STACKING rather than the geometry.
fn spine_section() -> sweep::Section {
    quad([(-0.05, -0.05), (0.05, -0.05), (0.05, 0.05), (-0.05, 0.05)])
}

/// Four copies of the loft prism's square section.
fn four_sections() -> Vec<sweep::Section> {
    vec![loft_prism_sections()[0].clone(); 4]
}

/// **The issue's executed signature, in the sweep crate's own
/// vocabulary**: `stations` sections placed along a planar circular
/// arc of UNIT RADIUS turning through `curl` toward `+z`, each
/// section's plane normal the spine's own tangent there.
///
/// This is lily's leaf-A spine with a constant section — the same
/// placement family (`demos/tour/src/lily.rs::lofted_blade`), reduced
/// to the part the stacking statement reads. Station `i` sits at
/// `θ = curl·i/(stations − 1)`, at `(sin θ, 0, 1 − cos θ)` with
/// tangent `(cos θ, 0, sin θ)`. Two consequences the rows below use: a
/// slab's displacement makes an angle of half the slab's turn with its
/// base tangent, so the slab margin goes negative exactly when the
/// SLAB turns past π; and the spine closes on itself at `curl = 2π`,
/// past which the body passes through itself.
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

/// The v-degree the skin can interpolate `stations` sections at — 3
/// wherever there is room for it, as lily lofts its blade.
fn spine_degree(stations: usize) -> usize {
    stations.min(4) - 1
}

/// Lofts the arc-spine family at one curl, keeping the body.
fn arc_spine_lofted(curl: f64, stations: usize) -> Result<Lofted<f64>, LoftError> {
    loft_body::<f64>(
        &vec![spine_section(); stations],
        &arc_spine_places(curl, stations),
        spine_degree(stations),
        Tol::witness(),
    )
}

/// Lofts the arc-spine family at one curl, keeping only the verdict.
fn arc_spine_loft(curl: f64, stations: usize) -> Result<(), LoftError> {
    arc_spine_lofted(curl, stations).map(|_| ())
}

/// **A planar arc spine lofts past π.** At 17 stations each slab
/// carries a sixteenth of the turn, so the fold's statement is met all
/// the way through and the loft builds. Every curl here stays well
/// inside the spine's own closure at `2π`, so none of these bodies
/// revisits itself; the family past that point is
/// [`a_curl_past_a_full_turn_builds_a_spine_that_revisits_itself`].
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

/// **The wall is per-slab turn π, and it is about SAMPLING.** One
/// total turn, both sides of the wall, with no self-overlap anywhere
/// in the row: `1.1π` over two stations puts the whole of it in one
/// slab and refuses naming that slab; the same `1.1π` over seventeen
/// puts `0.069π` in each and builds. Only the sampling differs.
#[test]
fn the_remaining_wall_is_per_slab_turn_pi() {
    let curl = 1.1 * core::f64::consts::PI;
    match arc_spine_loft(curl, 2) {
        Err(LoftError::ReversedStacking { slab }) => assert_eq!(
            slab, 0,
            "a two-station loft has one slab and it is the one named"
        ),
        other => {
            panic!("{curl:.3} rad in ONE slab is 1.1π and must refuse reversed, got {other:?}")
        }
    }
    assert!(
        arc_spine_loft(curl, 17).is_ok(),
        "{curl:.3} rad over 17 stations is 0.069π per slab and must build"
    );
}

/// **Past a full turn the spine revisits itself — and nothing refuses
/// it.** The fold is local by ruling, so a body that passes through
/// itself is not the fold's to see, and no other gate in the kernel
/// looks either: this row exists so that "curl 9.0 builds" is on
/// record as what it is rather than as a win.
///
/// What the row measures: two stations three or more apart come closer
/// together than the section is wide, so the tube passes through its
/// own wall. What the kernel says about that body: tiers 1, 2 and 3
/// are all `Ok`. What an index that can see it says,
/// `LevelIndex::contains` REFUSES to answer — "2 level rings claim
/// Point3 { x: 0.4529…, y: -0.025, z: 0.0625… } — the body overlaps
/// itself there and containment is not a function of position" — which
/// is why this row does not ask it for a verdict.
///
/// Adopted from the BOOL-6 R1 review probes.
#[test]
fn a_curl_past_a_full_turn_builds_a_spine_that_revisits_itself() {
    let tol = Tol::witness();
    let places = arc_spine_places(9.0, 17);
    let lofted = arc_spine_lofted(9.0, 17).expect("curl 9.0 over 17 stations builds");
    assert_eq!(topo::validate(&lofted.body), Ok(()), "tier 1 is silent");
    assert_eq!(
        topo::validate_closed(&lofted.body),
        Ok(()),
        "tier 2 is silent"
    );
    assert_eq!(
        topo::validate_geometric(&lofted.body, tol),
        Ok(()),
        "tier 3 is silent too"
    );
    let mut closest = f64::INFINITY;
    for i in 0..places.len() {
        for j in i + 3..places.len() {
            let d: Vec3<f64> = places[j].translation - places[i].translation;
            closest = closest.min(d.norm());
        }
    }
    assert!(
        closest < 0.1,
        "stations three or more apart come within {closest} — the section is 0.1 \
         wide, so a spine curled to 9.0 rad at unit radius passes through its own \
         body, and every tier above said Ok"
    );
}

/// **The widened accept side still faces out everywhere.** The
/// stacking gate exists to make the caps' and the walls' orientation
/// honest, so the family the fold newly admits has to be asked the
/// question the gate is for, not just `is_ok()`: on a spine curled
/// past π, every wall's chart normal points out of the material and
/// both caps do too.
///
/// The oracle is [`common::orient::LevelIndex`] for the reason the
/// module docs give.
///
/// Adopted from the BOOL-6 R1 review probes.
#[test]
fn a_spine_curled_past_pi_still_faces_out_everywhere() {
    for curl in [3.5, 6.0] {
        let lofted = arc_spine_lofted(curl, 17)
            .unwrap_or_else(|e| panic!("curl {curl} rad over 17 stations lofts: {e:?}"));
        assert_eq!(topo::validate(&lofted.body), Ok(()), "curl {curl}: tier 1");
        assert_eq!(
            topo::validate_closed(&lofted.body),
            Ok(()),
            "curl {curl}: tier 2"
        );
        let index = common::orient::LevelIndex::build(&lofted);
        assert!(
            index.total_turn() > 0.9 * curl,
            "curl {curl}: the level planes must really turn, got {}",
            index.total_turn()
        );
        let oracle = |q| index.contains(q);
        common::orient::assert_walls_face_out(
            &lofted,
            &oracle,
            &common::orient::along_v(),
            0.01,
            4,
        );
        common::orient::assert_caps_face_out(&lofted, &oracle, 0.01);
    }
}

/// **A reversed MIDDLE slab refuses, naming its pair.** `z = 0, 1,
/// 0.5, 2` steps backwards between sections 1 and 2 and forwards
/// either side of it; the fold decides that pair on its own.
#[test]
fn a_reversed_middle_slab_refuses_naming_its_pair() {
    let places = stacked_at(&[0.0, 1.0, 0.5, 2.0]);
    match loft_body::<f64>(&four_sections(), &places, 2, Tol::witness()) {
        Err(LoftError::ReversedStacking { slab }) => assert_eq!(
            slab, 1,
            "the refusal must name the pair that reverses (sections 1 and 2)"
        ),
        other => panic!("expected ReversedStacking naming slab 1, got {other:?}"),
    }
}

/// **A sliver MIDDLE slab refuses, naming its pair.**
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
    let sliver = 0.5 * Tol::witness().eps();
    let places = stacked_at(&[0.0, 1.0, 1.0 + sliver, 2.0]);
    match loft_body::<f64>(&four_sections(), &places, 2, Tol::witness()) {
        Err(LoftError::DegenerateStacking { slab }) => assert_eq!(
            slab, 1,
            "the refusal must name the sliver pair (sections 1 and 2)"
        ),
        other => panic!("expected DegenerateStacking naming slab 1, got {other:?}"),
    }
}

/// **An AMBIGUOUS middle slab escalates, carrying its pair.** The step
/// sits at the midpoint of the run's ambiguity band `(ε, K·ε)`, so the
/// fold can neither accept nor refuse it. This is the case that
/// decides the fold's shape: a fold that classified the MINIMUM slab
/// margin would let a definite verdict elsewhere in the list answer
/// for this pair, and an ambiguous margin is the one thing the D4 band
/// exists to surface.
#[test]
fn an_ambiguous_middle_slab_escalates_carrying_its_pair() {
    let tol = Tol::witness();
    let places = stacked_at(&[0.0, 1.0, 1.0 + band_midpoint(tol), 2.0]);
    match loft_body::<f64>(&four_sections(), &places, 2, tol) {
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

/// **The two-section loft is one slab, and its base section is the
/// first section.** Both signs, named.
///
/// That the slab's margin IS the step — the same sum over the same
/// vertices in the same order, at every edge of the band — is
/// `bool6r1_probes::the_two_section_slab_margin_is_the_step_itself`.
#[test]
fn the_two_section_loft_is_one_slab() {
    let sections = vec![loft_prism_sections()[0].clone(); 2];
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
/// positive.
#[test]
fn a_slab_verdict_ignores_the_sections_it_does_not_touch() {
    for last in [2.0, 40.0] {
        let places = stacked_at(&[0.0, 1.0, 0.5, last]);
        match loft_body::<f64>(&four_sections(), &places, 2, Tol::witness()) {
            Err(LoftError::ReversedStacking { slab }) => assert_eq!(
                slab, 1,
                "slab 1's verdict is its own; the last section sat at {last}"
            ),
            other => panic!("expected ReversedStacking naming slab 1, got {other:?}"),
        }
    }
}

/// The spine helper is a spine: consecutive stations advance along the
/// arc and the frame is right-handed with `c2` the tangent. A silent
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
}

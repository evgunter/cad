//! **Review probes (lane `radius-r2`) for the per-radius emission
//! record** (`edit/radius-emission-record`, PR 2892).
//!
//! These are a reviewer's rows, written to falsify the PR's claims by
//! execution rather than by inspection. They are kept because each
//! pins a fact no row on the branch pins: the EXACT-FIT close's
//! emission (the implementer's un-filed observation), the `Sweep`
//! leg's own-step address wherever the leg sits in the chain, and the
//! one-emission-per-segment property `eval::wire`'s `edge_radii`
//! silently assumes when it takes the FIRST matching ref.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{coverage_corpus, p2, tol};
use geom_core::Point2;
use profile::{
    ArcLen, ArcSide, ArcSweep, Center, ClosedLoop, Open, Radius, RadiusRole, Start, Sweep,
    replay_guided, replay_recording,
};

/// The emission record as a flat tuple list, in emission order.
fn radii(closed: &ClosedLoop<f64>) -> Vec<(usize, RadiusRole, usize)> {
    closed
        .structure
        .radii
        .iter()
        .map(|e| (e.step, e.role, e.segment))
        .collect()
}

/// The carrier radius of the segment LEAVING vertex `i`, read back off
/// the stored chord-and-bulge the way a reader classifies it —
/// `r = c(1 + b²) / (4|b|)` for chord `c` and bulge `b = tan(θ/4)`.
/// `None` where the stored segment is straight.
fn stored_radius(closed: &ClosedLoop<f64>, i: usize) -> Option<f64> {
    let vs = closed.loop_.vertices();
    let b = vs[i].bulge();
    if b == 0.0 {
        return None;
    }
    let a = vs[i].pos();
    let z = vs[(i + 1) % vs.len()].pos();
    let chord = (z - a).norm_squared().sqrt();
    Some(chord * (1.0 + b * b) / (4.0 * b.abs()))
}

/// **The EXACT-FIT close records its fillet on the closing segment.**
///
/// `family::resolve_arc_close`'s `else` arm is the branch where the
/// fillet arc consumes the whole arrival side and IS the closing
/// segment. It is the one emission site that does not go through
/// `Core::record_fillet_arc`, so the PR records its address by hand
/// there; this row is what says the hand-written address is the same
/// one the shared door would have computed. `r = 1.0` is the radius
/// that consumes the line × arc corner's outgoing side exactly
/// (`fillet_stored_tangency::an_exact_outgoing_fit_leaves_its_joint_
/// undeclared_and_still_validates` is the fixture).
#[test]
fn the_exact_fit_close_records_its_fillet_on_the_closing_segment() {
    let exact = Open
        .at(p2(0.0, 2.0))
        .line_to(p2(0.0, 0.0), tol())
        .unwrap()
        .toward(2.0, 0.0, tol())
        .unwrap()
        .fillet_arc(
            1.0,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            tol(),
        )
        .unwrap();
    let last = exact.loop_.vertices().len() - 1;
    assert_eq!(
        radii(&exact),
        vec![(3, RadiusRole::Fillet, last)],
        "the exact fit's one emission is the fillet's, on the CLOSING segment"
    );
    let got = stored_radius(&exact, last).expect("the closing segment is an arc");
    assert!(
        (got - 1.0).abs() < 1e-9,
        "and the segment it names is stored at the authored radius, not {got}"
    );
    // The same chain at a radius that leaves carrier run behind takes
    // the OTHER arm, where `record_fillet_arc` writes the address; the
    // two arms agree about what a fillet emission looks like.
    let inexact = Open
        .at(p2(0.0, 2.0))
        .line_to(p2(0.0, 0.0), tol())
        .unwrap()
        .toward(2.0, 0.0, tol())
        .unwrap()
        .fillet_arc(
            0.5,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            tol(),
        )
        .unwrap();
    let [(step, role, segment)] = radii(&inexact)[..] else {
        panic!("the inexact fit records one emission too: {:?}", radii(&inexact));
    };
    assert_eq!((step, role), (3, RadiusRole::Fillet));
    let got = stored_radius(&inexact, segment).expect("and it is an arc as well");
    assert!((got - 0.5).abs() < 1e-9, "at its own radius, not {got}");
}

/// **A radius-bearing LEG records its carrier on its own step,
/// wherever the leg sits** — first emitting step, middle, or last
/// before the closer.
///
/// `family::arc_to_kernel` is the one emission site that addresses
/// `current_step()` rather than a binder's `bound_at`, and the PR's
/// deviation 1 rests on the two coinciding there. A leg at three
/// positions of one chain is what measures that: each `Sweep` and
/// `ArcLen` leg's emission names the step that authored it, and the
/// straight legs between them contribute nothing.
#[test]
fn a_radius_bearing_leg_records_its_own_step_at_every_position() {
    let walk = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, tol())
        .unwrap()
        .arc_to(
            Sweep {
                r: 2.0,
                side: ArcSide::Left,
                angle: 0.5,
            },
            tol(),
        )
        .unwrap()
        .line(1.0, tol())
        .unwrap()
        .tangent()
        .arc_to(
            ArcLen {
                r: 3.0,
                side: ArcSide::Left,
                len: 1.5,
            },
            tol(),
        )
        .unwrap()
        .line(1.0, tol())
        .unwrap()
        .tangent()
        .arc_to(
            Sweep {
                r: 1.5,
                side: ArcSide::Left,
                angle: 0.7,
            },
            tol(),
        )
        .unwrap()
        .line_to(Start, tol())
        .unwrap();
    assert_eq!(
        radii(&walk),
        vec![
            (2, RadiusRole::Carrier, 0),
            (5, RadiusRole::Carrier, 2),
            (8, RadiusRole::Carrier, 4),
        ],
        "each leg's carrier radius names its own step and its own segment"
    );
    for (want, (_, _, segment)) in [2.0, 3.0, 1.5].into_iter().zip(radii(&walk)) {
        let got = stored_radius(&walk, segment).expect("the named segment is an arc");
        assert!(
            (got - want).abs() < 1e-6,
            "segment {segment} is stored at {got}, not the {want} its address names"
        );
    }
    // And the guided arm reproduces the same list, which is what makes
    // the record consumable rather than carried along unread.
    let program: Vec<_> = walk.program.clone();
    let (_, recorded) = replay_recording(&program, tol()).expect("the walk replays recording");
    assert_eq!(recorded.radii, walk.structure.radii);
    replay_guided(&program, &recorded, tol()).expect("and replays guided against its own record");
}

/// **No two emissions of one loop name the same segment.**
///
/// `eval::wire::edge_radii` reads the door's answer with
/// `.find(|(e, _)| *e == want)` — the FIRST pair naming a canonical
/// segment wins and any second is dropped without a word. That is
/// sound only while one segment carries at most one emission, and
/// nothing in the types says so, so it is measured here over every
/// chain the coverage corpus holds plus the three-radius fused shape
/// and the arc-arrival fused shape the corpus lacks.
#[test]
fn no_two_emissions_of_one_loop_name_the_same_segment() {
    let three = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, tol())
        .unwrap()
        .line(4.0, tol())
        .unwrap()
        .tangent()
        .arc_fillet_arc(
            Sweep {
                r: 2.0,
                side: ArcSide::Left,
                angle: 0.6,
            },
            0.25,
            Radius {
                r: 3.0,
                side: ArcSide::Left,
            },
            tol(),
        )
        .unwrap()
        .at(p2(2.0, 6.0))
        .toward(-1.0, 0.0, tol())
        .unwrap()
        .line(2.0, tol())
        .unwrap()
        .line_to(Start, tol())
        .unwrap();
    let mut all = coverage_corpus();
    all.push(three);
    for (i, closed) in all.iter().enumerate() {
        let mut seen: Vec<usize> = closed.structure.radii.iter().map(|e| e.segment).collect();
        let n = seen.len();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(
            seen.len(),
            n,
            "chain {i} records two radii on one segment, which `edge_radii`'s \
             first-match read would silently drop one of: {:?}",
            closed.structure.radii
        );
        for e in &closed.structure.radii {
            assert!(
                e.segment < closed.loop_.vertices().len(),
                "chain {i}: {e} names a segment the loop does not have"
            );
        }
    }
}

/// **The coverage corpus — the guided fence's positive half — reaches
/// no `Carrier2` emission at all.**
///
/// `path_program`'s rows pin the recording side of every role, and
/// `guided_replay`'s two rows pin the comparison on a chain whose one
/// emission is a `Fillet`. The claim the PR makes for the POSITIVE
/// half is the coverage-corpus fence row, and this measures what that
/// fence covers: `Fillet` and the `arc_to_kernel` `Carrier`, never the
/// `emit_fillet_in` `Carrier` and never `Carrier2`. Recorded as a
/// MEASUREMENT, so a corpus that later grows the shape moves this row
/// rather than leaving the gap unstated.
#[test]
fn the_coverage_corpus_fence_reaches_no_arrival_carrier_emission() {
    let mut roles: Vec<RadiusRole> = coverage_corpus()
        .iter()
        .flat_map(|c| c.structure.radii.iter().map(|e| e.role))
        .collect();
    roles.dedup();
    roles.sort_by_key(|r| match r {
        RadiusRole::Fillet => 0,
        RadiusRole::Carrier => 1,
        RadiusRole::Carrier2 => 2,
    });
    roles.dedup();
    assert_eq!(
        roles,
        vec![RadiusRole::Fillet, RadiusRole::Carrier],
        "the corpus authors no radius-bearing ARRIVAL spec, so the guided fence's \
         positive half never reproduces a `Carrier2`"
    );
}

/// The guided pass reproduces a `Carrier2` emission — the role the
/// fence row above does not reach — and a record that moves it is
/// refused. The positive half the corpus lacks, authored.
#[test]
fn a_guided_pass_reproduces_and_checks_an_arrival_carrier_emission() {
    let three = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, tol())
        .unwrap()
        .line(4.0, tol())
        .unwrap()
        .tangent()
        .arc_fillet_arc(
            Sweep {
                r: 2.0,
                side: ArcSide::Left,
                angle: 0.6,
            },
            0.25,
            Radius {
                r: 3.0,
                side: ArcSide::Left,
            },
            tol(),
        )
        .unwrap()
        .at(p2(2.0, 6.0))
        .toward(-1.0, 0.0, tol())
        .unwrap()
        .line(2.0, tol())
        .unwrap()
        .line_to(Start, tol())
        .unwrap();
    let program = three.program.clone();
    let (_, recorded) = replay_recording(&program, tol()).expect("the fused chain replays");
    assert!(
        recorded
            .radii
            .iter()
            .any(|e| e.role == RadiusRole::Carrier2),
        "the fixture records an arrival carrier: {:?}",
        recorded.radii
    );
    replay_guided(&program, &recorded, tol()).expect("guided against its own record");
    let mut lie = recorded.clone();
    let at = lie
        .radii
        .iter()
        .position(|e| e.role == RadiusRole::Carrier2)
        .expect("the arrival carrier is recorded");
    lie.radii[at].role = RadiusRole::Carrier;
    replay_guided(&program, &lie, tol()).expect_err("a swapped role is refused");
    let mut moved = recorded.clone();
    moved.radii[at].segment += 1;
    replay_guided(&program, &moved, tol()).expect_err("a moved segment is refused");
    let mut reordered = recorded.clone();
    reordered.radii.swap(0, at);
    replay_guided(&program, &reordered, tol())
        .expect_err("the same emissions in a different ORDER are refused");
}

/// The unused import guard: `Point2` is named by [`stored_radius`]'s
/// arithmetic through `p2`, and this keeps the suite honest about it.
#[allow(dead_code)]
fn _point_type(p: Point2<f64>) -> Point2<f64> {
    p
}

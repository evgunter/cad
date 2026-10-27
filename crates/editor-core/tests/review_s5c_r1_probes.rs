//! Review probes for PR 5810 (stage 5 C), reviewer lane `ciLkyX`.
//! Each states the spec's reading; a red one is the finding.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture::{fname, in_world, insert, len, on_frame, place};
use editor_core::{
    AssertionRelation, CheckEvidence, CheckId, ChecksConfig, Formula, Frame, MeasurePrimitive,
    Node, Overlap, ProfileDoc, RecipeNodeId, RoleSeg, Severity, SitedRef, enforce_checks,
};
use geom_core::{Tol, Turns};

fn gap(
    doc: ProfileDoc,
    outer: SitedRef,
    inner: SitedRef,
    relation: AssertionRelation,
    bound: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, measure) = crate::fixture::measure_node(
        &doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        vec![outer, inner],
    );
    let value: Formula = crate::fixture::value_of(&doc, measure);
    insert(
        doc,
        Node::Assertion {
            value,
            bound: len(bound),
            relation,
        },
    )
}

/// The in-band L of `an_in_band_pair_is_no_overlaps_evidence`, whose
/// overlap is `Overlap::Unlocalized` at f64.
fn in_band_l() -> ProfileDoc {
    let g = 3.0 * Tol::witness().eps();
    let doc = ProfileDoc::empty_derived("r1-unlocalized", Tol::witness());
    let ext = |doc, loop_: Vec<(f64, f64)>, h| {
        let (doc, profile) = on_frame(doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![loop_]);
        insert(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(h),
            },
        )
    };
    let (doc, a) = ext(doc, crate::fixture::square(0.0, 0.0, 0.5), 1.0);
    let (doc, l) = ext(
        doc,
        vec![
            (0.45, 0.25),
            (1.05, 0.25),
            (1.05, 1.2),
            (0.0, 1.2),
            (0.0, 1.0 + g),
            (0.45, 1.0 + g),
        ],
        0.6,
    );
    let stand = Frame::rotate_by_turns_then_translate(
        [1.0, 0.0, 0.0],
        Turns::new(1, 4).expect("a quarter turn"),
        [0.0, 0.3, 0.0],
        crate::fixture::band(),
    )
    .expect("a direction");
    let (doc, _) = place(doc, a);
    in_world(doc, l, &stand).0
}

/// Spec §4 "Severity and the refusing path" and CHECKS DS6: an
/// unlocalized overlap ("an overlap the kernel could not intersect") is
/// loud and never refuses, at any severity.
#[test]
fn p1_an_unlocalized_overlap_never_refuses() {
    let doc = in_band_l();
    let run = corpus::eval::<f64>(&doc);
    let error = ChecksConfig {
        at_rest: Severity::Error,
        ..ChecksConfig::default()
    };
    let report = editor_core::run_checks(&doc, &run, &error, Tol::witness()).expect("runs");
    let unlocalized: Vec<_> = report
        .findings
        .iter()
        .filter(|f| {
            f.check == CheckId::AtRest
                && matches!(&f.evidence, CheckEvidence::Interference { finding }
                    if matches!(finding.overlap, Overlap::Unlocalized(_)))
        })
        .collect();
    assert!(
        !unlocalized.is_empty(),
        "premise: an unlocalized overlap: {report}"
    );
    let refused = enforce_checks(&report, &error);
    assert!(
        refused.is_ok(),
        "an unlocalized overlap refused at Error: {:?}",
        refused.err().map(|r| r.findings)
    );
}

/// Two parallel convex shafts clashing 0.1 deep (A r=1 at the origin,
/// B r=0.3 at 1.2 from it), with `Gap(A, B) <= -0.01` holding: no
/// role is a socket, so the pair is not opposed and the finding is
/// loud at EVERY bearing of B about A. `opposed` reads the pairing at
/// one point of A's wall (its outer loop's start): turning B about A
/// moves which side of A that point faces.
#[test]
fn p2_convex_shafts_stay_loud_at_every_bearing() {
    use editor_core::{LoopProgram, ProfileProgram};
    let mut quiet_at = Vec::new();
    for k in 0..8u32 {
        let phi = f64::from(k) * std::f64::consts::FRAC_PI_4;
        let (cx, cy) = (1.2 * phi.cos(), 1.2 * phi.sin());
        let rod = |doc: ProfileDoc, cx: f64, cy: f64, r: f64, h: f64| {
            let (doc, profile) = insert(
                doc,
                Node::Profile(ProfileProgram {
                    loops: vec![LoopProgram::<Formula>::circle(cx, cy, r).expect("a circle")],
                    ids: Vec::new(),
                }),
            );
            insert(
                doc,
                Node::Extrude {
                    profile: profile.into(),
                    distance: len(h),
                },
            )
        };
        let doc = ProfileDoc::empty_derived("r1-shafts", Tol::witness());
        let (doc, a) = rod(doc, 0.0, 0.0, 1.0, 1.0);
        let (doc, b) = rod(doc, cx, cy, 0.3, 0.5);
        let (doc, p) = place(doc, a);
        let (doc, q) = in_world(doc, b, &Frame::translation([0.0, 0.0, 0.25]));
        let lateral = |doc: &ProfileDoc, node| {
            fname(
                node,
                RoleSeg::Lateral(crate::fixture::piece(doc, node, 0, 0).into()),
            )
        };
        let (fa, fb) = (lateral(&doc, a), lateral(&doc, b));
        let (doc, assertion) = gap(
            doc,
            SitedRef::new(p, fa),
            SitedRef::new(q, fb),
            AssertionRelation::AtMost,
            -0.01,
        );
        let run = corpus::eval::<f64>(&doc);
        let verdict = format!("{:?}", run.value(assertion).map(|v| &v.payload));
        let findings = crate::fixture::at_rest(&doc, &run).expect("runs");
        let interference = crate::fixture::interference(&findings);
        eprintln!(
            "bearing {k}/8 turn: {verdict}; {} finding(s), quiet: {:?}",
            interference.len(),
            interference
                .iter()
                .map(|f| f.quiet.is_some())
                .collect::<Vec<_>>()
        );
        if interference.iter().any(|f| f.quiet.is_some()) {
            quiet_at.push(k);
        }
    }
    assert!(
        quiet_at.is_empty(),
        "a convex-convex clash quieted at bearings {quiet_at:?} (eighths of a turn)"
    );
}

/// Deviation 1 (`less_carried`): the host instances the in-band part
/// once, and places its OWN copy of the same two blocks, the same
/// three ε apart, 8 m along y. The host's own pair is a census verdict
/// between two of the host's arrivals: it is reported as the host's,
/// beside the part's carried rows, whatever the part carried.
#[test]
fn p3_a_hosts_own_escalation_equal_to_a_parts_is_reported() {
    use editor_core::{AtRestEvidence, EvalOptions};
    let part = crate::intent_part_census_seam::in_band("r1-in-band-part");
    let run = |doc: &ProfileDoc, opts: &EvalOptions| {
        editor_core::evaluate::<f64>(
            doc,
            None,
            &editor_core::CancelToken::new(),
            opts,
            Tol::witness(),
        )
    };
    let own = crate::fixture::could_not_look(
        &crate::fixture::at_rest(&part, &run(&part, &EvalOptions::default())).expect("runs"),
    );
    eprintln!("part's own: {own:?}");
    let (host, _, opts) = crate::intent_part_census_seam::instantiating("r1-host", &part, 1);
    let ext = |doc, (cx, cy): (f64, f64), h: f64, height: f64, at: [f64; 3]| {
        let (doc, profile) = on_frame(
            doc,
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![crate::fixture::square(cx, cy, h)],
        );
        let (doc, body) = insert(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(height),
            },
        );
        in_world(doc, body, &Frame::translation(at))
    };
    let g = 3.0 * Tol::witness().eps();
    let (host, _) = ext(host, (0.0, 0.0), 0.5, 1.0, [0.0, 8.0, 0.0]);
    let (host, _) = ext(host, (0.5 + g + 0.3, 0.0), 0.3, 0.5, [0.0, 8.0, 0.25]);
    let found = crate::fixture::at_rest(&host, &run(&host, &opts)).expect("runs");
    let blind = crate::fixture::could_not_look(&found);
    let carried = blind
        .iter()
        .filter(|e| matches!(e, AtRestEvidence::Carried(_)))
        .count();
    let hosts: Vec<_> = blind
        .iter()
        .filter(|e| !matches!(e, AtRestEvidence::Carried(_)))
        .collect();
    eprintln!("carried {carried}, host's own {}: {hosts:?}", hosts.len());
    assert_eq!(carried, own.len(), "the part's rows, once each");
    assert_eq!(
        hosts.len(),
        own.len(),
        "the host's own copy of the pair is reported as the host's"
    );
}

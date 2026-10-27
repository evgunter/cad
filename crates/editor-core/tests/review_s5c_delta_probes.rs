//! Review probes for PR 5810's fix passes (lane delta): does a finding
//! refuse at `Error` exactly when an assertion could have quieted it?
//!
//! Each probe builds a bounded overlap that `quietable` calls quietable,
//! then tries every holding negative `Gap` over every pair of its
//! bounding faces (one of each copy, both role orders) and asks whether
//! any one quiets it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture::{ang, fname, in_world, insert, len, len2, piece, place, scl, slid, square};
use editor_core::{
    AssertionRelation, AssertionVerdict, Bodies, ChecksConfig, Formula, InterferenceFinding,
    LoopProgram, MeasurePrimitive, Node, Overlap, ProfileDoc, ProfileProgram, ProgramArcData,
    ProgramStep, ProgramTarget, RecipeNodeId, RoleSeg, Severity, SitedRef, ValuePayload,
    enforce_checks,
};
use geom_core::Tol;

fn report(doc: &ProfileDoc) -> editor_core::ChecksReport {
    let run = corpus::eval::<f64>(doc);
    editor_core::run_checks(doc, &run, &ChecksConfig::default(), Tol::witness())
        .unwrap_or_else(|e| panic!("the checks run: {e}"))
}

fn findings(doc: &ProfileDoc) -> Vec<InterferenceFinding> {
    crate::fixture::interference(&report(doc).findings)
}

fn refused_at_error(doc: &ProfileDoc) -> bool {
    let error = ChecksConfig {
        at_rest: Severity::Error,
        ..ChecksConfig::default()
    };
    enforce_checks(&report(doc), &error).is_err()
}

fn prism(doc: ProfileDoc, loops: Vec<LoopProgram<Formula>>, h: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops,
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
}

fn circle(cx: f64, cy: f64, r: f64) -> LoopProgram<Formula> {
    LoopProgram::<Formula>::circle(cx, cy, r).expect("a literal circle")
}

fn outline(h: f64) -> LoopProgram<Formula> {
    crate::fixture::desc(vec![square(0.0, 0.0, h)])
        .loops
        .remove(0)
}

/// Every holding `Gap ≤ -1e-5` over a pair of `finding`'s bounding faces
/// (one per copy, both orders): `(outer, inner, holds, quiets)`.
fn try_every_pair(
    doc: &ProfileDoc,
    finding: &InterferenceFinding,
) -> Vec<(String, String, bool, bool)> {
    let Overlap::Bounded { faces } = &finding.overlap else {
        panic!("bounded: {finding:?}")
    };
    let node = |var| doc.operation_of(var).expect("a placement");
    let mut out = Vec::new();
    for x in faces.iter().filter(|s| s.copy == finding.a) {
        for y in faces.iter().filter(|s| s.copy == finding.b) {
            for (o, i) in [(x, y), (y, x)] {
                let (d, measure) = crate::fixture::measure_node(
                    doc,
                    MeasurePrimitive::Gap { outer: 0, inner: 1 },
                    vec![
                        SitedRef::new(node(o.copy.var), o.face.clone()),
                        SitedRef::new(node(i.copy.var), i.face.clone()),
                    ],
                );
                let value = crate::fixture::value_of(&d, measure);
                let (d, assertion) = insert(
                    d,
                    Node::Assertion {
                        value,
                        bound: len(-1e-5),
                        relation: AssertionRelation::AtMost,
                    },
                );
                let run = corpus::eval::<f64>(&d);
                let holds = matches!(
                    run.value(assertion).map(|v| &v.payload),
                    Some(ValuePayload::Assertion(AssertionVerdict::Holds { .. }))
                );
                let fs = findings(&d);
                let quiets = holds && fs.iter().any(|f| f.quiet == Some(assertion));
                if holds {
                    for f in &fs {
                        let kind = match &f.overlap {
                            Overlap::Bounded { .. } => "Bounded".to_owned(),
                            Overlap::Unlocalized(u) => format!("{u:?}").chars().take(120).collect(),
                        };
                        eprintln!(
                            "    held -> finding quietable={} quiet={:?} overlap={kind}",
                            f.quietable, f.quiet
                        );
                    }
                }
                out.push((
                    format!("{:?}", o.face.path),
                    format!("{:?}", i.face.path),
                    holds,
                    quiets,
                ));
            }
        }
    }
    out
}

/// The quietable finding no candidate quiets: prints the table, and
/// asserts the claim "refuses at Error exactly when an assertion could
/// have quieted it".
fn claim(label: &str, doc: &ProfileDoc) {
    let found = findings(doc);
    let [finding] = found.as_slice() else {
        panic!("{label}: one finding, not {found:?}")
    };
    let refused = refused_at_error(doc);
    let tried = try_every_pair(doc, finding);
    let held = tried.iter().filter(|t| t.2).count();
    let quieted = tried.iter().filter(|t| t.3).count();
    eprintln!(
        "{label}: quietable={} refused_at_error={refused} pairs tried={} held={held} quieted={quieted} evidence={:?}",
        finding.quietable,
        tried.len(),
        finding
            .evidence
            .iter()
            .map(|e| format!("{e:?}").chars().take(60).collect::<String>())
            .collect::<Vec<_>>()
    );
    for t in &tried {
        eprintln!("  {t:?}");
    }
    assert_eq!(
        refused,
        quieted > 0,
        "{label}: refuses at Error ({refused}) but an assertion quiets it: {}",
        quieted > 0
    );
}

/// D1: a pin bottoming through a blind bore (`intent_s5_b_interference`'s
/// fixture). Quietable (bore/pin, floor/pin-end), yet the slug crosses the
/// bore's carrier and the annulus crosses the floor's.
#[test]
fn d1_a_pin_bottoming_in_a_blind_bore() {
    let doc = ProfileDoc::empty_derived("review-d1-blind", Tol::witness());
    let (doc, holed) = prism(doc, vec![outline(1.0), circle(0.0, 0.0, 0.5)], 1.0);
    let (doc, floor) = prism(doc, vec![outline(1.0)], 0.5);
    let (doc, floor) = slid(doc, holed, floor, [0.0, 0.0, -0.5]);
    let (doc, plate) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![holed.into(), floor.into()]),
        },
    );
    let (doc, pin) = prism(doc, vec![circle(0.0, 0.0, 0.505)], 1.7);
    let (doc, _) = place(doc, plate);
    let (doc, _) = in_world(doc, pin, &editor_core::Frame::translation([0.0, 0.0, -0.2]));
    claim("d1 blind bore", &doc);
}

/// D2: a headed pin pressed through a bore, its head's underside resting
/// on the plate's top: the touch rides the pair's evidence (`covered`).
#[test]
fn d2_a_headed_press_fit() {
    let doc = ProfileDoc::empty_derived("review-d2-headed", Tol::witness());
    let (doc, plate) = prism(doc, vec![outline(1.0), circle(0.0, 0.0, 0.5)], 1.0);
    let (doc, shank) = prism(doc, vec![circle(0.0, 0.0, 0.505)], 1.5);
    let (doc, head) = prism(doc, vec![circle(0.0, 0.0, 0.8)], 0.2);
    let (doc, head) = slid(doc, shank, head, [0.0, 0.0, 1.5]);
    let (doc, pin) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![shank.into(), head.into()]),
        },
    );
    let (doc, _) = place(doc, plate);
    let (doc, _) = in_world(doc, pin, &editor_core::Frame::translation([0.0, 0.0, -0.5]));
    claim("d2 headed press", &doc);
}

/// D3: a ball pressed into a concentric spherical socket (a hollow shell).
/// `gap_covers` accepts concentric spheres; `beyond` has no sphere arm.
#[test]
fn d3_a_ball_in_a_socket() {
    let ball = |doc: ProfileDoc, r: f64| {
        let meridian = LoopProgram::Chain(vec![
            ProgramStep::At(len2([0.0, -r])),
            ProgramStep::ArcTo(ProgramArcData::Bulge {
                target: ProgramTarget::Point(len2([0.0, r])),
                b: scl(1.0),
            }),
            ProgramStep::LineTo(ProgramTarget::Start),
        ]);
        let (doc, p) = insert(
            doc,
            Node::Profile(ProfileProgram {
                loops: vec![meridian],
                ids: Vec::new(),
            }),
        );
        insert(
            doc,
            crate::fixture::revolve_about(
                p,
                (0.0, 0.0),
                crate::fixture::along_y(),
                ang(std::f64::consts::TAU),
            ),
        )
    };
    let doc = ProfileDoc::empty_derived("review-d3-ball", Tol::witness());
    let (doc, outer) = ball(doc, 1.0);
    let (doc, cavity) = ball(doc, 0.5);
    let (doc, cav) = slid(
        doc,
        editor_core::Operand::output(outer, 0),
        editor_core::Operand::output(cavity, 0),
        [0.0, 0.0, 0.0],
    );
    let (doc, socket) = insert(
        doc,
        Node::Subtract {
            from: editor_core::Operand::output(outer, 0).into(),
            tool: cav.into(),
        },
    );
    let (doc, b) = ball(doc, 0.52);
    let (doc, _) = place(doc, socket);
    let bv = editor_core::Operand::Var(doc.output(b, 0).unwrap());
    let (doc, _) = in_world(doc, bv, &editor_core::Frame::translation([0.0, 0.0, 0.0]));
    let found = findings(&doc);
    for f in &found {
        eprintln!(
            "d3 finding: quietable={} quiet={:?} overlap={:?}",
            f.quietable, f.quiet, f.overlap
        );
    }
    claim("d3 ball in socket", &doc);
}

/// D4: one part of two pins pressed into a plate's two bores: each
/// component's undecided curved pairs sit on the other's overlap.
#[test]
fn d4_two_press_fits_between_one_pair() {
    let doc = ProfileDoc::empty_derived("review-d4-two", Tol::witness());
    let (doc, plate) = prism(
        doc,
        vec![outline(1.0), circle(-0.5, 0.0, 0.2), circle(0.5, 0.0, 0.2)],
        1.0,
    );
    let (doc, left) = prism(doc, vec![circle(-0.5, 0.0, 0.205)], 2.0);
    let (doc, right) = prism(doc, vec![circle(0.5, 0.0, 0.205)], 2.0);
    let (doc, right_in) = slid(doc, left, right, [0.0, 0.0, 0.0]);
    let (doc, pins) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![left.into(), right_in.into()]),
        },
    );
    let (doc, _) = place(doc, plate);
    let (doc, _) = in_world(
        doc,
        pins,
        &editor_core::Frame::translation([0.0, 0.0, -0.5]),
    );
    let found = findings(&doc);
    eprintln!("d4: {} findings", found.len());
    for f in &found {
        eprintln!(
            "  quietable={} quiet={:?} evidence={:?}",
            f.quietable, f.quiet, f.evidence
        );
    }
    // Every pair of each finding's bounding faces, both orders, asserted
    // at once: whatever could quiet either component is present.
    let mut d = doc.clone();
    let mut ids = Vec::new();
    let node = |doc: &ProfileDoc, var| doc.operation_of(var).expect("a placement");
    for f in &found {
        let Overlap::Bounded { faces } = &f.overlap else {
            panic!("bounded")
        };
        for x in faces.iter().filter(|s| s.copy == f.a) {
            for y in faces.iter().filter(|s| s.copy == f.b) {
                for (o, i) in [(x, y), (y, x)] {
                    let (d2, measure) = crate::fixture::measure_node(
                        &d,
                        MeasurePrimitive::Gap { outer: 0, inner: 1 },
                        vec![
                            SitedRef::new(node(&doc, o.copy.var), o.face.clone()),
                            SitedRef::new(node(&doc, i.copy.var), i.face.clone()),
                        ],
                    );
                    let value = crate::fixture::value_of(&d2, measure);
                    let (d2, a) = insert(
                        d2,
                        Node::Assertion {
                            value,
                            bound: len(-1e-5),
                            relation: AssertionRelation::AtMost,
                        },
                    );
                    d = d2;
                    ids.push(a);
                }
            }
        }
    }
    let run = corpus::eval::<f64>(&d);
    let held = ids
        .iter()
        .filter(|a| {
            matches!(
                run.value(**a).map(|v| &v.payload),
                Some(ValuePayload::Assertion(AssertionVerdict::Holds { .. }))
            )
        })
        .count();
    eprintln!("  {} assertions, {held} hold", ids.len());
    let after = findings(&d);
    for f in &after {
        eprintln!("  asserted: quietable={} quiet={:?}", f.quietable, f.quiet);
    }
    let refused = refused_at_error(&d);
    eprintln!("  asserted: refused at Error = {refused}");
    assert!(
        after.iter().all(|f| !f.is_loud()) || !after.iter().any(|f| f.quietable && f.is_loud()),
        "a quietable finding stays loud under both bores' gaps"
    );
}

// Results at b718e670 (all three ε): d1 green (Gap(plate top, pin bottom)
// quiets the bottoming pin); d2, d3, d4 red: quietable, refused at Error,
// and no holding Gap over any pair of bounding faces quiets them. d3's
// holding assertions turn the finding into Unlocalized::Containment
// ("the containment check has no lane for this carrier"), quietable=false.

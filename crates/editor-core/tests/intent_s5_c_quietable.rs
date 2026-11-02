//! **INTENT stage 5 C — a finding refuses only where an assertion could
//! have quieted it** (spec §4 "Severity and the refusing path"; CHECKS
//! DS6; D10 `[assertions.quiet]`).
//!
//! `InterferenceFinding::quietable` asks the quieting rule itself: a
//! finding is quietable exactly when some holding `Gap ≤ −δ` over a pair
//! of its bounding faces (one of each copy, either role order) would
//! quiet it, and `enforce_checks` at `Error` refuses exactly the loud
//! quietable ones. Each row authors every such candidate in turn and
//! holds the finding to both.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture::{ang, in_world, insert, len, len2, place, scl, slid, square};
use editor_core::{
    AssertionRelation, AssertionVerdict, Bodies, ChecksConfig, FaceSite, Formula, Frame,
    InterferenceFinding, LoopProgram, MeasurePrimitive, Node, Overlap, ProfileDoc, ProfileProgram,
    ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, Severity, SitedRef, ValuePayload,
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
            on: None,
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

/// `doc` with a holding-or-not `Gap(outer, inner) ≤ −1e-5`, and the
/// assertion.
fn with_gap(doc: &ProfileDoc, outer: &FaceSite, inner: &FaceSite) -> (ProfileDoc, RecipeNodeId) {
    let node = |var| {
        doc.operation_of(var)
            .expect("a copy is a placement's output")
    };
    let (d, measure) = crate::fixture::measure_node(
        doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        vec![
            SitedRef::new(node(outer.copy.var), outer.face.clone()),
            SitedRef::new(node(inner.copy.var), inner.face.clone()),
        ],
    );
    let value = crate::fixture::value_of(&d, measure);
    insert(
        d,
        Node::Assertion { claim: editor_core::Claim::Value {
            value,
            bound: len(-1e-5),
            relation: AssertionRelation::AtMost,
        } },
    )
}

/// **The rule `quietable` states, held on `doc`**: for each of its
/// interference findings, whether some holding `Gap ≤ −1e-5` over a pair
/// of its bounding faces (one of each copy, both role orders) quiets it
/// is exactly `quietable`; and `Error` refuses exactly when a loud
/// quietable finding is present. Answers each finding's `quietable`.
fn holds_the_rule(label: &str, doc: &ProfileDoc) -> Vec<bool> {
    let found = findings(doc);
    assert!(!found.is_empty(), "{label}: the premise, an overlap");
    let mut quietable = Vec::new();
    for finding in &found {
        let Overlap::Bounded { faces } = &finding.overlap else {
            assert!(
                !finding.quietable,
                "{label}: an unlocalized overlap: {finding:?}"
            );
            quietable.push(false);
            continue;
        };
        let mut quieted = false;
        for x in faces.iter().filter(|s| s.copy == finding.a) {
            for y in faces.iter().filter(|s| s.copy == finding.b) {
                for (outer, inner) in [(x, y), (y, x)] {
                    let (d, assertion) = with_gap(doc, outer, inner);
                    let run = corpus::eval::<f64>(&d);
                    let holds = matches!(
                        run.value(assertion).map(|v| &v.payload),
                        Some(ValuePayload::Assertion(AssertionVerdict::Holds { .. }))
                    );
                    quieted |= holds
                        && findings(&d).iter().any(|f| {
                            matches!(&f.overlap, Overlap::Bounded { faces: held } if held == faces)
                                && f.quiet == Some(assertion)
                        });
                }
            }
        }
        assert_eq!(
            finding.quietable, quieted,
            "{label}: quietable is whether an assertion quiets it: {finding:?}"
        );
        quietable.push(quieted);
    }
    assert_eq!(
        refused_at_error(doc),
        found.iter().any(|f| f.quietable && f.is_loud()),
        "{label}: Error refuses exactly a loud quietable finding"
    );
    quietable
}

/// **A pin bottoming through a blind bore is quietable, and refuses at
/// `Error` until quieted.** Its overlap lies between the plate's top and
/// the pin's bottom, so a `Gap` over those quiets it, though one over the
/// bore and the pin's wall does not (the slug past the floor crosses the
/// bore's carrier).
#[test]
fn a_blind_bore_pin_is_quietable_and_refuses_until_quieted() {
    let doc = ProfileDoc::empty_derived("s5c-q-blind", Tol::witness());
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
    let (doc, _) = in_world(doc, pin, &Frame::translation([0.0, 0.0, -0.2]));
    assert_eq!(holds_the_rule("blind bore", &doc), vec![true]);
}

/// **A coaxial press fit is quietable** (a curved pair): a `Gap` between
/// the bore and the pin's wall quiets it, and unasserted it refuses at
/// `Error`.
#[test]
fn a_coaxial_press_fit_is_quietable() {
    let (doc, ..) = crate::intent_s5_b_interference::bore_and_pin(0.505);
    assert_eq!(holds_the_rule("press fit", &doc), vec![true]);
}

/// **A headed press fit is not quietable, and refuses at no severity.**
/// The head's underside rests on the plate's top, a touch that rides the
/// pair's evidence and that no assertion over the overlap speaks for
/// (`localize`'s `covered`), so no holding `Gap` quiets it.
#[test]
fn a_headed_press_fit_is_not_quietable() {
    let doc = ProfileDoc::empty_derived("s5c-q-headed", Tol::witness());
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
    let (doc, _) = in_world(doc, pin, &Frame::translation([0.0, 0.0, -0.5]));
    assert_eq!(holds_the_rule("headed press", &doc), vec![false]);
}

/// **A ball in a concentric socket is not quietable, and refuses at no
/// severity.** The containment check has no sphere lane, so a holding
/// `Gap` between the socket and the ball turns the finding unlocalized
/// rather than quieting it
/// (`work/atrest/the-containment-check-has-no-sphere-lane.md`).
#[test]
fn a_ball_in_a_concentric_socket_is_not_quietable() {
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
                on: None,
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
    let doc = ProfileDoc::empty_derived("s5c-q-ball", Tol::witness());
    let (doc, outer) = ball(doc, 1.0);
    let (doc, cavity) = ball(doc, 0.5);
    let (doc, cavity) = slid(
        doc,
        editor_core::Operand::output(outer, 0),
        editor_core::Operand::output(cavity, 0),
        [0.0, 0.0, 0.0],
    );
    let (doc, socket) = insert(
        doc,
        Node::Subtract {
            from: editor_core::Operand::output(outer, 0).into(),
            tool: cavity.into(),
        },
    );
    let (doc, b) = ball(doc, 0.52);
    let (doc, _) = place(doc, socket);
    let b = editor_core::Operand::Var(doc.output(b, 0).expect("a ball"));
    let (doc, _) = in_world(doc, b, &Frame::translation([0.0, 0.0, 0.0]));
    assert_eq!(holds_the_rule("ball in socket", &doc), vec![false]);
}

/// **Two pins in two bores of one pair are not quietable, and refuse at
/// no severity.** Each overlap's undecided curved pairs lie on the other's
/// overlap, which no assertion over the one speaks for (`covered`).
#[test]
fn two_press_fits_between_one_pair_are_not_quietable() {
    let doc = ProfileDoc::empty_derived("s5c-q-two", Tol::witness());
    let (doc, plate) = prism(
        doc,
        vec![outline(1.0), circle(-0.5, 0.0, 0.2), circle(0.5, 0.0, 0.2)],
        1.0,
    );
    let (doc, left) = prism(doc, vec![circle(-0.5, 0.0, 0.205)], 2.0);
    let (doc, right) = prism(doc, vec![circle(0.5, 0.0, 0.205)], 2.0);
    let (doc, right) = slid(doc, left, right, [0.0, 0.0, 0.0]);
    let (doc, pins) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![left.into(), right.into()]),
        },
    );
    let (doc, _) = place(doc, plate);
    let (doc, _) = in_world(doc, pins, &Frame::translation([0.0, 0.0, -0.5]));
    let quietable = holds_the_rule("two press fits", &doc);
    assert_eq!(quietable, vec![false; 2]);
}

/// The press fit of [`crate::intent_s5_b_interference::bore_and_pin`],
/// the plate and the pin both stood at `pose` (the pin 0.5 down the
/// plate's axis first): `(doc, plate copy, pin copy, bore, pin wall)`.
fn tilted_press(
    pin_r: f64,
    pose: geom_core::linalg::Affine3<f64>,
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    editor_core::StableName,
    editor_core::StableName,
) {
    use crate::fixture::{fname, piece};
    use geom_core::linalg::{Affine3, Mat3, Vec3};
    let doc = ProfileDoc::empty_derived("s5c-q-tilted", Tol::witness());
    let (doc, plate) = prism(doc, vec![outline(1.0), circle(0.0, 0.0, 0.5)], 1.0);
    let (doc, pin) = prism(doc, vec![circle(0.0, 0.0, pin_r)], 2.0);
    let (doc, p) = in_world(doc, plate, &Frame::from_affine(pose));
    let down = Affine3::from_parts(Mat3::identity(), Vec3::new(0.0, 0.0, -0.5));
    let (doc, q) = in_world(doc, pin, &Frame::from_affine(pose * down));
    let lateral = |doc: &ProfileDoc, node, l| {
        fname(
            node,
            editor_core::RoleSeg::Lateral(piece(doc, node, l, 0).into()),
        )
    };
    let (bore, wall) = (lateral(&doc, plate, 1), lateral(&doc, pin, 0));
    (doc, p, q, bore, wall)
}

/// **A press fit tilted off every axis is quiet under its `Gap`, as at
/// the identity, and the quieting rule decides its axes through the
/// funnel.** Turned and moved, the bore's and the pin's axes agree only
/// to rounding, so whether they are one line is a decision at the band
/// (`at_rest_quiet_axes_parallel`, `at_rest_quiet_centred`), not an exact
/// comparison. The bore and the pin are opposed, so the gap reads −0.005
/// in either role order, and `Gap ≤ −0.001` holds and quiets it at both
/// poses, the bore first or the pin.
///
/// *Breaks if* `constant_pairing` compares the axes raw (the tilted fit
/// stays loud, and no verdict is recorded under either site).
#[test]
fn a_tilted_press_fit_is_quiet_and_its_axes_are_decided_in_band() {
    use geom_core::linalg::{Affine3, Mat3, Vec3};
    let tilt = Affine3::from_parts(
        Mat3::rotation_about(Vec3::new(0.3, -1.0, 0.7), 0.83),
        Vec3::new(1.7, -2.2, 0.9),
    );
    let identity = Affine3::from_parts(Mat3::identity(), Vec3::new(0.0, 0.0, 0.0));
    for (pose_name, pose) in [("identity", identity), ("tilt", tilt)] {
        let (doc, p, q, bore, wall) = tilted_press(0.505, pose);
        let [loud] = findings(&doc)
            .try_into()
            .unwrap_or_else(|f: Vec<_>| panic!("{pose_name}: one finding: {f:?}"));
        assert!(loud.is_loud() && loud.quietable, "{pose_name}: {loud:?}");
        for (why, outer, inner, quiets) in [
            ("bore first", (p, bore.clone()), (q, wall.clone()), true),
            ("pin first", (q, wall.clone()), (p, bore.clone()), true),
        ] {
            let (doc, assertion) = crate::intent_s5_b_interference::gap_at_copies(
                doc.clone(),
                outer,
                inner,
                AssertionRelation::AtMost,
                -0.001,
            );
            let bracket = geom_core::k_stats::Bracket::open();
            let [finding] = findings(&doc)
                .try_into()
                .unwrap_or_else(|f: Vec<_>| panic!("{pose_name} {why}: one finding: {f:?}"));
            let recorded = bracket.finish();
            assert_eq!(
                finding.quiet == Some(assertion),
                quiets,
                "{pose_name} {why}: {finding:?}"
            );
            if quiets {
                for site in ["at_rest_quiet_axes_parallel", "at_rest_quiet_centred"] {
                    assert!(
                        recorded
                            .verdicts
                            .iter()
                            .any(|v| v.predicate == site && v.sign == geom_core::Sign::Zero),
                        "{pose_name}: {site} decided the axes one line in band"
                    );
                }
            }
        }
    }
}

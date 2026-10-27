//! Review probes for stage 5 C (PR 5810), reviewer lane `wjfxCC`.
//! Each row states what the reviewer expected the head to do; a red row
//! is the finding.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::resolver::{PartStore, with_resolver};
use crate::fixture::{in_world, insert, len, on_frame, pin, square};
use editor_core::{
    AtRestEvidence, CancelToken, CheckEvidence, CheckFinding, CheckId, ChecksConfig, EvalOptions,
    Evaluation, Frame, FindingSubject, Node, ProfileDoc, RecipeNodeId, evaluate,
};
use geom_core::Tol;
use geom_core::linalg::{Affine3, Mat3, Vec3};

fn block(doc: ProfileDoc, h: f64, height: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, h)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    )
}

fn report(doc: &ProfileDoc, opts: &EvalOptions) -> editor_core::ChecksReport {
    let run: Evaluation<f64> =
        evaluate::<f64>(doc, None, &CancelToken::new(), opts, Tol::witness());
    editor_core::run_checks(doc, &run, &ChecksConfig::default(), Tol::witness())
        .unwrap_or_else(|e| panic!("the checks run: {e}"))
}

fn copies_rows(report: &editor_core::ChecksReport) -> Vec<CheckFinding> {
    report
        .findings
        .iter()
        .filter(|f| {
            f.check == CheckId::UnprovenCoincidence
                && matches!(f.subject, FindingSubject::Copies { .. })
        })
        .cloned()
        .collect()
}

/// A small block resting on a big one, with no world product: the world
/// `⊔` the fixture authored is still a node, now unread.
fn stacked_unread(label: &str, lift: f64, overlap: bool) -> ProfileDoc {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, big) = block(doc, 0.5, 1.0);
    let (doc, small) = block(doc, 0.3, 0.5);
    let (doc, _) = crate::fixture::place(doc, big);
    let z = if overlap { 0.75 } else { 1.0 + lift };
    let (doc, _) = in_world(doc, small, &Frame::translation([0.0, 0.0, z]));
    crate::fixture::step(
        doc,
        editor_core::DocEdit::SetProduct { product: None },
    )
    .0
}

/// **P1. With no world product, an unread `⊔`'s contact is still the
/// lint's.** The at-rest check gathers the `⊔`s itself under
/// `Subject::EmptyProduct` (its interference is reported, P1b); the
/// lint's `door_rows` reads no at-rest row there, so the contact row
/// should still be reported once. Red = the contact is silently dropped.
#[test]
fn p1_an_unread_assembles_contact_is_reported_without_a_product() {
    let doc = stacked_unread("p1-contact", 0.0, false);
    assert_eq!(doc.product(), None, "the premise: no product");
    let r = report(&doc, &EvalOptions::default());
    assert_eq!(copies_rows(&r).len(), 1, "one contact row: {r}");
}

/// **P1b. The same document's overlap IS reported** (the asymmetry's
/// other half).
#[test]
fn p1b_an_unread_assembles_overlap_is_reported_without_a_product() {
    let doc = stacked_unread("p1b-overlap", 0.0, true);
    let r = report(&doc, &EvalOptions::default());
    let n = r
        .findings
        .iter()
        .filter(|f| matches!(f.evidence, CheckEvidence::Interference { .. }))
        .count();
    assert_eq!(n, 1, "one interference: {r}");
}

/// The in-band part of `intent_part_census_seam`, instantiated once at
/// `pose`.
fn instantiated_at(label: &str, part: &ProfileDoc, pose: &Frame) -> (ProfileDoc, EvalOptions) {
    let mut store = PartStore::new();
    let part = crate::fixture::assemble_world_copies(part.clone());
    let doc_ref = store.insert(part, Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, id) = insert(doc, crate::fixture::mated_instance(doc_ref));
    let doc = pin(doc, id, pose, &opts);
    (doc, opts)
}

fn could_not_look(r: &editor_core::ChecksReport) -> (Vec<AtRestEvidence>, Vec<AtRestEvidence>) {
    let all: Vec<AtRestEvidence> = r
        .findings
        .iter()
        .filter_map(|f| match &f.evidence {
            CheckEvidence::AtRest { evidence } => Some(evidence.clone()),
            _ => None,
        })
        .collect();
    all.into_iter()
        .partition(|e| matches!(e, AtRestEvidence::Carried(_)))
}

/// **P2. `less_carried` under motion.** The in-band part, instantiated
/// once with a pin that turns it (not a pure translation). Every
/// could-not-look finding of the host should be the part's, once:
/// none of the host's own. Red = the census re-found the part's
/// escalation inside the turned copy with a margin that no longer equals
/// the part's, so the by-value discard missed it and it is reported
/// twice (once carried, once as the host's `NoLane`).
#[test]
fn p2_a_turned_instance_of_an_in_band_part_reports_its_findings_once() {
    let part = crate::intent_part_census_seam::in_band("p2-part");
    for (why, pose) in [
        ("a translation", Frame::translation([3.0, 0.0, 0.0])),
        (
            "a turn",
            Frame::from_affine(Affine3::from_parts(
                Mat3::rotation_about(Vec3::new(-2.0, 0.5, 1.0), 2.1),
                Vec3::new(-3.0, 0.5, 7.0),
            )),
        ),
    ] {
        let (doc, opts) = instantiated_at("p2-host", &part, &pose);
        let r = report(&doc, &opts);
        let (carried, own) = could_not_look(&r);
        assert!(!carried.is_empty(), "{why}: the premise, the part's carried rows: {r}");
        assert_eq!(own, Vec::new(), "{why}: nothing of the host's own: {r}");
    }
}

/// A square prism of `side` with its low corner at `(x, y)`, standing
/// in the world with its base at `z` (pncad-py `test_interference.block`).
fn corner_block(doc: ProfileDoc, (x, y): (f64, f64), side: f64, z: f64, height: f64) -> ProfileDoc {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x, y), (x + side, y), (x + side, y + side), (x, y + side)]],
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    );
    in_world(doc, body, &Frame::translation([0.0, 0.0, z])).0
}

/// pncad-py `test_interference.sliver`: two copies 3 nm apart.
fn sliver(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let doc = corner_block(doc, (0.0, 0.0), 1.0, 0.0, 1.0);
    corner_block(doc, (1.0 + 3e-9, 0.2), 0.6, 0.25, 0.5)
}

/// `part` instantiated at each of `poses`.
fn instantiated(label: &str, part: &ProfileDoc, poses: &[Frame]) -> (ProfileDoc, EvalOptions) {
    let mut store = PartStore::new();
    let part = crate::fixture::assemble_world_copies(part.clone());
    let doc_ref = store.insert(part, Tol::witness());
    let opts = with_resolver(store);
    let mut doc = ProfileDoc::empty_derived(label, Tol::witness());
    for pose in poses {
        let (next, id) = insert(doc, crate::fixture::mated_instance(doc_ref));
        doc = pin(next, id, pose, &opts);
    }
    (doc, opts)
}

fn escalations(e: &[AtRestEvidence]) -> usize {
    e.iter()
        .filter(|e| {
            let inner = match e {
                AtRestEvidence::Carried(row) => &row.finding,
                other => other,
            };
            matches!(
                inner,
                AtRestEvidence::NoLane(topo::ValidationError::CensusEscalated { .. })
            )
        })
        .count()
}

/// **P3. `less_carried`'s witness is gone.** The pncad-py lane's sliver
/// part, which deviation 1 was built for (12 + 12 findings), no longer
/// escalates on this head (it reads `ThinInBand`). Once, twice, and turned: every could-not-look finding of the
/// host is the part's, carried once per part row, and none is the
/// host's own. Red = a re-found escalation that no carried row answers
/// is reported again as the host's `NoLane`.
#[test]
fn p3_a_sliver_part_reports_its_escalations_once_however_placed() {
    let part = sliver("p3-part");
    let own = {
        let opts = EvalOptions::default();
        let part = crate::fixture::assemble_world_copies(part.clone());
        let r = report(&part, &opts);
        let (_, own) = could_not_look(&r);
        own
    };
    // The finding: since #5782 the sliver reads `ThinInBand`, which names
    // its pair, so no carried row is an escalation and `less_carried`
    // (deviation 1) has nothing to drop here. Red = the fixture escalates
    // again, and the rows below then exercise the by-value discard.
    assert_eq!(escalations(&own), 0, "the sliver no longer escalates: {own:?}");
    let turn = Frame::from_affine(Affine3::from_parts(
        Mat3::rotation_about(Vec3::new(-2.0, 0.5, 1.0), 2.1),
        Vec3::new(-3.0, 0.5, 7.0),
    ));
    for (why, poses) in [
        ("once", vec![Frame::translation([0.0, 0.0, 0.0])]),
        (
            "twice",
            vec![Frame::translation([0.0, 0.0, 0.0]), Frame::translation([3.0, 0.0, 0.0])],
        ),
        ("turned", vec![turn.clone()]),
    ] {
        let (doc, opts) = instantiated("p3-host", &part, &poses);
        let r = report(&doc, &opts);
        let (carried, host) = could_not_look(&r);
        assert_eq!(carried.len(), own.len(), "{why}: once per part row: {r}");
        assert_eq!(
            host.len(),
            0,
            "{why}: nothing of the host's own ({} escalations): {host:?}",
            escalations(&host)
        );
    }
}

/// The press fit of `intent_s5_b_interference::bore_and_pin`, the plate
/// and the pin both stood at `pose` (the pin 0.5 down the plate's axis
/// first): the same fit, tilted off every world axis.
fn tilted_press(pin_r: f64, pose: Affine3<f64>) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, editor_core::StableName, editor_core::StableName) {
    use crate::fixture::{fname, piece};
    use editor_core::{Formula, LoopProgram, ProfileProgram, RoleSeg};
    let circle = |r| LoopProgram::<Formula>::circle(0.0, 0.0, r).expect("a literal circle");
    let prism = |doc: ProfileDoc, loops, h| {
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
    };
    let doc = ProfileDoc::empty_derived("p4-press", Tol::witness());
    let outline = crate::fixture::desc(vec![square(0.0, 0.0, 1.0)])
        .loops
        .remove(0);
    let (doc, plate) = prism(doc, vec![outline, circle(0.5)], 1.0);
    let (doc, pin) = prism(doc, vec![circle(pin_r)], 2.0);
    let (doc, p) = in_world(doc, plate, &Frame::from_affine(pose));
    let down = Affine3::from_parts(Mat3::identity(), Vec3::new(0.0, 0.0, -0.5));
    let (doc, q) = in_world(doc, pin, &Frame::from_affine(pose * down));
    let lateral = |doc: &ProfileDoc, node, l| {
        fname(node, RoleSeg::Lateral(piece(doc, node, l, 0).into()))
    };
    let (bore, wall) = (lateral(&doc, plate, 1), lateral(&doc, pin, 0));
    (doc, p, q, bore, wall)
}

fn interference(doc: &ProfileDoc) -> Vec<editor_core::InterferenceFinding> {
    let r = report(doc, &EvalOptions::default());
    r.findings
        .iter()
        .filter_map(|f| match &f.evidence {
            CheckEvidence::Interference { finding } => Some((**finding).clone()),
            _ => None,
        })
        .collect()
}

/// **P4. The opposed test on a curved pair, off every axis.** B's press
/// fit (pin 0.505 in a 0.5 bore), tilted and moved: loud unasserted,
/// quiet under a holding `Gap(bore, pin) ≤ −0.001` at the identity and
/// tilted. `Gap(pin, bore)` measures +0.005 (the measure's own role
/// order) and is `Violated`, so it quiets nothing. Red = `opposed`'s
/// one-point reading or its lever reads the tilted pair as not opposed.
#[test]
fn p4_a_tilted_press_fit_is_quiet_in_either_role_order() {
    let tilt = Affine3::from_parts(
        Mat3::rotation_about(Vec3::new(0.3, -1.0, 0.7), 0.83),
        Vec3::new(1.7, -2.2, 0.9),
    );
    let mut table = Vec::new();
    for (pname, pose) in [("identity", Affine3::from_parts(Mat3::identity(), Vec3::new(0.0, 0.0, 0.0))), ("tilt", tilt)] {
        let (doc, p, q, bore, wall) = tilted_press(0.505, pose);
        let [loud] = interference(&doc).try_into().unwrap_or_else(|f: Vec<_>| panic!("one: {f:?}"));
        assert!(loud.is_loud());
        for (why, outer, inner) in [("bore first", (p, bore.clone()), (q, wall.clone())), ("pin first", (q, wall.clone()), (p, bore.clone()))] {
            let (doc, assertion) = crate::intent_s5_b_interference::gap_at_copies(
                doc.clone(),
                outer,
                inner,
                editor_core::AssertionRelation::AtMost,
                -0.001,
            );
            let run: Evaluation<f64> = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default(), Tol::witness());
            let verdict = format!("{:?}", run.value(assertion).map(|v| &v.payload));
            let [finding] = interference(&doc).try_into().unwrap_or_else(|f: Vec<_>| panic!("one: {f:?}"));
            table.push((pname, why, finding.quiet == Some(assertion), verdict.chars().take(160).collect::<String>()));
        }
    }
    // Bore first holds (−0.005) and quiets at both poses; pin first is
    // the Gap's own role order (+0.005, `Violated`), so it quiets nothing.
    for (pose, why, quiet, verdict) in &table {
        let holds = verdict.contains("Holds");
        assert_eq!((*quiet, holds), (*why == "bore first", *why == "bore first"), "{pose} {why}: {table:#?}");
    }
}

/// **P5. An overlap the kernel could not intersect never refuses.**
/// CHECKS DS6: "a finding outside what the document can yet say (... an
/// overlap the kernel could not intersect ...) is reported loud, never
/// refuses whatever the severity, and is not quietable"; spec §4: "an
/// unlocalized overlap is not among them, since `Overlap::Unlocalized` is
/// loud and never refuses". Row 16's `A ⊔ A`: its one interference
/// finding, and `enforce_checks` at `Error`. Red = an `Unlocalized`
/// interference refuses at `Error` (no assertion can quiet it, so the
/// document has no recourse but moving geometry).
#[test]
fn p5_an_unlocalized_overlap_refuses_at_no_severity() {
    let doc = ProfileDoc::empty_derived("p5-twice", Tol::witness());
    let (doc, a) = block(doc, 0.5, 1.0);
    let (doc, copy) = crate::fixture::place(doc, a);
    let read = editor_core::BodyRead::plain(editor_core::Operand::Var(
        doc.output(copy, 0).expect("a copy"),
    ));
    let (doc, twice) = insert(
        doc,
        Node::Assemble {
            members: editor_core::Bodies::Spelled(vec![read.clone(), read]),
        },
    );
    let doc = crate::fixture::step(
        doc,
        editor_core::DocEdit::SetProduct {
            product: Some(editor_core::Operand::Node(twice)),
        },
    )
    .0;
    let mut r = report(&doc, &EvalOptions::default());
    // `A ⊔ A` localizes (`Bounded`), so stand the finding at the overlap
    // the kernel could not intersect: the same finding with its overlap
    // `Unlocalized` (as the tour teapot's handle/vessel finding is). What
    // is under test is `enforce_checks`' filter, which reads `quiet` and
    // `could_not_look()` only.
    let mut n = 0;
    for f in &mut r.findings {
        if let CheckEvidence::Interference { finding } = &mut f.evidence {
            finding.overlap = editor_core::Overlap::Unlocalized(editor_core::Unlocalized::Empty);
            n += 1;
        }
    }
    assert_eq!(n, 1, "one interference: {r}");
    let error = ChecksConfig {
        at_rest: editor_core::Severity::Error,
        ..ChecksConfig::default()
    };
    assert!(
        editor_core::enforce_checks(&r, &error).is_ok(),
        "an unlocalized overlap refuses at no severity"
    );
}

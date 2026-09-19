//! **The edit door's write order is exact**: `SessionOp::EditProfile`
//! refuses `ProfileEditOrder` only when NO order of its one-slot writes
//! lands, as the refusal's sentence claims. Random small polygon edits
//! (a fixed-seed search) are pushed through the door; every one it
//! refuses for order is checked against a brute-force search over
//! every permutation of the same writes, and every one it lands must
//! land where asked, as one undo step.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::insert;
use pncad::document::{Doc, DocEdit, Node, ProfileProgram, RecipeNodeId, apply};
use pncad::geom_core::{Point2, Tol};
use pncad::profile::{Step, Target};
use viewer::session::{DocSession, ProfileShape, Refusal, SessionOp};
use viewer::sketch::{self, Notation};

fn polygon(points: &[(f64, f64)]) -> Vec<Step<f64>> {
    let mut steps = vec![Step::At(Point2::new(points[0].0, points[0].1))];
    for &(x, y) in &points[1..] {
        steps.push(Step::LineTo(Target::Point(Point2::new(x, y))));
    }
    steps.push(Step::LineTo(Target::Start));
    steps
}

fn lowered(steps: &[Step<f64>]) -> Vec<pncad::document::LoopProgram> {
    vec![
        sketch::loop_program(
            &ProfileShape::Path {
                steps: steps.to_vec(),
            },
            Notation::CANONICAL,
        )
        .expect("finite"),
    ]
}

fn with_profile(points: &[(f64, f64)]) -> (DocSession, RecipeNodeId) {
    let tol = Tol::witness();
    let mut session = DocSession::inline(Doc::empty_derived("probe", tol), tol);
    let plane = common::xy_frame_in(&mut session);
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane,
            loops: lowered(&polygon(points)),
        },
    );
    (session, profile)
}

/// The committed program of `node`.
fn base_program(session: &DocSession, node: RecipeNodeId) -> ProfileProgram {
    match session.committed_doc().node(node) {
        Some(Node::Profile(program)) => program.clone(),
        other => panic!("feature {} is not a profile: {other:?}", node.0),
    }
}

/// Every permutation of `edits`, depth-first; true when one applies
/// cleanly all the way.
fn some_order_lands(
    doc: &Doc<ProfileProgram>,
    edits: &[DocEdit<ProfileProgram>],
    used: &mut Vec<bool>,
    tol: Tol,
) -> bool {
    if used.iter().all(|&u| u) {
        return true;
    }
    for i in 0..edits.len() {
        if used[i] {
            continue;
        }
        if let Ok(next) = apply(doc, &edits[i], tol, &pncad::document::RefusingReach) {
            used[i] = true;
            if some_order_lands(&next.doc, edits, used, tol) {
                return true;
            }
            used[i] = false;
        }
    }
    false
}

/// A tiny deterministic LCG, so the search is reproducible.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

#[test]
fn accepted_order_refuses_only_when_no_order_lands() {
    let tol = Tol::witness();
    let mut rng = Lcg(0x5eed);
    let mut refused_order = 0;
    let mut landed = 0;
    let mut counterexamples = Vec::new();
    for _ in 0..3000 {
        // A random simple-ish pentagon on a circle, then 3-4 of its
        // ten coordinates moved.
        let n = 5;
        let base: Vec<(f64, f64)> = (0..n)
            .map(|i| {
                let a = core::f64::consts::TAU * (i as f64 + 0.3 * rng.next()) / n as f64;
                let r = 0.5 + 0.5 * rng.next();
                (r * a.cos(), r * a.sin())
            })
            .collect();
        let mut target = base.clone();
        let moves = 3 + (rng.next() * 2.0) as usize;
        for _ in 0..moves {
            let i = (rng.next() * n as f64) as usize % n;
            let v = 2.0 * rng.next() - 1.0;
            if rng.next() < 0.5 {
                target[i].0 = v;
            } else {
                target[i].1 = v;
            }
        }
        let (mut session, profile) = with_profile(&base);
        let before = session.committed_doc().clone();
        let state = session.history().current();
        let out = session.perform(SessionOp::EditProfile {
            node: profile,
            base: base_program(&session, profile),
            loops: lowered(&polygon(&target)),
        });
        match out.refusal {
            None => {
                landed += 1;
                let Some(Node::Profile(now)) = session.committed_doc().node(profile) else {
                    panic!("profile")
                };
                assert_eq!(now.loops, lowered(&polygon(&target)), "landed where asked");
                if !out.committed.is_empty() {
                    assert!(session.perform(SessionOp::Undo).refusal.is_none());
                    assert_eq!(session.history().current(), state, "one undo step");
                    assert!(session.committed_doc().bit_eq(&before));
                }
            }
            Some(Refusal::ProfileEditOrder { .. }) => {
                refused_order += 1;
                assert!(session.committed_doc().bit_eq(&before), "no residue");
                assert_eq!(session.history().current(), state, "no history residue");
                let Some(Node::Profile(current)) = before.node(profile) else {
                    panic!("profile")
                };
                let edits: Vec<_> = sketch::program_edits(current, &lowered(&polygon(&target)))
                    .expect("same shape")
                    .into_iter()
                    .map(|(slot, expr)| DocEdit::SetParam {
                        node: profile,
                        slot,
                        expr,
                    })
                    .collect();
                let mut used = vec![false; edits.len()];
                if some_order_lands(&before, &edits, &mut used, tol) {
                    counterexamples.push((base.clone(), target.clone()));
                }
            }
            Some(_) => {
                assert!(session.committed_doc().bit_eq(&before), "no residue");
            }
        }
    }
    eprintln!(
        "landed {landed}, refused-order {refused_order}, counterexamples {}",
        counterexamples.len()
    );
    assert!(
        counterexamples.is_empty(),
        "ProfileEditOrder refused although an order lands: {:?}",
        counterexamples.first()
    );
}

//! MSOLVE-8 acceptance — what a placement's poses say under the
//! witness, and the mate-less arm.
//!
//! The `c4` row measures the sentence on `MateFault` about its
//! mate-less `PosesOfAnotherDocument` arm; the last row pins the A11
//! rule-1 exemplar's verdicts. Every row runs at `Tol::witness()`.
//!
//! The documents: a loose copy of a unit box `B`, its mates' targets
//! read off a second box `T` standing beside it — a construction, of
//! the world's kind — so every pose on both sides is a read the fold
//! binds and the row can bind again.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::Dimension;
use editor_core::Formula;

use editor_core::{
    CapEnd, Coord, DocumentId, EvalOptions, MateFault, Node, NodeErrorClass, Operand, ProfileDoc,
    RecipeNodeId, RoleSeg, SitedRef, StableName, TargetAxis,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{insert, len, run, solve};
use geom_core::Tol;

// ---- Substrate ----

/// A unit box, the square `[x0, x0 + 1] × [0, 1]` on its own xy
/// extruded 1 tall, and its body.
fn unit_box(doc: ProfileDoc, x0: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = insert(
        doc,
        Node::Profile(fixture::desc(vec![vec![
            (x0, 0.0),
            (x0 + 1.0, 0.0),
            (x0 + 1.0, 1.0),
            (x0, 1.0),
        ]])),
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

/// The box `b` copied by the loose placement `copy`, the box `t` its
/// mates' targets are read off, standing beside it.
struct Rig {
    doc: ProfileDoc,
    b: RecipeNodeId,
    t: RecipeNodeId,
    copy: RecipeNodeId,
}

fn rig(label: &str) -> Rig {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, b) = unit_box(doc, 0.0);
    let (doc, t) = unit_box(doc, 3.0);
    let (doc, copy) = insert(
        doc,
        crate::fixture::stood(editor_core::BodyRead::plain(Operand::Node(b))),
    );
    Rig { doc, b, t, copy }
}

impl Rig {
    /// A read of `name`, an entity of `body`, at `body`.
    fn at(body: RecipeNodeId, name: StableName) -> Operand {
        SitedRef::new(body, name).into()
    }

    /// `body`'s lateral edge at canonical profile vertex `v`, as an axis.
    fn lateral(&self, body: RecipeNodeId, v: usize) -> Operand {
        fixture::axis(Self::at(
            body,
            fixture::ename(
                body,
                RoleSeg::LateralEdge(fixture::vpiece(&self.doc, body, 0, v)),
            ),
        ))
    }

    /// `body`'s side wall over canonical segment `k`, as a plane.
    fn side(&self, body: RecipeNodeId, k: usize) -> Operand {
        fixture::plane(Self::at(
            body,
            fixture::fname(
                body,
                RoleSeg::Lateral(fixture::piece(&self.doc, body, 0, k).into()),
            ),
        ))
    }

    /// Writes `on ≡ to` and `values` naming it as the copy's bundle.
    fn place(&mut self, on: Operand, to: Operand, values: Vec<(Coord, Formula)>) {
        self.doc = fixture::with_values(self.doc.clone(), self.copy, fixture::mate(on, to), values);
    }
}

// ---- C4: the mate-less arm, measured ----

/// **`PosesOfAnotherDocument` reaches no row.** A document of five
/// instances — three pinned to the world, two seated on two of them —
/// records it against nothing; `SolvedPoses::placement` raises it for
/// every instance when read against another document and places every
/// one against its own; and no node result of the evaluation carries
/// it.
///
/// *Breaks if* a solve answers a document it is not of, or a row
/// carries the mispairing.
#[test]
fn c4_poses_of_another_document_reaches_no_row() {
    let mut store = PartStore::new();
    let (part, body) = unit_box(
        ProfileDoc::empty(
            DocumentId::derive("msolve8-c4-mispair-part"),
            Tol::witness(),
        ),
        0.0,
    );
    let (doc_ref, body) = store.insert_part((part, body), Tol::witness());
    let o = with_resolver(store);
    let mut doc = ProfileDoc::empty(DocumentId::derive("msolve8-c4-mispair"), Tol::witness());
    let mut ids = Vec::new();
    for i in 0..5 {
        let (next, id) = if i % 2 == 0 {
            fixture::insert_pinned(doc, crate::fixture::mated_instance(doc_ref), &o)
        } else {
            insert(doc, fixture::mated_instance(doc_ref))
        };
        doc = next;
        ids.push(id);
    }
    for (lower, upper) in [(ids[0], ids[1]), (ids[2], ids[3])] {
        doc = fixture::with_values(
            doc,
            upper,
            fixture::rest(
                fixture::head_at(upper, in_part(upper, body, CapEnd::Start)),
                fixture::head_at(lower, in_part(lower, body, CapEnd::End)),
            ),
            vec![
                (
                    Coord::Slide(TargetAxis::X),
                    Formula::zero(Dimension::Length),
                ),
                (
                    Coord::Slide(TargetAxis::Y),
                    Formula::zero(Dimension::Length),
                ),
                (Coord::SPIN, Formula::zero(Dimension::Angle)),
            ],
        );
    }
    let tol = Tol::witness();
    let poses = solve(&doc, &o, tol);
    for id in doc.ids() {
        assert!(
            !matches!(
                poses.fault(id),
                Some(MateFault::PosesOfAnotherDocument { .. })
            ),
            "{id:?}: {:?}",
            poses.fault(id)
        );
    }
    let other = ProfileDoc::empty(DocumentId::derive("msolve8-c4-mispair-other"), tol);
    for &id in &ids {
        assert!(
            matches!(
                poses.placement(&other, id),
                Err(editor_core::PoseRefusal::Mate(ref f))
                    if matches!(**f, MateFault::PosesOfAnotherDocument { .. })
            ),
            "{id:?}"
        );
        assert!(
            poses.placement(&doc, id).is_ok(),
            "{id:?}: the own document places"
        );
    }
    let ev = run(&doc, &o);
    for id in doc.ids() {
        let Some(e) = ev.node_error(id) else {
            continue;
        };
        let classes: Vec<NodeErrorClass> = core::iter::once(e.kind.class())
            .chain(e.kind.carried_chain().map(|l| l.refusal.kind().class()))
            .collect();
        assert!(
            !classes.contains(&NodeErrorClass::MatePosesOfAnotherDocument),
            "{id:?} refuses on another document's poses: {e}"
        );
    }
}

// ---- A3: verdicts that must not move under the witness ----

/// **A determined bundle determines, and a V-block refuses.**
/// The A11 rule-1 exemplar — a coaxial mate and its slide and spin
/// values — pins the copy standing on `T`, its lateral edge on `T`'s
/// and slid the box's unit height along it. Two plane mates on two
/// adjacent side walls, flush with `T`'s, fix a normal twice: the door
/// refuses the second, naming both, whatever the walls' angle
/// (FORK-S3O).
///
/// *Breaks if* the exemplar's verdicts move under the witness, or the
/// door admits a second plane mate.
#[test]
fn a_determined_bundle_and_a_v_block_keep_their_verdicts_under_the_witness() {
    let mut r = rig("msolve8-determined");
    r.place(
        r.lateral(r.b, 0),
        r.lateral(r.t, 0),
        vec![
            (Coord::Slide(TargetAxis::Z), len(1.0)),
            (Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    );
    let poses = solve(&r.doc, &EvalOptions::default(), Tol::witness());
    assert!(
        poses.fault(r.copy).is_none() && poses.space(r.copy) == r.doc.node_space(r.t),
        "the exemplar is DETERMINED: {:?}",
        poses.fault(r.copy)
    );
    let seated = poses.relative(r.copy).expect("the copy is placed");
    let close = |a: &[f64], b: &[f64]| a.iter().zip(b).all(|(x, y)| (x - y).abs() <= 1e-12);
    let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    assert!(
        seated
            .columns
            .iter()
            .zip(&identity)
            .all(|(a, b)| close(a, b))
            && close(&seated.translation, &[3.0, 0.0, 1.0]),
        "the copy stands on `T`, flush: {seated:?}"
    );

    let r = rig("msolve8-v-block");
    let walls = [0, 1].map(|k| {
        editor_core::Constraint::of_mate(
            editor_core::ConstraintId::placeholder(k + 1),
            fixture::mate(r.side(r.b, k as usize), r.side(r.t, k as usize)),
        )
    });
    let fault = fixture::bundle_refusal(&r.doc, r.copy, walls.to_vec());
    assert!(
        matches!(
            &fault,
            MateFault::Overconstrained { placement, constraints, free: 1 }
                if *placement == r.copy && constraints.len() == 2
        ),
        "the V-block's second wall fixes a normal the first fixes: {fault:?}"
    );
}

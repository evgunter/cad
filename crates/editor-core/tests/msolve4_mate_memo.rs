//! **A mate's memo key carries the solve's answer.**
//!
//! A mate's value IS the solve's answer for it: a role when the solve
//! placed it, a typed refusal when the solve faulted it. The solve
//! runs afresh on every evaluation and a mate is a DAG leaf, so a key
//! of the mate's recipe payload alone would serve last evaluation's
//! answer into this one — an unedited mate reading `Ok` in the very
//! evaluation whose fault names it, or carrying a role an edit
//! elsewhere has since taken away from it.
//!
//! Every row here goes through ordinary doors — `DocEdit::InsertNode`,
//! `DocEdit::DeleteNode`, `evaluate` with the previous evaluation as
//! the prior — and reads the answer off the evaluation and off
//! `solve_document`, never off the key function.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, AxisSense, CancelToken, CapEnd, ContactClass, DocEdit, DocRef, DocumentId,
    EntityKind, EvalOptions, Evaluation, MateFault, MateFrame, MatePrimitive, MateRole, Node,
    NodeErrorKind, NodeResult, PartResolver, ProfileDoc, RecipeNodeId, ResolveFailure,
    ResolveFault, RoleSeg, SitedRef, StableName, ValuePayload, content_pin, evaluate,
    solve_document,
};
use fixture::{insert, len, on_frame, step};
use geom_core::Tol;

// ---- substrate ----

/// The part documents this suite instantiates, behind the ordinary
/// resolver door.
#[derive(Debug, Default)]
struct StubStore {
    docs: BTreeMap<DocumentId, ProfileDoc>,
}

impl StubStore {
    fn insert(&mut self, doc: ProfileDoc, tol: Tol) -> DocRef {
        let pin = content_pin(&doc, tol).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }
}

impl PartResolver for StubStore {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        let fail = |fault, message: &str| ResolveFailure {
            fault,
            message: message.to_string(),
        };
        let doc = self
            .docs
            .get(&doc_ref.id)
            .ok_or_else(|| fail(ResolveFault::Unresolved, "no such document"))?;
        let found = content_pin(doc, Tol::witness()).expect("the pin computes");
        if found != doc_ref.pin {
            return Err(fail(ResolveFault::PinMismatch, "the pin does not hold"));
        }
        Ok(doc.clone())
    }
}

/// The extrude in a one-block part document (frame, profile, extrude).
const PART_BODY: RecipeNodeId = RecipeNodeId(2);

/// A `wxwxh` block, as a whole part document.
fn slab(label: &str, w: f64, h: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, w), (0.0, w)]],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(h),
        },
    );
    doc
}

/// A face of `instance`'s part product — the plain member spelling.
fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: PART_BODY,
                path: vec![RoleSeg::Cap(cap)],
            }),
        }],
    }
}

// ---- the scene ----

/// The base slab's height and width, and the blocks' width and height.
const BASE_HEIGHT: f64 = 1.0;
const BASE_WIDTH: f64 = 4.0;
const BLOCK_WIDTH: f64 = 1.0;
const BLOCK_HEIGHT: f64 = 2.0;

/// A mate frame ON the base's top cap at `(x, y)`, axis along that
/// cap's OUTWARD normal.
fn base_frame(x: f64, y: f64) -> MateFrame {
    MateFrame {
        origin: [x, y, BASE_HEIGHT],
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    }
}

/// A block's bottom-cap corner, axis along THAT cap's outward normal,
/// which points DOWN in the block's own part coordinates. The two
/// outward normals and `Opposed` are what make this a physical seat:
/// the block stands ON what it is mated to.
fn block_bottom() -> MateFrame {
    MateFrame {
        origin: [0.0, 0.0, 0.0],
        axis: [0.0, 0.0, -1.0],
        reference: [1.0, 0.0, 0.0],
    }
}

/// A block's TOP-cap corner, axis along that cap's outward normal.
fn block_top() -> MateFrame {
    MateFrame {
        origin: [0.0, 0.0, BLOCK_HEIGHT],
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    }
}

/// A `Rest` mate seating `b`'s frame on `a`'s.
fn seat(
    a: SitedRef,
    b: SitedRef,
    a_frame: MateFrame,
    b_frame: MateFrame,
    primitive: MatePrimitive,
) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a,
        b,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: a_frame,
            b: b_frame,
            primitive,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

/// A wide base slab and two blocks, each instantiated through the
/// resolver — the document every row below edits, and the ids it
/// edits by.
struct Scene {
    doc: ProfileDoc,
    opts: EvalOptions,
    base: RecipeNodeId,
    top_a: RecipeNodeId,
    top_b: RecipeNodeId,
}

fn scene(label: &str) -> Scene {
    let mut store = StubStore::default();
    let base_ref = store.insert(
        slab(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let block_ref = store.insert(
        slab(&format!("{label}-block"), BLOCK_WIDTH, BLOCK_HEIGHT),
        Tol::witness(),
    );
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top_a) = insert(doc, Node::instantiate_part(block_ref));
    let (doc, top_b) = insert(doc, Node::instantiate_part(block_ref));
    Scene {
        doc,
        opts,
        base,
        top_a,
        top_b,
    }
}

impl Scene {
    /// Inserts `node`, answering its id.
    fn add(&mut self, node: Node<editor_core::ProfileProgram>) -> RecipeNodeId {
        let (doc, id) = insert(self.doc.clone(), node);
        self.doc = doc;
        id
    }

    /// A mate seating `block`'s bottom cap on the base's top cap at
    /// `(x, y)`, under `primitive`.
    fn seat_on_base(
        &mut self,
        block: RecipeNodeId,
        x: f64,
        y: f64,
        primitive: MatePrimitive,
    ) -> RecipeNodeId {
        let node = seat(
            SitedRef::new(self.base, in_part(self.base, CapEnd::End)),
            SitedRef::new(block, in_part(block, CapEnd::Start)),
            base_frame(x, y),
            block_bottom(),
            primitive,
        );
        self.add(node)
    }

    /// A mate seating `upper`'s bottom cap on `lower`'s top cap.
    fn stack(&mut self, lower: RecipeNodeId, upper: RecipeNodeId) -> RecipeNodeId {
        let node = seat(
            SitedRef::new(lower, in_part(lower, CapEnd::End)),
            SitedRef::new(upper, in_part(upper, CapEnd::Start)),
            block_top(),
            block_bottom(),
            MatePrimitive::FrameCoincidence,
        );
        self.add(node)
    }

    fn delete(&mut self, id: RecipeNodeId) {
        let (doc, _) = step(self.doc.clone(), DocEdit::DeleteNode { id });
        self.doc = doc;
    }

    /// One evaluation, optionally over a prior — the memo's own door.
    fn eval(&self, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
        evaluate::<f64>(
            &self.doc,
            prior,
            &CancelToken::new(),
            &self.opts,
            Tol::witness(),
        )
    }
}

// ---- readers ----

/// The mate fault a node's row carries, panicking on any other status.
fn row_fault(ev: &Evaluation<f64>, id: RecipeNodeId, what: &str) -> MateFault {
    match ev.result(id) {
        Some(NodeResult::Failed(error)) => match &error.kind {
            NodeErrorKind::Mate(fault) => (**fault).clone(),
            other => panic!("{what}: expected a mate refusal, got {other:?}"),
        },
        other => panic!("{what}: expected a failed row, got {other:?}"),
    }
}

/// The role a mate's VALUE carries, panicking on any other status.
fn row_role(ev: &Evaluation<f64>, id: RecipeNodeId, what: &str) -> MateRole {
    match ev.result(id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Mate(role) => *role,
            other => panic!("{what}: expected a mate payload, got {other:?}"),
        },
        other => panic!("{what}: expected an Ok row, got {other:?}"),
    }
}

/// A node's content key in this evaluation.
fn key(ev: &Evaluation<f64>, id: RecipeNodeId, what: &str) -> editor_core::ContentKey {
    ev.value(id)
        .unwrap_or_else(|| panic!("{what}: no value to key"))
        .content_key
}

// ---- the rows ----

/// **A contradiction added around an already-evaluated mate faults it
/// too, and its row carries the solve's own blame.**
///
/// The finding's second observed shape: two mates on ONE pair, the
/// fault saying they cannot both hold while the first reads `Ok` off
/// a memo its key never invalidated.
#[test]
fn a_contradiction_faults_the_mate_that_evaluated_before_it() {
    let mut s = scene("msolve4-contradiction");
    let held = s.seat_on_base(s.top_a, 1.0, 1.0, MatePrimitive::FrameCoincidence);
    // The first mate lands and EVALUATES: the memo now holds an `Ok`
    // for it, and the solve calls it determining.
    let first = s.eval(None);
    assert_eq!(
        row_role(&first, held, "the lone mate"),
        MateRole::Determining
    );

    // A second mate on the same pair, seating the same block a metre
    // away: the pair's fold cannot hold both.
    let added = s.seat_on_base(s.top_a, 2.0, 1.0, MatePrimitive::FrameCoincidence);
    let second = s.eval(Some(&first));

    let poses = solve_document(&s.doc, Tol::witness());
    for (mate, what) in [
        (held, "the mate that evaluated first"),
        (added, "the mate that broke the pair"),
    ] {
        let row = row_fault(&second, mate, what);
        assert!(
            matches!(
                row,
                MateFault::Contradictory {
                    held: h,
                    added: a,
                    ..
                } if h == held && a == added
            ),
            "{what}: expected the pair's contradiction naming both, got {row:?}"
        );
        // Blame and row agree — the consistency the whole unit is for.
        assert_eq!(
            Some(&row),
            poses.fault(mate),
            "{what}: the solve's blame and the row's own result disagree"
        );
    }
    // The instance the pair could not place carries it too.
    assert!(
        matches!(second.result(s.top_a), Some(NodeResult::Failed(_))),
        "the instance the refusal reached: {:?}",
        second.result(s.top_a)
    );
}

/// **A refusal elsewhere in the cluster faults the unedited mate that
/// is nowhere in the fault's words.**
///
/// The finding's first observed shape: a sound mate evaluates, a
/// second mate then breaks the cluster around it, and the sound
/// mate's row read `Ok` in the evaluation whose fault every instance
/// of its cluster reports. The fault here names only the offender —
/// [`MateFault::Under`] names one mate — so nothing about the sound
/// mate's own words changed; only the solve's answer for it did.
#[test]
fn a_cluster_refusal_faults_the_sound_mate_that_evaluated_before_it() {
    let mut s = scene("msolve4-cluster");
    let sound = s.seat_on_base(s.top_a, 1.0, 1.0, MatePrimitive::FrameCoincidence);
    let first = s.eval(None);
    assert_eq!(
        row_role(&first, sound, "the sound mate"),
        MateRole::Determining
    );

    // A second block, mated `Coaxial`: the residual is cylindrical, so
    // the tree edge does not determine and the whole cluster refuses.
    let offender = s.seat_on_base(s.top_b, 3.0, 1.0, MatePrimitive::Coaxial);
    let second = s.eval(Some(&first));

    let offence = row_fault(&second, offender, "the offending mate");
    assert!(
        matches!(offence, MateFault::Under { mate, .. } if mate == offender),
        "the offending mate carries the under-determination: {offence:?}"
    );
    // The sound mate is not in the fault's words at all, and it is
    // faulted all the same, with the blame the solve recorded.
    let poses = solve_document(&s.doc, Tol::witness());
    let carried = row_fault(&second, sound, "the sound mate");
    assert_eq!(
        Some(&carried),
        poses.fault(sound),
        "the solve's blame and the sound mate's row disagree"
    );
    assert_eq!(
        poses.role(sound),
        Some(MateRole::Refused),
        "the solve took the sound mate's role away"
    );
    for instance in [s.base, s.top_a, s.top_b] {
        assert!(
            matches!(second.result(instance), Some(NodeResult::Failed(_))),
            "every instance in the refused cluster reports it: {:?}",
            second.result(instance)
        );
    }
}

/// **The reverse direction: deleting the contradiction lets the
/// faulted mate evaluate `Ok` again.**
///
/// A faulted mate is never memoized — the memo serves `Ok` priors
/// only — so this direction was never broken. It is here because the
/// key now carries the fault flag, and a key that moved the wrong way
/// would strand the repair as easily as the break.
#[test]
fn deleting_the_contradiction_returns_the_faulted_mate_to_ok() {
    let mut s = scene("msolve4-repair");
    let held = s.seat_on_base(s.top_a, 1.0, 1.0, MatePrimitive::FrameCoincidence);
    let added = s.seat_on_base(s.top_a, 2.0, 1.0, MatePrimitive::FrameCoincidence);
    let broken = s.eval(None);
    assert!(
        matches!(broken.result(held), Some(NodeResult::Failed(_))),
        "the premise: the pair refuses"
    );

    s.delete(added);
    let repaired = s.eval(Some(&broken));
    assert_eq!(
        row_role(&repaired, held, "the surviving mate"),
        MateRole::Determining,
        "the repaired document evaluates the mate on its own solve"
    );
    assert!(
        solve_document(&s.doc, Tol::witness()).fault(held).is_none(),
        "and the solve records no blame against it"
    );
}

/// **A role change on an unedited mate reaches its value through the
/// memo, and moves its content key.**
///
/// The stack mate is a tree edge while its two blocks are a cluster of
/// their own; seating both blocks on the base makes the base the
/// gauge, both base mates tree edges, and the stack mate a non-tree
/// edge that declares. Its references, class and alignment never
/// change.
#[test]
fn a_role_change_on_an_unedited_mate_reaches_its_value() {
    let mut s = scene("msolve4-role");
    let stacked = s.stack(s.top_a, s.top_b);
    let first = s.eval(None);
    assert_eq!(
        row_role(&first, stacked, "the stack mate, alone in its cluster"),
        MateRole::Determining,
        "the pair's only mate is the tree edge that places the upper block"
    );

    // Both blocks now hang off the base, which becomes the gauge: the
    // tree reaches each of them directly and the stack mate is the
    // cycle's non-tree edge.
    s.seat_on_base(s.top_a, 1.0, 1.0, MatePrimitive::FrameCoincidence);
    s.seat_on_base(s.top_b, 3.0, 1.0, MatePrimitive::FrameCoincidence);
    let second = s.eval(Some(&first));

    assert_eq!(
        solve_document(&s.doc, Tol::witness()).role(stacked),
        Some(MateRole::Declaring),
        "the premise: the solve moved the unedited mate off the tree"
    );
    assert_eq!(
        row_role(&second, stacked, "the stack mate, after the base joined"),
        MateRole::Declaring,
        "the unedited mate's VALUE carries the new role, not the memo's old one"
    );
    assert_ne!(
        key(&first, stacked, "before"),
        key(&second, stacked, "after"),
        "and its content key moved with the answer"
    );
}

/// **A mate whose payload AND whose solve answer are unchanged is
/// still reused** — the key carries an answer, not a nonce.
#[test]
fn an_unchanged_mate_is_reused() {
    let mut s = scene("msolve4-reuse");
    let held = s.seat_on_base(s.top_a, 1.0, 1.0, MatePrimitive::FrameCoincidence);
    let first = s.eval(None);
    let second = s.eval(Some(&first));

    assert_eq!(
        key(&first, held, "the first evaluation"),
        key(&second, held, "the second"),
        "an unedited mate over an unchanged solve keys identically"
    );
    assert_eq!(
        second.recomputed, 0,
        "nothing re-ran over an unedited document: {:?}",
        second.order
    );
    assert_eq!(
        second.reused,
        second.order.len(),
        "every node came off the memo"
    );
}

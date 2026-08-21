//! **The declaration class, recipe-side** (M9-1 spec PR-2;
//! CONTACT-DESIGN C4): the class a finding reports is the class the
//! `Declare` node records is the class the boolean verifies against —
//! one vocabulary end-to-end, as DATA and not only as a type.
//!
//! The kernel half (the ladders, the verdicts, the ε rows) is pinned
//! beside the ladders in `topo`; what these rows own is the recipe
//! path: authoring, preservation, threading, and the end-to-end
//! refusal a wrong class produces at the op.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    BooleanOp, CancelToken, CapEnd, ContactClass, DocEdit, EvalOptions, Node, NodeResult,
    ProfileDoc, RecipeNodeId, RoleSeg, evaluate, find_flush_candidates,
};

mod fixture;

use fixture::{desc, fname, insert, len};
use geom_core::Tol;

fn block(
    doc: ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
        )),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

/// Two blocks meeting on z = 1: a genuine flush cap.
fn stacked() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("m9_1_declare_classes", Tol::witness());
    let (doc, a) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 2.5), (0.25, 2.25), 1.0, 1.0);
    (doc, a, b)
}

fn cap(node: RecipeNodeId, end: CapEnd) -> editor_core::StableName {
    fname(node, RoleSeg::Cap(end))
}

/// **The class-preservation row.**
///
/// `declare_node` used to build its pairs from `f.pair.clone()` alone,
/// so a finding's class reached the door and stopped there. Invisible
/// while `Rest` was the only class; a silent mis-verification the
/// moment a second one exists, because the consuming boolean would
/// re-default what the detector had already decided.
///
/// **Why the finding is synthetic.** An earlier version of this row
/// only fed it real detector output, and the review found that it
/// passed under a `declare_node` that re-defaults every class to
/// `Rest` — the detector emits `Rest` today, so `Rest == Rest` proved
/// nothing about PRESERVATION and only the type system stopped the
/// literal old code. `FlushFinding`'s fields are public, so the row
/// now hands the door a `Tangent` finding directly: the assertion
/// carries the claim, and a re-defaulting implementation goes red on
/// the class it did not preserve.
#[test]
fn declare_node_preserves_the_findings_class() {
    let (doc, a, b) = stacked();
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let detected = find_flush_candidates(&ev, a, b, Tol::witness()).expect("the detector runs");
    assert!(!detected.is_empty(), "the stack has a flush cap to find");

    // A finding the detector cannot mint today, carrying a class the
    // door must not re-decide. Built from a real finding so only the
    // class differs.
    let mut tangent = detected[0].clone();
    tangent.class = ContactClass::Tangent;
    let findings = vec![detected[0].clone(), tangent];

    let node: Node<editor_core::ProfileProgram> =
        editor_core::declare_node(&findings).expect("findings declare");
    let Node::Declare { pairs } = node else {
        panic!("declare_node builds a Declare");
    };
    assert_eq!(pairs.len(), findings.len());
    for (pair, finding) in pairs.iter().zip(&findings) {
        assert_eq!(pair.0, finding.pair, "the pair survives");
        assert_eq!(
            pair.1, finding.class,
            "and so does the CLASS — the detector's verdict is not re-decided downstream"
        );
    }
    // The bite, stated as its own assertion: the two pairs name the
    // SAME faces and differ only in class, so a door that dropped or
    // re-defaulted the class would produce two identical entries.
    assert_eq!(pairs[0].0, pairs[1].0, "same pair, by construction");
    assert_ne!(
        pairs[0].1, pairs[1].1,
        "so the classes are the only thing distinguishing them — a re-defaulting \
         declare_node collapses these two rows into one meaning"
    );
    assert_eq!(pairs[1].1, ContactClass::Tangent);
}

/// A hand-authored class is what the node holds: `Declare` is data,
/// and the class is part of the datum. The mixed node also proves one
/// node may carry pairs of different classes — the class rides the
/// pair, not the node.
#[test]
fn an_authored_class_is_what_the_node_holds() {
    let (doc, a, b) = stacked();
    let node: Node<editor_core::ProfileProgram> = Node::Declare {
        pairs: vec![
            (
                (cap(a, CapEnd::Top), cap(b, CapEnd::Bottom)),
                ContactClass::Rest,
            ),
            (
                (cap(a, CapEnd::Bottom), cap(b, CapEnd::Top)),
                ContactClass::Tangent,
            ),
        ],
    };
    let applied = doc
        .apply(&DocEdit::InsertNode { node }, Tol::witness())
        .expect("the Declare inserts");
    let id = applied.record.minted.unwrap();
    let Some(Node::Declare { pairs }) = applied.doc.node(id) else {
        panic!("the node is a Declare");
    };
    assert_eq!(pairs[0].1, ContactClass::Rest);
    assert_eq!(pairs[1].1, ContactClass::Tangent);
}

/// **The wrong class, end to end from a recipe.**
///
/// A genuine flush cap declared as `Tangent` reaches the kernel door
/// carrying the class the author wrote, and the op refuses it typed
/// rather than treating it as the class it can act on. The refusal is
/// the whole point of threading the class: before this change the
/// recipe could not express the mistake, so the kernel could not
/// refuse it.
#[test]
fn a_wrong_class_declaration_refuses_at_the_op() {
    let (doc, a, b) = stacked();
    let declare = |class| -> Node<editor_core::ProfileProgram> {
        Node::Declare {
            pairs: vec![((cap(a, CapEnd::Top), cap(b, CapEnd::Bottom)), class)],
        }
    };
    // Returns (ran_ok, debug rendering of the failure if any) — the
    // evaluation's NodeResult is not Clone, so the row reads what it
    // needs while the borrow lives.
    let run = |class| -> (bool, String) {
        let applied = doc
            .apply(
                &DocEdit::InsertNode {
                    node: declare(class),
                },
                Tol::witness(),
            )
            .expect("the Declare inserts");
        let d = applied.record.minted.unwrap();
        let applied = applied
            .doc
            .apply(
                &DocEdit::InsertNode {
                    node: Node::Boolean {
                        op: BooleanOp::Union,
                        a,
                        b,
                        declare: Some(d),
                    },
                },
                Tol::witness(),
            )
            .expect("the boolean inserts");
        let id = applied.record.minted.unwrap();
        let ev = evaluate::<f64>(
            &applied.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        match ev.nodes.get(&id) {
            Some(NodeResult::Ok(_)) => (true, String::new()),
            other => (false, format!("{other:?}")),
        }
    };

    // The RIGHT class: the op consumes the declaration and runs.
    let (ok, why) = run(ContactClass::Rest);
    assert!(ok, "a correctly-classed declaration still unions: {why}");

    // The WRONG class: typed refusal naming the class that was asked
    // for, never a silent re-interpretation.
    let (ok, msg) = run(ContactClass::Tangent);
    assert!(!ok, "a Tangent declaration on a conformal pair must refuse");
    assert!(
        msg.contains("UnsupportedDeclarationClass") && msg.contains("Tangent"),
        "the refusal names the class that was asked for: {msg}"
    );
}

/// The detector reports `Rest` and says so — and the finding's class
/// is the KERNEL's enum, not a recipe-layer copy of it. The identity
/// is what makes "one vocabulary end-to-end" checkable rather than
/// aspirational.
#[test]
fn the_detectors_class_is_the_kernels_enum() {
    let (doc, a, b) = stacked();
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let findings = find_flush_candidates(&ev, a, b, Tol::witness()).expect("the detector runs");
    for f in &findings {
        assert_eq!(f.class, ContactClass::Rest);
        // Same type, spelled through the kernel path: this would not
        // compile against a parallel enum.
        let kernel: topo::ContactClass = f.class;
        assert_eq!(kernel.name(), "Rest");
    }
}

/// The `Fit` deferral sentence a refusal steers to is the KERNEL's,
/// quoted verbatim through the re-export chain — one wording, and the
/// assembly layer's mate verification cites the same constant.
#[test]
fn the_fit_deferral_sentence_is_quoted_not_paraphrased() {
    assert_eq!(editor_core::FIT_DEFERRAL, topo::FIT_DEFERRAL);
    assert_eq!(pncad_selection_deferral(), topo::FIT_DEFERRAL);
    assert!(editor_core::FIT_DEFERRAL.contains("Fit { gap }"));
    assert!(editor_core::FIT_DEFERRAL.contains("not yet built"));
}

fn pncad_selection_deferral() -> &'static str {
    editor_core::names::FIT_DEFERRAL
}

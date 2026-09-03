//! **The blend refusal messages, PINNED** — adopted from the LIB-G16
//! R2 review probe, which printed them for a cross-tree byte
//! comparison and so expired with that comparison
//! (`memories/test-suite-cost.md`: a one-shot comparison artefact has
//! no consumer once the diff is taken, and a test that asserts nothing
//! is never a gate).
//!
//! What schedules it instead is the standing claim underneath the
//! comparison: the error family carries a `BlendKind` verb, one shared
//! `resolve_selection` ladder serves both nodes, and the fillet's
//! three selection texts must stay EXACTLY what they were (the op
//! row's kernel tail is prefix-pinned instead) while the chamfer's
//! say "chamfer". A `write!` that loses the verb, or a well-meant
//! reword of the shared ladder, moves a user-visible string that
//! nothing else in the tree reads; here it goes red with the old and
//! new text side by side.
//!
//! The chamfer half is not a copy of the fillet half: it is the same
//! four refusals under the other verb, which is the assertion that the
//! shared door discriminates rather than merely runs.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, EvalOptions, Node, NodeResult, ProfileDoc, ProfileProgram, RecipeNodeId, evaluate,
};
use geom_core::Tol;

fn cube_doc() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("g16-r2-msg", Tol::witness());
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    let (doc, cube) = fixture::insert(
        doc,
        Node::Extrude {
            profile,
            distance: fixture::len(1.0),
        },
    );
    (doc, cube)
}

fn msg_of(doc: &ProfileDoc, node: RecipeNodeId) -> String {
    let ev = evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match ev.nodes.get(&node) {
        Some(NodeResult::Failed(e)) => e.kind.to_string(),
        other => panic!("expected a refusal, got {other:?}"),
    }
}

/// The four refusals, for one blend verb, as `(label, message)`.
///
/// Merged into ONE row per verb rather than eight tests: nextest is
/// process-per-test and each of these rebuilds the same cube document,
/// so the split would pay the fixture eight times over
/// (`memories/test-suite-cost.md`). Every assertion carries its own
/// label so a red names the refusal without the test name helping.
fn messages(
    blend: fn(
        RecipeNodeId,
        editor_core::Expr,
        Vec<editor_core::StableName>,
    ) -> Node<ProfileProgram>,
) -> Vec<(&'static str, String)> {
    let size = fixture::len(0.1);
    let mut out = Vec::new();

    // (a) empty selection
    let (doc, cube) = cube_doc();
    let (d, n) = fixture::insert(doc, blend(cube, size.clone(), Vec::new()));
    out.push(("empty", msg_of(&d, n)));

    // (b) a name of the wrong KIND: a face that really is there, so
    // it resolves and then fails the door's edges-only check.
    let (doc, cube) = cube_doc();
    let face = editor_core::StableName {
        kind: editor_core::EntityKind::Face,
        node: cube,
        path: vec![fixture::wall(0)],
    };
    let (d, n) = fixture::insert(doc, blend(cube, size.clone(), vec![face]));
    out.push(("kind", msg_of(&d, n)));

    // (c) a name the target never minted — Vanished through N5.
    let (doc, cube) = cube_doc();
    let ghost = editor_core::StableName {
        kind: editor_core::EntityKind::Edge,
        node: cube,
        path: vec![fixture::wall(7)],
    };
    let (d, n) = fixture::insert(doc, blend(cube, size, vec![ghost]));
    out.push(("resolve", msg_of(&d, n)));

    // (d) the op itself refusing: a size far too large for the cube.
    let (doc, cube) = cube_doc();
    let (d, n) = fixture::insert(
        doc,
        blend(cube, fixture::len(0.9), fixture::prism_edges(cube, 4)),
    );
    out.push(("op", msg_of(&d, n)));
    out
}

/// **The fillet's three SELECTION refusal texts are byte-frozen, and
/// its op row is prefix-pinned** — deliberately not four byte pins.
///
/// The three selection rows are this layer's own text and freeze
/// whole. The op row's tail is the kernel's message, which quotes an
/// arena key, so it is pinned by PREFIXES instead: the wrapper's, and
/// the kernel payload's opening words — the exact text the verb-
/// vocabulary unit reworded (the payload no longer opens "fillet: ";
/// the wrapper is the one verb), so this is where a regression toward
/// a re-verbed inner Display shows up on the fillet side.
#[test]
fn the_fillets_selection_refusals_are_byte_frozen_and_the_op_row_prefix_pinned() {
    let got = messages(Node::fillet);
    let want = [
        (
            "empty",
            "the fillet selection is empty — an unfinished recipe, not the identity",
        ),
        (
            "kind",
            "the fillet selection name minted by node 1 denotes a face, not an edge",
        ),
        (
            "resolve",
            "a fillet selection name failed to resolve: the edge name minted by node 1 no \
             longer resolves in this evaluation: the recorded reference disagrees with the \
             recipe as it stands on the derivation path (node 1's payload differs)",
        ),
    ];
    for ((label, actual), (wl, expected)) in got.iter().zip(want.iter()) {
        assert_eq!(label, wl);
        assert_eq!(actual, expected, "the fillet's {label} refusal text moved");
    }
    // The op row is pinned by PREFIX, not whole. Its tail is the
    // kernel's own `BlendError` message, which quotes an arena key
    // (`FaceKey(3v1)`) — deterministic under D9 replay identity, but a
    // key is the kernel's business and freezing one here would make
    // this row fail for an unrelated allocation change. The claim that
    // belongs to THIS layer is the wrapper: the verb, then the kernel
    // payload forwarded unaltered.
    let (label, op) = &got[3];
    assert_eq!(label, &"op");
    assert!(
        op.starts_with("the fillet op refused: "),
        "the fillet's op wrapper moved: {op}"
    );
    // The kernel payload's own opening words, pinned past the wrapper:
    // verb-neutral (the wrapper above is the one verb on this path).
    assert!(
        op.starts_with("the fillet op refused: the clearance screen cannot certify"),
        "the op row's kernel payload moved, or regained a verb prefix: {op}"
    );
}

/// **The chamfer's four say "chamfer"** — the shared ladder
/// discriminates rather than merely running.
#[test]
fn the_chamfers_refusal_messages_name_the_chamfer() {
    for (label, msg) in messages(Node::chamfer) {
        assert!(
            msg.contains("chamfer"),
            "the chamfer's {label} refusal must name the chamfer: {msg}"
        );
        // No row says "fillet" — the `op` row included: it carries the
        // kernel's message UNALTERED (spec D2 — kernel errors are
        // never stringified over), and the kernel's shared refusal is
        // verb-neutral, the verb attached once at the door. The one
        // "fillet" substring a chamfer message may legitimately carry
        // is a `fillet3_*` predicate NAME (K-corpus roster carriers
        // both verbs meter under deliberately), stripped before the
        // check.
        assert!(
            !msg.replace("fillet3_", "").contains("fillet"),
            "the chamfer's {label} refusal must not say fillet: {msg}"
        );
    }
}

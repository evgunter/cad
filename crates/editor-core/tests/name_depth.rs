//! **A stable name nested past every stack, through every door, on the
//! smallest stack a door runs on.**
//!
//! A name nests one whole name per derivation level: a chain of K
//! patterns, each over the one before, names its faces K levels deep.
//! Every door that holds such a name — evaluation and the drop of its
//! result, a name's `Debug`, the edit door, the split's re-map, save,
//! load and the content pin — runs here on a thread whose stack is the
//! wasm32 build's (one mebibyte), at a depth a walk that recursed once
//! per level could not reach on it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    EvalOptions, Node, PatternKind, ProfileDoc, RecipeNodeId, StableName, all_faces,
    canonical_bytes, content_pin, load, remap_name, save,
};
use fixture::{in_copy, insert, len, on_frame, run, square};
use geom_core::Tol;

/// The wasm32 build's default stack, the smallest any door runs on.
const WASM_STACK: usize = 1 << 20;

/// Runs `f` on a thread with the wasm32 build's stack.
fn on_the_smallest_stack<R: Send + 'static>(f: impl FnOnce() -> R + Send + 'static) -> R {
    std::thread::Builder::new()
        .stack_size(WASM_STACK)
        .spawn(f)
        .expect("the thread starts")
        .join()
        .expect("the door returns")
}

/// A square extrude, then `k` patterns each over the one before, one
/// copy each: the top pattern names its faces `k` levels deep. Returns
/// the document, the extrude and the patterns in order.
fn chain(label: &str, k: usize) -> (ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 1.0)],
    );
    let (mut doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let mut patterns = Vec::with_capacity(k);
    let mut input = extrude;
    for _ in 0..k {
        let (next, pattern) = insert(
            doc,
            Node::Pattern {
                input,
                count: editor_core::Expr::count(1),
                kind: PatternKind::Linear {
                    direction: [fixture::scl(1.0), fixture::scl(0.0), fixture::scl(0.0)],
                    spacing: len(2.0),
                },
            },
        );
        doc = next;
        patterns.push(pattern);
        input = pattern;
    }
    (doc, extrude, patterns)
}

/// Past the depth a recursive `Debug` of one name reached on this
/// stack in a dev build (about 700 levels), and far past the reader's
/// recursion limit a saved name had to fit (41 levels for a bare name).
const PATTERNS: usize = 1_000;

/// One chain, through every door that holds its names: evaluated, a
/// name printed, the evaluation dropped; then a fillet authoring the
/// deepest edge name through the edit door, and the document saved,
/// loaded, re-saved and pinned.
#[test]
fn a_chain_of_patterns_names_past_every_stack_through_every_door_on_the_smallest_stack() {
    on_the_smallest_stack(|| {
        let tol = Tol::witness();
        let (doc, extrude, patterns) = chain("name-depth", PATTERNS);
        let top = *patterns.last().expect("a pattern");
        let ev = run(&doc, &EvalOptions::default());
        let faces = all_faces(&ev, top);
        assert_eq!(
            faces.len(),
            6,
            "the top pattern's one copy has a block's faces"
        );
        let shown = format!("{:?}", faces[0]);
        assert_eq!(
            shown.matches("Instance").count(),
            PATTERNS,
            "the name nests one copy per pattern, and Debug renders each"
        );
        drop(faces);
        drop(ev);
        // An edge of the top copy, named through every pattern: the
        // name a pick on it would author.
        let rim = fixture::rim_edge(
            extrude,
            editor_core::CapEnd::End,
            fixture::piece(&doc, extrude, 0, 0),
        );
        let deep = patterns.iter().fold(rim, |n, &p| in_copy(p, 0, n));
        // Through the edit door.
        let (doc, _) = insert(doc, Node::fillet(top, len(0.1), vec![deep.clone()]));
        let text = save(&doc, &[], tol).expect("the document saves");
        let loaded = load(&text, tol).expect("and loads back");
        assert_eq!(loaded.doc, doc, "the loaded document is the saved one");
        assert_eq!(
            canonical_bytes(&loaded.doc, tol).unwrap(),
            canonical_bytes(&doc, tol).unwrap(),
            "and pins the same bytes"
        );
        assert_eq!(
            content_pin(&loaded.doc, tol).unwrap(),
            content_pin(&doc, tol).unwrap()
        );
        let resaved = save(&loaded.doc, &[], tol).expect("the loaded document saves");
        assert_eq!(resaved, text, "and saves the same text");
        drop((doc, loaded, deep));
    });
}

#[test]
fn a_split_remaps_a_name_past_every_stack_on_the_smallest_stack() {
    on_the_smallest_stack(|| {
        let depth = 20_000;
        let leaf = StableName {
            kind: editor_core::EntityKind::Face,
            node: RecipeNodeId(1),
            path: vec![editor_core::RoleSeg::Cap(editor_core::CapEnd::End)],
        };
        let name = (0..depth).fold(leaf, |n, level| in_copy(RecipeNodeId(2 + level % 3), 0, n));
        let map = (1..=4)
            .map(|n| (RecipeNodeId(n), RecipeNodeId(n + 100)))
            .collect();
        let moved = remap_name(&name, &map, &Default::default()).expect("every id is mapped");
        let expect = (0..depth).fold(
            StableName {
                kind: editor_core::EntityKind::Face,
                node: RecipeNodeId(101),
                path: vec![editor_core::RoleSeg::Cap(editor_core::CapEnd::End)],
            },
            |n, level| in_copy(RecipeNodeId(102 + level % 3), 0, n),
        );
        assert!(
            moved == expect,
            "every level moves to the other document's ids"
        );
    });
}

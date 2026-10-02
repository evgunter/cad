//! M4 PR 4 spec D6: the PR 3 banked obligations.
//!
//! 1. **A far wall renames no piece**: A = 4×4×1 block; the cutter is
//!    a full-pierce band entering the west face horizontally (walls
//!    y=1.0/y=1.1) and turning to a DESCENDING diagonal that exits the
//!    east face. The N piece (above the band) never touches the lower
//!    horizontal wall H1. Translating the cutter east (a continuous
//!    Transform knob) moves the diagonal's east-face exit vertex
//!    (4, y_u) across H1's FIXED plane y=1, with no topology change. A
//!    piece is named by the walls it borders (N2's `Borders`), so no
//!    piece name moves.
//! 2. (R6 — `apply_with_names` — lands in `m4_pr4_resolve.rs`.)
//! 3. **Vanished diagnosis vs dropped fused-vertex identity** (R9):
//!    kept-key-wins fusion drops the losing operand corner's identity
//!    with no N3-style retirement row — `Vanished` must diagnose
//!    honestly (PredicateFlip/Cascade, never a recipe-edit lie) and
//!    offer nothing it cannot justify.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use std::collections::BTreeMap;

use editor_core::{
    BooleanOp, CancelToken, Diagnosis, DocEdit, EntityKind, Entry, EvalOptions, Evaluation, Node,
    ProfileDoc, Qualifier, RecipeNodeId, Resolution, ResolveError, RoleSeg, RunCtx, SlotId,
    StableName, diff_verdicts, evaluate, resolve_with_prior,
};
use fixture::{ang, insert, len, on_frame, scl, step};
use geom_core::Tol;

fn run(doc: &editor_core::ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
            side: ExtrudeSide::Along,
        },
    )
}

// ---- D6.1: the single-qualifier-flip fixture ----

struct BandCut {
    doc: ProfileDoc,
    transform: RecipeNodeId,
    sub: RecipeNodeId,
}

/// A − band(tx): the band's horizontal tail pierces the west face,
/// its descending diagonal exits the east face (module docs).
fn band_cut() -> BandCut {
    let doc = ProfileDoc::empty_derived("m4_pr4_banked", Tol::witness());
    let (doc, a) = block(doc, (0.0, 4.0), (0.0, 4.0), 0.0, 1.0);
    // The band profile on z = -0.5, extruded 2.0 (full pierce).
    // Lower boundary: (-2.5,1.0) → (2.0,1.0) → (4.5,0.8);
    // upper boundary: (-2.5,1.1) → (2.0,1.1) → (4.5,0.9).
    let (doc, bp) = on_frame(
        doc,
        [0.0, 0.0, -0.5],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (-2.5, 1.0),
            (2.0, 1.0),
            (4.5, 0.8),
            (4.5, 0.9),
            (2.0, 1.1),
            (-2.5, 1.1),
        ]],
    );
    let (doc, band) = insert(
        doc,
        Node::Extrude {
            profile: bp,
            distance: len(2.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, transform) = insert(
        doc,
        Node::transform(
            band,
            editor_core::Step::Rigid {
                translation: [len(0.0), len(0.0), len(0.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        ),
    );
    let (doc, sub) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b: transform,
            declare: None,
        },
    );
    BandCut {
        doc,
        transform,
        sub,
    }
}

type Rows = BTreeMap<StableName, Entry>;

fn rows(ev: &Evaluation<f64>, node: RecipeNodeId) -> Rows {
    ev.value(node)
        .unwrap()
        .name_table
        .iter()
        .map(|(n, e)| (n.clone(), e.clone()))
        .collect()
}

/// Whether a name's last segment is a `Borders` qualifier.
fn is_piece(name: &StableName) -> bool {
    matches!(
        name.path.last(),
        Some(RoleSeg::Fragment(Qualifier::Borders(_)))
    )
}

#[test]
fn a_flip_against_a_wall_no_piece_borders_renames_no_piece() {
    let f = band_cut();
    let ev1 = run(&f.doc, None);
    // Slide the cutter east: the diagonal's east-exit vertex crosses
    // the plane of the FIXED lower wall y = 1 (margins 0.06 → 0.04 at
    // the two positions — fat against every ε row; nothing is
    // band-edge), a wall that piece does not border.
    let (doc2, _) = step(
        f.doc.clone(),
        DocEdit::SetParam {
            node: f.transform,
            slot: SlotId::Translation(editor_core::Axis3::X),
            expr: len(1.25),
        },
    );
    let ev2 = run(&doc2, Some(&ev1));
    // Every delta is at the subtract: the slide moved geometry the
    // boolean decides about.
    let flips = diff_verdicts(&ev1, &ev2);
    assert_eq!(
        flips.nodes.keys().copied().collect::<Vec<_>>(),
        vec![f.sub],
        "flip cone wider than the subtract: {flips:?}"
    );
    // A piece is named by the walls it borders, and no piece gained or
    // lost a wall, so every piece keeps its name and its entry shape:
    // a face name reads nothing beyond its own boundary.
    let (r1, r2) = (rows(&ev1, f.sub), rows(&ev2, f.sub));
    let pieces =
        |r: &Rows| -> Vec<StableName> { r.keys().filter(|n| is_piece(n)).cloned().collect() };
    assert!(
        pieces(&r1).len() >= 4,
        "both caps are cut into pieces: {:?}",
        pieces(&r1)
    );
    assert_eq!(pieces(&r1), pieces(&r2), "a piece's name moved");
    let shape = |e: &Entry| match e {
        Entry::Unique(_) => 1usize,
        Entry::Tied(c) => c.len(),
    };
    for n in pieces(&r1) {
        assert_eq!(shape(&r1[&n]), shape(&r2[&n]), "{n:?}");
    }
}

// ---- D6.3: fused-vertex identity drop (R9) ----

#[test]
fn dropped_fused_vertex_identity_diagnoses_honestly() {
    // A ∪ B(tx): disjoint at tx = 2.5 (every operand corner name
    // survives under its wrap); overlapping at tx = 0.5 (kept-key
    // fusion drops the losing coincident-corner identities with no
    // retirement row).
    let doc = ProfileDoc::empty_derived("m4_pr4_banked", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b0) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, transform) = insert(
        doc,
        Node::transform(
            b0,
            editor_core::Step::Rigid {
                translation: [len(2.5), len(0.0), len(0.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        ),
    );
    // M4 PR 5: the slide's flush planes are declared (the disjoint
    // position keeps the same coplanarity, so ONE declare serves both).
    // The B side is read at the TRANSFORM, the boolean's operand;
    // a transform carries `b0`'s names verbatim (N1).
    let (doc, decl) = fixture::declare_x_offset_flush_at(doc, (a, a), (transform, b0));
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b: transform,
            declare: Some(decl),
        },
    );
    let (doc2, _) = step(
        doc.clone(),
        DocEdit::SetParam {
            node: transform,
            slot: SlotId::Translation(editor_core::Axis3::X),
            expr: len(0.5),
        },
    );
    // M5 PR 8: the scenario runs under BOTH sweep strategies. The
    // idealized (brute-force) run keeps this pin at its original
    // strength — the disjoint prior run decides `bool_vertex_face_side`
    // on every pair, so the fusion edit leaves recorded flip evidence
    // and the diagnosis MUST use it. The realized (production, BVH)
    // run prunes the disjoint prior's pair space EMPTY — the flip
    // evidence genuinely never existed — so the honest outcomes are
    // the flip (when any survives) or the DOCUMENTED evidence-free
    // fallback naming the minting node (resolve/mod.rs: "not a claim
    // that an edit happened"); anything else (a specific edit blame, a
    // structural-param blame) stays dishonest and fails here.
    for strategy in [
        topo::SweepStrategy::Idealized,
        topo::SweepStrategy::Realized,
    ] {
        fused_vertex_scenario(&doc, &doc2, u, strategy);
    }
}

fn fused_vertex_scenario(
    doc: &ProfileDoc,
    doc2: &ProfileDoc,
    u: RecipeNodeId,
    strategy: topo::SweepStrategy,
) {
    let opts = EvalOptions {
        boolean_sweep: strategy,
        ..EvalOptions::default()
    };
    let ev1 = evaluate::<f64>(doc, None, &CancelToken::new(), &opts, Tol::witness());
    let ev2 = evaluate::<f64>(doc2, Some(&ev1), &CancelToken::new(), &opts, Tol::witness());
    let (r1, r2) = (rows(&ev1, u), rows(&ev2, u));
    // The dropped identities: operand-corner vertex names present in
    // the disjoint union, absent from the overlapping one.
    let dropped: Vec<&StableName> = r1
        .keys()
        .filter(|n| {
            n.kind == EntityKind::Vertex
                && matches!(n.path.first(), Some(RoleSeg::FromA(_) | RoleSeg::FromB(_)))
                && !r2.contains_key(*n)
        })
        .collect();
    assert!(
        !dropped.is_empty(),
        "the overlap must fuse away some operand corner identities"
    );
    for name in dropped {
        // No N3-style retirement row: nothing in the new table lists
        // the loser as a Merged constituent.
        let res = resolve_with_prior(
            RunCtx {
                doc: doc2,
                eval: &ev2,
            },
            RunCtx { doc, eval: &ev1 },
            name,
        );
        let Resolution::Failed(f) = res else {
            panic!("expected Failed for {name:?}, got {res:?}");
        };
        let ResolveError::Vanished {
            diagnosis,
            last_good,
            ..
        } = &f.error
        else {
            panic!("expected Vanished for {name:?}, got {:?}", f.error);
        };
        // The pin: the diagnosis is HONEST — the identity died at a
        // recorded flip (or through a vanished operand), never blamed
        // on a recipe edit or structural parameter that did not
        // happen. Under the realized sweep the recorded evidence can
        // honestly be ABSENT (the disjoint prior pruned every pair —
        // module comment above), in which case the answers that claim
        // no cause are admitted, both at the MINTING node: the
        // documented evidence-free fallback, and the group-size table
        // fact. This is a closed list of `Diagnosis` arms — when N5
        // gains an arm, decide here whether it is honest without a
        // recorded flip.
        let honest_flip = matches!(
            diagnosis,
            Diagnosis::PredicateFlip { .. } | Diagnosis::Cascade { .. }
        );
        let honest_fallback = matches!(
            diagnosis,
            Diagnosis::RecipeEdit {
                edit: editor_core::RecipeEditRef::NodeChanged { node }
            } | Diagnosis::GroupResized { node, .. } if *node == name.node
        );
        match strategy {
            topo::SweepStrategy::Idealized => assert!(
                honest_flip,
                "dishonest diagnosis for {name:?} (idealized): {diagnosis:?}"
            ),
            topo::SweepStrategy::Realized => assert!(
                honest_flip || honest_fallback,
                "dishonest diagnosis for {name:?} (realized): {diagnosis:?}"
            ),
        }
        // The last-good entry survives as the tombstone (ghost
        // payload), and no merge offer is fabricated.
        assert!(last_good.is_some(), "prior run resolved {name:?}");
        assert!(
            f.offers
                .iter()
                .all(|o| !matches!(o.path.first(), Some(RoleSeg::Merged(_)))),
            "no fabricated merge offers for a fusion drop: {:?}",
            f.offers
        );
    }
}

//! **N5 reads a cited line as the rows that lie on it** (names README,
//! "A cited line"): a line no row lies on any longer is gone, one some
//! row lies on is present; a vanished edge piece is offered the
//! surviving pieces of its line; and a union reads a cited line as its
//! least member row on it.
//!
//! Every row starts from one bar, 20×2×1, notched at x ∈ [9, 11]
//! through its back top edge (y = 2, z = 1), which leaves that edge two
//! pieces of one line.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{insert, len, on_frame, step};
use editor_core::{
    Axis3, BooleanOp, CancelToken, Diagnosis, DocEdit, EntityKind, Entry, EvalOptions, Evaluation,
    ExtrudeSide, Node, ProfileDoc, RecipeNodeId, Resolution, ResolveError, RoleSeg, RunCtx, SlotId,
    StableName, evaluate, resolve_with_prior,
};
use geom_core::Tol;

fn run(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
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

fn boolean(
    doc: ProfileDoc,
    op: BooleanOp,
    a: RecipeNodeId,
    b: RecipeNodeId,
) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Boolean {
            op,
            a,
            b,
            declare: Vec::new(),
        },
    )
}

/// `block` behind a `Transform` with no motion yet: the block and the
/// transform.
fn moved_block(
    doc: ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, b) = block(doc, x, y, z0, dz);
    insert(
        doc,
        crate::fixture::xform(b, [0.0; 3], [0.0, 0.0, 1.0], 0.0),
    )
}

fn slide(doc: &ProfileDoc, node: RecipeNodeId, axis: Axis3, by: f64) -> ProfileDoc {
    step(
        doc.clone(),
        DocEdit::SetParam {
            node,
            slot: SlotId::Translation(axis),
            expr: len(by),
            fresh: Vec::new(),
        },
    )
    .0
}

/// The bar notched through its back top edge: the document and the
/// notched bar.
fn notched_bar(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, bar) = block(doc, (0.0, 20.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, notch) = block(doc, (9.0, 11.0), (1.5, 3.0), 0.5, 1.0);
    boolean(doc, BooleanOp::Subtract, bar, notch)
}

fn failed<'a>(res: &'a Resolution, what: &str) -> &'a editor_core::ResolutionFailure {
    match res {
        Resolution::Failed(f) => f,
        other => panic!("{what}: expected a failure, got {other:?}"),
    }
}

/// The vertex rows of `node` that cite a line by a `Crossing`, with the
/// line each cites.
fn crossings(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<(StableName, StableName)> {
    ev.value(node)
        .expect("the node evaluates")
        .name_table
        .iter()
        .filter_map(|(n, _)| match n.path.first() {
            Some(RoleSeg::Crossing { edge, .. }) => Some((n.clone(), (**edge).clone())),
            _ => None,
        })
        .collect()
}

/// **A line resolved on its own is read as a line.** A strip `M` behind a
/// transform lies clear of the notched bar, then is slid onto its back
/// top edge and swallows both pieces; a slot `N` across the first piece
/// crosses the edge's line at `M`. After the slide the crossing vanishes
/// through that line, and resolving the line itself states why it is
/// gone and keeps its last-good entry: the line at the notch it wraps
/// still has both pieces on it, so it is not where the cause lies.
#[test]
fn a_cascades_through_line_resolved_on_its_own_reads_the_lines_inside_it() {
    let (doc, notched) = notched_bar("cited-line-through");
    let (doc, strip) = moved_block(doc, (-1.0, 21.0), (2.2, 3.0), 0.6, 1.0);
    let (doc, m) = boolean(doc, BooleanOp::Subtract, notched, strip);
    let (doc, slot) = block(doc, (3.0, 4.0), (1.5, 3.0), 0.5, 1.0);
    let (doc, n) = boolean(doc, BooleanOp::Subtract, m, slot);
    let doc2 = slide(&doc, strip, Axis3::Y, -0.5);
    let ev1 = run(&doc, None);
    let ev2 = run(&doc2, Some(&ev1));
    let (crossing, line) = crossings(&ev1, n)
        .into_iter()
        .find(|(_, line)| line.node == m)
        .expect("the slot crosses the back top edge's line at M");
    let prior = RunCtx {
        doc: &doc,
        eval: &ev1,
    };
    let now = RunCtx {
        doc: &doc2,
        eval: &ev2,
    };
    let res = resolve_with_prior(now, prior, &crossing);
    let f = failed(&res, "the crossing");
    let ResolveError::Vanished { diagnosis, .. } = &f.error else {
        panic!("the crossing vanished: {:?}", f.error);
    };
    assert_eq!(
        *diagnosis,
        Diagnosis::Cascade {
            through: line.clone()
        },
        "the crossing vanished through the line at M"
    );
    let res = resolve_with_prior(now, prior, &line);
    let f = failed(&res, "the line");
    let ResolveError::Vanished {
        diagnosis,
        last_good,
        ..
    } = &f.error
    else {
        panic!("the line vanished: {:?}", f.error);
    };
    assert_eq!(
        *diagnosis,
        Diagnosis::RecipeEdit {
            edit: editor_core::RecipeEditRef::NodeChanged { node: m }
        },
        "the line at the notch still has both pieces on it, so the line at M is gone at M"
    );
    assert!(
        last_good.is_some(),
        "the line's last-good entry is its least row"
    );
}

/// **A vanished edge piece is offered the surviving pieces of its
/// line.** A slot `N` behind a transform cuts the first piece of the
/// back top edge in two; slid clear, it leaves that piece whole. Each
/// vanished half is offered the pieces of its line that `N` still has:
/// the first piece whole, and the second.
#[test]
fn a_vanished_piece_is_offered_the_pieces_of_its_line() {
    let (doc, notched) = notched_bar("cited-line-offers");
    let (doc, slot) = moved_block(doc, (3.0, 4.0), (1.5, 3.0), 0.5, 1.0);
    let (doc, n) = boolean(doc, BooleanOp::Subtract, notched, slot);
    let doc2 = slide(&doc, slot, Axis3::Y, 1.0);
    let ev1 = run(&doc, None);
    let ev2 = run(&doc2, Some(&ev1));
    let (t1, t2) = (
        &ev1.value(n).expect("the slot's cut evaluates").name_table,
        &ev2.value(n).expect("the slot's cut evaluates").name_table,
    );
    let halves: Vec<StableName> = t1
        .iter()
        .filter(|(name, e)| {
            matches!(e, Entry::Unique(_))
                && name.kind == EntityKind::Edge
                && matches!(
                    name.path.as_slice(),
                    [
                        RoleSeg::FromA(_),
                        RoleSeg::Fragment(editor_core::Qualifier::Ends(_))
                    ]
                )
                && t2.lookup(name).is_none()
        })
        .map(|(name, _)| name.clone())
        .collect();
    assert_eq!(
        halves.len(),
        2,
        "the slot's two halves of the first piece vanish: {halves:?}"
    );
    // The pieces the line has at `N` now: the two notch pieces, each
    // whole.
    let Some(RoleSeg::FromA(line)) = halves[0].path.first() else {
        panic!("a half is a piece of an operand edge's line");
    };
    let survivors: Vec<StableName> = t2
        .iter()
        .filter(|(name, _)| {
            name.kind == EntityKind::Edge
                && matches!(name.path.as_slice(), [RoleSeg::FromA(inner)]
                    if matches!(inner.path.last(), Some(RoleSeg::Fragment(editor_core::Qualifier::Ends(_))))
                        && inner.node == notched
                        && inner.path.first() == line.path.first())
        })
        .map(|(name, _)| name.clone())
        .collect();
    assert_eq!(
        survivors.len(),
        2,
        "the notch's two pieces pass through whole: {survivors:?}"
    );
    for half in &halves {
        let res = resolve_with_prior(
            RunCtx {
                doc: &doc2,
                eval: &ev2,
            },
            RunCtx {
                doc: &doc,
                eval: &ev1,
            },
            half,
        );
        let f = failed(&res, "a vanished half");
        for s in &survivors {
            assert!(
                f.offers.contains(s),
                "{half:?} is offered {s:?} on its line; offers {:?}",
                f.offers
            );
        }
    }
}

/// **A union reads a cited line as its least member row on it.** The
/// notched bar and a block standing across the back top edge's first
/// piece, united: the union names each crossing of that edge by its
/// line, which no row of the bar's table is, and reads the sense along
/// it through the line's rows.
#[test]
fn a_union_reads_a_cited_member_line_by_its_rows() {
    let (doc, notched) = notched_bar("cited-line-union");
    let (doc, post) = block(doc, (3.0, 4.0), (1.5, 3.0), 0.5, 1.0);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: vec![notched, post],
            declare: Vec::new(),
        },
    );
    let ev = run(&doc, None);
    assert!(
        crate::docm7_union_declare::failure(&ev, u).is_none(),
        "the union evaluates: {:?}",
        crate::docm7_union_declare::failure(&ev, u)
    );
    let bar_rows = &ev.value(notched).unwrap().name_table;
    let cited: Vec<StableName> = crossings(&ev, u)
        .into_iter()
        .filter_map(|(_, edge)| match edge.path.as_slice() {
            [RoleSeg::FromMember { member, of }] if *member == notched => Some((**of).clone()),
            _ => None,
        })
        .collect();
    assert!(
        !cited.is_empty(),
        "the post crosses the bar's back top edge"
    );
    for line in &cited {
        assert!(
            bar_rows.lookup(line).is_none(),
            "{line:?} is the edge's line, which the bar's table holds no row of"
        );
    }
}

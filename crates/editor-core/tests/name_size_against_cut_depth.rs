//! **A name's size against how deep its piece sits in a chain of cuts**
//! (names README N2, *Edge pieces*: a piece is spelled on its parent
//! edge's line, so a piece of a piece is a piece of that line).
//!
//! A 20×2×1 bar, trimmed `k` times at alternate ends, each trim crossing
//! the bar's four long edges: a long edge's last piece sits `k` cuts
//! deep, and its ends are crossings of its line. Two chains build the
//! trims: a boolean subtract of a block past each cut, and a Split by a
//! plane at each cut with a Part keeping the inner half. Every trim's
//! table is read off one evaluation of the deepest chain.
//!
//! On each chain the longest name's serialized bytes grow at most
//! linearly in `k`. A boolean names a piece of an operand's piece as a
//! piece of the operand edge's line, so the boolean chain's longest name
//! says the same number of words at every depth; a Split's piece is
//! spelled with its side of each plane it lies past, one more per cut,
//! so the Split chain's words grow by at most a fixed count per cut.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{insert, len, on_frame, scl};
use editor_core::{
    BooleanOp, Datum, Evaluation, ExtrudeSide, Node, PartSelect, ProfileDoc, RecipeNodeId, Speaker,
    SplitHalf,
};
use geom_core::Tol;

/// The bar's length along x, meters.
const LENGTH: f64 = 20.0;
/// How far each trim cuts past the one before at the same end, meters.
const BITE: f64 = 0.5;
/// The deepest chain the gate reads.
const DEEPEST: usize = 14;

/// Where trim `i` (from 0) cuts, and whether it keeps the bar below
/// that x: trims alternate ends, the high end first.
fn cut(i: usize) -> (f64, bool) {
    let bite = BITE * (i / 2 + 1) as f64;
    if i.is_multiple_of(2) {
        (LENGTH - bite, true)
    } else {
        (bite, false)
    }
}

/// The bar, `[0, LENGTH] × [0, 2] × [0, 1]`.
fn bar(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (LENGTH, 0.0), (LENGTH, 2.0), (0.0, 2.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    )
}

/// The bar trimmed `DEEPEST` times by subtracting a block past each cut:
/// the document and each trim's node in order.
fn boolean_chain() -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (mut doc, mut body) = bar("trim-chain-boolean");
    let mut trims = Vec::with_capacity(DEEPEST);
    for i in 0..DEEPEST {
        let (x, keep_below) = cut(i);
        let (x0, x1) = if keep_below {
            (x, LENGTH + 1.0)
        } else {
            (-1.0, x)
        };
        let (d, profile) = on_frame(
            doc,
            [0.0, 0.0, -1.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x0, -1.0), (x1, -1.0), (x1, 3.0), (x0, 3.0)]],
        );
        let (d, block) = insert(
            d,
            Node::Extrude {
                profile: profile.into(),
                distance: len(3.0),
                side: ExtrudeSide::Along,
            },
        );
        let (d, trimmed) = insert(
            d,
            Node::Boolean {
                op: BooleanOp::Subtract,
                a: body.into(),
                b: block.into(),
                declare: Vec::new(),
            },
        );
        doc = d;
        body = trimmed;
        trims.push(trimmed);
    }
    (doc, trims)
}

/// The bar trimmed `DEEPEST` times by a Split at each cut and a Part
/// keeping the inner half: the document and each trim's Part in order.
fn split_chain() -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (mut doc, mut body) = bar("trim-chain-split");
    let mut trims = Vec::with_capacity(DEEPEST);
    for i in 0..DEEPEST {
        let (x, keep_below) = cut(i);
        let (d, tool) = insert(
            doc,
            Node::Datum(Datum::Plane {
                origin: [len(x), len(0.0), len(0.0)],
                normal: [scl(1.0), scl(0.0), scl(0.0)],
            }),
        );
        let (d, split) = insert(d, Node::Split { target: body.into(), tool: tool.into() });
        let half = if keep_below {
            SplitHalf::Below
        } else {
            SplitHalf::Above
        };
        let (d, part) = insert(
            d,
            Node::Part {
                of: split.into(),
                select: PartSelect::SplitHalf(half),
            },
        );
        doc = d;
        body = part;
        trims.push(part);
    }
    (doc, trims)
}

/// Each trim's longest name, in spoken words (said in full from the
/// document) and in serialized bytes, over every name its table holds.
fn longest(doc: &ProfileDoc, ev: &Evaluation<f64>, trims: &[RecipeNodeId]) -> Vec<(usize, usize)> {
    let speaker = Speaker::of(doc);
    trims
        .iter()
        .map(|&id| {
            let value = ev.value(id).expect("every trim evaluates");
            value
                .name_table
                .iter()
                .fold((0, 0), |(words, bytes), (name, _)| {
                    let w = speaker.name(name).to_string().split_whitespace().count();
                    let b = serde_json::to_string(name)
                        .expect("a name serializes")
                        .len();
                    (words.max(w), bytes.max(b))
                })
        })
        .collect()
}

/// Holds one chain's sizes to the gate, from trim 3 to [`DEEPEST`]: each
/// trim grows the longest name's bytes by at most one and a half times
/// trim 3's growth, and its words by at most trim 3's growth in words
/// (none, where `flat`).
fn hold(chain: &str, sizes: &[(usize, usize)], flat: bool) {
    for (k, (w, b)) in sizes.iter().enumerate() {
        println!("{chain}: k = {}: longest name {w} words, {b} bytes", k + 1);
    }
    // `sizes[k - 1]` is trim k's.
    let at = |k: usize| sizes[k - 1];
    let words_step = if flat {
        0
    } else {
        at(3).0.saturating_sub(at(2).0)
    };
    let bytes_step = at(3).1.saturating_sub(at(2).1).max(1);
    if flat {
        assert_eq!(
            at(3).0,
            at(2).0,
            "{chain}: the longest name grew from {} to {} words at trim 3",
            at(2).0,
            at(3).0,
        );
    }
    for k in 4..=DEEPEST {
        let (w0, b0) = at(k - 1);
        let (w1, b1) = at(k);
        assert!(
            w1 <= w0 + words_step,
            "{chain}: the longest name grew from {w0} to {w1} words at trim {k}, past trim 3's \
             growth of {words_step}",
        );
        assert!(
            b1.saturating_sub(b0) * 2 <= bytes_step * 3,
            "{chain}: the longest name grew from {b0} to {b1} bytes at trim {k}, past one and \
             a half times trim 3's growth of {bytes_step}",
        );
    }
}

#[test]
fn a_pieces_name_grows_at_most_linearly_with_the_cuts_it_sits_under() {
    let (doc, trims) = boolean_chain();
    let ev = crate::fixture::run(&doc, &editor_core::EvalOptions::default());
    let sizes = longest(&doc, &ev, &trims);
    hold("boolean chain", &sizes, true);

    let (doc, trims) = split_chain();
    let ev = crate::fixture::run(&doc, &editor_core::EvalOptions::default());
    let sizes = longest(&doc, &ev, &trims);
    hold("split chain", &sizes, false);
}

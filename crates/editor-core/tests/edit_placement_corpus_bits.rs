//! **Every corpus `Transform` places its body by the same bits** — the
//! value channel of each `Node::Transform` in the registry, pinned per
//! node against digests taken on the merge base before the node held a
//! `Placement`.
//!
//! The digest is the fixture's value channel (`value_digest_nodes`):
//! the placed bodies' arena counts and every stored point, so a motion
//! that moved one bit of one coordinate moves the word. A golden in the
//! ordinary sense: when one moves, the question is whether the new
//! motion is right, never how to restore the old number.

use crate::corpus;
use crate::fixture::value_channel::value_digest_nodes;
use corpus::{eval, failures};
use editor_core::Node;

/// `(document, node id, digest)` for every `Transform` the registry
/// holds, in registry then document order — taken on `569540027`, and
/// the same at the default, `1e-6` and `1e-12` ε rows.
const PINNED: &[(&str, u64, u64)] = &[
    ("die", 21, 0xacc7bc9d5b6159ae),
    ("die", 24, 0x1bac8b9125b3e4bb),
    ("die", 27, 0xae8b02ea28f16880),
    ("die", 30, 0x5bec552c39aaab35),
    ("die", 33, 0x0aa23852df2dbeea),
    ("die", 36, 0x01509d88eac2f23b),
    ("die", 39, 0xb462bd6824e44604),
    ("die", 42, 0x5e2cad577e3b2b59),
    ("die", 45, 0x5f57786164eab056),
    ("die", 48, 0x43891853cff67e9b),
    ("die", 51, 0x4d9257f55b74fb08),
    ("die", 54, 0xcf46edbffd9731bd),
    ("die", 57, 0x05855ecc0c371a2a),
    ("die", 60, 0x62a0fc734230ecd7),
    ("die", 63, 0x0a1685afd5791e24),
    ("die", 66, 0x763d1bf89f5f9609),
    ("die", 69, 0xb0324e9e1027471e),
    ("die", 72, 0x73dfc2f56359940b),
    ("die", 75, 0x99d07143189df4b8),
    ("die", 78, 0x62cf108145420595),
    ("die", 81, 0xfc0003a456febf1a),
    ("heat_sink", 7, 0x37d9d66beb97600c),
    ("heat_sink", 9, 0xf3207ae427b9e4da),
    ("heat_sink", 11, 0x330609d22c046680),
    ("heat_sink", 13, 0xad855093cc44b526),
    ("heat_sink", 15, 0x71ce2cde1ca5fb1c),
    ("kitchen_sink", 12, 0x363844639dd1c817),
    ("die_pips", 7, 0x07a4de2bdb39a44c),
    ("part_select", 11, 0x1dc197b226ffb460),
    ("die_composed", 7, 0x07a4de2bdb39a44c),
    ("die_composed_tour", 8, 0x44527dc0659492db),
    ("die_composed_tour", 9, 0xe15afef931c64658),
    ("die_composed_tour", 10, 0xa46d2187dcb750b3),
    ("die_composed_tour", 11, 0xdd9d8848f8bf47c6),
    ("die_composed_tour", 12, 0x42b6c55ddb4ec641),
    ("die_composed_tour", 13, 0x0bbb3026d73e1fcc),
    ("die_composed_tour", 14, 0x356b8292cd8b2b2b),
    ("die_composed_tour", 15, 0xa9ac66f4be8e4b94),
    ("die_composed_tour", 16, 0x69322f5c1bcac0c3),
    ("die_composed_tour", 17, 0x87800db71ba1fdd0),
    ("die_composed_tour", 18, 0x06aab04d8828f253),
    ("die_composed_tour", 19, 0x7c5e810892ba1be2),
    ("die_composed_tour", 20, 0x85c822e4387556d5),
    ("die_composed_tour", 21, 0x1122cd513d2a2814),
    ("die_composed_tour", 22, 0xcfd88318a5841eed),
    ("die_composed_tour", 23, 0x223bedc1a6c855a8),
    ("die_composed_tour", 24, 0x7ef6f2ec603acac7),
    ("die_composed_tour", 25, 0x32bc2261c665d4e4),
    ("die_composed_tour", 26, 0x88ae29bf58f61873),
    ("die_composed_tour", 27, 0xa5b6b7a02136884a),
    ("die_composed_tour", 28, 0x073048d15ccf9421),
];

#[test]
fn every_corpus_transform_places_its_body_by_the_pinned_bits() {
    let mut got: Vec<(&'static str, u64, u64)> = Vec::new();
    for doc in corpus::documents() {
        let transforms: Vec<_> = doc
            .doc
            .order()
            .iter()
            .copied()
            .filter(|id| matches!(doc.doc.node(*id), Some(Node::Transform { .. })))
            .collect();
        if transforms.is_empty() {
            continue;
        }
        let ev = eval::<f64>(&doc.doc);
        let bad = failures(&ev);
        assert!(bad.is_empty(), "{} failed to evaluate: {bad:?}", doc.name);
        for (id, word, fed) in value_digest_nodes(&ev) {
            if transforms.contains(&id) {
                assert!(fed > 0, "{} node {}: nothing was digested", doc.name, id.0);
                got.push((doc.name, id.0, word));
            }
        }
    }
    for (name, id, word) in &got {
        println!("    (\"{name}\", {id}, {word:#018x}),");
    }
    assert_eq!(
        got.as_slice(),
        PINNED,
        "a corpus transform's placed body moved"
    );
}

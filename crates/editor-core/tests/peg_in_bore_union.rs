//! **A peg standing in a bore of the SAME radius unions** — the
//! curved `Rest` contact, on the smallest geometry that shows it: the
//! block overshoots the peg both ways, so **no plane of one body
//! coincides with a plane of the other** and the only coincidence is
//! the cylinder. The margins decide the walls one carrier, the join
//! glues them, and the peg's rim arcs are edges of both solids, which
//! the chord join copies.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, EvalOptions, Evaluation, LoopProgram, Node, NodeResult, ProfileDoc,
    ProfileProgram, RecipeNodeId, ValuePayload, evaluate,
};
use geom_core::Tol;
use topo::Body;

use fixture::{insert, len, square};

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// The peg and its block, with the bore's radius a knob: the peg is a
/// circle of radius `PEG_R` extruded `z ∈ [0, 1]`, the block a square
/// with an inner circle of radius `bore_r` on the SAME centre,
/// extruded `z ∈ [−0.5, 1.5]`.
///
/// The block overshoots the peg at both ends ON PURPOSE: with the caps
/// clear of each other the two bodies share no plane at all.
const PEG_R: f64 = 0.25;

fn peg_in_bore(bore_r: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("seatfw_curved_flush", Tol::witness());
    let circle = |r: f64| LoopProgram::Circle {
        centre: [len(0.0), len(0.0)],
        radius: len(r),
    };
    let (doc, peg_profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![circle(PEG_R)],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, peg) = insert(
        doc,
        Node::Extrude {
            profile: peg_profile.into(),
            distance: len(1.0),
        },
    );
    let (doc, block_profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![
                LoopProgram::polygon(square(0.0, 0.0, 1.0)).expect("finite corners"),
                circle(bore_r),
            ],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile: block_profile.into(),
            distance: len(2.0),
        },
    );
    let (doc, block) = fixture::slid(doc, peg, block, [0.0, 0.0, -0.5]);
    (doc, peg, block)
}

/// The evaluated body of one node: an extrude's, or a placement's one
/// copy.
fn body_of(ev: &Evaluation<f64>, n: RecipeNodeId) -> &Body<f64> {
    match &ev.value(n).expect("the node evaluates").payload {
        ValuePayload::Body(b) => b.body(),
        other => panic!("{n}'s payload is one body, got {}", other.kind_name()),
    }
}

/// **The peg and the block union into one sound body**: tiers 2 and
/// 3, the block's and the peg's volumes summed (they only touch), and a
/// legal operand.
#[test]
fn a_peg_in_a_bore_of_its_own_radius_unions() {
    let (doc, peg, block) = peg_in_bore(PEG_R);
    let ev = eval(&doc);
    let want = [peg, block]
        .map(|n| {
            topo::test_support::pinned_volume(
                "the measured body",
                &topo::test_support::finished(
                    "the measured body",
                    topo::Body::clone(body_of(&ev, n)),
                    Tol::witness(),
                ),
                Tol::witness(),
            )
            .volume
        })
        .iter()
        .sum::<f64>();
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![peg.into(), block.into()]),
        },
    );
    let ev = eval(&doc);
    let value = match ev.nodes.get(&union) {
        Some(NodeResult::Ok(value)) => value,
        other => panic!("the peg-in-bore union builds: {other:?}"),
    };
    let editor_core::ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. }) =
        &value.payload
    else {
        panic!("a union's value is a body");
    };
    assert_eq!(topo::validate_closed(body.body()), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(body.body(), Tol::witness()),
        Ok(()),
        "tier 3"
    );
    let v = topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(body.body()),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .volume;
    assert!(
        (v - want).abs() <= 1e-9 * want,
        "the peg and the block only touch: {v} against {want}"
    );
    sweep::test_support::assert_legal_operand("peg ∪ block", body.body(), Tol::witness());
}

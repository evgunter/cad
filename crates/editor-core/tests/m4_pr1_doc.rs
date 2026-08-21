//! The DIE as a document (spec D8): the 21-pip die authored as DATA
//! through `apply` calls, two variants diffed (changed pip depth),
//! and replay identity from the empty document (spec D7).
//!
//! Authoring choice (D8 left it open, reported in the PR): 21
//! EXPLICIT Transform + Subtract chains, not patterns — a die's pip
//! layout is not a uniform lattice (1..6 pips per face at standard
//! positions); patterns earn their keep in their own unit tests.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{Dimension, Doc, DocEdit, DocParam, Expr, Node, ParamName, RecipeNodeId, SlotId};
use geom_core::Tol;

/// Opaque profile payload (spec D1/D3): tests never look inside.
#[derive(Debug, Clone, PartialEq)]
struct FakeProfile(&'static str);
// The v4 payload trait: fake payloads take the slot-free, check-free
// defaults (LIB-SWITCH §4c — exactly the retired opaque behavior).
impl editor_core::ProfilePayload for FakeProfile {}

type TDoc = Doc<FakeProfile>;
type TEdit = DocEdit<FakeProfile>;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).unwrap()
}

fn ang(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).unwrap()
}

fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).unwrap()
}

/// Applies an edit, records it in the replay log, returns the doc
/// (and the minted id for inserts).
fn step(doc: TDoc, log: &mut Vec<TEdit>, edit: TEdit) -> (TDoc, Option<RecipeNodeId>) {
    let applied = doc.apply(&edit, Tol::witness()).unwrap();
    log.push(edit);
    (applied.doc, applied.record.minted)
}

/// Per-face pip layout: (face normal axis as unit translation,
/// rotation axis, rotation angle, pip (u,v) offsets in pip-pitch
/// units). Opposite faces sum to 7 on a real die.
type Face = ([f64; 3], [f64; 3], f64, Vec<(f64, f64)>);

fn faces() -> [Face; 6] {
    use std::f64::consts::{FRAC_PI_2, PI};
    let corners4 = vec![(-1.0, -1.0), (-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)];
    let mut five = corners4.clone();
    five.push((0.0, 0.0));
    let six = vec![
        (-1.0, -1.0),
        (-1.0, 0.0),
        (-1.0, 1.0),
        (1.0, -1.0),
        (1.0, 0.0),
        (1.0, 1.0),
    ];
    [
        // 1: +Z (pip tool extrudes along +Z; flip to cut downward).
        ([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], PI, vec![(0.0, 0.0)]),
        // 2: +X.
        (
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            FRAC_PI_2,
            vec![(-1.0, -1.0), (1.0, 1.0)],
        ),
        // 3: +Y.
        (
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
            -FRAC_PI_2,
            vec![(-1.0, -1.0), (0.0, 0.0), (1.0, 1.0)],
        ),
        // 4: -Y.
        ([0.0, -1.0, 0.0], [1.0, 0.0, 0.0], FRAC_PI_2, corners4),
        // 5: -X.
        ([-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], -FRAC_PI_2, five),
        // 6: -Z (tool already cuts upward into the bottom face).
        ([0.0, 0.0, -1.0], [1.0, 0.0, 0.0], 0.0, six),
    ]
}

struct Die {
    doc: TDoc,
    log: Vec<TEdit>,
    pip_extrude: RecipeNodeId,
}

/// Authors the 21-pip die purely through `apply` (spec D8).
fn author_die() -> Die {
    const HALF: f64 = 0.010; // 20 mm cube
    const PITCH: f64 = 0.005; // pip grid pitch
    let mut log = Vec::new();
    let doc = TDoc::empty_derived("m4_pr1_doc", Tol::witness());
    // Document parameter: pip depth (the variant knob).
    let (doc, _) = step(
        doc,
        &mut log,
        TEdit::SetDocParam {
            name: ParamName::new("pip_depth"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.002,
            },
        },
    );
    // Cube: profile wrap + extrude.
    let (doc, cube_profile) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Node::Profile(FakeProfile("square-20mm")),
        },
    );
    let (doc, cube) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Node::Extrude {
                profile: cube_profile.unwrap(),
                distance: len(2.0 * HALF),
            },
        },
    );
    // Pip tool: profile wrap + extrude by the pip_depth parameter.
    let (doc, pip_profile) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Node::Profile(FakeProfile("circle-2mm")),
        },
    );
    let (mut doc, pip_extrude) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Node::Extrude {
                profile: pip_profile.unwrap(),
                distance: Expr::param(ParamName::new("pip_depth"), Dimension::Length),
            },
        },
    );
    let pip_extrude = pip_extrude.unwrap();
    // 21 pips: place the tool, subtract from the running body.
    let mut body = cube.unwrap();
    let mut pips = 0;
    for (normal, rot_axis, rot_angle, offsets) in faces() {
        // In-face basis: any two axes orthogonal to the face normal.
        let (du, dv) = if normal[0] != 0.0 {
            ([0.0, 1.0, 0.0], [0.0, 0.0, 1.0])
        } else if normal[1] != 0.0 {
            ([1.0, 0.0, 0.0], [0.0, 0.0, 1.0])
        } else {
            ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0])
        };
        for (u, v) in offsets {
            let t: Vec<f64> = (0..3)
                .map(|i| HALF * normal[i] + PITCH * (u * du[i] + v * dv[i]))
                .collect();
            let (d2, placed) = step(
                doc,
                &mut log,
                TEdit::InsertNode {
                    node: Node::Transform {
                        input: pip_extrude,
                        translation: [len(t[0]), len(t[1]), len(t[2])],
                        rotation_axis: [scl(rot_axis[0]), scl(rot_axis[1]), scl(rot_axis[2])],
                        rotation_angle: ang(rot_angle),
                    },
                },
            );
            let (d3, cut) = step(
                d2,
                &mut log,
                TEdit::InsertNode {
                    node: Node::Boolean {
                        op: editor_core::BooleanOp::Subtract,
                        a: body,
                        b: placed.unwrap(),
                        declare: None,
                    },
                },
            );
            doc = d3;
            body = cut.unwrap();
            pips += 1;
        }
    }
    assert_eq!(pips, 21);
    Die {
        doc,
        log,
        pip_extrude,
    }
}

#[test]
fn die_authors_replays_and_diffs() {
    let die = author_die();
    // 2 profiles + 2 extrudes + 21 transforms + 21 subtracts.
    assert_eq!(die.doc.len(), 46);

    // Replay identity (spec D7): from empty, BIT-IDENTICAL.
    let replayed = TDoc::replay(die.doc.id(), &die.log, Tol::witness()).unwrap();
    assert_eq!(replayed, die.doc);
    assert!(replayed.diff(&die.doc).is_empty());
    assert_eq!(replayed.epsilon().to_bits(), die.doc.epsilon().to_bits());

    // Variant: pip depth changed ON THE NODE (a continuous SetParam),
    // so the node-granular diff reports exactly that node.
    let variant = die
        .doc
        .apply(
            &TEdit::SetParam {
                node: die.pip_extrude,
                slot: SlotId::Distance,
                expr: len(0.003),
            },
            Tol::witness(),
        )
        .unwrap();
    assert!(!variant.record.structural, "continuous edit");
    let d = die.doc.diff(&variant.doc);
    assert_eq!(
        d.nodes,
        vec![editor_core::NodeChange::Changed(die.pip_extrude)]
    );
    assert!(d.params.is_empty() && !d.order_changed && !d.epsilon_changed);

    // Variant 2: pip depth changed through the DOC PARAM the pip
    // extrude references — node payloads identical, param diff only.
    let variant2 = die
        .doc
        .apply(
            &TEdit::SetDocParam {
                name: ParamName::new("pip_depth"),
                value: DocParam::Continuous {
                    dim: Dimension::Length,
                    value: 0.003,
                },
            },
            Tol::witness(),
        )
        .unwrap();
    let d2 = die.doc.diff(&variant2.doc);
    assert!(d2.nodes.is_empty());
    assert_eq!(d2.params, vec![ParamName::new("pip_depth")]);

    // The original document is untouched by all of the above (D2:
    // apply is pure).
    assert_eq!(die.doc.len(), 46);
    assert!(
        die.doc
            .diff(&TDoc::replay(die.doc.id(), &die.log, Tol::witness()).unwrap())
            .is_empty()
    );
}

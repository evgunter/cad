//! ADVERSARIAL REVIEW R7 — the die acceptance, independently
//! re-authored with a DIFFERENT edit order (prelude nodes permuted;
//! all 21 Transforms inserted before any Subtract, instead of the
//! PR's interleaved Transform+Subtract pairs). Verifies:
//!  - replay bit-identity holds for BOTH authorings;
//!  - the two docs are payload-ISOMORPHIC under the id relabeling
//!    induced by the authoring orders (slot floats bit-equal);
//!  - `diff` between them is EXACTLY the relabeling residue: the
//!    shared first insert unchanged, every other node of one Removed
//!    and of the other Added (the two edit sequences part after the
//!    first insert, so their mints do), no param/ε deltas.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{ang, len, scl};
use editor_core::{
    Dimension, Doc, DocEdit, DocParam, Expr, Node, NodeChange, ParamName, RecipeNodeId, eval,
};
use geom_core::Tol;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
struct FakeProfile(&'static str);
// The v4 payload trait: fake payloads take the slot-free, check-free
// defaults (LIB-SWITCH §4c — exactly the retired opaque behavior).
impl editor_core::ProfilePayload for FakeProfile {
    fn drawn_pieces(
        &self,
        _env: &editor_core::ParamEnv<f64>,
        _tol: geom_core::Tol,
    ) -> Result<std::collections::BTreeSet<editor_core::ProfileEdgeRef>, editor_core::ProgramRefusal>
    {
        Ok(std::collections::BTreeSet::new())
    }
}

type TDoc = Doc<FakeProfile>;
type TEdit = DocEdit<FakeProfile>;

const HALF: f64 = 0.010;
const PITCH: f64 = 0.005;

/// Same pip geometry as the PR's die (m4_pr1_doc.rs `faces()`).
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
        ([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], PI, vec![(0.0, 0.0)]),
        (
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            FRAC_PI_2,
            vec![(-1.0, -1.0), (1.0, 1.0)],
        ),
        (
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
            -FRAC_PI_2,
            vec![(-1.0, -1.0), (0.0, 0.0), (1.0, 1.0)],
        ),
        ([0.0, -1.0, 0.0], [1.0, 0.0, 0.0], FRAC_PI_2, corners4),
        ([-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], -FRAC_PI_2, five),
        ([0.0, 0.0, -1.0], [1.0, 0.0, 0.0], 0.0, six),
    ]
}

/// The 21 (translation, rotation_axis, rotation_angle) placements, in
/// the PR's face/pip order.
fn placements() -> Vec<([f64; 3], [f64; 3], f64)> {
    let mut out = Vec::new();
    for (normal, rot_axis, rot_angle, offsets) in faces() {
        let (du, dv) = if normal[0] != 0.0 {
            ([0.0, 1.0, 0.0], [0.0, 0.0, 1.0])
        } else if normal[1] != 0.0 {
            ([1.0, 0.0, 0.0], [0.0, 0.0, 1.0])
        } else {
            ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0])
        };
        for (u, v) in offsets {
            let mut t = [0.0; 3];
            for i in 0..3 {
                t[i] = HALF * normal[i] + PITCH * (u * du[i] + v * dv[i]);
            }
            out.push((t, rot_axis, rot_angle));
        }
    }
    assert_eq!(out.len(), 21);
    out
}

fn step(doc: TDoc, log: &mut Vec<TEdit>, edit: TEdit) -> (TDoc, Option<RecipeNodeId>) {
    let applied = doc
        .apply(&edit, Tol::witness(), &editor_core::RefusingReach)
        .unwrap();
    log.push(edit);
    (applied.doc, applied.record.minted)
}

fn transform_node(pip: RecipeNodeId, p: &([f64; 3], [f64; 3], f64)) -> Node<FakeProfile> {
    let (t, r, a) = p;
    Node::transform(
        pip,
        editor_core::Step::Rigid {
            translation: [len(t[0]), len(t[1]), len(t[2])],
            axis: [scl(r[0]), scl(r[1]), scl(r[2])],
            angle: ang(*a),
        },
    )
}

fn subtract_node(a: RecipeNodeId, b: RecipeNodeId) -> Node<FakeProfile> {
    Node::Boolean {
        op: editor_core::BooleanOp::Subtract,
        a,
        b,
        declare: None,
    }
}

struct Authored {
    doc: TDoc,
    log: Vec<TEdit>,
    /// Ids by role: [cube_profile, cube_extrude, pip_profile,
    /// pip_extrude], then 21 transforms, then 21 subtracts.
    roles: Vec<RecipeNodeId>,
}

fn depth_param() -> TEdit {
    TEdit::SetDocParam {
        name: ParamName::from_static("pip_depth"),
        value: DocParam::continuous(Dimension::Length, 0.002),
    }
}

/// The PR's authoring order: param, cube pair, pip pair, then 21
/// interleaved (Transform, Subtract) pairs.
fn author_theirs() -> Authored {
    let mut log = Vec::new();
    let (doc, _) = step(
        TDoc::empty_derived("review_m4_pr1_die", Tol::witness()),
        &mut log,
        depth_param(),
    );
    let (doc, cube_p) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Box::new(Node::Profile(FakeProfile("square-20mm"))),
        },
    );
    let (doc, cube) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Box::new(Node::Extrude {
                profile: cube_p.unwrap(),
                distance: len(2.0 * HALF),
            }),
        },
    );
    let (doc, pip_p) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Box::new(Node::Profile(FakeProfile("circle-2mm"))),
        },
    );
    let (mut doc, pip_e) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Box::new(Node::Extrude {
                profile: pip_p.unwrap(),
                distance: Expr::param(ParamName::from_static("pip_depth"), Dimension::Length),
            }),
        },
    );
    let pip_e = pip_e.unwrap();
    let mut body = cube.unwrap();
    let (mut transforms, mut subtracts) = (Vec::new(), Vec::new());
    for p in placements() {
        let (d2, placed) = step(
            doc,
            &mut log,
            TEdit::InsertNode {
                node: Box::new(transform_node(pip_e, &p)),
            },
        );
        let (d3, cut) = step(
            d2,
            &mut log,
            TEdit::InsertNode {
                node: Box::new(subtract_node(body, placed.unwrap())),
            },
        );
        doc = d3;
        body = cut.unwrap();
        transforms.push(placed.unwrap());
        subtracts.push(cut.unwrap());
    }
    let mut roles = vec![cube_p.unwrap(), cube.unwrap(), pip_p.unwrap(), pip_e];
    roles.extend(transforms);
    roles.extend(subtracts);
    Authored { doc, log, roles }
}

/// MY authoring order: cube profile, pip profile, param, pip extrude,
/// cube extrude — then ALL 21 transforms, then ALL 21 subtracts.
fn author_mine() -> Authored {
    let mut log = Vec::new();
    let (doc, cube_p) = step(
        TDoc::empty_derived("review_m4_pr1_die", Tol::witness()),
        &mut log,
        TEdit::InsertNode {
            node: Box::new(Node::Profile(FakeProfile("square-20mm"))),
        },
    );
    let (doc, pip_p) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Box::new(Node::Profile(FakeProfile("circle-2mm"))),
        },
    );
    let (doc, _) = step(doc, &mut log, depth_param());
    let (doc, pip_e) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Box::new(Node::Extrude {
                profile: pip_p.unwrap(),
                distance: Expr::param(ParamName::from_static("pip_depth"), Dimension::Length),
            }),
        },
    );
    let (mut doc, cube) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Box::new(Node::Extrude {
                profile: cube_p.unwrap(),
                distance: len(2.0 * HALF),
            }),
        },
    );
    let pip_e = pip_e.unwrap();
    let mut transforms = Vec::new();
    for p in placements() {
        let (d2, placed) = step(
            doc,
            &mut log,
            TEdit::InsertNode {
                node: Box::new(transform_node(pip_e, &p)),
            },
        );
        doc = d2;
        transforms.push(placed.unwrap());
    }
    let mut body = cube.unwrap();
    let mut subtracts = Vec::new();
    for &t in &transforms {
        let (d2, cut) = step(
            doc,
            &mut log,
            TEdit::InsertNode {
                node: Box::new(subtract_node(body, t)),
            },
        );
        doc = d2;
        body = cut.unwrap();
        subtracts.push(cut.unwrap());
    }
    let mut roles = vec![cube_p.unwrap(), cube.unwrap(), pip_p.unwrap(), pip_e];
    roles.extend(transforms);
    roles.extend(subtracts);
    Authored { doc, log, roles }
}

/// Bit-compare two nodes under an id relabeling: same variant, inputs
/// map through the role bijection, slot expressions eval bit-equal.
fn assert_role_isomorphic(theirs: &Authored, mine: &Authored) {
    use std::collections::BTreeMap;
    let map: BTreeMap<RecipeNodeId, RecipeNodeId> = theirs
        .roles
        .iter()
        .copied()
        .zip(mine.roles.iter().copied())
        .collect();
    assert_eq!(map.len(), 46, "role bijection covers all nodes");
    for (&t_id, &m_id) in &map {
        let (tn, mn) = (theirs.doc.node(t_id).unwrap(), mine.doc.node(m_id).unwrap());
        assert_eq!(
            std::mem::discriminant(tn),
            std::mem::discriminant(mn),
            "variant of {t_id:?}"
        );
        let mapped: Vec<RecipeNodeId> = tn.inputs().iter().map(|i| map[i]).collect();
        assert_eq!(mapped, mn.inputs(), "inputs of {t_id:?}→{m_id:?}");
        assert_eq!(tn.slots(), mn.slots());
        for slot in tn.slots() {
            let tv = eval::<f64>(tn.expr(slot).unwrap(), &theirs.doc.param_env()).unwrap();
            let mv = eval::<f64>(mn.expr(slot).unwrap(), &mine.doc.param_env()).unwrap();
            assert_eq!(tv.to_bits(), mv.to_bits(), "slot {slot:?} of {t_id:?}");
        }
    }
}

#[test]
fn r7_die_reauthored_different_order_isomorphic_and_diff_exact() {
    let theirs = author_theirs();
    let mine = author_mine();
    assert_eq!(theirs.doc.len(), 46);
    assert_eq!(mine.doc.len(), 46);

    // Replay identity holds for BOTH edit orders (PartialEq + the
    // stricter role-isomorphism check against self is implied).
    assert_eq!(
        TDoc::replay(theirs.doc.id(), &theirs.log.to_vec(), Tol::witness()).unwrap(),
        theirs.doc
    );
    assert_eq!(
        TDoc::replay(mine.doc.id(), &mine.log.to_vec(), Tol::witness()).unwrap(),
        mine.doc
    );

    // The two authorings are payload-isomorphic under relabeling.
    assert_role_isomorphic(&theirs, &mine);

    // The diff is EXACTLY the relabeling residue. The first insert is
    // one edit from one mint in both authorings, so it is one id and
    // unchanged; from the second on the edit sequences differ, so the
    // two share no other id: the rest of theirs Removed in its order,
    // the rest of mine Added in its.
    assert_eq!(
        theirs.doc.order()[0],
        mine.doc.order()[0],
        "one first edit, one first id"
    );
    let d = theirs.doc.diff(&mine.doc);
    let expected: Vec<NodeChange> = theirs.doc.order()[1..]
        .iter()
        .copied()
        .map(NodeChange::Removed)
        .chain(mine.doc.order()[1..].iter().copied().map(NodeChange::Added))
        .collect();
    assert_eq!(d.nodes, expected, "diff is exactly the relabeling residue");
    assert!(d.params.is_empty(), "same params");
    assert!(d.order_changed, "the orders share only the first id");
    assert!(!d.epsilon_changed && !d.metadata_changed);
}

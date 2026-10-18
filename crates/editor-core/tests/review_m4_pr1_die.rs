//! ADVERSARIAL REVIEW R7 — the die acceptance, independently
//! re-authored with a DIFFERENT edit order (prelude nodes permuted;
//! all 21 placements inserted before any Subtract, instead of the
//! PR's interleaved placement+Subtract pairs). Verifies:
//!  - replay bit-identity holds for BOTH authorings;
//!  - the two docs are payload-ISOMORPHIC under the id relabeling
//!    induced by the authoring orders (slot floats bit-equal);
//!  - `diff` between them is EXACTLY the relabeling residue: the
//!    shared first insert unchanged, every other node of one Removed
//!    and of the other Added (the two edit sequences part after the
//!    first insert, so their mints do), no param/ε deltas.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{ang, len};
use editor_core::{
    Dimension, Doc, DocEdit, Formula, FreeVar, Node, NodeChange, RecipeNodeId, VarName, eval,
};
use geom_core::Tol;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
struct FakeProfile(&'static str);
// The v4 payload trait: fake payloads take the slot-free, check-free
// defaults (LIB-SWITCH §4c — exactly the retired opaque behavior).
impl editor_core::SlotPayload<editor_core::VarId> for FakeProfile {}
impl editor_core::SlotPayload<editor_core::Formula> for FakeProfile {}
impl editor_core::ProfilePayload for FakeProfile {
    type Authored = Self;
    fn lower<E>(
        authored: &Self,
        _: &mut dyn FnMut(&editor_core::Formula) -> Result<editor_core::VarId, E>,
    ) -> Result<Self, E> {
        Ok(authored.clone())
    }
    fn authored_with(
        &self,
        _: &mut dyn FnMut(editor_core::VarId, editor_core::Dimension) -> editor_core::Formula,
    ) -> Self {
        self.clone()
    }
    fn drawn_pieces(
        &self,
        _env: &editor_core::VarEnv<f64>,
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

/// A placed copy of the pip at `p`: a placement of it, a plane mate of
/// its start cap to the world's `xy`, and its slides and spin as
/// values. Returns the document and the placement.
fn place_pip(
    doc: TDoc,
    log: &mut Vec<TEdit>,
    pip: RecipeNodeId,
    p: &([f64; 3], [f64; 3], f64),
) -> (TDoc, RecipeNodeId) {
    use editor_core::{Coord, Operand, SlideAxis};
    let (t, _, a) = p;
    let (doc, placement) = step(
        doc,
        log,
        TEdit::InsertNode {
            node: Box::new(Node::place(vec![editor_core::BodyRead::plain(
                Operand::Node(pip),
            )])),
            fresh: Vec::new(),
        },
    );
    let placement = placement.expect("a placement mints");
    let mate = editor_core::Mate {
        on: crate::fixture::plane(Operand::select(
            pip,
            vec![crate::fixture::fname(
                pip,
                editor_core::RoleSeg::Cap(editor_core::CapEnd::Start),
            )],
        )),
        to: editor_core::Target::World(editor_core::WorldRead::through(
            editor_core::pose::PoseRole::Xy,
        )),
        class: editor_core::ContactClass::Rest,
    };
    let edit = TEdit::AddConstraint {
        placement,
        mate: Box::new(mate),
        fresh: Vec::new(),
    };
    let applied = doc
        .apply(&edit, Tol::witness(), &editor_core::RefusingReach)
        .unwrap();
    log.push(edit);
    let id = applied.record.minted_constraints[0];
    let doc = [
        (Coord::Slide(SlideAxis::X), len(t[0])),
        (Coord::Slide(SlideAxis::Y), len(t[1])),
        (Coord::Spin, ang(*a)),
    ]
    .into_iter()
    .fold(applied.doc, |doc, (coord, by)| {
        step(
            doc,
            log,
            TEdit::AddValue {
                placement,
                value: Box::new(editor_core::Value { mate: id, coord, by }),
                fresh: Vec::new(),
            },
        )
        .0
    });
    (doc, placement)
}

fn subtract_node(a: RecipeNodeId, b: RecipeNodeId) -> Node<FakeProfile, Formula> {
    Node::Subtract {
        from: a.into(),
        tool: b.into(),
    }
}

struct Authored {
    doc: TDoc,
    log: Vec<TEdit>,
    /// Ids by role: [cube_profile, cube_extrude, pip_profile,
    /// pip_extrude], then 21 placements, then 21 subtracts.
    roles: Vec<RecipeNodeId>,
}

fn depth_param() -> TEdit {
    TEdit::DeclareVar {
        name: VarName::from_static("pip_depth"),
        def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.002)),
    }
}

/// The PR's authoring order: param, cube pair, pip pair, then 21
/// interleaved (placement, Subtract) pairs.
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
            fresh: Vec::new(),
        },
    );
    let (doc, cube) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Box::new(Node::Extrude {
                profile: cube_p.unwrap().into(),
                distance: len(2.0 * HALF),
            }),
            fresh: Vec::new(),
        },
    );
    let (doc, pip_p) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Box::new(Node::Profile(FakeProfile("circle-2mm"))),
            fresh: Vec::new(),
        },
    );
    let (mut doc, pip_e) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Box::new(Node::Extrude {
                profile: pip_p.unwrap().into(),
                distance: Formula::named(VarName::from_static("pip_depth"), Dimension::Length),
            }),
            fresh: Vec::new(),
        },
    );
    let pip_e = pip_e.unwrap();
    let mut body = cube.unwrap();
    let (mut copies, mut subtracts) = (Vec::new(), Vec::new());
    for p in placements() {
        let (d2, placed) = place_pip(doc, &mut log, pip_e, &p);
        let (d3, cut) = step(
            d2,
            &mut log,
            TEdit::InsertNode {
                node: Box::new(subtract_node(body, placed)),
                fresh: Vec::new(),
            },
        );
        doc = d3;
        body = cut.unwrap();
        copies.push(placed);
        subtracts.push(cut.unwrap());
    }
    let mut roles = vec![cube_p.unwrap(), cube.unwrap(), pip_p.unwrap(), pip_e];
    roles.extend(copies);
    roles.extend(subtracts);
    Authored { doc, log, roles }
}

/// MY authoring order: cube profile, pip profile, param, pip extrude,
/// cube extrude — then ALL 21 placements, then ALL 21 subtracts.
fn author_mine() -> Authored {
    let mut log = Vec::new();
    let (doc, cube_p) = step(
        TDoc::empty_derived("review_m4_pr1_die", Tol::witness()),
        &mut log,
        TEdit::InsertNode {
            node: Box::new(Node::Profile(FakeProfile("square-20mm"))),
            fresh: Vec::new(),
        },
    );
    let (doc, pip_p) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Box::new(Node::Profile(FakeProfile("circle-2mm"))),
            fresh: Vec::new(),
        },
    );
    let (doc, _) = step(doc, &mut log, depth_param());
    let (doc, pip_e) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Box::new(Node::Extrude {
                profile: pip_p.unwrap().into(),
                distance: Formula::named(VarName::from_static("pip_depth"), Dimension::Length),
            }),
            fresh: Vec::new(),
        },
    );
    let (mut doc, cube) = step(
        doc,
        &mut log,
        TEdit::InsertNode {
            node: Box::new(Node::Extrude {
                profile: cube_p.unwrap().into(),
                distance: len(2.0 * HALF),
            }),
            fresh: Vec::new(),
        },
    );
    let pip_e = pip_e.unwrap();
    let mut copies = Vec::new();
    for p in placements() {
        let (d2, placed) = place_pip(doc, &mut log, pip_e, &p);
        doc = d2;
        copies.push(placed);
    }
    let mut body = cube.unwrap();
    let mut subtracts = Vec::new();
    for &t in &copies {
        let (d2, cut) = step(
            doc,
            &mut log,
            TEdit::InsertNode {
                node: Box::new(subtract_node(body, t)),
                fresh: Vec::new(),
            },
        );
        doc = d2;
        body = cut.unwrap();
        subtracts.push(cut.unwrap());
    }
    let mut roles = vec![cube_p.unwrap(), cube.unwrap(), pip_p.unwrap(), pip_e];
    roles.extend(copies);
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
        let mapped: Vec<RecipeNodeId> = theirs.doc.upstream(t_id).iter().map(|i| map[i]).collect();
        assert_eq!(
            mapped,
            mine.doc.upstream(m_id),
            "upstream of {t_id:?}→{m_id:?}"
        );
        assert_eq!(tn.slots(), mn.slots());
        for slot in tn.slots() {
            let tv = eval::<f64>(
                &theirs.doc.slot_expansion(t_id, slot).unwrap(),
                &theirs.doc.var_env(),
            )
            .unwrap();
            let mv = eval::<f64>(
                &mine.doc.slot_expansion(m_id, slot).unwrap(),
                &mine.doc.var_env(),
            )
            .unwrap();
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

    // The diff is EXACTLY the relabeling residue. Theirs opens with the
    // declare and mine with an insert, so the two sequences part at the
    // first minting edit and share no id: every node of theirs Removed
    // in its order, every node of mine Added in its, and the one
    // variable, declared at a different point of each chain, two ids.
    assert_ne!(
        theirs.doc.ids()[0],
        mine.doc.ids()[0],
        "two first edits, two first ids"
    );
    let d = theirs.doc.diff(&mine.doc);
    let expected: Vec<NodeChange> = theirs
        .doc
        .ids()
        .iter()
        .copied()
        .map(NodeChange::Removed)
        .chain(mine.doc.ids().iter().copied().map(NodeChange::Added))
        .collect();
    assert_eq!(d.nodes, expected, "diff is exactly the relabeling residue");
    let depth = |a: &Authored| a.doc.var_named("pip_depth").expect("declared");
    // `self`'s as `self` declared them, then `other`'s added ones: the
    // two chains share no id, so every variable — the one named one
    // and each typed value's own — is one of each side's.
    let expected: Vec<editor_core::VarId> = theirs
        .doc
        .var_ids()
        .into_iter()
        .chain(mine.doc.var_ids())
        .collect();
    assert_eq!(d.vars, expected, "every variable under two minted ids");
    assert!(
        d.vars.contains(&depth(&theirs)) && d.vars.contains(&depth(&mine)),
        "the one named variable under two minted ids"
    );
    assert_eq!(
        theirs.doc.var_scope(),
        mine.doc.var_scope(),
        "and one name at one kind"
    );
    assert!(!d.epsilon_changed && !d.metadata_changed);
}

//! **DOCM-7 review lane R2 — executed probes.**
//!
//! Independent falsification of the unit's claims: geometry the
//! implementer did not choose (C1), the accumulation-entity route on a
//! four-member document (C2/lane ii), the operand asymmetry's origin
//! (C6), and the corners the suite leaves unread.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    BooleanOp, BooleanValue, CancelToken, CapEnd, EntityKind, EvalOptions, Evaluation,
    NameTable, Node, NodeErrorKind, NodeResult, ProfileDoc, RecipeNodeId, RoleSeg, StableName,
    ValuePayload, evaluate,
};
use fixture::{ang, fname, insert, len, on_frame, scl, step, wall};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn table(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NameTable {
    &ev.value(id).expect("the node evaluated").name_table
}

fn failure(ev: &Evaluation<f64>, id: RecipeNodeId) -> Option<&NodeErrorKind> {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(e)) => Some(&e.kind),
        _ => None,
    }
}

fn body_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> topo::Body<f64> {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Body(b) => (**b).clone(),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => (**body).clone(),
        other => panic!("expected a body, got {other:?}"),
    }
}

/// A block spanning `(x0,x1) x (y0,y1) x (z0, z0+dz)`.
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
        },
    )
}

/// A rigid placement of `input` translated by `t`.
fn moved(doc: ProfileDoc, input: RecipeNodeId, t: [f64; 3]) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Transform {
            input,
            translation: [len(t[0]), len(t[1]), len(t[2])],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    )
}

fn member_face(union: RecipeNodeId, member: RecipeNodeId, of: StableName) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: union,
        path: vec![RoleSeg::FromMember {
            member,
            of: Box::new(of),
        }],
    }
}

fn merged(constituents: Vec<StableName>, union: RecipeNodeId) -> StableName {
    let mut set = constituents;
    set.sort();
    set.dedup();
    StableName {
        kind: EntityKind::Face,
        node: union,
        path: vec![RoleSeg::Merged(set)],
    }
}

/// The unit's own two-pass authoring shape, re-implemented here so the
/// probes do not inherit the suite's helper: a first union whose space
/// the `Declare` is written in, the real union, a `Rebind` per name,
/// and the first deleted.
fn declared_union(
    doc: ProfileDoc,
    members: &[RecipeNodeId],
    pairs: impl Fn(RecipeNodeId) -> Vec<(StableName, StableName)>,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, first) = insert(
        doc,
        Node::Union {
            members: members.to_vec(),
            declare: None,
        },
    );
    let (doc, decl) = insert(doc, Node::declare_rest(pairs(first)));
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: members.to_vec(),
            declare: Some(decl),
        },
    );
    let mut doc = doc;
    for ((fa, fb), (ta, tb)) in pairs(first).into_iter().zip(pairs(union)) {
        for (from, to) in [(fa, ta), (fb, tb)] {
            doc = step(doc, editor_core::DocEdit::Rebind { from, to }).0;
        }
    }
    let (doc, _) = step(doc, editor_core::DocEdit::DeleteNode { id: first });
    (doc, union)
}

// ---------------------------------------------------------------------
// C1 — geometry the implementer did not choose.
// ---------------------------------------------------------------------

/// **A prototype the unit did not pick, stacked along z.** Two
/// placements of one block overlapping in Z: the four SIDE walls are
/// the coplanar pairs (not the y-walls and caps the suite declares),
/// and the union fuses when they are declared and refuses when they
/// are not.
#[test]
fn r2_two_placements_stacked_in_z_fuse_when_declared() {
    let doc = ProfileDoc::empty_derived("r2_stack", Tol::witness());
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, m1) = moved(doc, proto, [0.0, 0.0, 0.0]);
    let (doc, m2) = moved(doc, proto, [0.0, 0.0, 0.5]);
    let (bare, plain) = insert(
        doc.clone(),
        Node::Union {
            members: vec![m1, m2],
            declare: None,
        },
    );
    let ev = run(&bare);
    assert!(
        matches!(
            failure(&ev, plain),
            Some(NodeErrorKind::UndeclaredContact { .. })
        ),
        "undeclared, the z-stack must refuse: {:?}",
        failure(&ev, plain)
    );
    let walls = |u: RecipeNodeId| {
        (0..4u32)
            .map(|s| {
                (
                    member_face(u, m1, fname(proto, wall(s))),
                    member_face(u, m2, fname(proto, wall(s))),
                )
            })
            .collect::<Vec<_>>()
    };
    let (doc, union) = declared_union(doc, &[m1, m2], walls);
    let ev = run(&doc);
    assert!(
        failure(&ev, union).is_none(),
        "the declared z-stack refused: {:?}",
        failure(&ev, union)
    );
    let volume = topo::mass_properties(&body_of(&ev, union), Tol::witness())
        .expect("mass")
        .volume;
    assert_eq!(volume, 1.5, "the fused volume of two 0.5-overlapping unit blocks");
}

// ---------------------------------------------------------------------
// C2 / lane (ii) — the accumulation-entity route, positively, on four
// members.
// ---------------------------------------------------------------------

/// **A `Merged` row minted at step k is routed to a step that holds
/// it**, on a four-member document — the case the suite exercises only
/// through its refusal.
///
/// Four blocks in a row along x, each overlapping the next. Step 1
/// declares member-member pairs; steps 2 and 3 declare the ACCUMULATED
/// merged face against the joining member's, and the merged rows nest
/// as the fold deepens.
#[test]
fn r2_an_accumulated_merged_row_routes_to_a_later_step() {
    let doc = ProfileDoc::empty_derived("r2_four", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (1.0, 2.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, d) = block(doc, (1.5, 2.5), (0.0, 1.0), 0.0, 1.0);
    let segs = || {
        [
            wall(0),
            wall(2),
            RoleSeg::Cap(CapEnd::Start),
            RoleSeg::Cap(CapEnd::End),
        ]
    };
    let pairs = move |u: RecipeNodeId| {
        let mut out = Vec::new();
        for seg in segs() {
            let fa = member_face(u, a, fname(a, seg.clone()));
            let fb = member_face(u, b, fname(b, seg.clone()));
            let fc = member_face(u, c, fname(c, seg.clone()));
            let fd = member_face(u, d, fname(d, seg.clone()));
            // Step 1: two members.
            out.push((fa.clone(), fb.clone()));
            // Step 2: the accumulation's merged face, against c.
            let ab = merged(vec![fa, fb], u);
            out.push((ab.clone(), fc.clone()));
            // Step 3: the nested merged face, against d.
            let abc = merged(vec![ab, fc], u);
            out.push((abc, fd));
        }
        out
    };
    let (doc, union) = declared_union(doc, &[a, b, c, d], pairs);
    let ev = run(&doc);
    assert!(
        failure(&ev, union).is_none(),
        "the accumulated-row declarations refused: {:?}",
        failure(&ev, union)
    );
    let volume = topo::mass_properties(&body_of(&ev, union), Tol::witness())
        .expect("mass")
        .volume;
    assert_eq!(volume, 2.5, "x 0..2.5 by 1 by 1");
    // Four fully-nested merged rows survive to the published table.
    let t = table(&ev, union);
    let count = t
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::Merged(_))))
        .count();
    assert_eq!(count, 4, "one merged face per declared plane");
}

// ---------------------------------------------------------------------
// C6 — where the operand asymmetry lives.
// ---------------------------------------------------------------------

/// **The PAIR boolean is itself asymmetric in its operands** when a
/// contact is declared: swapping `a` and `b` on a `Node::Boolean`
/// changes the result body's surfaces and curves, with nothing of the
/// union in the picture. Deviation 2 attributes the union's
/// order-sensitivity to the pair verb; this is that attribution,
/// measured on the pair verb alone.
#[test]
fn r2_the_pair_boolean_is_asymmetric_in_its_operands() {
    let build = |swap: bool| {
        let doc = ProfileDoc::empty_derived("r2_pair_swap", Tol::witness());
        let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
        let (doc, decl) = fixture::declare_x_offset_flush(doc, a, b);
        let (x, y) = if swap { (b, a) } else { (a, b) };
        let (doc, pair) = insert(
            doc,
            Node::Boolean {
                op: BooleanOp::Union,
                a: x,
                b: y,
                declare: Some(decl),
            },
        );
        (doc, pair)
    };
    let sorted = |mut v: Vec<String>| {
        v.sort();
        v
    };
    let describe = |doc: &ProfileDoc, id: RecipeNodeId| {
        let ev = run(doc);
        assert!(failure(&ev, id).is_none(), "{:?}", failure(&ev, id));
        let body = body_of(&ev, id);
        (
            sorted(body.surfaces().map(|(_, s)| format!("{s:?}")).collect()),
            sorted(body.curves().map(|(_, c)| format!("{c:?}")).collect()),
        )
    };
    let (d1, p1) = build(false);
    let (d2, p2) = build(true);
    let straight = describe(&d1, p1);
    let swapped = describe(&d2, p2);
    // The claim under test: the asymmetry is the PAIR verb's, so the
    // two operand orders of ONE `Node::Boolean` already differ.
    assert_ne!(
        straight, swapped,
        "the pair boolean is symmetric in its operands, so the union's \
         order-sensitivity is not inherited from it"
    );
}

// ---------------------------------------------------------------------
// The corners the suite does not read.
// ---------------------------------------------------------------------

/// **A declared name that IS a published row of this union refuses as
/// a VANISHED name.** The accumulated body's own `OutputBody` row is
/// in the union's published space (`collapse` carries it), but
/// `decl_site` cannot classify a name that mentions no member and
/// routes it to the N5 vanished refusal rather than to the pair
/// vocabulary's typed "not a face" answer.
#[test]
fn r2_the_output_body_row_refuses_as_vanished_not_as_an_unsupported_pair() {
    let doc = ProfileDoc::empty_derived("r2_outputbody", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, union) = declared_union(doc, &[a, b], |u| {
        vec![(
            StableName {
                kind: EntityKind::Body,
                node: u,
                path: vec![RoleSeg::OutputBody],
            },
            member_face(u, b, fname(b, wall(0))),
        )]
    });
    let ev = run(&doc);
    let got = failure(&ev, union);
    assert!(
        matches!(got, Some(NodeErrorKind::DeclareResolve { .. })),
        "the OutputBody row refuses, but as what? {got:?}"
    );
    // And the row it named IS published by this node: the undeclared
    // union of the same two members carries it, so the name did not
    // vanish — the classifier simply has no case for it.
    let (bare, plain) = insert(
        {
            let d = ProfileDoc::empty_derived("r2_outputbody_bare", Tol::witness());
            let (d, a2) = block(d, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
            let (d, b2) = block(d, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
            (d, a2, b2)
        }
        .0,
        Node::Union {
            members: vec![RecipeNodeId(2), RecipeNodeId(5)],
            declare: None,
        },
    );
    let ev2 = run(&bare);
    let published = table(&ev2, plain)
        .iter()
        .any(|(n, _)| n.path.as_slice() == [RoleSeg::OutputBody]);
    assert!(
        published,
        "the union publishes an OutputBody row, so the refusal above is not a vanished name"
    );
}

/// **Two accumulation rows paired together IS reachable** — deviation
/// 8 says no document was found that reaches it. Two merged rows of
/// the SAME step, declared as a pair, route to a bucket and land in
/// `resolve_declarations` as a same-operand FACE pair, which the v1
/// vocabulary refuses.
#[test]
fn r2_two_fold_rows_paired_together_is_reachable() {
    let doc = ProfileDoc::empty_derived("r2_two_rows", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (4.0, 5.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, union) = declared_union(doc, &[a, b, c], |u| {
        let w0 = merged(
            vec![
                member_face(u, a, fname(a, wall(0))),
                member_face(u, b, fname(b, wall(0))),
            ],
            u,
        );
        let w2 = merged(
            vec![
                member_face(u, a, fname(a, wall(2))),
                member_face(u, b, fname(b, wall(2))),
            ],
            u,
        );
        let mut out: Vec<(StableName, StableName)> = [
            wall(0),
            wall(2),
            RoleSeg::Cap(CapEnd::Start),
            RoleSeg::Cap(CapEnd::End),
        ]
        .into_iter()
        .map(|seg| {
            (
                member_face(u, a, fname(a, seg.clone())),
                member_face(u, b, fname(b, seg)),
            )
        })
        .collect();
        // The pair deviation 8 says no document reaches: two rows the
        // fold minted at step 1, declared against each other.
        out.push((w0, w2));
        out
    });
    let ev = run(&doc);
    let got = failure(&ev, union);
    assert!(
        matches!(got, Some(NodeErrorKind::DeclareUnsupportedPair { .. })),
        "two fold rows paired together reached: {got:?}"
    );
}

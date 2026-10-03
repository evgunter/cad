//! **EDIT-PLACEMENT P2 — checked offsets, the doors and the spaces**
//! (ASSEMBLY.md A4, A9, A11 (2)–(5)): every checked offset checked or
//! faulted, the mate and compound doors, split and inline at a gauge,
//! and the readers that must not compare an unplaced group with
//! anything outside it. Each row is composed against the two literal
//! block parts `p2_gauges` seats, and the poses against an independent
//! 4x4 composition written here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::ExtrudeSide;
use std::sync::Arc;

use crate::fixture;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, Dimension, DocEdit, DocParam, DocParamValue,
    DocRef, DocumentId, EvalOptions, Evaluation, Expr, Frame, MateFault, MateFrame, MatePrimitive,
    Node, NodeErrorKind, ParamName, PatternKind, Placement, ProfileDoc, RecipeNodeId, SitedFace,
    SlotId, StableName, Step, ValuePayload, evaluate, root_of,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{
    ang, head, head_at, in_copy, insert, len, offset_of, on_frame, run, scl, solve, step, xform,
};
use geom_core::{Bounds, Interval, Tol};
use topo::Body;

const BASE_HEIGHT: f64 = 1.0;
const TOP_HEIGHT: f64 = 3.0;

fn block(label: &str, w: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, w), (0.0, w)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(h),
            side: ExtrudeSide::Along,
        },
    )
}

struct Parts {
    store: PartStore,
    base: DocRef,
    base_body: RecipeNodeId,
    base_doc: ProfileDoc,
    top: DocRef,
    top_body: RecipeNodeId,
    top_doc: ProfileDoc,
}

fn parts(label: &str) -> Parts {
    let mut store = PartStore::new();
    let bd = block(&format!("{label}-base"), 3.0, BASE_HEIGHT);
    let td = block(&format!("{label}-top"), 1.0, TOP_HEIGHT);
    let (base, base_body) = store.insert_part(bd.clone(), Tol::witness());
    let (top, top_body) = store.insert_part(td.clone(), Tol::witness());
    Parts {
        store,
        base,
        base_body,
        base_doc: bd.0,
        top,
        top_body,
        top_doc: td.0,
    }
}

impl Parts {
    fn opts(&self) -> EvalOptions {
        with_resolver(self.store.clone())
    }
    fn base_cap(&self, base: RecipeNodeId) -> StableName {
        in_part(base, self.base_body, CapEnd::End)
    }
    fn top_cap(&self, top: RecipeNodeId) -> StableName {
        in_part(top, self.top_body, CapEnd::Start)
    }
    fn top_upper_cap(&self, top: RecipeNodeId) -> StableName {
        in_part(top, self.top_body, CapEnd::End)
    }
}

fn mframe(origin: [f64; 3], axis: [f64; 3]) -> MateFrame {
    MateFrame::authored(origin, axis, [1.0, 0.0, 0.0], geom_core::Tol::witness())
        .expect("a definite frame")
}

fn seat_on(mover: SitedFace, onto: SitedFace, at: [f64; 3]) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: mover,
        b: onto,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: mframe([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
            b: mframe(at, [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

fn seat(top: SitedFace, base: SitedFace) -> Node<editor_core::ProfileProgram> {
    seat_on(top, base, [1.0, 1.0, BASE_HEIGHT])
}

// ---- an independent 4x4 composition ----

type M4 = [[f64; 4]; 4];

fn mul(a: &M4, b: &M4) -> M4 {
    let mut out = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                out[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    out
}

fn ident() -> M4 {
    let mut m = [[0.0; 4]; 4];
    for (i, row) in m.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    m
}

/// x -> R(axis, angle) x + t (Rodrigues, written here).
fn rigid(t: [f64; 3], axis: [f64; 3], angle: f64) -> M4 {
    let n = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
    let (x, y, z) = (axis[0] / n, axis[1] / n, axis[2] / n);
    let (c, s) = (angle.cos(), angle.sin());
    let v = 1.0 - c;
    [
        [c + x * x * v, x * y * v - z * s, x * z * v + y * s, t[0]],
        [y * x * v + z * s, c + y * y * v, y * z * v - x * s, t[1]],
        [z * x * v - y * s, z * y * v + x * s, c + z * z * v, t[2]],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn tr(t: [f64; 3]) -> M4 {
    rigid(t, [0.0, 0.0, 1.0], 0.0)
}

fn of_frame(f: &Frame) -> M4 {
    let c = f.columns;
    [
        [c[0][0], c[1][0], c[2][0], f.translation[0]],
        [c[0][1], c[1][1], c[2][1], f.translation[1]],
        [c[0][2], c[1][2], c[2][2], f.translation[2]],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn to_frame(m: &M4) -> Frame {
    Frame {
        columns: [
            [m[0][0], m[1][0], m[2][0]],
            [m[0][1], m[1][1], m[2][1]],
            [m[0][2], m[1][2], m[2][2]],
        ],
        translation: [m[0][3], m[1][3], m[2][3]],
    }
}

fn inv(m: &M4) -> M4 {
    // rigid inverse: R^T, -R^T t
    let mut out = ident();
    for i in 0..3 {
        for j in 0..3 {
            out[i][j] = m[j][i];
        }
    }
    for i in 0..3 {
        out[i][3] = -(0..3).map(|k| m[k][i] * m[k][3]).sum::<f64>();
    }
    out
}

fn apply(m: &M4, p: [f64; 3]) -> [f64; 3] {
    let mut out = [0.0; 3];
    for (i, o) in out.iter_mut().enumerate() {
        *o = m[i][0] * p[0] + m[i][1] * p[1] + m[i][2] * p[2] + m[i][3];
    }
    out
}

fn gap(a: &M4, b: &M4) -> f64 {
    let mut g: f64 = 0.0;
    for i in 0..3 {
        for j in 0..4 {
            g = g.max((a[i][j] - b[i][j]).abs());
        }
    }
    g
}

fn body_of<T: geom_core::Decide>(ev: &Evaluation<T>, id: RecipeNodeId) -> Arc<Body<T>> {
    match &ev
        .value(id)
        .unwrap_or_else(|| panic!("{id:?} evaluated: {:?}", ev.node_error(id)))
        .payload
    {
        ValuePayload::Body(b) => Arc::clone(b),
        other => panic!("{id:?} is a {}, not a body", other.kind_name()),
    }
}

fn points(body: &Body<f64>) -> Vec<[f64; 3]> {
    body.points().map(|(_, p)| [p.x, p.y, p.z]).collect()
}

/// Every point of `placed` is `m` applied to the matching point of
/// `part` (arena order), to 1e-9; and in the interval lane `lane`
/// encloses the expected point.
fn assert_points(part: &Body<f64>, placed: &Body<f64>, m: &M4, what: &str) {
    let (pp, qq) = (points(part), points(placed));
    assert_eq!(pp.len(), qq.len(), "{what}");
    for (p, q) in pp.iter().zip(&qq) {
        let e = apply(m, *p);
        for k in 0..3 {
            assert!(
                (e[k] - q[k]).abs() <= 1e-9,
                "{what}: expected {e:?}, evaluated {q:?}"
            );
        }
    }
}

fn assert_encloses(part: &Body<f64>, lane: &Body<Interval>, m: &M4, what: &str) {
    let pp = points(part);
    let ll: Vec<_> = lane.points().map(|(_, p)| [p.x, p.y, p.z]).collect();
    assert_eq!(pp.len(), ll.len(), "{what}");
    for (p, i) in pp.iter().zip(&ll) {
        let e = apply(m, *p);
        for k in 0..3 {
            assert!(
                i[k].lo() - 1e-12 <= e[k]
                    && e[k] <= i[k].hi() + 1e-12
                    && i[k].hi() - i[k].lo() <= 1e-9,
                "{what}: interval [{}, {}] vs independent {}",
                i[k].lo(),
                i[k].hi(),
                e[k]
            );
        }
    }
}

fn lift() -> ParamName {
    ParamName::from_static("lift")
}

fn declare_lift(doc: ProfileDoc, value: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetDocParam {
            name: lift(),
            value: DocParam::continuous(Dimension::Length, value),
        },
    )
    .0
}

fn set_lift(doc: ProfileDoc, value: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetDocParamValue {
            name: lift(),
            value: DocParamValue::Continuous(value),
        },
    )
    .0
}

fn set_gauge(doc: ProfileDoc, node: RecipeNodeId, gauge: Option<RecipeNodeId>) -> ProfileDoc {
    step(doc, DocEdit::SetGauge { node, gauge }).0
}

fn set_offset(doc: ProfileDoc, instance: RecipeNodeId, offset: Option<Placement>) -> ProfileDoc {
    step(doc, DocEdit::SetOffset { instance, offset }).0
}

/// The seat relation, read off a control solve (base at the world
/// origin, the top mated onto it): the one input this probe does not
/// compose itself.
fn seat_rel(p: &Parts) -> M4 {
    let doc = ProfileDoc::empty(DocumentId::derive("r1-seat-control"), Tol::witness());
    let (doc, b) = insert(doc, Node::instantiate_part(p.base));
    let (doc, t) = insert(doc, Node::instantiate_part(p.top));
    let (doc, _) = insert(doc, seat(head(p.top_cap(t)), head(p.base_cap(b))));
    let poses = solve(&doc, &p.opts(), Tol::witness());
    of_frame(&poses.placement(&doc, t).unwrap())
}

fn band() -> geom_core::Band {
    fixture::band()
}

/// **A ∘ F ∘ B with a placer on BOTH sides of a tree mate** — a
/// Transform over the parent, a Pattern copy over the child — under a
/// gauge under a parametric gauge, with a rotated root offset, against an
/// independent 4x4 composition in f64 and in the interval lane at two
/// values of the driving parameter. Then a checked offset on the
/// non-root member set to the composed truth holds and one 0.01 off
/// faults: the conjugation of the placers by the group's frame is what
/// makes the truth true.
#[test]
fn a_placer_on_each_side_under_nested_parametric_gauges_poses_as_composed_and_checks_its_offset() {
    let p = parts("r1-afb");
    let o = p.opts();
    let rel = seat_rel(&p);

    let doc = ProfileDoc::empty(DocumentId::derive("r1-afb"), Tol::witness());
    let doc = declare_lift(doc, 2.0);
    let (doc, gout) = insert(
        doc,
        Node::gauge(
            None,
            Step::Rigid {
                translation: [len(0.0), len(0.0), Expr::param(lift(), Dimension::Length)],
                axis: [0.0, 0.0, 1.0].map(scl),
                angle: ang(0.25),
            },
        ),
    );
    let gin_frame =
        Frame::rotate_then_translate([1.0, 0.0, 0.0], 0.4, [5.0, 0.0, 0.0], band()).unwrap();
    let (doc, gin) = insert(doc, Node::gauge(Some(gout), Placement::literal(&gin_frame)));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, base, Some(gin));
    let doc = set_gauge(doc, top, Some(gin));
    let off_frame =
        Frame::rotate_then_translate([0.0, 0.0, 1.0], 0.3, [1.0, 2.0, 0.0], band()).unwrap();
    let doc = set_offset(doc, base, Some(Placement::literal(&off_frame)));
    // Parent-side placer: a transform over the base.
    let (doc, tp) = insert(doc, xform(base, [0.0, 0.0, 4.0], [1.0, 0.0, 0.0], 0.5));
    // Child-side placer: a linear pattern over the top, copy 2.
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: top,
            count: Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(0.0), scl(1.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, mate) = insert(
        doc,
        seat(
            head_at(pat, in_copy(pat, 2, p.top_cap(top))),
            head_at(tp, p.base_cap(base)),
        ),
    );
    assert_eq!(
        offset_of(&doc, top),
        None,
        "the mate door cleared the mover"
    );

    let part_base = run(&p.base_doc, &EvalOptions::default());
    let part_top = run(&p.top_doc, &EvalOptions::default());
    let (pb, pt) = (
        body_of(&part_base, p.base_body),
        body_of(&part_top, p.top_body),
    );

    let expect = |liftv: f64| -> (M4, M4, M4) {
        let g = mul(
            &rigid([0.0, 0.0, liftv], [0.0, 0.0, 1.0], 0.25),
            &of_frame(&gin_frame),
        );
        let w_base = mul(&g, &of_frame(&off_frame));
        let t_p = rigid([0.0, 0.0, 4.0], [1.0, 0.0, 0.0], 0.5);
        let p2 = tr([0.0, 4.0, 0.0]);
        let w_top = mul(&mul(&mul(&inv(&p2), &t_p), &w_base), &rel);
        (g, w_base, w_top)
    };

    for liftv in [2.0, 7.0] {
        let doc = set_lift(doc.clone(), liftv);
        let (_, w_base, w_top) = expect(liftv);
        let poses = solve(&doc, &o, Tol::witness());
        assert_eq!(poses.fault(mate), None);
        let pose_top = of_frame(&poses.placement(&doc, top).unwrap());
        let pose_base = of_frame(&poses.placement(&doc, base).unwrap());
        assert!(
            gap(&pose_base, &w_base) <= 1e-12,
            "base pose {liftv}: {}",
            gap(&pose_base, &w_base)
        );
        assert!(
            gap(&pose_top, &w_top) <= 1e-12,
            "top pose {liftv}: {}",
            gap(&pose_top, &w_top)
        );
        let ev = run(&doc, &o);
        assert_points(&pb, &body_of(&ev, base), &w_base, "base f64");
        assert_points(&pt, &body_of(&ev, top), &w_top, "top f64");
        let lane = evaluate::<Interval>(
            &doc,
            None,
            &editor_core::CancelToken::new(),
            &o,
            Tol::witness(),
        );
        assert_encloses(&pb, &body_of(&lane, base), &w_base, "base interval");
        assert_encloses(&pt, &body_of(&lane, top), &w_top, "top interval");
    }

    // A checked offset on the non-root member: the independent truth,
    // expressed in its gauge, holds; 0.01 off it faults.
    let (g, _, w_top) = expect(2.0);
    let truth = mul(&inv(&g), &w_top);
    let holds = set_offset(
        doc.clone(),
        top,
        Some(Placement::literal(&to_frame(&truth))),
    );
    let poses = solve(&holds, &o, Tol::witness());
    assert_eq!(root_of(&holds, top), base);
    assert_eq!(poses.fault(top), None, "a true statement holds");
    let off = mul(&tr([0.01, 0.0, 0.0]), &truth);
    let wrong = set_offset(doc, top, Some(Placement::literal(&to_frame(&off))));
    let poses = solve(&wrong, &o, Tol::witness());
    assert!(
        matches!(poses.fault(top), Some(MateFault::OffsetDisagrees { .. })),
        "{:?}",
        poses.fault(top)
    );
}

/// **A welded member the spanning tree cannot reach faults its checked
/// offset typed, naming the mate that strands it.** The top is mated
/// through copy 2 of a pattern; the pattern then shrinks to two
/// copies, so the mate's head dangles and the solve refuses it. The
/// top stays welded to the base's group (the partition is structural)
/// and the tree cannot reach it, so its offset — stating it at x = 20
/// — can be neither honoured nor checked: it is
/// `OffsetUnchecked { Unreached }`.
#[test]
fn a_member_the_tree_cannot_reach_faults_its_offset_naming_the_stranded_mate() {
    let p = parts("r1-strand");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-strand"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: top,
            count: Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(5.0),
            },
        },
    );
    let (doc, mate) = insert(
        doc,
        seat(
            head_at(pat, in_copy(pat, 2, p.top_cap(top))),
            head(p.base_cap(base)),
        ),
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetStructuralParam {
            node: pat,
            slot: SlotId::Count,
            expr: Expr::count(2),
        },
    );
    let unstated = solve(&doc, &o, Tol::witness());
    assert!(
        matches!(unstated.fault(mate), Some(MateFault::DanglingHead { .. })),
        "the stranded mate refuses: {:?}",
        unstated.fault(mate)
    );
    assert_eq!(
        unstated.fault(top),
        None,
        "an unreached member with no offset states nothing to check"
    );
    let doc = set_offset(
        doc,
        top,
        Some(Placement::literal(&Frame::translation([20.0, 0.0, 0.0]))),
    );
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.root(top), Some(base), "the top stays welded");
    match poses.fault(top) {
        Some(MateFault::OffsetUnchecked { instance, cause }) => {
            assert_eq!(*instance, top);
            assert!(
                matches!(**cause, editor_core::OffsetCheck::Unreached { mate: m } if m == mate),
                "{cause:?}"
            );
        }
        other => panic!("the unreached member's offset faults typed: {other:?}"),
    }
    let said = poses
        .fault(top)
        .map(ToString::to_string)
        .unwrap_or_default();
    assert!(
        said.contains("Recourse: repair mate") && said.matches("Recourse:").count() == 1,
        "{said}"
    );
}

/// **A dead-gauge group checks its offsets as usual, in its own space**
/// (A11 (2)). Base and top on gauge g, the top mated onto the base and
/// then given an offset that disagrees: it faults. Delete g: the group is
/// unplaced, and the disagreeing statement still faults.
#[test]
fn a_dead_gauge_group_checks_its_offsets_in_its_own_space() {
    let p = parts("r1-deadcheck");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-deadcheck"), Tol::witness());
    let (doc, g) = insert(
        doc,
        Node::gauge(
            None,
            Placement::literal(&Frame::translation([0.0, 5.0, 0.0])),
        ),
    );
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, base, Some(g));
    let doc = set_gauge(doc, top, Some(g));
    let (doc, _) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let doc = set_offset(
        doc,
        top,
        Some(Placement::literal(&Frame::translation([0.0, 0.0, 50.0]))),
    );
    let poses = solve(&doc, &o, Tol::witness());
    assert!(
        matches!(poses.fault(top), Some(MateFault::OffsetDisagrees { .. })),
        "placed: {:?}",
        poses.fault(top)
    );
    let (dead, _) = step(doc, DocEdit::DeleteNode { id: g });
    let poses = solve(&dead, &o, Tol::witness());
    assert!(
        poses.fault(top).is_some(),
        "the statement disagreed inside the group and still does; it is no longer checked"
    );
}

/// **The mate door clears every offset in the moving group** (ruling C).
/// p (top), q (top) stacked on p with q's offset stated where it sits;
/// r (base) inserted last at x = 30. "Mate p onto r" moves p's group: the
/// door clears p's offset AND q's, so r roots the merged group and stays
/// where it was.
#[test]
fn the_mate_door_clears_a_checked_offset_in_the_moving_group_too() {
    let p = parts("r1-door-order");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-door-order"), Tol::witness());
    let (doc, pp) = insert(doc, Node::instantiate_part(p.top));
    let (doc, q) = insert(doc, Node::instantiate_part(p.top));
    let (doc, _) = insert(
        doc,
        seat_on(
            head(p.top_cap(q)),
            head(p.top_upper_cap(pp)),
            [0.0, 0.0, TOP_HEIGHT],
        ),
    );
    let doc = fixture::offsets_where_solved(doc, &o);
    assert!(offset_of(&doc, q).is_some());
    assert_eq!(solve(&doc, &o, Tol::witness()).fault(q), None);
    let (doc, r) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(
        doc,
        r,
        Some(Placement::literal(&Frame::translation([30.0, 0.0, 0.0]))),
    );
    let doc_before_mate = doc.clone();
    let applied = editor_core::apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(seat(head(p.top_cap(pp)), head(p.base_cap(r)))),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the mate inserts");
    let cleared: Vec<RecipeNodeId> = applied
        .maintenance
        .iter()
        .filter_map(|row| match row {
            editor_core::Maintenance::OffsetCleared { instance, .. } => Some(instance.id()),
            _ => None,
        })
        .collect();
    assert_eq!(cleared, [pp, q], "one row per offset the moving group held");
    let doc = applied.doc;
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(
        root_of(&doc, r),
        r,
        "ruling C: r's group stays put and p's group moves onto it"
    );
    assert_eq!(
        offset_of(&doc, q),
        None,
        "the checked member's offset is cleared"
    );
    for id in [pp, q, r] {
        assert_eq!(poses.fault(id), None, "node {} stands", id.0);
    }
    assert_eq!(
        poses.placement(&doc, r).expect("r placed").translation,
        [30.0, 0.0, 0.0],
        "r does not move"
    );
    // Replay re-applies the edits alone and reaches the same document.
    let replayed = editor_core::apply_replayed(
        &doc_before_mate,
        &DocEdit::InsertNode {
            node: Box::new(seat(head(p.top_cap(pp)), head(p.base_cap(r)))),
        },
        Tol::witness(),
    )
    .expect("replays");
    assert!(replayed.doc.bit_eq(&doc), "replay clears the same offsets");
}

/// **A declaring mate across gauges is verified at the at-rest gate,
/// never placed** (A11 (2)): the top stands on its own gauge at the
/// seat, lifted by `d` off the base. At d = 0 the gate certifies; at a d
/// inside the band, and at d = 0.5, it does not.
#[test]
fn a_declaring_mate_across_gauges_certifies_only_at_contact() {
    let p = parts("r1-ambiguous");
    let o = p.opts();
    let rel = seat_rel(&p);
    let eps = Tol::witness().eps();
    for d in [0.0, 2.0 * eps, 0.5] {
        let doc = ProfileDoc::empty(DocumentId::derive("r1-ambiguous"), Tol::witness());
        let (doc, g) = insert(
            doc,
            Node::gauge(None, Placement::literal(&Frame::translation([0.0, 0.0, d]))),
        );
        let (doc, base) = insert(doc, Node::instantiate_part(p.base));
        let (doc, top) = insert(doc, Node::instantiate_part(p.top));
        let doc = set_gauge(doc, top, Some(g));
        let doc = set_offset(doc, top, Some(Placement::literal(&to_frame(&rel))));
        let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
        assert_eq!(
            solve(&doc, &o, Tol::witness()).role(mate),
            Some(editor_core::MateRole::Declaring)
        );
        let ev = run(&doc, &o);
        let verdict = editor_core::assemble(&doc, &ev, Tol::witness());
        if d == 0.0 {
            assert!(verdict.is_ok());
        } else {
            assert!(verdict.is_err(), "d = {d:e} certified");
        }
    }
}

/// **The memo does not carry a measure across a space change**: an
/// instance at the world identity and an unplaced one key their frame
/// alike, so clearing the top's offset with the prior evaluation handed
/// in must re-evaluate the measure, which then refuses `Unplaced`.
#[test]
fn a_measure_reused_across_an_offset_clear_refuses_across_spaces() {
    let p = parts("r1-measure");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-measure"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_offset(
        doc,
        top,
        Some(Placement::literal(&Frame::translation([0.0, 0.0, 1.0]))),
    );
    let measure = Node::measure(
        editor_core::MeasureExpr::primitive(editor_core::MeasurePrimitive::Distance { a: 0, b: 1 }),
        vec![
            editor_core::SitedRef::at_mint(p.base_cap(base)),
            editor_core::SitedRef::at_mint(p.top_cap(top)),
        ],
    )
    .unwrap();
    let (doc, m) = insert(doc, measure);
    let ev = run(&doc, &o);
    assert!(ev.value(m).is_some());
    // Clear the top's offset: it is unplaced; reuse the prior.
    let un = set_offset(doc, top, None);
    let ev2 = evaluate::<f64>(
        &un,
        Some(&ev),
        &editor_core::CancelToken::new(),
        &o,
        Tol::witness(),
    );
    let e = ev2.node_error(m);
    assert!(
        matches!(e.map(|e| &e.kind), Some(NodeErrorKind::Unplaced { .. })),
        "{e:?}"
    );
}

/// **Every saved document in the tree loads** (or, at a process ε
/// other than the one it records, refuses at D4's seam), and each
/// frozen older-build golden refuses typed — the instance keys being required
/// on the wire moves none of them, since none holds an instance.
#[test]
fn every_saved_document_in_the_tree_loads_or_refuses_typed() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let files = [
        "crates/editor-core/tests/corpus/die_tool.pncad",
        "crates/editor-core/tests/corpus/tour/die_composed_tour.pncad",
        "crates/editor-core/tests/golden/golden.cad",
        "crates/pncad/tests/plate_param.pncad",
        "crates/viewer/tests/gallery_ring.pncad",
    ];
    let mut all: Vec<String> = files.iter().map(|s| (*s).to_string()).collect();
    for v in 1..=19 {
        all.push(format!(
            "crates/editor-core/tests/bool13_goldens/v{v}_golden.cad"
        ));
    }
    for f in all {
        let text = std::fs::read_to_string(root.join(&f)).unwrap();
        let r = editor_core::load(&text, Tol::witness());
        if f.contains("bool13") {
            assert!(r.is_err());
            continue;
        }
        match r {
            Ok(_) => {}
            // D4's seam, before any replay: a document authored at
            // another ε refuses at this process's ε, typed — the load
            // door's answer on the other ε rows, not this row's subject.
            Err(editor_core::PersistError::ToleranceConflict { document, process }) => {
                assert_ne!(
                    document, process,
                    "{f}: the seam refuses only a real conflict"
                );
                assert_eq!(process, Tol::witness().eps(), "{f}");
            }
            Err(e) => panic!("{f}: {e}"),
        }
    }
}

/// **An instance written without its `gauge` and `offset` keys refuses
/// typed**: both are required on the wire, so a file from before
/// gauges cannot load as an unplaced instance. Either key alone missing
/// refuses too, and the keys at `null` load.
#[test]
fn an_instance_missing_its_gauge_or_offset_key_refuses_typed() {
    let p = parts("r1-wire");
    let doc = ProfileDoc::empty(DocumentId::derive("r1-wire"), Tol::witness());
    let (doc, inst) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(doc, inst, None);
    let text = editor_core::save(&doc, &[], Tol::witness()).unwrap();
    let loaded = editor_core::load(&text, Tol::witness()).expect("the keys at null load");
    assert_eq!(offset_of(&loaded.doc, inst), None);
    for missing in [&["gauge", "offset"][..], &["gauge"], &["offset"]] {
        let stripped = crate::wire::doctored(&text, |wire| {
            fn strip(v: &mut serde_json::Value, keys: &[&str]) {
                match v {
                    serde_json::Value::Object(m) => {
                        if m.contains_key("doc_ref") {
                            for k in keys {
                                m.remove(*k);
                            }
                        }
                        for (_, x) in m.iter_mut() {
                            strip(x, keys);
                        }
                    }
                    serde_json::Value::Array(a) => a.iter_mut().for_each(|x| strip(x, keys)),
                    _ => {}
                }
            }
            strip(&mut wire["snapshot"]["nodes"], missing);
        });
        match editor_core::load(&stripped, Tol::witness()) {
            Err(editor_core::PersistError::Unreadable { detail, .. }) => assert!(
                detail.contains(missing[0]),
                "the refusal names the missing key {missing:?}: {detail}"
            ),
            other => panic!("an instance without {missing:?} refuses typed: {other:?}"),
        }
    }
}

fn world_bounds(doc: &ProfileDoc, ev: &Evaluation<f64>) -> ([f64; 3], [f64; 3]) {
    let body = editor_core::product(doc, ev, Tol::witness()).expect("the product gathers");
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for (_, p) in body.points() {
        for (k, x) in [p.x, p.y, p.z].into_iter().enumerate() {
            lo[k] = lo[k].min(x);
            hi[k] = hi[k].max(x);
        }
    }
    (lo, hi)
}

/// **Every reference leaving the cut votes for an anchor** (A4): a cut
/// holding an instance on gauge g and plain world geometry lands on two
/// anchors — the instance votes g, the plain root votes the world — and
/// refuses `TwoAnchors` naming the plain root, rather than carry the
/// plain body through g. Without the gauge the same cut splits and
/// nothing moves.
#[test]
fn a_cut_of_a_gauged_instance_and_plain_geometry_lands_on_two_anchors() {
    let p = parts("r1-split-plain");
    let doc = ProfileDoc::empty(DocumentId::derive("r1-split-plain"), Tol::witness());
    let (doc, g) = insert(
        doc,
        Node::gauge(
            None,
            Placement::literal(&Frame::translation([0.0, 10.0, 0.0])),
        ),
    );
    let (world_doc, x) = insert(doc, Node::instantiate_part(p.base));
    let gauged = set_gauge(world_doc.clone(), x, Some(g));
    let plain_cut = |doc: ProfileDoc| {
        let before: std::collections::BTreeSet<RecipeNodeId> =
            doc.order().iter().copied().collect();
        let (doc, prof) = on_frame(
            doc,
            [50.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        );
        let (doc, ext) = insert(
            doc,
            Node::Extrude {
                profile: prof,
                distance: len(1.0),
                side: ExtrudeSide::Along,
            },
        );
        let mut cut: std::collections::BTreeSet<RecipeNodeId> = doc
            .order()
            .iter()
            .copied()
            .filter(|id| !before.contains(id))
            .collect();
        cut.insert(x);
        (doc, cut, ext)
    };
    let o = p.opts();
    let (doc, cut, ext) = plain_cut(gauged);
    match editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("r1-split-plain-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    ) {
        Err(editor_core::SplitError::TwoAnchors {
            node,
            first,
            second,
        }) => {
            assert_eq!(
                (node.id(), first.map(|f| f.id()), second.map(|s| s.id())),
                (ext, Some(g), None)
            );
        }
        other => panic!("a gauged instance and plain geometry are two anchors: {other:?}"),
    }
    // On the world, the same cut is one anchor, and nothing moves.
    let (doc, cut, _) = plain_cut(world_doc);
    let before = world_bounds(&doc, &run(&doc, &o));
    let out = editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("r1-split-plain-world-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("one anchor, the world, splits");
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let after = world_bounds(&out.remainder, &run(&out.remainder, &with_resolver(store)));
    for k in 0..3 {
        assert!(
            (before.0[k] - after.0[k]).abs() < 1e-9 && (before.1[k] - after.1[k]).abs() < 1e-9,
            "split moved the world: {before:?} -> {after:?}"
        );
    }
}

/// **A placer over a gauged instance votes for the world** (A4): a cut
/// holding an instance on gauge g and the transform over it lands on
/// two anchors, since the transform's map acts in the document's
/// coordinates and would act in the part's after the split. It refuses
/// `TwoAnchors` naming the transform.
#[test]
fn a_cut_of_a_gauged_instance_and_its_transform_lands_on_two_anchors() {
    let p = parts("r1-split-xform");
    let doc = ProfileDoc::empty(DocumentId::derive("r1-split-xform"), Tol::witness());
    let (doc, g) = insert(
        doc,
        Node::gauge(
            None,
            Placement::literal(&Frame::translation([0.0, 10.0, 0.0])),
        ),
    );
    let (doc, x) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, x, Some(g));
    let (doc, t) = insert(doc, xform(x, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.5));
    let o = p.opts();
    let cut: std::collections::BTreeSet<RecipeNodeId> = [x, t].into_iter().collect();
    match editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("r1-split-xform-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    ) {
        Err(editor_core::SplitError::TwoAnchors {
            node,
            first,
            second,
        }) => assert_eq!(
            (node.id(), first.map(|f| f.id()), second.map(|s| s.id())),
            (t, Some(g), None)
        ),
        other => panic!("a gauged instance under its transform is two anchors: {other:?}"),
    }
}
/// **A group nothing places for lack of an offset votes its gauge**
/// (A4; the P2-split spec's D2): a cut holding an instance on g whose
/// offset was cleared, beside plain world geometry, names two anchors,
/// rather than land the instance on the world and lose its gauge.
#[test]
fn a_cut_group_unplaced_for_lack_of_an_offset_votes_its_gauge() {
    let p = parts("r1-split-novote");
    let doc = ProfileDoc::empty(DocumentId::derive("r1-split-novote"), Tol::witness());
    let (doc, g) = insert(
        doc,
        Node::gauge(
            None,
            Placement::literal(&Frame::translation([0.0, 10.0, 0.0])),
        ),
    );
    let (doc, x) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, x, Some(g));
    let doc = set_offset(doc, x, None);
    let before: std::collections::BTreeSet<RecipeNodeId> = doc.order().iter().copied().collect();
    let (doc, prof) = on_frame(
        doc,
        [50.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile: prof,
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let o = p.opts();
    let mut cut: std::collections::BTreeSet<RecipeNodeId> = doc
        .order()
        .iter()
        .copied()
        .filter(|id| !before.contains(id))
        .collect();
    cut.insert(x);
    match editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("r1-split-novote-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    ) {
        Err(editor_core::SplitError::TwoAnchors {
            node,
            first,
            second,
        }) => assert_eq!(
            (node.id(), first.map(|f| f.id()), second.map(|s| s.id())),
            (ext, Some(g), None)
        ),
        other => panic!("the unplaced instance votes its gauge: {other:?}"),
    }
}

/// **A document of unplaced material alone**: the world gather refuses
/// `ProductError::Unplaced` naming the group, its cause and the place-it
/// recourse, and the gate still checks the group in its own space
/// before raising that refusal — so an interference there is the gate's
/// answer, not the empty world's.
#[test]
fn a_document_of_unplaced_material_alone_names_its_groups_and_still_checks_them() {
    let p = parts("r1-alone");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-alone"), Tol::witness());
    let (doc, x) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(doc, x, None);
    let ev = run(&doc, &o);
    match editor_core::product(&doc, &ev, Tol::witness()) {
        Err(editor_core::ProductError::Unplaced { groups }) => {
            assert_eq!(groups, vec![(x, editor_core::Unplaced::NoOffset)]);
        }
        other => panic!("the world gather names the unplaced group: {other:?}"),
    }
    let said = editor_core::product(&doc, &ev, Tol::witness())
        .err()
        .map(|e| e.to_string())
        .unwrap_or_default();
    assert!(
        said.contains(editor_core::UNPLACED_RECOURSE) && said.contains(&format!("node {x}")),
        "{said}"
    );
    match editor_core::assemble(&doc, &ev, Tol::witness()) {
        Err(editor_core::AssemblyError::Product(e)) => assert!(
            matches!(*e, editor_core::ProductError::Unplaced { .. }),
            "{e:?}"
        ),
        other => panic!("a clean own space raises the world's refusal: {other:?}"),
    }
    // Two tops seated at one spot on the unplaced base: they overlap
    // inside the one group's own space.
    let (doc, a) = insert(doc, Node::instantiate_part(p.top));
    let (doc, b) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_offset(set_offset(doc, a, None), b, None);
    let (doc, _) = insert(doc, seat(head(p.top_cap(a)), head(p.base_cap(x))));
    let (doc, _) = insert(doc, seat(head(p.top_cap(b)), head(p.base_cap(x))));
    let ev = run(&doc, &o);
    assert!(ev.unplaced.contains_key(&a) && ev.unplaced.contains_key(&b));
    let own_space_refuses = |doc: &ProfileDoc, ev: &Evaluation<f64>| match editor_core::assemble(
        doc,
        ev,
        Tol::witness(),
    ) {
        Err(editor_core::AssemblyError::Space { group, .. }) => assert_eq!(group, x),
        Err(editor_core::AssemblyError::AtRest { .. }) => {}
        other => panic!("the own space's interference is the gate's answer: {other:?}"),
    };
    own_space_refuses(&doc, &ev);
    // With placed material far off in the world, the world gathers and
    // certifies, and the gate over that product still checks the own
    // space: the product carries it (`Product::spaces`).
    let (doc, far) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(
        doc,
        far,
        Some(Placement::literal(&Frame::translation([100.0, 0.0, 0.0]))),
    );
    let ev = run(&doc, &o);
    let product =
        editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("the world gathers");
    assert_eq!(product.spaces.len(), 1, "the product carries the own space");
    assert!(editor_core::assemble_gathered(product, Tol::witness()).is_err());
    own_space_refuses(&doc, &ev);
}

/// **A parametric root offset moves with the cut, and `Promote` keeps
/// it in the host** (A4): the root's offset is a cut node's, so a
/// parameter a kept gauge reads too refuses `UncutParamReference`;
/// promoted, the offset is a kept gauge's placement, the group is cut
/// leaving that gauge behind, and the part gets no copy of a parameter
/// nothing in it reads.
#[test]
fn a_parametric_root_offset_moves_with_the_cut_and_promote_keeps_it_in_the_host() {
    let p = parts("r1-promote-param");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-promote-param"), Tol::witness());
    let doc = declare_lift(doc, 2.0);
    let (doc, _) = insert(
        doc,
        Node::gauge(
            None,
            Step::Rigid {
                translation: [len(0.0), Expr::param(lift(), Dimension::Length), len(0.0)],
                axis: [0.0, 0.0, 1.0].map(scl),
                angle: ang(0.0),
            },
        ),
    );
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let offset = Placement::from(Step::Rigid {
        translation: [Expr::param(lift(), Dimension::Length), len(0.0), len(0.0)],
        axis: [0.0, 0.0, 1.0].map(scl),
        angle: ang(0.0),
    });
    let doc = set_offset(doc, base, Some(offset.clone()));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let cut: std::collections::BTreeSet<RecipeNodeId> = [base, top, mate].into_iter().collect();
    let split = |doc: &ProfileDoc| {
        editor_core::split(
            doc,
            &cut,
            DocumentId::derive("r1-promote-param-part"),
            Tol::witness(),
            o.resolver.as_ref(),
        )
    };
    let err = split(&doc).expect_err("the cut root's offset reads a kept node's parameter");
    assert!(
        matches!(&err, editor_core::SplitError::UncutParamReference { param, cut_node, promote: true, .. }
            if *param == lift() && cut_node.id() == base),
        "{err:?}"
    );
    assert!(
        err.to_string().contains(&format!(
            "Recourse: promote {} (Promote), so its offset stays in this document",
            doc.spoken(base)
        )),
        "{err}"
    );
    let (doc, k) = step(doc, DocEdit::Promote { instance: base });
    let k = k.expect("the promote mints its gauge");
    let out = split(&doc).expect("the promoted offset stays in the host");
    assert!(
        matches!(out.remainder.node(k), Some(Node::Gauge { placement, .. }) if placement.bit_eq(&offset)),
        "the promoted gauge holds the parametric offset"
    );
    assert_eq!(
        out.remainder.node(out.instance).and_then(Node::gauge_ref),
        Some(k),
        "the instance sits on the promoted gauge"
    );
    assert_eq!(
        offset_of(&out.remainder, out.instance),
        Some(Placement::IDENTITY)
    );
    assert!(
        out.part.params().is_empty(),
        "the part copies no parameter: {:?}",
        out.part.params().keys().collect::<Vec<_>>()
    );
}

/// **The memo key carries the lane's group frame**: an evaluation handed
/// its prior moves every instance on a gauge whose parameter moved, and a
/// mate-placed member with its root when the root's offset moved —
/// neither changes a slot of its own, so only the frame in the key
/// (`SolveAnswer::feed_placement`) can tell the two runs apart.
#[test]
fn the_memo_moves_instances_when_their_gauge_or_their_root_moves() {
    let p = parts("r1-memo");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-memo"), Tol::witness());
    let doc = declare_lift(doc, 2.0);
    let (doc, g) = insert(
        doc,
        Node::gauge(
            None,
            Step::Rigid {
                translation: [len(0.0), len(0.0), Expr::param(lift(), Dimension::Length)],
                axis: [0.0, 0.0, 1.0].map(scl),
                angle: ang(0.0),
            },
        ),
    );
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, base, Some(g));
    let doc = set_gauge(doc, top, Some(g));
    let (doc, _) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let ev = run(&doc, &o);
    let minz = |ev: &Evaluation<f64>, id| {
        points(&body_of(ev, id))
            .iter()
            .map(|q| q[2])
            .fold(f64::INFINITY, f64::min)
    };
    let (b0, t0) = (minz(&ev, base), minz(&ev, top));
    let moved = set_lift(doc.clone(), 7.0);
    let ev2 = evaluate::<f64>(
        &moved,
        Some(&ev),
        &editor_core::CancelToken::new(),
        &o,
        Tol::witness(),
    );
    assert!(
        (minz(&ev2, base) - (b0 + 5.0)).abs() < 1e-9,
        "the gauge's instance moved: {} -> {}",
        b0,
        minz(&ev2, base)
    );
    assert!(
        (minz(&ev2, top) - (t0 + 5.0)).abs() < 1e-9,
        "the mated instance moved"
    );
    let shifted = set_offset(
        doc,
        base,
        Some(Placement::literal(&Frame::translation([0.0, 0.0, 3.0]))),
    );
    let ev3 = evaluate::<f64>(
        &shifted,
        Some(&ev),
        &editor_core::CancelToken::new(),
        &o,
        Tol::witness(),
    );
    assert!(
        (minz(&ev3, top) - (t0 + 3.0)).abs() < 1e-9,
        "the mated member moved with its root: {} -> {}",
        t0,
        minz(&ev3, top)
    );
}

/// **Inline's host-gauge guard**: at the empty offset on a non-world
/// gauge, a part whose root is plain geometry cannot be spliced in
/// place — plain geometry sits on no gauge — so inline refuses
/// `UnplaceableFrame`, with a recourse, rather than move it off the
/// gauge.
#[test]
fn inline_on_a_gauge_over_plain_geometry_refuses_rather_than_move_it() {
    let p = parts("r1-inline-plain");
    let doc = ProfileDoc::empty(DocumentId::derive("r1-inline-plain"), Tol::witness());
    let (doc, g) = insert(
        doc,
        Node::gauge(
            None,
            Placement::literal(&Frame::translation([0.0, 10.0, 0.0])),
        ),
    );
    let (doc, h) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, h, Some(g));
    let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(p.store.clone());
    match editor_core::inline(&doc, h, &resolver, Tol::witness()) {
        Err(e @ editor_core::InlineError::UnplaceableFrame { .. }) => {
            assert!(e.to_string().contains("Recourse:"), "{e}");
        }
        other => panic!("plain geometry on a gauge refuses typed: {other:?}"),
    }
}

/// **`InlineError::MatePlaced`'s recourse, followed, inlines**: a top
/// mated onto a base is not its group's root, and the refusal names the
/// root and the placing mate. The base sits where the seat puts the top
/// at the world origin, so following the recourse — delete the mate,
/// give the top its offset (the empty one) — leaves the top a group of
/// its own at the empty offset, which inlines with nothing moved.
#[test]
fn the_mate_placed_recourse_followed_inlines_in_place() {
    let p = parts("r1-mateplaced");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-mateplaced"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(
        doc,
        base,
        Some(Placement::literal(&to_frame(&inv(&seat_rel(&p))))),
    );
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(p.store.clone());
    match editor_core::inline(&doc, top, &resolver, Tol::witness()) {
        Err(e @ editor_core::InlineError::MatePlaced { .. }) => {
            let said = e.to_string();
            let editor_core::InlineError::MatePlaced {
                instance,
                host_root: root,
                mates,
                ..
            } = e
            else {
                unreachable!()
            };
            assert_eq!(
                (
                    instance.id(),
                    root.id(),
                    mates.iter().map(|m| m.id()).collect::<Vec<_>>()
                ),
                (top, base, vec![mate])
            );
            assert!(said.contains("not its group's root"), "{said}");
        }
        other => panic!("a mate-placed instance refuses typed: {other:?}"),
    }
    let before = world_bounds(&doc, &run(&doc, &o));
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: mate });
    let doc = set_offset(doc, top, Some(Placement::IDENTITY));
    let out = editor_core::inline(&doc, top, &resolver, Tol::witness())
        .expect("the recourse followed, inline admits it");
    let after = world_bounds(&out.doc, &run(&out.doc, &o));
    for k in 0..3 {
        assert!(
            (before.0[k] - after.0[k]).abs() < 1e-9 && (before.1[k] - after.1[k]).abs() < 1e-9,
            "nothing moved: {before:?} -> {after:?}"
        );
    }
}

/// **The flush detector refuses across spaces**: a world base lowered so
/// its top cap is the plane z = 0, and an unplaced top whose own space
/// puts its bottom cap on that plane. Nothing outside an unplaced group
/// is compared with it, so the detector refuses `AcrossSpaces` naming
/// the group, rather than answer `Rest`.
#[test]
fn the_flush_detector_refuses_an_unplaced_group_against_the_world() {
    let p = parts("r1-flush");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-flush"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(
        doc,
        base,
        Some(Placement::literal(&Frame::translation([0.0, 0.0, -1.0]))),
    );
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let unplaced = set_offset(doc.clone(), top, None);
    let ev = run(&unplaced, &o);
    assert!(ev.unplaced.contains_key(&top) && !ev.unplaced.contains_key(&base));
    match editor_core::find_flush_candidates(&ev, base, top, Tol::witness()) {
        Err(editor_core::SelectRefusal::AcrossSpaces { group, cause }) => {
            assert_eq!((group, cause), (top, editor_core::Unplaced::NoOffset));
        }
        other => panic!("the detector refuses across spaces: {other:?}"),
    }
    // Placed at the world origin, the same pair is flush.
    let ev = run(&doc, &o);
    let found = editor_core::find_flush_candidates(&ev, base, top, Tol::witness())
        .expect("one space decides");
    assert!(!found.is_empty(), "the placed pair is flush");
}

/// **The compound door refuses a re-gauge that would make another mate
/// start placing**: c and b are bases on gauge g; a1 (a top on the
/// world) declares a seat on c. "Copy b's gauge to a1, then mate a1 to
/// b" would put a1 and c on one gauge, so m1 would start placing and
/// move c's group, which the action never named. It refuses
/// `WouldStartPlacing` naming m1, with a recourse, and nothing moves.
#[test]
fn the_compound_door_refuses_a_regauge_that_would_start_a_declaring_mate_placing() {
    let p = parts("r1-compound-declare");
    let doc = ProfileDoc::empty(DocumentId::derive("r1-compound-declare"), Tol::witness());
    let (doc, g) = insert(
        doc,
        Node::gauge(
            None,
            Placement::literal(&Frame::translation([0.0, 0.0, 0.0])),
        ),
    );
    let (doc, c) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, c, Some(g));
    let doc = set_offset(
        doc,
        c,
        Some(Placement::literal(&Frame::translation([20.0, 0.0, 0.0]))),
    );
    let (doc, b) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, b, Some(g));
    let (doc, a1) = insert(doc, Node::instantiate_part(p.top));
    let (doc, m1) = insert(doc, seat(head(p.top_cap(a1)), head(p.base_cap(c))));
    let regauge_then_mate = |doc: &ProfileDoc| {
        editor_core::regauge_then_mate(
            doc,
            seat(head(p.top_cap(a1)), head(p.base_cap(b))),
            Tol::witness(),
            &editor_core::RefusingReach,
        )
    };
    match regauge_then_mate(&doc) {
        Err(e @ editor_core::EditError::WouldStartPlacing { .. }) => {
            assert!(
                matches!(&e, editor_core::EditError::WouldStartPlacing { mate } if mate.id() == m1)
            );
            let said = e.to_string();
            let problems = test_utils::refusal::problems("WouldStartPlacing", &said, &[], false);
            assert!(problems.is_empty(), "{said}: {problems:#?}");
        }
        other => panic!("the compound door refuses typed: {other:?}"),
    }
    // With m1 gone, the same action re-gauges a1 and places it on b.
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: m1 });
    let done = regauge_then_mate(&doc).expect("no other mate starts placing");
    assert_eq!(
        done.doc.node(a1).and_then(Node::gauge_ref),
        Some(g),
        "a1 is re-gauged onto b's gauge"
    );
    assert_eq!(root_of(&done.doc, a1), b, "b's group places a1");
}

/// **The gauge inline mints is the one a user would insert** (A4, the
/// spec's I1): a host instance at an offset over a part of two lone
/// instances inlines onto a gauge minted under its gauge holding its
/// offset. Inserting that gauge by hand, listing it just ahead of the
/// instance in the roots, putting the instance on it at the empty
/// offset (which moves nothing) and inlining there gives the same
/// document up to node ids, root order included, and no world pose
/// moves.
#[test]
fn the_minted_gauge_is_the_one_a_user_would_insert_and_moves_nothing() {
    let p = parts("minted-vs-hand");
    let o = p.opts();
    let sub = ProfileDoc::empty(DocumentId::derive("minted-vs-hand-sub"), Tol::witness());
    let (sub, _) = insert(sub, Node::instantiate_part(p.base));
    let (sub, s2) = insert(sub, Node::instantiate_part(p.base));
    let sub = set_offset(
        sub,
        s2,
        Some(Placement::literal(&Frame::translation([10.0, 0.0, 0.0]))),
    );
    let mut store = p.store.clone();
    let sub_ref = store.insert(sub, Tol::witness());
    let o = EvalOptions {
        resolver: with_resolver(store.clone()).resolver,
        ..o
    };
    let doc = ProfileDoc::empty(DocumentId::derive("minted-vs-hand"), Tol::witness());
    let (doc, h) = insert(doc, Node::instantiate_part(sub_ref));
    let offset = Placement::literal(
        &Frame::rotate_then_translate([0.0, 0.0, 1.0], 0.3, [2.0, 5.0, 0.0], band()).unwrap(),
    );
    let doc = set_offset(doc, h, Some(offset.clone()));
    let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(store);
    let before = world_bounds(&doc, &run(&doc, &o));
    let minted = editor_core::inline(&doc, h, &resolver, Tol::witness())
        .expect("an offset over a two-group part inlines onto a minted gauge");
    let after = world_bounds(&minted.doc, &run(&minted.doc, &o));
    for k in 0..3 {
        assert!(
            (before.0[k] - after.0[k]).abs() < 1e-9 && (before.1[k] - after.1[k]).abs() < 1e-9,
            "nothing moved: {before:?} -> {after:?}"
        );
    }

    let (hand, g) = insert(doc, Node::gauge(None, offset));
    let mut roots: Vec<RecipeNodeId> = hand.roots().iter().copied().filter(|&r| r != g).collect();
    let at = roots.iter().position(|&r| r == h).expect("h is a root");
    roots.insert(at, g);
    let (hand, _) = step(hand, DocEdit::SetRoots { roots });
    let hand = set_gauge(hand, h, Some(g));
    let hand = set_offset(hand, h, Some(Placement::IDENTITY));
    assert_eq!(
        before,
        world_bounds(&hand, &run(&hand, &o)),
        "the hand-made gauge moves nothing"
    );
    let by_hand = editor_core::inline(&hand, h, &resolver, Tol::witness())
        .expect("at the empty offset the part's content lands on the gauge");
    let gauge_of = |d: &ProfileDoc| {
        d.order()
            .iter()
            .copied()
            .find(|&id| matches!(d.node(id), Some(Node::Gauge { .. })))
            .expect("one gauge")
    };
    let mut map: editor_core::NodeMap = minted
        .node_map
        .iter()
        .map(|(part, &at)| (at, by_hand.node_map[part]))
        .collect();
    map.insert(gauge_of(&minted.doc), gauge_of(&by_hand.doc));
    fixture::round_trip::same_up_to_ids(
        &minted.doc,
        &by_hand.doc,
        &map,
        &editor_core::StepMap::new(),
    )
    .unwrap_or_else(|e| panic!("the minted gauge is the hand-made one:\n{e}"));
}

/// **The kernel's pick orders faces within one space only**: a ray
/// offered a world target and an unplaced one refuses `AcrossSpaces`
/// naming the group, since no one ray's coordinates meet both; each
/// space offered by itself picks.
#[test]
fn a_pick_across_spaces_refuses_and_each_space_picks_by_itself() {
    let p = parts("r1-pick");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-pick"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_offset(doc, top, None);
    let ev = run(&doc, &o);
    let mesh = |id| mesh::tessellate(&body_of(&ev, id), 0.1, Tol::witness()).expect("tessellates");
    let (mb, mt) = (mesh(base), mesh(top));
    let (pb, pt) = (
        editor_core::MeshPick::build(&mb).expect("well-formed"),
        editor_core::MeshPick::build(&mt).expect("well-formed"),
    );
    let tb = editor_core::PickTarget::new(&ev, base, 0, &pb);
    let tt = editor_core::PickTarget::new(&ev, top, 0, &pt);
    let down = bvh::test_support::ray([0.5, 0.5, 10.0], [0.0, 0.0, -1.0]);
    match editor_core::pick_face(&ev, &[tb, tt], &down) {
        Err(editor_core::HitTestError::AcrossSpaces { group, .. }) => assert_eq!(group, top),
        other => panic!("a pick across spaces refuses: {other:?}"),
    }
    for (target, node) in [(tb, base), (tt, top)] {
        let hit = editor_core::pick_face(&ev, &[target], &down)
            .expect("one space picks")
            .expect("the ray meets it");
        assert_eq!(hit.node, node);
    }
}

/// **Gauge references are reading edges** (A12, A11 (2)): an instance
/// reads its gauge and a gauge its parent, so A9 puts two instances on
/// one gauge, with no mate between them, in one component — the gauge
/// fixes their frames relative to each other — and an instance on
/// another gauge in another. A reference to a deleted gauge reads
/// nothing.
#[test]
fn two_instances_on_one_gauge_are_one_component_with_no_mate() {
    let p = parts("r1-reading");
    let doc = ProfileDoc::empty(DocumentId::derive("r1-reading"), Tol::witness());
    let (doc, outer) = insert(
        doc,
        Node::gauge(
            None,
            Placement::literal(&Frame::translation([1.0, 0.0, 0.0])),
        ),
    );
    let (doc, g) = insert(
        doc,
        Node::gauge(
            Some(outer),
            Placement::literal(&Frame::translation([0.0, 1.0, 0.0])),
        ),
    );
    let (doc, h) = insert(
        doc,
        Node::gauge(
            None,
            Placement::literal(&Frame::translation([0.0, 0.0, 1.0])),
        ),
    );
    let (doc, a) = insert(doc, Node::instantiate_part(p.base));
    let (doc, b) = insert(doc, Node::instantiate_part(p.top));
    let (doc, c) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(
        set_gauge(set_gauge(doc, a, Some(g)), b, Some(g)),
        c,
        Some(h),
    );
    let edges = editor_core::reading_edges(&doc);
    for edge in [(g, outer), (a, g), (b, g), (c, h)] {
        assert!(edges.contains(&edge), "{edge:?} in {edges:?}");
    }
    let components = editor_core::relative_freedom_components(&doc);
    let of = |id| components.iter().position(|comp| comp.contains(&id));
    assert_eq!(of(a), of(b), "one gauge, one component");
    assert_ne!(of(a), of(c), "another gauge, another component");
    let (dead, _) = step(doc, DocEdit::DeleteNode { id: h });
    assert!(
        !editor_core::reading_edges(&dead)
            .iter()
            .any(|&(r, _)| r == c),
        "a reference to a deleted gauge reads nothing"
    );
}

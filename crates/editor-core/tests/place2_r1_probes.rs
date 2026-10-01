//! Review lane place2-r1 probes on PR 3676 (P2-core). Not for merge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, Dimension, DocEdit, DocParam, DocParamValue,
    DocRef, DocumentId, EvalOptions, Evaluation, Expr, Frame, MateFault, MateFrame, MatePrimitive,
    Node, NodeErrorKind, ParamName, PatternKind, Placement, ProfileDoc, RecipeNodeId, SitedFace,
    SlotId, StableName, Step, ValuePayload, evaluate, root_of,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{ang, head, head_at, in_copy, insert, len, offset_of, on_frame, run, scl, solve, step, xform};
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
    MateFrame::authored(origin, axis, [1.0, 0.0, 0.0])
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
                i[k].lo() - 1e-12 <= e[k] && e[k] <= i[k].hi() + 1e-12 && i[k].hi() - i[k].lo() <= 1e-9,
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
    let (doc, b) = insert(doc, Node::instantiate_part(p.base.clone()));
    let (doc, t) = insert(doc, Node::instantiate_part(p.top.clone()));
    let (doc, _) = insert(doc, seat(head(p.top_cap(t)), head(p.base_cap(b))));
    let poses = solve(&doc, &p.opts(), Tol::witness());
    of_frame(&poses.placement(&doc, t).unwrap())
}

fn band() -> geom_core::Band {
    fixture::band()
}

/// **A ∘ F ∘ B with a Transform on the PARENT side and a Pattern copy
/// on the CHILD side, under a gauge under a parametric gauge, with a
/// rotated root offset** — compared against an independent 4x4
/// composition in f64 and in the interval lane, at two values of the
/// driving parameter; then a checked offset on the non-root member
/// set to the independently composed truth holds, and one 0.01 off
/// faults.
#[test]
fn r1_afb_parent_transform_child_pattern_nested_parametric_gauges() {
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
    let (doc, base) = insert(doc, Node::instantiate_part(p.base.clone()));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top.clone()));
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
    assert_eq!(offset_of(&doc, top), None, "the mate door cleared the mover");

    let part_base = run(&p.base_doc, &EvalOptions::default());
    let part_top = run(&p.top_doc, &EvalOptions::default());
    let (pb, pt) = (body_of(&part_base, p.base_body), body_of(&part_top, p.top_body));

    let expect = |liftv: f64| -> (M4, M4, M4) {
        let g = mul(&rigid([0.0, 0.0, liftv], [0.0, 0.0, 1.0], 0.25), &of_frame(&gin_frame));
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
        assert!(gap(&pose_base, &w_base) <= 1e-12, "base pose {liftv}: {}", gap(&pose_base, &w_base));
        assert!(gap(&pose_top, &w_top) <= 1e-12, "top pose {liftv}: {}", gap(&pose_top, &w_top));
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
    let holds = set_offset(doc.clone(), top, Some(Placement::literal(&to_frame(&truth))));
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

/// **A stranded tree mate leaves its child welded, unposed, and its
/// checked offset unread.** The top is mated through copy 2 of a
/// pattern; the pattern then shrinks to two copies, so the mate's
/// head dangles. The top stays welded to the base's group (the
/// partition is structural), the tree cannot reach it, and it sits at
/// the group's frame. Give it an offset stating it at x = 20: the
/// statement is neither honoured nor checked, and the top carries no
/// fault.
#[test]
fn r1_a_stranded_tree_mate_ignores_the_childs_checked_offset() {
    let p = parts("r1-strand");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-strand"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(p.base.clone()));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top.clone()));
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
        seat(head_at(pat, in_copy(pat, 2, p.top_cap(top))), head(p.base_cap(base))),
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetStructuralParam {
            node: pat,
            slot: SlotId::Count,
            expr: Expr::count(2),
        },
    );
    let doc = set_offset(doc, top, Some(Placement::literal(&Frame::translation([20.0, 0.0, 0.0]))));
    let poses = solve(&doc, &o, Tol::witness());
    eprintln!("mate fault: {:?}", poses.fault(mate));
    eprintln!("top fault: {:?}", poses.fault(top));
    eprintln!("top root: {:?} (base {base:?})", poses.root(top));
    eprintln!("top placement: {:?}", poses.placement(&doc, top));
    let ev = run(&doc, &o);
    eprintln!("top node error: {:?}", ev.node_error(top).map(|e| e.to_string()));
    if let Some(v) = ev.value(top)
        && let ValuePayload::Body(b) = &v.payload
    {
        let pts = points(b);
        let minx = pts.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min);
        eprintln!("top evaluated min x: {minx}");
    }
    assert!(
        poses.fault(top).is_some()
            || poses
                .placement(&doc, top)
                .map(|f| (f.translation[0] - 20.0).abs() < 1e-9)
                .unwrap_or(false),
        "the top's offset is neither honoured nor checked, and the top carries no fault"
    );
}

/// **A dead-gauge group's checked offsets are not checked.** Base and
/// top on gauge g, the top mated onto the base and then given an offset
/// that disagrees: it faults. Delete g: the group is unplaced (its own
/// space, where "everything inside it solves and checks as usual"), and
/// the disagreeing statement no longer faults.
#[test]
fn r1_a_dead_gauge_group_stops_checking_its_offsets() {
    let p = parts("r1-deadcheck");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-deadcheck"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, Placement::literal(&Frame::translation([0.0, 5.0, 0.0]))));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base.clone()));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top.clone()));
    let doc = set_gauge(doc, base, Some(g));
    let doc = set_gauge(doc, top, Some(g));
    let (doc, _) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let doc = set_offset(doc, top, Some(Placement::literal(&Frame::translation([0.0, 0.0, 50.0]))));
    let poses = solve(&doc, &o, Tol::witness());
    assert!(
        matches!(poses.fault(top), Some(MateFault::OffsetDisagrees { .. })),
        "placed: {:?}",
        poses.fault(top)
    );
    let (dead, _) = step(doc, DocEdit::DeleteNode { id: g });
    let poses = solve(&dead, &o, Tol::witness());
    eprintln!("unplaced: {:?}", poses.unplaced(top));
    eprintln!("dead-gauge top fault: {:?}", poses.fault(top));
    assert!(
        poses.fault(top).is_some(),
        "the statement disagreed inside the group and still does; it is no longer checked"
    );
}

/// **The mate door when the moving group holds a checked offset that
/// precedes the target group's root in document order.** p (top),
/// q (top) stacked on p, q's offset stated where it sits (holds); r
/// (base) inserted last. "Mate p onto r" — ruling C: p's group moves
/// onto r. The door clears p's offset; the merged group's earliest
/// offset-carrying member is now q, which roots it, so r is the one
/// placed (and its offset faults), and p does not move.
#[test]
fn r1_the_mate_door_with_a_checked_offset_in_the_moving_group() {
    let p = parts("r1-door-order");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-door-order"), Tol::witness());
    let (doc, pp) = insert(doc, Node::instantiate_part(p.top.clone()));
    let (doc, q) = insert(doc, Node::instantiate_part(p.top.clone()));
    let (doc, _) = insert(
        doc,
        seat_on(head(p.top_cap(q)), head(p.top_upper_cap(pp)), [0.0, 0.0, TOP_HEIGHT]),
    );
    let doc = fixture::offsets_where_solved(doc, &o);
    assert!(offset_of(&doc, q).is_some());
    assert_eq!(solve(&doc, &o, Tol::witness()).fault(q), None);
    let (doc, r) = insert(doc, Node::instantiate_part(p.base.clone()));
    let doc = set_offset(doc, r, Some(Placement::literal(&Frame::translation([30.0, 0.0, 0.0]))));
    let before_p = solve(&doc, &o, Tol::witness()).placement(&doc, pp).unwrap();
    let (doc, _) = insert(doc, seat(head(p.top_cap(pp)), head(p.base_cap(r))));
    let poses = solve(&doc, &o, Tol::witness());
    eprintln!("root of r: {:?} (p {pp:?}, q {q:?}, r {r:?})", root_of(&doc, r));
    eprintln!("fault r: {:?}", poses.fault(r));
    eprintln!("fault q: {:?}", poses.fault(q));
    eprintln!("p before {:?} after {:?}", before_p.translation, poses.placement(&doc, pp).map(|f| f.translation));
    eprintln!("r after {:?}", poses.placement(&doc, r).map(|f| f.translation));
    assert_eq!(root_of(&doc, r), r, "ruling C: r's group stays put and p's group moves onto it");
}

/// **A declaring mate across gauges whose contact is genuinely
/// ambiguous**: the top stands on its own gauge at the seat, lifted by
/// `d` off the base. At d = 0 the gate certifies; at a d inside the
/// band it must not certify; at d = 0.5 it must not certify.
#[test]
fn r1_a_declaring_mate_across_gauges_at_an_ambiguous_gap() {
    let p = parts("r1-ambiguous");
    let o = p.opts();
    let rel = seat_rel(&p);
    let eps = Tol::witness().eps();
    for d in [0.0, 2.0 * eps, 0.5] {
        let doc = ProfileDoc::empty(DocumentId::derive("r1-ambiguous"), Tol::witness());
        let (doc, g) = insert(doc, Node::gauge(None, Placement::literal(&Frame::translation([0.0, 0.0, d]))));
        let (doc, base) = insert(doc, Node::instantiate_part(p.base.clone()));
        let (doc, top) = insert(doc, Node::instantiate_part(p.top.clone()));
        let doc = set_gauge(doc, top, Some(g));
        let doc = set_offset(doc, top, Some(Placement::literal(&to_frame(&rel))));
        let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
        assert_eq!(
            solve(&doc, &o, Tol::witness()).role(mate),
            Some(editor_core::MateRole::Declaring)
        );
        let ev = run(&doc, &o);
        let verdict = editor_core::assemble(&doc, &ev, Tol::witness());
        eprintln!("d = {d:e}: {:?}", verdict.as_ref().map(|a| a.minted.iter().map(|m| m.mate).collect::<Vec<_>>()).map_err(|e| e.to_string()));
        if d == 0.0 {
            assert!(verdict.is_ok());
        } else {
            assert!(verdict.is_err(), "d = {d:e} certified");
        }
    }
}

/// The unplaced root keyed equal to a placed one: an instance at the
/// world identity and an unplaced instance key their frame the same;
/// check that switching between the two re-evaluates the space.
#[test]
fn r1_an_unplaced_group_and_the_world_identity_measure() {
    let p = parts("r1-measure");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-measure"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(p.base.clone()));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top.clone()));
    let doc = set_offset(doc, top, Some(Placement::literal(&Frame::translation([0.0, 0.0, 1.0]))));
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
    let ev2 = evaluate::<f64>(&un, Some(&ev), &editor_core::CancelToken::new(), &o, Tol::witness());
    let e = ev2.node_error(m);
    eprintln!("{e:?}");
    assert!(matches!(e.map(|e| &e.kind), Some(NodeErrorKind::Unplaced { .. })), "{e:?}");
}

/// Every serialized document in the repo: load it and report.
#[test]
fn r1_every_serialized_document_loads_or_refuses_typed() {
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
        all.push(format!("crates/editor-core/tests/bool13_goldens/v{v}_golden.cad"));
    }
    for f in all {
        let text = std::fs::read_to_string(root.join(&f)).unwrap();
        let r = editor_core::load(&text, Tol::witness());
        eprintln!(
            "{f}: {}",
            match &r {
                Ok(l) => format!("Ok ({} nodes)", l.doc.order().len()),
                Err(e) => format!("Err {}", e.to_string().chars().take(140).collect::<String>()),
            }
        );
        if f.contains("bool13") {
            assert!(r.is_err());
        } else {
            assert!(r.is_ok(), "{f}");
        }
    }
}

/// An instance written without its `gauge` and `offset` keys loads:
/// the two fields default to `None` on the wire, so the old-file
/// refusal rests on the sibling `placements` / `maintenance` keys.
#[test]
fn r1_an_instance_without_its_offset_key_loads_unplaced() {
    let p = parts("r1-wire");
    let doc = ProfileDoc::empty(DocumentId::derive("r1-wire"), Tol::witness());
    let (doc, inst) = insert(doc, Node::instantiate_part(p.base.clone()));
    let text = editor_core::save(&doc, &[], Tol::witness()).unwrap();
    let stripped = crate::wire::doctored(&text, |wire| {
        let nodes = &mut wire["snapshot"]["nodes"];
        let s = serde_json::to_string(nodes).unwrap();
        eprintln!("nodes wire: {}", s.chars().take(400).collect::<String>());
        fn strip(v: &mut serde_json::Value) {
            match v {
                serde_json::Value::Object(m) => {
                    if m.contains_key("doc_ref") {
                        m.remove("offset");
                        m.remove("gauge");
                    }
                    for (_, x) in m.iter_mut() {
                        strip(x);
                    }
                }
                serde_json::Value::Array(a) => a.iter_mut().for_each(strip),
                _ => {}
            }
        }
        strip(nodes);
    });
    let r = editor_core::load(&stripped, Tol::witness());
    match &r {
        Ok(l) => eprintln!("loaded; offset = {:?}", offset_of(&l.doc, inst)),
        Err(e) => eprintln!("refused: {e}"),
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

/// **A verbatim split of an instance on a gauge together with plain
/// world geometry.** The anchor is read off the cut's instances alone,
/// so the plain body casts no vote: the instance left behind names the
/// gauge, and the plain body, now inside the part, is carried by it.
/// Split-then-evaluate must equal the unsplit evaluation.
#[test]
fn r1_a_split_of_a_gauged_instance_with_plain_geometry_keeps_the_geometry_still() {
    let p = parts("r1-split-plain");
    let doc = ProfileDoc::empty(DocumentId::derive("r1-split-plain"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, Placement::literal(&Frame::translation([0.0, 10.0, 0.0]))));
    let (doc, x) = insert(doc, Node::instantiate_part(p.base.clone()));
    let doc = set_gauge(doc, x, Some(g));
    let before_nodes: std::collections::BTreeSet<RecipeNodeId> = doc.order().iter().copied().collect();
    let (doc, prof) = on_frame(
        doc,
        [50.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, ext) = insert(doc, Node::Extrude { profile: prof, distance: len(1.0) });
    let plain: Vec<RecipeNodeId> = doc.order().iter().copied().filter(|id| !before_nodes.contains(id)).collect();
    let o = p.opts();
    let ev = run(&doc, &o);
    let before = world_bounds(&doc, &ev);
    eprintln!("before: {before:?}");
    let mut cut: std::collections::BTreeSet<RecipeNodeId> = plain.iter().copied().collect();
    cut.insert(x);
    let _ = ext;
    let out = editor_core::split(&doc, &cut, DocumentId::derive("r1-split-plain-part"), Tol::witness(), o.resolver.as_ref());
    match out {
        Err(e) => eprintln!("split refused: {e}"),
        Ok(out) => {
            eprintln!("instance gauge: {:?}, offset: {:?}", out.remainder.node(out.instance).and_then(Node::gauge_ref), offset_of(&out.remainder, out.instance));
            let mut store = p.store.clone();
            store.insert(out.part.clone(), Tol::witness());
            let o2 = with_resolver(store);
            let ev2 = run(&out.remainder, &o2);
            let after = world_bounds(&out.remainder, &ev2);
            eprintln!("after: {after:?}");
            for k in 0..3 {
                assert!((before.0[k] - after.0[k]).abs() < 1e-9 && (before.1[k] - after.1[k]).abs() < 1e-9, "split moved the world: {before:?} -> {after:?}");
            }
        }
    }
}

/// The same class through a placer: a cut holding an instance on a
/// gauge and a transform over it. The transform's map acts in the
/// document's coordinates before the split (`T ∘ G ∘ x`) and in the
/// part's after it, which the instance left behind then puts on the
/// gauge (`G ∘ T ∘ x`).
#[test]
fn r1_a_split_of_a_gauged_instance_with_its_transform_keeps_the_geometry_still() {
    let p = parts("r1-split-xform");
    let doc = ProfileDoc::empty(DocumentId::derive("r1-split-xform"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, Placement::literal(&Frame::translation([0.0, 10.0, 0.0]))));
    let (doc, x) = insert(doc, Node::instantiate_part(p.base.clone()));
    let doc = set_gauge(doc, x, Some(g));
    let (doc, t) = insert(doc, xform(x, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.5));
    let o = p.opts();
    let ev = run(&doc, &o);
    let before = world_bounds(&doc, &ev);
    eprintln!("before: {before:?}");
    let cut: std::collections::BTreeSet<RecipeNodeId> = [x, t].into_iter().collect();
    match editor_core::split(&doc, &cut, DocumentId::derive("r1-split-xform-part"), Tol::witness(), o.resolver.as_ref()) {
        Err(e) => eprintln!("split refused: {e}"),
        Ok(out) => {
            let mut store = p.store.clone();
            store.insert(out.part.clone(), Tol::witness());
            let ev2 = run(&out.remainder, &with_resolver(store));
            let after = world_bounds(&out.remainder, &ev2);
            eprintln!("after: {after:?}");
            for k in 0..3 {
                assert!((before.0[k] - after.0[k]).abs() < 1e-9 && (before.1[k] - after.1[k]).abs() < 1e-9, "split moved the world: {before:?} -> {after:?}");
            }
        }
    }
}

/// A document whose only material is an unplaced group: what the
/// world gather and the gate say.
#[test]
fn r1_a_document_of_unplaced_material_alone_at_the_gather_and_the_gate() {
    let p = parts("r1-alone");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-alone"), Tol::witness());
    let (doc, x) = insert(doc, Node::instantiate_part(p.base.clone()));
    let doc = set_offset(doc, x, None);
    let ev = run(&doc, &o);
    eprintln!("unplaced: {:?}", ev.unplaced);
    eprintln!("product: {:?}", editor_core::product(&doc, &ev, Tol::witness()).map(|_| ()).map_err(|e| e.to_string()));
    eprintln!("assemble: {:?}", editor_core::assemble(&doc, &ev, Tol::witness()).map(|_| ()).map_err(|e| e.to_string()));
}

/// The group hoist with its root's offset driven by a parameter a kept
/// node also reads: the offset stays in the host (it becomes the
/// instance's), so nothing is shared across the seam.
#[test]
fn r1_the_group_hoist_of_a_parametric_root_offset() {
    let p = parts("r1-hoist-param");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-hoist-param"), Tol::witness());
    let doc = declare_lift(doc, 2.0);
    let (doc, g) = insert(
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
    let _ = g;
    let (doc, base) = insert(doc, Node::instantiate_part(p.base.clone()));
    let doc = set_offset(
        doc,
        base,
        Some(Placement::from(Step::Rigid {
            translation: [Expr::param(lift(), Dimension::Length), len(0.0), len(0.0)],
            axis: [0.0, 0.0, 1.0].map(scl),
            angle: ang(0.0),
        })),
    );
    let (doc, top) = insert(doc, Node::instantiate_part(p.top.clone()));
    let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let cut: std::collections::BTreeSet<RecipeNodeId> = [base, top, mate].into_iter().collect();
    let out = editor_core::split(&doc, &cut, DocumentId::derive("r1-hoist-param-part"), Tol::witness(), o.resolver.as_ref());
    match out {
        Err(e) => eprintln!("split refused: {e}"),
        Ok(out) => eprintln!(
            "split ok; instance offset {:?}; part params {:?}",
            offset_of(&out.remainder, out.instance),
            out.part.params().keys().collect::<Vec<_>>()
        ),
    }
}

/// **The memo after a gauge's parameter moves, and after the root's
/// offset moves**: an evaluation handed its prior must move every
/// instance on the gauge, and the mate-placed member with its root —
/// neither changes a slot of its own.
#[test]
fn r1_the_memo_moves_instances_when_their_gauge_or_root_moves() {
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
    let (doc, base) = insert(doc, Node::instantiate_part(p.base.clone()));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top.clone()));
    let doc = set_gauge(doc, base, Some(g));
    let doc = set_gauge(doc, top, Some(g));
    let (doc, _) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let ev = run(&doc, &o);
    let minz = |ev: &Evaluation<f64>, id| points(&body_of(ev, id)).iter().map(|q| q[2]).fold(f64::INFINITY, f64::min);
    let (b0, t0) = (minz(&ev, base), minz(&ev, top));
    let moved = set_lift(doc.clone(), 7.0);
    let ev2 = evaluate::<f64>(&moved, Some(&ev), &editor_core::CancelToken::new(), &o, Tol::witness());
    assert!((minz(&ev2, base) - (b0 + 5.0)).abs() < 1e-9, "the gauge's instance moved: {} -> {}", b0, minz(&ev2, base));
    assert!((minz(&ev2, top) - (t0 + 5.0)).abs() < 1e-9, "the mated instance moved");
    let shifted = set_offset(doc, base, Some(Placement::literal(&Frame::translation([0.0, 0.0, 3.0]))));
    let ev3 = evaluate::<f64>(&shifted, Some(&ev), &editor_core::CancelToken::new(), &o, Tol::witness());
    assert!((minz(&ev3, top) - (t0 + 3.0)).abs() < 1e-9, "the mated member moved with its root: {} -> {}", t0, minz(&ev3, top));
}

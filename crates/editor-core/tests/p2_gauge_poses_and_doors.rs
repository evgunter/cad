//! **EDIT-PLACEMENT P2 — world poses against an independent
//! composition, and the doors that move groups** (ASSEMBLY.md A4, A9,
//! A11 (2)–(5)): `A ∘ F ∘ B` through a transform and a pattern under a
//! parametric gauge chain, in f64 and in the interval lane; the mate and
//! compound doors; the at-rest gate, the document seam and the readers
//! across spaces; split and inline. The expected poses are composed
//! here from 4x4 matrices, never read back from the kernel.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::ExtrudeSide;
use std::collections::BTreeMap;
use std::sync::Arc;

use crate::fixture;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, Dimension, DocEdit, DocParam, DocParamValue,
    DocRef, DocumentId, EvalOptions, Evaluation, Expr, Frame, MateFault, MateFrame, MatePrimitive,
    MateRole, Node, ParamBox, ParamName, PatternKind, Placement, ProfileDoc, RecipeNodeId,
    SitedFace, SlotId, StableName, Step, ValuePayload, evaluate, regauge_then_mate, root_of,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{
    ang, head, head_at, in_copy, insert, len, offset_of, on_frame, run, scl, solve, step,
};
use geom_core::{Bounds, Interval, Tol};
use topo::Body;

const BASE_WIDTH: f64 = 3.0;
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
    top: DocRef,
    top_body: RecipeNodeId,
}

fn parts(label: &str) -> Parts {
    let mut store = PartStore::new();
    let (base, base_body) = store.insert_part(
        block(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let (top, top_body) = store.insert_part(
        block(&format!("{label}-top"), 1.0, TOP_HEIGHT),
        Tol::witness(),
    );
    Parts {
        store,
        base,
        base_body,
        top,
        top_body,
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

fn frame(origin: [f64; 3], axis: [f64; 3]) -> MateFrame {
    MateFrame::authored(origin, axis, [1.0, 0.0, 0.0])
}

fn seat_on(mover: SitedFace, onto: SitedFace, at: [f64; 3]) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: mover,
        b: onto,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
            b: frame(at, [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

fn seat(top: SitedFace, base: SitedFace) -> Node<editor_core::ProfileProgram> {
    seat_on(top, base, [1.0, 1.0, BASE_HEIGHT])
}

fn lift() -> ParamName {
    ParamName::from_static("lift")
}

fn set_gauge(doc: ProfileDoc, node: RecipeNodeId, gauge: Option<RecipeNodeId>) -> ProfileDoc {
    step(doc, DocEdit::SetGauge { node, gauge }).0
}

fn set_offset(doc: ProfileDoc, instance: RecipeNodeId, offset: Option<Placement>) -> ProfileDoc {
    step(doc, DocEdit::SetOffset { instance, offset }).0
}

fn literal(m: &M) -> Placement {
    Placement::literal(&m.frame())
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

// ---- an independent rigid-motion algebra (row-major 3x3 + t) ----

#[derive(Clone, Copy, Debug)]
struct M {
    r: [[f64; 3]; 3],
    t: [f64; 3],
}

impl M {
    const I: M = M {
        r: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        t: [0.0; 3],
    };
    fn tr(t: [f64; 3]) -> M {
        M { t, ..M::I }
    }
    /// Rodrigues about a unit axis.
    fn rot(axis: [f64; 3], a: f64) -> M {
        let n = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
        let [x, y, z] = axis.map(|v| v / n);
        let (s, c) = a.sin_cos();
        let k = 1.0 - c;
        M {
            r: [
                [c + x * x * k, x * y * k - z * s, x * z * k + y * s],
                [y * x * k + z * s, c + y * y * k, y * z * k - x * s],
                [z * x * k - y * s, z * y * k + x * s, c + z * z * k],
            ],
            t: [0.0; 3],
        }
    }
    /// `self ∘ o`.
    fn c(&self, o: &M) -> M {
        let mut r = [[0.0; 3]; 3];
        for (i, row) in r.iter_mut().enumerate() {
            for (j, v) in row.iter_mut().enumerate() {
                *v = (0..3).map(|k| self.r[i][k] * o.r[k][j]).sum();
            }
        }
        M { r, t: self.p(o.t) }
    }
    fn p(&self, x: [f64; 3]) -> [f64; 3] {
        let mut out = self.t;
        for (i, o) in out.iter_mut().enumerate() {
            *o += (0..3).map(|k| self.r[i][k] * x[k]).sum::<f64>();
        }
        out
    }
    fn inv(&self) -> M {
        let mut r = [[0.0; 3]; 3];
        for (i, row) in r.iter_mut().enumerate() {
            for (j, v) in row.iter_mut().enumerate() {
                *v = self.r[j][i];
            }
        }
        let m = M { r, t: [0.0; 3] };
        let t = m.p(self.t).map(|v| -v);
        M { r, t }
    }
    fn frame(&self) -> Frame {
        Frame {
            columns: [0, 1, 2].map(|j| [self.r[0][j], self.r[1][j], self.r[2][j]]),
            translation: self.t,
        }
    }
    fn of(f: &Frame) -> M {
        M {
            r: [0, 1, 2].map(|i| [f.columns[0][i], f.columns[1][i], f.columns[2][i]]),
            t: f.translation,
        }
    }
    fn dist(&self, o: &M) -> f64 {
        let mut d: f64 = 0.0;
        for i in 0..3 {
            d = d.max((self.t[i] - o.t[i]).abs());
            for j in 0..3 {
                d = d.max((self.r[i][j] - o.r[i][j]).abs());
            }
        }
        d
    }
}

/// The part's own points (the part evaluated alone at the identity).
fn part_points(doc_label: &str, w: f64, h: f64) -> Vec<[f64; 3]> {
    let (doc, body) = block(doc_label, w, h);
    let ev = run(&doc, &EvalOptions::default());
    body_of(&ev, body)
        .points()
        .map(|(_, p)| [p.x, p.y, p.z])
        .collect()
}

fn check_body_f64(ev: &Evaluation<f64>, id: RecipeNodeId, own: &[[f64; 3]], m: &M, what: &str) {
    let got: Vec<[f64; 3]> = body_of(ev, id)
        .points()
        .map(|(_, p)| [p.x, p.y, p.z])
        .collect();
    assert_eq!(got.len(), own.len(), "{what}: point count");
    for (g, o) in got.iter().zip(own) {
        let e = m.p(*o);
        for k in 0..3 {
            assert!(
                (g[k] - e[k]).abs() <= 1e-9,
                "DEFECT? {what}: point {g:?} vs independent {e:?}"
            );
        }
    }
}

fn check_body_interval(
    ev: &Evaluation<Interval>,
    id: RecipeNodeId,
    own: &[[f64; 3]],
    m: &M,
    what: &str,
) {
    let got: Vec<_> = body_of(ev, id)
        .points()
        .map(|(_, p)| [p.x, p.y, p.z])
        .collect();
    assert_eq!(got.len(), own.len(), "{what}: point count");
    for (g, o) in got.iter().zip(own) {
        let e = m.p(*o);
        for k in 0..3 {
            let (lo, hi) = (g[k].lo(), g[k].hi());
            assert!(
                lo - 1e-12 <= e[k] && e[k] <= hi + 1e-12 && hi - lo <= 1e-9,
                "DEFECT? {what}: interval [{lo}, {hi}] vs independent {}",
                e[k]
            );
        }
    }
}

fn declare(doc: ProfileDoc, name: ParamName, v: f64, dim: Dimension) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetDocParam {
            name,
            value: DocParam::continuous(dim, v),
        },
    )
    .0
}

fn set_value(doc: ProfileDoc, name: ParamName, v: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetDocParamValue {
            name,
            value: DocParamValue::Continuous(v),
        },
    )
    .0
}

/// The parametric chain: G0 literal (rot x 0.3, +[5,0,0]); G1 on G0
/// at [0,0,lift] turned `turn` rad about z where `turn` is a parameter.
struct Chain {
    doc: ProfileDoc,
    _g0: RecipeNodeId,
    g1: RecipeNodeId,
}

fn turn() -> ParamName {
    ParamName::from_static("turn")
}

fn chain(label: &str) -> Chain {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let doc = declare(doc, lift(), 2.0, Dimension::Length);
    let doc = declare(doc, turn(), 0.25, Dimension::Angle);
    let g0m = M::tr([5.0, 0.0, 0.0]).c(&M::rot([1.0, 0.0, 0.0], 0.3));
    let (doc, g0) = insert(doc, Node::gauge(None, literal(&g0m)));
    let (doc, g1) = insert(
        doc,
        Node::gauge(
            Some(g0),
            Step::Rigid {
                translation: [len(0.0), len(0.0), Expr::param(lift(), Dimension::Length)],
                axis: [0.0, 0.0, 1.0].map(scl),
                angle: Expr::param(turn(), Dimension::Angle),
            },
        ),
    );
    Chain { doc, _g0: g0, g1 }
}

fn chain_frame(lift: f64, turn: f64) -> M {
    let g0 = M::tr([5.0, 0.0, 0.0]).c(&M::rot([1.0, 0.0, 0.0], 0.3));
    // Rigid step: translation after rotation (rotate, then translate).
    let g1 = M::tr([0.0, 0.0, lift]).c(&M::rot([0.0, 0.0, 1.0], turn));
    g0.c(&g1)
}

/// The seat's representative B, by msolve's own (pre-P2) sense
/// convention: `Opposed` is a π turn about the frame's x, which with
/// both refs on +x lands the top turned π about z. Pinned against a
/// gauge-free, placer-free control in `probe_seat_rep_control`.
fn seat_rep() -> M {
    M::tr([1.0, 1.0, BASE_HEIGHT]).c(&M::rot([0.0, 0.0, 1.0], std::f64::consts::PI))
}

/// **The seat's relative pose, the one input the rows below take from
/// the kernel rather than compose**: a top mated onto a base at the world
/// origin sits at the composed seat to 1e-15.
#[test]
fn the_seat_relation_the_rows_compose_with_is_the_solves() {
    let p = parts("r2-control");
    let doc = ProfileDoc::empty(DocumentId::derive("r2-control"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let poses = solve(&doc, &p.opts(), Tol::witness());
    let got = M::of(&poses.placement(&doc, top).unwrap());
    assert!(got.dist(&seat_rep()) <= 1e-15, "{got:?}");
}

/// **A ∘ F ∘ B against an independent composition: a Transform between
/// the tree mate's operand and its instance, under a gauge under a gauge
/// whose placement two parameters drive, with a turned root offset**, in
/// f64 and in the interval lane at two parameter values; then a checked
/// offset on the non-root member, true and false.
#[test]
fn a_transform_placer_under_a_two_parameter_gauge_chain_poses_as_composed() {
    let p = parts("r2-afb-x");
    let o = p.opts();
    let Chain { doc, g1, .. } = chain("r2-afb-x");
    let root_off = M::tr([1.0, 2.0, 0.0]).c(&M::rot([0.0, 1.0, 0.0], 0.2));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, base, Some(g1));
    let doc = set_offset(doc, base, Some(literal(&root_off)));
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let doc = set_gauge(doc, top, Some(g1));
    let tm = M::tr([0.0, 0.0, 10.0]).c(&M::rot([1.0, 0.0, 0.0], 0.4));
    let (doc, t) = insert(
        doc,
        Node::transform(
            top,
            Step::Rigid {
                translation: [0.0, 0.0, 10.0].map(len),
                axis: [1.0, 0.0, 0.0].map(scl),
                angle: ang(0.4),
            },
        ),
    );
    let (doc, mate) = insert(
        doc,
        seat(head_at(t, p.top_cap(top)), head(p.base_cap(base))),
    );
    let top_pts = part_points("r2-afb-x-top", 1.0, TOP_HEIGHT);
    let base_pts = part_points("r2-afb-x-base", BASE_WIDTH, BASE_HEIGHT);
    for (lv, tv) in [(2.0, 0.25), (7.0, -1.1)] {
        let doc = set_value(set_value(doc.clone(), lift(), lv), turn(), tv);
        let f = chain_frame(lv, tv).c(&root_off);
        let w_top = tm.inv().c(&f).c(&seat_rep());
        let poses = solve(&doc, &o, Tol::witness());
        assert_eq!(poses.role(mate), Some(MateRole::Determining));
        let got = M::of(&poses.placement(&doc, top).expect("placed"));
        assert!(
            got.dist(&w_top) <= 1e-12,
            "DEFECT: top world pose {got:?} vs independent {w_top:?} at lift {lv} turn {tv}"
        );
        let got_b = M::of(&poses.placement(&doc, base).expect("placed"));
        assert!(got_b.dist(&f) <= 1e-12, "DEFECT: base pose");
        let ev = run(&doc, &o);
        check_body_f64(&ev, top, &top_pts, &w_top, "f64 top");
        check_body_f64(&ev, t, &top_pts, &tm.c(&w_top), "f64 transformed top");
        check_body_f64(&ev, base, &base_pts, &f, "f64 base");
        let lane = evaluate::<Interval>(
            &doc,
            None,
            &editor_core::CancelToken::new(),
            &o,
            Tol::witness(),
        );
        check_body_interval(&lane, top, &top_pts, &w_top, "interval top");
        check_body_interval(&lane, t, &top_pts, &tm.c(&w_top), "interval t");
        check_body_interval(&lane, base, &base_pts, &f, "interval base");

        // A checked offset on the non-root member: true, then false.
        let gauge_local = chain_frame(lv, tv).inv().c(&w_top);
        let agrees = set_offset(doc.clone(), top, Some(literal(&gauge_local)));
        let poses = solve(&agrees, &o, Tol::witness());
        assert_eq!(root_of(&agrees, top), base);
        assert_eq!(
            poses.fault(top),
            None,
            "DEFECT: a true checked offset faulted at lift {lv}"
        );
        let off = M::tr([0.0, 0.0, 1e-3]).c(&gauge_local);
        let disagrees = set_offset(doc.clone(), top, Some(literal(&off)));
        let poses = solve(&disagrees, &o, Tol::witness());
        assert!(
            matches!(poses.fault(top), Some(MateFault::OffsetDisagrees { .. })),
            "DEFECT: a false checked offset (1e-3 off) not faulted: {:?}",
            poses.fault(top)
        );
    }
}

/// **A ∘ F ∘ B with a Pattern between the mate's operand and its
/// instance**: the mate reads copy 1 of a linear pattern of the top.
#[test]
fn a_pattern_placer_poses_as_composed() {
    let p = parts("r2-afb-p");
    let o = p.opts();
    let Chain { doc, g1, .. } = chain("r2-afb-p");
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, base, Some(g1));
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let doc = set_gauge(doc, top, Some(g1));
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: top,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [0.0, 1.0, 0.0].map(scl),
                spacing: len(4.0),
            },
        },
    );
    let (doc, mate) = insert(
        doc,
        seat(
            head_at(pat, in_copy(pat, 1, p.top_cap(top))),
            head(p.base_cap(base)),
        ),
    );
    let top_pts = part_points("r2-afb-p-top", 1.0, TOP_HEIGHT);
    for (lv, tv) in [(2.0, 0.25), (-3.0, 2.0)] {
        let doc = set_value(set_value(doc.clone(), lift(), lv), turn(), tv);
        let f = chain_frame(lv, tv);
        let w_top = M::tr([0.0, -4.0, 0.0]).c(&f).c(&seat_rep());
        let poses = solve(&doc, &o, Tol::witness());
        assert_eq!(
            poses.role(mate),
            Some(MateRole::Determining),
            "{:?}",
            poses.fault(mate)
        );
        let got = M::of(&poses.placement(&doc, top).expect("placed"));
        assert!(
            got.dist(&w_top) <= 1e-12,
            "DEFECT: pattern top pose {got:?} vs {w_top:?}"
        );
        let ev = run(&doc, &o);
        check_body_f64(&ev, top, &top_pts, &w_top, "f64 top under pattern");
        let lane = evaluate::<Interval>(
            &doc,
            None,
            &editor_core::CancelToken::new(),
            &o,
            Tol::witness(),
        );
        check_body_interval(&lane, top, &top_pts, &w_top, "interval top under pattern");
    }
}

/// **A is the bit-exact identity with no placer, under a gauge chain**:
/// the world pose equals `F ∘ relative` bitwise.
#[test]
fn with_no_placer_the_world_pose_is_the_frame_onto_the_relative_bit_for_bit() {
    let p = parts("r2-bits");
    let o = p.opts();
    let Chain { doc, g1, .. } = chain("r2-bits");
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, base, Some(g1));
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let doc = set_gauge(doc, top, Some(g1));
    let (doc, _) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let poses = solve(&doc, &o, Tol::witness());
    let f = poses.placement(&doc, base).unwrap();
    let rel = poses.relative(top).unwrap();
    assert!(poses.placement(&doc, top).unwrap().bit_eq(&f.compose(&rel)));
}

// ---- the mate door (ruling C) ----

/// **The mate door with a checked member in the first operand's group**
/// (ruling C): a1 (a top) with a2 stacked on it, a2's offset stated where
/// the solve puts it; b (the base) inserted later at [20, 0, 0]. "Mate a1
/// to b" moves a's group onto b and leaves b where it is.
#[test]
fn mate_a_to_b_moves_a_whole_group_and_leaves_b_where_it_is() {
    let p = parts("r2-door2");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r2-door2"), Tol::witness());
    let (doc, a1) = insert(doc, Node::instantiate_part(p.top));
    let (doc, a2) = insert(doc, Node::instantiate_part(p.top));
    let (doc, _) = insert(
        doc,
        seat_on(
            head(p.top_cap(a2)),
            head(p.top_upper_cap(a1)),
            [0.0, 0.0, TOP_HEIGHT],
        ),
    );
    assert_eq!(offset_of(&doc, a2), None, "the door cleared a2's offset");
    // a2 states where it sits: a true checked statement.
    let solved = solve(&doc, &o, Tol::witness()).placement(&doc, a2).unwrap();
    let doc = set_offset(doc, a2, Some(Placement::literal(&solved)));
    assert_eq!(solve(&doc, &o, Tol::witness()).fault(a2), None);
    let (doc, b) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(
        doc,
        b,
        Some(Placement::literal(&Frame::translation([20.0, 0.0, 0.0]))),
    );
    let (doc, m) = insert(doc, seat(head(p.top_cap(a1)), head(p.base_cap(b))));
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.fault(m), None, "the mate places");
    assert_eq!(
        offset_of(&doc, a2),
        None,
        "the checked member's offset is cleared too"
    );
    assert_eq!(
        poses.fault(b),
        None,
        "DEFECT: the SECOND operand faulted after 'mate a1 to b'"
    );
    let b_at = poses.placement(&doc, b).expect("b placed").translation;
    assert_eq!(b_at, [20.0, 0.0, 0.0], "DEFECT: b moved");
    let a1_at = poses.placement(&doc, a1).expect("a1 placed").translation;
    assert!(
        (a1_at[0] - 21.0).abs() < 1e-12 && (a1_at[1] - 1.0).abs() < 1e-12,
        "DEFECT: a1 not moved onto b: {a1_at:?}"
    );
}

/// **The compound door refuses a re-gauge that turns a declaring mate
/// placing**: `other` (on gauge g) declares a seat on `top` (on the
/// world). "Copy base's gauge to top, then mate top to base" would put
/// `top` and `other` on one gauge, so `d` would start placing; the door
/// refuses `WouldStartPlacing` naming `d`, and the document is
/// untouched.
#[test]
fn the_compound_door_refuses_when_a_declaring_mate_would_start_placing() {
    let p = parts("r2-regauge");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r2-regauge"), Tol::witness());
    let (doc, g) = insert(
        doc,
        Node::gauge(
            None,
            Placement::literal(&Frame::translation([10.0, 0.0, 0.0])),
        ),
    );
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, base, Some(g));
    let (doc, other) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, other, Some(g));
    let doc = set_offset(
        doc,
        other,
        Some(Placement::literal(&Frame::translation([5.0, 5.0, 0.0]))),
    );
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let (doc, d) = insert(
        doc,
        seat_on(
            head(p.top_cap(other)),
            head(p.top_upper_cap(top)),
            [0.0, 0.0, TOP_HEIGHT],
        ),
    );
    assert_eq!(
        solve(&doc, &o, Tol::witness()).role(d),
        Some(MateRole::Declaring)
    );
    match regauge_then_mate(&doc, seat(head(p.top_cap(top)), head(p.base_cap(base)))) {
        Err(editor_core::EditError::WouldStartPlacing { mate }) => assert_eq!(mate.id(), d),
        other => panic!("the compound door refuses typed: {other:?}"),
    }
}

// ---- checked offsets the solve never reaches ----

/// **A welded member the spanning tree cannot reach has its offset
/// faulted, not ignored**: before the pattern shrinks, the top's false
/// statement disagrees; after, the mate's copy is gone, the tree cannot
/// reach the top, and its statement is `OffsetUnchecked { Unreached }`
/// naming the mate — so the top does not evaluate at a place its own
/// offset contradicts.
#[test]
fn an_unreachable_member_with_an_offset_faults_and_does_not_evaluate() {
    let p = parts("r2-unreach");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r2-unreach"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: top,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [0.0, 1.0, 0.0].map(scl),
                spacing: len(4.0),
            },
        },
    );
    let (doc, m) = insert(
        doc,
        seat(
            head_at(pat, in_copy(pat, 1, p.top_cap(top))),
            head(p.base_cap(base)),
        ),
    );
    let doc = set_offset(
        doc,
        top,
        Some(Placement::literal(&Frame::translation([50.0, 0.0, 0.0]))),
    );
    let poses = solve(&doc, &o, Tol::witness());
    assert!(matches!(
        poses.fault(top),
        Some(MateFault::OffsetDisagrees { .. })
    ));
    let doc = step(
        doc,
        DocEdit::SetStructuralParam {
            node: pat,
            slot: SlotId::Count,
            expr: Expr::count(1),
        },
    )
    .0;
    let poses = solve(&doc, &o, Tol::witness());
    assert!(
        matches!(
            poses.fault(top),
            Some(MateFault::OffsetUnchecked { cause, .. })
                if matches!(**cause, editor_core::OffsetCheck::Unreached { mate } if mate == m)
        ),
        "{:?}",
        poses.fault(top)
    );
    let ev = run(&doc, &o);
    assert!(ev.value(top).is_none(), "the faulted top does not evaluate");
}

// ---- the unplaced space ----

/// **The public clearance door refuses across spaces**: a world base
/// and a top on a deleted gauge. The two selections live in different
/// spaces, so the door refuses `AcrossSpaces` naming the group — before
/// it reads either body, which here has no resolver to build.
#[test]
fn the_clearance_door_refuses_two_selections_in_different_spaces() {
    use editor_core::clearance::{
        ClearanceRefusal, ClearanceVerdict, Selection, SelectionRefusal, clearance,
    };
    let p = parts("r2-clear");
    let doc = ProfileDoc::empty(DocumentId::derive("r2-clear"), Tol::witness());
    let (doc, g) = insert(
        doc,
        Node::gauge(
            None,
            Placement::literal(&Frame::translation([0.0, 0.0, 50.0])),
        ),
    );
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, top, Some(g));
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: g });
    let report = clearance(
        &doc,
        &ParamBox::from_axes(BTreeMap::new()),
        &Selection::body_of(base),
        &Selection::body_of(top),
        0.5,
        Tol::witness(),
    );
    match report.verdict() {
        ClearanceVerdict::Refused(ClearanceRefusal::Selection(
            SelectionRefusal::AcrossSpaces { group, .. },
        )) => assert_eq!(*group, top),
        other => panic!("clearance across spaces refuses typed: {other:?}"),
    }
}

/// **The at-rest gate checks own spaces regardless of the world**: an
/// unplaced group whose instance only an in-group measure reads has no
/// body root in its space, which holds nothing to check, so the gate
/// certifies the world. With the world's base deleted, the material is
/// unplaced alone: the gate checks the group in its own space and then
/// raises the gather's refusal, which names the group and the place-it
/// recourse.
#[test]
fn the_gate_checks_own_spaces_whatever_the_world_holds() {
    let p = parts("r2-gate");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r2-gate"), Tol::witness());
    let (doc, g) = insert(
        doc,
        Node::gauge(
            None,
            Placement::literal(&Frame::translation([0.0, 0.0, 50.0])),
        ),
    );
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, top, Some(g));
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: g });
    let (doc_m, meas) = insert(
        doc.clone(),
        Node::measure(
            editor_core::MeasureExpr::primitive(editor_core::MeasurePrimitive::Distance {
                a: 0,
                b: 1,
            }),
            vec![
                editor_core::SitedRef::at_mint(p.top_cap(top)),
                editor_core::SitedRef::at_mint(p.top_upper_cap(top)),
            ],
        )
        .unwrap(),
    );
    let ev = run(&doc_m, &o);
    assert!(ev.value(meas).is_some(), "{:?}", ev.node_error(meas));
    editor_core::assemble(&doc_m, &ev, Tol::witness())
        .expect("an own space with no body root holds nothing to check");
    let (alone, _) = step(doc, DocEdit::DeleteNode { id: base });
    let ev = run(&alone, &o);
    match editor_core::assemble(&alone, &ev, Tol::witness()) {
        Err(editor_core::AssemblyError::Product(e)) => match *e {
            editor_core::ProductError::Unplaced { ref groups } => {
                assert_eq!(groups.len(), 1);
                assert_eq!(groups[0].0, top);
                assert!(e.to_string().contains(editor_core::UNPLACED_RECOURSE));
            }
            ref other => panic!("the gather names the unplaced group: {other:?}"),
        },
        other => panic!("unplaced material alone refuses at the gather: {other:?}"),
    }
}

/// **An unplaced group below crosses the document seam as a fact**: a
/// sub-assembly holding a placed base and an unplaced top crosses as
/// its world product (A9: the top is not part of that body), so the
/// outer product holds the base's material alone and the outer gate
/// certifies it; the outer evaluation names the top, routed through
/// the instance it arrived by (`Evaluation::unplaced_below`), on the
/// instance and on a transform consuming it.
#[test]
fn an_unplaced_group_below_crosses_the_seam_as_a_named_fact() {
    let p = parts("r2-seam");
    let sub = ProfileDoc::empty(DocumentId::derive("r2-seam-sub"), Tol::witness());
    let (sub, g) = insert(
        sub,
        Node::gauge(
            None,
            Placement::literal(&Frame::translation([0.0, 0.0, 50.0])),
        ),
    );
    let (sub, base) = insert(sub, Node::instantiate_part(p.base));
    let (sub, top) = insert(sub, Node::instantiate_part(p.top));
    let sub = set_gauge(sub, top, Some(g));
    let (sub, _) = step(sub, DocEdit::DeleteNode { id: g });
    let o = p.opts();
    let ev_sub = run(&sub, &o);
    assert!(ev_sub.unplaced.contains_key(&top));
    let mut store = p.store.clone();
    let sub_ref = store.insert(sub.clone(), Tol::witness());
    let outer = ProfileDoc::empty(DocumentId::derive("r2-seam-outer"), Tol::witness());
    let (outer, inst) = insert(outer, Node::instantiate_part(sub_ref));
    let (outer, moved) = insert(
        outer,
        fixture::xform(inst, [1.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0),
    );
    let ev = run(&outer, &with_resolver(store));
    let expected = editor_core::CarriedUnplaced {
        route: editor_core::Route {
            through: inst,
            of: sub.id(),
            via: Vec::new(),
        },
        group: top,
        cause: editor_core::Unplaced::DeadGauge { gauge: g },
    };
    assert_eq!(ev.unplaced_below.get(&inst), Some(&vec![expected.clone()]));
    assert_eq!(ev.unplaced_below.get(&moved), Some(&vec![expected.clone()]));
    assert_eq!(ev.all_unplaced_below(), vec![expected]);
    let vol = |b: &Body<f64>| {
        topo::mass_properties(b, Tol::witness())
            .expect("mass")
            .volume
    };
    let gathered = editor_core::product(&outer, &ev, Tol::witness()).expect("the world gathers");
    assert!(
        (vol(&gathered) - vol(&body_of(&ev_sub, base))).abs() < 1e-9,
        "the outer product is the sub-assembly's world"
    );
    editor_core::assemble(&outer, &ev, Tol::witness()).expect("the outer gate certifies");
}

fn split_of(
    p: &Parts,
    doc: &ProfileDoc,
    ids: &[RecipeNodeId],
    label: &str,
) -> Result<editor_core::SplitOutcome, editor_core::SplitError> {
    editor_core::split(
        doc,
        &ids.iter().copied().collect(),
        DocumentId::derive(label),
        Tol::witness(),
        p.opts().resolver.as_ref(),
    )
}

/// **A placing mate never crosses a cut** (A4): a cut holding a placed
/// group's two instances but not the mate placing them refuses
/// `PlacingMateLeft` naming the mate, rather than leave a self-mate in
/// the remainder; with the mate in the cut the group hoists.
#[test]
fn a_cut_that_leaves_its_groups_placing_mate_behind_refuses() {
    let p = parts("r2-split-mate");
    let doc = ProfileDoc::empty(DocumentId::derive("r2-split-mate"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(
        doc,
        base,
        Some(Placement::literal(&Frame::translation([4.0, 0.0, 0.0]))),
    );
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    match split_of(&p, &doc, &[base, top], "r2-split-mate-part") {
        Err(e @ editor_core::SplitError::PlacingMateLeft { .. }) => {
            assert!(
                matches!(&e, editor_core::SplitError::PlacingMateLeft { mate: m } if m.id() == mate)
            );
            assert!(e.to_string().contains("Recourse: add Mate"), "{e}");
        }
        other => panic!("a placing mate left behind refuses typed: {other:?}"),
    }
    split_of(&p, &doc, &[base, top, mate], "r2-split-mate-whole")
        .expect("the whole group with its mate hoists");
}

/// **Inline of split on the verbatim shape** (a cut of two placed
/// groups on the world) returns every instance's world pose.
#[test]
fn inline_of_a_verbatim_split_returns_every_world_pose() {
    let p = parts("r2-rt");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r2-rt"), Tol::witness());
    let (doc, b1) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(
        doc,
        b1,
        Some(Placement::literal(&Frame::translation([4.0, 0.0, 0.0]))),
    );
    let (doc, t1) = insert(doc, Node::instantiate_part(p.top));
    let (doc, m1) = insert(doc, seat(head(p.top_cap(t1)), head(p.base_cap(b1))));
    let (doc, b2) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(
        doc,
        b2,
        Some(Placement::literal(
            &Frame::rotate_then_translate([0.0, 0.0, 1.0], 0.7, [-9.0, 2.0, 0.0], fixture::band())
                .unwrap(),
        )),
    );
    let before: Vec<M> = [b1, t1, b2]
        .iter()
        .map(|&i| M::of(&solve(&doc, &o, Tol::witness()).placement(&doc, i).unwrap()))
        .collect();
    let out = split_of(&p, &doc, &[b1, t1, m1, b2], "r2-rt-part").expect("verbatim");
    let mut store = p.store.clone();
    let _ = store.insert(out.part.clone(), Tol::witness());
    let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(store.clone());
    let back = editor_core::inline(&out.remainder, out.instance, &resolver, Tol::witness())
        .expect("inline of the verbatim split");
    let po = with_resolver(store);
    let poses = solve(&back.doc, &po, Tol::witness());
    for (i, old) in [b1, t1, b2].iter().enumerate() {
        let new = back.node_map[&out.node_map[old]];
        let got = M::of(&poses.placement(&back.doc, new).unwrap());
        assert!(
            got.dist(&before[i]) <= 1e-12,
            "DEFECT: {old:?} moved over inline(split): {got:?} vs {:?}",
            before[i]
        );
    }
}

/// **`InlineError::MatePlaced` says why truly when an earlier member
/// roots the group**: the base, inserted first, carries the root
/// offset; giving the top an offset alone leaves the base the root, and
/// the refusal names it and the mate that places the top. Deleting
/// that mate, as the recourse says, makes the top its own group's root,
/// and inline no longer refuses it as mate-placed.
#[test]
fn the_mate_placed_recourse_holds_when_an_earlier_member_roots_the_group() {
    let p = parts("r2-mp");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r2-mp"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(doc, base, Some(literal(&seat_rep().inv())));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(p.store.clone());
    let stated = set_offset(doc, top, Some(Placement::IDENTITY));
    assert_eq!(
        solve(&stated, &o, Tol::witness()).fault(top),
        None,
        "the top's stated offset is true"
    );
    match editor_core::inline(&stated, top, &resolver, Tol::witness()) {
        Err(editor_core::InlineError::MatePlaced {
            host_root: root,
            mates,
            ..
        }) => {
            assert_eq!(
                (root.id(), mates.iter().map(|m| m.id()).collect::<Vec<_>>()),
                (base, vec![mate])
            );
        }
        other => panic!("a checked member is still not the root: {other:?}"),
    }
    let (doc, _) = step(stated, DocEdit::DeleteNode { id: mate });
    editor_core::inline(&doc, top, &resolver, Tol::witness())
        .expect("the recourse followed, inline admits it");
}

/// **A declaring mate across gauges, at rest, swept through the
/// contact's ambiguity**: the top's stated offset puts it δ above the
/// base's cap (δ < 0 interpenetrates). Certified at δ = 0, never across
/// a real gap or overlap.
#[test]
fn a_declaring_mate_across_gauges_never_certifies_a_real_gap_or_overlap() {
    let p = parts("r2-xg");
    let o = p.opts();
    let eps = Tol::witness().eps();
    let mut report = Vec::new();
    for delta in [
        0.0,
        0.25 * eps,
        0.9 * eps,
        2.0 * eps,
        10.0 * eps,
        1e-6,
        1e-3,
        -1e-6,
        -1e-3,
    ] {
        let doc = ProfileDoc::empty(DocumentId::derive("r2-xg"), Tol::witness());
        let (doc, g) = insert(
            doc,
            Node::gauge(
                None,
                Placement::literal(&Frame::translation([0.0, 0.0, delta])),
            ),
        );
        let (doc, base) = insert(doc, Node::instantiate_part(p.base));
        let (doc, top) = insert(doc, Node::instantiate_part(p.top));
        let doc = set_gauge(doc, top, Some(g));
        let seated = Frame {
            columns: [[-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]],
            translation: [1.0, 1.0, BASE_HEIGHT],
        };
        let doc = set_offset(doc, top, Some(Placement::literal(&seated)));
        let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
        assert_eq!(
            solve(&doc, &o, Tol::witness()).role(mate),
            Some(MateRole::Declaring)
        );
        let verdict = match editor_core::assemble(&doc, &run(&doc, &o), Tol::witness()) {
            Ok(a) => format!("certified, minted {}", a.minted.len()),
            Err(e) => e.to_string(),
        };
        report.push((delta, verdict));
    }
    assert!(
        report[0].1.starts_with("certified"),
        "the seated contact certifies: {:?}",
        report[0]
    );
    for (delta, verdict) in &report {
        if delta.abs() > 2.0 * eps {
            assert!(
                !verdict.starts_with("certified"),
                "DEFECT: a declared contact across a real gap/overlap of {delta:e} certified"
            );
        }
    }
}

//! Review lane place2-r2's probes on PR #3676 (P2-core, frozen head
//! 1057040e3). Each row states what it falsifies; a row that panics
//! with "DEFECT" is a red probe.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::fixture;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, Dimension, DocEdit, DocParam, DocParamValue,
    DocRef, DocumentId, EvalOptions, Evaluation, Expr, Frame, MateFault, MateFrame, MatePrimitive,
    MateRole, Node, ParamBox, ParamName, PatternKind, Placement, ProfileDoc, RecipeNodeId,
    SitedFace, SlotId, StableName, Step, ValuePayload, evaluate, groups, regauge_then_mate,
    root_of,
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

#[test]
fn probe_seat_rep_control() {
    let p = parts("r2-control");
    let doc = ProfileDoc::empty(DocumentId::derive("r2-control"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let poses = solve(&doc, &p.opts(), Tol::witness());
    let got = M::of(&poses.placement(&doc, top).unwrap());
    assert!(got.dist(&seat_rep()) <= 1e-15, "{got:?}");
}

/// **A∘F∘B against an independent composition: a Transform between
/// the tree mate's operand and its instance, under a gauge under a
/// gauge whose placement two parameters drive, with a turned root
/// offset; f64 and Interval, two parameter values; then a checked
/// offset on the non-root member, true and false.**
#[test]
fn probe_afb_transform_gauge_under_gauge_independent() {
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

/// **A∘F∘B with a Pattern between the mate's operand and its
/// instance**: the mate reads copy 1 of a linear pattern of the top.
#[test]
fn probe_afb_pattern_independent() {
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

/// **A is the bit-exact identity with no placer, under a gauge
/// chain**: the world pose equals `F ∘ relative` bitwise.
#[test]
fn probe_no_placer_bits_under_gauge_chain() {
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

/// **The mate door when the FIRST operand's group has a second member
/// carrying a (true) checked offset.** a1 (a top block) with a2
/// stacked on it, a2's offset stated where the solve puts it; b (the
/// base) inserted later at [20,0,0]. "Mate a1 to b" must move a's
/// group onto b and leave b where it is.
#[test]
fn probe_mate_door_with_a_checked_member_in_the_first_group() {
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
    eprintln!(
        "root {:?}; offsets a1 {:?} a2 {:?} b {:?}; faults a1 {:?} a2 {:?} b {:?} m {:?}",
        root_of(&doc, a1),
        offset_of(&doc, a1).is_some(),
        offset_of(&doc, a2).is_some(),
        offset_of(&doc, b).is_some(),
        poses.fault(a1),
        poses.fault(a2),
        poses.fault(b),
        poses.fault(m)
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

/// **The compound door when the first operand carries a DECLARING
/// mate to an instance already on the target gauge.** Re-gauging turns
/// that mate placing; the door then clears the wrong root's offset.
#[test]
fn probe_regauge_then_mate_with_a_declaring_mate_that_starts_placing() {
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
    // other sits on top: across gauges, so it declares.
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
    let edits = regauge_then_mate(&doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let mut done = doc;
    for e in edits {
        done = step(done, e).0;
    }
    let poses = solve(&done, &o, Tol::witness());
    eprintln!(
        "groups {:?}; offsets other {:?} top {:?}; faults top {:?} other {:?} d {:?}; role d {:?}",
        groups(&done),
        offset_of(&done, other).is_some(),
        offset_of(&done, top).is_some(),
        poses.fault(top),
        poses.fault(other),
        poses.fault(d),
        poses.role(d)
    );
    assert_eq!(
        poses.fault(top),
        None,
        "DEFECT: the compound door left the mated instance faulted"
    );
    assert_eq!(
        poses.placement(&done, top).unwrap().translation,
        [11.0, 1.0, 1.0],
        "DEFECT: top not seated on the base"
    );
}

// ---- checked offsets the solve never reaches ----

/// **A welded member the spanning tree cannot reach (its mate refused
/// at `check_references`) keeps an offset of its own; is it checked,
/// or silently ignored?**
#[test]
fn probe_unreachable_member_offset() {
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
    // The user states where the top sits — far off the base.
    let doc = set_offset(
        doc,
        top,
        Some(Placement::literal(&Frame::translation([50.0, 0.0, 0.0]))),
    );
    // Before the shrink: the statement is checked, and false.
    let poses = solve(&doc, &o, Tol::witness());
    assert!(matches!(
        poses.fault(top),
        Some(MateFault::OffsetDisagrees { .. })
    ));
    // Shrink the pattern: the mate's copy no longer exists.
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
    eprintln!(
        "mate fault {:?}; top fault {:?}; top root {:?}; top pose {:?}",
        poses.fault(m).map(|f| f.to_string()),
        poses.fault(top),
        poses.root(top),
        poses.placement(&doc, top).map(|f| f.translation)
    );
    let ev = run(&doc, &o);
    if let Some(v) = ev.value(top)
        && let ValuePayload::Body(b) = &v.payload
    {
        let lo = b.points().fold(f64::INFINITY, |m, (_, p)| m.min(p.x));
        panic!(
            "DEFECT: the top evaluates with min x {lo} although its own offset says x = 50 and \
             no check ran on it (fault {:?})",
            poses.fault(top)
        );
    }
}

// ---- the unplaced space ----

/// **The public clearance door across spaces**: a world base, a top on
/// a deleted gauge. `clearance::clearance` takes two selections at two
/// nodes; does it refuse, or answer a number across spaces?
#[test]
fn probe_clearance_door_across_spaces() {
    use editor_core::clearance::{ClearanceVerdict, Selection, clearance};
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
    let ev = run(&doc, &p.opts());
    assert!(ev.unplaced.contains_key(&top), "the top is unplaced");
    // clearance evaluates the document itself, with the default
    // options; it has no resolver door, so ask through a part-free
    // twin first to see whether the door reads spaces at all.
    let report = clearance(
        &doc,
        &ParamBox::from_axes(BTreeMap::new()),
        &Selection::body_of(base),
        &Selection::body_of(top),
        0.5,
        Tol::witness(),
    );
    eprintln!("clearance verdict: {:?}", report.verdict());
    if !matches!(report.verdict(), ClearanceVerdict::Refused(_)) {
        panic!(
            "DEFECT: clearance across spaces answered {:?}",
            report.verdict()
        );
    }
}

/// **The at-rest gate when an unplaced group has no body ROOT in its
/// space** (its instance is consumed by an in-group measure), and when
/// the world holds no body at all.
#[test]
fn probe_assemble_with_unplaced_groups_and_no_body_roots() {
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
    // An in-group measure of the top alone.
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
    let gate = editor_core::assemble(&doc_m, &ev, Tol::witness());
    eprintln!("gate with an in-group measure: {:?}", gate.as_ref().err());
    let world_only = {
        let (d, _) = step(doc.clone(), DocEdit::DeleteNode { id: base });
        let ev = run(&d, &o);
        editor_core::assemble(&d, &ev, Tol::witness()).err()
    };
    eprintln!("gate with only an unplaced group: {world_only:?}");
    assert!(
        gate.is_ok(),
        "DEFECT: the gate refuses a document whose unplaced group's instance is measured: {:?}",
        gate.err()
    );
    assert!(
        world_only.is_none(),
        "DEFECT: the gate refuses a document holding only an unplaced group: {world_only:?}"
    );
}

/// **The document seam**: a sub-assembly holding a placed base and an
/// unplaced top, instantiated in an outer document. What crosses the
/// seam, and does anything say the top was left behind?
#[test]
fn probe_sub_assembly_with_an_unplaced_group_crosses_the_seam() {
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
    assert!(ev_sub.value(base).is_some());
    let mut store = p.store.clone();
    let sub_ref = store.insert(sub.clone(), Tol::witness());
    let outer = ProfileDoc::empty(DocumentId::derive("r2-seam-outer"), Tol::witness());
    let (outer, inst) = insert(outer, Node::instantiate_part(sub_ref));
    let oo = with_resolver(store);
    let ev = run(&outer, &oo);
    let vol = |b: &Body<f64>| {
        topo::mass_properties(b, Tol::witness())
            .expect("mass")
            .volume
    };
    let base_vol = vol(&body_of(&ev_sub, base));
    let top_vol = vol(&body_of(&ev_sub, top));
    match ev.value(inst) {
        None => eprintln!("outer instance refuses: {:?}", ev.node_error(inst)),
        Some(_) => {
            let v = vol(&body_of(&ev, inst));
            eprintln!(
                "outer instance volume {v} (base {base_vol}, top {top_vol}); outer unplaced {:?}",
                ev.unplaced
            );
            let gathered = editor_core::product(&outer, &ev, Tol::witness()).expect("gathers");
            let pv = vol(&gathered);
            panic!(
                "DEFECT?: the sub-assembly crossed the seam as its world alone (volume {pv}; the \
                 unplaced top's {top_vol} silently dropped), and the outer evaluation names no \
                 unplaced part"
            );
        }
    }
}

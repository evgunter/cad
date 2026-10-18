//! **EDIT-PLACEMENT P2 — world poses against an independent
//! composition, and the doors across spaces** (ASSEMBLY.md A4, A9,
//! A11 (2)–(5); D10, Spaces): a seat under a world pin whose pose two
//! parameters drive, and a copy placed against the seated copy, in f64 and in
//! the interval lane; a constraint that moves a loose space onto the
//! world; the clearance door, the at-rest gate and the document seam
//! across spaces; inline of split. The expected poses are composed here
//! from 4x4 matrices, never read back from the kernel.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::fixture;
use crate::p2_gauges::{
    BASE_HEIGHT, BASE_WIDTH, Parts, TOP_HEIGHT, block, body_of, parts, pinned, seat, seat_on,
};

use editor_core::pose::PoseRole;
use editor_core::{
    Dimension, DocEdit, DocRef, DocumentId, EvalOptions, Evaluation, Formula, Frame, FreeValue,
    FreeVar, Node, Operand, ParamBox, ProfileDoc, RecipeNodeId, Space, Target, VarName,
    WorldRead, evaluate,
};
use fixture::resolver::with_resolver;
use fixture::{ang, head, insert, len, run, solve, step};
use geom_core::{Bounds, Interval, Tol};
use topo::Body;

fn lift() -> VarName {
    VarName::from_static("lift")
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

fn declare(doc: ProfileDoc, name: VarName, v: f64, dim: Dimension) -> ProfileDoc {
    step(
        doc,
        DocEdit::DeclareVar {
            name,
            def: editor_core::VarDecl::Free(FreeVar::continuous(dim, v)),
        },
    )
    .0
}

fn set_value(doc: ProfileDoc, name: VarName, v: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetVarValue {
            var: name.into(),
            value: FreeValue::Continuous(v),
        },
    )
    .0
}

fn turn() -> VarName {
    VarName::from_static("spin")
}

/// A document declaring `lift` = 2 and `spin` = 0.25.
fn chain(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let doc = declare(doc, lift(), 2.0, Dimension::Length);
    declare(doc, turn(), 0.25, Dimension::Angle)
}

/// **An instance of `part` pinned to the world at the parametric pose
/// [`chain_frame`] composes, onto `root_off`**: the pin at `root_off`
/// over its geometry, its world read then spun `spin` about the world's
/// z, slid `lift` along it, spun 0.3 about the world's x and slid 5
/// along it — each move about the world's own axes, so each composes on
/// the left: `G0 ∘ G1(lift, spin) ∘ root_off`.
fn pinned_chain(
    doc: ProfileDoc,
    part: DocRef,
    root_off: &M,
    o: &EvalOptions,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, id) = insert(doc, fixture::mated_instance(part));
    let copy = Operand::Var(doc.output(id, 0).expect("an instance defines its copies"));
    let mut pin = editor_core::pin_at(&doc, &copy, &root_off.frame(), o, Tol::witness())
        .unwrap_or_else(|refusal| panic!("the copy pins over its geometry: {refusal}"));
    let Target::World(world) = pin.to else {
        unreachable!("a pin reads the world")
    };
    let axis = WorldRead::through;
    pin.to = Target::World(
        world
            .spin(axis(PoseRole::Z), Formula::named(turn(), Dimension::Angle))
            .slide(axis(PoseRole::Z), Formula::named(lift(), Dimension::Length))
            .spin(axis(PoseRole::X), ang(0.3))
            .slide(axis(PoseRole::X), len(5.0)),
    );
    (fixture::constrain_at(doc, id, pin).0, id)
}

fn chain_frame(lift: f64, turn: f64) -> M {
    let g0 = M::tr([5.0, 0.0, 0.0]).c(&M::rot([1.0, 0.0, 0.0], 0.3));
    // Rigid step: translation after rotation (rotate, then translate).
    let g1 = M::tr([0.0, 0.0, lift]).c(&M::rot([0.0, 0.0, 1.0], turn));
    g0.c(&g1)
}

/// The seat's representative B: the rest with zero spin keeps the top's
/// own axes on the base's, and the two slides put its origin at
/// `(1, 1)` on the base's top. Pinned against a control with the base
/// at the identity in
/// `the_seat_relation_the_rows_compose_with_is_the_solves`.
fn seat_rep() -> M {
    M::tr([1.0, 1.0, BASE_HEIGHT])
}

/// **The seat's relative pose, the one input the rows below take from
/// the kernel rather than compose**: a top mated onto a base at the world
/// origin sits at the composed seat to 1e-15.
#[test]
fn the_seat_relation_the_rows_compose_with_is_the_solves() {
    let p = parts("r2-control");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r2-control"), Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, Node::instantiate_part(p.base), &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = seat(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    let poses = solve(&doc, &o, Tol::witness());
    let got = M::of(&poses.placement(&doc, top).unwrap());
    assert!(got.dist(&seat_rep()) <= 1e-15, "{got:?}");
}

/// **A seat under a two-parameter world pin against an independent
/// composition**: the base pinned to the world at a pose two parameters
/// drive composed with a turned literal, the top seated on it, and a
/// copy placed against the seated top at a turned lift in its own
/// coordinates, in f64 and in the interval lane at two parameter values.
#[test]
fn a_seat_under_a_two_parameter_world_pin_poses_as_composed() {
    let p = parts("r2-afb-x");
    let o = p.opts();
    let doc = chain("r2-afb-x");
    let root_off = M::tr([1.0, 2.0, 0.0]).c(&M::rot([0.0, 1.0, 0.0], 0.2));
    let (doc, base) = pinned_chain(doc, p.base, &root_off, &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let tm = M::tr([0.0, 0.0, 10.0]).c(&M::rot([1.0, 0.0, 0.0], 0.4));
    let at = Frame::rotate_then_translate([1.0, 0.0, 0.0], 0.4, [0.0, 0.0, 10.0], fixture::band())
        .expect("a literal axis");
    let (doc, t) = crate::p2_gauges::copy_against(doc, top, &at, &o);
    let (doc, _) = seat(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    let top_pts = part_points("r2-afb-x-top", 1.0, TOP_HEIGHT);
    let base_pts = part_points("r2-afb-x-base", BASE_WIDTH, BASE_HEIGHT);
    for (lv, tv) in [(2.0, 0.25), (7.0, -1.1)] {
        let doc = set_value(set_value(doc.clone(), lift(), lv), turn(), tv);
        let f = chain_frame(lv, tv).c(&root_off);
        let w_top = f.c(&seat_rep());
        let poses = solve(&doc, &o, Tol::witness());
        assert_eq!(poses.fault(top), None, "the seat places the top");
        let got = M::of(&poses.placement(&doc, top).expect("placed"));
        assert!(
            got.dist(&w_top) <= 1e-12,
            "DEFECT: top world pose {got:?} vs independent {w_top:?} at lift {lv} turn {tv}"
        );
        let got_b = M::of(&poses.placement(&doc, base).expect("placed"));
        assert!(got_b.dist(&f) <= 1e-12, "DEFECT: base pose");
        let ev = run(&doc, &o);
        check_body_f64(&ev, top, &top_pts, &w_top, "f64 top");
        check_body_f64(&ev, t, &top_pts, &w_top.c(&tm), "f64 placed copy of the top");
        check_body_f64(&ev, base, &base_pts, &f, "f64 base");
        let lane = evaluate::<Interval>(
            &doc,
            None,
            &editor_core::CancelToken::new(),
            &o,
            Tol::witness(),
        );
        // At ε = 1e-12 a copy pinned over its geometry may escalate.
        crate::p2_gauges::encloses_or_escalates(&lane, top, || {
            check_body_interval(&lane, top, &top_pts, &w_top, "interval top");
        });
        crate::p2_gauges::encloses_or_escalates(&lane, t, || {
            check_body_interval(&lane, t, &top_pts, &w_top.c(&tm), "interval t");
        });
        crate::p2_gauges::encloses_or_escalates(&lane, base, || {
            check_body_interval(&lane, base, &base_pts, &f, "interval base");
        });
    }
}

// ---- a constraint moves a loose space ----

/// **"Mate a1 to b" moves a1's whole loose space onto b and leaves b
/// where it is**: a1 (a top) with a2 stacked on it, both loose — one
/// space rooted at a1 — and b (the base) pinned to the world at
/// `[20, 0, 0]`. The constraint in a1's bundle puts a1 on b, a2 follows
/// it into the world, and b does not move.
#[test]
fn mate_a_to_b_moves_a_whole_loose_space_and_leaves_b_where_it_is() {
    let p = parts("r2-door2");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r2-door2"), Tol::witness());
    let (doc, a1) = insert(doc, fixture::mated_instance(p.top));
    let (doc, a2) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = seat_on(
        doc,
        a2,
        head(p.top_cap(a2)),
        head(p.top_upper_cap(a1)),
        [0.0, 0.0],
    );
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.space(a2), Some(Space::Loose { root: a1 }));
    let a2_in_a1 = poses.relative(a2).expect("a2 sits in a1's space");
    let (doc, b) = pinned(doc, p.base, crate::p2_gauges::literal([20.0, 0.0, 0.0]), &o);
    let (doc, m) = seat(doc, a1, head(p.top_cap(a1)), head(p.base_cap(b)));
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.fault(m.placement), None, "the mate places");
    assert_eq!(
        poses.fault(b),
        None,
        "DEFECT: the target faulted after 'mate a1 to b'"
    );
    let b_at = poses.placement(&doc, b).expect("b placed").translation;
    assert!(
        b_at.iter()
            .zip([20.0, 0.0, 0.0])
            .all(|(x, y)| (x - y).abs() < 1e-12),
        "DEFECT: b moved: {b_at:?}"
    );
    let a1_at = poses.placement(&doc, a1).expect("a1 placed");
    assert!(
        (a1_at.translation[0] - 21.0).abs() < 1e-12 && (a1_at.translation[1] - 1.0).abs() < 1e-12,
        "DEFECT: a1 not moved onto b: {a1_at:?}"
    );
    assert_eq!(
        poses.space(a2),
        Some(Space::World),
        "a2 follows a1 into the world"
    );
    let a2_at = M::of(&poses.placement(&doc, a2).expect("a2 placed"));
    let want = M::of(&a1_at).c(&M::of(&a2_in_a1));
    assert!(
        a2_at.dist(&want) <= 1e-12,
        "DEFECT: a2 left its seat on a1: {a2_at:?} vs {want:?}"
    );
}

// ---- loose spaces ----

/// **The public clearance door refuses across spaces**: a world base
/// and a loose top. The two selections live in different spaces, so the
/// door refuses `AcrossSpaces` naming both — before it reads either
/// body, which here has no resolver to build.
#[test]
fn the_clearance_door_refuses_two_selections_in_different_spaces() {
    use editor_core::clearance::{
        ClearanceRefusal, ClearanceVerdict, Selection, SelectionRefusal, clearance,
    };
    let p = parts("r2-clear");
    let doc = ProfileDoc::empty(DocumentId::derive("r2-clear"), Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, Node::instantiate_part(p.base), &p.opts());
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
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
            SelectionRefusal::AcrossSpaces { a, b },
        )) => assert_eq!((*a, *b), (Space::World, Space::Loose { root: top })),
        other => panic!("clearance across spaces refuses typed: {other:?}"),
    }
}

/// **The at-rest gate checks loose spaces regardless of the world**: a
/// loose top whose copy only an in-space measure reads holds nothing to
/// check against, so the gate certifies the world. With the world's base
/// deleted, the loose top is all the document holds: the gather refuses
/// `EmptyProduct`, naming it loose.
#[test]
fn the_gate_checks_loose_spaces_whatever_the_world_holds() {
    let p = parts("r2-gate");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r2-gate"), Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, Node::instantiate_part(p.base), &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc_m, meas) = crate::fixture::measure_node(
        &doc,
        editor_core::MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            editor_core::SitedRef::at_mint(p.top_cap(top)),
            editor_core::SitedRef::at_mint(p.top_upper_cap(top)),
        ],
    );
    let ev = run(&doc_m, &o);
    assert!(ev.value(meas).is_some(), "{:?}", ev.node_error(meas));
    editor_core::assemble(&doc_m, &ev, Tol::witness())
        .expect("a loose space with nothing to compare holds nothing to check");
    // The base goes: the loose top is all the document holds.
    let (alone, _) = step(doc, DocEdit::DeleteNode { id: base });
    let ev = run(&alone, &o);
    match editor_core::assemble(&alone, &ev, Tol::witness()) {
        Err(editor_core::AssemblyError::Product(e)) => match *e {
            editor_core::ProductError::EmptyProduct { ref loose, .. } => {
                assert_eq!(loose, &vec![top]);
                assert!(e.to_string().contains("loose"), "{e}");
            }
            ref other => panic!("the gather names the loose copy: {other:?}"),
        },
        other => panic!("a loose copy alone refuses at the gather: {other:?}"),
    }
}

/// **A loose copy below does not cross the document seam**: a
/// sub-assembly holding a pinned base and a loose top crosses as its
/// world product (A9: the top is not part of that body), so the outer
/// product holds the base's material alone and the outer gate certifies
/// it.
#[test]
fn a_loose_copy_below_stays_out_of_the_outer_product() {
    let p = parts("r2-seam");
    let sub = ProfileDoc::empty(DocumentId::derive("r2-seam-sub"), Tol::witness());
    let o = p.opts();
    let (sub, base) = fixture::insert_pinned(sub, Node::instantiate_part(p.base), &o);
    let (sub, top) = insert(sub, fixture::mated_instance(p.top));
    let ev_sub = run(&sub, &o);
    assert_eq!(ev_sub.space(top), Space::Loose { root: top });
    let mut store = p.store.clone();
    let sub_ref = store.insert(sub.clone(), Tol::witness());
    let outer = ProfileDoc::empty(DocumentId::derive("r2-seam-outer"), Tol::witness());
    let (outer, _) = fixture::insert_pinned(
        outer,
        Node::instantiate_part(sub_ref),
        &with_resolver(store.clone()),
    );
    let ev = run(&outer, &with_resolver(store));
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
        &fixture::with_placements(doc, &ids.iter().copied().collect()),
        DocumentId::derive(label),
        Tol::witness(),
        p.opts().resolver.as_ref(),
    )
}

/// **Inline of split on the verbatim shape** (a cut of two pinned bases
/// and a top seated on one of them) returns every instance's world pose.
#[test]
fn inline_of_a_verbatim_split_returns_every_world_pose() {
    let p = parts("r2-rt");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r2-rt"), Tol::witness());
    let (doc, b1) = pinned(doc, p.base, crate::p2_gauges::literal([4.0, 0.0, 0.0]), &o);
    let (doc, t1) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = seat(doc, t1, head(p.top_cap(t1)), head(p.base_cap(b1)));
    let (doc, b2) = pinned(
        doc,
        p.base,
        Frame::rotate_then_translate([0.0, 0.0, 1.0], 0.7, [-9.0, 2.0, 0.0], fixture::band())
            .unwrap(),
        &o,
    );
    let before: Vec<M> = [b1, t1, b2]
        .iter()
        .map(|&i| M::of(&solve(&doc, &o, Tol::witness()).placement(&doc, i).unwrap()))
        .collect();
    let out = split_of(&p, &doc, &[b1, t1, b2], "r2-rt-part").expect("verbatim");
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

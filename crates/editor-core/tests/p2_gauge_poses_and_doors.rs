//! **EDIT-PLACEMENT P2 — world poses against an independent
//! composition, and the doors across spaces** (ASSEMBLY.md A4, A9,
//! A11 (2)–(5); D10, Spaces): a seat under a world mate whose pose two
//! parameters drive, and a copy stacked on the seated copy, in f64 and in
//! the interval lane; a constraint that moves a loose space onto the
//! world; the clearance door, the at-rest gate and the document seam
//! across spaces; inline of split. The expected poses are composed here
//! from 4x4 matrices, never read back from the kernel.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::fixture;
use crate::p2_gauges::{
    BASE_HEIGHT, BASE_WIDTH, Parts, TOP_HEIGHT, block, body_of, parts, pinned, seat,
};

use editor_core::{
    Dimension, DocEdit, DocRef, DocumentId, EvalOptions, Evaluation, Formula, Frame, FreeValue,
    FreeVar, ParamBox, ProfileDoc, RecipeNodeId, VarName, evaluate,
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

/// **An instance of `part` placed in the world at the parametric pose
/// [`chain_frame`] composes onto [`root_off`]**, by values alone (D10
/// `[placement.values]`): the chain of spins `0.2` about `y`, `spin`
/// about `z` and `0.3` about `x`, in that order, and the slides that put
/// its origin where `G0 ∘ G1(lift, spin) ∘ root_off` does — written as
/// the formulas over `lift` and `spin` that motion means, so both keep
/// driving the copy.
fn pinned_chain(doc: ProfileDoc, part: DocRef) -> (ProfileDoc, RecipeNodeId) {
    use editor_core::{Coord, TargetAxis};
    let (doc, id) = insert(doc, fixture::mated_instance(part));
    let lift = || Formula::named(lift(), Dimension::Length);
    let spin = || Formula::named(turn(), Dimension::Angle);
    let cos = || Formula::cos(spin()).expect("an angle's cosine");
    let sin = || Formula::sin(spin()).expect("an angle's sine");
    let times = |k: f64, f: Formula| Formula::mul(fixture::scl(k), f).expect("a product");
    let plus = |a: Formula, b: Formula| Formula::add(a, b).expect("a sum");
    // root_off's origin (1, 2, 0) turned by `spin` about z, lifted.
    let at = |k: f64, f: Formula| Formula::mul(len(k), f).expect("a length");
    let ax = plus(at(1.0, cos()), at(-2.0, sin()));
    let ay = plus(at(1.0, sin()), at(2.0, cos()));
    let (c, s) = (0.3_f64.cos(), 0.3_f64.sin());
    let values = vec![
        (Coord::Spin(Some(TargetAxis::Y)), ang(0.2)),
        (Coord::Spin(Some(TargetAxis::Z)), spin()),
        (Coord::Spin(Some(TargetAxis::X)), ang(0.3)),
        (Coord::Slide(TargetAxis::X), plus(len(5.0), ax)),
        (
            Coord::Slide(TargetAxis::Y),
            plus(times(c, ay.clone()), times(-s, lift())),
        ),
        (
            Coord::Slide(TargetAxis::Z),
            plus(times(s, ay), times(c, lift())),
        ),
    ];
    let placed = editor_core::Placed::unmated(values);
    (fixture::written(doc, id, placed), id)
}

/// The turned literal the parametric pose composes onto.
fn root_off() -> M {
    M::tr([1.0, 2.0, 0.0]).c(&M::rot([0.0, 1.0, 0.0], 0.2))
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

/// **Where a copy seated on a placed base stands, by D′2's chart**
/// (`docs/INTENT-STAGE3-SPEC.md` §5′.2), written out independently of
/// the solve: the target space's axes carried by the least turn from
/// the copy's mated `−z` onto the mated direction, the base's upper
/// cap's normal `n = (a, b, c)` reversed — the turn of `z` onto `n`,
/// `x' = (1 − a²/(1 + c),
/// −ab/(1 + c), −a)` and `y' = n × x'`; the copy's own axes land on them
/// (no spin), its origin `at` along `x'` and `y'` from the space's
/// origin and on the cap's plane, `height` up the base, along `n`.
fn space_seat(base: &M, height: f64, at: [f64; 2]) -> M {
    let n = [base.r[0][2], base.r[1][2], base.r[2][2]];
    let on_cap: [f64; 3] = std::array::from_fn(|i| base.r[i][2] * height + base.t[i]);
    let (a, b, c) = (n[0], n[1], n[2]);
    let one_plus_c = if c < 0.0 {
        (a * a + b * b) / (1.0 - c)
    } else {
        1.0 + c
    };
    let x = [1.0 - a * a / one_plus_c, -a * b / one_plus_c, -a];
    let y = [
        n[1] * x[2] - n[2] * x[1],
        n[2] * x[0] - n[0] * x[2],
        n[0] * x[1] - n[1] * x[0],
    ];
    let along: f64 = (0..3).map(|i| on_cap[i] * n[i]).sum();
    M {
        r: std::array::from_fn(|i| [x[i], y[i], n[i]]),
        t: std::array::from_fn(|i| at[0] * x[i] + at[1] * y[i] + along * n[i]),
    }
}

/// **The seat's relative pose, the one input the rows below take from
/// the kernel rather than compose**: a top mated onto a base at the world
/// origin sits at the composed seat to 1e-15.
#[test]
fn the_seat_relation_the_rows_compose_with_is_the_solves() {
    let p = parts("r2-control");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r2-control"), Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, crate::fixture::mated_instance(p.base), &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = seat(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    let poses = solve(&doc, &o, Tol::witness());
    let got = M::of(&poses.placement(&doc, top).unwrap());
    assert!(got.dist(&seat_rep()) <= 1e-15, "{got:?}");
}

/// **A seat under a two-parameter placement against an independent
/// composition**: the base placed in the world at a pose two parameters
/// drive composed with a turned literal, the top seated on it, and a
/// second top stacked on the seated one, each seat by the chart of the
/// space it is placed into ([`space_seat`]), in f64 and in the interval
/// lane at two parameter values.
#[test]
fn a_seat_under_a_two_parameter_world_pin_poses_as_composed() {
    let p = parts("r2-afb-x");
    let o = p.opts();
    let doc = chain("r2-afb-x");
    let root_off = root_off();
    let (doc, base) = pinned_chain(doc, p.base);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, t) = crate::p2_gauges::stacked(doc, p.top, p.top_body, head(p.top_upper_cap(top)));
    let (doc, _) = seat(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    let top_pts = part_points("r2-afb-x-top", 1.0, TOP_HEIGHT);
    let base_pts = part_points("r2-afb-x-base", BASE_WIDTH, BASE_HEIGHT);
    for (lv, tv) in [(2.0, 0.25), (7.0, -1.1)] {
        let doc = set_value(set_value(doc.clone(), lift(), lv), turn(), tv);
        let f = chain_frame(lv, tv).c(&root_off);
        // Each seat's values chart on the world, carried by the least
        // turn onto the cap it rests on: a seat does not ride with its base in the cap's plane.
        let w_top = space_seat(&f, BASE_HEIGHT, [1.0, 1.0]);
        let w_t = space_seat(&w_top, TOP_HEIGHT, [0.0, 0.0]);
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
        check_body_f64(&ev, t, &top_pts, &w_t, "f64 placed copy of the top");
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
            check_body_interval(&lane, t, &top_pts, &w_t, "interval t");
        });
        crate::p2_gauges::encloses_or_escalates(&lane, base, || {
            check_body_interval(&lane, base, &base_pts, &f, "interval base");
        });
    }
}

// ---- spaces ----

/// **The public clearance door refuses across spaces**: a world base
/// and a top in a space of its own. The two selections live in different spaces, so the
/// door refuses `AcrossSpaces` naming both — before it reads either
/// body, which here has no resolver to build.
#[test]
fn the_clearance_door_refuses_two_selections_in_different_spaces() {
    use crate::fixture::clearance;
    use editor_core::clearance::{ClearanceRefusal, ClearanceVerdict, Selection, SelectionRefusal};
    let p = parts("r2-clear");
    let doc = ProfileDoc::empty(DocumentId::derive("r2-clear"), Tol::witness());
    let (doc, base) =
        fixture::insert_pinned(doc, crate::fixture::mated_instance(p.base), &p.opts());
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, space) = fixture::into_new_space(doc, top);
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
        )) => assert_eq!((*a, *b), (editor_core::VarId::WORLD, space)),
        other => panic!("clearance across spaces refuses typed: {other:?}"),
    }
}

/// **The at-rest gate checks the world**: a top in a space of its own,
/// whose copy only an in-space measure reads, is no part of the world,
/// so the gate certifies the world. With the world's base deleted, the
/// world is empty: the gather refuses `EmptyProduct`.
#[test]
fn the_gate_checks_the_world_whatever_another_space_holds() {
    let p = parts("r2-gate");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r2-gate"), Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, crate::fixture::mated_instance(p.base), &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = fixture::into_new_space(doc, top);
    let (doc_m, meas) = crate::fixture::measure_node(
        &doc,
        editor_core::MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            crate::fixture::sited_ref(p.top_cap(top)),
            crate::fixture::sited_ref(p.top_upper_cap(top)),
        ],
    );
    let ev = run(&doc_m, &o);
    assert!(ev.value(meas).is_some(), "{:?}", ev.node_error(meas));
    crate::fixture::could_look(&doc_m, &ev)
        .expect("the world certifies whatever another space holds");
    // The base goes: the product reads a body that is gone.
    let (alone, _) = step(doc, DocEdit::DeleteNode { id: base });
    let ev = run(&alone, &o);
    match editor_core::product_recorded(&alone, &ev, Tol::witness()).map(|_| ()) {
        Err(editor_core::ProductError::StrandedProduct { .. }) => {}
        other => panic!("a stranded product refuses at the gather: {other:?}"),
    }
}

/// **A copy in another space below does not cross the document seam**:
/// a sub-assembly holding a pinned base and a top in a space of its own
/// crosses as its world product (A9: the top is not part of that body),
/// so the outer product holds the base's material alone and the outer
/// gate certifies it.
#[test]
fn a_copy_in_another_space_below_stays_out_of_the_outer_product() {
    let p = parts("r2-seam");
    let sub = ProfileDoc::empty(DocumentId::derive("r2-seam-sub"), Tol::witness());
    let o = p.opts();
    let (sub, base) = fixture::insert_pinned(sub, crate::fixture::mated_instance(p.base), &o);
    let (sub, top) = insert(sub, fixture::mated_instance(p.top));
    let (sub, space) = fixture::into_new_space(sub, top);
    assert_eq!(sub.node_space(top), Some(space));
    let ev_sub = run(&sub, &o);
    let mut store = p.store.clone();
    let sub = crate::fixture::assemble_world_copies(sub);
    let sub_ref = store.insert(sub.clone(), Tol::witness());
    let outer = ProfileDoc::empty(DocumentId::derive("r2-seam-outer"), Tol::witness());
    let (outer, _) = fixture::insert_pinned(
        outer,
        crate::fixture::mated_instance(sub_ref),
        &with_resolver(store.clone()),
    );
    let ev = run(&outer, &with_resolver(store));
    let vol = |b: &Body<f64>| {
        topo::test_support::pinned_volume(
            "mass",
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(b),
                Tol::witness(),
            ),
            Tol::witness(),
        )
        .volume
    };
    let gathered =
        crate::fixture::product_in_world(&outer, &ev, Tol::witness()).expect("the world gathers");
    assert!(
        (vol(&gathered) - vol(&body_of(&ev_sub, base))).abs() < 1e-9,
        "the outer product is the sub-assembly's world"
    );
    crate::fixture::could_look(&outer, &ev).expect("the outer product is at rest");
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
/// and a top seated on one of them), flattened, returns every instance's
/// world pose.
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
    let flat = fixture::round_trip::flat(&back);
    let po = with_resolver(store);
    let poses = solve(&flat, &po, Tol::witness());
    for (i, old) in [b1, t1, b2].iter().enumerate() {
        let new = back.node_map[&out.node_map[old]];
        let got = M::of(&poses.placement(&flat, new).unwrap());
        assert!(
            got.dist(&before[i]) <= 1e-12,
            "DEFECT: {old:?} moved over inline(split): {got:?} vs {:?}",
            before[i]
        );
    }
}

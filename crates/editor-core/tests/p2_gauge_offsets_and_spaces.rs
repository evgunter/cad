//! **EDIT-PLACEMENT P2 over placements — poses, the constraint door
//! and the spaces** (ASSEMBLY.md A4, A9, A11; D10, Spaces): a seat
//! under a parametric world pin, a loose stack joining the world
//! through one seat, split and
//! inline over pinned and mate-placed instances, and the readers that
//! must not compare a loose copy with anything outside its space — the
//! product being exactly the copies whose space reaches the world. Each
//! row is composed against the two literal block parts `p2_gauges`
//! seats, and the poses against an independent 4x4 composition written
//! here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::ExtrudeSide;
use std::sync::Arc;

use crate::fixture;
use crate::p2_gauges::{body_of, pinned, seat, seat_edits, seat_on, set_pin};

use editor_core::pose::PoseRole;
use editor_core::{
    CapEnd, Dimension, DocEdit, DocRef, DocumentId, EvalOptions, Evaluation, Formula, Frame,
    FreeValue, FreeVar, Node, NodeErrorKind, Operand, ProfileDoc, RecipeNodeId, Space, StableName,
    Target, VarName, WorldRead, evaluate,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{ang, head, insert, len, on_frame, run, solve, step, xform};
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
            profile: profile.into(),
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

fn lift() -> VarName {
    VarName::from_static("lift")
}

fn declare_lift(doc: ProfileDoc, value: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::DeclareVar {
            name: lift(),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, value)),
        },
    )
    .0
}

fn set_lift(doc: ProfileDoc, value: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetVarValue {
            var: lift().into(),
            value: FreeValue::Continuous(value),
        },
    )
    .0
}

/// The seat relation, read off a control solve (base at the world
/// origin, the top mated onto it): the one input this probe does not
/// compose itself.
fn seat_rel(p: &Parts) -> M4 {
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-seat-control"), Tol::witness());
    let (doc, b) = fixture::insert_pinned(doc, Node::instantiate_part(p.base), &o);
    let (doc, t) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = seat(doc, t, head(p.top_cap(t)), head(p.base_cap(b)));
    let poses = solve(&doc, &o, Tol::witness());
    of_frame(&poses.placement(&doc, t).unwrap())
}

fn band() -> geom_core::Band {
    fixture::band()
}

/// **`placement` pinned to the world at `then`, turned `angle` about
/// the world's z and slid the document's `lift` along the world's axis
/// `index`**: the pin at `then` over its geometry, the two world moves
/// appended — each about the world's own axes, so each composes on the
/// left.
fn pin_lifted(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    index: usize,
    angle: f64,
    then: &Frame,
    o: &EvalOptions,
) -> ProfileDoc {
    let copy = Operand::Var(doc.output(placement, 0).expect("an instance defines its copies"));
    let mut pin = editor_core::pin_at(&doc, &copy, then, o, Tol::witness())
        .unwrap_or_else(|refusal| panic!("the copy pins over its geometry: {refusal}"));
    let Target::World(world) = pin.to else {
        unreachable!("a pin reads the world")
    };
    let world = if angle == 0.0 {
        world
    } else {
        world.spin(WorldRead::through(PoseRole::Z), ang(angle))
    };
    let along = [PoseRole::X, PoseRole::Y, PoseRole::Z][index];
    pin.to = Target::World(world.slide(
        WorldRead::through(along),
        Formula::named(lift(), Dimension::Length),
    ));
    fixture::constrain_at(doc, placement, pin).0
}

/// [`pin_lifted`] over a new instance of `part`.
fn pinned_lifted(
    doc: ProfileDoc,
    part: DocRef,
    index: usize,
    angle: f64,
    then: &Frame,
    o: &EvalOptions,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, id) = insert(doc, fixture::mated_instance(part));
    (pin_lifted(doc, id, index, angle, then, o), id)
}

/// **A seat under a world pin whose pose composes a parametric lift, a
/// fixed turn and a rotated offset**, and a Transform of the pinned
/// base, against an independent 4x4 composition in f64 and in the
/// interval lane at two values of the driving parameter.
#[test]
fn a_seat_under_a_parametric_world_pin_poses_as_composed() {
    let p = parts("r1-afb");
    let o = p.opts();
    let rel = seat_rel(&p);

    let doc = ProfileDoc::empty(DocumentId::derive("r1-afb"), Tol::witness());
    let doc = declare_lift(doc, 2.0);
    let gin_frame =
        Frame::rotate_then_translate([1.0, 0.0, 0.0], 0.4, [5.0, 0.0, 0.0], band()).unwrap();
    let off_frame =
        Frame::rotate_then_translate([0.0, 0.0, 1.0], 0.3, [1.0, 2.0, 0.0], band()).unwrap();
    let (doc, base) = pinned_lifted(
        doc,
        p.base,
        2,
        0.25,
        &gin_frame.compose(&off_frame),
        &o,
    );
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    // A transform over the pinned base, reading its world copy.
    let (doc, tp) = insert(doc, xform(base, [0.0, 0.0, 4.0], [1.0, 0.0, 0.0], 0.5));
    let (doc, mate) = seat(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));

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
        let w_top = mul(&w_base, &rel);
        (w_base, mul(&t_p, &w_base), w_top)
    };

    for liftv in [2.0, 7.0] {
        let doc = set_lift(doc.clone(), liftv);
        let (w_base, w_moved, w_top) = expect(liftv);
        let poses = solve(&doc, &o, Tol::witness());
        assert_eq!(poses.fault(mate.placement), None);
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
        assert_points(&pb, &body_of(&ev, tp), &w_moved, "moved base f64");
        assert_points(&pt, &body_of(&ev, top), &w_top, "top f64");
        let lane = evaluate::<Interval>(
            &doc,
            None,
            &editor_core::CancelToken::new(),
            &o,
            Tol::witness(),
        );
        assert_encloses(&pb, &body_of(&lane, base), &w_base, "base interval");
        assert_encloses(&pb, &body_of(&lane, tp), &w_moved, "moved base interval");
        assert_encloses(&pt, &body_of(&lane, top), &w_top, "top interval");
    }
}

/// **One seat puts a whole loose stack in the world, and the target
/// stays put.** p (top) with q (top) stacked on it is a loose stack,
/// both in p's space; r (base) is pinned at x = 30. Seating p onto r —
/// a rest and its values — moves p's stack into the world: r does not
/// move, nothing faults, the edits record nothing, and a replay reaches
/// the same document.
#[test]
fn one_seat_puts_a_loose_stack_in_the_world_and_the_target_stays() {
    let p = parts("r1-door-order");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-door-order"), Tol::witness());
    let (doc, pp) = insert(doc, fixture::mated_instance(p.top));
    let (doc, q) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = seat_on(
        doc,
        q,
        head(p.top_cap(q)),
        head(p.top_upper_cap(pp)),
        [0.0, 0.0],
    );
    let poses = solve(&doc, &o, Tol::witness());
    for id in [pp, q] {
        assert_eq!(
            poses.space(id),
            Some(Space::Loose { root: pp }),
            "the stack is loose"
        );
    }
    let (doc, r) = pinned(doc, p.base, Frame::translation([30.0, 0.0, 0.0]), &o);
    let (seated, _, edits) = seat_edits(
        doc.clone(),
        pp,
        head(p.top_cap(pp)),
        head(p.base_cap(r)),
        [1.0, 1.0],
    );
    let applied = edits.iter().fold(doc.clone(), |doc, edit| {
        let applied = editor_core::apply(&doc, edit, Tol::witness(), &editor_core::RefusingReach)
            .expect("the seat is admitted");
        assert!(applied.maintenance.is_empty(), "{:?}", applied.maintenance);
        applied.doc
    });
    assert!(applied.bit_eq(&seated));
    let poses = solve(&applied, &o, Tol::witness());
    for id in [pp, q, r] {
        assert_eq!(poses.fault(id), None, "node {} stands", id.0);
        assert_eq!(
            poses.space(id),
            Some(Space::World),
            "node {} is in the world",
            id.0
        );
    }
    let at = poses.placement(&applied, r).expect("r placed").translation;
    assert!(
        at.iter()
            .zip([30.0, 0.0, 0.0])
            .all(|(x, y)| (x - y).abs() < 1e-12),
        "r does not move: {at:?}"
    );
    // Replay re-applies the edits alone and reaches the same document.
    let replayed = edits.iter().fold(doc, |doc, edit| {
        editor_core::apply_replayed(&doc, edit, Tol::witness())
            .expect("replays")
            .doc
    });
    assert!(replayed.bit_eq(&applied), "replay is the edit");
}

/// **The memo does not carry a measure across a space change**: with
/// the top's world pin removed and the prior evaluation handed in, the
/// measure must re-evaluate, and then refuses across the two spaces.
#[test]
fn a_measure_reused_across_a_pin_removal_refuses_across_spaces() {
    let p = parts("r1-measure");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-measure"), Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, Node::instantiate_part(p.base), &o);
    let (doc, top) = pinned(doc, p.top, Frame::translation([0.0, 0.0, 1.0]), &o);
    let measure = Node::Measure {
        primitive: editor_core::MeasurePrimitive::Distance {
            a: editor_core::SitedRef::at_mint(p.base_cap(base)).into(),
            b: editor_core::SitedRef::at_mint(p.top_cap(top)).into(),
        },
    };
    let (doc, m) = insert(doc, measure);
    let ev = run(&doc, &o);
    assert!(ev.value(m).is_some());
    // Remove the top's pin: it is loose; reuse the prior.
    let un = set_pin(doc, top, None, &o);
    let ev2 = evaluate::<f64>(
        &un,
        Some(&ev),
        &editor_core::CancelToken::new(),
        &o,
        Tol::witness(),
    );
    let e = ev2.node_error(m);
    assert!(
        matches!(
            e.map(|e| &e.kind),
            Some(NodeErrorKind::SpaceMismatch { .. })
        ),
        "{e:?}"
    );
}

/// **Every saved document in the tree loads** (or, at a process ε
/// other than the one it records, refuses at D4's seam), and each
/// frozen older-build golden refuses typed.
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

fn same_bounds(before: ([f64; 3], [f64; 3]), after: ([f64; 3], [f64; 3]), what: &str) {
    for k in 0..3 {
        assert!(
            (before.0[k] - after.0[k]).abs() < 1e-9 && (before.1[k] - after.1[k]).abs() < 1e-9,
            "{what}: {before:?} -> {after:?}"
        );
    }
}

/// **A cut of a pinned instance and placed plain geometry splits and
/// moves nothing** (A4): the instance and the plain body's placement
/// both go into the part, and the product of the remainder over the new
/// part is where the source's was.
#[test]
fn a_cut_of_a_pinned_instance_and_plain_geometry_moves_nothing() {
    let p = parts("r1-split-plain");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-split-plain"), Tol::witness());
    let (doc, x) = pinned(doc, p.base, Frame::translation([0.0, 10.0, 0.0]), &o);
    let before_ids: std::collections::BTreeSet<RecipeNodeId> = doc.ids().iter().copied().collect();
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
            profile: prof.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, _) = fixture::place(doc, ext);
    let mut cut: std::collections::BTreeSet<RecipeNodeId> = doc
        .ids()
        .iter()
        .copied()
        .filter(|id| !before_ids.contains(id))
        .collect();
    cut.insert(x);
    let cut = fixture::with_placements(&doc, &cut);
    let before = world_bounds(&doc, &run(&doc, &o));
    let out = editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("r1-split-plain-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("the cut splits");
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let after = world_bounds(&out.remainder, &run(&out.remainder, &with_resolver(store)));
    same_bounds(before, after, "split moved the world");
}

/// **A document of loose copies alone**: the world gather refuses
/// `ProductError::EmptyProduct` naming the loose copy and the recourse,
/// and the gate still checks the loose copy's space before raising that
/// refusal — so an interference there is the gate's answer, not the
/// empty world's.
#[test]
fn a_document_of_loose_copies_alone_names_them_and_still_checks_their_space() {
    let p = parts("r1-alone");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-alone"), Tol::witness());
    let (doc, x) = insert(doc, fixture::mated_instance(p.base));
    let ev = run(&doc, &o);
    match editor_core::product(&doc, &ev, Tol::witness()) {
        Err(editor_core::ProductError::EmptyProduct { unplaced, loose }) => {
            assert!(unplaced.is_empty(), "{unplaced:?}");
            assert_eq!(loose, vec![x]);
        }
        other => panic!("the world gather names the loose copy: {other:?}"),
    }
    let said = editor_core::product(&doc, &ev, Tol::witness())
        .err()
        .map(|e| e.spoken(&doc))
        .unwrap_or_default();
    assert!(
        said.contains("Recourse:") && said.contains(&doc.spoken(x).to_string()),
        "{said}"
    );
    match editor_core::assemble(&doc, &ev, Tol::witness()) {
        Err(editor_core::AssemblyError::Product(e)) => assert!(
            matches!(*e, editor_core::ProductError::EmptyProduct { .. }),
            "{e:?}"
        ),
        other => panic!("a clean loose space raises the world's refusal: {other:?}"),
    }
    // Two tops seated at one spot on the loose base: they overlap
    // inside its space.
    let (doc, a) = insert(doc, fixture::mated_instance(p.top));
    let (doc, b) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = seat(doc, a, head(p.top_cap(a)), head(p.base_cap(x)));
    let (doc, _) = seat(doc, b, head(p.top_cap(b)), head(p.base_cap(x)));
    let ev = run(&doc, &o);
    for id in [a, b] {
        assert_eq!(ev.space(id), Space::Loose { root: x });
    }
    let own_space_refuses = |doc: &ProfileDoc, ev: &Evaluation<f64>| match editor_core::assemble(
        doc,
        ev,
        Tol::witness(),
    ) {
        Err(editor_core::AssemblyError::Space { root, .. }) => assert_eq!(root, x),
        Err(editor_core::AssemblyError::AtRest { .. }) => {}
        other => panic!("the loose space's interference is the gate's answer: {other:?}"),
    };
    own_space_refuses(&doc, &ev);
    // With pinned material far off in the world, the world gathers and
    // the gate over that product still checks the loose space: the
    // product carries it (`Product::spaces`).
    let (doc, _) = pinned(doc, p.base, Frame::translation([100.0, 0.0, 0.0]), &o);
    let ev = run(&doc, &o);
    let product =
        editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("the world gathers");
    assert_eq!(
        product.spaces.len(),
        1,
        "the product carries the loose space"
    );
    assert!(editor_core::assemble_gathered(product, Tol::witness()).is_err());
    own_space_refuses(&doc, &ev);
}

/// **A parametric world pin moves with the cut** (A4): the base's pin
/// reads `lift`, which a kept instance's pin reads too, so the cut of
/// the pair refuses `UncutVarReference` naming the parameter, the cut
/// node and the kept one, with the recourse.
#[test]
fn a_parametric_pin_read_on_both_sides_of_a_cut_refuses() {
    let p = parts("r1-promote-param");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-promote-param"), Tol::witness());
    let doc = declare_lift(doc, 2.0);
    let (doc, kept) = pinned_lifted(doc, p.base, 1, 0.0, &Frame::IDENTITY, &o);
    let (doc, base) = pinned_lifted(doc, p.base, 0, 0.0, &Frame::IDENTITY, &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = seat(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    let cut = fixture::with_placements(&doc, &[base, top].into_iter().collect());
    let err = editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("r1-promote-param-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect_err("the cut base's pin reads a kept node's parameter");
    assert!(
        matches!(&err, editor_core::SplitError::UncutVarReference { var, cut_node, kept_node }
            if var.name() == Some(&lift()) && cut_node.id() == base && kept_node.id() == kept),
        "{err:?}"
    );
    assert!(err.to_string().contains("Recourse:"), "{err}");
}

/// **The memo key carries what places a copy**: an evaluation
/// handed its prior moves an instance whose world pin's parameter moved,
/// and a mate-placed instance with its target when the target's pin
/// moved — neither changes a slot of the mate-placed instance's own,
/// so only what its key folds of its target can tell the two runs
/// apart.
#[test]
fn the_memo_moves_instances_when_their_pin_or_their_target_moves() {
    let p = parts("r1-memo");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-memo"), Tol::witness());
    let doc = declare_lift(doc, 2.0);
    let (doc, base) = pinned_lifted(doc, p.base, 2, 0.0, &Frame::IDENTITY, &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = seat(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
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
        "the pinned instance moved: {} -> {}",
        b0,
        minz(&ev2, base)
    );
    assert!(
        (minz(&ev2, top) - (t0 + 5.0)).abs() < 1e-9,
        "the mated instance moved"
    );
    let shifted = pin_lifted(
        set_pin(doc, base, None, &o),
        base,
        2,
        0.0,
        &Frame::translation([0.0, 0.0, 3.0]),
        &o,
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
        "the mated instance moved with its target: {} -> {}",
        t0,
        minz(&ev3, top)
    );
}

/// **Inline at a pose over plain geometry moves nothing**: an instance
/// pinned at `[0, 10, 0]` over a part whose one placement is a plain
/// body pinned to its world at the identity inlines with the instance's
/// pose composed into that placement's pin, and the product does not
/// move.
#[test]
fn inline_at_a_pose_over_plain_geometry_moves_nothing() {
    let p = parts("r1-inline-plain");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-inline-plain"), Tol::witness());
    let (doc, h) = pinned(doc, p.base, Frame::translation([0.0, 10.0, 0.0]), &o);
    let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(p.store.clone());
    let before = world_bounds(&doc, &run(&doc, &o));
    let out = editor_core::inline(&doc, h, &resolver, Tol::witness())
        .expect("a pinned instance over plain geometry inlines");
    let after = world_bounds(&out.doc, &run(&out.doc, &o));
    same_bounds(before, after, "nothing moved");
}

/// **Every seated copy pinned to the world where it is solved**: each
/// placement whose bundle does not read the world gets its bundle — its
/// values first, then its mates — replaced by one world pin at its
/// solved pose.
fn pinned_where_solved(doc: ProfileDoc, o: &EvalOptions) -> ProfileDoc {
    let poses = solve(&doc, o, Tol::witness());
    let placed: Vec<(RecipeNodeId, Frame)> = doc
        .placements()
        .into_iter()
        .filter(|&p| !fixture::bundle(&doc, p).is_empty() && !fixture::on_world(&doc, p))
        .filter_map(|p| Some((p, poses.placement(&doc, p).ok()?)))
        .collect();
    placed.into_iter().fold(doc, |doc, (placement, frame)| {
        let doc = fixture::bundle(&doc, placement)
            .into_iter()
            .rev()
            .fold(doc, |doc, c| {
                step(
                    doc,
                    DocEdit::RemoveConstraint {
                        placement,
                        constraint: c.id,
                    },
                )
                .0
            });
        fixture::pin(doc, placement, &frame, o)
    })
}

/// **`InlineError::UnplaceablePart`'s recourse, followed, inlines**: a
/// top mated onto a base is placed by its mate, and its part is no one
/// instance at the identity that could take that bundle, so inline
/// refuses naming the instance, with a recourse. The base sits where
/// the seat puts the top at the world origin; following the recourse —
/// pin the top where it is shown — inlines with nothing moved.
#[test]
fn the_unplaceable_part_recourse_followed_inlines_in_place() {
    let p = parts("r1-mateplaced");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-mateplaced"), Tol::witness());
    let (doc, base) = pinned(doc, p.base, to_frame(&inv(&seat_rel(&p))), &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = seat(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(p.store.clone());
    match editor_core::inline(&doc, top, &resolver, Tol::witness()) {
        Err(e @ editor_core::InlineError::UnplaceablePart { .. }) => {
            assert!(e.to_string().contains("Recourse:"), "{e}");
            let editor_core::InlineError::UnplaceablePart { instance, .. } = e else {
                unreachable!()
            };
            assert_eq!(instance.id(), top);
        }
        other => panic!("a mate-placed instance refuses typed: {other:?}"),
    }
    let before = world_bounds(&doc, &run(&doc, &o));
    let doc = pinned_where_solved(doc, &o);
    assert!(fixture::offset_of(&doc, top).is_some(), "the top is pinned");
    let out = editor_core::inline(&doc, top, &resolver, Tol::witness())
        .expect("the recourse followed, inline admits it");
    let after = world_bounds(&out.doc, &run(&out.doc, &o));
    same_bounds(before, after, "nothing moved");
}

/// **Inline at a turned pose over a part of two pinned instances moves
/// nothing** (A4): the host instance's pose composes into each inner
/// instance's world pin, and no world pose moves.
#[test]
fn an_inline_at_a_turned_pose_over_two_pinned_instances_moves_nothing() {
    let p = parts("minted-vs-hand");
    let sub = ProfileDoc::empty(DocumentId::derive("minted-vs-hand-sub"), Tol::witness());
    let po = p.opts();
    let (sub, _) = fixture::insert_pinned(sub, Node::instantiate_part(p.base), &po);
    let (sub, _) = pinned(sub, p.base, Frame::translation([10.0, 0.0, 0.0]), &po);
    let mut store = p.store.clone();
    let sub_ref = store.insert(sub, Tol::witness());
    let o = with_resolver(store.clone());
    let pose =
        Frame::rotate_then_translate([0.0, 0.0, 1.0], 0.3, [2.0, 5.0, 0.0], band()).unwrap();
    let doc = ProfileDoc::empty(DocumentId::derive("minted-vs-hand"), Tol::witness());
    let (doc, h) = pinned(doc, sub_ref, pose, &o);
    let resolver: Arc<dyn editor_core::PartResolver> = Arc::new(store);
    let before = world_bounds(&doc, &run(&doc, &o));
    let inlined = editor_core::inline(&doc, h, &resolver, Tol::witness())
        .expect("a pose over two pinned instances inlines");
    let after = world_bounds(&inlined.doc, &run(&inlined.doc, &o));
    same_bounds(before, after, "nothing moved");
}

/// **The kernel's pick orders faces within one space only**: a ray
/// offered a world target and a loose one refuses `AcrossSpaces` naming
/// both spaces, since no one ray's coordinates meet both; each space
/// offered by itself picks.
#[test]
fn a_pick_across_spaces_refuses_and_each_space_picks_by_itself() {
    let p = parts("r1-pick");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-pick"), Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, Node::instantiate_part(p.base), &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
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
    let spaces = (Space::World, Space::Loose { root: top });
    match editor_core::pick_face(&ev, &[tb, tt], &down) {
        Err(editor_core::HitTestError::AcrossSpaces { a, b }) => {
            assert!((a, b) == spaces || (b, a) == spaces, "{a:?} {b:?}");
        }
        other => panic!("a pick across spaces refuses: {other:?}"),
    }
    for (target, node) in [(tb, base), (tt, top)] {
        let hit = editor_core::pick_face(&ev, &[target], &down)
            .expect("one space picks")
            .expect("the ray meets it");
        assert_eq!(hit.node, node);
    }
}

/// **Two instances pinned to the world are one component with no mate**
/// (A9): each is fixed in the world, so their relative pose is fixed
/// too — the world couples them as a shared gauge did — and a loose
/// instance is a component of its own until a constraint relates it to
/// one of them.
#[test]
fn two_instances_pinned_to_the_world_are_one_component_with_no_mate() {
    let p = parts("r1-reading");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-reading"), Tol::witness());
    let (doc, a) = pinned(doc, p.base, Frame::translation([1.0, 1.0, 0.0]), &o);
    let (doc, b) = fixture::insert_pinned(doc, Node::instantiate_part(p.top), &o);
    let (doc, c) = insert(doc, fixture::mated_instance(p.top));
    let components = editor_core::relative_freedom_components(&doc);
    let of = |components: &[Vec<RecipeNodeId>], id| {
        components.iter().position(|comp| comp.contains(&id))
    };
    assert!(
        components.iter().any(|comp| comp == &vec![a, b]),
        "one world, one component: {components:?}"
    );
    assert!(
        components.iter().any(|comp| comp == &vec![c]),
        "a loose instance couples nothing: {components:?}"
    );
    let (related, _) = seat_on(
        doc,
        c,
        head(p.top_cap(c)),
        head(p.top_upper_cap(b)),
        [0.0, 0.0],
    );
    let components = editor_core::relative_freedom_components(&related);
    assert_eq!(
        of(&components, c),
        of(&components, b),
        "a constraint couples its copy to its target"
    );
}

/// **A touch between two copies inside a loose copy's space is reported
/// like one in the world** (D1 (iii), D10; A11 (2)). Two tops seated
/// side by side on a loose base, flush along x = 1.5 with no mate
/// between them: the space's gather records the touch as rows of the
/// two copies, and the `unproven-coincidence` check reports them against
/// the copies, read off `Product::spaces` beside the world's (empty)
/// rows. Red if the check reads the world's rows alone.
#[test]
fn a_touch_between_copies_in_a_loose_space_reaches_the_coincidence_check() {
    let p = parts("r1-space-copies");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-space-copies"), Tol::witness());
    let (doc, x) = insert(doc, fixture::mated_instance(p.base));
    let (doc, a) = insert(doc, fixture::mated_instance(p.top));
    let (doc, b) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = seat_on(doc, a, head(p.top_cap(a)), head(p.base_cap(x)), [0.5, 1.0]);
    let (doc, _) = seat_on(doc, b, head(p.top_cap(b)), head(p.base_cap(x)), [1.5, 1.0]);
    // Pinned material far off, so the world gathers and the check runs.
    let (doc, _) = pinned(doc, p.base, Frame::translation([100.0, 0.0, 0.0]), &o);
    let ev = run(&doc, &o);
    for id in [a, b] {
        assert_eq!(ev.space(id), Space::Loose { root: x });
    }
    let product =
        editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("the world gathers");
    assert!(
        product.coincidences.is_empty(),
        "{:?}",
        product.coincidences
    );
    let [space] = product.spaces.as_slice() else {
        panic!("one loose space: {:?}", product.spaces.len())
    };
    let copies = space
        .gather
        .as_ref()
        .expect("the space gathers")
        .coincidences
        .iter()
        .filter(|r| matches!(r.by, editor_core::AtRestBy::Copies(..)))
        .count();
    assert!(copies > 0, "the tops' touch is the space's rows");
    let report = editor_core::run_checks(
        &doc,
        &ev,
        &editor_core::ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("checks run");
    let reported = report
        .findings
        .iter()
        .filter(|f| {
            f.check == editor_core::CheckId::UnprovenCoincidence
                && matches!(f.subject, editor_core::FindingSubject::Copies { .. })
        })
        .count();
    assert_eq!(reported, copies, "{:?}", report.findings);
}

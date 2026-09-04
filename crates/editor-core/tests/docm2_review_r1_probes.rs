//! DOCM-2 review lane R1 — probes against the frozen head `b59b2203`.
//! Not part of the unit; each row states which claim it exercises.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::corpus;
use crate::fixture::{Recorder, ang, len, scl};

use editor_core::{
    BooleanOp, CancelToken, Datum, DocEdit, EditError, EntityKey, EntityKind, EntityRef, Entry,
    EvalOptions, Evaluation, Expr, NameTable, NamingError, Node, NodeError, NodeErrorKind,
    NodeResult, PartSelect, PatternKind, ProfileDoc, RecipeNodeId, RoleSeg, SlotId, SplitHalf,
    SplitSide, StableName, ValuePayload, all_edges, apply, evaluate,
};
use geom_core::{Affine3, Mat3, Tol, Vec3};
use topo::{Body, BooleanResult, transform_rigid};

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn part(r: &mut Recorder, of: RecipeNodeId, select: PartSelect) -> RecipeNodeId {
    r.insert(Node::Part { of, select })
}

fn half(h: SplitHalf) -> PartSelect {
    PartSelect::SplitHalf(h)
}

fn instance(i: i64) -> PartSelect {
    PartSelect::Instance(Expr::count(i))
}

fn plane(r: &mut Recorder, origin: [f64; 3], normal: [f64; 3]) -> RecipeNodeId {
    r.insert(Node::Datum(Datum::Plane {
        origin: [len(origin[0]), len(origin[1]), len(origin[2])],
        normal: [scl(normal[0]), scl(normal[1]), scl(normal[2])],
    }))
}

fn prism(r: &mut Recorder, pts: Vec<(f64, f64)>, z0: f64, dz: f64) -> RecipeNodeId {
    let p = r.profile([0.0, 0.0, z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![pts]);
    r.insert(Node::Extrude {
        profile: p,
        distance: len(dz),
    })
}

fn block(r: &mut Recorder, (x0, x1): (f64, f64), (y0, y1): (f64, f64), z0: f64, dz: f64) -> RecipeNodeId {
    prism(r, vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)], z0, dz)
}

fn error_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NodeErrorKind {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(NodeError { kind, .. })) => kind,
        other => panic!("node {} must fail typed, got {other:?}", id.0),
    }
}

fn body_arc(ev: &Evaluation<f64>, id: RecipeNodeId) -> &Arc<Body<f64>> {
    match &ev.value(id).expect("a value").payload {
        ValuePayload::Body(b) => b,
        other => panic!("node {} is a {}, not a body", id.0, other.kind_name()),
    }
}

fn sides(ev: &Evaluation<f64>, split: RecipeNodeId) -> (Arc<Body<f64>>, Arc<Body<f64>>) {
    let ValuePayload::Split { above, below } = &ev.value(split).expect("the split").payload else {
        panic!("a split value");
    };
    let body = |s: &SplitSide<f64>| match s {
        SplitSide::Body(b) => Arc::clone(b),
        SplitSide::Empty => panic!("both halves carry material"),
    };
    (body(above), body(below))
}

fn key_free(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find("Key(") {
        let (head, tail) = rest.split_at(at + "Key(".len());
        out.push_str(head);
        out.push('_');
        rest = &tail[tail.find(')').expect("a closed key")..];
    }
    out.push_str(rest);
    out
}

fn bits(b: &Body<f64>) -> Vec<String> {
    let mut v: Vec<String> = vec![format!(
        "counts f{} e{} v{}",
        b.faces().count(),
        b.edges().count(),
        b.vertices().count()
    )];
    let mut s: Vec<String> = b.surfaces().map(|(_, s)| key_free(&format!("S {s:?}"))).collect();
    let mut c: Vec<String> = b.curves().map(|(_, c)| key_free(&format!("C {c:?}"))).collect();
    let mut p: Vec<String> = b.points().map(|(_, p)| key_free(&format!("P {p:?}"))).collect();
    s.sort();
    c.sort();
    p.sort();
    v.extend(s);
    v.extend(c);
    v.extend(p);
    v
}

/// The LIB-G14 survey's tie fixture: a 4×4×4 block minus a U-shaped
/// cutter whose two prongs (y ∈ [1, 1.5] and y ∈ [2.5, 3]) cross one
/// wall. The subtract's table carries genuine N2 ties.
fn u_cutter_tie(r: &mut Recorder) -> RecipeNodeId {
    let a = block(r, (0.0, 4.0), (0.0, 4.0), 0.0, 4.0);
    let b = prism(
        r,
        vec![
            (2.0, 1.0),
            (6.0, 1.0),
            (6.0, 3.0),
            (2.0, 3.0),
            (2.0, 2.5),
            (5.0, 2.5),
            (5.0, 1.5),
            (2.0, 1.5),
        ],
        1.0,
        2.0,
    );
    r.insert(Node::Boolean {
        op: BooleanOp::Subtract,
        a,
        b,
        declare: None,
    })
}

/// **C2, the tie rule.** A split at y = 2 separates the two tied prong
/// fragments WITHOUT cutting either: each passes through intact into a
/// different half, and `name_split` hands a pass-through entity its
/// upstream (tied) name with the side's own body index, so the split's
/// table carries one `Tied` row whose candidates straddle body 0 and
/// body 1. The spec's §4 letter ("a `Tied` entry keeps the refs in the
/// selected body and is dropped if none remain") has both Parts
/// evaluating; the head's projection refuses a straddling tie as an
/// emission bug. This row asserts the spec's letter.
#[test]
fn probe_c2_a_pass_through_tie_straddling_the_halves() {
    let mut r = Recorder::new();
    let sub = u_cutter_tie(&mut r);
    let tool = plane(&mut r, [0.0, 2.0, 0.0], [0.0, 1.0, 0.0]);
    let split = r.insert(Node::Split { target: sub, tool });
    let above = part(&mut r, split, half(SplitHalf::Above));
    let below = part(&mut r, split, half(SplitHalf::Below));
    let ev = eval(&r.doc);
    let t = &ev.value(split).expect("the split names its halves").name_table;
    let straddling: Vec<(&StableName, &Vec<EntityRef>)> = t
        .iter()
        .filter_map(|(n, e)| match e {
            Entry::Tied(es) => {
                let bodies: BTreeSet<u32> = es.iter().map(|e| e.body).collect();
                (bodies.len() > 1).then_some((n, es))
            }
            Entry::Unique(_) => None,
        })
        .collect();
    println!("R1-PROBE straddling ties in the split's table: {}", straddling.len());
    for (n, es) in &straddling {
        println!("  {n}: {es:?}");
    }
    assert!(
        !straddling.is_empty(),
        "the premise did not materialise: no tie straddles the two halves"
    );
    for id in [above, below] {
        match ev.nodes.get(&id) {
            Some(NodeResult::Failed(NodeError { kind, .. })) => {
                println!("R1-PROBE Part {} REFUSES: {kind}", id.0);
            }
            Some(NodeResult::Ok(_)) => println!("R1-PROBE Part {} evaluates", id.0),
            other => println!("R1-PROBE Part {}: {other:?}", id.0),
        }
    }
    assert!(
        ev.value(above).is_some() && ev.value(below).is_some(),
        "spec §4: both Parts evaluate, the tie narrowed to the selected body's refs; \
         got above = {:?}, below = {:?}",
        ev.nodes.get(&above).map(|n| matches!(n, NodeResult::Ok(_))),
        ev.nodes.get(&below).map(|n| matches!(n, NodeResult::Ok(_)))
    );
}

/// **C2 at the function.** `NameTable::project` on a hand-built
/// straddling tie: the head's rule, as coded.
#[test]
fn probe_c2_project_on_a_hand_built_straddling_tie() {
    let mut t = NameTable::new();
    let tie = StableName {
        kind: EntityKind::Body,
        node: RecipeNodeId(7),
        path: vec![],
    };
    t.insert_tied(
        tie,
        vec![
            EntityRef {
                body: 0,
                key: EntityKey::Body,
            },
            EntityRef {
                body: 1,
                key: EntityKey::Body,
            },
        ],
    )
    .unwrap();
    let r0 = t.project(0);
    let r2 = t.project(2);
    println!("R1-PROBE project(0) = {r0:?}; project(2) = {r2:?}");
    assert!(matches!(r0, Err(NamingError::Emission { .. })));
    assert!(matches!(&r2, Ok(p) if p.is_empty()));
}

/// **C1 on a body and a plane the implementer did not choose**: an
/// L-shaped prism cut by an OBLIQUE plane; each half's Part is the
/// side's own `Arc`, and a transform and a union of the Part are the
/// kernel's own doors run on the side. The fillet's outcome (Ok or a
/// refusal) must agree between the Part and the side; where both are
/// Ok the bodies agree.
#[test]
fn probe_c1_an_l_prism_cut_obliquely() {
    for h in SplitHalf::ALL {
        let mut r = Recorder::new();
        let l = prism(
            &mut r,
            vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
            0.0,
            1.0,
        );
        let other = block(&mut r, (-0.5, 0.5), (0.25, 0.75), 0.2, 1.0);
        let tool = plane(&mut r, [0.5, 0.0, 0.5], [0.6, 0.0, 0.8]);
        let split = r.insert(Node::Split { target: l, tool });
        let p = part(&mut r, split, half(h));
        let moved = r.insert(Node::Transform {
            input: p,
            translation: [len(0.1), len(0.2), len(0.3)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        });
        let joined = r.insert(Node::Boolean {
            op: BooleanOp::Union,
            a: p,
            b: other,
            declare: None,
        });
        let first = eval(&r.doc);
        assert!(
            corpus::failures(&first).is_empty(),
            "{h:?}: {:?}",
            corpus::failures(&first)
        );
        let selection = all_edges(&first, p);
        let rounded = r.insert(Node::fillet(p, len(0.02), selection.clone()));
        let ev = eval(&r.doc);
        let (above, below) = sides(&ev, split);
        let side = match h {
            SplitHalf::Above => &above,
            SplitHalf::Below => &below,
        };
        assert!(Arc::ptr_eq(body_arc(&ev, p), side), "{h:?}: the side's Arc");
        let map = Affine3::from_parts(
            Mat3::rotation_about(Vec3::new(0.0, 0.0, 1.0), 0.0),
            Vec3::new(0.1, 0.2, 0.3),
        );
        let placed = transform_rigid(side, &map, Tol::witness()).expect("rigid");
        assert_eq!(bits(body_arc(&ev, moved)), bits(&placed), "{h:?}: transform");
        let fused = match topo::union(side, corpus::body_of(&ev, other), Tol::witness())
            .expect("the kernel union")
        {
            BooleanResult::Body(bb) => bb.body,
            BooleanResult::Empty => panic!("not empty"),
        };
        assert_eq!(bits(corpus::body_of(&ev, joined)), bits(&fused), "{h:?}: union");
        let table = &ev.value(p).expect("the Part").name_table;
        let mut keys: Vec<topo::EdgeKey> = selection
            .iter()
            .map(|n| match table.lookup(n) {
                Some(Entry::Unique(e)) => match e.key {
                    EntityKey::Edge(k) => k,
                    other => panic!("{other:?}"),
                },
                other => panic!("{other:?}"),
            })
            .collect();
        keys.sort_unstable();
        let door = sweep::blend::build::fillet_edges(side, &keys, 0.02, Tol::witness());
        match (ev.nodes.get(&rounded), door) {
            (Some(NodeResult::Ok(_)), Ok(f)) => {
                assert_eq!(bits(body_arc(&ev, rounded)), bits(&f.body), "{h:?}: fillet");
                println!("R1-PROBE {h:?}: fillet Ok on both, bit-equal");
            }
            (Some(NodeResult::Failed(NodeError { kind, .. })), Err(e)) => {
                println!("R1-PROBE {h:?}: fillet refuses on both: doc={kind}; door={e:?}");
            }
            (doc, door) => panic!("{h:?}: fillet outcomes disagree: {doc:?} vs {door:?}"),
        }
    }
}

/// **C1 on a pattern the implementer did not choose**: four instances of
/// the L-prism along y; `Part(3)` is `v[3]`'s `Arc`, `Part(0)` the
/// input's; `Part(1) ∪ Part(3)` is the kernel union of the two.
#[test]
fn probe_c1_a_four_instance_pattern_of_an_l_prism() {
    let mut r = Recorder::new();
    let l = prism(
        &mut r,
        vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
        0.0,
        1.0,
    );
    let pat = r.insert(Node::Pattern {
        input: l,
        count: Expr::count(4),
        kind: PatternKind::Linear {
            direction: [scl(0.0), scl(1.0), scl(0.0)],
            spacing: len(1.5),
        },
    });
    let p0 = part(&mut r, pat, instance(0));
    let p1 = part(&mut r, pat, instance(1));
    let p3 = part(&mut r, pat, instance(3));
    let joined = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: p1,
        b: p3,
        declare: None,
    });
    let ev = eval(&r.doc);
    assert!(corpus::failures(&ev).is_empty(), "{:?}", corpus::failures(&ev));
    let ValuePayload::Instances(v) = &ev.value(pat).expect("the pattern").payload else {
        panic!("instances");
    };
    assert!(Arc::ptr_eq(body_arc(&ev, p0), &v[0]));
    assert!(Arc::ptr_eq(body_arc(&ev, p0), body_arc(&ev, l)));
    assert!(Arc::ptr_eq(body_arc(&ev, p1), &v[1]));
    assert!(Arc::ptr_eq(body_arc(&ev, p3), &v[3]));
    let fused = match topo::union(&v[1], &v[3], Tol::witness()).expect("kernel union") {
        BooleanResult::Body(bb) => bb.body,
        BooleanResult::Empty => panic!("not empty"),
    };
    assert_eq!(bits(corpus::body_of(&ev, joined)), bits(&fused));
}

/// **C3 beyond A4's rows**: the extreme indices, a Part of a Part, a
/// Part of a boolean and of a datum, a Length expression in the
/// Instance slot at the insert door and at the structural-edit door,
/// and the Instance slot addressed on a half selector.
#[test]
fn probe_c3_more_refusals() {
    let mut r = Recorder::new();
    let cube = block(&mut r, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let other = block(&mut r, (0.5, 1.5), (0.25, 0.75), 0.2, 1.0);
    let pat = r.insert(Node::Pattern {
        input: cube,
        count: Expr::count(3),
        kind: PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(3.0),
        },
    });
    let tool = plane(&mut r, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let split = r.insert(Node::Split { target: cube, tool });
    let above = part(&mut r, split, half(SplitHalf::Above));
    let joined = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: cube,
        b: other,
        declare: None,
    });
    let max = part(&mut r, pat, instance(i64::MAX));
    let min = part(&mut r, pat, instance(i64::MIN));
    let of_part = part(&mut r, above, half(SplitHalf::Above));
    let of_part_ix = part(&mut r, above, instance(0));
    let of_bool = part(&mut r, joined, instance(0));
    let of_datum = part(&mut r, tool, half(SplitHalf::Below));
    let ev = eval(&r.doc);
    for (id, res) in &ev.nodes {
        let what = match res {
            NodeResult::Ok(_) => "Ok".to_owned(),
            NodeResult::Failed(NodeError { kind, .. }) => format!("Failed({kind})"),
            NodeResult::Poisoned { through } => format!("Poisoned(through {})", through.0),
        };
        println!("R1-PROBE node {}: {what}", id.0);
    }
    assert!(matches!(
        error_of(&ev, max),
        NodeErrorKind::InstanceOutOfRange { index: i64::MAX, count: 3, .. }
    ));
    assert!(matches!(
        error_of(&ev, min),
        NodeErrorKind::InstanceOutOfRange { index: i64::MIN, count: 3, .. }
    ));
    for (id, expected, found) in [
        (of_part, "split", "body"),
        (of_part_ix, "instances", "body"),
        (of_bool, "instances", "boolean"),
        (of_datum, "split", "datum"),
    ] {
        assert!(
            matches!(
                error_of(&ev, id),
                NodeErrorKind::WrongOperand { expected: e, found: f, .. } if *e == expected && *f == found
            ),
            "{expected} on {found}: {:?}",
            error_of(&ev, id)
        );
    }
    // A Length in the Instance slot at the insert door.
    let inserted = apply(
        &r.doc,
        &DocEdit::InsertNode {
            node: Node::Part {
                of: pat,
                select: PartSelect::Instance(len(1.0)),
            },
        },
        Tol::witness(),
    );
    println!("R1-PROBE insert Part{{Instance(len)}}: {:?}", inserted.as_ref().err());
    assert!(matches!(
        inserted,
        Err(EditError::SlotDimensionMismatch { slot: SlotId::Instance, .. })
    ));
    // A Length through the structural door.
    let edited = apply(
        &r.doc,
        &DocEdit::SetStructuralParam {
            node: max,
            slot: SlotId::Instance,
            expr: len(1.0),
        },
        Tol::witness(),
    );
    println!("R1-PROBE SetStructuralParam(Instance, len): {:?}", edited.as_ref().err());
    assert!(edited.is_err());
    // The Instance slot on a half selector.
    let edited = apply(
        &r.doc,
        &DocEdit::SetStructuralParam {
            node: above,
            slot: SlotId::Instance,
            expr: Expr::count(1),
        },
        Tol::witness(),
    );
    println!("R1-PROBE SetStructuralParam(Instance) on Part{{SplitHalf}}: {:?}", edited.as_ref().err());
    assert!(edited.is_err());
    let _ = RoleSeg::SplitBody(SplitHalf::Above);
}

/// **C6(a), rung 1 on the two section planes of one split.** The
/// corpus document at f64: the two halves' section planes, their
/// sources, and what `oriented_plane_eq_verdict` decides for the pair
/// on those sources. On the head the sources differ and rung 1 does
/// not fire; on the OLD stamping (both `minted(split, 0)`) rung 1
/// answers on source identity alone — with the two normals exact
/// negations, that answer is `SameOriented`, which this row refuses.
/// Also prints what the union of the two halves came to.
#[test]
fn probe_c6a_rung_one_on_the_two_section_planes() {
    use topo::boolean::plane_eq::{PlaneDesc, PlaneIdentity, PlaneRelation, oriented_plane_eq_verdict};
    let cd = corpus::part_select::document();
    let ev = eval(&cd.doc);
    println!("R1-PROBE corpus failures: {:?}", corpus::failures(&ev));
    let split = *cd
        .doc
        .order()
        .iter()
        .find(|id| matches!(cd.doc.node(**id), Some(Node::Split { .. })))
        .expect("the split");
    let (above, below) = sides(&ev, split);
    let section = |b: &Body<f64>| {
        let minted: Vec<_> = b
            .surfaces()
            .filter_map(|(k, s)| {
                b.surface_source(k)
                    .filter(|src| src.node == split.0)
                    .map(|src| (src.clone(), s.clone()))
            })
            .collect();
        assert_eq!(minted.len(), 1, "one section plane per half");
        minted.into_iter().next().unwrap()
    };
    let (src_a, plane_a) = section(&above);
    let (src_b, plane_b) = section(&below);
    println!(
        "R1-PROBE sources: {src_a:?} / {src_b:?}; same_base = {}",
        src_a.same_base(&src_b)
    );
    let (
        topo::Surface::Plane {
            origin: oa,
            normal: na,
            ..
        },
        topo::Surface::Plane {
            origin: ob,
            normal: nb,
            ..
        },
    ) = (plane_a, plane_b)
    else {
        panic!("planes")
    };
    println!("R1-PROBE normals: {na:?} / {nb:?}");
    let band = geom_core::Band::linear(Tol::witness()).expect("a band");
    let verdict = oriented_plane_eq_verdict(
        &PlaneDesc {
            origin: oa,
            normal: na,
        },
        &PlaneDesc {
            origin: ob,
            normal: nb,
        },
        PlaneIdentity {
            s1: Some(&src_a),
            s2: Some(&src_b),
            declared: false,
        },
        1.0,
        band,
    );
    println!("R1-PROBE rung verdict on the pair: {verdict:?}");
    if let Some(whole) = cd.result {
        match ev.nodes.get(&whole) {
            Some(NodeResult::Ok(_)) => {
                let b = corpus::body_of(&ev, whole);
                let m = topo::mass_properties(b, Tol::witness()).expect("mass");
                println!(
                    "R1-PROBE union of the halves: faces {} volume {}",
                    b.faces().count(),
                    m.volume
                );
            }
            other => println!("R1-PROBE union of the halves: {other:?}"),
        }
    }
    assert!(
        !matches!(verdict, Ok((PlaneRelation::SameOriented, _))),
        "rung 1 read two opposed section planes as one oriented plane: {verdict:?}"
    );
}

/// **C6(a)/(b) at a channel-less scalar.** The corpus document at
/// `Dual64`: what the union of the halves comes to, printed beside the
/// f64 outcome's shape (6 faces, volume 4). On the head the two agree;
/// on the old stamping the assertion has no bit channel to fire on, so
/// whatever rung 1 decided flows through unasserted — this row shows
/// what that is.
#[test]
fn probe_c6_dual64_union_outcome() {
    let cd = corpus::part_select::document();
    let ev = corpus::eval::<geom_core::Dual<f64>>(&cd.doc);
    println!("R1-PROBE Dual64 failures: {:?}", corpus::failures(&ev));
    if let Some(whole) = cd.result
        && let Some(NodeResult::Ok(_)) = ev.nodes.get(&whole)
    {
        let b = corpus::body_of(&ev, whole);
        let m = topo::mass_properties(b, Tol::witness()).expect("mass");
        println!(
            "R1-PROBE Dual64 union of the halves: faces {} edges {} volume {:?}",
            b.faces().count(),
            b.edges().count(),
            m.volume
        );
        assert_eq!(b.faces().count(), 6, "the box again");
    }
}

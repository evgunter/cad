//! **DOCM-2 review lane R2 — probes.** Independent falsification of
//! the unit's claims C1–C6 on fixtures the implementer did not choose.
//! Nothing here is a fix; every row is a measurement.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::corpus;
use crate::fixture::{self, Recorder, insert, len, on_frame, scl};

use editor_core::{
    BooleanOp, CancelToken, Datum, EntityKey, EntityKind, EntityRef, Entry, EvalOptions, Evaluation,
    Expr, NameTable, Node, NodeError, NodeErrorKind, NodeResult, PartSelect, PatternKind,
    ProfileDoc, RecipeNodeId, RoleSeg, SlotId, SplitHalf, SplitSide, StableName, ValuePayload,
    evaluate,
};
use geom_core::Tol;
use topo::Body;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn body_arc(ev: &Evaluation<f64>, id: RecipeNodeId) -> &Arc<Body<f64>> {
    match &ev.value(id).expect("a value").payload {
        ValuePayload::Body(b) => b,
        other => panic!("node {} is a {}", id.0, other.kind_name()),
    }
}

fn sides(ev: &Evaluation<f64>, split: RecipeNodeId) -> (Arc<Body<f64>>, Arc<Body<f64>>) {
    let ValuePayload::Split { above, below } = &ev.value(split).expect("split").payload else {
        panic!("a split value");
    };
    let b = |s: &SplitSide<f64>| match s {
        SplitSide::Body(x) => Arc::clone(x),
        SplitSide::Empty => panic!("both halves carry material"),
    };
    (b(above), b(below))
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

fn bits<T: geom_core::Decide + core::fmt::Debug>(b: &Body<T>) -> Vec<String> {
    let mut v: Vec<String> = vec![format!(
        "counts f{} e{} v{}",
        b.faces().count(),
        b.edges().count(),
        b.vertices().count()
    )];
    let mut s: Vec<String> = b
        .surfaces()
        .map(|(_, s)| key_free(&format!("S {s:?}")))
        .collect();
    let mut c: Vec<String> = b
        .curves()
        .map(|(_, c)| key_free(&format!("C {c:?}")))
        .collect();
    let mut p: Vec<String> = b
        .points()
        .map(|(_, p)| key_free(&format!("P {p:?}")))
        .collect();
    s.sort();
    c.sort();
    p.sort();
    v.extend(s);
    v.extend(c);
    v.extend(p);
    v
}

fn kernel_union(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    match topo::union(a, b, Tol::witness()).expect("the kernel union succeeds") {
        topo::BooleanResult::Body(bb) => bb.body,
        topo::BooleanResult::Empty => panic!("not empty"),
    }
}

// ---------------------------------------------------------------
// R2-P1 (C1/C2) — a body and a plane the implementer did not choose:
// an L-shaped prism cut by an OBLIQUE plane.
// ---------------------------------------------------------------

/// The L prism `[(0,0),(3,0),(3,1),(1,1),(1,3),(0,3)] × z ∈ [0, 2]`.
fn l_prism(r: &mut Recorder) -> RecipeNodeId {
    let p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ]],
    );
    r.insert(Node::Extrude {
        profile: p,
        distance: len(2.0),
    })
}

/// **C1 on an unchosen fixture.** An L prism cut by the oblique plane
/// through `(1.2, 1.1, 0.9)` with normal `(1, 1, 1)`: each Part is the
/// side's own `Arc`, its consumers are the kernel's own doors on that
/// side, and its table is exactly the side's rows re-keyed to body 0
/// with the names verbatim.
#[test]
fn r2p1_the_half_is_the_half_on_an_oblique_cut_of_an_l_prism() {
    for h in SplitHalf::ALL {
        let mut r = Recorder::new();
        let solid = l_prism(&mut r);
        let far = {
            let p = r.profile(
                [10.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
            );
            r.insert(Node::Extrude {
                profile: p,
                distance: len(1.0),
            })
        };
        let tool = r.insert(Node::Datum(Datum::Plane {
            origin: [len(1.2), len(1.1), len(0.9)],
            normal: [scl(1.0), scl(1.0), scl(1.0)],
        }));
        let split = r.insert(Node::Split {
            target: solid,
            tool,
        });
        let p = r.insert(Node::Part {
            of: split,
            select: PartSelect::SplitHalf(h),
        });
        let joined = r.insert(Node::Boolean {
            op: BooleanOp::Union,
            a: p,
            b: far,
            declare: None,
        });
        let ev = run(&r.doc);
        assert!(
            corpus::failures(&ev).is_empty(),
            "{h:?}: {:?}",
            corpus::failures(&ev)
        );
        let (above, below) = sides(&ev, split);
        let side = match h {
            SplitHalf::Above => &above,
            SplitHalf::Below => &below,
        };
        assert!(Arc::ptr_eq(body_arc(&ev, p), side), "{h:?}: the side's Arc");
        let fused = kernel_union(side, corpus::body_of(&ev, far));
        assert_eq!(
            bits(corpus::body_of(&ev, joined)),
            bits(&fused),
            "{h:?}: the union of the Part is the kernel's union of the side"
        );

        // The table is the projection: the side's rows, re-keyed to 0,
        // names verbatim; nothing of the other side survives.
        let split_table = &ev.value(split).expect("split").name_table;
        let part_table = &ev.value(p).expect("part").name_table;
        let ix = h.output_body();
        let mut want = 0usize;
        for (name, entry) in split_table.iter() {
            let in_this = match entry {
                Entry::Unique(e) => e.body == ix,
                Entry::Tied(es) => {
                    assert!(
                        es.iter().all(|e| e.body == ix) || es.iter().all(|e| e.body != ix),
                        "{h:?}: a tie straddles the two halves: {name} {es:?}"
                    );
                    es[0].body == ix
                }
            };
            if !in_this {
                assert!(
                    part_table.lookup(name).is_none(),
                    "{h:?}: {name} is the other side's yet survived the projection"
                );
                continue;
            }
            want += 1;
            match (entry, part_table.lookup(name)) {
                (Entry::Unique(e), Some(Entry::Unique(g))) => {
                    assert_eq!(g.body, 0, "{h:?}: {name} re-keyed to body 0");
                    assert_eq!(g.key, e.key, "{h:?}: {name} keeps its key");
                }
                (Entry::Tied(es), Some(Entry::Tied(gs))) => {
                    assert_eq!(gs.len(), es.len(), "{h:?}: {name} keeps its candidates");
                }
                (_, got) => panic!("{h:?}: {name} projected as {got:?}"),
            }
        }
        assert_eq!(part_table.len(), want, "{h:?}: exactly the side's rows");
        assert!(want > 0, "{h:?}: the side has rows at all");
    }
}

// ---------------------------------------------------------------
// R2-P2 (C2) — the tie rule, built rather than assumed.
// ---------------------------------------------------------------

fn ent(body: u32, key: EntityKey) -> EntityRef {
    EntityRef { body, key }
}

fn fake(node: RecipeNodeId, tag: u32) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::SplitBody(if tag % 2 == 0 {
            SplitHalf::Above
        } else {
            SplitHalf::Below
        })],
    }
}

/// **The three tie shapes through `NameTable::project`, directly.** A
/// tie entirely inside the selected body survives as a tie; one
/// entirely outside is dropped; one that STRADDLES refuses
/// `NamingError::Emission`.
#[test]
fn r2p2_project_on_the_three_tie_shapes() {
    let n = RecipeNodeId(7);
    // Four REAL face keys, off a real body: the projection is being
    // asked about rows, and a row's key must be one a table can hold.
    let mut r = Recorder::new();
    let solid = l_prism(&mut r);
    let ev = run(&r.doc);
    let keys: Vec<topo::FaceKey> = corpus::body_of(&ev, solid)
        .faces()
        .map(|(k, _)| k)
        .take(4)
        .collect();
    assert!(keys.len() == 4, "four faces");
    let face = |k: usize| EntityKey::Face(keys[k - 1]);
    let inside = fake(n, 0);
    let outside = fake(n, 1);

    let mut t = NameTable::new();
    t.insert_tied(inside.clone(), vec![ent(0, face(1)), ent(0, face(2))])
        .expect("a tie inside body 0");
    t.insert_tied(outside.clone(), vec![ent(1, face(3)), ent(1, face(4))])
        .expect("a tie inside body 1");
    let p = t.project(0).expect("no straddle here");
    assert!(
        matches!(p.lookup(&inside), Some(Entry::Tied(c)) if c.len() == 2),
        "the in-body tie survives as a tie"
    );
    assert!(p.lookup(&outside).is_none(), "the other body's tie is gone");
    for e in [ent(0, face(1)), ent(0, face(2))] {
        assert_eq!(p.name_of(&e), Some(&inside), "re-keyed candidates");
    }

    let mut t2 = NameTable::new();
    let straddles = fake(n, 0);
    t2.insert_tied(straddles.clone(), vec![ent(0, face(1)), ent(1, face(2))])
        .expect("a straddling tie is a legal TABLE");
    let refused = t2.project(0);
    println!("R2P2 straddling tie -> {refused:?}");
    assert!(refused.is_err(), "the straddling tie refuses: {refused:?}");
}

// ---------------------------------------------------------------
// R2-P3 (C2) — can the emitter MINT a straddling tie? The U-cutter
// fixture of `lib_g14_split_walls`, cut so the plane separates the two
// tied prong fragments instead of sitting above both.
// ---------------------------------------------------------------

fn prism(doc: ProfileDoc, pts: Vec<(f64, f64)>, z0: f64, dz: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(doc, [0.0, 0.0, z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![pts]);
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    prism(doc, vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)], z0, dz)
}

/// `lib_g14_split_walls::u_cutter_tie`, verbatim: a 4×4×4 block minus a
/// U-shaped cutter whose two prongs cross one wall. Its table carries
/// genuine N2 ties, and the two tied candidates sit either side of
/// `y = 2`.
fn u_cutter_tie(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    let (doc, a) = block(doc, (0.0, 4.0), (0.0, 4.0), 0.0, 4.0);
    let (doc, b) = prism(
        doc,
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
    let (doc, sub) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b,
            declare: None,
        },
    );
    (doc, sub)
}

/// **Does a split MINT a tie whose candidates land in different
/// halves?** The U-cutter's tie is symmetric about `y = 2`; a split
/// there puts one candidate in each half. What the split's table holds,
/// and what a `Part` of either half then does with it, is printed and
/// asserted.
#[test]
fn r2p3_a_split_that_separates_a_tie_and_what_a_part_makes_of_it() {
    let (doc, sub) = u_cutter_tie(ProfileDoc::empty_derived("docm2_r2", Tol::witness()));
    let (doc, tool) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(2.0), len(0.0)],
            normal: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, split) = insert(doc, Node::Split { target: sub, tool });
    let (doc, above) = insert(
        doc,
        Node::Part {
            of: split,
            select: PartSelect::SplitHalf(SplitHalf::Above),
        },
    );
    let (doc, below) = insert(
        doc,
        Node::Part {
            of: split,
            select: PartSelect::SplitHalf(SplitHalf::Below),
        },
    );
    let ev = run(&doc);
    println!("R2P3 sub: {:?}", ev.nodes.get(&sub).map(discriminant));
    println!("R2P3 split: {:?}", ev.nodes.get(&split).map(discriminant));
    let up_ties: Vec<String> = ev
        .value(sub)
        .map(|v| {
            v.name_table
                .iter()
                .filter_map(|(n, e)| match e {
                    Entry::Tied(c) => Some(format!("{n} -> {c:?}")),
                    Entry::Unique(_) => None,
                })
                .collect()
        })
        .unwrap_or_default();
    println!("R2P3 upstream ties ({}):", up_ties.len());
    for t in &up_ties {
        println!("  {t}");
    }
    assert!(!up_ties.is_empty(), "the fixture lost its operand tie");

    let Some(v) = ev.value(split) else {
        panic!("the split failed: {:?}", ev.nodes.get(&split));
    };
    let mut straddling = Vec::new();
    let mut in_ties = Vec::new();
    for (n, e) in v.name_table.iter() {
        if let Entry::Tied(c) = e {
            let bodies: std::collections::BTreeSet<u32> = c.iter().map(|x| x.body).collect();
            if bodies.len() > 1 {
                straddling.push(format!("{n} -> {c:?}"));
            } else {
                in_ties.push(format!("{n} -> {c:?}"));
            }
        }
    }
    println!("R2P3 split ties in one body ({}):", in_ties.len());
    for t in &in_ties {
        println!("  {t}");
    }
    println!("R2P3 split ties STRADDLING ({}):", straddling.len());
    for t in &straddling {
        println!("  {t}");
    }
    println!("R2P3 Part(Above): {:?}", ev.nodes.get(&above));
    println!("R2P3 Part(Below): {:?}", ev.nodes.get(&below));
    // The measurement: a straddling tie in the split's table is exactly
    // the shape `NameTable::project`'s doc says cannot happen.
    assert!(
        straddling.is_empty(),
        "the split MINTED {} tie row(s) straddling its two output bodies, so both Parts refuse: \
         {straddling:?}",
        straddling.len()
    );
}

fn discriminant(r: &NodeResult<f64>) -> &'static str {
    match r {
        NodeResult::Ok(_) => "ok",
        NodeResult::Failed(_) => "failed",
        _ => "other",
    }
}

// ---------------------------------------------------------------
// R2-P4 (C3) — index edges the acceptance row does not reach.
// ---------------------------------------------------------------

fn error_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NodeErrorKind {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(NodeError { kind, .. })) => kind,
        other => panic!("node {} must fail typed, got {other:?}", id.0),
    }
}

/// **The extreme indices refuse, they do not wrap or clamp.**
/// `i64::MIN`, `i64::MAX` and `u32::MAX as i64 + 1` — the three values
/// a `usize`/`u32` conversion could quietly fold onto a live body.
#[test]
fn r2p4_extreme_instance_indices_refuse_typed() {
    let mut r = Recorder::new();
    let p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let cube = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    });
    let pat = r.insert(Node::Pattern {
        input: cube,
        count: Expr::count(3),
        kind: PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(3.0),
        },
    });
    let mut ids = Vec::new();
    for i in [i64::MIN, i64::MAX, i64::from(u32::MAX) + 1, -1] {
        ids.push((
            i,
            r.insert(Node::Part {
                of: pat,
                select: PartSelect::Instance(Expr::count(i)),
            }),
        ));
    }
    let ev = run(&r.doc);
    for (i, id) in ids {
        let e = error_of(&ev, id);
        println!("R2P4 index {i} -> {e}");
        assert!(
            matches!(
                e,
                NodeErrorKind::InstanceOutOfRange { index, count: 3, .. } if *index == i
            ),
            "index {i}: {e:?}"
        );
    }
}

/// **A `Part` whose input failed does not itself panic**, and a `Part`
/// of a `Part` refuses `WrongOperand` rather than nesting.
#[test]
fn r2p5_a_part_of_a_failed_input_and_a_part_of_a_part() {
    let mut r = Recorder::new();
    let p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let cube = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    });
    let tool = r.insert(Node::Datum(Datum::Plane {
        origin: [len(0.0), len(0.0), len(0.5)],
        normal: [scl(0.0), scl(0.0), scl(1.0)],
    }));
    let split = r.insert(Node::Split {
        target: cube,
        tool,
    });
    let one = r.insert(Node::Part {
        of: split,
        select: PartSelect::SplitHalf(SplitHalf::Above),
    });
    let two = r.insert(Node::Part {
        of: one,
        select: PartSelect::SplitHalf(SplitHalf::Below),
    });
    let ev = run(&r.doc);
    let e = error_of(&ev, two);
    println!("R2P5 Part of a Part -> {e}");
    assert!(
        matches!(
            e,
            NodeErrorKind::WrongOperand {
                expected: "split",
                found: "body",
                ..
            }
        ),
        "{e:?}"
    );
}

// ---------------------------------------------------------------
// R2-P6 (C6a) — the split's two section planes, at rung 1, in a build
// with the debug assertion COMPILED OUT.
// ---------------------------------------------------------------

/// **What rung 1 decides about the two section planes of one split.**
/// Reads the two halves' minted plane descriptions and their sources
/// off the corpus document and asks
/// `topo::boolean::plane_eq::oriented_plane_eq_verdict` directly, with
/// no declaration. The two normals are exact negations, so anything but
/// `SameOpposite`-or-not-same-source is a wrong verdict; with the old
/// stamping both carry `minted(split, 0)` and the rung answers
/// `SameOriented`. In a release build the debug assertion is gone, so
/// this row is the only thing that sees it.
#[test]
fn r2p6_rung_one_on_the_two_section_planes() {
    let cd = corpus::part_select::document();
    let ev = run(&cd.doc);
    assert!(
        corpus::failures(&ev).is_empty(),
        "{:?}",
        corpus::failures(&ev)
    );
    let split = *cd
        .doc
        .order()
        .iter()
        .find(|id| matches!(cd.doc.node(**id), Some(Node::Split { .. })))
        .expect("the split");
    let (above, below) = sides(&ev, split);
    let section = |b: &Body<f64>| {
        let mut found: Vec<(topo::GeomSource, geom_core::Point3<f64>, geom_core::Vec3<f64>)> =
            Vec::new();
        for (k, s) in b.surfaces() {
            let Some(src) = b.surface_source(k) else {
                continue;
            };
            if src.node != split.0 {
                continue;
            }
            let topo::Surface::Plane { origin, normal, .. } = s else {
                panic!("a section plane is a plane");
            };
            found.push((src.clone(), *origin, *normal));
        }
        assert_eq!(found.len(), 1, "one section plane per half");
        found.pop().expect("one")
    };
    let (sa, oa, na) = section(&above);
    let (sb, ob, nb) = section(&below);
    println!("R2P6 above: {sa:?} n={na:?}");
    println!("R2P6 below: {sb:?} n={nb:?}");
    use topo::boolean::plane_eq::{PlaneDesc, PlaneIdentity, oriented_plane_eq_verdict};
    let p1 = PlaneDesc {
        origin: oa,
        normal: na,
    };
    let p2 = PlaneDesc {
        origin: ob,
        normal: nb,
    };
    let id = PlaneIdentity {
        s1: Some(&sa),
        s2: Some(&sb),
        declared: false,
    };
    let verdict = oriented_plane_eq_verdict(&p1, &p2, id, 1.0, geom_core::Band::linear(geom_core::Tol::witness()).expect("a band"));
    println!("R2P6 rung-1 verdict: {verdict:?}");
    assert!(
        !matches!(
            verdict,
            Ok((topo::boolean::plane_eq::PlaneRelation::SameOriented, _))
        ),
        "two exactly-opposed section planes read as SameOriented: {verdict:?}"
    );
    let _ = fixture::len(0.0);
    let _: Option<SlotId> = None;
}

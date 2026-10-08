//! **The coincidence door** (D10, INTENT stage 4 PR B): every
//! coincidence an operation decides from values is a row on its node's
//! value, its cells named in the tables of the inputs the decision
//! read; the door proves a row structural or leaves it unproven; and
//! the `unproven-coincidence` check reports what it leaves.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{block, failure, run};
use crate::fixture::{ang, fname, insert, len, on_frame, scl, table};

use editor_core::{
    Advisory, BooleanCoincidence, BooleanOp, CapEnd, CheckEvidence, CheckId, ChecksConfig, Datum,
    EntityKey, EntityKind, Entry, Evaluation, NamedCell, NamedCoincidence, Node, PartSelect,
    ProfileDoc, Proof, RecipeNodeId, RoleSeg, Rung, Severity, SitedRef, SplitHalf, StableName,
    ValuePayload, coincide,
};
use geom_core::{MarginDiag, Point3, Tol};
use topo::{DecisionSite, Relation};

/// The `unproven-coincidence` check alone, over `ev`.
fn unproven(doc: &ProfileDoc, ev: &Evaluation<f64>) -> Vec<editor_core::CheckFinding> {
    let cfg = ChecksConfig {
        connectedness: Severity::Off,
        separation: Advisory::Off,
        chart_coherence: Advisory::Off,
        unproven_coincidence: Advisory::Warn,
        ..ChecksConfig::default()
    };
    let report = editor_core::run_checks(doc, ev, &cfg, Tol::witness()).expect("the checks run");
    assert!(
        report
            .findings
            .iter()
            .all(|f| f.check == CheckId::UnprovenCoincidence)
    );
    report.findings
}

/// The rows `node` decided.
fn rows(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<NamedCoincidence> {
    ev.value(node)
        .unwrap_or_else(|| panic!("{node:?} evaluated: {:?}", failure(ev, node)))
        .coincidences
        .to_vec()
}

fn entity(input: RecipeNodeId, name: &StableName) -> NamedCell {
    NamedCell::Entity {
        input,
        name: name.clone(),
    }
}

/// **A declared glue is recorded** (§11 row 3). A plate resting flush
/// on a block, unioned under a declared `Rest`: the union holds one
/// `SameOpposite` row naming the block's top and the plate's bottom by
/// their operands' own names, decided by the plane ladder's declared
/// rung, and the check reports it unproven: the two caps are two
/// constructions.
#[test]
fn a_declared_rest_is_one_unproven_row_named_by_its_operands() {
    let doc = ProfileDoc::empty_derived("coincide-declared-rest", Tol::witness());
    let (doc, base) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, plate) = block(doc, (0.5, 1.5), (0.5, 1.5), 1.0, 0.5);
    let top = fname(base, RoleSeg::Cap(CapEnd::End));
    let bottom = fname(plate, RoleSeg::Cap(CapEnd::Start));
    let (doc, union) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: base,
            b: plate,
            declare: vec![(
                (
                    SitedRef::at_mint(top.clone()),
                    SitedRef::at_mint(bottom.clone()),
                ),
                BooleanCoincidence::REST,
            )],
        },
    );
    let ev = run(&doc);
    let got = rows(&ev, union);
    assert_eq!(got.len(), 1, "one row for the one declared pair: {got:?}");
    let row = &got[0];
    assert_eq!(
        row.cells,
        [entity(base, &top), entity(plate, &bottom)],
        "the cells are the operands' own cap names"
    );
    assert_eq!(
        (row.relation, row.site),
        (Relation::SameOpposite, DecisionSite::PlaneLadder)
    );
    assert_eq!(
        row.margin.kind(),
        geom_core::MarginKind::Value,
        "the declared reading's own margin, never a synthetic one"
    );
    let Proof::Unproven { residual, .. } = coincide::prove(&doc, row) else {
        panic!("the same-source rung proves a declared glue of two extrudes")
    };
    assert!(
        matches!(&residual.constructions, [Some(a), Some(b)] if a != b),
        "two constructions: {residual:?}"
    );
    let findings = unproven(&doc, &ev);
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].root, union, "attributed to the deciding node");
    match &findings[0].evidence {
        CheckEvidence::UnprovenCoincidence { row: found, .. } => assert_eq!(found, row),
        other => panic!("{other:?}"),
    }
    let said = findings[0].to_string();
    assert!(
        said.contains("holds only at the current values") && said.contains("Recourse:"),
        "{said}"
    );
    // The check refuses nothing and changes no body: the union built.
    assert!(failure(&ev, union).is_none());
}

/// A face of `node`'s body 0 whose plane has outward normal `n` and
/// passes through `p`, by name.
fn face_on(ev: &Evaluation<f64>, node: RecipeNodeId, p: [f64; 3], n: [f64; 3]) -> StableName {
    let body = match &ev.value(node).expect("the node evaluated").payload {
        ValuePayload::Body(b) => b.clone(),
        other => panic!("expected a body, got {other:?}"),
    };
    table(ev, node)
        .iter()
        .find_map(|(name, entry)| match entry {
            Entry::Unique(r) if r.body == 0 => match r.key {
                EntityKey::Face(f) => match topo::face_carrier(&body, f)? {
                    topo::CarrierDesc::Plane { origin, normal } => {
                        let facing =
                            (normal - geom_core::Vec3::new(n[0], n[1], n[2])).norm() < 1e-12;
                        let on = normal.dot(Point3::new(p[0], p[1], p[2]) - origin).abs() < 1e-12;
                        (facing && on).then(|| name.clone())
                    }
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        })
        .unwrap_or_else(|| panic!("{node:?} has a face through {p:?} facing {n:?}"))
}

/// **The same construction read twice is proven, through the
/// placement** (§11 row 4). A box placed by a transform and split in
/// two: each half's `−x` wall is a piece of the one extruded wall, read
/// through the one transform, so a row over them is
/// `Structural(SameConstruction)`. The box's own wall against a half's
/// is two constructions — the placed copy is not the unplaced one, and
/// the walk sees the transform although it adds no name segment — and
/// stays unproven.
///
/// No production row reaches the door with one construction read
/// twice in this unit: the kernel settles a same-source pair by its own
/// structural rung before any margin (stage 4 spec §14 Q1), so the row
/// is built here over the scene's real cells.
#[test]
fn a_row_over_one_placed_construction_is_proven_the_same_construction() {
    let doc = ProfileDoc::empty_derived("coincide-same-source", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, placed) = insert(
        doc,
        Node::transform(
            the_box,
            editor_core::Step::Rigid {
                translation: [len(3.0), len(0.0), len(0.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        ),
    );
    let (doc, tool) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.5)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: placed,
            tool,
        },
    );
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
    let wall = |node, z| face_on(&ev, node, [3.0, 0.5, z], [-1.0, 0.0, 0.0]);
    let row = |cells| NamedCoincidence {
        cells,
        relation: Relation::SameOriented,
        site: DecisionSite::PlaneLadder,
        margin: MarginDiag::value(0.0),
        discharge: topo::Discharge::Numeric,
    };
    let halves = row([
        entity(above, &wall(above, 0.75)),
        entity(below, &wall(below, 0.25)),
    ]);
    assert_eq!(
        coincide::prove(&doc, &halves),
        Proof::Structural(Rung::SameConstruction),
        "the two pieces of one placed wall"
    );
    let unplaced = face_on(&ev, the_box, [0.0, 0.5, 0.5], [-1.0, 0.0, 0.0]);
    let across = row([
        entity(the_box, &unplaced),
        entity(above, &wall(above, 0.75)),
    ]);
    assert!(
        matches!(coincide::prove(&doc, &across), Proof::Unproven { .. }),
        "an unplaced wall and its placed copy are two constructions"
    );
}

/// **A union's rows are its pairwise judgement's**, so they are the
/// same in every member order (DM4's contact rule): two blocks
/// overlapping in x, their caps and y-walls declared continuations,
/// unioned as `[a, b]` and as `[b, a]`, record the same rows, each
/// naming its faces by the members' own names.
#[test]
fn a_unions_rows_do_not_depend_on_its_member_order() {
    let doc = ProfileDoc::empty_derived("coincide-union-order", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let pairs = crate::fixture::flush_pairs(&doc, (a, a), (b, b));
    let n = pairs.len();
    let mut seen = Vec::new();
    for members in [[a, b], [b, a]] {
        let (doc, union) =
            crate::docm7_union_declare::declared_union(doc.clone(), &members, pairs.clone());
        let ev = run(&doc);
        let got = rows(&ev, union);
        assert_eq!(got.len(), n, "{members:?}: one row per declared pair");
        for row in &got {
            assert_eq!(
                (row.relation, row.site),
                (Relation::SameOriented, DecisionSite::PlaneLadder)
            );
            assert!(
                row.cells.iter().all(|c| matches!(
                    c,
                    NamedCell::Entity { input, name } if (*input == a || *input == b) && name.node == *input
                )),
                "{members:?}: each cell is a member's own face: {row:?}"
            );
        }
        seen.push(got);
    }
    assert_eq!(seen[0], seen[1], "the rows moved with the member order");
}

/// **A pattern's instances are placed apart.** The walk reads each
/// instance's placement off its `Instance` segment, so instance 1's top
/// and instance 2's are two constructions even though one extrude
/// minted both, while one instance's top read through two parts of it
/// is one.
#[test]
fn a_patterns_instances_are_two_constructions_of_one_minted_face() {
    let doc = ProfileDoc::empty_derived("coincide-pattern", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: the_box,
            count: editor_core::Formula::count(3),
            kind: editor_core::PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(3.0),
            },
        },
    );
    let part = |doc: ProfileDoc, i: i64| {
        insert(
            doc,
            Node::Part {
                of: pattern,
                select: PartSelect::Instance(editor_core::Formula::count(i)),
            },
        )
    };
    let (doc, first) = part(doc, 1);
    let (doc, again) = part(doc, 1);
    let (doc, second) = part(doc, 2);
    let ev = run(&doc);
    let top_at = |node, x: f64| face_on(&ev, node, [x, 0.5, 1.0], [0.0, 0.0, 1.0]);
    let row = |cells| NamedCoincidence {
        cells,
        relation: Relation::SameOriented,
        site: DecisionSite::PlaneLadder,
        margin: MarginDiag::value(0.0),
        discharge: topo::Discharge::Numeric,
    };
    let one = row([
        entity(first, &top_at(first, 3.5)),
        entity(again, &top_at(again, 3.5)),
    ]);
    assert_eq!(
        coincide::prove(&doc, &one),
        Proof::Structural(Rung::SameConstruction),
        "one instance read twice"
    );
    let two = row([
        entity(first, &top_at(first, 3.5)),
        entity(second, &top_at(second, 6.5)),
    ]);
    let Proof::Unproven { residual, .. } = coincide::prove(&doc, &two) else {
        panic!("two instances are two placements")
    };
    let [Some(a), Some(b)] = &residual.constructions else {
        panic!("both cells walk to the box: {residual:?}")
    };
    assert_eq!(a.minted, b.minted, "one extrude minted both tops");
    assert_ne!(a.placed, b.placed, "placed by two instances");
}

/// The name of `node`'s edge between two points.
fn edge_named(ev: &Evaluation<f64>, node: RecipeNodeId, a: [f64; 3], b: [f64; 3]) -> StableName {
    let body = match &ev.value(node).expect("the node evaluated").payload {
        ValuePayload::Body(b) => b.clone(),
        other => panic!("expected a body, got {other:?}"),
    };
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let at = |p: Point3<f64>, q: [f64; 3]| (p - Point3::new(q[0], q[1], q[2])).norm() < 1e-12;
    table(ev, node)
        .iter()
        .find_map(|(n, entry)| match entry {
            Entry::Unique(r) => match r.key {
                EntityKey::Edge(e) => {
                    let he = body.get_edge(e).unwrap().he_plus;
                    let s = point(body.get_half_edge(he).unwrap().start);
                    let t = point(body.half_edge_end(he).unwrap());
                    ((at(s, a) && at(t, b)) || (at(s, b) && at(t, a))).then(|| n.clone())
                }
                _ => None,
            },
            Entry::Tied(_) => None,
        })
        .unwrap_or_else(|| panic!("an edge named between {a:?} and {b:?}"))
}

/// **The mitre leaves the battery** (§11 row 5). A box with both rims
/// filleted turns at its eight corners: the fillet holds eight
/// `EqualAngles` rows, each naming the two rim edges that turn there
/// by the box's own names, and each is unproven — the two edges are
/// two constructions.
#[test]
fn a_filleted_box_records_one_unproven_turn_per_corner() {
    let doc = ProfileDoc::empty_derived("coincide-mitre", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 2.0), (0.0, 1.5), 0.0, 1.0);
    let ev = run(&doc);
    let corners = [[0.0, 0.0], [2.0, 0.0], [2.0, 1.5], [0.0, 1.5]];
    let rim = |z: f64| -> Vec<StableName> {
        (0..4)
            .map(|i| {
                let (p, q) = (corners[i], corners[(i + 1) % 4]);
                edge_named(&ev, the_box, [p[0], p[1], z], [q[0], q[1], z])
            })
            .collect()
    };
    let edges: Vec<StableName> = [rim(0.0), rim(1.0)].concat();
    let (doc, fillet) = insert(doc, Node::fillet(the_box, len(0.1), edges.clone()));
    let ev = run(&doc);
    let got = rows(&ev, fillet);
    assert_eq!(got.len(), 8, "one row per corner: {got:?}");
    for row in &got {
        assert_eq!(
            (row.relation, row.site),
            (Relation::EqualAngles, DecisionSite::BatteryTurn)
        );
        for cell in &row.cells {
            let NamedCell::Entity { input, name } = cell else {
                panic!("a turn's cells are edges: {cell:?}")
            };
            assert!(
                *input == the_box && edges.contains(name) && name.kind == EntityKind::Edge,
                "{cell:?} is a requested rim edge of the box"
            );
        }
        assert!(matches!(coincide::prove(&doc, row), Proof::Unproven { .. }));
    }
    assert_eq!(unproven(&doc, &ev).len(), 8);
}

/// `profile` extruded one unit and split by the plane `y = at`.
fn split_prism(
    id: &str,
    profile: &[(f64, f64)],
    at: f64,
) -> (ProfileDoc, Evaluation<f64>, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived(id, Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![profile.to_vec()],
    );
    let (doc, prism) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
            side: editor_core::ExtrudeSide::Along,
        },
    );
    let (doc, tool) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(at), len(0.0)],
            normal: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: prism,
            tool,
        },
    );
    let ev = run(&doc);
    (doc, ev, split)
}

/// **A transversal cut records nothing** (§11 row 6's second half): a
/// block cut through its middle crosses its edges, and no ON verdict
/// leaves pieces touching. The pinch half of the row is the kernel's
/// (`topo`'s `m3_pr3_split`): a document cannot yet split through a
/// pinch (`work/wire/a-split-through-a-pinch-names-both-tip-copies-alike.md`).
#[test]
fn a_transversal_split_records_nothing() {
    let square = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
    let (doc, ev, split) = split_prism("coincide-transversal", &square, 1.0);
    assert!(rows(&ev, split).is_empty(), "a transversal cut is no pinch");
    assert!(unproven(&doc, &ev).is_empty());
}

/// **Turning the check off removes the reports and nothing else**
/// (§14 Q2): the report lists it skipped, and the evaluation it read is
/// the one it was handed.
#[test]
fn the_check_off_is_visibly_skipped() {
    let doc = ProfileDoc::empty_derived("coincide-off", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 2.0), (0.0, 1.5), 0.0, 1.0);
    let ev = run(&doc);
    let edges = vec![
        edge_named(&ev, the_box, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
        edge_named(&ev, the_box, [2.0, 0.0, 1.0], [2.0, 1.5, 1.0]),
    ];
    let (doc, fillet) = insert(doc, Node::fillet(the_box, len(0.1), edges));
    let ev = run(&doc);
    assert_eq!(rows(&ev, fillet).len(), 1, "one turn decided");
    let cfg = ChecksConfig {
        unproven_coincidence: Advisory::Off,
        ..ChecksConfig::default()
    };
    let report = editor_core::run_checks(&doc, &ev, &cfg, Tol::witness()).expect("the checks run");
    assert!(report.skipped.contains(&CheckId::UnprovenCoincidence));
    assert!(
        report
            .findings
            .iter()
            .all(|f| f.check != CheckId::UnprovenCoincidence),
        "{:?}",
        report.findings
    );
    assert_eq!(
        rows(&ev, fillet).len(),
        1,
        "the rows ride the value all the same"
    );
}

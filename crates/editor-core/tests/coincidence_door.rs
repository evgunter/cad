//! **The coincidence door** (D10, INTENT stage 4 PR B): every
//! coincidence an operation decides from values is a row on its node's
//! value, its cells named in the tables of the inputs the decision
//! read; the door proves a row structural or leaves it unproven; and
//! the `unproven-coincidence` check reports what it leaves.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{block, failure, run};
use crate::fixture::{ang, fname, insert, len, len2, on_frame, scl, table};

use editor_core::{
    Advisory, BooleanCoincidence, BooleanOp, CapEnd, CheckEvidence, CheckId, ChecksConfig, Datum,
    EntityKey, EntityKind, Entry, Evaluation, FindingSubject, Formula, LoopProgram, NamedCell,
    NamedCoincidence, Node, Operand, PartSelect, ProfileDoc, ProfileProgram, ProgramArcData, ProgramStep,
    ProgramTarget, Proof, RecipeNodeId, RoleSeg, Rung, Severity, SitedRef, SplitHalf, StableName,
    ValuePayload, coincide, spoken_by,
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
            a: base.into(),
            b: plate.into(),
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
        matches!(&residual.constructions, [Ok(a), Ok(b)] if a.origin != b.origin),
        "two constructions: {residual:?}"
    );
    let findings = unproven(&doc, &ev);
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(
        findings[0].subject,
        FindingSubject::Node(union),
        "attributed to the deciding node"
    );
    match &findings[0].evidence {
        CheckEvidence::UnprovenCoincidence { row: found, .. } => assert_eq!(**found, *row),
        other => panic!("{other:?}"),
    }
    let said = findings[0].to_string();
    assert!(
        said.contains("is not proven structural: the two cells are two constructions")
            && said.contains("Recourse:"),
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
    face_of(ev, node, &body, 0, p, n)
}

/// [`face_on`] on one half of `split`, read off the split's own value:
/// the half's body and the split's rows for that output.
fn half_face_on(
    ev: &Evaluation<f64>,
    split: RecipeNodeId,
    half: SplitHalf,
    p: [f64; 3],
    n: [f64; 3],
) -> StableName {
    let side = match &ev.value(split).expect("the split evaluated").payload {
        ValuePayload::Split { above, below } => match half {
            SplitHalf::Above => above,
            SplitHalf::Below => below,
        },
        other => panic!("expected a split, got {other:?}"),
    };
    let editor_core::SplitSide::Body(body) = side else {
        panic!("{half:?} holds material")
    };
    face_of(ev, split, body, half.output_body(), p, n)
}

/// The face of output `index` of `node`, a plane through `p` facing `n`.
fn face_of(
    ev: &Evaluation<f64>,
    node: RecipeNodeId,
    body: &topo::Body<f64>,
    index: u32,
    p: [f64; 3],
    n: [f64; 3],
) -> StableName {
    table(ev, node)
        .iter()
        .find_map(|(name, entry)| match entry {
            Entry::Unique(r) if r.body == index => match r.key {
                EntityKey::Face(f) => match topo::face_carrier(body, f)? {
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
/// The kernel settles a same-source pair like these walls by its own
/// structural rung before any margin (stage 4 spec §14 Q1), so no
/// production row reaches the door over them, and the row is built
/// here over the scene's real cells. The production row that does is
/// the section caps' ([`a_reunited_splits_section_caps_are_one_construction`]).
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
            target: placed.into(),
            tool: tool.into(),
        },
    );
    let ev = run(&doc);
    let wall = |half, z| half_face_on(&ev, split, half, [3.0, 0.5, z], [-1.0, 0.0, 0.0]);
    let row = |cells| NamedCoincidence {
        cells,
        relation: Relation::SameOriented,
        site: DecisionSite::PlaneLadder,
        margin: MarginDiag::value(0.0),
        discharge: topo::Discharge::Numeric,
    };
    let halves = row([
        entity(split, &wall(SplitHalf::Above, 0.75)),
        entity(split, &wall(SplitHalf::Below, 0.25)),
    ]);
    assert_eq!(
        coincide::prove(&doc, &halves),
        Proof::Structural(Rung::SameConstruction),
        "the two pieces of one placed wall"
    );
    let unplaced = face_on(&ev, the_box, [0.0, 0.5, 0.5], [-1.0, 0.0, 0.0]);
    let across = row([
        entity(the_box, &unplaced),
        entity(split, &wall(SplitHalf::Above, 0.75)),
    ]);
    assert!(
        matches!(coincide::prove(&doc, &across), Proof::Unproven { .. }),
        "an unplaced wall and its placed copy are two constructions"
    );
}

/// **A union's rows are its pairwise judgement's, in the author's
/// order** (#4323: the list is the author's stated order). Two blocks
/// overlapping in x, their caps and y-walls declared continuations,
/// unioned as `[a, b]` and as `[b, a]`: the same decisions either way,
/// each row's first cell the listed-first member's face, named by that
/// member's own table.
#[test]
fn a_unions_rows_follow_its_member_order() {
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
            let input = |c: &NamedCell| match c {
                NamedCell::Entity { input, name } if name.node == *input => *input,
                other => panic!("{members:?}: a cell is a member's own face: {other:?}"),
            };
            assert_eq!(
                row.cells.each_ref().map(input),
                members,
                "{members:?}: the listed-first member's cell first"
            );
        }
        seen.push(got);
    }
    let [ab, ba] = [&seen[0], &seen[1]];
    let mut ab: Vec<_> = ab.iter().map(|r| r.cells.clone()).collect();
    let mut ba: Vec<_> = ba
        .iter()
        .map(|r| [r.cells[1].clone(), r.cells[0].clone()])
        .collect();
    let key = |c: &[NamedCell; 2]| format!("{c:?}");
    ab.sort_by_key(key);
    ba.sort_by_key(key);
    assert_eq!(ab, ba, "the same decisions in either order");
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
            input: the_box.into(),
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
                of: pattern.into(),
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
    let [Ok(a), Ok(b)] = &residual.constructions else {
        panic!("both cells walk to the box: {residual:?}")
    };
    assert_eq!(a.origin, b.origin, "one extrude minted both tops");
    assert_ne!(a.placed, b.placed, "placed by two instances");
    assert_eq!(
        residual.to_string(),
        "the two cells are one construction placed apart"
    );
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
            profile: p.into(),
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
            target: prism.into(),
            tool: tool.into(),
        },
    );
    let ev = run(&doc);
    (doc, ev, split)
}

/// **A cut through a vertex records nothing** (§11 row 6's second
/// half): a diamond prism split by `y = 1` has four operand vertices ON
/// the plane (the diamond's side corners, at both ends), each with one
/// run on either side, so the plane cuts there and leaves no pieces
/// touching. The pinch half of the row is the kernel's (`topo`'s
/// `m3_pr3_split`): a document cannot yet split through a pinch
/// (`work/wire/a-split-through-a-pinch-names-both-tip-copies-alike.md`).
#[test]
fn a_split_through_a_vertex_that_only_cuts_records_nothing() {
    let diamond = [(1.0, 0.0), (2.0, 1.0), (1.0, 2.0), (0.0, 1.0)];
    let (doc, ev, split) = split_prism("coincide-through-vertex", &diamond, 1.0);
    assert!(failure(&ev, split).is_none(), "{:?}", failure(&ev, split));
    assert!(
        rows(&ev, split).is_empty(),
        "an ON vertex that cuts is no pinch"
    );
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

/// **A split's section faces are its tool's plane read twice** (§11
/// row 4, on the spec's own scene). A box split in two and the halves
/// unioned again, the section caps declared `Rest`: the union records
/// one `SameOpposite` row over the two caps, and the door proves it,
/// since both lie on the one tool plane the split read. The whole
/// check run over the document is quiet: the lint's silent half, end
/// to end.
#[test]
fn a_reunited_splits_section_caps_are_one_construction() {
    let doc = ProfileDoc::empty_derived("coincide-reunite", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
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
            target: the_box.into(),
            tool: tool.into(),
        },
    );
    let ev = run(&doc);
    let cap = |half, z, nz| half_face_on(&ev, split, half, [0.5, 0.5, z], [0.0, 0.0, nz]);
    let (above_cap, below_cap) = (
        cap(SplitHalf::Above, 0.5, -1.0),
        cap(SplitHalf::Below, 0.5, 1.0),
    );
    assert!(
        matches!(above_cap.path.as_slice(), [RoleSeg::SectionFace { .. }])
            && matches!(below_cap.path.as_slice(), [RoleSeg::SectionFace { .. }]),
        "the premise: each cap is a section face of the split, {above_cap:?}, {below_cap:?}"
    );
    let wall = |half, z| half_face_on(&ev, split, half, [0.0, 0.5, z], [-1.0, 0.0, 0.0]);
    // Both halves are read at the split's site; each name is sided by
    // the half whose rows hold it.
    let (doc, union) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: Operand::output(split, SplitHalf::Above.port()),
            b: Operand::output(split, SplitHalf::Below.port()),
            declare: vec![
                (
                    (
                        SitedRef::new(split, above_cap.clone()),
                        SitedRef::new(split, below_cap.clone()),
                    ),
                    BooleanCoincidence::REST,
                ),
                (
                    (
                        SitedRef::new(split, wall(SplitHalf::Above, 0.75)),
                        SitedRef::new(split, wall(SplitHalf::Below, 0.25)),
                    ),
                    BooleanCoincidence::Continuation,
                ),
            ],
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let got = rows(&ev, union);
    let caps = [entity(split, &above_cap), entity(split, &below_cap)];
    let row = got
        .iter()
        .find(|r| r.cells == caps)
        .unwrap_or_else(|| panic!("the caps' row: {got:?}"));
    assert_eq!(
        (row.relation, row.site),
        (Relation::SameOpposite, DecisionSite::PlaneLadder)
    );
    assert_eq!(
        coincide::prove(&doc, row),
        Proof::Structural(Rung::SameConstruction),
        "both caps lie on the one tool plane"
    );
    let [Ok(a), Ok(b)] = caps.each_ref().map(|c| match c {
        NamedCell::Entity { input, name } => coincide::construction(&doc, *input, name),
        NamedCell::Tool { .. } | NamedCell::Piece { .. } => unreachable!(),
    }) else {
        panic!("both caps walk")
    };
    assert_eq!(a.origin, coincide::Origin::Datum(tool));
    assert_eq!(a, b);
    for row in &got {
        assert!(
            matches!(coincide::prove(&doc, row), Proof::Structural(_)),
            "every row of the reunion is one construction: {row:?}"
        );
    }
    let report = editor_core::run_checks(&doc, &ev, &ChecksConfig::default(), Tol::witness())
        .expect("the checks run");
    assert!(
        !report.skipped.contains(&CheckId::UnprovenCoincidence),
        "the premise: the check ran"
    );
    assert!(
        report
            .findings
            .iter()
            .all(|f| f.check != CheckId::UnprovenCoincidence),
        "a proven row is not reported: {:?}",
        report.findings
    );
}

/// Two unit blocks `gap` apart along x, their flush caps and y-walls
/// declared continuations, joined by `op`: the boolean's own rows and
/// its node.
fn apart(op: BooleanOp, id: &str) -> (ProfileDoc, Evaluation<f64>, RecipeNodeId, usize) {
    let doc = ProfileDoc::empty_derived(id, Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (2.0, 3.0), (0.0, 1.0), 0.0, 1.0);
    let pairs = crate::fixture::flush_pairs(&doc, (a, a), (b, b));
    let n = pairs.len();
    let (doc, node) = insert(
        doc,
        Node::Boolean {
            op,
            a: a.into(),
            b: b.into(),
            declare: pairs
                .into_iter()
                .map(|p| (p, BooleanCoincidence::Continuation))
                .collect(),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, node).is_none(), "{:?}", failure(&ev, node));
    (doc, ev, node, n)
}

/// **The containment fallback carries the declaration door's rows.**
/// Two blocks apart, their flush faces declared: the union's boundaries
/// never cross, so it is the fallback's assembly, and the subtraction's
/// is operand A whole (the single-operand finish). Each records one row
/// per declared pair all the same: the declaration door decided them
/// before the fallback was chosen.
#[test]
fn the_fallbacks_carry_the_declared_rows() {
    for (op, id) in [
        (BooleanOp::Union, "coincide-apart-union"),
        (BooleanOp::Subtract, "coincide-apart-subtract"),
    ] {
        let (doc, ev, node, n) = apart(op, id);
        assert!(n > 0, "the premise: flush faces to declare");
        let got = rows(&ev, node);
        assert_eq!(got.len(), n, "{op:?}: one row per declared pair: {got:?}");
        assert!(
            got.iter().all(
                |r| r.relation == Relation::SameOriented && r.site == DecisionSite::PlaneLadder
            ),
            "{op:?}: {got:?}"
        );
        assert_eq!(unproven(&doc, &ev).len(), n, "{op:?}: two extrudes' faces");
    }
}

/// A one-loop profile program on a fresh xy frame.
fn profile_of(doc: ProfileDoc, steps: LoopProgram<Formula>) -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane) = insert(
        doc,
        crate::fixture::frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: plane.into(),
            loops: vec![steps],
            ids: Vec::new(),
        }),
    )
}

/// **A profile junction no constructor made is a row on its profile**
/// (§11 row 21, at the document). A line meets an arc at a turn
/// `φ = √ε` the lattice reads as a corner (`sin φ · arm`, far past the
/// band) and validation's carrier clearance (`r(1 − cos φ) ≈ ε/2`)
/// reads Zero: the profile builds, its node holds one
/// `Tangent { aligned: true }` row decided at `ProfileJunction` over
/// the line's piece and the arc's, and the check reports it unproven,
/// the two pieces being two constructions. A circle, whose joints its
/// form constructs, holds none.
///
/// Red if the junction refuses, or its row is not carried onto the
/// profile's value (no row), or a constructed joint is recorded (the
/// circle has one).
#[test]
fn a_profile_junction_decided_tangent_is_one_unproven_row_on_its_profile() {
    let phi = Tol::witness().eps().sqrt();
    let c = [1.0 - phi.sin(), phi.cos()];
    let doc = ProfileDoc::empty_derived("g_profile_junction", Tol::witness());
    let (doc, p) = profile_of(
        doc,
        LoopProgram::Chain(vec![
            ProgramStep::At(len2([0.0, 0.0])),
            ProgramStep::LineTo(ProgramTarget::Point(len2([1.0, 0.0]))),
            ProgramStep::ArcTo(ProgramArcData::Center {
                c: len2(c),
                winding: profile::ArcSweep::Ccw,
                target: ProgramTarget::Point(len2([c[0], c[1] + 1.0])),
            }),
            ProgramStep::LineTo(ProgramTarget::Start),
        ]),
    );
    let (doc, circle) = profile_of(
        doc,
        LoopProgram::Circle {
            centre: len2([0.0, 0.0]),
            radius: len(1.0),
        },
    );
    let ev = run(&doc);
    let got = rows(&ev, p);
    assert_eq!(got.len(), 1, "one decided junction: {got:?}");
    assert_eq!(
        (got[0].relation, got[0].site),
        (
            Relation::Tangent { aligned: true },
            DecisionSite::ProfileJunction
        )
    );
    match &got[0].cells {
        [
            NamedCell::Piece {
                profile: a,
                piece: line,
            },
            NamedCell::Piece {
                profile: b,
                piece: arc,
            },
        ] => {
            assert_eq!((*a, *b), (p, p), "both cells are the profile's own pieces");
            assert_ne!(line, arc, "the arriving and leaving pieces are two");
        }
        cells => panic!("a junction's cells are two pieces: {cells:?}"),
    }
    assert!(
        rows(&ev, circle).is_empty(),
        "a circle's joints are constructed"
    );
    let findings = unproven(&doc, &ev);
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].subject, FindingSubject::Node(p));
    let CheckEvidence::UnprovenCoincidence { row, .. } = &findings[0].evidence else {
        panic!("{:?}", findings[0].evidence)
    };
    assert_eq!(**row, got[0]);
    let said = spoken_by(&findings[0], &doc);
    assert!(
        said.contains("continues tangent into")
            && said.contains("a profile junction no constructor made")
            && said.contains("the two cells are two constructions"),
        "{said}"
    );
}

/// **A same-carrier junction no constructor made is recorded
/// `SameOriented`**, carrier identity, not a tangency between two
/// carriers. A leg of `ε/10` turns off the line by its whole length,
/// so the lattice (`sin φ · arm` over the long arriving leg) reads a
/// corner while validation, reading the short leg's far end against
/// the line, reads one carrier continuing: the node holds one
/// `SameOriented` row at `ProfileJunction`, and the check says so.
///
/// Red if the same-carrier arm is recorded as `Tangent`, or not at all.
#[test]
fn a_profile_junction_decided_on_one_carrier_is_one_same_oriented_row() {
    let rise = 0.1 * Tol::witness().eps();
    let doc = ProfileDoc::empty_derived("g_profile_same_carrier", Tol::witness());
    let (doc, p) = profile_of(
        doc,
        LoopProgram::Chain(vec![
            ProgramStep::At(len2([0.0, 0.0])),
            ProgramStep::LineTo(ProgramTarget::Point(len2([10.0, 0.0]))),
            ProgramStep::LineTo(ProgramTarget::Point(len2([10.001, rise]))),
            ProgramStep::LineTo(ProgramTarget::Point(len2([10.001, 5.0]))),
            ProgramStep::LineTo(ProgramTarget::Start),
        ]),
    );
    let ev = run(&doc);
    let got = rows(&ev, p);
    assert_eq!(
        got.iter().map(|r| (r.relation, r.site)).collect::<Vec<_>>(),
        [(Relation::SameOriented, DecisionSite::ProfileJunction)],
        "{got:?}"
    );
    let findings = unproven(&doc, &ev);
    assert_eq!(findings.len(), 1, "{findings:?}");
    let said = spoken_by(&findings[0], &doc);
    assert!(
        said.contains("continues on one carrier with")
            && said.contains("a profile junction no constructor made"),
        "{said}"
    );
}

//! **The coincidence door** (D10, INTENT stage 4 PR B): every
//! coincidence an operation decides from values is a row on its node's
//! value, its cells named in the tables of the inputs the decision
//! read; the door proves a row structural or leaves it unproven; and
//! the `unproven-coincidence` check reports what it leaves.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{block, failure, run};
use crate::fixture::{ang, fname, insert, len, len2, on_frame, out, scl, table};

use editor_core::{
    Advisory, BooleanCoincidence, CapEnd, CheckEvidence, CheckId, ChecksConfig, Datum, EntityKey,
    EntityKind, Entry, Evaluation, FindingSubject, Formula, LoopProgram, NamedCell,
    NamedCoincidence, Node, PartSelect, ProfileDoc, ProfileProgram, ProgramArcData, ProgramStep,
    ProgramTarget, Proof, RecipeNodeId, RoleSeg, Rung, Severity, SitedRef, SplitHalf, StableName,
    ValuePayload, VarId, coincide, spoken_by,
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

/// The door's proof of each of `node`'s `rows`, given in its decision
/// order.
fn proofs(doc: &ProfileDoc, node: RecipeNodeId, rows: &[NamedCoincidence]) -> Vec<Proof> {
    coincide::prove(
        doc,
        &[(node, rows)],
        Tol::witness(),
        &editor_core::CancelToken::new(),
    )
    .expect("an uncanceled door")
    .pop()
    .expect("one node asked")
}

/// What each proof left the replay saying, `None` for a proven row.
fn replays(proofs: &[Proof]) -> Vec<Option<coincide::Unreplayed>> {
    proofs
        .iter()
        .map(|p| match p {
            Proof::Structural(_) => None,
            Proof::Unproven { residual, .. } => Some(residual.replay),
        })
        .collect()
}

/// Each proof's recourse, `None` for a proven row.
fn recourses(proofs: &[Proof]) -> Vec<Option<coincide::Recourse>> {
    proofs
        .iter()
        .map(|p| match p {
            Proof::Structural(_) => None,
            Proof::Unproven { recourse, .. } => Some(*recourse),
        })
        .collect()
}

/// The door's proof of `row`, as the first of `node`'s rows at its site.
fn proof(doc: &ProfileDoc, node: RecipeNodeId, row: &NamedCoincidence) -> Proof {
    proofs(doc, node, core::slice::from_ref(row)).remove(0)
}

/// The variable `split`'s port for `half` defines.
fn port(doc: &ProfileDoc, split: RecipeNodeId, half: SplitHalf) -> VarId {
    doc.output(split, half.port())
        .expect("a split defines a port per half")
}

fn entity(input: VarId, name: &StableName) -> NamedCell {
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
    let (base_read, plate_read) = (out(&doc, base), out(&doc, plate));
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![base.into(), plate.into()]),
            declare: vec![(
                (
                    SitedRef::new(base_read, top.clone()),
                    SitedRef::new(plate_read, bottom.clone()),
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
        [
            entity(out(&doc, base), &top),
            entity(out(&doc, plate), &bottom)
        ],
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
    let Proof::Unproven { residual, .. } = proof(&doc, union, row) else {
        panic!("the same-construction rung proves a declared glue of two extrudes")
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
/// The row is built here over the scene's real cells; the production
/// rows over a split's walls and caps are the reunion's
/// ([`a_reunited_splits_section_caps_are_one_construction`]).
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
    let (above, below) = (
        port(&doc, split, SplitHalf::Above),
        port(&doc, split, SplitHalf::Below),
    );
    let halves = row([
        entity(above, &wall(SplitHalf::Above, 0.75)),
        entity(below, &wall(SplitHalf::Below, 0.25)),
    ]);
    assert_eq!(
        proof(&doc, split, &halves),
        Proof::Structural(Rung::SameConstruction),
        "the two pieces of one placed wall"
    );
    let unplaced = face_on(&ev, the_box, [0.0, 0.5, 0.5], [-1.0, 0.0, 0.0]);
    let across = row([
        entity(out(&doc, the_box), &unplaced),
        entity(above, &wall(SplitHalf::Above, 0.75)),
    ]);
    assert!(
        matches!(proof(&doc, split, &across), Proof::Unproven { .. }),
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
                NamedCell::Entity { input, name } if out(&doc, name.node) == *input => *input,
                other => panic!("{members:?}: a cell is a member's own face: {other:?}"),
            };
            assert_eq!(
                row.cells.each_ref().map(input),
                members.map(|m| out(&doc, m)),
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
        entity(out(&doc, first), &top_at(first, 3.5)),
        entity(out(&doc, again), &top_at(again, 3.5)),
    ]);
    assert_eq!(
        proof(&doc, again, &one),
        Proof::Structural(Rung::SameConstruction),
        "one instance read twice"
    );
    let two = row([
        entity(out(&doc, first), &top_at(first, 3.5)),
        entity(out(&doc, second), &top_at(second, 6.5)),
    ]);
    let Proof::Unproven { residual, .. } = proof(&doc, second, &two) else {
        panic!("two instances are two placements")
    };
    let [Ok(a), Ok(b)] = &residual.constructions else {
        panic!("both cells walk to the box: {residual:?}")
    };
    assert_eq!(a.origin, b.origin, "one extrude minted both tops");
    assert_ne!(a.placed, b.placed, "placed by two instances");
    assert_eq!(
        residual.to_string(),
        "the two cells are one construction placed apart; the replay with every variable a \
         symbol took other decisions there",
        "a row no node decided matches no row of the replay"
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
/// by the box's own names, and each is unproven: the two edges are two
/// constructions, and replayed with every variable a symbol the turn's
/// angle term is the zero polynomial but the distance between its two
/// bands' feet is not reduced to it
/// (`work/sym/the-mitres-feet-distance-freezes-at-sym.md`).
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
                *input == out(&doc, the_box)
                    && edges.contains(name)
                    && name.kind == EntityKind::Edge,
                "{cell:?} is a requested rim edge of the box"
            );
        }
    }
    let proved = proofs(&doc, fillet, &got);
    assert_eq!(
        replays(&proved),
        [Some(coincide::Unreplayed::Numeric); 8],
        "the door takes each turn, and its margin is not shown the zero polynomial \
         (work/sym/the-mitres-feet-distance-freezes-at-sym.md)"
    );
    assert_eq!(
        recourses(&proved),
        [Some(coincide::Recourse::OneConstruction); 8],
        "the feet distance freezes on the door's budget, which is no value computed apart \
         from the variables: no special recourse"
    );
    assert_eq!(unproven(&doc, &ev).len(), 8);
}

/// **The half of the mitre the algebra closes.** Replayed with every
/// variable a symbol, a box mitre's margin
/// `max(|cos₁ − cos₂|·arm, |y₂ − y₁|)` reduces to `max(0, …)`: the
/// angle term is the zero polynomial, so the isosceles fact itself is
/// an identity of the output definitions, and the distance between the
/// two bands' feet is what freezes
/// (`work/sym/the-mitres-feet-distance-freezes-at-sym.md`).
#[test]
fn a_box_mitres_angle_term_is_the_zero_polynomial() {
    let doc = ProfileDoc::empty_derived("coincide-mitre", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 2.0), (0.0, 1.5), 0.0, 1.0);
    let ev = run(&doc);
    let (doc, fillet) = insert(
        doc,
        Node::fillet(
            the_box,
            len(0.1),
            vec![
                edge_named(&ev, the_box, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
                edge_named(&ev, the_box, [2.0, 0.0, 1.0], [2.0, 1.5, 1.0]),
            ],
        ),
    );
    let form = turn_form(&doc, fillet);
    assert!(
        form.starts_with("1·max(0, "),
        "the angle term is identically zero, the feet distance is what freezes: {form}"
    );
}

/// `node`'s first row's deciding margin, replayed with every variable
/// a symbol and rendered with the tier's rules applied per node, at
/// the drive's budget.
fn turn_form(doc: &ProfileDoc, node: RecipeNodeId) -> String {
    use geom_core::sym::deferred;
    use geom_core::{Sym, SymBudget, SymRules};
    let opts = editor_core::EvalOptions {
        symbols: true,
        profile_lift: editor_core::ProfileLift::Guided,
        ..editor_core::EvalOptions::default()
    };
    let budget = SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    deferred::with_deferred_session(
        budget,
        SymRules::shipped(),
        geom_core::SymRetry::none(),
        || {
            let ev: Evaluation<Sym<f64>> = editor_core::evaluate(
                doc,
                None,
                &editor_core::CancelToken::new(),
                &opts,
                Tol::witness(),
            );
            ev.value(node)
                .expect("the node replays")
                .coincidences
                .iter()
                .flat_map(|r| deferred::render(r.margin))
                .collect()
        },
    )
    .0
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
    let (above_read, below_read) = (
        port(&doc, split, SplitHalf::Above),
        port(&doc, split, SplitHalf::Below),
    );
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![above_read.into(), below_read.into()]),
            declare: vec![
                (
                    (
                        SitedRef::new(above_read, above_cap.clone()),
                        SitedRef::new(below_read, below_cap.clone()),
                    ),
                    BooleanCoincidence::REST,
                ),
                (
                    (
                        SitedRef::new(above_read, wall(SplitHalf::Above, 0.75)),
                        SitedRef::new(below_read, wall(SplitHalf::Below, 0.25)),
                    ),
                    BooleanCoincidence::Continuation,
                ),
            ],
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let got = rows(&ev, union);
    let caps = [
        entity(above_read, &above_cap),
        entity(below_read, &below_cap),
    ];
    let row = got
        .iter()
        .find(|r| r.cells == caps)
        .unwrap_or_else(|| panic!("the caps' row: {got:?}"));
    assert_eq!(
        (row.relation, row.site),
        (Relation::SameOpposite, DecisionSite::PlaneLadder)
    );
    assert_eq!(
        proof(&doc, union, row),
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
    for proof in proofs(&doc, union, &got) {
        assert!(
            matches!(proof, Proof::Structural(_)),
            "every row of the reunion is one construction: {proof:?}"
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
fn apart(subtract: bool, id: &str) -> (ProfileDoc, Evaluation<f64>, RecipeNodeId, usize) {
    let doc = ProfileDoc::empty_derived(id, Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (2.0, 3.0), (0.0, 1.0), 0.0, 1.0);
    let pairs = crate::fixture::flush_pairs(&doc, (a, a), (b, b));
    let n = pairs.len();
    let declare = pairs
        .into_iter()
        .map(|p| (p, BooleanCoincidence::Continuation))
        .collect();
    let boolean = if subtract {
        Node::Subtract {
            from: a.into(),
            tool: b.into(),
            declare,
        }
    } else {
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
            declare,
        }
    };
    let (doc, node) = insert(doc, boolean);
    let ev = run(&doc);
    assert!(failure(&ev, node).is_none(), "{:?}", failure(&ev, node));
    (doc, ev, node, n)
}

/// **A declared pair whose faces never meet records no row, through
/// the containment fallback too.** Two blocks apart, their flush faces
/// declared: the union's boundaries never cross, so it is the
/// fallback's assembly, and the subtraction's is operand A whole (the
/// single-operand finish). The declaration door verified each pair one
/// carrier, but no face of one block meets a face of the other, so the
/// glue took no effect and the result records no row (D1).
#[test]
fn the_fallbacks_record_no_row_for_faces_that_never_meet() {
    for (op, subtract, id) in [
        ("union", false, "coincide-apart-union"),
        ("subtract", true, "coincide-apart-subtract"),
    ] {
        let (doc, ev, node, n) = apart(subtract, id);
        assert!(n > 0, "the premise: flush faces to declare");
        let got = rows(&ev, node);
        assert!(got.is_empty(), "{op:?}: no row for faces apart: {got:?}");
        assert!(unproven(&doc, &ev).is_empty(), "{op:?}");
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
    let CheckEvidence::UnprovenCoincidence { row, recourse, .. } = &findings[0].evidence else {
        panic!("{:?}", findings[0].evidence)
    };
    assert_eq!(**row, got[0]);
    assert_eq!(
        *recourse,
        coincide::Recourse::OneConstruction,
        "the junction's margin keeps no opaque atom: drawing the two pieces as one \
         construction would make it hold"
    );
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

/// The names of `node`'s edges with both ends at height `z`.
fn rim_named(ev: &Evaluation<f64>, node: RecipeNodeId, z: f64) -> Vec<StableName> {
    let body = match &ev.value(node).expect("the node evaluated").payload {
        ValuePayload::Body(b) => b.clone(),
        other => panic!("expected a body, got {other:?}"),
    };
    let height = |v| body.get_point(body.get_vertex(v).unwrap().point).unwrap().z;
    let mut rim: Vec<StableName> = table(ev, node)
        .iter()
        .filter_map(|(n, entry)| match entry {
            Entry::Unique(r) => match r.key {
                EntityKey::Edge(e) => {
                    let he = body.get_edge(e).unwrap().he_plus;
                    let ends = [
                        body.get_half_edge(he).unwrap().start,
                        body.half_edge_end(he).unwrap(),
                    ];
                    ends.iter()
                        .all(|&v| (height(v) - z).abs() < 1e-12)
                        .then(|| n.clone())
                }
                _ => None,
            },
            Entry::Tied(_) => None,
        })
        .collect();
    rim.sort();
    rim
}

/// **A blend's tangent joints and coaxial supports leave the battery**.
/// A disc (a circle profile extruded) filleted along its top rim: the
/// fillet holds one `Coaxial` row per rim link, decided at
/// `BatterySupportAxis` over the cap and the wall, and one
/// `Tangent { aligned: true }` row per junction, decided at
/// `BatteryJoint` over the two rim edges meeting there, each named by
/// the disc's own names. The cap and the wall are two constructions;
/// replayed with every variable a symbol, each coaxiality is the zero
/// polynomial, and each joint's form outgrows the door's budget, so the
/// joints alone are reported.
///
/// Red if either decision stops being recorded, or is not carried to
/// the fillet's value.
#[test]
fn a_filleted_disc_records_its_coaxial_supports_and_tangent_joints() {
    let doc = ProfileDoc::empty_derived("coincide-disc", Tol::witness());
    let (doc, p) = profile_of(
        doc,
        LoopProgram::Circle {
            centre: len2([0.0, 0.0]),
            radius: len(1.0),
        },
    );
    let (doc, disc) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
            side: editor_core::ExtrudeSide::Along,
        },
    );
    let ev = run(&doc);
    let rim = rim_named(&ev, disc, 1.0);
    let (doc, fillet) = insert(doc, Node::fillet(disc, len(0.1), rim.clone()));
    let ev = run(&doc);
    let got = rows(&ev, fillet);
    assert_eq!(rim.len(), 2, "the circle's rim is two arcs");
    assert_eq!(got.len(), 4, "two supports, two joints: {got:?}");
    let (axes, joints) = got.split_at(2);
    for row in axes {
        assert_eq!(
            (row.relation, row.site),
            (Relation::Coaxial, DecisionSite::BatterySupportAxis)
        );
        let kinds = row.cells.each_ref().map(|cell| match cell {
            NamedCell::Entity { input, name } if Some(*input) == doc.output(disc, 0) => name.kind,
            cell => panic!("a support is a face of the disc: {cell:?}"),
        });
        assert_eq!(kinds, [EntityKind::Face; 2], "{row:?}");
    }
    for row in joints {
        assert_eq!(
            (row.relation, row.site),
            (
                Relation::Tangent { aligned: true },
                DecisionSite::BatteryJoint
            )
        );
        let names = row.cells.each_ref().map(|cell| match cell {
            NamedCell::Entity { input, name } if Some(*input) == doc.output(disc, 0) => name,
            cell => panic!("a joint's cells are edges of the disc: {cell:?}"),
        });
        assert!(
            names[0] != names[1] && names.iter().all(|n| rim.contains(n)),
            "a joint names the disc's two rim edges: {row:?}"
        );
    }
    let proved = proofs(&doc, fillet, &got);
    assert_eq!(
        proved[..2],
        [const { Proof::Structural(Rung::MarginIdentity(geom_core::Theorem::SymbolicZero)) }; 2],
        "each support's departure from the shared axis is the zero polynomial"
    );
    for proof in &proved[2..] {
        assert!(
            matches!(proof, Proof::Unproven { residual, .. }
                if residual.replay == coincide::Unreplayed::Numeric),
            "a joint's form outgrows the door's budget: {proof:?}"
        );
    }
    assert_eq!(
        recourses(&proved[2..]),
        [Some(coincide::Recourse::OneConstruction); 2],
        "a joint's form freezes on the door's budget: no special recourse"
    );
    let findings = unproven(&doc, &ev);
    assert_eq!(
        findings.len(),
        2,
        "the report carries the door's verdict: the supports proven, the joints left: \
         {findings:?}"
    );
    let said: Vec<String> = findings.iter().map(|f| spoken_by(f, &doc)).collect();
    assert!(
        said.iter()
            .any(|s| s.contains("a blend chain's joint read as tangent")),
        "{said:?}"
    );
}

/// **A union records a vertex touch once, as a pair does.** Two blocks
/// kissing at a corner: their two-member union — the pair — holds one
/// `VertexFusion` row, cited by its one record. Folded into a union with
/// a third block apart, the pairwise judgement decides the same touch,
/// but its records are not the union's, so only the fold step's row,
/// which the union's record cites, is published: the union's rows and
/// findings are the pair's, the vertex touch once.
#[test]
fn a_union_records_a_vertex_touch_once() {
    let doc = ProfileDoc::empty_derived("coincide-union-kiss", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (1.0, 2.0), (1.0, 2.0), 1.0, 2.0);
    let (doc, far) = block(doc, (6.0, 7.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, pair) = crate::docm7_union_declare::declared_union(doc, &[a, b], Vec::new());
    // The kiss is the last step's, so its record is the union's.
    let (doc, union) = crate::docm7_union_declare::declared_union(doc, &[far, a, b], Vec::new());
    let ev = run(&doc);
    let mut all = Vec::new();
    for (what, node) in [("pair", pair), ("union", union)] {
        let got = rows(&ev, node);
        let fusions = got
            .iter()
            .filter(|r| r.site == DecisionSite::VertexFusion)
            .count();
        assert_eq!(
            fusions, 1,
            "{what}: one vertex row for the one kiss: {got:#?}"
        );
        let value = ev.value(node).expect("evaluated");
        let ValuePayload::Boolean(editor_core::BooleanValue::Body { contacts, .. }) =
            &value.payload
        else {
            panic!("{what}: a boolean body")
        };
        assert_eq!(contacts.len(), 1, "{what}: one record: {contacts:?}");
        all.push(got.len());
    }
    assert_eq!(all[0], all[1], "the union's rows are the pair's");
    let findings = unproven(&doc, &ev);
    let at = |node| {
        findings
            .iter()
            .filter(|f| matches!(f.subject, FindingSubject::Node(n) if n == node))
            .count()
    };
    assert_eq!(at(pair), at(union), "{findings:#?}");
}

/// **The door's cost on `node`'s rows, measured**: the deferred replay,
/// then each row's margin discharged, with its time and its DAG's size,
/// under `TERMS` terms (the drive's default unset) and the drive's
/// retry ladder unless `NORETRY`. Release builds; `--nocapture`.
fn measure_rows(doc: &ProfileDoc, node: RecipeNodeId) {
    use geom_core::sym::deferred;
    use geom_core::{Sym, SymBudget, SymRules};
    let opts = editor_core::EvalOptions {
        symbols: true,
        profile_lift: editor_core::ProfileLift::Guided,
        ..editor_core::EvalOptions::default()
    };
    let t = std::time::Instant::now();
    let ((), counts) = deferred::with_deferred_session(
        SymBudget {
            max_terms: std::env::var("TERMS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(editor_core::drive::DEFAULT_SYM_MAX_TERMS),
            max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
        },
        SymRules::shipped(),
        if std::env::var_os("NORETRY").is_some() {
            geom_core::SymRetry::none()
        } else {
            editor_core::drive::DEFAULT_SYM_RETRY
        },
        || {
            let ev: Evaluation<Sym<f64>> = editor_core::evaluate(
                doc,
                None,
                &editor_core::CancelToken::new(),
                &opts,
                Tol::witness(),
            );
            eprintln!("replay {:?}", t.elapsed());
            let rows = ev
                .value(node)
                .expect("the node replays")
                .coincidences
                .to_vec();
            for r in &rows {
                let t = std::time::Instant::now();
                let d = deferred::discharge(r.margin);
                eprintln!(
                    "ROW {:?} {:?} {:?} {:?} {:?}",
                    r.site,
                    r.relation,
                    d,
                    t.elapsed(),
                    deferred::dag_size(r.margin)
                );
                if d.is_none() {
                    for f in deferred::render(r.margin) {
                        eprintln!("  FORM {f}");
                    }
                }
            }
        },
    );
    eprintln!("total {:?} {counts:?}", t.elapsed());
}

#[test]
#[ignore = "a measurement: run in release with --nocapture"]
fn measure_the_box_mitre_at_sym() {
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
    let (doc, fillet) = insert(doc, Node::fillet(the_box, len(0.1), edges));
    measure_rows(&doc, fillet);
}

#[test]
#[ignore = "a measurement: run in release with --nocapture"]
fn measure_the_filleted_disc_at_sym() {
    let doc = ProfileDoc::empty_derived("coincide-disc", Tol::witness());
    let (doc, p) = profile_of(
        doc,
        LoopProgram::Circle {
            centre: len2([0.0, 0.0]),
            radius: len(1.0),
        },
    );
    let (doc, disc) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
            side: editor_core::ExtrudeSide::Along,
        },
    );
    let ev = run(&doc);
    let rim = rim_named(&ev, disc, 1.0);
    let (doc, fillet) = insert(doc, Node::fillet(disc, len(0.1), rim));
    measure_rows(&doc, fillet);
}

/// **The door binds every variable a symbol of its own; the analysis
/// lane binds an untoleranced one as its value**
/// (`unproven-coincidence-lint-binds-every-variable-as-a-symbol`). The
/// declared rest above stands a plate on a block whose height and the
/// plate's floor are two variables typed to one metre: the lint
/// reports the row (that test), and here the two environments read
/// their difference. In the analysis lane's environment both are the
/// constant 1, so the difference decides Zero as a theorem; in the
/// door's they are two symbols, so it decides Zero only by value.
#[test]
fn two_variables_typed_equal_are_one_constant_to_the_analysis_lane_and_two_symbols_to_the_door() {
    use editor_core::analysis::{ParamBox, var_env_over, var_env_symbolic};
    use editor_core::{FreeVar, ParamValue};
    use geom_core::{Band, Decide, Sym, SymBudget};
    let doc = ProfileDoc::empty_derived("coincide-two-metres", Tol::witness());
    let (doc, _) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, _) = block(doc, (0.5, 1.5), (0.5, 1.5), 1.0, 0.5);
    let metres: Vec<editor_core::VarId> = doc
        .free_vars()
        .filter_map(|(id, free)| match *free {
            FreeVar::Continuous { value, .. } => (value == 1.0).then_some(id),
            _ => None,
        })
        .collect();
    assert!(
        metres.len() >= 2,
        "the block's height and the plate's floor: {metres:?}"
    );
    let band = Band::linear(Tol::witness()).expect("a band");
    let difference = |env: &editor_core::VarEnv<Sym<f64>>| {
        let value = |id| match env.bindings.get(id) {
            Some(ParamValue::Continuous { value, .. }) => *value,
            other => panic!("{id:?} binds {other:?}"),
        };
        let decided = (value(&metres[0]) - value(&metres[1]))
            .sign_within(band)
            .expect("a difference of nothing decides");
        (decided.sign, decided.margin.theorem())
    };
    let budget = SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    let ((analysis, door), _) = geom_core::sym::with_session(budget, || {
        let over = var_env_over::<Sym<f64>, _>(&doc, &ParamBox::from_axes(Default::default()))
            .expect("the nominal box binds");
        let symbolic = var_env_symbolic::<Sym<f64>, _>(&doc).expect("every variable binds");
        (difference(&over), difference(&symbolic))
    });
    assert_eq!(
        analysis,
        (
            geom_core::Sign::Zero,
            Some(geom_core::Theorem::SymbolicZero)
        ),
        "the analysis lane reads two constants 1 and their difference is the zero polynomial"
    );
    assert_eq!(
        door,
        (geom_core::Sign::Zero, None),
        "the door reads two symbols: Zero at the values, no theorem"
    );
}

/// **A formula's constant is exact at `Sym`**: `Ratio::eval` embeds
/// its numerator and denominator and divides, and the symbolic tier
/// holds the quotient as the rational it is, so `(1/3)·3 − 1` and
/// `(1/10)·10 − 1` are the zero polynomial, where `f64` rounds both
/// quotients.
#[test]
fn a_ratio_evaluates_exactly_at_sym() {
    use geom_core::{Band, Decide, Sym, SymBudget};
    let band = Band::linear(Tol::witness()).expect("a band");
    let budget = SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    let (theorems, _) = geom_core::sym::with_session(budget, || {
        [(1, 3), (1, 10), (-7, 6)].map(|(num, den)| {
            let ratio = editor_core::Ratio::new(num, den).expect("in range");
            let whole = editor_core::Ratio::new(num, 1).expect("in range");
            let back = ratio.eval::<Sym<f64>>()
                * editor_core::Ratio::new(den as i64, 1)
                    .expect("in range")
                    .eval::<Sym<f64>>()
                - whole.eval::<Sym<f64>>();
            back.sign_within(band).expect("decides").margin.theorem()
        })
    });
    assert_eq!(theorems, [Some(geom_core::Theorem::SymbolicZero); 3]);
}

/// **What the door proves on the corpus**: every document's rows, by
/// the rung that proved each or why none did, and what the door costs:
/// the replay at `Sym` alone, and the whole door, whose difference is
/// the discharge. A census for the PR that changes a rung, not a gate.
#[test]
#[ignore = "a census: run in release with --nocapture"]
fn census_the_door_over_the_corpus() {
    for cd in crate::corpus::documents() {
        let ev = run(&cd.doc);
        let nodes: Vec<(RecipeNodeId, &[NamedCoincidence])> = ev
            .order
            .iter()
            .filter_map(|&node| ev.value(node).map(|v| (node, &*v.coincidences)))
            .filter(|(_, rows)| !rows.is_empty())
            .collect();
        let t = std::time::Instant::now();
        // The replay alone, as `coincide/replay.rs` runs it: its budget
        // and options spelled again here, kept in step with it by hand.
        let replay = (!nodes.is_empty()).then(|| {
            geom_core::sym::deferred::with_deferred_session(
                geom_core::SymBudget {
                    max_terms: 256,
                    max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
                },
                geom_core::SymRules::shipped(),
                editor_core::drive::DEFAULT_SYM_RETRY,
                || {
                    let opts = editor_core::EvalOptions {
                        symbols: true,
                        profile_lift: editor_core::ProfileLift::Guided,
                        ..editor_core::EvalOptions::default()
                    };
                    let _: Evaluation<geom_core::Sym<f64>> = editor_core::evaluate(
                        &cd.doc,
                        None,
                        &editor_core::CancelToken::new(),
                        &opts,
                        Tol::witness(),
                    );
                },
            );
            t.elapsed()
        });
        let t = std::time::Instant::now();
        let proofs = coincide::prove(
            &cd.doc,
            &nodes,
            Tol::witness(),
            &editor_core::CancelToken::new(),
        )
        .expect("an uncanceled door");
        let door = t.elapsed();
        let mut tally: std::collections::BTreeMap<String, usize> = Default::default();
        for ((_, rows), proofs) in nodes.iter().zip(&proofs) {
            for (row, proof) in rows.iter().zip(proofs) {
                let how = match proof {
                    Proof::Structural(rung) => format!("{rung:?}"),
                    Proof::Unproven { residual, recourse } => {
                        format!("unproven/{:?}/{recourse:?}", residual.replay)
                    }
                };
                *tally.entry(format!("{:?} {how}", row.site)).or_default() += 1;
            }
        }
        eprintln!(
            "CENSUS {} door {door:?} replay {replay:?} {tally:?}",
            cd.name
        );
    }
}

#[test]
#[ignore = "a measurement: CORPUS=<name>, release, --nocapture"]
fn measure_a_corpus_document_at_sym() {
    let name = std::env::var("CORPUS").expect("CORPUS names a corpus document");
    let cd = crate::corpus::documents()
        .into_iter()
        .find(|cd| cd.name == name)
        .expect("a corpus document");
    let ev = run(&cd.doc);
    for &node in &ev.order {
        if ev.value(node).is_some_and(|v| !v.coincidences.is_empty()) {
            eprintln!("NODE {node:?}");
            measure_rows(&cd.doc, node);
        }
    }
}

/// **The door's words for a row it did not prove**, each its own: the
/// replay found no identity, the row was not replayed at all, or it
/// stands on a part.
#[test]
fn an_unproven_row_says_why_the_replay_left_it() {
    use coincide::Unreplayed as U;
    assert_eq!(
        [U::Numeric, U::NotReplayed, U::ThroughAPart].map(|u| u.to_string()),
        [
            "with every variable a symbol, neither the carriers' verdict nor the deciding \
             margin is shown identically zero within the door's budget",
            "the row is not replayed with every variable a symbol",
            "the row stands on a part, and the replay with every variable a symbol resolves no \
             part",
        ]
    );
}

/// **What the door proves on the tour's composed dies**: the blend
/// joints whose `fillet3_chain_g1` margin is the zero polynomial. One of
/// `die_composed`'s two and 21 of `die_composed_tour`'s 42 prove as
/// theorems, and the rest stay unproven.
#[test]
fn the_door_proves_half_the_composed_dies_joints() {
    for (name, proved, rows) in [("die_composed", 1, 2), ("die_composed_tour", 21, 42)] {
        let cd = crate::corpus::documents()
            .into_iter()
            .find(|cd| cd.name == name)
            .unwrap_or_else(|| panic!("{name} is a corpus document"));
        let ev = run(&cd.doc);
        let nodes: Vec<(RecipeNodeId, &[NamedCoincidence])> = ev
            .order
            .iter()
            .filter_map(|&node| ev.value(node).map(|v| (node, &*v.coincidences)))
            .filter(|(_, rows)| !rows.is_empty())
            .collect();
        let all: Vec<Proof> = coincide::prove(
            &cd.doc,
            &nodes,
            Tol::witness(),
            &editor_core::CancelToken::new(),
        )
        .expect("an uncanceled door")
        .into_iter()
        .flatten()
        .collect();
        let theorems = all
            .iter()
            .filter(|p| {
                matches!(
                    p,
                    Proof::Structural(Rung::MarginIdentity(geom_core::Theorem::SymbolicZero))
                )
            })
            .count();
        let replayed = replays(&all);
        assert_eq!((theorems, replayed.len()), (proved, rows), "{name}");
        assert!(
            replayed
                .iter()
                .flatten()
                .all(|u| *u == coincide::Unreplayed::Numeric),
            "{name}: the rest are unproven: {replayed:?}"
        );
    }
}

/// A resolver that resolves nothing: what a run carries when it may
/// cross a part seam.
#[derive(Debug)]
struct NoParts;

impl editor_core::PartResolver for NoParts {
    fn resolve(
        &self,
        _: &editor_core::DocRef,
        _: Tol,
    ) -> Result<ProfileDoc, editor_core::ResolveFailure> {
        Err(editor_core::ResolveFailure {
            fault: editor_core::ResolveFault::Unresolved,
            message: "no parts here".to_owned(),
        })
    }
}

/// **A symbolic run takes one environment**: asked for symbols and a
/// parameter box too, it refuses every node, typed.
#[test]
fn a_symbolic_run_over_a_box_refuses_every_node() {
    use editor_core::{NodeErrorKind, NodeResult, ParamBox, ParamBoxError};
    let doc = ProfileDoc::empty_derived("coincide-symbols-box", Tol::witness());
    let (doc, _) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let opts = editor_core::EvalOptions {
        symbols: true,
        param_box: Some(std::sync::Arc::new(ParamBox::from_axes(
            std::collections::BTreeMap::new(),
        ))),
        ..editor_core::EvalOptions::default()
    };
    let ev: Evaluation<geom_core::Sym<f64>> = editor_core::evaluate(
        &doc,
        None,
        &editor_core::CancelToken::new(),
        &opts,
        Tol::witness(),
    );
    assert!(!ev.order.is_empty());
    for id in &ev.order {
        assert!(
            matches!(
                ev.result(*id),
                Some(NodeResult::Failed(e)) if matches!(
                    e.kind,
                    NodeErrorKind::ParamBox { source: ParamBoxError::SymbolsOverABox }
                )
            ),
            "{:?}",
            ev.result(*id)
        );
    }
}

/// **A symbolic run crosses no part seam.** Two documents built by the
/// same edits mint the same variable ids, and a symbol is keyed by its
/// id, so two parts' variables would be one symbol and their difference
/// a false identity. A run asked for symbols and a resolver refuses
/// every node, typed, before any runs.
#[test]
fn a_symbolic_run_with_a_resolver_refuses_every_node() {
    use editor_core::{NodeErrorKind, NodeResult, ParamBoxError};
    let doc = ProfileDoc::empty_derived("coincide-seam", Tol::witness());
    let (doc, _) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let opts = editor_core::EvalOptions {
        symbols: true,
        resolver: Some(std::sync::Arc::new(NoParts)),
        ..editor_core::EvalOptions::default()
    };
    let ev: Evaluation<geom_core::Sym<f64>> = editor_core::evaluate(
        &doc,
        None,
        &editor_core::CancelToken::new(),
        &opts,
        Tol::witness(),
    );
    assert!(!ev.order.is_empty());
    for id in &ev.order {
        assert!(
            matches!(
                ev.result(*id),
                Some(NodeResult::Failed(e)) if matches!(
                    e.kind,
                    NodeErrorKind::ParamBox { source: ParamBoxError::SymbolsAcrossAPartSeam }
                )
            ),
            "{:?}",
            ev.result(*id)
        );
    }
}

/// **The door enters a report one way.** On a filleted corpus document,
/// the registry split at rest reads the door pending and holds none of
/// its findings; its job, run and settled, gives back the synchronous
/// report exactly, findings in the same places. An expectation about a
/// subject the document lacks is a connectedness finding, which the
/// registry runs before the door: it stays first in both reports. Reds
/// if the door's findings go anywhere but after the residents before it.
#[test]
fn a_report_settled_at_rest_is_the_synchronous_report() {
    let cd = crate::corpus::documents()
        .into_iter()
        .find(|cd| cd.name == "reshaped_rod")
        .expect("a corpus document");
    let ev = run(&cd.doc);
    let tol = Tol::witness();
    let mut cfg = ChecksConfig::default();
    cfg.expected_components.insert((ev.order[0], 99), 1);
    let gathered = editor_core::product_recorded(&cd.doc, &ev, tol).expect("the product gathers");
    let sync = editor_core::run_checks_on(
        &cd.doc,
        &ev,
        editor_core::Subject::Product(&gathered),
        &cfg,
        tol,
    )
    .expect("the checks run");
    assert!(
        sync.pending.is_empty(),
        "a synchronous report pends on nothing"
    );
    assert_eq!(
        sync.findings.iter().map(|f| f.check).collect::<Vec<_>>()[..2],
        [CheckId::Connectedness, CheckId::UnprovenCoincidence],
        "the stale expectation, then the rod's support row: {sync}"
    );
    let (mut at_rest, job) = editor_core::run_checks_at_rest_on(
        &cd.doc,
        &ev,
        editor_core::Subject::Product(&gathered),
        &cfg,
        tol,
    )
    .expect("the checks run");
    assert_eq!(at_rest.pending, [CheckId::UnprovenCoincidence]);
    assert!(
        at_rest
            .findings
            .iter()
            .all(|f| f.check != CheckId::UnprovenCoincidence),
        "pending is not a finding: {at_rest}"
    );
    let job = job.expect("a row asks for the door");
    at_rest.settle(
        job.run(&cd.doc, &editor_core::CancelToken::new())
            .expect("an uncanceled door"),
    );
    assert_eq!(at_rest, sync);
}

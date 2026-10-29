//! **The coincidence door** (D10, INTENT stage 4 PR B): every
//! coincidence an operation decides from values is a row on its node's
//! value, its cells named in the tables of the inputs the decision
//! read; the door proves a row structural or leaves it unproven; and
//! the `unproven-coincidence` check reports what it leaves.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on, failure, run};
use crate::emit_shared_rim_several::{construction, own_name};
use crate::fixture::{fname, insert, len, len2, on_frame, out, table};

use editor_core::{
    Advisory, CapEnd, CheckEvidence, CheckId, ChecksConfig, EntityKey, EntityKind, Entry,
    EvalOptions, Evaluation, FindingSubject, Formula, LoopProgram, NamedCell, NamedCoincidence,
    Node, ProfileDoc, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, Proof,
    RecipeNodeId, RoleSeg, Rung, Severity, SplitHalf, StableName, ValuePayload, VarId, coincide,
};
use geom_core::{MarginDiag, Point3, Tol};
use topo::{DecisionSite, Relation};

/// The `unproven-coincidence` check alone, over `ev`.
fn unproven(doc: &ProfileDoc, ev: &Evaluation<f64>) -> Vec<editor_core::CheckFinding> {
    let cfg = ChecksConfig {
        connectedness: Severity::Off,
        at_rest: Severity::Off,
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
        .unwrap_or_else(|| {
            let root = match ev.nodes.get(&node) {
                Some(editor_core::NodeResult::Poisoned { through }) => *through,
                _ => node,
            };
            panic!("{node:?} evaluated: {root:?} {:?}", failure(ev, root))
        })
        .coincidences
        .to_vec()
}

/// The door's proof of each of `node`'s `rows`, given in its decision
/// order.
fn proofs(doc: &ProfileDoc, node: RecipeNodeId, rows: &[NamedCoincidence]) -> Vec<Proof> {
    coincide::prove(
        doc,
        &[(node, rows)],
        None,
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

/// **A glued rest is recorded** (§11 row 3). A plate resting flush on
/// a block, unioned: the union holds one `SameOpposite` row naming the
/// block's top and the plate's bottom by their operands' own names,
/// decided by the plane ladder, and the check reports it unproven: the
/// two caps are two constructions. The plate's four bottom corners,
/// each decided on the block's top, are a row each, reported alike.
#[test]
fn a_glued_rest_and_its_corners_are_unproven_rows_named_by_their_operands() {
    let doc = ProfileDoc::empty_derived("coincide-declared-rest", Tol::witness());
    let (doc, base) = block(doc, (0.0, 2.0), (0.0, 2.0), 1.0);
    let (doc, plate) = block_on(doc, base, (0.5, 1.5), (0.5, 1.5), 1.0, 0.5);
    let top = fname(base, RoleSeg::Cap(CapEnd::End));
    let bottom = own_name(
        &doc,
        plate,
        EntityKind::Face,
        vec![RoleSeg::Cap(CapEnd::Start)],
    );
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![base.into(), plate.into()]),
        },
    );
    let ev = run(&doc);
    let got = rows(&ev, union);
    assert_eq!(got.len(), 5, "the rest and its four corners: {got:?}");
    let row = &got[0];
    for corner in &got[1..] {
        assert!(
            corner.site == DecisionSite::VertexFusion
                && corner.relation == Relation::OnCarrier
                && matches!(&corner.cells[0], NamedCell::Entity { name, .. }
                    if name.node == construction(&doc, plate) && name.kind == EntityKind::Vertex)
                && corner.cells[1] == entity(out(&doc, base), &top),
            "a plate corner on the block's top: {corner:?}"
        );
    }
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
        "the reading's own margin, never a synthetic one"
    );
    let Proof::Unproven { residual, .. } = proof(&doc, union, row) else {
        panic!("the same-construction rung proves a glue of two extrudes")
    };
    assert!(
        matches!(residual.constructions.as_slice(), [Ok(a), Ok(b)] if a.origin != b.origin),
        "two constructions: {residual:?}"
    );
    let findings = unproven(&doc, &ev);
    assert_eq!(findings.len(), 5, "every row reported: {findings:?}");
    assert!(
        findings
            .iter()
            .all(|f| f.subject == FindingSubject::Node(union)),
        "attributed to the deciding node: {findings:?}"
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
    face_of(ev, node, &crate::fixture::in_space(&body), 0, p, n)
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
    face_of(
        ev,
        split,
        &crate::fixture::in_space(body),
        half.output_body(),
        p,
        n,
    )
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
    // The read holding output `index`, and the body's index in its table
    // (a split's half is body 0 of its own, NAMES N1).
    let (var, index) = ev.body_read_of(node, index).expect("the node evaluated");
    ev.table(var)
        .expect("the node evaluated")
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
/// placement** (§11 row 4). A box's placed copy split in two: each
/// half's `−x` wall is a piece of the one extruded wall, read through
/// the one placement, so a row over them is
/// `Structural(SameConstruction)`. The box's own wall against a half's
/// is two constructions — the placed copy is not the unplaced one — and
/// stays unproven.
///
/// The row is built here over the scene's real cells; the production
/// rows over a split's walls and caps are the reunion's
/// ([`a_reunited_splits_section_caps_are_one_construction`]).
#[test]
fn a_row_over_one_placed_construction_is_proven_the_same_construction() {
    let doc = ProfileDoc::empty_derived("coincide-same-source", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, placed) = crate::fixture::slid(doc, the_box, the_box, [3.0, 0.0, 0.0]);
    let tool = crate::fixture::plane_on(&doc, the_box, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: placed.into(),
            tool,
        },
    );
    let ev = run(&doc);
    let wall = |half, z| half_face_on(&ev, split, half, [3.0, 0.5, z], [-1.0, 0.0, 0.0]);
    let row = |cells: [NamedCell; 2]| NamedCoincidence {
        cells: cells.to_vec(),
        origin: editor_core::RowOrigin::Node,
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
/// overlapping in x, their caps and y-walls flush, unioned as `[a, b]`
/// and as `[b, a]`: the same decisions either way, a row per flush
/// family, each its first cell the listed-first member's face, named by
/// that member's own table, and a row per corner of each block decided
/// on the other's boundary.
#[test]
fn a_unions_rows_follow_its_member_order() {
    let doc = ProfileDoc::empty_derived("coincide-union-order", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let n = crate::fixture::flush_segs(&doc, a).len();
    let mut seen = Vec::new();
    for members in [[a, b], [b, a]] {
        let (doc, union) = crate::fixture::union_over(doc.clone(), &members);
        let ev = run(&doc);
        let got = rows(&ev, union);
        let (faces, corners): (Vec<_>, Vec<_>) = got
            .iter()
            .partition(|r| r.site == DecisionSite::PlaneLadder);
        assert_eq!(faces.len(), n, "{members:?}: one row per flush family");
        assert!(
            corners.len() == 8
                && corners
                    .iter()
                    .all(|r| (r.relation, r.site)
                        == (Relation::OnCarrier, DecisionSite::VertexFusion)),
            "{members:?}: the four corners of each block inside the other's x span: {corners:?}"
        );
        for row in faces {
            assert_eq!(row.relation, Relation::SameOriented);
            let input = |c: &NamedCell| match c {
                // A copy carries its source's names, so the cell's
                // name is the source's and its input the member read.
                NamedCell::Entity { input, .. }
                    if members.iter().any(|&m| out(&doc, m) == *input) =>
                {
                    *input
                }
                other => panic!("{members:?}: a cell is a member's own face: {other:?}"),
            };
            assert_eq!(
                row.cells.iter().map(input).collect::<Vec<_>>(),
                members.map(|m| out(&doc, m)),
                "{members:?}: the listed-first member's cell first"
            );
        }
        seen.push(got);
    }
    let unordered = |got: &[NamedCoincidence]| {
        let mut pairs: Vec<Vec<String>> = got
            .iter()
            .map(|r| {
                let mut pair: Vec<String> = r.cells.iter().map(|c| format!("{c:?}")).collect();
                pair.sort();
                pair
            })
            .collect();
        pairs.sort();
        pairs
    };
    assert_eq!(
        unordered(&seen[0]),
        unordered(&seen[1]),
        "the same decisions in either order"
    );
}

/// **A family's members are placed apart.** The walk reads each
/// member's placement off its `Member` segment, so member 1's top and
/// member 2's are two constructions even though one extrude minted
/// both, while one member's top read twice is one.
#[test]
fn a_familys_members_are_two_constructions_of_one_minted_face() {
    let doc = ProfileDoc::empty_derived("coincide-pattern", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, pattern) = crate::fixture::linear(doc, the_box, [1.0, 0.0, 0.0], 3.0, 3);
    let part = |doc: ProfileDoc, i: i64| crate::fixture::member(doc, pattern, i);
    let (doc, first) = part(doc, 1);
    // One member, read twice at one copy of it: two copies of a member
    // are two placements.
    let again = first;
    let (doc, second) = part(doc, 2);
    let ev = run(&doc);
    let top_at = |node, x: f64| face_on(&ev, node, [x, 0.5, 1.0], [0.0, 0.0, 1.0]);
    let row = |cells: [NamedCell; 2]| NamedCoincidence {
        cells: cells.to_vec(),
        origin: editor_core::RowOrigin::Node,
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
        "one member read twice"
    );
    let two = row([
        entity(out(&doc, first), &top_at(first, 3.5)),
        entity(out(&doc, second), &top_at(second, 6.5)),
    ]);
    let Proof::Unproven { residual, .. } = proof(&doc, second, &two) else {
        panic!("two members are two placements")
    };
    let [Ok(a), Ok(b)] = residual.constructions.as_slice() else {
        panic!("both cells walk to the box: {residual:?}")
    };
    assert_eq!(a.origin, b.origin, "one extrude minted both tops");
    assert_ne!(a.maps, b.maps, "placed by two members");
    assert_eq!(
        residual.to_string(),
        "the two cells are one construction placed or reflected apart; the replay with every variable a \
         symbol took other decisions there",
        "a row no node decided matches no row of the replay"
    );
}

/// The name of `node`'s edge between two points.
fn edge_named(ev: &Evaluation<f64>, node: RecipeNodeId, a: [f64; 3], b: [f64; 3]) -> StableName {
    let body = match &ev.value(node).expect("the node evaluated").payload {
        ValuePayload::Body(b) => crate::fixture::in_space(b),
        ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. }) => {
            crate::fixture::in_space(body)
        }
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
/// `CommonFoot` rows, each naming the two rim edges that turn there
/// by the box's own names, and each is unproven: the two edges are two
/// constructions, and replayed with every variable a symbol neither of
/// the turn's readings — the gap between its two bands' feet and the
/// overrun chord — is reduced to the zero polynomial
/// (`work/sym/the-mitres-feet-distance-freezes-at-sym.md`).
#[test]
fn a_filleted_box_records_one_unproven_turn_per_corner() {
    let doc = ProfileDoc::empty_derived("coincide-mitre", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 2.0), (0.0, 1.5), 1.0);
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
            (Relation::CommonFoot, DecisionSite::BatteryTurn)
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
        [Some(coincide::Recourse::Budget); 8],
        "the feet distance freezes on the door's budget, which is no value computed apart \
         from the variables: the recourse names the budget"
    );
    assert_eq!(unproven(&doc, &ev).len(), 8);
}

/// **A supplementary chamfer's turn reaches the door unproven.** The
/// sheared box `{0 ≤ z ≤ 1, s z ≤ x ≤ 2 + s z, s z ≤ y ≤ 1.5 + s z}`,
/// two sheared prisms intersected, has a top corner `(2 + s, s, 1)`
/// whose two top edges make supplementary angles with its lateral edge:
/// the chamfer's feet on that edge are one point, so its two top edges
/// chamfered mitre and record one `CommonFoot` row, which the door takes
/// and does not prove — the same freeze as the box's
/// (`work/sym/the-mitres-feet-distance-freezes-at-sym.md`).
#[test]
fn a_supplementary_chamfer_turn_is_recorded_unproven() {
    let s = 0.3;
    let (z0, z1) = (-0.5, 1.5);
    let doc = ProfileDoc::empty_derived("coincide-supplementary", Tol::witness());
    // Each prism is drawn on its own xy. The `y`-running prism is the
    // root, its own frame the world's plane `y = 3.5` (x along x, y
    // along z, extruded toward −y); the `x`-running one stands on the
    // plane `x = 3.5` (x along −y, y along z, extruded toward −x),
    // spelled in the root's own coordinates, where a world point `p`
    // reads `(p.x, p.z, 3.5 − p.y)`.
    let identity = ([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let (doc, along_y) = on_frame(
        doc,
        identity.0,
        identity.1,
        identity.2,
        vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0 + s, 1.0), (s, 1.0)]],
    );
    let (doc, along_x) = on_frame(
        doc,
        identity.0,
        identity.1,
        identity.2,
        vec![vec![
            (-1.5 - s * z0, z0),
            (-s * z0, z0),
            (-s * z1, z1),
            (-1.5 - s * z1, z1),
        ]],
    );
    let extrude = |profile: RecipeNodeId| Node::Extrude {
        profile: profile.into(),
        distance: len(5.0),
    };
    let (doc, y) = insert(doc, extrude(along_y));
    let (doc, x) = insert(doc, extrude(along_x));
    let (doc, x) = crate::fixture::against(
        doc,
        y,
        x,
        &crate::fixture::axes_at([3.5, 0.0, 3.5], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0]),
    );
    let (doc, sheared) = insert(
        doc,
        Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![y.into(), x.into()]),
        },
    );
    let ev = run(&doc);
    let into_y = |p: [f64; 3]| [p[0], p[2], 3.5 - p[1]];
    let w = into_y([2.0 + s, s, 1.0]);
    let edges = vec![
        edge_named(&ev, sheared, w, into_y([s, s, 1.0])),
        edge_named(&ev, sheared, w, into_y([2.0 + s, 1.5 + s, 1.0])),
    ];
    let (doc, chamfer) = insert(doc, Node::chamfer(sheared, len(0.1), edges));
    let ev = run(&doc);
    let got = rows(&ev, chamfer);
    let [row] = &got[..] else {
        panic!("one turn, one row: {got:?}")
    };
    assert_eq!(
        (row.relation, row.site),
        (Relation::CommonFoot, DecisionSite::BatteryTurn)
    );
    let proved = proofs(&doc, chamfer, &got);
    assert!(
        matches!(proved[..], [Proof::Unproven { .. }]),
        "the door takes the supplementary turn and does not prove it: {proved:?}"
    );
    assert_eq!(unproven(&doc, &ev).len(), 1);
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
        },
    );
    let tool = crate::fixture::plane_on_frame(&doc, prism, [0.0, at, 0.0], [0.0, 1.0, 0.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: prism.into(),
            tool,
        },
    );
    let ev = run(&doc);
    (doc, ev, split)
}

/// **A cut through a vertex records nothing** (§11 row 6's second
/// half): a diamond prism split by `y = 1` has four operand vertices ON
/// the plane (the diamond's side corners, at both ends), each with one
/// run on either side, so the plane cuts there and leaves no pieces
/// touching.
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

/// **A split through a pinch records it** (§11 row 6's first half): the
/// notched prism's tip `(4, 1)` has both neighbours above `y = 1`, so the
/// split leaves two pieces touching there, one row per tip vertex, each
/// the operand vertex the split's side verdict read ON the tool.
#[test]
fn a_split_through_a_pinch_records_one_row_per_pinched_vertex() {
    let notched = [
        (0.0, 0.0),
        (8.0, 0.0),
        (8.0, 2.0),
        (7.0, 1.0),
        (6.0, 1.0),
        (5.0, 2.0),
        (4.0, 1.0),
        (3.0, 2.0),
        (0.0, 2.0),
    ];
    let (_, ev, split) = split_prism("coincide-pinch", &notched, 1.0);
    let rows = rows(&ev, split);
    assert_eq!(rows.len(), 2, "one row per tip vertex: {rows:?}");
    for row in &rows {
        assert_eq!(
            (row.relation, row.site),
            (Relation::OnCarrier, DecisionSite::SplitOn)
        );
        assert!(
            matches!(
                (&row.cells[0], &row.cells[1]),
                (NamedCell::Entity { name, .. }, NamedCell::Tool { .. })
                    if name.kind == EntityKind::Vertex
            ),
            "an operand vertex on the tool: {row:?}"
        );
    }
    assert_ne!(rows[0].cells[0], rows[1].cells[0], "the two tip vertices");
}

/// **A pinch row's tool cell is the split's tool read, whatever pose
/// the tool is**: a frame read at its `xy`, a plane slid off a face,
/// and that slide named. None is an operation's output, so the cell is keyed by the
/// read, with the role it is read through.
#[test]
fn a_pinch_rows_tool_cell_is_the_tool_read_for_every_pose_tool() {
    let notched = [
        (0.0, 0.0),
        (8.0, 0.0),
        (8.0, 2.0),
        (7.0, 1.0),
        (6.0, 1.0),
        (5.0, 2.0),
        (4.0, 1.0),
        (3.0, 2.0),
        (0.0, 2.0),
    ];
    for shape in ["frame", "slide", "named slide"] {
        let doc = ProfileDoc::empty_derived(&format!("coincide-pinch-{shape}"), Tol::witness());
        let (doc, p) = on_frame(
            doc,
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![notched.to_vec()],
        );
        let (doc, prism) = insert(
            doc,
            Node::Extrude {
                profile: p.into(),
                distance: len(1.0),
            },
        );
        let tool = if shape == "frame" {
            crate::fixture::plane_on_frame(&doc, prism, [0.0, 1.0, 0.0], [0.0, 1.0, 0.0])
        } else {
            crate::fixture::plane_on(&doc, prism, [0.0, 1.0, 0.0], [0.0, 1.0, 0.0])
        };
        let (doc, split) = insert(
            doc,
            Node::Split {
                target: prism.into(),
                tool,
            },
        );
        let Some(Node::Split { tool, .. }) = doc.node(split).cloned() else {
            panic!("{shape}: the split")
        };
        let doc = if shape == "named slide" {
            crate::fixture::step(
                doc,
                editor_core::DocEdit::RenameVar {
                    var: tool.read.into(),
                    name: Some(editor_core::VarName::from_static("cut")),
                },
            )
            .0
        } else {
            doc
        };
        assert!(
            doc.var(tool.read)
                .is_some_and(|var| var.def().pose().is_some())
                && doc.operation_of(tool.read).is_none(),
            "{shape}: the premise: the tool is a pose definition, no operation's output"
        );
        let rows = rows(&run(&doc), split);
        assert_eq!(rows.len(), 2, "{shape}: one row per tip vertex: {rows:?}");
        for row in &rows {
            assert_eq!(
                row.cells[1],
                NamedCell::Tool {
                    input: tool.read,
                    role: tool.role,
                },
                "{shape}: the tool cell is the split's tool read"
            );
        }
    }
}

/// **Turning the check off removes the reports and nothing else**
/// (§14 Q2): the report lists it skipped, and the evaluation it read is
/// the one it was handed.
#[test]
fn the_check_off_is_visibly_skipped() {
    let doc = ProfileDoc::empty_derived("coincide-off", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 2.0), (0.0, 1.5), 1.0);
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
/// unioned again, the section caps resting on each other: the union records
/// one `SameOpposite` row over the two caps, and the door proves it,
/// since both lie on the one tool plane the split read. The whole
/// check run over the document is quiet: the lint's silent half, end
/// to end.
#[test]
fn a_reunited_splits_section_caps_are_one_construction() {
    let doc = ProfileDoc::empty_derived("coincide-reunite", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let tool = crate::fixture::plane_on(&doc, the_box, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: the_box.into(),
            tool,
        },
    );
    let ev = run(&doc);
    let cap = |half, z, nz| half_face_on(&ev, split, half, [0.5, 0.5, z], [0.0, 0.0, nz]);
    let (above_cap, below_cap) = (
        cap(SplitHalf::Above, 0.5, -1.0),
        cap(SplitHalf::Below, 0.5, 1.0),
    );
    assert!(
        matches!(above_cap.path.as_slice(), [RoleSeg::SectionFace])
            && matches!(below_cap.path.as_slice(), [RoleSeg::SectionFace]),
        "the premise: each cap is a section face of the split, {above_cap:?}, {below_cap:?}"
    );
    let (above_read, below_read) = (
        port(&doc, split, SplitHalf::Above),
        port(&doc, split, SplitHalf::Below),
    );
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![above_read.into(), below_read.into()]),
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
        NamedCell::Entity { input, name } => {
            coincide::construction(&doc, *input, name, Tol::witness())
        }
        NamedCell::Tool { .. } | NamedCell::Piece { .. } => unreachable!(),
    }) else {
        panic!("both caps walk")
    };
    let Some(Node::Split { tool, .. }) = doc.node(split) else {
        panic!("the split")
    };
    assert_eq!(a.origin, coincide::Origin::Pose(tool.read, tool.role));
    assert_eq!(a, b);
    // The halves' copies of one crossing are one construction: the
    // split, the crossed edge and the sense against the `Above` half.
    for proof in proofs(&doc, union, &got) {
        assert_eq!(
            proof,
            Proof::Structural(Rung::SameConstruction),
            "every row of the reunion is one construction, with no replay"
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

/// Whether `cell` names a split's crossing vertex (a placement's copy
/// carries it verbatim).
fn a_crossing(cell: &NamedCell) -> bool {
    matches!(
        cell,
        NamedCell::Entity { name, .. }
            if matches!(name.path.first(), Some(RoleSeg::CrossingVertex { .. }))
    )
}

/// **A split's crossing vertex lies on its tool's plane, and so on the
/// face the plane is read off.** A unit block `X` standing in a wider
/// slab `Y` half as tall is split by the plane of `Y`'s top, and `X`'s
/// upper half is unioned with `Y`. The four crossings of `X`'s struts
/// lie on `Y`'s top as the split built them, and the section face is
/// `Y`'s top read again, so every row the union decides there is the
/// first rung's, with no replay. The replay cannot prove the crossings
/// at any budget: the plane's normal is the document's, opaque to the
/// tier.
///
/// *Breaks if* a crossing vertex is not read as lying on its split's
/// tool plane.
#[test]
fn a_split_on_a_faces_plane_lays_its_crossings_on_that_face() {
    let doc = ProfileDoc::empty_derived("coincide-crossings-on-a-face", Tol::witness());
    let (doc, slab) = block(doc, (-1.0, 2.0), (-1.0, 2.0), 0.5);
    let (doc, x) = block_on(doc, slab, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let tool = crate::fixture::plane_on(&doc, slab, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: x.into(),
            tool,
        },
    );
    let upper = port(&doc, split, SplitHalf::Above);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![upper.into(), slab.into()]),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let got = rows(&ev, union);
    let crossings: Vec<&NamedCoincidence> = got
        .iter()
        .filter(|r| r.relation == Relation::OnCarrier && r.cells.iter().any(a_crossing))
        .collect();
    assert_eq!(
        crossings.len(),
        4,
        "the premise: each strut's crossing on the slab's top: {got:?}"
    );
    for (row, proof) in got.iter().zip(proofs(&doc, union, &got)) {
        assert_eq!(
            proof,
            Proof::Structural(Rung::SameConstruction),
            "the union's row over the slab's top is one construction: {row:?}"
        );
    }
}

/// **A half's copy of an operand vertex the tool passes through stays
/// a coincidence of values**: a unit cube turned off every axis stands
/// in a slab, levelled so that one of its middle corners is at the
/// slab's top by the numbers alone. The cube is split by the plane of
/// the slab's top, and its upper half is unioned with the slab. That
/// corner's copy (`OnToolVertex`) lies on the slab's top where its
/// values put it, not as the split built it, so the row laying it
/// there is no first-rung proof, and the check reports it. The
/// crossings beside it lie on the top by construction.
///
/// *Breaks if* an `OnToolVertex` is read as lying on its split's tool
/// by construction.
#[test]
fn a_corner_the_tool_passes_through_is_not_on_it_by_construction() {
    use geom_core::Point3;
    let band = geom_core::Band::linear(Tol::witness()).expect("the witness band");
    let turn =
        editor_core::Frame::rotate_then_translate([1.0, 2.0, 3.0], 0.4, [0.5, 0.5, 0.25], band)
            .expect("a turn about a unit axis");
    let mut heights: Vec<f64> = [0.0, 1.0]
        .into_iter()
        .flat_map(|x| {
            [0.0, 1.0]
                .into_iter()
                .flat_map(move |y| [0.0, 1.0].map(|z| [x, y, z]))
        })
        .map(|[x, y, z]| turn.affine::<f64>().transform_point(Point3::new(x, y, z)).z)
        .collect();
    heights.sort_by(f64::total_cmp);
    let level = heights[3];
    let doc = ProfileDoc::empty_derived("coincide-on-tool-vertex", Tol::witness());
    let (doc, slab) = block(doc, (-2.0, 3.0), (-2.0, 3.0), level);
    let (doc, cube) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, cube) = crate::fixture::against(doc, slab, cube, &turn);
    let tool = crate::fixture::plane_on(&doc, slab, [0.0, 0.0, level], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: cube.into(),
            tool,
        },
    );
    let upper = port(&doc, split, SplitHalf::Above);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![upper.into(), slab.into()]),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let got = rows(&ev, union);
    let on_tool = |cell: &NamedCell| {
        matches!(cell, NamedCell::Entity { name, .. }
            if matches!(name.path.first(), Some(RoleSeg::OnToolVertex { .. })))
    };
    let proven = proofs(&doc, union, &got);
    let laid: Vec<(&NamedCoincidence, &Proof)> = got
        .iter()
        .zip(&proven)
        .filter(|(r, _)| r.relation == Relation::OnCarrier && r.cells.iter().any(on_tool))
        .collect();
    assert_eq!(
        laid.len(),
        1,
        "the premise: the levelled corner laid on the slab's top: {got:?}"
    );
    for (row, proof) in &laid {
        assert!(
            matches!(proof, Proof::Unproven { .. }),
            "a corner on the tool by its values is unproven: {row:?} {proof:?}"
        );
    }
    for (row, proof) in got.iter().zip(&proven) {
        if row.cells.iter().any(a_crossing) {
            assert_eq!(
                *proof,
                Proof::Structural(Rung::SameConstruction),
                "a crossing lies on the top by construction: {row:?}"
            );
        }
    }
}

/// **A crossing of a half a placement moves is not on the unmoved
/// face by construction**: [`a_split_on_a_faces_plane_lays_its_crossings_on_that_face`]'s
/// scene, with the upper half placed `(0.25, 0, 0)` along the slab
/// before the union. Each crossing's construction carries the
/// placement's map and the slab's top does not, so though the half
/// still rests on the top, no crossing row is the first rung's.
///
/// *Breaks if* a crossing is laid on its tool's face without the maps
/// above the split.
#[test]
fn a_placed_halfs_crossings_are_not_on_the_unplaced_face_by_construction() {
    let doc = ProfileDoc::empty_derived("coincide-placed-half", Tol::witness());
    let (doc, slab) = block(doc, (-1.0, 2.0), (-1.0, 2.0), 0.5);
    let (doc, x) = block_on(doc, slab, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let tool = crate::fixture::plane_on(&doc, slab, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: x.into(),
            tool,
        },
    );
    let upper = port(&doc, split, SplitHalf::Above);
    let (doc, moved) = crate::fixture::against(
        doc,
        slab,
        upper,
        &editor_core::Frame::translation([0.25, 0.0, 0.0]),
    );
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![moved.into(), slab.into()]),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let got = rows(&ev, union);
    let crossings: Vec<(&NamedCoincidence, Proof)> = got
        .iter()
        .zip(proofs(&doc, union, &got))
        .filter(|(r, _)| r.relation == Relation::OnCarrier && r.cells.iter().any(a_crossing))
        .collect();
    assert_eq!(
        crossings.len(),
        4,
        "the premise: the four crossings: {got:?}"
    );
    for (row, proof) in crossings {
        assert_ne!(
            proof,
            Proof::Structural(Rung::SameConstruction),
            "a placed crossing is read on the unplaced top by construction: {row:?}"
        );
    }
}

/// **An arc crossed twice reunites**: a disc's cylinder split by a
/// plane off its axis, so each rim circle is crossed twice and each
/// half holds one crossing of each sense on it, then the halves
/// unioned again. The halves' copies pair per crossing, the `Above`
/// half's `Enters` with the `Below` half's `Leaves`, so every crossing
/// row is the first rung's.
///
/// *Breaks if* the two halves' copies of a crossing are read with each
/// half's own sense, or ranked apart.
#[test]
fn an_arc_crossed_twice_reunites_by_construction() {
    let doc = ProfileDoc::empty_derived("coincide-arc-twice", Tol::witness());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::circle(0.0, 0.0, 0.5).expect("a circle")],
            ids: Vec::new(),
        }),
    );
    let (doc, disc) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let (doc, wall) = block_on(doc, disc, (-1.0, 1.0), (-1.0, 0.2), 0.0, 1.0);
    let tool = crate::fixture::plane_on(&doc, wall, [0.0, 0.2, 0.0], [0.0, 1.0, 0.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: disc.into(),
            tool,
        },
    );
    let (above, below) = (
        port(&doc, split, SplitHalf::Above),
        port(&doc, split, SplitHalf::Below),
    );
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![above.into(), below.into()]),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let got = rows(&ev, union);
    let crossings: Vec<(&NamedCoincidence, Proof)> = got
        .iter()
        .zip(proofs(&doc, union, &got))
        .filter(|(r, _)| r.cells.iter().any(a_crossing))
        .collect();
    assert_eq!(
        crossings.len(),
        4,
        "the premise: two crossings on each rim: {got:?}"
    );
    for (row, proof) in crossings {
        assert_eq!(
            proof,
            Proof::Structural(Rung::SameConstruction),
            "a crossing's two copies are one construction: {row:?}"
        );
    }
}

/// **A section edge lies on the operand face it runs across**: a row
/// laying the upper half's section edge on the lower half's fragment
/// of that edge's wall is the first rung's, and one laying it on the
/// fragment of a wall it does not cross is not. The rows are written
/// here: no node decides an edge on a face of the other half.
///
/// *Breaks if* a section edge is not read as on its wall, or is read
/// as on every wall.
#[test]
fn a_section_edge_lies_on_the_wall_it_runs_across() {
    let doc = ProfileDoc::empty_derived("coincide-section-edge", Tol::witness());
    let (doc, cube) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let tool = crate::fixture::plane_on(&doc, cube, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: cube.into(),
            tool,
        },
    );
    let (above, below) = (
        port(&doc, split, SplitHalf::Above),
        port(&doc, split, SplitHalf::Below),
    );
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![above.into(), below.into()]),
        },
    );
    let ev = run(&doc);
    let names = |half| -> Vec<StableName> {
        crate::fixture::half(&ev, split, half)
            .iter()
            .map(|(name, _)| name.clone())
            .collect()
    };
    let (upper, lower) = (names(SplitHalf::Above), names(SplitHalf::Below));
    let (edge, across) = upper
        .iter()
        .find_map(|n| match n.path.first() {
            Some(RoleSeg::SectionEdge { face }) => Some((n.clone(), face.clone())),
            _ => None,
        })
        .expect("the premise: a section edge of the upper half");
    let fragment = |of: &dyn Fn(&StableName) -> bool| {
        lower
            .iter()
            .find(|n| {
                n.kind == EntityKind::Face
                    && matches!(n.path.first(), Some(RoleSeg::SplitFragment { parent }) if of(parent))
            })
            .cloned()
            .expect("the premise: a fragment of the lower half")
    };
    let its_wall = fragment(&|parent| *parent == *across);
    let another = fragment(&|parent| {
        *parent != *across && matches!(parent.path.first(), Some(RoleSeg::Lateral(_)))
    });
    let laid = |face: &StableName| {
        let margin = MarginDiag::value(0.0);
        NamedCoincidence {
            cells: vec![entity(above, &edge), entity(below, face)],
            relation: Relation::OnCarrier,
            site: DecisionSite::VertexFusion,
            margin,
            discharge: topo::Discharge::of(margin),
            origin: coincide::RowOrigin::Node,
        }
    };
    assert_eq!(
        proof(&doc, union, &laid(&its_wall)),
        Proof::Structural(Rung::SameConstruction),
        "the section edge on its own wall's fragment"
    );
    assert_ne!(
        proof(&doc, union, &laid(&another)),
        Proof::Structural(Rung::SameConstruction),
        "the section edge on a wall it does not cross"
    );
}

/// Two unit blocks apart along x, their caps and y-walls on one
/// carrier each, joined by `op`: the boolean's own rows and its node.
fn apart(subtract: bool, id: &str) -> (ProfileDoc, Evaluation<f64>, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived(id, Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (2.0, 3.0), (0.0, 1.0), 0.0, 1.0);
    let boolean = if subtract {
        Node::Subtract {
            from: a.into(),
            tool: b.into(),
        }
    } else {
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        }
    };
    let (doc, node) = insert(doc, boolean);
    let ev = run(&doc);
    assert!(failure(&ev, node).is_none(), "{:?}", failure(&ev, node));
    (doc, ev, node)
}

/// **Faces on one carrier that never meet record no row, through the
/// containment fallback too.** Two blocks apart, their caps and y-walls
/// on one carrier each: the union's boundaries never cross, so it is
/// the fallback's assembly, and the subtraction's is operand A whole
/// (the single-operand finish). No face of one block meets a face of
/// the other, so the result records no row (D1).
#[test]
fn the_fallbacks_record_no_row_for_faces_that_never_meet() {
    for (op, subtract, id) in [
        ("union", false, "coincide-apart-union"),
        ("subtract", true, "coincide-apart-subtract"),
    ] {
        let (doc, ev, node) = apart(subtract, id);
        let got = rows(&ev, node);
        assert!(got.is_empty(), "{op:?}: no row for faces apart: {got:?}");
        assert!(unproven(&doc, &ev).is_empty(), "{op:?}");
    }
}

/// A one-loop profile program.
fn profile_of(doc: ProfileDoc, steps: LoopProgram<Formula>) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![steps],
            ids: Vec::new(),
        }),
    )
}

/// **A profile junction the lattice reads as a corner is no row on its
/// profile** (§11 row 21, at the document). A line meets an arc at a
/// turn `φ = √ε`: `sin φ · arm`, far past the band, which the lattice
/// reads as a corner, and so does validation's joint pass, which reads
/// the same turn (`path_junction_turn`). The profile builds with no
/// tangent joint, so it carries no `ProfileJunction` row and the check
/// reports nothing; a circle, whose joints its form constructs, holds
/// none either. A junction a document's profile could decide tangent
/// would have to read Zero at validation's arm, which is at least the
/// lattice's, and the lattice refuses an undeclared Zero junction, so
/// no document reaches that arm
/// (`work/coincide/a-profile-junction-row-has-no-reaching-document.md`).
///
/// Red if the junction refuses, or a row is recorded for it or for the
/// circle.
#[test]
fn a_profile_junction_the_lattice_reads_as_a_corner_is_no_row() {
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
    assert!(
        rows(&ev, p).is_empty(),
        "the junction is a corner: {:?}",
        rows(&ev, p)
    );
    assert!(
        rows(&ev, circle).is_empty(),
        "a circle's joints are constructed"
    );
    let findings = unproven(&doc, &ev);
    assert!(findings.is_empty(), "{findings:?}");
}

/// **A same-carrier junction no constructor made refuses at the
/// door.** A leg of `ε/10` rise over 1 mm turns off the 10 m line by
/// `1e-7` rad. Levered by the shorter leg's arm, the lattice reads that
/// turn as `0.1ε`, Zero, and refuses the undeclared tangent junction at
/// step 2, as validation reads the same leg's far end within ε of the
/// line, one carrier continuing. Before the shorter-arm reading the
/// lattice levered the 10 m arriving leg and built a corner validation
/// recorded as one `SameOriented` row at `ProfileJunction`; with both
/// reading the short leg, a document reaches neither of that arm's
/// relations (`work/coincide/a-profile-junction-row-has-no-reaching-document.md`).
///
/// Red if the junction builds, or refuses for anything but its turn.
#[test]
fn a_profile_junction_on_one_carrier_refuses_at_the_door() {
    let rise = 0.1 * Tol::witness().eps();
    let doc = ProfileDoc::empty_derived("g_profile_same_carrier", Tol::witness());
    let node = Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::Chain(vec![
            ProgramStep::At(len2([0.0, 0.0])),
            ProgramStep::LineTo(ProgramTarget::Point(len2([10.0, 0.0]))),
            ProgramStep::LineTo(ProgramTarget::Point(len2([10.001, rise]))),
            ProgramStep::LineTo(ProgramTarget::Point(len2([10.001, 5.0]))),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])],
        ids: Vec::new(),
    });
    match doc.apply(
        &editor_core::DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        Tol::witness(),
    ) {
        Err(editor_core::EditError::ProfileProgramRefused { refusal, .. }) => match *refusal {
            editor_core::ProgramRefusal::Geometry {
                loop_: 0,
                step: 2,
                kind: profile::PathErrorKind::JunctionTangent,
                ..
            } => {}
            other => panic!("the junction refuses as tangent at step 2, got {other:?}"),
        },
        Ok(_) => panic!("the junction must refuse at the door"),
        Err(other) => panic!("the junction must refuse at the door, got {other:?}"),
    }
}

/// The names of `node`'s edges with both ends at height `z`.
fn rim_named(ev: &Evaluation<f64>, node: RecipeNodeId, z: f64) -> Vec<StableName> {
    let body = match &ev.value(node).expect("the node evaluated").payload {
        ValuePayload::Body(b) => crate::fixture::in_space(b),
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
/// polynomial, and so is each joint's form now that the disc reads no
/// frame (D10): nothing is reported.
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
        let kinds: Vec<EntityKind> = row
            .cells
            .iter()
            .map(|cell| match cell {
                NamedCell::Entity { input, name } if Some(*input) == doc.output(disc, 0) => {
                    name.kind
                }
                cell => panic!("a support is a face of the disc: {cell:?}"),
            })
            .collect();
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
        let names: Vec<&StableName> = row
            .cells
            .iter()
            .map(|cell| match cell {
                NamedCell::Entity { input, name } if Some(*input) == doc.output(disc, 0) => name,
                cell => panic!("a joint's cells are edges of the disc: {cell:?}"),
            })
            .collect();
        assert!(
            names[0] != names[1] && names.iter().all(|n| rim.contains(n)),
            "a joint names the disc's two rim edges: {row:?}"
        );
    }
    let proved = proofs(&doc, fillet, &got);
    assert_eq!(
        proved,
        [const { Proof::Structural(Rung::MarginIdentity) }; 4],
        "each support's departure from the shared axis, and each joint's, is the zero \
         polynomial"
    );
    let findings = unproven(&doc, &ev);
    assert!(
        findings.is_empty(),
        "the report carries the door's verdict, every row proven: {findings:?}"
    );
}

/// **A union records a vertex touch once, as a pair does.** Two blocks
/// kissing at a corner: their two-member union — the pair — holds one
/// `VertexFusion` row, cited by its one record. Folded into a union with
/// a third block apart, the pairwise judgement decides the same touch
/// and publishes its row, and the fold step's row, which the union's
/// record cites, states the same two member cells, so the record cites
/// the judged row: the union's rows and findings are the pair's, the
/// vertex touch once.
#[test]
fn a_union_records_a_vertex_touch_once() {
    let doc = ProfileDoc::empty_derived("coincide-union-kiss", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (1.0, 2.0), (1.0, 2.0), 1.0, 2.0);
    let (doc, far) = block_on(doc, a, (6.0, 7.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, pair) = crate::fixture::union_over(doc, &[a, b]);
    // The kiss is the last step's, so its record is the union's.
    let (doc, union) = crate::fixture::union_over(doc, &[far, a, b]);
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
        let ValuePayload::Boolean(editor_core::BooleanValue::Body { .. }) = &value.payload else {
            panic!("{what}: a boolean body")
        };
        let contacts = &value.carry.contacts;
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
/// in the door's own session (`coincide::with_replay_session`); a dial
/// opens another, at `TERMS` terms (the door's 256 unset) and without
/// the drive's retry ladder under `NORETRY`. Release builds;
/// `--nocapture`.
fn measure_rows(doc: &ProfileDoc, node: RecipeNodeId) {
    use geom_core::sym::deferred;
    use geom_core::{Sym, SymBudget, SymRules};
    let symbols = editor_core::EvalOptions {
        symbols: true,
        ..editor_core::EvalOptions::default()
    };
    let t = std::time::Instant::now();
    // The door's own session (`coincide::with_replay_session`), unless a
    // dial asks for another: `TERMS=<n>` a budget, `NORETRY` no ladder.
    let dialled = std::env::var_os("TERMS").is_some() || std::env::var_os("NORETRY").is_some();
    let body = |opts: &editor_core::EvalOptions| {
        let ev: Evaluation<Sym<f64>> = editor_core::evaluate(
            doc,
            None,
            &editor_core::CancelToken::new(),
            opts,
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
            if !d {
                for f in deferred::render(r.margin) {
                    eprintln!("  FORM {f}");
                }
            }
        }
    };
    let ((), counts) = if dialled {
        let opts = symbols.clone();
        deferred::with_deferred_session(
            SymBudget {
                max_terms: std::env::var("TERMS")
                    .ok()
                    .and_then(|t| t.parse().ok())
                    .unwrap_or(256),
                max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
            },
            SymRules::shipped(),
            if std::env::var_os("NORETRY").is_some() {
                geom_core::SymRetry::none()
            } else {
                editor_core::drive::DEFAULT_SYM_RETRY
            },
            || body(&opts),
        )
    } else {
        coincide::with_replay_session(symbols, |opts| body(opts))
    };
    eprintln!("total {:?} {counts:?}", t.elapsed());
}

#[test]
#[ignore = "a measurement: run in release with --nocapture"]
fn measure_the_box_mitre_at_sym() {
    let doc = ProfileDoc::empty_derived("coincide-mitre", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 2.0), (0.0, 1.5), 1.0);
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
/// plate's floor are two variables typed to one metre — the floor a
/// slide of the block's start cap along its outward normal, `−z`, by
/// `−1`: the lint reports the row (that test), and here the two
/// environments read their sum. In the analysis lane's environment
/// both are constants, so the sum decides Zero as a theorem; in the
/// door's they are two symbols, so it decides Zero only by value.
#[test]
fn two_variables_typed_equal_are_one_constant_to_the_analysis_lane_and_two_symbols_to_the_door() {
    use editor_core::analysis::{ParamBox, var_env_over, var_env_symbolic};
    use editor_core::{FreeVar, ParamValue};
    use geom_core::{Band, Decide, Sym, SymBudget};
    let doc = ProfileDoc::empty_derived("coincide-two-metres", Tol::witness());
    let (doc, base) = block(doc, (0.0, 2.0), (0.0, 2.0), 1.0);
    let (doc, _) = block_on(doc, base, (0.5, 1.5), (0.5, 1.5), 1.0, 0.5);
    let typed = |want: f64| -> Vec<editor_core::VarId> {
        doc.free_vars()
            .filter_map(|(id, free)| match *free {
                FreeVar::Continuous { value, .. } => (value == want).then_some(id),
                _ => None,
            })
            .collect()
    };
    let (heights, floors) = (typed(1.0), typed(-1.0));
    assert!(
        !heights.is_empty() && !floors.is_empty(),
        "the block's height and the plate's floor: {heights:?} {floors:?}"
    );
    let metres = [heights[0], floors[0]];
    let band = Band::linear(Tol::witness()).expect("a band");
    let difference = |env: &editor_core::VarEnv<Sym<f64>>| {
        let value = |id| match env.bindings.get(id) {
            Some(ParamValue::Continuous { value, .. }) => *value,
            other => panic!("{id:?} binds {other:?}"),
        };
        let decided = (value(&metres[0]) + value(&metres[1]))
            .sign_within(band)
            .expect("a difference of nothing decides");
        (decided.sign, decided.margin.proven())
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
        (geom_core::Sign::Zero, true),
        "the analysis lane reads two constants and their sum is the zero polynomial"
    );
    assert_eq!(
        door,
        (geom_core::Sign::Zero, false),
        "the door reads two symbols: Zero at the values, no proof"
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
            back.sign_within(band).expect("decides").margin.proven()
        })
    });
    assert_eq!(theorems, [true; 3]);
}

/// **The door's proof of each row of the corpus**, one line a row
/// (`ROW <document> <node> <index> <relation> <site> <proof>`), the
/// `⊔`s' at-rest rows among them, over the registry and INTENT stage
/// 6's scenes beside it, whose refused nodes are a line each (`REFUSED
/// <document> <node> <kind>`): the instrument a change to a rung is
/// diffed by, base against head. A census, not a gate.
#[test]
#[ignore = "a census: cargo nextest run -p editor-core --run-ignored only --no-capture -E 'test(=coincidence_door::census_the_door_per_row_over_the_corpus)'"]
fn census_the_door_per_row_over_the_corpus() {
    for cd in crate::corpus::documents()
        .into_iter()
        .chain(crate::corpus::stage6_beside_the_registry())
    {
        let ev = run(&cd.doc);
        for (node, result) in &ev.nodes {
            if let editor_core::NodeResult::Failed(e) = result {
                let kind = match &e.kind {
                    editor_core::NodeErrorKind::Boolean(b) => {
                        format!(
                            "Boolean/{}",
                            crate::fixture::variant_name(&format!("{b:?}"))
                        )
                    }
                    kind => crate::fixture::variant_name(&format!("{kind:?}")),
                };
                println!("REFUSED {} {} {kind}", cd.name, node.0.ordinal());
            }
        }
        for (node, rows, proofs) in proved_rows(&cd.doc, &ev) {
            for (i, (row, proof)) in rows.iter().zip(&proofs).enumerate() {
                println!(
                    "ROW {} {} {i} {:?} {:?} {}",
                    cd.name,
                    node.0.ordinal(),
                    row.relation,
                    row.site,
                    proof_words(proof)
                );
            }
        }
        // The `⊔`s' at-rest rows, each under its first copy's placement,
        // as the `unproven-coincidence` check asks them.
        for (i, (node, row)) in at_rest_rows(&cd.doc, &ev).iter().enumerate() {
            println!(
                "ROW {} {} at-rest/{i} {:?} {:?} {}",
                cd.name,
                node.0.ordinal(),
                row.relation,
                row.site,
                proof_words(&proof(&cd.doc, *node, row))
            );
        }
    }
}

/// **A proof in the census's words**: the rung that proved it, or
/// `unproven/<replay>/<recourse>`.
fn proof_words(proof: &Proof) -> String {
    match proof {
        Proof::Structural(rung) => format!("{rung:?}"),
        Proof::Unproven { residual, recourse } => {
            format!("unproven/{:?}/{recourse:?}", residual.replay)
        }
    }
}

/// **Every evaluated node's rows, each with the door's proof**, in
/// evaluation then decision order, asked of the door at once.
fn proved_rows(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
) -> Vec<(RecipeNodeId, Vec<NamedCoincidence>, Vec<Proof>)> {
    let nodes: Vec<(RecipeNodeId, &[NamedCoincidence])> = ev
        .order
        .iter()
        .filter_map(|&node| ev.value(node).map(|v| (node, &*v.coincidences)))
        .filter(|(_, rows)| !rows.is_empty())
        .collect();
    let proofs = coincide::prove(
        doc,
        &nodes,
        None,
        Tol::witness(),
        &editor_core::CancelToken::new(),
    )
    .expect("an uncanceled door");
    nodes
        .into_iter()
        .zip(proofs)
        .map(|((node, rows), proofs)| (node, rows.to_vec(), proofs))
        .collect()
}

/// **The at-rest rows of `doc`'s `⊔`s**: the world product's, then each
/// other `⊔`'s that gathered, each under the placement of the first of
/// its two copies.
fn at_rest_rows(doc: &ProfileDoc, ev: &Evaluation<f64>) -> Vec<(RecipeNodeId, NamedCoincidence)> {
    let Ok(product) = editor_core::product_recorded(doc, ev, Tol::witness()) else {
        return Vec::new();
    };
    product
        .coincidences
        .iter()
        .chain(
            product
                .assembles
                .iter()
                .filter_map(|assemble| assemble.gather.as_ref().ok())
                .flat_map(|gathered| gathered.coincidences.iter()),
        )
        .map(|r| {
            let node = doc
                .operation_of(r.copies.0.var)
                .expect("a gathered copy is a placement's output");
            (node, r.row.clone())
        })
        .collect()
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
        // The replay alone, in the door's own session.
        let replay = (!nodes.is_empty()).then(|| {
            coincide::with_replay_session(
                editor_core::EvalOptions {
                    symbols: true,
                    ..editor_core::EvalOptions::default()
                },
                |opts| {
                    let _: Evaluation<geom_core::Sym<f64>> = editor_core::evaluate(
                        &cd.doc,
                        None,
                        &editor_core::CancelToken::new(),
                        opts,
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
            None,
            Tol::witness(),
            &editor_core::CancelToken::new(),
        )
        .expect("an uncanceled door");
        let door = t.elapsed();
        let mut tally: std::collections::BTreeMap<String, usize> = Default::default();
        for ((_, rows), proofs) in nodes.iter().zip(&proofs) {
            for (row, proof) in rows.iter().zip(proofs) {
                *tally
                    .entry(format!("{:?} {}", row.site, proof_words(proof)))
                    .or_default() += 1;
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
/// `die_composed`'s two and 14 of `die_composed_tour`'s 42 prove as
/// theorems, and the rest stay unproven. The tour's pips are copies
/// placed against the die's faces; a copy placed by a frame turned off
/// the die's own axes reaches its rim through that turn's sine and
/// cosine, and its joints stay unproven
/// (`work/intent/a-turn-by-the-nearest-quarter-turn-rounds.md`). The
/// dies' booleans decide vertex identities too, each a point minted on
/// an edge, published as a `Meet` of the cells it lies in and left
/// unproven, a coincidence of values as far as the door can tell.
#[test]
fn the_door_proves_half_the_composed_dies_joints() {
    // `met`: the vertex identities the dies' booleans decide at a point
    // minted on an edge, each a `Meet` of the cells it lies in (measured:
    // a pip's meridian seam edge against a face of the die).
    for (name, proved, rows, met) in [("die_composed", 1, 4, 2), ("die_composed_tour", 14, 84, 42)]
    {
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
            None,
            Tol::witness(),
            &editor_core::CancelToken::new(),
        )
        .expect("an uncanceled door")
        .into_iter()
        .flatten()
        .collect();
        let theorems = all
            .iter()
            .filter(|p| matches!(p, Proof::Structural(Rung::MarginIdentity)))
            .count();
        let replayed = replays(&all);
        let meetings = nodes
            .iter()
            .flat_map(|(_, rows)| rows.iter())
            .filter(|r| r.relation == Relation::Meet)
            .count();
        assert_eq!(meetings, met, "{name}: the booleans' meetings");
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

/// **A symbolic run takes one environment**: asked for symbols and a
/// parameter box too, it refuses every node, typed.
#[test]
fn a_symbolic_run_over_a_box_refuses_every_node() {
    use editor_core::{NodeErrorKind, NodeResult, ParamBox, ParamBoxError};
    let doc = ProfileDoc::empty_derived("coincide-symbols-box", Tol::witness());
    let (doc, _) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
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

/// **A part's variables are symbols of its own document version.** Two
/// documents built by the same edits mint the same variable ids, and two
/// versions of one document share them; the door keys each symbol by the
/// document, the version its reference pins and the id, so two
/// documents' widths, or two versions', are two symbols and their
/// difference is no identity, while one version's width read twice (a
/// part instanced twice) is one symbol.
///
/// Red if a symbol is keyed by its id alone, or by its document without
/// the pin: the widths then cancel as a theorem.
#[test]
fn two_documents_variables_are_two_symbols() {
    use editor_core::ContentPin;
    use editor_core::ParamValue;
    use editor_core::analysis::{var_env_symbolic, var_env_symbolic_pinned};
    use geom_core::{Band, Decide, Sym, SymBudget};
    let built = |seed: &str| {
        let doc = ProfileDoc::empty_derived(seed, Tol::witness());
        block(doc, (0.0, 1.0), (0.0, 1.0), 1.0).0
    };
    let (a, b) = (built("coincide-seam-a"), built("coincide-seam-b"));
    let ids: Vec<VarId> = a.free_vars().map(|(id, _)| id).collect();
    assert_eq!(
        ids,
        b.free_vars().map(|(id, _)| id).collect::<Vec<_>>(),
        "the same edits mint the same ids"
    );
    let band = Band::linear(Tol::witness()).expect("a band");
    let budget = SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    let (decided, _) = geom_core::sym::with_session(budget, || {
        let (ea, eb) = (
            var_env_symbolic::<Sym<f64>, _>(&a).expect("every variable binds"),
            var_env_symbolic::<Sym<f64>, _>(&b).expect("every variable binds"),
        );
        let value = |env: &editor_core::VarEnv<Sym<f64>>, id| match env.bindings.get(id) {
            Some(ParamValue::Continuous { value, .. }) => *value,
            other => panic!("{id:?} binds {other:?}"),
        };
        let proven = |x: Sym<f64>| {
            let d = x.sign_within(band).expect("a difference decides");
            (d.sign, d.margin.proven())
        };
        let pinned = |pin: u8| {
            var_env_symbolic_pinned::<Sym<f64>, _>(&a, Some(&ContentPin([pin; 32])))
                .expect("every variable binds")
        };
        let (v1, v1_again, v2) = (pinned(1), pinned(1), pinned(2));
        (
            proven(value(&ea, &ids[0]) - value(&eb, &ids[0])),
            proven(value(&ea, &ids[0]) - value(&ea, &ids[0])),
            proven(value(&v1, &ids[0]) - value(&v2, &ids[0])),
            proven(value(&v1, &ids[0]) - value(&v1_again, &ids[0])),
            proven(value(&ea, &ids[0]) - value(&v1, &ids[0])),
        )
    });
    assert_eq!(
        decided.0,
        (geom_core::Sign::Zero, false),
        "two documents' variable of one id: Zero at the values, two symbols"
    );
    assert_eq!(
        decided.1,
        (geom_core::Sign::Zero, true),
        "one document's variable read twice is one symbol"
    );
    assert_eq!(
        decided.2,
        (geom_core::Sign::Zero, false),
        "two pinned versions of one document: two symbols"
    );
    assert_eq!(
        decided.3,
        (geom_core::Sign::Zero, true),
        "one pinned version bound twice (a part instanced twice): one symbol"
    );
    assert_eq!(
        decided.4,
        (geom_core::Sign::Zero, false),
        "the document being evaluated and a pinned version of it: two symbols"
    );
}

/// **Two copies of one body are two constructions placed apart, never
/// one** (D10, `Placed::World`). A unit block placed at the identity
/// and again turned half a turn about z and moved 2 along x and 1
/// along y: the copies meet at x = 1, each by the image of the block's
/// own +x wall, and lie flush on four sides, so every row between them
/// names the same face, edge or vertex of the one construction in each
/// cell. The door walks each cell through its copy's placement, so
/// none of those rows is proven `SameConstruction`.
///
/// The seat and the two side walls are reported. Their coincidence
/// holds only because the turned copy's move equals the block's width.
/// The top and bottom walls lie on the planes both copies hold for every
/// value (the turn is about z), which the replay's carrier verdict
/// proves.
///
/// Red if the walk drops the placement: same-cell rows between copies
/// then read as one construction and go silent.
#[test]
fn two_copies_meeting_by_the_same_face_are_not_one_construction() {
    use editor_core::coincide::{Proof, Rung};
    let doc = ProfileDoc::empty_derived("coincide-copies-same-face", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _) = crate::fixture::place(doc, the_box);
    let turned = editor_core::Frame::rotate_then_translate(
        [0.0, 0.0, 1.0],
        std::f64::consts::PI,
        [2.0, 1.0, 0.0],
        crate::fixture::band(),
    )
    .expect("a half-turn about z");
    let (doc, q) = crate::fixture::insert(
        doc,
        crate::fixture::stood(editor_core::BodyRead::plain(the_box)),
    );
    let doc = crate::fixture::pin(doc, q, &turned, &EvalOptions::default());
    let ev = run(&doc);
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    let proofs: Vec<(topo::Relation, Option<Rung>)> = product
        .coincidences
        .iter()
        .map(|r| {
            let (a, _) = r.copies;
            let node = doc.operation_of(a.var).expect("a copy's placement");
            let rows = [r.row.clone()];
            let proof = editor_core::coincide::prove(
                &doc,
                &[(node, &rows[..])],
                None,
                Tol::witness(),
                &editor_core::CancelToken::new(),
            )
            .expect("the door runs");
            let rung = match proof[0][0] {
                Proof::Structural(rung) => Some(rung),
                Proof::Unproven { .. } => None,
            };
            (r.row.relation, rung)
        })
        .collect();
    let mut kinds = proofs.clone();
    kinds.sort_by_key(|(relation, rung)| (format!("{relation:?}"), format!("{rung:?}")));
    assert_eq!(
        kinds,
        vec![
            (topo::Relation::SameOpposite, None),
            (topo::Relation::SameOriented, None),
            (topo::Relation::SameOriented, None),
            (topo::Relation::SameOriented, Some(Rung::CarrierVerdict)),
            (topo::Relation::SameOriented, Some(Rung::CarrierVerdict)),
        ],
        "the seat and the two side walls unproven, the top and bottom proven by their \
         carriers, none one construction"
    );
    let reported = unproven(&doc, &ev)
        .into_iter()
        .filter(|f| matches!(f.subject, FindingSubject::Copies { .. }))
        .count();
    assert_eq!(reported, 3, "and the check reports the three unproven");
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
        .find(|cd| cd.name == "die_composed")
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
        "the stale expectation, then the composed die's unproven row: {sync}"
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

/// The pinch of `union_pinch_member_order`: a plate and two blocks
/// whose edges touch along the line x = 1.5, y = 1, the second block
/// moved `dx` along x; the plate, the blocks and their union.
fn ring_pinch(dx: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("coincide-ring", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 1.0);
    let (doc, p1) = block_on(doc, plate, (1.0, 1.5), (-1.0, 1.0), 0.5, 1.5);
    let (doc, p2) = block_on(doc, plate, (1.5 + dx, 2.0), (1.0, 3.0), 0.47, 1.23);
    let (doc, blocks) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![p1.into(), p2.into()]),
        },
    );
    (doc, plate, blocks)
}

/// The x offset of `ring_pinch`'s second block for a fusion: 1e-13,
/// or ε/100 where that is smaller, so the blocks' union reads the gap
/// inside its zero band at every ε. The union's sector test reads it
/// about 17 times over (1.7e-12 for 1e-13), which at ε 1e-12 is in the
/// escalate band, and the union refuses.
fn pierce_gap() -> f64 {
    (Tol::witness().eps() / 100.0).min(1e-13)
}

/// The `VertexFusion` rows among `node`'s.
fn fusions(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<NamedCoincidence> {
    rows(ev, node)
        .into_iter()
        .filter(|r| r.site == DecisionSite::VertexFusion)
        .collect()
}

/// **Two pierces one ring serves are an unproven vertex fusion.** The
/// blocks' touching edges pierce the plate's top at (1.5, 1, 1), the
/// second block's [`pierce_gap`] off the first's along x: two vertices of the
/// blocks' union on two point keys, one point by `bool_contact_vertex`
/// and not bit for bit. The plate's top mints one ring for both
/// (`topo` `vtxfac::mint_ring`), and the intersection keeps both pierces
/// as two vertices. It records the decision: one `VertexFusion`
/// `Meet` row, its cells the two blocks' edges as the union names them
/// and the plate's top, whose carriers share the point each pierce was
/// minted at (the two edges alone share a line), its margin the gap. The door leaves it unproven, its form outgrowing the door's
/// budget, and the check reports it: the blocks are copies placed
/// against the plate, and no mate states their edges one. Red if
/// the ring glues the two by a shared point key alone, which records
/// nothing.
///
/// **Two pierces already on one point key are decided onto a face that
/// runs through them.** That intersection keeps the pierces on one key:
/// the ring's copies, the plate's side. A slab whose face runs through
/// (1.5, 1, 1) obliquely, intersected with it, decides the vertices
/// there onto that face, and records each decision, though the copies
/// were already one key's.
#[test]
fn a_ring_serving_two_pierces_records_their_fusion_unproven() {
    let (doc, plate, blocks) = ring_pinch(pierce_gap());
    let (doc, common) = insert(
        doc,
        Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![plate.into(), blocks.into()]),
        },
    );
    // The slab: the quadrilateral (0.5, 0), (2.5, 2), (4, 2), (4, 0) in
    // x, z, whose edge from (0.5, 0) to (2.5, 2) runs through (1.5, 1),
    // extruded across the blocks' y range.
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.5, 0.0), (4.0, 0.0), (4.0, 2.0), (2.5, 2.0)]],
    );
    let (doc, slab) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(6.0),
        },
    );
    let (doc, slab) = crate::fixture::against(
        doc,
        plate,
        slab,
        &crate::fixture::axes_at([0.0, 4.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
    );
    let (doc, cut) = insert(
        doc,
        Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![common.into(), slab.into()]),
        },
    );
    let ev = run(&doc);
    let fused = fusions(&ev, common);
    assert_eq!(fused.len(), 1, "one recorded fusion: {fused:?}");
    let row = &fused[0];
    assert_eq!(
        row.relation,
        Relation::Meet,
        "two points minted on edges, decided one, are the edges' meeting"
    );
    let gap = format!("{:?}", row.margin);
    assert!(
        row.margin.kind() == geom_core::MarginKind::Value
            && !gap.starts_with("MarginDiag(Value(0.0"),
        "the margin is the pierces' gap, not bit for bit zero: {gap}"
    );
    let of = |node: RecipeNodeId, kind: EntityKind| {
        row.cells
            .iter()
            .filter(|c| {
                matches!(c, NamedCell::Entity { input, name }
                if *input == out(&doc, node) && name.kind == kind)
            })
            .count()
    };
    assert_eq!(
        (
            of(blocks, EntityKind::Edge),
            of(plate, EntityKind::Face),
            row.cells.len()
        ),
        (2, 1, 3),
        "the blocks' union's two edges and the plate's top: {:?}",
        row.cells
    );
    assert_eq!(
        recourses(&proofs(&doc, common, &fused)),
        vec![Some(coincide::Recourse::Budget)],
        "unproven, its form outgrowing the door's budget"
    );
    let reported: Vec<_> = unproven(&doc, &ev)
        .into_iter()
        .filter(|f| {
            matches!(&f.evidence, CheckEvidence::UnprovenCoincidence { row: r, .. } if **r == *row)
        })
        .collect();
    assert_eq!(reported.len(), 1, "the check reports the fusion");
    assert_eq!(reported[0].subject, FindingSubject::Node(common));
    assert!(
        failure(&ev, cut).is_none(),
        "the slab's intersection builds: {:?}",
        failure(&ev, cut)
    );
    let onto = fusions(&ev, cut);
    let slab_face = |c: &NamedCell| {
        matches!(c, NamedCell::Entity { input, name }
        if *input == out(&doc, slab) && name.kind == EntityKind::Face)
    };
    assert!(
        onto.len() >= 2
            && onto.iter().all(|r| slab_face(&r.cells[1])
                && r.cells[1] == onto[0].cells[1]
                && matches!(&r.cells[0], NamedCell::Entity { input, name }
                    if *input == out(&doc, common) && name.kind == EntityKind::Vertex)),
        "the pierces and the vertices beside them, each on the slab's one face: {onto:?}"
    );
}

/// **An unproven fold row is reported under the judged row decided at
/// its point** (DM4: the judged rows are the union's order-free rows):
/// over orders of `union_pinch_member_order`'s prism crossing one wedge
/// with the plate, each unproven row a fold step logged that names a
/// member vertex an unproven judged row names goes under the least such
/// judged row by its sorted cells, relation and site, and every other
/// one is its own finding. The parent is computed here from the rows and
/// held against the report, and one fold statement must have one parent
/// statement in every order that logs it. Red if a fold row is lost,
/// reported twice, put under a row about another point, or under a
/// parent that follows the order; and the fixture must report some
/// under a judged row, so the rule is not passed on nothing.
#[test]
fn an_unproven_fold_row_is_reported_under_the_judged_row_decided_at_its_point() {
    use editor_core::RowOrigin;
    use std::collections::BTreeMap;
    let mut under = 0;
    let mut parents: BTreeMap<String, String> = BTreeMap::new();
    for order in [[0, 1, 2, 3], [1, 3, 0, 2], [3, 1, 0, 2], [2, 0, 3, 1]] {
        let doc = ProfileDoc::empty_derived("door-fold-rows", Tol::witness());
        let (doc, m) = crate::union_pinch_member_order::crossing_one_wedge(doc);
        let members: Vec<RecipeNodeId> = order.iter().map(|&i| m[i]).collect();
        let (doc, u) = crate::fixture::union_over(doc, &members);
        let ev = run(&doc);
        let logged = rows(&ev, u);
        let unproven: Vec<&NamedCoincidence> = logged
            .iter()
            .zip(proofs(&doc, u, &logged))
            .filter(|(_, p)| matches!(p, Proof::Unproven { .. }))
            .map(|(r, _)| r)
            .collect();
        let key = |r: &NamedCoincidence| {
            let mut cells = r.cells.clone();
            cells.sort();
            (cells, r.relation, r.site)
        };
        // Each cell by its member's fixture index and its name, so a
        // statement reads alike in every order.
        let said = |r: &NamedCoincidence| {
            let mut cells: Vec<String> = r
                .cells
                .iter()
                .map(|c| match c {
                    NamedCell::Entity { input, name } => {
                        let at = (0..m.len()).find(|&i| out(&doc, m[i]) == *input);
                        format!("{at:?} {name:?}")
                    }
                    other => format!("{other:?}"),
                })
                .collect();
            cells.sort();
            format!("{:?} {:?} {cells:?}", r.relation, r.site)
        };
        let vertex_shared = |a: &NamedCoincidence, b: &NamedCoincidence| {
            a.cells.iter().any(|c| {
                matches!(c, NamedCell::Entity { name, .. } if name.kind == EntityKind::Vertex)
                    && b.cells.contains(c)
            })
        };
        let mut want: Vec<(String, Vec<String>)> = Vec::new();
        let mut own: Vec<&NamedCoincidence> = Vec::new();
        let mut placed: Vec<(&NamedCoincidence, &NamedCoincidence)> = Vec::new();
        for f in &unproven {
            let parent = matches!(f.origin, RowOrigin::Fold { .. })
                .then(|| {
                    unproven
                        .iter()
                        .filter(|j| {
                            matches!(j.origin, RowOrigin::Judged { .. }) && vertex_shared(j, f)
                        })
                        .min_by_key(|j| key(j))
                })
                .flatten();
            match parent {
                Some(j) => placed.push((j, f)),
                None => own.push(f),
            }
        }
        for o in &own {
            let mut under: Vec<String> = placed
                .iter()
                .filter(|(j, _)| std::ptr::eq(*j, *o))
                .map(|(_, f)| said(f))
                .collect();
            under.sort();
            want.push((said(o), under));
        }
        let mut got: Vec<(String, Vec<String>)> = unproven_findings(&doc, &ev, u)
            .into_iter()
            .map(|(row, restated)| {
                let mut under: Vec<String> = restated.iter().map(&said).collect();
                under.sort();
                (said(&row), under)
            })
            .collect();
        got.sort();
        want.sort();
        assert_eq!(
            got, want,
            "{order:?}: each fold row under its point's judged row"
        );
        for (j, f) in &placed {
            under += 1;
            let (fold, judged) = (said(f), said(j));
            if let Some(before) = parents.insert(fold.clone(), judged.clone()) {
                assert_eq!(
                    before, judged,
                    "{order:?}: the fold row {fold} has one parent in every order"
                );
            }
        }
    }
    assert!(
        under > 0,
        "the fixture reports a fold row under a judged finding"
    );
}

/// The union `u`'s unproven-coincidence findings, each its row and the
/// fold rows it reports with it.
fn unproven_findings(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    u: RecipeNodeId,
) -> Vec<(NamedCoincidence, Vec<NamedCoincidence>)> {
    unproven(doc, ev)
        .into_iter()
        .filter(|f| f.subject == editor_core::FindingSubject::Node(u))
        .filter_map(|f| match f.evidence {
            CheckEvidence::UnprovenCoincidence { row, restated, .. } => {
                Some((*row, restated.into_iter().map(|(r, _, _)| r).collect()))
            }
            _ => None,
        })
        .collect()
}

// ---- INTENT stage 6's scenes, as the door proves them today ----

/// **Each row the door proves over `cd`**, one line a row in evaluation
/// then decision order (`<relation> <site> <proof>`, the census's
/// words), beside the rows themselves.
fn door_lines(cd: &crate::corpus::CorpusDoc) -> (Vec<NamedCoincidence>, Vec<String>) {
    let ev = run(&cd.doc);
    assert_eq!(
        crate::corpus::failures(&ev),
        Vec::<String>::new(),
        "{} evaluates",
        cd.name
    );
    let mut all = Vec::new();
    let mut lines = Vec::new();
    for (_, rows, proofs) in proved_rows(&cd.doc, &ev) {
        for (row, proof) in rows.iter().zip(&proofs) {
            lines.push(format!(
                "{:?} {:?} {}",
                row.relation,
                row.site,
                proof_words(proof)
            ));
        }
        all.extend(rows);
    }
    (all, lines)
}

/// **`cd`'s union escalates at `Interval`**, at the default ε on the
/// ruled region's span (`bool_ruled_region_span`; at `1e-12` a carrier
/// gap reads first), so the scene stands beside the registry
/// (`work/intent/the-ruled-region-span-holds-two-stage6-scenes-off-the-registry.md`).
fn escalates_at_interval(cd: &crate::corpus::CorpusDoc) {
    let ev = crate::corpus::eval::<geom_core::Interval>(&cd.doc);
    let union = cd.result.expect("the scene's head is its union");
    match ev.nodes.get(&union) {
        Some(editor_core::NodeResult::Failed(e)) => assert!(
            matches!(
                &e.kind,
                editor_core::NodeErrorKind::Boolean(topo::BooleanError::Escalated { diag, .. })
                    if Tol::witness().eps() != geom_core::tolerance::DEFAULT_EPS
                        || diag.predicate == Some("bool_ruled_region_span")
            ),
            "{}'s union escalates at Interval, on the ruled region's span at the default ε: \
             {:?}",
            cd.name,
            e.kind
        ),
        other => panic!("{}'s union escalates at Interval: {other:?}", cd.name),
    }
}

/// A row's two entity cells, as (input, name).
fn two_cells(row: &NamedCoincidence) -> [(VarId, &StableName); 2] {
    match row.cells.as_slice() {
        [
            NamedCell::Entity { input: a, name: na },
            NamedCell::Entity { input: b, name: nb },
        ] => [(*a, na), (*b, nb)],
        cells => panic!("a row over two entities: {cells:?}"),
    }
}

/// Whether `name` is a face a revolve sweeps (either half of a curved
/// wall).
fn a_band(name: &StableName) -> bool {
    name.kind == EntityKind::Face
        && matches!(
            name.path.as_slice(),
            [RoleSeg::Band(_) | RoleSeg::BandPi(_)]
        )
}

/// **A pin seated in a bore records no row, and the census says it
/// cannot look** (INTENT stage 6, `pin_in_bore`). Each pin stands on
/// its housing, its shoulder on the top and its shank in the bore. No
/// node decides a coincidence, and the world's `⊔` records no at-rest
/// row at all, neither the press fit's walls (one cylinder by value)
/// nor either shoulder's rest on its housing's top. The census names
/// the press fit's bore wall and shank wall as a pair it cannot clear
/// (`CensusUndecidable`: it cannot yet tell whether curved faces of two
/// parts touch), and does not pair the clearance fit's, which stand
/// apart; the rest of what it reports is tallied. Alike where the press
/// fit's shank reads a second variable typed equal to the bore
/// (`pin_in_bore_two_variables`).
///
/// Red if a pin stops seating, or once the census decides a curved
/// contact (then the press fit's walls are a row to pin here).
#[test]
fn a_pin_in_a_bore_is_a_contact_the_census_cannot_look_at() {
    for cd in [
        crate::corpus::pin_in_bore::document(),
        crate::corpus::pin_in_bore::two_variables(),
    ] {
        pin_in_bore_cannot_be_looked_at(&cd);
    }
}

/// [`a_pin_in_a_bore_is_a_contact_the_census_cannot_look_at`] over one
/// of its two scenes.
fn pin_in_bore_cannot_be_looked_at(cd: &crate::corpus::CorpusDoc) {
    let (rows, _) = door_lines(cd);
    assert_eq!(
        rows,
        Vec::new(),
        "{}: no node decides a coincidence",
        cd.name
    );
    let ev = run(&cd.doc);
    let product = editor_core::product_recorded(&cd.doc, &ev, Tol::witness()).expect("gathers");
    let feet: Vec<f64> = product
        .copies
        .iter()
        .map(|copy| {
            let body = &*copy.body;
            body.vertices()
                .map(|(_, v)| body.get_point(v.point).unwrap().y)
                .fold(f64::INFINITY, f64::min)
        })
        .collect();
    assert_eq!(
        feet,
        [0.0, 0.25, 0.0, 0.25],
        "each housing stands on its base and each shank's foot is 3/4 under its top"
    );
    assert_eq!(
        product.coincidences.len(),
        0,
        "the census records no at-rest row"
    );
    let mut tally: std::collections::BTreeMap<String, usize> = Default::default();
    for finding in &product.census {
        *tally
            .entry(crate::fixture::variant_name(&format!("{finding:?}")))
            .or_default() += 1;
    }
    // The pairs the census cannot clear, each face as its revolve and
    // the step its wall sweeps.
    let wall = |f| {
        let mut name = product.names.name_of(&editor_core::EntityRef {
            body: 0,
            key: EntityKey::Face(f),
        })?;
        // A copy's face is said by the read it came in through.
        while let [RoleSeg::From { of, .. }] = name.path.as_slice() {
            name = of;
        }
        match name.path.as_slice() {
            [RoleSeg::Band(run) | RoleSeg::BandPi(run)] => run
                .pieces()
                .first()
                .and_then(|p| p.step())
                .map(|step| (name.node, step)),
            _ => None,
        }
    };
    let undecided: Vec<[(RecipeNodeId, editor_core::StepId); 2]> = product
        .census
        .iter()
        .filter_map(|finding| match finding {
            topo::ValidationError::CensusUndecidable {
                a: topo::EntityId::Face(a),
                b: topo::EntityId::Face(b),
                ..
            } => Some([wall(*a)?, wall(*b)?]),
            _ => None,
        })
        .collect();
    let [press, loose] = crate::corpus::pin_in_bore::walls(&cd.doc);
    let names = |[x, y]: [(RecipeNodeId, editor_core::StepId); 2]| {
        undecided.contains(&[x, y]) || undecided.contains(&[y, x])
    };
    assert!(
        names(press),
        "{}: the census names the press fit's bore and shank walls as a pair it cannot \
         clear: {undecided:?}",
        cd.name
    );
    assert!(
        !names(loose),
        "{}: the clearance fit's walls are apart: {undecided:?}",
        cd.name
    );
    assert_eq!(
        tally,
        [
            ("CensusUndecidable".to_owned(), 15),
            ("UnrecordedContact".to_owned(), 11)
        ]
        .into_iter()
        .collect(),
        "{}: the census could not look: {:?}",
        cd.name,
        product.census
    );
}

/// **Two copies on one housing's axis glue their walls on one axis,
/// unproven while their radii are two variables** (INTENT stage 6,
/// `shaft_and_bearing`). The shaft and the collar are each mated to the
/// housing revolve's `axis` output; the union glues the shaft's wall to
/// the bore's and to the collar's, two `SameOpposite` rows at
/// `CarrierLadder` each (one per half of the shaft's wall), and decides
/// the walls' rims and seams by vertex fusion. Every wall is a band of
/// its revolve, so about that revolve's `axis` output, and the mates
/// make the three outputs one: the door reads each wall pair's axes as
/// one, `housing.axis` read twice. Each wall's radius is its own literal
/// coordinate, so its own symbol, and the row is unproven at its radius,
/// which the finding names. [`shaft_and_bearing::one_radius`] reads one
/// `bore` for the three radii, and its four wall rows are proven by the
/// carrier verdict, the axes as the mates state them; the seams on the
/// walls then prove too.
///
/// At `Interval` the union escalates, so both scenes stand beside the
/// registry.
///
/// Red if a scene's rows change in kind, site or proof, if the walls'
/// axes stop reading as one, or once its union passes at `Interval`
/// (then it joins the registry).
#[test]
fn two_copies_on_one_axis_glue_their_walls_on_one_axis() {
    use coincide::{Axes, Component};
    let cd = crate::corpus::shaft_and_bearing::document();
    let (rows, lines) = door_lines(&cd);
    let walls = "SameOpposite CarrierLadder unproven/Numeric/OneConstruction";
    let on = "OnCarrier VertexFusion unproven/Numeric/OneConstruction";
    let meet = "Meet VertexFusion unproven/Numeric/Budget";
    assert_eq!(
        lines,
        [
            walls, walls, on, on, meet, meet, meet, meet, walls, walls, meet, meet, meet, meet, on,
            on, meet, meet, meet, meet, meet, meet, meet, meet,
        ]
    );
    let wall_rows: Vec<&NamedCoincidence> = rows
        .iter()
        .filter(|row| row.site == DecisionSite::CarrierLadder)
        .collect();
    let mut inputs = std::collections::BTreeSet::new();
    for row in &wall_rows {
        let [(a, na), (b, nb)] = two_cells(row);
        assert!(
            a != b && a_band(na) && a_band(nb),
            "a wall pair of two parts: {row:?}"
        );
        inputs.extend([a, b]);
    }
    assert_eq!(inputs.len(), 3, "the housing, the shaft and the collar");
    // The housing is the meridian revolved first.
    let housing_axis = cd
        .doc
        .ids()
        .into_iter()
        .find(|&id| matches!(cd.doc.node(id), Some(Node::Revolve { .. })))
        .map(|revolve| axis_port(&cd.doc, revolve))
        .expect("the housing is revolved");
    let ev = run(&cd.doc);
    for (_, rows, proofs) in proved_rows(&cd.doc, &ev) {
        for (row, proof) in rows.iter().zip(&proofs) {
            let axes = match proof {
                Proof::Unproven { residual, .. } => residual.axes,
                Proof::Structural(_) => None,
            };
            assert_eq!(
                axes.map(|a: Axes| (a.reached, a.left)),
                (row.site == DecisionSite::CarrierLadder)
                    .then_some(([Some(housing_axis); 2], Component::Radius)),
                "a wall pair is one axis, housing.axis read twice, and two radii; no other row \
                 is read for its axes: {row:?}"
            );
        }
    }
    let words: Vec<String> = unproven(&cd.doc, &ev)
        .iter()
        .map(ToString::to_string)
        .filter(|f| f.contains("rests on one carrier against"))
        .collect();
    assert!(
        !words.is_empty()
            && words.iter().all(|f| f.contains(&format!(
                "the axes are one, both reading {}; the radii are equal only at the current values",
                tag(housing_axis)
            ))),
        "each wall finding names its one axis and its two radii: {words:?}"
    );
    escalates_at_interval(&cd);

    let cd = crate::corpus::shaft_and_bearing::one_radius();
    let (_, lines) = door_lines(&cd);
    let walls = "SameOpposite CarrierLadder CarrierVerdict";
    let on = "OnCarrier VertexFusion MarginIdentity";
    let meet = "Meet VertexFusion unproven/Numeric/OneConstruction";
    let budget = "Meet VertexFusion unproven/Numeric/Budget";
    assert_eq!(
        lines,
        [
            walls, walls, on, on, meet, meet, meet, meet, walls, walls, budget, budget, budget,
            budget, on, on, meet, meet, meet, meet, budget, budget, budget, budget,
        ],
        "with one radius the walls are one axis and one radius"
    );
    escalates_at_interval(&cd);
}

/// **Two copies of one bushing, stacked, glue their walls** (INTENT
/// stage 6, `axis_projection`). The revolve and its copy along its own
/// axis meet face to face: the two through bores, one piece of the one
/// revolve, are proven by the carrier verdict; the socket's wall and the
/// copy's spigot wall, two pieces of it, and the spigot's foot and the
/// shoulder on the socket's floor and the body's top, are unproven, as
/// is every rim the fusion decides. The socket and the spigot are one
/// axis (two bands of the revolve, about its `axis` output, which the
/// copy's mate reads twice); their radii are two literal coordinates,
/// so the wall row is unproven at its radius.
///
/// Red if the scene's rows change in kind, site or proof, or the wall
/// row's axes stop reading as one.
#[test]
fn a_bushing_stacked_on_its_own_axis_glues_its_walls() {
    let cd = crate::corpus::axis_projection::document();
    let (rows, lines) = door_lines(&cd);
    let bores = "SameOriented CarrierLadder CarrierVerdict";
    let rest = "SameOpposite PlaneLadder unproven/Numeric/OneConstruction";
    let walls = "SameOpposite CarrierLadder unproven/Numeric/OneConstruction";
    let on = "OnCarrier VertexFusion unproven/Numeric/OneConstruction";
    let mut want = vec![bores, rest, walls, rest, bores];
    want.extend([on; 8]);
    assert_eq!(lines, want);
    let [(a, na), (b, nb)] = two_cells(&rows[2]);
    assert!(
        a != b && a_band(na) && a_band(nb) && na.node == nb.node && na.path != nb.path,
        "the socket wall and the copy's spigot wall, two pieces of one revolve: {:?}",
        rows[2]
    );
    let axis = axis_port(&cd.doc, na.node);
    let ev = run(&cd.doc);
    let proof = proved_rows(&cd.doc, &ev)
        .into_iter()
        .find_map(|(_, rows_, proofs)| {
            rows_
                .iter()
                .position(|row| *row == rows[2])
                .map(|i| proofs[i].clone())
        })
        .expect("the wall row is proved");
    assert!(
        matches!(
            &proof,
            Proof::Unproven { residual, .. } if residual.axes.is_some_and(|a| {
                a.reached == [Some(axis); 2] && a.left == coincide::Component::Radius
            })
        ),
        "the socket and the spigot are one axis, the revolve's read twice, and two radii: \
         {proof:?}"
    );
}

/// **A boss typed tangent to a cylinder glues one unproven tangency**
/// (INTENT stage 6, `boss_tangent_by_value`). The union records one
/// `SurfaceTangent` row at `TangentWitness` over the cylinder's wall and
/// the boss's wall typed onto it, unproven: the witness lane's margin is
/// sampled, and the two walls are two constructions. The flush bases and
/// the corners on them prove as one construction; the corners on the
/// wall do not. At `Interval` the union escalates, so the scene stands
/// beside the registry.
///
/// Red if the scene's rows change in kind, site or proof, or once its
/// union passes at `Interval` (then it joins the registry); INTENT stage
/// 6 PR D proves a tangency the sketch constructs, not this one.
#[test]
fn a_boss_typed_tangent_to_a_cylinder_glues_one_unproven_tangency() {
    let cd = crate::corpus::boss_tangent_by_value::document();
    let (rows, lines) = door_lines(&cd);
    let on = "OnCarrier VertexFusion SameConstruction";
    let off = "OnCarrier VertexFusion unproven/Numeric/OneConstruction";
    let meet = "Meet VertexFusion unproven/Numeric/OneConstruction";
    assert_eq!(
        lines,
        [
            "SameOriented PlaneLadder SameConstruction",
            "SurfaceTangent TangentWitness unproven/Numeric/OneConstruction",
            on,
            off,
            meet,
            on,
            off,
            meet,
        ]
    );
    let [(a, na), (b, nb)] = two_cells(&rows[1]);
    assert!(
        a != b
            && [na, nb]
                .iter()
                .all(|n| matches!(n.path.as_slice(), [RoleSeg::Lateral(_)])),
        "the cylinder's wall and the boss's: {:?}",
        rows[1]
    );
    escalates_at_interval(&cd);
}

/// **Where a stage-6 scene's union refuses today** (INTENT stage 6):
/// the scenes beside the corpus registry, each refused at its union, by
/// kind.
///
/// - `cone_on_sphere`: a cone typed tangent to a sphere along a circle;
///   `CurvedPierceUnsupported`, or at a tighter ε an escalation on the
///   conic–quadric reading that comes before it (at `1e-12`,
///   `bool_conic_quadric_disc`).
/// - `shaft_and_bearing_rounded`: a shaft crossing the torus of a bore's
///   rounded mouth, coaxial with it; `CurvedBooleanUnsupported` on the
///   torus.
/// - `shaft_and_bearing_ball`: a sphere on the shaft, its centre on the
///   axis; `CurvedBooleanUnsupported` on the shaft's cylinder.
///
/// So no document reaches the sites `CoaxialCylinderTorus` or
/// `CoaxialSphere`
/// (`work/germ/coaxial-unions-with-a-torus-or-a-sphere-and-a-tangent-cone-refuse.md`).
///
/// Red once a union passes: that scene then joins the registry and its
/// rows are pinned like the others.
#[test]
fn the_curved_stage6_scenes_refuse_at_their_unions() {
    use editor_core::NodeErrorKind::Boolean;
    use geom::SurfaceKind;
    use topo::BooleanError as B;
    let refused = |cd: crate::corpus::CorpusDoc, kind: &dyn Fn(&B) -> bool| {
        let ev = run(&cd.doc);
        let union = cd.result.expect("a scene's head is its union");
        match failure(&ev, union) {
            Some(Boolean(e)) => assert!(kind(e), "{}: {e:?}", cd.name),
            other => panic!("{}'s union refuses as a boolean: {other:?}", cd.name),
        }
    };
    refused(crate::corpus::cone_on_sphere::document(), &|e| {
        matches!(e, B::CurvedPierceUnsupported { .. })
            || matches!(e, B::Escalated { diag, .. }
                if diag.predicate.is_some_and(|p| p.starts_with("bool_conic_quadric")))
    });
    refused(crate::corpus::shaft_and_bearing::rounded_mouth(), &|e| {
        matches!(
            e,
            B::CurvedBooleanUnsupported {
                kind: SurfaceKind::Torus,
                ..
            }
        )
    });
    refused(crate::corpus::shaft_and_bearing::with_a_ball(), &|e| {
        matches!(
            e,
            B::CurvedBooleanUnsupported {
                kind: SurfaceKind::Cylinder,
                ..
            }
        )
    });
}

/// **Two blocks resting on one floor are one plane at their bases**
/// (INTENT stage 6, `blocks_on_one_floor`). Each block is mated to the
/// floor's top, never to the other block; the two rests generate the
/// bases' equivalence (`[coincidence.structural]`), so the union's flush
/// pair of bases is proven by the first rung. Its tops and side walls
/// flush, and its abutting walls opposed, are flush through the cubes'
/// heights and the slides' values, no mate's statement, and are left
/// over the budget. The world's `⊔` records the union's base, which the
/// union mints from the two, on the floor's top at rest, unproven alike.
///
/// Red if the scene's rows change in kind, site or proof.
#[test]
fn two_blocks_resting_on_one_floor_are_one_plane_at_their_bases() {
    let cd = crate::corpus::blocks_on_one_floor::document();
    let (_, lines) = door_lines(&cd);
    let flush = "SameOriented PlaneLadder unproven/Numeric/Budget";
    let on = "OnCarrier VertexFusion unproven/Numeric/OneConstruction";
    let on_budget = "OnCarrier VertexFusion unproven/Numeric/Budget";
    assert_eq!(
        lines,
        [
            flush,
            "SameOriented PlaneLadder SameConstruction",
            flush,
            "SameOpposite PlaneLadder unproven/Numeric/Budget",
            flush,
            on,
            on,
            on_budget,
            on_budget,
        ]
    );
    let ev = run(&cd.doc);
    let at_rest: Vec<String> = at_rest_rows(&cd.doc, &ev)
        .iter()
        .map(|(node, row)| {
            format!(
                "{:?} {:?} {}",
                row.relation,
                row.site,
                proof_words(&proof(&cd.doc, *node, row))
            )
        })
        .collect();
    assert_eq!(
        at_rest,
        ["SameOpposite CensusAtRest unproven/Numeric/Budget"],
        "the union rests on the floor"
    );
}

/// **Two revolves reading one pair of axis variables glue unproven, one
/// axis and two radii** (INTENT stage 6, `revolves_on_one_axis`). Both
/// rings' `axis_through` slots read `ax` and `ay`; the second stands on
/// the first in its space. The union reads the two bores and the two
/// walls on one carrier each and glues the second's base to the first's
/// top, every row unproven. The two axes are one line in the symbols,
/// each revolve's own `axis` output; each radius is a literal coordinate
/// less `ax`, its own symbol, so the bores and the walls are left at the
/// radius.
///
/// Red if the scene's rows change in kind, site or proof, or the walls'
/// residual stops naming the radius.
#[test]
fn two_revolves_reading_one_axis_glue_unproven() {
    let cd = crate::corpus::revolves_on_one_axis::document();
    let (_, lines) = door_lines(&cd);
    let carrier = "SameOriented CarrierLadder unproven/Numeric/OneConstruction";
    let on = "OnCarrier VertexFusion unproven/Numeric/OneConstruction";
    assert_eq!(
        lines,
        [
            carrier,
            "SameOpposite PlaneLadder unproven/Numeric/OneConstruction",
            carrier,
            on,
            on,
        ]
    );
    let ev = run(&cd.doc);
    let axes: Vec<_> = proved_rows(&cd.doc, &ev)
        .into_iter()
        .flat_map(|(_, _, proofs)| proofs)
        .filter_map(|proof| match proof {
            Proof::Unproven { residual, .. } => residual.axes,
            Proof::Structural(_) => None,
        })
        .collect();
    assert!(
        axes.len() == 2
            && axes
                .iter()
                .all(|a| a.left == coincide::Component::Radius && a.reached == [None, None]),
        "the bores and the walls: two revolves' own axes, which no mate reaches, one line, and \
         two radii: {axes:?}"
    );
}

/// **The in-space axis identities of INTENT stage 6's scenes, at
/// `Sym`**, under the door's own replay session: for every mate whose
/// two sides are axes, the copy's axis carried into the target space
/// by the placement's solved motion against the target's axis, as the
/// six residuals of one line on another (`d′ × d` and `(f − f′) × d`);
/// and for a mate side that reads a revolve's wall, that wall's axis
/// against the revolve's own `axis` output carried by the same copy. One
/// line a residual (`AXIS <document> <placement> <identity> <component>
/// <discharge> <dag size>`). A measurement for the designers weighing
/// stage 6's coaxial rung, not a gate.
#[test]
#[ignore = "a measurement: cargo nextest run -p editor-core --cargo-profile release --run-ignored only --no-capture -E 'test(=coincidence_door::measure_the_stage6_axis_identities)'"]
fn measure_the_stage6_axis_identities() {
    use editor_core::pose::PoseValue;
    use geom_core::sym::deferred;
    use geom_core::{Band, Decide, Point3, Sym};
    type S = Sym<f64>;
    let band = Band::linear(Tol::witness()).expect("a band");
    let scenes = crate::corpus::documents()
        .into_iter()
        .chain(crate::corpus::stage6_beside_the_registry())
        .filter(|cd| {
            [
                "pin_in_bore",
                "pin_in_bore_two_variables",
                "shaft_and_bearing",
                "axis_projection",
                "revolves_on_one_axis",
            ]
            .contains(&cd.name)
        });
    for cd in scenes {
        let doc = &cd.doc;
        let opts = editor_core::EvalOptions {
            symbols: true,
            ..editor_core::EvalOptions::default()
        };
        coincide::with_replay_session(opts, |opts| {
            let ev: Evaluation<S> = editor_core::evaluate(
                doc,
                None,
                &editor_core::CancelToken::new(),
                opts,
                Tol::witness(),
            );
            let solved = editor_core::mate::SolvedPoses::of(doc, &ev);
            let axis_of = |slot, var: VarId| match editor_core::eval::bound_pose(
                doc,
                &ev,
                slot,
                var,
                Tol::witness(),
            ) {
                Ok(PoseValue::Axis(line)) => Some(line),
                _ => None,
            };
            let report = |what: &str, placement: RecipeNodeId, residuals: [S; 6]| {
                for (i, r) in residuals.into_iter().enumerate() {
                    let margin = r.sign_within(band).map(|d| d.margin);
                    let (why, size) = match margin {
                        Ok(m) => (
                            format!("{:?}", deferred::discharge_or_why(m)),
                            format!("{:?}", deferred::dag_size(m)),
                        ),
                        Err(e) => (format!("undecided {e:?}"), String::new()),
                    };
                    println!(
                        "AXIS {} {} {what} {i} {why} {size}",
                        cd.name,
                        placement.0.ordinal()
                    );
                }
            };
            let residuals = |(fa, da): (Point3<S>, geom_core::Vec3<S>),
                             b: &editor_core::pose::AxisLine<S>| {
                let (db, fb) = (b.dir().get(), b.foot());
                let cross = da.cross(db);
                let off = (fb - fa).cross(db);
                [cross.x, cross.y, cross.z, off.x, off.y, off.z]
            };
            for placement in doc.placements() {
                let Some(map) = solved.relative_map(placement) else {
                    continue;
                };
                let carried = |line: &editor_core::pose::AxisLine<S>| {
                    (
                        map.transform_point(line.foot()),
                        map.transform_vec(line.dir().get()),
                    )
                };
                // The revolve the copy copies, where it copies one.
                let revolve = match doc.node(placement) {
                    Some(Node::Place { source, .. }) => doc
                        .operation_of(source.read)
                        .filter(|&n| matches!(doc.node(n), Some(Node::Revolve { .. }))),
                    _ => None,
                };
                for c in crate::fixture::bundle(doc, placement) {
                    let Some(mate) = c.mate() else { continue };
                    let side = |side| editor_core::OperandSlot::Side {
                        constraint: c.id,
                        side,
                    };
                    let (Some(on), Some(to)) = (
                        axis_of(side(editor_core::MateSide::On), mate.on),
                        axis_of(side(editor_core::MateSide::To), mate.to),
                    ) else {
                        continue;
                    };
                    report("carried-on≡to", placement, residuals(carried(&on), &to));
                    // The copy's side against its revolve's own axis
                    // output, both in the revolve's coordinates.
                    if let Some(own) = revolve
                        .and_then(|r| doc.output(r, 1))
                        .and_then(|axis| axis_of(side(editor_core::MateSide::On), axis))
                    {
                        report(
                            "on≡revolve-axis",
                            placement,
                            residuals((own.foot(), own.dir().get()), &on),
                        );
                    }
                }
            }
        });
    }
}

// ---- INTENT stage 6 PR B: a coaxiality is one axis read twice ----

/// **The door's proofs of `rows`, decided at `node` of `doc`**, its
/// parts resolved through `opts`'s resolver.
fn proofs_with(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    node: RecipeNodeId,
    rows: &[NamedCoincidence],
) -> Vec<Proof> {
    coincide::prove(
        doc,
        &[(node, rows)],
        opts.resolver.as_ref(),
        Tol::witness(),
        &editor_core::CancelToken::new(),
    )
    .expect("an uncanceled door")
    .remove(0)
}

/// **An instanced shaft mated onto this document's revolve's `axis` is
/// coaxial with the bore by construction, with no replay**. The shaft
/// is a part (a rectangle on its own axis, revolved); its instance is
/// placed in a housing revolve's space by one mate, the axis of the
/// instance's shaft wall on the housing's `axis` output, and unioned
/// with the housing. The union stands on a part, so the replay reads
/// none of its rows (`Unreplayed::ThroughAPart`): the wall pair it glues
/// is unproven there, which is the premise, and its finding says the
/// axes are one and the radii unread. The same two cells read as
/// a coaxial row are one axis at the first rung: the shaft wall's axis
/// by the mate, the bore wall's by its revolve's own statement (a band
/// is about its revolve's `axis` output), so `housing.axis` read twice.
/// (The kernel's union refuses every coaxial pair it would record as
/// `Coaxial` today, so the row is the wall pair's cells read for their
/// axes alone.)
///
/// Red if the first rung stops reading a band's axis as its revolve's
/// output, or a mate side of an instance's face, or if the union's row
/// on the part is replayed.
#[test]
fn an_instanced_shaft_mated_onto_a_revolves_axis_is_coaxial_by_construction() {
    use crate::corpus::{Recorder, revolved};
    use editor_core::{Coord, Dimension, Operand, TargetAxis};
    let rect = |inner: f64, outer: f64, (lo, hi): (f64, f64)| {
        let to = |x, y| ProgramTarget::Point(len2([x, y]));
        vec![
            ProgramStep::At(len2([inner, lo])),
            ProgramStep::LineTo(to(outer, lo)),
            ProgramStep::LineTo(to(outer, hi)),
            ProgramStep::LineTo(to(inner, hi)),
            ProgramStep::LineTo(ProgramTarget::Start),
        ]
    };
    let mut part = Recorder::new();
    part.doc = ProfileDoc::empty_derived("s6b-shaft", Tol::witness());
    let shaft = revolved(&mut part, rect(0.0, 0.5, (-2.0, 1.5)));
    let wall = shaft.wall(&part, 2);
    let mut store = crate::fixture::resolver::PartStore::new();
    let (part_ref, _) = store.insert_part((part.doc, shaft.revolve), Tol::witness());
    let opts = crate::fixture::resolver::with_resolver(store);

    let mut r = Recorder::new();
    let housing = revolved(&mut r, rect(0.5, 1.0, (0.0, 1.0)));
    let space = r
        .doc
        .operand_space(&housing.body())
        .expect("the housing is in its space");
    let instance = r.insert(Node::instantiate_part(
        part_ref,
        Operand::Var(space),
        editor_core::stand().bundle(),
    ));
    r.constrain(
        instance,
        crate::fixture::mate(
            crate::fixture::axis(Operand::select(Operand::output(instance, 0), vec![wall])),
            housing.axis(),
        ),
        vec![
            (
                Coord::Slide(TargetAxis::Y),
                Formula::zero(Dimension::Length),
            ),
            (Coord::SPIN, Formula::turn_over(4).expect("a quarter turn")),
        ],
    );
    let union = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![
            editor_core::BodyRead::plain(housing.body()),
            editor_core::BodyRead::plain(Operand::output(instance, 0)),
        ]),
    });
    let ev = editor_core::evaluate::<f64>(
        &r.doc,
        None,
        &editor_core::CancelToken::new(),
        &opts,
        Tol::witness(),
    );
    assert_eq!(
        crate::corpus::failures(&ev),
        Vec::<String>::new(),
        "the host evaluates"
    );
    let walls: Vec<NamedCoincidence> = rows(&ev, union)
        .into_iter()
        .filter(|row| row.site == DecisionSite::CarrierLadder)
        .collect();
    assert!(
        !walls.is_empty()
            && walls.iter().all(|row| {
                let [(a, na), (b, nb)] = two_cells(row);
                a != b && a_band(na) && a_band(nb)
            }),
        "the union glues the shaft's wall to the bore: {walls:?}"
    );
    let at_the_door = proofs_with(&r.doc, &opts, union, &walls);
    let housing_axis = axis_port(&r.doc, housing.revolve);
    assert!(
        at_the_door.iter().all(|p| matches!(
            p,
            Proof::Unproven { residual, .. }
                if residual.replay == coincide::Unreplayed::ThroughAPart
                    && residual.axes.is_some_and(|a| {
                        a.reached == [Some(housing_axis); 2]
                            && a.left == coincide::Component::RadiusUnread
                    })
        )),
        "a node's row on a part is not replayed, and its walls are one axis, housing.axis, \
         their radii unread: {at_the_door:?}"
    );
    let coaxial: Vec<NamedCoincidence> = walls
        .iter()
        .map(|row| NamedCoincidence {
            relation: Relation::Coaxial,
            ..row.clone()
        })
        .collect();
    assert_eq!(
        proofs_with(&r.doc, &opts, union, &coaxial),
        vec![Proof::Structural(Rung::SameConstruction); coaxial.len()],
        "the shaft's wall and the bore are housing.axis read twice"
    );
}

/// **A collar placed by values is two axes, and the finding names the
/// mate that would make them one** (INTENT stage 6,
/// `shaft_and_bearing_collar_by_values`). The collar stands where
/// `shaft_and_bearing` mates it, by its six values alone. The union
/// glues the shaft's wall to the collar's as there, unproven: the shaft's
/// axis reads the housing's `axis` output by its mate, the collar's is
/// its own revolve's, and the two do not reduce to one line in the
/// symbols. The finding names both and the edit: mate the collar's copy
/// onto the housing's axis, which the shaft's copy reads. The shaft's
/// wall against the bore is one axis still, left at its radius.
///
/// Red if the door reads a values-placed copy's axis as stated, or the
/// finding stops naming the edit.
#[test]
fn a_collar_placed_by_values_is_two_axes_and_the_finding_names_the_mate() {
    let cd = crate::corpus::shaft_and_bearing::collar_by_values();
    let ev = run(&cd.doc);
    assert_eq!(
        crate::corpus::failures(&ev),
        Vec::<String>::new(),
        "{} evaluates",
        cd.name
    );
    let axes: Vec<coincide::Axes> = proved_rows(&cd.doc, &ev)
        .into_iter()
        .flat_map(|(_, _, proofs)| proofs)
        .filter_map(|proof| match proof {
            Proof::Unproven { residual, .. } => residual.axes,
            Proof::Structural(_) => None,
        })
        .collect();
    use coincide::Component::{Axis, Radius};
    assert_eq!(
        axes.iter().map(|a| a.left).collect::<Vec<_>>(),
        [Radius, Radius, Axis, Axis],
        "the bore and the shaft one axis, the shaft and the collar two"
    );
    // The housing, the shaft and the collar are revolved in that order,
    // and the shaft and the collar placed in that order.
    let revolves: Vec<RecipeNodeId> = cd
        .doc
        .ids()
        .into_iter()
        .filter(|&id| matches!(cd.doc.node(id), Some(Node::Revolve { .. })))
        .collect();
    let housing_axis = axis_port(&cd.doc, revolves[0]);
    let copies: Vec<VarId> = cd
        .doc
        .ids()
        .into_iter()
        .filter(|&id| matches!(cd.doc.node(id), Some(Node::Place { .. })))
        .map(|p| cd.doc.output(p, 0).expect("a placement defines its copy"))
        .collect();
    let want = [
        (Some(housing_axis), Some(copies[0])),
        (None, Some(copies[1])),
    ];
    for a in axes.iter().filter(|a| a.left == Axis) {
        let got = [(a.reached[0], a.copies[0]), (a.reached[1], a.copies[1])];
        assert!(
            got == want || got == [want[1], want[0]],
            "the shaft's axis reaches the housing's, no mate reaches the collar's: {a:?}"
        );
    }
    let edit = format!(
        "Edit, for the axis: mate {}'s axis onto {}, which {} reads",
        tag(copies[1]),
        tag(housing_axis),
        tag(copies[0])
    );
    let findings: Vec<String> = unproven(&cd.doc, &ev)
        .iter()
        .map(ToString::to_string)
        .filter(|f| f.contains("the axes are two"))
        .collect();
    assert!(
        findings.len() == 2 && findings.iter().all(|f| f.contains(&edit)),
        "each shaft-and-collar wall finding names the edit {edit:?}: {findings:?}"
    );
}

/// **A variable read as a speaker with no document says it**: `read`
/// and its tag.
fn tag(var: VarId) -> String {
    format!("read {:012x}", var.0.digest() >> 16)
}

/// **The edit mates the copy no axis mate reaches, in either cell
/// order**. Two scenes of a housing, a shaft and a collar unioned,
/// each with one of the two copies mated onto the housing's `axis`
/// output and the other placed by values (slides off the axis literal
/// lengths, each its own variable), the union's members in an order
/// that names the two walls' cells the other way round from the
/// corpus scene. Each wall finding between the shaft and the collar
/// names the copy placed by values as the one to mate, onto the
/// housing's axis, which the mated copy reads, whichever cell the row
/// names first.
///
/// Red if the edit is chosen by cell order, or names a pose no mate
/// reaches.
#[test]
fn the_axis_edit_mates_the_copy_no_mate_reaches_in_either_cell_order() {
    use crate::corpus::{Recorder, placed_in, revolved};
    use editor_core::{Coord, Dimension, Operand, TargetAxis};
    let rect = |inner: f64, outer: f64, (lo, hi): (f64, f64)| {
        let to = |x, y| ProgramTarget::Point(len2([x, y]));
        vec![
            ProgramStep::At(len2([inner, lo])),
            ProgramStep::LineTo(to(outer, lo)),
            ProgramStep::LineTo(to(outer, hi)),
            ProgramStep::LineTo(to(inner, hi)),
            ProgramStep::LineTo(ProgramTarget::Start),
        ]
    };
    let none = || Formula::zero(Dimension::Angle);
    let by_values = |y: f64, spin: Formula| editor_core::Placed {
        mates: Vec::new(),
        values: vec![
            (Coord::Slide(TargetAxis::X), len(0.0)),
            (Coord::Slide(TargetAxis::Y), len(y)),
            (Coord::Slide(TargetAxis::Z), len(0.0)),
            (Coord::Spin(Some(TargetAxis::X)), none()),
            (Coord::Spin(Some(TargetAxis::Y)), spin),
            (Coord::Spin(Some(TargetAxis::Z)), none()),
        ],
    };
    let mut firsts = std::collections::BTreeSet::new();
    for shaft_mated in [true, false] {
        let mut r = Recorder::new();
        let housing = revolved(&mut r, rect(0.5, 1.0, (0.0, 1.0)));
        let shaft = revolved(&mut r, rect(0.0, 0.5, (-2.0, 1.5)));
        let quarter = || Formula::turn_over(4).expect("a quarter turn");
        let shaft_placed = if shaft_mated {
            editor_core::Placed {
                mates: vec![crate::fixture::mate(shaft.axis(), housing.axis())],
                values: vec![
                    (
                        Coord::Slide(TargetAxis::Y),
                        Formula::zero(Dimension::Length),
                    ),
                    (Coord::SPIN, quarter()),
                ],
            }
        } else {
            by_values(0.0, quarter())
        };
        let shaft_copy = placed_in(&mut r, &housing.body(), shaft.body(), shaft_placed);
        let collar = revolved(&mut r, rect(0.5, 0.75, (0.0, 0.5)));
        let collar_placed = if shaft_mated {
            by_values(-1.0, none())
        } else {
            editor_core::Placed {
                mates: vec![crate::fixture::mate(collar.axis(), housing.axis())],
                values: vec![
                    (Coord::Slide(TargetAxis::Y), len(-1.0)),
                    (Coord::SPIN, none()),
                ],
            }
        };
        let collar_copy = placed_in(&mut r, &housing.body(), collar.body(), collar_placed);
        let union = r.insert(Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                editor_core::BodyRead::plain(housing.body()),
                editor_core::BodyRead::plain(Operand::output(collar_copy, 0)),
                editor_core::BodyRead::plain(Operand::output(shaft_copy, 0)),
            ]),
        });
        let ev = run(&r.doc);
        assert_eq!(
            crate::corpus::failures(&ev),
            Vec::<String>::new(),
            "evaluates"
        );
        let [shaft_copy, collar_copy] = [shaft_copy, collar_copy]
            .map(|p| r.doc.output(p, 0).expect("a placement defines its copy"));
        let (mated, unmated) = if shaft_mated {
            (shaft_copy, collar_copy)
        } else {
            (collar_copy, shaft_copy)
        };
        let housing_axis = axis_port(&r.doc, housing.revolve);
        // The housing's own bore is read where it was built, in no
        // copy: its edit names no reader.
        let onto = format!(
            "Edit, for the axis: mate {}'s axis onto {}",
            tag(unmated),
            tag(housing_axis)
        );
        let edit = format!("{onto}, which {} reads", tag(mated));
        let findings: Vec<String> = unproven(&r.doc, &ev)
            .iter()
            .filter(|f| f.subject == editor_core::FindingSubject::Node(union))
            .map(ToString::to_string)
            .filter(|f| f.contains("the axes are two"))
            .collect();
        assert!(
            findings.iter().any(|f| f.contains(&edit))
                && findings
                    .iter()
                    .all(|f| f.contains(&edit) || (f.contains(&onto) && !f.contains(", which "))),
            "shaft mated {shaft_mated}: each finding names {edit:?}: {findings:?}"
        );
        let findings: Vec<&String> = findings.iter().filter(|f| f.contains(&edit)).collect();
        for f in &findings {
            firsts.insert(f.contains(&format!("the first cell's reads {}", tag(housing_axis))));
        }
    }
    assert_eq!(
        firsts.len(),
        2,
        "the two scenes name the mated cell first in one and second in the other"
    );
}

/// **A wall whose two ends are two variables is not one radius by B's
/// reading** (`shaft_and_bearing_split_bore`). The housing's bore wall
/// reads `bore` at one end and `bore2`, of equal value, at the other;
/// the revolve reads the leg as a cylinder from the two ends' values,
/// its radius one end's. The door's own reading of two cylinders asks
/// each wall's rims to be its radius, so it leaves the bore against the
/// shaft at the radius whichever end reads `bore2`. With `bore2` at
/// the top the kernel's carrier-pair verdict, reading the cylinder's
/// radius off the bottom end alone, proves the row anyway: a wall
/// decided from values that no door row records
/// (`work/coincide/a-revolves-wall-classification-is-no-door-row.md`).
///
/// Red if the door's cylinder reading proves a wall whose ends are two
/// variables, or once the kernel's wall classification is a door row
/// (then both orders are unproven).
#[test]
fn a_wall_whose_two_ends_are_two_variables_is_not_one_radius() {
    for bore2_at_top in [true, false] {
        let cd = crate::corpus::shaft_and_bearing::split_bore(bore2_at_top);
        let ev = run(&cd.doc);
        assert_eq!(
            crate::corpus::failures(&ev),
            Vec::<String>::new(),
            "evaluates"
        );
        // The housing's band is the first cell of the bore's two rows.
        let housing = cd
            .doc
            .ids()
            .into_iter()
            .find(|&id| matches!(cd.doc.node(id), Some(Node::Revolve { .. })))
            .expect("the housing is revolved");
        let bore: Vec<Proof> = proved_rows(&cd.doc, &ev)
            .into_iter()
            .flat_map(|(_, rows, proofs)| rows.into_iter().zip(proofs))
            .filter(|(row, _)| {
                row.site == DecisionSite::CarrierLadder
                    && two_cells(row).iter().any(|(_, n)| n.node == housing)
            })
            .map(|(_, p)| p)
            .collect();
        assert_eq!(
            bore.len(),
            2,
            "bore2 at top {bore2_at_top}: the bore's two rows"
        );
        for p in &bore {
            match (bore2_at_top, p) {
                (true, Proof::Structural(Rung::CarrierVerdict)) => {}
                (false, Proof::Unproven { residual, .. })
                    if residual
                        .axes
                        .is_some_and(|a| a.left == coincide::Component::Radius) => {}
                _ => panic!("bore2 at top {bore2_at_top}: {p:?}"),
            }
        }
    }
}

/// **A bundle the doors refuse states nothing** (review probe R1). A
/// housing and a shaft placed in it by values (its off-axis slides
/// literal lengths) are unioned. A ghost ring is placed twice beside
/// them, one copy's outer wall mated onto the housing's `axis`, the
/// other's inner wall onto the shaft copy's wall; a load then moves the
/// second mate into the first copy's bundle, which no edit door admits
/// (two axis mates: `Overconstrained`). Read as statements, the two
/// mates would make the housing's axis and the shaft's one through the
/// ghost's, and the bore and the shaft one carrier. The refused bundle
/// places no copy and states nothing, so the union's wall rows stay
/// unproven.
///
/// Red if the first rung reads a bundle the doors refuse.
#[test]
fn a_refused_bundle_states_nothing() {
    use crate::corpus::{Recorder, placed_in, revolved};
    use editor_core::{Coord, Dimension, DocEdit, FreeVar, Operand, TargetAxis, VarDecl, VarName};
    let rect = |inner: &Formula, outer: &Formula, (lo, hi): (f64, f64)| {
        let to = |x: &Formula, y| ProgramTarget::Point([x.clone(), len(y)]);
        vec![
            ProgramStep::At([inner.clone(), len(lo)]),
            ProgramStep::LineTo(to(outer, lo)),
            ProgramStep::LineTo(to(outer, hi)),
            ProgramStep::LineTo(to(inner, hi)),
            ProgramStep::LineTo(ProgramTarget::Start),
        ]
    };
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("bore"),
        def: VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.5)),
    });
    let bore = Formula::named(VarName::from_static("bore"), Dimension::Length);
    let housing = revolved(&mut r, rect(&bore, &len(1.0), (0.0, 1.0)));
    let shaft = revolved(&mut r, rect(&len(0.0), &bore, (-2.0, 1.5)));
    let shaft_wall = shaft.wall(&r, 2);
    let none = || Formula::zero(Dimension::Angle);
    let shaft_copy = placed_in(
        &mut r,
        &housing.body(),
        shaft.body(),
        editor_core::Placed {
            mates: Vec::new(),
            values: vec![
                (Coord::Slide(TargetAxis::X), len(0.0)),
                (Coord::Slide(TargetAxis::Y), len(0.0)),
                (Coord::Slide(TargetAxis::Z), len(0.0)),
                (Coord::Spin(Some(TargetAxis::X)), none()),
                (
                    Coord::Spin(Some(TargetAxis::Y)),
                    Formula::turn_over(4).expect("a quarter turn"),
                ),
                (Coord::Spin(Some(TargetAxis::Z)), none()),
            ],
        },
    );
    let union = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![
            editor_core::BodyRead::plain(housing.body()),
            editor_core::BodyRead::plain(Operand::output(shaft_copy, 0)),
        ]),
    });
    let ghost = revolved(&mut r, rect(&len(0.1), &len(0.2), (5.0, 6.0)));
    let (outer, inner) = (ghost.wall(&r, 2), ghost.wall(&r, 4));
    let mut ghost_on = |wall, to| {
        placed_in(
            &mut r,
            &housing.body(),
            ghost.body(),
            editor_core::Placed {
                mates: vec![crate::fixture::mate(
                    crate::fixture::axis(Operand::select(ghost.body(), vec![wall])),
                    to,
                )],
                values: vec![
                    (
                        Coord::Slide(TargetAxis::Y),
                        Formula::zero(Dimension::Length),
                    ),
                    (Coord::SPIN, Formula::zero(Dimension::Angle)),
                ],
            },
        )
    };
    let first = ghost_on(outer, housing.axis());
    let second = ghost_on(
        inner,
        crate::fixture::axis(Operand::select(
            Operand::output(shaft_copy, 0),
            vec![shaft_wall],
        )),
    );
    let text = editor_core::save(&r.doc, &[], Tol::witness()).expect("saves");
    let doctored = crate::wire::doctored(&text, |w| {
        let nodes = &mut w["snapshot"]["nodes"];
        let moved = nodes[second.0.to_string()]["Place"]["constraints"]
            .as_array_mut()
            .expect("a bundle")
            .remove(0);
        nodes[first.0.to_string()]["Place"]["constraints"]
            .as_array_mut()
            .expect("a bundle")
            .insert(1, moved);
    });
    let doc = editor_core::load(&doctored, Tol::witness())
        .expect("the load door admits each mate on its own")
        .doc;
    let ev = run(&doc);
    assert!(
        crate::corpus::failures(&ev)
            .iter()
            .any(|f| f.contains("Overconstrained")),
        "the doctored bundle is refused: {:?}",
        crate::corpus::failures(&ev)
    );
    let walls: Vec<Proof> = proved_rows(&doc, &ev)
        .into_iter()
        .filter(|(node, _, _)| *node == union)
        .flat_map(|(_, rows, proofs)| rows.into_iter().zip(proofs))
        .filter(|(row, _)| row.site == DecisionSite::CarrierLadder)
        .map(|(_, p)| p)
        .collect();
    assert!(
        !walls.is_empty() && walls.iter().all(|p| matches!(p, Proof::Unproven { .. })),
        "the bore and the shaft are two axes: {walls:?}"
    );
}

/// **The revolve's port named `axis`**.
fn axis_port(doc: &ProfileDoc, revolve: RecipeNodeId) -> VarId {
    let port = doc
        .node(revolve)
        .expect("a revolve")
        .outputs()
        .iter()
        .position(|p| p.name == "axis")
        .expect("a revolve defines an axis");
    doc.output(revolve, u8::try_from(port).expect("a port"))
        .expect("the axis output")
}

/// **Every cell the first rung reads about a revolve's `axis` output is
/// about it**, in the symbols: the arm (`coincide::axis_of`) checked
/// against the symbolic evaluation (`[coincidence.door]`). For every
/// revolve of every corpus document with one, the document is
/// evaluated at `Sym` in the door's replay session, and every face,
/// edge and vertex the revolve mints is handed to the arm as the walk
/// reads it. Each one the arm reads about the revolve's `axis` output
/// has its carrier read against that output's line: a cylinder's, a
/// cone's or a torus's axis, a sphere's centre, a plane's normal, a
/// circle's axis, a line itself, or a point, as the residuals of one
/// line on another (`d′ × d`, `(p − f) × d`) or of a point on it, every
/// one proven zero by the tier. The arm reads every band and nothing
/// else.
///
/// Red if a revolve mints a band off its own axis, or the arm reads a
/// cell about the axis that is not (a meridian edge, a cap), or misses
/// a band.
#[test]
fn every_band_a_revolve_mints_is_about_its_axis_output_at_sym() {
    use coincide::{Construction, Origin};
    use editor_core::pose::PoseValue;
    use geom::{Curve3, Surface};
    use geom_core::sym::deferred;
    use geom_core::{Band, Decide, Sym};
    type S = Sym<f64>;
    let band = Band::linear(Tol::witness()).expect("a band");
    let (mut bands, mut revolves) = (0usize, 0usize);
    let docs = crate::corpus::documents()
        .into_iter()
        .chain(crate::corpus::stage6_beside_the_registry());
    for cd in docs {
        let doc = &cd.doc;
        let revolve_ids: Vec<RecipeNodeId> = doc
            .ids()
            .into_iter()
            .filter(|&id| matches!(doc.node(id), Some(Node::Revolve { .. })))
            .collect();
        if revolve_ids.is_empty() {
            continue;
        }
        let opts = EvalOptions {
            symbols: true,
            ..EvalOptions::default()
        };
        coincide::with_replay_session(opts, |opts| {
            let ev: Evaluation<S> = editor_core::evaluate(
                doc,
                None,
                &editor_core::CancelToken::new(),
                opts,
                Tol::witness(),
            );
            for &revolve in &revolve_ids {
                let axis_var = axis_port(doc, revolve);
                let body_var = doc.output(revolve, 0).expect("a revolve defines its body");
                let Ok(PoseValue::Axis(line)) = editor_core::eval::bound_pose(
                    doc,
                    &ev,
                    editor_core::OperandSlot::Axis,
                    axis_var,
                    Tol::witness(),
                ) else {
                    assert!(
                        ev.value(revolve).is_none(),
                        "{}: a revolve that built its body reads its axis",
                        cd.name
                    );
                    continue;
                };
                let Some(table) = ev.table(body_var) else {
                    continue;
                };
                revolves += 1;
                let (foot, dir) = (line.foot(), line.dir().get());
                let line_on = |p: Point3<S>, d: Option<geom_core::Vec3<S>>| {
                    let mut r = vec![];
                    if let Some(d) = d {
                        let c = d.cross(dir);
                        r.extend([c.x, c.y, c.z]);
                    }
                    let o = (p - foot).cross(dir);
                    r.extend([o.x, o.y, o.z]);
                    r
                };
                for (name, entry) in table.iter() {
                    if name.node != revolve {
                        continue;
                    }
                    let cell = Construction {
                        origin: Origin::Minted(name.clone()),
                        maps: Vec::new(),
                    };
                    let about =
                        coincide::axis_of(doc, &cell).origin == Origin::Pose(axis_var, None);
                    assert!(
                        about || !a_band(name),
                        "{}: the arm reads every band about its revolve's axis: {name:?}",
                        cd.name
                    );
                    if !about {
                        continue;
                    }
                    let Entry::Unique(editor_core::EntityRef { body, key }) = entry else {
                        continue;
                    };
                    let framed = ev
                        .read_body(body_var, *body)
                        .expect("the revolve stands")
                        .expect("its body");
                    let body = framed.body();
                    let residuals = match key {
                        EntityKey::Face(face) => {
                            match body
                                .get_face(*face)
                                .and_then(|f| body.get_surface(f.surface))
                                .expect("a face has a surface")
                            {
                                Surface::Cylinder { origin, axis, .. } => {
                                    line_on(*origin, Some(*axis))
                                }
                                Surface::Cone { apex, axis, .. } => line_on(*apex, Some(*axis)),
                                Surface::Torus { center, axis, .. } => {
                                    line_on(*center, Some(*axis))
                                }
                                Surface::Sphere { center, .. } => line_on(*center, None),
                                Surface::Plane { normal, .. } => {
                                    let c = normal.cross(dir);
                                    vec![c.x, c.y, c.z]
                                }
                                other => panic!("{}: a revolve's band on {other:?}", cd.name),
                            }
                        }
                        EntityKey::Edge(edge) => match body
                            .get_edge(*edge)
                            .and_then(|e| body.get_curve_geom(e.curve))
                            .and_then(topo::CurveGeom::certified)
                            .map(geom_brep::EdgeCurve::carrier)
                            .expect("an edge has a carrier")
                        {
                            Curve3::Circle { center, axis, .. } => line_on(*center, Some(*axis)),
                            Curve3::Line { origin, dir } => line_on(*origin, Some(*dir)),
                            other => {
                                panic!("{}: an edge read about the axis on {other:?}", cd.name)
                            }
                        },
                        EntityKey::Body => {
                            panic!("{}: the arm reads a body about an axis", cd.name)
                        }
                        EntityKey::Vertex(v) => line_on(
                            *body
                                .get_vertex(*v)
                                .and_then(|v| body.get_point(v.point))
                                .expect("a vertex has a point"),
                            None,
                        ),
                    };
                    for (i, r) in residuals.into_iter().enumerate() {
                        let why = r
                            .sign_within(band)
                            .map(|d| deferred::discharge_or_why(d.margin));
                        assert!(
                            matches!(why, Ok(Ok(()))),
                            "{}: {name:?}'s residual {i} against its revolve's axis: {why:?}",
                            cd.name
                        );
                    }
                    bands += 1;
                }
            }
        });
    }
    assert!(
        revolves >= 6 && bands >= 30,
        "the corpus's revolves and their bands were read: {revolves} revolves, {bands} bands"
    );
}

/// **A stated axis proves a wall pair the fold does not**. A housing
/// is copied into its own space by values (slid, and spun half a
/// radian about its `y`, each a literal variable), and a shaft is mated
/// onto that copy's bore wall's axis, slid and spun by literal
/// variables; the bore and the shaft read one `bore`, and the two are
/// unioned. The kernel's carrier-pair verdict reads the shaft's axis as
/// the fold builds it from the copy's spun axis, which is over the
/// door's budget at `Sym`; the replay reads it as the mate states it,
/// the housing copy's bore axis, and proves the two walls one carrier
/// by their radii (one `bore`, at both ends of each wall) and senses.
///
/// Red if the replay reads a mated copy's axis by its fold.
#[test]
fn a_stated_axis_proves_a_wall_pair_the_fold_does_not() {
    use crate::corpus::{Recorder, placed_in, revolved};
    use editor_core::{Coord, Dimension, DocEdit, FreeVar, Operand, TargetAxis, VarDecl, VarName};
    let bore = Formula::named(VarName::from_static("bore"), Dimension::Length);
    let rect = |inner: &Formula, outer: &Formula, (lo, hi): (f64, f64)| {
        let to = |x: &Formula, y| ProgramTarget::Point([x.clone(), len(y)]);
        vec![
            ProgramStep::At([inner.clone(), len(lo)]),
            ProgramStep::LineTo(to(outer, lo)),
            ProgramStep::LineTo(to(outer, hi)),
            ProgramStep::LineTo(to(inner, hi)),
            ProgramStep::LineTo(ProgramTarget::Start),
        ]
    };
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("bore"),
        def: VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.5)),
    });
    let housing = revolved(&mut r, rect(&bore, &len(1.0), (0.0, 1.0)));
    let wall = housing.wall(&r, 4);
    let housing_copy = placed_in(
        &mut r,
        &housing.body(),
        housing.body(),
        editor_core::Placed {
            mates: Vec::new(),
            values: vec![
                (Coord::Slide(TargetAxis::X), len(3.0)),
                (Coord::Slide(TargetAxis::Y), len(0.5)),
                (Coord::Slide(TargetAxis::Z), len(0.25)),
                (
                    Coord::Spin(Some(TargetAxis::X)),
                    Formula::zero(Dimension::Angle),
                ),
                (Coord::Spin(Some(TargetAxis::Y)), crate::fixture::ang(0.5)),
                (
                    Coord::Spin(Some(TargetAxis::Z)),
                    Formula::zero(Dimension::Angle),
                ),
            ],
        },
    );
    let shaft = revolved(&mut r, rect(&len(0.0), &bore, (-2.0, 1.5)));
    let shaft_copy = placed_in(
        &mut r,
        &housing.body(),
        shaft.body(),
        editor_core::Placed {
            mates: vec![crate::fixture::mate(
                shaft.axis(),
                crate::fixture::axis(Operand::select(
                    Operand::output(housing_copy, 0),
                    vec![wall],
                )),
            )],
            values: vec![
                (Coord::Slide(TargetAxis::Y), len(0.0)),
                (Coord::SPIN, crate::fixture::ang(0.7)),
            ],
        },
    );
    let union = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![
            editor_core::BodyRead::plain(Operand::output(housing_copy, 0)),
            editor_core::BodyRead::plain(Operand::output(shaft_copy, 0)),
        ]),
    });
    let ev = run(&r.doc);
    assert_eq!(
        crate::corpus::failures(&ev),
        Vec::<String>::new(),
        "evaluates"
    );
    let walls: Vec<Proof> = proved_rows(&r.doc, &ev)
        .into_iter()
        .filter(|(node, _, _)| *node == union)
        .flat_map(|(_, rows, proofs)| rows.into_iter().zip(proofs))
        .filter(|(row, _)| row.site == DecisionSite::CarrierLadder)
        .map(|(_, p)| p)
        .collect();
    assert_eq!(
        walls,
        vec![Proof::Structural(Rung::CarrierVerdict); 2],
        "the bore and the shaft: one axis by the mate, one bore, one sense"
    );
}

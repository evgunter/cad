//! **An assertion at its site quiets an operation's unproven row**
//! (D10 `[intent.checked]`, `[assertions.quiet]`;
//! `docs/INTENT-STAGE5-SPEC.md` §4, "The contact half of the quieting
//! rule", §11 Q1).
//!
//! The contact half is one rule over a row's two cells, whatever node
//! decided it, and it quiets a row only by an assertion that says what
//! the row decided (`[assertions.quiet]`). A boolean's glue is quiet
//! under a holding `Gap = 0` over its two cells, selected on the
//! deciding node's input reads, and loud under one over any other pair.
//! A row no `Gap = 0` says — a split's ON row, whose second cell is its
//! tool's pose, a flush pair, a blend's turn, a tangency — is loud and
//! never refuses (CHECKS DS6).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, failure, run};
use crate::fixture::{against, axes_at, insert, len, union_over};

use editor_core::{
    AssertionRelation, AssertionVerdict, CheckEvidence, CheckFinding, CheckId, ChecksConfig,
    EntityKey, Entry, Evaluation, FindingSubject, LoopProgram, MeasurePrimitive, NamedCell,
    NamedCoincidence, Node, ProfileDoc, ProfileProgram, RecipeNodeId, Severity, SitedRef,
    StableName, ValuePayload, enforce_checks,
};
use geom_core::Tol;
use topo::{DecisionSite, Relation};

/// A turn about the skew axis `(1, 2, 3)` by 0.7 rad: the frame every
/// fixture here stands its copies on, so no row lies on an axis.
fn turned(origin: [f64; 3]) -> editor_core::Frame {
    let [u, v, _] = turned_axes();
    axes_at(origin, u, v)
}

/// [`turned`]'s axes, each the image of a unit axis.
fn turned_axes() -> [[f64; 3]; 3] {
    let k = {
        let n = 14f64.sqrt();
        [1.0 / n, 2.0 / n, 3.0 / n]
    };
    let (s, c) = 0.7f64.sin_cos();
    let rot = |e: [f64; 3]| {
        let dot = k[0] * e[0] + k[1] * e[1] + k[2] * e[2];
        let cross = [
            k[1] * e[2] - k[2] * e[1],
            k[2] * e[0] - k[0] * e[2],
            k[0] * e[1] - k[1] * e[0],
        ];
        [0, 1, 2].map(|i| e[i] * c + cross[i] * s + k[i] * dot * (1.0 - c))
    };
    [
        rot([1.0, 0.0, 0.0]),
        rot([0.0, 1.0, 0.0]),
        rot([0.0, 0.0, 1.0]),
    ]
}

/// `origin + t·w`, `w` the turned frame's z.
fn up(origin: [f64; 3], t: f64) -> [f64; 3] {
    let w = turned_axes()[2];
    [0, 1, 2].map(|i| origin[i] + t * w[i])
}

/// A `Gap(outer, inner) = 0` over two cells, each the reference a
/// selection of its read makes: the document after it, and the
/// assertion.
fn gap_zero(doc: &ProfileDoc, outer: &SitedRef, inner: &SitedRef) -> (ProfileDoc, RecipeNodeId) {
    let (doc, measure) = crate::fixture::measure_node(
        doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        vec![outer.clone(), inner.clone()],
    );
    let value = crate::fixture::value_of(&doc, measure);
    insert(
        doc,
        Node::Assertion {
            value,
            bound: len(0.0),
            relation: AssertionRelation::Equal,
        },
    )
}

/// The selection reference a row's entity cell is: its name, read at
/// the node defining the read the deciding node took it in through.
fn site(doc: &ProfileDoc, cell: &NamedCell) -> SitedRef {
    let NamedCell::Entity { input, name } = cell else {
        panic!("an entity cell: {cell:?}")
    };
    SitedRef::new(doc.operation_of(*input).expect("a live read"), name.clone())
}

/// Both cells of a row of two entity cells.
fn sites(doc: &ProfileDoc, row: &NamedCoincidence) -> [SitedRef; 2] {
    let [a, b] = row.cells.as_slice() else {
        panic!("a row of two cells: {row:?}")
    };
    [site(doc, a), site(doc, b)]
}

/// `doc`'s checks at the default configuration, after the assertion
/// `holding` is confirmed to hold.
fn checks(doc: &ProfileDoc, holding: Option<RecipeNodeId>) -> editor_core::ChecksReport {
    let ev = run(doc);
    if let Some(assertion) = holding {
        assert!(
            matches!(
                ev.value(assertion).map(|v| &v.payload),
                Some(ValuePayload::Assertion(AssertionVerdict::Holds { .. }))
            ),
            "the premise is an assertion that holds: {:?}",
            ev.nodes.get(&assertion)
        );
    }
    editor_core::run_checks(doc, &ev, &ChecksConfig::default(), Tol::witness())
        .unwrap_or_else(|e| panic!("the checks run: {e}"))
}

/// The lint's finding over `row`, decided at `node`.
fn finding_of(
    report: &editor_core::ChecksReport,
    node: RecipeNodeId,
    row: &NamedCoincidence,
) -> CheckFinding {
    let found: Vec<&CheckFinding> = report
        .findings
        .iter()
        .filter(|f| {
            f.check == CheckId::UnprovenCoincidence
                && f.subject == FindingSubject::Node(node)
                && matches!(&f.evidence, CheckEvidence::UnprovenCoincidence { row: r, .. } if **r == *row)
        })
        .collect();
    let [one] = found.as_slice() else {
        panic!("one finding over {row:?}: {found:?}\n{report}")
    };
    (*one).clone()
}

/// **Whether [`enforce_checks`] refuses `finding` at `Error`.** The
/// lint's knob is an `Advisory` and spells no `Error`, so the finding is
/// read alone under the knob of a check that does: what the refusing
/// path reads of a finding is its `quiet` and its evidence, not which
/// check it is.
fn refuses_at_error(report: &editor_core::ChecksReport, finding: &CheckFinding) -> bool {
    let mut alone = report.clone();
    alone.findings = vec![CheckFinding {
        check: CheckId::AtRest,
        ..finding.clone()
    }];
    let cfg = ChecksConfig {
        at_rest: Severity::Error,
        ..ChecksConfig::default()
    };
    enforce_checks(&alone, &cfg).is_err()
}

/// The rows `node` decided.
fn rows(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<NamedCoincidence> {
    if let Some(e) = failure(ev, node) {
        panic!("{node:?} refused: {e:?}");
    }
    ev.value(node)
        .expect("the node evaluated")
        .coincidences
        .to_vec()
}

/// **Two turned blocks, the small one resting on the big one's top,
/// unioned**: both are copies standing on [`turned`] in an anchor's
/// space, so the glue lies on no axis. `(doc, union)`.
fn turned_rest() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("atrest-op-glue", Tol::witness());
    let (doc, anchor) = block(doc, (10.0, 11.0), (10.0, 11.0), 1.0);
    let (doc, big) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, small) = block(doc, (0.25, 0.75), (0.25, 0.75), 0.5);
    let origin = [0.3, -0.2, 0.5];
    let (doc, big) = against(doc, anchor, big, &turned(origin));
    let (doc, small) = against(doc, anchor, small, &turned(up(origin, 1.0)));
    union_over(doc, &[big, small])
}

/// **A boolean's glue is quiet under a holding `Gap = 0` over its two
/// cells, and under nothing else.** Two turned blocks resting top to
/// bottom, unioned: the union's glue row (the big block's top against
/// the small block's bottom, by the plane ladder) is unproven, and an
/// assertion over the two cap faces, selected on the union's operands
/// (the result merges them away), quiets it in either role order. A
/// `Gap = 0` over the top and one of the small block's bottom corners
/// holds too, and quiets that corner's row, not the glue. Loud, the glue
/// refuses at `Error`; quiet, it does not.
///
/// *Breaks if* an operation's row is never quietable (the lint passes
/// `None`), the rule matches by operand pair rather than by cells, the
/// cells resolve against the result, or a quietable loud row never
/// refuses.
#[test]
fn a_booleans_glue_is_quiet_under_its_gap_and_nothing_else() {
    let (doc, union) = turned_rest();
    let ev = run(&doc);
    let got = rows(&ev, union);
    let glue = got
        .iter()
        .find(|r| (r.relation, r.site) == (Relation::SameOpposite, DecisionSite::PlaneLadder))
        .unwrap_or_else(|| panic!("a glue row: {got:?}"))
        .clone();
    let corners: Vec<&NamedCoincidence> = got
        .iter()
        .filter(|r| r.site == DecisionSite::VertexFusion && r.cells.len() == 2)
        .collect();
    assert_eq!(corners.len(), 4, "the small block's four corners: {got:?}");
    let loud = checks(&doc, None);
    let found = finding_of(&loud, union, &glue);
    assert!(found.quiet.is_none(), "nothing asserted: {found}");
    assert!(
        refuses_at_error(&loud, &found),
        "an assertion could quiet it, so loud it refuses at Error: {found}"
    );

    let [top, bottom] = sites(&doc, &glue);
    for (outer, inner, why) in [
        (&top, &bottom, "the row's cells"),
        (&bottom, &top, "either role order"),
    ] {
        let (doc, assertion) = gap_zero(&doc, outer, inner);
        let report = checks(&doc, Some(assertion));
        let found = finding_of(&report, union, &glue);
        assert_eq!(found.quiet, Some(assertion), "{why}: {report}");
        assert!(
            !refuses_at_error(&report, &found),
            "{why}: a quiet finding refuses nothing"
        );
    }

    let corner = corners[0];
    let [vertex, face] = sites(&doc, corner);
    let (doc, assertion) = gap_zero(&doc, &face, &vertex);
    let report = checks(&doc, Some(assertion));
    assert_eq!(
        finding_of(&report, union, &glue).quiet,
        None,
        "a Gap over another pair of the two operands quiets no glue: {report}"
    );
    assert_eq!(
        finding_of(&report, union, corner).quiet,
        Some(assertion),
        "it quiets the row over its own two cells: {report}"
    );
    for other in &corners[1..] {
        assert_eq!(
            finding_of(&report, union, other).quiet,
            None,
            "and no other corner"
        );
    }
}

/// **A flush pair stays loud under a `Gap = 0` that holds over its
/// cells, and never refuses.** Two turned blocks side by side, unioned:
/// their tops are one carrier, both material sides facing the same way,
/// a `SameOriented` row. The `Gap` between them reads zero and holds,
/// and the row stays loud: a flush pair is no contact, and only a flag
/// mate or the replay establishes it (`[assertions]`). It refuses at no
/// severity.
///
/// *Breaks if* the contact half reads a flush row, or a flush finding
/// refuses.
#[test]
fn a_flush_glue_stays_loud_under_a_holding_gap() {
    let doc = ProfileDoc::empty_derived("atrest-op-flush", Tol::witness());
    let (doc, anchor) = block(doc, (10.0, 11.0), (10.0, 11.0), 1.0);
    let (doc, left) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, right) = block(doc, (1.0, 2.0), (0.0, 1.0), 1.0);
    let at = turned([0.3, -0.2, 0.5]);
    let (doc, left) = against(doc, anchor, left, &at);
    let (doc, right) = against(doc, anchor, right, &at);
    let (doc, union) = union_over(doc, &[left, right]);
    let ev = run(&doc);
    let flush: Vec<NamedCoincidence> = rows(&ev, union)
        .into_iter()
        .filter(|r| r.relation == Relation::SameOriented)
        .collect();
    assert!(!flush.is_empty(), "the tops and bottoms are flush");
    let row = &flush[0];
    let [a, b] = sites(&doc, row);
    let (doc, assertion) = gap_zero(&doc, &a, &b);
    let report = checks(&doc, Some(assertion));
    let found = finding_of(&report, union, row);
    assert_eq!(found.quiet, None, "a flush pair is no contact: {report}");
    assert!(
        !refuses_at_error(&report, &found),
        "no assertion could quiet it, so it never refuses: {found}"
    );
}

/// The notched prism of `coincide-pinch`: its tip `(4, 1)` has both
/// neighbours above `y = 1`.
const NOTCHED: [(f64, f64); 9] = [
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

/// The notched prism, extruded one unit, split through its tip edge by
/// the plane of normal `normal`. `(doc, prism, split)`.
fn pinched(id: &str, normal: [f64; 3]) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived(id, Tol::witness());
    let (doc, p) = crate::fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![NOTCHED.to_vec()],
    );
    let (doc, prism) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
        },
    );
    let tool = crate::fixture::plane_on_frame(&doc, prism, [4.0, 1.0, 0.0], normal);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: prism.into(),
            tool,
        },
    );
    (doc, prism, split)
}

/// The name of `node`'s planar face on the plane through `p` with
/// normal `n`.
fn face_through(ev: &Evaluation<f64>, node: RecipeNodeId, p: [f64; 3], n: [f64; 3]) -> StableName {
    // Where the body stands: `p` is in its space's coordinates.
    let body = match &ev.value(node).expect("the node evaluated").payload {
        ValuePayload::Body(b) => crate::fixture::in_space(b),
        other => panic!("expected a body, got {other:?}"),
    };
    crate::fixture::table(ev, node)
        .iter()
        .find_map(|(name, entry)| match entry {
            Entry::Unique(r) => match r.key {
                EntityKey::Face(f) => {
                    let pose = topo::readback::face_pose(&body, f).ok()?;
                    let (o, a) = (pose.origin, pose.axis);
                    let across = (a.y * n[2] - a.z * n[1]).abs()
                        + (a.z * n[0] - a.x * n[2]).abs()
                        + (a.x * n[1] - a.y * n[0]).abs();
                    let off = (o.x - p[0]) * n[0] + (o.y - p[1]) * n[1] + (o.z - p[2]) * n[2];
                    (across < 1e-9 && off.abs() < 1e-9).then(|| name.clone())
                }
                _ => None,
            },
            Entry::Tied(_) => None,
        })
        .unwrap_or_else(|| panic!("a face of {node:?} on the plane through {p:?}, {n:?}"))
}

/// **A split's ON row is loud and never refuses: no `Gap` reads a
/// pose.** Its cells are the operand's vertex and the split's tool, a
/// pose and no cell of any body, so no assertion the quieting rule
/// accepts is written over them (`[assertions.quiet]`). The notched
/// prism split through its tip edge by a turned plane, and by `y = 1`:
/// a `Gap = 0` from a tip vertex to a face it lies on — the notch's
/// flank, or the face `y = 1` itself lies on — holds, and the row stays
/// loud and refuses at no severity.
///
/// *Breaks if* the contact half reads a tool cell as the operand's, or
/// a split's row refuses.
#[test]
fn a_splits_on_row_is_loud_and_never_refuses() {
    let (s, c) = 0.17f64.sin_cos();
    let flank_normal = {
        let n = 2f64.sqrt();
        [1.0 / n, 1.0 / n, 0.0]
    };
    for (id, normal, face_at, face_normal) in [
        (
            "atrest-op-split-turned",
            [-s, c, 0.0],
            [4.0, 1.0, 0.0],
            flank_normal,
        ),
        (
            "atrest-op-split-on-a-face",
            [0.0, 1.0, 0.0],
            [6.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ),
    ] {
        let (doc, prism, split) = pinched(id, normal);
        let ev = run(&doc);
        let got = rows(&ev, split);
        assert_eq!(got.len(), 2, "{id}: one row per tip vertex: {got:?}");
        let row = &got[0];
        assert_eq!(
            (row.relation, row.site),
            (Relation::OnCarrier, DecisionSite::SplitOn)
        );
        assert!(
            matches!(
                row.cells.as_slice(),
                [NamedCell::Entity { .. }, NamedCell::Tool { .. }]
            ),
            "{id}: an operand vertex on the tool: {row:?}"
        );
        let vertex = site(&doc, &row.cells[0]);
        let face = SitedRef::new(prism, face_through(&ev, prism, face_at, face_normal));
        let (doc, assertion) = gap_zero(&doc, &face, &vertex);
        let report = checks(&doc, Some(assertion));
        let found = finding_of(&report, split, row);
        assert_eq!(found.quiet, None, "{id}: no Gap reads the tool: {report}");
        assert!(
            !refuses_at_error(&report, &found),
            "{id}: so none could quiet it, and it never refuses: {found}"
        );
    }
}

/// **A blend's turn is quieted by no `Gap = 0`, and never refuses.** A
/// turned box's copy with both rims filleted turns at its eight corners,
/// one `CommonFoot` row each over the two rim edges meeting there,
/// unproven. What the row decided is that the two bands' feet on the
/// third edge are one point, which a `Gap = 0` does not say
/// (`[assertions.quiet]`: an assertion that says the same). The two
/// edges share the corner, so a `Gap = 0` over them holds at every value
/// of the box, and it quiets nothing.
///
/// *Breaks if* the contact half quiets a row by an assertion that does
/// not say what it decided, or a turn refuses.
#[test]
fn a_blends_turn_is_quieted_by_no_gap_and_never_refuses() {
    let doc = ProfileDoc::empty_derived("atrest-op-turn", Tol::witness());
    let (doc, anchor) = block(doc, (10.0, 11.0), (10.0, 11.0), 1.0);
    let (doc, the_box) = block(doc, (0.0, 2.0), (0.0, 1.5), 1.0);
    let ev = run(&doc);
    let body = match &ev.value(the_box).expect("the box evaluated").payload {
        ValuePayload::Body(b) => crate::fixture::in_space(b),
        other => panic!("expected a body, got {other:?}"),
    };
    let height = |v| body.get_point(body.get_vertex(v).unwrap().point).unwrap().z;
    // Both rims: every edge whose two ends stand at one height.
    let rims: Vec<StableName> = crate::fixture::table(&ev, the_box)
        .iter()
        .filter_map(|(name, entry)| match entry {
            Entry::Unique(r) => match r.key {
                EntityKey::Edge(e) => {
                    let he = body.get_edge(e).unwrap().he_plus;
                    let (s, t) = (
                        body.get_half_edge(he).unwrap().start,
                        body.half_edge_end(he).unwrap(),
                    );
                    ((height(s) - height(t)).abs() < 1e-12).then(|| name.clone())
                }
                _ => None,
            },
            Entry::Tied(_) => None,
        })
        .collect();
    assert_eq!(rims.len(), 8, "both rims: {rims:?}");
    let (doc, copy) = against(doc, anchor, the_box, &turned([0.3, -0.2, 0.5]));
    let (doc, fillet) = insert(doc, Node::fillet(copy, len(0.1), rims));
    let ev = run(&doc);
    let turns = rows(&ev, fillet);
    assert_eq!(turns.len(), 8, "one turn per corner: {turns:?}");
    for row in &turns {
        assert_eq!(
            (row.relation, row.site),
            (Relation::CommonFoot, DecisionSite::BatteryTurn)
        );
    }
    let row = &turns[0];
    let adjacent = |r: &NamedCoincidence| {
        let ends = |cell: &NamedCell| {
            let NamedCell::Entity { input, name } = cell else {
                panic!("an edge cell: {cell:?}")
            };
            let table = ev.table(*input).expect("the read's table");
            let Some(Entry::Unique(ent)) = table.lookup(name) else {
                panic!("{name:?} names one entity")
            };
            let EntityKey::Edge(e) = ent.key else {
                panic!("an edge: {ent:?}")
            };
            let body = ev.read_body(*input, ent.body).unwrap().unwrap().body();
            let he = body.get_edge(e).unwrap().he_plus;
            [
                body.get_half_edge(he).unwrap().start,
                body.half_edge_end(he).unwrap(),
            ]
        };
        let [a, b] = [ends(&r.cells[0]), ends(&r.cells[1])];
        a.iter().any(|v| b.contains(v))
    };
    assert!(
        turns.iter().all(adjacent),
        "every turn's two edges share the corner: {turns:?}"
    );
    let [outer, inner] = sites(&doc, row);
    let (doc, assertion) = gap_zero(&doc, &outer, &inner);
    let report = checks(&doc, Some(assertion));
    for row in &turns {
        let found = finding_of(&report, fillet, row);
        assert_eq!(found.quiet, None, "a Gap = 0 says no common foot: {report}");
        assert!(
            !refuses_at_error(&report, &found),
            "and none could quiet it, so it never refuses: {found}"
        );
    }
}

/// **A tangency is quieted by no `Gap = 0`, and never refuses.** A
/// rod resting on a plate's side face, both turned, unioned: the
/// tangency witness's row decided the rod's wall and the plate's face
/// tangent along a ruling, which no `Gap = 0` says.
///
/// *Breaks if* a tangency row refuses.
#[test]
fn a_tangency_is_quieted_by_no_gap_and_never_refuses() {
    let doc = ProfileDoc::empty_derived("atrest-op-tangent", Tol::witness());
    let (doc, anchor) = block(doc, (10.0, 11.0), (10.0, 11.0), 1.0);
    let (doc, plate) = block(doc, (-2.0, 2.0), (-2.0, 2.0), 1.0);
    // Its seam vertices at (±0.5, 2.5), off the ruling it touches along.
    let (doc, rod) = rod(doc, (0.0, 2.5), 0.5, 0.0, 1.0);
    let at = turned([0.3, -0.2, 0.5]);
    let (doc, plate) = against(doc, anchor, plate, &at);
    let (doc, rod) = against(doc, anchor, rod, &at);
    let (doc, union) = union_over(doc, &[plate, rod]);
    let ev = run(&doc);
    let got = rows(&ev, union);
    let tangent = got
        .iter()
        .find(|r| r.site == DecisionSite::TangentWitness)
        .unwrap_or_else(|| panic!("a tangency row: {got:?}"));
    let report = checks(&doc, None);
    let found = finding_of(&report, union, tangent);
    assert!(
        !refuses_at_error(&report, &found),
        "a tangency is no contact a Gap = 0 says: {found}"
    );
}

/// A rod of radius `r` about `(cx, cy)` on its own xy, `h` up its own
/// z, its circle's two seam vertices at `start` and half a turn on: a
/// construction.
fn rod(
    doc: ProfileDoc,
    (cx, cy): (f64, f64),
    r: f64,
    start: f64,
    h: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::circle_split(cx, cy, r, 2, start).expect("a finite circle")],
            ids: Vec::new(),
            on: None,
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(h),
        },
    )
}

/// A quarter turn: rods side by side along x keep their seam vertices
/// off the line they touch along.
const QUARTER: f64 = std::f64::consts::FRAC_PI_2;

/// **A finding refuses only where a `Gap = 0` over its row's cells
/// holds** (the review's external-tangency probe): two parallel rods
/// externally tangent, as two world copies at rest and as the operands
/// of a turned union. Their gap reads `−2r`-ish in either role order,
/// so no `Gap = 0` over a tangency's cells holds, and whatever finding
/// would refuse at `Error` has one that does.
///
/// *Breaks if* a refusing finding has no holding `Gap = 0` over its
/// cells: `quietable` reading admission rather than the rule.
#[test]
fn a_refusing_finding_has_a_gap_zero_that_holds() {
    for (r1, r2) in [(0.5, 0.5), (0.5, 0.3)] {
        let doc = ProfileDoc::empty_derived("atrest-op-rods-rest", Tol::witness());
        let (doc, a) = rod(doc, (0.0, 0.0), r1, QUARTER, 1.0);
        let (doc, b) = rod(doc, (0.0, 0.0), r2, QUARTER, 1.0);
        let (doc, _) = crate::fixture::place(doc, a);
        let (doc, _) = crate::fixture::in_world(
            doc,
            b,
            &editor_core::Frame::translation([r1 + r2, 0.0, 0.0]),
        );
        every_refusing_finding_has_a_holding_gap(&doc, &format!("at rest ({r1}, {r2})"));

        let doc = ProfileDoc::empty_derived("atrest-op-rods-union", Tol::witness());
        let (doc, anchor) = block(doc, (10.0, 11.0), (10.0, 11.0), 1.0);
        let (doc, a) = rod(doc, (0.0, 0.0), r1, QUARTER, 1.0);
        let (doc, b) = rod(doc, (r1 + r2, 0.0), r2, QUARTER, 0.5);
        let origin = [0.3, -0.2, 0.5];
        let (doc, a) = against(doc, anchor, a, &turned(origin));
        let (doc, b) = against(doc, anchor, b, &turned(up(origin, 0.25)));
        let (doc, _) = union_over(doc, &[a, b]);
        every_refusing_finding_has_a_holding_gap(&doc, &format!("unioned ({r1}, {r2})"));
    }
}

/// That every unproven-coincidence finding of `doc` that refuses at
/// `Error` has a `Gap = 0` over its row's two cells holding in some role
/// order.
fn every_refusing_finding_has_a_holding_gap(doc: &ProfileDoc, what: &str) {
    let report = checks(doc, None);
    for found in &report.findings {
        let CheckEvidence::UnprovenCoincidence { row, .. } = &found.evidence else {
            continue;
        };
        if !refuses_at_error(&report, found) {
            continue;
        }
        let [a, b] = sites(doc, row);
        let held = [(&a, &b), (&b, &a)].into_iter().any(|(outer, inner)| {
            let (doc, assertion) = gap_zero(doc, outer, inner);
            matches!(
                run(&doc).value(assertion).map(|v| &v.payload),
                Some(ValuePayload::Assertion(AssertionVerdict::Holds { .. }))
            )
        });
        assert!(
            held,
            "{what}: {row:?} refuses at Error, and no Gap = 0 over its cells holds"
        );
    }
}

//! **An assertion at its site quiets an operation's unproven row**
//! (D10 `[intent.checked]`, `[assertions.quiet]`;
//! `docs/INTENT-STAGE5-SPEC.md` §4, "The contact half of the quieting
//! rule", §11 Q1).
//!
//! The row half is one rule over a row's two cells, whatever node
//! decided it, and it quiets a row only by an assertion that says what
//! the row decided (`[assertions.quiet]`). A boolean's glue is quiet
//! under a holding `Gap = 0` over its two cells, selected on the
//! deciding node's input reads, and loud under one over any other pair.
//! A flush pair and a tangency read on their faces' sides are quiet the
//! same way, and a coincidence of poses under a `Distance = 0` between
//! the poses read off its cells (a split's tool, under a `Gap = 0` to
//! its vertex). A row no assertion says — a blend's turn, two radii
//! written apart — is loud, never refuses (CHECKS DS6), and names the
//! construction.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, failure, run};
use crate::fixture::{against, axes_at, insert, len, union_over};

use editor_core::{
    AssertionRelation, AssertionVerdict, CheckEvidence, CheckFinding, CheckId, ChecksConfig,
    EntityKey, Entry, Evaluation, FindingSubject, LoopProgram, MeasurePrimitive, NamedCell,
    NamedCoincidence, Node, Operand, ProfileDoc, ProfileProgram, RecipeNodeId, Severity, SitedRef,
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
    gap_by(doc, outer, inner, AssertionRelation::Equal, 0.0)
}

/// A `Gap(outer, inner)` related to `bound` by `relation`: the document
/// after it, and the assertion.
fn gap_by(
    doc: &ProfileDoc,
    outer: &SitedRef,
    inner: &SitedRef,
    relation: AssertionRelation,
    bound: f64,
) -> (ProfileDoc, RecipeNodeId) {
    measured_by(
        doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        [outer.clone().into(), inner.clone().into()],
        relation,
        bound,
    )
}

/// `primitive` over `refs`, related to `bound` by `relation`: the
/// document after it, and the assertion.
fn measured_by(
    doc: &ProfileDoc,
    primitive: MeasurePrimitive<u32>,
    refs: [Operand; 2],
    relation: AssertionRelation,
    bound: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, measure) = crate::fixture::measure_node(doc, primitive, refs.to_vec());
    let value = crate::fixture::value_of(&doc, measure);
    insert(
        doc,
        Node::Assertion { claim: editor_core::Claim::Value {
            value,
            bound: len(bound),
            relation,
        } },
    )
}

/// **The row's `Gap = 0` quiets it, and a holding `Gap ≥ 0` or
/// `Gap ≤ 0.01` over the same cells does not** ([`quiet_only_under`],
/// over the row's two cells).
fn quiet_only_under_its_gap_zero(
    doc: &ProfileDoc,
    subject: FindingSubject,
    row: &NamedCoincidence,
    what: &str,
) {
    let [a, b] = sites(doc, row);
    quiet_only_under(
        doc,
        subject,
        row,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        [a.into(), b.into()],
        what,
    );
}

/// **`primitive = 0` over `refs` quiets the finding over `row`, in
/// either order, and a holding `≥ 0` or `≤ 0.01` does not**: only a
/// relation against zero says the row, and a bound is a tolerance
/// (`[assertions.quiet]`). The finding, of `subject`, is loud and
/// refuses at `Error` with nothing asserted, and quiet and refusing
/// nothing under its assertion.
fn quiet_only_under(
    doc: &ProfileDoc,
    subject: FindingSubject,
    row: &NamedCoincidence,
    primitive: MeasurePrimitive<u32>,
    [a, b]: [Operand; 2],
    what: &str,
) {
    let loud = checks(doc, None);
    let found = finding_over(&loud, subject, row);
    assert!(found.quiet.is_none(), "{what}: nothing asserted: {found}");
    assert!(
        refuses_at_error(&loud, &found),
        "{what}: an assertion could quiet it, so loud it refuses at Error: {found}"
    );
    for (refs, order) in [
        ([a.clone(), b.clone()], "in the row's order"),
        ([b.clone(), a.clone()], "swapped"),
    ] {
        let (doc, assertion) = measured_by(doc, primitive, refs, AssertionRelation::Equal, 0.0);
        let report = checks(&doc, Some(assertion));
        let found = finding_over(&report, subject, row);
        assert_eq!(found.quiet, Some(assertion), "{what}, {order}: {report}");
        assert!(
            !refuses_at_error(&report, &found),
            "{what}, {order}: a quiet finding refuses nothing"
        );
    }
    for (relation, bound, variant) in [
        (AssertionRelation::AtLeast, 0.0, "the opposite sense, ≥ 0"),
        (AssertionRelation::AtMost, 0.01, "a tolerance, ≤ 0.01"),
    ] {
        let (doc, assertion) = measured_by(doc, primitive, [a.clone(), b.clone()], relation, bound);
        let report = checks(&doc, Some(assertion));
        let found = finding_over(&report, subject, row);
        assert_eq!(
            found.quiet, None,
            "{what}: {variant} quiets nothing: {report}"
        );
    }
}

/// **A finding no assertion says stays loud and never refuses**, with a
/// recourse that names the construction and asks for no assertion.
fn loud_with_a_construction_recourse(report: &editor_core::ChecksReport, found: &CheckFinding) {
    assert_eq!(found.quiet, None, "{report}");
    assert!(
        !refuses_at_error(report, found),
        "none could quiet it, so it never refuses: {found}"
    );
    let said = found.to_string();
    assert!(
        said.contains("Recourse: ") && !said.contains("assert"),
        "the recourse names the construction alone: {said}"
    );
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
    finding_over(report, FindingSubject::Node(node), row)
}

/// The lint's finding over `row`, of `subject`.
fn finding_over(
    report: &editor_core::ChecksReport,
    subject: FindingSubject,
    row: &NamedCoincidence,
) -> CheckFinding {
    let found: Vec<&CheckFinding> = report
        .findings
        .iter()
        .filter(|f| {
            f.check == CheckId::UnprovenCoincidence
                && f.subject == subject
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

/// **A flush pair is quiet under a `Gap = 0` over its cells, read on
/// their sides.** Two turned blocks side by side, unioned: their tops
/// are one carrier, both material sides facing the same way, a
/// `SameOriented` row. The `Gap` between them reads zero in either role
/// order, and says the flush pair (`[assertions]`); a `Gap ≥ 0` or a
/// `Gap ≤ 0.01` over them holds and says nothing.
///
/// *Breaks if* the row half reads no flush row, or a bound quiets.
#[test]
fn a_flush_glue_is_quiet_under_its_gap() {
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
    quiet_only_under_its_gap_zero(&doc, FindingSubject::Node(union), &flush[0], "a flush glue");
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

/// **A split's ON row is quiet under a `Gap = 0` between its tool's
/// plane and the vertex** (`[assertions.quiet]`: a vertex on a split's
/// plane, read off the split's tool). The notched prism split through
/// its tip edge by a turned plane, and by `y = 1`: the row's cells are
/// the operand's tip vertex and the split's tool, and a `Gap` from the
/// tool read as a plane to the vertex says it, as a `Distance = 0`
/// between them does (DESIGN's spelling). A `Gap = 0` from the
/// vertex to a face it lies on, the notch's flank or the face `y = 1`
/// lies on, holds and says nothing of the tool.
///
/// *Breaks if* a measure cannot read a split's tool, the rule reads a
/// tool cell as the operand's, or a face's gap quiets the tool's row.
#[test]
fn a_splits_on_row_is_quiet_under_its_tool_gap() {
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
        let [
            NamedCell::Entity { .. },
            NamedCell::Tool { input: tool, .. },
        ] = row.cells.as_slice()
        else {
            panic!("{id}: an operand vertex on the tool: {row:?}")
        };
        let vertex = site(&doc, &row.cells[0]);
        let face = SitedRef::new(prism, face_through(&ev, prism, face_at, face_normal));
        let (doc_face, assertion) = gap_zero(&doc, &face, &vertex);
        let report = checks(&doc_face, Some(assertion));
        assert_eq!(
            finding_of(&report, split, row).quiet,
            None,
            "{id}: a face's gap says nothing of the tool: {report}"
        );
        // A variable two readers share is named.
        let doc = crate::fixture::step(
            doc,
            editor_core::DocEdit::RenameVar {
                var: (*tool).into(),
                name: Some(editor_core::VarName::from_static("cut")),
            },
        )
        .0;
        quiet_only_under(
            &doc,
            FindingSubject::Node(split),
            row,
            MeasurePrimitive::Gap { outer: 0, inner: 1 },
            [Operand::Var(*tool), vertex.clone().into()],
            id,
        );
        let (doc, assertion) = measured_by(
            &doc,
            MeasurePrimitive::Distance { a: 0, b: 1 },
            [Operand::Var(*tool), vertex.into()],
            AssertionRelation::Equal,
            0.0,
        );
        let report = checks(&doc, Some(assertion));
        assert_eq!(
            finding_of(&report, split, row).quiet,
            Some(assertion),
            "{id}: a Distance = 0 between the tool and the vertex says it too: {report}"
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
/// of the box, and it quiets nothing. Its recourse is the construction
/// alone (`[intent.checked]`: two feet only the blend computes).
///
/// *Breaks if* the row half quiets a row by an assertion that does not
/// say what it decided, a turn refuses, or its recourse asks for an
/// assertion.
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
        loud_with_a_construction_recourse(&report, &found);
    }
}

/// **A rod resting on a plate is quiet under its `Gap = 0`.** A rod
/// lying on a plate's side face, both turned, unioned: the tangency
/// witness's row decided the rod's wall and the plate's face tangent
/// along a ruling, opposed, which is a contact, and a `Gap = 0` over
/// the two faces read on their sides says it in either role order.
///
/// *Breaks if* the plane and the cylinder have no `Gap` arm, the arm
/// reads the rod as a bore, or the row half reads no tangency.
#[test]
fn a_rod_on_a_plate_is_quiet_under_its_gap() {
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
    quiet_only_under_its_gap_zero(
        &doc,
        FindingSubject::Node(union),
        tangent,
        "a rod on a plate",
    );
}

/// **Two rods touching side by side read `Gap = 0`, not `−2r`**: two
/// world copies whose walls are two convex faces externally tangent,
/// which the gap reads at `d − r_a − r_b` in either role order. A
/// `Gap = 0` over the two walls holds and a `Gap ≤ −0.01` does not. (The
/// census has no lane for two curved faces of different parts yet, so
/// the pair is a could-not-look finding, not a contact row.)
///
/// *Breaks if* two convex walls read as a pin in a bore.
#[test]
fn two_rods_side_by_side_read_a_gap_of_zero() {
    let (r1, r2) = (0.5, 0.3);
    let doc = ProfileDoc::empty_derived("atrest-op-two-rods", Tol::witness());
    let (doc, a) = rod(doc, (0.0, 0.0), r1, QUARTER, 1.0);
    let (doc, b) = rod(doc, (0.0, 0.0), r2, QUARTER, 1.0);
    let (doc, p) = crate::fixture::place(doc, a);
    let (doc, q) = crate::fixture::in_world(
        doc,
        b,
        &editor_core::Frame::translation([r1 + r2, 0.0, 0.0]),
    );
    let wall = |doc: &ProfileDoc, node| {
        crate::fixture::fname(
            node,
            editor_core::RoleSeg::Lateral(crate::fixture::piece(doc, node, 0, 0).into()),
        )
    };
    let (wa, wb) = (
        SitedRef::new(p, wall(&doc, a)),
        SitedRef::new(q, wall(&doc, b)),
    );
    for (outer, inner, order) in [(&wa, &wb, "a outer"), (&wb, &wa, "b outer")] {
        for (relation, bound, holds) in [
            (AssertionRelation::Equal, 0.0, true),
            (AssertionRelation::AtMost, -0.01, false),
        ] {
            let (doc, assertion) = gap_by(&doc, outer, inner, relation, bound);
            let got = run(&doc);
            let held = matches!(
                got.value(assertion).map(|v| &v.payload),
                Some(ValuePayload::Assertion(AssertionVerdict::Holds { .. }))
            );
            assert_eq!(
                held,
                holds,
                "{order}, {relation:?} {bound}: {:?}",
                got.value(assertion).map(|v| &v.payload)
            );
        }
    }
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

/// A ball of radius `r` about the origin of its own y axis: a semicircle
/// revolved a full turn about y.
fn ball(doc: ProfileDoc, r: f64) -> (ProfileDoc, RecipeNodeId) {
    use editor_core::{ProgramArcData, ProgramStep, ProgramTarget};
    let meridian = LoopProgram::Chain(vec![
        ProgramStep::At(crate::fixture::len2([0.0, -r])),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(crate::fixture::len2([0.0, r])),
            b: crate::fixture::scl(1.0),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    revolved(doc, meridian)
}

/// `meridian` revolved a full turn about its own y axis.
fn revolved(
    doc: ProfileDoc,
    meridian: LoopProgram<editor_core::Formula>,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![meridian],
            ids: Vec::new(),
            on: None,
        }),
    );
    insert(
        doc,
        crate::fixture::revolve_about(
            p,
            (0.0, 0.0),
            crate::fixture::along_y(),
            crate::fixture::ang(std::f64::consts::TAU),
        ),
    )
}

/// The union `node`'s first row decided at `site` with `relation`.
fn row_at(
    ev: &Evaluation<f64>,
    node: RecipeNodeId,
    site: DecisionSite,
    relation: Relation,
) -> NamedCoincidence {
    let got = rows(ev, node);
    got.iter()
        .find(|r| (r.site, r.relation) == (site, relation))
        .unwrap_or_else(|| panic!("a {relation:?} row at {site:?}: {got:?}"))
        .clone()
}

/// The name of `node`'s first face on a `kind` carrier (a full revolve
/// mints two bands).
fn face_of_kind(ev: &Evaluation<f64>, node: RecipeNodeId, kind: geom::SurfaceKind) -> StableName {
    let faces = editor_core::select_where(
        ev,
        node,
        &editor_core::Selector::of(editor_core::NamePat::of_kind(editor_core::EntityKind::Face)),
        &[editor_core::GeomPred::SurfaceKind(
            editor_core::SurfaceKindSet::just(kind),
        )],
        &ProfileDoc::empty_derived("atrest-op-no-params", Tol::witness()).var_env::<f64>(),
        Tol::witness(),
    )
    .expect("a surface-kind selection is exact");
    faces
        .first()
        .unwrap_or_else(|| panic!("a {kind:?} face of {node:?}"))
        .clone()
}

/// **A row of `site` deciding `relation`** over two cells, each a face
/// of a copy: what the kernel would record over those two faces.
fn row_over(
    doc: &ProfileDoc,
    site: DecisionSite,
    relation: Relation,
    cells: [(RecipeNodeId, StableName); 2],
) -> NamedCoincidence {
    NamedCoincidence {
        cells: cells
            .into_iter()
            .map(|(copy, name)| NamedCell::Entity {
                input: doc.output(copy, 0).expect("a placement's copy"),
                name,
            })
            .collect(),
        relation,
        site,
        margin: geom_core::MarginDiag::value(0.0),
        discharge: topo::Discharge::Numeric,
        origin: editor_core::coincide::RowOrigin::Node,
    }
}

/// **A coincidence of poses is quiet under a `Distance = 0` between the
/// poses read off its two cells, and only there** (`[assertions.quiet]`):
/// a sphere's centre on a cylinder's or a torus's axis, a cylinder's or
/// a torus's axis on a torus's, and a blend's supports on one axis (a
/// ball's support by its centre, against a wall's axis or another
/// ball's centre), each asserted with the pose each cell's carrier has:
/// the centre off a ball, the axis off a wall or a ring. A `Distance ≥ 0` or `≤ 0.01` over the same poses says
/// nothing; and with the ball moved off the axis, the `Distance = 0`
/// fails and no assertion could quiet the row.
///
/// The kernel's boolean refuses these curved pairs today, so no document
/// reaches the rows: each is the row the kernel records over the two
/// faces, asked of the rule over two placed copies (a rod, a ball, two
/// rings, all turned about one axis).
///
/// *Breaks if* a measure cannot read a pose, the rule does not see a
/// pose as the cell it was read off, or a pose of the wrong kind quiets.
#[test]
fn a_coincidence_of_poses_is_quiet_under_its_distance() {
    use editor_core::test_support::{quiet_row, row_quietable};
    use geom::SurfaceKind as S;
    let ring = |doc, major: f64, minor: f64| {
        revolved(
            doc,
            LoopProgram::Circle {
                centre: crate::fixture::len2([major, 0.0]),
                radius: len(minor),
            },
        )
    };
    for off in [0.0, 0.05] {
        let doc = ProfileDoc::empty_derived("atrest-op-poses", Tol::witness());
        let (doc, anchor) = block(doc, (10.0, 11.0), (10.0, 11.0), 1.0);
        let (doc, post) = revolved(
            doc,
            LoopProgram::polygon([(0.0, -0.6), (0.4, -0.6), (0.4, 0.6), (0.0, 0.6)])
                .expect("finite corners"),
        );
        let (doc, ring_a) = ring(doc, 0.5, 0.2);
        let (doc, ring_b) = ring(doc, 0.9, 0.15);
        let (doc, ball) = ball(doc, 0.5);
        let (doc, bead) = self::ball(doc, 0.2);
        let at = turned([0.3, -0.2, 0.5]);
        let (doc, post) = against(doc, anchor, Operand::output(post, 0), &at);
        let (doc, ring_a) = against(doc, anchor, Operand::output(ring_a, 0), &at);
        let (doc, ring_b) = against(doc, anchor, Operand::output(ring_b, 0), &at);
        let moved = turned([0.3 + off, -0.2, 0.5]);
        let (doc, ball) = against(doc, anchor, Operand::output(ball, 0), &moved);
        let (doc, bead) = against(doc, anchor, Operand::output(bead, 0), &at);
        let ev = run(&doc);
        let wall = (post, face_of_kind(&ev, post, S::Cylinder));
        let torus_a = (ring_a, face_of_kind(&ev, ring_a, S::Torus));
        let torus_b = (ring_b, face_of_kind(&ev, ring_b, S::Torus));
        let sphere = (ball, face_of_kind(&ev, ball, S::Sphere));
        let small = (bead, face_of_kind(&ev, bead, S::Sphere));
        let sited = |(copy, name): &(RecipeNodeId, StableName)| SitedRef::new(*copy, name.clone());
        use crate::fixture::{axis, point_of};
        let on_axis = off == 0.0;
        for (site, relation, cells, refs, poses) in [
            (
                DecisionSite::CoaxialSphere,
                Relation::OnCarrier,
                [wall.clone(), sphere.clone()],
                [axis(sited(&wall)), point_of(sited(&sphere))],
                "a centre on a cylinder's axis",
            ),
            (
                DecisionSite::SphereOnTorusAxis,
                Relation::OnCarrier,
                [torus_a.clone(), sphere.clone()],
                [axis(sited(&torus_a)), point_of(sited(&sphere))],
                "a centre on a torus's axis",
            ),
            (
                DecisionSite::CoaxialCylinderTorus,
                Relation::Coaxial,
                [wall.clone(), torus_a.clone()],
                [axis(sited(&wall)), axis(sited(&torus_a))],
                "a cylinder's axis on a torus's",
            ),
            (
                DecisionSite::CoaxialTori,
                Relation::Coaxial,
                [torus_a.clone(), torus_b.clone()],
                [axis(sited(&torus_a)), axis(sited(&torus_b))],
                "two tori on one axis",
            ),
            (
                DecisionSite::BatterySupportAxis,
                Relation::Coaxial,
                [wall.clone(), torus_b.clone()],
                [axis(sited(&wall)), axis(sited(&torus_b))],
                "a blend's supports on one axis",
            ),
            (
                DecisionSite::BatterySupportAxis,
                Relation::Coaxial,
                [wall.clone(), sphere.clone()],
                [axis(sited(&wall)), point_of(sited(&sphere))],
                "a blend's ball support centred on its wall's axis",
            ),
            (
                DecisionSite::BatterySupportAxis,
                Relation::Coaxial,
                [small.clone(), sphere.clone()],
                [point_of(sited(&small)), point_of(sited(&sphere))],
                "a blend's two ball supports about one centre",
            ),
        ] {
            let moves = cells.iter().any(|(copy, _)| *copy == ball);
            if !on_axis && !moves {
                continue;
            }
            let row = row_over(&doc, site, relation, cells);
            let what = format!("{poses}, off by {off}");
            assert_eq!(
                row_quietable(&doc, &ev, &row, Tol::witness()),
                on_axis,
                "{what}: whether a Distance = 0 over its poses could hold"
            );
            let [a, b] = refs;
            let distance = MeasurePrimitive::Distance { a: 0, b: 1 };
            for (refs, order) in [
                ([a.clone(), b.clone()], "in order"),
                ([b.clone(), a.clone()], "swapped"),
            ] {
                let (doc, assertion) =
                    measured_by(&doc, distance, refs, AssertionRelation::Equal, 0.0);
                let ev = run(&doc);
                let quiet = quiet_row(&doc, &ev, &row, Tol::witness());
                assert_eq!(
                    quiet,
                    on_axis.then_some(assertion),
                    "{what}, {order}: {:?}",
                    ev.value(assertion).map(|v| &v.payload)
                );
            }
            for (relation, bound) in [
                (AssertionRelation::AtLeast, 0.0),
                (AssertionRelation::AtMost, 0.01),
            ] {
                let (doc, _) = measured_by(&doc, distance, [a.clone(), b.clone()], relation, bound);
                let ev = run(&doc);
                assert_eq!(
                    quiet_row(&doc, &ev, &row, Tol::witness()),
                    None,
                    "{what}: {relation:?} {bound} quiets nothing"
                );
            }
        }
    }
}

/// **Two crossing cylinders: their meeting axes are quiet under a
/// `Distance = 0` between the axes, and their one radius is quiet under
/// nothing.** Two rods of radius 0.5 each written apart, crossing at
/// right angles, both spun about their own axes off the section's
/// nodes, unioned. `AxesMeet` reads the skew arm of the axes'
/// distance; `SameRadius` is two written values, which the construction
/// says (one variable for both) and no assertion does: loud, never
/// refusing, its recourse the construction.
///
/// *Breaks if* the skew arm is no arm, or a radius row asks for an
/// assertion.
#[test]
fn crossing_axes_are_quiet_under_their_distance_and_a_radius_is_not() {
    use std::f64::consts::PI;
    let turn = |axis: [f64; 3], angle: f64| {
        editor_core::Frame::rotate_then_translate(axis, angle, [0.0; 3], crate::fixture::band())
            .expect("a literal axis")
    };
    let doc = ProfileDoc::empty_derived("atrest-op-crossing", Tol::witness());
    let (doc, a) = rod(doc, (0.0, 0.0), 0.5, 0.0, 2.4);
    let (doc, b) = rod(doc, (0.0, 0.0), 0.5, 0.0, 2.4);
    let down = editor_core::Frame::translation([0.0, 0.0, -1.2]);
    let phi = PI / 4.0;
    let (doc, a_copy) = against(doc, a, a, &turn([0.0, 0.0, 1.0], phi).compose(&down));
    let (doc, b) = against(
        doc,
        a,
        b,
        &turn([0.0, 1.0, 0.0], phi)
            .compose(&turn([1.0, 0.0, 0.0], PI / 2.0))
            .compose(&down),
    );
    let (doc, union) = union_over(doc, &[a_copy, b]);
    let ev = run(&doc);
    let meet = row_at(
        &ev,
        union,
        DecisionSite::CrossingCylinders,
        Relation::AxesMeet,
    );
    let radius = row_at(
        &ev,
        union,
        DecisionSite::CrossingCylinders,
        Relation::SameRadius,
    );
    let [wa, wb] = sites(&doc, &meet);
    quiet_only_under(
        &doc,
        FindingSubject::Node(union),
        &meet,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        [
            crate::fixture::axis(wa.clone()),
            crate::fixture::axis(wb.clone()),
        ],
        "crossing axes",
    );
    let (doc, assertion) = measured_by(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        [crate::fixture::axis(wa), crate::fixture::axis(wb)],
        AssertionRelation::Equal,
        0.0,
    );
    let report = checks(&doc, Some(assertion));
    loud_with_a_construction_recourse(&report, &finding_of(&report, union, &radius));
}

/// **R1 probe (PR 6103): a split whose tool is a face read as a plane**
/// (`PoseDef::Plane { face }`, the first spelling `Node::Split`'s doc
/// names: "a `Plane` read"). The notched prism split at `y = 1` by its
/// own face there read as a plane: the row's tool cell is that plane
/// var, read plainly, so the row is quietable and loud refuses at
/// `Error`; a holding `Gap(tool, vertex) = 0` must then quiet it. Red:
/// `read_as` reads a `Plane { face }` var as the face's cell with kind
/// `Plane`, never as the tool, so no assertion can quiet a row the rule
/// calls quietable.
#[test]
fn r1_probe_a_split_by_a_face_plane_is_quiet_under_its_tool_gap() {
    let id = "r1-probe-split-face-plane";
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
    let ev0 = run(&doc);
    let face = face_through(&ev0, prism, [6.0, 1.0, 0.0], [0.0, 1.0, 0.0]);
    let tool = Operand::Pose(Box::new(editor_core::pose::PoseDef::Plane {
        face: Operand::select(Operand::output(prism, 0), vec![face]),
    }));
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: prism.into(),
            tool: tool.into(),
        },
    );
    let ev = run(&doc);
    let got = rows(&ev, split);
    let row = got
        .iter()
        .find(|r| r.site == DecisionSite::SplitOn)
        .unwrap_or_else(|| panic!("a SplitOn row: {got:?}"))
        .clone();
    let [
        NamedCell::Entity { .. },
        NamedCell::Tool { input: tool, .. },
    ] = row.cells.as_slice()
    else {
        panic!("an operand vertex on the tool: {row:?}")
    };
    let tool = *tool;
    let vertex = site(&doc, &row.cells[0]);
    let loud = checks(&doc, None);
    let found = finding_of(&loud, split, &row);
    let refuses = refuses_at_error(&loud, &found);
    let doc = crate::fixture::step(
        doc,
        editor_core::DocEdit::RenameVar {
            var: tool.into(),
            name: Some(editor_core::VarName::from_static("cut")),
        },
    )
    .0;
    let (doc_a, assertion) = measured_by(
        &doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        [Operand::Var(tool), vertex.into()],
        AssertionRelation::Equal,
        0.0,
    );
    let report = checks(&doc_a, Some(assertion));
    assert_eq!(
        finding_of(&report, split, &row).quiet,
        Some(assertion),
        "loud refuses at Error: {refuses}; a holding Gap(tool, vertex) = 0 quiets nothing: {report}"
    );
}

/// **R2 probe: a split whose tool is a plane read off a face.** The
/// notched prism split by the plane of its own notch floor on `y = 1`
/// (`PoseDef::Plane { face }`, the common way to name a cutting plane).
/// The row's tool cell is that plane variable; a `Gap(tool, vertex) = 0`
/// holds. Red if the finding refuses at `Error` (quietable) and yet
/// that assertion does not quiet it.
#[test]
fn r2_probe_a_split_by_a_faces_plane_is_quiet_under_its_tool_gap() {
    let doc = ProfileDoc::empty_derived("r2-split-face-plane", Tol::witness());
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
    let ev = run(&doc);
    // The notch's floor, on `y = 1`, the plane the existing
    // `atrest-op-split-on-a-face` case cuts along, read off the face.
    let face = SitedRef::new(
        prism,
        face_through(&ev, prism, [6.0, 1.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let tool = editor_core::pose::RoleRead::plain(crate::fixture::plane(face));
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: prism.into(),
            tool,
        },
    );
    let ev = run(&doc);
    let got = rows(&ev, split);
    let row = got
        .iter()
        .find(|r| r.site == DecisionSite::SplitOn)
        .unwrap_or_else(|| panic!("a SplitOn row: {got:?}"))
        .clone();
    let NamedCell::Tool { input: tool, .. } = &row.cells[1] else {
        panic!("vertex, tool: {row:?}")
    };
    let tool = *tool;
    let vertex = site(&doc, &row.cells[0]);
    let doc = crate::fixture::step(
        doc,
        editor_core::DocEdit::RenameVar {
            var: tool.into(),
            name: Some(editor_core::VarName::from_static("cut")),
        },
    )
    .0;
    let loud = checks(&doc, None);
    let found = finding_of(&loud, split, &row);
    let refuses = refuses_at_error(&loud, &found);
    let (doc, assertion) = gap_zero_var(&doc, tool, &vertex);
    let report = checks(&doc, Some(assertion));
    let quiet = finding_of(&report, split, &row).quiet;
    assert!(
        !refuses || quiet == Some(assertion),
        "loud it refuses at Error ({refuses}) with recourse {found}, yet a holding \
         Gap(tool, vertex) = 0 quiets nothing: {quiet:?}"
    );
}

fn gap_zero_var(
    doc: &ProfileDoc,
    tool: editor_core::VarId,
    vertex: &SitedRef,
) -> (ProfileDoc, RecipeNodeId) {
    measured_by(
        doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        [Operand::Var(tool), vertex.clone().into()],
        AssertionRelation::Equal,
        0.0,
    )
}

/// **R2 probe: a Coaxial row is quieted by a `Distance = 0` between
/// two axes that only meet.** Two rings about axes at right angles
/// through one point: the `Distance` between their axes reads the skew
/// arm, zero, and holds. A `CoaxialTori` row over the two torus faces
/// (the row the kernel records, as `a_coincidence_of_poses_...` asks
/// it) should not be said by it: red if it quiets.
#[test]
fn r2_probe_meeting_axes_do_not_say_coaxial() {
    use editor_core::test_support::{quiet_row, row_quietable};
    use geom::SurfaceKind as S;
    let ring = |doc, major: f64, minor: f64| {
        revolved(
            doc,
            LoopProgram::Circle {
                centre: crate::fixture::len2([major, 0.0]),
                radius: len(minor),
            },
        )
    };
    let doc = ProfileDoc::empty_derived("r2-meeting-axes", Tol::witness());
    let (doc, anchor) = block(doc, (10.0, 11.0), (10.0, 11.0), 1.0);
    let (doc, ring_a) = ring(doc, 0.5, 0.2);
    let (doc, ring_b) = ring(doc, 0.9, 0.15);
    let at = turned([0.3, -0.2, 0.5]);
    let crossed = at.compose(
        &editor_core::Frame::rotate_then_translate(
            [1.0, 0.0, 0.0],
            std::f64::consts::FRAC_PI_2,
            [0.0; 3],
            crate::fixture::band(),
        )
        .expect("a literal axis"),
    );
    let (doc, ring_a) = against(doc, anchor, Operand::output(ring_a, 0), &at);
    let (doc, ring_b) = against(doc, anchor, Operand::output(ring_b, 0), &crossed);
    let ev = run(&doc);
    let torus_a = (ring_a, face_of_kind(&ev, ring_a, S::Torus));
    let torus_b = (ring_b, face_of_kind(&ev, ring_b, S::Torus));
    let sited = |(copy, name): &(RecipeNodeId, StableName)| SitedRef::new(*copy, name.clone());
    let row = row_over(
        &doc,
        DecisionSite::CoaxialTori,
        Relation::Coaxial,
        [torus_a.clone(), torus_b.clone()],
    );
    let quietable = row_quietable(&doc, &ev, &row, Tol::witness());
    let (doc, assertion) = measured_by(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        [
            crate::fixture::axis(sited(&torus_a)),
            crate::fixture::axis(sited(&torus_b)),
        ],
        AssertionRelation::Equal,
        0.0,
    );
    let ev = run(&doc);
    let verdict = ev.value(assertion).map(|v| v.payload.clone());
    let quiet = quiet_row(&doc, &ev, &row, Tol::witness());
    assert!(
        quiet.is_none(),
        "axes at right angles quiet a Coaxial row: quietable {quietable}, verdict {verdict:?}"
    );
}

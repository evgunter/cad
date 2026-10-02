//! **LIB-G14 — the split-naming walls, executed** (`docs/NAMING-
//! DESIGN.md`, "The split-naming walls"; RATIFIED on #512).
//!
//! Two disjoint M4-era deferrals used to make `Node::Split` refuse.
//! Both are gone here, and these rows are the measurement.
//!
//! **Wall B (B1)** — `upstream_name` refused the WHOLE downstream op
//! if ANY operand-table entry was `Entry::Tied`, even for pass-through
//! entities nowhere near the tie. That was stricter than N2's own
//! ratified text ("naming a tie is fine; REFERENCING one is
//! `Ambiguous`") and stricter than the three emitters that already got
//! it right (`name_pattern`, `name_in_part`, `graft_names`). Ties now
//! PROPAGATE, in both directions off the survey's measured fixture:
//! split-over-a-tied-boolean, and boolean-over-a-tied-boolean.
//!
//! **Wall A** — `RoleSeg::SectionEdge{side, face}` names a chord
//! only by the operand face it crosses, so a section line that enters
//! one face TWICE would mint one name twice. It refused, on scenes
//! with no boolean anywhere (a plain L-shaped single-loop extrude cut
//! across both legs). Those chords are pieces of one parent now, each
//! named by its ends (N2's `Ends`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;
use test_utils::refusal::tagged;

use editor_core::{
    BooleanOp, CancelToken, Cmp, CurveKind, CurveKindSet, Datum, EntityKind, Entry, EvalOptions,
    Evaluation, GeomPred, NamePat, NameTable, NamingError, Node, NodeErrorKind, ParamEnv,
    ProfileDoc, RecipeNodeId, RoleSeg, SegPat, SegTag, Selector, StableName, evaluate, select,
    select_where,
};

// `table` panics on a node with no value rather than answering `None`,
// which is what this suite wants: a G14 regression IS a node failure,
// and the panic's message is the diagnosis.
use fixture::{insert, len, on_frame, scl, table};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn ties(t: &NameTable) -> Vec<(&StableName, usize)> {
    t.iter()
        .filter_map(|(n, e)| match e {
            Entry::Tied(c) => Some((n, c.len())),
            Entry::Unique(_) => None,
        })
        .collect()
}

/// A plane datum.
fn plane(doc: ProfileDoc, origin: [f64; 3], normal: [f64; 3]) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(origin[0]), len(origin[1]), len(origin[2])],
            normal: [scl(normal[0]), scl(normal[1]), scl(normal[2])],
        }),
    )
}

/// A prism: the polygon `pts` on z = `z0`, extruded `dz`.
fn prism(doc: ProfileDoc, pts: Vec<(f64, f64)>, z0: f64, dz: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![pts],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
            side: ExtrudeSide::Along,
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

/// The survey's tie fixture, verbatim in shape: a 4×4×4 block minus a
/// U-shaped cutter whose two prongs cross one wall. The subtract's
/// table carries genuine N2 ties (two equally-admissible prong
/// fragments per cap — no covariant qualifier separates them).
fn u_cutter_tie(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
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
    (doc, a, sub)
}

// ---- Wall B (B1): ties propagate through the downstream op. ----

/// The survey's measured refusal: a `Split` at z = 3.5, far above the
/// tied prong fragments at z ∈ [1, 3], refused the WHOLE op because
/// ONE entry somewhere in the operand table was tied. It names now,
/// and the tie is still THERE — propagated, not laundered.
#[test]
fn split_over_a_tied_operand_names_and_keeps_the_tie() {
    let (doc, _, sub) = u_cutter_tie(ProfileDoc::empty_derived("lib_g14", Tol::witness()));
    let (doc, tool) = plane(doc, [0.0, 0.0, 3.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(doc, Node::Split { target: sub, tool });
    let ev = run(&doc);
    let up = table(&ev, sub);
    assert!(!ties(up).is_empty(), "fixture lost its operand tie");
    let t = table(&ev, split);
    // Totality is the emitter's own postcondition; what this row adds
    // is that the tie SURVIVED as a tie rather than refusing or being
    // silently split into two distinct names.
    let after = ties(t);
    assert!(
        !after.is_empty(),
        "the operand's tie did not propagate into the split"
    );
    for (n, k) in &after {
        assert_eq!(*k, 2, "propagated tie should keep 2 candidates: {n:?}");
    }
    // Every propagated tie is a SPLIT-side name wrapping the operand's
    // tied name — the parent is in there, structurally (N1).
    assert!(
        after.iter().any(|(n, _)| matches!(
            n.path.first(),
            Some(RoleSeg::SplitFragment { .. } | RoleSeg::FromB(_))
        )),
        "no propagated tie carries a split or operand role: {:?}",
        after.iter().map(|(n, _)| *n).collect::<Vec<_>>()
    );
}

/// The other direction of the same guard: `name_boolean` shares
/// `upstream_name`, so a boolean whose OPERAND table holds a tie
/// refused identically. It names now.
#[test]
fn boolean_over_a_tied_operand_names_and_keeps_the_tie() {
    let (doc, _, sub) = u_cutter_tie(ProfileDoc::empty_derived("lib_g14", Tol::witness()));
    // A second cutter, nowhere near the tied prong fragments: it
    // crosses A's top face at z = 4, well above z ∈ [1, 3].
    let (doc, c) = block(doc, (1.0, 3.0), (1.0, 3.0), 3.5, 2.0);
    let (doc, sub2) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: sub,
            b: c,
            declare: None,
        },
    );
    let ev = run(&doc);
    let t = table(&ev, sub2);
    let after = ties(t);
    assert!(
        !after.is_empty(),
        "the operand's tie did not propagate into the boolean"
    );
    for (n, k) in &after {
        assert_eq!(*k, 2, "propagated tie should keep 2 candidates: {n:?}");
    }
}

/// The NARROWING arm of the flush — the other half of the ratified
/// `graft_names` shape, and the one the propagation rows above cannot
/// reach: when a downstream op kills all but ONE candidate of a tie,
/// the survivor's name collapses back to `Entry::Unique` rather than
/// staying a one-candidate "tie" (which `insert_tied` would refuse
/// outright) or vanishing.
///
/// Adopted from the LIB-G14 review's lane-local probe
/// `probe_a_tie_with_one_survivor_narrows_to_unique` (review MINOR-2:
/// the arm shipped untested in the first cut).
#[test]
fn a_tie_with_one_surviving_candidate_narrows_back_to_unique() {
    let (doc, _, sub) = u_cutter_tie(ProfileDoc::empty_derived("lib_g14", Tol::witness()));
    // Engulf the y-high prong region entirely: every candidate in
    // y ∈ [2.5, 3] dies, the y ∈ [1, 1.5] one survives intact.
    let (doc, c) = block(doc, (1.5, 4.5), (2.25, 3.25), 0.5, 3.0);
    let (doc, sub2) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: sub,
            b: c,
            declare: None,
        },
    );
    let ev = run(&doc);
    let up_ties = ties(table(&ev, sub));
    assert!(!up_ties.is_empty(), "fixture lost its upstream tie");
    let t = table(&ev, sub2);
    for (tn, _) in &up_ties {
        let descendants: Vec<(&StableName, &Entry)> = t
            .iter()
            .filter(|(n, _)| match n.path.first() {
                Some(RoleSeg::FromA(inner)) => &**inner == *tn,
                _ => false,
            })
            .collect();
        assert_eq!(
            descendants.len(),
            1,
            "expected exactly one descendant of {tn:?}, got {descendants:?}"
        );
        assert!(
            matches!(descendants[0].1, Entry::Unique(_)),
            "one survivor must narrow to Unique: {:?}",
            descendants[0]
        );
    }
}

/// Ties are a candidate LIST, so their order is a determinism risk
/// that unique names do not carry. `insert_tied` sorts and dedups and
/// every table on the path is a `BTreeMap`; this pins that end to end,
/// over a propagated tie (B1), and over chords told apart by their
/// ends.
///
/// Adopted from the review's `probe_naming_is_deterministic_across_runs`.
#[test]
fn tied_and_end_qualified_name_tables_are_identical_across_evaluations() {
    let builds: [fn() -> (ProfileDoc, RecipeNodeId); 2] = [
        || {
            let (doc, _, n) = l_split(ProfileDoc::empty_derived("lib_g14", Tol::witness()));
            (doc, n)
        },
        || {
            let (doc, _, sub) = u_cutter_tie(ProfileDoc::empty_derived("lib_g14", Tol::witness()));
            let (doc, tool) = plane(doc, [0.0, 0.0, 3.5], [0.0, 0.0, 1.0]);
            let (doc, split) = insert(doc, Node::Split { target: sub, tool });
            (doc, split)
        },
    ];
    for (build, exercises) in builds.into_iter().zip(["Ends", "Tied"]) {
        let (doc, n) = build();
        let a = format!("{:?}", table(&run(&doc), n));
        let b = format!("{:?}", table(&run(&doc), n));
        assert_eq!(a, b, "naming differed across two evaluations");
        assert!(a.contains(exercises), "fixture should exercise {exercises}");
    }
}

// ---- Wall A: two chords across one operand face, by their ends. ----

/// The survey's boolean-free repro. An L-shaped SINGLE-loop extrude
/// (no hole, no boolean anywhere in the document) cut by a vertical
/// plane that crosses both legs: each CAP face takes TWO section
/// chords, which `SectionEdge{side, face}` alone spells once.
fn l_split(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    // L in xy: [0,3]×[0,1] ∪ [0,1]×[1,3], extruded z ∈ [0,1].
    let (doc, ext) = prism(
        doc,
        vec![
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ],
        0.0,
        1.0,
    );
    // x + y = 2.5: two DISJOINT chords per cap — one across the
    // horizontal leg, one across the vertical leg.
    let (doc, tool) = plane(doc, [2.5, 0.0, 0.0], [1.0, 1.0, 0.0]);
    let (doc, split) = insert(doc, Node::Split { target: ext, tool });
    (doc, ext, split)
}

#[test]
fn l_shaped_extrude_cut_across_both_legs_names_its_chords_by_their_ends() {
    let (doc, _, split) = l_split(ProfileDoc::empty_derived("lib_g14", Tol::witness()));
    let ev = run(&doc);
    let t = table(&ev, split);
    // The wall was here: this document has no boolean at all, so the
    // refusal it used to raise had nothing to do with provenance.
    let mut by_parent: std::collections::BTreeMap<RoleSeg, Vec<&StableName>> =
        std::collections::BTreeMap::new();
    for (n, e) in t.iter() {
        let Some(head @ RoleSeg::SectionEdge { .. }) = n.path.first() else {
            continue;
        };
        assert!(matches!(e, Entry::Unique(_)), "a chord ties: {n:?} {e:?}");
        assert_eq!(n.kind, EntityKind::Edge);
        by_parent.entry(head.clone()).or_default().push(n);
    }
    assert!(!by_parent.is_empty(), "no section chords named at all");
    // Each cap, on each side, takes exactly two chords, each its own
    // name by its ends.
    let twice: Vec<_> = by_parent.values().filter(|ns| ns.len() > 1).collect();
    assert_eq!(twice.len(), 4, "two caps a side: {by_parent:#?}");
    for ns in twice {
        assert_eq!(ns.len(), 2, "a cap takes exactly two chords: {ns:?}");
        for n in ns {
            assert!(
                matches!(
                    n.path.as_slice(),
                    [_, RoleSeg::Fragment(editor_core::Qualifier::Ends(ends))] if ends.len() == 2
                ),
                "a chord of two is named by its ends: {n:?}"
            );
        }
    }
}

/// The chords are reachable through the SELECTOR layer one by one: each
/// is its own name, so a decided atom that tells the two cap chords
/// apart narrows to the ones it keeps rather than escalating over a
/// tie.
#[test]
fn the_chords_are_reachable_one_by_one_through_the_selector_layer() {
    let (doc, _, split) = l_split(ProfileDoc::empty_derived("lib_g14", Tol::witness()));
    let ev = run(&doc);
    let params = ParamEnv::default();
    // The chords of a cap crossed twice: `[SectionEdge, Fragment(Ends)]`.
    let sel = Selector::of(NamePat::of_kind(EntityKind::Edge).path([
        SegPat::tag(SegTag::SectionEdge),
        SegPat::tag(SegTag::Fragment),
    ]));
    let plain = select(&ev, split, &sel);
    assert_eq!(plain.len(), 8, "two chords a cap a side: {plain:?}");
    let t = table(&ev, split);
    assert!(
        plain.iter().all(|n| !t.is_tied(n)),
        "a chord name is tied: {plain:?}"
    );
    // Every chord of a polygonal prism cut by a plane is a line, so
    // the exact atom keeps the whole corpus.
    let lines = select_where(
        &ev,
        split,
        &sel,
        &[GeomPred::CurveKind(CurveKindSet::just(CurveKind::Line))],
        &params,
        Tol::witness(),
    )
    .expect("an exact atom never refuses");
    assert_eq!(lines, plain, "the exact atom dropped a chord");

    // A DECIDED atom that separates the two cap chords keeps some and
    // drops the others.
    let (doc, datum) = plane(doc, [1.25, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let ev = run(&doc);
    let far = select_where(
        &ev,
        split,
        &sel,
        &[GeomPred::DatumDistance {
            datum,
            cmp: Cmp::Greater,
            value: len(0.1),
        }],
        &params,
        Tol::witness(),
    )
    .expect("a separating atom over names that do not tie narrows");
    assert!(
        !far.is_empty() && far.len() < plain.len(),
        "the atom keeps some chords and drops others: {far:?} of {plain:?}"
    );
}

/// The retired refusal, from the outside: the L-split's chain names
/// end to end, so nothing in it reports a naming failure.
#[test]
fn the_l_split_chain_reports_no_naming_failure() {
    let (doc, ext, split) = l_split(ProfileDoc::empty_derived("lib_g14", Tol::witness()));
    let ev = run(&doc);
    for id in [ext, split] {
        assert!(
            ev.value(id).is_some(),
            "node {id:?} failed: {:?}",
            ev.nodes.get(&id)
        );
    }
}

// ---- #380: the emitter payload reaches the NODE-level prose. ----

/// `NamingError`'s own `Display` landed separately (#516); what this
/// row pins is the property G14 needed and did not have — that the
/// payload survives the `NodeErrorKind::Naming` boundary, which is the
/// ONLY route by which an emitter refusal reaches a human (Python's
/// typed exception message is exactly this prose). While it did not,
/// the two walls above read as one opaque "name emission failed" from
/// the audit's vantage point, and stayed conflated for a milestone.
#[test]
fn node_level_prose_carries_the_emitter_payload() {
    let carried = |e: NamingError| NodeErrorKind::Naming(e).to_string();

    let s = carried(NamingError::Emission {
        what: "section face classified On",
    });
    assert!(
        s.contains("section face classified On"),
        "the Emission payload is the diagnosis, and it must survive the \
         node boundary: {s}"
    );
    assert!(s.contains("name emission failed"), "category kept: {s}");

    let s = carried(NamingError::Unnamed {
        kind: EntityKind::Edge,
        body: 1,
    });
    assert!(s.contains("edge") && s.contains('1'), "{s}");

    let s = carried(NamingError::MissingUpstream {
        node: RecipeNodeId(tagged(7)),
    });
    assert!(s.contains('7'), "{s}");

    // Two DISTINCT emitter refusals must read differently at the node
    // level — the separability the audit needed to tell G14's two
    // walls apart without bisecting scenes.
    assert_ne!(
        carried(NamingError::Emission { what: "a" }),
        carried(NamingError::Emission { what: "b" })
    );
}

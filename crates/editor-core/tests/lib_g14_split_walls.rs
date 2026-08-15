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
//! **Wall A (A2)** — `RoleSeg::SectionEdge{side, face}` names a chord
//! only by the operand face it crosses, so a section line that enters
//! one face TWICE would mint one name twice. It refused, on scenes
//! with no boolean anywhere (a plain L-shaped single-loop extrude cut
//! across both legs). Those chords now become one N2 TIE.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    BooleanOp, CancelToken, CurveKind, CurveKindSet, Datum, EntityKind, Entry, EvalOptions,
    Evaluation, GeomPred, NamePat, NameTable, NamingError, Node, ParamEnv, ProfileDoc,
    RecipeNodeId, RoleSeg, SegPat, SegTag, Selector, SplitHalf, StableName, evaluate, select,
    select_where,
};

use fixture::{desc, insert, len, scl};

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default())
}

/// The node's table, or the node's failure — spelled out, because a
/// G14 regression IS a node failure and the message is the diagnosis.
fn table(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NameTable {
    &ev.value(id)
        .unwrap_or_else(|| panic!("node {id:?} has no value: {:?}", ev.nodes.get(&id)))
        .name_table
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
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![pts],
        )),
    );
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
    let (doc, _, sub) = u_cutter_tie(ProfileDoc::empty_derived("lib_g14"));
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
    let (doc, _, sub) = u_cutter_tie(ProfileDoc::empty_derived("lib_g14"));
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

// ---- Wall A (A2): two chords across one operand face TIE. ----

/// The survey's boolean-free repro. An L-shaped SINGLE-loop extrude
/// (no hole, no boolean anywhere in the document) cut by a vertical
/// plane that crosses both legs: each CAP face takes TWO section
/// chords, which `SectionEdge{side, face}` can only spell once.
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
fn l_shaped_extrude_cut_across_both_legs_names_with_tied_chords() {
    let (doc, _, split) = l_split(ProfileDoc::empty_derived("lib_g14"));
    let ev = run(&doc);
    let t = table(&ev, split);
    // The wall was here: this document has no boolean at all, so the
    // refusal it used to raise had nothing to do with provenance.
    let chords: Vec<(&StableName, &Entry)> = t
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::SectionEdge { .. })))
        .collect();
    assert!(!chords.is_empty(), "no section chords named at all");
    let tied: Vec<&StableName> = chords
        .iter()
        .filter(|(_, e)| matches!(e, Entry::Tied(_)))
        .map(|(n, _)| *n)
        .collect();
    assert!(
        !tied.is_empty(),
        "a cap face crossed twice should mint ONE tied SectionEdge, got: {:?}",
        chords.iter().map(|(n, _)| *n).collect::<Vec<_>>()
    );
    for n in &tied {
        let Some(Entry::Tied(c)) = t.lookup(n) else {
            panic!("tie lost")
        };
        assert_eq!(c.len(), 2, "a cap takes exactly two chords: {n:?}");
        assert_eq!(n.kind, EntityKind::Edge);
        // The tie is per (face, SIDE) — the side is in the name.
        assert!(matches!(
            n.path.first(),
            Some(RoleSeg::SectionEdge { side, .. })
                if *side == SplitHalf::Above || *side == SplitHalf::Below
        ));
    }
}

/// Reachability of the tied chords through the SELECTOR layer, as A2's
/// ratified text asks — MEASURED, not assumed.
///
/// What holds: the tied name is in the selectable corpus (`select`
/// returns it), and an exact geometric atom every candidate satisfies
/// keeps it (`select_where` with `curve_kind(Line)`), so the tie is a
/// first-class query result and not a hole.
///
/// What does NOT hold (REPORTED, LIB-G14 finding 3): `select_where`
/// cannot narrow a tie to ONE of its candidates. Its GS-Q4 rule is
/// all-or-nothing per NAME — mixed candidates are
/// `SelectRefusal::TiedDisagrees`, by design, because a filter that
/// half-selected a name would lie in one direction or the other.
/// Picking a specific chord needs a per-candidate narrowing door in
/// the SEL layer, which does not exist today.
#[test]
fn the_tied_chords_are_reachable_through_the_selector_layer() {
    let (doc, _, split) = l_split(ProfileDoc::empty_derived("lib_g14"));
    let ev = run(&doc);
    let params = ParamEnv::default();
    let sel =
        Selector::of(NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::SectionEdge)));
    let plain = select(&ev, split, &sel);
    assert!(!plain.is_empty(), "no SectionEdge name is selectable");
    let t = table(&ev, split);
    assert!(
        plain.iter().any(|n| t.is_tied(n)),
        "the tied chord name is not in the selectable corpus"
    );
    // Every chord of a polygonal prism cut by a plane is a line, so
    // the exact atom keeps the whole corpus — ties included, in one
    // piece.
    let lines = select_where(
        &ev,
        split,
        &sel,
        &[GeomPred::CurveKind(CurveKindSet::just(CurveKind::Line))],
        &params,
    )
    .expect("an exact atom never refuses");
    assert_eq!(lines, plain, "the exact atom dropped a chord");
}

/// The retired refusal, from the outside: the L-split's chain names
/// end to end, so nothing in it reports a naming failure.
#[test]
fn the_l_split_chain_reports_no_naming_failure() {
    let (doc, ext, split) = l_split(ProfileDoc::empty_derived("lib_g14"));
    let ev = run(&doc);
    for id in [ext, split] {
        assert!(
            ev.value(id).is_some(),
            "node {id:?} failed: {:?}",
            ev.nodes.get(&id)
        );
    }
}

// ---- #380: NamingError's Display carries the emitter's payload. ----

/// Before this unit `NamingError` had NO `Display` at all and
/// `NodeErrorKind::Naming` rendered the constant "name emission
/// failed", so Python — whose exception message IS this prose — could
/// not tell two emitter refusals apart. That opacity is exactly why
/// the two walls above stayed conflated in the audit for a milestone.
/// Every variant now names its subject.
#[test]
fn naming_errors_display_their_own_payload() {
    let e = NamingError::Emission {
        what: "section face classified On",
    };
    let s = e.to_string();
    assert!(
        s.contains("section face classified On"),
        "the Emission payload is the diagnosis: {s}"
    );
    assert!(s.contains("name emission failed"), "category kept: {s}");

    let u = NamingError::Unnamed {
        kind: EntityKind::Edge,
        body: 1,
    };
    let s = u.to_string();
    assert!(s.contains("Edge") && s.contains('1'), "{s}");

    let m = NamingError::MissingUpstream {
        node: RecipeNodeId(7),
    };
    assert!(m.to_string().contains('7'), "{}", m.to_string());

    // Two DISTINCT emitter refusals must read differently — the
    // property the audit needed and did not have.
    assert_ne!(
        NamingError::Emission { what: "a" }.to_string(),
        NamingError::Emission { what: "b" }.to_string()
    );
}

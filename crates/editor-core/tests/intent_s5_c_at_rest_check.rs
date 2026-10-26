//! **INTENT stage 5 C — the at-rest census is a check**
//! (`docs/INTENT-STAGE5-SPEC.md` §4, §6 rows 11–18; D10 `[assertions]`,
//! `[assertions.quiet]`).
//!
//! The census at each `⊔` decides each pair of arrivals apart, in
//! contact, overlapping or undecided, and nothing it finds refuses. A
//! contact is the `unproven-coincidence` lint's row, quiet under a
//! holding `Gap = 0` between its two cells; an overlap is an `AtRest`
//! interference finding; a pair the census could not look at is an
//! `AtRest` could-not-look finding, which refuses at no severity.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture::{ename, fname, in_world, insert, len, on_frame, place, square, vname, vpiece};

use editor_core::{
    AssertionRelation, AssertionVerdict, AtRestEvidence, CapEnd, CheckEvidence, CheckFinding,
    CheckId, ChecksConfig, Frame, MeasurePrimitive, Node, ProfileDoc, RecipeNodeId, RoleSeg,
    Severity, SitedRef, StableName, ValuePayload, enforce_checks,
};
use geom_core::{Tol, Turns};

/// An extrude of `loop_` on the profile's own xy, `height` up its z.
fn extrude(doc: ProfileDoc, loop_: Vec<(f64, f64)>, height: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![loop_],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    )
}

/// A block of half-side `h` and `height`, centred on its own xy.
fn block(doc: ProfileDoc, h: f64, height: f64) -> (ProfileDoc, RecipeNodeId) {
    extrude(doc, square(0.0, 0.0, h), height)
}

/// **A small block resting on a big one by values**, no mate: the big
/// block (half-side 0.5, height 1) at the identity, the small one (half
/// 0.3, height 0.5) `lift` above its top. `(doc, big, small, copy of
/// big, copy of small)`.
fn stacked(
    label: &str,
    lift: f64,
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, big) = block(doc, 0.5, 1.0);
    let (doc, small) = block(doc, 0.3, 0.5);
    let (doc, p) = place(doc, big);
    let (doc, q) = in_world(doc, small, &Frame::translation([0.0, 0.0, 1.0 + lift]));
    (doc, big, small, p, q)
}

/// The checks over `doc` at the default configuration.
fn checks(doc: &ProfileDoc) -> editor_core::ChecksReport {
    let run = corpus::eval::<f64>(doc);
    editor_core::run_checks(doc, &run, &ChecksConfig::default(), Tol::witness())
        .unwrap_or_else(|e| panic!("the checks run: {e}"))
}

/// `report`'s findings of `check`.
fn of(report: &editor_core::ChecksReport, check: CheckId) -> Vec<CheckFinding> {
    report
        .findings
        .iter()
        .filter(|f| f.check == check)
        .cloned()
        .collect()
}

/// The lint's at-rest rows of `relation`.
fn rows(report: &editor_core::ChecksReport, relation: topo::Relation) -> Vec<CheckFinding> {
    of(report, CheckId::UnprovenCoincidence)
        .into_iter()
        .filter(|f| {
            matches!(
                &f.evidence,
                CheckEvidence::UnprovenCoincidence { row, .. } if row.relation == relation
            ) && matches!(f.subject, editor_core::FindingSubject::Copies { .. })
        })
        .collect()
}

/// A `Gap(outer, inner)` related to `bound` metres by `relation`: the
/// document after them, and the assertion.
fn gap(
    doc: &ProfileDoc,
    outer: &SitedRef,
    inner: &SitedRef,
    relation: AssertionRelation,
    bound: f64,
) -> (ProfileDoc, RecipeNodeId) {
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
            bound: len(bound),
            relation,
        },
    )
}

/// An extrude's cap, read where its copy holds it.
fn cap(copy: RecipeNodeId, of: RecipeNodeId, end: CapEnd) -> SitedRef {
    SitedRef::new(copy, fname(of, RoleSeg::Cap(end)))
}

// ---- Row 11: a contact is one finding ----

/// **(Row 11) A contact is one finding.** Two bodies resting face to
/// face as operands of the product's `⊔`, placed by values, with no
/// mate: one `unproven-coincidence` finding and no `AtRest` finding.
///
/// *Breaks if* `AtRest` reports the contact (two findings for one row).
#[test]
fn a_contact_is_one_finding() {
    let (doc, ..) = stacked("s5c-contact", 0.0);
    let report = checks(&doc);
    assert_eq!(of(&report, CheckId::AtRest), Vec::new(), "{report}");
    let [rest] = rows(&report, topo::Relation::SameOpposite)
        .try_into()
        .unwrap_or_else(|found: Vec<_>| panic!("one rest, the lint's: {found:?}\n{report}"));
    assert!(rest.quiet.is_none(), "nothing asserted: {rest:?}");
    assert_eq!(
        of(&report, CheckId::UnprovenCoincidence).len(),
        1,
        "and no other row: {report}"
    );
}

// ---- Row 12: the contact half of the quieting rule ----

/// **(Row 12) A contact is quiet under a holding `Gap = 0` over its two
/// cells, and under nothing else.** `Gap(big.top, small.bottom) = 0`
/// over the copies quiets the lint's row, naming the assertion, in
/// either role order; `≥ 0` and `≤ 0` leave it loud; so does a `Gap = 0`
/// that holds over another cell pair of the same two copies (the big
/// block's top against the small block's bottom rim edge, which lies on
/// it).
///
/// *Breaks if* the rule matches by body pair (the other-cell `Gap`
/// quiets), `≥ 0` or `≤ 0` quiets (the straddle reading), or the rule
/// reads the role order.
#[test]
fn a_contact_is_quiet_under_its_gap_and_nothing_else() {
    let (doc, big, small, p, q) = stacked("s5c-contact-quiet", 0.0);
    let (top, bottom) = (cap(p, big, CapEnd::End), cap(q, small, CapEnd::Start));
    let rim = SitedRef::new(
        q,
        crate::fixture::rim_edge(
            small,
            CapEnd::Start,
            crate::fixture::piece(&doc, small, 0, 0),
        ),
    );
    for (outer, inner, relation, quiet, why) in [
        (
            &top,
            &bottom,
            AssertionRelation::Equal,
            true,
            "= 0 over the row's cells",
        ),
        (
            &bottom,
            &top,
            AssertionRelation::Equal,
            true,
            "either role order",
        ),
        (
            &top,
            &bottom,
            AssertionRelation::AtLeast,
            false,
            "≥ 0 admits clearance",
        ),
        (
            &top,
            &bottom,
            AssertionRelation::AtMost,
            false,
            "≤ 0 admits overlap",
        ),
        (
            &top,
            &rim,
            AssertionRelation::Equal,
            false,
            "another cell pair of the two copies",
        ),
    ] {
        let (doc, assertion) = gap(&doc, outer, inner, relation, 0.0);
        let run = corpus::eval::<f64>(&doc);
        assert!(
            matches!(
                run.value(assertion).map(|v| &v.payload),
                Some(ValuePayload::Assertion(AssertionVerdict::Holds { .. }))
            ),
            "{why}: the premise is an assertion that holds"
        );
        let report = checks(&doc);
        let [rest] = rows(&report, topo::Relation::SameOpposite)
            .try_into()
            .unwrap_or_else(|found: Vec<_>| panic!("{why}: one rest: {found:?}\n{report}"));
        assert_eq!(rest.quiet, quiet.then_some(assertion), "{why}: {}", report);
        if quiet {
            assert!(
                rest.to_string().contains("quiet:"),
                "a quiet finding is listed with what quiets it: {rest}"
            );
            assert!(
                enforce_checks(&report, &ChecksConfig::default()).is_ok(),
                "and refuses nothing"
            );
        }
    }
}

/// **(Row 12) A `Gap = 0` whose cells moved apart quiets nothing and
/// reports itself.** The small block lifted 0.01: the census finds no
/// contact, so there is no row, and the assertion is `Violated`.
#[test]
fn a_gap_whose_cells_parted_is_violated_and_there_is_no_row() {
    let (doc, big, small, p, q) = stacked("s5c-contact-parted", 0.01);
    let (doc, assertion) = gap(
        &doc,
        &cap(p, big, CapEnd::End),
        &cap(q, small, CapEnd::Start),
        AssertionRelation::Equal,
        0.0,
    );
    let run = corpus::eval::<f64>(&doc);
    assert!(
        matches!(
            run.value(assertion).map(|v| &v.payload),
            Some(ValuePayload::Assertion(AssertionVerdict::Violated { .. }))
        ),
        "the assertion reports its own failure"
    );
    let report = checks(&doc);
    assert_eq!(
        rows(&report, topo::Relation::SameOpposite),
        Vec::new(),
        "{report}"
    );
    assert_eq!(of(&report, CheckId::AtRest), Vec::new(), "{report}");
}

/// **(Row 12) A flush pair is never quieted by an assertion.** Two
/// blocks side by side, touching wall to wall: their walls rest, and
/// their caps lie flush. A holding `Gap = 0` over the two end caps
/// quiets nothing: a flush pair is no contact, and only a flag mate or
/// the replay establishes it.
///
/// *Breaks if* the flush pair quiets.
#[test]
fn a_flush_pair_stays_loud_under_a_gap() {
    let doc = ProfileDoc::empty_derived("s5c-flush", Tol::witness());
    let (doc, a) = block(doc, 0.5, 1.0);
    let (doc, b) = block(doc, 0.5, 1.0);
    let (doc, p) = place(doc, a);
    let (doc, q) = in_world(doc, b, &Frame::translation([1.0, 0.0, 0.0]));
    let (doc, assertion) = gap(
        &doc,
        &cap(p, a, CapEnd::End),
        &cap(q, b, CapEnd::End),
        AssertionRelation::Equal,
        0.0,
    );
    let run = corpus::eval::<f64>(&doc);
    assert!(
        matches!(
            run.value(assertion).map(|v| &v.payload),
            Some(ValuePayload::Assertion(AssertionVerdict::Holds { .. }))
        ),
        "the premise: the flush caps' gap holds at zero"
    );
    let report = checks(&doc);
    let flush = rows(&report, topo::Relation::SameOriented);
    assert!(!flush.is_empty(), "the caps lie flush: {report}");
    assert!(
        flush.iter().all(|f| f.quiet.is_none()),
        "no assertion quiets a flush pair: {flush:?}"
    );
}

/// Which of the prism's cells rests on the block.
#[derive(Clone, Copy, Debug)]
enum Rest {
    /// The apex edge lying along the block's top.
    Edge,
    /// The apex edge's end, a vertex, alone.
    Vertex,
}

/// **A triangular prism resting apex down on a block**, as
/// `gap_reads_vertex_and_edge_cells` builds it: `(doc, the block's top,
/// the prism's resting cell)`, each as its copy holds it.
fn resting(rest: Rest) -> (ProfileDoc, SitedRef, SitedRef) {
    let doc = ProfileDoc::empty_derived("s5c-cells", Tol::witness());
    let (doc, block) = extrude(
        doc,
        vec![(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)],
        1.0,
    );
    let (doc, prism) = extrude(doc, vec![(0.0, 0.0), (0.2, 0.3), (-0.2, 0.3)], 0.4);
    let (turns, at) = match rest {
        Rest::Edge => (Turns::new(1, 4), [0.0, 0.2, 1.0]),
        Rest::Vertex => (Turns::new(1, 3), [0.0, 0.3, 1.2]),
    };
    let frame = Frame::rotate_by_turns_then_translate(
        [1.0, 0.0, 0.0],
        turns.expect("a fraction of a turn"),
        at,
        crate::fixture::band(),
    )
    .expect("the x axis is a direction");
    let (doc, p) = place(doc, block);
    let (doc, q) = in_world(doc, prism, &frame);
    let apex = vpiece(&doc, prism, 0, 0);
    let cell: StableName = match rest {
        Rest::Edge => ename(prism, RoleSeg::LateralEdge(apex)),
        Rest::Vertex => vname(prism, RoleSeg::CapVertex(CapEnd::End, apex)),
    };
    let top = fname(block, RoleSeg::Cap(CapEnd::End));
    (doc, SitedRef::new(p, top), SitedRef::new(q, cell))
}

/// **(Row 12) The contact half reads a cell at whatever dimension it
/// is.** A vertex resting on a face and an edge lying on one are each
/// one lint row, quiet under a holding `Gap = 0` over the two cells and
/// loud under `Gap ≥ 0`.
///
/// *Breaks if* the rule reads face cells only.
#[test]
fn a_vertex_and_an_edge_on_a_face_are_quiet_under_their_gap() {
    for rest in [Rest::Vertex, Rest::Edge] {
        let (doc, top, cell) = resting(rest);
        let bare = checks(&doc);
        let unasserted: Vec<_> = of(&bare, CheckId::UnprovenCoincidence)
            .into_iter()
            .filter(|f| matches!(f.subject, editor_core::FindingSubject::Copies { .. }))
            .collect();
        assert!(
            !unasserted.is_empty() && unasserted.iter().all(|f| f.quiet.is_none()),
            "{rest:?}: the touch is the lint's, loud: {bare}"
        );
        assert_eq!(of(&bare, CheckId::AtRest), Vec::new(), "{rest:?}: {bare}");
        for (relation, quiet) in [
            (AssertionRelation::Equal, true),
            (AssertionRelation::AtLeast, false),
        ] {
            let (doc, assertion) = gap(&doc, &top, &cell, relation, 0.0);
            let report = checks(&doc);
            let at_rest: Vec<_> = of(&report, CheckId::UnprovenCoincidence)
                .into_iter()
                .filter(|f| matches!(f.subject, editor_core::FindingSubject::Copies { .. }))
                .collect();
            assert_eq!(at_rest.len(), unasserted.len(), "{rest:?}: {report}");
            let quieted = at_rest
                .iter()
                .filter(|f| f.quiet == Some(assertion))
                .count();
            assert_eq!(quieted > 0, quiet, "{rest:?} under {relation:?}: {report}");
        }
    }
}

// ---- Row 13: a could-not-look finding never refuses ----

/// **(Row 13) A could-not-look finding never refuses, a loud
/// interference refuses at `Error` only, and a quiet one at none.** A
/// clearance fit one box pad inside a bore is pairs the census leaves
/// undecided: could-not-look findings, reported at `Warn` and `Error`,
/// refused at neither. The slab overlap is one loud interference
/// finding, refused at `Error` alone; under its quieting assertion it
/// refuses at no severity.
///
/// *Breaks if* a could-not-look finding refuses at any severity, or a
/// quiet finding counts.
#[test]
fn a_could_not_look_finding_never_refuses() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let (fit, ..) =
        crate::intent_s5_b_interference::bore_and_pin(0.5 - topo::test_support::sweep_pad(band));
    let error = ChecksConfig {
        at_rest: Severity::Error,
        ..ChecksConfig::default()
    };
    for cfg in [ChecksConfig::default(), error.clone()] {
        let run = corpus::eval::<f64>(&fit);
        let report = editor_core::run_checks(&fit, &run, &cfg, Tol::witness()).expect("it runs");
        let blind = of(&report, CheckId::AtRest);
        assert!(
            !blind.is_empty()
                && blind.iter().all(|f| matches!(
                    &f.evidence,
                    CheckEvidence::AtRest {
                        evidence: AtRestEvidence::NoLane(
                            topo::ValidationError::CensusUndecidable { .. }
                        )
                    }
                )),
            "the census's undecided pairs, reported: {report}"
        );
        assert!(
            enforce_checks(&report, &cfg).is_ok(),
            "a could-not-look finding refuses at {:?}: {report}",
            cfg.at_rest
        );
    }
    let (slab, a, b, p, q) = crate::intent_s5_b_interference::slab_overlap();
    let loud = checks(&slab);
    assert!(
        enforce_checks(&loud, &ChecksConfig::default()).is_ok(),
        "Warn never refuses"
    );
    let refused = enforce_checks(&loud, &error).expect_err("a loud overlap refuses at Error");
    assert_eq!(refused.findings, of(&loud, CheckId::AtRest));
    let (quieted, assertion) = crate::intent_s5_b_interference::gap_at_copies(
        slab.clone(),
        (p, crate::intent_s5_b_interference::plus_x(&slab, a)),
        (q, crate::intent_s5_b_interference::minus_x(&slab, b)),
        AssertionRelation::AtMost,
        -0.01,
    );
    let quiet = checks(&quieted);
    let [finding] = of(&quiet, CheckId::AtRest)
        .try_into()
        .unwrap_or_else(|found: Vec<_>| panic!("one interference: {found:?}"));
    assert_eq!(finding.quiet, Some(assertion), "{quiet}");
    assert!(
        enforce_checks(&quiet, &error).is_ok(),
        "a quiet finding refuses at no severity"
    );
}

// ---- Row 14: `Separation` has no successor of its own ----

/// **(Row 14) Each fin of the heat sink is one interference finding**,
/// and nothing reports the boxes: the census's own pre-filter is the
/// pair filter. Two arrivals whose boxes overlap but which the census
/// clears — a block in the notch of an L, apart from it — have no
/// finding at all.
///
/// *Breaks if* a box test is kept as a verdict (the cleared pair is
/// reported).
#[test]
fn separation_has_no_successor_of_its_own() {
    let doc = ProfileDoc::empty_derived("s5c-notch", Tol::witness());
    let (doc, l) = extrude(
        doc,
        vec![
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ],
        1.0,
    );
    let (doc, notch) = extrude(doc, square(1.5, 1.5, 0.3), 1.0);
    let (doc, _) = place(doc, l);
    let (doc, _) = place(doc, notch);
    let notch = checks(&doc);
    assert!(
        notch.findings.is_empty(),
        "boxes that overlap are no finding where the census clears the pair: {notch}"
    );

    let heat_sink = corpus::documents()
        .into_iter()
        .find(|d| d.name == "heat_sink")
        .expect("the corpus carries the heat sink")
        .doc;
    let report = checks(&heat_sink);
    let interference = crate::fixture::interference(&report.findings);
    let run = corpus::eval::<f64>(&heat_sink);
    let product = editor_core::product_recorded(&heat_sink, &run, Tol::witness())
        .expect("the heat sink gathers");
    assert_eq!(
        interference.len(),
        product.copies.len() - 1,
        "one finding per fin, each against the base: {report}"
    );
    assert!(
        interference
            .iter()
            .all(editor_core::InterferenceFinding::is_loud)
    );
}

// ---- Row 15: per `⊔` ----

/// **(Row 15) Every `⊔` is checked, each against nothing outside it.**
/// Two overlapping copies assembled by a `⊔` the product slot does not
/// read report their interference under that `⊔`; the same two, one of
/// them in a space of its own, overlap only in coordinates and report
/// nothing.
///
/// *Breaks if* the resident reads only the slot's `⊔`, or compares
/// bodies across spaces.
#[test]
fn every_assemble_is_checked_against_nothing_outside_it() {
    let doc = ProfileDoc::empty_derived("s5c-unread", Tol::witness());
    let (doc, a) = block(doc, 0.5, 1.0);
    let (doc, b) = block(doc, 0.3, 0.5);
    let (doc, p) = place(doc, a);
    let (doc, q) = in_world(doc, b, &Frame::translation([0.6, 0.0, 0.25]));
    let members = [p, q]
        .iter()
        .map(|&copy| {
            editor_core::BodyRead::plain(editor_core::Operand::Var(
                doc.output(copy, 0).expect("a copy"),
            ))
        })
        .collect();
    let (doc, both) = insert(
        doc,
        Node::Assemble {
            members: editor_core::Bodies::Spelled(members),
        },
    );
    let (doc, far) = block(doc, 0.2, 0.2);
    let (doc, far) = in_world(doc, far, &Frame::translation([10.0, 0.0, 0.0]));
    let far = doc.output(far, 0).expect("a copy");
    let doc = crate::fixture::step(
        doc,
        editor_core::DocEdit::SetProduct {
            product: Some(editor_core::Operand::Var(far)),
        },
    )
    .0;
    let report = checks(&doc);
    let read = doc.output(both, 0).expect("the `⊔`'s body");
    let found: Vec<_> = of(&report, CheckId::AtRest)
        .into_iter()
        .filter(|f| f.subject == editor_core::FindingSubject::Census { read })
        .collect();
    assert_eq!(
        crate::fixture::interference(&found).len(),
        1,
        "the unread `⊔`'s overlap, under it: {report}"
    );

    // The same two, the smaller in a space of its own: they overlap only
    // in coordinates.
    let doc = ProfileDoc::empty_derived("s5c-spaces", Tol::witness());
    let (doc, a) = block(doc, 0.5, 1.0);
    let (doc, b) = block(doc, 0.3, 0.5);
    let (doc, _) = place(doc, a);
    let (doc, q) = in_world(doc, b, &Frame::translation([0.6, 0.0, 0.25]));
    let (apart, _) = crate::fixture::into_new_space(doc, q);
    let spaces = checks(&apart);
    assert_eq!(
        of(&spaces, CheckId::AtRest),
        Vec::new(),
        "two spaces' bodies are never compared: {spaces}"
    );
}

// ---- Row 16: `A ⊔ A` refuses nothing ----

/// **(Row 16) `A ⊔ A` gathers and refuses nothing.** One copy read twice
/// by a `⊔`: the same document gathers, the at-rest check reports what
/// the census found, and only `Error` refuses, and only a loud
/// interference.
///
/// *Breaks if* any severity but `Error` refuses it, or it reports a
/// contact.
#[test]
fn one_body_assembled_twice_refuses_nothing() {
    let doc = ProfileDoc::empty_derived("s5c-twice", Tol::witness());
    let (doc, a) = block(doc, 0.5, 1.0);
    let (doc, copy) = place(doc, a);
    let read = editor_core::BodyRead::plain(editor_core::Operand::Var(
        doc.output(copy, 0).expect("a copy"),
    ));
    let (doc, twice) = insert(
        doc,
        Node::Assemble {
            members: editor_core::Bodies::Spelled(vec![read.clone(), read]),
        },
    );
    let doc = crate::fixture::step(
        doc,
        editor_core::DocEdit::SetProduct {
            product: Some(editor_core::Operand::Node(twice)),
        },
    )
    .0;
    let run = corpus::eval::<f64>(&doc);
    editor_core::product_recorded(&doc, &run, Tol::witness()).expect("it gathers");
    let report = checks(&doc);
    let found = of(&report, CheckId::AtRest);
    // The census decides the two arrivals' coincident material
    // `SameSideInterference`: `A ⊔ A` is one interference finding
    // (`[coincidence.booleans]`).
    let interference = crate::fixture::interference(&found);
    let [overlap] = interference.as_slice() else {
        panic!("one interference finding: {report}")
    };
    assert_eq!(found.len(), 1, "and nothing else: {report}");
    assert!(
        overlap.is_loud()
            && overlap
                .evidence
                .iter()
                .any(|e| matches!(e, topo::ValidationError::SameSideInterference { .. })),
        "{overlap:?}"
    );
    assert!(
        rows(&report, topo::Relation::SameOpposite).is_empty(),
        "and it reports no contact: {report}"
    );
    assert!(
        enforce_checks(&report, &ChecksConfig::default()).is_ok(),
        "Warn refuses nothing"
    );
    let error = ChecksConfig {
        at_rest: Severity::Error,
        ..ChecksConfig::default()
    };
    assert!(enforce_checks(&report, &error).is_err(), "Error refuses it");
}

// ---- Row 17: carried, once ----

/// **(Row 17) A part's quiet interference is carried once, quiet by the
/// part's assertion.** The slab overlap, quieted by its `Gap ≤ −0.01`,
/// is a part instanced twice into a host: one carried finding, quiet.
///
/// *Breaks if* the finding is reported twice, or loses its quiet.
#[test]
fn a_parts_quiet_interference_is_carried_once() {
    let (slab, a, b, p, q) = crate::intent_s5_b_interference::slab_overlap();
    let (part, assertion) = crate::intent_s5_b_interference::gap_at_copies(
        slab.clone(),
        (p, crate::intent_s5_b_interference::plus_x(&slab, a)),
        (q, crate::intent_s5_b_interference::minus_x(&slab, b)),
        AssertionRelation::AtMost,
        -0.01,
    );
    let (host, ids, opts) = crate::intent_part_census_seam::instantiating("s5c-carried", &part, 2);
    let run = crate::fixture::run(&host, &opts);
    let findings = crate::fixture::at_rest(&host, &run).expect("the host's check runs");
    let carried: Vec<_> = findings
        .iter()
        .filter_map(|f| match &f.evidence {
            CheckEvidence::CarriedInterference { finding } => Some((f, finding)),
            _ => None,
        })
        .collect();
    let [(finding, row)] = carried.as_slice() else {
        panic!("one carried finding: {findings:?}")
    };
    assert_eq!(row.route.through, ids[0], "by the first instance");
    assert_eq!(
        (finding.quiet, row.finding.quiet),
        (Some(assertion), Some(assertion)),
        "quiet by the part's own assertion"
    );
    assert!(
        crate::fixture::interference(&findings).is_empty(),
        "and nothing of the host's own: {findings:?}"
    );
}

// ---- Row 18: not run at this scalar ----

/// **(Row 18) At `Dual64` the census does not run, and the at-rest
/// check says so: one `NotRun` finding.** The registry cannot be run at
/// a dual (it carries no chart-coherence lane), so the row reads the
/// resident's reading of the dual's gather directly.
///
/// *Breaks if* a dual's gather reads as a clean report.
#[test]
fn a_dual_runs_no_census_and_says_so() {
    let (slab, ..) = crate::intent_s5_b_interference::slab_overlap();
    let run = corpus::eval::<geom_core::Dual64>(&slab);
    let product = editor_core::product_recorded(&slab, &run, Tol::witness()).expect("it gathers");
    assert_eq!(
        editor_core::test_support::at_rest_could_not_look(&product, Tol::witness()),
        vec![AtRestEvidence::NotRun]
    );
}

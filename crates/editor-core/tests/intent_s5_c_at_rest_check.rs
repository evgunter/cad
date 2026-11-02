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
        Node::Assertion { claim: editor_core::Claim::Value {
            value,
            bound: len(bound),
            relation,
        } },
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

/// **(Row 12) A flush pair is quiet under a `Gap = 0` over its two
/// faces, and only that pair.** Two blocks side by side, touching wall
/// to wall: their walls rest, and their caps and side walls lie flush.
/// A holding `Gap = 0` over the two end caps, read on their sides, says
/// that flush pair (`[assertions]`), and every other flush pair stays
/// loud.
///
/// *Breaks if* the asserted flush pair stays loud, or the assertion
/// quiets a flush pair it is not over.
#[test]
fn a_flush_pair_is_quiet_under_its_gap() {
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
    let quiet: Vec<_> = flush.iter().filter(|f| f.quiet.is_some()).collect();
    assert_eq!(quiet.len(), 1, "the end caps' pair alone: {flush:?}");
    assert_eq!(quiet[0].quiet, Some(assertion), "{flush:?}");
    assert!(flush.len() > 1, "other flush pairs to stay loud: {flush:?}");
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
/// by a `⊔`: the same document gathers, and the at-rest check reports
/// what the census found, one loud interference no assertion can quiet,
/// which refuses at no severity, `Error` included (DS6).
///
/// *Breaks if* any severity refuses it, or it reports a contact.
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
    // No assertion can quiet it: both arrivals are one copy, so a
    // selection names a face of the first arrival and never of the
    // second, and no carrier pair of the two arrivals is one a signed
    // `Gap` can be written over. Outside what the document can say, it
    // refuses at no severity (DS6).
    assert!(!overlap.quietable, "{overlap:?}");
    assert!(
        enforce_checks(&report, &error).is_ok(),
        "and Error refuses nothing"
    );
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

/// **(Row 17) A host `Gap` over picks of an instance's pieces quiets
/// nothing of the part's.** The slab overlap, unasserted in the part, is
/// a part instanced once; the host asserts a holding `Gap ≤ −0.01`
/// between picks of the instance's two pieces, the slab's own faces.
/// The overlap is the part's, carried as the part judged it: loud. A
/// host's assertion speaks for the host's own arrivals, and the two
/// pieces are inside one of them.
///
/// What it pins, plainly: no code path today could quiet the carried
/// finding. A carried finding keeps the part's own `quiet`, and the
/// interference rule's selection lookup (`gap_assertions`' `face_of`)
/// resolves only names of the gathered product's own copies, which a
/// pick, a read of its own, never is. The row is a guard for whichever
/// of those changes first, not the witness of a decision.
///
/// *Breaks if* the host's assertion quiets the part's finding, or the
/// picks make it a finding of the host's.
#[test]
fn a_hosts_gap_over_picks_quiets_no_carried_finding() {
    use editor_core::Operand;
    let (slab, a, b, p, q) = crate::intent_s5_b_interference::slab_overlap();
    let part = crate::fixture::assemble_world_copies(slab.clone());
    let (p_var, q_var) = (
        part.output(p, 0).expect("a copy"),
        part.output(q, 0).expect("a copy"),
    );
    let (host, ids, opts) = crate::intent_part_census_seam::instantiating("s5c-picked", &slab, 1);
    let piece =
        |var, face| Operand::select(Operand::pick(Operand::Node(ids[0]), vec![var]), vec![face]);
    let (host, measure) = crate::fixture::measure_node(
        &host,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        vec![
            piece(p_var, crate::intent_s5_b_interference::plus_x(&slab, a)),
            piece(q_var, crate::intent_s5_b_interference::minus_x(&slab, b)),
        ],
    );
    let value = crate::fixture::value_of(&host, measure);
    let (host, assertion) = insert(
        host,
        Node::Assertion { claim: editor_core::Claim::Value {
            value,
            bound: len(-0.01),
            relation: AssertionRelation::AtMost,
        } },
    );
    let run = crate::fixture::run(&host, &opts);
    assert!(
        matches!(
            run.value(assertion).map(|v| &v.payload),
            Some(ValuePayload::Assertion(AssertionVerdict::Holds { .. }))
        ),
        "the premise: the host's Gap over the picks holds: {:?}",
        run.value(assertion).map(|v| &v.payload)
    );
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
    assert_eq!(
        (finding.quiet, row.finding.quiet),
        (None, None),
        "loud, as the part judged it"
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

// ---- An in-band pair is no overlap's evidence ----

/// **A pair read in band stands as its own finding, beside an overlap it
/// is no evidence of.** An L-shaped copy dips 0.05 into a block's +x
/// wall, and its arm stands three ε over the block's top, opposed, in
/// band: a `ThinInBand` pair (`topo::Undecidable::ThinInBand`), which
/// hides no overlap thicker than the band. The overlap is one
/// interference finding (unbounded here: intersecting the two copies
/// meets the same in-band arm and refuses), and the in-band pair is one
/// could-not-look finding of its own, none of the overlap's evidence.
///
/// *Breaks if* the in-band pair rides the interference as its evidence.
#[test]
fn an_in_band_pair_is_no_overlaps_evidence() {
    let report = checks(&in_band_l("s5c-thin-in-band"));
    let thin = |report: &editor_core::ChecksReport| {
        crate::fixture::could_not_look(&report.findings)
            .into_iter()
            .filter(|e| {
                matches!(
                    e,
                    AtRestEvidence::NoLane(topo::ValidationError::CensusUndecidable {
                        reason: topo::Undecidable::ThinInBand,
                        ..
                    })
                )
            })
            .count()
    };
    assert_eq!(
        thin(&report),
        1,
        "the arm over the top is one in-band pair, its own finding: {report}"
    );
    let interference = crate::fixture::interference(&report.findings);
    assert_eq!(interference.len(), 1, "one overlap: {report}");
    assert!(
        interference
            .iter()
            .all(|f| f.evidence.iter().all(|e| !matches!(
                e,
                topo::ValidationError::CensusUndecidable {
                    reason: topo::Undecidable::ThinInBand,
                    ..
                }
            ))),
        "and the in-band pair is none of its evidence: {interference:?}"
    );
}

/// A block and an L standing over its top 3 ε clear of it, the L's foot
/// sunk into the block's side: the census reads the arm over the top in
/// band, and the intersection of the two meets the same in-band arm and
/// refuses, so the foot's overlap is `Overlap::Unlocalized`.
fn in_band_l(label: &str) -> ProfileDoc {
    let g = 3.0 * Tol::witness().eps();
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, a) = block(doc, 0.5, 1.0);
    // The L drawn with its height on its own y and swept 0.6 along its
    // z, then turned a quarter turn about x, which stands its y up the
    // world's z and centres it on y = 0.
    let (doc, l) = extrude(
        doc,
        vec![
            (0.45, 0.25),
            (1.05, 0.25),
            (1.05, 1.2),
            (0.0, 1.2),
            (0.0, 1.0 + g),
            (0.45, 1.0 + g),
        ],
        0.6,
    );
    let stand = Frame::rotate_by_turns_then_translate(
        [1.0, 0.0, 0.0],
        Turns::new(1, 4).expect("a quarter turn"),
        [0.0, 0.3, 0.0],
        crate::fixture::band(),
    )
    .expect("the x axis is a direction");
    let (doc, _) = place(doc, a);
    in_world(doc, l, &stand).0
}

// ---- What no assertion could quiet refuses at no severity ----

/// `doc`'s one interference finding, and whether `enforce_checks`
/// refuses `doc` with the at-rest check at `Error`.
fn at_error(doc: &ProfileDoc) -> (editor_core::InterferenceFinding, bool) {
    let report = checks(doc);
    let [finding] = crate::fixture::interference(&report.findings)
        .try_into()
        .unwrap_or_else(|f: Vec<_>| panic!("one interference finding, not {}: {report}", f.len()));
    let error = ChecksConfig {
        at_rest: Severity::Error,
        ..ChecksConfig::default()
    };
    (finding, enforce_checks(&report, &error).is_err())
}

/// A big block, and a small one sunk into its top corner at `pose`.
fn sunk_corner(label: &str, pose: &Frame) -> ProfileDoc {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, big) = block(doc, 0.5, 1.0);
    let (doc, small) = block(doc, 0.2, 0.4);
    let (doc, _) = place(doc, big);
    in_world(doc, small, pose).0
}

/// **An overlap no assertion could quiet refuses at no severity** (spec
/// §4 "Severity and the refusing path"; CHECKS DS6). It is outside what
/// the document can yet say: an overlap the kernel could not intersect
/// (the in-band L's foot, `Overlap::Unlocalized`), and a bounded one
/// whose bounding faces are no carrier pair a signed `Gap` covers (a
/// block turned about a skew axis into another's corner: every face of
/// one is tilted against every face of the other, and `Gap` reads only
/// parallel planes). The same corner sunk square, which a `Gap` between
/// two parallel faces could quiet, refuses at `Error`.
///
/// *Breaks if* `enforce_checks` refuses an overlap no assertion could
/// quiet, or stops refusing one an assertion could.
#[test]
fn an_overlap_no_assertion_could_quiet_refuses_at_no_severity() {
    let (finding, refused) = at_error(&in_band_l("s5c-unlocalized"));
    assert!(
        matches!(finding.overlap, editor_core::Overlap::Unlocalized(_)) && !finding.quietable,
        "{finding:?}"
    );
    assert!(!refused, "an unlocalized overlap refused at Error");
    let skew = Frame::from_affine(geom_core::linalg::Affine3::from_parts(
        geom_core::linalg::Mat3::rotation_about(geom_core::linalg::Vec3::new(1.0, 2.0, 3.0), 0.7),
        geom_core::linalg::Vec3::new(0.5, 0.5, 0.85),
    ));
    let (finding, refused) = at_error(&sunk_corner("s5c-skew-corner", &skew));
    assert!(
        matches!(finding.overlap, editor_core::Overlap::Bounded { .. }) && !finding.quietable,
        "a skew corner is bounded and no Gap covers it: {finding:?}"
    );
    assert!(!refused, "an overlap no Gap covers refused at Error");
    let (finding, refused) = at_error(&sunk_corner(
        "s5c-square-corner",
        &Frame::translation([0.5, 0.5, 0.85]),
    ));
    assert!(finding.quietable && finding.is_loud(), "{finding:?}");
    assert!(refused, "a loud overlap a Gap covers refuses at Error");
}

// ---- One reading answers for a pair only where it is constant ----

/// **Two shafts side by side stay loud at every bearing** (the quieting
/// rule's opposed test, D10 `[assertions.quiet]`). A shaft of radius 1
/// and one of 0.3 stand 1.2 apart, clashing 0.1 deep, under a holding
/// `Gap(A, B) ≤ −0.01` (it reads −0.1). Neither wall is a socket for
/// the other, and two cylinders off one axis pair their materials
/// differently from point to point, so no reading at one point answers
/// for the pair: it is not opposed, and the finding is loud wherever B
/// stands about A.
///
/// No assertion the quieting rule accepts can quiet such a finding, so
/// it is not quietable and refuses at no severity, `Error` included: a
/// finding refuses only where an assertion could have quieted it.
///
/// *Breaks if* `opposed` reads such a pair at one point, which quieted
/// it at one bearing in eight, or the pair counts as quietable.
#[test]
fn shafts_side_by_side_stay_loud_at_every_bearing() {
    use editor_core::{Formula, LoopProgram, ProfileProgram};
    let rod = |doc: ProfileDoc, (cx, cy): (f64, f64), r: f64, h: f64| {
        let (doc, profile) = insert(
            doc,
            Node::Profile(ProfileProgram {
                loops: vec![LoopProgram::<Formula>::circle(cx, cy, r).expect("a circle")],
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
    };
    let lateral = |doc: &ProfileDoc, node| {
        fname(
            node,
            RoleSeg::Lateral(crate::fixture::piece(doc, node, 0, 0).into()),
        )
    };
    let mut quiet_at = Vec::new();
    let mut found = 0;
    for k in 0..8u32 {
        let bearing = f64::from(k) * std::f64::consts::FRAC_PI_4;
        let doc = ProfileDoc::empty_derived("s5c-shafts", Tol::witness());
        let (doc, a) = rod(doc, (0.0, 0.0), 1.0, 1.0);
        let (doc, b) = rod(doc, (1.2 * bearing.cos(), 1.2 * bearing.sin()), 0.3, 0.5);
        let (doc, p) = place(doc, a);
        let (doc, q) = in_world(doc, b, &Frame::translation([0.0, 0.0, 0.25]));
        let (fa, fb) = (lateral(&doc, a), lateral(&doc, b));
        let (doc, assertion) = gap(
            &doc,
            &SitedRef::new(p, fa),
            &SitedRef::new(q, fb),
            AssertionRelation::AtMost,
            -0.01,
        );
        let run = corpus::eval::<f64>(&doc);
        assert!(
            matches!(
                run.value(assertion).map(|v| &v.payload),
                Some(ValuePayload::Assertion(AssertionVerdict::Holds { .. }))
            ),
            "the premise at bearing {k}/8: the assertion holds"
        );
        let interference =
            crate::fixture::interference(&crate::fixture::at_rest(&doc, &run).expect("runs"));
        found += interference.len();
        if interference.iter().any(|f| !f.is_loud()) {
            quiet_at.push(k);
        }
        // `Gap` reads the two walls (parallel axes), but no reading
        // answers for their pairing, so no assertion the quieting rule
        // accepts exists, and nothing refuses at `Error`.
        assert!(
            interference.iter().all(|f| !f.quietable),
            "bearing {k}/8: not quietable: {interference:?}"
        );
        let error = ChecksConfig {
            at_rest: Severity::Error,
            ..ChecksConfig::default()
        };
        assert!(
            enforce_checks(&checks(&doc), &error).is_ok(),
            "bearing {k}/8: refused at Error"
        );
    }
    assert!(found > 0, "the premise: an overlap at some bearing");
    assert_eq!(
        quiet_at,
        Vec::<u32>::new(),
        "quiet at these eighths of a turn"
    );
}

// ---- With no world product, both residents read every `⊔` ----

/// **With no world product, an unread `⊔`'s contact is still the lint's,
/// and its overlap the at-rest check's.** The small block resting on the
/// big one, and sunk into it, with the world product cleared: the `⊔`
/// the fixture authored is a node no product reads. The at-rest check
/// gathers every `⊔` itself there (`Subject::EmptyProduct`), and the
/// lint reads the rows of the same gathers, so the two residents agree
/// on one document.
///
/// *Breaks if* the lint drops the contact the at-rest check's census
/// recorded.
#[test]
fn with_no_product_both_residents_read_every_assemble() {
    let unread = |label, z| {
        let doc = ProfileDoc::empty_derived(label, Tol::witness());
        let (doc, big) = block(doc, 0.5, 1.0);
        let (doc, small) = block(doc, 0.3, 0.5);
        let (doc, _) = place(doc, big);
        let (doc, _) = in_world(doc, small, &Frame::translation([0.0, 0.0, z]));
        let doc = crate::fixture::step(doc, editor_core::DocEdit::SetProduct { product: None }).0;
        assert_eq!(doc.product(), None, "the premise: no product");
        doc
    };
    let report = checks(&unread("s5c-unread-contact", 1.0));
    let contacts: Vec<_> = of(&report, CheckId::UnprovenCoincidence)
        .into_iter()
        .filter(|f| matches!(f.subject, editor_core::FindingSubject::Copies { .. }))
        .collect();
    assert_eq!(contacts.len(), 1, "one contact row, the lint's: {report}");
    let report = checks(&unread("s5c-unread-overlap", 0.75));
    assert_eq!(
        crate::fixture::interference(&report.findings).len(),
        1,
        "one interference, the at-rest check's: {report}"
    );
}

// ---- A pick of an arrival's piece names its cell ----

/// **A host `Gap = 0` over a pick of an arrival's piece quiets its
/// contact** (spec §4 rule 2: each selection reads the cell's arrival
/// "or a pick of that piece"). A copy of `bolt ⊔ plate` stands in the
/// world with a cap resting on the bolt's top: the world `⊔`'s row
/// names the bolt's top in the copy, keyed by the inner `⊔` too. A
/// `Gap = 0` between the pick of the bolt's top and the cap's bottom
/// quiets it; the same `Gap` over a pick of the plate names another
/// piece, and quiets nothing.
///
/// *Breaks if* a pick of the arrival's piece does not name the cell, or
/// a pick of another piece does.
#[test]
fn a_gap_over_a_pick_of_the_piece_quiets_its_contact() {
    use editor_core::{Bodies, Operand};
    let rested = |label, piece: fn(RecipeNodeId, RecipeNodeId) -> RecipeNodeId| {
        let doc = ProfileDoc::empty_derived(label, Tol::witness());
        let (doc, bolt) = block(doc, 0.2, 1.0);
        let (doc, plate) = extrude(doc, square(2.0, 0.0, 0.5), 1.0);
        let (doc, plate) = crate::fixture::slid(doc, bolt, plate, [0.0, 0.0, 0.0]);
        let (doc, seat) = insert(
            doc,
            Node::Assemble {
                members: Bodies::Spelled(vec![bolt.into(), plate.into()]),
            },
        );
        let (doc, copy) = place(doc, seat);
        let (doc, cap) = block(doc, 0.1, 0.5);
        let (doc, rest) = in_world(doc, cap, &Frame::translation([0.0, 0.0, 1.0]));
        let picked = piece(bolt, plate);
        let piece_var = doc.output(picked, 0).expect("a piece");
        let top = fname(picked, RoleSeg::Cap(CapEnd::End));
        let (doc, measure) = crate::fixture::measure_node(
            &doc,
            MeasurePrimitive::Gap { outer: 0, inner: 1 },
            vec![
                Operand::select(
                    Operand::pick(Operand::Node(copy), vec![piece_var]),
                    vec![top],
                ),
                Operand::select(
                    Operand::Node(rest),
                    vec![fname(cap, RoleSeg::Cap(CapEnd::Start))],
                ),
            ],
        );
        let value = crate::fixture::value_of(&doc, measure);
        let (doc, assertion) = insert(
            doc,
            Node::Assertion { claim: editor_core::Claim::Value {
                value,
                bound: len(0.0),
                relation: AssertionRelation::Equal,
            } },
        );
        let report = checks(&doc);
        let contacts: Vec<_> = of(&report, CheckId::UnprovenCoincidence)
            .into_iter()
            .filter(|f| matches!(f.subject, editor_core::FindingSubject::Copies { .. }))
            .collect();
        let [contact] = contacts.as_slice() else {
            panic!("one contact, the cap on the bolt: {report}")
        };
        (contact.quiet, assertion)
    };
    let (quiet, assertion) = rested("s5c-pick-bolt", |bolt, _| bolt);
    assert_eq!(
        quiet,
        Some(assertion),
        "the pick of the bolt names the cell"
    );
    let (quiet, _) = rested("s5c-pick-plate", |_, plate| plate);
    assert_eq!(quiet, None, "a pick of the plate names another piece");
}

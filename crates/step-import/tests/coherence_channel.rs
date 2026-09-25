//! **The import door's chart-coherence channel**
//! ([`ImportOptions::examine_chart_coherence`],
//! `StepImport::Solid::coherence`): the kernel's
//! `topo::examine_chart_coherence` asked about the body that ships.
//!
//! Four claims, one per row group.
//!
//! 1. **The channel is off by default, and off is not an empty
//!    report.** `None` says the caller did not ask — configuration,
//!    reversible by setting the option. A `Some` whose `unexamined` is
//!    non-empty says the examination RAN and the body put a loop out
//!    of its reach — data, which no option changes. Three states,
//!    three values, all three witnessed below on real fixtures.
//! 2. **The channel reports the door verbatim.** No filter, no
//!    reordering, no second vocabulary: the field equals the kernel
//!    door called directly on the shipped body at the same tolerance.
//!    This is the same no-opinion contract `lib.rs`'s `gate` states
//!    about the at-rest validator, and the reason the crate can carry
//!    no idea of coherence that could drift from the kernel's.
//! 3. **What the committed import corpora actually produce**, which
//!    is the measurement that priced this channel's default. The
//!    examination has no shape door, so a curved face whose outer loop
//!    carries a conic or spline trim carrier lands its loops in
//!    `unexamined` — the hazard being that a diagnostic firing on
//!    every analytic chart is worse than none. Measured across all
//!    three corpora at the run's ambient ε: **zero findings
//!    everywhere**, and `unexamined` on exactly ONE fixture. An
//!    ordinary cylinder, cone, sphere or torus reports nothing at all,
//!    in either list.
//! 4. **Nothing about acceptance moves.** The flag is read after the
//!    at-rest gate has already passed; the same file imports to the
//!    same disposition, the same census and the same certified
//!    enclosure with it on and off, and a file that refuses refuses
//!    identically.
//!
//! Every row rides the run's ambient ε, so the three-ε matrix asks
//! each question three times — which matters most for the half-cap
//! witness, whose gap is band-shaped by construction.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use crate::wild::{WILD_IMPORTS, wild};

use common::{
    FREECAD_FIXTURES, SOLID_FIXTURES, fixture, freecad_fixture, halfcap_fixture, own_import_options,
};
use geom_core::Tol;
use step_import::{ImportOptions, StepImport, StepImportError, import_step};
use topo::{Body, CoherenceCondition, CoherenceReport, Unexaminable};

/// Options asking for the examination; everything else default.
fn examining() -> ImportOptions {
    ImportOptions {
        examine_chart_coherence: true,
        ..ImportOptions::default()
    }
}

/// `(body, coherence)` of an own-corpus fixture imported with the
/// examination asked for.
fn own_examined(name: &str) -> (Body<f64>, CoherenceReport) {
    let text = fixture(name, "step");
    match import_step(&text, &own_import_options(name, true), Tol::witness()) {
        Ok(StepImport::Solid {
            body, coherence, ..
        }) => (
            body,
            coherence.expect("the examination was asked for, so the report is present"),
        ),
        Ok(StepImport::Wireframe { .. }) => panic!("{name} imported as a wireframe"),
        Err(e) => panic!("importing {name}: {e}"),
    }
}

/// One fixture's contribution to the corpus census.
struct Row {
    findings: usize,
    non_iso: usize,
    scaffold: usize,
    corrupt: usize,
}

fn census(report: &CoherenceReport) -> Row {
    let mut row = Row {
        findings: report.findings.len(),
        non_iso: 0,
        scaffold: 0,
        corrupt: 0,
    };
    for u in &report.unexamined {
        match u.why {
            Unexaminable::NonIsoCarrier { .. } => row.non_iso += 1,
            Unexaminable::NullScaffoldEdge { .. } => row.scaffold += 1,
            Unexaminable::Corrupt { .. } => row.corrupt += 1,
        }
    }
    row
}

// ---------------------------------------------------------------
// 1. Off by default, and off is not an empty report.
// ---------------------------------------------------------------

/// The default import asks nothing, and says so as `None` rather than
/// as a clean report.
///
/// `cube` is the subject because its examined report IS empty: if the
/// unasked state were spelled `CoherenceReport::default()` this row
/// could not tell the two apart, which is exactly the fold it exists
/// to forbid.
#[test]
fn the_channel_is_off_by_default_and_off_is_not_an_empty_report() {
    let text = fixture("cube", "step");
    let off = match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { coherence, .. }) => coherence,
        other => panic!("cube must import as a solid: {other:?}"),
    };
    assert!(
        off.is_none(),
        "the default import asked for no examination and reported one anyway: {off:?}"
    );
    // `cube` has nothing to report, which is the case where a folded
    // spelling would be invisible: `None` and this report must not be
    // one value. There is no `assert_ne!` against them here because
    // the fold is unrepresentable — no `Option<CoherenceReport>`
    // equals both — and the spelling that WOULD fold them, a bare
    // `CoherenceReport` field defaulting to empty when nobody asked,
    // does not compile at the line above. The runtime claim left to
    // make is that the examined-clean report really is clean, so a
    // reader of the two states is comparing the intended pair.
    let (_, on) = own_examined("cube");
    assert_eq!(
        (on.findings.len(), on.unexamined.len()),
        (0, 0),
        "cube was expected to report nothing, so this row is no longer \
         about the invisible case: {on:?}"
    );
}

/// The third state, on a real fixture: examined, and the body put
/// loops out of the door's reach. `cut_cylinder`'s curved face is
/// trimmed by an oblique plane, so its outer loop carries an ellipse
/// — not a chart iso curve, and none of the three conditions is about
/// it.
///
/// This is `unexamined` as DATA: no option, and no configuration of
/// any kind, makes those two loops examinable.
#[test]
fn a_trimmed_curved_face_is_unexamined_data_not_a_skipped_check() {
    let (_, report) = own_examined("cut_cylinder");
    let row = census(&report);
    assert_eq!(
        (row.findings, row.non_iso, row.scaffold, row.corrupt),
        (0, 2, 0, 0),
        "cut_cylinder's two trimmed loops are the door's reach boundary, and \
         nothing about them is a finding: {report:?}"
    );
}

// ---------------------------------------------------------------
// 2. The channel reports the door verbatim.
// ---------------------------------------------------------------

/// The field IS `topo::examine_chart_coherence` on the shipped body —
/// every finding, every unexamined loop, in the door's own order.
///
/// **What this kills**, stated as the mutations rather than as an
/// adjective: dropping `NonIsoCarrier` entries as "not interesting"
/// (`cut_cylinder` carries two and nothing else), dropping or
/// rewriting a finding (`halfcap.step` carries two), examining an
/// intermediate body instead of the one that ships, and examining at
/// a band other than the import's. Each of the three subjects has
/// something in one of the two lists, so no comparison is vacuous.
///
/// **What it does NOT kill, measured rather than assumed**: a
/// PERMUTATION of the findings keyed on their magnitude. No committed
/// fixture reports two findings with distinct `metres` — the two
/// `halfcap.step` reports at every band are one 1.6974e-2 m half-turn
/// read twice, once as `MeridianClosure` and once as
/// `MeridianContinuation` — so a magnitude sort is the identity on
/// every subject this crate has, and naming it as a falsifier would
/// be naming a mutant that survives. What the second row below does
/// kill is a permutation keyed on the CONDITION, by writing the
/// sequence out instead of reading it back from the door.
#[test]
fn the_channel_reports_the_kernel_door_verbatim() {
    let tol = Tol::witness();
    for name in ["cut_cylinder", "cube"] {
        let (body, reported) = own_examined(name);
        assert_eq!(
            reported,
            topo::examine_chart_coherence(&body, tol),
            "{name}: the channel's report is not the door's report on the body \
             it shipped"
        );
    }
    let text = halfcap_fixture("halfcap.step");
    let (body, reported) = match import_step(&text, &examining(), tol) {
        Ok(StepImport::Solid {
            body, coherence, ..
        }) => (body, coherence.unwrap()),
        other => panic!("halfcap must import as a solid: {other:?}"),
    };
    assert!(
        !reported.findings.is_empty(),
        "the half-cap's half-turn is 1.7e-2 m and is over every band this \
         matrix runs, so an empty list here means the channel is not reaching \
         the door"
    );
    assert_eq!(
        reported,
        topo::examine_chart_coherence(&body, tol),
        "the half-cap's findings are not the door's"
    );
}

/// The half-cap's two findings arrive in the door's order, written
/// out rather than read back — the one row here that does not consult
/// `examine_chart_coherence` to decide what it should have seen.
///
/// The row above compares the channel to the door, so a permutation
/// applied by the door itself is invisible to it and so is one the
/// channel applies if the same call is compared. This states the
/// sequence: the closure reading first, the continuation second. It
/// reds on any reordering, on either side, that moves the two
/// conditions past each other.
#[test]
fn the_half_cap_findings_arrive_in_the_doors_order() {
    let tol = Tol::witness();
    let text = halfcap_fixture("halfcap.step");
    let Ok(StepImport::Solid { coherence, .. }) = import_step(&text, &examining(), tol) else {
        panic!("halfcap must import as a solid at eps {:e}", tol.eps());
    };
    let report = coherence.expect("the examination was asked for");
    let sequence: Vec<&str> = report
        .findings
        .iter()
        .map(|f| match f.condition {
            CoherenceCondition::MeridianClosure { .. } => "closure",
            CoherenceCondition::RimContinuation { .. } => "rim",
            CoherenceCondition::MeridianContinuation { .. } => "meridian",
        })
        .collect();
    assert_eq!(
        sequence,
        ["closure", "meridian"],
        "the half-cap's finding sequence moved: {:?}",
        report.findings
    );
}

// ---------------------------------------------------------------
// 3. What the committed corpora produce.
// ---------------------------------------------------------------

/// **The measurement that priced this channel's default.** Every
/// fixture of the three committed import corpora — own-corpus (17),
/// FreeCAD (13), wild (9) — examined at the run's ambient ε.
///
/// The claim: **no fixture reports a finding**, and `unexamined` is
/// non-empty on exactly one, `cut_cylinder`, at two loops. In
/// particular the FreeCAD corpus's plain analytic charts — cylinder,
/// truncated cone, apex cone, sphere, torus, each a whole primitive
/// out of Open CASCADE — report nothing in either list, so
/// `NonIsoCarrier` is not a thing an ordinary cylinder produces: it is
/// what a curved face TRIMMED by a conic or spline produces.
///
/// **What this row cannot say**, inherited from
/// `mesh8_corpus_coherence`'s own disclosure and narrowed by the
/// corpora being imported rather than minted: it is a statement about
/// the files committed here, not about files a user has. The wild
/// corpus is nine translator outputs from four veins, which is the
/// broadest evidence in the tree and still not a population.
///
/// **A file that does not import contributes nothing, and the set of
/// those is PINNED rather than shrugged at.** `REFUSES_AT_FINEST` is
/// the whole of it: one wild fixture whose D7 adoption ladder cannot
/// certify an attachment at ε = 1e-12, which the tier-gate suite
/// already records as that band's disposition. Counting the examined
/// fixtures against the corpora minus exactly that set is what stops
/// a fixture silently vanishing from the census — a refusal that
/// spread to a second file would otherwise read as a quieter corpus.
///
/// The corpus-quiet rule this row applies is
/// `mesh/tests/mesh8_corpus_coherence.rs`'s, whose header states it
/// for the bodies this workspace MINTS; this is the same rule asked
/// of the bodies it RECEIVES, and that file is where the argument
/// lives.
#[test]
fn the_import_corpora_are_quiet_and_the_only_lane_boundary_is_the_trimmed_face() {
    let tol = Tol::witness();
    let mut noisy = Vec::new();
    let mut unexamined = Vec::new();
    let mut absent = Vec::new();
    let mut examined = 0_usize;

    let mut visit = |label: String, result: Result<StepImport, StepImportError>| {
        let Ok(StepImport::Solid { coherence, .. }) = result else {
            // No body shipped, so there is nothing to have examined.
            // Recorded, not passed over: the set of these is asserted
            // below.
            absent.push(label);
            return;
        };
        let report = coherence.expect("every call here asked for the examination");
        examined += 1;
        let row = census(&report);
        if row.findings > 0 {
            noisy.push(format!("{label}: {:?}", report.findings));
        }
        if row.non_iso + row.scaffold + row.corrupt > 0 {
            unexamined.push((label, row.non_iso + row.scaffold + row.corrupt, row.non_iso));
        }
    };

    for name in SOLID_FIXTURES {
        let text = fixture(name, "step");
        visit(
            format!("own/{name}"),
            import_step(&text, &own_import_options(name, true), tol),
        );
    }
    for name in FREECAD_FIXTURES {
        let text = freecad_fixture(name);
        visit(
            format!("freecad/{name}"),
            import_step(&text, &examining(), tol),
        );
    }
    for name in WILD_IMPORTS {
        let text = wild(name);
        visit(
            format!("wild/{name}"),
            import_step(&text, &examining(), tol),
        );
    }

    // The band's own disposition for `nist_ftc_09`: at 1e-12 the D7
    // ladder cannot certify an attachment on edge #2928 and the file
    // does not import, which `tests/tier_gate.rs` records as that
    // band's answer for it. At every other band it imports and is
    // examined like the rest.
    const REFUSES_AT_FINEST: &str = "wild/nist/nist_ftc_09_asme1_rd.stp";
    let expected_absent: Vec<String> = if tol.eps() <= 1.0e-12 {
        vec![REFUSES_AT_FINEST.to_owned()]
    } else {
        Vec::new()
    };
    assert_eq!(
        absent,
        expected_absent,
        "the set of corpus fixtures that ship no body at eps {:e} moved. A file \
         that stopped importing is a finding about that file or about the door \
         it refuses at, and this census cannot report on a body it never got",
        tol.eps()
    );
    assert_eq!(
        examined,
        SOLID_FIXTURES.len() + FREECAD_FIXTURES.len() + WILD_IMPORTS.len() - expected_absent.len(),
        "the census examined {examined} fixtures, which is not every fixture of \
         the three corpora less the ones pinned absent above"
    );
    assert!(
        noisy.is_empty(),
        "the examination reported a finding on a committed import fixture at eps \
         {:e}. That is a finding ABOUT THE FILE or about the condition, not a \
         threshold to widen: read its metres against the band and decide which.\n{}",
        tol.eps(),
        noisy.join("\n")
    );
    assert_eq!(
        unexamined,
        vec![("own/cut_cylinder".to_owned(), 2, 2)],
        "the set of fixtures with loops out of the door's reach moved. Each entry \
         is (fixture, unexamined loops, of which NonIsoCarrier); a new one is \
         either a new trimmed-curved-face fixture or the door's reach changing"
    );
}

// ---------------------------------------------------------------
// 4. The half-cap witness, band-shaped.
// ---------------------------------------------------------------

/// **Issue 723's recorded witness reaches the channel**, and it is
/// band-shaped, which is what makes it a measurement rather than a
/// flag.
///
/// The fixtures are one solid stated at four coordinate precisions;
/// the sphere face's meridian side is a pole-crossing arc, so its
/// carrier's midpoint sits a half-turn from its own endpoint and the
/// gap opens at the endpoint's distance from the axis. Each row
/// asserts the reported set against the metres the file states, which
/// is a different answer at each of the matrix's three ε.
///
/// **The COUNT is pinned, not just the emptiness.** Three of the four
/// files split that arc with an ordinary vertex, so the same half-turn
/// is read twice — once as `MeridianClosure` against the endpoint,
/// once as `MeridianContinuation` against the neighbouring edge — and
/// `halfcap_nosplit`, whose arc is one edge, has no column junction
/// and reports once. Those are the 2 / 1 / 2 / 2 the measurement
/// table states, and a row asserting only non-emptiness would let one
/// of the two readings disappear.
#[test]
fn the_half_cap_witness_reaches_the_channel_band_shaped() {
    let tol = Tol::witness();
    let eps = tol.eps();
    // (fixture, the meridian-closure gap the file states in metres,
    // how many findings that one gap is read as when it is over band)
    for (name, metres, count) in [
        ("halfcap.step", 1.697_409_754_832_974_3e-2, 2),
        ("halfcap_nosplit.step", 2.757_006_929_353_305_5e-2, 1),
        ("halfcap_eps6.step", 3.141_592_653_523_188_5e-8, 2),
        ("halfcap_eps7.step", 3.141_592_657_347_735e-9, 2),
    ] {
        let text = halfcap_fixture(name);
        let Ok(StepImport::Solid { coherence, .. }) = import_step(&text, &examining(), tol) else {
            panic!("{name} must import as a solid at eps {eps:e}");
        };
        let report = coherence.expect("the examination was asked for");
        let over_band = metres >= eps;
        assert_eq!(
            report.findings.len(),
            if over_band { count } else { 0 },
            "{name} states a {metres:e} m half-turn and the band is {eps:e}, so the \
             channel must report it exactly when it is over the band, and read it \
             {count} time(s) when it does: {report:?}"
        );
        for f in &report.findings {
            assert_eq!(
                f.metres, metres,
                "{name}: the reported length is not the one the file states"
            );
            assert_eq!(
                f.eps, eps,
                "{name}: a finding judged at a band that is not \
                 this run's"
            );
        }
        assert!(
            report.unexamined.is_empty(),
            "{name}: the half-cap's loops are all within the door's reach"
        );
    }
}

// ---------------------------------------------------------------
// 5. Not a gate.
// ---------------------------------------------------------------

/// **Asking the question changes no answer.** For every own-corpus
/// fixture, the import with the examination on and the import with it
/// off agree on the disposition, the arena census and the certified
/// enclosure, bit for bit — including the two fixtures that have
/// something to report.
///
/// The enclosure is the sharp field: it is the at-rest gate's own
/// certified `MassProperties`, so a channel that had crept into the
/// gate — reordering a certification, consuming a report, running at a
/// different band — would move it.
#[test]
fn asking_for_the_examination_changes_nothing_about_acceptance() {
    let tol = Tol::witness();
    for name in SOLID_FIXTURES {
        let text = fixture(name, "step");
        let one = |examine: bool| match import_step(&text, &own_import_options(name, examine), tol)
        {
            Ok(StepImport::Solid {
                body, enclosure, ..
            }) => {
                let enclosure =
                    enclosure.unwrap_or_else(|e| panic!("{name}: the enclosure measures: {e}"));
                (
                    common::arena_census(&body),
                    [
                        enclosure.volume,
                        enclosure.surface_area,
                        enclosure.volume_pad,
                        enclosure.area_pad,
                    ],
                )
            }
            other => panic!("{name} must import as a solid: {other:?}"),
        };
        assert_eq!(
            one(false),
            one(true),
            "{name}: the examination moved the import"
        );
    }
}

/// A file the adoption ladder refuses refuses identically with the
/// examination asked for: the flag is read after the gate, on a body
/// that exists, so a refusing file never reaches it.
#[test]
fn a_refusing_file_refuses_identically_with_the_examination_asked_for() {
    let tol = Tol::witness();
    let text = wild("stepcode/TAIL_TURBINE.stp");
    let off = import_step(&text, &ImportOptions::default(), tol);
    let on = import_step(&text, &examining(), tol);
    let (Err(off), Err(on)) = (off, on) else {
        panic!("TAIL_TURBINE's spline-carried edge must refuse in both calls");
    };
    assert_eq!(
        off.to_string(),
        on.to_string(),
        "the refusal changed when the examination was asked for"
    );
}

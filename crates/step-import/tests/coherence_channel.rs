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
use topo::construction::Construct;

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
    match import_step(&text, &own_import_options(true), Tol::witness()) {
        Ok(StepImport::Solid {
            body, coherence, ..
        }) => (
            body.into_body(),
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
/// (`cut_cylinder` carries two and nothing else), examining an
/// intermediate body instead of the one that ships, and examining at
/// a band other than the import's.
///
/// **What it does NOT kill, measured rather than assumed**: dropping,
/// rewriting or permuting a FINDING. The one committed fixture that
/// shipped findings was `halfcap.step`, whose meridian arc crossed the
/// pole; a chart singularity in a face is a vertex of it, so import now
/// splits that arc at the pole and the body ships quiet
/// (`chart_singularity_vertex.rs`). No file this door ships carries a
/// finding, so the channel's handling of one is read through no shipped
/// body (`work/foreign/the-coherence-channel-has-no-shipped-finding.md`).
/// The second row below still states the DOOR's order on the half-cap's
/// stated form, built by hand.
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
}

/// The half-cap's two findings arrive in the door's order, written
/// out rather than read back — the one row here that does not consult
/// `examine_chart_coherence` to decide what it should have seen.
///
/// The row above compares the channel to the door, so a permutation
/// applied by the door itself is invisible to it and so is one the
/// channel applies if the same call is compared. This states the
/// sequence: on this body, the continuation reading first and the
/// closure second. It reds on any reordering, on either side, that
/// moves the two conditions past each other.
///
/// The stated arc is rebuilt here by hand: import makes the pole a
/// vertex and ends with the join (`docs/DESIGN.md`, maximal edges), so
/// neither the pole-crossing arc nor `halfcap.step`'s ordinary vertex
/// ships, and a body holding them is construction state — which the
/// door, a pure function of the body, reads all the same.
#[test]
fn the_half_cap_findings_arrive_in_the_doors_order() {
    let tol = Tol::witness();
    let report = topo::examine_chart_coherence(&split_half_cap(tol), tol);
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
        ["meridian", "closure"],
        "the half-cap's finding sequence moved: {:?}",
        report.findings
    );
}

/// `halfcap_nosplit`'s solid as the file states it, its meridian arc
/// one edge through the pole, then split at an ordinary point halfway
/// from the arc's start to the pole: the split twin as the file states
/// it, before the import's join.
///
fn split_half_cap(tol: Tol) -> Body<f64> {
    let mut body = common::halfcap_with_its_arc_through_the_pole(tol);
    let (centre, axis) = body
        .faces()
        .find_map(|(_, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Sphere { center, axis, .. }) => Some((*center, *axis)),
            _ => None,
        })
        .expect("a sphere face");
    let (arc, at) = body
        .edges()
        .find_map(|(e, d)| {
            let c = body.get_curve_geom(d.curve)?.certified()?;
            let geom::Curve3::Circle { radius, .. } = c.carrier() else {
                return None;
            };
            let (t0, t1) = c.params();
            [1.0, -1.0].into_iter().find_map(|sign| {
                let pole = centre + axis * (sign * radius);
                let t = c.carrier().param_near(pole, 0.5 * (t0 + t1))?;
                let on = (c.carrier().eval(t) - pole).norm() < 1e-12 && t0 < t && t < t1;
                on.then_some((e, 0.5 * (t0 + t)))
            })
        })
        .expect("the meridian arc through the pole");
    body.split_edge(arc, at, tol)
        .expect("an ordinary point splits");
    body
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
            import_step(&text, &own_import_options(true), tol),
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
// 4. The half-cap witness, quiet.
// ---------------------------------------------------------------

/// **Issue 723's recorded witness ships quiet.** The fixtures are one
/// solid stated at four coordinate precisions, the sphere face's
/// meridian side a pole-crossing arc whose carrier's midpoint sat a
/// half-turn from its own endpoint: the channel's one finding-carrying
/// witness. A chart singularity in a face is a vertex of it, so import
/// splits that arc at the pole wherever it ships (`chart_singularity_vertex.rs`),
/// and the half-turn is gone: the report is empty, in both lists, at
/// every band.
///
/// Where it ships is band-shaped: the near-pole twins (Ev, PR 4251)
/// state a vertex 1e-8 m (eps6) and 1e-9 m (eps7) off the pole, which
/// at 1e-6 reads AT the pole, so the vertex is the pole's and the file
/// ships as stated; at the default ε lies in the band of the pole, so
/// the import refuses rather than guess; and at 1e-12 is off it, so the
/// arc is split at the pole and the ordinary vertex joined.
#[test]
fn the_half_cap_witness_ships_quiet_at_every_band() {
    let tol = Tol::witness();
    let default_band = (0.99e-9..=1.01e-9).contains(&tol.eps());
    for (name, ships) in [
        ("halfcap.step", true),
        ("halfcap_nosplit.step", true),
        ("halfcap_eps6.step", !default_band),
        ("halfcap_eps7.step", !default_band),
    ] {
        let imported = import_step(&halfcap_fixture(name), &examining(), tol);
        if !ships {
            assert!(
                matches!(&imported, Err(StepImportError::SingularityUndecided { .. })),
                "{name}: the pole in the band of a vertex refuses: {imported:?}"
            );
            continue;
        }
        let Ok(StepImport::Solid { coherence, .. }) = imported else {
            panic!("{name} must import as a solid at eps {:e}", tol.eps());
        };
        let report = coherence.expect("the examination was asked for");
        assert!(
            report.findings.is_empty() && report.unexamined.is_empty(),
            "{name}: the pole is a vertex, so the half-cap reports nothing: {report:?}"
        );
    }
}

// ---------------------------------------------------------------
// 5. Not a gate.
// ---------------------------------------------------------------

/// **Asking the question changes no answer.** On a planar fixture and
/// on one the examination has something to report about (the cut
/// cylinder's trimmed curved face), the import with the examination on
/// and the import with it off agree on the disposition, the arena
/// census and the certified enclosure, bit for bit.
///
/// The enclosure is the sharp field: it is the at-rest gate's own
/// certified `MassProperties`, so a channel that had crept into the
/// gate — reordering a certification, consuming a report, running at a
/// different band — would move it.
#[test]
fn asking_for_the_examination_changes_nothing_about_acceptance() {
    let tol = Tol::witness();
    for name in ["cube", "cut_cylinder"] {
        let text = fixture(name, "step");
        let one = |examine: bool| match import_step(&text, &own_import_options(examine), tol) {
            Ok(StepImport::Solid { body, .. }) => {
                let enclosure = body
                    .measure(topo::test_support::pinned_resolution(tol))
                    .unwrap_or_else(|e| panic!("{name}: the certificate measures: {e}"))
                    .met()
                    .expect("the pinned resolution is met");
                (
                    common::arena_census(body.body()),
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

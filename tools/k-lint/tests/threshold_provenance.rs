//! **The three thresholds #783 left unpinned, and what each one can
//! honestly be held to.**
//!
//! `BASELINE_FLOOR_MARGIN` has had an executable claim since M4:
//! `tests/litmus.rs` re-measures the #99 datum from the real kernel
//! predicates and asserts the floor still covers it. The crate's other
//! three constants had none, and they do not all support the same kind
//! of claim. Forcing one shape onto all three would put the weakest
//! argument on the strongest constant and the reverse, so each gets
//! the claim its provenance can carry:
//!
//! * [`EPS_COUPLED_FLOOR_RATIO`] is **derived from data**, so it is
//!   RE-DERIVED here: its population is re-cut from the committed M7
//!   sweep and the constant is re-checked against that population's
//!   P0 and the headroom its doc claims.
//! * [`AMBIENT_BAND_MIN`] is a **separator**, and what it can be held
//!   to is the SEPARATION, not the digit: the sweep's `band_zero`
//!   population is bimodal with an 88-decade gap, and every value in
//!   that gap classifies identically. So the interval is re-derived
//!   and the constant asserted to lie in it — which also says what the
//!   digit is: arbitrary within the gap, and therefore carrying no
//!   information that a re-measurement could move. Every run derives
//!   it from [`M7`]; the same derivation over all four committed eras
//!   is `#[ignore]`d and run on demand, for the reason [`ERAS`] gives.
//! * [`PROXIMITY_FACTOR`] is a **policy choice** (spec D3, "within
//!   10² of the band"). Nothing derives 1e2 and no test can pretend
//!   to; asserting it equals 1e2 would be the declaration written
//!   twice. What IS derivable is the CONSEQUENCE of moving it — the
//!   interval of factors that leave the lint's shipped behaviour
//!   unchanged — and both edges of that interval are computed here
//!   rather than written down.
//!
//! # Where the data comes from, and why a subprocess reads it
//!
//! `docs/k-report-data/`, whose own README states the rule this file
//! leans on: **rows are never re-cut or back-filled in place**, a new
//! distribution is a NEW file. So a committed era is immutable and
//! re-reading it is a re-derivation, not a baseline to preserve. When
//! a later era supersedes M7, the constants are re-cut against it and
//! [`M7`] moves — this test is where that happens, and its failure is
//! never an instruction to restore a number.
//!
//! The gzipped eras are read through `gzip -dc`, the same tool
//! `scripts/k_probe_sweep.sh --gzip` wrote them with. `include_str!`
//! is not available (the payload is deflate and this crate has no
//! dependencies), and an extracted copy committed beside this file
//! would be exactly the transcription the pins exist to remove. A
//! missing or unreadable `gzip` fails the test in the harness voice
//! rather than passing over an empty population.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use k_lint::{
    AMBIENT_BAND_MIN, BASELINE_FLOOR_MARGIN, EPS_COUPLED_FLOOR_RATIO, EPS_COUPLED_PREDICATES,
    PROXIMITY_FACTOR, Reason, lint_sample,
};

/// The era the shipped thresholds were cut from — the one
/// `docs/k-report-data/README.md` marks "current lint baseline".
const M7: &str = "m7-";

/// Every committed era, oldest first — M2, M4, M5, M7.
///
/// **Only [`M7`] is swept on every run, and this is why.** The gap
/// claim over the historical three cannot go red at any value of
/// [`AMBIENT_BAND_MIN`] that leaves M7 green, because M7 DOMINATES
/// them: M4 and M5 record the same two tie-break bands (`5e-324`,
/// `1e-100`) and the same ambient side, so their coupled assertions
/// are pointwise identical to M7's, and M2 records only the smaller
/// band, so its lower bound is strictly weaker. Nor can the eras
/// themselves move — `docs/k-report-data/README.md` forbids re-cutting
/// a committed file in place, a new distribution being a new file. So
/// the three historical arms answer to nothing but an edit to this
/// test, while costing 8.2 s of a 12.0 s suite on every `k-lint (gate)`
/// job.
///
/// They are kept, not deleted, because "the gap has never been
/// crossed" is a stronger statement than "the gap is not crossed
/// today" and it is worth having executable rather than transcribed
/// into prose. It runs on demand:
/// `cargo test --test threshold_provenance -- --ignored`.
const ERAS: [&str; 4] = ["", "m4-", "m5-", M7];

/// The ratified ε matrix. `band_escalate` is `K·ε` at the ratified
/// K = 10; both are re-read off the rows rather than assumed, and
/// these strings only name the files.
const ROWS: [&str; 3] = ["1e-6", "1e-9", "1e-12"];

/// One parsed sweep row, in the sweep's own column order. The text
/// columns are borrowed from the line: these files are ~1.8M rows
/// each and a `String` per column per row is most of the runtime.
struct Row<'a> {
    shape: &'a str,
    predicate: &'a str,
    margin: f64,
    band_zero: f64,
    band_escalate: f64,
    outcome: &'a str,
}

/// Streams one committed sweep file, calling `visit` per data row.
///
/// The M2 era is uncompressed and the rest are gzipped; the extension
/// is derived from the era rather than passed in, so a caller cannot
/// name a file this reader would silently skip.
fn for_each_row(era: &str, row: &str, mut visit: impl FnMut(Row<'_>)) {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/k-report-data/");
    let gz = !era.is_empty();
    let path = format!("{dir}{era}eps-{row}.csv{}", if gz { ".gz" } else { "" });
    let mut child = None;
    let reader: Box<dyn BufRead> = if gz {
        let mut c = Command::new("gzip")
            .args(["-dc", &path])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap_or_else(|e| panic!("HARNESS: cannot run `gzip -dc {path}`: {e}"));
        let out = c.stdout.take().expect("gzip stdout is piped");
        child = Some(c);
        Box::new(BufReader::new(out))
    } else {
        Box::new(BufReader::new(
            std::fs::File::open(&path).unwrap_or_else(|e| panic!("HARNESS: {path}: {e}")),
        ))
    };
    let mut seen = 0usize;
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_else(|e| panic!("HARNESS: {path} line {i}: {e}"));
        if i == 0 {
            assert_eq!(
                line, "shape,predicate,margin,band_zero,band_escalate,outcome",
                "HARNESS: {path} is not a sweep CSV"
            );
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let mut it = line.split(',');
        let (Some(shape), Some(pred), Some(m), Some(bz), Some(be), Some(out), None) = (
            it.next(),
            it.next(),
            it.next(),
            it.next(),
            it.next(),
            it.next(),
            it.next(),
        ) else {
            panic!("HARNESS: {path} line {}: {line}", i + 1);
        };
        let num = |s: &str| -> f64 {
            s.parse()
                .unwrap_or_else(|_| panic!("HARNESS: {path} line {}: {s}", i + 1))
        };
        seen += 1;
        visit(Row {
            shape,
            predicate: pred,
            margin: num(m),
            band_zero: num(bz),
            band_escalate: num(be),
            outcome: out,
        });
    }
    if let Some(mut c) = child {
        let st = c.wait().expect("gzip exits");
        assert!(st.success(), "HARNESS: `gzip -dc {path}` failed: {st}");
    }
    assert!(seen > 0, "HARNESS: {path} carried no data rows");
}

/// **[`EPS_COUPLED_FLOOR_RATIO`], re-derived.** Its doc calls it "the
/// P0 of the baseline's own `|m|/ε` population"; this re-cuts that
/// population from the committed M7 era and checks the constant
/// against it. Both halves of the doc's claim are asserted: the
/// population's SIZE (the "108 draws" the doc leans on when it argues
/// this P0 is weaker than its sibling's) and its MINIMUM.
///
/// The consequence assertion is the one that matters: the era the
/// constant was cut from must lint clean under rule (4). A constant
/// raised above P0 flags its own provenance.
#[test]
fn eps_coupled_floor_ratio_is_re_derivable_from_the_m7_population() {
    let mut draws = Vec::new();
    for row in ROWS {
        let (mut n, mut min, mut at) = (0usize, f64::INFINITY, String::new());
        for_each_row(M7, row, |r| {
            if !EPS_COUPLED_PREDICATES.contains(&r.predicate) {
                return;
            }
            // Rule (4)'s own statistic, in rule (4)'s own units: the
            // recorded headroom over the row's ε, which `lint_sample`
            // reads as `band_zero`.
            let ratio = r.margin.abs() / r.band_zero;
            n += 1;
            if ratio < min {
                min = ratio;
                at = r.shape.to_string();
            }
            // The whole family must be `positive`/`negative`: rule (4)
            // lives on the definite arm, so a `zero` or in-band member
            // would leave the population this constant is cut from
            // without any of them noticing.
            assert!(
                r.outcome == "positive" || r.outcome == "negative",
                "an eps-coupled sample at eps={row} is {} — rule (4) \
                 never sees it, so it is not in this population",
                r.outcome
            );
            // The consequence, in the same pass and in the lint's own
            // voice: not one row of the era this threshold was cut
            // from may fire rule (4).
            assert!(
                !lint_sample(
                    r.predicate,
                    r.margin,
                    r.band_zero,
                    r.band_escalate,
                    r.outcome
                )
                .contains(&Reason::BelowEpsCoupledFloor),
                "the M7 baseline flags its own eps-coupled floor at \
                 {}/{} (eps={row}, |m|/eps {ratio:e})",
                r.shape,
                r.predicate
            );
        });
        draws.push((row, n, min, at));
    }

    // The population, as the doc describes it: 12/36/60 draws, 108 in
    // total, its minimum at demo/tiltedcut on the 1e-9 row.
    let sizes: Vec<usize> = draws.iter().map(|d| d.1).collect();
    assert_eq!(
        sizes,
        vec![12, 36, 60],
        "the eps-coupled population moved: {sizes:?} draws per row \
         (the constant's doc argues from 12/36/60 = 108)"
    );
    let p0 = draws
        .iter()
        .min_by(|a, b| a.2.total_cmp(&b.2))
        .expect("three rows");
    assert_eq!(p0.0, "1e-9", "the population minimum changed row");
    assert_eq!(
        p0.3, "demo/tiltedcut",
        "the population minimum changed shape"
    );
    assert!(
        (164.674..164.675).contains(&p0.2),
        "the eps-coupled P0 moved: {:e}",
        p0.2
    );

    // The constant against that P0, and the headroom its doc claims.
    assert!(
        EPS_COUPLED_FLOOR_RATIO < p0.2,
        "EPS_COUPLED_FLOOR_RATIO {EPS_COUPLED_FLOOR_RATIO} is at or \
         above the population's P0 {:e} — the baseline flags itself",
        p0.2
    );
    let headroom = (p0.2 - EPS_COUPLED_FLOOR_RATIO) / p0.2;
    assert!(
        (0.088..0.090).contains(&headroom),
        "the doc claims 8.9% of headroom; re-derived it is {:.3}%",
        headroom * 100.0
    );
}

/// **[`AMBIENT_BAND_MIN`], re-derived as an interval.** Its doc says
/// the separator "sits in a gap between two things a sweep cannot
/// move" and that no re-measurement could shift it. Both halves are
/// checkable against the committed eras, and this checks them over
/// [`M7`], the era the shipped thresholds were cut from: the
/// `band_zero` population is bimodal, the gap is 88 decades wide, and
/// the constant is inside it. The historical eras are the `#[ignore]`d
/// twin below, for the reason [`ERAS`] gives.
///
/// What this establishes is the SEPARATION, which is the whole content
/// of the constant. Every factor between the exact bands' top and the
/// tightest supported ε classifies every recorded row identically, so
/// the digit 1e-13 is a choice within that gap and carries nothing a
/// re-measurement could move — which is the doc's claim, made
/// executable.
///
/// **What it does NOT cover, and the doc names it too**: the falsifier
/// is a kernel change putting a NEW exact band above the constant.
/// These files are past snapshots, so a future band construction is
/// invisible here — the residue stays exactly where the doc leaves it,
/// with the recorder.
#[test]
fn ambient_band_min_lies_in_the_gap_the_current_era_shows() {
    the_gap_holds_across(&[M7]);
}

/// The same claim over **every** committed era — the archaeological
/// half, *"the gap has never been crossed"*.
///
/// `#[ignore]`d rather than deleted or transcribed, for the reason
/// [`ERAS`] gives: M7 dominates the historical three, so nothing this
/// can red is not already red above, and the eras are frozen by
/// `docs/k-report-data/README.md`'s never-re-cut-in-place rule, so
/// they cannot move under it either. It is still CODE, so a re-derivation
/// is one command away and the compiler keeps it in step with the
/// crate. Run it when a new era lands, or when the dominance argument
/// in [`ERAS`] stops obviously holding:
/// `cargo test --test threshold_provenance -- --ignored`.
#[test]
#[ignore = "M7 dominates the historical eras and they are frozen; see ERAS"]
fn ambient_band_min_lies_in_the_gap_every_committed_era_shows() {
    the_gap_holds_across(&ERAS);
}

fn the_gap_holds_across(eras: &[&str]) {
    for era in eras.iter().copied() {
        for row in ROWS {
            let mut exact = Vec::new();
            let mut ambient = Vec::new();
            for_each_row(era, row, |r| {
                // Distinct values only: the population is three
                // numbers over ~1.8M rows, and the interesting fact is
                // the SET, not the counts.
                let side = if r.band_zero < AMBIENT_BAND_MIN {
                    &mut exact
                } else {
                    &mut ambient
                };
                if !side.contains(&r.band_zero) {
                    side.push(r.band_zero);
                }
            });
            exact.sort_by(f64::total_cmp);
            ambient.sort_by(f64::total_cmp);

            // The ambient side is exactly the row's own ε — one value,
            // the ratified matrix's, and nothing else answers to the
            // ratio rules.
            let eps: f64 = row.parse().expect("the row names its eps");
            assert_eq!(
                ambient,
                vec![eps],
                "era {era:?} row {row}: the ambient side is not exactly \
                 the ratified eps"
            );
            // The exact side is the RECORDER's tie-break bands. M2
            // predates the second one; from M4 on there are two.
            let expected: &[f64] = if era.is_empty() {
                &[5e-324]
            } else {
                &[5e-324, 1e-100]
            };
            assert_eq!(
                exact, expected,
                "era {era:?} row {row}: the exact tie-break bands the \
                 doc names are not what the sweep recorded"
            );

            // The gap, re-derived: nothing sits between the top exact
            // band and the row's eps, and the constant is inside it.
            let top_exact = *exact.last().expect("at least one exact band");
            assert!(
                top_exact < AMBIENT_BAND_MIN && AMBIENT_BAND_MIN <= eps,
                "era {era:?} row {row}: AMBIENT_BAND_MIN {AMBIENT_BAND_MIN:e} \
                 is not between the top exact band {top_exact:e} and eps \
                 {eps:e}"
            );
            let decades = eps.log10() - top_exact.log10();
            assert!(
                decades >= 88.0,
                "era {era:?} row {row}: the gap narrowed to {decades:.0} \
                 decades — the separator is becoming a threshold"
            );

            // The separation is what the constant is FOR: any factor in
            // the gap classifies the whole population the same way.
            // Two witnesses, one at each end of the gap, agreeing with
            // the shipped constant on every recorded band.
            for alt in [top_exact * 1e4, eps / 1e4] {
                for b in exact.iter().chain(ambient.iter()) {
                    assert_eq!(
                        *b >= AMBIENT_BAND_MIN,
                        *b >= alt,
                        "era {era:?} row {row}: band {b:e} classifies \
                         differently at {alt:e} — the digit is doing \
                         work the gap does not justify"
                    );
                }
            }
        }
    }
}

/// **[`PROXIMITY_FACTOR`], pinned by consequence.** Nothing derives
/// 1e2: it is spec D3's policy choice, and a test asserting the digit
/// would be the declaration written twice. What a test CAN say is what
/// the choice buys and what moving it would cost, and this derives
/// both edges of the interval it may move in without changing what the
/// lint does to the committed corpus.
///
/// **The lower edge is structural**, from rule (2)'s discrimination
/// floor (module docs): the cap must bind at the loosest supported row
/// and nowhere else. **The upper edge is MEASURED**, and it is the
/// half nothing said before: rule (2)-below flags a zero-classified
/// margin above `band_zero / PROXIMITY_FACTOR`, and the M7 corpus's
/// own largest zero-side ratio puts a ceiling on the factor at the
/// tightest row. That ceiling is the BINDING one of the two edges —
/// asserted as a comparison against the structural edge rather than as
/// a number — and the room it leaves is bounded above by the doc's own
/// "under 2x", so a corpus that moves toward it says so. It is not a
/// ranking against this crate's other constants: their headrooms are
/// fractions below a measured datum and this one is a multiple of a
/// policy digit.
///
/// The definite arm is deliberately not part of this: post-cap,
/// `proximity_above_threshold` never exceeds [`BASELINE_FLOOR_MARGIN`],
/// so `NearBandAbove` is a strictly stronger REASON on a sample rule
/// (3) already flags and never an extra flag. `tests/review_probes.rs`
/// carries that half.
#[test]
fn proximity_factor_is_a_policy_choice_and_this_is_what_moving_it_costs() {
    // One pass over M7 per row, collecting everything the three claims
    // below need: the row's escalation threshold (read off the rows,
    // not assumed from K), the corpus's largest zero-side ratio, and
    // the lint's own verdict on every zero-classified sample.
    let mut escalate = Vec::new();
    let (mut worst, mut at) = (0.0f64, String::new());
    for row in ROWS {
        let mut seen: Option<f64> = None;
        for_each_row(M7, row, |r| {
            if r.band_zero < AMBIENT_BAND_MIN {
                return;
            }
            match seen {
                None => seen = Some(r.band_escalate),
                Some(e) => assert!(
                    e == r.band_escalate,
                    "eps={row} records two escalation thresholds"
                ),
            }
            if r.outcome != "zero" {
                return;
            }
            let ratio = r.margin.abs() / r.band_zero;
            if ratio > worst {
                worst = ratio;
                at = format!("{}/{} at eps={row}", r.shape, r.predicate);
            }
            // The consequence, in the lint's own voice: not one
            // zero-classified row of the era the thresholds were cut
            // from may fire rule (2)-below.
            assert!(
                !lint_sample(
                    r.predicate,
                    r.margin,
                    r.band_zero,
                    r.band_escalate,
                    r.outcome
                )
                .contains(&Reason::NearBandBelow),
                "M7 flags its own zero side at {}/{} (eps={row})",
                r.shape,
                r.predicate
            );
        });
        escalate.push(seen.expect("every era carries ambient rows"));
    }

    // The shipped arrangement: the cap binds at the loosest row and
    // nowhere else, which is what makes rule (2) the stronger
    // statement at 1e-9 and 1e-12 and a tautology at 1e-6.
    let capped = |factor: f64| -> Vec<bool> {
        escalate
            .iter()
            .map(|e| factor * e >= BASELINE_FLOOR_MARGIN)
            .collect()
    };
    assert_eq!(
        capped(PROXIMITY_FACTOR),
        vec![true, false, false],
        "the rule-2 cap no longer binds at exactly the loosest \
         supported row — the module docs' discrimination-floor \
         argument is about a different arrangement than the one \
         shipping"
    );

    // The structural edges of that arrangement, SOLVED from the rows:
    // the cap binds at a row exactly when factor ≥ floor/escalate.
    let lower = BASELINE_FLOOR_MARGIN / escalate[0];
    let upper = BASELINE_FLOOR_MARGIN / escalate[1];
    assert_eq!(capped(lower), vec![true, false, false], "lower edge");
    assert_eq!(
        capped(lower * (1.0 - 1e-12)),
        vec![false, false, false],
        "just under the lower edge the cap stops binding anywhere"
    );
    assert_eq!(
        capped(upper),
        vec![true, true, false],
        "at the upper edge the cap reaches the middle row"
    );
    assert!(
        (lower..upper).contains(&PROXIMITY_FACTOR),
        "PROXIMITY_FACTOR {PROXIMITY_FACTOR:e} left the structural \
         interval [{lower:e}, {upper:e})"
    );

    // The measured ceiling, from the corpus's own zero-classified
    // population: rule (2)-below fires when |m|/band_zero > 1/factor,
    // so the largest recorded ratio is where the baseline starts
    // flagging itself.
    let ceiling = 1.0 / worst;
    assert!(
        PROXIMITY_FACTOR < ceiling,
        "PROXIMITY_FACTOR {PROXIMITY_FACTOR:e} is at or above the \
         corpus's own zero-side ceiling {ceiling:e} ({at}) — the M7 \
         baseline flags itself under rule (2)-below"
    );
    // The ceiling is the BINDING edge, and by a wide margin: the
    // structural interval reaches 4e3 and the corpus stops the factor
    // an order of magnitude earlier. Said as a comparison rather than
    // a number so a moved corpus moves the claim.
    assert!(
        ceiling < upper,
        "the corpus no longer binds PROXIMITY_FACTOR before the \
         structural edge does ({ceiling:e} vs {upper:e}) — the upper \
         half of this pin has gone slack and wants re-stating"
    );
    // How much room is actually left. The UPPER edge here is the
    // constant's doc sentence — "under 2x above the shipped factor" —
    // made load-bearing rather than approximated: a corpus drifting to
    // 2.2× must red here, not leave the doc quietly false. The lower
    // edge is not a sentence anywhere; it is an alarm that the corpus
    // loosened enough to want re-reading, and it is deliberately the
    // looser half.
    let room = ceiling / PROXIMITY_FACTOR;
    assert!(
        (1.5..2.0).contains(&room),
        "the corpus's zero-side headroom over PROXIMITY_FACTOR moved \
         to {room:.3}× ({at}); the shipped factor was cut with ~1.88×, \
         and PROXIMITY_FACTOR's doc says \"under 2x above the shipped \
         factor\""
    );
}

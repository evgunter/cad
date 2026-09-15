//! M4 PR 8b spec D3 — the K-telemetry Probe run over the Band 4
//! corpus (the harness the K-REPORT M3 addendum recorded as missing).
//!
//! Mechanics are the M2 report's, unchanged (docs/K-REPORT.md
//! "Collection method for the future run"): every corpus document is
//! evaluated end-to-end at the recording scalar `Probe` — decisions
//! bit-identical to f64, every `k_stats::decide` classification
//! recorded — then the result body runs the same tier-1/closed
//! validation and mass-properties pass the corpus rows run, and every
//! `MarginSample` dumps as CSV. One process per ε (`Tolerance` is a
//! OnceLock):
//!
//! ```sh
//! CAD_TOLERANCE_EPS=1e-9 CAD_K_REPORT_OUT=/tmp/corpus-eps-1e-9.csv \
//!   cargo test -p editor-core --test m4_pr8_k_probe -- --ignored --nocapture
//! ```
//!
//! Without `CAD_K_REPORT_OUT` the CSV goes to stdout. Columns are the
//! M2 file convention: `shape,predicate,margin,band_zero,
//! band_escalate,outcome`, with shapes namespaced `corpus/<doc>` so a
//! merged corpus+demos CSV stays attributable (the demo sweep — the
//! tour binary's `k-probe` mode — namespaces `demo/<scene>`).
//! `scripts/k_probe_sweep.sh` runs both and merges; the committed
//! baseline lives in `docs/k-report-data/m7-eps-<ε>.csv.gz`
//! (the m4-/m5- rows stay committed as the historical record).
//!
//! Every test here is Probe-lane by construction — the dump run, the
//! standing greenness pin, and the `Probe`-vs-`f64` differential that
//! turns "decisions bit-identical to f64" from a claim into a
//! comparison — so the whole file is gated on the `probe` feature and
//! nothing non-Probe is lost from the default build.

#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture::value_channel::{Digest, ValueChannelBits, value_digest_nodes};

use std::io::Write as _;

use geom_core::k_stats::{self, MarginSample, Probe, SampleOutcome};
use topo::{mass_properties, validate, validate_closed};

use corpus::{body_of, documents, eval, failures};
use editor_core::NodeResult;
use geom_core::Tol;

fn outcome_str(o: SampleOutcome) -> &'static str {
    // The ONE spelling of the sweep's outcome vocabulary lives
    // on the enum, because `tools/k-lint` has to read what this
    // writes and a hand-kept copy on each side of that boundary
    // silently disarmed the E6 driver gate once already.
    o.token()
}

/// One document's Probe sweep: evaluation (sequential — the sample
/// sink is thread-local) plus the corpus rows' body pass.
fn run_doc(d: &corpus::CorpusDoc) -> Vec<MarginSample> {
    k_stats::start_recording();
    let ev = eval::<Probe>(&d.doc);
    let bad = failures(&ev);
    assert!(
        bad.is_empty(),
        "{}: corpus document must evaluate green at Probe. This row is \
         one-sided — the differential beside it is what compares Probe \
         against f64 — so a red here is either a Probe-lane divergence \
         or an f64-lane break, and the f64 corpus rows say which:\n{}",
        d.name,
        bad.join("\n")
    );
    if let Some(result) = d.result {
        let body = body_of(&ev, result);
        validate(body).expect("tier 1");
        validate_closed(body).expect("closed");
        mass_properties(body, Tol::witness()).expect("mass properties");
    }
    k_stats::take_samples()
}

/// The dump entry point (ignored: run explicitly, one process per ε).
#[test]
#[ignore = "K-telemetry collection run; one process per eps (see module docs)"]
fn dump_corpus_k_samples() {
    let eps = Tol::witness().get().eps;
    let mut csv = String::from("shape,predicate,margin,band_zero,band_escalate,outcome\n");
    let mut total = 0usize;
    let mut unnamed = 0usize;
    for d in documents() {
        let samples = run_doc(&d);
        total += samples.len();
        for s in &samples {
            if s.predicate == "<unnamed>" {
                unnamed += 1;
            }
            csv.push_str(&format!(
                "corpus/{},{},{:e},{:e},{:e},{}\n",
                d.name,
                s.predicate,
                s.margin,
                s.band_zero,
                s.band_escalate,
                outcome_str(s.outcome)
            ));
        }
    }
    assert_eq!(
        unnamed, 0,
        "<unnamed> must be unreachable from shipped decide paths"
    );
    eprintln!("k_probe(corpus): eps={eps:e}, {total} samples");
    match std::env::var("CAD_K_REPORT_OUT") {
        Ok(path) => {
            let mut f = std::fs::File::create(&path).expect("create CAD_K_REPORT_OUT");
            f.write_all(csv.as_bytes()).expect("write csv");
            eprintln!("k_probe(corpus): wrote {path}");
        }
        Err(_) => print!("{csv}"),
    }
}

/// The whole corpus evaluates green at `Probe` — one-sided, and that is
/// the whole of the assertion.
///
/// The property it reaches for is that the recording scalar is a
/// WRAPPER and not a second arithmetic. **Nothing here compares the
/// two**: greenness at `Probe` is evidence for that claim, not a check
/// of it, and greenness is tolerance-dependent, so this says what it
/// says at one ε. The check itself is
/// [`probe_agrees_with_f64_bit_for_bit_over_the_corpus`] below, which
/// compares the two evaluations' bits at every ε row the dump is swept
/// at; this row stays because it is the one of the two the DEFAULT
/// selection runs, and because a one-sided greenness failure names its
/// document more directly than a differential does.
///
/// Runs under the DEFAULT selection, which `scripts/k_probe_sweep.sh`
/// invokes once per rostered suite. `run_doc` inside the `#[ignore]`d
/// dump beside it asserts the same predicate over the same documents at
/// all three ε, so what this row adds is that its body executes at all
/// — not the property.
#[test]
fn corpus_evaluates_green_at_probe() {
    for d in documents() {
        let ev = eval::<Probe>(&d.doc);
        let bad = failures(&ev);
        assert!(
            bad.is_empty(),
            "{}: not green at Probe:\n{}",
            d.name,
            bad.join("\n")
        );
    }
}

/// `Probe`'s value channel is the wrapped `f64`, bit for bit — the
/// whole content of "transparent newtype".
///
/// The impl lives in this suite rather than beside the trait because
/// `Probe` exists only under `probe`, and a `cfg(feature = "probe")`
/// attribute inside `tests/fixture/` would enter the probe census as a
/// suite NESTED under `tests/`, which `probe-suite-census.sh` refuses
/// outright rather than mis-report.
impl ValueChannelBits for Probe {
    fn feed(self, d: &mut Digest) {
        d.u64(self.0.to_bits());
    }
}

/// A mass-properties certificate, fed exactly. The two pads are `f64`
/// at every lane scalar, so they enter as raw words; the two measured
/// quantities go through the lane's own value channel.
fn feed_props<T: geom_core::Decide + ValueChannelBits>(
    d: &mut Digest,
    m: &topo::MassProperties<T>,
) {
    d.scalar(m.volume);
    d.scalar(m.surface_area);
    d.u64(m.volume_pad.to_bits());
    d.u64(m.area_pad.to_bits());
}

/// A result's arm, as a scalar-free token: the coarsest channel the
/// differential compares, and the one that catches an arm which went
/// red at one scalar and green at the other.
fn arm<T: geom_core::Decide>(r: Option<&NodeResult<T>>) -> &'static str {
    match r {
        None => "absent",
        Some(NodeResult::Ok(_)) => "ok",
        Some(NodeResult::Failed(_)) => "failed",
        Some(NodeResult::Poisoned { .. }) => "poisoned",
    }
}

/// **The wrapper property, as a differential rather than as a claim:
/// evaluating the corpus at `Probe` and at `f64` produces BIT-IDENTICAL
/// results.**
///
/// `Probe` is a transparent newtype over `f64`
/// (`geom_core::k_stats::Probe(pub f64)`), so the property to assert is
/// the strong one — identical bits, not agreement within a tolerance.
/// Every scalar therefore enters through `to_bits` and every comparison
/// is over words: `NaN != NaN` and `-0.0 == 0.0` both lie about a
/// wrapper, and a differential that cannot tell `-0.0` from `0.0` is
/// not checking one.
///
/// # ε is an INPUT here, never a comparison tolerance
///
/// Nothing in the comparison is approximate, but the thing compared is
/// not ε-free: ε selects which side of a band every classification
/// falls on, so it decides what is evaluated at all. So this row fixes
/// ε (the ambient `Tol::witness()`, one process per ε — `Tolerance` is
/// a `OnceLock`), evaluates BOTH arithmetics at that ε, and compares
/// bits. `scripts/k_probe_sweep.sh` runs the `--ignored` selection over
/// this module once per ε row the corpus dump is swept at — 1e-6, 1e-9,
/// 1e-12 — so the property is asserted at each of them and at none of
/// them by extrapolation. *"There is nothing per-ε about bit-identity"*
/// is the wrong argument: it is a claim about the COMPARISON, and the
/// ε-dependence is in the EVALUATION.
///
/// What today's corpus measures is printed rather than assumed: the
/// run's folded value channel is in this row's own log line, and on
/// this tree the three ε rows print the same word, so the three
/// comparisons are over the same numbers. That is a reading of one
/// corpus at one SHA, not a licence to keep one row — ε does reach the
/// arithmetic (the quadrature arm below), and the printed word is what
/// would say so if the corpus stopped being insensitive to it.
///
/// # What is compared
///
/// Per node, in evaluation order, four channels — three of them
/// scalar-free and compared directly rather than through a digest:
///
/// - the result's arm, and on a refusal the `NodeError`'s rendering, on
///   a poisoning its failed ancestor (`NodeError` is not generic over
///   the lane scalar, so the two arms' copies compare as text);
/// - the content key, which `ContentBits for Probe` claims "feeds bits
///   identical to the f64 lane by construction" and which nothing
///   checked;
/// - the name table, and the verdict and escalation logs — the
///   decision SEQUENCE, which is what "never a different arithmetic"
///   means most directly;
/// - the payload's stored geometry through `fixture::value_channel`,
///   one digest word per node, so a divergence names the node.
///
/// # What it does NOT compare
///
/// Curve carriers, surface geometry and pcurves (the shared feed's
/// stated hole — the review digests sample those at `Dual64` and
/// nothing samples them at `Probe`): a `Probe` divergence confined to a
/// NURBS control net, or to a face's surface with every stored point
/// unmoved, passes this row. Nor does it reach the documents that sit
/// BESIDE the registry rather than in it (`cup` and `vessel`, the
/// hollowing pair — no `Shell` result is compared here at all), the
/// three registered documents that carry no `result` node and so take
/// the mass-properties arm not at all, the demo scenes, or the M2
/// `sweep::k_report` corpus — the same sweep dumps the last two and no
/// differential reads either.
///
/// # Not a greenness row
///
/// It deliberately does not require either arm to be green. The
/// greenness rows beside it do that; requiring it here would make the
/// differential's subject tolerance-dependent and would hide exactly
/// the divergence worth catching, which is a `Probe` arm that refuses
/// where `f64` succeeds. An arm mismatch is a failure of THIS row.
#[test]
#[ignore = "swept at every eps row beside the dump; one process per eps (see module docs)"]
fn probe_agrees_with_f64_bit_for_bit_over_the_corpus() {
    let eps = Tol::witness().get().eps;
    let docs = documents();
    assert!(
        !docs.is_empty(),
        "the corpus registry is empty, so this differential compared nothing"
    );
    let mut words = 0usize;
    let mut nodes = 0usize;
    let mut samples = 0usize;
    // The run's whole compared surface, folded, and printed below. NOT
    // an oracle — nothing compares it to a constant. It is the line a
    // reader of the sweep log uses to see whether two eps rows compared
    // the same numbers, which is the question a collapsed sweep answers
    // by assertion. On this tree they do: the three rows print one
    // word. That is a measurement of today's corpus, not a licence —
    // eps reaches the arithmetic (the quadrature arm below), and the
    // measurement is what would say so if it stopped being true.
    let mut roll = Digest::new();

    for d in &docs {
        let ev_f = corpus::eval::<f64>(&d.doc);
        // Recorded around the `Probe` arm ONLY, and asserted non-empty
        // in total below: a run that recorded no classification at all
        // did not evaluate at the recording scalar, which is the way
        // this differential could compare an arithmetic against itself
        // and pass.
        k_stats::start_recording();
        let ev_p = corpus::eval::<Probe>(&d.doc);
        samples += k_stats::take_samples().len();

        assert_eq!(ev_f.order, ev_p.order, "{}: evaluation order", d.name);
        assert_eq!(ev_f.outcome, ev_p.outcome, "{}: evaluation outcome", d.name);

        for &id in &ev_f.order {
            let rf = ev_f.result(id);
            let rp = ev_p.result(id);
            assert_eq!(
                arm(rf),
                arm(rp),
                "{}: node {id:?}: the two arithmetics took different result arms",
                d.name
            );
            match (rf, rp) {
                (Some(NodeResult::Failed(ef)), Some(NodeResult::Failed(ep))) => assert_eq!(
                    format!("{ef:?}"),
                    format!("{ep:?}"),
                    "{}: node {id:?}: refusal differs",
                    d.name
                ),
                (
                    Some(NodeResult::Poisoned { through: tf }),
                    Some(NodeResult::Poisoned { through: tp }),
                ) => assert_eq!(tf, tp, "{}: node {id:?}: poisoned through", d.name),
                (Some(NodeResult::Ok(vf)), Some(NodeResult::Ok(vp))) => {
                    assert_eq!(
                        vf.content_key, vp.content_key,
                        "{}: node {id:?}: content key — `ContentBits for Probe` feeds the \
                         wrapped f64's bits, so the two lanes' memo keys are the same word",
                        d.name
                    );
                    assert!(
                        vf.name_table == vp.name_table,
                        "{}: node {id:?}: name table ({} entries at f64, {} at Probe)",
                        d.name,
                        vf.name_table.iter().count(),
                        vp.name_table.iter().count()
                    );
                    assert_eq!(
                        vf.verdicts, vp.verdicts,
                        "{}: node {id:?}: verdict log — the two lanes decided differently, \
                         which is the wrapper property failing at the decision itself",
                        d.name
                    );
                    assert_eq!(
                        vf.escalations, vp.escalations,
                        "{}: node {id:?}: escalation log",
                        d.name
                    );
                }
                _ => unreachable!("the arms were just asserted equal"),
            }
        }

        // THE QUADRATURE, which is where eps reaches the arithmetic
        // rather than only the bands: a certified face's enclosure is
        // refined against an eps-derived band, so the iterate it stops
        // at, and the volume and pad it reports, are functions of eps.
        // The stored payload the node digest reads is not — mass
        // properties are COMPUTED from it and stored nowhere — so
        // without this arm the eps-sensitive half of the run is outside
        // everything compared here.
        if let Some(result) = d.result
            && matches!(ev_f.result(result), Some(NodeResult::Ok(_)))
        {
            let tol = Tol::witness();
            let mf = mass_properties(body_of(&ev_f, result), tol);
            let mp = mass_properties(body_of(&ev_p, result), tol);
            match (&mf, &mp) {
                (Ok(a), Ok(b)) => {
                    let (mut da, mut db) = (Digest::new(), Digest::new());
                    feed_props(&mut da, a);
                    feed_props(&mut db, b);
                    assert_eq!(
                        (da.hash(), da.fed()),
                        (db.hash(), db.fed()),
                        "{}: mass properties diverged at eps={eps:e} — f64 {a:?} vs \
                         Probe volume/area/pads",
                        d.name
                    );
                    roll.u64(da.hash());
                    words += da.fed();
                }
                (Err(a), Err(b)) => assert_eq!(
                    format!("{a:?}"),
                    format!("{b:?}"),
                    "{}: the two arithmetics refused mass properties differently",
                    d.name
                ),
                _ => panic!(
                    "{}: mass properties succeeded at one arithmetic and not the other \
                     (f64: {}, Probe: {})",
                    d.name,
                    if mf.is_ok() { "ok" } else { "refused" },
                    if mp.is_ok() { "ok" } else { "refused" }
                ),
            }
        }

        let bits_f = value_digest_nodes(&ev_f);
        let bits_p = value_digest_nodes(&ev_p);
        assert_eq!(
            bits_f.len(),
            bits_p.len(),
            "{}: digested node count",
            d.name
        );
        for (f, p) in bits_f.iter().zip(&bits_p) {
            assert_eq!(
                f, p,
                "{}: node {:?}: the value channels diverged — (node, digest, words fed) \
                 at f64 vs at Probe, at eps={eps:e}",
                d.name, f.0
            );
            roll.u64(f.1);
            words += f.2;
            nodes += 1;
        }
    }

    assert!(
        words > 0,
        "the value-channel feed read no word from any of the {} corpus documents: two empty \
         digests agree, and that agreement is not the wrapper property",
        docs.len()
    );
    assert!(
        samples > 0,
        "the `Probe` arm classified nothing through the k_stats funnel across {} documents, \
         so nothing here ran at the recording scalar",
        docs.len()
    );
    eprintln!(
        "probe-vs-f64 differential: eps={eps:e}, {} documents, {nodes} nodes, {words} value \
         words, {samples} recorded classifications, corpus value channel {:#018x}",
        docs.len(),
        roll.hash()
    );
}

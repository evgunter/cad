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
//! comparison FOR THIS CRATE'S EVALUATION LANE — so the whole file is
//! gated on the `probe` feature and nothing non-Probe is lost from the
//! default build.
//!
//! **What was unchecked, stated exactly.** `Probe`'s wrapper property
//! was not unchecked in the tree: `profile`'s
//! `review_m2_pr2_probe.rs` has compared canonical profile output and
//! refusal outcomes at `Probe` against `f64` through `to_bits()` since
//! M2, and it is rostered and runs under the default selection. What
//! had no differential was the EVALUATION lane — a whole document
//! through this crate's evaluator, its bodies, keys, name tables and
//! decision logs — which is what the row below compares. On main the
//! two lanes' content and naming keys currently split on the nodes it
//! pins (`PINNED_KEY_SPLITS`), and everything else it compares agrees.

#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture::value_channel::{
    Digest, ValueChannelBits, fold_words, props_digest, value_digest_nodes,
};

use std::io::Write as _;

use geom_core::k_stats::{self, MarginSample, Probe, SampleOutcome};
use topo::{validate, validate_closed};

use corpus::{body_of, documents, eval, failures};
use editor_core::{BooleanValue, NodeResult, ValuePayload};
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
    let Some(result) = d.result else {
        return k_stats::take_samples();
    };
    validate(&body_of(&ev, result)).expect("tier 1");
    validate_closed(&body_of(&ev, result)).expect("closed");
    let mut samples = k_stats::take_samples();
    // The finish (tier 3) runs between the two recording windows, so the
    // corpus holds what it held before the typed door: the evaluation,
    // tiers 1 and 2, and the measurement.
    let body = topo::test_support::finished("body", body_of(&ev, result), Tol::witness());
    k_stats::start_recording();
    topo::mass_properties(
        &body,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .expect("mass properties");
    samples.extend(k_stats::take_samples());
    samples
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
/// says at one ε. The check at this lane is
/// [`probe_agrees_with_f64_bit_for_bit_save_the_pinned_key_splits`] below, which
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
/// attribute inside `tests/fixture/` would enter the probe census.
/// `probe-suite-census.sh`'s `--suites` mode refuses a nested file
/// outright, and that mode is not one CI calls; what binds is the
/// DISPOSITION half, which runs on every code-tier row and would
/// demand that the helper be rostered in `RUN_FLOOR` or declare itself
/// unrun — neither of which a shared feed can honestly say.
impl ValueChannelBits for Probe {
    fn feed(self, d: &mut Digest) {
        d.u64(self.0.to_bits());
    }
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

/// The nodes whose content and naming keys split between the lanes on
/// main: per document, the mint ordinals of the nodes that split, every
/// one splitting both keys. Pinned so the rest of the differential runs
/// (`work/tally/probe-lane-content-and-naming-keys-split-from-f64-on-every-corpus-document.md`);
/// every other comparison holds on them, and the pin goes red the day
/// the set moves.
const PINNED_KEY_SPLITS: &[(&str, &[u32])] = &[
    (
        "die",
        &[
            10, 18, 29, 36, 47, 54, 65, 72, 83, 90, 101, 108, 119, 126, 160, 163, 196, 199, 232,
            235, 268, 271, 304, 307, 340, 343, 390, 393, 440, 443, 483, 486, 526, 529, 569, 572,
            612, 615, 655, 658, 705, 708, 755, 758, 805, 808, 841, 844, 877, 880, 913, 916, 949,
            952, 992, 995, 1003,
        ],
    ),
    (
        "corner_table",
        &[
            9, 17, 28, 36, 50, 56, 66, 74, 88, 94, 104, 112, 126, 132, 142, 150, 164, 170, 178,
        ],
    ),
    ("heat_sink", &[10, 18, 29, 37, 48, 56, 64, 78, 86]),
    (
        "crossing_slots",
        &[9, 17, 28, 36, 50, 56, 66, 74, 88, 94, 102],
    ),
    (
        "nested_islands_105",
        &[
            9, 17, 28, 36, 47, 55, 69, 75, 88, 94, 104, 112, 126, 132, 140,
        ],
    ),
    (
        "nested_islands_106_depth1",
        &[
            9, 17, 28, 36, 47, 55, 69, 75, 88, 94, 104, 112, 126, 132, 140,
        ],
    ),
    (
        "nested_islands_106_depth2",
        &[
            9, 17, 28, 36, 47, 55, 69, 75, 88, 94, 104, 112, 126, 132, 142, 150, 164, 170, 178,
        ],
    ),
    ("declared_tangency", &[16, 28, 44, 57, 66, 80, 88]),
    (
        "kitchen_sink",
        &[
            26, 35, 46, 54, 64, 70, 78, 88, 102, 111, 122, 131, 141, 152, 166, 180, 196, 212, 228,
            236,
        ],
    ),
    ("cut_cylinder", &[5, 9, 36, 45, 59, 67]),
    ("measured_web", &[10, 18, 23, 27, 32, 36, 46, 54, 65]),
    ("boss_union", &[9, 17, 24, 28, 42, 48, 56]),
    ("die_fillet", &[9, 17, 22, 30]),
    ("die_chamfer", &[9, 17, 22, 30]),
    ("die_pips", &[9, 17, 25, 34, 80, 85, 93]),
    ("heat_sink_fins", &[10, 18, 29, 37, 45]),
    (
        "die_tool",
        &[9, 17, 25, 34, 52, 63, 77, 91, 105, 119, 133, 141, 143, 151],
    ),
    ("face_sketch", &[9, 17, 28, 36, 45, 65, 73]),
    ("part_select", &[11, 18, 28, 31, 41, 55, 70, 78]),
    ("symmetric_plate", &[14, 23, 28, 30, 38]),
    ("loft_prism", &[9, 24, 39, 49, 58]),
    ("die_composed", &[9, 17, 25, 34, 80, 85, 89, 97]),
    (
        "die_composed_tour",
        &[
            9, 17, 22, 29, 38, 98, 173, 248, 323, 398, 473, 548, 623, 698, 773, 848, 923, 998,
            1073, 1148, 1223, 1298, 1373, 1448, 1523, 1598, 1603, 1605, 1609, 1613, 1621,
        ],
    ),
    ("plate_param", &[14, 24, 35, 43, 57, 63, 71]),
    ("kiss_carry", &[9, 17, 28, 36, 50, 56, 66, 74, 88, 94, 102]),
    ("reshaped_rod", &[14, 24, 29, 55]),
    (
        "pin_in_bore",
        &[9, 20, 34, 47, 57, 71, 98, 109, 123, 136, 146, 162, 170],
    ),
    ("axis_projection", &[17, 32, 38, 43, 51]),
    (
        "blocks_on_one_floor",
        &[9, 17, 28, 36, 45, 73, 81, 90, 110, 118, 132, 140],
    ),
    ("revolves_on_one_axis", &[11, 20, 32, 41, 51, 59, 67]),
];

/// **The wrapper property at the EVALUATION lane, as a differential
/// rather than as a claim: evaluating the corpus at `Probe` and at
/// `f64` produces BIT-IDENTICAL results** — save, today, the content and
/// naming keys of the nodes [`PINNED_KEY_SPLITS`] names, which split
/// between the lanes on 30 documents while everything else about those
/// nodes agrees.
///
/// Scope, because the scope is the part that was got wrong once: the
/// wrapper property as such was already checked one crate over —
/// `profile`'s `review_m2_pr2_probe.rs` compares canonical form and
/// refusal outcomes at the two scalars bit for bit, and carries the
/// same recording idiom. What no row compared was a DOCUMENT through
/// this crate's evaluator.
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
/// **Geometry.** Curve carriers, surface geometry and pcurves — the
/// shared feed's stated hole, and the review digests sample those at
/// `Dual64` while nothing samples them at `Probe`. A `Probe` divergence
/// confined to a NURBS control net, or to a face's surface with every
/// stored point unmoved, passes this row.
///
/// **The record.** Of `NodeValue`'s fields, eight are read here —
/// payload, name tables, verdicts, escalations, content key, naming
/// key, contacts, carried declarations — and a placement's motion is
/// read through the payload of the copy it places. Two of the eight are
/// read but EMPTY on today's corpus: `contacts` and
/// `carried` are filled at the instantiate op alone and the registry
/// has none, so those two comparisons hold vacuously and are counted
/// as covered by nothing. A boolean's surviving contacts are a
/// different channel, on the PAYLOAD, and are compared with a counter
/// that says how many nodes carried a non-empty one (two, today —
/// `kiss_carry`'s).
///
/// **Documents.** `cup` and `vessel` sit BESIDE the registry, so no
/// `Shell` result is compared here at any scalar; three registered
/// documents carry no `result` node and take the mass-properties arm
/// not at all; the demo scenes and the M2 `sweep::k_report` corpus are
/// dumped by the same sweep and read by no differential.
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
fn probe_agrees_with_f64_bit_for_bit_save_the_pinned_key_splits() {
    let eps = Tol::witness().get().eps;
    let docs = documents();
    assert!(
        !docs.is_empty(),
        "the corpus registry is empty, so this differential compared nothing"
    );
    let mut words = 0usize;
    let mut nodes = 0usize;
    let mut samples = 0usize;
    // COUNTED, NOT ASSUMED, and this is the guard the other three do
    // not give. Both conditional arms below are reached through a
    // `d.result` that may be `None` and a payload arm that may not
    // match, and `!docs.is_empty()`, `words > 0` and `samples > 0` are
    // all satisfied with either arm dead — `words` is fed by the node
    // digests too. A reviewer switched the mass-properties arm off and
    // this row stayed green over a hundred fewer words; these two
    // counters are what closed that, and both are asserted positive.
    let mut props = 0usize;
    let mut contacts = 0usize;
    // The run's whole compared surface, folded, and printed below. NOT
    // an oracle — nothing compares it to a constant. It is the line a
    // reader of the sweep log uses to see whether two eps rows compared
    // the same numbers, which is the question a collapsed sweep answers
    // by assertion. On this tree they do: the three rows print one
    // word. That is a measurement of today's corpus, not a licence —
    // eps reaches the arithmetic (the quadrature arm below), and the
    // measurement is what would say so if it stopped being true.
    let mut rolled: Vec<u64> = Vec::new();
    let mut content_splits: Vec<(&str, u32)> = Vec::new();
    let mut naming_splits: Vec<(&str, u32)> = Vec::new();

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
                    if vf.content_key != vp.content_key {
                        content_splits.push((d.name, id.0.ordinal()));
                    }
                    let (tf, tp) = (
                        crate::fixture::tables(&ev_f, id),
                        crate::fixture::tables(&ev_p, id),
                    );
                    let count = |ts: &[&editor_core::NameTable]| -> usize {
                        ts.iter().map(|t| t.iter().count()).sum()
                    };
                    assert!(
                        tf == tp,
                        "{}: node {id:?}: name tables ({} entries at f64, {} at Probe)",
                        d.name,
                        count(&tf),
                        count(&tp)
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
                    if vf.naming_key != vp.naming_key {
                        naming_splits.push((d.name, id.0.ordinal()));
                    }
                    // The value's contact records: a boolean's surviving
                    // contacts, and an instance's declared ones, which
                    // this registry has none of.
                    let (cf, cp) = (&vf.carry.contacts, &vp.carry.contacts);
                    assert!(
                        cf == cp,
                        "{}: node {id:?}: contact records ({} vv / {} v-on-f / {} e-f / {} f-f \
                         at f64)",
                        d.name,
                        cf.vertex_vertex().count(),
                        cf.vertex_face().count(),
                        cf.edge_face().count(),
                        cf.face_face().count()
                    );
                    assert!(
                        vf.carry.findings == vp.carry.findings,
                        "{}: node {id:?}: carried mate declarations",
                        d.name
                    );
                    // The shared node feed digests a boolean's result
                    // BODY and drops its kind and records. `kiss_carry`
                    // is in the registry PRECISELY because it is the
                    // corpus boolean whose result carries non-empty
                    // surviving contacts, and the counter is what says
                    // this channel was fed by something.
                    if let (
                        ValuePayload::Boolean(BooleanValue::Body { kind: kf, .. }),
                        ValuePayload::Boolean(BooleanValue::Body { kind: kp, .. }),
                    ) = (&vf.payload, &vp.payload)
                    {
                        assert_eq!(
                            kf, kp,
                            "{}: node {id:?}: boolean result classification",
                            d.name
                        );
                        if cf.vertex_vertex().next().is_some()
                            || cf.vertex_face().next().is_some()
                            || cf.edge_face().next().is_some()
                            || cf.face_face().next().is_some()
                        {
                            contacts += 1;
                        }
                    }
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
            let mf = topo::mass_properties(
                &topo::test_support::finished("the measured body", body_of(&ev_f, result), tol),
                tol,
                topo::test_support::pinned_resolution(tol),
            );
            let mp = topo::mass_properties(
                &topo::test_support::finished("the measured body", body_of(&ev_p, result), tol),
                tol,
                topo::test_support::pinned_resolution(tol),
            );
            match (&mf, &mp) {
                (Ok(a), Ok(b)) => {
                    let (ha, wa) = props_digest(&a.enclosure);
                    assert_eq!(
                        (ha, wa),
                        props_digest(&b.enclosure),
                        "{}: mass properties diverged at eps={eps:e} — f64 {a:?} vs \
                         Probe volume/area/pads",
                        d.name
                    );
                    rolled.push(ha);
                    words += wa;
                    props += 1;
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
            rolled.push(f.1);
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
    let pinned: Vec<(&str, u32)> = (PINNED_KEY_SPLITS.iter())
        .flat_map(|&(doc, nodes)| nodes.iter().map(move |&n| (doc, n)))
        .collect();
    let by_doc = |splits: &[(&'static str, u32)]| {
        let mut out: Vec<(&str, Vec<u32>)> = Vec::new();
        for &(doc, n) in splits {
            match out.iter_mut().find(|(d, _)| *d == doc) {
                Some((_, nodes)) => nodes.push(n),
                None => out.push((doc, vec![n])),
            }
        }
        out
    };
    for (key, splits) in [("content", &content_splits), ("naming", &naming_splits)] {
        assert!(
            *splits == pinned,
            "the {key} keys split between the lanes on other nodes than the pinned ones — \
             `ContentBits for Probe` feeds the wrapped f64's bits, so the two lanes' memo keys \
             should be the same word, and on main they split on the documents and nodes \
             `PINNED_KEY_SPLITS` names: {:?}",
            by_doc(splits)
        );
    }
    assert!(
        samples > 0,
        "the `Probe` arm classified nothing through the k_stats funnel across {} documents, \
         so nothing here ran at the recording scalar",
        docs.len()
    );
    assert!(
        props > 0,
        "no corpus document reached the mass-properties arm across {} of them, so the one \
         channel here that eps reaches was compared on nothing while every other guard \
         stayed satisfied",
        docs.len()
    );
    assert!(
        contacts > 0,
        "no boolean in the corpus carried a non-empty surviving contact record, so that \
         channel agreed over empty vectors only — `kiss_carry` is registered to make this \
         false"
    );
    eprintln!(
        "probe-vs-f64 differential: eps={eps:e}, {} documents, {nodes} nodes, {words} value \
         words, {props} mass certificates, {contacts} contact-carrying nodes, {samples} \
         recorded classifications, corpus value channel {:#018x}",
        docs.len(),
        fold_words(rolled)
    );
}

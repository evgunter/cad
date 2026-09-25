//! **The runtime half of E4's door** (`docs/DUAL-DESIGN.md` DL2/DL3;
//! `docs/M10-DI-SPEC.md`): the whole Band 4 corpus evaluates AND
//! gathers at `Dual64`, with the value channel bit-identical to the
//! plain `f64` run, and the memo behaves soundly when a prior
//! `Evaluation<Dual64>` is threaded.
//!
//! # What the value-channel digest reads — and does not
//!
//! The feed is `fixture::value_channel`'s, shared with the other
//! cross-scalar differential over this corpus (`m4_pr8_k_probe`'s
//! `Probe`-vs-`f64` row) so that the claim means one thing for both.
//! It is not shared with every cross-scalar differential in the tree —
//! `profile`'s and `sweep`'s read their own crates' values and cannot
//! reach an `Evaluation` at all. For every node in evaluation order:
//! the result's arm (`Ok`/`Failed`/`Poisoned`), the payload's arm, and
//! the payload's stored geometry read through each scalar's OWN value channel —
//! `f64` bits at `f64`, the value channel's bits at `Dual64`,
//! `repr_bits` with the decoration at the interval pair. It does NOT
//! read curve carriers, surface geometry, or pcurves — the adopted
//! review digests do (`r1_dual_probes`'s lattice-sampled carriers,
//! `r2_m10_di_probes`'s control nets/weights/knots), and those rows
//! gate beside these.
//!
//! # The DL3 witnesses, by name
//!
//! Corpus documents that gather green at `Dual64` while the direct
//! at-rest door refuses their product bodies at that scalar are the
//! measured reason the policy seam exists. The door a dual can take is
//! now the validator's STRUCTURAL half — the composed entry carries the
//! +V invariant's certified bound and cannot be called at a dual — so
//! the witness set is the set that half refuses, pinned by name in both
//! directions below, and a witness silently going green (or a new one
//! appearing) is loud.
//!
//! **The set is EMPTY today, and that is the measurement rather than an
//! omission.** Neither `VolumeUncomputable` NOR `NegativeVolume` is
//! among the classes a dual can collect here any more, and the reason
//! is the door rather than the scalar: the whole +V invariant — its
//! certified quadrature AND its closed form, which computes at any
//! scalar — lives in the certified half, so the structural door says
//! nothing about orientation at all. DL3's two named witnesses,
//! `cut_cylinder` (ellipse-trimmed cylinder) and `loft_prism` (NURBS
//! walls), now pass the structural half at `Dual64`; the row below pins
//! them there and at the composed `f64` door, so the retirement is
//! asserted rather than left as an absence. **What a dual gives up here
//! is not only the refusal it used to receive but, on a closed-form
//! body, the SIGN it used to be given** — recorded at the doors and
//! pinned by `topo/tests/geometric_cube.rs`'s
//! `the_structural_half_does_not_judge_orientation_at_any_scalar`. The
//! verdict is still reachable at a dual through the `_structural`
//! passes (`validate_pseudomanifold_structural`, `contact_marks_structural`,
//! `mass_properties_structural`).
//!
//! # What the memo rows assert
//!
//! The seeding surface is M10-4's, so no public door can yet put a
//! nonzero tangent into an evaluation; the *different-seeds* half of
//! DL2's soundness law is pinned at the key level (`ContentBits for
//! Dual` feeds BOTH channels, so tangent bits move the key), and the
//! through-the-door halves that ARE reachable — same-seed replay
//! reuses everything; a parameter edit reuses exactly the complement
//! of its downstream cone — run against the real evaluator.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;

use crate::fixture::value_channel::{body_digest, value_digest};

use corpus::{CorpusDoc, cone, documents, eval, failures};
use editor_core::eval::KeyHasher;
use editor_core::{CancelToken, ContentKey, EvalOptions, EvalOutcome, evaluate, product_recorded};
use geom_core::{Dual64, Tol};

/// The gather-door witnesses (module docs): they gather at `Dual64`
/// because the policy gate is absent, and the door a dual CAN take —
/// the validator's structural half — refuses their product bodies at
/// that scalar.
const DUAL_REFUSED_BY_THE_STRUCTURAL_DOOR: [&str; 0] = [];
const FORMER_DL3_WITNESSES: [&str; 2] = ["cut_cylinder", "loft_prism"];

/// The corpus documents a NAMED witness list addresses, looked up
/// loudly: a renamed or retired witness fails here instead of
/// silently emptying the row.
fn named<'a>(docs: &'a [CorpusDoc], names: &[&str]) -> Vec<&'a CorpusDoc> {
    names
        .iter()
        .map(|n| {
            docs.iter()
                .find(|d| d.name == *n)
                .unwrap_or_else(|| panic!("witness `{n}` is not in the corpus — re-derive the set"))
        })
        .collect()
}

/// DL2 + DL3, end to end: every corpus document evaluates green at
/// `Dual64` and its value channel is bit-identical, node by node, to
/// the `f64` evaluation of the same document.
#[test]
fn every_document_evaluates_at_dual64_with_the_f64_value_channel() {
    for doc in documents() {
        let ev_f = eval::<f64>(&doc.doc);
        let ev_d = eval::<Dual64>(&doc.doc);
        let bad = failures(&ev_d);
        assert!(
            bad.is_empty(),
            "{}: {} node(s) did not evaluate green at Dual64:\n{}",
            doc.name,
            bad.len(),
            bad.join("\n")
        );
        assert_eq!(
            ev_d.outcome,
            EvalOutcome::Completed,
            "{}: outcome",
            doc.name
        );
        assert_eq!(ev_f.order, ev_d.order, "{}: evaluation order", doc.name);
        assert_eq!(
            value_digest(&ev_f),
            value_digest(&ev_d),
            "{}: the Dual64 value channel diverged from the f64 evaluation",
            doc.name
        );
    }
}

/// DL3 working at the gather, with its witness sets pinned BY NAME:
/// every corpus product gathers at `f64` (the refusal set is EMPTY —
/// if a refusing document ever joins the corpus, this row is where the
/// f64 `gate_at_rest` arm gains its corpus-level witness; today that
/// arm's refusing pin is `topo`'s `at_rest_policy_tests`), every one
/// gathers at `Dual64` too with a bit-equal value channel, and the
/// documents whose product bodies the structural door refuses at
/// `Dual64` are exactly [`DUAL_REFUSED_BY_THE_STRUCTURAL_DOOR`] —
/// gathered green only because the policy gate is absent there.
#[test]
fn the_gather_opens_at_dual64_and_the_witness_set_is_pinned() {
    let tol = Tol::witness();
    let docs = documents();
    let mut structural_door_refused: Vec<&'static str> = Vec::new();
    for doc in &docs {
        let ev_f = eval::<f64>(&doc.doc);
        let ev_d = eval::<Dual64>(&doc.doc);
        let product_f = product_recorded(&doc.doc, &ev_f, tol)
            .unwrap_or_else(|e| panic!("{}: the f64 product gather refused: {e}", doc.name));
        let product_d = product_recorded(&doc.doc, &ev_d, tol)
            .unwrap_or_else(|e| panic!("{}: gathers at f64 but refused at Dual64: {e}", doc.name));
        assert_eq!(
            body_digest(&product_f.body),
            body_digest(&product_d.body),
            "{}: product value channel",
            doc.name
        );
        if topo::validate_geometric_structural(&product_d.body, tol).is_err() {
            structural_door_refused.push(doc.name);
        }
    }
    assert_eq!(
        structural_door_refused,
        DUAL_REFUSED_BY_THE_STRUCTURAL_DOOR.to_vec(),
        "the structural-door-refuses-at-Dual64 witness set moved — re-derive \
         DL3's witnesses and update the module docs"
    );
}

/// **DL3's two named witnesses no longer refuse anything at `Dual64`,
/// and that is the measurement this row now carries.** Both refused the
/// direct door with `VolumeUncomputable`, raised by the dual's refusing
/// quadrature arm inside the +V invariant — and the invariant as a
/// whole, closed form included, is what the split moved behind the
/// certified bound. So these two lose a refusal here; a body whose
/// closed form DOES compute loses the verdict instead. With the validator split, `cut_cylinder` and
/// `loft_prism` pass the structural half at `Dual64` outright, and the
/// same documents pass the composed door at `f64`, so the pair is a
/// scalar difference in what may be CLAIMED and no longer a difference
/// in what is FOUND.
///
/// Pinned by name, both directions, on both doors: a witness that
/// starts refusing again is loud, and so is one whose `f64` verdict
/// moves. What this row does NOT assert is that the policy seam is
/// unnecessary — the seam is why these gather at all, and its own
/// justification is a scalar's certification rights, not this corpus.
#[test]
fn the_dl3_witnesses_pass_both_doors_they_can_still_reach() {
    let tol = Tol::witness();
    let docs = documents();
    for doc in named(&docs, &FORMER_DL3_WITNESSES) {
        let ev_d = eval::<Dual64>(&doc.doc);
        let product = product_recorded(&doc.doc, &ev_d, tol)
            .unwrap_or_else(|e| panic!("{}: must gather at Dual64: {e}", doc.name));
        assert_eq!(
            topo::validate_geometric_structural(&product.body, tol),
            Ok(()),
            "{}: the structural half must pass at Dual64 — its refusal was the \
             +V invariant's, and this door runs no part of that invariant",
            doc.name
        );
        let ev_f = eval::<f64>(&doc.doc);
        let product_f = product_recorded(&doc.doc, &ev_f, tol)
            .unwrap_or_else(|e| panic!("{}: the f64 product gather refused: {e}", doc.name));
        assert_eq!(
            topo::validate_geometric(&product_f.body, tol),
            Ok(()),
            "{}: the composed door must pass at f64 — the +V invariant is a claim \
             about this body that a certifying scalar can make",
            doc.name
        );
    }
}

/// The census arm at `Interval` (the certifying scalar the hosted
/// interval lane gates): `assemble` refuses and accepts on exactly the
/// same corpus documents as at `f64`. This is the corpus-level pin of
/// `Interval::gate_at_rest_declared` actually validating — gutting it
/// flips the refusing documents green and reds this row. (The f64
/// side of the same pin is `r2_m10_di_probes`'s divergence row; the
/// refusing-subject pins for every certifying arm are `topo`'s
/// `at_rest_policy_tests`.)
#[test]
fn assemble_census_verdicts_match_f64_at_interval() {
    use editor_core::assemble;
    use geom_core::Interval;
    let tol = Tol::witness();
    let mut refused: Vec<&'static str> = Vec::new();
    for doc in documents() {
        let ev_f = eval::<f64>(&doc.doc);
        let ev_i = eval::<Interval>(&doc.doc);
        let f = assemble(&doc.doc, &ev_f, tol);
        let i = assemble(&doc.doc, &ev_i, tol);
        assert_eq!(
            f.is_ok(),
            i.is_ok(),
            "{}: the census door disagreed across certifying scalars \
             (f64 {:?} vs Interval {:?})",
            doc.name,
            f.as_ref().err().map(|e| e.to_string()),
            i.as_ref().err().map(|e| e.to_string())
        );
        if i.is_err() {
            refused.push(doc.name);
        }
    }
    assert!(
        !refused.is_empty(),
        "no corpus document refuses the census at Interval — this row \
         no longer pins the Interval census arm; find a refusing witness"
    );
}

/// Same-seed replay: threading a prior `Evaluation<Dual64>` of the
/// SAME document reuses every node — the memo's keys are total over
/// the dual's fed bits, so bit-equal channels mean bit-equal keys.
#[test]
fn same_seed_replay_at_dual64_reuses_everything() {
    for doc in documents() {
        let prior = eval::<Dual64>(&doc.doc);
        let replay = evaluate::<Dual64>(
            &doc.doc,
            Some(&prior),
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        assert_eq!(
            replay.reused,
            doc.len(),
            "{}: replay must reuse every node",
            doc.name
        );
        assert_eq!(replay.recomputed, 0, "{}: replay recomputed", doc.name);
    }
}

/// Parameter-independent reuse: after the corpus bump edit, exactly
/// the bump's downstream cone recomputes at `Dual64`; everything
/// outside it reuses from the threaded prior.
#[test]
fn a_bumped_parameter_at_dual64_recomputes_exactly_its_cone() {
    for doc in documents() {
        let prior = eval::<Dual64>(&doc.doc);
        let bumped = doc.bumped();
        let after = evaluate::<Dual64>(
            &bumped,
            Some(&prior),
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        let cone = cone(&bumped, doc.bump_root);
        assert_eq!(
            after.recomputed,
            cone.len(),
            "{}: recomputed ≠ downstream cone",
            doc.name
        );
        assert_eq!(
            after.reused,
            bumped.len() - cone.len(),
            "{}: reuse outside the cone",
            doc.name
        );
    }
}

/// The different-seeds half of DL2's law, at the key level (module
/// docs): the tangent channel's bits move a content key, so a
/// seed-downstream node's key in pass pᵢ can never equal its key in
/// pass pⱼ, while bit-equal channels reproduce the key exactly.
#[test]
fn tangent_bits_separate_keys_and_equal_channels_share_them() {
    use editor_core::ContentBits;
    let key = |x: Dual64| -> ContentKey {
        let mut h = KeyHasher::new();
        // Disambiguated: the memo's own feed, not this file's digest.
        ContentBits::feed(&x, &mut h);
        h.finish()
    };
    let seeded = key(Dual64::variable(3.5));
    let unseeded = key(Dual64::constant(3.5));
    assert_ne!(
        seeded, unseeded,
        "a seeded and an unseeded pass over the same value must not share a key"
    );
    // The channels stay distinguishable because each scalar's feed has
    // a FIXED width (two words at `Dual64`) and the evaluator's key
    // prefixes every slot with its index and every list with its
    // length (`eval::content_key`), so value words can never slide
    // into tangent positions across slots. This row checks only the
    // one-slot swap; the cross-slot re-grouping measurement is
    // `r1_dual_probes`' collision-search row and
    // `r2_m10_di_probes::cross_scalar_feed_streams_alias...`'s second
    // half.
    assert_ne!(
        key(Dual64::new(2.0, 5.0)),
        key(Dual64::new(5.0, 2.0)),
        "swapping the channels within one scalar must move the key"
    );
    // Bit-equal channels reproduce the key — the reuse direction.
    assert_eq!(seeded, key(Dual64::variable(3.5)));
}

/// DL2's `Dual<Interval>` instantiation, through the same generic
/// impls: the derivative-enclosure scalar walks the whole door too,
/// with its value channel — `repr_bits`, decoration included —
/// identical to the plain `Interval` run's.
#[test]
fn dual_interval_evaluates_with_the_interval_value_channel() {
    use geom_core::{DualInterval, Interval};
    // The rows this lane runs (budget: one closed-form and one
    // NURBS-walled document, not the whole corpus — each row costs two
    // interval evaluations). A LOUD list: a renamed document fails the
    // lookup rather than silently shrinking the row.
    const DUAL_INTERVAL_ROWS: [&str; 2] = ["die", "loft_prism"];
    let docs = documents();
    for doc in named(&docs, &DUAL_INTERVAL_ROWS) {
        let ev_i = eval::<Interval>(&doc.doc);
        let ev_d = eval::<DualInterval>(&doc.doc);
        let bad = failures(&ev_d);
        assert!(
            bad.is_empty(),
            "{}: {} node(s) did not evaluate green at Dual<Interval>:\n{}",
            doc.name,
            bad.len(),
            bad.join("\n")
        );
        assert_eq!(
            value_digest(&ev_i),
            value_digest(&ev_d),
            "{}: the Dual<Interval> value channel diverged from Interval",
            doc.name
        );
    }
}

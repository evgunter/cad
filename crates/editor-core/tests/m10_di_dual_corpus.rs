//! **The runtime half of E4's door** (`docs/DUAL-DESIGN.md` DL2/DL3;
//! `docs/M10-DI-SPEC.md`): the whole Band 4 corpus evaluates AND
//! gathers at `Dual64`, with the value channel bit-identical to the
//! plain `f64` run, and the memo behaves soundly when a prior
//! `Evaluation<Dual64>` is threaded.
//!
//! # What the value-channel digest reads — and does not
//!
//! For every node in evaluation order: the result's arm
//! (`Ok`/`Failed`/`Poisoned`), the payload's arm, and the payload's
//! stored geometry read through each scalar's OWN value channel
//! ([`ValueChannelBits`] below — `f64` bits at `f64`, the value
//! channel's bits at `Dual64`, `repr_bits` with the decoration at the
//! interval pair): every body point, every datum frame, every profile
//! loop's vertices and bulges, plus arena counts. It does NOT read
//! curve carriers, surface geometry, or pcurves — the adopted review
//! digests do (`r1_dual_probes`'s lattice-sampled carriers,
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
//! verdict is still reachable at a dual through the mixed passes that
//! keep their lanes (`validate_pseudomanifold`, `contact_marks`,
//! `mass_properties`).
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

use corpus::{CorpusDoc, cone, documents, eval, failures};
use editor_core::eval::KeyHasher;
use editor_core::{
    AssertionVerdict, BooleanValue, CancelToken, ContentKey, DatumValue, Dimension, EvalOptions,
    EvalOutcome, Evaluation, NodeResult, SplitSide, ValuePayload, evaluate, product_recorded,
};
use geom_core::{Decide, Dual64, Tol};
use topo::Body;

/// The gather-door witnesses (module docs): they gather at `Dual64`
/// because the policy gate is absent, and the door a dual CAN take —
/// the validator's structural half — refuses their product bodies at
/// that scalar.
const DUAL_REFUSED_BY_THE_STRUCTURAL_DOOR: [&str; 0] = [];
const FORMER_DL3_WITNESSES: [&str; 2] = ["cut_cylinder", "loft_prism"];

/// A scalar's VALUE CHANNEL as exact bits — what "bit-identical to the
/// base scalar's run" quantifies over. At the interval pair this is
/// `repr_bits`, decoration included, so a decoration drift cannot hide
/// behind equal endpoints.
trait ValueChannelBits: Copy {
    fn feed(self, d: &mut Digest);
}

impl ValueChannelBits for f64 {
    fn feed(self, d: &mut Digest) {
        d.u64(self.to_bits());
    }
}

impl ValueChannelBits for Dual64 {
    fn feed(self, d: &mut Digest) {
        d.u64(self.value.to_bits());
    }
}

#[cfg(feature = "interval")]
impl ValueChannelBits for geom_core::Interval {
    fn feed(self, d: &mut Digest) {
        let (lo, hi, dec) = self.repr_bits();
        d.u64(lo);
        d.u64(hi);
        d.u64(u64::from(dec));
    }
}

#[cfg(feature = "interval")]
impl ValueChannelBits for geom_core::DualInterval {
    fn feed(self, d: &mut Digest) {
        self.value.feed(d);
    }
}

/// FNV-1a 64 over the evaluation's value-channel bits (module docs).
struct Digest(u64);

impl Digest {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }

    fn u64(&mut self, x: u64) {
        for b in x.to_le_bytes() {
            self.0 ^= u64::from(b);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    fn scalar<T: ValueChannelBits>(&mut self, x: T) {
        x.feed(self);
    }

    fn point3<T: Decide + ValueChannelBits>(&mut self, p: geom_core::Point3<T>) {
        self.scalar(p.x);
        self.scalar(p.y);
        self.scalar(p.z);
    }

    fn vec3<T: Decide + ValueChannelBits>(&mut self, v: geom_core::Vec3<T>) {
        self.scalar(v.x);
        self.scalar(v.y);
        self.scalar(v.z);
    }

    fn body<T: Decide + ValueChannelBits>(&mut self, body: &Body<T>) {
        self.u64(body.solids().count() as u64);
        self.u64(body.faces().count() as u64);
        self.u64(body.edges().count() as u64);
        self.u64(body.vertices().count() as u64);
        for (_k, p) in body.points() {
            self.point3(*p);
        }
    }
}

fn value_digest<T: Decide + ValueChannelBits>(ev: &Evaluation<T>) -> u64 {
    let mut d = Digest::new();
    for &id in &ev.order {
        d.u64(id.0);
        match ev.result(id) {
            None => d.u64(0),
            Some(NodeResult::Failed(_)) => d.u64(1),
            Some(NodeResult::Poisoned { .. }) => d.u64(2),
            Some(NodeResult::Ok(v)) => {
                d.u64(3);
                match &v.payload {
                    ValuePayload::Datum(DatumValue::Plane { origin, normal }) => {
                        d.u64(10);
                        d.point3(*origin);
                        d.vec3(normal.get());
                    }
                    ValuePayload::Datum(DatumValue::Axis { origin, dir }) => {
                        d.u64(11);
                        d.point3(*origin);
                        d.vec3(dir.get());
                    }
                    ValuePayload::Datum(DatumValue::Point { position }) => {
                        d.u64(12);
                        d.point3(*position);
                    }
                    ValuePayload::Datum(DatumValue::Frame { origin, u, v }) => {
                        d.u64(23);
                        d.point3(*origin);
                        d.vec3(u.get());
                        d.vec3(v.get());
                    }
                    // Tag 24, appended: an in-plane axis is its own
                    // payload, and BOTH its spellings are digested —
                    // the sketch pair is what a revolve consumes, so a
                    // drift there that the world lift happened to hide
                    // must still move the digest.
                    ValuePayload::Datum(DatumValue::AxisInPlane {
                        plane_origin,
                        plane_dir,
                        origin,
                        dir,
                    }) => {
                        d.u64(24);
                        d.scalar(plane_origin.x);
                        d.scalar(plane_origin.y);
                        d.scalar(plane_dir.x);
                        d.scalar(plane_dir.y);
                        d.point3(*origin);
                        d.vec3(dir.get());
                    }
                    ValuePayload::Profile(p) => {
                        d.u64(13);
                        for lp in p.validated.loops() {
                            d.u64(lp.vertices().len() as u64);
                            for v in lp.vertices() {
                                d.scalar(v.pos().x);
                                d.scalar(v.pos().y);
                                d.scalar(v.bulge());
                            }
                        }
                    }
                    ValuePayload::Body(b) => {
                        d.u64(14);
                        d.body(b);
                    }
                    ValuePayload::Boolean(BooleanValue::Empty) => d.u64(15),
                    ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
                        d.u64(16);
                        d.body(body);
                    }
                    ValuePayload::Split { above, below } => {
                        d.u64(17);
                        for side in [above, below] {
                            match side {
                                SplitSide::Empty => d.u64(0),
                                SplitSide::Body(b) => {
                                    d.u64(1);
                                    d.body(b);
                                }
                            }
                        }
                    }
                    ValuePayload::Instances(bodies) => {
                        d.u64(18);
                        d.u64(bodies.len() as u64);
                        for b in bodies {
                            d.body(b);
                        }
                    }
                    ValuePayload::Declarations(pairs) => {
                        d.u64(19);
                        d.u64(pairs.len() as u64);
                    }
                    ValuePayload::Mate(_) => d.u64(20),
                    // The measured quantity IS a lane value, so it is
                    // digested through the same value-channel bracket
                    // every coordinate takes.
                    ValuePayload::Measure { value, dim } => {
                        d.u64(21);
                        d.u64(match dim {
                            Dimension::Length => 1,
                            Dimension::Angle => 2,
                            Dimension::Count => 3,
                            Dimension::Scalar => 4,
                        });
                        d.scalar(*value);
                    }
                    ValuePayload::Assertion(verdict) => {
                        d.u64(22);
                        d.u64(match verdict.holds() {
                            Some(true) => 1,
                            Some(false) => 2,
                            None => 3,
                        });
                        match verdict {
                            AssertionVerdict::Holds { measured, bound }
                            | AssertionVerdict::Violated { measured, bound } => {
                                d.scalar(*measured);
                                d.scalar(*bound);
                            }
                            AssertionVerdict::Unevaluated { .. } => {}
                        }
                    }
                }
            }
        }
    }
    d.0
}

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
        let (mut df, mut dd) = (Digest::new(), Digest::new());
        df.body(&product_f.body);
        dd.body(&product_d.body);
        assert_eq!(df.0, dd.0, "{}: product value channel", doc.name);
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
#[cfg(feature = "interval")]
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
#[cfg(feature = "interval")]
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

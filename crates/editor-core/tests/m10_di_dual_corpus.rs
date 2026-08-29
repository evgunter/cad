//! **The runtime half of E4's door** (`docs/DUAL-DESIGN.md` DL2/DL3;
//! `docs/M10-DI-SPEC.md`): the whole Band 4 corpus evaluates AND
//! gathers at `Dual64`, with the value channel bit-identical to the
//! plain `f64` run — the dual contract, measured over every node
//! rather than only final bodies — and the memo behaves soundly when
//! a prior `Evaluation<Dual64>` is threaded.
//!
//! # What the value-channel digest reads
//!
//! For every node in evaluation order: the result's arm
//! (`Ok`/`Failed`/`Poisoned`), the payload's arm, and the payload's
//! geometry through the scalar's OWN bracket (`geom_core::Bounds` —
//! at `f64` the bracket ends are the value; at a dual they are the
//! value channel's, by the `Bounds for Dual` delegation): every body
//! point's coordinates, every datum frame, every profile loop's
//! vertices and bulges, plus arena counts. Declarations and mate
//! roles carry no `T` geometry and are pinned by arm tag. Equal
//! digests therefore assert bit-equal value channels at every
//! geometric datum the evaluation stores, node by node.
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

mod corpus;
mod fixture;

use corpus::{cone, documents, eval, failures};
use editor_core::eval::KeyHasher;
use editor_core::{
    BooleanValue, CancelToken, ContentKey, DatumValue, EvalOptions, EvalOutcome, Evaluation,
    NodeResult, SplitSide, ValuePayload, evaluate, product_recorded,
};
use geom_core::{Bounds, Decide, Dual64, Tol};
use topo::Body;

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

    /// Both bracket ends, as bits — the value channel exactly, at
    /// `f64` and at `Dual64` alike (module docs).
    fn scalar<T: Bounds>(&mut self, x: T) {
        self.u64(x.lo().to_bits());
        self.u64(x.hi().to_bits());
    }

    fn point3<T: Decide + Bounds>(&mut self, p: geom_core::Point3<T>) {
        self.scalar(p.x);
        self.scalar(p.y);
        self.scalar(p.z);
    }

    fn vec3<T: Decide + Bounds>(&mut self, v: geom_core::Vec3<T>) {
        self.scalar(v.x);
        self.scalar(v.y);
        self.scalar(v.z);
    }

    fn body<T: Decide + Bounds>(&mut self, body: &Body<T>) {
        self.u64(body.solids().count() as u64);
        self.u64(body.faces().count() as u64);
        self.u64(body.edges().count() as u64);
        self.u64(body.vertices().count() as u64);
        for (_k, p) in body.points() {
            self.point3(*p);
        }
    }
}

fn value_digest<T: Decide + Bounds>(ev: &Evaluation<T>) -> u64 {
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
                        d.vec3(*normal);
                    }
                    ValuePayload::Datum(DatumValue::Axis { origin, dir }) => {
                        d.u64(11);
                        d.point3(*origin);
                        d.vec3(*dir);
                    }
                    ValuePayload::Datum(DatumValue::Point { position }) => {
                        d.u64(12);
                        d.point3(*position);
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
                }
            }
        }
    }
    d.0
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
        assert_eq!(ev_d.outcome, EvalOutcome::Completed, "{}: outcome", doc.name);
        assert_eq!(ev_f.order, ev_d.order, "{}: evaluation order", doc.name);
        assert_eq!(
            value_digest(&ev_f),
            value_digest(&ev_d),
            "{}: the Dual64 value channel diverged from the f64 evaluation",
            doc.name
        );
    }
}

/// DL3 working at the gather: the product door opens at `Dual64` for
/// every document it opens at `f64` for — including the documents
/// whose faces the certified gates could not validate at a dual
/// (`die_fillet`'s trimmed blends, `loft_prism`'s NURBS walls,
/// `die_composed`'s torus band) — and gathers the same solids.
#[test]
fn every_f64_product_gathers_at_dual64_too() {
    let tol = Tol::witness();
    let mut gathered = 0usize;
    for doc in documents() {
        let ev_f = eval::<f64>(&doc.doc);
        let ev_d = eval::<Dual64>(&doc.doc);
        match product_recorded(&doc.doc, &ev_f, tol) {
            Ok(product_f) => {
                let product_d = product_recorded(&doc.doc, &ev_d, tol).unwrap_or_else(|e| {
                    panic!("{}: the product gathers at f64 but refused at Dual64: {e}", doc.name)
                });
                let (mut df, mut dd) = (Digest::new(), Digest::new());
                df.body(&product_f.body);
                dd.body(&product_d.body);
                assert_eq!(df.0, dd.0, "{}: product value channel", doc.name);
                gathered += 1;
            }
            Err(e) => {
                // A document with no body product refuses identically
                // at both scalars — never a Dual-only refusal.
                let dual = product_recorded(&doc.doc, &ev_d, tol);
                assert!(
                    dual.is_err(),
                    "{}: refused at f64 ({e}) but gathered at Dual64",
                    doc.name
                );
            }
        }
    }
    assert!(gathered > 0, "the corpus gathered no products at all");
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
        x.feed(&mut h);
        h.finish()
    };
    let seeded = key(Dual64::variable(3.5));
    let unseeded = key(Dual64::constant(3.5));
    assert_ne!(
        seeded, unseeded,
        "a seeded and an unseeded pass over the same value must not share a key"
    );
    // Position separates the channels: value and tangent bits cannot
    // alias each other.
    assert_ne!(
        key(Dual64::new(2.0, 5.0)),
        key(Dual64::new(5.0, 2.0)),
        "value and tangent channels must be position-separated in the feed"
    );
    // Bit-equal channels reproduce the key — the reuse direction.
    assert_eq!(seeded, key(Dual64::variable(3.5)));
}

/// DL2's `Dual<Interval>` instantiation, through the same generic
/// impls: the derivative-enclosure scalar walks the whole door too,
/// with its value channel bit-identical to the plain `Interval` run.
#[cfg(feature = "interval")]
#[test]
fn dual_interval_evaluates_with_the_interval_value_channel() {
    use geom_core::{DualInterval, Interval};
    for doc in documents() {
        if !matches!(doc.name, "die" | "loft_prism") {
            continue; // one closed-form and one NURBS-walled row
        }
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

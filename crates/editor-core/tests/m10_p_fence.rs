//! **The bit-identity fence.**
//!
//! The profile lift is additive machinery beside an unchanged build
//! path, and the fence is what makes "unchanged" a measurement rather
//! than an intention. With the lift OFF — the default, and the only
//! setting the build path ever uses — evaluating the whole Band 4
//! corpus must produce exactly the bits it produced before the lift
//! existed.
//!
//! The digest below is that measurement, and it is deliberately
//! computed from the observable evaluation and nothing else: every
//! node's outcome in id order, and for every body the bits of every
//! point it carries. It is a golden number in the ordinary sense — if
//! it moves, the question is whether the new behaviour is correct, not
//! how to get the old number back. What makes it useful HERE is that
//! this file uses no API the lift introduced, so the same digest can be
//! taken on a pre-lift tree and compared. It was: all three numbers
//! below are the ones a checkout of d0b64b7f — the lift's merge base —
//! produces from this same file, which is what "the build path did not
//! move" means here.
//!
//! `interval` and `probe` rows ride the same helper, so the fence
//! covers the three scalars the review names rather than the value lane
//! alone.
//!
//! **ITS PROBE-GATED CODE IS NOT EXECUTED BY CI**, and that is the
//! right disposition rather than an accident of a filter: the `probe`
//! row asserts the SAME digest the `f64` row does, because `Probe` is a
//! transparent `f64` whose every operation delegates exactly. It is
//! there to catch a telemetry scalar that started changing decisions —
//! a claim about `Probe`, not about this fence — and the fence itself
//! is carried by the `f64` and `interval` rows, which run on every
//! merge. Rostering it into the probe sweep would buy a third copy of a
//! number the sweep has no reason to be the keeper of.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::{
    CancelToken, ContentBits, EvalOptions, NodeResult, ValuePayload, evaluate,
};
use geom_core::{Decide, Tol};

/// A 128-bit FNV-1a over the evaluation's observable bits.
struct Digest {
    lo: u64,
    hi: u64,
}

impl Digest {
    fn new() -> Self {
        Self {
            lo: 0xcbf2_9ce4_8422_2325,
            hi: 0x9dc5_bb32_e0f7_1a49,
        }
    }

    fn u64(&mut self, x: u64) {
        for b in x.to_le_bytes() {
            self.lo = (self.lo ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
            self.hi = (self.hi ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    fn text(&mut self, s: &str) {
        for b in s.as_bytes() {
            self.u64(u64::from(*b));
        }
        self.u64(u64::MAX);
    }
}

/// The corpus's evaluation, digested at one scalar.
///
/// `bits` maps the scalar's coordinate to its exact representation.
/// Feeding through the caller keeps this file free of any per-scalar
/// door, which is what lets it compile against a pre-lift tree.
fn corpus_digest<T, F>(bits: F) -> (u64, u64)
where
    T: Decide + ContentBits + geom_core::Bounds + Send + Sync + topo::PropsQuadLane,
    F: Fn(&mut Digest, &geom_core::Point3<T>),
{
    let mut d = Digest::new();
    for doc in corpus::documents() {
        d.text(doc.name);
        let ev = evaluate::<T>(
            &doc.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        for (id, result) in ev.nodes.iter() {
            d.u64(id.0);
            match result {
                NodeResult::Poisoned { through } => {
                    d.text("poisoned");
                    d.u64(through.0);
                }
                NodeResult::Failed(e) => {
                    d.text("failed");
                    d.text(&e.to_string());
                }
                NodeResult::Ok(v) => {
                    d.text(v.payload.kind_name());
                    if let ValuePayload::Body(b) = &v.payload {
                        for (_, p) in b.points() {
                            bits(&mut d, p);
                        }
                    }
                }
            }
        }
    }
    (d.lo, d.hi)
}

fn f64_bits(d: &mut Digest, p: &geom_core::Point3<f64>) {
    for c in [p.x, p.y, p.z] {
        d.u64(c.to_bits());
    }
}

/// **The fence at `f64`.** Evaluating the corpus with the default
/// options produces exactly these bits.
#[test]
fn the_corpus_evaluation_is_bit_identical_at_f64() {
    let got = corpus_digest::<f64, _>(f64_bits);
    println!("m10-p fence f64: {got:016x?}");
    assert_eq!(
        got,
        (0xde11_5f28_f35f_e857, 0x6cfe_ba44_6867_dab3),
        "the corpus's f64 evaluation moved — see this file's header before \
         touching the number"
    );
}

/// The same fence at `Interval`, where the lift's second pass would
/// otherwise be tempting to leave on.
#[cfg(feature = "interval")]
#[test]
fn the_corpus_evaluation_is_bit_identical_at_interval() {
    use geom_core::{Bounds, Interval};
    let got = corpus_digest::<Interval, _>(|d, p| {
        for c in [p.x, p.y, p.z] {
            d.u64(c.lo().to_bits());
            d.u64(c.hi().to_bits());
        }
    });
    println!("m10-p fence interval: {got:016x?}");
    assert_eq!(
        got,
        (0xeaeb_0835_1c92_e041, 0x99a8_0e0b_f64c_aadd),
        "the corpus's Interval evaluation moved"
    );
}

/// And at `Probe`, the K-telemetry scalar.
#[cfg(feature = "probe")]
#[test]
fn the_corpus_evaluation_is_bit_identical_at_probe() {
    use geom_core::Probe;
    let got = corpus_digest::<Probe, _>(|d, p| {
        for c in [p.x, p.y, p.z] {
            d.u64(c.0.to_bits());
        }
    });
    println!("m10-p fence probe: {got:016x?}");
    // Probe is a transparent f64, so this is the f64 row's number and
    // must stay so: a Probe digest that drifted from it would mean the
    // telemetry scalar had started changing decisions.
    assert_eq!(
        got,
        (0xde11_5f28_f35f_e857, 0x6cfe_ba44_6867_dab3),
        "the corpus's Probe evaluation moved"
    );
}

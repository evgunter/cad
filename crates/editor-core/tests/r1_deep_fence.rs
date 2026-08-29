//! **R1's deep fence** — the unit's `m10_p_fence.rs` digests node
//! outcome kinds and BODY points only; a Profile node's own payload
//! bits are invisible to it except transitively. This instrument
//! digests the PROFILE payloads too (every validated loop's vertices,
//! bulges, joints and role), and prints rather than asserts: it is a
//! cross-revision differential instrument, run by the review on the
//! frozen head and on the merge base under the same file. Like the
//! unit's fence it uses no API the lift introduced, so it compiles on
//! a pre-lift tree.
//!
//! EVIDENCE-ONLY (one-shot comparison artefact per
//! memories/test-suite-cost.md): it expires with the review and is not
//! meant to merge.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::{CancelToken, ContentBits, EvalOptions, NodeResult, ValuePayload, evaluate};
use geom_core::{Decide, Tol};

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

fn deep_digest<T, F>(bits: F) -> (u64, u64)
where
    T: Decide + ContentBits + geom_core::Bounds + Send + Sync + topo::AtRestPolicy,
    F: Fn(&mut Digest, &T),
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
                    match &v.payload {
                        ValuePayload::Body(b) => {
                            for (_, p) in b.points() {
                                bits(&mut d, &p.x);
                                bits(&mut d, &p.y);
                                bits(&mut d, &p.z);
                            }
                        }
                        ValuePayload::Profile(p) => {
                            for lp in p.validated.loops() {
                                d.text("loop");
                                d.text(&format!("{:?}", lp.role()));
                                for vx in lp.vertices() {
                                    bits(&mut d, &vx.pos().x);
                                    bits(&mut d, &vx.pos().y);
                                    bits(&mut d, &vx.bulge());
                                }
                                for j in lp.tangent_joints() {
                                    d.u64(*j as u64);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    (d.lo, d.hi)
}

/// Prints the deep digest at `f64` — compare across revisions.
#[test]
fn r1_deep_fence_f64() {
    let got = deep_digest::<f64, _>(|d, c| d.u64(c.to_bits()));
    println!("R1 deep fence f64: {:016x} {:016x}", got.0, got.1);
}

/// And at `Interval`.
#[cfg(feature = "interval")]
#[test]
fn r1_deep_fence_interval() {
    use geom_core::{Bounds, Interval};
    let got = deep_digest::<Interval, _>(|d, c| {
        d.u64(c.lo().to_bits());
        d.u64(c.hi().to_bits());
    });
    println!("R1 deep fence interval: {:016x} {:016x}", got.0, got.1);
}

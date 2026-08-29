//! **R2 independent merge-base differential** (review probe, M10-P).
//!
//! An adversarial reviewer's own instrument for review claim 1: with
//! the lift OFF, is the corpus evaluation bit-identical to the merge
//! base? The unit ships `m10_p_fence.rs`, which asserts three golden
//! 128-bit FNV numbers over ONE combined channel. This file re-derives
//! the question with a different hash (`std`'s `DefaultHasher`) and,
//! more importantly, splits the observation into FOUR INDEPENDENT
//! CHANNELS, so a difference can be attributed rather than merely
//! detected:
//!
//! - `shape` — every node's outcome kind and, for bodies, the face and
//!   edge counts: topology at fixed coordinates, which a points-only
//!   digest cannot see.
//! - `geom` — every body point's bits.
//! - `profile` — the PROFILE payloads' canonical geometry: vertices,
//!   bulges, segment kinds, roles, declared joints. The shipped fence
//!   stops at `kind_name()` here, so the lift's own subject matter is
//!   outside it.
//! - `naming` — every node's `NamingKey`. The shipped fence drops it.
//!
//! Content keys are deliberately not digested anywhere: the unit bumps
//! the key format tag 2 → 3 on purpose.
//!
//! Like the shipped fence, this file uses NO API the lift introduced,
//! so the same source compiles and runs on a pre-lift checkout. It
//! PRINTS its numbers and asserts only self-consistency: the
//! comparison it exists for is made across two trees by the reviewer.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use editor_core::{CancelToken, ContentBits, EvalOptions, NodeResult, ValuePayload, evaluate};
use geom_core::{Decide, Tol};

/// Four independent SipHash channels.
struct D {
    shape: DefaultHasher,
    geom: DefaultHasher,
    profile: DefaultHasher,
    naming: DefaultHasher,
}

impl D {
    fn new() -> Self {
        Self {
            shape: DefaultHasher::new(),
            geom: DefaultHasher::new(),
            profile: DefaultHasher::new(),
            naming: DefaultHasher::new(),
        }
    }
    fn out(&self) -> [u64; 4] {
        [
            self.shape.finish(),
            self.geom.finish(),
            self.profile.finish(),
            self.naming.finish(),
        ]
    }
}

/// The corpus's evaluation, digested at one scalar into four channels.
fn corpus_digest<T, F, G>(body_bits: F, prof_bits: G) -> [u64; 4]
where
    T: Decide + ContentBits + geom_core::Bounds + Send + Sync + topo::AtRestPolicy,
    F: Fn(&mut DefaultHasher, &T),
    G: Fn(&mut DefaultHasher, &T),
{
    let mut d = D::new();
    for doc in corpus::documents() {
        doc.name.hash(&mut d.shape);
        let ev = evaluate::<T>(
            &doc.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        for (id, result) in ev.nodes.iter() {
            id.0.hash(&mut d.shape);
            match result {
                NodeResult::Poisoned { through } => {
                    "poisoned".hash(&mut d.shape);
                    through.0.hash(&mut d.shape);
                }
                NodeResult::Failed(e) => {
                    "failed".hash(&mut d.shape);
                    e.to_string().hash(&mut d.shape);
                }
                NodeResult::Ok(v) => {
                    v.payload.kind_name().hash(&mut d.shape);
                    format!("{:?}", v.naming_key).hash(&mut d.naming);
                    match &v.payload {
                        ValuePayload::Body(b) => {
                            let mut np = 0u64;
                            for (vid, p) in b.points() {
                                np += 1;
                                format!("{vid:?}").hash(&mut d.geom);
                                for c in [p.x, p.y, p.z] {
                                    body_bits(&mut d.geom, &c);
                                }
                            }
                            np.hash(&mut d.shape);
                            (b.faces().count() as u64).hash(&mut d.shape);
                            (b.edges().count() as u64).hash(&mut d.shape);
                        }
                        ValuePayload::Profile(pv) => {
                            for lp in pv.validated.loops() {
                                format!("{:?}", lp.role()).hash(&mut d.profile);
                                (lp.vertices().len() as u64).hash(&mut d.profile);
                                for vtx in lp.vertices() {
                                    prof_bits(&mut d.profile, &vtx.pos().x);
                                    prof_bits(&mut d.profile, &vtx.pos().y);
                                    prof_bits(&mut d.profile, &vtx.bulge());
                                }
                                for s in lp.segments() {
                                    format!("{:?}", core::mem::discriminant(&s.kind))
                                        .hash(&mut d.profile);
                                }
                                for j in lp.tangent_joints() {
                                    (*j as u64).hash(&mut d.profile);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    d.out()
}

fn show(label: &str, v: [u64; 4]) {
    println!(
        "R2-FENCE {label}: shape={:#018x} geom={:#018x} profile={:#018x} naming={:#018x}",
        v[0], v[1], v[2], v[3]
    );
}

/// **R2's fence at `f64`.** Prints; the comparison is across trees.
#[test]
fn r2_corpus_digest_at_f64() {
    let got = corpus_digest::<f64, _, _>(|h, c| c.to_bits().hash(h), |h, c| c.to_bits().hash(h));
    show("f64", got);
    let again = corpus_digest::<f64, _, _>(|h, c| c.to_bits().hash(h), |h, c| c.to_bits().hash(h));
    assert_eq!(got, again, "the corpus digest is not deterministic");
}

/// **R2's fence at `Interval`.**
#[cfg(feature = "interval")]
#[test]
fn r2_corpus_digest_at_interval() {
    use geom_core::{Bounds, Interval};
    let f = |h: &mut DefaultHasher, c: &Interval| {
        c.lo().to_bits().hash(h);
        c.hi().to_bits().hash(h);
    };
    let got = corpus_digest::<Interval, _, _>(f, f);
    show("interval", got);
}

/// **R2's fence at `Probe`.**
#[cfg(feature = "probe")]
#[test]
fn r2_corpus_digest_at_probe() {
    use geom_core::Probe;
    let f = |h: &mut DefaultHasher, c: &Probe| c.0.to_bits().hash(h);
    let got = corpus_digest::<Probe, _, _>(f, f);
    show("probe", got);
}

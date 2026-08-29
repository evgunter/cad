//! **R1 review probe — the merge-base differential** (M10-DI, PR
//! #1154, claim 1). EVIDENCE-ONLY: this suite asserts nothing about a
//! fixed constant. It PRINTS one digest line per corpus document per
//! certifying scalar, and the comparison it serves is taken by the
//! reviewer BETWEEN TWO BUILDS (the unit's merge base `f2fd13cc` and
//! its head `2435345d`) — the cross-build shape
//! `memories/test-suite-cost.md` names.
//!
//! Deliberately deeper than the unit's own digest: besides evaluation
//! order, node arms, datum frames, profile loops, body counts and
//! vertex points, it samples every stored SURFACE on a fixed (u, v)
//! lattice and every stored CURVE carrier at fixed parameters — the
//! `T`-parameterized geometry the arena holds that a points-only
//! digest never reads. It also digests the product-gather door's
//! verdict (the `Ok`/`Err` arm and the refusal's `Debug` text), which
//! is the exact site the DL3 policy swap rewired.
//!
//! It expires with its comparison (`test-suite-cost`): delete after
//! the M10-DI review; nothing schedules a future run of it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use corpus::{documents, eval};
use editor_core::{
    BooleanValue, DatumValue, Evaluation, NodeResult, SplitSide, ValuePayload, product_recorded,
};
use geom_core::{Bounds, Decide, Real as _, Tol};
use topo::Body;

/// FNV-1a 64 over whatever is fed. Not a content key — a probe digest.
pub struct D(pub u64);

impl D {
    pub fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }

    pub fn u64(&mut self, x: u64) {
        for b in x.to_le_bytes() {
            self.0 ^= u64::from(b);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    pub fn s(&mut self, t: &str) {
        self.u64(t.len() as u64);
        for b in t.as_bytes() {
            self.u64(u64::from(*b));
        }
    }

    /// A scalar through its own bracket — the value channel at every
    /// scalar this probe instantiates (`f64`, `Interval`, `Dual`).
    pub fn sc<T: Bounds>(&mut self, x: T) {
        self.u64(x.lo().to_bits());
        self.u64(x.hi().to_bits());
    }

    pub fn p3<T: Decide + Bounds>(&mut self, p: geom_core::Point3<T>) {
        self.sc(p.x);
        self.sc(p.y);
        self.sc(p.z);
    }

    pub fn v3<T: Decide + Bounds>(&mut self, v: geom_core::Vec3<T>) {
        self.sc(v.x);
        self.sc(v.y);
        self.sc(v.z);
    }
}

/// The (u, v) lattice every surface is sampled on, and the parameters
/// every curve carrier is sampled at. Fixed, dyadic, and off the
/// origin so a frame's translation and its rotation both move a
/// sample.
const UV: [f64; 5] = [-1.5, -0.25, 0.0, 0.375, 2.0];

/// The whole `T`-geometry of a body: counts, every stored point, every
/// stored SURFACE sampled on the lattice, every stored CURVE carrier
/// sampled at the lattice parameters plus its stored parameter pair.
pub fn body_deep<T>(d: &mut D, body: &Body<T>)
where
    T: Decide + Bounds + geom_core::SpanLocate,
{
    d.u64(body.solids().count() as u64);
    d.u64(body.faces().count() as u64);
    d.u64(body.edges().count() as u64);
    d.u64(body.vertices().count() as u64);
    for (_k, p) in body.points() {
        d.p3(*p);
    }
    for (_k, s) in body.surfaces() {
        for u in UV {
            for v in UV {
                d.p3(s.eval(T::from_f64(u), T::from_f64(v)));
            }
        }
    }
    for (_k, c) in body.curves() {
        match c.certified() {
            None => d.u64(0),
            Some(ec) => {
                d.u64(1);
                let (t0, t1) = ec.params();
                d.sc(t0);
                d.sc(t1);
                for t in UV {
                    d.p3(ec.carrier().eval(T::from_f64(t)));
                }
            }
        }
    }
}

/// Every node of the evaluation, in order, with its full `T` payload.
pub fn eval_deep<T>(ev: &Evaluation<T>) -> u64
where
    T: Decide + Bounds + geom_core::SpanLocate,
{
    let mut d = D::new();
    for &id in &ev.order {
        d.u64(id.0);
        match ev.result(id) {
            None => d.u64(0),
            Some(NodeResult::Failed(e)) => {
                d.u64(1);
                d.s(&format!("{e:?}"));
            }
            Some(NodeResult::Poisoned { through }) => {
                d.u64(2);
                d.s(&format!("{through:?}"));
            }
            Some(NodeResult::Ok(v)) => {
                d.u64(3);
                match &v.payload {
                    ValuePayload::Datum(DatumValue::Plane { origin, normal }) => {
                        d.u64(10);
                        d.p3(*origin);
                        d.v3(*normal);
                    }
                    ValuePayload::Datum(DatumValue::Axis { origin, dir }) => {
                        d.u64(11);
                        d.p3(*origin);
                        d.v3(*dir);
                    }
                    ValuePayload::Datum(DatumValue::Point { position }) => {
                        d.u64(12);
                        d.p3(*position);
                    }
                    ValuePayload::Profile(p) => {
                        d.u64(13);
                        for lp in p.validated.loops() {
                            d.u64(lp.vertices().len() as u64);
                            for vx in lp.vertices() {
                                d.sc(vx.pos().x);
                                d.sc(vx.pos().y);
                                d.sc(vx.bulge());
                            }
                        }
                    }
                    ValuePayload::Body(b) => {
                        d.u64(14);
                        body_deep(&mut d, b);
                    }
                    ValuePayload::Boolean(BooleanValue::Empty) => d.u64(15),
                    ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
                        d.u64(16);
                        body_deep(&mut d, body);
                    }
                    ValuePayload::Split { above, below } => {
                        d.u64(17);
                        for side in [above, below] {
                            match side {
                                SplitSide::Empty => d.u64(0),
                                SplitSide::Body(b) => {
                                    d.u64(1);
                                    body_deep(&mut d, b);
                                }
                            }
                        }
                    }
                    ValuePayload::Instances(bodies) => {
                        d.u64(18);
                        d.u64(bodies.len() as u64);
                        for b in bodies {
                            body_deep(&mut d, b);
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

/// The product-gather door's verdict, digested: the arm and, on a
/// refusal, its exact `Debug` text — so a policy swap that changed
/// WHICH refusal a certifying scalar reports would move this number.
/// Takes the already-gathered `Result` so this helper carries no
/// scalar-policy bound of its own: the bound's SPELLING is what the
/// PR changes, and this file must compile unmodified at both
/// revisions.
pub fn product_deep<T>(r: Result<editor_core::Product<T>, editor_core::ProductError>) -> u64
where
    T: Decide + Bounds + geom_core::SpanLocate,
{
    let mut d = D::new();
    match r {
        Ok(p) => {
            d.u64(1);
            body_deep(&mut d, &p.body);
        }
        Err(e) => {
            d.u64(0);
            d.s(&format!("{e:?}"));
        }
    }
    d.0
}

/// One concrete report per scalar — MONOMORPHIC on purpose: naming a
/// generic bound here would name the very trait whose spelling this
/// differential is testing for behavioral invisibility.
macro_rules! report_for {
    ($name:ident, $scalar:ty, $label:literal) => {
        #[test]
        fn $name() {
            for doc in documents() {
                let ev = eval::<$scalar>(&doc.doc);
                let prod = product_recorded(&doc.doc, &ev, Tol::witness());
                println!(
                    "R1MBDIFF {} {} eval={:016x} product={:016x}",
                    $label,
                    doc.name,
                    eval_deep(&ev),
                    product_deep(prod),
                );
            }
        }
    };
}

report_for!(r1_mb_diff_f64, f64, "f64");

#[cfg(feature = "interval")]
report_for!(r1_mb_diff_interval, geom_core::Interval, "interval");

#[cfg(feature = "probe")]
report_for!(r1_mb_diff_probe, geom_core::Probe, "probe");

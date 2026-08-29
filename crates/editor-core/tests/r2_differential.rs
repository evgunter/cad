//! **EVIDENCE-ONLY, one-shot comparison artefact** (R2's M10-DI
//! review, claim 1): dumps a per-document, per-scalar deep digest of
//! the whole corpus evaluation (points, curve carriers, surface
//! geometry, per-node arms) plus the `product_recorded` and `assemble`
//! outcomes, at `f64`, `Interval` and `Probe` — run once at the unit's
//! merge base and once at the frozen head, then diffed byte-for-byte.
//! Per the test-suite-cost memory this artefact EXPIRES with its
//! comparison: it is `#[ignore]`d, gated on an env var, and recorded
//! on the review branch only as the review's reproducibility record.
//! Blind spots, disclosed: Interval decorations are read only through
//! `Bounds::lo/hi` (endpoint bits) and refusal payloads through their
//! `Debug` text; `Surface::Approx` internals are pinned by tag.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use std::io::Write as _;

use corpus::{documents, eval};
use editor_core::{
    BooleanValue, DatumValue, Evaluation, NodeResult, SplitSide, ValuePayload, assemble,
    product_recorded,
};
use geom::{Curve3, Surface};
use geom_core::{Bounds, Decide, Tol};
use topo::{Body, CurveGeom};

struct Fnv(u64);

impl Fnv {
    fn new() -> Self {
        Self(14_695_981_039_346_656_037)
    }
    fn u64(&mut self, x: u64) {
        for b in x.to_le_bytes() {
            self.0 = (self.0 ^ u64::from(b)).wrapping_mul(1_099_511_628_211);
        }
    }
    fn bytes(&mut self, s: &[u8]) {
        self.u64(s.len() as u64);
        for b in s {
            self.u64(u64::from(*b));
        }
    }
    fn s<T: Bounds>(&mut self, x: T) {
        self.u64(x.lo().to_bits());
        self.u64(x.hi().to_bits());
    }
    fn p3<T: Decide + Bounds>(&mut self, p: &geom_core::Point3<T>) {
        self.s(p.x);
        self.s(p.y);
        self.s(p.z);
    }
    fn v3<T: Decide + Bounds>(&mut self, v: &geom_core::Vec3<T>) {
        self.s(v.x);
        self.s(v.y);
        self.s(v.z);
    }

    fn curve3<T: Decide + Bounds>(&mut self, c: &Curve3<T>) {
        match c {
            Curve3::Line { origin, dir } => {
                self.u64(40);
                self.p3(origin);
                self.v3(dir);
            }
            Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => {
                self.u64(41);
                self.p3(center);
                self.v3(axis);
                self.s(*radius);
                self.v3(u_ref);
            }
            Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } => {
                self.u64(42);
                self.p3(center);
                self.v3(axis);
                self.s(*major);
                self.s(*minor);
                self.v3(u_ref);
            }
            Curve3::Nurbs(n) => {
                self.u64(43);
                self.u64(n.control().len() as u64);
                for p in n.control() {
                    self.p3(p);
                }
                for w in n.weights() {
                    self.u64(w.to_bits());
                }
                for k in n.knots().knots() {
                    self.u64(k.to_bits());
                }
            }
        }
    }

    fn surface<T: Decide + Bounds>(&mut self, s: &Surface<T>) {
        match s {
            Surface::Plane {
                origin,
                normal,
                u_ref,
            } => {
                self.u64(60);
                self.p3(origin);
                self.v3(normal);
                self.v3(u_ref);
            }
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => {
                self.u64(61);
                self.p3(origin);
                self.v3(axis);
                self.s(*radius);
                self.v3(u_ref);
            }
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => {
                self.u64(62);
                self.p3(apex);
                self.v3(axis);
                self.s(*half_angle);
                self.v3(u_ref);
            }
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => {
                self.u64(63);
                self.p3(center);
                self.s(*radius);
                self.v3(axis);
                self.v3(u_ref);
            }
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => {
                self.u64(64);
                self.p3(center);
                self.v3(axis);
                self.s(*major_radius);
                self.s(*minor_radius);
                self.v3(u_ref);
            }
            Surface::Nurbs(n) => {
                self.u64(65);
                let (cu, cv) = n.control_counts();
                self.u64(cu as u64);
                self.u64(cv as u64);
                for p in n.control() {
                    self.p3(p);
                }
                for w in n.weights() {
                    self.u64(w.to_bits());
                }
                for k in n.knots_u().knots() {
                    self.u64(k.to_bits());
                }
                for k in n.knots_v().knots() {
                    self.u64(k.to_bits());
                }
            }
            Surface::Approx(_) => self.u64(66),
        }
    }

    fn body<T: Decide + Bounds>(&mut self, body: &Body<T>) {
        self.u64(body.solids().count() as u64);
        self.u64(body.faces().count() as u64);
        self.u64(body.edges().count() as u64);
        self.u64(body.vertices().count() as u64);
        for (_k, p) in body.points() {
            self.p3(p);
        }
        for (_k, c) in body.curves() {
            match c {
                CurveGeom::Certified(ec) => {
                    self.u64(50);
                    self.curve3(ec.carrier());
                    let (t0, t1) = ec.params();
                    self.s(t0);
                    self.s(t1);
                }
                CurveGeom::NullScaffold(_) => self.u64(51),
            }
        }
        for (_k, s) in body.surfaces() {
            self.surface(s);
        }
    }
}

fn deep_digest<T: Decide + Bounds>(ev: &Evaluation<T>) -> u64 {
    let mut d = Fnv::new();
    for &id in &ev.order {
        d.u64(id.0);
        match ev.result(id) {
            None => d.u64(0),
            Some(NodeResult::Failed(e)) => {
                d.u64(1);
                d.bytes(format!("{e:?}").as_bytes());
            }
            Some(NodeResult::Poisoned { through }) => {
                d.u64(2);
                d.bytes(format!("{through:?}").as_bytes());
            }
            Some(NodeResult::Ok(v)) => {
                d.u64(3);
                match &v.payload {
                    ValuePayload::Datum(DatumValue::Plane { origin, normal }) => {
                        d.u64(10);
                        d.p3(origin);
                        d.v3(normal);
                    }
                    ValuePayload::Datum(DatumValue::Axis { origin, dir }) => {
                        d.u64(11);
                        d.p3(origin);
                        d.v3(dir);
                    }
                    ValuePayload::Datum(DatumValue::Point { position }) => {
                        d.u64(12);
                        d.p3(position);
                    }
                    ValuePayload::Profile(p) => {
                        d.u64(13);
                        for lp in p.validated.loops() {
                            d.u64(lp.vertices().len() as u64);
                            for v in lp.vertices() {
                                d.s(v.pos().x);
                                d.s(v.pos().y);
                                d.s(v.bulge());
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

/// The dump row. Run with `R2_DIFF_OUT=/path cargo test ... -- --ignored r2_differential`.
#[test]
#[ignore = "one-shot cross-revision comparison artefact; see module docs"]
fn r2_differential_dump() {
    let path = std::env::var("R2_DIFF_OUT").expect("set R2_DIFF_OUT to the output file path");
    let mut out = std::fs::File::create(&path).unwrap();
    let tol = Tol::witness();
    for doc in documents() {
        let ev = eval::<f64>(&doc.doc);
        let p = outcome(product_recorded(&doc.doc, &ev, tol).map(|p| p.body));
        let a = outcome(assemble(&doc.doc, &ev, tol).map(|a| a.body));
        writeln!(
            out,
            "{} f64 eval={:016x} product={p} assemble={a}",
            doc.name,
            deep_digest(&ev)
        )
        .unwrap();
        #[cfg(feature = "interval")]
        {
            let ev = eval::<geom_core::Interval>(&doc.doc);
            let p = outcome(product_recorded(&doc.doc, &ev, tol).map(|p| p.body));
            let a = outcome(assemble(&doc.doc, &ev, tol).map(|a| a.body));
            writeln!(
                out,
                "{} interval eval={:016x} product={p} assemble={a}",
                doc.name,
                deep_digest(&ev)
            )
            .unwrap();
        }
        #[cfg(feature = "probe")]
        {
            let ev = eval::<geom_core::Probe>(&doc.doc);
            let p = outcome(product_recorded(&doc.doc, &ev, tol).map(|p| p.body));
            let a = outcome(assemble(&doc.doc, &ev, tol).map(|a| a.body));
            writeln!(
                out,
                "{} probe eval={:016x} product={p} assemble={a}",
                doc.name,
                deep_digest(&ev)
            )
            .unwrap();
        }
    }
}

fn outcome<T: Decide + Bounds, E: std::fmt::Debug>(r: Result<Body<T>, E>) -> String {
    match r {
        Ok(body) => {
            let mut f = Fnv::new();
            f.body(&body);
            format!("ok:{:016x}", f.0)
        }
        Err(e) => {
            let mut f = Fnv::new();
            f.bytes(format!("{e:?}").as_bytes());
            format!("err:{:016x}", f.0)
        }
    }
}

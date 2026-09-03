//! **R2's adversarial consumer probes for M10-DI** (the Dual contract
//! implementation, PR #1154, frozen head `2435345d`). Independent
//! derivations — nothing here re-reads the unit's own digest; the deep
//! digest below is a second, wider instrument derived from the `Body`
//! arenas directly.
//!
//! What this suite adds over `m10_di_dual_corpus.rs`:
//!
//! 1. **A deeper value-channel instrument.** The unit's digest reads
//!    arena COUNTS plus vertex POINTS; a `Body<T>` also stores
//!    `CurveGeom<T>` carriers (line/circle/ellipse/NURBS control
//!    points, certified params) and `Surface<T>` geometry (plane
//!    frames, quadric parameters, NURBS control nets) — all of it
//!    T-valued and none of it read by the unit's row. The deep digest
//!    here walks those arenas too, so a value-channel divergence
//!    confined to, say, a loft wall's control net goes red HERE even
//!    though the unit's row would stay green.
//! 2. **A no-contamination oracle for threaded priors.** The unit
//!    asserts reuse/recompute COUNTS after a parameter bump; equal
//!    counts cannot see a node that reused the WRONG entry. The row
//!    here asserts the whole after-eval is deep-digest-identical to a
//!    fresh evaluation of the bumped document.
//! 3. **The census door at `Dual64`.** `assemble` is the third DL3
//!    gating site (`AtRestPolicy::gate_at_rest_declared`) and the
//!    unit's suites never drive it at a dual; the row here does, over
//!    every corpus document, refusal arms compared with `f64`.
//! 4. **An own-authored consumer document** built through the public
//!    edit API (arc profile → extrude → tilted split → boolean-free
//!    product) and driven at `Dual64` as an API consumer would.
//!
//! Evidence-only rows are marked in their doc comments; they assert
//! current facts a reviewer needed visible, and may be retired with
//! the review per the standing reviewer-suite policy. No fuzzing: every
//! row is a static-fixture enumeration over the committed corpus
//! (test-suite-cost: a witness you can write down is not a search).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use corpus::{documents, eval, failures};
use editor_core::eval::KeyHasher;
use editor_core::{
    AssertionVerdict, BooleanValue, CancelToken, ContentKey, Datum, DatumValue, Dimension,
    EvalOptions, Evaluation, LoopProgram, Node, NodeResult, ProductError, ProfileProgram,
    SplitSide, ValuePayload, assemble, evaluate, product_recorded,
};
use geom::{Curve3, Surface};
use geom_core::{Bounds, Decide, Dual64, Tol};
use profile::SketchPlane;
use topo::{Body, CurveGeom};

/// FNV-1a 64 (independent constants derivation: offset basis
/// 14695981039346656037, prime 1099511628211).
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
    /// A scalar's value channel, as bits, read through the scalar's
    /// own bracket: at `f64` lo = hi = the value; at `Dual` the
    /// bracket is the value channel's by the `Bounds for Dual`
    /// delegation. Both ends fed, so an asymmetric bracket cannot
    /// hide either.
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
            // Blind spot, disclosed: the approx payload's certificate
            // internals are not walked; the arm is pinned by tag.
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

/// The deep per-node value-channel digest (module docs, instrument 1).
fn deep_digest<T: Decide + Bounds>(ev: &Evaluation<T>) -> u64 {
    let mut d = Fnv::new();
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
                        d.p3(origin);
                        d.v3(&normal.get());
                    }
                    ValuePayload::Datum(DatumValue::Axis { origin, dir }) => {
                        d.u64(11);
                        d.p3(origin);
                        d.v3(&dir.get());
                    }
                    ValuePayload::Datum(DatumValue::Point { position }) => {
                        d.u64(12);
                        d.p3(position);
                    }
                    ValuePayload::Datum(DatumValue::Frame { origin, u, v }) => {
                        d.u64(23);
                        d.p3(origin);
                        d.v3(&u.get());
                        d.v3(&v.get());
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
                        d.s(*value);
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
                                d.s(*measured);
                                d.s(*bound);
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

/// Instrument 1: the value channel is bit-identical to `f64` in the
/// CURVE and SURFACE arenas too — the T-valued geometry the unit's
/// digest does not read (NURBS control nets on `loft_prism` and
/// `die_composed`'s torus band included).
#[test]
fn deep_value_channel_identity_f64_vs_dual64_including_carrier_arenas() {
    for doc in documents() {
        let ev_f = eval::<f64>(&doc.doc);
        let ev_d = eval::<Dual64>(&doc.doc);
        assert!(
            failures(&ev_d).is_empty(),
            "{}: Dual64 evaluation not green",
            doc.name
        );
        assert_eq!(
            deep_digest(&ev_f),
            deep_digest(&ev_d),
            "{}: value channel diverged from f64 in the deep digest \
             (points, curve carriers, surface geometry)",
            doc.name
        );
    }
}

/// Instrument 1 at `Dual<Interval>`, one closed-form and one
/// NURBS-walled document (mirrors the unit's own draw).
#[cfg(feature = "interval")]
#[test]
fn deep_value_channel_identity_interval_vs_dual_interval() {
    use geom_core::{DualInterval, Interval};
    // A LOUD row list (the unit suite's shape): a renamed document
    // fails the lookup rather than silently emptying the row.
    let docs = documents();
    for name in ["die", "loft_prism"] {
        let doc = docs
            .iter()
            .find(|d| d.name == name)
            .unwrap_or_else(|| panic!("row `{name}` is not in the corpus"));
        let ev_i = eval::<Interval>(&doc.doc);
        let ev_d = eval::<DualInterval>(&doc.doc);
        assert_eq!(
            deep_digest(&ev_i),
            deep_digest(&ev_d),
            "{}: Dual<Interval> value channel diverged in the deep digest",
            doc.name
        );
    }
}

/// Instrument 2: a threaded prior can never CONTAMINATE values — after
/// the corpus bump edit at `Dual64`, the incremental evaluation is
/// deep-digest-identical to a from-scratch evaluation of the bumped
/// document. (The unit's rows assert reuse/recompute counts, which a
/// wrong-entry reuse with the right count would satisfy.)
#[test]
fn threaded_prior_never_contaminates_values_at_dual64() {
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
        let fresh = eval::<Dual64>(&bumped);
        assert_eq!(
            deep_digest(&after),
            deep_digest(&fresh),
            "{}: incremental result diverged from a fresh evaluation",
            doc.name
        );
    }
}

/// Instrument 3: the census door (`assemble`, DL3's third gating site,
/// `AtRestPolicy::gate_at_rest_declared`) opens at `Dual64` exactly
/// where it opens at `f64`, over the whole corpus — including the
/// mate-carrying documents — and refuses on the same documents where
/// it refuses.
#[test]
fn assemble_census_door_matches_f64_at_dual64() {
    let tol = Tol::witness();
    let mut assembled = 0usize;
    let mut divergent: Vec<&'static str> = Vec::new();
    for doc in documents() {
        let ev_f = eval::<f64>(&doc.doc);
        let ev_d = eval::<Dual64>(&doc.doc);
        match assemble(&doc.doc, &ev_f, tol) {
            Ok(asm_f) => {
                let asm_d = assemble(&doc.doc, &ev_d, tol).unwrap_or_else(|e| {
                    panic!("{}: assembles at f64 but refused at Dual64: {e}", doc.name)
                });
                let (mut a, mut b) = (Fnv::new(), Fnv::new());
                a.body(&asm_f.body);
                b.body(&asm_d.body);
                assert_eq!(a.0, b.0, "{}: assembly value channel", doc.name);
                assembled += 1;
            }
            Err(e) => match assemble(&doc.doc, &ev_d, tol) {
                Err(_) => {}
                Ok(_) => {
                    eprintln!(
                        "DIVERGENCE {}: f64 refuses [{e:?}] but Dual64 assembles",
                        doc.name
                    );
                    divergent.push(doc.name);
                }
            },
        }
    }
    assert!(assembled > 0, "the corpus assembled nothing at all");
    // THE PINNED DL3 CONSEQUENCE (R2's review finding, on record): at
    // a dual the census door is structurally absent, so a document the
    // f64 census REFUSES (an at-rest validity verdict about the
    // geometry, not a "dual may not certify" refusal) assembles green
    // at `Dual64`. The E4 pairing discipline — a dual pass rides
    // BESIDE a validated base-scalar run — is what makes this sound,
    // and nothing in the type system enforces that pairing at this
    // public door. This assertion pins the divergence set so a change
    // in either direction (a gate returning to duals, or new corpus
    // members joining the divergence) is loud rather than silent.
    // Observed 2026-08-29 at head 2435345d: `heat_sink` and
    // `kitchen_sink` refuse at f64 with `UndeclaredContact` /
    // `CensusUndecidable` findings; `cut_cylinder` with the split
    // sides' undeclared section contacts. All three assemble green at
    // `Dual64`.
    assert_eq!(
        divergent,
        vec!["heat_sink", "kitchen_sink", "cut_cylinder"],
        "the f64-refuses/Dual64-assembles divergence set changed"
    );
}

/// Instrument 4: an own-authored consumer document through the public
/// edit API — an arc-walled disc extruded and split by a tilted plane
/// (curved section edges), driven at `Dual64` the way an API consumer
/// would, gathered through `product_recorded`, value channel checked
/// against `f64` with the deep digest.
#[test]
fn own_document_builds_at_dual64_with_f64_value_channel() {
    let mut r = fixture::Recorder::new();
    let disc = LoopProgram::circle(0.0, 0.0, 0.75).unwrap();
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![disc],
    }));
    let puck = r.insert(Node::Extrude {
        profile,
        distance: fixture::len(0.5),
    });
    let tool = r.insert(Node::Datum(Datum::Plane {
        origin: [fixture::len(0.1), fixture::len(0.0), fixture::len(0.25)],
        normal: [
            fixture::scl(0.25f64.sin()),
            fixture::scl(0.0),
            fixture::scl(0.25f64.cos()),
        ],
    }));
    let _split = r.insert(Node::Split { target: puck, tool });
    let doc = r.doc;

    let ev_f = eval::<f64>(&doc);
    let ev_d = eval::<Dual64>(&doc);
    assert!(
        failures(&ev_d).is_empty(),
        "own document not green at Dual64: {:?}",
        failures(&ev_d)
    );
    assert_eq!(deep_digest(&ev_f), deep_digest(&ev_d), "own doc channel");

    let tol = Tol::witness();
    let p_f = product_recorded(&doc, &ev_f, tol);
    let p_d = product_recorded(&doc, &ev_d, tol);
    // The two arms agree EXCEPT where the certified at-rest gate
    // decides: `SolidInvalid` is minted only by `gate_at_rest`, which
    // is structurally absent at `Dual` (DUAL-DESIGN DL3), so an f64
    // gate refusal — which this document produces at tight ε, where
    // `props_quad_converged` escalates on the arc-walled split — is
    // the RATIFIED divergence, and Dual64 gathering Ok beside it is
    // the design working. Any other refusal runs identical code on a
    // bit-identical value channel and must agree exactly.
    match &p_f {
        Ok(_) => assert!(
            p_d.is_ok(),
            "f64 gathered but Dual64 refused: {:?}",
            p_d.as_ref().err()
        ),
        Err(ProductError::SolidInvalid { .. }) => assert!(
            p_d.is_ok(),
            "the at-rest gate is structurally absent at Dual64 (DL3), so an \
             f64 gate refusal must leave the Dual64 gather Ok; got {:?}",
            p_d.as_ref().err()
        ),
        Err(other) => assert_eq!(
            format!("{other:?}"),
            format!("{:?}", p_d.as_ref().err().unwrap()),
            "non-gate refusals must agree across arms"
        ),
    }
}

/// EVIDENCE-ONLY (review record; retire freely): the DIRECT validation
/// door stays spellable at a dual — `AtRestPolicy` governs only the
/// evaluation service's gates. `topo::validate_geometric::<Dual64>`
/// still runs tier 1–3 with the quad lane's refusing dual arms: on a
/// NURBS-walled body it refuses typed (never silently), and on an
/// all-planar body it returns `Ok(())` — a tier-3 pass computed at a
/// dual, sound by value-channel delegation but indistinguishable at
/// the call site from a certified-scalar pass. Beside it,
/// `AtRestPolicy::gate_at_rest::<Dual64>` returns
/// `Ok(NotRunAtThisScalar)` — the fix pass made the two adjacent
/// doors' successes different WORDS (`topo::AtRestOutcome`), which is
/// what this row originally put on record.
#[test]
fn direct_validation_door_behavior_at_dual64() {
    let tol = Tol::witness();
    let docs = documents();
    let planar = docs.iter().find(|d| d.name == "die").unwrap();
    let ev = eval::<Dual64>(&planar.doc);
    let body = corpus::body_of(&ev, planar.result.unwrap());
    // `die` carries filleted (cylindrical/spherical) faces — curved but
    // closed-form; what matters here is that the door a dual CAN take
    // runs and answers. That door is the structural half: the composed
    // entry carries the +V invariant's certified bound and cannot be
    // called at a dual at all.
    let direct = topo::validate_geometric_structural(body, tol);
    let policy = <Dual64 as topo::AtRestPolicy>::gate_at_rest(body, tol);
    assert_eq!(
        policy,
        Ok(topo::AtRestOutcome::NotRunAtThisScalar),
        "the policy arm must say it did not run — never a grant"
    );
    if let Some(nurbs) = docs.iter().find(|d| d.name == "loft_prism")
        && let Some(result) = nurbs.result
    {
        let ev_n = eval::<Dual64>(&nurbs.doc);
        let body_n = corpus::body_of(&ev_n, result);
        // MEASURED, and the reverse of what this row asserted before the
        // validator split: a NURBS-walled body PASSES the structural
        // half at a dual. Its refusal was the +V invariant's
        // `VolumeUncomputable`, raised by the dual's refusing quadrature
        // arm, and the split moved that invariant WHOLE — closed form
        // included — into the certified half, so the structural door
        // reports no orientation verdict of any kind. Nothing else the
        // structural checks consult refuses this body.
        assert_eq!(
            topo::validate_geometric_structural(body_n, tol),
            Ok(()),
            "a NURBS-walled body passes the door a dual can take; its refusal was \
             the certified half's"
        );
    }
    // Record the die outcome either way — the row's value is the pair
    // of spellings being on the record, not a particular verdict.
    eprintln!("direct validate_geometric_structural::<Dual64>(die) = {direct:?}");
}

/// EVIDENCE-ONLY (review record; retire freely): `ContentBits::feed`
/// has no per-scalar domain separation — an `f64` pass feeding `a`
/// then `b` produces the SAME key bytes as one `Dual64` feeding
/// `Dual(a, b)`. That is a fact, not a defect: a memo lives inside one
/// monomorphic `Evaluation<T>` and a prior of a different `T` is
/// unrepresentable at the type level, so no lookup can cross the
/// aliased streams. This row makes the fact visible so the day keys
/// ever persist or cross scalars (the memo module's own "revisit
/// alongside PR 6 persistence" note) the collision is on record.
#[test]
fn cross_scalar_feed_streams_alias_and_the_type_system_is_the_guard() {
    use editor_core::ContentBits;
    let key_f64_pair = {
        let mut h = KeyHasher::new();
        2.0f64.feed(&mut h);
        5.0f64.feed(&mut h);
        h.finish()
    };
    let key_dual = {
        let mut h = KeyHasher::new();
        Dual64::new(2.0, 5.0).feed(&mut h);
        h.finish()
    };
    assert_eq!(
        key_f64_pair, key_dual,
        "the streams alias today; if this row ever goes red, the feed \
         gained framing and this record is stale"
    );
    // Within one scalar the fixed width IS the framing: no dual's
    // 16-byte feed can be a prefix or suffix confusion of another's.
    let a: ContentKey = {
        let mut h = KeyHasher::new();
        Dual64::new(2.0, 5.0).feed(&mut h);
        Dual64::new(7.0, 9.0).feed(&mut h);
        h.finish()
    };
    let b: ContentKey = {
        let mut h = KeyHasher::new();
        Dual64::new(2.0, 7.0).feed(&mut h);
        Dual64::new(5.0, 9.0).feed(&mut h);
        h.finish()
    };
    assert_ne!(a, b, "channel positions must not be re-groupable");
}

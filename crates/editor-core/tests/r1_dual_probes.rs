//! **R1 review probes for M10-DI** (PR #1154, frozen head `2435345d`).
//! Independent derivations, not re-readings of the unit's own rows.
//!
//! Four things this suite does that the unit's suites do not:
//!
//! 1. **A DEEPER value-channel comparison.** The unit's digest reads
//!    body counts and stored POINTS. This one additionally samples
//!    every stored SURFACE on a fixed (u, v) lattice and every stored
//!    CURVE carrier at fixed parameters (`r1_mb_diff`'s `body_deep`),
//!    so a divergence confined to a face's carrier or an edge's
//!    carrier — geometry no vertex point has to move for — is visible.
//!    It also exercises the dual ARITHMETIC (`Surface::eval` at
//!    `Dual64`) rather than only stored bits.
//! 2. **The aliasing attempt DL2's law is about**, built at the key
//!    layer because no public door can seed a tangent yet (M10-4's
//!    surface). It mirrors `eval::content_key`'s own composition —
//!    slot-index prefix, value feed, upstream-key length prefix — over
//!    a two-seed pass with a SHARED subgraph, and asserts separation
//!    exactly on the seeded cone and merging off it.
//! 3. **A counterexample search for a value-only key collision**
//!    (varying seed, EFFORT dial, logged unconditionally — shape 1 of
//!    `memories/test-suite-cost.md`): can two `Dual64`s with different
//!    (value, tangent) pairs feed one key, and in particular can one
//!    pass's value bits alias another's value+tangent prefix?
//! 4. **The consumer e2e**: a corpus document AND a document written
//!    here, driven through the public evaluation door at `Dual64`,
//!    with the value channel read back and the certified doors poked.
//!
//! EVIDENCE-ONLY where marked; the value-channel and aliasing rows
//! assert and gate. The deep-digest instrument (`D`, `body_deep`,
//! `eval_deep`) lives here since the one-shot merge-base differential
//! that first carried it (`r1_mb_diff`) expired with its comparison,
//! per its own in-file note.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use corpus::{Recorder, documents, eval, failures};
use editor_core::eval::{ContentBits, KeyHasher};
use editor_core::{
    AssertionVerdict, ContentKey, Datum, DocEdit, LoopProgram, Node, ProfileDoc, ProfileProgram,
    SlotId, ValuePayload, product_recorded,
};
use editor_core::{BooleanValue, DatumValue, Evaluation, NodeResult, SplitSide};
use fixture::{len, scl};
use geom_core::{Bounds, Decide, Dual64, Tol};
use profile::SketchPlane;
use topo::Body;

/// FNV-1a 64 over whatever is fed. Not a content key — a probe digest.
struct D(u64);

impl D {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }

    fn u64(&mut self, x: u64) {
        for b in x.to_le_bytes() {
            self.0 ^= u64::from(b);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    fn s(&mut self, t: &str) {
        self.u64(t.len() as u64);
        for b in t.as_bytes() {
            self.u64(u64::from(*b));
        }
    }

    /// A scalar through its own bracket — the value channel at every
    /// scalar this probe instantiates (`f64`, `Interval`, `Dual`).
    fn sc<T: Bounds>(&mut self, x: T) {
        self.u64(x.lo().to_bits());
        self.u64(x.hi().to_bits());
    }

    fn p3<T: Decide + Bounds>(&mut self, p: geom_core::Point3<T>) {
        self.sc(p.x);
        self.sc(p.y);
        self.sc(p.z);
    }

    fn v3<T: Decide + Bounds>(&mut self, v: geom_core::Vec3<T>) {
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
fn body_deep<T>(d: &mut D, body: &Body<T>)
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
fn eval_deep<T>(ev: &Evaluation<T>) -> u64
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
                        d.v3(normal.get());
                    }
                    ValuePayload::Datum(DatumValue::Axis { origin, dir }) => {
                        d.u64(11);
                        d.p3(*origin);
                        d.v3(dir.get());
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
                    // The measured quantity IS a lane value, so it is
                    // digested through the same value-channel bracket
                    // every coordinate takes.
                    ValuePayload::Measure { value, dim } => {
                        d.u64(21);
                        d.s(&format!("{dim:?}"));
                        d.sc(*value);
                    }
                    ValuePayload::Assertion(verdict) => {
                        d.u64(22);
                        d.s(verdict.label());
                        match verdict {
                            AssertionVerdict::Holds { measured, bound }
                            | AssertionVerdict::Violated { measured, bound } => {
                                d.sc(*measured);
                                d.sc(*bound);
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

// ---------------------------------------------------------------- 1

/// The value channel at `Dual64` equals the `f64` build's, read
/// through a digest that also samples SURFACES and CURVE CARRIERS —
/// the arena geometry the unit's points-only digest never reads.
#[test]
fn r1_dual_value_channel_matches_f64_including_carriers() {
    for doc in documents() {
        let ev_f = eval::<f64>(&doc.doc);
        let ev_d = eval::<Dual64>(&doc.doc);
        assert!(
            failures(&ev_d).is_empty(),
            "{}: not green at Dual64: {:?}",
            doc.name,
            failures(&ev_d)
        );
        assert_eq!(
            eval_deep(&ev_f),
            eval_deep(&ev_d),
            "{}: value channel diverged under the DEEP digest (surfaces \
             and curve carriers sampled, not only stored points)",
            doc.name
        );
    }
}

/// The same comparison at the product-gather door, deep.
#[test]
fn r1_dual_product_matches_f64_including_carriers() {
    let tol = Tol::witness();
    for doc in documents() {
        let ev_f = eval::<f64>(&doc.doc);
        let ev_d = eval::<Dual64>(&doc.doc);
        let (pf, pd) = (
            product_recorded(&doc.doc, &ev_f, tol),
            product_recorded(&doc.doc, &ev_d, tol),
        );
        match (pf, pd) {
            (Ok(a), Ok(b)) => {
                let (mut da, mut db) = (D::new(), D::new());
                body_deep(&mut da, &a.body);
                body_deep(&mut db, &b.body);
                assert_eq!(da.0, db.0, "{}: product carriers diverged", doc.name);
            }
            (Err(_), Err(_)) => {}
            (a, b) => panic!(
                "{}: the product door disagreed across scalars: f64={:?} dual={:?}",
                doc.name,
                a.is_ok(),
                b.is_ok()
            ),
        }
    }
}

// ---------------------------------------------------------------- 2

/// One node's content key, composed the way `eval::content_key` does:
/// a tag, the slot values each preceded by their slot INDEX, then the
/// upstream keys behind a length prefix. Re-derived here rather than
/// called (the real one is private), so this row is an independent
/// statement of the composition rather than a mirror of it.
fn node_key(tag: u8, slots: &[Dual64], upstream: &[ContentKey]) -> ContentKey {
    let mut h = KeyHasher::new();
    h.write_tag(tag);
    for (i, s) in slots.iter().enumerate() {
        h.write_u64(i as u64);
        s.feed(&mut h);
    }
    h.write_u64(upstream.len() as u64);
    for k in upstream {
        h.write_key(*k);
    }
    h.finish()
}

/// **The aliasing attempt.** Two passes over one DAG:
///
/// ```text
///   shared_a   shared_b        (no seed reaches either)
///        \       /
///         merge_ab            (parameter-independent subgraph)
///            |
///          p_i / p_j          (the seeded parameter, one per pass)
///            |
///          downstream
/// ```
///
/// Pass i seeds `p`, pass j seeds `q`; both share `shared_*` and
/// `merge_ab` bit-for-bit. The law DL2 states is that keys separate
/// EXACTLY on the seeded cone. This row asserts both halves — the
/// separation (so a threaded prior cannot serve pass i's node from
/// pass j) and the merging (so the parameter-independent subgraph
/// really is reused, which is the whole point of threading a prior).
#[test]
fn r1_two_seeds_over_a_shared_subgraph_separate_exactly_on_the_cone() {
    // The parameter-independent subgraph: identical in both passes
    // because no seed reaches it (tangent 0 everywhere).
    let shared = |_pass: u8| {
        let a = node_key(1, &[Dual64::constant(2.0)], &[]);
        let b = node_key(1, &[Dual64::constant(-0.5)], &[]);
        node_key(8, &[Dual64::constant(1.0)], &[a, b])
    };
    assert_eq!(
        shared(0),
        shared(1),
        "a node no seed reaches must carry ONE key across passes — \
         without this the prior buys nothing"
    );

    // The seeded parameter node, same VALUE in both passes (this is
    // the aliasing attempt: only the tangent differs, which is
    // precisely what a value-only feed would have thrown away).
    let seeded_i = node_key(5, &[Dual64::variable(3.0)], &[shared(0)]);
    let seeded_j = node_key(5, &[Dual64::new(3.0, 0.0)], &[shared(1)]);
    assert_ne!(
        seeded_i, seeded_j,
        "two passes' seeded nodes share a value and differ only in the \
         tangent — they MUST NOT share a memo key"
    );

    // And the separation propagates: a downstream node inherits it
    // through the upstream-key link even when its own slots agree.
    let down_i = node_key(8, &[Dual64::constant(1.0)], &[seeded_i]);
    let down_j = node_key(8, &[Dual64::constant(1.0)], &[seeded_j]);
    assert_ne!(
        down_i, down_j,
        "the seed's cone must stay separated downstream through the \
         Merkle link"
    );

    // A THIRD pass that seeds a different parameter of the same value
    // must also separate from both.
    let seeded_k = node_key(5, &[Dual64::new(3.0, -1.0)], &[shared(0)]);
    assert_ne!(seeded_k, seeded_i);
    assert_ne!(seeded_k, seeded_j);
}

// ---------------------------------------------------------------- 3

/// Counterexample search (shape 1 — varying seed, EFFORT-dialed,
/// logged unconditionally): no two `Dual64`s with different
/// (value, tangent) bit pairs feed one key, and no pass's value bits
/// alias another's value+tangent prefix. The prefix half is what
/// "position separation must be real" actually asks: `feed` writes a
/// FIXED number of words per scalar, so a shorter feed cannot be a
/// prefix of a longer one at the same slot — this searches for a
/// counterexample anyway.
#[test]
fn r1_no_value_only_key_collision_search() {
    let effort: u64 = std::env::var("EFFORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    let seed: u64 = std::env::var("R1_SEED")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64
                | 1
        });
    println!("r1_no_value_only_key_collision_search: R1_SEED={seed} EFFORT={effort}");
    let mut s = seed;
    let mut next = move || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    let mut seen: std::collections::HashMap<u128, (f64, f64)> = std::collections::HashMap::new();
    let n = 20_000 * effort;
    for _ in 0..n {
        // Small integral values and tangents, where an FNV walk is
        // most likely to alias: the high bits of the two words agree.
        let v = f64::from(((next() % 41) as i32) - 20) / 4.0;
        let t = f64::from(((next() % 41) as i32) - 20) / 4.0;
        let mut h = KeyHasher::new();
        Dual64::new(v, t).feed(&mut h);
        let k = h.finish().0;
        if let Some(&(pv, pt)) = seen.get(&k) {
            assert!(
                pv.to_bits() == v.to_bits() && pt.to_bits() == t.to_bits(),
                "KEY COLLISION (R1_SEED={seed}): ({pv}, {pt}) and ({v}, {t}) \
                 feed the same key"
            );
        }
        seen.insert(k, (v, t));
    }
    // The prefix question, stated directly: the value-only feed of one
    // pass must never equal the value+tangent feed of another.
    for _ in 0..(2_000 * effort) {
        let v = f64::from(((next() % 41) as i32) - 20) / 4.0;
        let t = f64::from(((next() % 41) as i32) - 20) / 4.0;
        let mut a = KeyHasher::new();
        v.feed(&mut a); // the value channel alone
        let mut b = KeyHasher::new();
        Dual64::new(v, t).feed(&mut b); // value + tangent
        assert_ne!(
            a.finish(),
            b.finish(),
            "value-only and value+tangent feeds aliased at ({v}, {t}) \
             (R1_SEED={seed})"
        );
    }
}

// ---------------------------------------------------------------- 4

/// A document written for this review, not borrowed from the corpus:
/// a circular boss on a square plate, cut by a tilted datum, so the
/// body carries planar faces, a CYLINDER carrier, an ELLIPSE section
/// carrier and a boolean — the mix DL3 is about. The returned id is
/// the parameter node the e2e bumps.
fn r1_study_document() -> (ProfileDoc, editor_core::RecipeNodeId) {
    let mut r = Recorder::new();
    let plate = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![
            LoopProgram::polygon([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]).unwrap(),
        ],
    }));
    let slab = r.insert(Node::Extrude {
        profile: plate,
        distance: len(0.25),
    });
    let boss_profile = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![LoopProgram::circle(0.0, 0.0, 0.5).unwrap()],
    }));
    let boss = r.insert(Node::Extrude {
        profile: boss_profile,
        distance: len(1.0),
    });
    let fused = r.insert(Node::Boolean {
        op: editor_core::BooleanOp::Union,
        a: slab,
        b: boss,
        declare: None,
    });
    let tool = r.insert(Node::Datum(Datum::Plane {
        origin: [len(0.0), len(0.0), len(0.75)],
        normal: [scl(0.25_f64.sin()), scl(0.0), scl(0.25_f64.cos())],
    }));
    let _split = r.insert(Node::Split {
        target: fused,
        tool,
    });
    (r.doc, tool)
}

/// **The consumer e2e** (evidence-only for the friction it prints;
/// the assertions are the door's own contract). Drives a corpus
/// document and the study document above through `evaluate::<Dual64>`
/// exactly as an outside consumer would, reads the value channel back
/// through `Bounds`, gathers the product, and reports what the
/// certified doors do at a dual.
#[test]
fn r1_e2e_consumer_drive_at_dual64() {
    let tol = Tol::witness();
    let (study, tool) = r1_study_document();
    let named: Vec<(&str, ProfileDoc)> = vec![
        ("corpus:die_composed", {
            documents()
                .into_iter()
                .find(|d| d.name == "die_composed")
                .expect("die_composed is registered")
                .doc
        }),
        ("r1:study", study.clone()),
    ];

    for (name, doc) in &named {
        let ev_d = eval::<Dual64>(doc);
        let ev_f = eval::<f64>(doc);
        println!(
            "R1E2E {name}: f64 failures={} dual failures={}",
            failures(&ev_f).len(),
            failures(&ev_d).len()
        );
        for f in failures(&ev_f) {
            println!("R1E2E {name}   f64  {f}");
        }
        for f in failures(&ev_d) {
            println!("R1E2E {name}   dual {f}");
        }
        // The contract under test: whatever f64 does, the dual does the
        // SAME, and its value channel is the f64 channel.
        assert_eq!(
            failures(&ev_f).len(),
            failures(&ev_d).len(),
            "{name}: the two scalars disagreed on which nodes evaluate"
        );
        assert_eq!(eval_deep(&ev_f), eval_deep(&ev_d), "{name}: value channel");

        // Read the value channel back the way a consumer would.
        let mut points = 0usize;
        let mut tangents_nonzero = 0usize;
        for &id in &ev_d.order {
            if let Some(editor_core::NodeResult::Ok(v)) = ev_d.result(id)
                && let ValuePayload::Body(b)
                | ValuePayload::Boolean(editor_core::BooleanValue::Body { body: b, .. }) =
                    &v.payload
            {
                for (_k, p) in b.points() {
                    points += 1;
                    if p.x.deriv != 0.0 || p.y.deriv != 0.0 || p.z.deriv != 0.0 {
                        tangents_nonzero += 1;
                    }
                    // `Bounds` is the value channel and only it.
                    assert_eq!(p.x.lo(), p.x.hi());
                    assert_eq!(p.x.lo(), p.x.value);
                }
            }
        }
        println!(
            "R1E2E {name}: nodes={} dual points={points} nonzero tangents={tangents_nonzero}",
            ev_d.order.len()
        );

        // The gather door.
        match product_recorded(doc, &ev_d, tol) {
            Ok(p) => {
                println!(
                    "R1E2E {name}: product gathered at Dual64, {} faces",
                    p.body.faces().count()
                );
                // The friction DL3 is about, made visible on a body the
                // service just handed back: the validation DOOR is still
                // callable at a dual (it is `PropsQuadLane`-bounded, not
                // policy-bounded) and refuses there. What DL3 removes is
                // the evaluation service's CALL, not the door.
                match topo::validate_geometric(&p.body, tol) {
                    Ok(()) => println!("R1E2E {name}: direct validate_geometric at Dual64 PASSED"),
                    Err(errs) => println!(
                        "R1E2E {name}: direct validate_geometric at Dual64 refused {} finding(s);                          first = {:?}",
                        errs.len(),
                        errs.first()
                    ),
                }
                // The same body through the f64 lane, for the contrast.
                if let Ok(pf) = product_recorded(doc, &ev_f, tol) {
                    match topo::validate_geometric(&pf.body, tol) {
                        Ok(()) => println!("R1E2E {name}: direct validate_geometric at f64 PASSED"),
                        Err(errs) => println!(
                            "R1E2E {name}: direct validate_geometric at f64 refused {} finding(s)",
                            errs.len()
                        ),
                    }
                }
                // The advisory registry USED to be probed here, as a
                // gap DL3 did not cover. It is no longer reachable at a
                // dual: `run_checks` is `Decide + AtRestPolicy +
                // CertifiedBounds`, and no `Dual` implements
                // `CertifiedEnclosure`. That is DL1 holding one door
                // further out — the registry's separation resident
                // GRANTS a certificate (box non-overlap is a genuine
                // separation claim), and a dual never certifies. The
                // gap this row recorded is closed rather than
                // unobserved.
                println!("R1E2E {name}: run_checks is not callable at Dual64 (CertifiedBounds)");
            }
            Err(e) => println!("R1E2E {name}: product REFUSED at Dual64: {e:?}"),
        }
    }

    // The friction DL3 is about, made visible: the validation DOOR is
    // still callable at a dual (it is `PropsQuadLane`-bounded, not
    // policy-bounded) and refuses there. What DL3 removes is the
    // evaluation service's CALL, not the door.
    let ev_d = eval::<Dual64>(&study);
    if let Some(editor_core::NodeResult::Ok(v)) = ev_d.result(tool) {
        let _ = v; // the datum node itself carries no body
    }
    let body_node = ev_d
        .order
        .iter()
        .rev()
        .find(|&&id| {
            matches!(
                ev_d.result(id),
                Some(editor_core::NodeResult::Ok(v))
                    if matches!(v.payload, ValuePayload::Split { .. })
            )
        })
        .copied();
    if let Some(id) = body_node
        && let Some(editor_core::NodeResult::Ok(v)) = ev_d.result(id)
        && let ValuePayload::Split { above, .. } = &v.payload
        && let editor_core::SplitSide::Body(b) = above
    {
        let direct = topo::validate_geometric(b.as_ref(), tol);
        println!(
            "R1E2E direct validate_geometric at Dual64: {}",
            match &direct {
                Ok(()) => "PASSED (no refusal)".to_string(),
                Err(errs) => format!("{} refusal(s): {:?}", errs.len(), errs.first()),
            }
        );
        // The advisory registry is no longer callable at a dual —
        // see the sibling row above for why (DL1, one door out).
        println!("R1E2E run_checks is not callable at Dual64 (CertifiedBounds)");
    }

    // A parameter bump through the public door with a threaded prior —
    // the incremental path a sensitivity sweep would use.
    let bumped = editor_core::apply(
        &study,
        &DocEdit::SetParam {
            node: tool,
            slot: SlotId::Origin(editor_core::Axis3::Z),
            expr: len(0.6875),
        },
        tol,
    )
    .expect("bump applies")
    .doc;
    let after = editor_core::evaluate::<Dual64>(
        &bumped,
        Some(&ev_d),
        &editor_core::CancelToken::new(),
        &editor_core::EvalOptions::default(),
        tol,
    );
    println!(
        "R1E2E study bump: reused={} recomputed={} of {}",
        after.reused,
        after.recomputed,
        bumped.len()
    );
    assert!(
        after.reused > 0,
        "threading a prior Evaluation<Dual64> must reuse the untouched subgraph"
    );
}

/// **Is DL3's measured problem real?** DUAL-DESIGN DL3 opens with a
/// measurement: *"if `evaluate::<Dual64>` compiled today, a corpus
/// document with an ellipse-trimmed or spline face would FAIL the
/// product gather's tier-3 ... with `VolumeUncomputable`; `Approx`
/// faces report `ApproxLaneUnsupported`; curved coincident pairs
/// `CensusUnsupported`."* The seam this PR builds exists to route
/// around that. This row re-takes the measurement independently: it
/// gathers each corpus product at `Dual64` (which no longer gates) and
/// then calls the certified door DIRECTLY on the body it got back,
/// beside the same call at `f64`. EVIDENCE-ONLY — it prints a table.
#[test]
fn r1_is_dl3s_measured_problem_reproducible() {
    let tol = Tol::witness();
    let (mut refused_dual, mut refused_f64, mut gathered) = (0usize, 0usize, 0usize);
    for doc in documents() {
        let ev_d = eval::<Dual64>(&doc.doc);
        let ev_f = eval::<f64>(&doc.doc);
        let Ok(pd) = product_recorded(&doc.doc, &ev_d, tol) else {
            continue;
        };
        gathered += 1;
        let d = topo::validate_geometric(&pd.body, tol);
        let f = product_recorded(&doc.doc, &ev_f, tol)
            .ok()
            .map(|pf| topo::validate_geometric(&pf.body, tol));
        if d.is_err() {
            refused_dual += 1;
        }
        if matches!(f, Some(Err(_))) {
            refused_f64 += 1;
        }
        println!(
            "R1DL3 {:<28} dual={:<48} f64={}",
            doc.name,
            match &d {
                Ok(()) => "PASSED".to_string(),
                Err(e) => format!("{} refusal(s): {:?}", e.len(), e.first().map(kindname)),
            },
            match &f {
                None => "n/a".to_string(),
                Some(Ok(())) => "PASSED".to_string(),
                Some(Err(e)) => format!("{} refusal(s)", e.len()),
            }
        );
    }
    println!(
        "R1DL3 SUMMARY: {gathered} products gathered at Dual64; the certified \
         door refuses {refused_dual} of them at Dual64 and {refused_f64} at f64"
    );
}

fn kindname(e: &topo::ValidationError) -> String {
    let s = format!("{e:?}");
    s.split(|c: char| !c.is_alphanumeric())
        .next()
        .unwrap_or("?")
        .to_string()
}

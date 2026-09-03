//! Adversarial review of the M4 PR 2 evaluation service (R2–R7):
//! content-key collisions and memo soundness under mutation-shaped
//! edits, poisoning determinism, cancelation pollution, the parallel
//! lane, the revolve τ door, and wire refusal doors.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::fmt::Write as _;

use editor_core::{
    BooleanOp, BooleanValue, CancelToken, DocEdit, EvalOptions, EvalOutcome, Evaluation, Expr,
    Node, NodeErrorKind, NodeResult, ProfileDoc, RecipeNodeId, SlotId, ValuePayload, evaluate,
};
use editor_core::{CapEnd, RoleSeg};
use fixture::{
    ang, axis_in_plane, die, insert, len, on_frame, on_frame_keeping, scl, square, step,
};
use geom_core::Tol;
use topo::{Body, mass_properties};

fn run(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>, parallel: bool) -> Evaluation<f64> {
    let opts = EvalOptions {
        parallel,
        ..EvalOptions::default()
    };
    evaluate::<f64>(doc, prior, &CancelToken::new(), &opts, Tol::witness())
}

fn fingerprint(b: &Body<f64>) -> String {
    let mut s = String::new();
    let m = mass_properties(b, Tol::witness()).unwrap();
    let _ = writeln!(
        s,
        "V {:016x} A {:016x}",
        m.volume.to_bits(),
        m.surface_area.to_bits()
    );
    for (k, p) in b.points() {
        let _ = writeln!(
            s,
            "P {k:?} {:016x} {:016x} {:016x}",
            p.x.to_bits(),
            p.y.to_bits(),
            p.z.to_bits()
        );
    }
    for (k, c) in b.curves() {
        let _ = writeln!(s, "C {k:?} {c:?}");
    }
    for (k, sf) in b.surfaces() {
        let _ = writeln!(s, "S {k:?} {sf:?}");
    }
    for (k, e) in b.edges() {
        let _ = writeln!(s, "E {k:?} {e:?}");
    }
    s
}

fn boolean_body(ev: &Evaluation<f64>, id: RecipeNodeId) -> &Body<f64> {
    match &ev.value(id).expect("node evaluated").payload {
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body,
        other => panic!("expected boolean body, got {}", other.kind_name()),
    }
}

/// Two overlapping bricks A ([0,2]³-ish) and B, then one Subtract with
/// the operands in the given order. Node ids are minted monotonically,
/// so both docs address the Subtract by the SAME id.
fn subtract_doc(swap: bool) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("review_m4_pr2", Tol::witness());
    let (doc, pa) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(1.0, 1.0, 1.0)],
    );
    let (doc, a) = insert(
        doc,
        Node::Extrude {
            profile: pa,
            distance: len(2.0),
        },
    );
    let (doc, pb) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(1.5, 1.5, 1.0)],
    );
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: pb,
            distance: len(1.0),
        },
    );
    let (x, y) = if swap { (b, a) } else { (a, b) };
    // M4 PR 5: the flush bottom caps are declared (sides resolve
    // per-operand, so ONE Declare serves both operand orders).
    let (doc, decl) = insert(
        doc,
        Node::declare_rest(vec![(
            fixture::fname(a, RoleSeg::Cap(CapEnd::Bottom)),
            fixture::fname(b, RoleSeg::Cap(CapEnd::Bottom)),
        )]),
    );
    let (doc, s) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: x,
            b: y,
            declare: Some(decl),
        },
    );
    (doc, s)
}

/// R2: Subtract(A,B)'s memo entry offered as prior to a doc holding
/// Subtract(B,A) at the SAME node id with the SAME upstream keys in
/// swapped order — a key collision here silently reuses A−B for B−A.
#[test]
fn operand_swap_never_reuses_the_prior_subtract() {
    let (d1, s1) = subtract_doc(false);
    let (d2, s2) = subtract_doc(true);
    assert_eq!(s1, s2, "attack precondition: aligned node ids");
    let e1 = run(&d1, None, false);
    let e2_scratch = run(&d2, None, false);
    let e2 = run(&d2, Some(&e1), false);
    // The swapped subtract must have been RECOMPUTED (its key differs)
    // and must bit-match its own scratch result, not the prior's.
    assert_eq!(
        fingerprint(boolean_body(&e2, s2)),
        fingerprint(boolean_body(&e2_scratch, s2)),
        "swapped operands must recompute, not reuse A-B for B-A"
    );
    assert_ne!(
        fingerprint(boolean_body(&e2, s2)),
        fingerprint(boolean_body(&e1, s1)),
        "A-B and B-A genuinely differ (attack is live)"
    );
    // Upstream nodes (identical in both docs) MAY reuse; the subtract
    // may not.
    assert!(e2.recomputed >= 1, "the swapped subtract must recompute");
}

/// R3: delete a node and insert an identical replacement — the fresh
/// id must not inherit the old memo entry (ids are never reused; the
/// memo is keyed by id THEN content key).
#[test]
fn delete_and_reinsert_identical_node_recomputes() {
    let doc = ProfileDoc::empty_derived("review_m4_pr2", Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.5, 0.5, 0.5)],
    );
    let (doc, e_old) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    );
    let e0 = run(&doc, None, false);
    let fp_old = fingerprint(match &e0.value(e_old).unwrap().payload {
        ValuePayload::Body(b) => b,
        other => panic!("expected body, got {}", other.kind_name()),
    });
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: e_old });
    let (doc, e_new) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    );
    assert_ne!(e_old, e_new, "ids are never reused");
    let e1 = run(&doc, Some(&e0), false);
    // The new node has no prior entry at its id: it must RECOMPUTE
    // (reuse would require an id-collision) — and the result must be
    // bit-identical anyway (D9 determinism).
    assert!(e1.value(e_old).is_none(), "the deleted id must not appear");
    let fp_new = fingerprint(match &e1.value(e_new).unwrap().payload {
        ValuePayload::Body(b) => b,
        other => panic!("expected body, got {}", other.kind_name()),
    });
    assert_eq!(fp_new, fp_old, "identical recipe, identical bits");
    assert_eq!(e1.recomputed, 1, "only the re-inserted node recomputes");
}

/// R4: a diamond whose BOTH mid ancestors fail — `through` must be
/// deterministic (first blocking input in Node::inputs() order) and
/// identical across the sequential and parallel schedules; every
/// `through` must land on a Failed entry.
#[test]
fn diamond_with_two_failed_ancestors_has_deterministic_through() {
    let doc = ProfileDoc::empty_derived("review_m4_pr2", Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.5, 0.5, 0.5)],
    );
    // Two failing extrudes: distance = 1/0 (NonFiniteResult).
    let bad = || Expr::div(len(1.0), scl(0.0)).unwrap();
    let (doc, fa) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: bad(),
        },
    );
    let (doc, fb) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: bad(),
        },
    );
    let (doc, join) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: fa,
            b: fb,
            declare: None,
        },
    );
    // One more hop: a transform downstream of the poisoned join.
    let (doc, tail) = insert(
        doc,
        Node::Transform {
            input: join,
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    for parallel in [false, true] {
        let ev = run(&doc, None, parallel);
        for id in [fa, fb] {
            assert!(
                matches!(ev.nodes.get(&id), Some(NodeResult::Failed(_))),
                "extrude {id:?} must fail typed (parallel={parallel})"
            );
        }
        // through = FIRST blocking input in inputs() order = fa.
        match ev.nodes.get(&join) {
            Some(NodeResult::Poisoned { through }) => {
                assert_eq!(*through, fa, "parallel={parallel}")
            }
            other => panic!("join: expected Poisoned, got {other:?}"),
        }
        // Propagated through the poisoned join to the NEAREST FAILED
        // ancestor — still fa, and it must be a Failed entry.
        match ev.nodes.get(&tail) {
            Some(NodeResult::Poisoned { through }) => {
                assert_eq!(*through, fa, "parallel={parallel}");
                assert!(matches!(ev.nodes.get(through), Some(NodeResult::Failed(_))));
            }
            other => panic!("tail: expected Poisoned, got {other:?}"),
        }
        // The profile itself still completed (descendants only).
        assert!(ev.value(p).is_some());
    }
}

/// R4: cancelation must never pollute the memo — a canceled partial
/// result, used as the prior, must yield a full evaluation
/// bit-identical to scratch. Races the token against a real die
/// evaluation until a genuine mid-run partial is observed (plus the
/// deterministic pre-canceled boundary).
#[test]
fn canceled_partials_are_typed_and_never_pollute_the_memo() {
    let d = die();
    let scratch = run(&d.doc, None, false);
    let fp = fingerprint(boolean_body(&scratch, d.final_node));
    let total = scratch.order.len();

    // Boundary: pre-canceled → empty completed prefix, typed.
    let pre = CancelToken::new();
    pre.cancel();
    let ev = evaluate::<f64>(&d.doc, None, &pre, &EvalOptions::default(), Tol::witness());
    assert_eq!(ev.outcome, EvalOutcome::Canceled);
    assert_eq!(ev.nodes.len(), 0);
    assert_eq!(ev.order.len(), total, "order is data, not schedule");

    // Race: cancel from another thread; retry until we catch a strict
    // mid-run partial (0 < n < total). Every catch must be internally
    // consistent and memo-safe.
    let mut caught = 0;
    for delay_us in [
        50u64, 200, 500, 1000, 2000, 5000, 10000, 20000, 50000, 100000,
    ] {
        let tok = CancelToken::new();
        let t2 = tok.clone();
        let h = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_micros(delay_us));
            t2.cancel();
        });
        let part = evaluate::<f64>(&d.doc, None, &tok, &EvalOptions::default(), Tol::witness());
        h.join().unwrap();
        if part.outcome != EvalOutcome::Canceled || part.nodes.is_empty() {
            continue;
        }
        caught += 1;
        assert!(
            part.nodes.len() < total,
            "canceled run reported full completion"
        );
        // The completed prefix must be exactly a prefix of `order`
        // (sequential path evaluates in order).
        for (i, id) in part.order.iter().enumerate() {
            let present = part.nodes.contains_key(id);
            assert_eq!(
                present,
                i < part.nodes.len(),
                "non-prefix completion at {i}"
            );
        }
        // Memo safety: resuming from the partial must reproduce the
        // scratch bits exactly, reusing only completed nodes.
        let resumed = run(&d.doc, Some(&part), false);
        assert_eq!(resumed.outcome, EvalOutcome::Completed);
        assert_eq!(fingerprint(boolean_body(&resumed, d.final_node)), fp);
        assert_eq!(resumed.reused + resumed.recomputed, total);
        if caught >= 3 {
            break;
        }
    }
    assert!(
        caught >= 1,
        "race never caught a mid-run partial; retune delays"
    );
}

/// A doc mixing a diamond, a circular pattern, a revolve, a split,
/// and a poisoned subgraph — the R5 stressor.
fn rich_doc() -> (ProfileDoc, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty_derived("review_m4_pr2", Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(1.0, 1.0, 0.5)],
    );
    let (doc, base) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    );
    // Diamond: two transforms of base, unioned. Decoupled offsets
    // (M4 PR 5): the same body through two placements shares its
    // NAMES on both sides, so flush planes here could not even be
    // declared by name pair — the stressor shears all three axes so
    // no plane coincides and no declaration is needed.
    let tr = |dx: f64, dy: f64, dz: f64| Node::Transform {
        input: base,
        translation: [len(dx), len(dy), len(dz)],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: ang(0.0),
    };
    let (doc, t1) = insert(doc, tr(0.25, 0.125, 0.0625));
    let (doc, t2) = insert(doc, tr(-0.25, -0.125, -0.0625));
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: t1,
            b: t2,
            declare: None,
        },
    );
    // Circular pattern of base about a datum axis (data only).
    let (doc, ax) = insert(
        doc,
        Node::Datum(editor_core::Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, pat) = insert(
        doc,
        Node::Pattern {
            input: base,
            count: Expr::count(4),
            kind: editor_core::PatternKind::Circular {
                axis: ax,
                step: ang(std::f64::consts::FRAC_PI_2),
            },
        },
    );
    // Revolve: half-turn of a square offset from the axis.
    let (doc, plane, rp) = on_frame_keeping(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(1.5, 0.0, 0.25)],
    );
    let (doc, rax) = insert(
        doc,
        // The axis, in the frame's own coordinates: the profile's v is
        // world +Y, so the line the revolve turns about is that
        // frame's +y through (0, 0).
        axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)),
    );
    let (doc, rev) = insert(
        doc,
        Node::Revolve {
            profile: rp,
            axis: rax,
            angle: ang(std::f64::consts::PI),
        },
    );
    // Split the union by a datum plane.
    let (doc, pl) = insert(
        doc,
        Node::Datum(editor_core::Datum::Plane {
            origin: [len(1.0), len(1.0), len(0.5)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, sp) = insert(
        doc,
        Node::Split {
            target: u,
            tool: pl,
        },
    );
    // Poisoned subgraph: failing extrude + a dependent subtract.
    let (doc, bad) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: Expr::div(len(1.0), scl(0.0)).unwrap(),
        },
    );
    let (doc, poisoned) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: u,
            b: bad,
            declare: None,
        },
    );
    (
        doc,
        vec![
            p, base, t1, t2, u, ax, pat, rp, rax, rev, pl, sp, bad, poisoned,
        ],
    )
}

/// A structural digest of an ENTIRE evaluation: every node's result
/// kind, `through` targets, and full body fingerprints for every
/// body-bearing payload (split sides and pattern instances included).
fn eval_digest(ev: &Evaluation<f64>) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "order {:?}", ev.order);
    for (id, r) in &ev.nodes {
        match r {
            NodeResult::Ok(v) => {
                let _ = write!(s, "{id:?} Ok key={:?} ", v.content_key);
                match &v.payload {
                    ValuePayload::Body(b) => {
                        let _ = writeln!(s, "body\n{}", fingerprint(b));
                    }
                    ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
                        let _ = writeln!(s, "bool\n{}", fingerprint(body));
                    }
                    ValuePayload::Boolean(BooleanValue::Empty) => {
                        let _ = writeln!(s, "empty");
                    }
                    ValuePayload::Split { above, below } => {
                        for side in [above, below] {
                            match side {
                                editor_core::SplitSide::Body(b) => {
                                    let _ = writeln!(s, "side\n{}", fingerprint(b));
                                }
                                editor_core::SplitSide::Empty => {
                                    let _ = writeln!(s, "side empty");
                                }
                            }
                        }
                    }
                    ValuePayload::Instances(v) => {
                        for b in v {
                            let _ = writeln!(s, "inst\n{}", fingerprint(b));
                        }
                    }
                    other => {
                        let _ = writeln!(s, "{}", other.kind_name());
                    }
                }
            }
            NodeResult::Failed(e) => {
                let _ = writeln!(s, "{id:?} Failed {:?}", std::mem::discriminant(&e.kind));
            }
            NodeResult::Poisoned { through } => {
                let _ = writeln!(s, "{id:?} Poisoned through {through:?}");
            }
        }
    }
    s
}

/// R5: sequential/parallel × scratch/memo — four evaluations of the
/// rich doc must be digest-identical (bodies bit-equal, poison
/// structure equal), and a poisoned node inside a parallel level must
/// not tear.
#[test]
fn four_way_schedule_memo_identity_on_rich_doc() {
    let (doc, _ids) = rich_doc();
    let seq = run(&doc, None, false);
    // The rich doc must actually be rich: union, split, pattern, and
    // revolve all Ok; exactly the bad extrude Failed and its subtract
    // Poisoned.
    assert!(seq.value(_ids[4]).is_some(), "union must succeed");
    assert!(seq.value(_ids[6]).is_some(), "pattern must succeed");
    assert!(seq.value(_ids[9]).is_some(), "revolve must succeed");
    assert!(
        seq.value(_ids[11]).is_some(),
        "split must succeed: {:?}",
        seq.nodes.get(&_ids[11])
    );
    assert!(matches!(
        seq.nodes.get(&_ids[12]),
        Some(NodeResult::Failed(_))
    ));
    assert!(matches!(
        seq.nodes.get(&_ids[13]),
        Some(NodeResult::Poisoned { .. })
    ));
    let par = run(&doc, None, true);
    let seq_memo = run(&doc, Some(&seq), false);
    let par_memo = run(&doc, Some(&seq), true);
    let d0 = eval_digest(&seq);
    assert_eq!(d0, eval_digest(&par), "seq vs par scratch");
    assert_eq!(d0, eval_digest(&seq_memo), "seq scratch vs seq memo");
    assert_eq!(d0, eval_digest(&par_memo), "seq scratch vs par memo");
    // Memo runs may not RE-run failed/poisoned work as reuse: failures
    // recompute (only Ok values are memo currency).
    assert!(seq_memo.reused > 0 && seq_memo.recomputed >= 1);
}

/// A revolve doc: square at x∈[1.25,1.75] revolved about the y axis
/// by `angle`.
fn revolve_doc(angle: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("review_m4_pr2", Tol::witness());
    let (doc, plane, rp) = on_frame_keeping(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(1.5, 0.0, 0.25)],
    );
    let (doc, rax) = insert(
        doc,
        // The axis, in the frame's own coordinates: the profile's v is
        // world +Y, so the line the revolve turns about is that
        // frame's +y through (0, 0).
        axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)),
    );
    let (doc, rev) = insert(
        doc,
        Node::Revolve {
            profile: rp,
            axis: rax,
            angle: ang(angle),
        },
    );
    (doc, rev)
}

fn revolve_volume(angle: f64) -> Result<f64, String> {
    let (doc, rev) = revolve_doc(angle);
    let ev = run(&doc, None, false);
    match ev.nodes.get(&rev) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Body(b) => Ok(mass_properties(b, Tol::witness()).unwrap().volume),
            other => Err(format!("non-body: {}", other.kind_name())),
        },
        Some(NodeResult::Failed(e)) => Err(format!("{:?}", e.kind)),
        other => Err(format!("{other:?}")),
    }
}

/// R6: the τ-coincidence door, swept at ulp and band scale. The
/// decision must be MARGINED (decided Zero band ⇒ Full; in-band ⇒
/// typed escalation; definite ⇒ Partial), never a raw compare.
#[test]
fn revolve_tau_door_is_margined_not_raw() {
    use std::f64::consts::TAU;
    // Pappus: V_full = 2π·R̄·A with R̄=1.5, A=0.25 ⇒ V ≈ 2.356…; the
    // faceted kernel volume differs, so compare RELATIVE to the
    // full-revolve result instead of an analytic oracle.
    let v_full = revolve_volume(TAU).expect("exact τ must be Full");
    // Probes derived from the AMBIENT tolerance so this test is valid
    // on every ε row of the gate matrix (the first draft hard-coded
    // the 1e-9 default and failed the 1e-6/1e-12 rows).
    let tol = geom_core::Tol::witness().get();
    let (eps, kesc) = (tol.eps, tol.eps * tol.k);
    // ±1 ulp and ±eps/10: inside the zero band ⇒ silently Full,
    // bit-identical volume to exact τ.
    for a in [
        TAU - f64::EPSILON * TAU, // 1 ulp under
        TAU + f64::EPSILON * TAU,
        TAU - eps * 0.1,
        TAU + eps * 0.1,
        -TAU, // sign door: |θ|
    ] {
        let v = revolve_volume(a).unwrap_or_else(|e| panic!("θ={a:.17}: {e}"));
        assert_eq!(
            v.to_bits(),
            v_full.to_bits(),
            "θ={a:.17} must classify Full"
        );
    }
    // In-band margin (eps < |m| < K·eps): typed escalation, NOT a
    // silent guess either way. Geometric mean of the band edges.
    let in_band = (eps * kesc).sqrt();
    let e = revolve_volume(TAU - in_band).expect_err("in-band must escalate typed");
    assert!(e.contains("Escalated"), "got {e}");
    assert!(
        e.contains("revolve_full_vs_partial"),
        "predicate must be k_stats-named: {e}"
    );
    // Definite partial: strictly less volume.
    let v_half = revolve_volume(TAU / 2.0).expect("half revolve");
    assert!(v_half < v_full * 0.51 && v_half > v_full * 0.49);
    // Definitely OVER τ, past the band: the kernel's own
    // classification must refuse loudly, not wrap silently.
    match revolve_volume(TAU + kesc * 10.0) {
        Ok(v) => panic!("θ=τ+10Kε silently produced a body (V={v})"),
        Err(e) => assert!(!e.is_empty()),
    }
}

/// R6: the die deviation's other half — a pip placed by a REAL
/// rotation (π/2 about z) must land where the translation would, up
/// to non-dyadic rounding: volume within 1e-12 of the translated
/// pip's exact result, and NOT bit-equal (the rounding is real).
#[test]
fn rotational_pip_matches_translated_pip_to_rounding() {
    // Master pip on the +z face at face-coords (1,1) authored two
    // ways: translated (exact), and rotated π/2 about the cube center
    // axis from face-coords (1,1) — same target pocket by symmetry.
    let build = |rotate: bool| {
        let doc = ProfileDoc::empty_derived("review_m4_pr2", Tol::witness());
        let (doc, cp) = on_frame(
            doc,
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]],
        );
        let (doc, cube) = insert(
            doc,
            Node::Extrude {
                profile: cp,
                distance: len(2.0),
            },
        );
        let (doc, pp) = on_frame(
            doc,
            [0.0, 0.0, 2.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(0.0, 0.0, 0.125)],
        );
        let (doc, pip) = insert(
            doc,
            Node::Extrude {
                profile: pp,
                distance: len(-0.125),
            },
        );
        let (doc, tr) = insert(
            doc,
            if rotate {
                // The pip is CENTERED on the face-plane origin, so a
                // π/2 rotation about the world z axis maps the square
                // onto itself — same pocket — but through inexact
                // sin/cos bits: the resulting subtract is the honest
                // non-dyadic bracket of the same oracle.
                Node::Transform {
                    input: pip,
                    translation: [len(1.0), len(1.0), len(0.0)],
                    rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                    rotation_angle: ang(std::f64::consts::FRAC_PI_2),
                }
            } else {
                Node::Transform {
                    input: pip,
                    translation: [len(1.0), len(1.0), len(0.0)],
                    rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                    rotation_angle: ang(0.0),
                }
            },
        );
        // M4 PR 5: the pip's outer cap lies ON the cube's top —
        // declared (the rotational variant maps the SAME names).
        let (doc, decl) = insert(
            doc,
            Node::declare_rest(vec![(
                fixture::fname(cube, RoleSeg::Cap(CapEnd::Top)),
                fixture::fname(pip, RoleSeg::Cap(CapEnd::Bottom)),
            )]),
        );
        let (doc, sub) = insert(
            doc,
            Node::Boolean {
                op: BooleanOp::Subtract,
                a: cube,
                b: tr,
                declare: Some(decl),
            },
        );
        (doc, sub)
    };
    let (d_exact, s_exact) = build(false);
    let (d_rot, s_rot) = build(true);
    let v_exact = mass_properties(
        boolean_body(&run(&d_exact, None, false), s_exact),
        Tol::witness(),
    )
    .unwrap()
    .volume;
    let v_rot = mass_properties(
        boolean_body(&run(&d_rot, None, false), s_rot),
        Tol::witness(),
    )
    .unwrap()
    .volume;
    assert_eq!(
        v_exact,
        8.0 - 0.25 * 0.25 * 0.125,
        "translated pip is the exact dyadic oracle"
    );
    assert!(
        (v_rot - v_exact).abs() < 1e-12,
        "rotational pip drifted: {v_rot} vs {v_exact}"
    );
}

/// R6: the refusal doors — every wrong wiring is a TYPED per-node
/// failure, never a panic and never a silent guess.
#[test]
fn wire_doors_refuse_typed() {
    let (doc, ids) = rich_doc();
    let (p, base, u, ax, pat) = (ids[0], ids[1], ids[4], ids[5], ids[6]);
    // Instances fed to a boolean: WrongOperand (their flag #5).
    let (d, bad_bool) = insert(
        doc.clone(),
        Node::Boolean {
            op: BooleanOp::Union,
            a: u,
            b: pat,
            declare: None,
        },
    );
    let ev = run(&d, None, false);
    match ev.nodes.get(&bad_bool) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::WrongOperand {
                expected, found, ..
            } => {
                assert_eq!((*expected, *found), ("body", "instances"));
            }
            other => panic!("expected WrongOperand, got {other:?}"),
        },
        other => panic!("expected Failed, got {other:?}"),
    }
    // Revolve about the rich doc's `ax` — a 3-D z-axis datum. This
    // asserted `AxisNotInSketchPlane`, a decided projection finding the
    // direction out of plane. A revolve seats an axis written IN a
    // frame now, so a world-space axis never reaches that question: it
    // is refused one door earlier, as the wrong KIND of node, with no
    // band consulted.
    let (d, bad_rev) = insert(
        doc.clone(),
        Node::Revolve {
            profile: p,
            axis: ax,
            angle: ang(1.0),
        },
    );
    let ev = run(&d, None, false);
    match ev.nodes.get(&bad_rev) {
        Some(NodeResult::Failed(e)) => {
            assert!(
                matches!(e.kind, NodeErrorKind::WrongOperand { input, .. } if input == ax),
                "got {:?}",
                e.kind
            );
        }
        other => panic!("expected Failed, got {other:?}"),
    }
    // Split by an axis datum: WrongOperand (needs a plane).
    let (d, bad_split) = insert(
        doc.clone(),
        Node::Split {
            target: u,
            tool: ax,
        },
    );
    let ev = run(&d, None, false);
    match ev.nodes.get(&bad_split) {
        Some(NodeResult::Failed(e)) => {
            assert!(
                matches!(e.kind, NodeErrorKind::WrongOperand { .. }),
                "got {:?}",
                e.kind
            );
        }
        other => panic!("expected Failed, got {other:?}"),
    }
    // Pattern count 0: NonPositiveCount.
    let (d, bad_pat) = insert(
        doc.clone(),
        Node::Pattern {
            input: base,
            count: Expr::count(0),
            kind: editor_core::PatternKind::Circular {
                axis: ax,
                step: ang(1.0),
            },
        },
    );
    let ev = run(&d, None, false);
    match ev.nodes.get(&bad_pat) {
        Some(NodeResult::Failed(e)) => {
            assert!(
                matches!(e.kind, NodeErrorKind::NonPositiveCount { count: 0 }),
                "got {:?}",
                e.kind
            );
        }
        other => panic!("expected Failed, got {other:?}"),
    }
    // Boolean whose declare input is not a Declare node: WrongOperand.
    let (d, bad_decl) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: u,
            b: base,
            declare: Some(ax),
        },
    );
    let ev = run(&d, None, false);
    match ev.nodes.get(&bad_decl) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::WrongOperand { expected, .. } => assert_eq!(*expected, "declarations"),
            other => panic!("expected WrongOperand(declarations), got {other:?}"),
        },
        other => panic!("expected Failed, got {other:?}"),
    }
}

/// R2: same evaluated floats under DIFFERENT op tags must not collide
/// — a Plane and an Axis datum with identical slot values at the same
/// node id (parallel docs), prior offered across: no reuse.
#[test]
fn datum_kind_is_key_separated() {
    let build = |axis: bool| {
        let doc = ProfileDoc::empty_derived("review_m4_pr2", Tol::witness());
        let node = if axis {
            Node::Datum(editor_core::Datum::Axis {
                origin: [len(0.5), len(0.25), len(0.125)],
                direction: [scl(0.0), scl(0.0), scl(1.0)],
            })
        } else {
            Node::Datum(editor_core::Datum::Plane {
                origin: [len(0.5), len(0.25), len(0.125)],
                normal: [scl(0.0), scl(0.0), scl(1.0)],
            })
        };
        insert(doc, node)
    };
    let (d1, n1) = build(false);
    let (d2, n2) = build(true);
    assert_eq!(n1, n2);
    let e1 = run(&d1, None, false);
    let e2 = run(&d2, Some(&e1), false);
    assert_ne!(
        e1.value(n1).unwrap().content_key,
        e2.value(n2).unwrap().content_key,
        "identical floats under different datum kinds must not collide"
    );
    assert!(matches!(
        &e2.value(n2).unwrap().payload,
        ValuePayload::Datum(editor_core::DatumValue::Axis { .. })
    ));
    assert_eq!(e2.reused, 0, "nothing may be reused across the kind flip");
}

/// R7: the Interval lane's memo — keys built from `repr_bits` must
/// behave exactly like the f64 lane: full reuse on an identical
/// re-evaluation, full invalidation of the edited cone, enclosures
/// bracketing the f64 result.
#[cfg(feature = "interval")]
#[test]
fn interval_memo_reuses_and_invalidates_like_f64() {
    use geom_core::{Bounds, Interval};
    let (doc, s) = subtract_doc(false);
    let opts = EvalOptions::default();
    let e1 = evaluate::<Interval>(&doc, None, &CancelToken::new(), &opts, Tol::witness());
    let e2 = evaluate::<Interval>(&doc, Some(&e1), &CancelToken::new(), &opts, Tol::witness());
    assert_eq!(
        e2.recomputed, 0,
        "identical doc must fully reuse at Interval"
    );
    assert_eq!(e2.reused, e1.recomputed + e1.reused);
    // The reused body is the SAME value (Merkle key match), and its
    // volume enclosure brackets the f64 volume.
    let vf = mass_properties(boolean_body(&run(&doc, None, false), s), Tol::witness())
        .unwrap()
        .volume;
    let vi = match &e2.value(s).unwrap().payload {
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
            mass_properties(body.as_ref(), Tol::witness())
                .unwrap()
                .volume
        }
        other => panic!("expected boolean body, got {}", other.kind_name()),
    };
    assert!(
        vi.lo() <= vf && vf <= vi.hi(),
        "enclosure [{},{}] must bracket {vf}",
        vi.lo(),
        vi.hi()
    );
    // Edit one operand's extrude distance: the cone invalidates.
    let extrude_id = doc
        .order()
        .iter()
        .copied()
        .find(|&id| matches!(doc.node(id), Some(Node::Extrude { .. })))
        .unwrap();
    let (doc2, _) = step(
        doc,
        DocEdit::SetParam {
            node: extrude_id,
            slot: SlotId::Distance,
            expr: len(1.75),
        },
    );
    let e3 = evaluate::<Interval>(&doc2, Some(&e2), &CancelToken::new(), &opts, Tol::witness());
    assert!(
        e3.recomputed >= 2,
        "edited extrude + subtract must recompute"
    );
    assert!(e3.value(s).is_some());
}

/// R3: edit a parameter, evaluate, edit BACK, evaluate against the
/// intermediate memo — the final body must be bit-identical to the
/// original scratch evaluation.
#[test]
fn edit_back_restores_bit_identical_bodies() {
    let d = die();
    let e0 = run(&d.doc, None, false);
    let fp0 = fingerprint(boolean_body(&e0, d.final_node));

    // Edit the +z pip transform's x-translation.
    let (moved, _) = step(
        d.doc.clone(),
        DocEdit::SetParam {
            node: d.pz_transform,
            slot: SlotId::Translation(editor_core::Axis3::X),
            expr: len(1.25),
        },
    );
    let e1 = run(&moved, Some(&e0), false);
    assert_ne!(
        fingerprint(boolean_body(&e1, d.final_node)),
        fp0,
        "the edit must actually change the body"
    );

    // Edit BACK to the original literal (1.0 = the +z pip x).
    let (back, _) = step(
        moved,
        DocEdit::SetParam {
            node: d.pz_transform,
            slot: SlotId::Translation(editor_core::Axis3::X),
            expr: len(1.0),
        },
    );
    let e2 = run(&back, Some(&e1), false);
    assert_eq!(
        fingerprint(boolean_body(&e2, d.final_node)),
        fp0,
        "edit-back must restore the ORIGINAL bits (memo through a stale prior)"
    );
    // Reverted transform + final subtract recompute; the rest reuses —
    // the die's seven sketch frames among them, since a slot edit on a
    // transform does not touch a plane.
    assert_eq!((e2.recomputed, e2.reused), (2, 82));
}

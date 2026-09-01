//! R1 review probes for MATE-1 (issue 945, PR #1400).
//!
//! These rows exist to attack the claims, not to re-state them. The
//! headline probe is the one the committed suite has no analogue of: a
//! cluster whose recorded frame `F` is NOT the identity, which is the
//! only condition under which the solve's `F⁻¹ · O_c⁻¹ · O_p · F`
//! conjugation is distinguishable from the bare `O_c⁻¹ · O_p` middle.
//! Every row in `mate1_member_vocab.rs` leaves every placement unset,
//! so `F = I` throughout and the conjugation is a no-op there.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, AxisSense, CancelToken, CapEnd, ContactClass, DocEdit, DocRef, DocumentId,
    EntityKind, EvalOptions, Evaluation, Expr, Frame, MateFrame, MatePrimitive, MateRole, Node,
    PartResolver, PatternKind, ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault, RoleSeg,
    StableName, clusters, content_pin, evaluate, solve_document,
};
use fixture::{desc, insert, len, scl, step};
use geom_core::Tol;

// ---- Substrate (mirrors the unit suite's stub resolver) ----

#[derive(Debug, Default)]
struct StubStore {
    docs: BTreeMap<DocumentId, ProfileDoc>,
}

impl StubStore {
    fn insert(&mut self, doc: ProfileDoc, tol: Tol) -> DocRef {
        let pin = content_pin(&doc, tol).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }
}

impl PartResolver for StubStore {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        let fail = |fault, message: &str| ResolveFailure {
            fault,
            message: message.to_string(),
        };
        let doc = self
            .docs
            .get(&doc_ref.id)
            .ok_or_else(|| fail(ResolveFault::Unresolved, "no such document"))?;
        let found = content_pin(doc, Tol::witness()).expect("the pin computes");
        if found != doc_ref.pin {
            return Err(fail(ResolveFault::PinMismatch, "the pin does not hold"));
        }
        Ok(doc.clone())
    }
}

fn opts(store: StubStore) -> EvalOptions {
    EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    }
}

fn run(doc: &ProfileDoc, o: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), o, Tol::witness())
}

fn block_part(label: &str, x: (f64, f64), y: (f64, f64), z0: f64, dz: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
        )),
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    );
    doc
}

fn leg_part(label: &str) -> ProfileDoc {
    block_part(label, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0)
}

fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(1),
                path: vec![RoleSeg::Cap(cap)],
            }),
        }],
    }
}

fn in_copy(pattern: RecipeNodeId, i: u32, master: StableName) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: pattern,
        path: vec![RoleSeg::Instance {
            i,
            of: Box::new(master),
        }],
    }
}

fn mate_frame(origin: [f64; 3], axis: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis,
        reference: [1.0, 0.0, 0.0],
    }
}

fn seat_mate(
    a: StableName,
    b: StableName,
    origin: [f64; 3],
    sense: AxisSense,
) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a,
        b,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: mate_frame(origin, [0.0, 0.0, 1.0]),
            b: mate_frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense,
            clocking: None,
        },
    }
}

fn near(a: Frame, b: Frame, tol: f64) -> bool {
    a.columns
        .iter()
        .flatten()
        .chain(a.translation.iter())
        .zip(b.columns.iter().flatten().chain(b.translation.iter()))
        .all(|(x, y)| (x - y).abs() <= tol)
}

// ---------------------------------------------------------------
// PROBE 1 — the conjugation, on a cluster frame that is not the
// identity. Hand-derived end to end; nothing read back from the solve.
// ---------------------------------------------------------------

/// The gauge (the leg) carries a recorded cluster frame `F` = rotate
/// +90° about z, then translate `(5, 7, 11)`. The leg is patterned
/// linearly along the master's own +x at spacing 2; the top seats onto
/// copy 2, ALIGNED, with the a-frame at the master's `[0, 0, 1]` and
/// the b-frame at its origin — same axis, same reference, so the
/// coincidence's representative is the bare translation `(0, 0, 1)`
/// and every rotation in the answer comes from `F` alone.
///
/// By hand, with `d = (2·spacing, 0, 0) = (4, 0, 0)` the copy's derived
/// offset and `R` the +90° z-rotation:
///
/// ```text
/// F⁻¹ ∘ O₂ ∘ F (x) = R⁻¹((R x + t) + d − t) = x + R⁻¹d
/// R⁻¹d = R_z(−90°)·(4, 0, 0) = (0, −4, 0)
/// rel_top = translate(0, −4, 0) ∘ translate(0, 0, 1) = translate(0, −4, 1)
/// ```
///
/// If the conjugation were dropped — `left = O_c⁻¹ ∘ O_p` bare — the
/// answer would be `translate(4, 0, 1)` instead. On the committed
/// suite's fixtures `F = I`, so those two are the SAME frame and no
/// committed row can tell them apart.
#[test]
fn r1_conjugation_through_a_non_identity_cluster_frame() {
    let spacing = 2.0;
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part("r1-conj-leg"), Tol::witness());
    let top_ref = store.insert(leg_part("r1-conj-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r1-conj"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(4),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(spacing),
            },
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(pattern, 2, in_part(leg, CapEnd::Top)),
                in_part(top, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let mate = mate.expect("the mate mints");

    // The cluster frame: rotate +90° about z, then translate.
    let f = Frame {
        columns: [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [5.0, 7.0, 11.0],
    };
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: leg,
            frame: f,
        },
    );

    assert_eq!(
        clusters(&doc),
        vec![vec![leg, top]],
        "the top joins the pattern's cluster; the leg is the gauge"
    );

    let poses = solve_document(&doc, Tol::witness());
    assert_eq!(poses.fault(mate), None, "the mate solves — no fault");
    assert_eq!(poses.role(mate), Some(MateRole::Determining));
    assert_eq!(poses.gauge(top), Some(leg));

    let got = poses.relative(top).expect("the top has a pose");

    // Hand-derived above; the bare-middle answer is the falsifier.
    let expected = Frame {
        columns: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.0, -4.0, 1.0],
    };
    let unconjugated = Frame {
        columns: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [4.0, 0.0, 1.0],
    };
    assert!(
        !near(expected, unconjugated, 1e-9),
        "the probe is only meaningful if the two candidate answers differ"
    );
    assert!(
        near(got, expected, 1e-12),
        "the derived offset must conjugate through the cluster frame:\n\
         got          {got:?}\n expected     {expected:?}\n \
         (unconjugated would be {unconjugated:?})"
    );

    // And the world placement: F ∘ rel_top = (9, 7, 12), which is also
    // O₂ ∘ F ∘ translate(0,0,1) computed the other way round.
    let world = poses.placement(&doc, top).expect("the top places");
    assert!(
        (world.translation[0] - 9.0).abs() < 1e-12
            && (world.translation[1] - 7.0).abs() < 1e-12
            && (world.translation[2] - 12.0).abs() < 1e-12,
        "the top's world origin sits on copy 2's seat: {:?}",
        world.translation
    );

    let ev = run(&doc, &opts(store));
    assert!(
        matches!(ev.result(mate), Some(editor_core::NodeResult::Ok(_))),
        "the mate evaluates: {:?}",
        ev.result(mate)
    );
}

// ---------------------------------------------------------------
// PROBE 2 — an OBLIQUE circular datum axis, off the origin, with a
// non-identity cluster frame. The expected offset is recomputed in the
// test by Rodrigues, independently of `derived_offset`.
// ---------------------------------------------------------------

/// Rodrigues rotation of `p` about the line through `o` with unit
/// direction `k` by `theta` — written here so the expectation does not
/// come from the code under test.
fn rodrigues(o: [f64; 3], k: [f64; 3], theta: f64, p: [f64; 3]) -> [f64; 3] {
    let n = (k[0] * k[0] + k[1] * k[1] + k[2] * k[2]).sqrt();
    let k = [k[0] / n, k[1] / n, k[2] / n];
    let v = [p[0] - o[0], p[1] - o[1], p[2] - o[2]];
    let (s, c) = theta.sin_cos();
    let kv = k[0] * v[0] + k[1] * v[1] + k[2] * v[2];
    let cross = [
        k[1] * v[2] - k[2] * v[1],
        k[2] * v[0] - k[0] * v[2],
        k[0] * v[1] - k[1] * v[0],
    ];
    [
        o[0] + v[0] * c + cross[0] * s + k[0] * kv * (1.0 - c),
        o[1] + v[1] * c + cross[1] * s + k[1] * kv * (1.0 - c),
        o[2] + v[2] * c + cross[2] * s + k[2] * kv * (1.0 - c),
    ]
}

/// Apply a `Frame` to a point.
fn apply(f: Frame, p: [f64; 3]) -> [f64; 3] {
    [
        f.columns[0][0] * p[0] + f.columns[1][0] * p[1] + f.columns[2][0] * p[2] + f.translation[0],
        f.columns[0][1] * p[0] + f.columns[1][1] * p[1] + f.columns[2][1] * p[2] + f.translation[1],
        f.columns[0][2] * p[0] + f.columns[1][2] * p[1] + f.columns[2][2] * p[2] + f.translation[2],
    ]
}

/// The mate's seat point, carried two independent ways to the same
/// world point. The solve puts the top at `W_top`; the seat is the
/// top's b-frame origin, so `W_top·(0,0,0)` must equal the copy's
/// version of the a-frame origin, `O_i ∘ F ∘ (0,0,1)`, where `O_i` is
/// recomputed here by Rodrigues about an OBLIQUE axis that misses the
/// origin. This is the composition-order attack: `O_i ∘ F` and
/// `F ∘ O_i` differ badly for this pair, as the row asserts.
#[test]
fn r1_oblique_circular_axis_with_a_non_identity_cluster_frame() {
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part("r1-obl-leg"), Tol::witness());
    let top_ref = store.insert(leg_part("r1-obl-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r1-obl"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));

    let axis_origin = [0.3, -1.2, 0.7];
    let axis_dir = [1.0, 2.0, 3.0];
    let theta = 0.7;
    let (doc, axis) = insert(
        doc,
        Node::Datum(editor_core::Datum::Axis {
            origin: [
                len(axis_origin[0]),
                len(axis_origin[1]),
                len(axis_origin[2]),
            ],
            direction: [scl(axis_dir[0]), scl(axis_dir[1]), scl(axis_dir[2])],
        }),
    );
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(3),
            kind: PatternKind::Circular {
                axis,
                step: Expr::literal(theta, editor_core::Dimension::Angle).expect("an angle"),
            },
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(pattern, 2, in_part(leg, CapEnd::Top)),
                in_part(top, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let mate = mate.expect("the mate mints");

    let f = Frame {
        columns: [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [5.0, 7.0, 11.0],
    };
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: leg,
            frame: f,
        },
    );

    let poses = solve_document(&doc, Tol::witness());
    assert_eq!(poses.fault(mate), None, "the oblique circular mate solves");
    let world = poses.placement(&doc, top).expect("the top places");

    // Independent: the a-frame origin (0,0,1) in the master's part
    // coordinates, carried by F into document coordinates, then
    // rotated twice about the oblique axis (copy index 2).
    let seat_master = apply(f, [0.0, 0.0, 1.0]);
    let expected_seat = rodrigues(axis_origin, axis_dir, theta * 2.0, seat_master);
    // The solve's answer for the same point: the top's b-frame origin.
    let got_seat = apply(world, [0.0, 0.0, 0.0]);

    // Order-of-composition falsifier: F ∘ O_i rather than O_i ∘ F.
    let wrong_order = apply(
        f,
        rodrigues(axis_origin, axis_dir, theta * 2.0, [0.0, 0.0, 1.0]),
    );
    let differs = (0..3)
        .map(|k| (wrong_order[k] - expected_seat[k]).abs())
        .fold(0.0_f64, f64::max);
    assert!(
        differs > 1e-3,
        "the probe is only meaningful if the two composition orders differ: {differs}"
    );

    for k in 0..3 {
        assert!(
            (got_seat[k] - expected_seat[k]).abs() < 1e-9,
            "the seat point disagrees on coordinate {k}:\n got      {got_seat:?}\n \
             expected {expected_seat:?}\n (wrong order would be {wrong_order:?})"
        );
    }
    let _ = store;
}

// ---------------------------------------------------------------
// PROBE 3 — per-instance freedom: try to author one.
// ---------------------------------------------------------------

/// Claim 4's last clause: NO mate can give one pattern copy a pose
/// apart from its siblings. The attack: mate a plain top to copy 1,
/// then read the whole cluster's poses — the pattern's copies have no
/// entry of their own anywhere in the solve, and the only instance
/// that moved is the top. A pattern copy is not a keyed vertex, so
/// there is no representation in which one copy could hold a pose.
#[test]
fn r1_no_mate_can_give_one_copy_a_pose_apart_from_its_siblings() {
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part("r1-freedom-leg"), Tol::witness());
    let top_ref = store.insert(leg_part("r1-freedom-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r1-freedom"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(pattern, 1, in_part(leg, CapEnd::Top)),
                in_part(top, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let mate = mate.expect("the mate mints");
    let poses = solve_document(&doc, Tol::witness());
    assert_eq!(poses.fault(mate), None);

    // The pattern node holds no pose and no gauge: copies are not
    // vertices, so per-instance freedom is unrepresentable.
    assert_eq!(poses.relative(pattern), None, "a pattern holds no pose");
    assert_eq!(poses.gauge(pattern), None, "a pattern has no gauge");
    // The master stays the gauge at the identity — the mate moved the
    // OTHER member, exactly as rule 2 says.
    assert_eq!(poses.relative(leg), Some(Frame::IDENTITY));

    // And the edit door refuses to place a pattern directly.
    let bad = editor_core::apply(
        &doc,
        &DocEdit::SetPlacement {
            node: pattern,
            frame: Frame::translation([1.0, 0.0, 0.0]),
        },
        Tol::witness(),
    );
    assert!(
        bad.is_err(),
        "a pattern node cannot carry a placement of its own: {bad:?}"
    );
    let _ = store;
}

// ---------------------------------------------------------------
// PROBE 4 — "absent by construction": pattern-free solves must be
// BIT-identical to the merge base. Executed, not inspected.
// ---------------------------------------------------------------

/// A chain of pattern-free mate documents whose solved relative poses
/// are dumped as raw f64 BIT PATTERNS. Run once on this branch and once
/// with the merge base's `src/mate/` checked out; the two dumps must be
/// byte-identical. Reading the diff cannot establish this — the new
/// `pair_left_factor` short-circuit is the whole claim.
///
/// Writes to `$R1_BITS_OUT` when set; otherwise just exercises the
/// documents (so the row is harmless in a normal run).
#[test]
fn r1_pattern_free_solves_are_bit_identical() {
    let mut lines: Vec<String> = Vec::new();

    // Three shapes: a plain seat, an OPPOSED seat, and a three-instance
    // chain — each with a non-identity cluster frame so `F` is live.
    for (tag, sense, chain, fr) in [
        ("seat-aligned", AxisSense::Aligned, false, Frame::IDENTITY),
        (
            "seat-opposed",
            AxisSense::Opposed,
            false,
            Frame {
                columns: [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
                translation: [5.0, 7.0, 11.0],
            },
        ),
        (
            "chain-3",
            AxisSense::Aligned,
            true,
            Frame {
                columns: [[1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, -1.0, 0.0]],
                translation: [-3.25, 0.5, 2.125],
            },
        ),
    ] {
        let mut store = StubStore::default();
        let a_ref = store.insert(leg_part(&format!("r1-bits-{tag}-a")), Tol::witness());
        let b_ref = store.insert(leg_part(&format!("r1-bits-{tag}-b")), Tol::witness());
        let c_ref = store.insert(leg_part(&format!("r1-bits-{tag}-c")), Tol::witness());
        let doc = ProfileDoc::empty(
            DocumentId::derive(&format!("r1-bits-{tag}")),
            Tol::witness(),
        );
        let (doc, a) = insert(doc, Node::instantiate_part(a_ref));
        let (doc, b) = insert(doc, Node::instantiate_part(b_ref));
        let (doc, c) = insert(doc, Node::instantiate_part(c_ref));
        let (doc, _m0) = step(
            doc,
            DocEdit::InsertNode {
                node: seat_mate(
                    in_part(a, CapEnd::Top),
                    in_part(b, CapEnd::Bottom),
                    [0.0, 0.0, 1.0],
                    sense,
                ),
            },
        );
        let doc = if chain {
            let (doc, _m1) = step(
                doc,
                DocEdit::InsertNode {
                    node: seat_mate(
                        in_part(b, CapEnd::Top),
                        in_part(c, CapEnd::Bottom),
                        [0.25, 0.0, 1.0],
                        AxisSense::Aligned,
                    ),
                },
            );
            doc
        } else {
            doc
        };
        let (doc, _) = step(doc, DocEdit::SetPlacement { node: a, frame: fr });

        let poses = solve_document(&doc, Tol::witness());
        for inst in [a, b, c] {
            let Some(f) = poses.relative(inst) else {
                lines.push(format!("{tag} {} NONE", inst.0));
                continue;
            };
            let bits: Vec<String> = f
                .columns
                .iter()
                .flatten()
                .chain(f.translation.iter())
                .map(|v| format!("{:016x}", v.to_bits()))
                .collect();
            lines.push(format!("{tag} {} {}", inst.0, bits.join(" ")));
            // The world placement too — that is what evaluation consumes.
            let w = poses.placement(&doc, inst).expect("it places");
            let wbits: Vec<String> = w
                .columns
                .iter()
                .flatten()
                .chain(w.translation.iter())
                .map(|v| format!("{:016x}", v.to_bits()))
                .collect();
            lines.push(format!("{tag} {} world {}", inst.0, wbits.join(" ")));
        }
        let _ = store;
    }

    let dump = lines.join("\n");
    assert!(!dump.is_empty());
    if let Ok(path) = std::env::var("R1_BITS_OUT") {
        std::fs::write(&path, &dump).expect("the dump writes");
    }
}

// ---------------------------------------------------------------
// PROBE 5 — what the "consistent loop VERIFIES" row actually gets.
// ---------------------------------------------------------------

/// The committed row `a_consistent_sibling_loop_declares_and_verifies`
/// accepts EITHER `Ok(assembly)` with two minted declarations OR
/// `Err(Uncertified { .. })` whose findings are all declines. Those are
/// materially different outcomes — one is "the gate verified the loop",
/// the other is "the gate could not certify it and said so" — and the
/// PR body claims only the first ("the gate verifies (no finding
/// against the document; both declarations minted)"). This row records
/// which branch the fixture actually takes, so the claim is checkable.
#[test]
fn r1_which_branch_does_the_consistent_loop_row_take() {
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part("r1-loopbranch-leg"), Tol::witness());
    let top_ref = store.insert(
        block_part("r1-loopbranch-top", (0.0, 2.5), (0.0, 1.0), 0.0, 0.5),
        Tol::witness(),
    );
    let doc = ProfileDoc::empty(DocumentId::derive("r1-loopbranch"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(1.5),
            },
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, _m0) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(pattern, 0, in_part(leg, CapEnd::Top)),
                in_part(top, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let (doc, _m1) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(pattern, 1, in_part(leg, CapEnd::Top)),
                in_part(top, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let ev = run(&doc, &opts(store));
    let result = editor_core::assemble(&doc, &ev, Tol::witness());
    let branch = match &result {
        Ok(a) => format!("Ok(minted = {})", a.minted.len()),
        Err(e) => format!("Err({e})"),
    };
    // Not an assertion about which is right — a recorded observation the
    // review report quotes.
    println!("R1-OBSERVED consistent-loop branch: {branch}");
    if let Ok(path) = std::env::var("R1_LOOP_OUT") {
        std::fs::write(&path, &branch).expect("the observation writes");
    }
}

// ---------------------------------------------------------------
// PROBE 6 — the two fence cases the PR body names but the committed
// suite does not build: a NESTED pattern and a pattern of a TRANSFORM.
// ---------------------------------------------------------------

/// The PR body discloses that "nested patterns and pattern-of-transform
/// refuse `DanglingHead`". The committed fence row builds neither — it
/// builds an out-of-range copy index and a pattern of an EXTRUDE. These
/// two rows build the disclosed shapes, so the disclosure is checked
/// rather than taken on trust.
#[test]
fn r1_nested_pattern_and_pattern_of_transform_refuse_dangling() {
    // (a) a pattern OF A PATTERN of an instance.
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part("r1-nested-leg"), Tol::witness());
    let top_ref = store.insert(leg_part("r1-nested-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r1-nested"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, inner) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, outer) = insert(
        doc,
        Node::Pattern {
            input: inner,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(0.0), scl(1.0), scl(0.0)],
                spacing: len(3.0),
            },
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, m) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(outer, 1, in_part(leg, CapEnd::Top)),
                in_part(top, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let m = m.expect("the mate mints");
    let poses = solve_document(&doc, Tol::witness());
    let fault = poses.fault(m).expect("a nested pattern head refuses");
    assert!(
        matches!(
            fault,
            editor_core::MateFault::DanglingHead { head, .. } if *head == outer
        ),
        "a nested pattern refuses DanglingHead at the OUTER pattern: {fault:?}"
    );
    let _ = store;

    // (b) a pattern of a TRANSFORM of an instance.
    let mut store2 = StubStore::default();
    let leg2 = store2.insert(leg_part("r1-xform-leg"), Tol::witness());
    let top2 = store2.insert(leg_part("r1-xform-top"), Tol::witness());
    let doc2 = ProfileDoc::empty(DocumentId::derive("r1-xform"), Tol::witness());
    let (doc2, li) = insert(doc2, Node::instantiate_part(leg2));
    let (doc2, xf) = insert(
        doc2,
        Node::Transform {
            input: li,
            translation: [len(0.0), len(0.0), len(0.5)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: Expr::literal(0.0, editor_core::Dimension::Angle).expect("an angle"),
        },
    );
    let (doc2, pat) = insert(
        doc2,
        Node::Pattern {
            input: xf,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc2, ti) = insert(doc2, Node::instantiate_part(top2));
    let (doc2, m2) = step(
        doc2,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(pat, 1, in_part(li, CapEnd::Top)),
                in_part(ti, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let m2 = m2.expect("the mate mints");
    let poses2 = solve_document(&doc2, Tol::witness());
    let fault2 = poses2
        .fault(m2)
        .expect("a pattern-of-transform head refuses");
    assert!(
        matches!(
            fault2,
            editor_core::MateFault::DanglingHead { head, .. } if *head == pat
        ),
        "a pattern of a transform refuses DanglingHead at the pattern: {fault2:?}"
    );
    let _ = store2;
}

// ---------------------------------------------------------------
// PROBE 7 — the fence is position-dependent: an out-of-range copy
// index refuses as a TREE edge but not as a DECLARING one.
// ---------------------------------------------------------------

/// `derived_offset` — the door that rejects an index at or past the
/// count, an unevaluable slot, a degenerate direction and an explicit
/// rule — is reached ONLY from `pair_left_factor`, which
/// `solve_cluster` calls only on TREE edges. A non-tree (declaring)
/// mate never has its member's offset derived, so the same malformed
/// head that refuses `DanglingHead` in the committed fence row goes
/// unrefused when a sibling seat gets the tree edge first.
///
/// The committed row `out_of_vocabulary_pattern_heads_still_refuse_
/// dangling` builds the copy-5-of-2 head as the document's ONLY mate,
/// i.e. exactly in the position where the check runs. This row builds
/// the same malformed head in the other position.
#[test]
fn r1_an_out_of_range_copy_escapes_the_fence_on_a_declaring_mate() {
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part("r1-escape-leg"), Tol::witness());
    let top_ref = store.insert(leg_part("r1-escape-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r1-escape"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    // A WELL-FORMED seat first: it takes the tree edge.
    let (doc, good) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(pattern, 0, in_part(leg, CapEnd::Top)),
                in_part(top, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    // Then the SAME malformed head the committed fence row uses:
    // copy 5 of a count-2 pattern. It closes a loop, so it declares.
    let (doc, bad) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(pattern, 5, in_part(leg, CapEnd::Top)),
                in_part(top, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let good = good.expect("the good mate mints");
    let bad = bad.expect("the malformed mate mints");

    let poses = solve_document(&doc, Tol::witness());
    assert_eq!(poses.role(good), Some(MateRole::Determining));

    // The observation: in the committed row this exact head refuses
    // `DanglingHead`. Here it does not — it is recorded as a healthy
    // DECLARING mate and carried to the gate as a real declaration.
    let fault = poses.fault(bad);
    let role = poses.role(bad);
    println!("R1-OBSERVED malformed declaring head: fault={fault:?} role={role:?}");
    assert_eq!(
        fault, None,
        "documenting the asymmetry: the out-of-range copy is NOT refused here"
    );
    assert_eq!(
        role,
        Some(MateRole::Declaring),
        "and it is carried as a live declaration"
    );

    // What the downstream gate makes of it — the question that decides
    // whether the escape is merely a misplaced fence or a real hole.
    let ev = run(&doc, &opts(store));
    let at_gate = editor_core::assemble(&doc, &ev, Tol::witness());
    let summary = match &at_gate {
        Ok(a) => format!("Ok(minted = {})", a.minted.len()),
        Err(e) => format!("Err({e})"),
    };
    println!("R1-OBSERVED escaped head at the gate: {summary}");
}

// ---------------------------------------------------------------
// PROBE 8 — the PR body's quoted red-first fault, reproduced.
// ---------------------------------------------------------------

/// Rebuilds the PR's four-legs-one-top fixture EXACTLY (leg = node 0,
/// pattern = 1, top = 2, mate = 3; the `a` side carries the pattern
/// head) and prints the solve's fault plus its Display. Run with the
/// merge base's `src/mate/` checked out, this should print the fault
/// the PR body quotes:
///
/// ```text
/// DanglingHead { mate: RecipeNodeId(3), side: A, head: RecipeNodeId(1) }
/// mate 3's a reference resolves through node 1, which is not a live
/// instance — rebind it
/// ```
///
/// On this branch it prints no fault at all.
#[test]
fn r1_reproduce_the_quoted_red_first_fault() {
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part("mate1-red-first-leg"), Tol::witness());
    let top_ref = store.insert(leg_part("mate1-red-first-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("mate1-red-first"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(4),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(pattern, 2, in_part(leg, CapEnd::Top)),
                in_part(top, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                AxisSense::Opposed,
            ),
        },
    );
    let mate = mate.expect("the mate mints");
    assert_eq!(
        (leg.0, pattern.0, top.0, mate.0),
        (0, 1, 2, 3),
        "the node ids the PR body's quote names"
    );
    let poses = solve_document(&doc, Tol::witness());
    let line = match poses.fault(mate) {
        Some(f) => format!("{f:?} | {f}"),
        None => "NO FAULT (the head resolves)".to_string(),
    };
    println!("R1-OBSERVED red-first fault: {line}");
    if let Ok(path) = std::env::var("R1_RED_OUT") {
        std::fs::write(&path, &line).expect("the observation writes");
    }
    let _ = store;
}

//! MATE-1 R2 review probes (blinded adversarial review lane R2).
//!
//! These rows attack the PR #1400 claims by execution:
//! - P1: the frame conjugation `F⁻¹·O_c⁻¹·O_p·F` at a NON-IDENTITY
//!   cluster frame, against a pose composed by hand (Rodrigues written
//!   in this file, never read back from the solver);
//! - P1b: the same conjugation with the GATE as the independent oracle
//!   (a consistent sibling loop under a rotated+translated cluster
//!   frame must still verify against the evaluated geometry);
//! - P2: a tree edge whose members are copies of TWO different
//!   patterns — the `(Some, Some)` arm of the pair's left factor;
//! - P3: the reversed tree direction — the patterned member as the
//!   tree CHILD (the `oc.inverse()` arm);
//! - P4: an out-of-range copy index on a DECLARING (non-tree) mate —
//!   the solve never derives its offset, so what refuses, and where?
//! - P5: a nested pattern head (pattern of a pattern) refuses
//!   `DanglingHead`, as the PR discloses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, AssemblyError, AxisSense, CancelToken, CapEnd, ContactClass, DocEdit, DocRef,
    DocumentId, EntityKind, EvalOptions, Evaluation, Expr, Frame, MateFrame, MatePrimitive,
    MateRole, Node, PartResolver, PatternKind, ProfileDoc, RecipeNodeId, ResolveFailure,
    ResolveFault, RoleSeg, StableName, assemble, content_pin, evaluate, solve_document,
};
use fixture::{insert, len, on_frame, scl, step};
use geom_core::Tol;

// ---- Substrate (as in the unit's own suite) ----

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
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
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

fn frame(origin: [f64; 3], axis: [f64; 3]) -> MateFrame {
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
            a: frame(origin, [0.0, 0.0, 1.0]),
            b: frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
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

// ---- Tiny hand algebra (this file's own, never the solver's) ----

/// Row-major 3x3 times column vector.
fn mv(m: [[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

fn mm(a: [[f64; 3]; 3], b: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut out = [[0.0; 3]; 3];
    for r in 0..3 {
        for c in 0..3 {
            out[r][c] = (0..3).map(|k| a[r][k] * b[k][c]).sum();
        }
    }
    out
}

fn transpose(m: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut out = [[0.0; 3]; 3];
    for r in 0..3 {
        for c in 0..3 {
            out[r][c] = m[c][r];
        }
    }
    out
}

/// A rigid map as (row-major linear part, translation), acting
/// `p ↦ R·p + t`.
#[derive(Clone, Copy, Debug)]
struct Rigid {
    r: [[f64; 3]; 3],
    t: [f64; 3],
}

impl Rigid {
    const IDENTITY: Rigid = Rigid {
        r: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        t: [0.0, 0.0, 0.0],
    };
    fn translation(t: [f64; 3]) -> Rigid {
        Rigid {
            r: Rigid::IDENTITY.r,
            t,
        }
    }
    /// Rotation by `angle` about the axis through `origin` along the
    /// (unnormalized) direction `d` — Rodrigues, written here.
    fn rotation_about_axis(origin: [f64; 3], d: [f64; 3], angle: f64) -> Rigid {
        let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        let u = [d[0] / n, d[1] / n, d[2] / n];
        let (s, c) = angle.sin_cos();
        let k = [[0.0, -u[2], u[1]], [u[2], 0.0, -u[0]], [-u[1], u[0], 0.0]];
        let mut r = [[0.0; 3]; 3];
        let kk = mm(k, k);
        for i in 0..3 {
            for j in 0..3 {
                let id = if i == j { 1.0 } else { 0.0 };
                r[i][j] = id + s * k[i][j] + (1.0 - c) * kk[i][j];
            }
        }
        // p ↦ R·(p − o) + o
        let ro = mv(r, origin);
        Rigid {
            r,
            t: [origin[0] - ro[0], origin[1] - ro[1], origin[2] - ro[2]],
        }
    }
    fn compose(self, other: Rigid) -> Rigid {
        // self ∘ other
        let r = mm(self.r, other.r);
        let ot = mv(self.r, other.t);
        Rigid {
            r,
            t: [ot[0] + self.t[0], ot[1] + self.t[1], ot[2] + self.t[2]],
        }
    }
    fn inverse(self) -> Rigid {
        let rt = transpose(self.r);
        let t = mv(rt, self.t);
        Rigid {
            r: rt,
            t: [-t[0], -t[1], -t[2]],
        }
    }
    /// As the editor's column-major `Frame`.
    fn as_frame(self) -> Frame {
        Frame {
            columns: [
                [self.r[0][0], self.r[1][0], self.r[2][0]],
                [self.r[0][1], self.r[1][1], self.r[2][1]],
                [self.r[0][2], self.r[1][2], self.r[2][2]],
            ],
            translation: self.t,
        }
    }
    fn from_frame(f: Frame) -> Rigid {
        Rigid {
            r: transpose([f.columns[0], f.columns[1], f.columns[2]]),
            t: f.translation,
        }
    }
}

/// The cluster frame used by the placed probes: a quarter turn about z
/// plus a translation — rotation AND translation, so a conjugation
/// written in either direction (or skipped) computes a DIFFERENT pose.
fn cluster_frame() -> Frame {
    Rigid {
        r: [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        t: [1.0, 2.0, 3.0],
    }
    .as_frame()
}

// ---- P1: hand-derived conjugation at F ≠ identity, oblique axis ----

/// PROBE (claim 1): a CIRCULAR pattern about an OBLIQUE axis (direction
/// (1,1,1), origin (1,0,0), step 2π/5), with the cluster's recorded
/// frame a rotation+translation. Expected relative pose composed by
/// hand in this file: `F⁻¹ ∘ O₁ ∘ F ∘ A` with `O₁` this file's own
/// Rodrigues and `A` the seat translation. The solver's output is
/// never read into the expectation.
#[test]
fn r2_oblique_circular_conjugation_at_a_placed_cluster_frame() {
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part("r2-obl-leg"), Tol::witness());
    let top_ref = store.insert(leg_part("r2-obl-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r2-obl"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: leg,
            frame: cluster_frame(),
        },
    );
    let (doc, axis) = insert(
        doc,
        Node::Datum(editor_core::Datum::Axis {
            origin: [len(1.0), len(0.0), len(0.0)],
            direction: [scl(1.0), scl(1.0), scl(1.0)],
        }),
    );
    let theta = 2.0 * core::f64::consts::PI / 5.0;
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(3),
            kind: PatternKind::Circular {
                axis,
                step: Expr::literal(theta, editor_core::Dimension::Angle)
                    .expect("an angle literal"),
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
    assert_eq!(
        poses.fault(mate),
        None,
        "the mate solves: {:?}",
        poses.fault(mate)
    );
    assert_eq!(poses.role(mate), Some(MateRole::Determining));

    let f = Rigid::from_frame(cluster_frame());
    let o1 = Rigid::rotation_about_axis([1.0, 0.0, 0.0], [1.0, 1.0, 1.0], theta);
    let a = Rigid::translation([0.0, 0.0, 1.0]);
    let expected = f.inverse().compose(o1).compose(f).compose(a).as_frame();
    let got = poses.relative(top).expect("the top has a pose");
    assert!(
        near(got, expected, 1e-12),
        "conjugation through the recorded frame, hand-derived:\n got      {got:?}\n expected {expected:?}"
    );
    let _ = store;
}

// ---- P1b: the gate as the oracle at F ≠ identity ----

/// PROBE (claims 1+3): the consistent sibling loop, with the cluster
/// frame rotated AND translated. The declared sibling seat is verified
/// by the gate against the EVALUATION's geometry, so a solve-side
/// conjugation error (either direction, or a skipped conjugation)
/// would put the top in the wrong place and the gate would refute —
/// an oracle independent of this reviewer's own algebra.
#[test]
fn r2_consistent_loop_still_verifies_under_a_placed_cluster_frame() {
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part("r2-loopf-leg"), Tol::witness());
    let top_ref = store.insert(
        block_part("r2-loopf-top", (0.0, 2.5), (0.0, 1.0), 0.0, 0.5),
        Tol::witness(),
    );
    let doc = ProfileDoc::empty(DocumentId::derive("r2-loopf"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: leg,
            frame: cluster_frame(),
        },
    );
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                // Document ŷ: the placed cluster frame turns the leg
                // and top a quarter turn about z, so the top's long
                // local-x side lies along WORLD ŷ — the copies must
                // march there for the loop to be consistent. (A first
                // draft marched along x̂ and the gate correctly refuted
                // it — the pattern offset is a document-coordinate map,
                // exactly as the PR states.)
                direction: [scl(0.0), scl(1.0), scl(0.0)],
                spacing: len(1.5),
            },
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, m0) = step(
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
    let (doc, m1) = step(
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
    let (m0, m1) = (m0.expect("mate 0 mints"), m1.expect("mate 1 mints"));

    let poses = solve_document(&doc, Tol::witness());
    assert_eq!(poses.fault(m0), None);
    assert_eq!(poses.fault(m1), None);
    assert_eq!(poses.role(m1), Some(MateRole::Declaring));

    // NOTE: the pattern direction is a DOCUMENT-coordinate map applied
    // outside the placement, so the copies march along document x̂ even
    // though the leg is rotated; the top must land so both seats hold.
    let ev = run(&doc, &opts(store));
    let result = assemble(&doc, &ev, Tol::witness());
    match &result {
        Ok(_) => {}
        Err(AssemblyError::Uncertified { findings, .. }) => {
            assert!(
                findings
                    .iter()
                    .all(|f| matches!(f.attribution, editor_core::Attribution::Declined(_))),
                "declines only: {findings:?}"
            );
        }
        Err(other) => panic!("the placed consistent loop verifies; the gate said: {other}"),
    }
}

// ---- P2: both members patterned (two patterns), tree edge ----

/// PROBE (claims 1+8): a tree edge whose BOTH members are pattern
/// copies — of two different patterns — exercises the
/// `O_c⁻¹ ∘ O_p` arm, which no committed row reaches. Expected pose
/// composed by hand: `T(s1·x̂ − s2·ŷ) ∘ T([0,0,1])`.
#[test]
fn r2_two_patterns_tree_edge_composes_both_offsets() {
    let mut store = StubStore::default();
    let l1 = store.insert(leg_part("r2-twop-l1"), Tol::witness());
    let l2 = store.insert(leg_part("r2-twop-l2"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r2-twop"), Tol::witness());
    let (doc, leg1) = insert(doc, Node::instantiate_part(l1));
    let (doc, p1) = insert(
        doc,
        Node::Pattern {
            input: leg1,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(3.0),
            },
        },
    );
    let (doc, leg2) = insert(doc, Node::instantiate_part(l2));
    let (doc, p2) = insert(
        doc,
        Node::Pattern {
            input: leg2,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(0.0), scl(1.0), scl(0.0)],
                spacing: len(5.0),
            },
        },
    );
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(p1, 1, in_part(leg1, CapEnd::Top)),
                in_copy(p2, 1, in_part(leg2, CapEnd::Bottom)),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let mate = mate.expect("the mate mints");

    let poses = solve_document(&doc, Tol::witness());
    assert_eq!(poses.fault(mate), None, "{:?}", poses.fault(mate));
    assert_eq!(poses.role(mate), Some(MateRole::Determining));
    assert_eq!(poses.gauge(leg2), Some(leg1), "leg1 is document-first");

    // O_p = T(3·x̂) (parent = leg1's copy 1), O_c = T(5·ŷ) (child =
    // leg2's copy 1); F = identity. rel(leg2) = O_c⁻¹∘O_p∘T([0,0,1]).
    let expected = Frame {
        columns: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [3.0, -5.0, 1.0],
    };
    let got = poses.relative(leg2).expect("leg2 has a pose");
    assert!(
        near(got, expected, 1e-12),
        "both offsets compose:\n got      {got:?}\n expected {expected:?}"
    );
    let _ = store;
}

// ---- P3: the patterned member as the tree CHILD ----

/// PROBE (claims 1+8): the top is document-FIRST, so it is the gauge
/// and the patterned member is the tree CHILD — the `O_c⁻¹` arm alone.
/// Hand-derived: rep = B∘A⁻¹ = T([0,0,−1]) (the mate reads a = copy,
/// b = top, and the child member's frame must land on the parent's),
/// rel(leg) = O_c⁻¹ ∘ rep = T([−2, 0, −1]).
#[test]
fn r2_patterned_member_as_tree_child_uses_the_inverse_offset() {
    let mut store = StubStore::default();
    let top_ref = store.insert(leg_part("r2-rev-top"), Tol::witness());
    let leg_ref = store.insert(leg_part("r2-rev-leg"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r2-rev"), Tol::witness());
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
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
    assert_eq!(poses.fault(mate), None, "{:?}", poses.fault(mate));
    assert_eq!(poses.gauge(leg), Some(top), "the top is document-first");

    let expected = Frame {
        columns: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [-2.0, 0.0, -1.0],
    };
    let got = poses.relative(leg).expect("the leg has a pose");
    assert!(
        near(got, expected, 1e-12),
        "the child member's offset enters inverted:\n got      {got:?}\n expected {expected:?}"
    );
    let _ = store;
}

// ---- P4: an out-of-range copy on a DECLARING mate ----

/// PROBE (claims 5+7+8): copy 5 of a count-2 pattern, but as the
/// SECOND (non-tree) mate of the pair graph — the solve never derives
/// its offset (only tree edges reach `derived_offset`), so the
/// committed fence row does not cover this shape. The document must
/// still refuse somewhere typed, or the nonsense declaration would
/// verify silently.
#[test]
fn r2_out_of_range_copy_on_a_declaring_mate_still_refuses_somewhere() {
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part("r2-oor-leg"), Tol::witness());
    let top_ref = store.insert(leg_part("r2-oor-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r2-oor"), Tol::witness());
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
    let (doc, m0) = step(
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
    let (doc, m1) = step(
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
    let (m0, m1) = (m0.expect("mate 0 mints"), m1.expect("mate 1 mints"));

    let poses = solve_document(&doc, Tol::witness());
    let solve_fault = poses.fault(m1).cloned();
    let role = poses.role(m1);

    let ev = run(&doc, &opts(store));
    let result = assemble(&doc, &ev, Tol::witness());
    // The probe's assertion: SOMETHING typed refuses this document —
    // either the solve faults the mate, or the gate refuses.
    let gate_refused = result.is_err();
    assert!(
        solve_fault.is_some() || gate_refused,
        "an out-of-range DECLARING copy must refuse somewhere: solve fault {solve_fault:?}, \
         role {role:?}, gate {result:?}"
    );
    // Record the shape for the report (printed on failure of the next
    // assertion if the refusal is somewhere surprising).
    eprintln!(
        "P4 shape: solve fault = {solve_fault:?}, role = {role:?}, gate = {:?}",
        result.as_ref().err()
    );
    let _ = m0;
}

// ---- P5: a nested pattern head ----

/// PROBE (claim 7): a pattern of a pattern — the head resolves through
/// the OUTER pattern whose input is the inner pattern, not a live
/// instance. The PR discloses this refuses `DanglingHead`; hold it to
/// that.
#[test]
fn r2_nested_pattern_head_refuses_dangling() {
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part("r2-nest-leg"), Tol::witness());
    let top_ref = store.insert(leg_part("r2-nest-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r2-nest"), Tol::witness());
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
                spacing: len(2.0),
            },
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(outer, 1, in_copy(inner, 1, in_part(leg, CapEnd::Top))),
                in_part(top, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let mate = mate.expect("the mate mints");
    let poses = solve_document(&doc, Tol::witness());
    let fault = poses.fault(mate).expect("a nested pattern head refuses");
    assert!(
        matches!(
            fault,
            editor_core::MateFault::DanglingHead { head, .. } if *head == outer
        ),
        "a nested pattern head is outside the vocabulary: {fault:?}"
    );
    let _ = store;
}

// ---- P6: plain-document pose bits (cross-revision instrument) ----

/// Not an assertion — an INSTRUMENT: prints the solved relative poses
/// of a mate chain with rotation+translation alignments, bit-exact, to
/// compare a run on this head against a run with the merge base's
/// `mate/` sources checked out (claim 2, absent-by-construction).
#[test]
fn r2_plain_document_pose_bits() {
    let mut store = StubStore::default();
    let a_ref = store.insert(leg_part("r2-bits-a"), Tol::witness());
    let b_ref = store.insert(leg_part("r2-bits-b"), Tol::witness());
    let c_ref = store.insert(leg_part("r2-bits-c"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r2-bits"), Tol::witness());
    let (doc, ia) = insert(doc, Node::instantiate_part(a_ref));
    let (doc, ib) = insert(doc, Node::instantiate_part(b_ref));
    let (doc, ic) = insert(doc, Node::instantiate_part(c_ref));
    let (doc, m0) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_part(ia, CapEnd::Top),
                in_part(ib, CapEnd::Bottom),
                [0.25, 0.5, 1.0],
                AxisSense::Opposed,
            ),
        },
    );
    let (doc, m1) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_part(ib, CapEnd::Top),
                in_part(ic, CapEnd::Bottom),
                [0.75, 0.125, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let _ = (m0.expect("m0 mints"), m1.expect("m1 mints"));
    let poses = solve_document(&doc, Tol::witness());
    for id in [ia, ib, ic] {
        let f = poses.relative(id).expect("a pose");
        let bits: Vec<u64> = f
            .columns
            .iter()
            .flatten()
            .chain(f.translation.iter())
            .map(|v| v.to_bits())
            .collect();
        eprintln!("P6 bits {}: {bits:x?}", id.0);
    }
    let _ = store;
}

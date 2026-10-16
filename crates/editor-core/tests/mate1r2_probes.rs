//! MATE-1 R2 review probes (blinded adversarial review lane R2).
//!
//! These rows attack the PR #1400 claims by execution:
//! - P1: the frame conjugation `F⁻¹·O_c⁻¹·O_p·F` at a NON-IDENTITY
//!   world frame `F` (the target's world pin), against a pose composed
//!   by hand (Rodrigues written in this file, never read back from the
//!   solver);
//! - P1b: the same conjugation with the GATE as the independent oracle
//!   (a seat under a rotated+translated world frame must still verify
//!   against the evaluated geometry);
//! - P2: a constraint whose two sides are copies of TWO different
//!   patterns — the `(Some, Some)` arm of the pair's left factor;
//! - P3: the reversed direction — the patterned member as the copy the
//!   constraint places (the `oc.inverse()` arm);
//! - P4: an out-of-range copy index on a SIBLING mate — one placing a
//!   second copy off the same pattern — so what refuses, and where?
//! - P5: a nested pattern head (pattern of a pattern) is a member,
//!   its copy chain carrying both levels.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::{
    Alignment, AssemblyError, AxisSense, CapEnd, ContactClass, DocEdit, DocumentId, Formula, Frame,
    Mate, MateFrame, MatePrimitive, MateRef, Node, PatternKind, Placement, ProfileDoc,
    RecipeNodeId, Space, StableName, assemble,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{
    ang, constrain, constrain_at, in_copy, insert, len, on_frame, run, scl, solve, step,
};
use geom_core::Tol;

// ---- Substrate (the shared resolver, `fixture::resolver`) ----

fn block_part(
    label: &str,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
            side: ExtrudeSide::Along,
        },
    )
}

fn leg_part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    block_part(label, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0)
}

/// **Each of `pattern`'s `copies` placed in the world**: a `Part` per
/// copy, each placed at the identity — the world a pattern's copies are
/// in the product by (A10). Copy 0 of a pattern over an instance is the
/// instance itself, already in the product, so a caller places it only
/// to have it twice.
fn place_copies(doc: ProfileDoc, pattern: RecipeNodeId, copies: &[i64]) -> ProfileDoc {
    copies.iter().fold(doc, |doc, &i| {
        let (doc, copy) = insert(
            doc,
            Node::Part {
                of: pattern.into(),
                select: editor_core::PartSelect::Instance(Formula::count(i)),
            },
        );
        fixture::place(doc, copy).0
    })
}

fn frame(origin: [f64; 3], axis: [f64; 3]) -> MateFrame<Formula> {
    MateFrame::authored(origin, axis, [1.0, 0.0, 0.0], geom_core::Tol::witness())
        .expect("a definite frame")
}

fn seat_mate(a: StableName, b: StableName, origin: [f64; 3], sense: AxisSense) -> Mate<Formula> {
    fixture::mate(
        fixture::head(a),
        fixture::head(b),
        ContactClass::Rest,
        Alignment {
            a: frame(origin, [0.0, 0.0, 1.0]),
            b: frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense,
            clocking: None,
        },
    )
}

/// **An instance pinned to the world at `pose`**: a bundle of one world
/// pin.
fn instance_at(
    doc: ProfileDoc,
    part: editor_core::DocRef,
    pose: Frame,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, inst) = insert(doc, fixture::mated_instance(part));
    let (doc, _) = constrain_at(doc, inst, Mate::world_pin(Placement::literal(&pose)));
    (doc, inst)
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

/// The leg's world pose in the pinned probes: a quarter turn about z
/// plus a translation — rotation AND translation, so a conjugation
/// written in either direction (or skipped) computes a DIFFERENT pose.
fn leg_frame() -> Frame {
    Rigid {
        r: [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        t: [1.0, 2.0, 3.0],
    }
    .as_frame()
}

// ---- P1: hand-derived conjugation at F ≠ identity, oblique axis ----

/// PROBE (claim 1): a CIRCULAR pattern about an OBLIQUE axis (direction
/// (1,1,1), origin (1,0,0), step 2π/5), with the leg pinned to the
/// world at a rotation+translation. Expected world pose composed by hand
/// in this file: `O₁ ∘ F ∘ A` with `O₁` this file's own Rodrigues and
/// `A` the seat translation — the copy's offset acts in document
/// coordinates, outside the leg's world frame — and the pose relative
/// to that frame `O₁ ∘ A`. The solver's output is never read into
/// the expectation.
#[test]
fn r2_oblique_circular_conjugation_at_a_pinned_world_frame() {
    let mut store = PartStore::default();
    let (leg_ref, leg_body) = store.insert_part(leg_part("r2-obl-leg"), Tol::witness());
    let (top_ref, top_body) = store.insert_part(leg_part("r2-obl-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r2-obl"), Tol::witness());
    let (doc, leg) = instance_at(doc, leg_ref, leg_frame());
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
            input: leg.into(),
            count: Formula::count(3),
            kind: PatternKind::Circular {
                axis: axis.into(),
                step: ang(theta),
            },
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, mate) = constrain(
        doc,
        seat_mate(
            in_copy(pattern, 1, in_part(leg, leg_body, CapEnd::End)),
            in_part(top, top_body, CapEnd::Start),
            [0.0, 0.0, 1.0],
            AxisSense::Aligned,
        ),
    );
    assert_eq!(mate.placement, top, "the mate places the top");

    let o = with_resolver(store);
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(
        poses.fault(top),
        None,
        "the mate solves: {:?}",
        poses.fault(top)
    );
    assert_eq!(poses.space(top), Some(Space::World));

    let f = Rigid::from_frame(leg_frame());
    let o1 = Rigid::rotation_about_axis([1.0, 0.0, 0.0], [1.0, 1.0, 1.0], theta);
    let a = Rigid::translation([0.0, 0.0, 1.0]);
    let expected = o1.compose(f).compose(a).as_frame();
    let got = poses.placement(&doc, top).expect("the top places");
    assert!(
        near(got, expected, 1e-12),
        "the copy's offset composes outside the leg's world frame, hand-derived:\n got      {got:?}\n expected {expected:?}"
    );
    let own = o1.compose(a).as_frame();
    let got = poses.relative(top).expect("the top has a pose");
    assert!(
        near(got, own, 1e-12),
        "and the pose relative to the leg's frame leaves it out:\n got      {got:?}\n expected {own:?}"
    );
    let _ = store;
}

// ---- P1b: the gate as the oracle at F ≠ identity ----

/// PROBE (claims 1+3): the top seated on pattern copy 1, with the leg's
/// world pose rotated AND translated; the top is as wide as one leg, so
/// it touches no other copy. The seat is verified by the gate
/// against the EVALUATION's geometry, so a solve-side conjugation error
/// (either direction, or a skipped conjugation) would put the top in
/// the wrong place and the gate would refute — an oracle independent of
/// this reviewer's own algebra. A second seat on the sibling copy
/// relates the top to a second target copy, which the door refuses
/// (`TargetsDiffer`, the stage's named interim: a bundle reads one
/// target copy).
#[test]
fn r2_a_seat_still_verifies_under_a_pinned_world_frame() {
    let mut store = PartStore::default();
    let (leg_ref, leg_body) = store.insert_part(leg_part("r2-loopf-leg"), Tol::witness());
    let (top_ref, top_body) = store.insert_part(leg_part("r2-loopf-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r2-loopf"), Tol::witness());
    let (doc, leg) = instance_at(doc, leg_ref, leg_frame());
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                // Document ŷ: the pattern offset is a document-coordinate
                // map, applied outside the leg's world pose.
                direction: [scl(0.0), scl(1.0), scl(0.0)],
                spacing: len(1.5),
            },
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let doc = place_copies(doc, pattern, &[1]);
    let (doc, m0) = constrain(
        doc,
        seat_mate(
            in_copy(pattern, 1, in_part(leg, leg_body, CapEnd::End)),
            in_part(top, top_body, CapEnd::Start),
            [0.0, 0.0, 1.0],
            AxisSense::Aligned,
        ),
    );
    assert_eq!(m0.placement, top, "the seat places the top");
    let sibling = fixture::door_refusal(
        &doc,
        seat_mate(
            in_copy(pattern, 0, in_part(leg, leg_body, CapEnd::End)),
            in_part(top, top_body, CapEnd::Start),
            [0.0, 0.0, 1.0],
            AxisSense::Aligned,
        ),
    );
    assert!(
        matches!(
            &sibling,
            editor_core::MateFault::TargetsDiffer { placement, constraints }
                if *placement == top && constraints.contains(&m0.id)
        ),
        "the sibling seat relates the top to a second target copy: {sibling:?}"
    );

    let o = with_resolver(store);
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.fault(top), None);
    assert_eq!(poses.space(top), Some(Space::World));

    // NOTE: the pattern direction is a DOCUMENT-coordinate map applied
    // outside the leg's world pose, so the copies march along document
    // ŷ even though the leg is rotated; the top must land so the seat
    // holds.
    let ev = run(&doc, &o);
    let result = assemble(&doc, &ev, Tol::witness());
    match &result {
        Ok(_) => {}
        Err(AssemblyError::Uncertified { findings, .. }) => {
            assert!(
                findings
                    .iter()
                    .all(|f| matches!(f.attribution, Some(editor_core::Attribution::Declined(_)))),
                "declines only: {findings:?}"
            );
        }
        Err(other) => panic!("the seat under a placed leg verifies; the gate said: {other}"),
    }
}

// ---- P2: both sides patterned (two patterns) ----

/// PROBE (claims 1+8): a constraint whose BOTH sides are pattern
/// copies — of two different patterns — exercises the
/// `O_c⁻¹ ∘ O_p` arm, which no committed row reaches. Expected pose
/// composed by hand: `T(s1·x̂ − s2·ŷ) ∘ T([0,0,1])`.
#[test]
fn r2_two_patterns_constraint_composes_both_offsets() {
    let mut store = PartStore::default();
    let (l1, l1_body) = store.insert_part(leg_part("r2-twop-l1"), Tol::witness());
    let (l2, l2_body) = store.insert_part(leg_part("r2-twop-l2"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r2-twop"), Tol::witness());
    let (doc, leg1) = insert(doc, Node::instantiate_part(l1));
    let (doc, p1) = insert(
        doc,
        Node::Pattern {
            input: leg1.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(3.0),
            },
        },
    );
    let (doc, leg2) = insert(doc, fixture::mated_instance(l2));
    let (doc, p2) = insert(
        doc,
        Node::Pattern {
            input: leg2.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(0.0), scl(1.0), scl(0.0)],
                spacing: len(5.0),
            },
        },
    );
    // Both sides read pattern copies, so the placement the mate places
    // is named: leg2, whose bundle is empty.
    let (doc, _) = constrain_at(
        doc,
        leg2,
        seat_mate(
            in_copy(p1, 1, in_part(leg1, l1_body, CapEnd::End)),
            in_copy(p2, 1, in_part(leg2, l2_body, CapEnd::Start)),
            [0.0, 0.0, 1.0],
            AxisSense::Aligned,
        ),
    );

    let o = with_resolver(store);
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.fault(leg2), None, "{:?}", poses.fault(leg2));
    assert_eq!(
        poses.space(leg2),
        Some(Space::World),
        "leg2 is pinned through leg1, which is on the world"
    );

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

// ---- P3: the patterned member as the copy placed ----

/// PROBE (claims 1+8): the top is a loose copy, the space the leg is
/// pinned in, and the patterned member is the copy the mate places —
/// the `O_c⁻¹` arm alone.
/// Hand-derived: rep = B∘A⁻¹ = T([0,0,−1]) (the mate reads a = copy,
/// b = top, and the child member's frame must land on the parent's),
/// rel(leg) = O_c⁻¹ ∘ rep = T([−2, 0, −1]).
#[test]
fn r2_patterned_member_as_the_placed_copy_uses_the_inverse_offset() {
    let mut store = PartStore::default();
    let (top_ref, top_body) = store.insert_part(leg_part("r2-rev-top"), Tol::witness());
    let (leg_ref, leg_body) = store.insert_part(leg_part("r2-rev-leg"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r2-rev"), Tol::witness());
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    // The leg sits where its mate puts it: its bundle starts empty.
    let (doc, leg) = insert(doc, fixture::mated_instance(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, _) = constrain_at(
        doc,
        leg,
        seat_mate(
            in_copy(pattern, 1, in_part(leg, leg_body, CapEnd::End)),
            in_part(top, top_body, CapEnd::Start),
            [0.0, 0.0, 1.0],
            AxisSense::Aligned,
        ),
    );

    let o = with_resolver(store);
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.fault(leg), None, "{:?}", poses.fault(leg));
    assert_eq!(
        poses.space(leg),
        Some(Space::Loose { root: top }),
        "the leg is pinned to the top, a loose copy"
    );

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

// ---- P4: an out-of-range copy on a SIBLING mate ----

/// PROBE (claims 5+7+8): a copy past the pattern's count, read by the
/// SECOND of two mates that each place a top on a copy of one pattern
/// — the pattern shrunk under a mate that named copy 1, since a head
/// past the count at insert is the edit door's to refuse.
///
/// It refuses at the SOLVE, naming the pattern — not "somewhere" and
/// not at the gate. The index-against-the-count check is a fact about
/// a REFERENCE, so it runs where the solve reads each reference, once
/// per side of every live constraint, and the sibling on copy 0 is
/// unaffected.
#[test]
fn r2_an_out_of_range_copy_on_a_sibling_mate_refuses_at_the_solve() {
    let mut store = PartStore::default();
    let (leg_ref, leg_body) = store.insert_part(leg_part("r2-oor-leg"), Tol::witness());
    let (top_ref, top_body) = store.insert_part(leg_part("r2-oor-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r2-oor"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref.clone()));
    let (doc, top2) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, m0) = constrain(
        doc,
        seat_mate(
            in_copy(pattern, 0, in_part(leg, leg_body, CapEnd::End)),
            in_part(top, top_body, CapEnd::Start),
            [0.0, 0.0, 1.0],
            AxisSense::Aligned,
        ),
    );
    // The second mate names copy 1, well formed at insert (the edit
    // door refuses a head that resolves to no member where it is
    // authored); the pattern then shrinks to one copy under it, which
    // is how a head stops resolving after insert (N5).
    let (doc, m1) = constrain(
        doc,
        seat_mate(
            in_copy(pattern, 1, in_part(leg, leg_body, CapEnd::End)),
            in_part(top2, top_body, CapEnd::Start),
            [0.0, 0.0, 1.0],
            AxisSense::Aligned,
        ),
    );
    let (m0, m1): (MateRef, MateRef) = (m0, m1);
    assert_eq!((m0.placement, m1.placement), (top, top2));
    let (doc, _) = step(
        doc,
        DocEdit::SetStructuralParam {
            node: pattern,
            slot: editor_core::SlotId::Count,
            expr: Formula::count(1),
            fresh: Vec::new(),
        },
    );

    let o = with_resolver(store);
    let poses = solve(&doc, &o, Tol::witness());
    let fault = poses
        .fault(top2)
        .cloned()
        .expect("an out-of-range copy refuses at the solve");
    assert!(
        matches!(
            fault,
            editor_core::MateFault::DanglingHead { head, .. } if head == pattern
        ),
        "the refusal names the pattern whose count the index is past: {fault:?}"
    );
    assert_eq!(
        poses.space(top2),
        None,
        "and the refused copy is posed in no space"
    );
    // The well-formed sibling is unaffected: one constraint's refusal
    // is not its sibling's, and not the document's.
    assert_eq!(poses.space(top), Some(Space::World));
    assert!(poses.fault(top).is_none());
}

// ---- P5: a nested pattern head ----

/// PROBE (claim 7): a pattern of a pattern — the head resolves through
/// the OUTER pattern whose input is the inner pattern, and on down
/// through the inner one to the instance that mints the name. Both
/// `Instance(i)` qualifiers are in the name and the walk consumes
/// both, so the reference is a MEMBER and the solve places it.
///
/// (The document does not GATHER: `Node::Pattern` takes one body and
/// a pattern's value is many, so the outer pattern refuses
/// `WrongOperand` at the evaluation. That fence is the node
/// vocabulary's, not the member vocabulary's, and the shape a user
/// builds a nested copy through is `Part { Instance(i) }` between the
/// two patterns — `msolve2_member_chain`'s ground.)
#[test]
fn r2_nested_pattern_head_is_a_member() {
    let mut store = PartStore::default();
    let (leg_ref, leg_body) = store.insert_part(leg_part("r2-nest-leg"), Tol::witness());
    let (top_ref, top_body) = store.insert_part(leg_part("r2-nest-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r2-nest"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, inner) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, outer) = insert(
        doc,
        Node::Pattern {
            input: inner.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(0.0), scl(1.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, mate) = constrain(
        doc,
        seat_mate(
            in_copy(
                outer,
                1,
                in_copy(inner, 1, in_part(leg, leg_body, CapEnd::End)),
            ),
            in_part(top, top_body, CapEnd::Start),
            [0.0, 0.0, 1.0],
            AxisSense::Aligned,
        ),
    );
    assert_eq!(mate.placement, top);
    let o = with_resolver(store);
    let poses = solve(&doc, &o, Tol::witness());
    assert!(
        poses.fault(top).is_none(),
        "a nested pattern head resolves through both levels: {:?}",
        poses.fault(top)
    );
    assert_eq!(
        poses.space(top),
        Some(Space::World),
        "the nested copy's reference places the top"
    );
    let _ = inner;
}

// ---- P6: plain-document pose bits (cross-revision instrument) ----

/// Not an assertion — an INSTRUMENT: prints the solved relative poses
/// of a mate chain with rotation+translation alignments, bit-exact, to
/// compare a run on this head against a run with the merge base's
/// `mate/` sources checked out (claim 2, absent-by-construction).
#[test]
fn r2_plain_document_pose_bits() {
    let mut store = PartStore::default();
    let (a_ref, a_body) = store.insert_part(leg_part("r2-bits-a"), Tol::witness());
    let (b_ref, b_body) = store.insert_part(leg_part("r2-bits-b"), Tol::witness());
    let (c_ref, c_body) = store.insert_part(leg_part("r2-bits-c"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r2-bits"), Tol::witness());
    let (doc, ia) = insert(doc, Node::instantiate_part(a_ref));
    let (doc, ib) = insert(doc, fixture::mated_instance(b_ref));
    let (doc, ic) = insert(doc, fixture::mated_instance(c_ref));
    let (doc, m0) = constrain(
        doc,
        seat_mate(
            in_part(ia, a_body, CapEnd::End),
            in_part(ib, b_body, CapEnd::Start),
            [0.25, 0.5, 1.0],
            AxisSense::Opposed,
        ),
    );
    let (doc, m1) = constrain(
        doc,
        seat_mate(
            in_part(ib, b_body, CapEnd::End),
            in_part(ic, c_body, CapEnd::Start),
            [0.75, 0.125, 1.0],
            AxisSense::Aligned,
        ),
    );
    assert_eq!((m0.placement, m1.placement), (ib, ic));
    let o = with_resolver(store);
    let poses = solve(&doc, &o, Tol::witness());
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

//! MATE-1 R2 review probes (blinded adversarial review lane R2).
//!
//! These rows attack the PR #1400 claims by execution:
//! - P1: a seat on a pattern copy at a NON-IDENTITY world frame `F`
//!   (the target's world pin), against the plane the seat pins composed
//!   by hand (Rodrigues written in this file, never read back from the
//!   solver);
//! - P1b: the same seat with the GATE as the independent oracle (a seat
//!   under a rotated+translated world frame must still verify against
//!   the evaluated geometry);
//! - P3: the reversed direction — a mate whose `on` reads a pattern
//!   copy of the placed instance rather than the instance itself;
//! - P4: an out-of-range copy index on a SIBLING mate — one placing a
//!   second copy off the same pattern — so what refuses, and where?

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    AssemblyError, CapEnd, ContactClass, Coord, DocEdit, DocumentId, EvalOptions, Formula, Frame,
    MateRef, MateSide, Node, NodeErrorKind, PatternKind, ProfileDoc, RecipeNodeId, SlideAxis,
    Space, StableName, Value, assemble,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{ang, constrain, in_copy, insert, len, on_frame, run, scl, solve, step};
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
/// to have it twice. `o` reaches the parts the pins evaluate.
fn place_copies(
    doc: ProfileDoc,
    pattern: RecipeNodeId,
    copies: &[i64],
    o: &EvalOptions,
) -> ProfileDoc {
    copies.iter().fold(doc, |doc, &i| {
        let (doc, copy) = insert(
            doc,
            Node::Part {
                of: pattern.into(),
                select: editor_core::PartSelect::Instance(Formula::count(i)),
            },
        );
        fixture::insert_pinned(
            doc,
            Node::place(vec![editor_core::BodyRead::plain(copy)]),
            o,
        )
        .0
    })
}

/// **`placed`'s face seated on `target`'s**: a plane mate — the faces
/// facing each other (`facing`, a rest), or their outward normals
/// equated — with the three freedoms it leaves set: `placed`'s origin
/// slid `slide` from `target`'s in the plane, and no spin. Added to the
/// placement `placed` reads.
fn seat(
    doc: ProfileDoc,
    target: StableName,
    placed: StableName,
    slide: [f64; 2],
    facing: bool,
) -> (ProfileDoc, MateRef) {
    let to = fixture::plane(fixture::head(target));
    let to = if facing { fixture::flip(to) } else { to };
    let (doc, mate) = constrain(
        doc,
        fixture::mate(
            fixture::plane(fixture::head(placed)),
            to,
            ContactClass::Rest,
        ),
    );
    let doc = [
        (Coord::Slide(SlideAxis::X), len(slide[0])),
        (Coord::Slide(SlideAxis::Y), len(slide[1])),
        (Coord::Spin, ang(0.0)),
    ]
    .into_iter()
    .fold(doc, |doc, (coord, by)| {
        step(
            doc,
            DocEdit::AddValue {
                placement: mate.placement,
                value: Box::new(Value {
                    mate: mate.id,
                    coord,
                    by,
                }),
                fresh: Vec::new(),
            },
        )
        .0
    });
    (doc, mate)
}

/// **An instance pinned to the world at `pose`**: a bundle of one world
/// pin, read off its geometry through `o`.
fn instance_at(
    doc: ProfileDoc,
    part: editor_core::DocRef,
    pose: Frame,
    o: &EvalOptions,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, inst) = insert(doc, fixture::mated_instance(part));
    (fixture::pin(doc, inst, &pose, o), inst)
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

// ---- P1: hand-derived seat at F ≠ identity, oblique axis ----

/// PROBE (claim 1): a CIRCULAR pattern about an OBLIQUE axis (direction
/// (1,1,1), origin (1,0,0), step 2π/5), with the leg pinned to the
/// world at a rotation+translation, and the top seated on copy 1. The
/// plane the seat pins is composed by hand in this file: copy 1's end
/// cap is the leg's `z = 1` plane under `O₁ ∘ F`, with `O₁` this file's
/// own Rodrigues — the copy's offset acts in document coordinates,
/// outside the leg's world frame. The top's own `z` axis is that cap's
/// normal and its start cap (through its own origin) lies in that
/// plane. The solver's output is never read into the expectation.
#[test]
fn r2_oblique_circular_seat_at_a_pinned_world_frame() {
    let mut store = PartStore::default();
    let (leg_ref, leg_body) = store.insert_part(leg_part("r2-obl-leg"), Tol::witness());
    let (top_ref, top_body) = store.insert_part(leg_part("r2-obl-top"), Tol::witness());
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("r2-obl"), Tol::witness());
    let (doc, leg) = instance_at(doc, leg_ref, leg_frame(), &o);
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
    let (doc, mate) = seat(
        doc,
        in_copy(pattern, 1, in_part(leg, leg_body, CapEnd::End)),
        in_part(top, top_body, CapEnd::Start),
        [0.0, 0.0],
        true,
    );
    assert_eq!(mate.placement, top, "the mate places the top");

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
    let cap = o1.compose(f).compose(Rigid::translation([0.0, 0.0, 1.0]));
    let normal = [cap.r[0][2], cap.r[1][2], cap.r[2][2]];
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let got = Rigid::from_frame(poses.placement(&doc, top).expect("the top places"));
    let z = [got.r[0][2], got.r[1][2], got.r[2][2]];
    assert!(
        z.iter().zip(normal).all(|(a, b)| (a - b).abs() <= 1e-12),
        "the top's z is copy 1's cap normal, hand-derived:\n got      {z:?}\n expected {normal:?}"
    );
    assert!(
        (dot(normal, got.t) - dot(normal, cap.t)).abs() <= 1e-12,
        "the top's start cap lies in copy 1's end cap plane: {:?} against {:?}",
        got.t,
        cap.t
    );
}

// ---- P1b: the gate as the oracle at F ≠ identity ----

/// PROBE (claims 1+3): the top seated on pattern copy 1, with the leg's
/// world pose rotated AND translated; the top is as wide as one leg, so
/// it touches no other copy. The seat is verified by the gate
/// against the EVALUATION's geometry, so a solve that puts the top in
/// the wrong place makes the gate refute — an oracle independent of
/// this reviewer's own algebra.
#[test]
fn r2_a_seat_still_verifies_under_a_pinned_world_frame() {
    let mut store = PartStore::default();
    let (leg_ref, leg_body) = store.insert_part(leg_part("r2-loopf-leg"), Tol::witness());
    let (top_ref, top_body) = store.insert_part(leg_part("r2-loopf-top"), Tol::witness());
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("r2-loopf"), Tol::witness());
    let (doc, leg) = instance_at(doc, leg_ref, leg_frame(), &o);
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
    let doc = place_copies(doc, pattern, &[1], &o);
    // A value on a pattern copy's target is charted on its space's
    // frame — the world's here — so the top's slides are where copy 1's
    // footprint lies in the world: the leg's unit square turned a
    // quarter about z and moved by (1, 2), then 1.5 along ŷ.
    let (doc, m0) = seat(
        doc,
        in_copy(pattern, 1, in_part(leg, leg_body, CapEnd::End)),
        in_part(top, top_body, CapEnd::Start),
        [0.0, 3.5],
        true,
    );
    assert_eq!(m0.placement, top, "the seat places the top");

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

// ---- P3: a pattern copy of the placed instance as `on` ----

/// PROBE (claims 1+8): the leg is the copy the mate places, and the
/// mate's `on` reads copy 1 of a pattern over the leg rather than the
/// leg itself. A pattern copy is other geometry than the instance's own
/// output, so the mate pins nothing of the leg: the door refuses it
/// `NotOnTheCopy`, naming the mate it would have added.
#[test]
fn r2_a_pattern_copy_of_the_placed_instance_is_not_its_copy() {
    let mut store = PartStore::default();
    let (top_ref, top_body) = store.insert_part(leg_part("r2-rev-top"), Tol::witness());
    let (leg_ref, leg_body) = store.insert_part(leg_part("r2-rev-leg"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r2-rev"), Tol::witness());
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
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
    let refused = fixture::at_the_door_of(
        &doc,
        &editor_core::RefusingReach,
        leg,
        fixture::rest(
            fixture::head(in_copy(pattern, 1, in_part(leg, leg_body, CapEnd::End))),
            fixture::head(in_part(top, top_body, CapEnd::Start)),
            ContactClass::Rest,
        ),
    );
    match refused {
        Err((at, editor_core::MateFault::NotOnTheCopy { mate })) => {
            assert_eq!(
                (at, mate.placement),
                (leg, leg),
                "the refusal names the leg's mate"
            );
        }
        Err((at, fault)) => panic!("expected NotOnTheCopy at the leg, got {at:?}: {fault}"),
        Ok(_) => panic!("a mate whose `on` reads a pattern copy pins nothing of the leg"),
    }
}

// ---- P4: an out-of-range copy on a SIBLING mate ----

/// PROBE (claims 5+7+8): a copy past the pattern's count, read by the
/// SECOND of two mates that each place a top on a copy of one pattern
/// — the pattern shrunk under a mate that named copy 1.
///
/// It refuses where the second top's placement EVALUATES, naming the
/// mate and its target pose, whose selection no longer resolves — not
/// "somewhere" and not at the gate. A pose is read once per mate, where
/// its placement folds, and the sibling on copy 0 is unaffected.
#[test]
fn r2_an_out_of_range_copy_on_a_sibling_mate_refuses_at_its_placement() {
    let mut store = PartStore::default();
    let (leg_ref, leg_body) = store.insert_part(leg_part("r2-oor-leg"), Tol::witness());
    let (top_ref, top_body) = store.insert_part(leg_part("r2-oor-top"), Tol::witness());
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("r2-oor"), Tol::witness());
    let (doc, leg) = fixture::insert_pinned(doc, Node::instantiate_part(leg_ref), &o);
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
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, top2) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, m0) = seat(
        doc,
        in_copy(pattern, 0, in_part(leg, leg_body, CapEnd::End)),
        in_part(top, top_body, CapEnd::Start),
        [0.0, 0.0],
        true,
    );
    // The second mate names copy 1, live when it is authored; the
    // pattern then shrinks to one copy under it, which is how a read
    // stops resolving after insert (N5).
    let (doc, m1) = seat(
        doc,
        in_copy(pattern, 1, in_part(leg, leg_body, CapEnd::End)),
        in_part(top2, top_body, CapEnd::Start),
        [0.0, 0.0],
        true,
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

    let poses = solve(&doc, &o, Tol::witness());
    let fault = poses
        .fault(top2)
        .cloned()
        .expect("an out-of-range copy refuses where its placement evaluates");
    assert!(
        matches!(
            &fault,
            editor_core::MateFault::SideUnevaluated { mate, side: MateSide::To, refusal }
                if *mate == m1 && matches!(refusal.kind(), NodeErrorKind::SelectResolve { .. })
        ),
        "the refusal names the mate whose target's selection the count left: {fault:?}"
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
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("r2-bits"), Tol::witness());
    let (doc, ia) = fixture::insert_pinned(doc, Node::instantiate_part(a_ref), &o);
    let (doc, ib) = insert(doc, fixture::mated_instance(b_ref));
    let (doc, ic) = insert(doc, fixture::mated_instance(c_ref));
    let (doc, m0) = seat(
        doc,
        in_part(ia, a_body, CapEnd::End),
        in_part(ib, b_body, CapEnd::Start),
        [0.25, 0.5],
        false,
    );
    let (doc, m1) = seat(
        doc,
        in_part(ib, b_body, CapEnd::End),
        in_part(ic, c_body, CapEnd::Start),
        [0.75, 0.125],
        true,
    );
    assert_eq!((m0.placement, m1.placement), (ib, ic));
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
}

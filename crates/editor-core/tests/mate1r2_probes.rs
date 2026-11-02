//! MATE-1 R2 review probes (blinded adversarial review lane R2).
//!
//! These rows attack the PR #1400 claims by execution:
//! - P1: a seat on a pattern copy at a NON-IDENTITY world frame `F`
//!   (the target's world mate), against the plane the seat pins composed
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
use editor_core::Dimension;

use editor_core::{
    CapEnd, Coord, DocumentId, EvalOptions, Formula, Frame, MateRef, MateSide, Node, NodeErrorKind,
    ProfileDoc, RecipeNodeId, TargetAxis, VarId,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{in_copy, insert, len, on_frame, run, solve};
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

/// **`placed`'s face seated on `target`'s**: a plane mate — the faces
/// facing each other (`facing`, a rest), or their outward normals
/// equated — with the three freedoms it leaves set: `placed`'s origin
/// slid `slide` from `target`'s in the plane, and no spin. Added to the
/// placement `placed` reads.
fn seat(
    doc: ProfileDoc,
    target: impl crate::fixture::Sited,
    placed: impl crate::fixture::Sited,
    slide: [f64; 2],
    facing: bool,
    upended: bool,
) -> (ProfileDoc, MateRef) {
    let to = fixture::plane(fixture::head(target));
    let to = if facing { fixture::flip(to) } else { to };
    let mut mate = fixture::mate(fixture::plane(fixture::head(placed)), to);
    // Where the start cap faces the mated direction's opposite — unflipped
    // against an end cap, or flipped against one an upended copy holds
    // facing down — the copy stands upended, its `z` along `−z`.
    if upended {
        mate.orient = editor_core::Orientation::new(
            editor_core::SignedAxis::PlusX,
            editor_core::SignedAxis::MinusZ,
        )
        .expect("x and z lie along two axes");
    }
    let (doc, mate) = crate::fixture::constrain_with(
        doc,
        mate,
        &[
            (Coord::Slide(TargetAxis::X), len(slide[0])),
            (Coord::Slide(TargetAxis::Y), len(slide[1])),
            (Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    );
    (doc, mate)
}

/// **An instance pinned to the world at `pose`**: a bundle of one world
/// mate, read off its geometry through `o`.
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

// ---- P1: a seat at F ≠ identity, an axis off the origin ----

/// PROBE (claim 1): a CIRCULAR family about an axis that misses the
/// origin (direction z, through (1,0,0), step 2π/5), with the leg pinned to the
/// world at a rotation+translation, and one top seated on copy 1 and
/// another on copy 0. The axis is read off the placed leg, so it is
/// authored in the leg's own coordinates (the document's line carried
/// back through `F`), and the line it reads is the document's. Copy 1
/// is copy 0 turned by the step about that line in document
/// coordinates, so the plane the top on copy 1 is seated in is the one
/// the top on copy 0 is seated in under the turn, composed here by
/// Rodrigues; where member 0 stands is the ring's chart
/// (`mate/value.rs`) and not this probe's question.
///
/// *Breaks if* the member's turn is composed inside `F` rather than
/// outside it, or about the wrong line.
#[test]
fn r2_off_origin_circular_seat_at_a_pinned_world_frame() {
    let mut store = PartStore::default();
    let (leg_ref, leg_body) = store.insert_part(leg_part("r2-obl-leg"), Tol::witness());
    let (top_ref, top_body) = store.insert_part(leg_part("r2-obl-top"), Tol::witness());
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("r2-obl"), Tol::witness());
    let (doc, leg) = instance_at(doc, leg_ref, leg_frame(), &o);
    let f = Rigid::from_frame(leg_frame());
    let unturn = |v: [f64; 3]| mv(transpose(f.r), v);
    let own_origin = unturn([1.0 - f.t[0], 0.0 - f.t[1], 0.0 - f.t[2]]);
    let axis =
        crate::mate1_member_vocab::axis_with(&doc, leg, own_origin, unturn([0.0, 0.0, 1.0]), &o);
    let theta = 2.0 * core::f64::consts::PI / 5.0;
    let (doc, k, pattern) = crate::fixture::circular(doc, leg, (axis, TargetAxis::Z), theta, 3);
    let mut doc = doc;
    let mut tops = Vec::new();
    for member in [0, 1] {
        let (next, top) = insert(doc, fixture::mated_instance(top_ref));
        let (next, mate) = seat(
            next,
            in_copy(pattern, k, member, in_part(leg, leg_body, CapEnd::End)),
            in_part(top, top_body, CapEnd::Start),
            [0.0, 0.0],
            true,
            false,
        );
        assert_eq!(mate.placement, top, "the mate places the top");
        doc = next;
        tops.push(top);
    }

    let poses = solve(&doc, &o, Tol::witness());
    for &top in &tops {
        assert_eq!(
            poses.fault(top),
            None,
            "the mate solves: {:?}",
            poses.fault(top)
        );
        assert_eq!(poses.space(top), Some(VarId::WORLD));
    }
    let placed = |top| Rigid::from_frame(poses.placement(&doc, top).expect("the top places"));
    let (on0, on1) = (placed(tops[0]), placed(tops[1]));
    // The seat's values are charted on the space's axes (D′2), so what
    // the turn carries is the seated plane: its normal (each top's
    // `z`) and its offset.
    let want = Rigid::rotation_about_axis([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], theta).compose(on0);
    let z = |r: Rigid| [r.r[0][2], r.r[1][2], r.r[2][2]];
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    assert!(
        z(on1)
            .iter()
            .zip(z(want))
            .all(|(a, b)| (a - b).abs() <= 1e-12),
        "the top on copy 1 faces as the top on copy 0 turned:\n got      {:?}\n expected {:?}",
        z(on1),
        z(want)
    );
    assert!(
        (dot(z(want), on1.t) - dot(z(want), want.t)).abs() <= 1e-12,
        "the top on copy 1 lies in copy 0's seat plane turned: {:?} against {:?}",
        on1.t,
        want.t
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
    // Document ŷ: the family's slide is on the world's axes, applied
    // outside the leg's world pose. One member, one pitch on, so it
    // stands clear of the leg in the product.
    let (doc, k, pattern) = crate::fixture::linear_on(doc, leg, [0.0, 1.0, 0.0], 1.5, 1);
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    // A value on a member's target is charted on its space's axes — the
    // world's here — so the top's slides are where member 0's footprint
    // lies in the world: the leg's unit square turned a quarter about z
    // and moved by (1, 2), then 1.5 along ŷ.
    let (doc, m0) = seat(
        doc,
        in_copy(pattern, k, 0, in_part(leg, leg_body, CapEnd::End)),
        in_part(top, top_body, CapEnd::Start),
        [0.0, 3.5],
        true,
        false,
    );
    assert_eq!(m0.placement, top, "the seat places the top");

    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.fault(top), None);
    assert_eq!(poses.space(top), Some(VarId::WORLD));

    // NOTE: the pattern direction is a DOCUMENT-coordinate map applied
    // outside the leg's world pose, so the copies march along document
    // ŷ even though the leg is rotated; the top must land so the seat
    // holds.
    let ev = run(&doc, &o);
    if let Err(refusal) = crate::fixture::could_look(&doc, &ev) {
        panic!("the seat under a placed leg verifies; the at-rest check said: {refusal}");
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
    let (doc, k, pattern) = crate::fixture::linear(doc, leg, [1.0, 0.0, 0.0], 2.0, 2);
    let refused = fixture::at_the_door_of(
        &doc,
        leg,
        fixture::rest(
            fixture::head(in_copy(pattern, k, 1, in_part(leg, leg_body, CapEnd::End))),
            fixture::head_at(top, in_part(top, top_body, CapEnd::Start)),
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
    let (doc, leg) = fixture::insert_pinned(doc, crate::fixture::mated_instance(leg_ref), &o);
    let (doc, k, pattern) = crate::fixture::linear(doc, leg, [1.0, 0.0, 0.0], 2.0, 2);
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, top2) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, m0) = seat(
        doc,
        in_copy(pattern, k, 0, in_part(leg, leg_body, CapEnd::End)),
        in_part(top, top_body, CapEnd::Start),
        [0.0, 0.0],
        true,
        false,
    );
    // The second mate names copy 1, live when it is authored; the
    // pattern then shrinks to one copy under it, which is how a read
    // stops resolving after insert (N5).
    let (doc, m1) = seat(
        doc,
        in_copy(pattern, k, 1, in_part(leg, leg_body, CapEnd::End)),
        in_part(top2, top_body, CapEnd::Start),
        [0.0, 0.0],
        true,
        false,
    );
    let (m0, m1): (MateRef, MateRef) = (m0, m1);
    assert_eq!((m0.placement, m1.placement), (top, top2));
    let doc = crate::fixture::recount(doc, k, Formula::count(1));

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
    assert_eq!(poses.space(top), Some(VarId::WORLD));
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
    let (doc, ia) = fixture::insert_pinned(doc, crate::fixture::mated_instance(a_ref), &o);
    let (doc, ib) = insert(doc, fixture::mated_instance(b_ref));
    let (doc, ic) = insert(doc, fixture::mated_instance(c_ref));
    let (doc, m0) = seat(
        doc,
        in_part(ia, a_body, CapEnd::End),
        in_part(ib, b_body, CapEnd::Start),
        [0.25, 0.5],
        false,
        true,
    );
    let (doc, m1) = seat(
        doc,
        in_part(ib, b_body, CapEnd::End),
        in_part(ic, c_body, CapEnd::Start),
        [0.75, 0.125],
        true,
        true,
    );
    assert_eq!((m0.placement, m1.placement), (ib, ic));
    let poses = solve(&doc, &o, Tol::witness());
    for id in [ia, ib, ic] {
        let f = poses
            .relative(id)
            .unwrap_or_else(|| panic!("a pose: {:?}", poses.fault(id)));
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

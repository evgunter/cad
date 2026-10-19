//! R1 review probes for MATE-1 (issue 945, PR #1400).
//!
//! These rows exist to attack the claims, not to re-state them. The
//! headline probe is the one the committed suite has no analogue of: a
//! target copy pinned to the world at a frame `F` that is NOT the
//! identity, which is the only condition under which composing a
//! pattern copy's offset on the wrong side of `F` is distinguishable
//! from the right side. Every row in `mate1_member_vocab.rs` pins its
//! base at the identity, so `F = I` throughout and the two orders agree
//! there.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::{
    CapEnd, ContactClass, DocEdit, DocumentId, EntityKind, EvalOptions, Formula, Frame, MateRef,
    MateSide, Node, PatternKind, ProfileDoc, RecipeNodeId, RoleSeg, Space, StableName,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{
    ang, constrain, head, in_copy, insert, insert_pinned, len, mated_instance, on_frame, plane,
    rest, run, scl, solve, step,
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

/// **A stored unit-cube part**: its reference, its body, and the
/// part's own names for its walls 0 (`y = 0`) and 1 (`x = 1`).
struct Cube {
    doc_ref: editor_core::DocRef,
    body: RecipeNodeId,
    /// The profile's corners 0 and 1, the ends of wall 0 (`y = 0`).
    corners: [editor_core::ProfileVertexRef; 2],
}

fn cube(store: &mut PartStore, label: &str) -> Cube {
    let (doc, body) = block_part(label, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let corners = [0, 1].map(|v| fixture::vpiece(&doc, body, 0, v));
    let (doc_ref, body) = store.insert_part((doc, body), Tol::witness());
    Cube {
        doc_ref,
        body,
        corners,
    }
}

impl Cube {
    /// The cap `end` of this part as `instance` places it.
    fn cap(&self, instance: RecipeNodeId, end: CapEnd) -> StableName {
        in_part(instance, self.body, end)
    }

    /// The frame at corner 0 of cap `end` as `instance` places it,
    /// each name spelled by `wrap` ([`fixture::corner_frame`]): `z` up
    /// its lateral edge, `x` along wall 0.
    fn frame(
        &self,
        instance: RecipeNodeId,
        end: CapEnd,
        wrap: &dyn Fn(StableName) -> StableName,
    ) -> editor_core::Operand {
        let name = |kind, seg| {
            fixture::read_minted(wrap(fixture::resolver::in_part_of(
                kind, instance, self.body, seg,
            )))
        };
        fixture::corner_frame(
            name(EntityKind::Edge, RoleSeg::LateralEdge(self.corners[0])),
            name(EntityKind::Vertex, RoleSeg::CapVertex(end, self.corners[1])),
        )
    }
}

/// **The seat**: one pin (FORK-S3O) equating the top's corner frame
/// at its start cap with the copy's at its end cap (`copy` wraps the
/// copy's part names), so the top stands on the copy flush, one unit up
/// the copy's own coordinates, whatever the copy's pose. Three plane
/// mates (a rest and two walls) said it before the bundle rule, which
/// refuses them.
fn seat(
    doc: ProfileDoc,
    top: RecipeNodeId,
    top_part: &Cube,
    copy: impl Fn(StableName) -> StableName,
    copy_part: &Cube,
    copy_at: RecipeNodeId,
) -> (ProfileDoc, MateRef) {
    constrain(
        doc,
        fixture::frame_seat(
            top_part.frame(top, CapEnd::Start, &|n| n),
            copy_part.frame(copy_at, CapEnd::End, &copy),
            0.0,
            0.0,
        ),
    )
}

/// An instance of `part` pinned to the world at `at` over its own
/// geometry ([`fixture::pin`]).
fn pinned_at(
    doc: ProfileDoc,
    part: editor_core::DocRef,
    at: &Frame,
    o: &EvalOptions,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, id) = insert(doc, mated_instance(part));
    (fixture::pin(doc, id, at, o), id)
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
// PROBE 1 — a pattern copy of a base pinned at a frame that is not the
// identity. Hand-derived end to end; nothing read back from the solve.
// ---------------------------------------------------------------

/// The leg is pinned to the world at `F` = rotate +90° about z, then
/// translate `(5, 7, 11)`. The leg is patterned linearly along the
/// document's +x at spacing 2; the top is seated onto copy 2, one unit
/// up the copy's own coordinates, so every rotation in the answer comes
/// from `F` alone.
///
/// The copy's offset `O₂ = translate(d)`, `d = (2·spacing, 0, 0) =
/// (4, 0, 0)`, acts in document coordinates, OUTSIDE the leg's frame,
/// so the top sits at `O₂ ∘ F ∘ translate(0, 0, 1)`. By hand, with `R`
/// the +90° z-rotation:
///
/// ```text
/// O₂ ∘ F ∘ translate(0, 0, 1) (0) = (4, 0, 0) + (5, 7, 11) + R·(0, 0, 1)
///                                 = (9, 7, 12)
/// ```
///
/// *Breaks if* `F` is composed on the wrong side of `O₂` — `F ∘ O₂ ∘
/// translate(0, 0, 1)` puts the top's origin at `(5, 11, 12)` — or the
/// copy's face is read without `F`'s turn.
#[test]
fn r1_conjugation_through_a_non_identity_group_frame() {
    let spacing = 2.0;
    let mut store = PartStore::default();
    let leg_part = cube(&mut store, "r1-conj-leg");
    let top_part = cube(&mut store, "r1-conj-top");
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("r1-conj"), Tol::witness());
    // The leg's world frame: rotate +90° about z, then translate.
    let f = Frame {
        columns: [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [5.0, 7.0, 11.0],
    };
    let (doc, leg) = pinned_at(doc, leg_part.doc_ref, &f, &o);
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(4),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(spacing),
            },
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, mate) = seat(
        doc,
        top,
        &top_part,
        |n| in_copy(pattern, 2, n),
        &leg_part,
        leg,
    );

    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(mate.placement, top, "the mate places the top");
    assert_eq!(poses.fault(top), None, "the mates solve — no fault");
    assert_eq!(poses.space(top), Some(Space::World));

    // Hand-derived above.
    let expected = Frame {
        columns: f.columns,
        translation: [9.0, 7.0, 12.0],
    };
    let got = poses.relative(top).expect("the top has a pose");
    assert!(
        near(got, expected, 1e-12),
        "the copy's offset stays outside the leg's frame:\n\
         got          {got:?}\n expected     {expected:?}"
    );
    let world = poses.placement(&doc, top).expect("the top places");
    assert!(
        near(world, expected, 1e-12),
        "the world placement is the copy's pose: {world:?}"
    );

    let ev = run(&doc, &o);
    assert!(
        matches!(ev.result(top), Some(editor_core::NodeResult::Ok(_))),
        "the placed top evaluates: {:?}",
        ev.result(top)
    );
}

// ---------------------------------------------------------------
// PROBE 2 — an OBLIQUE circular datum axis, off the origin, with a
// leg pinned at a non-identity frame. The expected offset is recomputed in the
// test by Rodrigues, independently of the evaluator.
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

/// The seat point, carried two independent ways to the same world
/// point. The solve puts the top at `W_top`; the seat stands the top's
/// origin on the copy's `(0, 0, 1)`, so `W_top·(0,0,0)` must equal
/// `O_i ∘ F ∘ (0,0,1)`, where `O_i` is recomputed here by Rodrigues
/// about an OBLIQUE axis that misses the origin. This is the
/// composition-order attack: `O_i ∘ F` and `F ∘ O_i` differ badly for
/// this pair, as the row asserts.
///
/// *Breaks if* the copy's turn is composed inside `F` rather than
/// outside it, or about the wrong line.
#[test]
fn r1_oblique_circular_axis_with_a_non_identity_group_frame() {
    let mut store = PartStore::default();
    let leg_part = cube(&mut store, "r1-obl-leg");
    let top_part = cube(&mut store, "r1-obl-top");
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("r1-obl"), Tol::witness());
    let f = Frame {
        columns: [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [5.0, 7.0, 11.0],
    };
    let (doc, leg) = pinned_at(doc, leg_part.doc_ref, &f, &o);

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
            input: leg.into(),
            count: Formula::count(3),
            kind: PatternKind::Circular {
                axis: editor_core::pose::RoleRead::plain(axis),
                step: ang(theta),
            },
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, mate) = seat(
        doc,
        top,
        &top_part,
        |n| in_copy(pattern, 2, n),
        &leg_part,
        leg,
    );

    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(
        poses.fault(mate.placement),
        None,
        "the oblique circular seat solves"
    );
    let world = poses.placement(&doc, top).expect("the top places");

    // Independent: the copy's (0,0,1) in the master's part
    // coordinates, carried by F into document coordinates, then
    // rotated twice about the oblique axis (copy index 2).
    let seat_master = apply(f, [0.0, 0.0, 1.0]);
    let expected_seat = rodrigues(axis_origin, axis_dir, theta * 2.0, seat_master);
    // The solve's answer for the same point: the top's own origin.
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
}

// ---------------------------------------------------------------
// PROBE 3 — per-instance freedom: try to author one.
// ---------------------------------------------------------------

/// Claim 4's last clause: NO mate can give one pattern copy a pose
/// apart from its siblings. The attack: seat a plain top on copy 1,
/// then read every pose the solve holds — the pattern's copies have no
/// entry of their own anywhere in the solve, and the only instance
/// that moved is the top. A pattern copy is not a placement, so there
/// is no representation in which one copy could hold a pose.
///
/// *Breaks if* a pattern node answers a pose or a space, the seat moves
/// the leg, or the edit door takes a constraint on a pattern.
#[test]
fn r1_no_mate_can_give_one_copy_a_pose_apart_from_its_siblings() {
    let mut store = PartStore::default();
    let leg_part = cube(&mut store, "r1-freedom-leg");
    let top_part = cube(&mut store, "r1-freedom-top");
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("r1-freedom"), Tol::witness());
    // The leg is a loose copy: the identity of its own space.
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_part.doc_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, mate) = seat(
        doc,
        top,
        &top_part,
        |n| in_copy(pattern, 1, n),
        &leg_part,
        leg,
    );
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.fault(mate.placement), None);

    // The pattern node holds no pose and no space: copies are not
    // placements, so per-instance freedom is unrepresentable.
    assert_eq!(poses.relative(pattern), None, "a pattern holds no pose");
    assert_eq!(poses.space(pattern), None, "a pattern is no placement");
    // The master stays at the identity of its own space — the mates
    // moved the copy they place, the top.
    assert_eq!(poses.relative(leg), Some(Frame::IDENTITY));

    // And the edit door refuses a constraint on a pattern.
    let bad = editor_core::apply(
        &doc,
        &DocEdit::AddConstraint {
            placement: pattern,
            mate: Box::new(editor_core::Mate {
                on: plane(head(in_copy(pattern, 0, leg_part.cap(leg, CapEnd::End)))),
                to: editor_core::Target::World(editor_core::WorldRead::through(
                    editor_core::pose::PoseRole::Xy,
                )),
                class: ContactClass::Rest,
            }),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    assert!(
        matches!(
            bad,
            Err(editor_core::EditError::ConstraintOnNonPlacement { .. })
        ),
        "a pattern node cannot carry a placement of its own: {:?}",
        bad.err()
    );
}

// ---------------------------------------------------------------
// PROBE 4 — pattern-free solves dumped as bits, for comparison
// against another revision. Executed, not inspected.
// ---------------------------------------------------------------

/// A chain of pattern-free mate documents whose solved poses are
/// dumped as raw f64 BIT PATTERNS. Run once on each of two revisions;
/// the two dumps must be byte-identical where neither changed the
/// solve. Reading a diff cannot establish this.
///
/// Writes to `$R1_BITS_OUT` when set; otherwise just exercises the
/// documents (so the row is harmless in a normal run).
#[test]
fn r1_pattern_free_solves_are_bit_identical() {
    let mut lines: Vec<String> = Vec::new();

    // Three shapes: a plain seat, a seat on a base pinned at a
    // non-identity frame, and a three-instance chain whose second seat
    // is one rest with its freedoms set by values.
    for (tag, chain, fr) in [
        ("seat-identity", false, Frame::IDENTITY),
        (
            "seat-turned",
            false,
            Frame {
                columns: [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
                translation: [5.0, 7.0, 11.0],
            },
        ),
        (
            "chain-3",
            true,
            Frame {
                columns: [[1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, -1.0, 0.0]],
                translation: [-3.25, 0.5, 2.125],
            },
        ),
    ] {
        let mut store = PartStore::default();
        let a_part = cube(&mut store, &format!("r1-bits-{tag}-a"));
        let b_part = cube(&mut store, &format!("r1-bits-{tag}-b"));
        let c_part = cube(&mut store, &format!("r1-bits-{tag}-c"));
        let o = with_resolver(store);
        let doc = ProfileDoc::empty(
            DocumentId::derive(&format!("r1-bits-{tag}")),
            Tol::witness(),
        );
        let (doc, a) = pinned_at(doc, a_part.doc_ref, &fr, &o);
        let (doc, b) = insert(doc, fixture::mated_instance(b_part.doc_ref));
        // `c` sits where its mates put it only when the chain mates it;
        // otherwise it is pinned at the identity.
        let (doc, c) = if chain {
            insert(doc, fixture::mated_instance(c_part.doc_ref))
        } else {
            insert_pinned(doc, Node::instantiate_part(c_part.doc_ref), &o)
        };
        let (doc, _m0) = seat(doc, b, &b_part, |n| n, &a_part, a);
        let doc = if chain {
            // `c` rests on `b`, its three freedoms set by values: a
            // quarter along `b`'s x, none along its y, no turn.
            let (doc, m1) = constrain(
                doc,
                rest(
                    head(c_part.cap(c, CapEnd::Start)),
                    head(b_part.cap(b, CapEnd::End)),
                    ContactClass::Rest,
                ),
            );
            [
                (editor_core::Coord::Spin, ang(0.0)),
                (
                    editor_core::Coord::Slide(editor_core::SlideAxis::X),
                    len(0.25),
                ),
                (
                    editor_core::Coord::Slide(editor_core::SlideAxis::Y),
                    len(0.0),
                ),
            ]
            .into_iter()
            .fold(doc, |doc, (coord, by)| add_value(doc, m1, coord, by))
        } else {
            doc
        };

        let poses = solve(&doc, &o, Tol::witness());
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
    }

    let dump = lines.join("\n");
    assert!(!dump.is_empty());
    if let Ok(path) = std::env::var("R1_BITS_OUT") {
        std::fs::write(&path, &dump).expect("the dump writes");
    }
}

/// **`by` set on freedom `coord` of `mate`**, through the edit door.
fn add_value(doc: ProfileDoc, mate: MateRef, coord: editor_core::Coord, by: Formula) -> ProfileDoc {
    step(
        doc,
        DocEdit::AddValue {
            placement: mate.placement,
            value: Box::new(editor_core::Value {
                mate: mate.id,
                coord,
                by,
            }),
            fresh: Vec::new(),
        },
    )
    .0
}

// ---------------------------------------------------------------
// PROBE 6 — the two fence cases the PR body names but the committed
// suite does not build: a NESTED pattern and a pattern of a TRANSFORM.
// ---------------------------------------------------------------

/// The two shapes the member vocabulary is asked about by name, built
/// rather than taken on trust: a pattern of a PATTERN under a name
/// that carries ONE `Instance(i)` qualifier, which reads no face, and a
/// pattern of a TRANSFORM, which places.
///
/// **The first half is a NAME row, not a nesting row.** A nested copy
/// wears one `Instance(i)` qualifier per pattern level, and a
/// two-level nest wears two. This name wears one, which is the name a
/// nested pattern's table never mints, so it resolves to no face: the
/// door admits the mate (it decides structure alone) and the placement
/// refuses where it evaluates, its `to` pose unevaluated.
///
/// *Breaks if* a one-level name over a nest resolves to some face, or
/// a pattern of a transform does not place the top.
#[test]
fn r1_an_underqualified_nested_name_refuses_and_a_pattern_of_transform_places() {
    // (a) a pattern OF A PATTERN of an instance.
    let mut store = PartStore::default();
    let leg_part = cube(&mut store, "r1-nested-leg");
    let top_part = cube(&mut store, "r1-nested-top");
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("r1-nested"), Tol::witness());
    let (doc, leg) = insert_pinned(doc, Node::instantiate_part(leg_part.doc_ref), &o);
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
                spacing: len(3.0),
            },
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, mate) = constrain(
        doc,
        rest(
            head(top_part.cap(top, CapEnd::Start)),
            head(in_copy(outer, 1, leg_part.cap(leg, CapEnd::End))),
            ContactClass::Rest,
        ),
    );
    let poses = solve(&doc, &o, Tol::witness());
    let fault = poses.fault(top).cloned().expect("the placement refuses");
    assert!(
        matches!(
            fault,
            editor_core::MateFault::SideUnevaluated { mate: m, side: MateSide::To, .. }
                if m == mate
        ),
        "a one-level name over a nested pattern reads no face: {fault:?}"
    );

    // (b) a pattern of a TRANSFORM of an instance.
    let mut store2 = PartStore::default();
    let leg2 = cube(&mut store2, "r1-xform-leg");
    let top2 = cube(&mut store2, "r1-xform-top");
    let o2 = with_resolver(store2);
    let doc2 = ProfileDoc::empty(DocumentId::derive("r1-xform"), Tol::witness());
    let (doc2, li) = insert_pinned(doc2, Node::instantiate_part(leg2.doc_ref), &o2);
    let (doc2, xf) = insert(
        doc2,
        Node::transform(
            li,
            editor_core::Step::Rigid {
                translation: [len(0.0), len(0.0), len(0.5)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        ),
    );
    let (doc2, pat) = insert(
        doc2,
        Node::Pattern {
            input: xf.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc2, ti) = insert(doc2, mated_instance(top2.doc_ref));
    let (doc2, m2) = seat(doc2, ti, &top2, |n| in_copy(pat, 1, n), &leg2, li);
    let poses2 = solve(&doc2, &o2, Tol::witness());
    // A pattern OF A TRANSFORM resolves: the face is read through the
    // pattern's `Instance(i)` and on through the transform, and the
    // copy's pose composes `M(i) ∘ T`.
    assert!(
        poses2.fault(m2.placement).is_none(),
        "a pattern of a transform resolves: {:?}",
        poses2.fault(m2.placement)
    );
    assert_eq!(
        poses2.space(ti),
        Some(Space::World),
        "and it places the top"
    );
}

// ---------------------------------------------------------------
// PROBE 7 — an out-of-range copy index refuses where the placement
// evaluates, and only for the copy it places.
// ---------------------------------------------------------------

/// A face is read where its placement evaluates, so a malformed head
/// refuses wherever it arises. This row reaches a head past the count
/// the way one arises after insert — the pattern shrinks under the
/// mate — and the placement refuses there, while a second top seated
/// on a copy that survives is untouched.
///
/// *Breaks if* the stranded top is placed anyway, or its refusal
/// reaches the top on the surviving copy.
#[test]
fn r1_an_out_of_range_copy_refuses_at_the_solve_too() {
    let mut store = PartStore::default();
    let leg_part = cube(&mut store, "r1-escape-leg");
    let top_part = cube(&mut store, "r1-escape-top");
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("r1-escape"), Tol::witness());
    let (doc, leg) = insert_pinned(doc, Node::instantiate_part(leg_part.doc_ref), &o);
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
    let (doc, kept) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, stranded) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    // A seat on copy 0.
    let (doc, good) = seat(
        doc,
        kept,
        &top_part,
        |n| in_copy(pattern, 0, n),
        &leg_part,
        leg,
    );
    // Then a seat on copy 1.
    let (doc, bad) = seat(
        doc,
        stranded,
        &top_part,
        |n| in_copy(pattern, 1, n),
        &leg_part,
        leg,
    );
    // Now the pattern SHRINKS under it: copy 1 of a count-1 pattern
    // reads no face, reached the way a head stops resolving after
    // insert (N5).
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
    assert_eq!(poses.fault(good.placement), None);
    assert_eq!(poses.space(good.placement), Some(Space::World));

    let fault = poses
        .fault(bad.placement)
        .cloned()
        .expect("the out-of-range copy refuses at the solve");
    assert!(
        matches!(
            fault,
            editor_core::MateFault::SideUnevaluated { mate, side: MateSide::To, .. }
                if mate == bad
        ),
        "the refusal names the mate whose copy the count no longer holds: {fault:?}"
    );
    assert_eq!(
        poses.space(bad.placement),
        None,
        "and the copy it would place has no space to be carried in"
    );
}

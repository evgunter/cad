//! **A mate reads the body its side names** — the transform-aware solve.
//!
//! A mate's two sides are reads of a face: a FACE name, selected in the
//! body it is read at (the side's operand). The solve walks from that
//! body down to
//! the name's minting instance and composes the map of every
//! pose-bearing node it passes, so a mate on a TRANSFORMED instance
//! seats the transformed geometry and a mate on the instance seats
//! the instance.
//!
//! Every row here goes through ordinary doors — `DocEdit::AddConstraint`,
//! `solve_document`, `evaluate`, `product` — and measures the
//! PRODUCT's own face frames: the planes the mate's two names resolve
//! to in the body a consumer actually gathers, never a solved frame
//! read by eye.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocEdit, DocumentId, EditError, EvalOptions,
    Evaluation, Formula, Mate, MateFault, MateFrame, MatePrimitive, MateRef, MateSide, Node,
    PatternKind, Placement, ProfileDoc, RecipeNodeId, SideRead, SitedFace, Space, StableName, load,
    product, save,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::seat::map_gap;
use fixture::{
    constrain_at, gate, in_copy, insert, len, on_frame, run, scl, solve, step, step_with, xform,
};
use geom_core::Tol;
use geom_core::linalg::Affine3;
use std::sync::Arc;

// ---- substrate ----

/// A `wxwxh` block, as a whole part document.
fn slab(label: &str, w: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, w), (0.0, w)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(h),
            side: ExtrudeSide::Along,
        },
    )
}

/// A `1x1xh` block, as a whole part document.
fn block(label: &str, h: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(h),
            side: ExtrudeSide::Along,
        },
    )
}

/// The `a` frame: a point ON the base's top cap, axis along that
/// cap's OUTWARD normal.
fn a_frame() -> MateFrame<Formula> {
    MateFrame::authored(
        [1.0, 1.0, BASE_HEIGHT],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        geom_core::Tol::witness(),
    )
    .expect("a definite frame")
}

/// The `b` frame: the top block's bottom-cap corner, axis along THAT
/// cap's outward normal, which points DOWN in the block's own part
/// coordinates.
///
/// The two outward normals and `Opposed` are what make this a
/// physical seat: the top block stands ON the base. Authoring `b`'s
/// axis as `+z` instead would satisfy the same coset and stand the
/// block THROUGH the base — a solve the at-rest gate then refuses for
/// every document, transform or none, which is a fixture that cannot
/// tell a correct seat from a wrong one.
fn b_frame() -> MateFrame<Formula> {
    MateFrame::authored(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, -1.0],
        [1.0, 0.0, 0.0],
        geom_core::Tol::witness(),
    )
    .expect("a definite frame")
}

/// A `Rest` mate seating `b`'s bottom cap onto `a`'s top cap, both
/// frames authored in their member's own part coordinates.
fn seat(a: SitedFace, b: SitedFace) -> Mate<Formula> {
    seat_with(a, b, MatePrimitive::FrameCoincidence, None)
}

fn seat_with(
    a: SitedFace,
    b: SitedFace,
    primitive: MatePrimitive,
    clocking: Option<f64>,
) -> Mate<Formula> {
    fixture::mate(
        a,
        b,
        ContactClass::Rest,
        Alignment {
            a: a_frame(),
            b: b_frame(),
            primitive,
            sense: AxisSense::Opposed,
            clocking,
        },
    )
}

// ---- the scene ----

/// The base's height and half-width: a `3x3x1` slab, wide enough that
/// the `1x1x3` block seated on it stands clear of its edges. Width is
/// what makes the seat a real one — a same-footprint pair would rest
/// face-on-face with every edge coincident, where the census has more
/// to say than the mate does.
const BASE_HEIGHT: f64 = 1.0;
const BASE_WIDTH: f64 = 3.0;
const TOP_HEIGHT: f64 = 3.0;

/// `base` (a wide slab) and `top` (a tall block), each optionally
/// wrapped in a chain of transforms, with a `Rest`/`FrameCoincidence`/
/// `Opposed` mate seating `top`'s bottom cap on `base`'s top cap, each
/// side authored at the LAST node of its own chain.
struct Scene {
    doc: ProfileDoc,
    opts: EvalOptions,
    base: RecipeNodeId,
    top: RecipeNodeId,
    base_body: RecipeNodeId,
    top_body: RecipeNodeId,
    /// The last node of `base`'s chain (the `a` operand).
    a_at: RecipeNodeId,
    /// The last node of `top`'s chain (the `b` operand).
    b_at: RecipeNodeId,
    /// The seat, a constraint of `top`'s bundle.
    mate: MateRef,
}

/// **The world the product is** (A10): each of `bodies` that is not
/// already a placement placed, then each `(pattern, count)`'s copies
/// placed through a `Part` per copy — the sinks of each row's chains.
/// An instance is a placement already in the product, so it is never
/// placed a second time.
fn world(doc: ProfileDoc, bodies: &[RecipeNodeId], patterns: &[(RecipeNodeId, i64)]) -> ProfileDoc {
    let bodies: Vec<RecipeNodeId> = bodies
        .iter()
        .copied()
        .filter(|&b| !matches!(doc.node(b), Some(Node::InstantiatePart { .. })))
        .collect();
    let doc = fixture::place_all(doc, &bodies);
    patterns.iter().fold(doc, |doc, &(pattern, count)| {
        (0..count).fold(doc, |doc, i| {
            let (doc, copy) = insert(
                doc,
                Node::Part {
                    of: pattern.into(),
                    select: editor_core::PartSelect::Instance(Formula::count(i)),
                },
            );
            fixture::place(doc, copy).0
        })
    })
}

/// **`name`'s world copy**: the one name in the gathered product's
/// table that is a copy of `name` ([`StableName::copy_of`]) — what the
/// product names a face by under the `Place` that put a transformed or
/// patterned body in the world — or, where no `Place` copies it, `name`
/// itself, which is how the product names a face of an instance (an
/// instance is a placement, its face names already its copy's).
fn copy_name(doc: &ProfileDoc, ev: &editor_core::Evaluation<f64>, name: &StableName) -> StableName {
    let (_, names) =
        editor_core::product_named(doc, ev, Tol::witness()).expect("the product gathers");
    let copies: Vec<StableName> = names
        .iter()
        .map(|(n, _)| n.clone())
        .filter(|n| n.copy_of().is_some_and(|(_, of)| of == name))
        .collect();
    match copies.as_slice() {
        [copy] => copy.clone(),
        [] if names.iter().any(|(n, _)| n == name) => name.clone(),
        _ => panic!("{name:?} has one world copy, not {copies:?}"),
    }
}

/// [`fixture::seat::assert_seated`] over the world copies of `a` and
/// `b` ([`copy_name`]).
fn assert_seated(
    doc: &ProfileDoc,
    ev: &editor_core::Evaluation<f64>,
    a: &StableName,
    b: &StableName,
    control: &Affine3<f64>,
    what: &str,
) {
    let (a, b) = (copy_name(doc, ev, a), copy_name(doc, ev, b));
    fixture::seat::assert_seated(doc, ev, &a, &b, control, what);
}

/// [`fixture::seat::seat_map`] over the world copies of `a` and `b`.
fn seat_map(
    doc: &ProfileDoc,
    ev: &editor_core::Evaluation<f64>,
    a: &StableName,
    b: &StableName,
) -> Affine3<f64> {
    fixture::seat::seat_map(doc, ev, &copy_name(doc, ev, a), &copy_name(doc, ev, b))
}

/// [`fixture::seat::product_face_frame`] of `name`'s world copy.
fn product_face_frame(
    doc: &ProfileDoc,
    ev: &editor_core::Evaluation<f64>,
    name: &StableName,
) -> Affine3<f64> {
    fixture::seat::product_face_frame(doc, ev, &copy_name(doc, ev, name))
}

/// One link of a transform chain: `(translation, rotation axis,
/// angle)`.
type Step = ([f64; 3], [f64; 3], f64);

/// Builds that scene. `on_base` / `on_top` are the transform chains,
/// innermost first.
fn scene(label: &str, on_base: &[Step], on_top: &[Step]) -> Scene {
    let mut store = PartStore::default();
    let (base_ref, base_body) = store.insert_part(
        slab(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let (top_ref, top_body) =
        store.insert_part(block(&format!("{label}-top"), TOP_HEIGHT), Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, a_at) = chain(doc, base, on_base);
    let (doc, b_at) = chain(doc, top, on_top);
    let doc = world(doc, &[a_at, b_at], &[]);
    let (doc, mate) = constrain_at(
        doc,
        top,
        seat(
            crate::fixture::head_at(a_at, in_part(base, base_body, CapEnd::End)),
            crate::fixture::head_at(b_at, in_part(top, top_body, CapEnd::Start)),
        ),
    );
    Scene {
        doc,
        opts,
        base,
        top,
        base_body,
        top_body,
        a_at,
        b_at,
        mate,
    }
}

/// Wraps `at` in each step of a transform chain, innermost first,
/// answering the last node.
fn chain(mut doc: ProfileDoc, mut at: RecipeNodeId, steps: &[Step]) -> (ProfileDoc, RecipeNodeId) {
    for &(t, ax, ang) in steps {
        let (d, id) = insert(doc, xform(at, t, ax, ang));
        doc = d;
        at = id;
    }
    (doc, at)
}

const LIFT: Step = ([0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0);

/// **The seat this fixture's alignment asks for**, measured on the
/// document that has no transform and no pattern anywhere: the
/// relative frame every other row must reproduce.
///
/// It is derived, not written down — a constant transcribed here
/// would be a second statement of what the alignment says, and the
/// two could drift apart.
fn control_seat(label: &str) -> Affine3<f64> {
    let s = scene(&format!("{label}-control"), &[], &[]);
    let ev = s.eval();
    seat_map(&s.doc, &ev, &s.face_a(), &s.face_b())
}

impl Scene {
    fn eval(&self) -> Evaluation<f64> {
        run(&self.doc, &self.opts)
    }
    fn face_a(&self) -> StableName {
        in_part(self.base, self.base_body, CapEnd::End)
    }
    fn face_b(&self) -> StableName {
        in_part(self.top, self.top_body, CapEnd::Start)
    }
    /// Every node evaluated, no mate fault, the mated faces seated in
    /// the product, and **the at-rest gate satisfied** — which is the
    /// check that says the declaration the document makes is the one
    /// the geometry keeps.
    fn assert_green_and_seated(&self, control: &Affine3<f64>, what: &str) {
        let poses = solve(&self.doc, &self.opts, Tol::witness());
        assert!(
            poses.fault(self.mate.placement).is_none(),
            "{what}: the solve refused: {:?}",
            poses.fault(self.mate.placement)
        );
        let ev = self.eval();
        for (name, id) in [
            ("base", self.base),
            ("top", self.top),
            ("a_at", self.a_at),
            ("b_at", self.b_at),
        ] {
            assert!(
                matches!(ev.result(id), Some(editor_core::NodeResult::Ok(_))),
                "{what}: {name} ({id:?}) did not evaluate: {:?}",
                ev.result(id)
            );
        }
        assert!(
            product(&self.doc, &ev, Tol::witness()).is_ok(),
            "{what}: the product gathers"
        );
        assert_seated(
            &self.doc,
            &ev,
            &self.face_a(),
            &self.face_b(),
            control,
            what,
        );
        assert!(
            gate(&self.doc, &ev).is_ok(),
            "{what}: the at-rest gate refused a seat the solve placed: {:?}",
            gate(&self.doc, &ev).err()
        );
    }
}

// ---- A1: the finding's own document, fixed ----

/// **A1.** The two-block document with a +z translation over the
/// mated instance (angle 0): the solve records no fault, the product
/// gathers, the mated faces MEET in the product, and the solved
/// relative pose differs from the control's by exactly the
/// transform's map.
#[test]
fn a1_a_translated_instance_seats_in_the_product() {
    let seat = control_seat("msolve1-a1");
    let control = scene("msolve1-a1-control", &[], &[]);
    control.assert_green_and_seated(&seat, "A1 control");

    let test = scene("msolve1-a1-lifted", &[], &[LIFT]);
    test.assert_green_and_seated(&seat, "A1 lifted");

    // The mechanism, stated as arithmetic rather than as a string
    // comparison: the solve moved the instance by exactly the
    // transform's translation, in the opposite sense, so the placed
    // body lands where the un-transformed one did.
    let c = solve(&control.doc, &control.opts, Tol::witness())
        .relative(control.top)
        .expect("the control solves");
    let t = solve(&test.doc, &test.opts, Tol::witness())
        .relative(test.top)
        .expect("the test solves");
    assert_eq!(
        (c.translation[2] - t.translation[2]).to_bits(),
        10.0_f64.to_bits(),
        "the solve absorbed exactly the transform's +10 lift \
         (control {:?} vs test {:?})",
        c.translation,
        t.translation
    );
    // ...and the rotation part is untouched by a translation-only
    // transform, bit for bit.
    assert_eq!(c.columns, t.columns, "a translation moves no axis");
}

// ---- A2: rotation ----

/// **A2.** A transform with a non-zero angle about z, and ones about
/// non-axis directions: the mated faces still seat in the product,
/// their outward normals opposed and their whole relative frame the
/// control's — and the at-rest gate still passes.
///
/// A rotation is the case a translation-only fix cannot reach, and
/// the about-z rows are the case a normal-and-gap oracle cannot
/// reach: a spin about the seat normal leaves both faces coplanar and
/// both normals opposed while turning the block. The relative frame
/// is what sees it.
#[test]
fn a2_a_rotated_instance_seats_and_keeps_the_whole_frame() {
    let seat = control_seat("msolve1-a2");
    for (what, axis, angle) in [
        (
            "about z, pi/2",
            [0.0, 0.0, 1.0],
            std::f64::consts::FRAC_PI_2,
        ),
        (
            "about z, pi/6",
            [0.0, 0.0, 1.0],
            std::f64::consts::FRAC_PI_6,
        ),
        (
            "about x, pi/6",
            [1.0, 0.0, 0.0],
            std::f64::consts::FRAC_PI_6,
        ),
        (
            "about (1,1,1), pi/3",
            [1.0, 1.0, 1.0],
            std::f64::consts::FRAC_PI_3,
        ),
        ("about (-1,2,3), 1.1", [-1.0, 2.0, 3.0], 1.1),
    ] {
        let s = scene(
            &format!("msolve1-a2-{what}"),
            &[],
            &[([0.0, 0.0, 10.0], axis, angle)],
        );
        s.assert_green_and_seated(&seat, &format!("A2 {what}"));
    }
}

// ---- A3: patterns over transforms, and transforms over patterns ----

/// The pattern-headed scene: `base`, then `top` under an optional
/// transform, then a `Pattern` (or the other way round), with the
/// mate seating copy `COPY` of the pattern on the base.
const COPY: u32 = 1;

/// **A3.** Pattern-of-transform (the finding's `step4` document, which
/// refuses `DanglingHead` before this unit) and transform-of-pattern:
/// both solve, and the named copy seats on the base in the product.
#[test]
fn a3_pattern_of_transform_seats_and_transform_of_pattern_resolves() {
    // The seat is a constant of the ALIGNMENT and the two part
    // documents, so the plain control's relative frame is the one a
    // pattern copy must land in too — the copy's own offset is the
    // solve's to absorb.
    let seat_frame = control_seat("msolve1-a3");
    // (a) PATTERN over TRANSFORM over the instance. The mate is read
    // at the pattern; the offset is M(1) ∘ T.
    {
        let mut store = PartStore::default();
        let (base_ref, base_body) = store.insert_part(
            slab("msolve1-a3a-base", BASE_WIDTH, BASE_HEIGHT),
            Tol::witness(),
        );
        let (top_ref, top_body) =
            store.insert_part(block("msolve1-a3a-top", TOP_HEIGHT), Tol::witness());
        let opts = with_resolver(store);
        let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a3a"), Tol::witness());
        let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
        let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
        let (doc, xf) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
        let (doc, pattern) = insert(
            doc,
            Node::Pattern {
                input: xf.into(),
                count: Formula::count(3),
                // Spaced wide enough that copies 0 and 2 clear the
                // slab entirely: the row is about the copy the mate
                // names, and a sibling resting on the base uninvited
                // is an UNRECORDED contact the gate is right to refuse.
                kind: PatternKind::Linear {
                    direction: [scl(1.0), scl(0.0), scl(0.0)],
                    spacing: len(5.0),
                },
            },
        );
        let doc = world(doc, &[base], &[(pattern, 3)]);
        let a = in_part(base, base_body, CapEnd::End);
        let b = in_copy(pattern, COPY, in_part(top, top_body, CapEnd::Start));
        let (doc, _) = constrain_at(
            doc,
            top,
            seat(
                crate::fixture::head(a.clone()),
                crate::fixture::head_at(pattern, b.clone()),
            ),
        );
        let poses = solve(&doc, &opts, Tol::witness());
        assert!(
            poses.fault(top).is_none(),
            "A3 pattern-of-transform refused: {:?}",
            poses.fault(top)
        );
        let ev = run(&doc, &opts);
        assert_seated(&doc, &ev, &a, &b, &seat_frame, "A3 pattern-of-transform");
        assert!(
            gate(&doc, &ev).is_ok(),
            "A3 pattern-of-transform: the gate refused: {:?}",
            gate(&doc, &ev).err()
        );
    }

    // (b) TRANSFORM over PATTERN over the instance. The mate names a
    // copy and is read at the transform; the walk admits it and the
    // offset is T ∘ M(1) — the mate SOLVES and pins the top, and the
    // document GATHERS: a transform is shape-preserving over
    // its input's value, so the transform of the pattern is the
    // pattern's instances under one map, and the named copy seats on
    // the base in the product. Copies 0 and 2 are spaced to clear the
    // slab, as in (a).
    {
        let mut store = PartStore::default();
        let (base_ref, base_body) = store.insert_part(
            slab("msolve1-a3b-base", BASE_WIDTH, BASE_HEIGHT),
            Tol::witness(),
        );
        let (top_ref, top_body) =
            store.insert_part(block("msolve1-a3b-top", TOP_HEIGHT), Tol::witness());
        let opts = with_resolver(store);
        let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a3b"), Tol::witness());
        let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
        let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
        let (doc, pattern) = insert(
            doc,
            Node::Pattern {
                input: top.into(),
                count: Formula::count(3),
                kind: PatternKind::Linear {
                    direction: [scl(1.0), scl(0.0), scl(0.0)],
                    spacing: len(5.0),
                },
            },
        );
        let (doc, xf) = insert(
            doc,
            xform(
                pattern,
                [0.0, 0.0, 10.0],
                [0.0, 0.0, 1.0],
                std::f64::consts::FRAC_PI_2,
            ),
        );
        let doc = world(doc, &[base], &[(xf, 3)]);
        let a = in_part(base, base_body, CapEnd::End);
        let b = in_copy(pattern, COPY, in_part(top, top_body, CapEnd::Start));
        let (doc, _) = constrain_at(
            doc,
            top,
            seat(
                crate::fixture::head(a.clone()),
                crate::fixture::head_at(xf, b.clone()),
            ),
        );
        let poses = solve(&doc, &opts, Tol::witness());
        assert!(
            poses.fault(top).is_none(),
            "A3 transform-of-pattern refused at the solve: {:?}",
            poses.fault(top)
        );
        assert_eq!(
            poses.space(top),
            Some(Space::World),
            "the transform-of-pattern reference places the top"
        );
        let ev = run(&doc, &opts);
        assert!(
            ev.value(xf).is_some(),
            "the transform of the pattern evaluates: {:?}",
            ev.node_error(xf)
        );
        assert_seated(&doc, &ev, &a, &b, &seat_frame, "A3 transform-of-pattern");
        assert!(
            gate(&doc, &ev).is_ok(),
            "A3 transform-of-pattern: the gate refused: {:?}",
            gate(&doc, &ev).err()
        );
    }
}

// ---- A4: which side, and how many ----

/// **A4.** A transform over the TARGET side, over both sides, chains
/// of two on either side, and — the row the rest do not reach —
/// chains whose two maps DO NOT COMMUTE.
///
/// The target is `base`, the copy the top is pinned to, so a transform
/// over `base` is the case where the solve must un-wind the map on
/// the side it is measuring FROM. The non-commuting chains are what
/// exercise the fold's ORDER: a lift along z and a spin about z
/// commute, so every such chain composes to the same map whichever
/// way round the offset folds, and a fold written backwards would
/// pass. A rotation about x followed by a translation along z does
/// not commute with itself reversed, on either side.
#[test]
fn a4_the_offset_holds_on_either_side_and_through_a_chain() {
    let seat = control_seat("msolve1-a4");
    let lift = |z: f64| ([0.0, 0.0, z], [0.0, 0.0, 1.0], 0.0);
    let spin = |a: f64| ([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], a);
    let tip = |a: f64| ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], a);
    for (what, on_base, on_top) in [
        ("target side only", vec![lift(4.0)], vec![]),
        ("both sides", vec![lift(4.0)], vec![lift(10.0)]),
        (
            "a chain of two on the mated side",
            vec![],
            vec![lift(10.0), spin(std::f64::consts::FRAC_PI_3)],
        ),
        (
            "a chain of two, rotation first",
            vec![],
            vec![spin(std::f64::consts::FRAC_PI_3), lift(10.0)],
        ),
        (
            "a chain on the target and one on the mate",
            vec![spin(std::f64::consts::FRAC_PI_6), lift(2.0)],
            vec![lift(10.0)],
        ),
        // NON-COMMUTING, on the mated side: `tip` then `lift` is not
        // `lift` then `tip` — the x-rotation turns the z-translation.
        (
            "a non-commuting chain on the mated side, tip then lift",
            vec![],
            vec![tip(std::f64::consts::FRAC_PI_3), lift(10.0)],
        ),
        (
            "a non-commuting chain on the mated side, lift then tip",
            vec![],
            vec![lift(10.0), tip(std::f64::consts::FRAC_PI_3)],
        ),
        // ...and on the TARGET side, where the map is un-wound.
        (
            "a non-commuting chain on the target side",
            vec![tip(-std::f64::consts::FRAC_PI_4), lift(5.0)],
            vec![],
        ),
        (
            "non-commuting chains on both sides",
            vec![tip(-std::f64::consts::FRAC_PI_4), lift(5.0)],
            vec![lift(10.0), tip(std::f64::consts::FRAC_PI_3)],
        ),
    ] {
        let s = scene(&format!("msolve1-a4-{what}"), &on_base, &on_top);
        s.assert_green_and_seated(&seat, &format!("A4 {what}"));
    }
}

/// **A4′ — the two orders really are different maps.** The row above
/// asserts both non-commuting chains seat; this one asserts they are
/// not the same document dressed twice, by measuring that the two
/// place the block somewhere different. Without it "both seat" could
/// be true of a pair the fold happened to see as one.
#[test]
fn a4_the_two_non_commuting_orders_place_different_geometry() {
    let tip = std::f64::consts::FRAC_PI_3;
    let one = scene(
        "msolve1-a4-order-1",
        &[],
        &[([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], tip), LIFT],
    );
    let two = scene(
        "msolve1-a4-order-2",
        &[],
        &[LIFT, ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], tip)],
    );
    // Measured on what the SOLVE absorbed, not on the seat: the seat
    // is the same by construction — that is the point of the row
    // above — so the two chains being different maps has to be read
    // off the pose the solve hands the instance.
    let absorbed = |s: &Scene| {
        solve(&s.doc, &s.opts, Tol::witness())
            .relative(s.top)
            .expect("the mated instance solves")
    };
    let (p1, p2) = (absorbed(&one), absorbed(&two));
    let moved = map_gap(&p1.affine::<f64>(), &p2.affine::<f64>());
    assert!(
        moved > 1.0,
        "the two chain orders compose to the same map ({p1:?} vs {p2:?}) — \
         the row above would then prove nothing about composition order"
    );
}

/// **A4″ — the target pinned to the world at a non-identity pose**,
/// with transforms on both sides. The pair's static factor is
/// conjugated through the target's world frame, so a document whose
/// target carries an authored pose is the case where that conjugation
/// has to be right as well as the composition.
#[test]
fn a4_a_placed_target_seats_through_both_chains() {
    let seat = control_seat("msolve1-a4-placed");
    let s = scene(
        "msolve1-a4-placed-frame",
        &[([1.0, 2.0, 3.0], [1.0, 1.0, 0.0], -0.6)],
        &[([0.0, 5.0, 10.0], [0.0, 1.0, 1.0], 1.3)],
    );
    // The base's one world pin, re-authored at the pose.
    let [pin] = fixture::bundle(&s.doc, s.base)
        .try_into()
        .expect("one world pin");
    let (doc, _) = step(
        s.doc.clone(),
        DocEdit::RemoveConstraint {
            placement: s.base,
            constraint: pin.id,
        },
    );
    let (doc, _) = constrain_at(
        doc,
        s.base,
        Mate::world_pin(Placement::literal(
            &editor_core::Frame::rotate_then_translate(
                [0.3, -0.2, 0.9],
                0.8,
                [2.0, -1.0, 3.0],
                fixture::band(),
            )
            .expect("a literal axis has a definite direction"),
        )),
    );
    let placed = Scene { doc, ..s };
    placed.assert_green_and_seated(&seat, "A4 placed target");
}

// ---- A5: two operands, one instance ----

/// One seat from `top` to `base` read at a transform of `top`, and a
/// second seat read at a DIFFERENT transform of `top`, each transform
/// placed so both copies reach the product. Answers the document with the first seat in `top`'s
/// bundle, the first seat's address, the second seat (not added), and
/// `top`.
///
/// `x1` lifts the block, and the first mate seats it at the slab's
/// `(1, 1)` corner. `x2` is `x1` moved `(2, 2)` across the slab plus
/// `extra_lift`, and the second mate seats it at the slab's `(3, 3)`
/// corner: with no extra lift the two seats agree, and with one they
/// do not. The two copies stand apart either way.
fn two_operands(
    label: &str,
    extra_lift: f64,
) -> (
    ProfileDoc,
    EvalOptions,
    MateRef,
    Mate<Formula>,
    RecipeNodeId,
) {
    let mut store = PartStore::default();
    let (base_ref, base_body) = store.insert_part(
        slab(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let (top_ref, top_body) =
        store.insert_part(block(&format!("{label}-top"), TOP_HEIGHT), Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, x1) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, x2) = insert(
        doc,
        xform(top, [2.0, 2.0, 10.0 + extra_lift], [0.0, 0.0, 1.0], 0.0),
    );
    let doc = world(doc, &[base, x1, x2], &[]);
    let a = in_part(base, base_body, CapEnd::End);
    let b = in_part(top, top_body, CapEnd::Start);
    let (doc, m1) = constrain_at(
        doc,
        top,
        seat(
            crate::fixture::head(a.clone()),
            crate::fixture::head_at(x1, b.clone()),
        ),
    );
    let far_corner = fixture::mate(
        crate::fixture::head(a),
        crate::fixture::head_at(x2, b),
        ContactClass::Rest,
        Alignment {
            a: MateFrame::authored(
                [3.0, 3.0, BASE_HEIGHT],
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 0.0],
                geom_core::Tol::witness(),
            )
            .expect("a definite frame"),
            b: b_frame(),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    );
    (doc, opts, m1, far_corner, top)
}

/// **A5.** A seat read through one transform of an instance pins the
/// instance, and a second seat through another transform of the same
/// instance reads the copy through a second reading of it, which the
/// door refuses, consistent with the first or not (`TargetsDiffer`, the
/// stage's named interim: a bundle reads its copy through one reading).
/// The first seat alone solves, and the product gathers.
#[test]
fn a5_a_second_seat_through_another_operand_is_refused() {
    for (what, label, extra_lift) in [
        ("consistent", "msolve1-a5-consistent", 0.0),
        ("inconsistent", "msolve1-a5-inconsistent", 3.0),
    ] {
        let (doc, opts, m1, far_corner, top) = two_operands(label, extra_lift);
        let poses = solve(&doc, &opts, Tol::witness());
        assert!(
            poses.fault(top).is_none(),
            "A5 {what}: the first seat solves: {:?}",
            poses.fault(top)
        );
        let refused =
            match fixture::at_the_door_of(&doc, &editor_core::RefusingReach, top, far_corner) {
                Err((_, fault)) => fault,
                Ok(_) => panic!("A5 {what}: the door admitted a second seat"),
            };
        assert!(
            matches!(
                &refused,
                MateFault::TargetsDiffer { placement, constraints }
                    if *placement == top && constraints.contains(&m1.id)
            ),
            "A5 {what}: the second seat is refused: {refused:?}"
        );
        let ev = run(&doc, &opts);
        assert!(
            product(&doc, &ev, Tol::witness()).is_ok(),
            "A5 {what}: the product gathers"
        );
    }
}

// ---- A6: the item-7 measurement, pinned ----

/// **A6.** The `Prismatic` measurement of item 7(a), pinned as the
/// behaviour it measured. The mate table's prismatic residual is
/// reachable as `Coaxial` + a clocking rider, and a bundle that leaves
/// a residual pins less than the copy needs: the copy is LOOSE, its own
/// space's root, with no refusal — with and without the transform,
/// identically. So the blindness was never class-dependent: it lived
/// only where the fold DETERMINES, and a free direction never got the
/// chance to absorb anything.
#[test]
fn a6_a_residual_bundle_leaves_the_copy_loose_with_or_without_the_transform() {
    // Each document's space for the top, and its fault.
    let residual = |label: &str, lift: bool| -> (Option<Space>, Option<MateFault>, RecipeNodeId) {
        let mut store = PartStore::default();
        let (base_ref, base_body) =
            store.insert_part(block(&format!("{label}-base"), 1.0), Tol::witness());
        let (top_ref, top_body) =
            store.insert_part(block(&format!("{label}-top"), 3.0), Tol::witness());
        let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
        let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
        let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
        let (doc, at) = if lift {
            insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0))
        } else {
            (doc, top)
        };
        let (doc, _) = constrain_at(
            doc,
            top,
            seat_with(
                crate::fixture::head(in_part(base, base_body, CapEnd::End)),
                crate::fixture::head_at(at, in_part(top, top_body, CapEnd::Start)),
                MatePrimitive::Coaxial,
                Some(0.0),
            ),
        );
        let o = EvalOptions {
            resolver: Some(Arc::new(store)),
            ..EvalOptions::default()
        };
        let poses = solve(&doc, &o, Tol::witness());
        (poses.space(top), poses.fault(top).cloned(), top)
    };
    for (what, lift) in [("plain", false), ("lifted", true)] {
        let (space, fault, top) = residual(&format!("msolve1-a6-{what}"), lift);
        assert_eq!(fault, None, "A6 {what}: a residual is no refusal");
        assert_eq!(
            space,
            Some(Space::Loose { root: top }),
            "A6 {what}: the top is a loose copy, its own space's root"
        );
    }
}

// ---- A7: nothing else moves ----

/// **A7.** A document with no transform and no pattern solves BIT for
/// bit whatever placers exist elsewhere in the vocabulary: the offset
/// is composed only when the walk's chain has a placer in it, and
/// `None` is kept as ABSENCE rather than as an identity, precisely so
/// this document composes nothing.
///
/// The pin is on BITS, and the numbers are this fixture's own solve —
/// an `Opposed` half turn about z (the `-1, -1, +1` diagonal, which
/// stands the block upright rather than flipping it) and the seat's
/// own translation.
#[test]
fn a7_a_document_with_no_placer_solves_bit_for_bit() {
    let s = scene("msolve1-a7", &[], &[]);
    let poses = solve(&s.doc, &s.opts, Tol::witness());
    let f = poses.relative(s.top).expect("the mated instance solves");
    println!("A7 relative(top) = {f:?}");
    assert_eq!(
        f.columns,
        [[-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]],
        "the Opposed half turn, unchanged"
    );
    assert_eq!(
        f.translation.map(f64::to_bits),
        [1.0_f64.to_bits(), 1.0_f64.to_bits(), BASE_HEIGHT.to_bits()],
        "the seat, bit for bit"
    );
    assert_eq!(
        poses.relative(s.base).map(|g| g.columns),
        Some(editor_core::Frame::IDENTITY.columns),
        "the target's own relative pose is the identity"
    );
}

// ---- A8: the doors ----

/// **A8(a).** A constraint whose operand never existed refuses typed
/// at the edit door — the name half's rule applied to the node half.
#[test]
fn a8a_an_operand_that_never_existed_refuses_at_the_insert_door() {
    let s = scene("msolve1-a8a", &[], &[]);
    let ghost = RecipeNodeId::new(0, 9_999);
    let err = s
        .doc
        .apply(
            &DocEdit::AddConstraint {
                placement: s.top,
                mate: Box::new(seat(
                    crate::fixture::head_at(ghost, in_part(s.base, s.base_body, CapEnd::End)),
                    crate::fixture::head(in_part(s.top, s.top_body, CapEnd::Start)),
                )),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect_err("a never-existed operand is a typo");
    assert!(
        matches!(&err, EditError::UnresolvedInput { input } if input.id() == ghost),
        "expected the unresolved read, got {err:?}"
    );
}

/// **A8(b).** Deleting the transform a mate reads at strands the side:
/// D10's unresolved reader, refused at the solve naming the side and at
/// evaluation as the unresolved read, until the side is re-authored.
#[test]
fn a8b_deleting_the_operand_leaves_a_dangling_head() {
    let s = scene("msolve1-a8b", &[], &[LIFT]);
    // Deleting the operand re-reads the constraint: the store's reach.
    let reach = editor_core::mate_reach::<f64>(&s.opts, Tol::witness());
    let (doc, _) = step_with(s.doc, DocEdit::DeleteNode { id: s.b_at }, &reach);
    let poses = solve(&doc, &s.opts, Tol::witness());
    let fault = poses
        .fault(s.mate.placement)
        .expect("the stranded mate refuses");
    // The side read a body the delete took: the solve names the side,
    // and the mate's evaluation refuses the unresolved read.
    assert!(
        matches!(
            fault,
            MateFault::SideUnresolved { mate, side, .. }
                if *mate == s.mate && *side == MateSide::B
        ),
        "expected the stranded side, got {fault:?}"
    );
    let ev = run(&doc, &s.opts);
    assert!(
        matches!(
            ev.node_error(s.mate.placement).map(|e| &e.kind),
            Some(editor_core::NodeErrorKind::UnresolvedRead {
                slot: editor_core::OperandSlot::Side {
                    constraint,
                    side: MateSide::B,
                },
                ..
            }) if *constraint == s.mate.id
        ),
        "{:?}",
        ev.node_error(s.mate.placement)
    );
    // The stranded SIDE contributes no edge; the live side still
    // does, which is what makes the refusal a per-side one.
    let edges: Vec<RecipeNodeId> = crate::fixture::mate_members(&doc)
        .into_iter()
        .filter(|&(m, _)| m == s.mate)
        .map(|(_, to)| to)
        .collect();
    assert_eq!(
        edges,
        vec![s.base],
        "only the resolving side lands on a member"
    );
}

/// **A8(c).** Two placements whose constraints differ only in their
/// operand are different content keys — the memo cannot serve one's
/// answer for the other.
/// Measured on the evaluated values, which is where a key is read.
#[test]
fn a8c_the_content_key_separates_two_operands() {
    // Two documents with the SAME nodes; only the `b` operand moves.
    let key_of = |label: &str, at_transform: bool| -> editor_core::ContentKey {
        let mut store = PartStore::default();
        let (base_ref, base_body) =
            store.insert_part(block("msolve1-a8c-base", 1.0), Tol::witness());
        let (top_ref, top_body) = store.insert_part(block("msolve1-a8c-top", 3.0), Tol::witness());
        let opts = with_resolver(store);
        let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
        let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
        let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
        let (doc, xf) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
        let (doc, mate) = constrain_at(
            doc,
            top,
            seat(
                crate::fixture::head(in_part(base, base_body, CapEnd::End)),
                crate::fixture::head_at(
                    if at_transform { xf } else { top },
                    in_part(top, top_body, CapEnd::Start),
                ),
            ),
        );
        let ev = run(&doc, &opts);
        ev.value(mate.placement)
            .expect("the placement is a value")
            .content_key
    };
    assert_ne!(
        key_of("msolve1-a8c-at-mint", false),
        key_of("msolve1-a8c-at-xform", true),
        "the operand is part of what a mate says"
    );
}

/// **A8(d).** A mate with a transform operand round-trips through
/// persistence: the same document, byte for byte, and the same solve.
#[test]
fn a8d_a_transform_operand_round_trips_through_persistence() {
    let s = scene("msolve1-a8d", &[], &[LIFT]);
    let bytes = save(&s.doc, &[], Tol::witness()).expect("the document saves");
    let back = load(&bytes, Tol::witness()).expect("it loads").doc;
    assert_eq!(
        save(&back, &[], Tol::witness()).expect("it saves again"),
        bytes,
        "the wire form is stable across the round trip"
    );
    let m = fixture::constraint(&back, s.mate);
    let (SideRead::Face(a), SideRead::Face(b)) = (m.a, m.b) else {
        panic!("the mate survived with two face sides");
    };
    assert_eq!(
        crate::fixture::side_head(&back, a).at,
        s.base,
        "the `a` side's body rode the wire"
    );
    assert_eq!(
        crate::fixture::side_head(&back, b).at,
        s.b_at,
        "the `b` side's body rode the wire"
    );
    let poses = solve(&back, &s.opts, Tol::witness());
    assert!(
        poses.fault(s.mate.placement).is_none(),
        "the loaded document solves"
    );
    let ev = run(&back, &s.opts);
    assert_seated(
        &back,
        &ev,
        &s.face_a(),
        &s.face_b(),
        &control_seat("msolve1-a8d"),
        "A8(d)",
    );
}

// ---- A10: a nested copy is a member ----

/// **A10.** A nested pattern's copy is a MEMBER: the walk consumes
/// both `Instance(i)` qualifiers, the solve places the mate, and the
/// document GATHERS — a pattern over an `Instances` value places
/// every instance, so the outer pattern's value is the `2 × 3` copies
/// laid out placement-major (output body `j·M + i`).
///
/// **The walk's numbering and the flat index coincide**, asserted on
/// the evaluated document: the name the walk reads as copies
/// `(j, i) = (1, 1)` resolves in the outer pattern's table to flat
/// body `1·3 + 1`, and that body's cap sits in the product exactly
/// where the walk's offset — the outer map at 1 over the inner map at
/// 1, LEFT of the placement the solve produced for the instance —
/// puts the part's own cap. The seat itself is measured against the
/// placer-free control, as every row here is.
#[test]
fn a10_a_nested_pattern_head_is_a_member() {
    let mut store = PartStore::default();
    let (base_ref, base_body) = store.insert_part(
        slab("msolve1-a10-base", BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let (top_ref, top_body) =
        store.insert_part(block("msolve1-a10-top", TOP_HEIGHT), Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a10"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    // Spaced so that every copy but the mated one clears the slab —
    // a sibling resting on the base uninvited is an unrecorded contact
    // the gate is right to refuse. Inner along x, outer along y.
    const SPACING: f64 = 5.0;
    let rule = |dir: [f64; 3]| PatternKind::Linear {
        direction: dir.map(scl),
        spacing: len(SPACING),
    };
    let (doc, inner) = insert(
        doc,
        Node::Pattern {
            input: top.into(),
            count: Formula::count(3),
            kind: rule([1.0, 0.0, 0.0]),
        },
    );
    let (doc, outer) = insert(
        doc,
        Node::Pattern {
            input: inner.into(),
            count: Formula::count(2),
            kind: rule([0.0, 1.0, 0.0]),
        },
    );
    let doc = world(doc, &[base], &[(outer, 6)]);
    let a = in_part(base, base_body, CapEnd::End);
    let nested = in_copy(
        outer,
        1,
        in_copy(inner, 1, in_part(top, top_body, CapEnd::Start)),
    );
    let (doc, _) = constrain_at(
        doc,
        top,
        seat(
            crate::fixture::head(a.clone()),
            crate::fixture::head_at(outer, nested.clone()),
        ),
    );
    let poses = solve(&doc, &opts, Tol::witness());
    assert!(
        poses.fault(top).is_none(),
        "a nested-pattern head resolves through both levels: {:?}",
        poses.fault(top)
    );
    assert_eq!(
        poses.space(top),
        Some(Space::World),
        "the nested copy's reference places the top"
    );

    // The document gathers, and the named copy seats on the base.
    let ev = run(&doc, &opts);
    assert!(
        ev.value(outer).is_some(),
        "the pattern of the pattern evaluates: {:?}",
        ev.node_error(outer)
    );
    assert_seated(&doc, &ev, &a, &nested, &control_seat("msolve1-a10"), "A10");
    assert!(
        gate(&doc, &ev).is_ok(),
        "A10: the gate refused: {:?}",
        gate(&doc, &ev).err()
    );

    // The numbering: the walk's (j, i) = (1, 1) is flat body 1·3 + 1.
    let table = &ev.value(outer).expect("the outer").name_table;
    let Some(editor_core::Entry::Unique(row)) = table.lookup(&nested) else {
        panic!(
            "the nested name resolves uniquely: {:?}",
            table.lookup(&nested)
        );
    };
    assert_eq!(row.body, 3 + 1, "copy (1, 1) is output body j·M + i");

    // The pose: the copy's cap in the product is the walk's offset
    // (outer map at 1 over inner map at 1: the authored spacing along
    // both authored directions) left of the placement the solve
    // produced for the instance, over the part's own cap frame — read
    // off a document that instantiates the part alone, at the identity.
    let placement = poses
        .placement(&doc, top)
        .expect("the instance is placed")
        .affine::<f64>();
    let alone = ProfileDoc::empty(DocumentId::derive("msolve1-a10-alone"), Tol::witness());
    let (alone, top_alone) = insert(alone, Node::instantiate_part(top_ref));
    let local = product_face_frame(
        &alone,
        &run(&alone, &opts),
        &in_part(top_alone, top_body, CapEnd::Start),
    );
    let offset = Affine3::translation(geom_core::Vec3::new(SPACING, SPACING, 0.0));
    let expected = offset * placement * local;
    let found = product_face_frame(&doc, &ev, &nested);
    let gap = map_gap(&found, &expected);
    assert!(
        gap <= 1e-12,
        "the copy's cap sits {gap} from the walk's offset over the solved placement"
    );
}

/// **A8(e).** A cut that would sever a constraint from its target is
/// refused: the cut takes `top` and the transform over it, and the
/// seat in `top`'s bundle reads `base`, which the cut leaves behind.
/// A side is a read, so this is D-2's ordinary severed read, named at
/// the side.
#[test]
fn a8e_a_cut_that_would_sever_the_target_refuses() {
    let s = scene("msolve1-a8e", &[], &[LIFT]);
    let cut = fixture::with_placements(
        &s.doc,
        &[s.top, s.b_at]
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>(),
    );
    let err = editor_core::split(
        &s.doc,
        &cut,
        DocumentId::derive("msolve1-a8e-part"),
        Tol::witness(),
        None,
    )
    .expect_err("the cut severs the seat's target");
    assert!(
        matches!(
            &err,
            editor_core::SplitError::SeveredEdge {
                consumer,
                slot: Some(editor_core::OperandSlot::Side { constraint, side: MateSide::A }),
                input,
                consumer_is_cut: true,
            } if *consumer == s.doc.spoken(s.top)
                && *constraint == s.mate.id
                && *input == s.doc.spoken(s.base)
        ),
        "expected the severed read of the seat's target, got {err:?}"
    );
}

/// **A8(f).** An ACCEPTED cut carries a constraint's operand through
/// the remap: the same id door the measure's `at` goes through, applied
/// to the half of a mate reference that is a node rather than a name.
/// Both halves come out pointing at the same nodes' images — the `b`
/// reference still read at the transform over its instance, the `a`
/// reference still read at its own mint.
#[test]
fn a8f_an_accepted_cut_carries_the_operand_through_the_remap() {
    let mut store = PartStore::default();
    let (base_ref, base_body) = store.insert_part(block("msolve1-a8f-base", 1.0), Tol::witness());
    let (top_ref, top_body) = store.insert_part(block("msolve1-a8f-top", 3.0), Tol::witness());
    // Local geometry FIRST, so the cut takes the low ids and the
    // instances and the mate all shift.
    let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a8f"), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 20.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, local) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, xf) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, _) = constrain_at(
        doc,
        top,
        seat(
            crate::fixture::head(in_part(base, base_body, CapEnd::End)),
            crate::fixture::head_at(xf, in_part(top, top_body, CapEnd::Start)),
        ),
    );
    let doc = world(doc, &[local, base, xf], &[]);
    // Cut the LOCAL block out into its own part, with the world
    // placement of what it moves: nothing it holds is read by a
    // constraint, so the cut is accepted.
    let frame = doc
        .ids()
        .into_iter()
        .find(|&id| id != RecipeNodeId::WORLD)
        .expect("the profile's frame is the first node");
    let cut = fixture::with_placements(
        &doc,
        &[profile, local, frame]
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>(),
    );
    let out = editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("msolve1-a8f-part"),
        Tol::witness(),
        None,
    )
    .expect("a cut of untouched local geometry is accepted");
    // The placement moved in the remainder's numbering; its
    // constraint's operand moved with it and still names the transform
    // over its instance.
    let [moved] = fixture::constrained(&out.remainder)
        .try_into()
        .expect("one constrained placement stayed in the remainder");
    let [c] = fixture::bundle(&out.remainder, moved)
        .try_into()
        .expect("its bundle holds the seat");
    let (SideRead::Face(a), SideRead::Face(b)) = (c.mate.a, c.mate.b) else {
        panic!("the seat kept two face sides");
    };
    let (a, b) = (
        crate::fixture::side_head(&out.remainder, a),
        crate::fixture::side_head(&out.remainder, b),
    );
    assert_ne!(
        b.at, b.name.node,
        "the `b` reference is still read somewhere other than its mint"
    );
    let Some(Node::Transform { input, .. }) = out.remainder.node(b.at) else {
        panic!("the `b` operand still names a transform");
    };
    assert_eq!(
        out.remainder.operation_of(*input),
        Some(b.name.node),
        "over the very instance the name is headed at"
    );
    assert_eq!(
        a.at, a.name.node,
        "the `a` reference is still read at its own mint"
    );
    assert!(
        matches!(out.remainder.node(a.at), Some(Node::InstantiatePart { .. })),
        "which is a live instance in the remainder"
    );
    let o = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    assert!(
        solve(&out.remainder, &o, Tol::witness())
            .fault(moved)
            .is_none(),
        "and the remainder still solves"
    );
}

/// **A8(g) — a KEPT placement whose constraint reads a body inside
/// the cut refuses.** The seat in `top`'s bundle reads the base through
/// a transform at its `b` side; the cut takes the base and that
/// transform, and keeps `top`. A side is a read, so this is D-2's
/// ordinary severed read.
#[test]
fn a8g_a_kept_constraint_whose_operand_is_cut_refuses_at_the_door() {
    let (doc, mate, base, xf) = severed_operand_scene("msolve1-a8g");
    let cut = fixture::with_placements(&doc, &[base, xf].into_iter().collect());
    let err = editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("msolve1-a8g-part"),
        Tol::witness(),
        None,
    )
    .expect_err("a kept constraint cannot keep an operand the cut took");
    assert!(
        matches!(
            &err,
            editor_core::SplitError::SeveredEdge {
                consumer,
                slot: Some(editor_core::OperandSlot::Side { constraint, side: MateSide::B }),
                input,
                consumer_is_cut: false,
            } if *consumer == doc.spoken(mate.placement)
                && *constraint == mate.id
                && *input == doc.spoken(xf)
        ),
        "expected the severed-read refusal naming the placement and the body \
         its constraint reads, got {err:?}"
    );
    let text = err.to_string();
    for id in [mate.placement, xf] {
        assert!(
            !text.contains(&id.0.to_string()),
            "{id:?} in decimal: {text}"
        );
    }
}

/// **A8(h) — and the other direction.** A CUT placement whose
/// constraint reads a body that stays in the remainder refuses with the
/// same variant.
#[test]
fn a8h_a_cut_constraint_whose_operand_is_kept_refuses_with_the_same_variant() {
    let (doc, mate, _, xf) = severed_operand_scene("msolve1-a8h");
    let cut = fixture::with_placements(&doc, &[mate.placement].into_iter().collect());
    let err = editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("msolve1-a8h-part"),
        Tol::witness(),
        None,
    )
    .expect_err("a cut constraint cannot carry an operand the part does not have");
    assert!(
        matches!(
            &err,
            editor_core::SplitError::SeveredEdge {
                consumer,
                slot: Some(editor_core::OperandSlot::Side { constraint, side: MateSide::B }),
                input,
                consumer_is_cut: true,
            } if *consumer == doc.spoken(mate.placement)
                && *constraint == mate.id
                && *input == doc.spoken(xf)
        ),
        "expected the severed-read refusal, got {err:?}"
    );
}

/// A document whose seat, in `top`'s bundle, reads `top`'s own face at
/// its `a` side and the base through a transform over it at its `b`
/// side; answers the seat, the base and the transform.
fn severed_operand_scene(label: &str) -> (ProfileDoc, MateRef, RecipeNodeId, RecipeNodeId) {
    let mut store = PartStore::default();
    let (base_ref, base_body) = store.insert_part(
        slab(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let (top_ref, top_body) =
        store.insert_part(block(&format!("{label}-top"), TOP_HEIGHT), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, xf) = insert(doc, xform(base, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, mate) = constrain_at(
        doc,
        top,
        seat(
            crate::fixture::head(in_part(top, top_body, CapEnd::Start)),
            crate::fixture::head_at(xf, in_part(base, base_body, CapEnd::End)),
        ),
    );
    (doc, mate, base, xf)
}

// ---- A11: a transform BETWEEN two patterns ----

/// **A11.** A transform between the two patterns: the walk's chain is
/// `[outer(j), T, inner(i)]` and the evaluator's body is
/// `M_o(j) ∘ T ∘ M_i(i)` — asserted on the product with a ROTATION,
/// so the other order is a different map and the row can tell them
/// apart.
#[test]
fn a11_a_transform_between_two_patterns_composes_outer_t_inner() {
    let mut store = PartStore::default();
    let (base_ref, base_body) = store.insert_part(
        slab("msolve1-a11-base", BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let (top_ref, top_body) =
        store.insert_part(block("msolve1-a11-top", TOP_HEIGHT), Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a11"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let rule = |dir: [f64; 3], s: f64| PatternKind::Linear {
        direction: dir.map(scl),
        spacing: len(s),
    };
    let (doc, inner) = insert(
        doc,
        Node::Pattern {
            input: top.into(),
            count: Formula::count(3),
            kind: rule([1.0, 0.0, 0.0], 5.0),
        },
    );
    let spin = std::f64::consts::FRAC_PI_2;
    let (doc, t) = insert(doc, xform(inner, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], spin));
    let (doc, outer) = insert(
        doc,
        Node::Pattern {
            input: t.into(),
            count: Formula::count(2),
            kind: rule([0.0, 1.0, 0.0], 20.0),
        },
    );
    let doc = world(doc, &[base], &[(outer, 6)]);
    let a = in_part(base, base_body, CapEnd::End);
    let nested = in_copy(
        outer,
        1,
        in_copy(inner, 1, in_part(top, top_body, CapEnd::Start)),
    );
    let (doc, _) = constrain_at(
        doc,
        top,
        seat(
            crate::fixture::head(a.clone()),
            crate::fixture::head_at(outer, nested.clone()),
        ),
    );
    let poses = solve(&doc, &opts, Tol::witness());
    assert!(poses.fault(top).is_none(), "{:?}", poses.fault(top));
    let ev = run(&doc, &opts);
    assert!(ev.value(outer).is_some(), "{:?}", ev.node_error(outer));
    assert_seated(&doc, &ev, &a, &nested, &control_seat("msolve1-a11"), "A11");
    assert!(gate(&doc, &ev).is_ok(), "{:?}", gate(&doc, &ev).err());
    let table = &ev.value(outer).unwrap().name_table;
    let Some(editor_core::Entry::Unique(row)) = table.lookup(&nested) else {
        panic!("{:?}", table.lookup(&nested))
    };
    assert_eq!(row.body, 3 + 1, "copy (1, 1) is flat body j·M + i");
    let placement = poses.placement(&doc, top).unwrap().affine::<f64>();
    let alone = ProfileDoc::empty(DocumentId::derive("msolve1-a11-alone"), Tol::witness());
    let (alone, top_alone) = insert(alone, Node::instantiate_part(top_ref));
    let local = product_face_frame(
        &alone,
        &run(&alone, &opts),
        &in_part(top_alone, top_body, CapEnd::Start),
    );
    let m_o = Affine3::translation(geom_core::Vec3::new(0.0, 20.0, 0.0));
    let t_map = Affine3::rotation_about_axis(
        geom_core::Point3::origin(),
        geom_core::Vec3::new(0.0, 0.0, 1.0),
        spin,
    );
    let m_i = Affine3::translation(geom_core::Vec3::new(5.0, 0.0, 0.0));
    let expected = m_o * t_map * m_i * placement * local;
    let found = product_face_frame(&doc, &ev, &nested);
    let gap = map_gap(&found, &expected);
    assert!(gap <= 1e-12, "outer ∘ T ∘ inner: gap {gap}");
    let wrong = m_i * t_map * m_o * placement * local;
    assert!(
        map_gap(&found, &wrong) > 1.0,
        "the two orders are distinguishable here"
    );
}

// ---- A12: a Part over a nested pattern, and the mate read AT it ----

/// What a `Part(k)` row expects: the mate seats (the `Part`'s `k` IS the
/// flat body the name's chain says), or the solve refuses
/// `PartSelectsAnotherCopy` naming the flat index the name says.
#[derive(Clone, Copy)]
enum PartCase {
    Seats,
    Refuses { named: u32 },
}

/// The nested document of A10 with a `Part(k)` over the outer pattern
/// — through a transform when `via_transform` — and the mate read AT
/// that part, naming copy `(j, i)`.
fn part_over_nested(k: i64, j: u32, i: u32, via_transform: bool, expect: PartCase) {
    let label = format!("msolve1-a12-{k}-{j}-{i}-{via_transform}");
    let mut store = PartStore::default();
    let (base_ref, base_body) = store.insert_part(
        slab(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let (top_ref, top_body) =
        store.insert_part(block(&format!("{label}-top"), TOP_HEIGHT), Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive(&label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let rule = |dir: [f64; 3]| PatternKind::Linear {
        direction: dir.map(scl),
        spacing: len(5.0),
    };
    let (doc, inner) = insert(
        doc,
        Node::Pattern {
            input: top.into(),
            count: Formula::count(3),
            kind: rule([1.0, 0.0, 0.0]),
        },
    );
    let (doc, outer) = insert(
        doc,
        Node::Pattern {
            input: inner.into(),
            count: Formula::count(2),
            kind: rule([0.0, 1.0, 0.0]),
        },
    );
    let (doc, of) = if via_transform {
        insert(doc, xform(outer, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0))
    } else {
        (doc, outer)
    };
    let (doc, part) = insert(
        doc,
        Node::Part {
            of: of.into(),
            select: editor_core::PartSelect::Instance(Formula::count(k)),
        },
    );
    let a = in_part(base, base_body, CapEnd::End);
    let nested = in_copy(
        outer,
        j,
        in_copy(inner, i, in_part(top, top_body, CapEnd::Start)),
    );
    let mate = seat(
        crate::fixture::head(a.clone()),
        crate::fixture::head_at(part, nested.clone()),
    );
    let what = format!("Part({k}) naming ({j}, {i}), via transform: {via_transform}");
    // A `Part` that disagrees with the name is a fact about the mate
    // alone, so the edit door refuses it where the mate is authored
    // with the solve's own fault; a seating one enters and solves.
    let inserted = doc.apply(
        &DocEdit::AddConstraint {
            placement: top,
            mate: Box::new(mate),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    match expect {
        PartCase::Seats => {
            let applied = inserted.unwrap_or_else(|err| panic!("{what}: {err:?}"));
            assert_eq!(
                applied.record.minted_constraints.len(),
                1,
                "{what}: an added constraint mints an id"
            );
            let doc = world(applied.doc, &[base, part], &[]);
            let poses = solve(&doc, &opts, Tol::witness());
            let ev = run(&doc, &opts);
            assert!(poses.fault(top).is_none(), "{what}: {:?}", poses.fault(top));
            assert_eq!(poses.space(top), Some(Space::World), "{what}");
            assert!(
                ev.node_error(top).is_none(),
                "{what}: {:?}",
                ev.node_error(top)
            );
            assert!(
                ev.value(part).is_some(),
                "{what}: {:?}",
                ev.node_error(part)
            );
            assert_seated(&doc, &ev, &a, &nested, &control_seat(&label), &what);
            assert!(
                gate(&doc, &ev).is_ok(),
                "{what}: {:?}",
                gate(&doc, &ev).err()
            );
        }
        PartCase::Refuses { named } => {
            let err = inserted.expect_err("a disagreeing Part refuses at the door");
            let EditError::MateRefused { fault, .. } = &err else {
                panic!("{what}: expected MateRefused, got {err:?}");
            };
            assert!(
                matches!(
                    **fault,
                    MateFault::PartSelectsAnotherCopy {
                        part: p,
                        named: n,
                        selected,
                        ..
                    } if p == part && n == named && selected == k
                ),
                "{what}: expected PartSelectsAnotherCopy(named {named}, selected {k}), got {fault:?}"
            );
        }
    }
}

/// **A12.** A `Part(k)` over a nested pattern selects FLAT body `k`,
/// and a mate read at it names a copy `(j, i)` whose flat index is
/// `j·3 + i`: the solve compares the two in the `Part`'s own index
/// space, so `k == j` with `i ≠ 0` is a disagreement (the `Part`
/// gathers copy `(0, k)`, five units from the copy the name places),
/// not an agreement — and the same through a transform above the
/// outer pattern, which preserves the value's indices.
#[test]
fn a12_a_part_over_a_nested_pattern_agrees_in_the_flat_index() {
    for (k, j, i, via) in [
        (4, 1, 1, false),
        (1, 0, 1, false),
        (4, 1, 1, true),
        (0, 0, 0, true),
    ] {
        part_over_nested(k, j, i, via, PartCase::Seats);
    }
    for (k, j, i, via, named) in [
        (1, 1, 1, false, 4),
        (0, 0, 1, false, 1),
        (4, 0, 1, false, 1),
        (1, 1, 1, true, 4),
    ] {
        part_over_nested(k, j, i, via, PartCase::Refuses { named });
    }
}

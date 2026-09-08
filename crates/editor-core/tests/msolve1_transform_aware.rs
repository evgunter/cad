//! **A mate reads at its operand** — the transform-aware solve.
//!
//! A mate's two references are `SitedRef`s: a name, and the node the
//! reference is read at. The solve walks from that operand down to
//! the name's minting instance and composes the map of every
//! pose-bearing node it passes, so a mate on a TRANSFORMED instance
//! seats the transformed geometry and a mate on the instance seats
//! the instance.
//!
//! Every row here goes through ordinary doors — `DocEdit::InsertNode`,
//! `solve_document`, `evaluate`, `product` — and measures the
//! PRODUCT's own face frames: the planes the mate's two names resolve
//! to in the body a consumer actually gathers, never a solved frame
//! read by eye.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, AssemblyError, Attribution, AxisSense, CapEnd, ContactClass, DocEdit, DocRef,
    DocumentId, EditError, EntityKind, EvalOptions, Evaluation, Expr, MateFault, MateFrame,
    MatePrimitive, MateRole, MateSide, Node, PartResolver, PatternKind, ProfileDoc, RecipeNodeId,
    ResolveFailure, ResolveFault, RoleSeg, SitedRef, StableName, content_pin, load, product, save,
    solve_document,
};
use fixture::seat::{assert_seated, map_gap, product_face_frame, seat_map};
use fixture::{gate, in_copy, insert, len, on_frame, run, scl, step, xform};
use geom_core::Tol;
use geom_core::linalg::Affine3;

// ---- substrate ----

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

/// The extrude in a one-block part document (frame, profile, extrude).
const PART_BODY: RecipeNodeId = RecipeNodeId(2);

/// A `wxwxh` block, as a whole part document.
fn slab(label: &str, w: f64, h: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, w), (0.0, w)]],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(h),
        },
    );
    doc
}

/// A `1x1xh` block, as a whole part document.
fn block(label: &str, h: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(h),
        },
    );
    doc
}

/// A face of `instance`'s part product — the plain member spelling.
fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: PART_BODY,
                path: vec![RoleSeg::Cap(cap)],
            }),
        }],
    }
}

/// The `a` frame: a point ON the base's top cap, axis along that
/// cap's OUTWARD normal.
fn a_frame() -> MateFrame {
    MateFrame {
        origin: [1.0, 1.0, BASE_HEIGHT],
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    }
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
fn b_frame() -> MateFrame {
    MateFrame {
        origin: [0.0, 0.0, 0.0],
        axis: [0.0, 0.0, -1.0],
        reference: [1.0, 0.0, 0.0],
    }
}

/// A `Rest` mate seating `b`'s bottom cap onto `a`'s top cap, both
/// frames authored in their member's own part coordinates.
fn seat(a: SitedRef, b: SitedRef) -> Node<editor_core::ProfileProgram> {
    seat_with(a, b, MatePrimitive::FrameCoincidence, None)
}

fn seat_with(
    a: SitedRef,
    b: SitedRef,
    primitive: MatePrimitive,
    clocking: Option<f64>,
) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a,
        b,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: a_frame(),
            b: b_frame(),
            primitive,
            sense: AxisSense::Opposed,
            clocking,
        },
    }
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
    /// The last node of `base`'s chain (the `a` operand).
    a_at: RecipeNodeId,
    /// The last node of `top`'s chain (the `b` operand).
    b_at: RecipeNodeId,
    mate: RecipeNodeId,
}

/// One link of a transform chain: `(translation, rotation axis,
/// angle)`.
type Step = ([f64; 3], [f64; 3], f64);

/// Builds that scene. `on_base` / `on_top` are the transform chains,
/// innermost first.
fn scene(label: &str, on_base: &[Step], on_top: &[Step]) -> Scene {
    let mut store = StubStore::default();
    let base_ref = store.insert(
        slab(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let top_ref = store.insert(block(&format!("{label}-top"), TOP_HEIGHT), Tol::witness());
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, a_at) = chain(doc, base, on_base);
    let (doc, b_at) = chain(doc, top, on_top);
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(
                SitedRef::new(a_at, in_part(base, CapEnd::End)),
                SitedRef::new(b_at, in_part(top, CapEnd::Start)),
            ),
        },
    );
    Scene {
        doc,
        opts,
        base,
        top,
        a_at,
        b_at,
        mate: mate.unwrap(),
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
        in_part(self.base, CapEnd::End)
    }
    fn face_b(&self) -> StableName {
        in_part(self.top, CapEnd::Start)
    }
    /// Every node evaluated, no mate fault, the mated faces seated in
    /// the product, and **the at-rest gate satisfied** — which is the
    /// check that says the declaration the document makes is the one
    /// the geometry keeps.
    fn assert_green_and_seated(&self, control: &Affine3<f64>, what: &str) {
        let poses = solve_document(&self.doc, Tol::witness());
        assert!(
            poses.fault(self.mate).is_none() && poses.fault(self.top).is_none(),
            "{what}: the solve refused: {:?} / {:?}",
            poses.fault(self.mate),
            poses.fault(self.top)
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
    let c = solve_document(&control.doc, Tol::witness())
        .relative(control.top)
        .expect("the control solves");
    let t = solve_document(&test.doc, Tol::witness())
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
        let mut store = StubStore::default();
        let base_ref = store.insert(
            slab("msolve1-a3a-base", BASE_WIDTH, BASE_HEIGHT),
            Tol::witness(),
        );
        let top_ref = store.insert(block("msolve1-a3a-top", TOP_HEIGHT), Tol::witness());
        let opts = EvalOptions {
            resolver: Some(Arc::new(store)),
            ..EvalOptions::default()
        };
        let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a3a"), Tol::witness());
        let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
        let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
        let (doc, xf) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
        let (doc, pattern) = insert(
            doc,
            Node::Pattern {
                input: xf,
                count: Expr::count(3),
                // Spaced wide enough that copies 0 and 2 clear the
                // slab entirely: the row is about the copy the mate
                // names, and a sibling resting on the base uninvited
                // is an UNDECLARED contact the gate is right to refuse.
                kind: PatternKind::Linear {
                    direction: [scl(1.0), scl(0.0), scl(0.0)],
                    spacing: len(5.0),
                },
            },
        );
        let a = in_part(base, CapEnd::End);
        let b = in_copy(pattern, COPY, in_part(top, CapEnd::Start));
        let (doc, mate) = step(
            doc,
            DocEdit::InsertNode {
                node: seat(
                    SitedRef::at_mint(a.clone()),
                    SitedRef::new(pattern, b.clone()),
                ),
            },
        );
        let mate = mate.unwrap();
        let poses = solve_document(&doc, Tol::witness());
        assert!(
            poses.fault(mate).is_none(),
            "A3 pattern-of-transform refused: {:?}",
            poses.fault(mate)
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
    // offset is T ∘ M(1) — the mate SOLVES and determines its pair,
    // and the document GATHERS: a transform is shape-preserving over
    // its input's value, so the transform of the pattern is the
    // pattern's instances under one map, and the named copy seats on
    // the base in the product. Copies 0 and 2 are spaced to clear the
    // slab, as in (a).
    {
        let mut store = StubStore::default();
        let base_ref = store.insert(
            slab("msolve1-a3b-base", BASE_WIDTH, BASE_HEIGHT),
            Tol::witness(),
        );
        let top_ref = store.insert(block("msolve1-a3b-top", TOP_HEIGHT), Tol::witness());
        let opts = EvalOptions {
            resolver: Some(Arc::new(store)),
            ..EvalOptions::default()
        };
        let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a3b"), Tol::witness());
        let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
        let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
        let (doc, pattern) = insert(
            doc,
            Node::Pattern {
                input: top,
                count: Expr::count(3),
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
        let a = in_part(base, CapEnd::End);
        let b = in_copy(pattern, COPY, in_part(top, CapEnd::Start));
        let (doc, mate) = step(
            doc,
            DocEdit::InsertNode {
                node: seat(SitedRef::at_mint(a.clone()), SitedRef::new(xf, b.clone())),
            },
        );
        let mate = mate.unwrap();
        let poses = solve_document(&doc, Tol::witness());
        assert!(
            poses.fault(mate).is_none(),
            "A3 transform-of-pattern refused at the solve: {:?}",
            poses.fault(mate)
        );
        assert_eq!(
            poses.role(mate),
            Some(MateRole::Determining),
            "the transform-of-pattern reference places its pair"
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

/// **A4.** A transform over the GAUGE side, over both sides, chains
/// of two on either side, and — the row the rest do not reach —
/// chains whose two maps DO NOT COMMUTE.
///
/// The gauge is the document-order-first instance, so a transform
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
        ("gauge side only", vec![lift(4.0)], vec![]),
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
            "a chain on the gauge and one on the mate",
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
        // ...and on the GAUGE side, where the map is un-wound.
        (
            "a non-commuting chain on the gauge side",
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
        solve_document(&s.doc, Tol::witness())
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

/// **A4″ — a non-identity recorded frame on the gauge's cluster**,
/// with transforms on both sides. The pair's static factor is
/// conjugated through the cluster frame (`pair_left_factor`), so a
/// document whose gauge carries an authored placement is the case
/// where that conjugation has to be right as well as the composition.
#[test]
fn a4_a_placed_gauge_cluster_seats_through_both_chains() {
    let seat = control_seat("msolve1-a4-placed");
    let s = scene(
        "msolve1-a4-placed-frame",
        &[([1.0, 2.0, 3.0], [1.0, 1.0, 0.0], -0.6)],
        &[([0.0, 5.0, 10.0], [0.0, 1.0, 1.0], 1.3)],
    );
    let (doc, _) = step(
        s.doc.clone(),
        DocEdit::SetPlacement {
            node: s.base,
            frame: editor_core::Frame::rotate_then_translate(
                [0.3, -0.2, 0.9],
                0.8,
                [2.0, -1.0, 3.0],
                fixture::band(),
            )
            .expect("a literal axis has a definite direction"),
        },
    );
    let placed = Scene { doc, ..s };
    placed.assert_green_and_seated(&seat, "A4 placed gauge cluster");
}

// ---- A5: two operands, one instance ----

/// Two mates from `base` to `top`, each read at a DIFFERENT node over
/// `top`: `x1` lifts, and `x2` sits over `x1` carrying `second`.
///
/// `second` is what makes the pair consistent or not, and it is never
/// the identity: two operands that compose to the same map are one
/// member wearing two names, and a row built on them would show
/// nothing about two members at all. The consistent case turns the
/// block a quarter turn about its own vertical centre line — a
/// DIFFERENT map that asks for the same seat, because the block is
/// square and the alignment fixes no roll the turn disturbs.
fn two_operands(label: &str, second: Step) -> (ProfileDoc, EvalOptions, [RecipeNodeId; 2]) {
    let mut store = StubStore::default();
    let base_ref = store.insert(
        slab(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let top_ref = store.insert(block(&format!("{label}-top"), TOP_HEIGHT), Tol::witness());
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, x1) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, x2) = insert(doc, xform(x1, second.0, second.1, second.2));
    let a = in_part(base, CapEnd::End);
    let b = in_part(top, CapEnd::Start);
    let (doc, m1) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(SitedRef::at_mint(a.clone()), SitedRef::new(x1, b.clone())),
        },
    );
    let (doc, m2) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(SitedRef::at_mint(a), SitedRef::new(x2, b)),
        },
    );
    (doc, opts, [m1.unwrap(), m2.unwrap()])
}

/// **A5.** Two mates from one instance through two different maps are
/// two MEMBERS over one instance: they key as different pairs, so the
/// second is a loop-closing DECLARING edge rather than a fold-mate of
/// the first. A geometrically consistent pair solves AND passes the
/// at-rest gate; an inconsistent one is refused there, attributed to
/// the declaration the geometry contradicts.
///
/// The gate is where a declaring mate is verified — the solve places
/// on the tree edge and never checks the loop (A11 rule 4) — so a
/// fixture whose CONSISTENT pair the gate also refuses would make
/// this row vacuous. Both halves are asserted.
#[test]
fn a5_two_operands_over_one_instance_are_two_members() {
    // Consistent: `x2` is a quarter turn of the square block about its
    // own vertical centre line. A different map from `x1`'s, and one
    // that asks for the same seat.
    let quarter_turn: Step = (
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        std::f64::consts::FRAC_PI_2,
    );
    let (doc, opts, [m1, m2]) = two_operands("msolve1-a5-consistent", quarter_turn);
    let poses = solve_document(&doc, Tol::witness());
    assert!(
        poses.fault(m1).is_none() && poses.fault(m2).is_none(),
        "A5 consistent: {:?} / {:?}",
        poses.fault(m1),
        poses.fault(m2)
    );
    assert_eq!(
        poses.role(m1),
        Some(MateRole::Determining),
        "the first pair is the tree edge"
    );
    assert_eq!(
        poses.role(m2),
        Some(MateRole::Declaring),
        "the second member pair closes a loop"
    );
    let ev = run(&doc, &opts);
    assert!(
        product(&doc, &ev, Tol::witness()).is_ok(),
        "A5 consistent: the product gathers"
    );
    assert!(
        gate(&doc, &ev).is_ok(),
        "A5 consistent: the gate refused a consistent pair: {:?}",
        gate(&doc, &ev).err()
    );

    // Inconsistent: the second operand lifts 3 further, so the two
    // mates cannot both be satisfied. The solve still places on the
    // tree edge; the GATE is where the declaring mate is verified
    // against the solved geometry, and it refuses.
    let (doc, opts, [m1, m2]) = two_operands(
        "msolve1-a5-inconsistent",
        ([0.0, 0.0, 3.0], [0.0, 0.0, 1.0], 0.0),
    );
    let poses = solve_document(&doc, Tol::witness());
    assert!(
        poses.fault(m1).is_none() && poses.fault(m2).is_none(),
        "A5 inconsistent: the SOLVE places on the tree edge and does \
         not verify the loop; the gate does"
    );
    let ev = run(&doc, &opts);
    let err = gate(&doc, &ev).expect_err("the gate refuses the unmet declaration");
    let AssemblyError::AtRest { findings } = &err else {
        panic!("A5 inconsistent: expected the at-rest gate's refusal, got {err:?}");
    };
    assert!(
        findings
            .iter()
            .any(|f| matches!(f.attribution, Attribution::Refuted(_))),
        "A5 inconsistent: the refusal names the declaration the geometry \
         contradicts, rather than only an undeclared contact: {findings:?}"
    );
}

// ---- A6: the item-7 measurement, pinned ----

/// **A6.** The `Prismatic` measurement of item 7(a), pinned as the
/// behaviour it measured. The mate table's prismatic residual is
/// reachable as `Coaxial` + a clocking rider, and a residual on a
/// TREE edge refuses `Under` (A11 rule 4) before any pose exists —
/// with and without the transform, identically. So the blindness was
/// never class-dependent: it lived only where the fold DETERMINES,
/// and a free direction never got the chance to absorb anything.
#[test]
fn a6_a_residual_tree_edge_refuses_under_with_or_without_the_transform() {
    let residual = |label: &str, lift: bool| -> MateFault {
        let mut store = StubStore::default();
        let base_ref = store.insert(block(&format!("{label}-base"), 1.0), Tol::witness());
        let top_ref = store.insert(block(&format!("{label}-top"), 3.0), Tol::witness());
        let _ = &store;
        let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
        let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
        let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
        let (doc, at) = if lift {
            insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0))
        } else {
            (doc, top)
        };
        let (doc, mate) = step(
            doc,
            DocEdit::InsertNode {
                node: seat_with(
                    SitedRef::at_mint(in_part(base, CapEnd::End)),
                    SitedRef::new(at, in_part(top, CapEnd::Start)),
                    MatePrimitive::Coaxial,
                    Some(0.0),
                ),
            },
        );
        solve_document(&doc, Tol::witness())
            .fault(mate.unwrap())
            .cloned()
            .expect("a residual tree edge refuses")
    };
    let plain = residual("msolve1-a6-plain", false);
    let lifted = residual("msolve1-a6-lifted", true);
    for (what, fault) in [("plain", &plain), ("lifted", &lifted)] {
        let MateFault::Under { residual, .. } = fault else {
            panic!("A6 {what}: expected Under, got {fault:?}");
        };
        assert!(
            matches!(residual, editor_core::Subgroup::Prismatic { .. }),
            "A6 {what}: the residual is the prismatic one, got {residual:?}"
        );
    }
    // The two refusals differ only in the mate's own node id (the
    // lifted document has one node more): same parent, same child,
    // same residual. A mate that never places is blind to nothing.
    let parts = |f: &MateFault| match f {
        MateFault::Under {
            parent,
            child,
            residual,
            ..
        } => (*parent, *child, format!("{residual:?}")),
        other => panic!("expected Under, got {other:?}"),
    };
    assert_eq!(
        parts(&plain),
        parts(&lifted),
        "A6: the refusal does not depend on the transform"
    );
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
    let poses = solve_document(&s.doc, Tol::witness());
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
        "the gauge's own relative pose is the identity"
    );
}

// ---- A8: the doors ----

/// **A8(a).** An insert whose operand never existed refuses typed at
/// the edit door — the name half's rule applied to the node half.
#[test]
fn a8a_an_operand_that_never_existed_refuses_at_the_insert_door() {
    let s = scene("msolve1-a8a", &[], &[]);
    let ghost = RecipeNodeId(9_999);
    let err = s
        .doc
        .apply(
            &DocEdit::InsertNode {
                node: seat(
                    SitedRef::new(ghost, in_part(s.base, CapEnd::End)),
                    SitedRef::at_mint(in_part(s.top, CapEnd::Start)),
                ),
            },
            Tol::witness(),
        )
        .expect_err("a never-existed operand is a typo");
    assert!(
        matches!(err, EditError::ReadSiteMissingNode { at } if at == ghost),
        "expected ReadSiteMissingNode, got {err:?}"
    );
}

/// **A8(b).** Deleting the transform a mate reads at strands the
/// operand: N5's dangling semantics, refused at the SOLVE naming the
/// side and the head, with no edge until the mate is re-authored.
#[test]
fn a8b_deleting_the_operand_leaves_a_dangling_head() {
    let s = scene("msolve1-a8b", &[], &[LIFT]);
    let (doc, _) = step(s.doc, DocEdit::DeleteNode { id: s.b_at });
    let poses = solve_document(&doc, Tol::witness());
    let fault = poses.fault(s.mate).expect("the stranded mate refuses");
    // The head the fault names is where the WALK STOPPED — the
    // stranded operand — not the reference's own head node, which is
    // still live and still fine.
    assert!(
        matches!(
            fault,
            MateFault::DanglingHead { mate, side, head }
                if *mate == s.mate && *side == MateSide::B && *head == s.b_at
        ),
        "expected a dangling head at the deleted operand ({:?}), got {fault:?}",
        s.b_at
    );
    // The stranded SIDE contributes no edge; the live side still
    // does, which is what makes the refusal a per-side one.
    let edges: Vec<RecipeNodeId> = editor_core::reading_edges(&doc)
        .into_iter()
        .filter(|&(m, _)| m == s.mate)
        .map(|(_, to)| to)
        .collect();
    assert_eq!(
        edges,
        vec![s.base],
        "only the resolving side contributes a reading edge"
    );
}

/// **A8(c).** Two mates differing only in their operand are different
/// content keys — the memo cannot serve one's answer for the other.
/// Measured on the evaluated values, which is where a key is read.
#[test]
fn a8c_the_content_key_separates_two_operands() {
    // Two documents with the SAME nodes; only the `b` operand moves.
    let key_of = |label: &str, at_transform: bool| -> editor_core::ContentKey {
        let mut store = StubStore::default();
        let base_ref = store.insert(block("msolve1-a8c-base", 1.0), Tol::witness());
        let top_ref = store.insert(block("msolve1-a8c-top", 3.0), Tol::witness());
        let opts = EvalOptions {
            resolver: Some(Arc::new(store)),
            ..EvalOptions::default()
        };
        let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
        let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
        let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
        let (doc, xf) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
        let (doc, mate) = step(
            doc,
            DocEdit::InsertNode {
                node: seat(
                    SitedRef::at_mint(in_part(base, CapEnd::End)),
                    SitedRef::new(
                        if at_transform { xf } else { top },
                        in_part(top, CapEnd::Start),
                    ),
                ),
            },
        );
        let ev = run(&doc, &opts);
        ev.value(mate.unwrap())
            .expect("the mate is a value")
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
    let Some(Node::Mate { a, b, .. }) = back.node(s.mate) else {
        panic!("the mate survived");
    };
    assert_eq!(a.at, s.base, "the `a` operand rode the wire");
    assert_eq!(b.at, s.b_at, "the `b` operand rode the wire");
    let poses = solve_document(&back, Tol::witness());
    assert!(poses.fault(s.mate).is_none(), "the loaded document solves");
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
    let mut store = StubStore::default();
    let base_ref = store.insert(
        slab("msolve1-a10-base", BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let top_ref = store.insert(block("msolve1-a10-top", TOP_HEIGHT), Tol::witness());
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a10"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    // Spaced so that every copy but the mated one clears the slab —
    // a sibling resting on the base uninvited is an undeclared contact
    // the gate is right to refuse. Inner along x, outer along y.
    const SPACING: f64 = 5.0;
    let rule = |dir: [f64; 3]| PatternKind::Linear {
        direction: dir.map(scl),
        spacing: len(SPACING),
    };
    let (doc, inner) = insert(
        doc,
        Node::Pattern {
            input: top,
            count: Expr::count(3),
            kind: rule([1.0, 0.0, 0.0]),
        },
    );
    let (doc, outer) = insert(
        doc,
        Node::Pattern {
            input: inner,
            count: Expr::count(2),
            kind: rule([0.0, 1.0, 0.0]),
        },
    );
    let a = in_part(base, CapEnd::End);
    let nested = in_copy(outer, 1, in_copy(inner, 1, in_part(top, CapEnd::Start)));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(
                SitedRef::at_mint(a.clone()),
                SitedRef::new(outer, nested.clone()),
            ),
        },
    );
    let mate = mate.unwrap();
    let poses = solve_document(&doc, Tol::witness());
    assert!(
        poses.fault(mate).is_none(),
        "a nested-pattern head resolves through both levels: {:?}",
        poses.fault(mate)
    );
    assert_eq!(
        poses.role(mate),
        Some(MateRole::Determining),
        "the nested copy's reference places its pair"
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
    // off a document that instantiates the part alone, unplaced.
    let placement = poses
        .placement(&doc, top)
        .expect("the instance is placed")
        .affine::<f64>();
    let alone = ProfileDoc::empty(DocumentId::derive("msolve1-a10-alone"), Tol::witness());
    let (alone, top_alone) = insert(alone, Node::instantiate_part(top_ref));
    let local = product_face_frame(
        &alone,
        &run(&alone, &opts),
        &in_part(top_alone, CapEnd::Start),
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

/// **A8(e).** A cut that would sever a mate from its operand is
/// refused, and it is refused EARLY: the mate welds its two members
/// into one placement cluster, and the split's precondition accepts
/// only cuts that are unions of whole clusters, so `TornCluster`
/// fires before the remap is reached.
///
/// The remap arm behind it — `at` through the id door, the name
/// through the name door, either one the cut severed MISSING loudly —
/// is the second gate, unreachable past this precondition for a
/// welding mate, on the same argument the interface-crossing
/// collector's own unreachability note makes (`refactor.rs`). The
/// positive half of the arm is the row below: an accepted cut
/// RENUMBERS the remainder, and the operand follows.
#[test]
fn a8e_a_cut_that_would_sever_the_operand_refuses_at_the_precondition() {
    let s = scene("msolve1-a8e", &[], &[LIFT]);
    // The cut takes `top` and the transform over it, and would leave
    // the mate — whose `b` operand is that transform — behind.
    let cut = [s.top, s.b_at]
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>();
    let err = editor_core::split(
        &s.doc,
        &cut,
        DocumentId::derive("msolve1-a8e-part"),
        Tol::witness(),
    )
    .expect_err("the cut severs the mate's operand");
    assert!(
        matches!(err, editor_core::SplitError::TornCluster { .. }),
        "expected the whole-cluster precondition, got {err:?}"
    );
}

/// **A8(f).** An ACCEPTED cut carries a mate's operand through the
/// remap: the same id door the measure's `at` goes through, applied
/// to the half of a mate reference that is a node rather than a name.
/// Both halves come out pointing at the same nodes' images — the `b`
/// reference still read at the transform over its instance, the `a`
/// reference still read at its own mint.
#[test]
fn a8f_an_accepted_cut_carries_the_operand_through_the_remap() {
    let mut store = StubStore::default();
    let base_ref = store.insert(block("msolve1-a8f-base", 1.0), Tol::witness());
    let top_ref = store.insert(block("msolve1-a8f-top", 3.0), Tol::witness());
    let _ = &store;
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
            profile,
            distance: len(1.0),
        },
    );
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, xf) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(
                SitedRef::at_mint(in_part(base, CapEnd::End)),
                SitedRef::new(xf, in_part(top, CapEnd::Start)),
            ),
        },
    );
    let mate = mate.unwrap();
    // Cut the LOCAL block out into its own part: it touches no
    // cluster, so the precondition accepts.
    let cut = [profile, local, doc.order()[0]]
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>();
    let out = editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("msolve1-a8f-part"),
        Tol::witness(),
    )
    .expect("a cut of untouched local geometry is accepted");
    // The mate moved in the remainder's numbering; its operand moved
    // with it and still names the transform over its instance.
    let (moved_mate, a, b) = out
        .remainder
        .order()
        .iter()
        .find_map(|&id| match out.remainder.node(id) {
            Some(Node::Mate { a, b, .. }) => Some((id, a.clone(), b.clone())),
            _ => None,
        })
        .expect("the mate stayed in the remainder");
    let _ = mate;
    assert_ne!(
        b.at, b.name.node,
        "the `b` reference is still read somewhere other than its mint"
    );
    let Some(Node::Transform { input, .. }) = out.remainder.node(b.at) else {
        panic!("the `b` operand still names a transform");
    };
    assert_eq!(
        *input, b.name.node,
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
    assert!(
        solve_document(&out.remainder, Tol::witness())
            .fault(moved_mate)
            .is_none(),
        "and the remainder still solves"
    );
}

/// **A8(g) — a KEPT mate whose operand is inside the cut refuses.**
/// The mate welds nothing (its `b` reference names local geometry, so
/// it resolves to no member and `TornCluster` has nothing to say), and
/// its operand is a transform the cut takes. Before the reading edge
/// had a closure rule of its own the split ACCEPTED this: the remap
/// runs over cut nodes only, so the remainder kept an operand naming a
/// node it no longer had, invisible to the load door, and the solve
/// reported a dangling reference some time later.
#[test]
fn a8g_a_kept_mate_whose_operand_is_cut_refuses_at_the_door() {
    let (doc, mate, cut, xf) = severed_operand_scene("msolve1-a8g", false);
    let err = editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("msolve1-a8g-part"),
        Tol::witness(),
    )
    .expect_err("a kept mate cannot keep an operand the cut took");
    assert!(
        matches!(
            err,
            editor_core::SplitError::OperandSeveredFromMate {
                mate: m,
                side: MateSide::A,
                operand,
                mate_is_cut: false,
            } if m == mate && operand == xf
        ),
        "expected the operand-severed refusal naming the mate, the side \
         and the operand, got {err:?}"
    );
}

/// **A8(h) — and the other direction.** A CUT mate whose operand stays
/// in the remainder refuses with the same variant. It used to refuse
/// as `PartEdit { UnresolvedInput { input: at } }` — the INPUT's
/// vocabulary, for a node this whole design says is not an input.
#[test]
fn a8h_a_cut_mate_whose_operand_is_kept_refuses_with_the_same_variant() {
    let (doc, mate, cut, xf) = severed_operand_scene("msolve1-a8h", true);
    let err = editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("msolve1-a8h-part"),
        Tol::witness(),
    )
    .expect_err("a cut mate cannot carry an operand the part does not have");
    assert!(
        matches!(
            err,
            editor_core::SplitError::OperandSeveredFromMate {
                mate: m,
                side: MateSide::A,
                operand,
                mate_is_cut: true,
            } if m == mate && operand == xf
        ),
        "expected the operand-severed refusal, got {err:?}"
    );
}

/// A document whose mate WELDS NOTHING — its `b` reference names a
/// local extrude, which is no member — read at a transform over an
/// instance, with a cut that separates the two. `mate_in_cut` picks
/// which side of the cut the mate itself lands on.
///
/// Welding nothing is what makes the row about THIS rule: a mate that
/// welded a cluster would meet the whole-cluster precondition first.
fn severed_operand_scene(
    label: &str,
    mate_in_cut: bool,
) -> (
    ProfileDoc,
    RecipeNodeId,
    std::collections::BTreeSet<RecipeNodeId>,
    RecipeNodeId,
) {
    let mut store = StubStore::default();
    let top_ref = store.insert(block(&format!("{label}-top"), TOP_HEIGHT), Tol::witness());
    let _ = &store;
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, xf) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
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
            profile,
            distance: len(1.0),
        },
    );
    let local_face = StableName {
        kind: EntityKind::Face,
        node: local,
        path: vec![RoleSeg::Cap(CapEnd::Start)],
    };
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(
                SitedRef::new(xf, in_part(top, CapEnd::Start)),
                SitedRef::at_mint(local_face),
            ),
        },
    );
    let mate = mate.unwrap();
    let mut cut: std::collections::BTreeSet<RecipeNodeId> = [top, xf].into_iter().collect();
    if mate_in_cut {
        // The mate moves into the cut and its operand stays behind:
        // swap which side each is on.
        cut = [mate].into_iter().collect();
    }
    (doc, mate, cut, xf)
}

// ---- A11: a transform BETWEEN two patterns ----

/// **A11.** A transform between the two patterns: the walk's chain is
/// `[outer(j), T, inner(i)]` and the evaluator's body is
/// `M_o(j) ∘ T ∘ M_i(i)` — asserted on the product with a ROTATION,
/// so the other order is a different map and the row can tell them
/// apart.
#[test]
fn a11_a_transform_between_two_patterns_composes_outer_t_inner() {
    let mut store = StubStore::default();
    let base_ref = store.insert(
        slab("msolve1-a11-base", BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let top_ref = store.insert(block("msolve1-a11-top", TOP_HEIGHT), Tol::witness());
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a11"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let rule = |dir: [f64; 3], s: f64| PatternKind::Linear {
        direction: dir.map(scl),
        spacing: len(s),
    };
    let (doc, inner) = insert(
        doc,
        Node::Pattern {
            input: top,
            count: Expr::count(3),
            kind: rule([1.0, 0.0, 0.0], 5.0),
        },
    );
    let spin = std::f64::consts::FRAC_PI_2;
    let (doc, t) = insert(doc, xform(inner, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], spin));
    let (doc, outer) = insert(
        doc,
        Node::Pattern {
            input: t,
            count: Expr::count(2),
            kind: rule([0.0, 1.0, 0.0], 20.0),
        },
    );
    let a = in_part(base, CapEnd::End);
    let nested = in_copy(outer, 1, in_copy(inner, 1, in_part(top, CapEnd::Start)));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(
                SitedRef::at_mint(a.clone()),
                SitedRef::new(outer, nested.clone()),
            ),
        },
    );
    let mate = mate.unwrap();
    let poses = solve_document(&doc, Tol::witness());
    assert!(poses.fault(mate).is_none(), "{:?}", poses.fault(mate));
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
        &in_part(top_alone, CapEnd::Start),
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
    let mut store = StubStore::default();
    let base_ref = store.insert(
        slab(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let top_ref = store.insert(block(&format!("{label}-top"), TOP_HEIGHT), Tol::witness());
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let doc = ProfileDoc::empty(DocumentId::derive(&label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let rule = |dir: [f64; 3]| PatternKind::Linear {
        direction: dir.map(scl),
        spacing: len(5.0),
    };
    let (doc, inner) = insert(
        doc,
        Node::Pattern {
            input: top,
            count: Expr::count(3),
            kind: rule([1.0, 0.0, 0.0]),
        },
    );
    let (doc, outer) = insert(
        doc,
        Node::Pattern {
            input: inner,
            count: Expr::count(2),
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
            of,
            select: editor_core::PartSelect::Instance(Expr::count(k)),
        },
    );
    let a = in_part(base, CapEnd::End);
    let nested = in_copy(outer, j, in_copy(inner, i, in_part(top, CapEnd::Start)));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(
                SitedRef::at_mint(a.clone()),
                SitedRef::new(part, nested.clone()),
            ),
        },
    );
    let mate = mate.unwrap();
    let poses = solve_document(&doc, Tol::witness());
    let ev = run(&doc, &opts);
    let what = format!("Part({k}) naming ({j}, {i}), via transform: {via_transform}");
    match expect {
        PartCase::Seats => {
            assert!(
                poses.fault(mate).is_none(),
                "{what}: {:?}",
                poses.fault(mate)
            );
            assert_eq!(poses.role(mate), Some(MateRole::Determining), "{what}");
            assert!(
                ev.node_error(mate).is_none(),
                "{what}: {:?}",
                ev.node_error(mate)
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
            let fault = poses.fault(mate).cloned();
            assert!(
                matches!(
                    fault,
                    Some(MateFault::PartSelectsAnotherCopy {
                        part: p,
                        named: n,
                        selected,
                        ..
                    }) if p == part && n == named && selected == k
                ),
                "{what}: expected PartSelectsAnotherCopy(named {named}, selected {k}), got {fault:?}"
            );
            assert!(
                ev.node_error(mate).is_some(),
                "{what}: the mate node fails typed at the evaluation"
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

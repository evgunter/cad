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
    Alignment, AssemblyError, Attribution, AxisSense, CancelToken, CapEnd, ContactClass, Dimension,
    DocEdit, DocRef, DocumentId, EditError, EntityKind, EvalOptions, Evaluation, Expr, MateFault,
    MateFrame, MatePrimitive, MateRole, MateSide, Node, PartResolver, PatternKind, ProfileDoc,
    RecipeNodeId, ResolveFailure, ResolveFault, RoleSeg, SitedRef, StableName, assemble,
    content_pin, evaluate, load, product, product_named, save, solve_document,
};
use fixture::{insert, len, on_frame, scl, step};
use geom_core::Tol;
use geom_core::linalg::{Affine3, Mat3, Point3};

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

fn run(doc: &ProfileDoc, o: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), o, Tol::witness())
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

/// That same face as copy `i` of `pattern`.
fn in_copy(pattern: RecipeNodeId, i: u32, of: StableName) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: pattern,
        path: vec![RoleSeg::Instance {
            i,
            of: Box::new(of),
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

/// A `Transform` over `input`: translation, and `angle` about `axis`.
fn xform(
    input: RecipeNodeId,
    translation: [f64; 3],
    axis: [f64; 3],
    angle: f64,
) -> Node<editor_core::ProfileProgram> {
    Node::Transform {
        input,
        translation: translation.map(len),
        rotation_axis: axis.map(scl),
        rotation_angle: Expr::literal(angle, Dimension::Angle).unwrap(),
    }
}

// ---- the PRODUCT oracle ----

/// **A named planar face's own frame, read out of the body the
/// product GATHERS and its own name table**: a point on it, its
/// OUTWARD normal (the surface's chart normal times the face's
/// orientation sense — the direction material is not), and the
/// in-plane reference the surface carries.
///
/// This is what a consumer sees: the gathered body and its table, no
/// solved frame read by eye.
fn product_face_frame(doc: &ProfileDoc, ev: &Evaluation<f64>, name: &StableName) -> Affine3<f64> {
    let (body, names) = product_named(doc, ev, Tol::witness()).expect("the product gathers");
    let entry = names
        .lookup(name)
        .unwrap_or_else(|| panic!("{name:?} is absent from the product's table"));
    let editor_core::Entry::Unique(ent) = entry else {
        panic!("expected a unique entry, got {entry:?}")
    };
    let editor_core::EntityKey::Face(f) = ent.key else {
        panic!("expected a face, got {:?}", ent.key)
    };
    let face = body.get_face(f).expect("the face");
    match body.get_surface(face.surface).expect("the surface") {
        topo::Surface::Plane {
            origin,
            normal,
            u_ref,
        } => {
            let n = if face.sense { *normal } else { -*normal };
            // A right-handed frame on the face: u, n x u, n.
            Affine3::from_parts(
                Mat3::from_cols(*u_ref, n.cross(*u_ref), n),
                *origin - Point3::origin(),
            )
        }
        other => panic!("expected a plane, got {other:?}"),
    }
}

/// **The SEAT, as one rigid map**: the `b` face's product frame
/// expressed in the `a` face's — `F_a⁻¹ ∘ F_b`.
///
/// This is the whole of what `FrameCoincidence` pins, and it is a
/// CONSTANT of the alignment. Each mate frame sits at a fixed offset
/// from its own face's frame in part coordinates, so the relative map
/// between the two faces is that pair of offsets composed with the
/// coset the primitive admits — and no transform anywhere in either
/// chain can move it. A row that measures only the normal gap and the
/// normal dot leaves a spin about the seat normal and a slide along
/// it unmeasured; this leaves nothing.
fn seat_map(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    a: &StableName,
    b: &StableName,
) -> Affine3<f64> {
    product_face_frame(doc, ev, a).inverse() * product_face_frame(doc, ev, b)
}

/// The largest absolute difference between two rigid maps, over all
/// twelve numbers.
fn map_gap(x: &Affine3<f64>, y: &Affine3<f64>) -> f64 {
    let cols = |m: &Affine3<f64>| [m.linear.c0, m.linear.c1, m.linear.c2, m.translation];
    let (cx, cy) = (cols(x), cols(y));
    (0..4)
        .flat_map(|i| {
            let (u, v) = (cx[i], cy[i]);
            [(u.x - v.x).abs(), (u.y - v.y).abs(), (u.z - v.z).abs()]
        })
        .fold(0.0_f64, f64::max)
}

/// The seat, measured in the product and checked three ways: the two
/// named faces are COPLANAR, their outward normals are OPPOSED (the
/// top block stands ON the base, not through it), and the whole
/// relative frame is the one the CONTROL — the same document with no
/// transform and no pattern anywhere — puts them in.
///
/// The third check is the one a rotation or a lateral slide cannot
/// pass: it pins the spin about the seat normal and the in-plane
/// offset as well as the standoff.
fn assert_seated(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    a: &StableName,
    b: &StableName,
    control: &Affine3<f64>,
    what: &str,
) {
    let (fa, fb) = (
        product_face_frame(doc, ev, a),
        product_face_frame(doc, ev, b),
    );
    let (na, nb) = (fa.linear.c2, fb.linear.c2);
    let gap = (fb.translation - fa.translation).dot(na).abs();
    assert!(
        gap <= Tol::witness().eps(),
        "{what}: the mated faces are {gap} apart in the product"
    );
    assert!(
        (na.dot(nb) + 1.0).abs() <= 1e-9,
        "{what}: the outward normals are not opposed (dot {}) — the blocks \
         interpenetrate rather than seat",
        na.dot(nb)
    );
    let moved = map_gap(&(fa.inverse() * fb), control);
    assert!(
        moved <= 1e-9,
        "{what}: the seat's whole relative frame moved by {moved} from the \
         control's — a spin about the seat normal or a slide in it is a \
         transform the solve did not absorb"
    );
}

/// The at-rest gate's verdict, as the row wants to read it.
fn gate(doc: &ProfileDoc, ev: &Evaluation<f64>) -> Result<(), AssemblyError> {
    assemble(doc, ev, Tol::witness()).map(|_| ())
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
    // offset is T ∘ M(1) — the mate SOLVES and determines its pair.
    //
    // The document itself does not gather, and the reason is not the
    // mate's: `Node::Transform` takes ONE BODY, and a pattern's value
    // is `Instances`, so the transform refuses `WrongOperand` at the
    // evaluation. That is the node vocabulary's fence, unchanged by
    // this unit and unreachable past — so the row pins BOTH halves:
    // the reference resolves and places, and the document refuses
    // where a transform meets a multi-body value.
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
                    spacing: len(2.0),
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
        let _ = (&a, &b);
        let ev = run(&doc, &opts);
        let err = ev
            .node_error(xf)
            .expect("a transform over a pattern refuses at the evaluation");
        assert!(
            matches!(
                err.kind,
                editor_core::NodeErrorKind::WrongOperand { expected, .. } if expected == "body"
            ),
            "expected the one-body operand refusal, got {:?}",
            err.kind
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
            ),
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

// ---- A10: the vocabulary's fence ----

/// **A10.** A nested pattern's copy is still outside the vocabulary:
/// it refuses `DanglingHead`. `Member::copy` carries one level, and a
/// nested member's identity needs the whole chain — MSOLVE-2's
/// change to the pair keying and the spanning tree, stated at
/// `member_of`.
#[test]
fn a10_a_nested_pattern_head_still_refuses() {
    let mut store = StubStore::default();
    let base_ref = store.insert(block("msolve1-a10-base", 1.0), Tol::witness());
    let top_ref = store.insert(block("msolve1-a10-top", 3.0), Tol::witness());
    let _ = &store;
    let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a10"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let rule = || PatternKind::Linear {
        direction: [scl(1.0), scl(0.0), scl(0.0)],
        spacing: len(2.0),
    };
    let (doc, inner) = insert(
        doc,
        Node::Pattern {
            input: top,
            count: Expr::count(3),
            kind: rule(),
        },
    );
    let (doc, outer) = insert(
        doc,
        Node::Pattern {
            input: inner,
            count: Expr::count(2),
            kind: rule(),
        },
    );
    let nested = in_copy(outer, 1, in_copy(inner, 1, in_part(top, CapEnd::Start)));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(
                SitedRef::at_mint(in_part(base, CapEnd::End)),
                SitedRef::new(outer, nested),
            ),
        },
    );
    let mate = mate.unwrap();
    let fault = solve_document(&doc, Tol::witness())
        .fault(mate)
        .cloned()
        .expect("a nested-pattern head refuses");
    // The walk gets through the OUTER pattern — one copy level is in
    // the vocabulary — and stops at the inner one, which is where the
    // reference resolves to no member.
    assert!(
        matches!(fault, MateFault::DanglingHead { head, .. } if head == inner),
        "expected a dangling head at the inner pattern ({inner:?}), got {fault:?}"
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

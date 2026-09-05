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
    Alignment, AxisSense, CancelToken, CapEnd, ContactClass, Dimension, DocEdit, DocRef,
    DocumentId, EditError, EntityKind, EvalOptions, Evaluation, Expr, MateFault, MateFrame,
    MatePrimitive, MateRole, MateSide, Node, PartResolver, PatternKind, ProfileDoc, RecipeNodeId,
    ResolveFailure, ResolveFault, RoleSeg, SitedRef, StableName, content_pin, evaluate, load,
    product, product_named, save, solve_document,
};
use fixture::{insert, len, on_frame, scl, step};
use geom_core::Tol;
use geom_core::linalg::{Point3, Vec3};

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

fn mate_frame(origin: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis: [0.0, 0.0, 1.0],
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
            a: mate_frame([0.0, 0.0, 1.0]),
            b: mate_frame([0.0, 0.0, 0.0]),
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

/// A named planar face's plane, read out of the body the product
/// GATHERS and its own name table, as `(a point on it, its OUTWARD
/// normal)`. The outward normal is the surface's chart normal times
/// the face's orientation sense — the direction material is not — so
/// two faces that seat against each other have opposed ones.
///
/// This is what a consumer sees: the gathered body and its table, no
/// solved frame read by eye.
fn product_plane(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    name: &StableName,
) -> (Point3<f64>, Vec3<f64>) {
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
        topo::Surface::Plane { origin, normal, .. } => {
            (*origin, if face.sense { *normal } else { -*normal })
        }
        other => panic!("expected a plane, got {other:?}"),
    }
}

/// How the two named faces stand to each other in the product: the
/// distance of `b`'s plane from `a`'s along `a`'s normal, and the dot
/// of the two outward normals.
fn seats(doc: &ProfileDoc, ev: &Evaluation<f64>, a: &StableName, b: &StableName) -> (f64, f64) {
    let (pa, na) = product_plane(doc, ev, a);
    let (pb, nb) = product_plane(doc, ev, b);
    ((pb - pa).dot(na).abs(), na.dot(nb))
}

/// The seat, measured in the product: the two named faces are
/// COPLANAR to `Tol::witness()`, and their outward normals are
/// COLLINEAR — which is the axis agreement the `Opposed` sense
/// produces on these two caps.
///
/// Collinearity is the half a rotating transform breaks: an angle the
/// solve did not absorb tilts the placed cap, and a tilted plane is
/// neither coplanar with the other nor collinear to it. The SIGN is
/// `+1` on this alignment, and it is the control's sign: each cap's
/// outward normal is the direction its own material is not, and the
/// `Opposed` half-turn that points the two mate-frame AXES at each
/// other carries the mated cap's outward normal round with it.
fn assert_seated(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    a: &StableName,
    b: &StableName,
    what: &str,
) {
    let (gap, agreement) = seats(doc, ev, a, b);
    assert!(
        gap <= Tol::witness().eps(),
        "{what}: the mated faces are {gap} apart in the product"
    );
    assert!(
        (agreement - 1.0).abs() <= 1e-9,
        "{what}: the mated faces' normals are not collinear (dot {agreement}) —          a rotation the solve did not absorb tilts the placed cap"
    );
}

// ---- the scene ----

/// `base` (a 1-tall block) and `top` (3-tall), each optionally wrapped
/// in a chain of transforms, with a `Rest`/`FrameCoincidence`/`Opposed`
/// mate seating `top`'s bottom cap on `base`'s top cap, each side
/// authored at the LAST node of its own chain.
///
/// Different heights on purpose: with equal cubes the `Opposed` flip
/// maps the top block back onto the base's own z-range, and an
/// identity solve is indistinguishable from a correct one.
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

/// Builds that scene. `on_base` / `on_top` are the transform chains,
/// innermost first, as `(translation, axis, angle)`.
fn scene(
    label: &str,
    on_base: &[([f64; 3], [f64; 3], f64)],
    on_top: &[([f64; 3], [f64; 3], f64)],
) -> Scene {
    let mut store = StubStore::default();
    let base_ref = store.insert(block(&format!("{label}-base"), 1.0), Tol::witness());
    let top_ref = store.insert(block(&format!("{label}-top"), 3.0), Tol::witness());
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let mut doc = doc;
    let mut a_at = base;
    for &(t, ax, ang) in on_base {
        let (d, id) = insert(doc, xform(a_at, t, ax, ang));
        doc = d;
        a_at = id;
    }
    let mut b_at = top;
    for &(t, ax, ang) in on_top {
        let (d, id) = insert(doc, xform(b_at, t, ax, ang));
        doc = d;
        b_at = id;
    }
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

const LIFT: ([f64; 3], [f64; 3], f64) = ([0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0);

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
    /// Every node evaluated, no mate fault, and the mated faces meet
    /// in the product.
    fn assert_green_and_seated(&self, what: &str) {
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
        assert_seated(&self.doc, &ev, &self.face_a(), &self.face_b(), what);
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
    let control = scene("msolve1-a1-control", &[], &[]);
    control.assert_green_and_seated("A1 control");

    let test = scene("msolve1-a1-lifted", &[], &[LIFT]);
    test.assert_green_and_seated("A1 lifted");

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

/// **A2.** A transform with a non-zero angle about z, and one about a
/// non-axis direction: the mated faces coincide in the product within
/// `Tol::witness()` and the `Opposed` axis agreement holds. A
/// rotating transform mis-ORIENTS as well as displaces, so this is
/// the row a translation-only fix would not pass.
#[test]
fn a2_a_rotated_instance_seats_and_keeps_the_axis_agreement() {
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
    ] {
        let s = scene(
            &format!("msolve1-a2-{what}"),
            &[],
            &[([0.0, 0.0, 10.0], axis, angle)],
        );
        s.assert_green_and_seated(&format!("A2 {what}"));
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
    // (a) PATTERN over TRANSFORM over the instance. The mate is read
    // at the pattern; the offset is M(1) ∘ T.
    {
        let mut store = StubStore::default();
        let base_ref = store.insert(block("msolve1-a3a-base", 1.0), Tol::witness());
        let top_ref = store.insert(block("msolve1-a3a-top", 3.0), Tol::witness());
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
                kind: PatternKind::Linear {
                    direction: [scl(1.0), scl(0.0), scl(0.0)],
                    spacing: len(2.0),
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
        assert_seated(&doc, &ev, &a, &b, "A3 pattern-of-transform");
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
        let base_ref = store.insert(block("msolve1-a3b-base", 1.0), Tol::witness());
        let top_ref = store.insert(block("msolve1-a3b-top", 3.0), Tol::witness());
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

/// **A4.** A transform over the GAUGE side, over both sides, and a
/// chain of two transforms on one side: all seat. The gauge is the
/// document-order-first instance, so a transform over `base` is the
/// case where the solve must un-wind the map on the side it is
/// measuring FROM.
#[test]
fn a4_the_offset_holds_on_either_side_and_through_a_chain() {
    let lift = |z: f64| ([0.0, 0.0, z], [0.0, 0.0, 1.0], 0.0);
    let spin = |a: f64| ([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], a);
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
    ] {
        let s = scene(&format!("msolve1-a4-{what}"), &on_base, &on_top);
        s.assert_green_and_seated(&format!("A4 {what}"));
    }
}

// ---- A5: two operands, one instance ----

/// Two mates from `base` to `top`, each read at a DIFFERENT transform
/// over `top`. Returns the document, the two mate ids, and the two
/// transforms.
fn two_operands(label: &str, second_lift: f64) -> (ProfileDoc, EvalOptions, [RecipeNodeId; 2]) {
    let mut store = StubStore::default();
    let base_ref = store.insert(block(&format!("{label}-base"), 1.0), Tol::witness());
    let top_ref = store.insert(block(&format!("{label}-top"), 3.0), Tol::witness());
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, x1) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, x2) = insert(
        doc,
        xform(x1, [0.0, 0.0, second_lift], [0.0, 0.0, 1.0], 0.0),
    );
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

/// **A5.** Two mates from one instance through two different
/// transforms are two MEMBERS over one instance: they key as
/// different pairs, so the second is a loop-closing DECLARING edge
/// rather than a fold-mate of the first. A geometrically consistent
/// pair solves; an inconsistent one refuses typed.
#[test]
fn a5_two_operands_over_one_instance_are_two_members() {
    // Consistent: the second transform is the identity translation, so
    // both mates ask for the same seat.
    let (doc, opts, [m1, m2]) = two_operands("msolve1-a5-consistent", 0.0);
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

    // Inconsistent: the two operands are 3 apart, so the two mates
    // cannot both be satisfied. The declaring mate is verified
    // against the solved geometry at the assembly gate, which is
    // where an inconsistent loop dies (A11 rule 4).
    let (doc, opts, [m1, m2]) = two_operands("msolve1-a5-inconsistent", 3.0);
    let poses = solve_document(&doc, Tol::witness());
    assert!(
        poses.fault(m1).is_none() && poses.fault(m2).is_none(),
        "A5 inconsistent: the SOLVE places on the tree edge and does \
         not verify the loop; the gate does"
    );
    let ev = run(&doc, &opts);
    let refusal = editor_core::assemble(&doc, &ev, Tol::witness());
    assert!(
        refusal.is_err(),
        "A5 inconsistent: the gate refuses the unmet declaration, got {:?}",
        refusal.map(|_| "assembled")
    );
    let err = refusal.unwrap_err();
    assert!(
        matches!(err, editor_core::AssemblyError::AtRest { .. }),
        "A5 inconsistent: expected the at-rest gate's refusal, got {err:?}"
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
/// bit what it solved before the operand existed. The numbers below
/// are the pre-fix solve of exactly this document, recorded off the
/// characterization run this unit deleted: an `Opposed` half-turn
/// (the `-1` diagonal) and a +1 lift seating the cap.
///
/// The pin is on BITS, not on a value: the offset is composed only
/// when a chain has a placer in it, and `None` is kept as absence
/// precisely so this document composes nothing.
#[test]
fn a7_a_document_with_no_placer_solves_bit_for_bit() {
    let s = scene("msolve1-a7", &[], &[]);
    let poses = solve_document(&s.doc, Tol::witness());
    let f = poses.relative(s.top).expect("the mated instance solves");
    assert_eq!(
        f.columns,
        [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]],
        "the Opposed half-turn, unchanged"
    );
    assert_eq!(
        f.translation.map(f64::to_bits),
        [0.0_f64.to_bits(), 0.0_f64.to_bits(), 1.0_f64.to_bits()],
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
    assert!(
        matches!(
            fault,
            MateFault::DanglingHead { mate, side, head }
                if *mate == s.mate && *side == MateSide::B && *head == s.top
        ),
        "expected a dangling head naming side B, got {fault:?}"
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
    assert_seated(&back, &ev, &s.face_a(), &s.face_b(), "A8(d)");
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
    assert!(
        matches!(fault, MateFault::DanglingHead { head, .. } if head == outer),
        "expected a dangling head at the outer pattern, got {fault:?}"
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

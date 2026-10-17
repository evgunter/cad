//! **A mate reads the body its target names** — the transform-aware
//! seat.
//!
//! A mate's `to` pose is read off a face of the space the copy joins:
//! a FACE name, selected in the body it is read at (the read's
//! operand), which may be a transform, a pattern or a `Part` over the
//! instance that minted the name. The face is read where that body
//! stands, so a mate on a TRANSFORMED target seats the copy on the
//! transformed geometry and a mate on the instance seats it on the
//! instance. The `on` pose reads the copy's own geometry, never a body
//! downstream of it.
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
    CapEnd, ContactClass, DocEdit, DocRef, DocumentId, EditError, EntityKind, EvalOptions,
    Evaluation, MateFault, MateRef, MateSide, NameRef, Node, Operand, PatternKind, ProfileDoc,
    RecipeNodeId, RoleSeg, SitedFace, Space, StableName, load, product, save,
};
use fixture::resolver::{PartStore, in_part, placement_of, with_resolver};
use fixture::seat::map_gap;
use fixture::{
    constrain_at, gate, head, head_at, in_copy, insert, insert_pinned, len, on_frame, plane, rest,
    run, scl, solve, step, step_with, xform,
};
use geom_core::Tol;
use geom_core::linalg::Affine3;

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
    slab(label, 1.0, h)
}

/// **A stored part**: its reference, its body, and the part's own
/// names for two adjacent walls of its square profile (segments 0 and
/// 1), as its placement carries them.
struct Part {
    doc_ref: DocRef,
    body: RecipeNodeId,
    walls: [RoleSeg; 2],
}

fn stored(store: &mut PartStore, (doc, body): (ProfileDoc, RecipeNodeId)) -> Part {
    let walls = [fixture::wall(&doc, body, 0), fixture::wall(&doc, body, 1)];
    let (doc_ref, body) = store.insert_part((doc, body), Tol::witness());
    Part {
        doc_ref,
        body,
        walls,
    }
}

impl Part {
    /// The cap `end` of this part as `instance` places it.
    fn cap(&self, instance: RecipeNodeId, end: CapEnd) -> StableName {
        in_part(instance, self.body, end)
    }

    /// Wall `k` of this part as `instance` places it — the part's own
    /// name under its placement's copy, worn inside the instance's
    /// `InPart`.
    fn wall(&self, instance: RecipeNodeId, k: usize) -> StableName {
        let placed = StableName {
            kind: EntityKind::Face,
            node: placement_of(self.body),
            path: vec![RoleSeg::Placed {
                of: NameRef::new(StableName {
                    kind: EntityKind::Face,
                    node: self.body,
                    path: vec![self.walls[k].clone()],
                }),
            }],
        };
        fixture::face(placed).in_part(instance).into_name()
    }
}

/// **`face`'s plane slid `by` along its own normal**: a standoff, which
/// places and declares no contact.
fn standoff(face: impl Into<Operand>, by: f64) -> Operand {
    let p = plane(face);
    Operand::Pose(Box::new(editor_core::pose::PoseDef::Slide {
        pose: editor_core::pose::RoleRead::plain(p.clone()),
        along: editor_core::pose::RoleRead::through(p, editor_core::pose::PoseRole::Normal),
        by: len(by),
    }))
}

/// **The seat**, added to `top`'s bundle: the top's bottom cap resting
/// on the base's top cap, and the top's walls 0 and 1 each standing
/// one unit inside the base's own (a standoff of −1 along the base
/// wall's outward normal), so the `1x1` block stands on the `3x3`
/// slab's middle square, `[1, 2]²`. `base_face` reads a part name of
/// the base where the row reads it — at the instance, through a
/// transform, a pattern or a `Part`. Only the rest declares a contact.
/// Answers the rest.
fn seat(
    doc: ProfileDoc,
    top: RecipeNodeId,
    top_part: &Part,
    base_face: &dyn Fn(StableName) -> SitedFace,
    base_part: &Part,
    base: RecipeNodeId,
) -> (ProfileDoc, MateRef) {
    let (doc, m) = constrain_at(
        doc,
        top,
        rest(
            head(top_part.cap(top, CapEnd::Start)),
            base_face(base_part.cap(base, CapEnd::End)),
            ContactClass::Rest,
        ),
    );
    let doc = (0..2).fold(doc, |doc, k| {
        constrain_at(
            doc,
            top,
            fixture::mate(
                plane(head(top_part.wall(top, k))),
                standoff(base_face(base_part.wall(base, k)), -1.0),
                ContactClass::Rest,
            ),
        )
        .0
    });
    (doc, m)
}

// ---- the scene ----

/// The base's height and width: a `3x3x1` slab, wide enough that the
/// `1x1x3` block seated on it stands clear of its edges — a
/// same-footprint pair would rest face-on-face with every edge
/// coincident, where the census has more to say than the mate does.
const BASE_HEIGHT: f64 = 1.0;
const BASE_WIDTH: f64 = 3.0;
const TOP_HEIGHT: f64 = 3.0;

/// `base` (a wide slab, pinned in the world) optionally wrapped in a
/// chain of transforms, and `top` (a tall block) seated on the LAST
/// node of the base's chain.
struct Scene {
    doc: ProfileDoc,
    opts: EvalOptions,
    base: RecipeNodeId,
    top: RecipeNodeId,
    base_part: Part,
    top_part: Part,
    /// The last node of `base`'s chain, where the seat reads the base.
    a_at: RecipeNodeId,
    /// The seat's rest, a constraint of `top`'s bundle.
    mate: MateRef,
}

/// **The world the product is** (A10): each of `bodies` that is not
/// already a placement placed, then each `(pattern, count)`'s copies
/// placed through a `Part` per copy. An instance is a placement already
/// in the product, so it is never placed a second time.
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
                    select: editor_core::PartSelect::Instance(editor_core::Formula::count(i)),
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
fn copy_name(doc: &ProfileDoc, ev: &Evaluation<f64>, name: &StableName) -> StableName {
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
    ev: &Evaluation<f64>,
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
    ev: &Evaluation<f64>,
    a: &StableName,
    b: &StableName,
) -> Affine3<f64> {
    fixture::seat::seat_map(doc, ev, &copy_name(doc, ev, a), &copy_name(doc, ev, b))
}

/// [`fixture::seat::product_face_frame`] of `name`'s world copy.
fn product_face_frame(doc: &ProfileDoc, ev: &Evaluation<f64>, name: &StableName) -> Affine3<f64> {
    fixture::seat::product_face_frame(doc, ev, &copy_name(doc, ev, name))
}

/// One link of a transform chain: `(translation, rotation axis,
/// angle)`.
type Step = ([f64; 3], [f64; 3], f64);

/// The two parts every scene stores, and the options reaching them.
fn parts(label: &str) -> (Part, Part, EvalOptions) {
    let mut store = PartStore::default();
    let base = stored(
        &mut store,
        slab(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
    );
    let top = stored(&mut store, block(&format!("{label}-top"), TOP_HEIGHT));
    (base, top, with_resolver(store))
}

/// Builds that scene. `on_base` is the base's transform chain,
/// innermost first.
fn scene(label: &str, on_base: &[Step]) -> Scene {
    let (base_part, top_part, opts) = parts(label);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert_pinned(doc, Node::instantiate_part(base_part.doc_ref), &opts);
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, a_at) = chain(doc, base, on_base);
    let doc = world(doc, &[a_at], &[]);
    let (doc, mate) = seat(doc, top, &top_part, &|n| head_at(a_at, n), &base_part, base);
    Scene {
        doc,
        opts,
        base,
        top,
        base_part,
        top_part,
        a_at,
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

/// **The seat this fixture's mates ask for**, measured on the document
/// that has no transform and no pattern anywhere: the relative frame
/// every other row must reproduce.
///
/// It is derived, not written down — a constant transcribed here
/// would be a second statement of what the mates say, and the two
/// could drift apart.
fn control_seat(label: &str) -> Affine3<f64> {
    let s = scene(&format!("{label}-control"), &[]);
    let ev = s.eval();
    seat_map(&s.doc, &ev, &s.face_a(), &s.face_b())
}

impl Scene {
    fn eval(&self) -> Evaluation<f64> {
        run(&self.doc, &self.opts)
    }
    fn face_a(&self) -> StableName {
        self.base_part.cap(self.base, CapEnd::End)
    }
    fn face_b(&self) -> StableName {
        self.top_part.cap(self.top, CapEnd::Start)
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
        for (name, id) in [("base", self.base), ("top", self.top), ("a_at", self.a_at)] {
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

// ---- A1: the finding's own document, on the target ----

/// **A1.** The two-block document with a +z translation over the
/// target instance (angle 0): the solve records no fault, the product
/// gathers, the mated faces MEET in the product, and the copy's solved
/// pose differs from the control's by exactly the transform's map.
///
/// *Breaks if* the seat reads the base where its instance stands
/// rather than where the transform puts it (the top stays at `z = 1`),
/// or a translation turns the top.
#[test]
fn a1_a_translated_target_seats_in_the_product() {
    let seat = control_seat("msolve1-a1");
    let control = scene("msolve1-a1-control", &[]);
    control.assert_green_and_seated(&seat, "A1 control");

    let test = scene("msolve1-a1-lifted", &[LIFT]);
    test.assert_green_and_seated(&seat, "A1 lifted");

    // The mechanism, stated as arithmetic rather than as a string
    // comparison: the top rose by exactly the transform's lift, so it
    // stands on the lifted base where it stood on the plain one.
    let c = solve(&control.doc, &control.opts, Tol::witness())
        .relative(control.top)
        .expect("the control solves");
    let t = solve(&test.doc, &test.opts, Tol::witness())
        .relative(test.top)
        .expect("the test solves");
    assert!(
        (t.translation[2] - c.translation[2] - 10.0).abs() <= 1e-12,
        "the seat followed exactly the transform's +10 lift \
         (control {:?} vs test {:?})",
        c.translation,
        t.translation
    );
    // ...and the rotation part is untouched by a translation-only
    // transform, bit for bit.
    assert_eq!(c.columns, t.columns, "a translation moves no axis");
}

// ---- A2: rotation ----

/// **A2.** A transform over the target with a non-zero angle about z,
/// and ones about non-axis directions: the mated faces still seat in
/// the product, their outward normals opposed and their whole relative
/// frame the control's — and the at-rest gate still passes.
///
/// The about-z rows are the case a normal-and-gap oracle cannot reach:
/// a spin about the seat normal leaves both faces coplanar and both
/// normals opposed while turning the block. The relative frame is what
/// sees it.
///
/// *Breaks if* any of the target's turn is lost from the faces the
/// seat reads.
#[test]
fn a2_a_rotated_target_seats_and_keeps_the_whole_frame() {
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
            &[([0.0, 0.0, 10.0], axis, angle)],
        );
        s.assert_green_and_seated(&seat, &format!("A2 {what}"));
    }
}

// ---- A3: patterns over transforms, and transforms over patterns ----

/// The copy of the base's pattern the seat reads.
const COPY: u32 = 1;

/// **A3.** Pattern-of-transform and transform-of-pattern of the target:
/// both solve, and the top seats on the named copy in the product.
///
/// *Breaks if* a pattern copy's face is read at the master's pose, or
/// the transform and the pattern compose in the wrong order.
#[test]
fn a3_pattern_of_transform_seats_and_transform_of_pattern_resolves() {
    // The seat is a constant of the mates and the two part documents,
    // so the plain control's relative frame is the one a pattern copy
    // must give too.
    let seat_frame = control_seat("msolve1-a3");
    // (a) PATTERN over TRANSFORM over the base. The seat is read at the
    // pattern; the copy stands at M(1) ∘ T.
    {
        let (base_part, top_part, opts) = parts("msolve1-a3a");
        let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a3a"), Tol::witness());
        let (doc, base) = insert_pinned(doc, Node::instantiate_part(base_part.doc_ref), &opts);
        let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
        let (doc, xf) = insert(doc, xform(base, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
        let (doc, pattern) = insert(
            doc,
            Node::Pattern {
                input: xf.into(),
                count: editor_core::Formula::count(3),
                // Spaced wide enough that copies 0 and 2 stand clear of
                // the top: the row is about the copy the seat names, and
                // a sibling touching the top uninvited is an UNRECORDED
                // contact the gate is right to refuse.
                kind: PatternKind::Linear {
                    direction: [scl(1.0), scl(0.0), scl(0.0)],
                    spacing: len(5.0),
                },
            },
        );
        let doc = world(doc, &[], &[(pattern, 3)]);
        let a = in_copy(pattern, COPY, base_part.cap(base, CapEnd::End));
        let b = top_part.cap(top, CapEnd::Start);
        let (doc, _) = seat(
            doc,
            top,
            &top_part,
            &|n| head(in_copy(pattern, COPY, n)),
            &base_part,
            base,
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

    // (b) TRANSFORM over PATTERN over the base. The seat names a copy
    // and is read at the transform: the copy stands at T ∘ M(1), the
    // seat pins the top, and the document GATHERS — a transform is
    // shape-preserving over its input's value, so the transform of the
    // pattern is the pattern's copies under one map.
    {
        let (base_part, top_part, opts) = parts("msolve1-a3b");
        let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a3b"), Tol::witness());
        let (doc, base) = insert_pinned(doc, Node::instantiate_part(base_part.doc_ref), &opts);
        let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
        let (doc, pattern) = insert(
            doc,
            Node::Pattern {
                input: base.into(),
                count: editor_core::Formula::count(3),
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
        let doc = world(doc, &[], &[(xf, 3)]);
        let a = in_copy(pattern, COPY, base_part.cap(base, CapEnd::End));
        let b = top_part.cap(top, CapEnd::Start);
        let (doc, _) = seat(
            doc,
            top,
            &top_part,
            &|n| head_at(xf, in_copy(pattern, COPY, n)),
            &base_part,
            base,
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
            "the transform-of-pattern read places the top"
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

// ---- A4: chains ----

/// **A4.** Chains of one and two transforms over the target, and —
/// the row the rest do not reach — chains whose two maps DO NOT
/// COMMUTE. A lift along z and a spin about z commute, so every such
/// chain composes to the same map whichever way round it is read, and
/// a composition written backwards would pass. A rotation about x and
/// a translation along z do not.
///
/// *Breaks if* the chain's maps compose in the wrong order, or one
/// link is dropped.
#[test]
fn a4_the_seat_holds_through_a_chain_on_the_target() {
    let seat = control_seat("msolve1-a4");
    let lift = |z: f64| ([0.0, 0.0, z], [0.0, 0.0, 1.0], 0.0);
    let spin = |a: f64| ([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], a);
    let tip = |a: f64| ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], a);
    for (what, on_base) in [
        ("one lift", vec![lift(4.0)]),
        (
            "a chain of two, lift first",
            vec![lift(10.0), spin(std::f64::consts::FRAC_PI_3)],
        ),
        (
            "a chain of two, rotation first",
            vec![spin(std::f64::consts::FRAC_PI_3), lift(10.0)],
        ),
        (
            "a spin and a short lift",
            vec![spin(std::f64::consts::FRAC_PI_6), lift(2.0)],
        ),
        // NON-COMMUTING: `tip` then `lift` is not `lift` then `tip` —
        // the x-rotation turns the z-translation.
        (
            "a non-commuting chain, tip then lift",
            vec![tip(std::f64::consts::FRAC_PI_3), lift(10.0)],
        ),
        (
            "a non-commuting chain, lift then tip",
            vec![lift(10.0), tip(std::f64::consts::FRAC_PI_3)],
        ),
        (
            "a non-commuting chain, tipped back",
            vec![tip(-std::f64::consts::FRAC_PI_4), lift(5.0)],
        ),
    ] {
        let s = scene(&format!("msolve1-a4-{what}"), &on_base);
        s.assert_green_and_seated(&seat, &format!("A4 {what}"));
    }
}

/// **A4′ — the two orders really are different maps.** The row above
/// asserts both non-commuting chains seat; this one asserts they are
/// not the same document dressed twice, by measuring that the two
/// place the block somewhere different. Without it "both seat" could
/// be true of a pair the composition happened to see as one.
///
/// *Breaks if* the two chain orders place the top in the same place.
#[test]
fn a4_the_two_non_commuting_orders_place_different_geometry() {
    let tip = std::f64::consts::FRAC_PI_3;
    let one = scene(
        "msolve1-a4-order-1",
        &[([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], tip), LIFT],
    );
    let two = scene(
        "msolve1-a4-order-2",
        &[LIFT, ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], tip)],
    );
    // Measured on where the top was placed, not on the seat: the seat
    // is the same by construction — that is the point of the row above
    // — so the two chains being different maps has to be read off the
    // copy's pose.
    let placed = |s: &Scene| {
        solve(&s.doc, &s.opts, Tol::witness())
            .relative(s.top)
            .expect("the mated instance solves")
    };
    let (p1, p2) = (placed(&one), placed(&two));
    let moved = map_gap(&p1.affine::<f64>(), &p2.affine::<f64>());
    assert!(
        moved > 1.0,
        "the two chain orders compose to the same map ({p1:?} vs {p2:?}) — \
         the row above would then prove nothing about composition order"
    );
}

/// **A4″ — the target pinned to the world at a non-identity pose**,
/// with a transform over it: the face the seat reads stands at the
/// transform over the pinned pose, so a target that carries an
/// authored pose is the case where that composition has to be right.
///
/// *Breaks if* the target's pin is dropped from the face the seat
/// reads, or composed on the wrong side of the transform.
#[test]
fn a4_a_placed_target_seats_through_its_chain() {
    let seat = control_seat("msolve1-a4-placed");
    let s = scene(
        "msolve1-a4-placed-frame",
        &[([1.0, 2.0, 3.0], [1.0, 1.0, 0.0], -0.6)],
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
    let at = editor_core::Frame::rotate_then_translate(
        [0.3, -0.2, 0.9],
        0.8,
        [2.0, -1.0, 3.0],
        fixture::band(),
    )
    .expect("a literal axis has a definite direction");
    let doc = fixture::pin(doc, s.base, &at, &s.opts);
    let placed = Scene { doc, ..s };
    placed.assert_green_and_seated(&seat, "A4 placed target");
}

// ---- A5: two seats through two operands ----

/// One seat of `top` on `base` read at a transform `x1` of the base,
/// and a second rest of the top read at a DIFFERENT transform `x2` of
/// the base, each transform placed so both copies reach the product.
/// Answers the document with the seat in `top`'s bundle, the seat's
/// rest, the second rest (not added), and `top`.
///
/// `x1` lifts the base by 10; `x2` moves it `(5, 5)` across, clear of
/// `x1`, plus `extra_lift`: with no extra lift the two top caps are
/// one plane, and with one they are parallel planes apart.
fn two_operands(
    label: &str,
    extra_lift: f64,
) -> (
    ProfileDoc,
    EvalOptions,
    MateRef,
    editor_core::Mate<editor_core::Formula>,
    RecipeNodeId,
) {
    let (base_part, top_part, opts) = parts(label);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert_pinned(doc, Node::instantiate_part(base_part.doc_ref), &opts);
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, x1) = insert(doc, xform(base, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, x2) = insert(
        doc,
        xform(base, [5.0, 5.0, 10.0 + extra_lift], [0.0, 0.0, 1.0], 0.0),
    );
    let doc = world(doc, &[x1, x2], &[]);
    let (doc, m1) = seat(doc, top, &top_part, &|n| head_at(x1, n), &base_part, base);
    let second = rest(
        head(top_part.cap(top, CapEnd::Start)),
        head_at(x2, base_part.cap(base, CapEnd::End)),
        ContactClass::Rest,
    );
    (doc, opts, m1, second, top)
}

/// **A5.** A seat read through one transform of the target pins the
/// top, and a second rest through another transform of the same
/// target is a mate beyond the pin. The door decides structure alone
/// and admits it; the placement refuses where it evaluates — as
/// overconstrained where the second rest agrees with the seat (it and
/// the seat's own rest are each fixed by the others), and as a
/// contradiction where it does not (parallel planes apart). Before the
/// second rest, the seat alone solves and the product gathers.
///
/// *Breaks if* a mate beyond the pin is folded silently, or an
/// inconsistent one is taken for a redundant one.
#[test]
fn a5_a_second_seat_through_another_operand_is_refused() {
    for (what, label, extra_lift) in [
        ("consistent", "msolve1-a5-consistent", 0.0),
        ("inconsistent", "msolve1-a5-inconsistent", 3.0),
    ] {
        let (doc, opts, held, second, top) = two_operands(label, extra_lift);
        let poses = solve(&doc, &opts, Tol::witness());
        assert!(
            poses.fault(top).is_none(),
            "A5 {what}: the first seat solves: {:?}",
            poses.fault(top)
        );
        let ev = run(&doc, &opts);
        assert!(
            product(&doc, &ev, Tol::witness()).is_ok(),
            "A5 {what}: the product gathers"
        );
        let (doc, added) = constrain_at(doc, top, second);
        let refused = solve(&doc, &opts, Tol::witness())
            .fault(top)
            .cloned()
            .unwrap_or_else(|| panic!("A5 {what}: a second rest beyond the pin was folded"));
        if extra_lift == 0.0 {
            assert!(
                matches!(
                    &refused,
                    MateFault::Overconstrained { placement, constraints, .. }
                        if *placement == top && *constraints == vec![held.id, added.id]
                ),
                "A5 {what}: the second rest is beyond the pin: {refused:?}"
            );
        } else {
            assert!(
                matches!(
                    &refused,
                    MateFault::Contradictory { added: a, .. } if *a == added
                ),
                "A5 {what}: the second rest contradicts the seat: {refused:?}"
            );
        }
    }
}

// ---- A6: a residual leaves the copy loose ----

/// **A6.** A bundle that leaves a residual pins less than the copy
/// needs: a lone rest leaves the top free to slide and spin on the
/// base, so the copy is LOOSE, its own space's root, with no refusal —
/// with and without a transform over the target, identically.
///
/// *Breaks if* a residual is refused, or the copy is taken as pinned.
#[test]
fn a6_a_residual_bundle_leaves_the_copy_loose_with_or_without_the_transform() {
    let residual = |label: &str, lift: bool| -> (Option<Space>, Option<MateFault>, RecipeNodeId) {
        let (base_part, top_part, opts) = parts(label);
        let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
        let (doc, base) = insert_pinned(doc, Node::instantiate_part(base_part.doc_ref), &opts);
        let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
        let (doc, at) = if lift {
            insert(doc, xform(base, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0))
        } else {
            (doc, base)
        };
        let (doc, _) = constrain_at(
            doc,
            top,
            rest(
                head(top_part.cap(top, CapEnd::Start)),
                head_at(at, base_part.cap(base, CapEnd::End)),
                ContactClass::Rest,
            ),
        );
        let poses = solve(&doc, &opts, Tol::witness());
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

// ---- A8: the doors ----

/// **A8(a).** A constraint whose operand never existed refuses typed
/// at the edit door — the name half's rule applied to the node half.
///
/// *Breaks if* a read at a node the document never held is admitted.
#[test]
fn a8a_an_operand_that_never_existed_refuses_at_the_insert_door() {
    let s = scene("msolve1-a8a", &[]);
    let ghost = RecipeNodeId::new(0, 9_999);
    let err = s
        .doc
        .apply(
            &DocEdit::AddConstraint {
                placement: s.top,
                mate: Box::new(rest(
                    head(s.face_b()),
                    head_at(ghost, s.face_a()),
                    ContactClass::Rest,
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

/// **A8(b).** Deleting the transform a seat reads the target at
/// strands its `to` pose: D10's unresolved reader, which the placement
/// refuses where it evaluates, naming the mate and the pose, until the
/// read is re-authored.
///
/// *Breaks if* the stranded seat is folded anyway, or its refusal is
/// laid on the `on` pose.
#[test]
fn a8b_deleting_the_operand_leaves_the_pose_unevaluated() {
    let s = scene("msolve1-a8b", &[LIFT]);
    let reach = editor_core::mate_reach::<f64>(&s.opts, Tol::witness());
    let (doc, _) = step_with(s.doc, DocEdit::DeleteNode { id: s.a_at }, &reach);
    let poses = solve(&doc, &s.opts, Tol::witness());
    let fault = poses
        .fault(s.mate.placement)
        .expect("the stranded mate refuses");
    assert!(
        matches!(
            fault,
            MateFault::SideUnevaluated { mate, side: MateSide::To, .. } if *mate == s.mate
        ),
        "expected the stranded pose, got {fault:?}"
    );
    let ev = run(&doc, &s.opts);
    assert!(
        matches!(
            ev.node_error(s.mate.placement).map(|e| &e.kind),
            Some(editor_core::NodeErrorKind::Mate(f))
                if matches!(**f, MateFault::SideUnevaluated { side: MateSide::To, .. })
        ),
        "{:?}",
        ev.node_error(s.mate.placement)
    );
    // The stranded pose lands on no member; the live one still does,
    // which is what makes the refusal a per-pose one.
    let edges: Vec<RecipeNodeId> = crate::fixture::mate_members(&doc)
        .into_iter()
        .filter(|&(m, _)| m == s.mate)
        .map(|(_, to)| to)
        .collect();
    assert_eq!(
        edges,
        vec![s.top],
        "only the resolving pose lands on a member"
    );
}

/// **A8(c).** Two placements whose constraints differ only in the
/// operand their target is read at are different content keys — the
/// memo cannot serve one's answer for the other. Measured on the
/// evaluated values, which is where a key is read.
///
/// *Breaks if* the operand a face is read at drops out of the key.
#[test]
fn a8c_the_content_key_separates_two_operands() {
    let key_of = |label: &str, at_transform: bool| -> editor_core::ContentKey {
        let (base_part, top_part, opts) = parts("msolve1-a8c");
        let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
        let (doc, base) = insert_pinned(doc, Node::instantiate_part(base_part.doc_ref), &opts);
        let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
        let (doc, xf) = insert(doc, xform(base, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
        let (doc, mate) = constrain_at(
            doc,
            top,
            rest(
                head(top_part.cap(top, CapEnd::Start)),
                head_at(
                    if at_transform { xf } else { base },
                    base_part.cap(base, CapEnd::End),
                ),
                ContactClass::Rest,
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

/// **A8(d).** A mate whose target is read at a transform round-trips
/// through persistence: the same document, byte for byte, and the same
/// solve.
///
/// *Breaks if* the operand a pose's face is read at is lost on the
/// wire.
#[test]
fn a8d_a_transform_operand_round_trips_through_persistence() {
    let s = scene("msolve1-a8d", &[LIFT]);
    let bytes = save(&s.doc, &[], Tol::witness()).expect("the document saves");
    let back = load(&bytes, Tol::witness()).expect("it loads").doc;
    assert_eq!(
        save(&back, &[], Tol::witness()).expect("it saves again"),
        bytes,
        "the wire form is stable across the round trip"
    );
    let m = fixture::constraint(&back, s.mate);
    let on = fixture::face_reads(&back, m.on);
    let to = fixture::face_reads(&back, *m.to.pose().expect("the seat's target is a pose"));
    let ([on], [to]) = (on.as_slice(), to.as_slice()) else {
        panic!("the rest survived reading one face on each pose: {on:?} {to:?}");
    };
    assert_eq!(
        crate::fixture::side_head(&back, *on).at,
        s.top,
        "the `on` pose's body rode the wire"
    );
    assert_eq!(
        crate::fixture::side_head(&back, *to).at,
        s.a_at,
        "the `to` pose's body rode the wire"
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

// ---- A10: a nested copy ----

/// **A10.** A nested pattern's copy is read through both `Instance(i)`
/// qualifiers, the seat places the top on it, and the document GATHERS
/// — a pattern over an `Instances` value places every instance, so the
/// outer pattern's value is the `2 × 3` copies laid out
/// placement-major (output body `j·M + i`).
///
/// **The name's numbering and the flat index coincide**, asserted on
/// the evaluated document: the name of copies `(j, i) = (1, 1)`
/// resolves in the outer pattern's table to flat body `1·3 + 1`, and
/// that body's cap sits in the product exactly where the outer map at
/// 1 over the inner map at 1, LEFT of the base's own placement, puts
/// the part's own cap. The seat itself is measured against the
/// placer-free control, as every row here is.
///
/// *Breaks if* the nested name resolves to another flat body, or the
/// copy's offset is composed off the base's placement.
#[test]
fn a10_a_nested_pattern_head_is_read_at_its_copy() {
    let (base_part, top_part, opts) = parts("msolve1-a10");
    let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a10"), Tol::witness());
    let (doc, base) = insert_pinned(doc, Node::instantiate_part(base_part.doc_ref), &opts);
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    // Spaced so that every copy clears the others — inner along x,
    // outer along y.
    const SPACING: f64 = 5.0;
    let rule = |dir: [f64; 3]| PatternKind::Linear {
        direction: dir.map(scl),
        spacing: len(SPACING),
    };
    let (doc, inner) = insert(
        doc,
        Node::Pattern {
            input: base.into(),
            count: editor_core::Formula::count(3),
            kind: rule([1.0, 0.0, 0.0]),
        },
    );
    let (doc, outer) = insert(
        doc,
        Node::Pattern {
            input: inner.into(),
            count: editor_core::Formula::count(2),
            kind: rule([0.0, 1.0, 0.0]),
        },
    );
    // The base is a placement, so its own copy stands where copy
    // (0, 0) of the outer pattern does: the world takes the other five
    // copies, and the instance stands for the first.
    let doc = (1..6).fold(doc, |doc, i| {
        let (doc, copy) = insert(
            doc,
            Node::Part {
                of: outer.into(),
                select: editor_core::PartSelect::Instance(editor_core::Formula::count(i)),
            },
        );
        fixture::place(doc, copy).0
    });
    let nest = |n: StableName| in_copy(outer, 1, in_copy(inner, 1, n));
    let a = nest(base_part.cap(base, CapEnd::End));
    let b = top_part.cap(top, CapEnd::Start);
    let (doc, _) = seat(doc, top, &top_part, &|n| head(nest(n)), &base_part, base);
    let poses = solve(&doc, &opts, Tol::witness());
    assert!(
        poses.fault(top).is_none(),
        "a nested-pattern head resolves through both levels: {:?}",
        poses.fault(top)
    );
    assert_eq!(
        poses.space(top),
        Some(Space::World),
        "the nested copy's faces place the top"
    );

    // The document gathers, and the top seats on the named copy.
    let ev = run(&doc, &opts);
    assert!(
        ev.value(outer).is_some(),
        "the pattern of the pattern evaluates: {:?}",
        ev.node_error(outer)
    );
    assert_seated(&doc, &ev, &a, &b, &control_seat("msolve1-a10"), "A10");
    assert!(
        gate(&doc, &ev).is_ok(),
        "A10: the gate refused: {:?}",
        gate(&doc, &ev).err()
    );

    // The numbering: (j, i) = (1, 1) is flat body 1·3 + 1.
    let table = ev.table_at(outer).expect("the outer");
    let Some(editor_core::Entry::Unique(row)) = table.lookup(&a) else {
        panic!("the nested name resolves uniquely: {:?}", table.lookup(&a));
    };
    assert_eq!(row.body, 3 + 1, "copy (1, 1) is output body j·M + i");

    // The pose: the copy's cap in the product is the offset (outer map
    // at 1 over inner map at 1: the authored spacing along both
    // authored directions) left of the base's placement, over the
    // part's own cap frame — read off a document that pins the part
    // alone at the identity.
    let placement = poses
        .placement(&doc, base)
        .expect("the base is placed")
        .affine::<f64>();
    let alone = ProfileDoc::empty(DocumentId::derive("msolve1-a10-alone"), Tol::witness());
    let (alone, base_alone) =
        insert_pinned(alone, Node::instantiate_part(base_part.doc_ref), &opts);
    let local = product_face_frame(
        &alone,
        &run(&alone, &opts),
        &base_part.cap(base_alone, CapEnd::End),
    );
    let offset = Affine3::translation(geom_core::Vec3::new(SPACING, SPACING, 0.0));
    let expected = offset * placement * local;
    let found = product_face_frame(&doc, &ev, &a);
    let gap = map_gap(&found, &expected);
    assert!(
        gap <= 1e-12,
        "the copy's cap sits {gap} from the offset over the base's placement"
    );
}

/// **A8(e).** A cut that would sever a constraint from its target is
/// refused: the cut takes `top`, and the seat in its bundle reads
/// `base`, which the cut leaves behind. A pose's face is a read, so
/// this is D-2's ordinary severed read, named at the pose.
///
/// *Breaks if* the cut is accepted, or the refusal names another read.
#[test]
fn a8e_a_cut_that_would_sever_the_target_refuses() {
    let s = scene("msolve1-a8e", &[]);
    let cut = fixture::with_placements(
        &s.doc,
        &[s.top]
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
                slot: Some(editor_core::OperandSlot::Side { constraint, side: MateSide::To }),
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
/// to the half of a face read that is a node rather than a name. Both
/// poses come out pointing at the same nodes' images — the `to` face
/// still read at the transform over its instance, the `on` face still
/// read at its own mint.
///
/// *Breaks if* the remap drops the operand a face is read at, or points
/// it at another node.
#[test]
fn a8f_an_accepted_cut_carries_the_operand_through_the_remap() {
    let mut store = PartStore::default();
    let base_part = stored(&mut store, block("msolve1-a8f-base", 1.0));
    let top_part = stored(&mut store, block("msolve1-a8f-top", 3.0));
    let o = with_resolver(store);
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
    let (doc, base) = insert_pinned(doc, Node::instantiate_part(base_part.doc_ref), &o);
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, xf) = insert(doc, xform(base, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, _) = constrain_at(
        doc,
        top,
        rest(
            head(top_part.cap(top, CapEnd::Start)),
            head_at(xf, base_part.cap(base, CapEnd::End)),
            ContactClass::Rest,
        ),
    );
    let doc = world(doc, &[local, xf], &[]);
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
        .expect("its bundle holds the rest");
    let m = c.mate().expect("the rest is a mate");
    let on = fixture::face_reads(&out.remainder, m.on);
    let to = fixture::face_reads(&out.remainder, *m.to.pose().expect("a pose target"));
    let ([on], [to]) = (on.as_slice(), to.as_slice()) else {
        panic!("the rest kept one face on each pose: {on:?} {to:?}");
    };
    let (a, b) = (
        crate::fixture::side_head(&out.remainder, *on),
        crate::fixture::side_head(&out.remainder, *to),
    );
    assert_ne!(
        b.at, b.name.node,
        "the `to` face is still read somewhere other than its mint"
    );
    let Some(Node::Transform { input, .. }) = out.remainder.node(b.at) else {
        panic!("the `to` operand still names a transform");
    };
    assert_eq!(
        out.remainder.operation_of(*input),
        Some(b.name.node),
        "over the very instance the name is headed at"
    );
    assert_eq!(
        a.at, a.name.node,
        "the `on` face is still read at its own mint"
    );
    assert!(
        matches!(out.remainder.node(a.at), Some(Node::InstantiatePart { .. })),
        "which is a live instance in the remainder"
    );
    assert!(
        solve(&out.remainder, &o, Tol::witness())
            .fault(moved)
            .is_none(),
        "and the remainder still solves"
    );
}

/// **A8(g) — a KEPT placement whose constraint reads a body inside
/// the cut refuses.** The rest in `top`'s bundle reads the base through
/// a transform at its `to` pose; the cut takes the base and that
/// transform, and keeps `top`. A face is a read, so this is D-2's
/// ordinary severed read.
///
/// *Breaks if* the cut is accepted, or its refusal speaks a node id in
/// decimal.
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
                slot: Some(editor_core::OperandSlot::Side { constraint, side: MateSide::To }),
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
///
/// *Breaks if* the cut is accepted, or the refusal names another read.
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
                slot: Some(editor_core::OperandSlot::Side { constraint, side: MateSide::To }),
                input,
                consumer_is_cut: true,
            } if *consumer == doc.spoken(mate.placement)
                && *constraint == mate.id
                && *input == doc.spoken(xf)
        ),
        "expected the severed-read refusal, got {err:?}"
    );
}

/// A document whose rest, in `top`'s bundle, reads `top`'s own face at
/// its `on` pose and the base through a transform over it at its `to`
/// pose; answers the rest, the base and the transform.
fn severed_operand_scene(label: &str) -> (ProfileDoc, MateRef, RecipeNodeId, RecipeNodeId) {
    let (base_part, top_part, opts) = parts(label);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert_pinned(doc, Node::instantiate_part(base_part.doc_ref), &opts);
    let (doc, xf) = insert(doc, xform(base, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, mate) = constrain_at(
        doc,
        top,
        rest(
            head(top_part.cap(top, CapEnd::Start)),
            head_at(xf, base_part.cap(base, CapEnd::End)),
            ContactClass::Rest,
        ),
    );
    (doc, mate, base, xf)
}

// ---- A11: a transform BETWEEN two patterns ----

/// **A11.** A transform between the two patterns: the copy's chain is
/// `[outer(j), T, inner(i)]` and the evaluator's body is
/// `M_o(j) ∘ T ∘ M_i(i)` — asserted on the product with a ROTATION, so
/// the other order is a different map and the row can tell them apart.
/// `T` lifts as well as turns, so no copy touches the base instance
/// the patterns start from.
///
/// *Breaks if* the three maps compose in another order.
#[test]
fn a11_a_transform_between_two_patterns_composes_outer_t_inner() {
    let (base_part, top_part, opts) = parts("msolve1-a11");
    let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a11"), Tol::witness());
    let (doc, base) = insert_pinned(doc, Node::instantiate_part(base_part.doc_ref), &opts);
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let rule = |dir: [f64; 3], s: f64| PatternKind::Linear {
        direction: dir.map(scl),
        spacing: len(s),
    };
    let (doc, inner) = insert(
        doc,
        Node::Pattern {
            input: base.into(),
            count: editor_core::Formula::count(3),
            kind: rule([1.0, 0.0, 0.0], 5.0),
        },
    );
    let spin = std::f64::consts::FRAC_PI_2;
    let (doc, t) = insert(doc, xform(inner, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], spin));
    let (doc, outer) = insert(
        doc,
        Node::Pattern {
            input: t.into(),
            count: editor_core::Formula::count(2),
            kind: rule([0.0, 1.0, 0.0], 20.0),
        },
    );
    let doc = world(doc, &[], &[(outer, 6)]);
    let nest = |n: StableName| in_copy(outer, 1, in_copy(inner, 1, n));
    let a = nest(base_part.cap(base, CapEnd::End));
    let b = top_part.cap(top, CapEnd::Start);
    let (doc, _) = seat(doc, top, &top_part, &|n| head(nest(n)), &base_part, base);
    let poses = solve(&doc, &opts, Tol::witness());
    assert!(poses.fault(top).is_none(), "{:?}", poses.fault(top));
    let ev = run(&doc, &opts);
    assert!(ev.value(outer).is_some(), "{:?}", ev.node_error(outer));
    assert_seated(&doc, &ev, &a, &b, &control_seat("msolve1-a11"), "A11");
    assert!(gate(&doc, &ev).is_ok(), "{:?}", gate(&doc, &ev).err());
    let table = ev.table_at(outer).unwrap();
    let Some(editor_core::Entry::Unique(row)) = table.lookup(&a) else {
        panic!("{:?}", table.lookup(&a))
    };
    assert_eq!(row.body, 3 + 1, "copy (1, 1) is flat body j·M + i");
    let placement = poses.placement(&doc, base).unwrap().affine::<f64>();
    let alone = ProfileDoc::empty(DocumentId::derive("msolve1-a11-alone"), Tol::witness());
    let (alone, base_alone) =
        insert_pinned(alone, Node::instantiate_part(base_part.doc_ref), &opts);
    let local = product_face_frame(
        &alone,
        &run(&alone, &opts),
        &base_part.cap(base_alone, CapEnd::End),
    );
    let m_o = Affine3::translation(geom_core::Vec3::new(0.0, 20.0, 0.0));
    let t_map = Affine3::translation(geom_core::Vec3::new(0.0, 0.0, 10.0))
        * Affine3::rotation_about_axis(
            geom_core::Point3::origin(),
            geom_core::Vec3::new(0.0, 0.0, 1.0),
            spin,
        );
    let m_i = Affine3::translation(geom_core::Vec3::new(5.0, 0.0, 0.0));
    let expected = m_o * t_map * m_i * placement * local;
    let found = product_face_frame(&doc, &ev, &a);
    let gap = map_gap(&found, &expected);
    assert!(gap <= 1e-12, "outer ∘ T ∘ inner: gap {gap}");
    let wrong = m_i * t_map * m_o * placement * local;
    assert!(
        map_gap(&found, &wrong) > 1.0,
        "the two orders are distinguishable here"
    );
}

// ---- A12: a Part over a nested pattern, and the seat read AT it ----

/// What a `Part(k)` row expects: the seat places the top (the `Part`'s
/// `k` IS the flat body the name's chain says), or the name is no face
/// of the body the `Part` selects and the placement refuses where it
/// evaluates.
#[derive(Clone, Copy)]
enum PartCase {
    Seats,
    Refuses,
}

/// The nested document of A10 with a `Part(k)` over the outer pattern
/// — through a transform when `via_transform` — and the seat read AT
/// that part, naming copy `(j, i)`.
fn part_over_nested(k: i64, j: u32, i: u32, via_transform: bool, expect: PartCase) {
    let label = format!("msolve1-a12-{k}-{j}-{i}-{via_transform}");
    let (base_part, top_part, opts) = parts(&label);
    let doc = ProfileDoc::empty(DocumentId::derive(&label), Tol::witness());
    let (doc, base) = insert_pinned(doc, Node::instantiate_part(base_part.doc_ref), &opts);
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let rule = |dir: [f64; 3]| PatternKind::Linear {
        direction: dir.map(scl),
        spacing: len(5.0),
    };
    let (doc, inner) = insert(
        doc,
        Node::Pattern {
            input: base.into(),
            count: editor_core::Formula::count(3),
            kind: rule([1.0, 0.0, 0.0]),
        },
    );
    let (doc, outer) = insert(
        doc,
        Node::Pattern {
            input: inner.into(),
            count: editor_core::Formula::count(2),
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
            select: editor_core::PartSelect::Instance(editor_core::Formula::count(k)),
        },
    );
    let nest = |n: StableName| in_copy(outer, j, in_copy(inner, i, n));
    let a = nest(base_part.cap(base, CapEnd::End));
    let b = top_part.cap(top, CapEnd::Start);
    let what = format!("Part({k}) naming ({j}, {i}), via transform: {via_transform}");
    // The door decides structure alone, so either seat enters.
    let (doc, mate) = seat(
        doc,
        top,
        &top_part,
        &|n| head_at(part, nest(n)),
        &base_part,
        base,
    );
    match expect {
        PartCase::Seats => {
            let doc = world(doc, &[part], &[]);
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
            assert_seated(&doc, &ev, &a, &b, &control_seat(&label), &what);
            assert!(
                gate(&doc, &ev).is_ok(),
                "{what}: {:?}",
                gate(&doc, &ev).err()
            );
        }
        PartCase::Refuses => {
            let fault = solve(&doc, &opts, Tol::witness())
                .fault(top)
                .cloned()
                .unwrap_or_else(|| panic!("{what}: a disagreeing Part placed the top"));
            assert!(
                matches!(
                    fault,
                    MateFault::SideUnevaluated { mate: m, side: MateSide::To, .. } if m == mate
                ),
                "{what}: expected the `to` pose unevaluated, got {fault:?}"
            );
        }
    }
}

/// **A12.** A `Part(k)` over a nested pattern selects FLAT body `k`,
/// and a face read at it names a copy `(j, i)` whose flat index is
/// `j·3 + i`: the name is a face of the `Part`'s body only where the
/// two agree, so `k == j` with `i ≠ 0` is a disagreement (the `Part`
/// gathers copy `(0, k)`, five units from the copy the name reads), not
/// an agreement — and the same through a transform above the outer
/// pattern, which preserves the value's indices.
///
/// *Breaks if* a `Part`'s index is read as the outer copy's rather
/// than the flat one, or a disagreeing name resolves to some face.
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
    for (k, j, i, via) in [
        (1, 1, 1, false),
        (0, 0, 1, false),
        (4, 0, 1, false),
        (1, 1, 1, true),
    ] {
        part_over_nested(k, j, i, via, PartCase::Refuses);
    }
}

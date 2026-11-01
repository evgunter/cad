//! **A mate reads the body its target names** — the placement-aware
//! seat.
//!
//! A mate's `to` pose is read off a face of the space the copy joins:
//! a FACE name, selected in the body it is read at (the read's
//! operand), which may be a placed copy of the instance that minted
//! the name, a pattern or a `Part` over it. The face is read where that
//! body stands, so a mate on a target's PLACED COPY seats the copy on
//! the placed copy's geometry and a mate on the instance seats it on
//! the instance. The `on` pose reads the copy's own geometry, never a
//! body downstream of it.
//!
//! Every row here goes through ordinary doors — `DocEdit::AddConstraint`,
//! `solve_document`, `evaluate`, `product` — and measures the
//! PRODUCT's own face frames: the planes the mate's two names resolve
//! to in the body a consumer actually gathers, never a solved frame
//! read by eye.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CapEnd, DocEdit, DocRef, DocumentId, EditError, EntityKind, EvalOptions, Evaluation, Formula,
    MateFault, MateRef, MateSide, Node, Operand, ProfileDoc, RecipeNodeId, RoleSeg, StableName,
    VarId, load, product, save,
};
use fixture::resolver::{PartStore, in_part, in_part_of, with_resolver};
use fixture::seat::map_gap;
use fixture::{
    constrain_at, could_look, head, head_at, in_member, insert, insert_pinned, len, on_frame, rest,
    run, solve, step,
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
        },
    )
}

/// A `1x1xh` block, as a whole part document.
fn block(label: &str, h: f64) -> (ProfileDoc, RecipeNodeId) {
    slab(label, 1.0, h)
}

/// **A stored part**: its reference, its body, and its square
/// profile's first two corners, which its corner frame reads.
struct Part {
    doc_ref: DocRef,
    body: RecipeNodeId,
    /// The profile's corners 0 and 1, the ends of wall 0.
    corners: [editor_core::ProfileVertexRef; 2],
}

fn stored(store: &mut PartStore, (doc, body): (ProfileDoc, RecipeNodeId)) -> Part {
    let corners = [0, 1].map(|v| fixture::vpiece(&doc, body, 0, v));
    let (doc_ref, body) = store.insert_part((doc, body), Tol::witness());
    Part {
        doc_ref,
        body,
        corners,
    }
}

impl Part {
    /// The frame at corner 0 of cap `end` as `instance` places it, read
    /// through `at` ([`fixture::corner_frame`]): `z` up its lateral
    /// edge, `x` along wall 0.
    fn frame(
        &self,
        instance: RecipeNodeId,
        end: CapEnd,
        at: &dyn Fn(StableName) -> Operand,
    ) -> Operand {
        let name = |kind, seg| in_part_of(kind, instance, self.body, seg).name;
        fixture::corner_frame(
            at(name(
                EntityKind::Edge,
                RoleSeg::LateralEdge(self.corners[0]),
            )),
            at(name(
                EntityKind::Vertex,
                RoleSeg::CapVertex(end, self.corners[1]),
            )),
        )
    }

    /// The cap `end` of this part as `instance` places it.
    fn cap(&self, instance: RecipeNodeId, end: CapEnd) -> crate::fixture::resolver::PartName {
        in_part(instance, self.body, end)
    }
}

/// **The seat**, added to `top`'s bundle: one pin (FORK-S3O) equating
/// the top's corner frame at its bottom cap with the base's at its top
/// cap, slid one unit along each of the base's walls 0 and 1, so the
/// `1x1` block stands on the `3x3` slab's middle square, `[1, 2]²`.
/// `base_at` reads a part name of the base where the row reads it — at
/// the instance, through a placed copy, a pattern or a `Part`.
/// Answers the pin.
fn seat(
    doc: ProfileDoc,
    top: RecipeNodeId,
    top_part: &Part,
    base_at: &dyn Fn(StableName) -> Operand,
    base_part: &Part,
    base: RecipeNodeId,
) -> (ProfileDoc, MateRef) {
    constrain_at(
        doc,
        top,
        fixture::frame_seat(
            top_part.frame(top, CapEnd::Start, &|n| fixture::read_at(top, n)),
            base_part.frame(base, CapEnd::End, base_at),
            1.0,
            1.0,
        ),
    )
}

// ---- the scene ----

/// The base's height and width: a `3x3x1` slab, wide enough that the
/// `1x1x3` block seated on it stands clear of its edges — a
/// same-footprint pair would rest face-on-face with every edge
/// coincident, where the census has more to say than the mate does.
const BASE_HEIGHT: f64 = 1.0;
const BASE_WIDTH: f64 = 3.0;
const TOP_HEIGHT: f64 = 3.0;

/// `base` (a wide slab, pinned in the world), optionally a copy of it
/// placed against it by one rigid motion, and `top` (a tall block)
/// seated on that copy — or on the base, where there is none.
struct Scene {
    doc: ProfileDoc,
    opts: EvalOptions,
    base: RecipeNodeId,
    top: RecipeNodeId,
    base_part: Part,
    top_part: Part,
    /// Where the seat reads the base: its placed copy, or the base.
    a_at: RecipeNodeId,
    /// The seat's rest, a constraint of `top`'s bundle.
    mate: MateRef,
}

/// **The world the product is**: each of `bodies` that is not already
/// a placement placed. An instance or a `Place` is a placement already
/// in the product, so it is never placed a second time — and a family
/// placed in the world is in it already, member by member. Each copy is
/// pinned at the identity over its geometry, which `o` reaches.
fn world(doc: ProfileDoc, o: &EvalOptions, bodies: &[RecipeNodeId]) -> ProfileDoc {
    bodies
        .iter()
        .copied()
        .filter(|&b| {
            !matches!(
                doc.node(b),
                Some(Node::InstantiatePart { .. } | Node::Place { .. })
            )
        })
        .collect::<Vec<_>>()
        .into_iter()
        .fold(doc, |doc, body| place(doc, o, body))
}

/// **A row of `count` copies of `input` along `dir`, the first one
/// `spacing` on** (D10 `[repetition]`), in `input`'s space under a fresh
/// index, `within` another or none: member `k` slid
/// `(scalar(k) + 1)·spacing`, so no member stands where `input` does —
/// a family placed in the world is in the product, beside its source.
/// The document, the index and the placement.
fn row_on(
    doc: ProfileDoc,
    input: RecipeNodeId,
    (dir, spacing): ([f64; 3], f64),
    count: i64,
    within: Option<VarId>,
) -> (ProfileDoc, VarId, RecipeNodeId) {
    use editor_core::{Coord, Dimension, TargetAxis};
    let name = fixture::fresh_index_name(&doc);
    let (doc, k) = fixture::index_of(doc, &name, Formula::count(count), within);
    let next = Formula::count_to_scalar(
        Formula::add(Formula::var(k, Dimension::Count), Formula::count(1)).unwrap(),
    )
    .unwrap();
    let values = [TargetAxis::X, TargetAxis::Y, TargetAxis::Z]
        .into_iter()
        .zip(dir)
        .filter(|&(_, d)| d != 0.0)
        .map(|(axis, d)| {
            (
                Coord::Slide(axis),
                Formula::mul(next.clone(), len(d * spacing)).unwrap(),
            )
        })
        .collect();
    let (doc, row) = fixture::family_of(doc, input, values);
    (doc, k, row)
}

/// **One copy of `body` in the world**, pinned at the identity over
/// its geometry as `o` evaluates it.
fn place(doc: ProfileDoc, o: &EvalOptions, body: RecipeNodeId) -> ProfileDoc {
    let read = fixture::body_read(&doc, body.into());
    let (doc, placement) = insert(
        doc,
        crate::fixture::stood(editor_core::BodyRead::plain(read)),
    );
    fixture::pinned(doc, placement, o)
}

/// **A name read at `at`**: a copy carries its source's names verbatim,
/// so a name is read at a placement as at its source.
fn read_at(_doc: &ProfileDoc, at: RecipeNodeId) -> impl Fn(StableName) -> Operand + use<> {
    move |name| fixture::read_at(at, name)
}

/// [`fixture::seat::assert_seated`] over the product names of `a` and
/// `b` ([`fixture::product_name`]).
fn assert_seated(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    a: &(impl fixture::Sited + Clone),
    b: &(impl fixture::Sited + Clone),
    control: &Affine3<f64>,
    what: &str,
) {
    let (a, b) = (
        fixture::product_name(doc, ev, a.clone()),
        fixture::product_name(doc, ev, b.clone()),
    );
    fixture::seat::assert_seated(doc, ev, &a, &b, control, what);
}

/// [`fixture::seat::seat_map`] over the product names of `a` and `b`.
fn seat_map(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    a: &(impl fixture::Sited + Clone),
    b: &(impl fixture::Sited + Clone),
) -> Affine3<f64> {
    fixture::seat::seat_map(
        doc,
        ev,
        &fixture::product_name(doc, ev, a.clone()),
        &fixture::product_name(doc, ev, b.clone()),
    )
}

/// [`fixture::seat::product_face_frame`] of `name`'s product name.
fn product_face_frame(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    name: &(impl fixture::Sited + Clone),
) -> Affine3<f64> {
    fixture::seat::product_face_frame(doc, ev, &fixture::product_name(doc, ev, name.clone()))
}

/// A rigid motion: `(translation, rotation axis, angle)`.
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

/// Builds that scene. `moved` places a copy of the base against it by
/// that motion, where the seat then reads the base.
fn scene(label: &str, moved: Option<Step>) -> Scene {
    scene_at(label, moved, &editor_core::Frame::IDENTITY)
}

/// [`scene`] with the base pinned to the world at `at`.
fn scene_at(label: &str, moved: Option<Step>, at: &editor_core::Frame) -> Scene {
    let (base_part, top_part, opts) = parts(label);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, fixture::mated_instance(base_part.doc_ref));
    let doc = fixture::pin(doc, base, at, &opts);
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, a_at) = match moved {
        Some((t, axis, angle)) => place_moved(doc, base, t, axis, angle, &opts),
        None => (doc, base),
    };
    let read = read_at(&doc, a_at);
    let (doc, mate) = seat(doc, top, &top_part, &read, &base_part, base);
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

/// **A copy of `input` moved within its own space** by the rigid
/// motion `(t, axis, angle)`: a placement of it against its own
/// geometry, under `o`. Returns the document and the placement.
fn place_moved(
    doc: ProfileDoc,
    input: RecipeNodeId,
    t: [f64; 3],
    axis: [f64; 3],
    angle: f64,
    o: &EvalOptions,
) -> (ProfileDoc, RecipeNodeId) {
    let at = editor_core::Frame::rotate_then_translate(axis, angle, t, fixture::band())
        .expect("a rotation axis with a direction");
    crate::mate1_member_vocab::against_with(doc, input, input, &at, o)
}

const LIFT: Step = ([0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0);

/// **The seat this fixture's mates ask for**, measured on the document
/// that has no placed copy and no pattern anywhere: the relative frame
/// every other row must reproduce.
///
/// It is derived, not written down — a constant transcribed here
/// would be a second statement of what the mates say, and the two
/// could drift apart.
fn control_seat(label: &str) -> Affine3<f64> {
    let s = scene(&format!("{label}-control"), None);
    let ev = s.eval();
    seat_map(&s.doc, &ev, &s.face_a(), &s.face_b())
}

impl Scene {
    fn eval(&self) -> Evaluation<f64> {
        run(&self.doc, &self.opts)
    }
    /// The base's top cap where the seat reads it: at its placed copy,
    /// or at the base.
    fn face_a(&self) -> crate::fixture::resolver::PartName {
        crate::fixture::resolver::PartName {
            at: self.a_at,
            name: self.base_part.cap(self.base, CapEnd::End).name,
        }
    }
    fn face_b(&self) -> crate::fixture::resolver::PartName {
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
        if let Err(e) = product(&self.doc, &ev, Tol::witness()) {
            panic!(
                "{what}: the product gathers: {e:?}; {:?}",
                self.doc
                    .ids()
                    .into_iter()
                    .filter_map(|id| ev.node_error(id))
                    .collect::<Vec<_>>()
            );
        }
        assert_seated(
            &self.doc,
            &ev,
            &self.face_a(),
            &self.face_b(),
            control,
            what,
        );
        assert!(
            could_look(&self.doc, &ev).is_ok(),
            "{what}: the at-rest check could not look at a seat the solve placed: {:?}",
            could_look(&self.doc, &ev).err()
        );
    }
}

// ---- A1: the finding's own document, on the target ----

/// **A1.** The two-block document with a copy of the target instance
/// placed against it `+10` up `z` (angle 0), the seat read at the copy:
/// the solve records no fault, the product gathers, the mated faces
/// MEET in the product, and the top's solved pose differs from the
/// control's by exactly the copy's motion.
///
/// *Breaks if* the seat reads the base where its instance stands
/// rather than where its copy's placement puts it (the top stays at
/// `z = 1`), or a translation turns the top.
#[test]
fn a1_a_translated_target_seats_in_the_product() {
    let seat = control_seat("msolve1-a1");
    let control = scene("msolve1-a1-control", None);
    control.assert_green_and_seated(&seat, "A1 control");

    let test = scene("msolve1-a1-lifted", Some(LIFT));
    test.assert_green_and_seated(&seat, "A1 lifted");

    // The mechanism, stated as arithmetic rather than as a string
    // comparison: the top rose by exactly the copy's lift, so it
    // stands on the lifted copy where it stood on the plain base.
    let c = solve(&control.doc, &control.opts, Tol::witness())
        .relative(control.top)
        .expect("the control solves");
    let t = solve(&test.doc, &test.opts, Tol::witness())
        .relative(test.top)
        .expect("the test solves");
    assert_eq!(
        (t.translation[2] - c.translation[2]).to_bits(),
        10.0_f64.to_bits(),
        "the seat followed exactly the copy's +10 lift \
         (control {:?} vs test {:?})",
        c.translation,
        t.translation
    );
    // ...and the rotation part is untouched by a translation-only
    // motion, bit for bit.
    assert_eq!(c.columns, t.columns, "a translation moves no axis");
}

// ---- A2: rotation ----

/// **A2.** A copy of the target placed against it by a non-zero turn
/// about z, and by ones about non-axis directions, the seat read at the
/// copy: the mated faces still seat in the product, their outward normals opposed and their whole relative
/// frame the control's — and the at-rest gate still passes.
///
/// The about-z rows are the case a normal-and-gap oracle cannot reach:
/// a spin about the seat normal leaves both faces coplanar and both
/// normals opposed while turning the block. The relative frame is what
/// sees it.
///
/// *Breaks if* any of the copy's turn is lost from the faces the seat
/// reads.
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
            Some(([0.0, 0.0, 10.0], axis, angle)),
        );
        s.assert_green_and_seated(&seat, &format!("A2 {what}"));
    }
}

// ---- A3: patterns over placed copies, and placed copies of patterns ----

/// The copy of the base's pattern the seat reads.
/// The member the seat reads: the row's first, one spacing on.
const COPY: u32 = 0;

/// **A3(a).** A FAMILY over a PLACED COPY of the target, the seat
/// read at the family: it solves, and the top seats on the named member
/// in the product, which stands at `M(1) ∘ T`. The seat is a constant
/// of the mates and the two part documents, so the plain control's
/// relative frame is the one a pattern copy must give too.
///
/// *Breaks if* a member's face is read at the master's pose, or the
/// placement's motion and the family's compose in the wrong order.
#[test]
fn a3a_a_pattern_of_a_placed_copy_seats() {
    let seat_frame = control_seat("msolve1-a3a");
    {
        let (base_part, top_part, opts) = parts("msolve1-a3a");
        let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a3a"), Tol::witness());
        let (doc, base) = insert_pinned(
            doc,
            crate::fixture::mated_instance(base_part.doc_ref),
            &opts,
        );
        let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
        let (doc, xf) = place_moved(doc, base, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0, &opts);
        // Spaced wide enough that the members stand clear of the top:
        // the row is about the member the seat names, and a sibling
        // touching the top uninvited is an UNRECORDED contact the gate
        // is right to refuse.
        let (doc, k, pattern) = row_on(doc, xf, ([1.0, 0.0, 0.0], 5.0), 2, None);
        let a = in_member(pattern, vec![(k, COPY)], base_part.cap(base, CapEnd::End));
        let b = top_part.cap(top, CapEnd::Start);
        let (doc, _) = seat(
            doc,
            top,
            &top_part,
            &|n| fixture::read_minted(in_member(pattern, vec![(k, COPY)], n)),
            &base_part,
            base,
        );
        let poses = solve(&doc, &opts, Tol::witness());
        assert!(
            poses.fault(top).is_none(),
            "A3 family of a placed copy refused: {:?}",
            poses.fault(top)
        );
        let ev = run(&doc, &opts);
        assert_seated(&doc, &ev, &a, &b, &seat_frame, "A3 family of a placed copy");
        assert!(
            could_look(&doc, &ev).is_ok(),
            "A3 family of a placed copy: the gate refused: {:?}",
            could_look(&doc, &ev).err()
        );
    }
}

// ---- A4: a target pinned off the identity ----

/// **A4.** The target pinned to the world at a non-identity pose,
/// turned off every axis and slid: the face the seat reads stands where
/// the pin puts it, so a target that carries an authored pose seats
/// the top as the control's does.
///
/// *Breaks if* the target's pin is dropped from the face the seat
/// reads.
#[test]
fn a4_a_target_pinned_off_the_identity_seats() {
    let seat = control_seat("msolve1-a4-placed");
    let at = editor_core::Frame::rotate_then_translate(
        [0.3, -0.2, 0.9],
        0.8,
        [2.0, -1.0, 3.0],
        fixture::band(),
    )
    .expect("a literal axis has a definite direction");
    let placed = scene_at("msolve1-a4-placed-frame", None, &at);
    placed.assert_green_and_seated(&seat, "A4 placed target");
}

// ---- A5: two seats through two operands ----

/// One seat of `top` on `base` read at a placed copy `x1` of the base,
/// and a second rest of the top read at a DIFFERENT placed copy `x2` of
/// the base; both copies are in the product.
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
    let (doc, base) = insert_pinned(
        doc,
        crate::fixture::mated_instance(base_part.doc_ref),
        &opts,
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, x1) = place_moved(doc, base, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0, &opts);
    let (doc, x2) = place_moved(
        doc,
        base,
        [5.0, 5.0, 10.0 + extra_lift],
        [0.0, 0.0, 1.0],
        0.0,
        &opts,
    );
    let (at1, at2) = (read_at(&doc, x1), read_at(&doc, x2));
    let (doc, m1) = seat(doc, top, &top_part, &at1, &base_part, base);
    let second = rest(
        head(top_part.cap(top, CapEnd::Start)),
        at2(base_part.cap(base, CapEnd::End).name),
    );
    (doc, opts, m1, second, top)
}

/// **A5.** A seat read through one placed copy of the target pins the
/// top, and a second rest through another placed copy of the same
/// target is a mate beyond the pin: the door refuses it as
/// overconstrained, naming the seat's own mate and the rest, whether
/// the rest agrees with the seat or not (parallel planes apart). Before
/// the second rest, the seat alone solves and the product gathers.
/// *Breaks if* the door admits a mate beyond the pin, or tells a rest
/// that agrees from one that does not (it measured them).
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
        let refused = match fixture::at_the_door_of(&doc, top, second) {
            Err((_, fault)) => fault,
            Ok(_) => panic!("A5 {what}: the door admitted a second rest beyond the pin"),
        };
        assert!(
            matches!(
                &refused,
                MateFault::Overconstrained { placement, constraints, free: 0 }
                    if *placement == top
                        && constraints.len() == 2
                        && constraints.contains(&held.id)
            ),
            "A5 {what}: the second rest is beyond the pin: {refused:?}"
        );
    }
}

// ---- A6: a residual refuses at the door ----

/// **A6.** A bundle that leaves a residual pins less than the copy
/// needs: a lone rest leaves the top free to slide and spin on the
/// base, so writing it as the top's bundle refuses `Unpinned` at the
/// door with the three freedoms it leaves — read at the target or at a
/// placed copy of it, identically (D10 `[placement.placement]`: every
/// bundle pins its copy).
///
/// *Breaks if* a residual is admitted, or the copy is taken as pinned.
#[test]
fn a6_a_residual_bundle_refuses_on_the_target_or_its_copy() {
    let residual = |label: &str, lift: bool| -> (RecipeNodeId, editor_core::EditError) {
        let (base_part, top_part, opts) = parts(label);
        let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
        let (doc, base) = insert_pinned(
            doc,
            crate::fixture::mated_instance(base_part.doc_ref),
            &opts,
        );
        let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
        let (doc, at) = if lift {
            place_moved(doc, base, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0, &opts)
        } else {
            (doc, base)
        };
        let to = read_at(&doc, at)(base_part.cap(base, CapEnd::End).name);
        let mate = rest(head(top_part.cap(top, CapEnd::Start)), to);
        let refused = fixture::bundle_edits(
            &doc,
            top,
            vec![editor_core::Constraint::of_mate(
                editor_core::ConstraintId::placeholder(1),
                mate,
            )],
        )
        .into_iter()
        .try_fold(doc, |doc, edit| {
            editor_core::apply(&doc, &edit, Tol::witness()).map(|applied| applied.doc)
        })
        .expect_err("a residual refuses");
        (top, refused)
    };
    for (what, lift) in [("plain", false), ("lifted", true)] {
        let (top, refused) = residual(&format!("msolve1-a6-{what}"), lift);
        assert!(
            matches!(
                &refused,
                editor_core::EditError::MateRefused { fault, .. }
                    if matches!(**fault, MateFault::Unpinned { placement, free: 3 } if placement == top)
            ),
            "A6 {what}: the residual refuses Unpinned: {refused:?}"
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
    let s = scene("msolve1-a8a", None);
    let ghost = RecipeNodeId::new(0, 9_999);
    let err = s
        .doc
        .apply(
            &DocEdit::AddConstraint {
                placement: s.top,
                mate: Box::new(rest(head(s.face_b()), head_at(ghost, s.face_a()))),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect_err("a never-existed operand is a typo");
    assert!(
        matches!(&err, EditError::UnresolvedInput { input } if input.id() == ghost),
        "expected the unresolved read, got {err:?}"
    );
}

/// **A8(b).** Deleting the placed copy a seat reads the target at
/// strands its `to` pose: D10's unresolved reader, which the placement
/// refuses where it evaluates, naming the constraint and the pose, until
/// the read is re-authored — and the copy it would place has no pose.
///
/// *Breaks if* the stranded seat is folded anyway, or its refusal is
/// laid on the `on` pose.
#[test]
fn a8b_deleting_the_operand_leaves_the_pose_unresolved() {
    let s = scene("msolve1-a8b", Some(LIFT));
    let (doc, _) = step(s.doc, DocEdit::DeleteNode { id: s.a_at });
    let ev = run(&doc, &s.opts);
    assert!(
        matches!(
            ev.node_error(s.mate.placement).map(|e| &e.kind),
            Some(editor_core::NodeErrorKind::UnresolvedRead {
                slot: editor_core::OperandSlot::Side {
                    constraint,
                    side: MateSide::To,
                },
                ..
            }) if *constraint == s.mate.id
        ),
        "expected the stranded pose, got {:?}",
        ev.node_error(s.mate.placement)
    );
    assert!(
        solve(&doc, &s.opts, Tol::witness())
            .placement(&doc, s.mate.placement)
            .is_err(),
        "a copy placed by a stranded constraint has no pose"
    );
    // The stranded pose's reads reach no live node; the live pose's
    // still read the top, which is what makes the refusal a per-pose
    // one.
    let m = fixture::constraint(&doc, s.mate);
    let reached = |pose| -> Vec<RecipeNodeId> {
        fixture::face_reads(&doc, pose)
            .into_iter()
            .flat_map(|read| doc.read_ports(read).into_iter().map(|(at, _)| at))
            .collect()
    };
    assert!(
        reached(m.to).is_empty(),
        "the stranded pose reads nothing live"
    );
    assert!(
        reached(m.on).iter().all(|&at| at == s.top) && !reached(m.on).is_empty(),
        "the live pose still reads the top"
    );
}

/// **A8(c).** Two placements whose constraints differ only in the
/// operand their target is read at — one name, copy 1's cap, read at a
/// pattern and at the `Part` picking that copy out of it — are
/// different content keys: the memo cannot serve one's answer for the
/// other. Measured on the evaluated values, which is where a key is
/// read.
///
/// *Breaks if* the operand a face is read at drops out of the key.
#[test]
fn a8c_the_content_key_separates_two_operands() {
    let key_of = |label: &str, at_part: bool| -> editor_core::ContentKey {
        let (base_part, top_part, opts) = parts("msolve1-a8c");
        let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
        let (doc, base) = insert_pinned(
            doc,
            crate::fixture::mated_instance(base_part.doc_ref),
            &opts,
        );
        let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
        let (doc, k, pattern) = row_on(doc, base, ([1.0, 0.0, 0.0], 5.0), 2, None);
        let (doc, part) = fixture::member(doc, pattern, 1);
        let (doc, mate) = constrain_at(
            doc,
            top,
            rest(
                head(top_part.cap(top, CapEnd::Start)),
                head_at(
                    if at_part { part } else { pattern },
                    in_member(pattern, vec![(k, 1)], base_part.cap(base, CapEnd::End)),
                ),
            ),
        );
        let ev = run(&doc, &opts);
        ev.value(mate.placement)
            .expect("the placement is a value")
            .content_key
    };
    assert_ne!(
        key_of("msolve1-a8c-at-family", false),
        key_of("msolve1-a8c-at-member", true),
        "the operand is part of what a mate says"
    );
}

/// **A8(d).** A mate whose target is read at a placed copy round-trips
/// through persistence: the same document, byte for byte, and the same
/// solve.
///
/// *Breaks if* the operand a pose's face is read at is lost on the
/// wire.
#[test]
fn a8d_a_placed_copy_operand_round_trips_through_persistence() {
    let s = scene("msolve1-a8d", Some(LIFT));
    let bytes = save(&s.doc, &[], Tol::witness()).expect("the document saves");
    let back = load(&bytes, Tol::witness()).expect("it loads").doc;
    assert_eq!(
        save(&back, &[], Tol::witness()).expect("it saves again"),
        bytes,
        "the wire form is stable across the round trip"
    );
    let m = fixture::constraint(&back, s.mate);
    let reached = |pose| -> std::collections::BTreeSet<RecipeNodeId> {
        fixture::face_reads(&back, pose)
            .into_iter()
            .flat_map(|read| back.read_ports(read).into_iter().map(|(at, _)| at))
            .collect()
    };
    assert_eq!(
        reached(m.on),
        [s.top].into(),
        "the `on` pose's edge and corner rode the wire at the top"
    );
    assert_eq!(
        reached(m.to),
        [s.a_at].into(),
        "the `to` pose's edge and corner rode the wire at the placed copy"
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

/// **A10.** A family of families' member is read through its one
/// `Member { (i, j) }` qualifier, the seat places the top on it, and the document
/// GATHERS — a placement over a family is a member per `(i, j)`, `i`
/// outer (D10 `[repetition]`), every member in the world.
///
/// **The name's indices and the member's place coincide**, asserted on
/// the evaluated document: the name of member `(i, j) = (1, 1)`
/// resolves in the outer family's table to body `1·2 + 1`, and that
/// body's cap sits in the product exactly where the outer map at 1
/// over the inner map at 1, LEFT of the base's own placement, puts the
/// part's own cap. The seat itself is measured against the placer-free
/// control, as every row here is.
///
/// *Breaks if* the nested name resolves to another member, or the
/// member's offset is composed off the base's placement.
#[test]
fn a10_a_nested_family_head_is_read_at_its_member() {
    let (base_part, top_part, opts) = parts("msolve1-a10");
    let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a10"), Tol::witness());
    let (doc, base) = insert_pinned(
        doc,
        crate::fixture::mated_instance(base_part.doc_ref),
        &opts,
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    // Spaced so that every member clears the others — inner along x,
    // outer along y.
    const SPACING: f64 = 5.0;
    let (doc, i, inner) = row_on(doc, base, ([1.0, 0.0, 0.0], SPACING), 3, None);
    let (doc, j, outer) = row_on(doc, inner, ([0.0, 1.0, 0.0], SPACING), 2, Some(i));
    let nest = |n: StableName| in_member(outer, vec![(i, 1), (j, 1)], n);
    let a = nest(base_part.cap(base, CapEnd::End).name);
    let b = top_part.cap(top, CapEnd::Start);
    let (doc, _) = seat(
        doc,
        top,
        &top_part,
        &|n| fixture::read_minted(nest(n)),
        &base_part,
        base,
    );
    let poses = solve(&doc, &opts, Tol::witness());
    assert!(
        poses.fault(top).is_none(),
        "a nested-family head resolves through its member: {:?}",
        poses.fault(top)
    );
    assert_eq!(
        poses.space(top),
        Some(VarId::WORLD),
        "the nested member's faces place the top"
    );

    // The document gathers, and the top seats on the named copy.
    let ev = run(&doc, &opts);
    assert!(
        ev.value(outer).is_some(),
        "the family of the family evaluates: {:?}",
        ev.node_error(outer)
    );
    assert_seated(&doc, &ev, &a, &b, &control_seat("msolve1-a10"), "A10");
    assert!(
        could_look(&doc, &ev).is_ok(),
        "A10: the gate refused: {:?}",
        could_look(&doc, &ev).err()
    );

    // The numbering: (i, j) = (1, 1) is body 1·2 + 1.
    let table = ev.table_at(outer).expect("the outer");
    let Some(editor_core::Entry::Unique(row)) = table.lookup(&a) else {
        panic!("the nested name resolves uniquely: {:?}", table.lookup(&a));
    };
    assert_eq!(row.body, 2 + 1, "member (1, 1) is body i·N + j");

    // The pose: the member's cap in the product is the offset (outer
    // map at 1 over inner map at 1: twice the authored spacing along
    // both authored directions) left of the base's placement, over the
    // part's own cap frame — read off a document that pins the part
    // alone at the identity.
    let placement = poses
        .placement(&doc, base)
        .expect("the base is placed")
        .affine::<f64>();
    let alone = ProfileDoc::empty(DocumentId::derive("msolve1-a10-alone"), Tol::witness());
    let (alone, base_alone) = insert_pinned(
        alone,
        crate::fixture::mated_instance(base_part.doc_ref),
        &opts,
    );
    let local = product_face_frame(
        &alone,
        &run(&alone, &opts),
        &base_part.cap(base_alone, CapEnd::End),
    );
    let offset = Affine3::translation(geom_core::Vec3::new(2.0 * SPACING, 2.0 * SPACING, 0.0));
    let expected = offset * placement * local;
    let found = product_face_frame(&doc, &ev, &a);
    let gap = map_gap(&found, &expected);
    assert!(
        gap <= 1e-12,
        "the member's cap sits {gap} from the offset over the base's placement"
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
    let s = scene("msolve1-a8e", None);
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
/// still read at the placed copy of its instance, the `on` face still
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
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, local) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let (doc, base) = insert_pinned(doc, crate::fixture::mated_instance(base_part.doc_ref), &o);
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, xf) = place_moved(doc, base, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0, &o);
    let to = read_at(&doc, xf)(base_part.cap(base, CapEnd::End).name);
    let (doc, _) = constrain_at(doc, top, rest(head(top_part.cap(top, CapEnd::Start)), to));
    let doc = world(doc, &o, &[local, xf]);
    // Cut the LOCAL block out into its own part, with the world
    // placement of what it moves: nothing it holds is read by a
    // constraint, so the cut is accepted.
    let cut = fixture::with_placements(
        &doc,
        &[profile, local]
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
    // constraint's operand moved with it and still names the placed
    // copy of its instance.
    let [moved] = fixture::constrained(&out.remainder)
        .into_iter()
        .filter(|&p| matches!(out.remainder.node(p), Some(Node::InstantiatePart { .. })))
        .collect::<Vec<_>>()
        .try_into()
        .expect("one constrained instance stayed in the remainder");
    let bundle = fixture::bundle(&out.remainder, moved);
    let [m] = bundle
        .iter()
        .filter_map(editor_core::Constraint::mate)
        .collect::<Vec<_>>()
        .try_into()
        .expect("its bundle holds the rest, and its values");
    assert_eq!(bundle.len(), 4, "the rest and its three values");
    let on = fixture::face_reads(&out.remainder, m.on);
    let to = fixture::face_reads(&out.remainder, m.to);
    let ([on], [to]) = (on.as_slice(), to.as_slice()) else {
        panic!("the rest kept one face on each pose: {on:?} {to:?}");
    };
    let (a, b) = (
        crate::fixture::side_head(&out.remainder, *on),
        crate::fixture::side_head(&out.remainder, *to),
    );
    let Some(Node::Place { source, .. }) = out.remainder.node(b.at) else {
        panic!("the `to` operand still names a placement");
    };
    assert!(
        out.remainder
            .operation_of(source.read)
            .and_then(|at| out.remainder.node(at))
            .is_some_and(|node| matches!(node, Node::InstantiatePart { .. })),
        "the placement copies the instance, whose names the `to` face carries: {:?}",
        b.name
    );
    assert_eq!(
        a.at, moved,
        "the `on` face is still read at the instance whose copy carries it"
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
/// a placed copy at its `to` pose; the cut takes the base and that
/// copy, and keeps `top`. A face is a read, so this is D-2's
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
/// its `on` pose and the base through a placed copy of it at its `to`
/// pose; answers the rest, the base and the copy.
fn severed_operand_scene(label: &str) -> (ProfileDoc, MateRef, RecipeNodeId, RecipeNodeId) {
    let (base_part, top_part, opts) = parts(label);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert_pinned(
        doc,
        crate::fixture::mated_instance(base_part.doc_ref),
        &opts,
    );
    let (doc, xf) = place_moved(doc, base, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0, &opts);
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, mate) = constrain_at(
        doc,
        top,
        rest(
            head(top_part.cap(top, CapEnd::Start)),
            head_at(xf, base_part.cap(base, CapEnd::End)),
        ),
    );
    (doc, mate, base, xf)
}

// ---- A11: a placement BETWEEN two families ----

/// **A11.** A placed copy `T` of the inner family, a family of its own
/// over `i`, placed again over `j` within `i`: the member's chain is
/// `[outer(i, j), T(i), inner(i)]` and the evaluator's body is
/// `M_o(j) ∘ T ∘ M_i(i)` — asserted on the product with a ROTATION, so
/// the other order is a different map and the row can tell them apart.
/// `T` lifts as well as turns, so no member touches the base instance
/// the families start from.
///
/// *Breaks if* the three maps compose in another order.
#[test]
fn a11_a_placement_between_two_patterns_composes_outer_t_inner() {
    let (base_part, top_part, opts) = parts("msolve1-a11");
    let doc = ProfileDoc::empty(DocumentId::derive("msolve1-a11"), Tol::witness());
    let (doc, base) = insert_pinned(
        doc,
        crate::fixture::mated_instance(base_part.doc_ref),
        &opts,
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, i, inner) = row_on(doc, base, ([1.0, 0.0, 0.0], 5.0), 3, None);
    let spin = std::f64::consts::FRAC_PI_2;
    let (doc, t) = place_moved(doc, inner, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], spin, &opts);
    let (doc, j, outer) = row_on(doc, t, ([0.0, 1.0, 0.0], 20.0), 2, Some(i));
    let nest = |n: StableName| in_member(outer, vec![(i, 1), (j, 1)], n);
    let a = nest(base_part.cap(base, CapEnd::End).name);
    let b = top_part.cap(top, CapEnd::Start);
    let (doc, _) = seat(
        doc,
        top,
        &top_part,
        &|n| fixture::read_minted(nest(n)),
        &base_part,
        base,
    );
    let poses = solve(&doc, &opts, Tol::witness());
    assert!(poses.fault(top).is_none(), "{:?}", poses.fault(top));
    let ev = run(&doc, &opts);
    assert!(ev.value(outer).is_some(), "{:?}", ev.node_error(outer));
    assert_seated(&doc, &ev, &a, &b, &control_seat("msolve1-a11"), "A11");
    assert!(
        could_look(&doc, &ev).is_ok(),
        "{:?}",
        could_look(&doc, &ev).err()
    );
    let table = ev.table_at(outer).unwrap();
    let Some(editor_core::Entry::Unique(row)) = table.lookup(&a) else {
        panic!("{:?}", table.lookup(&a))
    };
    assert_eq!(row.body, 2 + 1, "member (1, 1) is body i·N + j");
    let placement = poses.placement(&doc, base).unwrap().affine::<f64>();
    let alone = ProfileDoc::empty(DocumentId::derive("msolve1-a11-alone"), Tol::witness());
    let (alone, base_alone) = insert_pinned(
        alone,
        crate::fixture::mated_instance(base_part.doc_ref),
        &opts,
    );
    let local = product_face_frame(
        &alone,
        &run(&alone, &opts),
        &base_part.cap(base_alone, CapEnd::End),
    );
    let m_o = Affine3::translation(geom_core::Vec3::new(0.0, 40.0, 0.0));
    let t_map = Affine3::translation(geom_core::Vec3::new(0.0, 0.0, 10.0))
        * Affine3::rotation_about_axis(
            geom_core::Point3::origin(),
            geom_core::Vec3::new(0.0, 0.0, 1.0),
            spin,
        );
    let m_i = Affine3::translation(geom_core::Vec3::new(10.0, 0.0, 0.0));
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

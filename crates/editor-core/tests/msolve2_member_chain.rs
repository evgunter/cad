//! **The member chain** — nested copies through `Node::Part`, and
//! sibling distinctness at every level of the nest.
//!
//! A pattern's value is many bodies and a pattern's input is one, so
//! a pattern OVER a pattern does not evaluate. The nested shape a
//! user can build runs through `Node::Part { select: Instance(i) }`,
//! which projects one body out of the instances carrying every name
//! VERBATIM: `Pattern` over `Part` over `Pattern`. The walk from the
//! body a constraint's side reads passes the `Part` contributing
//! nothing, consumes an `Instance(i)` qualifier at each pattern, and
//! lands on the instance that minted the name — so a nested copy's
//! member is that instance, the CHAIN of copies, and the body read.
//!
//! Every row seats a `cap` instance, whose bundle the constraint joins,
//! on a nested copy of a `top` instance pinned to the world: the copy
//! is the constraint's target, and the solve composes its chain into
//! the cap's pose. Every row goes through ordinary doors —
//! `DocEdit::InsertNode`, `DocEdit::AddConstraint`, `solve_document`,
//! `evaluate`, `product`, `assemble` — and the seating rows measure the
//! PRODUCT's own face frames with the shared whole-frame oracle
//! (`fixture::seat`), never a solved frame read by eye.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::AuthoredNode;
use editor_core::ExtrudeSide;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, Datum, Dimension, DocEdit, DocumentId, EvalOptions,
    Formula, FreeValue, FreeVar, Mate, MateFault, MateFrame, MatePrimitive, MateRef, MateSide,
    Node, PartSelect, PatternKind, ProfileDoc, RecipeNodeId, RefusingReach, SitedFace, Space,
    StableName, VarName, member_of, product,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::seat::{assert_seated as assert_seated_named, seat_map as seat_map_named};
use fixture::{
    ang, at_the_door, door_refusal, gate, in_copy, insert, len, on_frame, run, scl, solve, step,
    xform,
};
use geom_core::Tol;
use geom_core::linalg::Affine3;

// ---- the scene's part ----

/// The block's width and height: a `1x1x3` block, patterned as `top`
/// and instantiated again as the `cap` seated on its copies.
const TOP_WIDTH: f64 = 1.0;
const TOP_HEIGHT: f64 = 3.0;

/// A `w x w x h` block, as a whole part document, and its body.
fn part_doc(label: &str, w: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
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

/// A `Rest`/`FrameCoincidence`/`Opposed` mate seating `b`'s bottom cap
/// onto the point `a_origin` of `a`'s top cap, each frame authored in
/// its own member's part coordinates.
///
/// The two outward normals and `Opposed` are what make this a
/// physical seat: the cap stands ON the copy.
fn seat_at(a: SitedFace, b: SitedFace, a_origin: [f64; 3]) -> Mate<Formula> {
    fixture::mate(
        a,
        b,
        ContactClass::Rest,
        Alignment {
            a: MateFrame::authored(
                a_origin,
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 0.0],
                geom_core::Tol::witness(),
            )
            .expect("a definite frame"),
            b: MateFrame::authored(
                [0.0, 0.0, 0.0],
                [0.0, 0.0, -1.0],
                [1.0, 0.0, 0.0],
                geom_core::Tol::witness(),
            )
            .expect("a definite frame"),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    )
}

/// **The world the product is** (A10): each of `bodies` placed, then
/// each `(pattern, count)`'s copies placed through a `Part` per copy.
/// The instances are placements of their own and are never placed
/// again here.
fn world(doc: ProfileDoc, bodies: &[RecipeNodeId], patterns: &[(RecipeNodeId, i64)]) -> ProfileDoc {
    let doc = fixture::place_all(doc, bodies);
    patterns.iter().fold(doc, |doc, &(pattern, count)| {
        (0..count).fold(doc, |doc, i| {
            let (doc, copy) = insert(doc, part_of(pattern, i));
            fixture::place(doc, copy).0
        })
    })
}

/// **`name` as the product names it**: the name itself where the
/// product's table holds it (an instance's own face), else the one
/// name that is a copy of `name` ([`StableName::copy_of`]) — what the
/// product names a placed member's face by, under the placement that
/// put it in the world.
fn world_name(
    doc: &ProfileDoc,
    ev: &editor_core::Evaluation<f64>,
    name: &StableName,
) -> StableName {
    let (_, names) =
        editor_core::product_named(doc, ev, Tol::witness()).expect("the product gathers");
    if names.lookup(name).is_some() {
        return name.clone();
    }
    let copies: Vec<StableName> = names
        .iter()
        .map(|(n, _)| n.clone())
        .filter(|n| n.copy_of().is_some_and(|(_, of)| of == name))
        .collect();
    let [copy] = copies.as_slice() else {
        panic!("{name:?} has one world copy, not {copies:?}");
    };
    copy.clone()
}

/// [`fixture::seat::assert_seated`] over the world names of `a` and
/// `b` ([`world_name`]).
fn assert_seated(
    doc: &ProfileDoc,
    ev: &editor_core::Evaluation<f64>,
    a: &StableName,
    b: &StableName,
    control: &Affine3<f64>,
    what: &str,
) {
    let (a, b) = (world_name(doc, ev, a), world_name(doc, ev, b));
    assert_seated_named(doc, ev, &a, &b, control, what);
}

/// [`fixture::seat::seat_map`] over the world names of `a` and `b`.
fn seat_map(
    doc: &ProfileDoc,
    ev: &editor_core::Evaluation<f64>,
    a: &StableName,
    b: &StableName,
) -> Affine3<f64> {
    seat_map_named(doc, ev, &world_name(doc, ev, a), &world_name(doc, ev, b))
}

/// The seat every row's constraint authors: the copy's top cap at its
/// centre.
const SEAT: [f64; 3] = [0.5 * TOP_WIDTH, 0.5 * TOP_WIDTH, TOP_HEIGHT];

/// The lift a rule whose copies stand clear of each other steps by.
const LIFT: f64 = 10.0;

/// A document holding the patterned instance `top`, pinned to the
/// world, and the `cap` whose bundle is empty, with their resolver.
struct Scene {
    doc: ProfileDoc,
    opts: EvalOptions,
    top: RecipeNodeId,
    cap: RecipeNodeId,
    top_ref: editor_core::DocRef,
    top_body: RecipeNodeId,
}

impl Scene {
    /// The cap's bottom cap, as the copy side of its constraint.
    fn cap_bottom(&self) -> StableName {
        in_part(self.cap, self.top_body, CapEnd::Start)
    }

    /// `name` of the top's part, as a nested copy's chain wears it.
    fn top_cap(&self) -> StableName {
        in_part(self.top, self.top_body, CapEnd::End)
    }
}

/// `top` (the tall block), instantiated on the world at the identity,
/// and `cap`, an instance of the same part placed by nothing yet.
fn scene(label: &str) -> Scene {
    let mut store = PartStore::new();
    let (top_ref, top_body) = store.insert_part(
        part_doc(&format!("{label}-top"), TOP_WIDTH, TOP_HEIGHT),
        Tol::witness(),
    );
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref.clone()));
    let (doc, cap) = insert(doc, fixture::mated_instance(top_ref.clone()));
    Scene {
        doc,
        opts,
        top,
        cap,
        top_ref,
        top_body,
    }
}

/// **The seat this fixture's alignment asks for**, measured on the
/// document that has no placer anywhere: the cap seated on the top's
/// own top cap, the relative frame a seated row must reproduce.
/// Derived, never transcribed — a constant written down here would be
/// a second statement of what the alignment says, and the two could
/// drift apart.
fn control_seat(label: &str) -> Affine3<f64> {
    let s = scene(&format!("{label}-control"));
    let (a, b) = (s.top_cap(), s.cap_bottom());
    let (doc, _) = fixture::constrain(
        s.doc,
        seat_at(fixture::head(a.clone()), fixture::head(b.clone()), SEAT),
    );
    let ev = run(&doc, &s.opts);
    seat_map(&doc, &ev, &a, &b)
}

/// A linear pattern node over `input`.
fn linear(input: RecipeNodeId, dir: [f64; 3], spacing: f64, count: i64) -> AuthoredNode {
    Node::Pattern {
        input: input.into(),
        count: Formula::count(count),
        kind: PatternKind::Linear {
            direction: dir.map(scl),
            spacing: len(spacing),
        },
    }
}

/// A `Part` selecting instance `i` of `of` — the identity-transparent
/// projection a nested pattern is built through.
fn part_of(of: RecipeNodeId, i: i64) -> AuthoredNode {
    Node::Part {
        of: of.into(),
        select: PartSelect::Instance(Formula::count(i)),
    }
}

/// A `Datum::Axis` parallel to `+z` through `(x, y, 0)`.
fn axis_z(doc: ProfileDoc, at: [f64; 2]) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Datum(Datum::Axis {
            origin: [len(at[0]), len(at[1]), len(0.0)],
            direction: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    )
}

/// A circular pattern about a `+z` axis through `(x, y)` — the rule
/// whose copy map ROTATES, which is what makes a chain's two levels
/// fail to commute.
fn circular(
    doc: ProfileDoc,
    input: RecipeNodeId,
    at: [f64; 2],
    step_angle: f64,
    count: i64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, axis) = axis_z(doc, at);
    insert(
        doc,
        Node::Pattern {
            input: input.into(),
            count: Formula::count(count),
            kind: PatternKind::Circular {
                axis: axis.into(),
                step: ang(step_angle),
            },
        },
    )
}

/// A quarter turn — the step every rotating rule here takes.
const Q: f64 = std::f64::consts::FRAC_PI_2;

/// **The cap seated and in the world**: the solve records no fault for
/// its placement, its copy is of the world's kind, and the product
/// gathers.
fn assert_placed(doc: &ProfileDoc, opts: &EvalOptions, cap: RecipeNodeId, what: &str) {
    let poses = solve(doc, opts, Tol::witness());
    assert!(
        poses.fault(cap).is_none(),
        "{what}: the solve refused: {:?}",
        poses.fault(cap)
    );
    let ev = run(doc, opts);
    assert_eq!(
        ev.space(cap),
        Space::World,
        "{what}: the cap is in the world"
    );
    assert!(
        product(doc, &ev, Tol::witness()).is_ok(),
        "{what}: the product gathers"
    );
}

// ---- A1: a nested copy seats at the composed pose ----

/// **A1.** `Pattern` over `Part { Instance(1) }` over `Pattern`, the
/// target read at the outer pattern under the name a nested copy
/// actually wears — `Instance { 1, of: Instance { 1, of: … } }`. The
/// cap seats on that copy at `M₂(1) ∘ M₁(1) ∘ seat`, measured on the
/// product's own face frames against the placer-free control.
///
/// **The two maps do not commute**, which is what makes the row an
/// assertion about the ORDER and not only about the pair: the inner
/// rule translates along `+x` and the outer one is a quarter turn
/// about the document's `z` axis, so `M₂(1) ∘ M₁(1)` carries the
/// master's origin to `(0, 4)` where `M₁(1) ∘ M₂(1)` carries it to
/// `(4, 0)`. A solve that folded the chain the other way round would
/// place the cap somewhere the named copy is not, and the whole-frame
/// oracle sees it.
#[test]
fn a1_a_nested_copy_seats_at_the_composed_pose() {
    let control = control_seat("msolve2-a1");
    let s = scene("msolve2-a1");
    let top = s.top;
    let (doc, inner) = insert(s.doc.clone(), linear(top, [1.0, 0.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(inner, 1));
    let (doc, axis) = insert(
        doc,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, outer) = insert(
        doc,
        Node::Pattern {
            input: part.into(),
            count: Formula::count(2),
            kind: PatternKind::Circular {
                axis: axis.into(),
                step: ang(std::f64::consts::FRAC_PI_2),
            },
        },
    );
    let doc = world(doc, &[], &[(outer, 2)]);
    let a = in_copy(outer, 1, in_copy(inner, 1, s.top_cap()));
    let b = s.cap_bottom();
    let (doc, _) = fixture::constrain(
        doc,
        seat_at(
            fixture::head_at(outer, a.clone()),
            fixture::head(b.clone()),
            SEAT,
        ),
    );

    // The member the reference resolves to, read at the door the
    // solve reads: the chain is OUTERMOST first and the instance is
    // the one that minted the name.
    let member =
        member_of(&doc, &fixture::head_at(outer, a.clone())).expect("a nested copy is a member");
    assert_eq!(member.instance, top, "the member stands on the instance");
    assert_eq!(
        member.copy(),
        vec![(outer, 1), (inner, 1)],
        "the copy chain is outermost first"
    );
    assert_eq!(
        member.chain.first().map(|p| p.node()),
        Some(outer),
        "the placing chain starts at the outermost placer"
    );

    assert_placed(&doc, &s.opts, s.cap, "A1");
    let ev = run(&doc, &s.opts);
    assert_seated(&doc, &ev, &a, &b, &control, "A1 nested copy");
}

// ---- A2: sibling copies are distinct members, at each level ----

/// **Two siblings are two members.** A bundle relates its copy to ONE
/// target member, so a second constraint whose target is a sibling of
/// the first's refuses at the door as a second target
/// (`MateFault::TargetsDiffer`), naming the bundle the first holds. A
/// sibling the member key confused with the first would instead meet
/// the fold, which already pins the cap, and refuse
/// `Overconstrained` — the refusal A3(b) reads for one copy spelled
/// two ways.
fn assert_siblings_distinct(
    doc: &ProfileDoc,
    cap: RecipeNodeId,
    first: MateRef,
    second: Mate<Formula>,
    what: &str,
) {
    match at_the_door(doc, &RefusingReach, second) {
        Err((
            at,
            MateFault::TargetsDiffer {
                placement,
                constraints,
            },
        )) => {
            assert_eq!((at, placement), (cap, cap), "{what}: the cap's bundle");
            assert_eq!(
                constraints,
                vec![first.id],
                "{what}: the bundle as it stands"
            );
        }
        other => panic!("{what}: a sibling is a second target, not {other:?}"),
    }
}

/// **A2(b).** Two constraints onto sibling copies of the OUTER pattern
/// — the same inner copy, outer copies 0 and 1 of one chain. The two
/// members differ only in the outer index, which is enough to make
/// them different targets.
#[test]
fn a2b_sibling_outer_copies_are_distinct_members() {
    let s = scene("msolve2-a2b");
    let (doc, inner) = insert(s.doc.clone(), linear(s.top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(inner, 1));
    let (doc, outer) = insert(doc, linear(part, [0.0, 1.0, 0.0], 4.0, 2));
    let doc = world(doc, &[], &[(outer, 2)]);
    let named = |i2: u32| in_copy(outer, i2, in_copy(inner, 1, s.top_cap()));
    let cap = fixture::head(s.cap_bottom());
    let (doc, m1) = fixture::constrain(
        doc,
        seat_at(fixture::head_at(outer, named(0)), cap.clone(), SEAT),
    );
    assert_placed(&doc, &s.opts, s.cap, "A2(b) first");
    assert_siblings_distinct(
        &doc,
        s.cap,
        m1,
        seat_at(fixture::head_at(outer, named(1)), cap, SEAT),
        "A2(b)",
    );
}

/// **A2(a).** Two constraints onto sibling copies of the INNER pattern.
///
/// A pattern takes ONE body, so an outer pattern stands over exactly
/// one `Part` and one inner copy: two inner siblings are therefore
/// two `Part`s under two outer patterns, and what the row holds
/// constant is the outer INDEX.
///
/// **Say plainly what the key compares.** The chains are
/// `[(outer_a, 0), (inner, 2)]` and `[(outer_b, 0), (inner, 1)]`, and
/// a lexicographic comparison reaches the outer NODE ids first — so
/// what separates these two members is the outer pattern node, not
/// the inner index, and the inner index is never reached. What the
/// row therefore shows is that the chain distinguishes members whose
/// difference is at the inner level; the row that makes an INDEX
/// itself the deciding field is `a2b`, whose two members share one
/// chain and differ only in an index.
#[test]
fn a2a_sibling_inner_copies_are_distinct_members() {
    let s = scene("msolve2-a2a");
    let (doc, inner) = insert(s.doc.clone(), linear(s.top, [0.0, -1.0, 0.0], 4.0, 3));
    let (doc, part_a) = insert(doc, part_of(inner, 2));
    let (doc, outer_a) = insert(doc, linear(part_a, [0.0, 0.0, 1.0], LIFT, 2));
    let (doc, part_b) = insert(doc, part_of(inner, 1));
    let (doc, outer_b) = insert(doc, linear(part_b, [0.0, 0.0, 1.0], LIFT, 2));
    let doc = world(doc, &[], &[(outer_a, 2), (outer_b, 2)]);
    let cap = fixture::head(s.cap_bottom());
    let (doc, m1) = fixture::constrain(
        doc,
        seat_at(
            fixture::head_at(outer_a, in_copy(outer_a, 0, in_copy(inner, 2, s.top_cap()))),
            cap.clone(),
            SEAT,
        ),
    );
    assert_placed(&doc, &s.opts, s.cap, "A2(a) first");
    assert_siblings_distinct(
        &doc,
        s.cap,
        m1,
        seat_at(
            fixture::head_at(outer_b, in_copy(outer_b, 0, in_copy(inner, 1, s.top_cap()))),
            cap,
            SEAT,
        ),
        "A2(a)",
    );
}

/// **A2(c).** Two constraints onto copies differing at BOTH levels —
/// `(0, 1)` and `(1, 2)` — which is the case a chain compared
/// lexicographically has to get right at more than its first element.
#[test]
fn a2c_copies_differing_at_both_levels_are_distinct_members() {
    let s = scene("msolve2-a2c");
    let (doc, inner) = insert(s.doc.clone(), linear(s.top, [0.0, -1.0, 0.0], 4.0, 3));
    let (doc, part_a) = insert(doc, part_of(inner, 1));
    let (doc, outer_a) = insert(doc, linear(part_a, [0.0, 0.0, 1.0], LIFT, 2));
    let (doc, part_b) = insert(doc, part_of(inner, 2));
    let (doc, outer_b) = insert(doc, linear(part_b, [0.0, 1.0, 0.0], 8.0, 2));
    let doc = world(doc, &[], &[(outer_a, 2), (outer_b, 2)]);
    let cap = fixture::head(s.cap_bottom());
    let (doc, m1) = fixture::constrain(
        doc,
        seat_at(
            fixture::head_at(outer_a, in_copy(outer_a, 0, in_copy(inner, 1, s.top_cap()))),
            cap.clone(),
            SEAT,
        ),
    );
    assert_placed(&doc, &s.opts, s.cap, "A2(c) first");
    assert_siblings_distinct(
        &doc,
        s.cap,
        m1,
        seat_at(
            fixture::head_at(outer_b, in_copy(outer_b, 1, in_copy(inner, 2, s.top_cap()))),
            cap,
            SEAT,
        ),
        "A2(c)",
    );
}

/// **A2′.** A target whose member carries a non-identity map at the
/// OUTER level seats, and its sibling is a distinct member.
///
/// A2(a)–(c) give their first target outer copy 0, whose map is the
/// identity by the stepped rule's construction — so the pose the fold
/// composes never has an outer map in it. Here the first target is
/// outer copy 1 of a quarter turn about a `+z` axis at `(-2, 5)`: the
/// cap seats on it (measured on the product against the placer-free
/// control), and a second constraint onto copy 2 is a second target.
#[test]
fn a2d_a_rotating_outer_map_seats_and_its_sibling_is_distinct() {
    let control = control_seat("msolve2-a2d");
    let s = scene("msolve2-a2d");
    let (doc, inner) = insert(s.doc.clone(), linear(s.top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(inner, 1));
    let (doc, outer) = circular(doc, part, [-2.0, 5.0], Q, 3);
    let doc = world(doc, &[], &[(outer, 3)]);
    let named = |i2: u32| in_copy(outer, i2, in_copy(inner, 1, s.top_cap()));
    let b = s.cap_bottom();
    let (doc, m1) = fixture::constrain(
        doc,
        seat_at(
            fixture::head_at(outer, named(1)),
            fixture::head(b.clone()),
            SEAT,
        ),
    );
    assert_placed(&doc, &s.opts, s.cap, "A2′");
    // The first target's own copy — the one whose chain carries the
    // quarter turn — seats where the alignment asks.
    let ev = run(&doc, &s.opts);
    assert_seated(&doc, &ev, &named(1), &b, &control, "A2′ rotating outer map");
    assert_siblings_distinct(
        &doc,
        s.cap,
        m1,
        seat_at(fixture::head_at(outer, named(2)), fixture::head(b), SEAT),
        "A2′",
    );
}

// ---- A3: the `Part` in the walk ----

/// **A3(a).** A `Part`-selected copy read AT the `Part`, with no
/// outer pattern anywhere: the walk passes the `Part` contributing
/// nothing, consumes the pattern's `Instance(1)` qualifier, and the
/// cap seats. The `Part` is what the world places here, so what the
/// row measures is the body a consumer gathers through that door.
#[test]
fn a3a_a_part_selected_copy_read_at_the_part_is_a_member() {
    let control = control_seat("msolve2-a3a");
    let s = scene("msolve2-a3a");
    let top = s.top;
    let (doc, pattern) = insert(s.doc.clone(), linear(top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(pattern, 1));
    let doc = world(doc, &[part], &[]);
    let a = in_copy(pattern, 1, s.top_cap());
    let b = s.cap_bottom();
    let (doc, _) = fixture::constrain(
        doc,
        seat_at(
            fixture::head_at(part, a.clone()),
            fixture::head(b.clone()),
            SEAT,
        ),
    );
    let member = member_of(&doc, &fixture::head_at(part, a.clone())).expect("a member");
    assert_eq!(member.instance, top);
    assert_eq!(
        member.copy(),
        vec![(pattern, 1)],
        "one level, through the Part"
    );
    assert_eq!(
        member.chain.len(),
        1,
        "the Part places nothing: the pattern's copy is the whole chain"
    );
    assert_placed(&doc, &s.opts, s.cap, "A3(a)");
    let ev = run(&doc, &s.opts);
    assert_seated(&doc, &ev, &a, &b, &control, "A3(a) part-selected copy");
    assert!(
        gate(&doc, &ev).is_ok(),
        "A3(a): the gate refused: {:?}",
        gate(&doc, &ev).err()
    );
}

/// **A3(b).** One copy, two bodies read: a target read at the pattern
/// under `Instance(1)` and a target read at a `Part` selecting that
/// same instance are one placement spelled two ways, so one member:
/// the `Part` places nothing, and both walks pass the same copy. So a
/// second constraint onto it is no second target: it meets the fold,
/// which the first constraint already pins, and refuses as a mate
/// beyond the pin.
#[test]
fn a3b_two_operands_over_one_copy_are_one_member() {
    let s = scene("msolve2-a3b");
    let (doc, pattern) = insert(s.doc.clone(), linear(s.top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(pattern, 1));
    let doc = world(doc, &[part], &[]);
    let a = in_copy(pattern, 1, s.top_cap());
    let at_pattern = fixture::head_at(pattern, a.clone());
    let at_part = fixture::head_at(part, a.clone());
    let cap = fixture::head(s.cap_bottom());
    let (doc, _) = fixture::constrain(doc, seat_at(at_pattern.clone(), cap.clone(), SEAT));
    let (mp, mq) = (
        member_of(&doc, &at_pattern).expect("a member at the pattern"),
        member_of(&doc, &at_part).expect("a member at the Part"),
    );
    assert_eq!(mp, mq, "one instance, one placing chain — one member");
    assert_placed(&doc, &s.opts, s.cap, "A3(b)");
    let reach = editor_core::mate_reach::<f64>(&s.opts, Tol::witness());
    match at_the_door(&doc, &reach, seat_at(at_part, cap, SEAT)) {
        Err((at, MateFault::Overconstrained { placement, .. })) => {
            assert_eq!((at, placement), (s.cap, s.cap));
        }
        other => panic!("A3(b): the same member, already pinned, not {other:?}"),
    }
}

/// **A3(c).** `Transform` over `Part { Instance(1) }` over a pattern,
/// the target read at the TRANSFORM: the walk composes the transform's
/// map onto the copy's, passing the `Part` in between, and the cap
/// seats. This is the shape MSOLVE-1 could not build — a transform
/// over ONE copy of a pattern — because nothing then projected a
/// single body out of a pattern's instances.
#[test]
fn a3c_transform_over_part_over_a_pattern_seats() {
    let control = control_seat("msolve2-a3c");
    let s = scene("msolve2-a3c");
    let (doc, pattern) = insert(s.doc.clone(), linear(s.top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(pattern, 1));
    let (doc, moved) = insert(
        doc,
        xform(
            part,
            [2.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            std::f64::consts::FRAC_PI_2,
        ),
    );
    let doc = world(doc, &[moved], &[]);
    let a = in_copy(pattern, 1, s.top_cap());
    let b = s.cap_bottom();
    let (doc, _) = fixture::constrain(
        doc,
        seat_at(
            fixture::head_at(moved, a.clone()),
            fixture::head(b.clone()),
            SEAT,
        ),
    );
    let member = member_of(&doc, &fixture::head_at(moved, a.clone())).expect("a member");
    assert_eq!(
        member.copy(),
        vec![(pattern, 1)],
        "the transform contributes no copy — only the pattern does"
    );
    assert_eq!(member.chain.first().map(|p| p.node()), Some(moved));
    assert_placed(&doc, &s.opts, s.cap, "A3(c)");
    let ev = run(&doc, &s.opts);
    assert_seated(&doc, &ev, &a, &b, &control, "A3(c) transform over part");
    assert!(
        gate(&doc, &ev).is_ok(),
        "A3(c): the gate refused: {:?}",
        gate(&doc, &ev).err()
    );
}

// ---- A4: the `Part`'s index, checked against the name ----

/// **A4.** A `Part` whose index expression evaluates to a copy the
/// reference's NAME does not name refuses typed, and the refusal
/// carries both numbers.
///
/// The name is the authority: a document where the two disagree would
/// be PLACED by the name (which says which copy's master pose the
/// alignment is authored against) and GATHERED by the `Part` (which
/// says which body exists at all). The solve refuses rather than
/// choosing, and the check runs where the offset already evaluates
/// the pattern's own structural slots at the document's bindings —
/// never in the walk, which decides admission on node kinds and name
/// segments alone.
#[test]
fn a4_a_part_that_selects_another_copy_refuses_typed() {
    let s = scene("msolve2-a4");
    let (doc, pattern) = insert(s.doc.clone(), linear(s.top, [0.0, -1.0, 0.0], 4.0, 3));
    // The Part selects copy 2; the name below says copy 1.
    let (doc, part) = insert(doc, part_of(pattern, 2));
    let a = in_copy(pattern, 1, s.top_cap());
    let reference = fixture::head_at(part, a.clone());
    // ADMISSION is structural and evaluates nothing, so the walk
    // still resolves this reference to a member: the disagreement is
    // a fact about two numbers, which the per-reference check reads
    // — and that check is the constraint's own, so the constraint
    // door asks it where the constraint is authored and refuses with
    // the solve's fault.
    assert!(
        member_of(&doc, &reference).is_some(),
        "the walk admits the reference; the number check is what refuses"
    );
    let (at_placement, fault) = at_the_door(
        &doc,
        &RefusingReach,
        seat_at(reference.clone(), fixture::head(s.cap_bottom()), SEAT),
    )
    .expect_err("a disagreeing Part refuses");
    let MateFault::PartSelectsAnotherCopy {
        mate: at,
        side,
        part: named_part,
        named,
        selected,
    } = &fault
    else {
        panic!("expected PartSelectsAnotherCopy, got {fault:?}");
    };
    assert_eq!(at_placement, s.cap, "the door names the cap's placement");
    assert_eq!(
        (at.placement, *side, *named_part),
        (s.cap, MateSide::A, part)
    );
    assert_eq!((*named, *selected), (1, 2), "both indices are reported");
    // ONE word for the index on both sides of the sentence: a reader
    // comparing "copy 1" with "instance 2" has to be told they are
    // the same kind of number.
    let said = fault.to_string();
    assert!(
        said.contains("names copy 1") && said.contains("selects copy 2"),
        "the message names both indices in one word: {said}"
    );
}

// ---- the gate on a nested document ----

/// **What the at-rest gate says for a constraint read BELOW the outer
/// pattern** — on a nested document. The SOLVE places the cap on such
/// a target; the gate mints nothing for it, because the side reads the
/// `Part`, which no placement places — the outer pattern's copies are
/// what the world holds. Reading the same document's target AT the
/// outer pattern (every other row here) holds, because there the name
/// is a placed copy's own row.
#[test]
fn the_gate_on_a_constraint_read_below_the_outer_pattern_mints_nothing() {
    let s = scene("msolve2-gate");
    let (doc, inner) = insert(s.doc.clone(), linear(s.top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(inner, 1));
    let (doc, outer) = insert(doc, linear(part, [0.0, 0.0, 1.0], LIFT, 2));
    let doc = world(doc, &[], &[(outer, 2)]);
    // Read at the `Part`, BELOW the outer pattern: the name stops at
    // the inner pattern's own `Instance(1)` row.
    let a = in_copy(inner, 1, s.top_cap());
    let (doc, mate) = fixture::constrain(
        doc,
        seat_at(
            fixture::head_at(part, a),
            fixture::head(s.cap_bottom()),
            SEAT,
        ),
    );
    assert_placed(
        &doc,
        &s.opts,
        s.cap,
        "a target read below the outer pattern",
    );
    let ev = run(&doc, &s.opts);
    // No placement reads the `Part` the target is read at, so the
    // constraint has no world copy to mint on.
    fixture::assert_mints_nothing(&doc, &ev, mate);
}

// ---- A1′: the chain, deeper and with transforms interleaved ----

/// **A1′(a).** `Transform` over `Part(P₂, 2)` over `P₂` (linear)
/// over `Transform` over `Part(P₁, 1)` over `P₁` (circular) over the
/// instance, the target read at the outermost transform.
///
/// Every map in the chain is non-identity and no adjacent pair
/// commutes, so the seat holds only if the fold composes all four
/// outermost-first onto the target's pose. A2's rows fold at most
/// two; this one folds a chain that alternates the two placer kinds.
#[test]
fn a1b_two_levels_with_transforms_between_and_above_seat() {
    let control = control_seat("msolve2-a1b");
    let s = scene("msolve2-a1b");
    let top = s.top;
    let (doc, p1) = circular(s.doc.clone(), top, [0.0, 0.0], Q, 4);
    let (doc, part1) = insert(doc, part_of(p1, 1));
    let (doc, t_mid) = insert(doc, xform(part1, [2.0, 0.0, 0.0], [0.0, 0.0, 1.0], Q));
    let (doc, p2) = insert(doc, linear(t_mid, [1.0, 0.0, 0.0], 4.0, 3));
    let (doc, part2) = insert(doc, part_of(p2, 2));
    let (doc, t_top) = insert(doc, xform(part2, [0.0, 3.0, 0.0], [0.0, 0.0, 1.0], Q));
    let doc = world(doc, &[t_top], &[]);
    let a = in_copy(p2, 2, in_copy(p1, 1, s.top_cap()));
    let b = s.cap_bottom();
    let r = fixture::head_at(t_top, a.clone());
    let (doc, _) = fixture::constrain(doc, seat_at(r.clone(), fixture::head(b.clone()), SEAT));
    let m = member_of(&doc, &r).expect("a member through both transforms");
    assert_eq!(
        (m.instance, m.chain.first().map(|p| p.node())),
        (top, Some(t_top))
    );
    assert_eq!(
        m.copy(),
        vec![(p2, 2), (p1, 1)],
        "the transforms contribute no copy — only the patterns do"
    );
    assert_placed(&doc, &s.opts, s.cap, "A1′(a)");
    let ev = run(&doc, &s.opts);
    assert_seated(
        &doc,
        &ev,
        &a,
        &b,
        &control,
        "A1′(a) two levels + transforms",
    );
}

/// **A1′(b).** Three levels — circular, linear, circular — each with
/// a non-identity map, the target read at the outermost pattern. The
/// member's chain is three pairs long and the fold composes three
/// maps.
#[test]
fn a1c_three_levels_deep_seat() {
    let control = control_seat("msolve2-a1c");
    let s = scene("msolve2-a1c");
    let (doc, p1) = circular(s.doc.clone(), s.top, [0.0, 0.0], Q, 4);
    let (doc, part1) = insert(doc, part_of(p1, 1));
    let (doc, p2) = insert(doc, linear(part1, [1.0, 0.0, 0.0], 4.0, 2));
    let (doc, part2) = insert(doc, part_of(p2, 1));
    let (doc, p3) = circular(doc, part2, [5.0, 5.0], std::f64::consts::FRAC_PI_3, 4);
    let doc = world(doc, &[], &[(p3, 4)]);
    let a = in_copy(p3, 2, in_copy(p2, 1, in_copy(p1, 1, s.top_cap())));
    let b = s.cap_bottom();
    let r = fixture::head_at(p3, a.clone());
    let (doc, _) = fixture::constrain(doc, seat_at(r.clone(), fixture::head(b.clone()), SEAT));
    let m = member_of(&doc, &r).expect("a member three levels down");
    assert_eq!(m.copy(), vec![(p3, 2), (p2, 1), (p1, 1)]);
    assert_placed(&doc, &s.opts, s.cap, "A1′(b)");
    let ev = run(&doc, &s.opts);
    assert_seated(&doc, &ev, &a, &b, &control, "A1′(b) three levels");
}

// ---- A4′: the check is per reference ----

/// **A4′(a) — a `Part` mismatch is checked per reference.**
///
/// Two `Part`s over one pattern: the cap is seated on `part1`'s copy
/// (selects 0, names 0); a second cap is seated on a transform over
/// `part2` (selects 2 once re-pointed, NAMES 0) — a different member,
/// the transform on its chain. The well-formed constraint is
/// untouched, and the second refuses at the solve: placed by the name,
/// gathered by the `Part`, it would seat a body on a copy the `Part`
/// does not gather.
#[test]
fn a4b_a_part_mismatch_is_checked_per_reference() {
    let s = scene("msolve2-a4b");
    let (doc, cap2) = insert(s.doc.clone(), fixture::mated_instance(s.top_ref.clone()));
    let (doc, pattern) = insert(doc, linear(s.top, [0.0, -1.0, 0.0], 4.0, 3));
    let (doc, part1) = insert(doc, part_of(pattern, 0));
    // `part2` agrees with the name at insert — the constraint door
    // refuses a disagreeing `Part` where the constraint is authored —
    // and is re-pointed at copy 2 below, which is how a `Part` comes to
    // disagree after insert.
    let (doc, part2) = insert(doc, part_of(pattern, 0));
    let (doc, moved) = insert(doc, xform(part2, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0], 0.0));
    let a = in_copy(pattern, 0, s.top_cap());
    let (doc, _) = fixture::constrain(
        doc,
        seat_at(
            fixture::head_at(part1, a.clone()),
            fixture::head(s.cap_bottom()),
            SEAT,
        ),
    );
    let (doc, _) = fixture::constrain(
        doc,
        seat_at(
            fixture::head_at(moved, a.clone()),
            fixture::head(in_part(cap2, s.top_body, CapEnd::Start)),
            SEAT,
        ),
    );
    assert_ne!(
        member_of(&doc, &fixture::head_at(part1, a.clone())),
        member_of(&doc, &fixture::head_at(moved, a.clone())),
        "the premise: two placements, two members"
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetStructuralParam {
            node: part2,
            slot: editor_core::SlotId::Instance,
            expr: Formula::count(2),
            fresh: Vec::new(),
        },
    );
    // Both references are members: admission is structural and the
    // disagreement is about two numbers.
    assert!(member_of(&doc, &fixture::head_at(moved, a)).is_some());
    let poses = solve(&doc, &s.opts, Tol::witness());
    assert!(
        poses.fault(s.cap).is_none(),
        "the well-formed constraint is untouched: {:?}",
        poses.fault(s.cap)
    );
    let fault = poses
        .fault(cap2)
        .cloned()
        .expect("the second reference's Part is checked too");
    let MateFault::PartSelectsAnotherCopy {
        part,
        named,
        selected,
        ..
    } = fault
    else {
        panic!("expected PartSelectsAnotherCopy, got {fault:?}");
    };
    assert_eq!((part, named, selected), (part2, 0, 2));
}

/// **A4′(b) — the index is EVALUATED, at the document's bindings.**
/// The `Part`'s index is a document parameter: at `k = 1` it agrees
/// with the name and the cap seats; setting `k = 2` refuses the same
/// constraint typed, naming both indices — and the MEMBER is unchanged
/// across the edit, because admission is structural and a parameter
/// is not part of it.
#[test]
fn a4c_the_part_index_is_evaluated_at_the_documents_bindings() {
    let control = control_seat("msolve2-a4c");
    let s = scene("msolve2-a4c");
    let k = VarName::from_static("k");
    let (doc, _) = step(
        s.doc.clone(),
        DocEdit::DeclareVar {
            name: k.clone(),
            def: editor_core::VarDecl::Free(FreeVar::Count { value: 1 }),
        },
    );
    let (doc, pattern) = insert(doc, linear(s.top, [0.0, -1.0, 0.0], 4.0, 3));
    let (doc, part) = insert(
        doc,
        Node::Part {
            of: pattern.into(),
            select: PartSelect::Instance(Formula::named(k.clone(), Dimension::Count)),
        },
    );
    let doc = world(doc, &[part], &[]);
    let a = in_copy(pattern, 1, s.top_cap());
    let b = s.cap_bottom();
    let r = fixture::head_at(part, a.clone());
    let (doc, _) = fixture::constrain(doc, seat_at(r.clone(), fixture::head(b.clone()), SEAT));
    let before = member_of(&doc, &r).expect("a member at k = 1");
    assert_placed(&doc, &s.opts, s.cap, "A4′(b) at k = 1");
    let ev = run(&doc, &s.opts);
    assert_seated(&doc, &ev, &a, &b, &control, "A4′(b) at k = 1");

    let (doc, _) = step(
        doc,
        DocEdit::SetVarValue {
            var: k.into(),
            value: FreeValue::Count(2),
        },
    );
    let after = member_of(&doc, &r).expect("still a member at k = 2");
    assert_eq!(
        before, after,
        "the member does not move with a parameter: admission is structural"
    );
    let fault = solve(&doc, &s.opts, Tol::witness())
        .fault(s.cap)
        .cloned()
        .expect("the disagreement refuses at k = 2");
    let MateFault::PartSelectsAnotherCopy {
        part: p,
        named,
        selected,
        side,
        ..
    } = fault
    else {
        panic!("expected PartSelectsAnotherCopy, got {fault:?}");
    };
    assert_eq!((p, named, selected, side), (part, 1, 2, MateSide::A));
}

// ---- the walk's fences, on the shapes `Part` brings ----

/// **A `Part` over a pattern the NAME does not continue into stops
/// the walk.** The `Part` is pose-neutral and passes every name
/// through, but it passes through ITS OWN input's names: a name whose
/// next segment belongs to a different pattern reaches that pattern's
/// input node under a name headed elsewhere, and no member stands
/// there.
#[test]
fn a3d_a_part_over_the_wrong_pattern_stops_the_walk() {
    let s = scene("msolve2-a3d");
    let (doc, inner_a) = insert(s.doc.clone(), linear(s.top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, inner_b) = insert(doc, linear(s.top, [1.0, 0.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(inner_b, 1));
    let (doc, outer) = insert(doc, linear(part, [0.0, 0.0, 1.0], LIFT, 2));
    // The name continues into `inner_a`; the chain runs through
    // `inner_b`.
    let a = in_copy(outer, 1, in_copy(inner_a, 1, s.top_cap()));
    let r = fixture::head_at(outer, a);
    assert!(member_of(&doc, &r).is_none(), "no member stands there");
    let fault = door_refusal(&doc, seat_at(r, fixture::head(s.cap_bottom()), SEAT));
    assert!(
        matches!(fault, MateFault::DanglingHead { head, .. } if head == inner_b),
        "the walk stops at the pattern it actually reached: {fault:?}"
    );
}

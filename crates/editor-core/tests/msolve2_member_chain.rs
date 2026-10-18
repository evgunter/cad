//! **Nested copies as a mate's target** — `Pattern` over `Part` over
//! `Pattern`, read where the copy stands.
//!
//! A pattern's value is many bodies and a pattern's input is one, so
//! a pattern OVER a pattern does not evaluate. The nested shape a
//! user can build runs through `Node::Part { select: Instance(i) }`,
//! which projects one body out of the instances carrying every name
//! VERBATIM. A mate's target is a pose read off geometry of the space
//! the copy joins: read at a nested copy, under the name that copy
//! wears, it is that copy's face where the chain of placers put it —
//! the evaluation of each placer composes the chain, and the fold
//! reads the result. A pose is read off one body, so a target on a
//! pattern's copy is read at a `Part` (or a placer over one) that
//! projects it.
//!
//! Every row seats a `cap` instance, whose bundle the constraints
//! join, on a nested copy of a `top` instance pinned to the world: the
//! cap's bottom cap rests on the copy's top cap, and two of its lateral
//! edges are coaxial with two of the copy's, each slid half the block's
//! width across the copy so the cap overhangs it — three mates that pin
//! the cap with no value. Every row goes through ordinary doors —
//! `DocEdit::InsertNode`, `DocEdit::AddConstraint`, `solve_document`,
//! `evaluate`, `product`, `assemble` — and the seating rows measure the
//! PRODUCT's own face frames with the shared whole-frame oracle
//! (`fixture::seat`) against a control seated on the top itself, never
//! a solved frame read by eye.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::AuthoredNode;

use editor_core::pose::{PoseDef, RoleRead};
use editor_core::{
    CapEnd, ContactClass, Dimension, DocEdit, DocumentId, EvalOptions, Evaluation, Formula,
    FreeValue, FreeVar, Mate, MateFault, MateRef, MateSide, NameRef, Node, NodeErrorKind,
    NodeResult, Operand, PartSelect, PatternKind, ProfileDoc, RecipeNodeId, RoleSeg, SitedRef,
    Space, StableName, VarName, product,
};
use fixture::resolver::{PartStore, in_part, placement_of, with_resolver};
use fixture::seat::{assert_seated as assert_seated_named, seat_map as seat_map_named};
use fixture::{ang, gate, in_copy, insert, len, on_frame, run, scl, solve, step};
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
        },
    )
}

/// **`name`, an entity of the part's body, as `instance` wears it**:
/// under the placement the part's world copies the body by, inside the
/// instance's `InPart` wrapper.
fn worn(instance: RecipeNodeId, body: RecipeNodeId, name: StableName) -> StableName {
    let placed = StableName {
        kind: name.kind,
        node: placement_of(body),
        path: vec![RoleSeg::Placed {
            of: NameRef::new(name),
        }],
    };
    StableName {
        kind: placed.kind,
        node: instance,
        path: vec![RoleSeg::InPart { of: placed.into() }],
    }
}

/// The part's own names for what a seat reads: two lateral edges, at
/// the profile's first two vertices, and the rim edges of the first
/// two profile segments, whose lines run along the block's two sides.
#[derive(Clone)]
struct PartNames {
    lateral: [StableName; 2],
    rim: [StableName; 2],
}

/// **The world the product is** (A10): each of `bodies` placed, then
/// each `(pattern, count)`'s copies placed through a `Part` per copy,
/// each pinned at the identity over its own geometry. The instances
/// are placements of their own and are never placed again here.
fn world(
    doc: ProfileDoc,
    opts: &EvalOptions,
    bodies: &[RecipeNodeId],
    patterns: &[(RecipeNodeId, i64)],
) -> ProfileDoc {
    let place = |doc: ProfileDoc, body: RecipeNodeId| {
        let read = fixture::body_read(&doc, body.into());
        let (doc, placement) = insert(doc, Node::place(vec![editor_core::BodyRead::plain(read)]));
        fixture::pinned(doc, placement, opts)
    };
    let doc = bodies.iter().fold(doc, |doc, &body| place(doc, body));
    patterns.iter().fold(doc, |doc, &(pattern, count)| {
        (0..count).fold(doc, |doc, i| {
            let (doc, copy) = insert(doc, part_of(pattern, i));
            place(doc, copy)
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

/// **The axis parallel to `+z` through `(x, y)`**, read off the top's
/// geometry: its lateral edge at the profile's first vertex (the
/// origin), slid `x` along its first rim line and `y` along its second.
fn axis_z(s: &Scene, at: [f64; 2]) -> Operand {
    let read = |name: &StableName| SitedRef::new(s.top, s.on_top(name));
    let mut axis = fixture::axis(read(&s.names.lateral[0]));
    for (rim, by) in s.names.rim.iter().zip(at) {
        if by != 0.0 {
            axis = Operand::Pose(Box::new(PoseDef::Slide {
                pose: RoleRead::plain(axis),
                along: RoleRead::plain(fixture::axis(read(rim))),
                by: len(by),
            }));
        }
    }
    axis
}

/// A circular pattern about `axis` — the rule whose copy map ROTATES,
/// which is what makes a chain's two levels fail to commute.
fn circular(
    doc: ProfileDoc,
    input: RecipeNodeId,
    axis: Operand,
    step_angle: f64,
    count: i64,
) -> (ProfileDoc, RecipeNodeId) {
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

/// Half the block's width: how far each target edge is slid across the
/// copy, so the cap overhangs it.
const OVERHANG: f64 = 0.5 * TOP_WIDTH;

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
    names: PartNames,
}

impl Scene {
    /// The cap's bottom cap, as the copy side of its rest.
    fn cap_bottom(&self) -> StableName {
        in_part(self.cap, self.top_body, CapEnd::Start)
    }

    /// The top's top cap, as a nested copy's chain wears it.
    fn top_cap(&self) -> StableName {
        in_part(self.top, self.top_body, CapEnd::End)
    }

    /// `name`, an entity of the top's part, as the top wears it.
    fn on_top(&self, name: &StableName) -> StableName {
        worn(self.top, self.top_body, name.clone())
    }

    /// **The seat of `cap` on the copy read at `at` under `wrap`**:
    /// the cap's bottom cap resting on the copy's top cap, and its two
    /// lateral edges coaxial with the copy's, each of those slid
    /// [`OVERHANG`] along both rim lines — `wrap` spells a name of the
    /// top as the copy read at `at` wears it.
    fn seat(
        &self,
        cap: RecipeNodeId,
        at: RecipeNodeId,
        wrap: &dyn Fn(StableName) -> StableName,
    ) -> Vec<Mate<Formula>> {
        let read = |name: &StableName| SitedRef::new(at, wrap(self.on_top(name)));
        let slid = |pose: Operand, along: &StableName| {
            Operand::Pose(Box::new(PoseDef::Slide {
                pose: RoleRead::plain(pose),
                along: RoleRead::plain(fixture::axis(read(along))),
                by: len(OVERHANG),
            }))
        };
        let mut mates = vec![fixture::rest(
            fixture::head(in_part(cap, self.top_body, CapEnd::Start)),
            fixture::head_at(at, wrap(self.top_cap())),
            ContactClass::Rest,
        )];
        for lateral in &self.names.lateral {
            let target = slid(
                slid(fixture::axis(read(lateral)), &self.names.rim[0]),
                &self.names.rim[1],
            );
            mates.push(fixture::mate(
                fixture::axis(SitedRef::new(
                    cap,
                    worn(cap, self.top_body, lateral.clone()),
                )),
                target,
                ContactClass::Rest,
            ));
        }
        mates
    }
}

/// `mates` added to `cap`'s bundle in order; their addresses.
fn seated(
    doc: ProfileDoc,
    cap: RecipeNodeId,
    mates: Vec<Mate<Formula>>,
) -> (ProfileDoc, Vec<MateRef>) {
    mates
        .into_iter()
        .fold((doc, Vec::new()), |(doc, mut refs), mate| {
            let (doc, m) = fixture::constrain_at(doc, cap, mate);
            refs.push(m);
            (doc, refs)
        })
}

/// `top` (the tall block), instantiated and pinned to the world at the
/// identity, and `cap`, an instance of the same part placed by nothing
/// yet.
fn scene(label: &str) -> Scene {
    let mut store = PartStore::new();
    let (part, body) = part_doc(&format!("{label}-top"), TOP_WIDTH, TOP_HEIGHT);
    let lateral = |v| {
        fixture::ename(
            body,
            RoleSeg::LateralEdge(fixture::vpiece(&part, body, 0, v)),
        )
    };
    let rim = |k| fixture::rim_edge(body, CapEnd::End, fixture::piece(&part, body, 0, k));
    let names = PartNames {
        lateral: [lateral(0), lateral(1)],
        rim: [rim(0), rim(1)],
    };
    let (top_ref, top_body) = store.insert_part((part, body), Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, top) = fixture::insert_pinned(doc, Node::instantiate_part(top_ref), &opts);
    let (doc, cap) = insert(doc, fixture::mated_instance(top_ref));
    Scene {
        doc,
        opts,
        top,
        cap,
        top_ref,
        top_body,
        names,
    }
}

/// **The seat the mates ask for**, measured on the document that has
/// no placer anywhere: the cap seated on the top's own top cap, the
/// relative frame a seated row must reproduce. Derived, never
/// transcribed — a constant written down here would be a second
/// statement of what the mates say, and the two could drift apart.
fn control_seat(label: &str) -> Affine3<f64> {
    let s = scene(&format!("{label}-control"));
    let (a, b) = (s.top_cap(), s.cap_bottom());
    let (doc, _) = seated(s.doc.clone(), s.cap, s.seat(s.cap, s.top, &|n| n));
    let ev = run(&doc, &s.opts);
    seat_map(&doc, &ev, &a, &b)
}

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

/// **The cap's target does not resolve**: the placement refuses with
/// its first mate's target unevaluated, on the selection's own refusal.
fn assert_target_unresolved(ev: &Evaluation<f64>, cap: RecipeNodeId, what: &str) {
    let refusal = match ev.result(cap) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::Mate(fault) => match &**fault {
                MateFault::SideUnevaluated {
                    side: MateSide::To,
                    refusal,
                    ..
                } => refusal.kind(),
                other => panic!("{what}: expected the target unevaluated, got {other:?}"),
            },
            other => panic!("{what}: expected a mate refusal, got {other:?}"),
        },
        other => panic!("{what}: expected the cap to refuse, got {other:?}"),
    };
    assert!(
        matches!(refusal, NodeErrorKind::SelectResolve { .. }),
        "{what}: the target's refusal is its selection's: {refusal:?}"
    );
}

// ---- A1: a nested copy seats where its chain stands ----

/// **A1.** `Pattern` over `Part { Instance(1) }` over `Pattern`, the
/// target read at a `Part` projecting the outer pattern's copy 1, under
/// the name a nested copy actually wears — `Instance { 1, of: Instance { 1, of: … } }`. The
/// cap seats on that copy, measured on the product's own face frames
/// against the placer-free control.
///
/// **The two maps do not commute**, which is what makes the row an
/// assertion about the ORDER and not only about the pair: the inner
/// rule translates along `+x` and the outer one is a quarter turn
/// about the document's `z` axis, so `M₂(1) ∘ M₁(1)` carries the
/// master's origin to `(0, 4)` where `M₁(1) ∘ M₂(1)` carries it to
/// `(4, 0)`. A target read anywhere but where the named copy stands
/// would seat the cap somewhere else, and the whole-frame oracle sees
/// it.
///
/// *Breaks if* a pose read at a nested copy is read off another copy,
/// or composed in another order.
#[test]
fn a1_a_nested_copy_seats_where_its_chain_stands() {
    let control = control_seat("msolve2-a1");
    let s = scene("msolve2-a1");
    let (doc, inner) = insert(s.doc.clone(), linear(s.top, [1.0, 0.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(inner, 1));
    let (doc, outer) = circular(doc, part, axis_z(&s, [0.0, 0.0]), Q, 2);
    let doc = world(doc, &s.opts, &[], &[(outer, 2)]);
    let wrap = |n| in_copy(outer, 1, in_copy(inner, 1, n));
    let (doc, copy) = insert(doc, part_of(outer, 1));
    let (doc, _) = seated(doc, s.cap, s.seat(s.cap, copy, &wrap));
    assert_placed(&doc, &s.opts, s.cap, "A1");
    let ev = run(&doc, &s.opts);
    assert_seated(
        &doc,
        &ev,
        &wrap(s.top_cap()),
        &s.cap_bottom(),
        &control,
        "A1 nested copy",
    );
}

/// **A2′.** A target whose OUTER copy carries a non-identity map
/// seats: outer copy 1 of a quarter turn about a `+z` axis at
/// `(-2, 5)`, over inner copy 1 of a linear rule, read at a `Part`
/// projecting it — measured on the
/// product against the placer-free control.
///
/// *Breaks if* an outer rotation about an axis off the origin is lost
/// between the copy and the pose read off it.
#[test]
fn a2d_a_rotating_outer_map_seats() {
    let control = control_seat("msolve2-a2d");
    let s = scene("msolve2-a2d");
    let (doc, inner) = insert(s.doc.clone(), linear(s.top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(inner, 1));
    let (doc, outer) = circular(doc, part, axis_z(&s, [-2.0, 5.0]), Q, 3);
    let doc = world(doc, &s.opts, &[], &[(outer, 3)]);
    let wrap = |n| in_copy(outer, 1, in_copy(inner, 1, n));
    let (doc, copy) = insert(doc, part_of(outer, 1));
    let (doc, _) = seated(doc, s.cap, s.seat(s.cap, copy, &wrap));
    assert_placed(&doc, &s.opts, s.cap, "A2′");
    let ev = run(&doc, &s.opts);
    assert_seated(
        &doc,
        &ev,
        &wrap(s.top_cap()),
        &s.cap_bottom(),
        &control,
        "A2′ rotating outer map",
    );
}

// ---- A3: the `Part` between ----

/// **A3(a).** A `Part`-selected copy read AT the `Part`, with no
/// outer pattern anywhere: the `Part` carries the pattern's copy 1
/// under that copy's own names, and the cap seats on it. The `Part` is
/// what the world places here, so what the row measures is the body a
/// consumer gathers through that door; the at-rest gate holds.
///
/// *Breaks if* a `Part`'s pass-through rows read as another copy.
#[test]
fn a3a_a_part_selected_copy_read_at_the_part_seats() {
    let control = control_seat("msolve2-a3a");
    let s = scene("msolve2-a3a");
    let (doc, pattern) = insert(s.doc.clone(), linear(s.top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(pattern, 1));
    let doc = world(doc, &s.opts, &[part], &[]);
    let wrap = |n| in_copy(pattern, 1, n);
    let (doc, _) = seated(doc, s.cap, s.seat(s.cap, part, &wrap));
    assert_placed(&doc, &s.opts, s.cap, "A3(a)");
    let ev = run(&doc, &s.opts);
    assert_seated(
        &doc,
        &ev,
        &wrap(s.top_cap()),
        &s.cap_bottom(),
        &control,
        "A3(a) part-selected copy",
    );
    assert!(
        gate(&doc, &ev).is_ok(),
        "A3(a): the gate refused: {:?}",
        gate(&doc, &ev).err()
    );
}

// ---- A4: the `Part`'s index against the name ----

/// **A4.** A `Part` whose index selects a copy the target's NAME does
/// not name: the `Part` carries copy 2's rows and the name says
/// copy 1. The door decides nothing that needs a value, so the mates enter;
/// where the cap evaluates, its target resolves nothing, and the cap
/// refuses with the selection's own refusal — the name is never
/// re-read as the copy the `Part` happens to select.
///
/// *Breaks if* a name read at a `Part` resolves against a copy it
/// does not name.
#[test]
fn a4_a_part_that_selects_another_copy_leaves_the_target_unresolved() {
    let s = scene("msolve2-a4");
    let (doc, pattern) = insert(s.doc.clone(), linear(s.top, [0.0, -1.0, 0.0], 4.0, 3));
    // The Part selects copy 2; the name below says copy 1.
    let (doc, part) = insert(doc, part_of(pattern, 2));
    let wrap = |n| in_copy(pattern, 1, n);
    let (doc, _) = seated(doc, s.cap, s.seat(s.cap, part, &wrap));
    let ev = run(&doc, &s.opts);
    assert_target_unresolved(&ev, s.cap, "A4");
}

// ---- the gate on a nested document ----

/// **What the at-rest gate says for a seat read BELOW the outer
/// pattern** — on a nested document. The cap seats on such a target;
/// the gate mints nothing for its mates, because their targets read the
/// `Part`, which no placement places — the outer pattern's copies are
/// what the world holds.
///
/// *Breaks if* a mate whose target body has no world copy mints a
/// declaration.
#[test]
fn the_gate_on_a_seat_read_below_the_outer_pattern_mints_nothing() {
    let s = scene("msolve2-gate");
    let (doc, inner) = insert(s.doc.clone(), linear(s.top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(inner, 1));
    let (doc, outer) = insert(doc, linear(part, [0.0, 0.0, 1.0], LIFT, 2));
    let doc = world(doc, &s.opts, &[], &[(outer, 2)]);
    // Read at the `Part`, BELOW the outer pattern: the name stops at
    // the inner pattern's own `Instance(1)` row.
    let wrap = |n| in_copy(inner, 1, n);
    let (doc, mates) = seated(doc, s.cap, s.seat(s.cap, part, &wrap));
    assert_placed(&doc, &s.opts, s.cap, "a seat read below the outer pattern");
    let ev = run(&doc, &s.opts);
    // No placement reads the `Part` the targets are read at, so no
    // mate has a world copy to mint on.
    for mate in mates {
        fixture::assert_mints_nothing(&doc, &ev, mate);
    }
}

// ---- A1′: the chain, deeper and with transforms interleaved ----

/// **A1′(b).** Three levels — circular, linear, circular — each with
/// a non-identity map, the target read at a `Part` projecting the
/// outermost pattern's copy 2.
///
/// *Breaks if* the copy three levels down is read anywhere but where
/// its chain stands.
#[test]
fn a1c_three_levels_deep_seat() {
    let control = control_seat("msolve2-a1c");
    let s = scene("msolve2-a1c");
    let (doc, p1) = circular(s.doc.clone(), s.top, axis_z(&s, [0.0, 0.0]), Q, 4);
    let (doc, part1) = insert(doc, part_of(p1, 1));
    let (doc, p2) = insert(doc, linear(part1, [1.0, 0.0, 0.0], 4.0, 2));
    let (doc, part2) = insert(doc, part_of(p2, 1));
    let (doc, p3) = circular(
        doc,
        part2,
        axis_z(&s, [5.0, 5.0]),
        std::f64::consts::FRAC_PI_3,
        4,
    );
    let doc = world(doc, &s.opts, &[], &[(p3, 4)]);
    let wrap = |n| in_copy(p3, 2, in_copy(p2, 1, in_copy(p1, 1, n)));
    let (doc, copy) = insert(doc, part_of(p3, 2));
    let (doc, _) = seated(doc, s.cap, s.seat(s.cap, copy, &wrap));
    assert_placed(&doc, &s.opts, s.cap, "A1′(b)");
    let ev = run(&doc, &s.opts);
    assert_seated(
        &doc,
        &ev,
        &wrap(s.top_cap()),
        &s.cap_bottom(),
        &control,
        "A1′(b) three levels",
    );
}

// ---- A4′: per reference, and at the document's bindings ----

/// **A4′(a) — a `Part` mismatch is met per reference.**
///
/// Two `Part`s over one pattern: the cap is seated on `part1`'s copy
/// (selects 0, names 0); a second cap is seated on `part2` (selects 0
/// at insert, NAMES 0). `part2` is then re-pointed
/// at copy 2, which is how a `Part` comes to disagree with a name after
/// insert: the second cap's target no longer resolves, and the first
/// cap's seat is untouched.
///
/// *Breaks if* one placement's unresolved target reaches another's.
#[test]
fn a4b_a_part_mismatch_is_met_per_reference() {
    let s = scene("msolve2-a4b");
    let (doc, cap2) = insert(s.doc.clone(), fixture::mated_instance(s.top_ref));
    let (doc, pattern) = insert(doc, linear(s.top, [0.0, -1.0, 0.0], 4.0, 3));
    let (doc, part1) = insert(doc, part_of(pattern, 0));
    let (doc, part2) = insert(doc, part_of(pattern, 0));
    let wrap = |n| in_copy(pattern, 0, n);
    let (doc, _) = seated(doc, s.cap, s.seat(s.cap, part1, &wrap));
    let (doc, _) = seated(doc, cap2, s.seat(cap2, part2, &wrap));
    assert!(
        solve(&doc, &s.opts, Tol::witness()).fault(cap2).is_none(),
        "the premise: the second cap seats while its `Part` agrees with the name"
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
    let poses = solve(&doc, &s.opts, Tol::witness());
    assert!(
        poses.fault(s.cap).is_none(),
        "the well-formed seat is untouched: {:?}",
        poses.fault(s.cap)
    );
    let ev = run(&doc, &s.opts);
    assert_target_unresolved(&ev, cap2, "A4′(a) the re-pointed `Part`");
}

/// **A4′(b) — the index is EVALUATED, at the document's bindings.**
/// The `Part`'s index is a document parameter: at `k = 1` it agrees
/// with the name and the cap seats; setting `k = 2` leaves the same
/// target unresolved, where the cap evaluates.
///
/// *Breaks if* the index is read anywhere but the document's own
/// bindings, or the cap keeps a seat on a copy the `Part` no longer
/// carries.
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
    let doc = world(doc, &s.opts, &[part], &[]);
    let wrap = |n| in_copy(pattern, 1, n);
    let (doc, _) = seated(doc, s.cap, s.seat(s.cap, part, &wrap));
    assert_placed(&doc, &s.opts, s.cap, "A4′(b) at k = 1");
    let ev = run(&doc, &s.opts);
    assert_seated(
        &doc,
        &ev,
        &wrap(s.top_cap()),
        &s.cap_bottom(),
        &control,
        "A4′(b) at k = 1",
    );

    let (doc, _) = step(
        doc,
        DocEdit::SetVarValue {
            var: k.into(),
            value: FreeValue::Count(2),
        },
    );
    let ev = run(&doc, &s.opts);
    assert_target_unresolved(&ev, s.cap, "A4′(b) at k = 2");
}

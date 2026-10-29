//! **Nested copies as a mate's target** — a family over a copy of
//! one member `p[i]` of a family, read where the member stands.
//!
//! A copy of the indexed read `p[i]` carries member `i`'s names
//! VERBATIM, each under the family's `Member` qualifier, and a family
//! over that copy wraps them once more under its own (D10
//! `[repetition]`). A mate's target is a pose read off geometry of the
//! space the copy joins: read at a family under the name its member
//! wears, it is that member's face where the chain of placers put it —
//! the evaluation of each placer composes the chain, and the fold reads
//! the result.
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

use editor_core::pose::{PoseDef, RoleRead};
use editor_core::{
    CapEnd, Dimension, DocEdit, DocumentId, EvalOptions, Evaluation, Formula, FreeValue, FreeVar,
    Mate, MateFault, MateRef, MateSide, Node, NodeErrorKind, NodeResult, Operand, OperandSlot,
    ProfileDoc, RecipeNodeId, RoleSeg, SitedRef, StableName, TargetAxis, VarId, VarName, product,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::seat::{assert_seated as assert_seated_named, seat_map as seat_map_named};
use fixture::{could_look, in_copy, insert, len, on_frame, run, solve, step};
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

/// The part's own names for what a seat reads: two lateral edges, at
/// the profile's first two vertices, and the rim edges of the first
/// two profile segments, whose lines run along the block's two sides.
#[derive(Clone)]
struct PartNames {
    lateral: [StableName; 2],
    rim: [StableName; 2],
    /// The cap vertices at corner 1, the start cap's then the end
    /// cap's: where a corner frame up `lateral[0]` points its `x`.
    corner: [StableName; 2],
}

/// [`fixture::seat::assert_seated`] over the product names of `a` and
/// `b` ([`fixture::product_name`]).
fn assert_seated(
    doc: &ProfileDoc,
    ev: &editor_core::Evaluation<f64>,
    a: &(impl fixture::Sited + Clone),
    b: &(impl fixture::Sited + Clone),
    control: &Affine3<f64>,
    what: &str,
) {
    let (a, b) = (
        fixture::product_name(doc, ev, a.clone()),
        fixture::product_name(doc, ev, b.clone()),
    );
    assert_seated_named(doc, ev, &a, &b, control, what);
}

/// [`fixture::seat::seat_map`] over the product names of `a` and `b`.
fn seat_map(
    doc: &ProfileDoc,
    ev: &editor_core::Evaluation<f64>,
    a: &(impl fixture::Sited + Clone),
    b: &(impl fixture::Sited + Clone),
) -> Affine3<f64> {
    seat_map_named(
        doc,
        ev,
        &fixture::product_name(doc, ev, a.clone()),
        &fixture::product_name(doc, ev, b.clone()),
    )
}

/// **The axis parallel to `+z` through the point `at` along the rim
/// lines of `node`'s copy**, read off that copy's geometry under `wrap`:
/// its lateral edge at the profile's first vertex, slid `at[0]` along
/// its first rim line and `at[1]` along its second.
fn axis_at(
    s: &Scene,
    node: RecipeNodeId,
    wrap: &dyn Fn(StableName) -> StableName,
    at: [f64; 2],
) -> Operand {
    let read = |name: &StableName| SitedRef::new(node, wrap(s.on_top(name)));
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

/// **A ring of `count` copies of `input` about `axis`**, `step_angle`
/// apart ([`fixture::circular`]) — the family whose member map ROTATES,
/// which is what makes a chain's two levels fail to commute. `axis`
/// reads `input`'s own geometry.
fn ring(
    doc: ProfileDoc,
    input: RecipeNodeId,
    axis: Operand,
    step_angle: f64,
    count: i64,
) -> (ProfileDoc, VarId, RecipeNodeId) {
    fixture::circular(
        doc,
        input,
        (RoleRead::plain(axis), TargetAxis::Z),
        step_angle,
        count,
    )
}

/// A quarter turn — the step every rotating rule here takes.
const Q: f64 = std::f64::consts::FRAC_PI_2;

/// Half the block's width: how far each target edge is slid across the
/// copy, so the cap overhangs it.
const OVERHANG: f64 = 0.5 * TOP_WIDTH;

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
    fn cap_bottom(&self) -> crate::fixture::resolver::PartName {
        in_part(self.cap, self.top_body, CapEnd::Start)
    }

    /// The top's top cap, as a nested copy's chain wears it.
    fn top_cap(&self) -> crate::fixture::resolver::PartName {
        in_part(self.top, self.top_body, CapEnd::End)
    }

    /// `name`, an entity of the top's part, as the top wears it.
    fn on_top(&self, name: &StableName) -> StableName {
        name.clone()
    }

    /// **The seat of `cap` on the copy read at `at` under `wrap`**: one
    /// frame mate equating the cap's corner frame at its bottom cap
    /// (up its lateral edge 0, `x` towards corner 1) with the copy's at
    /// its top cap, slid [`OVERHANG`] along both rim lines, so the cap
    /// overhangs the copy — `wrap` spells a name of the top as the copy
    /// read at `at` wears it.
    fn seat(
        &self,
        cap: RecipeNodeId,
        at: RecipeNodeId,
        wrap: &dyn Fn(StableName) -> StableName,
    ) -> Vec<Mate<Formula>> {
        let read = |name: &StableName| SitedRef::new(at, wrap(self.on_top(name)));
        let own = |name: &StableName| SitedRef::new(cap, name.clone());
        let slid = |pose: Operand, along: &StableName| {
            Operand::Pose(Box::new(PoseDef::Slide {
                pose: RoleRead::plain(pose),
                along: RoleRead::plain(fixture::axis(read(along))),
                by: len(OVERHANG),
            }))
        };
        let target = slid(
            slid(
                fixture::corner_frame(read(&self.names.lateral[0]), read(&self.names.corner[1])),
                &self.names.rim[0],
            ),
            &self.names.rim[1],
        );
        vec![fixture::mate(
            fixture::corner_frame(own(&self.names.lateral[0]), own(&self.names.corner[0])),
            target,
        )]
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
    let corner = |end| fixture::cap_vertex(body, end, fixture::vpiece(&part, body, 0, 1));
    let names = PartNames {
        lateral: [lateral(0), lateral(1)],
        rim: [rim(0), rim(1)],
        corner: [corner(CapEnd::Start), corner(CapEnd::End)],
    };
    let (top_ref, top_body) = store.insert_part((part, body), Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, top) = fixture::insert_pinned(doc, crate::fixture::mated_instance(top_ref), &opts);
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
        doc.node_space(cap),
        Some(VarId::WORLD),
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

/// **A1.** A ring over a copy of `inner[1]`, a row's member, the target
/// read at the ring under the name its member 1 wears —
/// `Member { (j, 1), of: Member { (i, 1), of: … } }`. The cap seats on
/// that member, measured on the product's own face frames against the
/// placer-free control.
///
/// **The two maps do not commute**, which is what makes the row an
/// assertion about the ORDER and not only about the pair: the inner
/// row translates along `+x` and the ring is a quarter turn about the
/// document's `z` axis, so `M₂(1) ∘ M₁(1)` carries the
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
    let (doc, i, inner) = fixture::linear(s.doc.clone(), s.top, [1.0, 0.0, 0.0], 4.0, 2);
    let (doc, part) = fixture::member(doc, inner, 1);
    let on_part = |n| in_copy(inner, i, 1, n);
    // The ring's axis is the world's `z`: four back along the copy's
    // first rim line from its lateral edge.
    let axis = axis_at(&s, part, &on_part, [-4.0, 0.0]);
    let (doc, j, outer) = ring(doc, part, axis, Q, 2);
    let wrap = |n| in_copy(outer, j, 1, in_copy(inner, i, 1, n));
    let (doc, _) = seated(doc, s.cap, s.seat(s.cap, outer, &wrap));
    assert_placed(&doc, &s.opts, s.cap, "A1");
    let ev = run(&doc, &s.opts);
    assert_seated(
        &doc,
        &ev,
        &wrap(s.top_cap().name),
        &s.cap_bottom(),
        &control,
        "A1 nested copy",
    );
}

/// **A2′.** A target whose OUTER member carries a non-identity map
/// seats: member 1 of a quarter-turn ring about a `+z` axis at
/// `(-2, 5)`, over a copy of member 1 of a row, read at the ring —
/// measured on the product against the placer-free control.
///
/// *Breaks if* an outer rotation about an axis off the origin is lost
/// between the copy and the pose read off it.
#[test]
fn a2d_a_rotating_outer_map_seats() {
    let control = control_seat("msolve2-a2d");
    let s = scene("msolve2-a2d");
    let (doc, i, inner) = fixture::linear(s.doc.clone(), s.top, [0.0, -1.0, 0.0], 4.0, 2);
    let (doc, part) = fixture::member(doc, inner, 1);
    let on_part = |n| in_copy(inner, i, 1, n);
    // `(-2, 5)` in the world, from the copy's lateral edge at `(0, -4)`.
    let axis = axis_at(&s, part, &on_part, [-2.0, 9.0]);
    let (doc, j, outer) = ring(doc, part, axis, Q, 3);
    let wrap = |n| in_copy(outer, j, 1, in_copy(inner, i, 1, n));
    let (doc, _) = seated(doc, s.cap, s.seat(s.cap, outer, &wrap));
    assert_placed(&doc, &s.opts, s.cap, "A2′");
    let ev = run(&doc, &s.opts);
    assert_seated(
        &doc,
        &ev,
        &wrap(s.top_cap().name),
        &s.cap_bottom(),
        &control,
        "A2′ rotating outer map",
    );
}

// ---- A3: the member, read at its family ----

/// **A3(a).** A member read AT its family, with no outer family
/// anywhere: the family carries member 0 under that member's own names
/// in the world, its first member one pitch clear of the top, and the
/// cap seats on it. The family is what the world places here, so what
/// the row measures is the body a consumer gathers; the at-rest gate
/// holds.
///
/// *Breaks if* a family's member rows read as another member.
#[test]
fn a3a_a_member_read_at_its_family_seats() {
    let control = control_seat("msolve2-a3a");
    let s = scene("msolve2-a3a");
    let (doc, k, pattern) = fixture::linear_on(s.doc.clone(), s.top, [0.0, -1.0, 0.0], 4.0, 1);
    let wrap = |n| in_copy(pattern, k, 0, n);
    let (doc, _) = seated(doc, s.cap, s.seat(s.cap, pattern, &wrap));
    assert_placed(&doc, &s.opts, s.cap, "A3(a)");
    let ev = run(&doc, &s.opts);
    assert_seated(
        &doc,
        &ev,
        &wrap(s.top_cap().name),
        &s.cap_bottom(),
        &control,
        "A3(a) a member read at its family",
    );
    assert!(
        could_look(&doc, &ev).is_ok(),
        "A3(a): the gate refused: {:?}",
        could_look(&doc, &ev).err()
    );
}

// ---- A4: a member read's index against the name ----

/// **A4.** A copy of `p[2]` read under a name of member 1: the copy
/// carries member 2's rows and the name says member 1. The door decides
/// nothing that needs a value, so the mates enter; where the cap
/// evaluates, its target resolves nothing, and the cap refuses with the
/// selection's own refusal — the name is never re-read as the member
/// the read happens to pick.
///
/// *Breaks if* a name read at a member's copy resolves against a member
/// it does not name.
#[test]
fn a4_a_read_of_another_member_leaves_the_target_unresolved() {
    let s = scene("msolve2-a4");
    let (doc, k, pattern) = fixture::linear(s.doc.clone(), s.top, [0.0, -1.0, 0.0], 4.0, 3);
    // The read picks member 2; the name below says member 1.
    let (doc, part) = fixture::member(doc, pattern, 2);
    let wrap = |n| in_copy(pattern, k, 1, n);
    let (doc, _) = seated(doc, s.cap, s.seat(s.cap, part, &wrap));
    let ev = run(&doc, &s.opts);
    assert_target_unresolved(&ev, s.cap, "A4");
}

// ---- A1′: the chain, deeper and with transforms interleaved ----

/// **A1′(b).** Three levels — ring, row, ring — each with a
/// non-identity map, each over a copy of one member of the level
/// below, the target read at the outermost ring's member 2.
///
/// *Breaks if* the copy three levels down is read anywhere but where
/// its chain stands.
#[test]
fn a1c_three_levels_deep_seat() {
    let control = control_seat("msolve2-a1c");
    let s = scene("msolve2-a1c");
    let own = |n| n;
    let axis = axis_at(&s, s.top, &own, [0.0, 0.0]);
    let (doc, k1, p1) = ring(s.doc.clone(), s.top, axis, Q, 4);
    let (doc, part1) = fixture::member(doc, p1, 1);
    let (doc, k2, p2) = fixture::linear(doc, part1, [1.0, 0.0, 0.0], 4.0, 2);
    let (doc, part2) = fixture::member(doc, p2, 1);
    let on_part2 = |n| in_copy(p2, k2, 1, in_copy(p1, k1, 1, n));
    // `(5, 5)` in the world, from part 2's lateral edge at `(4, 0)`: its
    // rim lines run along `+y` and `-x`, the block a quarter turn round.
    let axis = axis_at(&s, part2, &on_part2, [5.0, -1.0]);
    let (doc, k3, p3) = ring(doc, part2, axis, std::f64::consts::FRAC_PI_3, 4);
    let wrap = |n| in_copy(p3, k3, 2, in_copy(p2, k2, 1, in_copy(p1, k1, 1, n)));
    let (doc, _) = seated(doc, s.cap, s.seat(s.cap, p3, &wrap));
    assert_placed(&doc, &s.opts, s.cap, "A1′(b)");
    let ev = run(&doc, &s.opts);
    assert_seated(
        &doc,
        &ev,
        &wrap(s.top_cap().name),
        &s.cap_bottom(),
        &control,
        "A1′(b) three levels",
    );
}

// ---- A4′: per reference, and at the document's bindings ----

/// **A4′(a) — a member read's mismatch is met per reference.**
///
/// Two copies of `p[0]` of one family: the cap is seated on `part1`
/// (reads 0, names 0); a second cap is seated on `part2` (reads 0 at
/// insert, NAMES 0). `part2` is then re-pointed at member 2, which is
/// how a read comes to disagree with a name after insert: the second
/// cap's target no longer resolves, and the first cap's seat is
/// untouched.
///
/// *Breaks if* one placement's unresolved target reaches another's.
#[test]
fn a4b_a_part_mismatch_is_met_per_reference() {
    let s = scene("msolve2-a4b");
    let (doc, cap2) = insert(s.doc.clone(), fixture::mated_instance(s.top_ref));
    let (doc, k, pattern) = fixture::linear(doc, s.top, [0.0, -1.0, 0.0], 4.0, 3);
    let (doc, part1) = fixture::member(doc, pattern, 0);
    let (doc, part2) = fixture::member(doc, pattern, 0);
    let wrap = |n| in_copy(pattern, k, 0, n);
    let (doc, _) = seated(doc, s.cap, s.seat(s.cap, part1, &wrap));
    let (doc, _) = seated(doc, cap2, s.seat(cap2, part2, &wrap));
    assert!(
        solve(&doc, &s.opts, Tol::witness()).fault(cap2).is_none(),
        "the premise: the second cap seats while its read agrees with the name"
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetStructuralParam {
            node: part2,
            slot: editor_core::SlotId::Index {
                seat: OperandSlot::Source,
                k: 0,
            },
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
    assert_target_unresolved(&ev, cap2, "A4′(a) the re-pointed read");
}

/// **A4′(b) — the index is EVALUATED, at the document's bindings.**
/// The member read's index is a document parameter: at `which = 1` it
/// agrees with the name and the cap seats; setting `which = 2` leaves
/// the same target unresolved, where the cap evaluates.
///
/// *Breaks if* the index is read anywhere but the document's own
/// bindings, or the cap keeps a seat on a member the read no longer
/// carries.
#[test]
fn a4c_the_read_index_is_evaluated_at_the_documents_bindings() {
    let control = control_seat("msolve2-a4c");
    let s = scene("msolve2-a4c");
    let which = VarName::from_static("which");
    let (doc, _) = step(
        s.doc.clone(),
        DocEdit::DeclareVar {
            name: which.clone(),
            def: editor_core::VarDecl::Free(FreeVar::Count { value: 1 }),
        },
    );
    let (doc, k, pattern) = fixture::linear(doc, s.top, [0.0, -1.0, 0.0], 4.0, 3);
    let (doc, part) = fixture::member_at(
        doc,
        pattern,
        vec![Formula::named(which.clone(), Dimension::Count)],
    );
    let wrap = |n| in_copy(pattern, k, 1, n);
    let (doc, _) = seated(doc, s.cap, s.seat(s.cap, part, &wrap));
    assert_placed(&doc, &s.opts, s.cap, "A4′(b) at which = 1");
    let ev = run(&doc, &s.opts);
    assert_seated(
        &doc,
        &ev,
        &wrap(s.top_cap().name),
        &s.cap_bottom(),
        &control,
        "A4′(b) at which = 1",
    );

    let (doc, _) = step(
        doc,
        DocEdit::SetVarValue {
            var: which.into(),
            value: FreeValue::Count(2),
        },
    );
    let ev = run(&doc, &s.opts);
    assert_target_unresolved(&ev, s.cap, "A4′(b) at which = 2");
}

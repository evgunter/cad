//! **The member chain** — nested copies through `Node::Part`, and
//! sibling distinctness at every level of the nest.
//!
//! A pattern's value is many bodies and a pattern's input is one, so
//! a pattern OVER a pattern does not evaluate. The nested shape a
//! user can build runs through `Node::Part { select: Instance(i) }`,
//! which projects one body out of the instances carrying every name
//! VERBATIM: `Pattern` over `Part` over `Pattern`. The walk from a
//! mate's operand passes the `Part` contributing nothing, consumes an
//! `Instance(i)` qualifier at each pattern, and lands on the
//! instance that minted the name — so a nested copy's member is that
//! instance, the CHAIN of copies, and the operand.
//!
//! Every row goes through ordinary doors — `DocEdit::InsertNode`,
//! `solve_document`, `evaluate`, `product`, `assemble` — and the
//! seating rows measure the PRODUCT's own face frames with the shared
//! whole-frame oracle (`fixture::seat`), never a solved frame read by
//! eye.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, AssemblyError, Attribution, AxisSense, CapEnd, ContactClass, Datum, Dimension,
    DocEdit, DocParam, DocParamValue, DocumentId, EvalOptions, Expr, MateFault, MateFrame,
    MatePrimitive, MateRole, MateSide, Node, ParamName, PartSelect, PatternKind, ProfileDoc,
    ProfileProgram, RecipeNodeId, SitedRef, SplitHalf, StableName, clusters, member_of, product,
    solve_document,
};
use fixture::resolver::{PartStore, in_part};
use fixture::seat::{assert_seated, seat_map};
use fixture::{ang, gate, in_copy, insert, len, on_frame, run, scl, step, xform};
use geom_core::Tol;
use geom_core::linalg::Affine3;

// ---- the scene's two parts ----

/// The base's half-width and height: a `9x9x1` slab, wide enough that
/// two declared seats stand clear of each other and of its edges.
const BASE_WIDTH: f64 = 9.0;
const BASE_HEIGHT: f64 = 1.0;
const TOP_HEIGHT: f64 = 3.0;

/// A `w x w x h` block, as a whole part document.
fn part_doc(label: &str, w: f64, h: f64) -> ProfileDoc {
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

/// A `Rest`/`FrameCoincidence`/`Opposed` mate seating `b`'s bottom cap
/// onto the point `a_origin` of `a`'s top cap, each frame authored in
/// its own member's part coordinates.
///
/// The two outward normals and `Opposed` are what make this a
/// physical seat: the block stands ON the slab. `a_origin` moves the
/// declared contact point across the slab, which is how a second mate
/// declares the seat a sibling copy actually lands in.
fn seat_at(a: SitedRef, b: SitedRef, a_origin: [f64; 3]) -> Node<ProfileProgram> {
    Node::Mate {
        a,
        b,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame {
                origin: a_origin,
                axis: [0.0, 0.0, 1.0],
                reference: [1.0, 0.0, 0.0],
            },
            b: MateFrame {
                origin: [0.0, 0.0, 0.0],
                axis: [0.0, 0.0, -1.0],
                reference: [1.0, 0.0, 0.0],
            },
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

/// The seat every row's first mate declares: the slab's top cap at
/// `(1, 1)`.
const FIRST_SEAT: [f64; 3] = [1.0, 1.0, BASE_HEIGHT];

/// A document holding the two instances and their resolver.
struct Scene {
    doc: ProfileDoc,
    opts: EvalOptions,
    base: RecipeNodeId,
    top: RecipeNodeId,
}

/// `base` (the wide slab) and `top` (the tall block), instantiated,
/// with nothing between them yet.
fn scene(label: &str) -> Scene {
    let mut store = PartStore::new();
    let base_ref = store.insert(
        part_doc(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let top_ref = store.insert(
        part_doc(&format!("{label}-top"), 1.0, TOP_HEIGHT),
        Tol::witness(),
    );
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    Scene {
        doc,
        opts,
        base,
        top,
    }
}

/// **The seat this fixture's alignment asks for**, measured on the
/// document that has no placer anywhere: the relative frame a seated
/// row must reproduce. Derived, never transcribed — a constant
/// written down here would be a second statement of what the
/// alignment says, and the two could drift apart.
fn control_seat(label: &str) -> Affine3<f64> {
    let s = scene(&format!("{label}-control"));
    let (base, top) = (s.base, s.top);
    let a = in_part(base, CapEnd::End);
    let b = in_part(top, CapEnd::Start);
    let (doc, _) = step(
        s.doc,
        DocEdit::InsertNode {
            node: seat_at(
                SitedRef::at_mint(a.clone()),
                SitedRef::at_mint(b.clone()),
                FIRST_SEAT,
            ),
        },
    );
    let ev = run(&doc, &s.opts);
    seat_map(&doc, &ev, &a, &b)
}

/// A linear pattern node over `input`.
fn linear(input: RecipeNodeId, dir: [f64; 3], spacing: f64, count: i64) -> Node<ProfileProgram> {
    Node::Pattern {
        input,
        count: Expr::count(count),
        kind: PatternKind::Linear {
            direction: dir.map(scl),
            spacing: len(spacing),
        },
    }
}

/// A `Part` selecting instance `i` of `of` — the identity-transparent
/// projection a nested pattern is built through.
fn part_of(of: RecipeNodeId, i: i64) -> Node<ProfileProgram> {
    Node::Part {
        of,
        select: PartSelect::Instance(Expr::count(i)),
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
            input,
            count: Expr::count(count),
            kind: PatternKind::Circular {
                axis,
                step: ang(step_angle),
            },
        },
    )
}

/// A quarter turn — the step every rotating rule here takes.
const Q: f64 = std::f64::consts::FRAC_PI_2;

/// **The partition the registry is keyed by and the partition the
/// solve folds over are ONE partition.** `clusters` and
/// `solve_document` read which mates weld through one door
/// (`read_mates`), so a document's cluster membership and its solved
/// gauges cannot disagree; this asserts that on whatever document it
/// is handed.
fn assert_partition_agrees(doc: &ProfileDoc, what: &str) {
    let poses = solve_document(doc, Tol::witness());
    let mut from_solve: BTreeMap<RecipeNodeId, Vec<RecipeNodeId>> = BTreeMap::new();
    for c in clusters(doc) {
        for &i in &c {
            let g = poses.gauge(i).expect("every live instance has a gauge");
            from_solve.entry(g).or_default().push(i);
        }
    }
    let mut from_solve: Vec<Vec<RecipeNodeId>> = from_solve.into_values().collect();
    let mut from_clusters = clusters(doc);
    for c in from_solve.iter_mut().chain(from_clusters.iter_mut()) {
        c.sort();
    }
    from_solve.sort();
    from_clusters.sort();
    assert_eq!(from_solve, from_clusters, "{what}: the partitions disagree");
}

// ---- A1: a nested copy seats at the composed pose ----

/// **A1.** `Pattern` over `Part { Instance(1) }` over `Pattern`, the
/// mate read at the outer pattern under the name a nested copy
/// actually wears — `Instance { 1, of: Instance { 1, of: … } }`. The
/// copy seats on the slab at `M₂(1) ∘ M₁(1) ∘ placement`, measured on
/// the product's own face frames against the placer-free control.
///
/// **The two maps do not commute**, which is what makes the row an
/// assertion about the ORDER and not only about the pair: the inner
/// rule translates along `+x` and the outer one is a quarter turn
/// about the document's `z` axis, so `M₂(1) ∘ M₁(1)` carries the
/// master's origin to `(0, 4)` where `M₁(1) ∘ M₂(1)` carries it to
/// `(4, 0)`. A solve that folded the chain the other way round would
/// place the instance somewhere the named copy does not seat, and the
/// whole-frame oracle sees it.
#[test]
fn a1_a_nested_copy_seats_at_the_composed_pose() {
    let control = control_seat("msolve2-a1");
    let s = scene("msolve2-a1");
    let (base, top) = (s.base, s.top);
    let (doc, inner) = insert(s.doc, linear(top, [1.0, 0.0, 0.0], 4.0, 2));
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
            input: part,
            count: Expr::count(2),
            kind: PatternKind::Circular {
                axis,
                step: ang(std::f64::consts::FRAC_PI_2),
            },
        },
    );
    let a = in_part(base, CapEnd::End);
    let b = in_copy(outer, 1, in_copy(inner, 1, in_part(top, CapEnd::Start)));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(
                SitedRef::at_mint(a.clone()),
                SitedRef::new(outer, b.clone()),
                FIRST_SEAT,
            ),
        },
    );
    let mate = mate.unwrap();

    // The member the reference resolves to, read at the door the
    // solve reads: the chain is OUTERMOST first and the instance is
    // the one that minted the name.
    let member =
        member_of(&doc, &SitedRef::new(outer, b.clone())).expect("a nested copy is a member");
    assert_eq!(member.instance, top, "the member stands on the instance");
    assert_eq!(
        member.copy,
        vec![(outer, 1), (inner, 1)],
        "the copy chain is outermost first"
    );
    assert_eq!(
        member.at, outer,
        "the operand is the node the mate is read at"
    );

    let poses = solve_document(&doc, Tol::witness());
    assert!(
        poses.fault(mate).is_none(),
        "A1: the solve refused a nested copy: {:?}",
        poses.fault(mate)
    );
    assert_eq!(poses.role(mate), Some(MateRole::Determining));
    let ev = run(&doc, &s.opts);
    assert!(
        product(&doc, &ev, Tol::witness()).is_ok(),
        "A1: the product gathers"
    );
    assert_seated(&doc, &ev, &a, &b, &control, "A1 nested copy");
}

// ---- A2: loop closure, at each level of the nest ----

/// **The loop-closure shape, asserted.** Two mates from one base to
/// two DIFFERENT members over the same instance key as two pairs, so
/// the first is the cluster's tree edge and the second closes a loop:
/// it determines nothing and is carried to the at-rest gate as a pure
/// declaration. The gate is therefore where a loop is verified — a
/// consistent pair passes it, an inconsistent one is refused there
/// and attributed to the declaration the geometry contradicts.
///
/// Both halves are asserted for every row, because a fixture whose
/// CONSISTENT pair the gate also refused would make the other half
/// vacuous.
fn assert_loop_closes(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    tree: RecipeNodeId,
    closer: RecipeNodeId,
    consistent: bool,
    what: &str,
) {
    let poses = solve_document(doc, Tol::witness());
    assert!(
        poses.fault(tree).is_none() && poses.fault(closer).is_none(),
        "{what}: the solve places on the tree edge and never verifies the \
         loop: {:?} / {:?}",
        poses.fault(tree),
        poses.fault(closer)
    );
    assert_eq!(
        poses.role(tree),
        Some(MateRole::Determining),
        "{what}: the first member pair is the tree edge"
    );
    assert_eq!(
        poses.role(closer),
        Some(MateRole::Declaring),
        "{what}: the second member pair closes a loop"
    );
    let ev = run(doc, opts);
    assert!(
        product(doc, &ev, Tol::witness()).is_ok(),
        "{what}: the product gathers"
    );
    if consistent {
        assert!(
            gate(doc, &ev).is_ok(),
            "{what}: the gate refused a consistent pair: {:?}",
            gate(doc, &ev).err()
        );
        return;
    }
    let err = gate(doc, &ev).expect_err("the gate refuses the unmet declaration");
    let AssemblyError::AtRest { findings } = &err else {
        panic!("{what}: expected the at-rest gate's refusal, got {err:?}");
    };
    assert!(
        findings
            .iter()
            .any(|f| matches!(f.attribution, Attribution::Refuted(_))),
        "{what}: the refusal names the declaration the geometry \
         contradicts, rather than only an undeclared contact: {findings:?}"
    );
}

/// The second seat, four units along `+y` from the first — where a
/// sibling copy one inner or one outer step along that rule actually
/// lands.
const SECOND_SEAT: [f64; 3] = [1.0, 5.0, BASE_HEIGHT];

/// **How an inconsistent pair is built here, and why not by moving
/// the declared frame.** A declaring mate is verified against the
/// GEOMETRY: the gate asks whether the two faces the mate names are
/// in contact, and the mate frames the solve folds are not part of
/// that question. So an unsatisfiable declaration is one whose named
/// copy is somewhere else — a sibling the rule lifts ten units clear
/// of the slab — and moving the authored `a` frame instead would
/// change nothing the gate reads. (Measured: an `a` frame lifted
/// three units off the slab leaves the gate green, because the copy
/// it names still rests where the tree edge put it.)
const LIFT: f64 = 10.0;
/// The rule that lifts every sibling clear of the slab.
const LIFTING: ([f64; 3], f64) = ([0.0, 0.0, 1.0], LIFT);
/// The rule whose next sibling lands on the slab at [`SECOND_SEAT`].
const ALONG: ([f64; 3], f64) = ([0.0, 1.0, 0.0], 4.0);

/// **A2(b).** Two mates onto sibling copies of the OUTER pattern —
/// the same inner copy, outer copies 0 and 1 of one chain. The two
/// members differ only in the outer index, which is enough to make
/// them different pairs.
#[test]
fn a2b_sibling_outer_copies_close_a_loop() {
    let outer_siblings = |label: &str, rule: ([f64; 3], f64), second: [f64; 3]| {
        let s = scene(label);
        let (base, top) = (s.base, s.top);
        let (doc, inner) = insert(s.doc, linear(top, [0.0, -1.0, 0.0], 4.0, 2));
        let (doc, part) = insert(doc, part_of(inner, 1));
        let (doc, outer) = insert(doc, linear(part, rule.0, rule.1, 2));
        let a = in_part(base, CapEnd::End);
        let master = in_part(top, CapEnd::Start);
        let named = |i2: u32| in_copy(outer, i2, in_copy(inner, 1, master.clone()));
        let (doc, m1) = step(
            doc,
            DocEdit::InsertNode {
                node: seat_at(
                    SitedRef::at_mint(a.clone()),
                    SitedRef::new(outer, named(0)),
                    FIRST_SEAT,
                ),
            },
        );
        let (doc, m2) = step(
            doc,
            DocEdit::InsertNode {
                node: seat_at(SitedRef::at_mint(a), SitedRef::new(outer, named(1)), second),
            },
        );
        (doc, s.opts, m1.unwrap(), m2.unwrap())
    };
    // Consistent: the outer rule steps along the slab, so copy 1 rests
    // exactly where the second mate declares it does.
    let (doc, opts, m1, m2) = outer_siblings("msolve2-a2b-consistent", ALONG, SECOND_SEAT);
    assert_loop_closes(&doc, &opts, m1, m2, true, "A2(b) consistent");
    // Inconsistent: the outer rule lifts, so copy 1 floats clear of
    // the slab while the second mate declares it rests on it.
    let (doc, opts, m1, m2) = outer_siblings("msolve2-a2b-inconsistent", LIFTING, FIRST_SEAT);
    assert_loop_closes(&doc, &opts, m1, m2, false, "A2(b) inconsistent");
}

/// **A2(a).** Two mates onto sibling copies of the INNER pattern.
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
/// difference is at the inner level; the row that makes the INNER
/// INDEX itself the deciding field is `a2b`, whose two members share
/// one chain and differ only in an index.
#[test]
fn a2a_sibling_inner_copies_close_a_loop() {
    let inner_siblings =
        |label: &str, rule: ([f64; 3], f64), picks: (i64, i64), second: [f64; 3]| {
            let s = scene(label);
            let (base, top) = (s.base, s.top);
            let (doc, inner) = insert(s.doc, linear(top, rule.0, rule.1, 3));
            let (doc, part_a) = insert(doc, part_of(inner, picks.0));
            let (doc, outer_a) = insert(doc, linear(part_a, [0.0, 0.0, 1.0], LIFT, 2));
            let (doc, part_b) = insert(doc, part_of(inner, picks.1));
            let (doc, outer_b) = insert(doc, linear(part_b, [0.0, 0.0, 1.0], LIFT, 2));
            let a = in_part(base, CapEnd::End);
            let master = in_part(top, CapEnd::Start);
            let (i1a, i1b) = (picks.0 as u32, picks.1 as u32);
            let (doc, m1) = step(
                doc,
                DocEdit::InsertNode {
                    node: seat_at(
                        SitedRef::at_mint(a.clone()),
                        SitedRef::new(
                            outer_a,
                            in_copy(outer_a, 0, in_copy(inner, i1a, master.clone())),
                        ),
                        FIRST_SEAT,
                    ),
                },
            );
            let (doc, m2) = step(
                doc,
                DocEdit::InsertNode {
                    node: seat_at(
                        SitedRef::at_mint(a),
                        SitedRef::new(outer_b, in_copy(outer_b, 0, in_copy(inner, i1b, master))),
                        second,
                    ),
                },
            );
            (doc, s.opts, m1.unwrap(), m2.unwrap())
        };
    // Consistent: the inner rule steps along the slab, so inner copy 1
    // rests one step from inner copy 2.
    let (doc, opts, m1, m2) = inner_siblings(
        "msolve2-a2a-consistent",
        ([0.0, -1.0, 0.0], 4.0),
        (2, 1),
        SECOND_SEAT,
    );
    assert_loop_closes(&doc, &opts, m1, m2, true, "A2(a) consistent");
    // Inconsistent: the inner rule lifts, so inner copy 1 floats clear
    // of the slab while the second mate declares it rests on it.
    let (doc, opts, m1, m2) =
        inner_siblings("msolve2-a2a-inconsistent", LIFTING, (0, 1), FIRST_SEAT);
    assert_loop_closes(&doc, &opts, m1, m2, false, "A2(a) inconsistent");
}

/// **A2(c).** Two mates onto copies differing at BOTH levels —
/// `(0, 1)` and `(1, 2)` — which is the case a chain compared
/// lexicographically has to get right at more than its first element.
#[test]
fn a2c_copies_differing_at_both_levels_close_a_loop() {
    let both_levels = |label: &str, outer_b_rule: ([f64; 3], f64), second: [f64; 3]| {
        let s = scene(label);
        let (base, top) = (s.base, s.top);
        let (doc, inner) = insert(s.doc, linear(top, [0.0, -1.0, 0.0], 4.0, 3));
        let (doc, part_a) = insert(doc, part_of(inner, 1));
        let (doc, outer_a) = insert(doc, linear(part_a, [0.0, 0.0, 1.0], LIFT, 2));
        let (doc, part_b) = insert(doc, part_of(inner, 2));
        let (doc, outer_b) = insert(doc, linear(part_b, outer_b_rule.0, outer_b_rule.1, 2));
        let a = in_part(base, CapEnd::End);
        let master = in_part(top, CapEnd::Start);
        let (doc, m1) = step(
            doc,
            DocEdit::InsertNode {
                node: seat_at(
                    SitedRef::at_mint(a.clone()),
                    SitedRef::new(
                        outer_a,
                        in_copy(outer_a, 0, in_copy(inner, 1, master.clone())),
                    ),
                    FIRST_SEAT,
                ),
            },
        );
        let (doc, m2) = step(
            doc,
            DocEdit::InsertNode {
                node: seat_at(
                    SitedRef::at_mint(a),
                    SitedRef::new(outer_b, in_copy(outer_b, 1, in_copy(inner, 2, master))),
                    second,
                ),
            },
        );
        (doc, s.opts, m1.unwrap(), m2.unwrap())
    };
    // Consistent: the second chain's outer step carries inner copy 2
    // back onto the slab, one seat along from the first.
    let (doc, opts, m1, m2) = both_levels(
        "msolve2-a2c-consistent",
        ([0.0, 1.0, 0.0], 8.0),
        SECOND_SEAT,
    );
    assert_loop_closes(&doc, &opts, m1, m2, true, "A2(c) consistent");
    // Inconsistent: it lifts instead, so the named copy floats.
    let (doc, opts, m1, m2) = both_levels("msolve2-a2c-inconsistent", LIFTING, FIRST_SEAT);
    assert_loop_closes(&doc, &opts, m1, m2, false, "A2(c) inconsistent");
}

// ---- A3: the `Part` in the walk ----

/// **A3(a).** A `Part`-selected copy read AT the `Part`, with no
/// outer pattern anywhere: the walk passes the `Part` contributing
/// nothing, consumes the pattern's `Instance(1)` qualifier, and the
/// copy seats. The `Part` IS the product root here, so what the row
/// measures is the body a consumer gathers through that door.
#[test]
fn a3a_a_part_selected_copy_read_at_the_part_is_a_member() {
    let control = control_seat("msolve2-a3a");
    let s = scene("msolve2-a3a");
    let (base, top) = (s.base, s.top);
    let (doc, pattern) = insert(s.doc, linear(top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(pattern, 1));
    let a = in_part(base, CapEnd::End);
    let b = in_copy(pattern, 1, in_part(top, CapEnd::Start));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(
                SitedRef::at_mint(a.clone()),
                SitedRef::new(part, b.clone()),
                FIRST_SEAT,
            ),
        },
    );
    let mate = mate.unwrap();
    let member = member_of(&doc, &SitedRef::new(part, b.clone())).expect("a member");
    assert_eq!(member.instance, top);
    assert_eq!(
        member.copy,
        vec![(pattern, 1)],
        "one level, through the Part"
    );
    assert_eq!(
        member.at, part,
        "the operand is the Part the mate is read at"
    );
    let poses = solve_document(&doc, Tol::witness());
    assert!(
        poses.fault(mate).is_none(),
        "A3(a): the solve refused: {:?}",
        poses.fault(mate)
    );
    let ev = run(&doc, &s.opts);
    assert_seated(&doc, &ev, &a, &b, &control, "A3(a) part-selected copy");
    assert!(
        gate(&doc, &ev).is_ok(),
        "A3(a): the gate refused: {:?}",
        gate(&doc, &ev).err()
    );
}

/// **A3(b).** One copy, two OPERANDS: a mate read at the pattern
/// under `Instance(1)` and a mate read at a `Part` selecting that
/// same instance are two members over one body. They agree on
/// `instance` and on the whole copy chain and differ at
/// `Member::at`, which is enough to key them as two pairs — so the
/// second closes a loop and declares rather than folding into the
/// first.
#[test]
fn a3b_two_operands_over_one_copy_are_two_members() {
    let s = scene("msolve2-a3b");
    let (base, top) = (s.base, s.top);
    let (doc, pattern) = insert(s.doc, linear(top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(pattern, 1));
    let a = in_part(base, CapEnd::End);
    let b = in_copy(pattern, 1, in_part(top, CapEnd::Start));
    let at_pattern = SitedRef::new(pattern, b.clone());
    let at_part = SitedRef::new(part, b.clone());
    let (doc, m1) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(SitedRef::at_mint(a.clone()), at_pattern.clone(), FIRST_SEAT),
        },
    );
    let (doc, m2) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(SitedRef::at_mint(a), at_part.clone(), FIRST_SEAT),
        },
    );
    let (m1, m2) = (m1.unwrap(), m2.unwrap());
    let (mp, mq) = (
        member_of(&doc, &at_pattern).expect("a member at the pattern"),
        member_of(&doc, &at_part).expect("a member at the Part"),
    );
    assert_eq!(
        (mp.instance, &mp.copy),
        (mq.instance, &mq.copy),
        "one instance, one copy chain — the same body"
    );
    assert_ne!(mp.at, mq.at, "two operands: the members differ at `at`");
    assert_ne!(mp, mq, "and are therefore two members");
    // Both declare the same seat, so the pair is consistent: the loop
    // closes and the gate holds it.
    assert_loop_closes(&doc, &s.opts, m1, m2, true, "A3(b)");
}

/// **A3(c).** `Transform` over `Part { Instance(1) }` over a pattern,
/// the mate read at the TRANSFORM: the walk composes the transform's
/// map onto the copy's, passing the `Part` in between, and the copy
/// seats. This is the shape MSOLVE-1 could not build — a transform
/// over ONE copy of a pattern — because nothing then projected a
/// single body out of a pattern's instances.
#[test]
fn a3c_transform_over_part_over_a_pattern_seats() {
    let control = control_seat("msolve2-a3c");
    let s = scene("msolve2-a3c");
    let (base, top) = (s.base, s.top);
    let (doc, pattern) = insert(s.doc, linear(top, [0.0, -1.0, 0.0], 4.0, 2));
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
    let a = in_part(base, CapEnd::End);
    let b = in_copy(pattern, 1, in_part(top, CapEnd::Start));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(
                SitedRef::at_mint(a.clone()),
                SitedRef::new(moved, b.clone()),
                FIRST_SEAT,
            ),
        },
    );
    let mate = mate.unwrap();
    let member = member_of(&doc, &SitedRef::new(moved, b.clone())).expect("a member");
    assert_eq!(
        member.copy,
        vec![(pattern, 1)],
        "the transform contributes no copy — only the pattern does"
    );
    assert_eq!(member.at, moved);
    let poses = solve_document(&doc, Tol::witness());
    assert!(
        poses.fault(mate).is_none(),
        "A3(c): the solve refused: {:?}",
        poses.fault(mate)
    );
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
    let (base, top) = (s.base, s.top);
    let (doc, pattern) = insert(s.doc, linear(top, [0.0, -1.0, 0.0], 4.0, 3));
    // The Part selects copy 2; the name below says copy 1.
    let (doc, part) = insert(doc, part_of(pattern, 2));
    let a = in_part(base, CapEnd::End);
    let b = in_copy(pattern, 1, in_part(top, CapEnd::Start));
    let reference = SitedRef::new(part, b.clone());
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(SitedRef::at_mint(a), reference.clone(), FIRST_SEAT),
        },
    );
    let mate = mate.unwrap();
    // ADMISSION is structural and evaluates nothing, so the walk
    // still resolves this reference to a member: the disagreement is
    // a fact about two numbers, and numbers are the offset's half.
    assert!(
        member_of(&doc, &reference).is_some(),
        "the walk admits the reference; the offset is what refuses"
    );
    let fault = solve_document(&doc, Tol::witness())
        .fault(mate)
        .cloned()
        .expect("a disagreeing Part refuses");
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
    assert_eq!((*at, *side, *named_part), (mate, MateSide::B, part));
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

// ---- the gate, measured on a nested document (out of this unit's scope) ----

/// **What the at-rest gate says for a mate read BELOW the outer
/// pattern** — measured on a nested document, pinned, and reported
/// rather than fixed here.
///
/// `work/msolve/assembly-gate-refuses-vanished-on-a-mate-read-below-a-
/// pattern` found this one level down: the SOLVE places such a mate
/// correctly, and the gate then refuses `Reference { why: Vanished }`
/// because the bare name has no row in the product's table — only the
/// pattern is a root and its rows are `Instance(i)`-qualified. The
/// nested document says the same thing, and this row is the
/// measurement the unit owed: reading a mate at the `Part` under an
/// outer pattern reproduces it, and reading the same document's mate
/// AT the outer pattern (every other row here) does not, because
/// there the name is a root's own row.
///
/// Nothing here is worse than it was: the refusal is the same one,
/// raised for the same reason, at one more depth.
#[test]
fn the_gate_on_a_mate_read_below_the_outer_pattern_still_says_vanished() {
    let s = scene("msolve2-gate");
    let (base, top) = (s.base, s.top);
    let (doc, inner) = insert(s.doc, linear(top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(inner, 1));
    let (doc, outer) = insert(doc, linear(part, [0.0, 0.0, 1.0], LIFT, 2));
    let a = in_part(base, CapEnd::End);
    // Read at the `Part`, BELOW the outer pattern: the name stops at
    // the inner pattern's own `Instance(1)` row.
    let b = in_copy(inner, 1, in_part(top, CapEnd::Start));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(
                SitedRef::at_mint(a),
                SitedRef::new(part, b.clone()),
                FIRST_SEAT,
            ),
        },
    );
    let mate = mate.unwrap();
    let poses = solve_document(&doc, Tol::witness());
    assert!(
        poses.fault(mate).is_none(),
        "the solve places a mate read below the outer pattern: {:?}",
        poses.fault(mate)
    );
    assert_eq!(poses.role(mate), Some(MateRole::Determining));
    let ev = run(&doc, &s.opts);
    assert!(
        product(&doc, &ev, Tol::witness()).is_ok(),
        "the product gathers"
    );
    let err = gate(&doc, &ev).expect_err("the gate refuses the unrooted name");
    let AssemblyError::Reference { mate: at, side, .. } = &err else {
        panic!("expected the reference refusal, got {err:?}");
    };
    assert_eq!((*at, *side), (mate, MateSide::B));
    // The measurement, asserted rather than printed: the gate says
    // the name is absent from the product, not that the operand is
    // not a root — which is the mis-description the issue names.
    let said = err.to_string();
    assert!(
        said.contains("does not name a face of the product"),
        "the gate reports the name as vanished: {said}"
    );
    // Read AT the outer pattern instead, the same document gathers
    // and the gate holds: the difference is whether the name is a
    // product ROOT's own row.
    assert!(
        gate(&doc, &ev).is_err() && solve_document(&doc, Tol::witness()).fault(outer).is_none(),
        "the outer pattern itself carries no fault"
    );
}

// ---- A1′: the chain, deeper and with transforms interleaved ----

/// **A1′(a).** `Transform` over `Part(P₂, 2)` over `P₂` (linear)
/// over `Transform` over `Part(P₁, 1)` over `P₁` (circular) over the
/// instance, the mate read at the outermost transform.
///
/// Every map in the chain is non-identity and no adjacent pair
/// commutes, so the seat holds only if the fold composes all four
/// outermost-first onto the instance's pose. A2's rows fold at most
/// two; this one folds a chain that alternates the two placer kinds.
#[test]
fn a1b_two_levels_with_transforms_between_and_above_seat() {
    let control = control_seat("msolve2-a1b");
    let s = scene("msolve2-a1b");
    let (base, top) = (s.base, s.top);
    let (doc, p1) = circular(s.doc, top, [0.0, 0.0], Q, 4);
    let (doc, part1) = insert(doc, part_of(p1, 1));
    let (doc, t_mid) = insert(doc, xform(part1, [2.0, 0.0, 0.0], [0.0, 0.0, 1.0], Q));
    let (doc, p2) = insert(doc, linear(t_mid, [1.0, 0.0, 0.0], 4.0, 3));
    let (doc, part2) = insert(doc, part_of(p2, 2));
    let (doc, t_top) = insert(doc, xform(part2, [0.0, 3.0, 0.0], [0.0, 0.0, 1.0], Q));
    let a = in_part(base, CapEnd::End);
    let b = in_copy(p2, 2, in_copy(p1, 1, in_part(top, CapEnd::Start)));
    let r = SitedRef::new(t_top, b.clone());
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(SitedRef::at_mint(a.clone()), r.clone(), FIRST_SEAT),
        },
    );
    let mate = mate.unwrap();
    let m = member_of(&doc, &r).expect("a member through both transforms");
    assert_eq!((m.instance, m.at), (top, t_top));
    assert_eq!(
        m.copy,
        vec![(p2, 2), (p1, 1)],
        "the transforms contribute no copy — only the patterns do"
    );
    let poses = solve_document(&doc, Tol::witness());
    assert!(poses.fault(mate).is_none(), "{:?}", poses.fault(mate));
    assert_eq!(poses.role(mate), Some(MateRole::Determining));
    let ev = run(&doc, &s.opts);
    assert!(
        product(&doc, &ev, Tol::witness()).is_ok(),
        "the product gathers"
    );
    assert_seated(
        &doc,
        &ev,
        &a,
        &b,
        &control,
        "A1′(a) two levels + transforms",
    );
    assert_partition_agrees(&doc, "A1′(a)");
}

/// **A1′(b).** Three levels — circular, linear, circular — each with
/// a non-identity map, the mate read at the outermost pattern. The
/// member's chain is three pairs long and the fold composes three
/// maps.
#[test]
fn a1c_three_levels_deep_seat() {
    let control = control_seat("msolve2-a1c");
    let s = scene("msolve2-a1c");
    let (base, top) = (s.base, s.top);
    let (doc, p1) = circular(s.doc, top, [0.0, 0.0], Q, 4);
    let (doc, part1) = insert(doc, part_of(p1, 1));
    let (doc, p2) = insert(doc, linear(part1, [1.0, 0.0, 0.0], 4.0, 2));
    let (doc, part2) = insert(doc, part_of(p2, 1));
    let (doc, p3) = circular(doc, part2, [5.0, 5.0], std::f64::consts::FRAC_PI_3, 4);
    let a = in_part(base, CapEnd::End);
    let b = in_copy(
        p3,
        2,
        in_copy(p2, 1, in_copy(p1, 1, in_part(top, CapEnd::Start))),
    );
    let r = SitedRef::new(p3, b.clone());
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(SitedRef::at_mint(a.clone()), r.clone(), FIRST_SEAT),
        },
    );
    let mate = mate.unwrap();
    let m = member_of(&doc, &r).expect("a member three levels down");
    assert_eq!(m.copy, vec![(p3, 2), (p2, 1), (p1, 1)]);
    let poses = solve_document(&doc, Tol::witness());
    assert!(poses.fault(mate).is_none(), "{:?}", poses.fault(mate));
    let ev = run(&doc, &s.opts);
    assert!(
        product(&doc, &ev, Tol::witness()).is_ok(),
        "the product gathers"
    );
    assert_seated(&doc, &ev, &a, &b, &control, "A1′(b) three levels");
    assert_partition_agrees(&doc, "A1′(b)");
}

// ---- A2′: a loop whose TREE EDGE carries a non-identity outer map ----

/// `m1` names outer copy 1 and is the tree edge (the smaller member
/// key); `m2` names copy 2 and closes the loop. The outer rule is a
/// quarter turn about a `+z` axis at `axis_at`, so the tree edge's own
/// fold composes a ROTATION at the outer level — which A2's rows,
/// whose tree edges all name outer copy 0, never do.
///
/// With the axis at `(-2, 5)` copy 2 lands on the slab where `m2`
/// declares it; with it at `(-2, -4)` copy 2 swings clear and the
/// declaration is unmet.
fn rotating_outer_tree_edge(
    label: &str,
    axis_at: [f64; 2],
) -> (
    ProfileDoc,
    EvalOptions,
    RecipeNodeId,
    RecipeNodeId,
    StableName,
    StableName,
) {
    let s = scene(label);
    let (base, top) = (s.base, s.top);
    let (doc, inner) = insert(s.doc, linear(top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(inner, 1));
    let (doc, outer) = circular(doc, part, axis_at, Q, 3);
    let a = in_part(base, CapEnd::End);
    let master = in_part(top, CapEnd::Start);
    let named = |i2: u32| in_copy(outer, i2, in_copy(inner, 1, master.clone()));
    let (doc, m1) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(
                SitedRef::at_mint(a.clone()),
                SitedRef::new(outer, named(1)),
                FIRST_SEAT,
            ),
        },
    );
    let (doc, m2) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(
                SitedRef::at_mint(a.clone()),
                SitedRef::new(outer, named(2)),
                [8.5, 8.5, BASE_HEIGHT],
            ),
        },
    );
    (doc, s.opts, m1.unwrap(), m2.unwrap(), a, named(1))
}

/// **A2′.** Loop closure where the TREE EDGE's own member carries a
/// non-identity map at the OUTER level.
///
/// A2(a)–(c) all give their tree edge outer copy 0, whose map is the
/// identity by the stepped rule's construction — so the pose the
/// spanning tree folds never has an outer map in it, and only the
/// declaring side exercises the outer level. Here the tree edge is
/// outer copy 1 of a rotating rule: the placed copy seats (measured
/// on the product against the placer-free control), the second closes
/// the loop, and the gate holds the consistent pair and refuses the
/// inconsistent one.
#[test]
fn a2d_a_rotating_outer_map_on_the_tree_edge_seats_and_closes() {
    let control = control_seat("msolve2-a2d");
    for (label, axis, consistent) in [
        ("msolve2-a2d-consistent", [-2.0, 5.0], true),
        ("msolve2-a2d-inconsistent", [-2.0, -4.0], false),
    ] {
        let (doc, opts, m1, m2, a, b1) = rotating_outer_tree_edge(label, axis);
        assert_loop_closes(&doc, &opts, m1, m2, consistent, label);
        // And the TREE edge's own copy — the one whose chain carries
        // the quarter turn — seats where the alignment asks.
        let ev = run(&doc, &opts);
        assert_seated(&doc, &ev, &a, &b1, &control, label);
        assert_partition_agrees(&doc, label);
    }
}

// ---- A4′: the check is per reference, not per tree edge ----

/// **A4′(a) — a `Part` mismatch on a DECLARING mate refuses too.**
///
/// Two `Part`s over one pattern: `m1` (at `part1`, selects 0, names 0)
/// takes the tree edge and seats copy 0; `m2` (at `part2`, selects 2,
/// NAMES 0) is a different member — a different operand — so its pair
/// is not the tree edge and its offset is never derived.
///
/// While the check lived in the offset this document was SILENTLY
/// GREEN: `m2` solved `Declaring` with no fault, the product gathered,
/// and the gate said `Ok` because the copy `m2` NAMES really is seated
/// — while the body `part2` actually gathers floats off the slab with
/// a seat declared on it. Placed by the name, gathered by the `Part`.
/// The check runs per reference now, so `m2` refuses at the solve.
#[test]
fn a4b_a_part_mismatch_on_a_declaring_mate_refuses_too() {
    let s = scene("msolve2-a4b");
    let (base, top) = (s.base, s.top);
    let (doc, pattern) = insert(s.doc, linear(top, [0.0, -1.0, 0.0], 4.0, 3));
    let (doc, part1) = insert(doc, part_of(pattern, 0));
    let (doc, part2) = insert(doc, part_of(pattern, 2));
    let a = in_part(base, CapEnd::End);
    let b = in_copy(pattern, 0, in_part(top, CapEnd::Start));
    let (doc, m1) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(
                SitedRef::at_mint(a.clone()),
                SitedRef::new(part1, b.clone()),
                FIRST_SEAT,
            ),
        },
    );
    let (doc, m2) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(
                SitedRef::at_mint(a),
                SitedRef::new(part2, b.clone()),
                FIRST_SEAT,
            ),
        },
    );
    let (m1, m2) = (m1.unwrap(), m2.unwrap());
    // Both references are members: admission is structural and the
    // disagreement is about two numbers.
    assert!(member_of(&doc, &SitedRef::new(part2, b)).is_some());
    let poses = solve_document(&doc, Tol::witness());
    assert!(
        poses.fault(m1).is_none(),
        "the well-formed mate is untouched: {:?}",
        poses.fault(m1)
    );
    let fault = poses
        .fault(m2)
        .cloned()
        .expect("a declaring mate's Part is checked too");
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
    assert_eq!(poses.role(m2), Some(MateRole::Refused));
}

/// **A4′(b) — the index is EVALUATED, at the document's bindings.**
/// The `Part`'s index is a document parameter: at `k = 1` it agrees
/// with the name and the copy seats; setting `k = 2` refuses the same
/// mate typed, naming both indices — and the MEMBER is unchanged
/// across the edit, because admission is structural and a parameter
/// is not part of it.
#[test]
fn a4c_the_part_index_is_evaluated_at_the_documents_bindings() {
    let control = control_seat("msolve2-a4c");
    let s = scene("msolve2-a4c");
    let (base, top) = (s.base, s.top);
    let k = ParamName::new("k");
    let (doc, _) = step(
        s.doc,
        DocEdit::SetDocParam {
            name: k.clone(),
            value: DocParam::Count { value: 1 },
        },
    );
    let (doc, pattern) = insert(doc, linear(top, [0.0, -1.0, 0.0], 4.0, 3));
    let (doc, part) = insert(
        doc,
        Node::Part {
            of: pattern,
            select: PartSelect::Instance(Expr::param(k.clone(), Dimension::Count)),
        },
    );
    let a = in_part(base, CapEnd::End);
    let b = in_copy(pattern, 1, in_part(top, CapEnd::Start));
    let r = SitedRef::new(part, b.clone());
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(SitedRef::at_mint(a.clone()), r.clone(), FIRST_SEAT),
        },
    );
    let mate = mate.unwrap();
    let before = member_of(&doc, &r).expect("a member at k = 1");
    let poses = solve_document(&doc, Tol::witness());
    assert!(poses.fault(mate).is_none(), "{:?}", poses.fault(mate));
    let ev = run(&doc, &s.opts);
    assert_seated(&doc, &ev, &a, &b, &control, "A4′(b) at k = 1");

    let (doc, _) = step(
        doc,
        DocEdit::SetDocParamValue {
            name: k,
            value: DocParamValue::Count(2),
        },
    );
    let after = member_of(&doc, &r).expect("still a member at k = 2");
    assert_eq!(
        before, after,
        "the member does not move with a parameter: admission is structural"
    );
    let fault = solve_document(&doc, Tol::witness())
        .fault(mate)
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
    assert_eq!((p, named, selected, side), (part, 1, 2, MateSide::B));
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
    let (base, top) = (s.base, s.top);
    let (doc, inner_a) = insert(s.doc, linear(top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, inner_b) = insert(doc, linear(top, [1.0, 0.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(inner_b, 1));
    let (doc, outer) = insert(doc, linear(part, [0.0, 0.0, 1.0], 10.0, 2));
    // The name continues into `inner_a`; the chain runs through
    // `inner_b`.
    let b = in_copy(outer, 1, in_copy(inner_a, 1, in_part(top, CapEnd::Start)));
    let r = SitedRef::new(outer, b);
    assert!(member_of(&doc, &r).is_none(), "no member stands there");
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(SitedRef::at_mint(in_part(base, CapEnd::End)), r, FIRST_SEAT),
        },
    );
    let fault = solve_document(&doc, Tol::witness())
        .fault(mate.unwrap())
        .cloned()
        .expect("refuses");
    assert!(
        matches!(fault, MateFault::DanglingHead { head, .. } if head == inner_b),
        "the walk stops at the pattern it actually reached: {fault:?}"
    );
}

/// **A split half is a different body, not this one placed.**
/// `Part { SplitHalf }` is the other arm of the same node, and it is
/// NOT a member-vocabulary pass-through: the walk stops at it.
#[test]
fn a3e_a_part_naming_a_split_half_stops_the_walk() {
    let s = scene("msolve2-a3e");
    let (base, top) = (s.base, s.top);
    let (doc, pattern) = insert(s.doc, linear(top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, part) = insert(
        doc,
        Node::Part {
            of: pattern,
            select: PartSelect::SplitHalf(SplitHalf::Above),
        },
    );
    let b = in_copy(pattern, 1, in_part(top, CapEnd::Start));
    let r = SitedRef::new(part, b);
    assert!(member_of(&doc, &r).is_none(), "not a member");
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(SitedRef::at_mint(in_part(base, CapEnd::End)), r, FIRST_SEAT),
        },
    );
    let fault = solve_document(&doc, Tol::witness())
        .fault(mate.unwrap())
        .cloned()
        .expect("refuses");
    assert!(
        matches!(fault, MateFault::DanglingHead { head, .. } if head == part),
        "the walk stops at the Part itself: {fault:?}"
    );
}

// ---- what the gate reads, and what it does not ----

/// **The measurement behind [`LIFT`], pinned.** A declaring mate is
/// verified against the GEOMETRY: the gate asks whether the two faces
/// the mate NAMES are in contact, and the mate frames the solve folds
/// are no part of that question.
///
/// So lifting the authored `a` frame three units clear of the slab
/// leaves the gate green — the copy the second mate names still rests
/// where the tree edge put it. This is why every inconsistent variant
/// above is built by naming a copy the rule puts elsewhere, and the
/// row exists so that a later change making the gate read the frames
/// reds it rather than quietly inverting the argument.
#[test]
fn a_lifted_declared_frame_does_not_move_what_the_gate_reads() {
    let s = scene("msolve2-lifted");
    let (base, top) = (s.base, s.top);
    let (doc, inner) = insert(s.doc, linear(top, [0.0, -1.0, 0.0], 4.0, 2));
    let (doc, part) = insert(doc, part_of(inner, 1));
    let (doc, outer) = insert(doc, linear(part, ALONG.0, ALONG.1, 2));
    let a = in_part(base, CapEnd::End);
    let master = in_part(top, CapEnd::Start);
    let named = |i2: u32| in_copy(outer, i2, in_copy(inner, 1, master.clone()));
    let (doc, m1) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(
                SitedRef::at_mint(a.clone()),
                SitedRef::new(outer, named(0)),
                FIRST_SEAT,
            ),
        },
    );
    // The declaring mate's frame is authored THREE UNITS above the
    // slab's face — a seat the geometry does not keep.
    let (doc, m2) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_at(
                SitedRef::at_mint(a),
                SitedRef::new(outer, named(1)),
                [SECOND_SEAT[0], SECOND_SEAT[1], SECOND_SEAT[2] + 3.0],
            ),
        },
    );
    let (m1, m2) = (m1.unwrap(), m2.unwrap());
    let poses = solve_document(&doc, Tol::witness());
    assert_eq!(poses.role(m1), Some(MateRole::Determining));
    assert_eq!(poses.role(m2), Some(MateRole::Declaring));
    let ev = run(&doc, &s.opts);
    assert!(
        gate(&doc, &ev).is_ok(),
        "MEASURED: the gate reads the named FACES, not the authored frames, \
         so a frame lifted clear of the slab leaves it green: {:?}",
        gate(&doc, &ev).err()
    );
}

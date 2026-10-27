//! **Split and inline over placements** (ASSEMBLY.md A4, A11 (2)–(5);
//! the spec's `## P2-split` rows S2–S6, I1–I5 and R1).
//!
//! A cut takes whole placements and moves them verbatim: each keeps its
//! bundle in the part, and the remainder receives one instance pinned
//! to the world at the identity over its own geometry. A cut placement
//! whose bundle reads a kept copy severs that read; a kept mate whose
//! pose reads a cut instance's face crosses, its pose written afresh
//! over the instance's copies. Inline of an instance pinned to the world
//! splices its part's placements with every world mate moved by the
//! motion the pin solved to; an instance placed by other constraints
//! inlines over a part whose one placement on its world is an instance
//! pinned there at the identity, which takes the instance's bundle.
//! Every cut split admits, inlined back at the identity, returns the
//! document it was given up to node ids (R1).
//!
//! The scenes are `p2_gauges`'s two literal blocks; every pin is a
//! translation by dyadic lengths, so the evaluations compared agree to
//! the band a pin read off geometry stands at.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::Formula;
use std::sync::Arc;

use crate::fixture;
use crate::p2_gauges::{Parts, body_of, cut, min_corner, parts, placed_pair, points};

use editor_core::pose::PoseRole;
use editor_core::{
    CapEnd, DocEdit, DocRef, DocumentId, EvalOptions, Frame, InlineOutcome, Label, MateRef, Node,
    PartResolver, ProfileDoc, RecipeNodeId, SitedFace, SplitError, SplitOutcome, StableName, VarId,
};
use fixture::resolver::{PartName, PartStore, in_part, with_resolver};
use fixture::round_trip::{composed, same_up_to_ids};
use fixture::{head, insert, offset_of, run, step};
use geom_core::Tol;

// ---- shared seats and pins ----

/// **`mover` seated on `onto` at `at`** (A5): `mover`'s plane rests on
/// `onto`'s, outward normals opposed, and three values pin what the rest
/// leaves ([`crate::p2_gauges::seat_on`]). The rest's address.
pub(crate) fn seat_at(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    mover: SitedFace,
    onto: SitedFace,
    at: [f64; 2],
) -> (ProfileDoc, MateRef) {
    crate::p2_gauges::seat_on(doc, placement, mover, onto, at)
}

/// [`seat_at`] at `(1, 1)`: "mate the top to the base".
pub(crate) fn seated(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    mover: SitedFace,
    onto: SitedFace,
) -> (ProfileDoc, MateRef) {
    crate::p2_gauges::seat(doc, placement, mover, onto)
}

/// **An instance of `part` pinned to the world at the translation
/// `at`** over its own geometry ([`fixture::pin`]).
pub(crate) fn pinned_at(
    doc: ProfileDoc,
    part: DocRef,
    at: [f64; 3],
    o: &EvalOptions,
) -> (ProfileDoc, RecipeNodeId) {
    crate::p2_gauges::pinned(doc, part, Frame::translation(at), o)
}

/// **An instance of `part` pinned to the world turned `angle` about z
/// and lifted by the document's `lift`**: the identity pin over its
/// geometry with the two world moves appended, so the pin reads the
/// parameter.
pub(crate) fn pinned_lifted(
    doc: ProfileDoc,
    part: DocRef,
    along: PoseRole,
    angle: f64,
    o: &EvalOptions,
) -> (ProfileDoc, RecipeNodeId) {
    use editor_core::{Coord, TargetAxis};
    let _ = o;
    let (doc, id) = insert(doc, fixture::mated_instance(part));
    // Spun `angle` about the world's z, then lifted along `along` by the
    // variable, by values alone.
    let axis = match along {
        PoseRole::X => TargetAxis::X,
        PoseRole::Y => TargetAxis::Y,
        _ => TargetAxis::Z,
    };
    let mut values = vec![(
        Coord::Slide(axis),
        Formula::named(crate::p2_gauges::lift(), editor_core::Dimension::Length),
    )];
    if angle != 0.0 {
        values.push((Coord::Spin(Some(TargetAxis::Z)), fixture::ang(angle)));
    }
    let placed = editor_core::Placed::unmated(values);
    (fixture::written(doc, id, placed), id)
}

/// **`placement`'s bundle replaced by one pin at the translation `at`**.
pub(crate) fn repin(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    at: [f64; 3],
    o: &EvalOptions,
) -> ProfileDoc {
    crate::p2_gauges::set_pin(doc, placement, Some(Frame::translation(at)), o)
}

/// Whether `placement` solves to the identity of `doc`'s world.
pub(crate) fn at_identity(doc: &ProfileDoc, placement: RecipeNodeId, o: &EvalOptions) -> bool {
    fixture::solve(doc, o, Tol::witness()).at_identity(placement)
}

// ---- substrate ----

/// **An instance of `part`, pinned to `doc`'s world at the identity**
/// (A10): each instance here is in the product.
fn instance(doc: ProfileDoc, part: DocRef, o: &EvalOptions) -> (ProfileDoc, RecipeNodeId) {
    fixture::insert_pinned(doc, crate::fixture::mated_instance(part), o)
}

fn split(
    doc: &ProfileDoc,
    ids: &[RecipeNodeId],
    label: &str,
    o: &EvalOptions,
) -> Result<SplitOutcome, SplitError> {
    editor_core::split(
        doc,
        &cut(doc, ids),
        DocumentId::derive(&format!("{label}-part")),
        Tol::witness(),
        o.resolver.as_ref(),
    )
}

fn inline(doc: &ProfileDoc, instance: RecipeNodeId, store: &PartStore) -> InlineOutcome {
    let r: Arc<dyn PartResolver> = Arc::new(store.clone());
    editor_core::inline(doc, instance, &r, Tol::witness())
        .unwrap_or_else(|e| panic!("inline refused: {e}"))
}

/// The whole document's product: its corners and volume.
fn extent(doc: &ProfileDoc, o: &EvalOptions) -> ([f64; 3], [f64; 3], f64) {
    let body = topo::test_support::finished(
        "body",
        crate::fixture::product_in_world(doc, &run(doc, o), Tol::witness()).expect("gathers"),
        Tol::witness(),
    );
    let pts = points(&body);
    let lo = pts.iter().fold([f64::INFINITY; 3], |m, p| {
        [m[0].min(p[0]), m[1].min(p[1]), m[2].min(p[2])]
    });
    let hi = pts.iter().fold([f64::NEG_INFINITY; 3], |m, p| {
        [m[0].max(p[0]), m[1].max(p[1]), m[2].max(p[2])]
    });
    let volume = topo::test_support::pinned_volume("mass properties", &body, Tol::witness()).volume;
    (lo, hi, volume)
}

fn same_extent(a: ([f64; 3], [f64; 3], f64), b: ([f64; 3], [f64; 3], f64), what: &str) {
    let near = |x: [f64; 3], y: [f64; 3]| x.iter().zip(y).all(|(p, q)| (p - q).abs() <= 1e-9);
    assert!(
        near(a.0, b.0) && near(a.1, b.1) && (a.2 - b.2).abs() <= 1e-9,
        "{what}: {a:?} vs {b:?}"
    );
}

/// **`name`, an entity of a copy in `part`, as `part`'s product names
/// it** (A10): a copy carries its source's names as they are, so a lone
/// copy's product names it as it is, and a `⊔` of copies names it by
/// the read it takes the copy through — an instance's own copy, or the
/// copy of the one `Place` in `part` that copies its minting node.
fn in_world_of(part: &ProfileDoc, name: impl crate::fixture::Sited) -> StableName {
    let site = name.site();
    let name = name.name();
    let copy = if matches!(part.node(site), Some(Node::InstantiatePart { .. })) {
        site
    } else {
        let copies: Vec<RecipeNodeId> = part
            .placements()
            .into_iter()
            .filter(|&at| {
                matches!(part.node(at), Some(Node::Place { source, .. })
                    if (part.operation_of(source.read) == Some(name.node)))
            })
            .collect();
        let [copy] = copies[..] else {
            panic!("one placement places {:?}, not {copies:?}", name.node);
        };
        copy
    };
    let product = part
        .product()
        .and_then(|var| part.operation_of(var))
        .expect("the part has a product");
    match part.node(product) {
        Some(Node::Assemble { .. }) => StableName {
            kind: name.kind,
            node: product,
            path: vec![editor_core::RoleSeg::From {
                read: part.output(copy, 0).expect("the copy"),
                of: name.into(),
            }],
        },
        _ => name,
    }
}

/// **`name`, as `part`'s product names it, landed by an inline**: the
/// name [`in_world_of`] gives, its `⊔` and the read it takes the copy
/// through mapped onto the nodes and outputs the inline spliced.
fn landed(
    out: &editor_core::InlineOutcome,
    part: &ProfileDoc,
    name: impl crate::fixture::Sited,
) -> StableName {
    let name = in_world_of(part, name);
    let node = out.node_map.get(&name.node).copied().unwrap_or(name.node);
    let path = match name.path.as_slice() {
        [editor_core::RoleSeg::From { read, of }] => {
            let (at, port) = part.defined_by(*read).expect("the read is an output");
            vec![editor_core::RoleSeg::From {
                read: out
                    .doc
                    .output(out.node_map[&at], port)
                    .expect("the spliced copy"),
                of: of.clone(),
            }]
        }
        _ => name.path.clone(),
    };
    StableName {
        kind: name.kind,
        node,
        path,
    }
}

/// `name` as a host spells a face of its part through `instance`: the
/// part's own name, which the instance's copy carries verbatim.
fn wrap(instance: RecipeNodeId, name: impl Into<StableName>) -> crate::fixture::resolver::PartName {
    crate::fixture::resolver::PartName {
        at: instance,
        name: name.into(),
    }
}

/// The document `instance` in `doc` references.
fn part_id_of(doc: &ProfileDoc, instance: RecipeNodeId) -> DocumentId {
    match doc.node(instance) {
        Some(Node::InstantiatePart { doc_ref, .. }) => doc_ref.id,
        other => panic!("an instance: {other:?}"),
    }
}

/// The base block's bottom cap at `base`.
fn base_bottom(p: &Parts, base: RecipeNodeId) -> crate::fixture::resolver::PartName {
    in_part(base, p.base_body, CapEnd::Start)
}

/// The face name a stored mate's pose reads: its one selection's.
fn read_name(doc: &ProfileDoc, pose: VarId) -> StableName {
    let [select] = fixture::face_reads(doc, pose)[..] else {
        panic!("a pose of one face")
    };
    fixture::side_name(doc, select)
}

/// The pose a stored constraint's target reads.
fn target_of(c: &editor_core::Constraint) -> VarId {
    c.mate().expect("a mate").to
}

/// The pose a stored constraint's own side reads.
fn on_of(c: &editor_core::Constraint) -> VarId {
    c.mate().expect("a mate").on
}

/// The one mate `placement`'s bundle holds, beside the values on it.
fn only_constraint(doc: &ProfileDoc, placement: RecipeNodeId) -> editor_core::Constraint {
    let bundle: Vec<_> = fixture::bundle(doc, placement)
        .into_iter()
        .filter(|c| c.mate().is_some())
        .collect();
    let [one] = bundle.as_slice() else {
        panic!("{placement:?} holds one mate, not {bundle:?}");
    };
    one.clone()
}

/// **R1**: `doc` split at `ids`, the instance inlined back at the
/// identity it sits at, and the placement inline wrote flattened
/// ([`editor_core::flatten`]) is `doc` up to node ids.
fn round_trip(doc: &ProfileDoc, ids: &[RecipeNodeId], p: &Parts, label: &str) -> SplitOutcome {
    let out = split(doc, ids, label, &p.opts()).unwrap_or_else(|e| panic!("{label}: {e}"));
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    assert!(
        offset_of(&out.remainder, out.instance).is_some()
            && at_identity(&out.remainder, out.instance, &with_resolver(store.clone())),
        "{label}: the instance a split leaves is pinned to the world at the identity"
    );
    let back = inline(&out.remainder, out.instance, &store);
    let flat = editor_core::flatten(&back.doc, back.placement, Tol::witness())
        .unwrap_or_else(|e| panic!("{label}: the identity placement flattens: {e:?}"));
    let (map, steps) = composed(doc, &out, &back);
    same_up_to_ids(doc, &flat.doc, &map, &steps)
        .unwrap_or_else(|e| panic!("{label}: inline(split(d)) is d up to node ids:\n{e}"));
    out
}

// ---- S2: a cut moved verbatim ----

/// **A cut of pinned instances moves verbatim** (A4): two instances
/// pinned to the world, one labelled. Each keeps its world mate in the
/// part, the label follows its node, the evaluation is unchanged, a cut
/// face resolves through the instance qualifier, and the round trip is
/// the document up to node ids.
#[test]
fn s2_a_cut_of_pinned_instances_moves_verbatim_and_round_trips() {
    let p = parts("s2-verbatim");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("s2-verbatim"), Tol::witness());
    let (doc, low) = pinned_at(doc, p.base, [2.0, 8.0, 4.5], &o);
    let (doc, high) = pinned_at(doc, p.top, [16.0, 8.0, 0.0], &o);
    let (doc, _) = step(
        doc,
        DocEdit::SetLabel {
            node: low,
            label: Some(Label::new("bench").expect("a label")),
        },
    );
    let out = round_trip(&doc, &[low, high], &p, "s2-verbatim");
    for id in [low, high] {
        assert!(
            offset_of(&doc, id).is_some()
                && fixture::same_offset(&out.part, out.node_map[&id], &doc, id),
            "{id:?} keeps its world mate in the part"
        );
    }
    assert_eq!(
        out.part.label(out.node_map[&low]).map(ToString::to_string),
        Some("bench".to_owned()),
        "the label follows its node"
    );
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let split_o = with_resolver(store);
    let (before, after) = (extent(&doc, &o), extent(&out.remainder, &split_o));
    assert_eq!(
        before.2.to_bits(),
        after.2.to_bits(),
        "split-then-evaluate keeps the volume"
    );
    same_extent(before, after, "split-then-evaluate keeps the material");
    // Name resolution: a cut face resolves before, and through the
    // instance qualifier after.
    // A part's name resolves in the copy of the instance it is read at.
    let resolves = |d: &ProfileDoc, o: &EvalOptions, name: &PartName| {
        let eval = run(d, o);
        let read = d.read_of_node(name.at).expect("the instance's copy");
        matches!(
            editor_core::resolve(
                editor_core::RunCtx {
                    doc: d,
                    eval: &eval
                },
                read,
                &name.name
            )
            .expect("the copy holds a body"),
            editor_core::Resolution::Resolved(_)
        )
    };
    for (face, in_part) in [
        (p.base_cap(low), p.base_cap(out.node_map[&low])),
        (p.top_cap(high), p.top_cap(out.node_map[&high])),
    ] {
        assert!(
            resolves(&doc, &o, &face),
            "{face:?} resolves before the split"
        );
        let through = wrap(out.instance, in_world_of(&out.part, in_part));
        assert!(
            resolves(&out.remainder, &split_o, &through),
            "{through:?} resolves after the split"
        );
    }
}

/// **A placement inserted before the copy its bundle reads is carried
/// whole**: a top inserted first, then the base, then the top mated
/// onto the base. The round trip is the document, and an inline of a
/// part holding the same order carries the top's constraint onto the
/// spliced base.
#[test]
fn s2_a_placement_inserted_before_its_target_round_trips() {
    let p = parts("s2-order");
    let o = p.opts();
    let build = |label: &str| {
        let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
        let (doc, early) = insert(doc, fixture::mated_instance(p.top));
        let (doc, base) = instance(doc, p.base, &o);
        let (doc, _) = seated(doc, early, head(p.top_cap(early)), head(p.base_cap(base)));
        (doc, early, base)
    };
    let (doc, early, base) = build("s2-order");
    round_trip(&doc, &[early, base], &p, "s2-order");

    // A part document holding the same order, inlined at the identity.
    let mut store = p.store.clone();
    let part_ref = store.insert(build("s2-order-sub").0, Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("s2-order-host"), Tol::witness());
    let (host, h) = instance(host, part_ref, &with_resolver(store.clone()));
    let back = inline(&host, h, &store);
    let carried = only_constraint(&back.doc, back.node_map[&early]);
    assert_eq!(
        read_name(&back.doc, target_of(&carried)),
        p.base_cap(back.node_map[&base]),
        "the carried constraint reads the spliced base"
    );
}

// ---- S3: the severed read ----

/// **A cut placement whose bundle reads a kept copy severs that read**
/// (A4's cut rule): a top mated onto a kept base, cut alone, refuses
/// `SeveredEdge` naming the top as the cut consumer, with a recourse.
#[test]
fn s3_a_cut_placement_reading_a_kept_copy_refuses_severed_edge() {
    let (p, doc, [base, top], _) = placed_pair("s3-severed");
    let err = split(&doc, &[top], "s3-severed", &p.opts()).expect_err("a severed read");
    assert!(
        matches!(&err, SplitError::SeveredEdge { consumer, input, consumer_is_cut: true, .. }
            if consumer.id() == top && input.id() == base),
        "{err:?}"
    );
    assert!(err.to_string().contains("Recourse:"), "{err}");
}

// ---- S5: a kept mate reading the cut ----

/// **A kept mate reading a cut instance's face crosses wherever that
/// instance stands** (A4): the cut holds a base pinned at the identity
/// and one pinned at `[8, 0, 0]`. A kept top seated on either base
/// crosses — its target written afresh over the instance's copies — and
/// stands where it stood: the seat's values are charted on the target
/// space, the world on both sides of the split, not on the base it
/// reads.
///
/// *Breaks if* the crossing keeps reading the cut node or moves the top,
/// or a seat on a copy off the identity is charted on that copy.
#[test]
fn s5_a_kept_mate_reading_a_cut_instance_crosses_wherever_it_stands() {
    let p = parts("s5-frame");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("s5-frame"), Tol::witness());
    let (doc, shifted) = pinned_at(doc, p.base, [8.0, 0.0, 0.0], &o);
    let (doc, at_identity) = instance(doc, p.base, &o);
    let (doc, kept) = insert(doc, fixture::mated_instance(p.top));
    let seat_on = |base: RecipeNodeId| {
        seated(
            doc.clone(),
            kept,
            head(p.top_cap(kept)),
            head(p.base_cap(base)),
        )
        .0
    };

    for base in [at_identity, shifted] {
        let mated = seat_on(base);
        let before = min_corner(&body_of(&run(&mated, &o), kept));
        let out = split(&mated, &[shifted, at_identity], "s5-frame", &o)
            .unwrap_or_else(|e| panic!("the kept mate crosses: {e}"));
        let carried = only_constraint(&out.remainder, kept);
        let [select] = fixture::face_reads(&out.remainder, target_of(&carried))[..] else {
            panic!("the target reads one face")
        };
        assert_eq!(
            out.remainder.selection(select).map(|s| s.body),
            out.remainder.output(out.instance, 0),
            "the kept mate's target reads the instance's copies"
        );
        let mut store = p.store.clone();
        store.insert(out.part.clone(), Tol::witness());
        let after = min_corner(&body_of(&run(&out.remainder, &with_resolver(store)), kept));
        crate::p2_gauges::close(before, after, "the top is where it was");
    }
}

// ---- I1–I3: inline at a pose ----

/// A part of two lone bases, the second pinned at `[16, 0, 0]`, and its
/// first base, pinned at the identity.
fn two_pinned(p: &Parts, label: &str) -> (ProfileDoc, RecipeNodeId) {
    let o = p.opts();
    let sub = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (sub, first) = instance(sub, p.base, &o);
    let (sub, _) = pinned_at(sub, p.base, [16.0, 0.0, 0.0], &o);
    (sub, first)
}

/// A host instance of `sub` pinned to the world at `[2, 8, 4]`, and the
/// store holding `sub`.
fn host_of(p: &Parts, sub: ProfileDoc, label: &str) -> (ProfileDoc, PartStore, RecipeNodeId) {
    let mut store = p.store.clone();
    let sub = crate::fixture::assemble_world_copies(sub);
    let sub_ref = store.insert(sub, Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, h) = pinned_at(doc, sub_ref, [2.0, 8.0, 4.0], &with_resolver(store.clone()));
    (doc, store, h)
}

/// **Inline at a pose places the part's world under the instance's
/// bundle** (A4, one rule): an instance pinned at `[2, 8, 4]`, over a
/// part of two pinned instances, over a part whose one instance sits
/// off the identity, and over a part whose one instance is pinned up,
/// splices the part's placements into a new space verbatim, each where
/// it stood in the part, and places their copies into the host's world
/// under the instance's own pin, last; each copy lands where it stood
/// in the world, and the material is where it was.
///
/// *Breaks if* a spliced placement is moved by the instance's motion,
/// or the new placement composes anything but the instance's bundle.
#[test]
fn i1_inline_at_a_pose_places_the_parts_world_under_the_instances_bundle() {
    let p = parts("i1-pose");
    let o = p.opts();
    // Each part, with where each of its instances' lowest corner lands
    // in the world: the host pose at [2, 8, 4], then the part's own.
    let two = {
        let sub = ProfileDoc::empty(DocumentId::derive("i1-two"), Tol::witness());
        let (sub, first) = instance(sub, p.base, &o);
        let (sub, second) = pinned_at(sub, p.base, [16.0, 0.0, 0.0], &o);
        (
            sub,
            vec![(first, [2.0, 8.0, 4.0]), (second, [18.0, 8.0, 4.0])],
        )
    };
    let shifted = {
        let sub = ProfileDoc::empty(DocumentId::derive("i1-shifted"), Tol::witness());
        let (sub, shifted) = pinned_at(sub, p.base, [1.0, 0.0, 0.0], &o);
        (sub, vec![(shifted, [3.0, 8.0, 4.0])])
    };
    let raised = {
        let sub = ProfileDoc::empty(DocumentId::derive("i1-raised"), Tol::witness());
        let (sub, up) = pinned_at(sub, p.base, [0.0, 0.0, 2.0], &o);
        (sub, vec![(up, [2.0, 8.0, 6.0])])
    };
    for ((sub, corners), what) in [
        (two, "two pinned instances"),
        (shifted, "an instance off the identity"),
        (raised, "an instance pinned up"),
    ] {
        let (doc, store, h) = host_of(&p, sub.clone(), &format!("i1-host-{what}"));
        let out = inline(&doc, h, &store);
        assert!(out.doc.node(h).is_none(), "{what}: the instance is gone");
        // The new space: the part's placements, carried verbatim in its
        // own order; the new placement of their copies comes last.
        let spliced: Vec<RecipeNodeId> = sub.placements().iter().map(|r| out.node_map[r]).collect();
        for (&old, &new) in sub.placements().iter().zip(&spliced) {
            assert!(
                fixture::same_offset(&out.doc, new, &sub, old),
                "{what}: {old:?} stands in the new space as it stood in the part"
            );
        }
        assert_eq!(
            out.doc.placements().last(),
            Some(&out.placement),
            "{what}: the new placement comes last"
        );
        assert!(
            fixture::same_offset(&out.doc, out.placement, &doc, h),
            "{what}: under the instance's own pin"
        );
        let o = with_resolver(store);
        let ev = run(&out.doc, &o);
        let stood = match &ev.value(out.placement).expect("the copy evaluates").payload {
            editor_core::ValuePayload::Body(body) => crate::fixture::in_space(body)
                .vertex_points()
                .fold([f64::INFINITY; 3], |m, (_, p)| {
                    [m[0].min(p.x), m[1].min(p.y), m[2].min(p.z)]
                }),
            other => panic!(
                "{what}: the new placement holds a copy: {}",
                other.kind_name()
            ),
        };
        let lowest = corners.iter().fold([f64::INFINITY; 3], |m, (_, c)| {
            [m[0].min(c[0]), m[1].min(c[1]), m[2].min(c[2])]
        });
        crate::p2_gauges::close(stood, lowest, &format!("{what}: the copies' world pose"));
        same_extent(
            extent(&doc, &o),
            extent(&out.doc, &o),
            &format!("{what}: inline moves no material"),
        );
    }
}

/// A host whose instance of a two-instance part, pinned at a pose, has a
/// top mated onto the part's first base: `[h, top, first]`.
fn member_through_a_mate(p: &Parts, label: &str) -> (ProfileDoc, PartStore, [RecipeNodeId; 3]) {
    let (sub, first) = two_pinned(p, &format!("{label}-sub"));
    let inner = in_world_of(&sub, p.base_cap(first));
    let (doc, store, h) = host_of(p, sub, label);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = seated(doc, top, head(p.top_cap(top)), head(wrap(h, inner)));
    (doc, store, [h, top, first])
}

/// **A host constraint reading the instance crosses onto the spliced
/// copy** (A4): a top mated onto the instance's first base, which the
/// part pins at the identity, reads the spliced base after the inline
/// and is where it was. The edit list replays to the result.
#[test]
fn i2_a_host_constraint_reading_the_instance_crosses_onto_the_spliced_copy() {
    let p = parts("i2-crossed");
    let (doc, store, [h, top, first]) = member_through_a_mate(&p, "i2-crossed");
    let o = with_resolver(store.clone());
    let before = min_corner(&body_of(&run(&doc, &o), top));
    let out = inline(&doc, h, &store);
    let carried = only_constraint(&out.doc, top);
    let sub = store.doc(part_id_of(&doc, h));
    assert_eq!(
        read_name(&out.doc, target_of(&carried)),
        landed(&out, &sub, p.base_cap(first)),
        "the constraint reads the spliced base's copy in the host's world"
    );
    let after = min_corner(&body_of(&run(&out.doc, &o), top));
    crate::p2_gauges::close(before, after, "the top is where it was");
    same_extent(extent(&doc, &o), extent(&out.doc, &o), "no material moves");
    // The edit list is the record: it replays with no reach and no
    // solve to the same document.
    let replayed = out.edits.iter().fold(doc, |d, e| {
        editor_core::apply_replayed(&d, e, Tol::witness())
            .unwrap_or_else(|err| panic!("{e:?} replays: {err}"))
            .doc
    });
    assert!(replayed.bit_eq(&out.doc), "the list replays to the result");
}

// ---- I4–I5: the mate-placed inline ----

/// **A part whose one placement on its world is a base at `base_at`**
/// (at the identity, an instance pinned there with no pose of its own),
/// with a top mated onto it, under the document id `label`: the part,
/// its base and its top.
fn pair_part(p: &Parts, label: &str, base_at: [f64; 3]) -> (ProfileDoc, [RecipeNodeId; 2]) {
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = pinned_at(doc, p.base, base_at, &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = seated(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    (doc, [base, top])
}

/// A part of one base pinned to its world at the identity and a top
/// mated onto it, in a store, its base and top, and a host in which a
/// tall block `ht`, pinned first, places an instance `i` of that part
/// through a mate reading the part's base bottom; that mate's address.
fn mate_placed(
    label: &str,
) -> (
    Parts,
    PartStore,
    ProfileDoc,
    [RecipeNodeId; 4],
    editor_core::MateRef,
) {
    let p = parts(label);
    let (part, [part_base, part_top]) = pair_part(&p, &format!("{label}-part"), [0.0; 3]);
    let bottom = in_world_of(&part, base_bottom(&p, part_base));
    let mut store = p.store.clone();
    let part = crate::fixture::assemble_world_copies(part);
    let part_ref = store.insert(part, Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive(&format!("{label}-host")), Tol::witness());
    let (host, ht) = instance(host, p.top, &with_resolver(store.clone()));
    let (host, i) = insert(host, fixture::mated_instance(part_ref));
    let (host, m) = seat_at(
        host,
        i,
        head(wrap(i, bottom)),
        head(p.top_upper_cap(ht)),
        [0.0, 0.0],
    );
    (p, store, host, [ht, i, part_base, part_top], m)
}

/// **The mate-placed inline** (A4, one rule): an instance its
/// constraint places on a host block, over a part whose placements on
/// its world are a base pinned at the identity and a top mated onto it,
/// inlines. The part's placements land in a new space verbatim, the base
/// keeping its own pin, and the instance becomes a placement of their
/// copies under its own bundle, whose mate reads the spliced base. The
/// material is where it was. A mate reading the part's top instead
/// inlines the same way, reading the spliced top.
///
/// *Breaks if* the base's pin gives way to the instance's bundle, the
/// new placement's mate reads anything but the spliced copy it read, or
/// material moves.
#[test]
fn i4_a_mate_placed_instance_inlines_under_its_own_bundle() {
    let (p, store, host, [ht, i, part_base, part_top], _) = mate_placed("i4-mate-placed");
    let o = with_resolver(store.clone());
    let part = store.doc(part_id_of(&host, i));
    let out = inline(&host, i, &store);
    let ev = run(&out.doc, &o);
    assert!(
        ev.node_error(out.placement).is_none(),
        "the new placement evaluates: {:?}",
        ev.node_error(out.placement)
    );
    let spliced = out.node_map[&part_base];
    assert_eq!(
        offset_of(&out.doc, spliced),
        offset_of(&part, part_base),
        "the base keeps its own pin"
    );
    let taken = only_constraint(&out.doc, out.placement);
    assert_eq!(
        read_name(&out.doc, on_of(&taken)),
        landed(&out, &part, base_bottom(&p, part_base)),
        "the instance's mate reads the spliced base"
    );
    same_extent(extent(&host, &o), extent(&out.doc, &o), "no material moves");

    // Through the top, the part's other placement on its world.
    let top_cap = in_world_of(&part, p.top_cap(part_top));
    let (through_top, _) = seat_at(
        host,
        i,
        head(wrap(i, top_cap)),
        head(p.top_upper_cap(ht)),
        [0.0, 0.0],
    );
    let out = inline(&through_top, i, &store);
    let taken = only_constraint(&out.doc, out.placement);
    assert_eq!(
        fixture::side_name(&out.doc, fixture::face_reads(&out.doc, on_of(&taken))[0]),
        landed(&out, &part, p.top_cap(part_top)),
        "the instance's mate reads the spliced top"
    );
    same_extent(
        extent(&through_top, &o),
        extent(&out.doc, &o),
        "no material moves",
    );
}

/// **The mate-placed inline asks nothing of the part's numbers** (A4,
/// one rule): over a part whose base sits off the identity, and over a
/// part of two pinned bases, the instance inlines, and no material
/// moves.
///
/// *Breaks if* inline refuses a part for where its placements stand.
#[test]
fn i5_a_mate_placed_instance_over_any_part_inlines() {
    let (p, store, host, [ht, i, _, _], _) = mate_placed("i5-mate-placed");
    let (shifted, [shifted_base, _]) = pair_part(&p, "i5-mate-placed-part", [1.0, 0.0, 0.0]);
    let shifted = crate::fixture::assemble_world_copies(shifted);
    // The shifted part's product is a `⊔` minted after its own edits,
    // so the host names its faces anew in the new version
    // (`work/intent/a-name-through-a-parts-assemble-moves-with-every-earlier-part-edit.md`).
    let shifted_bottom = in_world_of(&shifted, base_bottom(&p, shifted_base));
    let mut shifted_store = store.clone();
    let shifted_ref = shifted_store.insert(shifted, Tol::witness());
    let (repinned, _) = step(
        host.clone(),
        DocEdit::UpdateReference {
            node: i,
            new_pin: shifted_ref.pin,
        },
    );
    let (repinned, _) = seat_at(
        repinned,
        i,
        head(wrap(i, shifted_bottom)),
        head(p.top_upper_cap(ht)),
        [0.0, 0.0],
    );
    let out = inline(&repinned, i, &shifted_store);
    let o = with_resolver(shifted_store.clone());
    same_extent(
        extent(&repinned, &o),
        extent(&out.doc, &o),
        "a part off the identity: no material moves",
    );

    let (two, first) = two_pinned(&p, "i5-mate-placed-part");
    let first_bottom = in_world_of(&two, base_bottom(&p, first));
    let mut two_store = p.store.clone();
    let two_ref = two_store.insert(crate::fixture::assemble_world_copies(two), Tol::witness());
    let (other, _) = step(
        host,
        DocEdit::UpdateReference {
            node: i,
            new_pin: two_ref.pin,
        },
    );
    let (other, _) = seat_at(
        other,
        i,
        head(wrap(i, first_bottom)),
        head(p.top_upper_cap(ht)),
        [0.0, 0.0],
    );
    let out = inline(&other, i, &two_store);
    let o = with_resolver(two_store.clone());
    same_extent(
        extent(&other, &o),
        extent(&out.doc, &o),
        "a part of two placements: no material moves",
    );
}

// ---- R1: the round trip ----

/// **A cut of placements, one pinned at a parametric pose, moved
/// verbatim and inlined at the identity, is the document it was given
/// up to node ids** (R1): a base pinned at `[0, 0, lift]`, a top mated
/// onto it, and a further pinned base, beside a kept instance. The
/// parameter moves with the cut. The comparator is not vacuous: one
/// pin changed fails it.
#[test]
fn r1_a_cut_with_a_parametric_pin_round_trips_exactly() {
    let p = parts("r1-parametric");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-parametric"), Tol::witness());
    let doc = crate::p2_gauges::declare_lift(doc, 0.5);
    // The kept instance comes first: split appends its instance and
    // inline its splice, so the cut's placements, listed last, come
    // back in their order.
    let (doc, _kept) = pinned_at(doc, p.top, [0.0, 8.0, 0.0], &o);
    let (doc, base) = pinned_lifted(doc, p.base, PoseRole::Z, 0.0, &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = seated(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    let (doc, deep) = pinned_at(doc, p.base, [4.0, 8.0, 0.0], &o);
    let out = round_trip(&doc, &[base, top, deep], &p, "r1-parametric");
    assert!(
        out.part
            .var_named(crate::p2_gauges::lift().as_str())
            .is_some(),
        "the pin's parameter moves with it"
    );

    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let back = inline(&out.remainder, out.instance, &store);
    let (map, steps) = composed(&doc, &out, &back);
    let moved = repin(back.doc, map[&deep], [4.0, 8.5, 0.0], &with_resolver(store));
    assert!(
        same_up_to_ids(&doc, &moved, &map, &steps).is_err(),
        "the comparator reads pins"
    );
}

// ---- No material ----

/// **A cut that holds no material refuses** (A4: split-then-evaluate
/// equals the unsplit evaluation): a spare profile beside placed
/// material refuses `NoMaterial` naming it, with a recourse, rather
/// than leave an instance of a part with no body.
#[test]
fn a_cut_of_a_profile_alone_refuses_no_material() {
    let p = parts("no-material");
    let o = p.opts();
    let (block, body) = crate::p2_gauges::block("no-material-profile", 2.0, 1.0);
    let block = fixture::place(block, body).0;
    let (with_profile, spare) = insert(
        block.clone(),
        Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, 0.5)])),
    );
    let err = split(&with_profile, &[spare], "no-material", &o).expect_err("a spare profile");
    assert!(
        matches!(&err, SplitError::NoMaterial { node } if node.id() == spare),
        "{err:?}"
    );
    assert!(err.to_string().contains("Recourse:"), "{err}");
}

/// **A chain of constraints authored backwards round-trips**: a top,
/// then a second top, then a base are inserted; the second top is
/// mated onto the base and the first onto the second. Each bundle reads
/// a copy inserted after its placement; the round trip is the document.
#[test]
fn s2_a_constraint_chain_authored_backwards_round_trips() {
    let p = parts("s2-chain-order");
    let doc = ProfileDoc::empty(DocumentId::derive("s2-chain-order"), Tol::witness());
    let (doc, upper) = insert(doc, fixture::mated_instance(p.top));
    let (doc, lower) = insert(doc, fixture::mated_instance(p.top));
    let (doc, base) = instance(doc, p.base, &p.opts());
    let (doc, _) = seated(doc, lower, head(p.top_cap(lower)), head(p.base_cap(base)));
    let (doc, _) = seat_at(
        doc,
        upper,
        head(p.top_cap(upper)),
        head(p.top_upper_cap(lower)),
        [0.0, 0.0],
    );
    round_trip(&doc, &[upper, lower, base], &p, "s2-chain-order");
}

// ---- The comparator's own rows ----

/// The comparator's scene: a block's profile and extrude, an
/// anchor pinned at a parametric pose and labelled, a base pinned
/// beside it, a top mated onto the base, and a lone top mated onto the
/// anchor. `tweak` changes one thing; every variant mints the same ids
/// in the same order, except `Order`, which inserts the top before the
/// base.
#[derive(Clone, Copy, PartialEq)]
enum Tweak {
    None,
    Target,
    Pin,
    Pose,
    Value,
    Head,
    Param,
    Label,
    Order,
}

fn comparator_scene(p: &Parts, tweak: Tweak) -> (ProfileDoc, [RecipeNodeId; 4]) {
    let (doc, _) = crate::p2_gauges::block("comparator-scene", 2.0, 1.0);
    let doc = crate::p2_gauges::declare_lift(doc, if tweak == Tweak::Param { 0.75 } else { 0.5 });
    let o = p.opts();
    let angle = if tweak == Tweak::Pose { 0.25 } else { 0.0 };
    let (doc, anchor) = pinned_lifted(doc, p.base, PoseRole::Z, angle, &o);
    let label = if tweak == Tweak::Label {
        "other"
    } else {
        "bench"
    };
    let (doc, _) = step(
        doc,
        DocEdit::SetLabel {
            node: anchor,
            label: Some(Label::new(label).expect("a label")),
        },
    );
    let x = if tweak == Tweak::Pin { 3.0 } else { 2.0 };
    let (doc, base, top) = if tweak == Tweak::Order {
        let (doc, top) = insert(doc, fixture::mated_instance(p.top));
        let (doc, base) = pinned_at(doc, p.base, [x, 8.0, 0.0], &o);
        (doc, base, top)
    } else {
        let (doc, base) = pinned_at(doc, p.base, [x, 8.0, 0.0], &o);
        let (doc, top) = insert(doc, fixture::mated_instance(p.top));
        (doc, base, top)
    };
    let onto = if tweak == Tweak::Head {
        base_bottom(p, base)
    } else {
        p.base_cap(base)
    };
    let at = if tweak == Tweak::Value {
        [1.0, 1.5]
    } else {
        [1.0, 1.0]
    };
    let (doc, _) = seat_at(doc, top, head(p.top_cap(top)), head(onto), at);
    let (doc, lone) = insert(doc, fixture::mated_instance(p.top));
    let target = if tweak == Tweak::Target { base } else { anchor };
    let (doc, _) = seated(doc, lone, head(p.top_cap(lone)), head(p.base_cap(target)));
    (doc, [anchor, base, top, lone])
}

/// **The comparator reads every field it claims** — and it reads them
/// itself, not through the remapping under test. A scene holding a
/// profile is itself under the identity maps; each one-thing change —
/// a constraint's target, a pin, a parametric pose, a value on a
/// mate, a head, a parameter, a label, the placements' order —
/// fails the check named, as do a wrong profile step and a collapsed
/// map.
#[test]
fn r1_the_comparator_reads_every_field() {
    let p = parts("comparator");
    let (a, roles) = comparator_scene(&p, Tweak::None);
    let (nodes, steps) = fixture::round_trip::identity(&a);
    same_up_to_ids(&a, &a, &nodes, &steps).expect("a document is itself");
    // A variant's ids, read by position: the scenes insert alike.
    let by_position = |b: &ProfileDoc| {
        let live = |d: &ProfileDoc| -> Vec<RecipeNodeId> {
            d.ids()
                .iter()
                .copied()
                .filter(|&id| d.node(id).is_some())
                .collect()
        };
        let mut map = editor_core::NodeMap::new();
        let mut step_map = editor_core::StepMap::new();
        for (x, y) in live(&a).into_iter().zip(live(b)) {
            map.insert(x, y);
            if let (Some(Node::Profile(px)), Some(Node::Profile(py))) = (a.node(x), b.node(y)) {
                step_map.extend(
                    px.ids
                        .iter()
                        .flatten()
                        .copied()
                        .zip(py.ids.iter().flatten().copied()),
                );
            }
        }
        (map, step_map)
    };
    let fails = |b: &ProfileDoc, nodes: &editor_core::NodeMap, steps: &editor_core::StepMap| {
        same_up_to_ids(&a, b, nodes, steps).expect_err("the comparator reads the change")
    };
    for (tweak, check, role) in [
        (Tweak::Target, "payload", 3),
        (Tweak::Pin, "payload", 1),
        (Tweak::Pose, "payload", 0),
        (Tweak::Value, "payload", 2),
        (Tweak::Head, "payload", 2),
        (Tweak::Param, "variables", 0),
        (Tweak::Label, "label", 0),
    ] {
        let (b, _) = comparator_scene(&p, tweak);
        let (map, step_map) = by_position(&b);
        let node = roles[role];
        let said = fails(&b, &map, &step_map);
        assert!(
            said.lines().any(|l| l.starts_with(check)
                && (check == "variables" || l.contains(&format!("{node:?}")))),
            "{check}: {said}"
        );
    }
    // The placements' order: the top inserted before the base, each
    // mapped to its own role.
    let (b, b_roles) = comparator_scene(&p, Tweak::Order);
    let (mut by_role, step_map) = by_position(&b);
    for (x, y) in roles.iter().zip(b_roles) {
        by_role.insert(*x, y);
    }
    let said = fails(&b, &by_role, &step_map);
    assert!(
        said.lines().any(|l| l.starts_with("order")),
        "order: {said}"
    );
    // A profile's step read as another step.
    let mut wrong = steps.clone();
    let (&s, _) = wrong.iter().next().expect("the scene draws a profile");
    let other = *wrong.keys().nth(1).expect("two steps");
    wrong.insert(s, other);
    let said = fails(&a, &nodes, &wrong);
    assert!(
        said.lines().any(|l| l.starts_with("payload")),
        "profile step: {said}"
    );
    // Two nodes collapsed onto one.
    let mut collapsed = nodes.clone();
    collapsed.insert(roles[3], roles[1]);
    let said = fails(&a, &collapsed, &steps);
    assert!(
        said.lines().any(|l| l.starts_with("injective")),
        "collapse: {said}"
    );
}

/// **The comparator over a round trip that keeps a profile** (a block
/// beside a cut pinned instance): equal, profile and all.
#[test]
fn r1_a_round_trip_keeping_a_profile_compares_equal() {
    let p = parts("r1-profile");
    let (doc, _) = crate::p2_gauges::block("r1-profile", 2.0, 1.0);
    let (doc, lone) = pinned_at(doc, p.base, [0.0, 8.0, 0.0], &p.opts());
    round_trip(&doc, &[lone], &p, "r1-profile");
}

/// **R1 over every shape split admits**: a cut moves as selected, so
/// inlining it back at the identity returns the document up to node ids
/// whatever the cut holds — one placed pair, a lone pinned instance, a
/// pair whose base is pinned at a parametric pose, two pinned instances
/// beside a pair, and plain geometry on the world.
#[test]
fn r1_every_shape_split_admits_round_trips_exactly() {
    type Scene = (ProfileDoc, Vec<RecipeNodeId>);
    let p = parts("r1-every");
    let o = p.opts();
    let empty = |label: &str| ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let seat_pair = |doc: ProfileDoc, base: RecipeNodeId| {
        let (doc, top) = insert(doc, fixture::mated_instance(p.top));
        let (doc, _) = seated(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
        (doc, [base, top])
    };
    let pair_at = |doc: ProfileDoc, at: [f64; 3]| {
        let (doc, base) = pinned_at(doc, p.base, at, &o);
        seat_pair(doc, base)
    };
    let one_pair = || -> Scene {
        let (doc, ids) = pair_at(empty("r1-one-pair"), [4.0, 0.0, 0.0]);
        (doc, ids.to_vec())
    };
    let lone = || -> Scene {
        let (doc, _kept) = pinned_at(empty("r1-lone"), p.top, [0.0, 9.0, 0.0], &o);
        let (doc, lone) = pinned_at(doc, p.base, [4.0, 0.0, 0.0], &o);
        (doc, vec![lone])
    };
    let parametric = || -> Scene {
        let doc = crate::p2_gauges::declare_lift(empty("r1-parametric-pin"), 0.5);
        let (doc, base) = pinned_lifted(doc, p.base, PoseRole::X, 0.0, &o);
        let (doc, ids) = seat_pair(doc, base);
        (doc, ids.to_vec())
    };
    let pins_beside_a_pair = || -> Scene {
        let (doc, _kept) = pinned_at(empty("r1-pins-beside-a-pair"), p.top, [0.0, 8.0, 0.0], &o);
        let (doc, pair) = pair_at(doc, [2.0, 10.0, 0.5]);
        let (doc, lone) = pinned_at(doc, p.base, [18.0, 8.0, 0.5], &o);
        let (doc, deep) = pinned_at(doc, p.top, [6.0, 8.0, 0.5], &o);
        (doc, [&[lone, deep][..], &pair].concat())
    };
    let plain = || -> Scene {
        let (doc, body) = crate::p2_gauges::block("r1-plain", 2.0, 1.0);
        let doc = fixture::place(doc, body).0;
        let all = doc
            .ids()
            .into_iter()
            .filter(|&id| id != RecipeNodeId::WORLD)
            .collect();
        (doc, all)
    };
    let scenes: [(&str, &dyn Fn() -> Scene); 5] = [
        ("one placed pair", &one_pair),
        ("a lone pinned instance", &lone),
        ("a pair at a parametric pin", &parametric),
        ("two pinned instances beside a pair", &pins_beside_a_pair),
        ("plain geometry on the world", &plain),
    ];
    for (shape, scene) in scenes {
        let (doc, ids) = scene();
        round_trip(&doc, &ids, &p, shape);
    }
}

/// **A cut a kept placement separates comes back regrouped** (A4:
/// inline-of-split, flattened, is the document up to minted ids and
/// order): three pinned instances x, y, z, inserted in that order, and a
/// cut of x and z. Split appends its instance and inline its splice, so
/// the round trip lists y, then x and z, and the regrouping is the only thing the
/// comparator reads as changed. The regrouped document's cut comes
/// last, so a second round trip changes nothing.
#[test]
fn r1_a_cut_whose_placements_a_kept_one_separates_round_trips() {
    let p = parts("r1-interleaved");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("r1-interleaved"), Tol::witness());
    let (doc, x) = pinned_at(doc, p.base, [4.0, 0.0, 0.0], &o);
    let (doc, y) = pinned_at(doc, p.base, [0.0, 9.0, 0.0], &o);
    let (doc, z) = pinned_at(doc, p.top, [0.0, 0.0, 7.0], &o);
    let out = split(&doc, &[x, z], "r1-interleaved", &p.opts()).expect("two pinned instances");
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let back = inline(&out.remainder, out.instance, &store);
    let flat = editor_core::flatten(&back.doc, back.placement, Tol::witness())
        .expect("the identity placement flattens")
        .doc;
    let (map, steps) = composed(&doc, &out, &back);
    let said = same_up_to_ids(&doc, &flat, &map, &steps)
        .expect_err("the round trip regroups the placements");
    let [only] = said.lines().collect::<Vec<_>>()[..] else {
        panic!("the regrouping is the one difference: {said}")
    };
    assert!(only.starts_with("order: "), "the regrouping: {said}");
    let order: Vec<RecipeNodeId> = flat.placements();
    assert_eq!(
        order,
        vec![map[&y], map[&x], map[&z]],
        "the kept placement, then the cut's in their order"
    );
    round_trip(&flat, &[map[&x], map[&z]], &p, "the regrouped document");
}

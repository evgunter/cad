//! INTENT stage 3 D′2 acceptance — a mate reads only geometry, and a
//! value charts on the target space (`docs/INTENT-STAGE3-SPEC.md`
//! §5′.2, tests 21 and 24, and the flatten edit).
//!
//! No space has roles a mate can read: a value names no mate and is a
//! coordinate of the target space, its axes carried onto the one mated
//! direction by the least turn, so a value edit never rides with the
//! body a pose was read off. Inline is one rule — the part's nodes land in a `new_space()`
//! verbatim and the instance becomes a placement of their copies under
//! its own bundle — and flatten folds such a placement, at the
//! identity, back into its target.
//!
//! Test 20 (a lone point mate's chain) is
//! `asm_r2a_mate_solve::a_lone_point_mates_chain_turns_its_copy_in_the_order_written`;
//! inline then flatten of a split is the R1 rows of `p2_split` and
//! `asm4_split_inline`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::fixture::resolver::{PartStore, with_resolver};
use editor_core::Formula;
use editor_core::pose::PoseDef;
use editor_core::{
    BodyRead, CapEnd, Coord, Dimension, DocEdit, DocumentId, EvalOptions, FlattenError, FreeValue,
    FreeVar, Node, Operand, PartResolver, Placed, ProfileDoc, RecipeNodeId, RoleSeg, StableName,
    TargetAxis, VarName, flatten, inline,
};
use fixture::{head, head_at, insert, len, on_frame, run, solve, step};
use geom_core::Tol;
use std::sync::Arc;

/// A box `x` by `y` by `z` on its own xy, at its own origin: the
/// document and its extrude.
fn block(doc: ProfileDoc, x: f64, y: f64, z: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (x, 0.0), (x, y), (0.0, y)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(z),
        },
    )
}

/// The name of `body`'s planar face through `at` with outward normal
/// `normal`, both in its own coordinates.
fn face(doc: &ProfileDoc, body: RecipeNodeId, at: [f64; 3], normal: [f64; 3]) -> StableName {
    let read = editor_core::test_support::plane_on(doc, body, at, normal, Tol::witness()).read;
    let Operand::Pose(def) = read else {
        panic!("a plane read: {read:?}")
    };
    let PoseDef::Plane {
        face: Operand::Select { names, .. },
    } = *def
    else {
        panic!("a plane of one face")
    };
    names[0].clone()
}

// ---- Test 24: a value charts on the target space ----

/// The row-190 scene: a 20 × 10 × 2 plate placed into the world by
/// `turn` about the world's `z` (copy `B`), and, where `through_union`,
/// a second copy `C` far along `x` unioned with it; a unit boss rested
/// on `B`'s `+x` side face — read off `B`, or off `B ∪ C` — with
/// `Slide(X) = 10` and the spin and the other slide `0`. Returns the
/// boss's solved placement.
fn boss_on_the_plate(turn: f64, through_union: bool) -> editor_core::Frame {
    let doc = ProfileDoc::empty(DocumentId::derive("dprime2-row190"), Tol::witness());
    let (doc, plate) = block(doc, 20.0, 10.0, 2.0);
    let (doc, boss) = block(doc, 1.0, 1.0, 1.0);
    let side = face(&doc, plate, [20.0, 5.0, 1.0], [1.0, 0.0, 0.0]);
    let (doc, b) = fixture::in_world_by(
        doc,
        plate,
        Placed::unmated(vec![(Coord::Spin(Some(TargetAxis::Z)), fixture::ang(turn))]).values,
    );
    let (doc, target) = if through_union {
        let (doc, c) = fixture::in_world_by(
            doc,
            plate,
            Placed::unmated(vec![(Coord::Slide(TargetAxis::X), len(100.0))]).values,
        );
        let (doc, both) = fixture::union_over(doc, &[b, c]);
        let name = fixture::member_name(&doc, both, b, side);
        (doc, head_at(both, name))
    } else {
        (doc, head_at(b, side))
    };
    let (doc, placement) = insert(doc, fixture::stood(BodyRead::plain(Operand::from(boss))));
    let start = fixture::fname(boss, RoleSeg::Cap(CapEnd::Start));
    let placed = Placed {
        mates: vec![fixture::rest(head(start), target)],
        values: vec![
            (Coord::Slide(TargetAxis::X), len(10.0)),
            (
                Coord::Slide(TargetAxis::Y),
                Formula::zero(Dimension::Length),
            ),
            (Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    };
    let (doc, _) = fixture::write_bundle(doc, placement, placed.bundle());
    let poses = solve(&doc, &EvalOptions::default(), Tol::witness());
    assert_eq!(poses.fault(placement), None, "the boss is placed");
    poses
        .placement(&doc, placement)
        .expect("the boss is placed")
}

/// **Test 24 (D′2): a value charts on the target space.** A boss rests
/// on a side face of plate copy `B`, `B` turned 30° in the world, and
/// `Slide(X) = 10`. The identity orientation puts the boss's bottom at
/// `−z`, and the mated direction is the face's flipped outward normal
/// `−n`, `n = (cos θ, sin θ, 0)`; the slide is measured from the world's
/// origin along the world's `x` as the least turn from `−z` onto `−n`
/// carries it — `x' = (sin² θ, −sin θ cos θ, −cos θ)` — so the boss's
/// origin stands at `10 x' + 20 n`, on the face. Turning `B` to
/// 40° moves the boss across the face to the same formula's point, not
/// with `B`; and the same face read off `B ∪ C` places the same copy,
/// bit for bit.
///
/// *Breaks if* the chart reads the body the pose is read off (the boss
/// would ride with `B`, at `B`'s turn of its 30° place).
#[test]
fn a_value_charts_on_the_target_space() {
    let want = |turn: f64| {
        let (s, c) = turn.sin_cos();
        let x = [s * s, -s * c, -c];
        let n = [c, s, 0.0];
        [0, 1, 2].map(|i| 10.0 * x[i] + 20.0 * n[i])
    };
    let (thirty, forty) = (30.0_f64.to_radians(), 40.0_f64.to_radians());
    let at_thirty = boss_on_the_plate(thirty, false);
    let at_forty = boss_on_the_plate(forty, false);
    for (got, turn) in [(&at_thirty, thirty), (&at_forty, forty)] {
        let off = got
            .translation
            .iter()
            .zip(want(turn))
            .map(|(g, w)| (g - w).abs())
            .fold(0.0_f64, f64::max);
        assert!(
            off <= 1e-12,
            "the boss stands where the space's chart puts it at {turn}: {:?}, {off:e} off",
            got.translation
        );
    }
    // Ridden with `B`, the boss at 40° would be its 30° place turned 10°.
    let (s, c) = (forty - thirty).sin_cos();
    let t = at_thirty.translation;
    let ridden = [c * t[0] - s * t[1], s * t[0] + c * t[1], t[2]];
    let apart = ridden
        .iter()
        .zip(at_forty.translation)
        .map(|(r, g)| (r - g).abs())
        .fold(0.0_f64, f64::max);
    assert!(apart > 1.0, "the boss does not ride with B: {apart}");
    assert_eq!(
        boss_on_the_plate(thirty, true),
        at_thirty,
        "the same face read off B ∪ C gives the same copy"
    );
}

// ---- Test 21: inline keeps a variable driving ----

fn lift() -> VarName {
    VarName::from_static("lift")
}

/// **Test 21 (D′2): inline keeps a variable driving.** A part whose one
/// placement slides its block 3 along `x` and up by the named variable
/// `lift`, under an instance whose bundle turns it a quarter turn
/// (`turn/4`) about the world's `z`, inlines: the part's placement lands in a `new_space()`
/// verbatim, still reading `lift`, and the instance becomes a placement
/// of its copy into the world under the instance's own bundle. Nothing
/// moves — the part's slide along `x` is turned by the instance, which
/// a composition in the other order would not do; and moving `lift`
/// then moves the spliced copy up as it moved the part's copy before
/// the splice.
///
/// *Breaks if* inline refuses, composes a motion into the part's
/// placement or composes the two in the other order, or writes the
/// variable's current value as a number.
#[test]
fn inline_keeps_a_variable_driving() {
    let part = ProfileDoc::empty(DocumentId::derive("dprime2-t21-part"), Tol::witness());
    let (part, _) = step(
        part,
        DocEdit::DeclareVar {
            name: lift(),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, 2.0)),
        },
    );
    let (part, body) = block(part, 1.0, 1.0, 1.0);
    let (part, lifted) = fixture::in_world_by(
        part,
        body,
        Placed::unmated(vec![
            (Coord::Slide(TargetAxis::X), len(3.0)),
            (
                Coord::Slide(TargetAxis::Z),
                editor_core::Formula::named(lift(), Dimension::Length),
            ),
        ])
        .values,
    );
    let mut store = PartStore::default();
    let part = crate::fixture::assemble_world_copies(part);
    let doc_ref = store.insert(part.clone(), Tol::witness());
    let opts = with_resolver(store.clone());
    let host = ProfileDoc::empty(DocumentId::derive("dprime2-t21-host"), Tol::witness());
    let (host, instance) = insert(host, fixture::mated_instance(doc_ref));
    let host = fixture::written(
        host,
        instance,
        Placed::unmated(vec![(
            Coord::Spin(Some(TargetAxis::Z)),
            editor_core::Formula::turn_over(4).expect("a quarter turn"),
        )]),
    );
    let resolver: Arc<dyn PartResolver> = Arc::new(store);
    let out = inline(&host, instance, &resolver, Tol::witness())
        .unwrap_or_else(|e| panic!("the instance inlines: {e}"));
    let spliced = out.node_map[&lifted];
    assert_eq!(
        out.doc.node_space(spliced),
        Some(out.space),
        "the part's placement lands in the new space"
    );
    let slide = fixture::bundle(&out.doc, spliced)
        .into_iter()
        .find_map(|c| {
            c.value()
                .filter(|v| v.coord == Coord::Slide(TargetAxis::Z))
                .cloned()
        })
        .expect("the part's slide");
    assert_eq!(
        out.doc.var_name(slide.by),
        Some(&lift()),
        "verbatim, its slide still reading `lift`"
    );
    assert_eq!(
        fixture::bundle(&out.doc, spliced).len(),
        fixture::bundle(&part, lifted).len(),
        "and nothing composed into it"
    );
    assert_eq!(
        fixture::offset_of(&out.doc, out.placement),
        fixture::offset_of(&host, instance),
        "the new placement holds the instance's bundle as written"
    );
    let bounds = |doc: &ProfileDoc, o: &EvalOptions| {
        let body =
            crate::fixture::product_in_world(doc, &run(doc, o), Tol::witness()).expect("gathers");
        let pts: Vec<[f64; 3]> = body.points().map(|(_, p)| [p.x, p.y, p.z]).collect();
        let lo = pts.iter().fold([f64::INFINITY; 3], |m, p| {
            [m[0].min(p[0]), m[1].min(p[1]), m[2].min(p[2])]
        });
        let hi = pts.iter().fold([f64::NEG_INFINITY; 3], |m, p| {
            [m[0].max(p[0]), m[1].max(p[1]), m[2].max(p[2])]
        });
        (lo, hi)
    };
    let before = bounds(&host, &opts);
    assert_eq!(
        bounds(&out.doc, &EvalOptions::default()),
        before,
        "nothing moves"
    );
    // The part's slide `x = 3` turned by the instance's quarter turn
    // about `z`: the block spans `x ∈ [−1, 0]`, `y ∈ [3, 4]`.
    let near = |a: f64, b: f64| (a - b).abs() < 1e-12;
    assert!(
        near(before.0[0], -1.0) && near(before.0[1], 3.0) && before.0[2] == 2.0,
        "the part's copy stands `lift` up, its slide turned by the instance: {before:?}"
    );
    let (moved, _) = step(
        out.doc,
        DocEdit::SetVarValue {
            var: lift().into(),
            value: FreeValue::Continuous(5.0),
        },
    );
    let after = bounds(&moved, &EvalOptions::default());
    assert_eq!(
        (after.0[0], after.0[1], after.0[2]),
        (before.0[0], before.0[1], 5.0),
        "moving `lift` moves the spliced copy up, and nothing else"
    );
}

// ---- The flatten edit's refusals ----

/// A part of one block placed into its world at `x`, and a host holding
/// one instance of it whose bundle slides it `dx` along the world's `x`,
/// inlined: the host before and after, the inline's outcome, and the
/// part's placement.
fn inlined(dx: f64) -> (ProfileDoc, editor_core::InlineOutcome, RecipeNodeId) {
    let part = ProfileDoc::empty(DocumentId::derive("dprime2-flatten-part"), Tol::witness());
    let (part, body) = block(part, 1.0, 1.0, 1.0);
    let (part, placed) = fixture::place(part, body);
    let mut store = PartStore::default();
    let part = crate::fixture::assemble_world_copies(part);
    let doc_ref = store.insert(part, Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("dprime2-flatten-host"), Tol::witness());
    let (host, instance) = insert(host, fixture::mated_instance(doc_ref));
    // At `0` the identity bundle as written, the constant `0` on every
    // freedom: a slide written `0 m` is a value, not the identity.
    let bundle = if dx == 0.0 {
        Placed::identity()
    } else {
        Placed::unmated(vec![(Coord::Slide(TargetAxis::X), len(dx))])
    };
    let host = fixture::written(host, instance, bundle);
    let resolver: Arc<dyn PartResolver> = Arc::new(store);
    let out = inline(&host, instance, &resolver, Tol::witness()).expect("the instance inlines");
    (host, out, placed)
}

/// **Flatten folds an identity placement out of its new space, and
/// refuses everything else typed**: an unknown id, a node that is no
/// placement, a placement off the identity, one whose source is no new
/// space, one beside another placement copying out of that space, and
/// one whose copy and source are both named. A copy of a part of two
/// placements is one body, the part's `⊔`, so a reader of it whole
/// flattens.
///
/// *Breaks if* flatten moves a placement that is not the identity, or
/// strands a reader it cannot re-point.
#[test]
fn flatten_folds_the_identity_and_refuses_the_rest_typed() {
    // The identity: the host's product comes back unmoved.
    let (host, out, placed) = inlined(0.0);
    let flat = flatten(&out.doc, out.placement, Tol::witness()).expect("the identity flattens");
    assert!(flat.doc.node(out.placement).is_none(), "the placement goes");
    assert_eq!(
        flat.doc.node_space(out.node_map[&placed]),
        host.node_space(host.placements()[0]),
        "the part's placement takes the target"
    );
    let volume = |doc: &ProfileDoc| {
        let body = crate::fixture::product_in_world(
            doc,
            &run(doc, &EvalOptions::default()),
            Tol::witness(),
        )
        .expect("gathers");
        let measured = topo::test_support::finished("the product", body, Tol::witness());
        topo::test_support::pinned_volume("mass", &measured, Tol::witness()).volume
    };
    assert_eq!(volume(&flat.doc), 1.0);

    let refused = |doc: &ProfileDoc, at: RecipeNodeId| match flatten(doc, at, Tol::witness()) {
        Ok(_) => panic!("flatten admitted {at:?}"),
        Err(e) => {
            assert!(e.to_string().contains("Recourse:"), "{e}");
            e
        }
    };
    assert!(matches!(
        refused(&out.doc, RecipeNodeId::new(0, 999)),
        FlattenError::UnknownNode { .. }
    ));
    let extrude = out
        .doc
        .ids()
        .into_iter()
        .find(|&id| matches!(out.doc.node(id), Some(Node::Extrude { .. })))
        .expect("the spliced block");
    assert!(matches!(
        refused(&out.doc, extrude),
        FlattenError::NotAPlacement { .. }
    ));
    let (_, moved, _) = inlined(5.0);
    assert!(matches!(
        refused(&moved.doc, moved.placement),
        FlattenError::NotTheIdentity { .. }
    ));
    let spliced = out.node_map[&placed];
    assert!(matches!(
        refused(&out.doc, spliced),
        FlattenError::NotANewSpace { .. }
    ));

    // A second placement copying the spliced block out of the space.
    let source = out
        .doc
        .output(spliced, 0)
        .expect("the spliced placement defines its copy");
    let (shared, _) = fixture::in_world_by(
        out.doc.clone(),
        Operand::Var(source),
        Placed::unmated(vec![(Coord::Slide(TargetAxis::Y), len(5.0))]).values,
    );
    assert!(matches!(
        refused(&shared, out.placement),
        FlattenError::SpaceShared { .. }
    ));

    // A part of two placements: the new placement copies the `⊔` of
    // the two, one body, and a reader of its copy whole reads that `⊔`
    // once flattened.
    let part = ProfileDoc::empty(DocumentId::derive("dprime2-flatten-two"), Tol::witness());
    let (part, a) = block(part, 1.0, 1.0, 1.0);
    let (part, b) = block(part, 1.0, 1.0, 1.0);
    let (part, _) = fixture::place(part, a);
    let (part, _) = fixture::in_world_by(
        part,
        b,
        Placed::unmated(vec![(Coord::Slide(TargetAxis::X), len(3.0))]).values,
    );
    let mut store = PartStore::default();
    let part = crate::fixture::assemble_world_copies(part);
    let doc_ref = store.insert(part, Tol::witness());
    let host = ProfileDoc::empty(
        DocumentId::derive("dprime2-flatten-two-host"),
        Tol::witness(),
    );
    let (host, instance) = insert(host, fixture::mated_instance(doc_ref));
    let (host, _) = insert(
        host,
        fixture::stood(BodyRead::plain(Operand::from(instance))),
    );
    let resolver: Arc<dyn PartResolver> = Arc::new(store);
    let two = inline(&host, instance, &resolver, Tol::witness()).expect("inlines");
    flatten(&two.doc, two.placement, Tol::witness())
        .expect("a reader of the copy whole reads the `⊔` it copies");

    // The copy and the shape it copies both named.
    let copy = out
        .doc
        .output(out.placement, 0)
        .expect("the new placement defines its copy");
    let (named, _) = step(
        out.doc.clone(),
        DocEdit::RenameVar {
            var: copy.into(),
            name: Some(VarName::from_static("copy")),
        },
    );
    let (named, _) = step(
        named,
        DocEdit::RenameVar {
            var: source.into(),
            name: Some(VarName::from_static("shape")),
        },
    );
    assert!(matches!(
        refused(&named, out.placement),
        FlattenError::BothNamed { .. }
    ));
}

// ---- A slide's width on a box lane ----

/// **A slide value sets its translation, not a difference** (absorbed
/// `a-slide-value-doubles-its-translations-width`): a unit box resting
/// on a floor's top with `Slide(X) = 3` and `Slide(Y) = 2` is placed,
/// at `Interval`, by a map whose translation along each slid axis is
/// the value's own point — no width. The chart's
/// axes are the world's, so the box lane sets each slid component from
/// its value directly, where it once added the difference between the
/// value and the folded map's own translation, carrying that
/// translation's width twice.
///
/// *Breaks if* a slide on a frame lane is set by a difference.
#[test]
fn a_slide_value_sets_its_translation_with_no_width() {
    use geom_core::{Bounds, Interval};
    let doc = ProfileDoc::empty(DocumentId::derive("dprime2-slide-width"), Tol::witness());
    let (doc, top, _) = fixture::floor(doc, Vec::new());
    let (doc, boss) = block(doc, 1.0, 1.0, 1.0);
    let (doc, placement) = insert(doc, fixture::stood(BodyRead::plain(Operand::from(boss))));
    let start = fixture::fname(boss, RoleSeg::Cap(CapEnd::Start));
    let placed = Placed {
        mates: vec![editor_core::Mate {
            on: fixture::plane(head(start)),
            to: fixture::flip(top),
            orient: editor_core::Orientation::IDENTITY,
        }],
        values: vec![
            (Coord::Slide(TargetAxis::X), len(3.0)),
            (Coord::Slide(TargetAxis::Y), len(2.0)),
            (Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    };
    let (doc, _) = fixture::write_bundle(doc, placement, placed.bundle());
    let poses = editor_core::mate::solve_document_at::<Interval>(
        &doc,
        &EvalOptions::default(),
        Tol::witness(),
    );
    let map = poses.relative_map(placement).expect("the box is placed");
    let t = map.translation;
    assert_eq!(
        [(t.x.lo(), t.x.hi()), (t.y.lo(), t.y.hi())],
        [(3.0, 3.0), (2.0, 2.0)],
        "each slid component is the value's own point, with no width"
    );
}

/// A unit block in the world turned by the chain `Z(0.1), X(0.2),
/// Z(0.3)` and slid `(3, 0, 7)`: the document, the floor's placement
/// and the block's.
fn chained_block(label: &str) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, floor) = block(doc, 20.0, 20.0, 1.0);
    let (doc, boss) = block(doc, 1.0, 1.0, 1.0);
    let (doc, f) = fixture::in_world_by(doc, floor, Placed::unmated(vec![]).values);
    let (doc, copy) = fixture::in_world_by(doc, boss, Placed::unmated(chain(None)).values);
    (doc, boss, f, copy)
}

/// The chain `Z(0.1), X(0.2), Z(0.3)` and the slides `(3, 0, 7)`, the
/// link at `zero` written `0`.
fn chain(zero: Option<usize>) -> Vec<(Coord, editor_core::Formula)> {
    let links = [
        (TargetAxis::Z, 0.1),
        (TargetAxis::X, 0.2),
        (TargetAxis::Z, 0.3),
    ];
    let mut out: Vec<_> = links
        .into_iter()
        .enumerate()
        .map(|(k, (axis, a))| {
            let a = if zero == Some(k) { 0.0 } else { a };
            (Coord::Spin(Some(axis)), fixture::ang(a))
        })
        .collect();
    out.push((Coord::Slide(TargetAxis::X), len(3.0)));
    out.push((Coord::Slide(TargetAxis::Z), len(7.0)));
    out
}

/// **Removing a link of a written chain writes `0` in its place**: the
/// first, middle or last spin of `Z, X, Z` removed places the copy as
/// the same chain written with that link `0` does, bit for bit, and no
/// value is dropped. The middle link's removal turns the copy by the
/// two `z` links' sum.
///
/// *Breaks if* the removal drops the link and the chain's rule then
/// drops the second `z` as a repeat (the copy turns `0.1`, not `0.4`),
/// or appends the `0` at the chain's end (another order).
#[test]
fn removing_a_chain_link_writes_zero_in_its_place() {
    let opts = EvalOptions::default();
    for k in 0..3 {
        let (doc, boss, _, copy) = chained_block(&format!("dprime2-unlink-{k}"));
        let link = fixture::bundle(&doc, copy)[k].id;
        let applied = doc
            .apply(
                &DocEdit::RemoveConstraint {
                    placement: copy,
                    constraint: link,
                },
                Tol::witness(),
            )
            .unwrap_or_else(|e| panic!("link {k}: the removal is admitted: {e}"));
        assert!(
            !applied
                .maintenance
                .iter()
                .any(|m| matches!(m, editor_core::Maintenance::ValueDropped { .. })),
            "link {k}: a removal drops no other value: {:?}",
            applied.maintenance
        );
        let got = solve(&applied.doc, &opts, Tol::witness())
            .placement(&applied.doc, copy)
            .expect("the unlinked copy places");
        let written = ProfileDoc::empty(
            DocumentId::derive(&format!("dprime2-unlink-{k}")),
            Tol::witness(),
        );
        let (written, _) = block(written, 20.0, 20.0, 1.0);
        let (written, again) = block(written, 1.0, 1.0, 1.0);
        assert_eq!(again, boss);
        let (written, want) =
            fixture::in_world_by(written, again, Placed::unmated(chain(Some(k))).values);
        let want = solve(&written, &opts, Tol::witness())
            .placement(&written, want)
            .expect("the written chain places");
        assert_eq!(
            got, want,
            "link {k}: the removal is the chain with that link written 0"
        );
        if k == 1 {
            let turned = got.columns[0][1].atan2(got.columns[0][0]);
            assert!(
                (turned - 0.4).abs() < 1e-12,
                "the middle link removed, the copy turns 0.1 + 0.3 about z: {turned}"
            );
        }
    }
}

/// **A mate added over written values reports each value it drops.**
/// A rest on the floor's top fixes the block's `z` slide and its turns
/// off the floor's normal. The chain `Z(0.1), X(tilt), Z(0.3)` turns
/// about the world's axes, and no link of it is the spin about the
/// floor's own normal, so the door drops all three (the `x` link read the named
/// `tilt`, still declared) and writes `0` on the spin; and it drops the
/// `z` slide. Each drop is a `ValueDropped` row naming its freedom and
/// the variable it read.
///
/// *Breaks if* a value goes with no row, or a row names a value the
/// bundle kept.
#[test]
fn a_mate_reports_each_value_it_drops() {
    let doc = ProfileDoc::empty(DocumentId::derive("dprime2-drops"), Tol::witness());
    let (doc, _) = step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("tilt"),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Angle, 0.2)),
        },
    );
    let (doc, floor) = block(doc, 20.0, 20.0, 1.0);
    let (doc, boss) = block(doc, 1.0, 1.0, 1.0);
    let (doc, f) = fixture::in_world_by(doc, floor, Placed::unmated(vec![]).values);
    let mut moves = chain(None);
    moves[1].1 = editor_core::Formula::named(VarName::from_static("tilt"), Dimension::Angle);
    let (doc, copy) = fixture::in_world_by(doc, boss, Placed::unmated(moves).values);
    let tilt = doc.var_named("tilt").expect("declared");
    let top = fixture::fname(floor, RoleSeg::Cap(CapEnd::End));
    let start = fixture::fname(boss, RoleSeg::Cap(CapEnd::Start));
    let mate = fixture::rest(head(start), head_at(f, top));
    let applied = doc
        .apply(
            &DocEdit::AddConstraint {
                placement: copy,
                mate: Box::new(mate),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect("the rest is admitted");
    let dropped: Vec<(Coord, editor_core::VarId)> = applied
        .maintenance
        .iter()
        .filter_map(|m| match m {
            editor_core::Maintenance::ValueDropped {
                placement,
                coord,
                by,
            } => {
                assert_eq!(placement.id(), copy, "the row names the copy's placement");
                Some((*coord, by.id()))
            }
            _ => None,
        })
        .collect();
    let coords: Vec<Coord> = dropped.iter().map(|&(c, _)| c).collect();
    assert_eq!(
        coords,
        [
            Coord::Spin(Some(TargetAxis::Z)),
            Coord::Spin(Some(TargetAxis::X)),
            Coord::Spin(Some(TargetAxis::Z)),
            Coord::Slide(TargetAxis::Z),
        ],
        "the three links and the z slide go, each reported"
    );
    assert_eq!(dropped[1].1, tilt, "the x link's row names `tilt`");
    assert!(
        applied.doc.var_named("tilt").is_some(),
        "the named variable stays declared"
    );
    let kept: Vec<Coord> = fixture::bundle(&applied.doc, copy)
        .iter()
        .filter_map(|c| c.value().map(|v| v.coord))
        .collect();
    assert_eq!(
        kept,
        [
            Coord::Slide(TargetAxis::X),
            Coord::Slide(TargetAxis::Y),
            Coord::SPIN,
        ],
        "the x slide, then the 0 on the y slide and on the spin"
    );
}

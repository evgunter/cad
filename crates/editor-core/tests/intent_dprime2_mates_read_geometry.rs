//! INTENT stage 3 D′2 acceptance — a mate reads only geometry, and a
//! value charts on the target space (`docs/INTENT-STAGE3-SPEC.md`
//! §5′.2, tests 21 and 24, and the flatten edit).
//!
//! No space has roles a mate can read: a value names no mate and is a
//! coordinate of the target space, its axes swung onto the one mated
//! direction, so a value edit never rides with the body a pose was read
//! off. Inline is one rule — the part's nodes land in a `new_space()`
//! verbatim and the instance becomes a placement of their copies under
//! its own bundle — and flatten folds such a placement, at the
//! identity, back into its target.
//!
//! Test 20 (a lone point mate's chain) is
//! `asm_r2a_mate_solve::a_lone_point_mates_chain_turns_its_copy_about_the_mated_point`;
//! inline then flatten of a split is the R1 rows of `p2_split` and
//! `asm4_split_inline`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::fixture::resolver::{PartStore, with_resolver};
use editor_core::pose::PoseDef;
use editor_core::{
    BodyRead, CapEnd, ContactClass, Coord, Dimension, DocEdit, DocumentId, EvalOptions,
    FlattenError, FreeValue, FreeVar, Node, Operand, PartResolver, Placed, ProfileDoc,
    RecipeNodeId, RoleSeg, StableName, TargetAxis, VarName, flatten, inline, product,
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
        Placed::unmated(vec![(Coord::Spin(TargetAxis::Z), fixture::ang(turn))]).values,
    );
    let (doc, target) = if through_union {
        let (doc, c) = fixture::in_world_by(
            doc,
            plate,
            Placed::unmated(vec![(Coord::Slide(TargetAxis::X), len(100.0))]).values,
        );
        let (doc, both) = fixture::union_over(doc, &[b, c]);
        let name = fixture::member_name(&doc, both, b, side.in_copy(b));
        (doc, head_at(both, name))
    } else {
        (doc, head_at(b, side.in_copy(b)))
    };
    let (doc, placement) = insert(doc, fixture::stood(vec![BodyRead::plain(Operand::from(boss))]));
    let start = fixture::fname(boss, RoleSeg::Cap(CapEnd::Start));
    let placed = Placed {
        mates: vec![fixture::rest(head(start), target, ContactClass::Rest)],
        values: vec![
            (Coord::Slide(TargetAxis::X), len(10.0)),
            (Coord::Slide(TargetAxis::Y), len(0.0)),
            (Coord::Spin(TargetAxis::Z), fixture::ang(0.0)),
        ],
    };
    let (doc, _) = fixture::write_bundle(doc, placement, placed.bundle());
    let poses = solve(&doc, &EvalOptions::default(), Tol::witness());
    assert_eq!(poses.fault(placement), None, "the boss is placed");
    poses.placement(&doc, placement).expect("the boss is placed")
}

/// **Test 24 (D′2): a value charts on the target space.** A boss rests
/// on a side face of plate copy `B`, `B` turned 30° in the world, and
/// `Slide(X) = 10`. The slide is measured from the world's origin along
/// its `x` swung onto the face's outward normal `n = (cos θ, sin θ, 0)`
/// by the shortest turn — `x' = (sin² θ, −sin θ cos θ, −cos θ)` — so the
/// boss's origin stands at `10 x' + 20 n`, on the face. Turning `B` to
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
/// placement slides its block up by the named variable `lift`, under an
/// instance whose bundle turns it a quarter turn (`turn/4`) about the
/// world's `z`, inlines: the part's placement lands in a `new_space()`
/// verbatim, still reading `lift`, and the instance becomes a placement
/// of its copy into the world under the instance's own bundle. Nothing
/// moves; and moving `lift` then moves the spliced copy up as it moved
/// the part's copy before the splice.
///
/// *Breaks if* inline refuses, composes a motion into the part's
/// placement, or writes the variable's current value as a number.
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
        Placed::unmated(vec![(
            Coord::Slide(TargetAxis::Z),
            editor_core::Formula::named(lift(), Dimension::Length),
        )])
        .values,
    );
    let mut store = PartStore::default();
    let doc_ref = store.insert(part.clone(), Tol::witness());
    let opts = with_resolver(store.clone());
    let host = ProfileDoc::empty(DocumentId::derive("dprime2-t21-host"), Tol::witness());
    let (host, instance) = insert(host, fixture::mated_instance(doc_ref));
    let host = fixture::written(
        host,
        instance,
        Placed::unmated(vec![(
            Coord::Spin(TargetAxis::Z),
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
        .find_map(|c| c.value().filter(|v| v.coord == Coord::Slide(TargetAxis::Z)).cloned())
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
        let body = product(doc, &run(doc, o), Tol::witness()).expect("gathers");
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
    assert_eq!(before.0[2], 2.0, "the part's copy stands `lift` up");
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
/// space, one beside another placement copying out of that space, one
/// whose copy a reader reads whole over two shapes, and one whose copy
/// and source are both named.
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
        topo::test_support::unmigrated_measure(
            &product(doc, &run(doc, &EvalOptions::default()), Tol::witness()).expect("gathers"),
            Tol::witness(),
        )
        .expect("mass")
        .volume
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

    // A part of two placements: the new placement copies two shapes,
    // and a reader of its copies whole names no one shape.
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
    let doc_ref = store.insert(part, Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("dprime2-flatten-two-host"), Tol::witness());
    let (host, instance) = insert(host, fixture::mated_instance(doc_ref));
    let (host, _) = insert(host, fixture::stood(vec![BodyRead::plain(Operand::from(instance))]));
    let resolver: Arc<dyn PartResolver> = Arc::new(store);
    let two = inline(&host, instance, &resolver, Tol::witness()).expect("inlines");
    assert!(matches!(
        refused(&two.doc, two.placement),
        FlattenError::ReadUncarried { .. }
    ));

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

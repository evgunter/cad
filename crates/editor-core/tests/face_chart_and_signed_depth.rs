//! **A profile drawn on a face, on the face's chart, and a signed
//! depth** (D10 `[placement]`; `REFERENCES.md` DM1;
//! `work/doctail/a-face-frame-and-a-signed-extrude-for-a-sketch-on-a-face.md`).
//!
//! The face's chart is the frame a plane placement onto the face gives
//! under the same orientation, spin and slides, computed by that
//! placement's own code; within the band about the least turn's pole it
//! refuses, naming the orientation to write. A profile drawn on it is in
//! the face's body's space, and an extrude of it by a `+` depth is a boss
//! above the face, by a `−` depth a pocket's tool under it.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::result_large_err
)]

use crate::fixture::{self, ang, fname, len};
use editor_core::eval::bound_pose;
use editor_core::mate::face_chart_slides;
use editor_core::pose::{PoseConstruction, PoseDef};
use editor_core::{
    BodyRead, BooleanValue, CancelToken, CapEnd, Coord, Dimension, DocEdit, EvalOptions,
    Evaluation, Formula, FreeValue, FreeVar, LoopProgram, Mate, MateFault, Node, NodeErrorKind,
    NodeResult, OnFace, Operand, OperandSlot, Orientation, Placed, PoseValue, ProfileDoc,
    RecipeNodeId, RoleSeg, SignedAxis, StableName, TargetAxis, ValuePayload, VarName, draw_on_face,
    evaluate,
};
use geom_core::{RigidMotion, Tol};

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn apply(doc: &ProfileDoc, edit: DocEdit<editor_core::ProfileProgram>) -> ProfileDoc {
    editor_core::apply(doc, &edit, Tol::witness())
        .unwrap_or_else(|e| panic!("the edit applies: {e}"))
        .doc
}

/// A `w` by `w` by `h` box on its own `xy`, its side `w` a document
/// parameter, and its extrude.
fn block(w: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("face_chart_block", Tol::witness());
    let doc = apply(
        &doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("w"),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, w)),
        },
    );
    let side = || Formula::named(VarName::from_static("w"), Dimension::Length);
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(editor_core::ProfileProgram {
            loops: vec![LoopProgram::polygon_expr([
                [len(0.0), len(0.0)],
                [side(), len(0.0)],
                [side(), side()],
                [len(0.0), side()],
            ])],
            ids: Vec::new(),
            on: None,
        }),
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(h),
        },
    )
}

fn cap(node: RecipeNodeId, end: CapEnd) -> StableName {
    fname(node, RoleSeg::Cap(end))
}

/// A disc of radius `r` about the profile's origin.
fn disc(r: f64) -> LoopProgram<Formula> {
    LoopProgram::Circle {
        centre: [len(0.0), len(0.0)],
        radius: len(r),
    }
}

/// A profile drawn on `face` of `body` through the door, and its node.
fn drawn(
    doc: &ProfileDoc,
    body: RecipeNodeId,
    face: StableName,
    on: OnFace,
) -> (ProfileDoc, RecipeNodeId) {
    drawn_loop(doc, body, face, on, disc(0.25))
}

/// [`drawn`], the loop `drawn`.
fn drawn_loop(
    doc: &ProfileDoc,
    body: RecipeNodeId,
    face: StableName,
    on: OnFace,
    drawn: LoopProgram<Formula>,
) -> (ProfileDoc, RecipeNodeId) {
    let out = draw_on_face(
        doc,
        BodyRead::plain(Operand::from(body)),
        face,
        vec![drawn],
        on,
        &EvalOptions::default(),
        &CancelToken::new(),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the door draws on the face: {e}"));
    (out.doc, out.profile)
}

/// The face's chart the profile is drawn on, as written: its frame
/// variable's definition.
fn written(doc: &ProfileDoc, profile: RecipeNodeId) -> PoseDef {
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("a profile")
    };
    let frame = program.on.as_ref().expect("drawn on a frame").frame;
    doc.var(frame)
        .and_then(|v| v.def().pose())
        .cloned()
        .expect("a pose definition")
}

/// The face's chart the profile is drawn on, as the profile's own
/// carriage decides it (over its loops' reach), or the profile's refusal.
fn chart(doc: &ProfileDoc, profile: RecipeNodeId) -> Result<RigidMotion<f64>, NodeErrorKind> {
    chart_in(&mut eval(doc), profile)
}

/// [`chart`], taken out of an evaluation.
fn chart_in(
    ev: &mut Evaluation<f64>,
    profile: RecipeNodeId,
) -> Result<RigidMotion<f64>, NodeErrorKind> {
    match ev.nodes.remove(&profile) {
        Some(NodeResult::Ok(value)) => match &value.payload {
            ValuePayload::Profile(drawn) => Ok(*drawn.drawn_frame().expect("drawn on a frame")),
            other => panic!("a profile's value, not {}", other.kind_name()),
        },
        Some(NodeResult::Failed(refused)) => Err(refused.kind),
        other => panic!("the profile runs: {other:?}"),
    }
}

/// Whether `refused` is the face's chart refusing backwards under
/// `orient`, naming an orientation to write that differs from it.
fn refuses_backwards(refused: &NodeErrorKind, orient: Orientation) -> bool {
    matches!(
        refused,
        NodeErrorKind::PoseDegenerate {
            construction: PoseConstruction::FaceChartBackwards { orient: o, write },
        } if *o == orient && *write != orient
    )
}

fn xyz(v: geom_core::Vec3<f64>) -> [f64; 3] {
    [v.x, v.y, v.z]
}

/// Every bit of a frame: its axes and its origin.
fn bits(f: &RigidMotion<f64>) -> Vec<u64> {
    let o = f.origin();
    [
        xyz(f.u().get()),
        xyz(f.v().get()),
        xyz(f.w().get()),
        [o.x, o.y, o.z],
    ]
    .concat()
    .into_iter()
    .map(f64::to_bits)
    .collect()
}

/// `node`'s body, a sweep's or a Boolean's, in its space, and its
/// vertices' heights, distinct and ascending.
fn heights(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<f64> {
    let value = ev.value(node).unwrap_or_else(|| {
        panic!("{node:?} evaluates: {:?}", ev.nodes.get(&node));
    });
    let body = match &value.payload {
        ValuePayload::Body(body) | ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body,
        other => panic!("a body, not {}", other.kind_name()),
    };
    let mut z: Vec<f64> = fixture::in_space(body)
        .vertex_points()
        .map(|(_, p)| p.z)
        .collect();
    z.sort_by(f64::total_cmp);
    z.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
    z
}

fn oriented(x: SignedAxis, z: SignedAxis) -> Orientation {
    Orientation::new(x, z).expect("two axes")
}

fn on(orient: Option<Orientation>, spin: f64, slides: [f64; 2]) -> OnFace {
    on_by(orient, ang(spin), slides)
}

fn on_by(orient: Option<Orientation>, spin: Formula, slides: [f64; 2]) -> OnFace {
    OnFace {
        orient,
        spin,
        slides: slides.map(len),
        traced: Vec::new(),
    }
}

/// **A bottom cap's chart is the least turn's identity under `x: +x,
/// z: −z`, and a `+` depth on it is a boss outward.** The cap's outward
/// normal is `−ẑ` exactly; the door writes the nearest orientation,
/// which puts the frame's `z` there, so the least turn is the identity
/// and the frame's axes are the space's under the orientation, every bit:
/// `x = +x`, `y = −y`, `z = −z`, at the space's origin.
///
/// *Breaks if* the chart carries the space's `+z` onto the cap by a bare
/// least turn (the pole every bottom cap sat on), or the door writes no
/// orientation into the recipe.
#[test]
fn a_bottom_caps_chart_is_the_identity_least_turn_and_a_boss_stands_outward() {
    let (doc, body) = block(2.0, 1.0);
    let (doc, profile) = drawn(&doc, body, cap(body, CapEnd::Start), OnFace::nearest());
    let want = oriented(SignedAxis::PlusX, SignedAxis::MinusZ);
    match written(&doc, profile) {
        PoseDef::FaceChart { orient, .. } => {
            assert_eq!(orient, want, "the door writes the nearest orientation")
        }
        other => panic!("a face's chart, not {other:?}"),
    }
    let f = chart(&doc, profile).expect("the bottom cap charts");
    assert_eq!(xyz(f.u().get()), [1.0, 0.0, 0.0], "x is the space's x");
    assert_eq!(xyz(f.v().get()), [0.0, -1.0, 0.0], "y is the space's −y");
    assert_eq!(xyz(f.w().get()), [0.0, 0.0, -1.0], "z is the outward −z");
    let o = f.origin();
    assert_eq!([o.x, o.y, o.z], [0.0, 0.0, 0.0], "at the space's origin");

    let (doc, boss) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(0.5),
        },
    );
    let (doc, whole) = fixture::insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![body.into(), boss.into()]),
        },
    );
    let ev = eval(&doc);
    assert_eq!(
        heights(&ev, boss),
        [-0.5, 0.0],
        "the boss stands below the cap"
    );
    assert_eq!(
        heights(&ev, whole),
        [-0.5, 0.0, 1.0],
        "the boss joins the block outward"
    );
}

/// **A top cap's chart is the space's own axes at the cap**: the nearest
/// orientation is the identity, the least turn the identity, and a `+`
/// depth stands above the cap.
#[test]
fn a_top_caps_chart_is_the_space_at_the_cap_and_a_boss_stands_above() {
    let (doc, body) = block(2.0, 1.0);
    let (doc, profile) = drawn(&doc, body, cap(body, CapEnd::End), OnFace::nearest());
    match written(&doc, profile) {
        PoseDef::FaceChart { orient, .. } => assert_eq!(orient, Orientation::IDENTITY),
        other => panic!("a face's chart, not {other:?}"),
    }
    let f = chart(&doc, profile).expect("the top cap charts");
    assert_eq!(xyz(f.u().get()), [1.0, 0.0, 0.0]);
    assert_eq!(xyz(f.v().get()), [0.0, 1.0, 0.0]);
    assert_eq!(xyz(f.w().get()), [0.0, 0.0, 1.0]);
    let o = f.origin();
    assert_eq!(
        [o.x, o.y, o.z],
        [0.0, 0.0, 1.0],
        "on the cap, over the origin"
    );
    let (doc, boss) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(0.5),
        },
    );
    assert_eq!(heights(&eval(&doc), boss), [1.0, 1.5]);
}

/// The block's copy in the world, turned off every axis, and its
/// placement.
fn tilted() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    tilted_by([0.3, -0.2, 0.1], [0.5, -0.25, 0.75])
}

/// The block's copy in the world, spun about `x`, `y` and `z` in turn
/// and slid along them, and its placement.
fn tilted_by(spins: [f64; 3], slides: [f64; 3]) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, body) = block(2.0, 1.0);
    let axes = [TargetAxis::X, TargetAxis::Y, TargetAxis::Z];
    let spun = axes
        .iter()
        .zip(spins)
        .map(|(&axis, by)| (Coord::Spin(Some(axis)), ang(by)));
    let slid = axes
        .iter()
        .zip(slides)
        .map(|(&axis, by)| (Coord::slide(axis), len(by)));
    let (doc, copy) = fixture::in_world_by(doc, body, spun.chain(slid).collect());
    (doc, body, copy)
}

/// **A tilted face's chart stands on the face, its `z` the face's
/// outward normal**: the copy's top cap, turned off every world axis,
/// read as the plane it is.
#[test]
fn a_tilted_faces_chart_stands_on_the_face_along_its_outward_normal() {
    let (doc, body, copy) = tilted();
    let top = cap(body, CapEnd::End);
    let (doc, profile) = drawn(&doc, copy, top.clone(), on(None, 0.0, [0.0, 0.0]));
    let f = chart(&doc, profile).expect("the tilted cap charts");
    let (doc, split) = split_at(doc, copy, top);
    let (n, offset) = tool_plane(&doc, &eval(&doc), split);
    for (k, (a, b)) in xyz(f.w().get()).into_iter().zip(n).enumerate() {
        assert!((a - b).abs() < 1e-12, "z[{k}] = {a} is the normal's {b}");
    }
    let o = f.origin();
    let along = o.x * n[0] + o.y * n[1] + o.z * n[2];
    assert!(
        (along - offset).abs() < 1e-12,
        "the origin lies on the face's plane: {along} vs {offset}"
    );
}

/// **Within the band about the least turn's pole the chart refuses,
/// naming the orientation to write, and that orientation charts.** A
/// bottom cap under the identity puts the frame's `z` on `+z`, exactly
/// opposite the cap's outward normal.
#[test]
fn the_pole_refuses_naming_the_orientation_to_write() {
    let (doc, body) = block(2.0, 1.0);
    let (doc, profile) = drawn(
        &doc,
        body,
        cap(body, CapEnd::Start),
        on(Some(Orientation::IDENTITY), 0.0, [0.0, 0.0]),
    );
    let refused = chart(&doc, profile);
    let Err(NodeErrorKind::PoseDegenerate {
        construction: PoseConstruction::FaceChartBackwards { orient, write },
    }) = refused
    else {
        panic!("the pole refuses typed: {refused:?}")
    };
    assert_eq!(orient, Orientation::IDENTITY, "it names what was written");
    assert_eq!(
        xyz(write.apply(geom_core::Vec3::<f64>::unit_z())),
        [0.0, 0.0, -1.0],
        "the orientation to write puts z on the outward normal: {write}"
    );
    let text = NodeErrorKind::PoseDegenerate {
        construction: PoseConstruction::FaceChartBackwards { orient, write },
    }
    .to_string();
    assert!(
        text.contains(&format!("Recourse: write the orientation {write}")),
        "{text}"
    );
    let (doc, mended) = drawn(
        &doc,
        body,
        cap(body, CapEnd::Start),
        on(Some(write), 0.0, [0.0, 0.0]),
    );
    let f = chart(&doc, mended).expect("the recourse charts");
    assert_eq!(xyz(f.w().get()), [0.0, 0.0, -1.0]);
}

/// **A re-dimension that keeps the face's plane leaves the chart bit for
/// bit**: the block's side and height change, its bottom cap's plane does
/// not, and nothing about the cap's cells enters its chart.
#[test]
fn a_redimension_that_keeps_the_plane_leaves_the_chart_bit_equal() {
    let (doc, body) = block(2.0, 1.0);
    let (doc, profile) = drawn(
        &doc,
        body,
        cap(body, CapEnd::Start),
        on(None, 0.3, [0.2, -0.1]),
    );
    let before = chart(&doc, profile).expect("the cap charts");
    let doc = apply(
        &doc,
        DocEdit::SetVarValue {
            var: VarName::from_static("w").into(),
            value: FreeValue::Continuous(3.0),
        },
    );
    let doc = apply(
        &doc,
        DocEdit::SetParam {
            node: body,
            slot: editor_core::SlotId::Distance,
            value: len(1.5).into(),
            fresh: Vec::new(),
        },
    );
    let after = chart(&doc, profile).expect("the re-dimensioned cap charts");
    assert_eq!(bits(&before), bits(&after), "{before:?} vs {after:?}");
}

/// The motion a plane placement of a cube of side `side` onto `face` of
/// `copy` solves to, under `orient` and the values a face's chart
/// writes: its start cap flipped to look up its own `z`, as DM1 places
/// a body on a face. Its fault where it refuses.
fn placed_on(
    doc: ProfileDoc,
    (copy, face): (RecipeNodeId, StableName),
    orient: Orientation,
    (spin, slides): (Formula, [f64; 2]),
    side: f64,
) -> Result<geom_core::Affine3<f64>, MateFault> {
    let (doc, square) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(vec![vec![
            (0.0, 0.0),
            (side, 0.0),
            (side, side),
            (0.0, side),
        ]])),
    );
    let (doc, source) = fixture::insert(
        doc,
        Node::Extrude {
            profile: square.into(),
            distance: len(side),
        },
    );
    let (doc, placement) =
        fixture::insert(doc, fixture::stood(BodyRead::plain(Operand::from(source))));
    let mate = Mate {
        on: fixture::flip(fixture::plane(Operand::select(
            source,
            vec![cap(source, CapEnd::Start)],
        ))),
        to: fixture::plane(Operand::select(
            BodyRead::plain(Operand::from(copy)),
            vec![face],
        )),
        orient,
    };
    let [a, b] = face_chart_slides(orient);
    let placed = Placed {
        mates: vec![mate],
        values: vec![
            (Coord::slide(a), len(slides[0])),
            (Coord::slide(b), len(slides[1])),
            (Coord::SPIN, spin),
        ],
    };
    let (doc, _) = fixture::write_bundle(doc, placement, placed.bundle());
    let poses = fixture::solve(&doc, &EvalOptions::default(), Tol::witness());
    if let Some(fault) = poses.fault(placement) {
        return Err(fault.clone());
    }
    Ok(poses.relative_map(placement).expect("the block's motion"))
}

/// Every bit of an affine map: its linear part's columns, then its
/// translation.
fn affine_bits(a: &geom_core::Affine3<f64>) -> Vec<u64> {
    [
        xyz(a.linear.c0),
        xyz(a.linear.c1),
        xyz(a.linear.c2),
        xyz(a.translation),
    ]
    .concat()
    .into_iter()
    .map(f64::to_bits)
    .collect()
}

/// On the tilted face, under the orientation the door wrote, the bits of
/// the chart a profile is drawn on with `spin` and two slides, and of the
/// motion a plane placement of a block onto the face with the same values
/// solves to.
fn chart_and_placement(spin: Formula) -> (Vec<u64>, Vec<u64>) {
    let (doc, body, copy) = tilted();
    let top = cap(body, CapEnd::End);
    let slides = [0.25, -0.15];
    let (doc, profile) = drawn(&doc, copy, top.clone(), on_by(None, spin.clone(), slides));
    let PoseDef::FaceChart { orient, .. } = written(&doc, profile) else {
        panic!("a face's chart")
    };
    let f = chart(&doc, profile)
        .expect("the tilted cap charts")
        .to_affine();
    let m = placed_on(doc, (copy, top), orient, (spin, slides), 1.0)
        .unwrap_or_else(|f| panic!("the block is placed: {f:?}"));
    (affine_bits(&f), affine_bits(&m))
}

/// **The chart is the plane placement's, with the same values**: on the
/// tilted face, under the orientation the door wrote, a spin and two
/// slides, a profile's chart and the motion a plane placement of a block
/// onto the face solves to are one map, every bit.
///
/// *Breaks if* the chart is spelled a second time beside the
/// placement's: any reordering of its arithmetic moves a low bit.
#[test]
fn the_chart_is_the_plane_placements_with_the_same_values() {
    let (chart, placed) = chart_and_placement(ang(0.4));
    assert_eq!(chart, placed, "the chart is the placement's, every bit");
}

/// **The chart is the placement's for a spin written as a fraction of a
/// turn**: the placement turns a third of a turn by the sine and cosine
/// of the form it is written in (a cosine of `−1/2` exactly), and so
/// does the chart.
///
/// *Breaks if* the chart reads the spin's number and drops its form,
/// turning by the radians' rounding where the placement turns exactly.
/// An eighth of a turn cannot see that: its sine and cosine round alike
/// from the radians.
#[test]
fn the_chart_is_the_placements_for_a_spin_written_as_a_fraction_of_a_turn() {
    let third = Formula::turn_over(3).expect("a third of a turn");
    let (chart, placed) = chart_and_placement(third);
    assert_eq!(chart, placed, "the chart is the placement's, every bit");
}

/// The tilted block's top cap `height` from the origin, under the
/// backwards orientation `x: +x, z: −z` tilted `tilt` off its pole: the
/// document, the block's copy, the cap, and the orientation.
fn near_the_pole(tilt: f64, height: f64) -> (ProfileDoc, RecipeNodeId, StableName, Orientation) {
    let (doc, body, copy) = tilted_by([tilt, 0.0, 0.0], [0.0, 0.0, height - 1.0]);
    let top = cap(body, CapEnd::End);
    (
        doc,
        copy,
        top,
        oriented(SignedAxis::PlusX, SignedAxis::MinusZ),
    )
}

/// What a boss drawn on the cap and the same body placed there do at
/// `tilt`: a square of side `side` from the frame's origin drawn on the
/// face's chart under the backwards orientation and extruded `side`
/// (the cube), and a cube of side `side` placed on the same face by a
/// plane mate under the same orientation. The profile's result, the
/// boss's, and the placement's.
struct Carried {
    profile: Result<RigidMotion<f64>, NodeErrorKind>,
    /// `Err(None)` where the boss never ran, its profile having refused.
    boss: Result<(), Option<NodeErrorKind>>,
    placed: Result<geom_core::Affine3<f64>, MateFault>,
}

fn carried(tilt: f64, height: f64, side: f64) -> Carried {
    let (doc, copy, top, backwards) = near_the_pole(tilt, height);
    let at = on(Some(backwards), 0.0, [0.0, 0.0]);
    let square = LoopProgram::polygon([(0.0, 0.0), (side, 0.0), (side, side), (0.0, side)])
        .expect("finite corners");
    let (drawn_doc, profile) = drawn_loop(&doc, copy, top.clone(), at, square);
    let (drawn_doc, boss) = fixture::insert(
        drawn_doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(side),
        },
    );
    let mut ev = eval(&drawn_doc);
    let boss = match ev.nodes.remove(&boss) {
        Some(NodeResult::Ok(_)) => Ok(()),
        Some(NodeResult::Failed(refused)) => Err(Some(refused.kind)),
        Some(NodeResult::Poisoned { through }) if through == profile => Err(None),
        other => panic!("the boss runs: {other:?}"),
    };
    Carried {
        profile: chart_in(&mut ev, profile),
        boss,
        placed: placed_on(doc, (copy, top), backwards, (ang(0.0), [0.0, 0.0]), side),
    }
}

/// **Near the pole a wide boss refuses where the equal placed cube
/// does, past its profile's edge** (`[placement]`: what stands in a
/// face's chart is carried over its own reach). A top cap `100` from the
/// origin, tilted about `x` off the pole of the orientation `x: +x,
/// z: −z`. A square of side 500 drawn on the chart is carried over its
/// loops' reach, `500·√2 + 100`; its boss of height 500, the cube, over
/// its own, `500·√3 + 100`, the arm a placed cube of side 500 levers the
/// same turn over. The tilt sits between the two arms' conditioning
/// edges ([`geom_core::test_support::least_turn_band_edge`]): the profile
/// charts, and its boss refuses backwards, naming the orientation to
/// write, as the placed cube does.
#[test]
fn near_the_pole_a_wide_boss_refuses_where_the_equal_placed_cube_does() {
    let tol = Tol::witness();
    let (side, height) = (500.0, 100.0);
    let (loops_arm, cube_arm) = (side * 2f64.sqrt() + height, side * 3f64.sqrt() + height);
    let edge = |arm| geom_core::test_support::least_turn_band_edge(tol, arm);
    let pole = |arm| tol.k() * tol.eps() / arm;
    let (charts, refuses) = (pole(loops_arm).max(edge(loops_arm)), edge(cube_arm));
    assert!(
        1.1 * charts < refuses,
        "FIXTURE: the profile's verdict {charts} and the cube's {refuses} leave room between them"
    );
    let tilt = (charts * refuses).sqrt();
    let backwards = oriented(SignedAxis::PlusX, SignedAxis::MinusZ);
    let run = carried(tilt, height, side);
    run.profile
        .unwrap_or_else(|e| panic!("the profile charts at tilt {tilt}: {e:?}"));
    assert!(
        run.boss
            .as_ref()
            .is_err_and(|e| e.as_ref().is_some_and(|e| refuses_backwards(e, backwards))),
        "the boss refuses backwards at tilt {tilt}: {:?}",
        run.boss
    );
    assert!(
        matches!(run.placed, Err(MateFault::Backwards { orient, .. }) if orient == backwards),
        "the equal placed cube refuses backwards at tilt {tilt}: {:?}",
        run.placed
    );
}

/// **A boss on a face and the same body placed there agree on both
/// edges of the pole's band**: the band has two edges that move apart
/// as the arm grows (the pole verdict's, `Kε` over the arm, and the
/// conditioning's, `2·arm/R`), and at each a boss drawn on the face's
/// chart refuses and builds where the equal cube placed by a plane mate
/// does, both carried over the cube's reach.
///
/// - **The wide edge**: a cube of side 500 at 100 from the origin, at
///   half its conditioning edge (refused) and at twice it (built).
/// - **The narrow edge**: a cube a twentieth of the arm at which the two
///   edges meet (`√(Kε·R/2)`), as far from the origin as it is wide, at
///   half its pole edge (refused) and four times its profile's (built).
///   Below the narrow edge the profile, the shorter arm, refuses first,
///   so the boss refuses through it, as a placement of the profile's
///   size would.
#[test]
fn a_boss_and_the_equal_placed_cube_agree_on_both_edges_of_the_band() {
    let tol = Tol::witness();
    let edge = |arm| geom_core::test_support::least_turn_band_edge(tol, arm);
    let pole = |arm| tol.k() * tol.eps() / arm;
    let backwards = oriented(SignedAxis::PlusX, SignedAxis::MinusZ);
    let arms = |side: f64, height: f64| (side * 2f64.sqrt() + height, side * 3f64.sqrt() + height);
    let refused = |row: &str, tilt, height, side| {
        let run = carried(tilt, height, side);
        assert!(
            run.boss.is_err(),
            "{row}: the boss refuses at tilt {tilt}: {:?}",
            run.profile
        );
        assert!(
            matches!(run.placed, Err(MateFault::Backwards { orient, .. }) if orient == backwards),
            "{row}: the placed cube refuses backwards at tilt {tilt}: {:?}",
            run.placed
        );
        run
    };
    let built = |row: &str, tilt, height, side| {
        let run = carried(tilt, height, side);
        assert!(
            run.profile.is_ok() && run.boss.is_ok(),
            "{row}: the profile and its boss build at tilt {tilt}: {:?} / {:?}",
            run.profile.err(),
            run.boss
        );
        assert!(
            run.placed.is_ok(),
            "{row}: the placed cube builds at tilt {tilt}: {:?}",
            run.placed
        );
    };

    let (side, height) = (500.0, 100.0);
    let (_, cube) = arms(side, height);
    assert!(
        pole(cube) < 0.1 * edge(cube),
        "FIXTURE: the wide cube's band is its conditioning edge"
    );
    refused("wide edge", 0.5 * edge(cube), height, side);
    built("wide edge", 2.0 * edge(cube), height, side);

    let meet = (tol.k() * tol.eps() * tol.rounding_reach() / 2.0).sqrt();
    let side = meet / 20.0;
    let (loops, cube) = arms(side, side);
    assert!(
        edge(loops) < 0.1 * pole(cube),
        "FIXTURE: the narrow cube's band is its pole verdict"
    );
    let narrow = refused("narrow edge", 0.5 * pole(cube), side, side);
    assert!(
        narrow
            .profile
            .as_ref()
            .is_err_and(|e| refuses_backwards(e, backwards)),
        "narrow edge: the profile refuses backwards first: {:?}",
        narrow.profile
    );
    built("narrow edge", 4.0 * pole(loops), side, side);
}

/// **A tilted face through its space's origin charts with zero slides**:
/// the block's copy spun about `x`, its bottom cap's plane through the
/// origin, drawn on under the nearest orientation with no slides. The
/// chart's arm, its slides and the face's distance, is zero there, and
/// what the frame carries reaches out to the disc's radius: the profile
/// charts and its boss builds.
///
/// *Breaks if* the frame decides its turn over its own arm alone: it
/// then refuses backwards naming the orientation already written.
#[test]
fn a_tilted_face_through_its_spaces_origin_charts_at_zero_slides() {
    let (doc, body, copy) = tilted_by([0.2, 0.0, 0.0], [0.0, 0.0, 0.0]);
    let (doc, profile) = drawn(
        &doc,
        copy,
        cap(body, CapEnd::Start),
        on(None, 0.0, [0.0, 0.0]),
    );
    let f = chart(&doc, profile).unwrap_or_else(|e| panic!("the cap charts: {e:?}"));
    let o = f.origin();
    assert!(
        o.x.abs() + o.y.abs() + o.z.abs() < 1e-12,
        "the frame stands at the origin, on the face: {o:?}"
    );
    let (doc, boss) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(0.5),
        },
    );
    let ev = eval(&doc);
    assert!(
        matches!(ev.nodes.get(&boss), Some(NodeResult::Ok(_))),
        "its boss builds: {:?}",
        ev.nodes.get(&boss)
    );
}

/// **A face's chart read anywhere but a profile's frame seat refuses
/// typed**: the frame variable a profile is drawn on, bound by the pose
/// door as a split's tool or a mate's side would bind it, refuses
/// [`PoseConstruction::FaceChartUnseated`], so nothing carries geometry
/// by the chart unlevered. The profile itself, drawn on it, charts.
#[test]
fn a_face_chart_read_outside_a_frame_seat_refuses() {
    let (doc, body) = block(2.0, 1.0);
    let (doc, profile) = drawn(&doc, body, cap(body, CapEnd::End), OnFace::nearest());
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("a profile")
    };
    let frame = program.on.as_ref().expect("drawn on a frame").frame;
    let mut ev = eval(&doc);
    for slot in [OperandSlot::Tool, OperandSlot::Frame] {
        let read = bound_pose(&doc, &ev, slot, frame, Tol::witness());
        assert!(
            matches!(
                read,
                Err(NodeErrorKind::PoseDegenerate {
                    construction: PoseConstruction::FaceChartUnseated
                })
            ),
            "{slot:?}: the pose door refuses the chart: {read:?}"
        );
    }
    let text = NodeErrorKind::PoseDegenerate {
        construction: PoseConstruction::FaceChartUnseated,
    }
    .to_string();
    assert!(
        text.contains("Recourse: draw a profile on the chart itself"),
        "{text}"
    );
    chart_in(&mut ev, profile).unwrap_or_else(|e| panic!("the profile charts: {e:?}"));
}

/// **An un-swept profile on a face stands in a decided frame**: nothing
/// sweeps the disc drawn on the top cap, and the profile's own carriage,
/// over its loops' reach, decides the chart: the frame stands on the cap.
#[test]
fn an_unswept_profile_on_a_face_has_its_frame() {
    let (doc, body) = block(2.0, 1.0);
    let (doc, profile) = drawn(&doc, body, cap(body, CapEnd::End), OnFace::nearest());
    let f = chart(&doc, profile).unwrap_or_else(|e| panic!("the profile charts: {e:?}"));
    let o = f.origin();
    assert_eq!([o.x, o.y, o.z], [0.0, 0.0, 1.0], "on the cap");
}

/// The plane `split`'s tool reads, as its reader binds it.
fn tool_plane(doc: &ProfileDoc, ev: &Evaluation<f64>, split: RecipeNodeId) -> ([f64; 3], f64) {
    let Some(Node::Split { tool, .. }) = doc.node(split) else {
        panic!("a split")
    };
    match bound_pose(doc, ev, OperandSlot::Tool, *tool, Tol::witness()) {
        Ok(PoseValue::Plane { normal, offset }) => (xyz(normal.get()), offset),
        other => panic!("a plane, not {other:?}"),
    }
}

/// `body` split by the plane of its face `face`, inserted: the reader a
/// row binds the face's plane at.
fn split_at(doc: ProfileDoc, body: RecipeNodeId, face: StableName) -> (ProfileDoc, RecipeNodeId) {
    let plane = fixture::plane(Operand::select(
        BodyRead::plain(Operand::from(body)),
        vec![face],
    ));
    fixture::insert(
        doc,
        Node::Split {
            target: Operand::from(body).into(),
            tool: plane.into(),
        },
    )
}

/// **A `−` depth is a pocket's tool: the sweep of its size whose start
/// cap sits the depth along `z`.** Drawn on the block's top cap and swept
/// `−0.5`, the tool fills `[0.5, 1]` under the cap, its start cap the
/// deep one (outward `−z`) and its end cap on the face; cut from the
/// block it leaves a pocket half a unit deep.
#[test]
fn a_negative_depth_is_a_pockets_tool_under_the_face() {
    let (doc, body) = block(2.0, 1.0);
    let (doc, profile) = drawn(&doc, body, cap(body, CapEnd::End), OnFace::nearest());
    let (doc, tool) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(-0.5),
        },
    );
    let (doc, at_start) = split_at(doc, tool, cap(tool, CapEnd::Start));
    let (doc, at_end) = split_at(doc, tool, cap(tool, CapEnd::End));
    let (doc, pocket) = fixture::insert(
        doc,
        Node::Subtract {
            from: Operand::from(body).into(),
            tool: Operand::from(tool).into(),
        },
    );
    let ev = eval(&doc);
    assert_eq!(
        heights(&ev, tool),
        [0.5, 1.0],
        "the tool lies under the cap"
    );
    assert_eq!(
        tool_plane(&doc, &ev, at_start),
        ([0.0, 0.0, -1.0], -0.5),
        "the start cap sits the depth along z, facing down"
    );
    assert_eq!(
        tool_plane(&doc, &ev, at_end),
        ([0.0, 0.0, 1.0], 1.0),
        "the end cap is the face's plane"
    );
    assert_eq!(
        heights(&ev, pocket),
        [0.0, 0.5, 1.0],
        "the pocket is half a unit deep"
    );
}

/// **A zero depth refuses**, typed, as the sweep's own degenerate
/// extrusion.
#[test]
fn a_zero_depth_refuses() {
    let (doc, body) = block(2.0, 1.0);
    let (doc, profile) = drawn(&doc, body, cap(body, CapEnd::End), OnFace::nearest());
    let (doc, flat) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: Formula::zero(Dimension::Length),
        },
    );
    match eval(&doc).nodes.get(&flat) {
        Some(NodeResult::Failed(e)) => assert!(
            matches!(
                e.kind,
                NodeErrorKind::Extrude(sweep::ExtrudeError::DegenerateExtrusion)
            ),
            "{:?}",
            e.kind
        ),
        other => panic!("a zero depth refuses, got {other:?}"),
    }
}

/// **A pocket on a bottom cap lies inside the body**: drawn on the cap,
/// whose chart's `z` is the outward `−z`, and swept `−0.5`, the tool
/// fills `[0, 0.5]`. Its start cap sits the depth along the chart's `z`,
/// at `0.5` facing `+z`, into the body; its end cap is the face's plane.
///
/// *Breaks if* the lift runs along the space's `z` rather than the
/// chart's (the chart and the lift composed the other way about), which
/// puts the tool under the cap; on a top cap the two agree.
#[test]
fn a_pocket_on_a_bottom_cap_lies_inside_the_body() {
    let (doc, body) = block(2.0, 1.0);
    let (doc, profile) = drawn(&doc, body, cap(body, CapEnd::Start), OnFace::nearest());
    let (doc, tool) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(-0.5),
        },
    );
    let (doc, at_start) = split_at(doc, tool, cap(tool, CapEnd::Start));
    let (doc, at_end) = split_at(doc, tool, cap(tool, CapEnd::End));
    let (doc, pocket) = fixture::insert(
        doc,
        Node::Subtract {
            from: Operand::from(body).into(),
            tool: Operand::from(tool).into(),
        },
    );
    let ev = eval(&doc);
    assert_eq!(
        heights(&ev, tool),
        [0.0, 0.5],
        "the tool lies over the cap, inside the body"
    );
    assert_eq!(
        tool_plane(&doc, &ev, at_start),
        ([0.0, 0.0, 1.0], 0.5),
        "the start cap sits the depth along the chart's z, facing into the body"
    );
    assert_eq!(
        tool_plane(&doc, &ev, at_end),
        ([0.0, 0.0, -1.0], 0.0),
        "the end cap is the face's plane"
    );
    assert_eq!(
        heights(&ev, pocket),
        [0.0, 0.5, 1.0],
        "the pocket is half a unit deep"
    );
}

/// **A sweep lifted past the session box refuses at its lift**
/// (`[placement.frame]`: a door that stands a body somewhere asks where
/// it stands). The block's copy 700 m up `z`, a disc drawn on its bottom
/// cap and extruded by a `−400` depth, the pocket's tool through the
/// block and on up: the sweep builds 400 m long in its own coordinates,
/// inside the box, and the negative depth lifts it
/// (`ProfileValue::lifted`) to stand from 700 m to 1100 m. The extrude
/// refuses with the session box's finding alone; the copy it is drawn
/// on stands inside the box.
///
/// *Breaks if* the lifted door keeps its frame without asking: the
/// extrude then builds past the box.
#[test]
fn a_sweep_lifted_past_the_box_refuses_at_its_lift() {
    let (doc, body) = block(2.0, 1.0);
    let (doc, copy) = editor_core::test_support::in_world(
        doc,
        body,
        &editor_core::Frame::translation([0.0, 0.0, 700.0]),
        Tol::witness(),
    );
    let (doc, profile) = drawn(&doc, copy, cap(body, CapEnd::Start), OnFace::nearest());
    let (doc, tool) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(-400.0),
        },
    );
    let ev = eval(&doc);
    assert!(ev.value(copy).is_some(), "the copy stands inside the box");
    let refused = ev.node_error(tool).expect("the lifted extrude refuses");
    assert!(
        refused.node == tool
            && matches!(
                &refused.kind,
                NodeErrorKind::NotFinished { findings }
                    if matches!(&findings[..], [topo::ValidationError::OutsideSessionBox { .. }])
            ),
        "at its lift, with the session box's finding alone: {refused:?}"
    );
}

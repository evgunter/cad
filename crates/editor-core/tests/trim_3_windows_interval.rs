//! **Clearance cells decided against their faces**, from outside,
//! through the public doors only.
//!
//! **E4** is the planted-tight approach that says dropping cells is not
//! a blanket `Holds`; **E7** and **E7b** are cylinder bands, one on each
//! side of zero; **E8** a revolved band beside the material it does not
//! have; and the quarter ball is the same question on a sphere, its
//! verdict checked against a dense model-space distance.
//!
//! The basename carries `interval` because the suite's subject is the
//! certified scalar.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;

use crate::fixture::clearance;
use editor_core::UnitSym;
use editor_core::analysis::{BoxAxis, ParamBox};
use editor_core::clearance::{
    Budgets, ClearanceBound, ClearanceConfig, ClearanceQuery, ClearanceRefusal, ClearanceReport,
    ClearanceVerdict, FaceScope, NoTangents, Pruning, Selection,
};
use editor_core::{
    CapEnd, Dimension, Distribution, DocEdit, Formula, FreeVar, LoopProgram, Node, ProfileDoc,
    ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, RoleSeg, VarName,
};
use geom_core::Tol;

use fixture::{Recorder, ang, len, len2, scl};

// ------------------------------------------------------------ authoring

/// The analysis box's half-width — the M10-5 suites' ε/64, for their
/// reason (no node's interval replay builds over a wider one).
fn half() -> f64 {
    Tol::witness().eps() / 64.0
}

fn eps() -> f64 {
    Tol::witness().eps()
}

/// The funnel's escalation threshold in metres: the width inside which
/// nothing is dropped and no witness coordinate is claimed exact.
fn k_eps() -> f64 {
    Tol::witness().k() * eps()
}

fn name(n: &'static str) -> VarName {
    VarName::from_static(n)
}

fn box_of(doc: &ProfileDoc, axis: &'static str) -> ParamBox {
    let mut axes = BTreeMap::new();
    axes.insert(
        doc.var_named(axis).expect("the fixture declares the axis"),
        BoxAxis::Varying {
            lo: -half(),
            hi: half(),
        },
    );
    ParamBox::from_axes(axes)
}

fn declare(r: &mut Recorder, axis: &'static str, nominal: f64) {
    r.push(DocEdit::DeclareVar {
        name: name(axis),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Length,
            value: nominal,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -half(),
                hi: half(),
            }),
        }),
    });
}

/// The sentinel slide [`translated`] places at, then re-spells.
const SENTINEL: f64 = 0.5;

/// **A copy of `input` slid by `d` in `target`'s own coordinates** (the
/// world's where `target` is `None`), each component a formula: placed
/// against `target` at a literal slide along `z`
/// ([`editor_core::test_support::placed_against`]; three slides with no mate in the
/// world), the mate's slide then
/// re-spelled as `d[2]` and its `x` and `y` values as `d[0]` and `d[1]`.
/// Returns the placement, whose output is the copy.
fn translated(
    r: &mut Recorder,
    target: Option<RecipeNodeId>,
    input: RecipeNodeId,
    d: [Formula; 3],
) -> RecipeNodeId {
    use editor_core::pose::PoseDef;
    use editor_core::{Coord, Operand, TargetAxis};
    let [dx, dy, dz] = d;
    let body = Operand::Node(input);
    let at = editor_core::Frame::translation([0.0, 0.0, SENTINEL]);
    // With no target, the copy stands in the world by its three slides.
    let Some(target) = target else {
        let placement = r.insert(crate::fixture::stood(editor_core::BodyRead::plain(body)));
        r.write(
            placement,
            editor_core::Placed::unmated(vec![
                (Coord::Slide(TargetAxis::X), dx),
                (Coord::Slide(TargetAxis::Y), dy),
                (Coord::Slide(TargetAxis::Z), dz),
            ]),
        );
        return placement;
    };
    let placed = editor_core::test_support::placed_against(
        &r.doc,
        &fixture::body_read(&r.doc, target.into()),
        &body,
        &at,
        Tol::witness(),
    );
    let lit = |v: f64| Formula::literal(v, Dimension::Length).expect("a literal");
    let mut replaced = 0;
    fn walk(op: &mut Operand, f: &mut dyn FnMut(&mut Formula)) {
        if let Operand::Pose(def) = op {
            match &mut **def {
                PoseDef::Slide { pose, by, .. } => {
                    f(by);
                    walk(&mut pose.read, f);
                }
                PoseDef::Spin { pose, .. }
                | PoseDef::Flip { pose }
                | PoseDef::Carried { pose, .. } => walk(&mut pose.read, f),
                _ => {}
            }
        }
    }
    let mut respell = |by: &mut Formula| {
        if *by == lit(SENTINEL) {
            *by = dz.clone();
            replaced += 1;
        } else if *by == lit(-SENTINEL) {
            *by = Formula::neg(dz.clone()).expect("a length");
            replaced += 1;
        }
    };
    let mut mate = placed
        .mates
        .into_iter()
        .next()
        .expect("the copy is placed by a mate");
    walk(&mut mate.on, &mut respell);
    walk(&mut mate.to, &mut respell);
    assert_eq!(replaced, 1, "the mate slides the copy once: {mate:?}");
    let values = placed
        .values
        .into_iter()
        .map(|(coord, by)| match coord {
            Coord::Slide(TargetAxis::X) => (coord, dx.clone()),
            Coord::Slide(TargetAxis::Y) => (coord, dy.clone()),
            _ => (coord, by),
        })
        .collect();
    let placement = r.insert(crate::fixture::stood(editor_core::BodyRead::plain(body)));
    r.constrain(placement, mate, values);
    placement
}

fn extruded(r: &mut Recorder, points: &[(f64, f64)], depth: f64) -> RecipeNodeId {
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::polygon(points.iter().copied()).expect("finite corners")],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(depth),
    })
}

fn cfg(pairs: usize, depth: u32) -> ClearanceConfig {
    ClearanceConfig {
        budgets: Budgets {
            max_cell_depth: depth,
            max_cell_pairs: pairs,
        },
        pruning: Pruning::Off,
    }
}

fn at_least(c: f64, config: ClearanceConfig) -> ClearanceQuery<'static> {
    ClearanceQuery {
        bound: ClearanceBound::at_least(c),
        tol: Tol::witness(),
        config,
        oracle: &NoTangents,
        resolver: None,
        symbolic: editor_core::drive::SymbolicDials::default(),
    }
}

fn named(at: RecipeNodeId, names: Vec<editor_core::StableName>) -> Selection {
    Selection {
        at,
        body: 0,
        faces: FaceScope::Named(names),
    }
}

/// The distance between a violation witness's own two points,
/// recomputed here from the report's public fields.
fn witness_distance(report: &ClearanceReport) -> f64 {
    let ClearanceVerdict::Violated(v) = report.verdict() else {
        panic!("expected a violation, got {}", report.verdict().label());
    };
    let (a, b) = (v.geometry.a_point, v.geometry.b_point);
    let (dx, dy, dz) = (a.x - b.x, a.y - b.y, a.z - b.z);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

// --------------------------------------------- E4: the planted-tight approach

/// The M10-5 R2 L — outer loop `(0,0) (1,0) (1,0.4) (0.4,0.4) (0.4,1)
/// (0,1)`, extruded 1 — with a block PLANTED 0.05 from the notch wall
/// at `x = 0.4` instead of deep in the notch: `x ∈ [0.45, 0.55]`,
/// `y ∈ [0.85, 0.95]`, straddling the bottom cap's plane `z = 0`.
fn ell_with_a_planted_block() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    declare(&mut r, "place", 0.0);
    let solid = extruded(
        &mut r,
        &[
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 0.4),
            (0.4, 0.4),
            (0.4, 1.0),
            (0.0, 1.0),
        ],
        1.0,
    );
    let probe = extruded(
        &mut r,
        &[(0.45, 0.85), (0.55, 0.85), (0.55, 0.95), (0.45, 0.95)],
        0.1,
    );
    let placed = translated(
        &mut r,
        Some(solid),
        probe,
        [
            Formula::named(name("place"), Dimension::Length),
            Formula::zero(Dimension::Length),
            len(-0.05),
        ],
    );
    (r.doc, solid, placed)
}

/// **E4 — the tightening is not a blanket `Holds`.** The same L cap
/// whose window covers the notch, and a block the cap's MATERIAL
/// really does come within 0.05 of: the nearest face point is on the
/// notch wall at `x = 0.4`, the block starts at `x = 0.45`, and the
/// block straddles the cap's plane, so the true face-to-face distance
/// is 0.05. A bound of 0.3 is violated, and it must still be reported.
///
/// **What this row kills, measured.** An inverted region reading would
/// certify the MATERIAL outside, drop the cells that
/// hold the real approach, and leave `Holds`. That is the one mutant
/// the v6 dual reproduced here, on both arms.
///
/// **What it does NOT kill, also measured**, and stated because the
/// PR that landed it claimed otherwise. *Drop-on-indeterminate* — a
/// separating-axis test that lets `Zero`, an in-band margin or poison
/// count as separating — leaves this row green; so does *a
/// region reading tight by `≥ 0.05`* (cells eroded by that much). The
/// one-sided assertion that used to be credited with catching the
/// second was monotone the wrong way, since a tighter reading
/// pushes the cap witness to SMALLER x, and the two-sided pin below
/// replaces it — but the honest result of re-running both mutants
/// against it is that **this row still does not catch either**.
///
/// What catches them is the pinned receipts on
/// `m10_5_r1_probes_interval::an_l_shaped_face_holds_where_it_has_no_material`
/// and `::e2e_channel_slider_over_an_epsilon_box`, whose discharge and
/// split counts move when the drop rule changes at all, and `sweep`'s
/// `cell_region_bodies`, which checks every decided cell against an
/// independent oracle. A drop rule that moves the cap witness reds the
/// pin below; one that changes which cells are dropped without moving
/// it reds those.
#[test]
fn a_planted_approach_to_the_notch_wall_is_still_violated() {
    let (doc, ell, block_node) = ell_with_a_planted_block();
    let cap = named(ell, vec![fixture::fname(ell, RoleSeg::Cap(CapEnd::Start))]);
    let block = Selection::body_of(block_node);
    let report = editor_core::clearance::clearance_with(
        &doc,
        &box_of(&doc, "place"),
        &cap,
        &block,
        &at_least(0.3, cfg(65_536, 40)),
    );
    println!(
        "[E4] planted block vs the L cap at c = 0.3: {}",
        report.serialize()
    );
    assert!(report.receipt().holds(), "{:?}", report.receipt());
    let ClearanceVerdict::Violated(v) = report.verdict() else {
        panic!(
            "the cap's material reaches within 0.05 of the block, so 0.3 is broken on the \
             FACES and not only on the window: {}",
            report.serialize()
        );
    };
    let d = witness_distance(&report);
    assert!(
        (d - v.geometry.distance).abs() <= 1e-12,
        "the report's distance is the distance between its own points: {d} vs {}",
        v.geometry.distance
    );
    // The lattice search returns a real pair of points, so its distance
    // is at least the true minimum and under the bound it violated.
    assert!(
        (0.05 - k_eps()..0.3).contains(&d),
        "the reported approach is the real one, not a phantom: {d}"
    );
    // The witness on the cap sits on the notch wall, which is the
    // material's own edge — not out in the notch where the window is,
    // and not pulled back off it either. The lattice station the
    // search lands on is `x = 0.375`: the cell corner one split inside
    // the wall, which is where the nine-point lattice of the cell that
    // survives the drop rule puts its nearest point. Pinned on both
    // sides at the funnel's own width, so a drop rule that moved the
    // boundary by a hair in EITHER direction reds this row.
    let p = v.geometry.a_point;
    assert!(
        p.x <= 0.4 + k_eps(),
        "the cap witness is on the face, at or behind the notch wall x = 0.4: {p:?}"
    );
    assert!(
        (p.x - 0.375).abs() <= k_eps(),
        "and it is the measured station x = 0.375, not merely somewhere behind the wall — \
         a drop rule tightened or loosened at this scale moves it: {p:?}"
    );
    println!(
        "[E4] witness {:?} -> {:?} d = {d}, decided {:?}",
        p,
        v.geometry.b_point,
        report.decided()
    );
}

// ------------------------------------------ E7: a band the wrong side of zero

/// A block with a semicircular SCALLOP cut into its top edge: profile
/// `(0,0) → (2,0) → (2,1) → (1.5,1) ⌒ (0.5,1) → (0,1)`, the arc
/// bulging DOWN through `(1, 0.5)`, extruded 1 along z.
///
/// The scallop's carrier is the cylinder centred `(1, 1)` of radius
/// 0.5, and the face runs the arc the NEGATIVE way round it: the walk
/// pins the branch from the first half-edge's principal azimuth and
/// the band comes out on the far side of zero, a real interval about
/// half a turn wide and not a residue mod `τ`. A probe block sits IN
/// the scallop, 0.12 above its lowest point.
///
/// An extrude, deliberately: no revolve on this tree replays at the
/// interval scalar over an ε-scaled box, so a revolved band never
/// reaches `window_of` to be asked the question.
fn scalloped_block() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    declare(&mut r, "place", 0.0);
    let chain = LoopProgram::Chain(vec![
        ProgramStep::At(len2([0.0, 0.0])),
        ProgramStep::LineTo(ProgramTarget::Point(len2([2.0, 0.0]))),
        ProgramStep::LineTo(ProgramTarget::Point(len2([2.0, 1.0]))),
        ProgramStep::LineTo(ProgramTarget::Point(len2([1.5, 1.0]))),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(len2([0.5, 1.0])),
            b: scl(-1.0),
        }),
        ProgramStep::LineTo(ProgramTarget::Point(len2([0.0, 1.0]))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let profile = r.insert(Node::Profile(ProfileProgram {
        loops: vec![chain],
        ids: Vec::new(),
        on: None,
    }));
    let solid = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: len(1.0),
    });
    // The probe: x ∈ [0.9, 1.1], y ∈ [0.62, 0.8] — 0.12 above the
    // scallop's lowest point (1, 0.5) — placed along z by the
    // parameter so the two bodies overlap in z.
    let probe = extruded(
        &mut r,
        &[(0.9, 0.62), (1.1, 0.62), (1.1, 0.8), (0.9, 0.8)],
        1.0,
    );
    let placed = translated(
        &mut r,
        Some(solid),
        probe,
        [
            Formula::zero(Dimension::Length),
            Formula::zero(Dimension::Length),
            Formula::named(name("place"), Dimension::Length),
        ],
    );
    (r.doc, solid, placed)
}

/// **E7 — a cylinder band answers on its whole-turn window.** What
/// this row pins is a `Violated` at the built 0.12, on the arc and not
/// on the coplanar caps, with every window's cells decided against its
/// face. The band is positive here (`[0, θ]`, the scallop minted with
/// `u_ref` at the arc's start);
/// [`a_negative_band_is_not_intersected_with_the_canonical_turn`] is
/// the row on a band that runs negative.
#[test]
fn a_cylinder_band_answers_on_its_whole_turn_window() {
    let (doc, solid, probe) = scalloped_block();
    // The SCALLOP alone on the first side — outer-loop segment 3, the
    // arc — so the witness this row reads is on the cylinder and not
    // on the coplanar z-caps, which approach each other at the same
    // 0.12 through the same void.
    let ss = named(
        solid,
        vec![fixture::fname(solid, fixture::wall(&doc, solid, 3))],
    );
    let sp = Selection::body_of(probe);
    let report = clearance(&doc, &box_of(&doc, "place"), &ss, &sp, 1.0, Tol::witness());
    println!(
        "[E7] scalloped block vs the probe in the scallop at c = 1.0: decided {:?}, {}",
        report.decided(),
        report.serialize()
    );
    assert!(report.receipt().holds(), "{:?}", report.receipt());
    if let ClearanceVerdict::Refused(ClearanceRefusal::Selection(s)) = report.verdict() {
        panic!("the scalloped block did not build at the interval scalar: {s}");
    }
    assert!(
        report.decided() > 0,
        "the faces' regions were read: some cell was decided against its face: {}",
        report.serialize()
    );
    let ClearanceVerdict::Violated(v) = report.verdict() else {
        panic!(
            "the probe stands 0.12 from the scallop, so a bound of 1.0 is broken: {}",
            report.serialize()
        );
    };
    let d = witness_distance(&report);
    assert!(
        (0.12 - k_eps()..1.0).contains(&d),
        "the reported approach is the built 0.12: {d}"
    );
    // The row asserted `a_chart_axis.is_none()` here — "the first side
    // is the cylinder, so its witness carries no planar re-chart".
    // PROPS's sign-hull unit retired the planar re-chart outright, so
    // NO witness carries one and the field itself is gone; the claim
    // that is left is the one below about `u` being an azimuth.
    println!(
        "[E7] witness uv = {:?} {:?} -> {:?} d = {d}",
        v.geometry.a_uv, v.geometry.a_point, v.geometry.b_point
    );
}

// ------------------------- E7b: a band that really does run negative

/// **A peg whose walls are arcs of ONE carrier, split `n` ways with the
/// first vertex at `phase`.** `CircleSplit` mints every wall on the
/// same cylinder, so each wall's loop is walked on its own branch,
/// pinned from its first half-edge's PRINCIPAL azimuth — a value in
/// `(-π, π]`. A wall whose arc starts past `π` therefore gets a
/// NEGATIVE band. The azimuth is the cylinder chart's, whose zero is
/// the loop's start vertex — the first vertex, at `phase` — so at
/// `n = 4`, `phase = -3π/4`, wall 3 (the arc that closes the loop)
/// measures `u ∈ [-π/2, 0]` straight out of `window_of`.
///
/// The construction is R2's, from the v6 dual's probe P6
/// (`t3s-r2` lane, `t3s_probes_interval.rs::p6_block_mid_way_along_the_negative_band_wall`),
/// adopted here with its measurement.
fn split_peg(r: &mut Recorder, n: u32, phase: f64) -> RecipeNodeId {
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::CircleSplit {
            centre: [len(0.0), len(0.0)],
            radius: len(0.5),
            n,
            phase: ang(phase),
        }],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    })
}

/// A 0.1 × 0.1 block whose near face stands `gap` off the peg's surface
/// at world azimuth `theta`, spanning `z ∈ [0.3, 0.7]` so it sits
/// mid-way up the wall rather than against either cap.
fn block_at_azimuth(r: &mut Recorder, peg: RecipeNodeId, theta: f64, gap: f64) -> RecipeNodeId {
    let (c, s) = (theta.cos(), theta.sin());
    let (r0, r1) = (0.5 + gap, 0.5 + gap + 0.1);
    let (tx, ty) = (-s * 0.05, c * 0.05);
    let pts = [
        (r0 * c - tx, r0 * s - ty),
        (r1 * c - tx, r1 * s - ty),
        (r1 * c + tx, r1 * s + ty),
        (r0 * c + tx, r0 * s + ty),
    ];
    let b = extruded(r, &pts, 0.4);
    translated(
        r,
        Some(peg),
        b,
        [
            Formula::zero(Dimension::Length),
            Formula::zero(Dimension::Length),
            Formula::add(len(0.3), Formula::named(name("place"), Dimension::Length))
                .expect("a length"),
        ],
    )
}

/// **E7b — a band that runs the wrong side of zero.**
///
/// Wall 3 of a four-way split peg phased at `-3π/4` has the band
/// `[-π/2, 0]` on its loop walk's branch: the circle's chart azimuth is
/// measured from the loop's start vertex, here at `-3π/4`, and wall 3
/// is the arc that closes the loop back onto it. The block stands 0.1
/// off that wall at world azimuth `+7π/8` — mid-band, because the
/// wall's own material runs from `+3π/4` round through `π` to `-3π/4`
/// in world terms. At `c = 0.3` the approach is real and the answer is
/// `Violated`.
///
/// The window is the canonical whole turn, and the wall's cells are
/// decided off its edges in model space, which read no branch: the
/// witness's chart `u` is the band's own azimuth mod `τ`. A drop rule
/// that read the band on its branch and folded it wrongly would drop
/// the wall's material and answer a phantom `Holds` here.
#[test]
fn a_negative_band_is_not_intersected_with_the_canonical_turn() {
    let mut r = Recorder::new();
    declare(&mut r, "place", 0.0);
    let peg = split_peg(&mut r, 4, -3.0 * core::f64::consts::FRAC_PI_4);
    let block = block_at_azimuth(&mut r, peg, 7.0 * core::f64::consts::FRAC_PI_8, 0.1);
    let wall = named(
        peg,
        vec![fixture::fname(peg, fixture::wall(&r.doc, peg, 3))],
    );
    let report = clearance(
        &r.doc,
        &box_of(&r.doc, "place"),
        &wall,
        &Selection::body_of(block),
        0.3,
        Tol::witness(),
    );
    println!(
        "[E7b] split peg wall 3 vs a block at +7π/8, c = 0.3: decided {:?}, {}",
        report.decided(),
        report.serialize()
    );
    assert!(report.receipt().holds(), "{:?}", report.receipt());
    if let ClearanceVerdict::Refused(ClearanceRefusal::Selection(s)) = report.verdict() {
        panic!("the split peg did not build at the interval scalar: {s}");
    }
    assert!(
        report.decided() > 0,
        "the faces' regions were read: some cell was decided against its face: {}",
        report.serialize()
    );
    let ClearanceVerdict::Violated(v) = report.verdict() else {
        panic!(
            "the block stands 0.1 off this wall's material; a `Holds` here is the band \
             intersected away to a sliver at the origin: {}",
            report.serialize()
        );
    };
    let d = witness_distance(&report);
    assert!(
        (0.1 - k_eps()..0.3).contains(&d),
        "the reported approach is the built 0.1, found on the wall's own lattice: {d}"
    );
    let tau = core::f64::consts::TAU;
    assert!(
        (3.0 * core::f64::consts::FRAC_PI_2..=tau).contains(&v.geometry.a_uv.0.rem_euclid(tau)),
        "and the witness lies on the wall's band `[-π/2, 0]`, mod τ: {:?}",
        v.geometry.a_uv
    );
    println!(
        "[E7b] witness uv = {:?} {:?} -> {:?} d = {d}",
        v.geometry.a_uv, v.geometry.a_point, v.geometry.b_point
    );
}

// ------------------------------------- E8: a revolved band, decided cell by cell

/// **E8 — a quarter annulus revolved about ŷ, beside a block where
/// the other three quarters of its outer cylinder would be.** The
/// true clearance is above 2; the outer wall's whole-turn window comes
/// within 0.1 of the block, and its cells there are decided out of the
/// wall, so the query holds at `c = 1`.
#[test]
fn a_revolved_quarter_band_is_windowed_and_holds() {
    let mut r = Recorder::new();
    declare(&mut r, "place", 0.0);
    let profile = r.insert(Node::Profile(fixture::desc(vec![vec![
        (1.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
    ]])));
    // The axis line, in the profile's own coordinates: its v is the
    // world axis the quarter turns about.
    let quarter = r.insert(fixture::revolve_about(
        profile,
        (0.0, 0.0),
        crate::fixture::along_y(),
        ang(core::f64::consts::FRAC_PI_2),
    ));
    let block = extruded(
        &mut r,
        &[(-2.5, 0.0), (-2.1, 0.0), (-2.1, 1.0), (-2.5, 1.0)],
        0.4,
    );
    // A partial revolve offers no frame a copy is placed against, so
    // both stand in the world.
    let quarter = r.place(quarter);
    let placed = translated(
        &mut r,
        None,
        block,
        [
            Formula::zero(Dimension::Length),
            Formula::zero(Dimension::Length),
            Formula::add(len(-0.2), Formula::named(name("place"), Dimension::Length))
                .expect("a length"),
        ],
    );
    let (sq, sb) = (Selection::body_of(quarter), Selection::body_of(placed));
    let report = clearance(
        &r.doc,
        &box_of(&r.doc, "place"),
        &sq,
        &sb,
        1.0,
        Tol::witness(),
    );
    println!(
        "[E8] y-axis quarter annulus, verdict {} decided {:?}: {}",
        report.verdict().label(),
        report.decided(),
        report.serialize()
    );
    assert!(report.receipt().holds(), "{:?}", report.receipt());
    assert_eq!(
        report.verdict(),
        &ClearanceVerdict::Holds,
        "the block clears the quarter annulus by more than the window comes within: {}",
        report.serialize()
    );
    assert!(
        report.decided() > 0,
        "the revolved band's region was read: some cell was decided against its face: {}",
        report.serialize()
    );
}

// ----------------------------------- a quarter ball, against a dense oracle

/// The least distance from `p` to the axis-aligned box `lo..hi`.
fn to_box(p: [f64; 3], lo: [f64; 3], hi: [f64; 3]) -> f64 {
    (0..3)
        .map(|k| (lo[k] - p[k]).max(p[k] - hi[k]).max(0.0).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// **A quarter ball beside a block parked where its other three
/// quarters would be.** The unit ball's profile, a half disc on the
/// axis, revolved a quarter turn about ŷ: material on `x ≥ 0, z ≤ 0`.
/// The block stands on `x ∈ [-1.5, -1.1]`, so the sphere's whole-carrier
/// window comes within 0.1 of it while the quarter's own nearest point
/// is its cap on `x = 0`, 1.1 away.
///
/// The oracle is a dense model-space distance: the quarter's sphere and
/// both caps sampled on fine lattices, each sample's distance to the
/// block read in closed form. A `Holds` at `c` claims `c` is below the
/// least of them; a `Violated` witness is a pair of face points, so its
/// distance is not below it (to the lattice's own spacing). At
/// `c = 0.5` the query holds, which it cannot without dropping the
/// sphere's cells in the missing quarters; at `c = 1.2` it is violated,
/// at the cap, not at the phantom sphere.
#[test]
fn a_quarter_ball_beside_its_missing_quarters_holds_against_a_dense_distance() {
    let mut r = Recorder::new();
    declare(&mut r, "place", 0.0);
    let profile = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::Chain(vec![
            ProgramStep::At(len2([0.0, -1.0])),
            ProgramStep::ArcTo(ProgramArcData::Bulge {
                target: ProgramTarget::Point(len2([0.0, 1.0])),
                b: scl(1.0),
            }),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])],
        ids: Vec::new(),
        on: None,
    }));
    let quarter = r.insert(fixture::revolve_about(
        profile,
        (0.0, 0.0),
        crate::fixture::along_y(),
        ang(core::f64::consts::FRAC_PI_2),
    ));
    let block = extruded(
        &mut r,
        &[(-1.5, 0.0), (-1.1, 0.0), (-1.1, 1.0), (-1.5, 1.0)],
        0.4,
    );
    let quarter = r.place(quarter);
    let placed = translated(
        &mut r,
        None,
        block,
        [
            Formula::zero(Dimension::Length),
            Formula::zero(Dimension::Length),
            Formula::add(len(-0.2), Formula::named(name("place"), Dimension::Length))
                .expect("a length"),
        ],
    );
    let (lo, hi) = ([-1.5, 0.0, -0.2], [-1.1, 1.0, 0.2]);
    let n = 400;
    let mut dense = f64::INFINITY;
    for i in 0..=n {
        for j in 0..=n {
            let (a, b) = (f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
            let phi = a * core::f64::consts::FRAC_PI_2;
            let lat = (b - 0.5) * core::f64::consts::PI;
            let sphere = [lat.cos() * phi.cos(), lat.sin(), -lat.cos() * phi.sin()];
            let (rho, y) = (a, 2.0 * b - 1.0);
            let cap0 = [rho, y, 0.0];
            let cap1 = [0.0, y, -rho];
            for p in [sphere, cap0, cap1] {
                if (p[0].powi(2) + p[1].powi(2) + p[2].powi(2)) <= 1.0 + 1e-12 {
                    dense = dense.min(to_box(p, lo, hi));
                }
            }
        }
    }
    let spacing = core::f64::consts::PI / f64::from(n);
    assert!(
        (dense - 1.1).abs() < 1e-9,
        "the oracle's least distance is the cap's 1.1: {dense}"
    );
    let (sq, sb) = (Selection::body_of(quarter), Selection::body_of(placed));
    let at = |c: f64| {
        clearance(
            &r.doc,
            &box_of(&r.doc, "place"),
            &sq,
            &sb,
            c,
            Tol::witness(),
        )
    };
    let holds = at(0.5);
    println!("[ball] c = 0.5: {}", holds.serialize());
    assert!(holds.receipt().holds(), "{:?}", holds.receipt());
    assert_eq!(
        (holds.verdict(), holds.receipt().outside > 0),
        (&ClearanceVerdict::Holds, true),
        "the quarter ball clears the block by the dense oracle's {dense}, and the sphere's \
         cells in the missing quarters drop: {}",
        holds.serialize()
    );
    let violated = at(1.2);
    println!("[ball] c = 1.2: {}", violated.serialize());
    let d = witness_distance(&violated);
    assert!(
        d >= dense - spacing && d < 1.2,
        "the witness is a pair of face points, no nearer than the dense oracle allows: {d}"
    );
}

// ------------------------------------------- a holed cylinder wall

/// **A cylinder wall with a hole keeps its drops.** A unit cylinder,
/// two tall, with a box cut through its wall at `+x`
/// (`x ∈ [0.8, 1.5]`, `y ∈ [-0.3, 0.3]`, `z ∈ [0.7, 1.3]`), so its
/// lateral face carries a ring. A probe stands in the cut, on the
/// carrier: `x ∈ [0.9, 1.1]` meets the cylinder `r = 1` where the wall
/// has no material. The cut body's nearest material is the cut's own
/// floor at `x = 0.8`, 0.1 off; at `c = 0.05` the query holds only if
/// the lateral face's cells in the hole drop.
#[test]
fn a_holed_cylinder_wall_drops_the_cells_in_its_hole() {
    let mut r = Recorder::new();
    declare(&mut r, "place", 0.0);
    let disc = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::circle(0.0, 0.0, 1.0).expect("a literal circle")],
        ids: Vec::new(),
        on: None,
    }));
    let cylinder = r.insert(Node::Extrude {
        profile: disc.into(),
        distance: len(2.0),
    });
    let tool = extruded(
        &mut r,
        &[(0.0, 0.0), (0.7, 0.0), (0.7, 0.6), (0.0, 0.6)],
        0.6,
    );
    // Placed against the cylinder, whose slides read along the world's
    // axes: the tool spans `y ∈ [−0.3, 0.3]`, across the probe.
    let tool = translated(
        &mut r,
        Some(cylinder),
        tool,
        [len(0.8), len(-0.3), len(0.7)],
    );
    let cut = r.insert(Node::Subtract {
        from: cylinder.into(),
        tool: tool.into(),
    });
    let cut = r.place(cut);
    let probe = extruded(
        &mut r,
        &[(0.0, 0.0), (0.2, 0.0), (0.2, 0.2), (0.0, 0.2)],
        0.2,
    );
    let probe = translated(
        &mut r,
        None,
        probe,
        [
            len(0.9),
            len(-0.1),
            Formula::add(len(0.9), Formula::named(name("place"), Dimension::Length))
                .expect("a length"),
        ],
    );
    let report = clearance(
        &r.doc,
        &box_of(&r.doc, "place"),
        &Selection::body_of(cut),
        &Selection::body_of(probe),
        0.05,
        Tol::witness(),
    );
    println!("[holed wall] c = 0.05: {}", report.serialize());
    if let ClearanceVerdict::Refused(ClearanceRefusal::Selection(s)) = report.verdict() {
        panic!("the holed cylinder did not build at the interval scalar: {s}");
    }
    assert!(report.receipt().holds(), "{:?}", report.receipt());
    assert_eq!(
        (report.verdict(), report.receipt().outside > 0),
        (&ClearanceVerdict::Holds, true),
        "the probe in the hole clears the cut body by 0.1, and the wall's cells in the hole \
         drop: {}",
        report.serialize()
    );
}

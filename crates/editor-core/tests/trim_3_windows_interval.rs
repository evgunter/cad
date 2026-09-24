//! **TRIM-3 PR-2: the clearance window cut to the face's chart
//! boundary**, from outside, through the public doors only.
//!
//! `docs/TRIM-3-SPEC.md` §4 lists nine e2e rows. Six of them are
//! FLIPS of rows that already pin the old behaviour, and they stay
//! where they are — a pinned counterexample is worth most when the fix
//! turns it red in the file that made the claim:
//!
//! | row | where |
//! | --- | --- |
//! | E1 L cap vs block in the notch | `m10_5_r2_probes_interval` |
//! | E2 L plate vs floating block | `m10_5_r1_probes_interval` |
//! | E3 U-channel whole body | `m10_5_r1_probes_interval` |
//! | E5 bumped block, strictly positive | `m10_5_r1_probes_interval` |
//! | E6 z-axis quarter annulus | `m10_5_r1_probes_interval` |
//! | E9 the notch bracket | `m10_6_r1_probes_interval` |
//!
//! What is here is the three rows with no existing home: **E4**, the
//! planted-tight approach that says the tightening is not a blanket
//! `Holds`; **E7**, a cylinder band answering through a cut root; and
//! **E8**, the identity a refused description owes.
//!
//! E7 is cut on an EXTRUDED scallop rather than the spec's
//! negative-angle revolve, and E6 and E8 record a skip rather than an
//! assertion, for one measured reason: no revolve on this tree replays
//! at the interval scalar over an ε-scaled box, so a revolved band
//! refuses at the SELECTION door and never reaches `window_of`.
//!
//! The basename carries `interval` because the suite is
//! `#![cfg(feature = "interval")]`, which is what selects it into the
//! interval legs.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;

use editor_core::UnitSym;
use editor_core::analysis::{BoxAxis, ParamBox};
use editor_core::clearance::{
    ClearanceBound, ClearanceConfig, ClearanceQuery, ClearanceRefusal, ClearanceReport,
    ClearanceVerdict, FaceScope, NoTangents, Pruning, Selection, clearance,
};
use editor_core::{
    CapEnd, Datum, Dimension, Distribution, DocEdit, DocParam, Expr, LoopProgram, Node, ParamName,
    ProfileDoc, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, RoleSeg,
};
use geom_core::Tol;

use fixture::{Recorder, ang, len, scl};

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

fn name(n: &str) -> ParamName {
    ParamName::new(n)
}

fn box_of(axis: &str) -> ParamBox {
    let mut axes = BTreeMap::new();
    axes.insert(
        name(axis),
        BoxAxis::Varying {
            lo: -half(),
            hi: half(),
        },
    );
    ParamBox::from_axes(axes)
}

fn declare(r: &mut Recorder, axis: &str, nominal: f64) {
    r.push(DocEdit::SetDocParam {
        name: name(axis),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: nominal,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -half(),
                hi: half(),
            }),
        },
    });
}

fn translated(input: RecipeNodeId, d: [Expr; 3]) -> Node<ProfileProgram> {
    let [dx, dy, dz] = d;
    Node::Transform {
        input,
        translation: [dx, dy, dz],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite angle"),
    }
}

fn xy_frame(r: &mut Recorder) -> RecipeNodeId {
    r.insert(fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]))
}

fn extruded(r: &mut Recorder, points: &[(f64, f64)], depth: f64) -> RecipeNodeId {
    let plane = xy_frame(r);
    let p = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::polygon(points.iter().copied()).expect("finite corners")],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: len(depth),
    })
}

fn cfg(pairs: usize, depth: u32) -> ClearanceConfig {
    ClearanceConfig {
        max_cell_depth: depth,
        max_cell_pairs: pairs,
        pruning: Pruning::Off,
    }
}

fn at_least(c: f64, config: ClearanceConfig) -> ClearanceQuery<'static> {
    ClearanceQuery {
        bound: ClearanceBound::AtLeast(c),
        tol: Tol::witness(),
        config,
        oracle: &NoTangents,
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
    let placed = r.insert(translated(
        probe,
        [
            Expr::param(name("place"), Dimension::Length),
            len(0.0),
            len(-0.05),
        ],
    ));
    (r.doc, solid, placed)
}

/// **E4 — the tightening is not a blanket `Holds`.** The same L cap
/// whose window covers the notch, and a block the cap's MATERIAL
/// really does come within 0.05 of: the nearest face point is on the
/// notch wall at `x = 0.4`, the block starts at `x = 0.45`, and the
/// block straddles the cap's plane, so the true face-to-face distance
/// is 0.05. A bound of 0.3 is violated, and it must still be reported.
///
/// **What this row kills, measured.** An inverted parity: the
/// description would certify the MATERIAL outside, drop the cells that
/// hold the real approach, and leave `Holds`. That is the one mutant
/// the v6 dual reproduced here, on both arms.
///
/// **What it does NOT kill, also measured**, and stated because the
/// PR that landed it claimed otherwise. *Drop-on-indeterminate* — a
/// separating-axis test that lets `Zero`, an in-band margin or poison
/// count as separating — leaves this row green; so does *a
/// description tight by `≥ 0.05`* (cells eroded by that much). The
/// one-sided assertion that used to be credited with catching the
/// second was monotone the wrong way, since a tighter description
/// pushes the cap witness to SMALLER x, and the two-sided pin below
/// replaces it — but the honest result of re-running both mutants
/// against it is that **this row still does not catch either**.
///
/// What catches them, measured in the same pass: `topo`'s own T4/T6/T10
/// rows on hand-built descriptions, and — e2e, newly — the pinned
/// receipts on
/// `m10_5_r1_probes_interval::an_l_shaped_face_holds_where_it_has_no_material`
/// and `::e2e_channel_slider_over_an_epsilon_box`, whose discharge and
/// split counts move when the drop rule changes at all. A description
/// that moves the cap witness still reds the pin below; a description
/// that changes which cells are dropped without moving it reds those
/// two.
#[test]
fn a_planted_approach_to_the_notch_wall_is_still_violated() {
    let (doc, ell, block_node) = ell_with_a_planted_block();
    let cap = named(ell, vec![fixture::fname(ell, RoleSeg::Cap(CapEnd::Start))]);
    let block = Selection::body_of(block_node);
    let report = editor_core::clearance::clearance_with(
        &doc,
        &box_of("place"),
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
    // survives the description puts its nearest point. Pinned on both
    // sides at the funnel's own width, so a description that moved the
    // boundary by a hair in EITHER direction reds this row.
    let p = v.geometry.a_point;
    assert!(
        p.x <= 0.4 + k_eps(),
        "the cap witness is on the face, at or behind the notch wall x = 0.4: {p:?}"
    );
    assert!(
        (p.x - 0.375).abs() <= k_eps(),
        "and it is the measured station x = 0.375, not merely somewhere behind the wall — \
         a description tightened or loosened at this scale moves it: {p:?}"
    );
    println!(
        "[E4] witness {:?} -> {:?} d = {d}, windows {:?}",
        p,
        v.geometry.b_point,
        report.windows()
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
    let p2 = |x: f64, y: f64| [len(x), len(y)];
    let chain = LoopProgram::Chain(vec![
        ProgramStep::At(p2(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(p2(2.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(p2(2.0, 1.0))),
        ProgramStep::LineTo(ProgramTarget::Point(p2(1.5, 1.0))),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(p2(0.5, 1.0)),
            b: scl(-1.0),
        }),
        ProgramStep::LineTo(ProgramTarget::Point(p2(0.0, 1.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let plane = xy_frame(&mut r);
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![chain],
    }));
    let solid = r.insert(Node::Extrude {
        profile,
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
    let placed = r.insert(translated(
        probe,
        [
            len(0.0),
            len(0.0),
            Expr::param(name("place"), Dimension::Length),
        ],
    ));
    (r.doc, solid, placed)
}

/// **E7 — the periodic root rule, as far as this tree can carry it.**
/// A cylinder's window `u` is an azimuth on the walk's own branch, and
/// the root is cut to the description's hull VERBATIM when that hull
/// spans no more than a turn. What this row pins is that a cylinder
/// face answers THROUGH such a cut root: a `Violated` at the built
/// 0.12, on the arc and not on the coplanar caps, with every window in
/// the query described.
///
/// **What it does not kill is the `[0, τ] ∩ hull` mutant**, because
/// this fixture's band is positive: the scallop is minted with `u_ref`
/// at the arc's start, so the band is `[0, θ]` (measured: `u = π/2` at
/// the scallop's lowest point, `v = -1`, so the axis is `-ẑ` and the
/// band is the positive half turn) and the intersection is the
/// identity on it. The row below,
/// [`a_negative_band_is_not_intersected_with_the_canonical_turn`],
/// is the one that kills it, on a band that does run negative.
#[test]
fn a_cylinder_band_answers_through_a_cut_root() {
    let (doc, solid, probe) = scalloped_block();
    // The SCALLOP alone on the first side — outer-loop segment 3, the
    // arc — so the witness this row reads is on the cylinder and not
    // on the coplanar z-caps, which approach each other at the same
    // 0.12 through the same void.
    let ss = named(solid, vec![fixture::fname(solid, fixture::wall(3))]);
    let sp = Selection::body_of(probe);
    let report = clearance(&doc, &box_of("place"), &ss, &sp, 1.0, Tol::witness());
    println!(
        "[E7] scalloped block vs the probe in the scallop at c = 1.0: windows {:?}, {}",
        report.windows(),
        report.serialize()
    );
    assert!(report.receipt().holds(), "{:?}", report.receipt());
    if let ClearanceVerdict::Refused(ClearanceRefusal::Selection(s)) = report.verdict() {
        panic!("the scalloped block did not build at the interval scalar: {s}");
    }
    assert_eq!(
        report.windows().1,
        0,
        "every carrier here is a plane or a cylinder and every one of them describes — a \
         band emptied by an intersection mod τ would have refused at the door: {}",
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
    assert!(
        v.geometry.a_chart_axis.is_none(),
        "the first side is the cylinder, so its witness carries no planar re-chart and its \
         `u` is an azimuth: {:?}",
        v.geometry.a_chart_axis
    );
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
    let plane = xy_frame(r);
    let p = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::CircleSplit {
            centre: [len(0.0), len(0.0)],
            radius: len(0.5),
            n,
            phase: ang(phase),
        }],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    })
}

/// A 0.1 × 0.1 block whose near face stands `gap` off the peg's surface
/// at world azimuth `theta`, spanning `z ∈ [0.3, 0.7]` so it sits
/// mid-way up the wall rather than against either cap.
fn block_at_azimuth(r: &mut Recorder, theta: f64, gap: f64) -> RecipeNodeId {
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
    r.insert(translated(
        b,
        [
            len(0.0),
            len(0.0),
            Expr::add(len(0.3), Expr::param(name("place"), Dimension::Length)).expect("a length"),
        ],
    ))
}

/// **E7b — the periodic root rule, on a band that runs the wrong side
/// of zero.** This is the row the spec's E7 wanted and the row TRIM-3
/// PR-2 first filed as unreachable.
///
/// Wall 3 of a four-way split peg phased at `-3π/4` has the band
/// `[-π/2, 0]`: the circle's chart azimuth is measured from the loop's
/// start vertex, here at `-3π/4`, and wall 3 is the arc that closes
/// the loop back onto it. The block stands 0.1 off that wall at world
/// azimuth `+7π/8` — mid-band, because the wall's own material runs
/// from `+3π/4` round through `π` to `-3π/4` in world terms while its
/// chart azimuth runs `-π/2` to `0`. (The same peg phased at `-π/4`,
/// the row's first spelling, reached this wall as wall 2 only while
/// validation moved every loop's start to its lex-min vertex; the
/// start is the authored one now, so the phase says where the chart
/// starts.) At `c = 0.3` the approach is real and
/// the answer is `Violated`, with a witness whose chart `u` is
/// NEGATIVE: the band was never folded.
///
/// **The two mistakes this forbids, and what each leaves behind.**
/// Both spellings of "intersect the band with the canonical turn" —
/// `narrowed(full_turn(), hull)` and the bare
/// `(0.0.max(hu.0), τ.min(hu.1))` — leave the sliver
/// `[-5e-324, 3.5e-14]`, because the hull's outward rounding puts the
/// band's top end a hair ABOVE zero and the non-overlap fallback
/// therefore never fires. The sweep then subdivides a sliver that
/// holds none of the wall, every cell of it is far from the block, and
/// the query answers **`Holds`** — a phantom certificate over a face
/// whose material is 0.1 from the block. Planted, both spellings, both
/// red this row; `clearance::root_rule` decides the same rule directly.
#[test]
fn a_negative_band_is_not_intersected_with_the_canonical_turn() {
    let mut r = Recorder::new();
    declare(&mut r, "place", 0.0);
    let peg = split_peg(&mut r, 4, -3.0 * core::f64::consts::FRAC_PI_4);
    let block = block_at_azimuth(&mut r, 7.0 * core::f64::consts::FRAC_PI_8, 0.1);
    let wall = named(peg, vec![fixture::fname(peg, fixture::wall(3))]);
    let report = clearance(
        &r.doc,
        &box_of("place"),
        &wall,
        &Selection::body_of(block),
        0.3,
        Tol::witness(),
    );
    println!(
        "[E7b] split peg wall 3 vs a block at +7π/8, c = 0.3: windows {:?}, {}",
        report.windows(),
        report.serialize()
    );
    assert!(report.receipt().holds(), "{:?}", report.receipt());
    if let ClearanceVerdict::Refused(ClearanceRefusal::Selection(s)) = report.verdict() {
        panic!("the split peg did not build at the interval scalar: {s}");
    }
    assert_eq!(
        report.windows().1,
        0,
        "the wall and the block's faces are all planes and cylinders, so every window \
         describes: {}",
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
    assert!(
        v.geometry.a_chart_axis.is_none(),
        "the first side is the cylinder, so its `u` is an azimuth: {:?}",
        v.geometry.a_chart_axis
    );
    assert!(
        v.geometry.a_uv.0 < 0.0,
        "and the azimuth is on the walk's own branch, the wrong side of zero — a root \
         intersected with `[0, τ]` could not carry this witness: {:?}",
        v.geometry.a_uv
    );
    println!(
        "[E7b] witness uv = {:?} {:?} -> {:?} d = {d}",
        v.geometry.a_uv, v.geometry.a_point, v.geometry.b_point
    );
}

// -------------------------------------------- E8: a refusal is the identity

/// **E8 — the query that never reaches a description at all, and the
/// coverage boundary that is.**
///
/// The spec wrote this row as "a description that refuses is the
/// identity", on the M10-5 R1 y-axis quarter annulus: revolved about
/// ŷ, whose stored `u_ref` is the two-sided sign hull, and which the
/// engine re-charts not at all because it re-charts PLANES only. The
/// row does not certify that, and the honest thing is to say what it
/// does certify instead.
///
/// **Measured**: the revolve does not replay at the interval scalar
/// over an ε-scaled box, so this query refuses at the SELECTION door
/// (`node did not build in this leaf's replay`) and
/// `ClearanceReport::refused` mints `windows = (0, 0)` and a default
/// receipt **before `window_of` runs**. Both numbers below are that
/// constant. They pin something real — a refusal at the door reports
/// no windows, which is what a reader of `windows()` must be able to
/// rely on — and they are not the identity claim.
///
/// **The identity claim is unexercised**, and cannot be exercised on
/// this tree: no fixture reaches `chart_boundary`'s `Err` arm from
/// `window_of`, because the three ways it refuses are a loop that
/// wraps a period (no constructor makes one), a singular joint (whose
/// carriers `chart_arms` declines before the walk is called) and a
/// `General` carrier (which `EdgeCurve::certify` refuses at build
/// time). Filed as
/// `work/trim/a-refused-chart-boundary-has-no-reachable-window.md`;
/// `work/trim/revolved-bands-reach-no-clearance-row.md` is why this
/// fixture in particular stops at the door. A mutant that turns the
/// `Err` arm into a `ClearanceRefusal::Unsupported` survives the whole
/// suite, and no row in this file claims otherwise.
#[test]
fn a_selection_door_refusal_reports_no_windows_at_all() {
    let mut r = Recorder::new();
    declare(&mut r, "place", 0.0);
    let plane = xy_frame(&mut r);
    let profile = r.insert(Node::Profile(fixture::desc(
        plane,
        vec![vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]],
    )));
    let axis = r.insert(Node::Datum(Datum::Axis {
        origin: [len(0.0), len(0.0), len(0.0)],
        direction: [scl(0.0), scl(1.0), scl(0.0)],
    }));
    let quarter = r.insert(Node::Revolve {
        profile,
        axis,
        angle: ang(core::f64::consts::FRAC_PI_2),
    });
    let block = extruded(
        &mut r,
        &[(-2.5, 0.0), (-2.1, 0.0), (-2.1, 1.0), (-2.5, 1.0)],
        0.4,
    );
    let placed = r.insert(translated(
        block,
        [
            len(0.0),
            len(0.0),
            Expr::add(len(-0.2), Expr::param(name("place"), Dimension::Length)).expect("a length"),
        ],
    ));
    let (sq, sb) = (Selection::body_of(quarter), Selection::body_of(placed));
    let report = clearance(&r.doc, &box_of("place"), &sq, &sb, 1.0, Tol::witness());
    println!(
        "[E8] y-axis quarter annulus, verdict {} windows {:?}: {}",
        report.verdict().label(),
        report.windows(),
        report.serialize()
    );
    assert!(report.receipt().holds(), "{:?}", report.receipt());
    let ClearanceVerdict::Refused(ClearanceRefusal::Selection(_)) = report.verdict() else {
        panic!(
            "this row's subject is the SELECTION door's refusal; if the revolve has started \
             replaying at the interval scalar this fixture now reaches `window_of` and the \
             row must be rewritten to say what it finds there: {}",
            report.serialize()
        );
    };
    assert_eq!(
        report.windows(),
        (0, 0),
        "a refusal at the door reports no windows in either column — it did not look at \
         one: {}",
        report.serialize()
    );
    assert_eq!(
        report.receipt(),
        editor_core::clearance::CellReceipt::default(),
        "and nothing was classified: {}",
        report.serialize()
    );
}

//! R1 reviewer probes for M10-5 (PR #1638, frozen head 9f143595): the
//! E7 clearance engine attacked through its public doors only.
//!
//! Every row below is either a PINNED counterexample (a behaviour the
//! spec or the module's own door-prose contradicts, written so the row
//! goes red when the behaviour is fixed — delete the row with the fix)
//! or EVIDENCE (a measurement the review reads; asserts only the
//! receipt identity and the arm label). Each row says which in its
//! doc-comment. Nothing here fixes a seed: there is no fuzzing in this
//! file, every fixture is written out.
//!
//! The helpers at the top re-derive the shipped suite's fixtures
//! (`m10_5_clearance_interval.rs`: `half`, `box_of`, `declare`,
//! `extruded`) rather than sharing them — a probe file must not depend
//! on the suite it is probing. Same reasons for the ε-scaled box: no
//! node's interval replay builds over a wider one (issue 1191's class).
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::BTreeMap;

use bvh::Aabb;
use editor_core::UnitSym;
use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::clearance::{
    CellBudget, ClearanceBound, ClearanceConfig, ClearanceQuery, ClearanceRefusal, ClearanceReport,
    ClearanceVerdict, FaceScope, NoTangents, Selection, clearance, clearance_over,
    clearance_with, self_intersection,
};
use editor_core::drive::{DriveConfig, drive};
use editor_core::{
    CapEnd, Datum, Dimension, Distribution, DocEdit, DocParam, Expr, LoopProgram, Node, ParamName,
    ProfileDoc, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, RoleSeg,
};
use geom_core::{Bounds, Interval, Tol, Vec3};

use fixture::{Recorder, ang, len, scl};

/// The analysis box's half-width: the shipped suite's ε/64, for the
/// shipped suite's reason (module header).
fn half() -> f64 {
    Tol::witness().eps() / 64.0
}

fn eps() -> f64 {
    Tol::witness().eps()
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

fn declare_with(r: &mut Recorder, axis: &str, nominal: f64, hw: f64) {
    r.push(DocEdit::SetDocParam {
        name: name(axis),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: nominal,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform { lo: -hw, hi: hw }),
        },
    });
}

fn declare(r: &mut Recorder, axis: &str, nominal: f64) {
    declare_with(r, axis, nominal, half());
}

fn translated(input: RecipeNodeId, by: [Expr; 3]) -> Node<ProfileProgram> {
    Node::Transform {
        input,
        translation: by,
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite angle"),
    }
}

/// The xy sketch frame, as the `Datum::Frame` node a profile now names.
///
/// `ProfileProgram::plane` became a node reference under this branch
/// (main's move), so every fixture mints the frame first and hands the
/// profile its id.
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

fn param(n: &str) -> Expr {
    Expr::param(name(n), Dimension::Length)
}

/// Two unit blocks whose facing walls stand `gap` apart, the gap a
/// document parameter (the shipped suite's `facing_blocks`, re-derived).
fn blocks(gap: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    declare(&mut r, "gap", gap);
    let a = extruded(
        &mut r,
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        1.0,
    );
    let by = Expr::add(len(1.0), param("gap")).expect("a length");
    let b = r.insert(translated(a, [by, len(0.0), len(0.0)]));
    (r.doc, a, b)
}

fn recomputed_distance(report: &ClearanceReport) -> f64 {
    let ClearanceVerdict::Violated(v) = report.verdict() else {
        panic!("expected a violation, got {}", report.serialize());
    };
    let (a, b) = (v.geometry.a_point, v.geometry.b_point);
    let (dx, dy, dz) = (a.x - b.x, a.y - b.y, a.z - b.z);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn query(bound: ClearanceBound, config: ClearanceConfig) -> ClearanceQuery<'static> {
    ClearanceQuery {
        bound,
        tol: Tol::witness(),
        config,
        oracle: &NoTangents,
    }
}

// ------------------------------------------------ claim 2 / deviation D4

/// **PINNED COUNTEREXAMPLE (D4).** The interval BVH prunes on a RAW
/// compare `separation_lo > c`, and the funnel classifies through the
/// band. Inside the band the two disagree, and the pruned answer wins.
///
/// Under `SELF_INTERSECTION_GAP` the module's own prose (and ledger row
/// F17) says a definite `Sign::Zero` — two faces coincident at the
/// run's tolerance, `d ≤ ε` — IS the violation the check exists to
/// find. Two blocks `ε/2` apart are coincident at tolerance by that
/// reading; the tree's raw compare sees `separation_lo ≈ ε/2 − ε/32 >
/// 0 = c`, prunes the only pair, and the engine certifies `Holds` with
/// zero candidates. The funnel never sees the pair.
///
/// Goes red when the prune is padded by the band (or funnelled). The
/// second arm is the same shape under `CLEARANCE_MARGIN`: a gap of
/// `c + 5ε` is a margin wholly inside `(ε, Kε)`, which the driver's
/// own `sliver` rule (the one this engine says it shares) refuses as a
/// terminal sliver — pruned, it is a silent `Holds`.
#[test]
fn the_bvh_prune_decides_inside_the_band_where_the_funnel_would_not() {
    // Arm 1: strictly positive, gap = ε/2 (≤ ε: `Sign::Zero` at the
    // funnel). The blocks' boxes are widened by ±ε/64 by the leaf box,
    // so the tree's separation is ~ε/2 − ε/32, still > 0.
    let (doc, a, b) = blocks(eps() / 2.0);
    let (sa, sb) = (Selection::body_of(a), Selection::body_of(b));
    let strict = query(ClearanceBound::StrictlyPositive, ClearanceConfig::default());
    let report = clearance_with(&doc, &box_of("gap"), &sa, &sb, &strict);
    let r = report.receipt();
    assert!(r.holds(), "{r:?}");
    assert_eq!(
        report.verdict(),
        &ClearanceVerdict::Holds,
        "the pruned answer today is Holds: {}",
        report.serialize()
    );
    assert_eq!(
        r.candidates, 0,
        "the only pair was pruned on the raw compare, so the funnel never classified it: {r:?}"
    );

    // Arm 2: `≥ c` with the margin wholly inside the ambiguity band —
    // the driver's terminal-sliver shape, refused everywhere else.
    let c = 0.4;
    let (doc, a, b) = blocks(c + 5.0 * eps());
    let (sa, sb) = (Selection::body_of(a), Selection::body_of(b));
    let report = clearance(&doc, &box_of("gap"), &sa, &sb, c, Tol::witness());
    let r = report.receipt();
    assert!(r.holds(), "{r:?}");
    assert_eq!(
        report.verdict(),
        &ClearanceVerdict::Holds,
        "a sliver margin is a Holds once the tree has pruned it: {}",
        report.serialize()
    );
    assert_eq!(r.candidates, 0, "{r:?}");
}

/// **EVIDENCE.** The same two questions when the tree CANNOT prune
/// (the gap is inside the box widening, so the boxes overlap): what the
/// funnel does with a pair it is actually handed. Read beside the row
/// above — the verdict a coincident-at-tolerance pair gets depends on
/// whether the tree happened to hand it over.
#[test]
fn a_pair_the_tree_hands_over_is_classified_at_the_funnel() {
    let (doc, a, b) = blocks(eps() / 256.0);
    let (sa, sb) = (Selection::body_of(a), Selection::body_of(b));
    let strict = query(
        ClearanceBound::StrictlyPositive,
        ClearanceConfig {
            max_cell_pairs: 4_096,
            ..ClearanceConfig::default()
        },
    );
    let report = clearance_with(&doc, &box_of("gap"), &sa, &sb, &strict);
    let r = report.receipt();
    println!("[r1] handed-over strict pair: {}", report.serialize());
    assert!(r.holds(), "{r:?}");
    assert!(r.candidates >= 1, "the boxes overlap, so the pair is a candidate: {r:?}");
    assert_ne!(
        report.verdict(),
        &ClearanceVerdict::Holds,
        "a pair the funnel sees at gap ε/256 is never certified strictly positive: {}",
        report.serialize()
    );
}

// ------------------------------------------------ claim 5 / deviation D3

/// An L-shaped plate (its top cap is an L, bounding rectangle
/// `[0,2]²`) and a small block floating 0.5 above the plate's MISSING
/// quadrant, placed by a parameter.
fn l_plate_and_floating_block() -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    declare(&mut r, "lift", 0.0);
    let plate = extruded(
        &mut r,
        &[
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ],
        1.0,
    );
    let block = extruded(
        &mut r,
        &[(1.2, 1.2), (1.8, 1.2), (1.8, 1.8), (1.2, 1.8)],
        1.0,
    );
    let up = Expr::add(len(1.5), param("lift")).expect("a length");
    let floated = r.insert(translated(block, [len(0.0), len(0.0), up]));
    (r.doc, plate, block, floated)
}

/// **PINNED COUNTEREXAMPLE (D3), constructed as the dispatch asked.**
/// The plate's top cap is an L; its carrier WINDOW is the bounding
/// rectangle, which includes the quadrant the plate does not occupy.
/// The block's bottom cap floats 0.5 above exactly that quadrant. The
/// true clearance between the two FACES is `√(0.2² + 0.5²) ≈ 0.539`
/// (the block's cap corner to the L's inner edge); the two WINDOWS
/// come within 0.5.
///
/// At `c = 0.52` the truth is `Holds` and the engine reports
/// `Violated`, with a witness whose plate point lies where the plate
/// has no material (`x > 1 ∧ y > 1`). Disclosed at the module door
/// (D3), and this is the disclosure exercised: a defect gate built on
/// this verdict refuses a sound design. `Holds` at `c = 0.45` stays
/// sound.
#[test]
fn an_l_shaped_face_is_violated_where_it_has_no_material() {
    let (doc, plate, block, floated) = l_plate_and_floating_block();
    let top = Selection {
        at: plate,
        body: 0,
        faces: FaceScope::Named(vec![fixture::fname(plate, RoleSeg::Cap(CapEnd::Top))]),
    };
    let bottom = Selection {
        at: floated,
        body: 0,
        faces: FaceScope::Named(vec![fixture::fname(block, RoleSeg::Cap(CapEnd::Bottom))]),
    };
    let leaf = box_of("lift");
    let sound = clearance(&doc, &leaf, &top, &bottom, 0.45, Tol::witness());
    assert_eq!(sound.verdict(), &ClearanceVerdict::Holds, "{}", sound.serialize());
    assert!(sound.receipt().holds());

    let loose = clearance(&doc, &leaf, &top, &bottom, 0.52, Tol::witness());
    assert!(loose.receipt().holds(), "{:?}", loose.receipt());
    let ClearanceVerdict::Violated(v) = loose.verdict() else {
        panic!(
            "the carrier windows come within 0.5, so today this is Violated: {}",
            loose.serialize()
        );
    };
    let d = recomputed_distance(&loose);
    assert!((d - v.geometry.distance).abs() <= 1e-12, "{d} vs {}", v.geometry.distance);
    assert!(d < 0.52 && d >= 0.5 - 1e-9, "the phantom approach: {d}");
    let p = v.geometry.a_point;
    assert!(
        (p.z - 1.0).abs() <= 1e-9,
        "the plate witness lies on the cap's plane: {p:?}"
    );
    assert!(
        p.x > 1.0 && p.y > 1.0,
        "the plate witness lies in the quadrant the L does not occupy — a point on the \
         carrier window and not on the face: {p:?}"
    );
    println!("[r1] L-plate witness: {:?} -> {:?} d = {d}", p, v.geometry.b_point);
}

/// A block with a semicircular BUMP on its top edge: profile
/// `(0,0) → (2,0) → (2,0.5) → (1.5,0.5) ⌒ (0.5,0.5) → (0,0.5)`, the arc
/// bulging up through `(1, 1)`, extruded 1 along z and placed by a
/// parameter. The bump's cylindrical carrier (centre `(1, 0.5)`,
/// radius 0.5) has its full turn as its window, and the phantom lower
/// half of that turn reaches `y = 0` — the bottom wall's plane.
fn bumped_block() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    declare(&mut r, "place", 0.0);
    let p2 = |x: f64, y: f64| [len(x), len(y)];
    let chain = LoopProgram::Chain(vec![
        ProgramStep::At(p2(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(p2(2.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(p2(2.0, 0.5))),
        ProgramStep::LineTo(ProgramTarget::Point(p2(1.5, 0.5))),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(p2(0.5, 0.5)),
            b: scl(1.0),
        }),
        ProgramStep::LineTo(ProgramTarget::Point(p2(0.0, 0.5))),
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
    let placed = r.insert(translated(solid, [param("place"), len(0.0), len(0.0)]));
    (r.doc, solid, placed)
}

/// **EVIDENCE (claim 6 + D3 on the self-intersection door).** A sound
/// body with one rounded feature. The bump face's window is the whole
/// cylinder, whose phantom lower half touches the bottom wall's plane,
/// and the bottom wall shares no vertex with the bump — so the wedge
/// rule keeps the pair and the engine must classify a separation whose
/// true value on the WINDOWS is 0. What the strictly-positive question
/// answers on this body is printed; the row asserts only the receipt
/// and the arm. (Prediction, from the measured limit: `c = 0⁺` has no
/// slack, so neither `Violated` nor `Holds` is reachable and the answer
/// is a budget refusal — a sound rounded body gets no answer.)
#[test]
fn a_block_with_a_rounded_bump_asks_the_self_intersection_question() {
    let (doc, solid, _placed) = bumped_block();
    let sel = Selection::body_of(solid);
    let q = ClearanceQuery {
        bound: ClearanceBound::StrictlyPositive,
        tol: Tol::witness(),
        config: ClearanceConfig {
            max_cell_pairs: 8_192,
            ..ClearanceConfig::default()
        },
        oracle: &NoTangents,
    };
    let report = clearance_with(&doc, &box_of("place"), &sel, &sel, &q);
    println!("[r1] bumped block self-intersection: {}", report.serialize());
    let r = report.receipt();
    assert!(r.holds(), "{r:?}");
    if let ClearanceVerdict::Refused(ClearanceRefusal::Selection(s)) = report.verdict() {
        panic!("the bumped block did not build at the interval scalar: {s}");
    }
    assert!(
        r.candidates >= 1,
        "the bump's full-turn window reaches the bottom wall, so at least that pair is a \
         candidate: {r:?}"
    );
    assert_ne!(
        report.verdict(),
        &ClearanceVerdict::Holds,
        "a window at separation zero cannot be certified strictly positive: {}",
        report.serialize()
    );
}

/// **EVIDENCE (D3, the dispatch's cylinder case).** A quarter-annulus
/// (a rectangle `r ∈ [1, 2]`, `y ∈ [0, 1]` revolved 90° about the
/// y-axis) beside a block that sits where the OTHER three quarters of
/// the outer cylinder would be. The true clearance is above 2; the
/// outer band's full-turn window comes within 0.1 of the block. Loud
/// skip if the revolve does not build at the interval scalar over an
/// ε-box.
#[test]
fn a_partial_revolve_band_reports_its_phantom_turn() {
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
    // The block: x ∈ [-2.5, -2.1], y ∈ [0, 1], z ∈ [-0.2, 0.2], placed
    // by the parameter along z (a rigid translation).
    let block = extruded(
        &mut r,
        &[(-2.5, 0.0), (-2.1, 0.0), (-2.1, 1.0), (-2.5, 1.0)],
        0.4,
    );
    let placed = r.insert(translated(
        block,
        [len(0.0), len(0.0), Expr::add(len(-0.2), param("place")).expect("a length")],
    ));
    let (sq, sb) = (Selection::body_of(quarter), Selection::body_of(placed));
    let report = clearance(&r.doc, &box_of("place"), &sq, &sb, 1.0, Tol::witness());
    println!("[r1] quarter annulus vs block at c = 1.0: {}", report.serialize());
    let rc = report.receipt();
    assert!(rc.holds(), "{rc:?}");
    match report.verdict() {
        ClearanceVerdict::Refused(ClearanceRefusal::Selection(s)) => {
            println!("[r1] interval_lane_skipped_revolve_did_not_build: {s}");
        }
        ClearanceVerdict::Refused(ClearanceRefusal::Unsupported { carrier, face }) => {
            println!("[r1] interval_lane_skipped_unsupported: {carrier} at {face:?}");
        }
        ClearanceVerdict::Violated(v) => {
            let d = recomputed_distance(&report);
            println!(
                "[r1] phantom-turn witness d = {d}: {:?} -> {:?}",
                v.geometry.a_point, v.geometry.b_point
            );
            assert!(d < 1.0, "the witness is under the bound: {d}");
            // The quarter annulus occupies x ≥ 0 only; a witness at
            // x < 0 is on the phantom turn.
            assert!(
                v.geometry.a_point.x < 0.0 || v.geometry.b_point.x < 0.0,
                "one witness point is on the phantom three quarters: {v:?}"
            );
        }
        other => println!("[r1] quarter annulus answered {other:?}"),
    }
}

/// **CONTROL for the row above (claim 9, the sign hull one carrier
/// over).** The same quarter annulus revolved about the z-axis instead
/// of the y-axis: the band's stored `u_ref` comes from
/// `orthonormal_basis(axis)`, and for `axis = ẑ` that frame is clean
/// while for `axis = ŷ` (`n.z = 0`) it is the two-sided hull. The
/// engine re-charts PLANES only, and `refines` tests one halving, so a
/// hulled cylinder passes the door and then cannot decide. Both
/// verdicts are printed; the row asserts only the receipt.
#[test]
fn a_partial_revolve_about_z_is_the_control_for_the_hulled_band() {
    let mut r = Recorder::new();
    declare(&mut r, "place", 0.0);
    // Profile on the xz-plane (u = x̂, v = ẑ): the rectangle r ∈ [1, 2],
    // z ∈ [0, 1], revolved 90° about ẑ.
    let plane = r.insert(fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]));
    let profile = r.insert(Node::Profile(fixture::desc(
        plane,
        vec![vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]],
    )));
    let axis = r.insert(Node::Datum(Datum::Axis {
        origin: [len(0.0), len(0.0), len(0.0)],
        direction: [scl(0.0), scl(0.0), scl(1.0)],
    }));
    let quarter = r.insert(Node::Revolve {
        profile,
        axis,
        angle: ang(core::f64::consts::FRAC_PI_2),
    });
    // The block: x ∈ [-2.5, -2.1], y ∈ [-0.2, 0.2] (placed), z ∈ [0, 1].
    let block = extruded(
        &mut r,
        &[(-2.5, -0.2), (-2.1, -0.2), (-2.1, 0.2), (-2.5, 0.2)],
        1.0,
    );
    let placed = r.insert(translated(block, [len(0.0), param("place"), len(0.0)]));
    let (sq, sb) = (Selection::body_of(quarter), Selection::body_of(placed));
    let started = std::time::Instant::now();
    let report = clearance(&r.doc, &box_of("place"), &sq, &sb, 1.0, Tol::witness());
    println!(
        "[r1] z-axis quarter annulus vs block at c = 1.0 in {:?}: {}",
        started.elapsed(),
        report.serialize()
    );
    assert!(report.receipt().holds());
    if let ClearanceVerdict::Violated(v) = report.verdict() {
        println!(
            "[r1] z-axis phantom-turn witness d = {}: {:?} -> {:?}",
            recomputed_distance(&report),
            v.geometry.a_point,
            v.geometry.b_point
        );
    }
}

// ------------------------------------------------ claim 4: totality

/// **EVIDENCE + PINS.** The receipt identity and the three arms over
/// the degenerate configurations the shipped sweep does not reach:
/// zero candidates, a pair budget of 0 and 1, a depth budget of 0 on a
/// pair that needs splitting, a negative bound, an infinite bound, an
/// empty named selection, and a duplicated name.
#[test]
fn degenerate_queries_land_in_an_arm_with_a_holding_receipt() {
    let (doc, a, b) = blocks(0.4);
    let (sa, sb) = (Selection::body_of(a), Selection::body_of(b));
    let leaf = box_of("gap");
    let cfg = |pairs: usize, depth: u32| ClearanceConfig {
        max_cell_pairs: pairs,
        max_cell_depth: depth,
        ..ClearanceConfig::default()
    };
    let run = |c: f64, config: ClearanceConfig| {
        let report = clearance_with(&doc, &leaf, &sa, &sb, &query(ClearanceBound::AtLeast(c), config));
        assert!(report.receipt().holds(), "c = {c}: {}", report.serialize());
        report
    };

    // Zero candidates: everything further than 5 m is pruned.
    let far = run(0.1, cfg(0, 0));
    assert_eq!(far.receipt().candidates, 0, "{}", far.serialize());
    assert_eq!(far.verdict(), &ClearanceVerdict::Holds);

    // A pair budget of 0 and of 1 on a bound that needs splitting.
    for pairs in [0usize, 1] {
        let starved = run(0.5, cfg(pairs, 40));
        assert!(
            matches!(
                starved.verdict(),
                ClearanceVerdict::Refused(ClearanceRefusal::Budget(CellBudget::Pairs { .. }))
                    | ClearanceVerdict::Violated(_)
            ),
            "pairs = {pairs}: {}",
            starved.serialize()
        );
        println!("[r1] pairs = {pairs}: {}", starved.serialize());
    }

    // A negative bound holds vacuously; the tree hands over everything
    // (a negative pad is the fail-safe direction) and the funnel
    // discharges every root.
    let negative = run(-1.0, ClearanceConfig::default());
    assert_eq!(negative.verdict(), &ClearanceVerdict::Holds, "{}", negative.serialize());
    assert_eq!(negative.receipt().splits, 0);

    // An infinite bound (evidence): `d − ∞` is not a certified
    // enclosure, so every cell is indeterminate, every pair burns to
    // the budget, and the answer is `Refused(Budget)` — a typed arm,
    // but the wrong name for "the bound is not a distance".
    let infinite = run(
        f64::INFINITY,
        ClearanceConfig {
            max_cell_pairs: 1_024,
            ..ClearanceConfig::default()
        },
    );
    println!("[r1] c = +inf: {}", infinite.serialize());
    assert!(
        matches!(infinite.verdict(), ClearanceVerdict::Refused(ClearanceRefusal::Budget(_))),
        "{}",
        infinite.serialize()
    );

    // An EMPTY named selection: no faces, no candidates — and `Holds`.
    // Read as a finding: a question about nothing is certified rather
    // than refused typed.
    let nothing = Selection {
        at: a,
        body: 0,
        faces: FaceScope::Named(vec![]),
    };
    let vacuous = clearance(&doc, &leaf, &nothing, &sb, 0.1, Tol::witness());
    println!("[r1] empty selection: {}", vacuous.serialize());
    assert_eq!(vacuous.verdict(), &ClearanceVerdict::Holds);
    assert_eq!(vacuous.receipt().candidates, 0);

    // A duplicated name is one face: the same candidates as the name
    // given once.
    let wall = fixture::fname(a, fixture::wall(1));
    let twice = Selection {
        at: a,
        body: 0,
        faces: FaceScope::Named(vec![wall.clone(), wall.clone()]),
    };
    let once = Selection {
        at: a,
        body: 0,
        faces: FaceScope::Named(vec![wall]),
    };
    let cfg_small = ClearanceConfig {
        max_cell_pairs: 256,
        ..ClearanceConfig::default()
    };
    let doubled =
        clearance_with(&doc, &leaf, &twice, &sb, &query(ClearanceBound::AtLeast(0.5), cfg_small));
    let single =
        clearance_with(&doc, &leaf, &once, &sb, &query(ClearanceBound::AtLeast(0.5), cfg_small));
    assert!(doubled.receipt().holds());
    assert!(doubled.receipt().candidates >= 1, "{}", doubled.serialize());
    assert_eq!(doubled.serialize(), single.serialize());
}

/// **EVIDENCE.** A NaN bound is not validated at any door. What the
/// engine does with it is printed and timed; the row asserts only that
/// it lands in an arm with a holding receipt.
#[test]
fn a_nan_bound_is_not_refused_at_the_door() {
    let (doc, a, b) = blocks(0.4);
    let (sa, sb) = (Selection::body_of(a), Selection::body_of(b));
    let started = std::time::Instant::now();
    let report = clearance_with(
        &doc,
        &box_of("gap"),
        &sa,
        &sb,
        &query(
            ClearanceBound::AtLeast(f64::NAN),
            ClearanceConfig {
                max_cell_pairs: 4_096,
                ..ClearanceConfig::default()
            },
        ),
    );
    println!(
        "[r1] c = NaN in {:?}: {}",
        started.elapsed(),
        report.serialize()
    );
    assert!(report.receipt().holds());
}

// ------------------------------------------------ claim 8: determinism

/// **PIN (D9).** The driver-level fold over a drive run under both
/// schedules answers the same bits, on a subdivision of thousands of
/// cell pairs. The engine itself has no parallel path (grep: no rayon
/// in `clearance.rs`); the only schedule that can vary is the driver's.
#[test]
fn the_fold_is_the_same_under_both_driver_schedules() {
    let (doc, a, b) = blocks(0.4);
    let (sa, sb) = (Selection::body_of(a), Selection::body_of(b));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let q = query(
        ClearanceBound::AtLeast(0.41),
        ClearanceConfig {
            max_cell_pairs: 16_384,
            ..ClearanceConfig::default()
        },
    );
    let mut folds = Vec::new();
    for parallel in [false, true, false] {
        let config = DriveConfig {
            parallel,
            ..DriveConfig::default()
        };
        let verdict = drive(&doc, &analyzed, &config, Tol::witness()).expect("builds");
        assert!(!verdict.certified().is_empty(), "{}", verdict.serialize());
        folds.push(clearance_over(&doc, &analyzed, &verdict, &sa, &sb, &q));
    }
    let cells = folds[0].receipt.discharged + folds[0].receipt.violated + folds[0].receipt.refused;
    println!("[r1] fold over {} leaves, {cells} cells: {:?}", folds[0].leaves.len(), folds[0].receipt);
    assert!(cells > 2_000, "a multi-thousand-cell run: {cells}");
    assert_eq!(folds[0], folds[1], "sequential vs rayon");
    assert_eq!(folds[0], folds[2], "repeat");
}

// ------------------------------------------------ the e2e document

/// A U-channel (inner width 2, walls at `x = 0.5` and `x = 2.5`, floor
/// at `y = 0.5`) and a unit slider placed inside it by a `place`
/// parameter: nominal 0.5 of clearance to each wall and to the floor.
/// The macroscopic question is "does the slider keep ≥ c to the
/// channel across the placement tolerance?"
fn channel_and_slider(place_half: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    declare_with(&mut r, "place", 0.0, place_half);
    let channel = extruded(
        &mut r,
        &[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 2.0),
            (2.5, 2.0),
            (2.5, 0.5),
            (0.5, 0.5),
            (0.5, 2.0),
            (0.0, 2.0),
        ],
        1.0,
    );
    let slider = extruded(
        &mut r,
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        1.0,
    );
    let placed = r.insert(translated(
        slider,
        [
            Expr::add(len(1.0), param("place")).expect("a length"),
            len(1.0),
            len(0.0),
        ],
    ));
    (r.doc, channel, placed)
}

/// **E2E, the ε-box arm.** What a consumer gets when the box is one the
/// kernel can replay.
///
/// First the WHOLE-BODY question, slider against channel (evidence):
/// the channel's two caps are U-shaped, their windows are the full
/// `[0,3]×[0,2]` rectangles, and the slider's caps lie IN those
/// rectangles on the same planes — so the windows are at distance 0
/// and the whole-body question is `Violated` at every bound, however
/// generous. That is D3 met on the first realistic document: a part
/// inside a pocket or channel cannot be asked a whole-body clearance
/// question at all.
///
/// Then the question a user would have to learn to ask instead — the
/// slider against the channel's three INNER faces — where the
/// generous and tight bounds `Hold` and the broken bound is `Violated`
/// with a verified witness. Timings printed.
#[test]
fn e2e_channel_slider_over_an_epsilon_box() {
    let (doc, channel, slider) = channel_and_slider(half());
    let (sc, ss) = (Selection::body_of(channel), Selection::body_of(slider));
    let leaf = box_of("place");
    let whole = clearance(&doc, &leaf, &sc, &ss, 0.3, Tol::witness());
    println!("[r1 e2e] whole-body c = 0.3: {}", whole.serialize());
    assert!(whole.receipt().holds());
    assert_eq!(
        whole.verdict().label(),
        "Violated",
        "the U-shaped caps' windows overlap the slider's caps: {}",
        whole.serialize()
    );
    let inner = Selection {
        at: channel,
        body: 0,
        faces: FaceScope::Named(vec![
            fixture::fname(channel, fixture::wall(3)),
            fixture::fname(channel, fixture::wall(4)),
            fixture::fname(channel, fixture::wall(5)),
        ]),
    };
    for (c, expect) in [(0.3, "Holds"), (0.45, "Holds"), (0.55, "Violated")] {
        let started = std::time::Instant::now();
        let report = clearance(&doc, &leaf, &inner, &ss, c, Tol::witness());
        println!(
            "[r1 e2e] c = {c}: {:?} {}",
            started.elapsed(),
            report.serialize()
        );
        assert!(report.receipt().holds());
        assert_eq!(report.verdict().label(), expect, "{}", report.serialize());
        if expect == "Violated" {
            let d = recomputed_distance(&report);
            assert!(d < c && d >= 0.5 - 1e-9, "{d}");
        }
    }
}

/// **E2E, the real-tolerance arm — PINNED FINDING.** A placement
/// tolerance of ±0.05 m is the question a user actually has. No node
/// replays over that box (issue 1191's class), so the drive certifies
/// NO leaf — and `clearance_over` then answers `Holds` over zero
/// leaves: the fold's accumulator starts at `Holds` and nothing moves
/// it. The accounting beside it says 100 % of the mass is refused, but
/// the verdict field a consumer reads first is a pass. E7's
/// "trichotomy, never silence" has a fourth state here — a certificate
/// about nothing — and it is spelled `Holds`.
#[test]
fn e2e_a_real_tolerance_gets_a_holds_over_zero_leaves() {
    let (doc, channel, slider) = channel_and_slider(0.05);
    let (sc, ss) = (Selection::body_of(channel), Selection::body_of(slider));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let config = DriveConfig {
        max_leaves: 64,
        ..DriveConfig::default()
    };
    let verdict = drive(&doc, &analyzed, &config, Tol::witness()).expect("builds at nominal");
    println!(
        "[r1 e2e] drive over ±0.05: certified = {}, refused = {}, accounting = {:?}",
        verdict.certified().len(),
        verdict.refused().len(),
        verdict.accounting()
    );
    let fold = clearance_over(
        &doc,
        &analyzed,
        &verdict,
        &sc,
        &ss,
        &ClearanceQuery::at_least(0.3, Tol::witness()),
    );
    println!("[r1 e2e] fold: leaves = {}, verdict = {:?}", fold.leaves.len(), fold.verdict);
    if fold.leaves.is_empty() {
        assert_eq!(
            fold.verdict,
            ClearanceVerdict::Holds,
            "the finding: a fold over no certified leaf is reported as Holds"
        );
        assert_eq!(fold.verdict.holds(), Some(true));
    } else {
        println!("[r1 e2e] the driver certified {} leaves over a ±0.05 box", fold.leaves.len());
    }
}

// ------------------------------------------------ the fold's accounting

/// **PINNED FINDING (E7 "refuse, typed and PRICED by measure").** The
/// fold hands back the DRIVE's accounting verbatim, so a leaf whose
/// clearance query REFUSED still sits in the `certified` mass column.
/// The fold's own prose says the refused mass "is exactly the share of
/// the parameter box this certificate says nothing about" — here the
/// certificate says nothing about the whole box, the verdict is
/// `Refused`, and the accounting still prices 100 % of the mass as
/// certified. No per-leaf verdict list survives in the fold either, so
/// a consumer cannot recover which leaves refused.
#[test]
fn a_refused_leaf_is_still_priced_as_certified_mass_by_the_fold() {
    let (doc, a, b) = blocks(0.4);
    let (sa, sb) = (Selection::body_of(a), Selection::body_of(b));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness()).expect("builds");
    let certified_mass = verdict.accounting().certified.clone();
    let fold = clearance_over(
        &doc,
        &analyzed,
        &verdict,
        &sa,
        &sb,
        &query(
            ClearanceBound::AtLeast(0.5),
            ClearanceConfig {
                max_cell_pairs: 4,
                ..ClearanceConfig::default()
            },
        ),
    );
    println!(
        "[r1] fold verdict {:?}; accounting {:?}",
        fold.verdict, fold.drive_accounting
    );
    assert!(
        matches!(fold.verdict, ClearanceVerdict::Refused(_)),
        "a starved budget refuses: {:?}",
        fold.verdict
    );
    assert_eq!(
        fold.drive_accounting.certified, certified_mass,
        "the refused query did not move a gram of mass out of `certified`"
    );
    assert!(
        !fold.drive_accounting.refused.contains_key(&editor_core::drive::ReasonClass::Budget),
        "and the clearance budget refusal is not priced under any reason: {:?}",
        fold.drive_accounting.refused
    );
}

/// **EVIDENCE (claim 6).** What the shipped self-intersection rows
/// actually put through the funnel: the dumbbell's and hexagon's
/// candidate counts at `c = 0⁺`. If every non-adjacent pair is pruned
/// by the tree, the strictly-positive funnel site decided nothing on
/// those rows.
#[test]
fn what_the_sound_prism_rows_hand_to_the_funnel() {
    let mut r = Recorder::new();
    declare(&mut r, "place", 0.0);
    let dumbbell = extruded(
        &mut r,
        &[
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 0.8),
            (3.0, 0.8),
            (3.0, 0.0),
            (5.0, 0.0),
            (5.0, 2.0),
            (3.0, 2.0),
            (3.0, 1.2),
            (2.0, 1.2),
            (2.0, 2.0),
            (0.0, 2.0),
        ],
        2.0,
    );
    let _placed = r.insert(translated(dumbbell, [param("place"), len(0.0), len(0.0)]));
    let report = self_intersection(
        &r.doc,
        &box_of("place"),
        &Selection::body_of(dumbbell),
        Tol::witness(),
    );
    println!("[r1] dumbbell self-intersection receipt: {}", report.serialize());
    assert!(report.receipt().holds());
    assert_eq!(report.verdict(), &ClearanceVerdict::Holds);
}

// ------------------------------------------------ claim 9: the basis

/// **PIN (the filed finding, verified).** `Vec3::orthonormal_basis` at
/// `Interval` for a normal with `n.z = 0` returns a sign-hulled frame:
/// both basis vectors carry a `[-1, 1]` factor.
#[test]
fn the_orthonormal_basis_is_sign_hulled_at_interval_when_nz_is_zero() {
    let (zero, one) = (Interval::from_bounds(0.0, 0.0), Interval::from_bounds(1.0, 1.0));
    let n = Vec3::new(one, zero, zero);
    let (b1, b2) = n.orthonormal_basis();
    println!("[r1] basis of +x at Interval: b1 = {b1:?}, b2 = {b2:?}");
    assert!(
        b1.z.lo() <= -1.0 && b1.z.hi() >= 1.0,
        "b1.z is the two-sided hull: {:?}",
        b1.z
    );
    assert!(
        b2.y.lo() <= -1.0 && b2.y.hi() >= 1.0,
        "b2.y is the two-sided hull: {:?}",
        b2.y
    );
    // And a normal with n.z definitely positive is framed cleanly.
    let (c1, _) = Vec3::new(zero, zero, one).orthonormal_basis();
    assert!(c1.x.lo() > 0.5 && c1.x.hi() < 1.5, "{c1:?}");
}

// ------------------------------------------------ claim 2: separation_lo

/// **PINNED COUNTEREXAMPLE (claim 2, the huge-magnitude case).**
/// `Aabb::separation_lo` is documented as a certified LOWER bound on
/// the boxes' separation for every configuration. The norm it takes is
/// the plain `sqrt(x² + y² + z²)` (no scaled hypot, by `geom_core`'s
/// own rule), so a per-axis gap above ~1.34e154 squares to `+∞`, the
/// norm is `+∞`, four `next_down`s land near `f64::MAX`, and the
/// "lower bound" EXCEEDS the true separation by ~1e108. Absurd as a
/// distance, but the contract is stated universally and the proptest
/// generator stops at ±20. Goes red when the bound is made scale-safe
/// (or the contract fenced to a finite range).
#[test]
fn separation_lo_over_claims_when_a_gap_squares_to_infinity() {
    let unit = Aabb {
        min_x: 0.0,
        min_y: 0.0,
        min_z: 0.0,
        max_x: 1.0,
        max_y: 1.0,
        max_z: 1.0,
    };
    let far = Aabb {
        min_x: 1e200,
        min_y: 0.0,
        min_z: 0.0,
        max_x: 1e200,
        max_y: 1.0,
        max_z: 1.0,
    };
    let lo = unit.separation_lo(&far);
    let truth = 1e200 - 1.0;
    println!("[r1] separation_lo = {lo:e} against a true separation of {truth:e}");
    assert!(
        lo > truth,
        "the over-claim is present today (lo = {lo:e}, truth = {truth:e}); if this row is \
         red the bound has been made scale-safe — delete the row"
    );
}

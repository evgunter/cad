//! **The E7 clearance engine** — the trichotomy, the receipt, the
//! witness, the wedge rule, the accelerator seam, and the VERBS
//! acceptance fixtures.
//!
//! Everything here goes through the public doors a consumer has:
//! `clearance` / `self_intersection` / `clearance_with` for the answer,
//! the report's own accessors for what it says, `drive` for a leaf that
//! was actually certified. Nothing reaches past them.
//!
//! # Why every fixture's parameter is a rigid PLACEMENT
//!
//! Two facts about the substrate shape these fixtures, and both are
//! findings rather than preferences.
//!
//! A parameter that feeds an extrude's own magnitude widens that
//! extrude's certification predicates — identities whose enclosure is
//! `[0, c·w]` over a box of width `w` — so above a small fraction of ε
//! the node does not BUILD at the interval scalar and there is no
//! geometry to measure. That is the driver's own measured limit, and
//! issue 1191's class.
//!
//! Separately, every planar face whose normal has a zero z-component —
//! every vertical wall of an extruded prism — carries a SIGN-HULLED
//! `u_ref` at the interval scalar, because the branchless orthonormal
//! basis starts at `copysign(1, n.z)`. The engine re-charts planes at
//! its own door rather than reading that frame; the defect itself is
//! filed as `work/issues/interval-orthonormal-basis-sign-hull.md`.
//!
//! A rigid translation keeps the rest clean: its rotation is the
//! identity, so every stored direction passes through exactly. It does
//! not lift the width ceiling — a transform re-certifies its mapped
//! edges against their carriers, and that identity widens with the box
//! like every other — so every box here is ε-scaled.
//!
//! One consequence is worth stating rather than leaving to be inferred.
//! The monotonicity accelerator's value is to collapse an axis whose
//! own width is what leaves a margin undecided, and at ε scale no
//! parameter-driven width comes near a clearance margin: on today's
//! kernel there is no buildable fixture on which the pruning can change
//! an outcome. The rows below therefore check what is checkable — that
//! the pruning restricts the box to exactly the facet its oracle names,
//! and that removing it changes no verdict — and the demonstration that
//! it BUYS something waits on the widening class (issue 1191).
//!
//! # The measured limit, and where it comes from
//!
//! A `Violated` verdict needs some cell pair whose separation enclosure
//! is definitely BELOW the bound — every point of one cell within `c`
//! of every point of the other. For two flat walls a true gap `g`
//! apart, a cell pair of diameter `w` encloses separations up to about
//! `√(g² + w²)`, so a violation is only reachable once `w² ≲ c² − g²`.
//! The subdivision halves `w` per split, so the depth is logarithmic in
//! the SLACK `c − g` and the cost is exponential in it: a bound
//! comfortably past the gap is cheap, a bound a hair past it is not
//! reachable at all. `the_widths_at_which_cells_discharge_are_measured`
//! reports where the subdivision had to go, and records that at the
//! SHIPPED pair budget every one of those bounds answers definitely and
//! still leaves part of its subdivision priced-refused.
//!
//! The basename carries `interval` deliberately: the engine is gated on
//! that feature (there is nothing to exclude WITH at `f64`) and
//! `scripts/ci-filter.py` reads the name.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::BTreeMap;

use editor_core::UnitSym;
use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::clearance::{
    CellBudget, ClearanceBound, ClearanceConfig, ClearanceQuery, ClearanceRefusal, ClearanceReport,
    ClearanceVerdict, FaceScope, MonotoneOracle, NoTangents, Pruning, Selection, clearance,
    clearance_over, clearance_with, self_intersection,
};
use editor_core::drive::{DriveConfig, drive};
use editor_core::{
    Dimension, Distribution, DocEdit, DocParam, Expr, LoopProgram, Node, ParamName, ProfileDoc,
    ProfileProgram, RecipeNodeId,
};
use geom_core::{Sign, Tol};
use profile::SketchPlane;

use fixture::{Recorder, len, scl};

/// The analysis box's half-width, in metres.
///
/// See the module header: no node's interval replay survives a wider
/// one, on any fixture, because every certification predicate it runs
/// is an identity whose enclosure grows with the box.
fn half() -> f64 {
    Tol::witness().eps() / 64.0
}

fn name(n: &str) -> ParamName {
    ParamName::new(n)
}

/// The leaf box: one axis at [`half`] around the nominal.
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

/// Declares one continuous parameter with a uniform distribution of
/// half-width [`half`].
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

/// A rigid translation along +x — identity rotation, so every stored
/// direction passes through exactly and the placed body's charts are as
/// clean as the literal one's.
fn translated(input: RecipeNodeId, by: Expr) -> Node<ProfileProgram> {
    Node::Transform {
        input,
        translation: [by, len(0.0), len(0.0)],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite angle"),
    }
}

/// A prism over a literal polygon, extruded a literal depth.
fn extruded(r: &mut Recorder, points: &[(f64, f64)], depth: f64) -> RecipeNodeId {
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![LoopProgram::polygon(points.iter().copied()).expect("finite corners")],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: len(depth),
    })
}

/// **The VERBS acceptance fixture** — issue 1055's own dumbbell, placed
/// by a document parameter.
///
/// Two 2×2 blobs joined by a neck whose two facing walls sit at
/// `y = 0.8` and `y = 1.2`, so the neck GAP is `0.4`: exactly the
/// measured instance in issue 1055, a body that shells to a
/// self-intersecting cavity at `t = 0.3` because two 0.3 walls need 0.6
/// of neck and have 0.4.
///
/// The two neck walls share no vertex, so the wedge rule does not
/// exclude them: this pair is precisely the one a wall-clearance
/// certificate has to see.
fn dumbbell() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    declare(&mut r, "place", 0.0);
    let solid = extruded(
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
    let placed = r.insert(translated(
        solid,
        Expr::param(name("place"), Dimension::Length),
    ));
    (r.doc, solid, placed)
}

/// The dumbbell's two NECK walls, by the names the extrude minted:
/// outer-loop segments 2 and 9 are the `y = 0.8` and `y = 1.2` faces.
///
/// Selecting them by name is what keeps the acceptance rows' cost
/// honest AND small: one candidate pair, the one the certificate is
/// about, rather than every pair of a fourteen-face body.
fn neck_walls(minted_at: RecipeNodeId, read_at: RecipeNodeId) -> Selection {
    Selection {
        at: read_at,
        body: 0,
        faces: FaceScope::Named(vec![
            fixture::fname(minted_at, fixture::wall(2)),
            fixture::fname(minted_at, fixture::wall(9)),
        ]),
    }
}

/// **The hexagon family**: a regular hexagonal prism of unit apothem —
/// opposite flats exactly 2 m apart, the walls one apart from each
/// other `2/√3 ≈ 1.155` — placed by the same parameter.
///
/// Its walls are NOT axis-aligned, which makes it the fixture where the
/// inner subdivision earns its keep: a slanted pair's boxes are far
/// closer than the faces are, so the tree cannot exclude the pair and
/// the engine has to discharge it by refining.
fn hexagon() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let radius = 1.0 / (core::f64::consts::PI / 6.0).cos();
    let corners: Vec<(f64, f64)> = (0..6)
        .map(|k| {
            let t = core::f64::consts::PI / 3.0 * f64::from(k);
            (radius * t.cos(), radius * t.sin())
        })
        .collect();
    let mut r = Recorder::new();
    declare(&mut r, "place", 0.0);
    let solid = extruded(&mut r, &corners, 1.0);
    let placed = r.insert(translated(
        solid,
        Expr::param(name("place"), Dimension::Length),
    ));
    (r.doc, solid, placed)
}

/// **The parametric-clearance fixture**: a unit block and a copy of it
/// translated so their two facing walls stand `gap` apart, with `gap` a
/// document parameter.
///
/// This is the fixture whose ANSWER moves with the parameter: over a
/// ±0.05 box around a 0.4 nominal the separation encloses [0.35, 0.45],
/// so a bound below that band holds, one above it is violated, and one
/// INSIDE it is the case no domain subdivision can settle — which is
/// where the monotonicity accelerator earns its place.
fn facing_blocks(gap: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    declare(&mut r, "gap", gap);
    let a = extruded(
        &mut r,
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        1.0,
    );
    let by = Expr::add(len(1.0), Expr::param(name("gap"), Dimension::Length))
        .expect("1 m + a length is a length");
    let b = r.insert(translated(a, by));
    (r.doc, a, b)
}

/// The distance between a violation witness's two points, recomputed
/// HERE rather than read off the report — the suite's own independent
/// check that a witness is a real configuration.
fn witness_distance(report: &ClearanceReport) -> f64 {
    let ClearanceVerdict::Violated(v) = report.verdict() else {
        panic!("expected a violation, got {}", report.verdict().label());
    };
    let (a, b) = (v.geometry.a_point, v.geometry.b_point);
    let (dx, dy, dz) = (a.x - b.x, a.y - b.y, a.z - b.z);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// A query at a bound and a config, with no accelerator.
fn query(c: f64, config: ClearanceConfig) -> ClearanceQuery<'static> {
    ClearanceQuery {
        bound: ClearanceBound::AtLeast(c),
        tol: Tol::witness(),
        config,
        oracle: &NoTangents,
    }
}

// -------------------------------------------------------- trichotomy

/// **VERBS acceptance, arm 1**: a generous bound holds over the whole
/// leaf, and the receipt accounts for every candidate pair.
///
/// It holds with NOTHING subdivided, and that is worth stating: the
/// dumbbell's walls are axis-aligned, so each face's interval box IS
/// its face, the tree's certified separation bound is exact, and every
/// non-adjacent pair is excluded before the engine looks at it. The
/// pruning is a certificate here rather than a heuristic — and
/// `a_slanted_pair_is_discharged_by_refining_it` is the row where it is
/// not enough and the subdivision does the work.
#[test]
fn a_generous_wall_clearance_holds_over_the_dumbbell() {
    let (doc, minted, _at) = dumbbell();
    let sel = Selection::body_of(minted);
    let report = clearance(&doc, &box_of("place"), &sel, &sel, 0.2, Tol::witness());
    assert_eq!(
        report.verdict(),
        &ClearanceVerdict::Holds,
        "{}",
        report.serialize()
    );
    assert_eq!(report.verdict().holds(), Some(true));
    let r = report.receipt();
    assert!(r.holds(), "receipt identity: {r:?}");
    assert_eq!(r.violated, 0);
    assert_eq!(r.refused, 0);
    assert_eq!(r.splits, 0, "{}", report.serialize());
    assert_eq!(r.discharged, r.candidates);
}

/// **VERBS acceptance, arm 2**: the bound issue 1055's shell verb asks
/// about — two 0.3 walls need 0.6 of neck — is violated, and the
/// witness re-verifies at `f64` independently of the report.
#[test]
fn the_bound_the_neck_breaks_is_violated_with_a_verified_witness() {
    let (doc, minted, _at) = dumbbell();
    let sel = neck_walls(minted, minted);
    let report = clearance(&doc, &box_of("place"), &sel, &sel, 0.6, Tol::witness());
    let ClearanceVerdict::Violated(v) = report.verdict() else {
        panic!("expected a violation at c = 0.6: {}", report.serialize());
    };
    assert_eq!(report.verdict().holds(), Some(false));
    // The report's own number and the suite's recomputation from its
    // two points agree — and both are under the bound, at the neck's
    // true gap.
    let recomputed = witness_distance(&report);
    assert!(
        (recomputed - v.geometry.distance).abs() <= 1e-12,
        "the report's distance {} is not the distance between its own points {recomputed}",
        v.geometry.distance
    );
    // A witness is A violating configuration, not the extremal one:
    // the engine reports the first cell pair its deterministic walk
    // proves violating, which is somewhere across the neck rather than
    // at the neck's own closest approach.
    assert!(recomputed < 0.6, "a witness under the bound: {recomputed}");
    assert!(
        recomputed >= 0.4 - 1e-9,
        "and no closer than the geometry allows: {recomputed}"
    );
    assert!(v.geometry.a != v.geometry.b, "a pair of distinct faces");
    let r = report.receipt();
    assert!(r.holds(), "receipt identity under a violation: {r:?}");
    assert!(r.violated > 0);
}

/// **VERBS acceptance, arm 3**: a starved budget refuses, priced and
/// named — never a silent partial and never a false certificate.
#[test]
fn a_starved_cell_budget_refuses_typed() {
    let (doc, minted, _at) = dumbbell();
    let sel = neck_walls(minted, minted);
    let q = query(
        0.6,
        ClearanceConfig {
            max_cell_pairs: 8,
            ..ClearanceConfig::default()
        },
    );
    let report = clearance_with(&doc, &box_of("place"), &sel, &sel, &q);
    match report.verdict() {
        ClearanceVerdict::Refused(ClearanceRefusal::Budget(CellBudget::Pairs {
            max_cell_pairs,
        })) => assert_eq!(*max_cell_pairs, 8),
        other => panic!("expected a priced budget refusal, got {other:?}"),
    }
    let r = report.receipt();
    assert!(r.holds(), "receipt identity under budget exhaustion: {r:?}");
    assert!(r.refused > 0, "the refusal is counted, not dropped");
}

/// The depth budget is its own arm, and widening the pair budget does
/// not move it.
#[test]
fn a_starved_cell_depth_refuses_on_its_own_axis() {
    let (doc, minted, _at) = dumbbell();
    let sel = neck_walls(minted, minted);
    let q = query(
        0.6,
        ClearanceConfig {
            max_cell_depth: 0,
            ..ClearanceConfig::default()
        },
    );
    let report = clearance_with(&doc, &box_of("place"), &sel, &sel, &q);
    match report.verdict() {
        ClearanceVerdict::Refused(ClearanceRefusal::Budget(CellBudget::Depth {
            max_cell_depth,
        })) => assert_eq!(*max_cell_depth, 0),
        other => panic!("expected a depth refusal, got {other:?}"),
    }
    assert!(report.receipt().holds());
}

/// **Trichotomy totality**: across a sweep of bounds and budgets every
/// run lands in exactly one arm and its receipt holds.
///
/// The sweep's blind spot: it varies the BOUND and the two budgets over
/// one fixture, so it says nothing about carriers the fixture does not
/// build (a sphere, a torus, a free-form patch) or about a selection
/// that does not resolve. Those are the table rows below.
#[test]
fn every_run_lands_in_exactly_one_arm_with_a_holding_receipt() {
    let (doc, minted, _at) = dumbbell();
    let sel = neck_walls(minted, minted);
    let leaf = box_of("place");
    for c in [0.0, 0.1, 0.39, 0.6, 5.0] {
        for pairs in [4, 64, 4_096] {
            let q = query(
                c,
                ClearanceConfig {
                    max_cell_pairs: pairs,
                    ..ClearanceConfig::default()
                },
            );
            let report = clearance_with(&doc, &leaf, &sel, &sel, &q);
            let r = report.receipt();
            assert!(
                r.holds(),
                "receipt broken at c = {c}, pairs = {pairs}: {}",
                report.serialize()
            );
            let labels = ["Holds", "Violated", "Refused"];
            assert!(labels.contains(&report.verdict().label()));
        }
    }
}

/// **The subdivision does the work the tree cannot.**
///
/// Two opposite walls of the hexagon are 2 m apart, but they are
/// SLANTED, so their axis-aligned boxes come within 2/√3 ≈ 1.155 of
/// each other. A 1.5 m bound therefore survives the tree — the boxes
/// are closer than that — and only the inner subdivision can discharge
/// it. For an axis-aligned pair the two numbers coincide and the tree
/// answers alone, which is what the dumbbell rows show.
#[test]
fn a_slanted_pair_the_tree_cannot_exclude_is_discharged_by_refining_it() {
    let (doc, minted, _at) = hexagon();
    let sel = Selection {
        at: minted,
        body: 0,
        faces: FaceScope::Named(vec![
            fixture::fname(minted, fixture::wall(0)),
            fixture::fname(minted, fixture::wall(3)),
        ]),
    };
    let report = clearance(&doc, &box_of("place"), &sel, &sel, 1.5, Tol::witness());
    assert_eq!(
        report.verdict(),
        &ClearanceVerdict::Holds,
        "opposite flats are 2 m apart: {}",
        report.serialize()
    );
    let r = report.receipt();
    assert!(r.holds());
    assert_eq!(r.candidates, 1, "the named pair, not excluded: {r:?}");
    assert!(
        r.splits > 0,
        "and refined rather than answered at its root: {r:?}"
    );
    assert_eq!(r.discharged, r.splits + r.candidates);
}

// -------------------------------------------- the self-intersection

/// A sound prism is certified strictly positive between every
/// NON-ADJACENT face pair — which is only possible because the wedge
/// rule excludes the adjacent ones: every side wall touches its
/// neighbours and both caps at distance exactly zero, and a query that
/// examined those pairs would report a well-formed body as
/// self-intersecting.
#[test]
fn a_sound_prism_certifies_strictly_positive_between_non_adjacent_faces() {
    for (doc, at) in [
        {
            let (d, m, _) = hexagon();
            (d, m)
        },
        {
            let (d, m, _) = dumbbell();
            (d, m)
        },
    ] {
        let sel = Selection::body_of(at);
        let report = self_intersection(&doc, &box_of("place"), &sel, Tol::witness());
        assert_eq!(
            report.verdict(),
            &ClearanceVerdict::Holds,
            "{}",
            report.serialize()
        );
        let r = report.receipt();
        assert!(r.holds());
        assert_eq!(r.violated, 0);
        assert_eq!(r.refused, 0);
    }
}

// ------------------------------------------- the parametric question

/// A bound BELOW the whole parameter band holds over the whole box.
#[test]
fn a_bound_under_the_parameter_band_holds() {
    let (doc, a, b) = facing_blocks(0.4);
    let (sa, sb) = (Selection::body_of(a), Selection::body_of(b));
    let report = clearance(&doc, &box_of("gap"), &sa, &sb, 0.3, Tol::witness());
    assert_eq!(
        report.verdict(),
        &ClearanceVerdict::Holds,
        "the gap is at least 0.35 everywhere in the box: {}",
        report.serialize()
    );
    assert!(report.receipt().holds());
}

/// A bound ABOVE the whole band is violated, with a witness the `f64`
/// rebuild confirms at the box's own midpoint gap.
#[test]
fn a_bound_over_the_parameter_band_is_violated() {
    let (doc, a, b) = facing_blocks(0.4);
    let (sa, sb) = (Selection::body_of(a), Selection::body_of(b));
    let report = clearance(&doc, &box_of("gap"), &sa, &sb, 0.5, Tol::witness());
    assert_eq!(
        report.verdict().holds(),
        Some(false),
        "the gap is at most 0.45 everywhere in the box: {}",
        report.serialize()
    );
    let d = witness_distance(&report);
    assert!(d < 0.5, "the witness is under the bound: {d}");
    // Measured at the leaf's midpoint, where the gap is its nominal
    // 0.4 — so no witness can be closer than that.
    assert!(
        d >= 0.4 - 1e-9,
        "and no closer than the midpoint gap allows: {d}"
    );
    assert!(report.receipt().holds());
}

// ---------------------------------------------------------- refusals

/// A name that does not resolve to a face of the selected body refuses
/// typed rather than silently dropping it.
#[test]
fn a_selection_that_is_not_a_face_refuses_naming_itself() {
    let (doc, _minted, at) = hexagon();
    let sel = Selection {
        at,
        body: 0,
        faces: FaceScope::Named(vec![fixture::ename(at, fixture::wall(0))]),
    };
    let whole = Selection::body_of(at);
    let report = clearance(&doc, &box_of("place"), &sel, &whole, 0.1, Tol::witness());
    match report.verdict() {
        ClearanceVerdict::Refused(ClearanceRefusal::Selection(_)) => {}
        other => panic!("expected a selection refusal, got {other:?}"),
    }
}

/// A node the document does not carry has no faces, and the engine says
/// so rather than answering about nothing.
#[test]
fn a_node_that_does_not_exist_refuses_at_the_selection() {
    let (doc, _minted, _at) = hexagon();
    let sel = Selection::body_of(RecipeNodeId(9999));
    let report = clearance(&doc, &box_of("place"), &sel, &sel, 0.1, Tol::witness());
    match report.verdict() {
        ClearanceVerdict::Refused(ClearanceRefusal::Selection(_)) => {}
        other => panic!("expected a selection refusal, got {other:?}"),
    }
}

// -------------------------------------------------------- determinism

/// D9 across repeats: the same question over the same document answers
/// the same bits, on a run of several thousand cell pairs.
#[test]
fn the_answer_is_deterministic_across_repeats() {
    let (doc, minted, _at) = dumbbell();
    let sel = neck_walls(minted, minted);
    let leaf = box_of("place");
    let first = clearance(&doc, &leaf, &sel, &sel, 0.6, Tol::witness());
    let cells = first.receipt().discharged + first.receipt().violated + first.receipt().refused;
    assert!(
        cells > 200,
        "the determinism row runs on a many-hundred-cell subdivision: {:?}",
        first.receipt()
    );
    for _ in 0..3 {
        let again = clearance(&doc, &leaf, &sel, &sel, 0.6, Tol::witness());
        assert_eq!(again.serialize(), first.serialize());
    }
}

// ----------------------------------------------------- the accelerator

/// An oracle that certifies one axis' monotonicity — the SEAM's test
/// implementor, standing in for the tangent seed door until it lands.
///
/// The claim is honest for [`facing_blocks`]: the two blocks' facing
/// walls stand exactly `gap` apart, so the separation is strictly
/// increasing in that parameter and its minimum over the leaf is at the
/// axis' lower end.
struct GapIncreasing;

impl MonotoneOracle for GapIncreasing {
    fn monotone_in(&self, param: &ParamName) -> Option<Sign> {
        (param == &name("gap")).then_some(Sign::Positive)
    }
}

fn gap_run(
    doc: &ProfileDoc,
    sa: &Selection,
    sb: &Selection,
    c: f64,
    pruning: Pruning,
    oracle: &dyn MonotoneOracle,
) -> ClearanceReport {
    let q = ClearanceQuery {
        bound: ClearanceBound::AtLeast(c),
        tol: Tol::witness(),
        config: ClearanceConfig {
            pruning,
            max_cell_pairs: 4_096,
            ..ClearanceConfig::default()
        },
        oracle,
    };
    clearance_with(doc, &box_of("gap"), sa, sb, &q)
}

/// **The accelerator is removable**: on the bounds the engine answers
/// definitely, the verdict is the same with the pruning off.
#[test]
fn pruning_does_not_change_a_definite_verdict() {
    let (doc, a, b) = facing_blocks(0.4);
    let (sa, sb) = (Selection::body_of(a), Selection::body_of(b));
    for (c, expected) in [(0.3, "Holds"), (0.5, "Violated")] {
        let off = gap_run(&doc, &sa, &sb, c, Pruning::Off, &NoTangents);
        let on = gap_run(&doc, &sa, &sb, c, Pruning::Facets, &GapIncreasing);
        assert_eq!(off.verdict().label(), expected, "{}", off.serialize());
        assert_eq!(on.verdict().label(), expected, "{}", on.serialize());
        assert_eq!(off.verdict().holds(), on.verdict().holds());
    }
}

/// **The pruning fires, and what it does is exactly what it says**: a
/// run with the accelerator over the box equals, bit for bit, a run
/// WITHOUT it over the facet the oracle names.
///
/// That equality is the whole content of the mechanism, and it is
/// checkable without a box wide enough for the saving to show (the
/// module header states why there is none).
#[test]
fn pruning_restricts_the_box_to_the_facet_it_names() {
    let (doc, a, b) = facing_blocks(0.4);
    let (sa, sb) = (Selection::body_of(a), Selection::body_of(b));
    let mut facet_axes = BTreeMap::new();
    facet_axes.insert(
        name("gap"),
        BoxAxis::Varying {
            lo: -half(),
            hi: -half(),
        },
    );
    let facet = ParamBox::from_axes(facet_axes);
    let q = |pruning, oracle: &'static dyn MonotoneOracle| ClearanceQuery {
        bound: ClearanceBound::AtLeast(0.5),
        tol: Tol::witness(),
        config: ClearanceConfig {
            pruning,
            max_cell_pairs: 4_096,
            ..ClearanceConfig::default()
        },
        oracle,
    };
    let on = clearance_with(
        &doc,
        &box_of("gap"),
        &sa,
        &sb,
        &q(Pruning::Facets, &GapIncreasing),
    );
    let at_facet = clearance_with(&doc, &facet, &sa, &sb, &q(Pruning::Off, &NoTangents));
    assert_eq!(
        on.serialize(),
        at_facet.serialize(),
        "the accelerator did not restrict the box to the facet its oracle named"
    );
}

/// **E9 live**: an oracle that certifies nothing forfeits exactly the
/// pruning. `Pruning::Facets` with [`NoTangents`] is `Pruning::Off`,
/// bit for bit — which is what "a degraded tangent forfeits the pruning
/// and NOTHING else" means at this seam.
#[test]
fn a_degraded_tangent_forfeits_the_pruning_and_nothing_else() {
    let (doc, a, b) = facing_blocks(0.4);
    let (sa, sb) = (Selection::body_of(a), Selection::body_of(b));
    assert_eq!(
        gap_run(&doc, &sa, &sb, 0.4, Pruning::Facets, &NoTangents).serialize(),
        gap_run(&doc, &sa, &sb, 0.4, Pruning::Off, &NoTangents).serialize()
    );
}

// -------------------------------------------------- the driver's leaf

/// The engine runs INSIDE a leaf the driver actually certified, and the
/// fold carries the drive's own accounting through unaltered — the
/// honest denominator for every sentence a report writes about the
/// answer.
#[test]
fn a_driver_certified_leaf_carries_the_certificate() {
    let (doc, minted, _at) = dumbbell();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness())
        .expect("the dumbbell builds at its nominal");
    assert!(
        !verdict.certified().is_empty(),
        "the driver certifies this box: {}",
        verdict.serialize()
    );
    let sel = Selection::body_of(minted);
    let fold = clearance_over(
        &doc,
        &verdict,
        &sel,
        &sel,
        &ClearanceQuery::at_least(0.2, Tol::witness()),
    );
    assert_eq!(fold.verdict, ClearanceVerdict::Holds);
    assert!(fold.receipt.holds(), "{:?}", fold.receipt);
    assert_eq!(fold.leaves, verdict.certified().len());
    assert_eq!(&fold.accounting, verdict.accounting());
}

// --------------------------------------------------- the honest limit

/// **The measured limit** (the module header's argument, executed).
///
/// Nothing here is a baseline to preserve: the counts are a fact about
/// this fixture. What is asserted is the SHAPE the argument predicts —
/// a definite answer at every bound, a subdivision spanning orders of
/// magnitude in cell size, and, at the shipped pair budget, part of
/// every one of those subdivisions left priced-refused.
#[test]
fn the_widths_at_which_cells_discharge_are_measured() {
    let (doc, minted, _at) = dumbbell();
    let sel = neck_walls(minted, minted);
    let leaf = box_of("place");
    let mut previous = 0usize;
    for c in [0.9, 0.8, 0.7, 0.6, 0.5] {
        let report = clearance(&doc, &leaf, &sel, &sel, c, Tol::witness());
        let r = report.receipt();
        assert!(r.holds());
        assert_eq!(
            report.verdict().holds(),
            Some(false),
            "the neck's 0.4 gap breaks every bound in this row: c = {c}, {}",
            report.serialize()
        );
        let cells = r.discharged + r.violated + r.refused;
        let w = report.widths();
        println!(
            "[limit] c = {c}: cells = {cells}, splits = {}, widths = {w:?}",
            r.splits
        );
        // Every row exhausts the shipped pair budget, and still answers
        // definitely: the violation is found long before the receipt is
        // complete, and what the budget refuses is the REST of the
        // subdivision, priced and counted.
        assert_eq!(cells, previous.max(cells));
        assert!(
            r.refused > 0,
            "the shipped dial does not complete this: {r:?}"
        );
        let (widest, narrowest) = (w.widest.unwrap_or(0.0), w.narrowest.unwrap_or(0.0));
        assert!(
            narrowest * 100.0 < widest,
            "the subdivision spans orders of magnitude in cell size: {w:?}"
        );
        previous = cells;
    }
}

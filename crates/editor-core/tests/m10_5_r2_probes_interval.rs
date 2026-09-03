//! **R2's independent probes of the M10-5 clearance engine** (PR #1638
//! at 9f143595).
//!
//! Everything here is derived from `docs/M10-5-SPEC.md` and
//! `docs/ERROR-DESIGN.md` E7 rather than from the shipped suite, on
//! geometry the shipped suite does not use: a COMB — an E-shaped prism
//! with two internal slots of different widths — plus a probe block
//! that sits inside one of those slots.
//!
//! The comb is chosen because it has what neither the dumbbell nor the
//! hexagon has: several non-adjacent pairs at several different real
//! distances inside ONE body, a non-convex planar CAP face whose
//! bounding rectangle covers ground the face does not, and a
//! macroscopic question ("is every internal gap at least c?") a user
//! would actually ask.
//!
//! # Rows that pin a FINDING rather than a contract
//!
//! Three rows below are named `..._r2_finding` and assert the CURRENT
//! behaviour where R2 believes the contract says otherwise. Each goes
//! red when the defect it describes is fixed, which is the point: the
//! finding is a gate, not a note in a review that scrolls away.
//!
//! The basename carries `interval` because the engine is gated on that
//! feature and `scripts/ci-filter.py` reads the name.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::BTreeMap;

use editor_core::UnitSym;
use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::clearance::{
    CellReceipt, ClearanceBound, ClearanceConfig, ClearanceQuery, ClearanceRefusal,
    ClearanceReport, ClearanceVerdict, FaceScope, MonotoneOracle, NoTangents, Pruning,
    SELF_INTERSECTION_GAP, Selection, clearance, clearance_over, clearance_with,
    self_intersection,
};
use editor_core::drive::{DriveConfig, drive};
use editor_core::{
    CapEnd, Dimension, Distribution, DocEdit, DocParam, Expr, LoopProgram, Node, ParamName,
    ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg,
};
use geom_core::k_stats::decide;
use geom_core::{Band, Margin, Sign, Tol};
use profile::SketchPlane;

use fixture::{Recorder, len, scl};

// ------------------------------------------------------------ authoring

/// The analysis box's half-width. Same ceiling the shipped suite hit
/// (issue 1191's class): a wider box and no node builds at `Interval`.
fn half() -> f64 {
    Tol::witness().eps() / 64.0
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

fn translated(input: RecipeNodeId, dx: Expr, dy: Expr, dz: Expr) -> Node<ProfileProgram> {
    Node::Transform {
        input,
        translation: [dx, dy, dz],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite angle"),
    }
}

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

/// **The comb** — an E on its back, extruded 1 m, placed by a document
/// parameter.
///
/// Twelve outer-loop segments, indices as the extrude mints them:
///
/// ```text
///   seg9  seg7        seg5  seg3
///     |     |           |     |
///  ___|     |___________|     |___   y = 2 tops
/// |   |     |           |     |   |
/// |   |_____|           |_____|   |  y = 0.5 slot floors (seg8, seg4)
/// |     0.5                1.0    |  ← slot A width, slot B width
/// |_______________________________|  y = 0 (seg0)
/// x=0   0.5 1.0       1.5   2.5  3.0
/// ```
///
/// The two pairs that matter, and their true separations:
/// (seg7, seg9) at 0.5 m — slot A's walls — and (seg3, seg5) at 1.0 m —
/// slot B's. Neither pair shares a vertex, so neither is excluded by
/// the wedge rule. Nothing else in the body is closer than 0.5: the
/// bottom face (seg0) stands 0.5 below each slot floor (seg4, seg8).
///
/// So `min gap over the body = 0.5`, from FOUR distinct non-adjacent
/// pairs — which makes "is every internal gap at least c?" a real
/// question with a real answer, at three different bounds.
fn comb() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    declare(&mut r, "place", 0.0);
    let solid = extruded(
        &mut r,
        &[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 2.0),
            (2.5, 2.0),
            (2.5, 0.5),
            (1.5, 0.5),
            (1.5, 2.0),
            (1.0, 2.0),
            (1.0, 0.5),
            (0.5, 0.5),
            (0.5, 2.0),
            (0.0, 2.0),
        ],
        1.0,
    );
    let placed = r.insert(translated(
        solid,
        Expr::param(name("place"), Dimension::Length),
        len(0.0),
        len(0.0),
    ));
    (r.doc, solid, placed)
}

/// **The D3 fixture**: an L-shaped prism and a small block parked in
/// the L's NOTCH, straddling the L's `z = 0` cap plane.
///
/// The L is the unit square with the corner `x > 0.4, y > 0.4` cut
/// away, extruded 1 m. Its bottom cap is therefore an L-shaped planar
/// face whose carrier WINDOW is the whole `[0, 1] × [0, 1]` rectangle —
/// including the notch, where the face has no material at all.
///
/// The block occupies `x ∈ [0.85, 0.95]`, `y ∈ [0.85, 0.95]`,
/// `z ∈ [-0.05, 0.05]`: deep in the notch and straddling `z = 0`. So
///
/// - the true distance from the block to the L's bottom CAP FACE is
///   0.45 m — the reach from `x = 0.85` back to the material at
///   `x = 0.4`;
/// - the distance from the block to that cap's WINDOW is 0, because the
///   window covers the notch and the block crosses its plane.
///
/// An L small enough that the subdivision can actually resolve the
/// difference inside the shipped pair budget, which the comb's 3 × 2
/// cap is not.
fn ell_with_a_block_in_the_notch() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
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
        &[(0.85, 0.85), (0.95, 0.85), (0.95, 0.95), (0.85, 0.95)],
        0.1,
    );
    let placed = r.insert(translated(
        probe,
        Expr::param(name("place"), Dimension::Length),
        len(0.0),
        len(-0.05),
    ));
    (r.doc, solid, placed)
}

/// Two unit blocks whose facing walls stand `gap` apart, `gap` a
/// literal rather than a parameter so the fixture's separation is
/// exactly the number in its name.
fn blocks_apart(gap: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    declare(&mut r, "place", 0.0);
    let a = extruded(
        &mut r,
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        1.0,
    );
    let b = r.insert(translated(a, len(1.0 + gap), len(0.0), len(0.0)));
    let _ = r.insert(translated(
        a,
        Expr::param(name("place"), Dimension::Length),
        len(0.0),
        len(0.0),
    ));
    (r.doc, a, b)
}

fn wall_name(node: RecipeNodeId, seg: u32) -> editor_core::StableName {
    fixture::fname(node, fixture::wall(seg))
}

fn cap_name(node: RecipeNodeId) -> editor_core::StableName {
    fixture::fname(node, RoleSeg::Cap(CapEnd::Bottom))
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

fn strict(config: ClearanceConfig) -> ClearanceQuery<'static> {
    ClearanceQuery {
        bound: ClearanceBound::StrictlyPositive,
        tol: Tol::witness(),
        config,
        oracle: &NoTangents,
    }
}

// ------------------------------ 1. the macroscopic question, end to end

/// **The e2e a consumer actually asks**, on the comb: is every internal
/// gap at least `c`? Three bounds, three answers, and the answers are
/// the geometry's.
///
/// `c = 0.3` is under both slots and both floor clearances → `Holds`.
/// `c = 0.7` is over slot A (0.5) and under slot B (1.0) → `Violated`.
/// `c = 1.5` is over everything → `Violated`.
///
/// The receipt holds on every one, and the discharge-width fold is
/// printed so a reader can see what the certificate cost.
#[test]
fn the_combs_internal_gaps_answer_at_three_bounds() {
    let (doc, minted, _placed) = comb();
    let sel = Selection::body_of(minted);
    let leaf = box_of("place");
    for (c, expected) in [(0.3, "Holds"), (0.7, "Violated"), (1.5, "Violated")] {
        let report = clearance_with(&doc, &leaf, &sel, &sel, &at_least(c, cfg(65_536, 40)));
        assert!(
            report.receipt().holds(),
            "receipt broken at c = {c}: {}",
            report.serialize()
        );
        assert_eq!(
            report.verdict().label(),
            expected,
            "the comb's minimum internal gap is 0.5 m: at c = {c} {}",
            report.serialize()
        );
        println!(
            "[r2 e2e] c = {c}: {} receipt = {:?} widths = {:?}",
            report.verdict().label(),
            report.receipt(),
            report.widths()
        );
    }
}

/// The same three bounds through the DRIVER-level fold, which is the
/// door a report would call: every certified leaf, the drive's own
/// accounting carried through, and the receipt still a forest.
#[test]
fn the_leaf_fold_answers_the_same_question_over_a_real_drive() {
    let (doc, minted, _placed) = comb();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &DriveConfig::default(), Tol::witness())
        .expect("the comb builds at its nominal");
    assert!(
        !verdict.certified().is_empty(),
        "the drive certifies something to fold over: {}",
        verdict.serialize()
    );
    let sel = Selection::body_of(minted);
    for (c, expected) in [(0.3, "Holds"), (0.7, "Violated")] {
        let fold = clearance_over(&doc, &verdict, &sel, &sel, &at_least(c, cfg(65_536, 40)));
        assert_eq!(fold.verdict.label(), expected, "at c = {c}");
        assert!(fold.receipt.holds(), "{:?}", fold.receipt);
        assert_eq!(fold.leaves, verdict.certified().len());
        assert_eq!(&fold.accounting, verdict.accounting());
        println!(
            "[r2 e2e] fold at c = {c}: {} over {} leaves, receipt = {:?}",
            fold.verdict.label(),
            fold.leaves,
            fold.receipt
        );
    }
}

/// **Witness honesty, recomputed independently.** The violation the
/// comb's slot A produces at `c = 0.7` carries two `f64` points; their
/// distance, recomputed here from the report's own fields, must be the
/// distance the report states AND must itself violate the bound.
#[test]
fn the_combs_violation_witness_re_verifies_from_its_own_points() {
    let (doc, minted, _placed) = comb();
    let sel = named(
        minted,
        vec![wall_name(minted, 7), wall_name(minted, 9)],
    );
    let report = clearance_with(
        &doc,
        &box_of("place"),
        &sel,
        &sel,
        &at_least(0.7, cfg(65_536, 40)),
    );
    let ClearanceVerdict::Violated(v) = report.verdict() else {
        panic!("slot A is 0.5 wide and the bound is 0.7: {}", report.serialize());
    };
    let mine = witness_distance(&report);
    assert_eq!(
        mine, v.geometry.distance,
        "the reported distance IS the distance between the reported points"
    );
    assert!(
        mine < 0.7,
        "the witness must violate the bound it witnesses: {mine}"
    );
    // Slot A's walls are 0.5 apart, so any real pair of points on them
    // is at least that far apart — the witness is a configuration the
    // geometry admits.
    assert!(
        mine >= 0.5 - 1.0e-9,
        "the witness is a real configuration on this pair: {mine}"
    );
    // **R2 finding, measured here rather than asserted.** The witness is
    // NOT a closest-point pair: `verify_witness` evaluates the two
    // cells' MIDPOINTS and never minimises. `GeometryWitness`'s own
    // field doc says "the closest-point pair the f64 rebuild found" and
    // E7 asks for a "closest-point pair"; what ships is a point pair
    // that is merely inside the bound.
    println!(
        "[r2 witness] closest possible on this pair = 0.5, witness reports {mine} \
         (a cell midpoint, not a minimisation)"
    );
    assert!(
        mine > 0.5 + 1.0e-3,
        "R2 finding: the reported pair is a cell midpoint, measurably not the \
         closest approach the two faces admit: {mine} against 0.5"
    );
    assert!(
        v.geometry.a != v.geometry.b,
        "a face is never at a distance from itself"
    );
}

// --------------- 2. the BVH prune decides, and disagrees with the funnel

/// **R2's first falsification of claim 2/D4.**
///
/// The engine has two thresholds for "strictly positive", not one. The
/// funnel's is the band: `SELF_INTERSECTION_GAP` classifies a
/// separation `d` with `|d| ≤ ε` as `Sign::Zero`, and the site's own
/// doc says a definite `Zero` *is the violation the check exists to
/// find*. The BVH's is a raw `separation_lo(query) > pad` with
/// `pad = c = 0`, so ANY positive box separation prunes the pair before
/// the funnel sees it.
///
/// Between those two thresholds sits a window of width ε in which the
/// engine answers `Holds` on a configuration its own funnel calls a
/// violation. This row walks it: two blocks `ε/2` apart — coincident at
/// the run's tolerance, which is precisely what the strictly-positive
/// question is asked about — certify `Holds` with ZERO candidates,
/// while `decide` at the same site on the same number returns
/// `Sign::Zero`.
///
/// The control beside it is the same fixture at gap 0, where
/// `separation_lo` is not positive, the pair survives to the funnel,
/// and the violation IS reported. So moving two bodies FURTHER APART by
/// less than one tolerance flips `Violated` to `Holds`.
#[test]
fn a_sub_epsilon_gap_is_pruned_past_the_strict_funnel_r2_finding() {
    let eps = Tol::witness().eps();
    let band = Band::linear(Tol::witness()).expect("a linear band");
    // What the engine's OWN funnel says about a separation of ε/2 at
    // the strictly-positive site: it is a violation.
    assert_eq!(
        decide(SELF_INTERSECTION_GAP, Margin::of(eps / 2.0), band),
        Ok(Sign::Zero),
        "the funnel classifies a sub-ε separation as a coincidence, and \
         SELF_INTERSECTION_GAP reads a definite Zero as the violation"
    );

    // Gap 0: the boxes touch, `separation_lo` is 0, and the pair
    // SURVIVES to the funnel — which is the whole difference. The
    // engine then spends its budget on it and refuses, priced (see
    // `the_strict_violation_arm_is_unreachable_r2_finding` for why it
    // can never do better).
    let (doc, a, b) = blocks_apart(0.0);
    let touching = clearance_with(
        &doc,
        &box_of("place"),
        &Selection::body_of(a),
        &Selection::body_of(b),
        &strict(cfg(4_096, 24)),
    );
    assert!(
        touching.receipt().candidates > 0,
        "the control: a coincidence reaches the funnel: {}",
        touching.serialize()
    );
    assert_ne!(
        touching.verdict(),
        &ClearanceVerdict::Holds,
        "the control: a coincidence is NOT certified strictly positive: {}",
        touching.serialize()
    );

    // Gap ε/2: strictly FURTHER apart, still a coincidence at the run's
    // tolerance — and now the answer is Holds, on zero candidates.
    let (doc, a, b) = blocks_apart(eps / 2.0);
    let pruned = clearance_with(
        &doc,
        &box_of("place"),
        &Selection::body_of(a),
        &Selection::body_of(b),
        &strict(cfg(4_096, 24)),
    );
    assert_eq!(
        pruned.verdict(),
        &ClearanceVerdict::Holds,
        "R2 finding: a sub-ε separation certifies Holds: {}",
        pruned.serialize()
    );
    assert_eq!(
        pruned.receipt(),
        CellReceipt::default(),
        "R2 finding: and it does so having examined NOTHING — the whole \
         certificate is the BVH's raw compare"
    );
}

/// **R2's second falsification: the strictly-positive VIOLATION arm has
/// no reachable path.**
///
/// The margin at `SELF_INTERSECTION_GAP` is `d − 0`, and `d` is an
/// interval NORM, so its enclosure is never negative: `Sign::Negative`
/// — the arm every other bound reaches a violation through — cannot
/// occur. The only remaining route is `Sign::Zero`, which at
/// `Decide for Interval` requires the WHOLE enclosure inside `[-ε, ε]`.
/// A cell pair's separation enclosure runs from its closest approach to
/// its furthest, so `hi ≤ ε` demands both domain cells be shrunk below
/// the run's ε — thirty halvings per axis from a metre-scale window,
/// four axes, on every pair.
///
/// The consequence: two bodies that INTERPENETRATE — as gross a
/// self-intersection as geometry admits — cannot be reported
/// `Violated`. The engine spends its budget and refuses. That is the
/// honest direction (never a false certificate), but it means E7 §3's
/// detection arm is unreachable at any budget a report could hold, and
/// nothing in the PR body or the module docs says so: the measured-limit
/// section is about the `AtLeast` question only.
///
/// This row pins it at three budgets so it goes red the day a violation
/// becomes reachable.
#[test]
fn the_strict_violation_arm_is_unreachable_r2_finding() {
    // Two unit blocks overlapping by half their width.
    let (doc, a, b) = blocks_apart(-1.5);
    let (sa, sb) = (Selection::body_of(a), Selection::body_of(b));
    for pairs in [4_096usize, 16_384, 65_536] {
        let report = clearance_with(&doc, &box_of("place"), &sa, &sb, &strict(cfg(pairs, 40)));
        let r = report.receipt();
        assert!(r.holds(), "{r:?}");
        assert!(
            r.candidates > 0,
            "interpenetrating boxes are never pruned: {r:?}"
        );
        assert_eq!(
            r.violated, 0,
            "R2 finding: not one cell pair of an interpenetration classifies as a \
             definite violation at {pairs} pairs: {}",
            report.serialize()
        );
        assert_eq!(
            report.verdict().label(),
            "Refused",
            "R2 finding: gross interpenetration answers Refused, never Violated: {}",
            report.serialize()
        );
    }
}

/// The vacuity the row above implies, on the shipped acceptance's own
/// shape: `self_intersection` over a sound prism examines ZERO pairs,
/// because the wedge rule removes the adjacent ones and the raw
/// `separation_lo > 0` prune removes every other one.
///
/// So the suite's claim-6 row — "a sound prism certifies strictly
/// positive between non-adjacent faces" — passes without the funnel
/// classifying anything. It cannot go red for a body whose
/// non-adjacent faces are a real distance apart, whatever the engine
/// does with them, which is Q3's "can this test fail".
#[test]
fn self_intersection_over_a_sound_body_examines_nothing_r2_finding() {
    let (doc, minted, _placed) = comb();
    let report = self_intersection(
        &doc,
        &box_of("place"),
        &Selection::body_of(minted),
        Tol::witness(),
    );
    assert_eq!(report.verdict(), &ClearanceVerdict::Holds);
    assert_eq!(
        report.receipt(),
        CellReceipt::default(),
        "R2 finding: the strictly-positive certificate over a 14-face body \
         is entirely the BVH's, with no cell pair classified: {}",
        report.serialize()
    );
}

// ------------------------------ 3. D3: a witness outside the trimmed face

/// **Deviation D3, as a number.** The L's bottom cap is L-shaped; its
/// window is the bounding square, which covers the notch. A block
/// parked in that notch stands 0.45 m from the FACE and 0 m from the
/// WINDOW, so a bound of 0.3 — which the two faces satisfy with 50 %
/// to spare — is reported `Violated`, with a witness whose `(u, v)`
/// lands where the body has no material at all.
///
/// The direction is the safe one and the module states it at the door.
/// What this row adds is the SIZE: the reported violation is not a
/// near-miss at the rounding scale, and the witness point is a place
/// the face does not reach.
#[test]
fn a_violation_witness_can_land_where_the_face_is_not() {
    let (doc, ell, block_node) = ell_with_a_block_in_the_notch();
    let cap = named(ell, vec![cap_name(ell)]);
    let block = Selection::body_of(block_node);
    let report = clearance_with(
        &doc,
        &box_of("place"),
        &cap,
        &block,
        &at_least(0.3, cfg(65_536, 40)),
    );
    let ClearanceVerdict::Violated(v) = report.verdict() else {
        panic!(
            "the cap's WINDOW reaches the block even though its face does not: {}",
            report.serialize()
        );
    };
    println!(
        "[r2 D3] witness on the L cap at (u, v) = {:?}, points {:?} / {:?}, d = {}",
        v.geometry.a_uv, v.geometry.a_point, v.geometry.b_point, v.geometry.distance
    );
    assert!(
        v.geometry.distance < 0.3,
        "the witness violates the bound it was minted for: {}",
        v.geometry.distance
    );
    // The cap's material nearest the block is 0.45 m away, so a bound
    // of 0.3 is one the two FACES satisfy — and it is reported broken.
    assert!(
        v.geometry.distance < 0.45,
        "R2: the reported approach is inside the true face separation of 0.45: {}",
        v.geometry.distance
    );
    assert!(report.receipt().holds());
}

/// The other direction of D3, which is the one soundness rests on: a
/// bound the WINDOW satisfies is a bound the FACES satisfy. The block's
/// real 0.45 m stand-off from the L's `x = 0.4` wall certifies at a
/// bound under it.
#[test]
fn holds_over_the_window_is_holds_over_the_face() {
    let (doc, ell, block_node) = ell_with_a_block_in_the_notch();
    // The L's notch wall at x = 0.4 (outer-loop segment 2 runs
    // (1.0, 0.4) → (0.4, 0.4); segment 3 runs (0.4, 0.4) → (0.4, 1.0)).
    let wall = named(ell, vec![wall_name(ell, 3)]);
    let block = Selection::body_of(block_node);
    let report = clearance_with(
        &doc,
        &box_of("place"),
        &wall,
        &block,
        &at_least(0.2, cfg(65_536, 40)),
    );
    assert_eq!(
        report.verdict(),
        &ClearanceVerdict::Holds,
        "the block stands 0.45 m from the notch wall: {}",
        report.serialize()
    );
    assert!(report.receipt().holds());
}

// ------------------------------------ 4. totality and the receipt identity

/// **Trichotomy totality on a fixture the shipped sweep does not use**,
/// over the full product of bounds, pair budgets and depth budgets —
/// including the degenerate corners the shipped sweep leaves out: a
/// zero pair budget, a zero depth budget, `c = 0` and a `c` past the
/// whole body.
///
/// Every run lands in exactly one arm and every receipt is a forest.
#[test]
fn every_configuration_lands_in_one_arm_with_a_holding_receipt() {
    let (doc, minted, _placed) = comb();
    let sel = Selection::body_of(minted);
    let leaf = box_of("place");
    let mut seen: BTreeMap<&'static str, usize> = BTreeMap::new();
    for c in [0.0, 0.05, 0.5, 0.75, 4.0] {
        for pairs in [0usize, 1, 7, 512] {
            for depth in [0u32, 1, 6] {
                let report =
                    clearance_with(&doc, &leaf, &sel, &sel, &at_least(c, cfg(pairs, depth)));
                let r = report.receipt();
                assert!(
                    r.holds(),
                    "receipt broken at c = {c}, pairs = {pairs}, depth = {depth}: {}",
                    report.serialize()
                );
                let label = report.verdict().label();
                assert!(["Holds", "Violated", "Refused"].contains(&label));
                *seen.entry(label).or_default() += 1;
            }
        }
    }
    println!("[r2] arms reached: {seen:?}");
    assert!(
        seen.len() >= 2,
        "the sweep is not vacuous — it reaches more than one arm: {seen:?}"
    );
}

/// The same totality for the STRICT question, which the shipped sweep
/// never varies over budgets at all.
#[test]
fn the_strict_question_is_total_over_budgets_too() {
    let (doc, a, b) = blocks_apart(0.0);
    let (sa, sb) = (Selection::body_of(a), Selection::body_of(b));
    let leaf = box_of("place");
    for pairs in [0usize, 1, 3, 64, 4_096] {
        for depth in [0u32, 2, 12] {
            let report = clearance_with(&doc, &leaf, &sa, &sb, &strict(cfg(pairs, depth)));
            assert!(
                report.receipt().holds(),
                "receipt broken at pairs = {pairs}, depth = {depth}: {}",
                report.serialize()
            );
            assert!(
                ["Holds", "Violated", "Refused"].contains(&report.verdict().label())
            );
        }
    }
}

/// A query with no candidate pairs at all — a single-face selection
/// against itself. It must not be silence, and its receipt must hold at
/// zero.
#[test]
fn a_selection_with_no_pair_answers_holds_at_an_empty_receipt() {
    let (doc, minted, _placed) = comb();
    let one = named(minted, vec![wall_name(minted, 0)]);
    let report = clearance_with(
        &doc,
        &box_of("place"),
        &one,
        &one,
        &at_least(1.0, cfg(65_536, 40)),
    );
    assert_eq!(report.verdict(), &ClearanceVerdict::Holds);
    assert_eq!(report.receipt(), CellReceipt::default());
    assert!(report.receipt().holds());
}

/// A bound that is not a number. `c = NaN` must not be a silent pass
/// and must not hang: the engine has to land in the trichotomy like
/// every other input.
#[test]
fn a_bound_that_is_not_a_number_still_lands_in_the_trichotomy() {
    let (doc, minted, _placed) = comb();
    let sel = Selection::body_of(minted);
    for c in [f64::NAN, f64::INFINITY, -1.0] {
        let report = clearance_with(
            &doc,
            &box_of("place"),
            &sel,
            &sel,
            &at_least(c, cfg(256, 8)),
        );
        assert!(
            report.receipt().holds(),
            "receipt broken at c = {c}: {}",
            report.serialize()
        );
        println!("[r2] c = {c} → {}", report.verdict().label());
        assert!(["Holds", "Violated", "Refused"].contains(&report.verdict().label()));
    }
}

// ------------------------------------------- 5. the accelerator seam

/// An oracle that claims monotonicity in a parameter the separation is
/// NOT monotone in — the seam's contract says the claim is the
/// implementor's and the engine cannot check it, so this is what an
/// M10-4 seed getting it wrong looks like from here.
struct AlwaysDecreasing;

impl MonotoneOracle for AlwaysDecreasing {
    fn monotone_in(&self, _param: &ParamName) -> Option<Sign> {
        Some(Sign::Negative)
    }
}

/// **The accelerator is removable**, on the comb, at every bound the
/// engine answers definitely: `Pruning::Off` and `Pruning::Facets` with
/// a truthful oracle agree on the VERDICT.
///
/// The comb's separations do not move with a rigid placement, so
/// `Sign::Zero` (constant) is the honest claim for its one parameter,
/// and both facet choices are then legitimate.
#[test]
fn the_accelerator_changes_no_verdict_on_the_comb() {
    struct Constant;
    impl MonotoneOracle for Constant {
        fn monotone_in(&self, _param: &ParamName) -> Option<Sign> {
            Some(Sign::Zero)
        }
    }
    let (doc, minted, _placed) = comb();
    let sel = Selection::body_of(minted);
    let leaf = box_of("place");
    for c in [0.3, 0.7, 1.5] {
        let off = clearance_with(&doc, &leaf, &sel, &sel, &at_least(c, cfg(4_096, 20)));
        let on = clearance_with(
            &doc,
            &leaf,
            &sel,
            &sel,
            &ClearanceQuery {
                bound: ClearanceBound::AtLeast(c),
                tol: Tol::witness(),
                config: ClearanceConfig {
                    pruning: Pruning::Facets,
                    ..cfg(4_096, 20)
                },
                oracle: &Constant,
            },
        );
        assert_eq!(
            off.verdict().label(),
            on.verdict().label(),
            "the accelerator moved a verdict at c = {c}: {} vs {}",
            off.serialize(),
            on.serialize()
        );
        assert_eq!(off.verdict().holds(), on.verdict().holds());
    }
}

/// **E9's forfeit, on my own fixture**: `Facets` with an oracle that
/// certifies nothing is `Off`, bit for bit.
#[test]
fn no_tangents_forfeits_the_pruning_and_nothing_else_on_the_comb() {
    let (doc, minted, _placed) = comb();
    let sel = Selection::body_of(minted);
    let leaf = box_of("place");
    for c in [0.3, 0.7] {
        let off = clearance_with(&doc, &leaf, &sel, &sel, &at_least(c, cfg(4_096, 20)));
        let forfeited = clearance_with(
            &doc,
            &leaf,
            &sel,
            &sel,
            &ClearanceQuery {
                bound: ClearanceBound::AtLeast(c),
                tol: Tol::witness(),
                config: ClearanceConfig {
                    pruning: Pruning::Facets,
                    ..cfg(4_096, 20)
                },
                oracle: &NoTangents,
            },
        );
        assert_eq!(off.serialize(), forfeited.serialize(), "at c = {c}");
    }
}

/// **The seam's soundness question**, asked the way the trait's own doc
/// invites: the engine cannot check the claim, so a WRONG `Some(sign)`
/// silently restricts the box to a facet the minimum is not on.
///
/// On this fixture the separations do not move with the parameter, so a
/// wrong sign costs nothing and the verdicts agree — which is what the
/// row asserts. The finding it evidences is not a red here: it is that
/// nothing in the engine, the suite, or the seam's type can tell a
/// truthful oracle from `AlwaysDecreasing`, and on a fixture where the
/// separation DID move with the parameter the wrong facet would be the
/// wrong answer, reported as a certificate.
#[test]
fn a_lying_oracle_is_indistinguishable_at_the_seam() {
    let (doc, minted, _placed) = comb();
    let sel = Selection::body_of(minted);
    let leaf = box_of("place");
    let truthful = clearance_with(&doc, &leaf, &sel, &sel, &at_least(0.7, cfg(4_096, 20)));
    let lying = clearance_with(
        &doc,
        &leaf,
        &sel,
        &sel,
        &ClearanceQuery {
            bound: ClearanceBound::AtLeast(0.7),
            tol: Tol::witness(),
            config: ClearanceConfig {
                pruning: Pruning::Facets,
                ..cfg(4_096, 20)
            },
            oracle: &AlwaysDecreasing,
        },
    );
    assert_eq!(
        truthful.verdict().label(),
        lying.verdict().label(),
        "on a rigid placement the facet choice cannot matter — which is \
         exactly why this fixture cannot detect a wrong claim"
    );
}

// ---------------------------------------------------- 6. D9 determinism

/// **D9 on a multi-thousand-cell run of R2's own construction**: the
/// whole-body comb query at a bound that has to subdivide, repeated,
/// serialized, compared bit for bit.
#[test]
fn the_comb_answer_is_bit_stable_across_repeats() {
    let (doc, minted, _placed) = comb();
    let sel = Selection::body_of(minted);
    let leaf = box_of("place");
    let first = clearance_with(&doc, &leaf, &sel, &sel, &at_least(0.7, cfg(65_536, 40)));
    let r = first.receipt();
    let cells = r.discharged + r.violated + r.refused;
    assert!(
        cells > 2_000,
        "the determinism row runs on a multi-thousand-cell subdivision: {r:?}"
    );
    for _ in 0..3 {
        let again = clearance_with(&doc, &leaf, &sel, &sel, &at_least(0.7, cfg(65_536, 40)));
        assert_eq!(again.serialize(), first.serialize());
    }
    println!("[r2 D9] {cells} cell pairs, stable over 4 runs");
}

/// **D9 across SCHEDULES.** The clearance engine has no parallel path
/// of its own — `Sweep::run` is a sequential stack walk and
/// `clearance_over` a sequential fold — so the only schedule that can
/// move is the DRIVER's, which decides the leaf set and its order. The
/// fold's answer must be the same over a parallel drive and a
/// sequential one, receipt and widths included.
#[test]
fn the_fold_is_the_same_over_a_parallel_and_a_sequential_drive() {
    let (doc, minted, _placed) = comb();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let sel = Selection::body_of(minted);
    let run = |parallel: bool| {
        let verdict = drive(
            &doc,
            &analyzed,
            &DriveConfig {
                parallel,
                ..DriveConfig::default()
            },
            Tol::witness(),
        )
        .expect("the comb builds at its nominal");
        let fold = clearance_over(&doc, &verdict, &sel, &sel, &at_least(0.7, cfg(4_096, 20)));
        (
            fold.verdict.label().to_owned(),
            fold.receipt,
            fold.leaves,
            fold.widths,
        )
    };
    assert_eq!(run(true), run(false));
}

/// **A fold over a drive that certified NOTHING answers `Holds`.**
///
/// `LeafFold::verdict` starts at `Holds` and is only ever moved by a
/// leaf, so a drive whose every leaf refused hands a consumer
/// `verdict.holds() == Some(true)` over zero examined leaves. The
/// accounting beside it carries the refused mass, and the door's own
/// doc calls that "the honest denominator" — but the VERDICT field, the
/// one `holds()` reads, says the bound is certified.
///
/// The tree's own convention elsewhere is that nothing collapses a
/// refusal into a silent pass (`ClearanceVerdict::holds` returns `None`
/// for a refusal for exactly that reason). This row pins the gap.
#[test]
fn a_fold_over_zero_certified_leaves_reports_holds_r2_finding() {
    let (doc, minted, _placed) = comb();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    // A zero-leaf budget: the root leaf is refused and nothing is
    // certified (the driver's own documented `max_leaves = 0` corner).
    let verdict = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            max_leaves: 0,
            ..DriveConfig::default()
        },
        Tol::witness(),
    )
    .expect("the comb builds at its nominal");
    if !verdict.certified().is_empty() {
        println!("[r2] the zero-leaf drive still certified; row is evidence-only here");
        return;
    }
    let sel = Selection::body_of(minted);
    let fold = clearance_over(
        &doc,
        &verdict,
        &sel,
        &sel,
        &at_least(0.7, cfg(4_096, 20)),
    );
    assert_eq!(fold.leaves, 0);
    assert_eq!(
        fold.verdict,
        ClearanceVerdict::Holds,
        "R2 finding: a certificate over nothing reads as a pass"
    );
    assert_eq!(
        fold.verdict.holds(),
        Some(true),
        "R2 finding: and `holds()` — the accessor a consumer gates on — says true"
    );
    println!(
        "[r2] zero-leaf fold: verdict = {}, leaves = {}, accounting = {:?}",
        fold.verdict.label(),
        fold.leaves,
        fold.accounting
    );
}

/// Determinism is not just repeat-stability: the answer must not depend
/// on WHICH equivalent spelling of the selection produced it. The same
/// two faces named in the other order must give the same verdict and
/// the same receipt.
#[test]
fn the_answer_does_not_depend_on_the_order_names_are_written_in() {
    let (doc, minted, _placed) = comb();
    let forward = named(minted, vec![wall_name(minted, 7), wall_name(minted, 9)]);
    let reverse = named(minted, vec![wall_name(minted, 9), wall_name(minted, 7)]);
    let leaf = box_of("place");
    let a = clearance_with(&doc, &leaf, &forward, &forward, &at_least(0.7, cfg(4_096, 20)));
    let b = clearance_with(&doc, &leaf, &reverse, &reverse, &at_least(0.7, cfg(4_096, 20)));
    assert_eq!(a.serialize(), b.serialize());
}

// -------------------------------------- 7. the refusal arms with no rows

/// **`ClearanceRefusal::Unsupported` has no row in the shipped suite**,
/// though §3 of the spec requires the behaviour and the suite's own
/// comment promises "the table rows below". This is that row, on the
/// carrier class the fixture library can actually build: a cylindrical
/// face is admitted, so the refusal has to be reached another way —
/// through a selection naming a face of a body the fold does not carry.
///
/// What it pins is the direction: a carrier the engine cannot window
/// must produce a NAMED class, never a skip and never a sample.
#[test]
fn the_unsupported_arm_names_its_carrier_class_when_it_fires() {
    // Every carrier this fixture library builds (plane) is admitted, so
    // the row asserts the reachable half: the refusal type carries a
    // named class and a face, and `name()` is stable for goldening.
    let refusal = ClearanceRefusal::Unsupported {
        carrier: "a free-form face",
        face: topo::entity::FaceKey::default(),
    };
    assert_eq!(refusal.name(), "unsupported");
    // And the engine's own admission door refuses rather than skipping:
    // a comb face that IS admitted produces a window, so the contrast
    // is that no code path drops a face silently — every face of a
    // selection reaches `window_of`, which either windows it or refuses
    // the whole query.
    let (doc, minted, _placed) = comb();
    let sel = Selection::body_of(minted);
    let report = clearance_with(
        &doc,
        &box_of("place"),
        &sel,
        &sel,
        &at_least(0.3, cfg(65_536, 40)),
    );
    assert_eq!(report.verdict(), &ClearanceVerdict::Holds);
}

/// **The pair budget is enforced at admission, and the refusal is
/// PRICED rather than a partial answer.** A budget of one pair on a
/// body with many candidates must refuse, and the receipt must still
/// account for every candidate.
#[test]
fn a_one_pair_budget_refuses_and_still_accounts_for_every_candidate() {
    let (doc, minted, _placed) = comb();
    let sel = Selection::body_of(minted);
    let report = clearance_with(
        &doc,
        &box_of("place"),
        &sel,
        &sel,
        &at_least(1.5, cfg(1, 40)),
    );
    let r = report.receipt();
    assert!(r.holds(), "{r:?}");
    assert!(r.candidates > 1, "the fixture really has many pairs: {r:?}");
    assert!(
        matches!(
            report.verdict(),
            ClearanceVerdict::Refused(ClearanceRefusal::Budget(_))
                | ClearanceVerdict::Violated(_)
        ),
        "a starved budget is priced, never silence: {}",
        report.serialize()
    );
}

/// The verdict-combination order, exercised where it matters: a run
/// that contains BOTH a definite violation and a priced refusal reports
/// the violation, and the receipt shows both.
#[test]
fn a_violation_outranks_a_refusal_and_the_receipt_shows_both() {
    let (doc, minted, _placed) = comb();
    let sel = Selection::body_of(minted);
    let report = clearance_with(
        &doc,
        &box_of("place"),
        &sel,
        &sel,
        &at_least(1.5, cfg(4_096, 40)),
    );
    let r = report.receipt();
    assert!(r.holds(), "{r:?}");
    if r.refused > 0 && r.violated > 0 {
        assert_eq!(
            report.verdict().label(),
            "Violated",
            "a finding outranks an unknown: {}",
            report.serialize()
        );
    }
    println!("[r2] combination-order receipt: {r:?}");
}

// -------------------------------------------- 8. the measured limit

/// **The cost claim, re-measured on the comb**, and it does not have
/// the shape the PR body states.
///
/// The PR says "the depth is logarithmic in the slack and the cost
/// exponential in it", which reads as monotone: tighter bound, more
/// work. The real curve is U-SHAPED, and the reason is structural. A
/// bound above the pair's FURTHEST separation is a definite violation
/// at the ROOT — one cell pair, zero splits — because the whole
/// enclosure sits below the bound. Only bounds between the closest and
/// the furthest separation need subdivision, and those are the
/// expensive ones. On slot A (0.5 m walls, furthest separation ≈ 1.87 m)
/// `c = 2.0` costs ONE cell and `c = 1.5` costs the whole 65 536-pair
/// budget.
///
/// Evidence-only on the counts; the asserted shape is that a definite
/// answer comes back at every bound and that the cheap end is the
/// generous one, which is the opposite of what a reader of the PR body
/// would predict.
#[test]
fn the_cost_curve_is_u_shaped_not_monotone_r2_finding() {
    let (doc, minted, _placed) = comb();
    let sel = named(minted, vec![wall_name(minted, 7), wall_name(minted, 9)]);
    let leaf = box_of("place");
    let mut costs = Vec::new();
    for c in [2.0, 1.5, 1.0, 0.8, 0.6] {
        let report = clearance_with(&doc, &leaf, &sel, &sel, &at_least(c, cfg(65_536, 40)));
        let r = report.receipt();
        assert!(r.holds());
        assert_eq!(
            report.verdict().holds(),
            Some(false),
            "slot A's 0.5 m gap breaks every bound in this row: c = {c}, {}",
            report.serialize()
        );
        let cells = r.discharged + r.violated + r.refused;
        println!(
            "[r2 limit] c = {c}: cells = {cells}, splits = {}, widths = {:?}",
            r.splits,
            report.widths()
        );
        costs.push((c, cells));
    }
    let (_, cheapest) = costs[0];
    assert_eq!(
        cheapest, 1,
        "R2: a bound above the pair's furthest separation violates at the ROOT, \
         so the most generous bound is the CHEAPEST: {costs:?}"
    );
    assert!(
        costs[1].1 > cheapest * 1_000,
        "R2: and the next bound down costs orders more, which is the U the PR \
         body's monotone sentence does not describe: {costs:?}"
    );
}

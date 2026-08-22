//! M8-14 (#222): **long-turn sweeps certify.** The executed frontier
//! this unit retires: `sweep_body` on helix arc paths ≥ ~0.5 turn
//! refused at `nurbs_span_meter` (`ParamSpan` escalation) because the
//! integral speed meter's single global chord collapsed under
//! `sweep_places`' near-antipodal frame roll. The meter's integral arm
//! now joins a per-span scan with the global chord (derivation on
//! `NurbsCurve3::speed_lower_bound`; corpus + flips in
//! `geom/tests/curves/m8_14_long_turn_meter.rs`), so a long-turn
//! corner path whose speed never drops meters positively and the body
//! assembles and certifies.
//!
//! The oracle is independent (Pappus-style): a constant CENTERED
//! SYMMETRIC profile swept in a normal-section path-following frame
//! encloses volume `A·L` in the continuum — the curvature moment
//! cancels by the profile's symmetry and roll about the tangent drops
//! out of the Jacobian (the twisted-cubic demo's argument, applied to
//! a path the kernel refused until this unit). `L` is the
//! interpolant's arc length by dense chordal summation; the analytic
//! helix length `Δθ·√(r² + c²)` brackets the interpolant's from
//! above.
//!
//! The certified volume is the volume of the BODY the kernel built —
//! a stack of station-to-station skinned slabs whose frame turns by
//! `θ_slab` per slab — not of the continuum tube, and the two differ
//! by a MEASURED `≈ θ_slab²/4` (0.0099 at θ_slab ≈ 0.196 across the
//! half/full/two-turn rows, which share that per-slab turn). So the
//! oracle here is a CONVERGENCE pin, which is the stronger statement:
//! doubling the stations must shrink the A·L gap by ~4× (second
//! order), on top of an absolute cap at the measured level and the
//! certified enclosure pad staying orders tighter than the
//! discretization bracket. A volume bug would break the order; a
//! meter regression would refuse the build outright.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsCurve3;
use geom_core::{Affine3, Mat3, Point3, Vec3};
use sweep::sweep_body;

mod common;
use common::orient::{
    FIXED_AXIS_GUARD_COS, LevelIndex, along_v, assert_caps_face_out, assert_walls_face_out,
    min_roll_turn, stack_axis,
};
use common::quad;
use geom_core::Tol;

/// Profile half-width: small against the helix radius, so the swept
/// tube is embedded (no station's section reaches the axis).
const H: f64 = 0.08;
/// Helix radius (m) and pitch per revolution (m).
const R: f64 = 1.0;
const PITCH: f64 = 0.4;

/// A helix arc of `turns` revolutions, radius [`R`], pitch [`PITCH`]
/// per revolution, interpolated at 33 exact points per revolution
/// (min 33) — the same path family the montage-v2 probe measured
/// refusing at ≥ 0.5 turn.
fn helix_path(turns: f64) -> NurbsCurve3<f64> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = ((32.0 * turns).ceil() as usize).max(32);
    let pts: Vec<Point3<f64>> = (0..=n)
        .map(|k| {
            let s = f64::from(u32::try_from(k).unwrap()) / f64::from(u32::try_from(n).unwrap());
            let a = std::f64::consts::TAU * turns * s;
            Point3::new(R * a.cos(), R * a.sin(), PITCH * turns * s)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

/// The profile placement: plane through the path start, normal to the
/// start tangent (the skinned demo's frame recipe — `sweep_places`
/// carries this frame along the path by minimal rotation).
fn start_place(path: &NurbsCurve3<f64>) -> Affine3<f64> {
    let (lo, _) = path.domain();
    let d = path.deriv(lo);
    let n = d / d.norm();
    let helper = if n.z.abs() < 0.9 {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let u = helper.cross(n);
    let u = u / u.norm();
    let v = n.cross(u);
    let p = path.eval(lo);
    Affine3::from_parts(Mat3::from_cols(u, v, n), Vec3::new(p.x, p.y, p.z))
}

/// The interpolant's arc length by dense chordal summation — the
/// oracle's `L`, independent of every kernel meter.
fn chordal_length(path: &NurbsCurve3<f64>, samples: usize) -> f64 {
    let (lo, hi) = path.domain();
    #[allow(clippy::cast_precision_loss)]
    let n = samples as f64;
    (0..samples)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let a = lo + (hi - lo) * (i as f64) / n;
            #[allow(clippy::cast_precision_loss)]
            let b = lo + (hi - lo) * ((i + 1) as f64) / n;
            (path.eval(b) - path.eval(a)).norm()
        })
        .sum()
}

/// Sweep a centered square along a helix of `turns` revolutions at
/// `stations` stations; certify; return the relative gap to the
/// Pappus `A·L` oracle (asserting the pad stays orders tighter).
fn helix_oracle_gap(turns: f64, stations: usize) -> f64 {
    let path = helix_path(turns);
    let place = start_place(&path);
    let profile = quad([(-H, -H), (H, -H), (H, H), (-H, H)]);
    let swept = sweep_body::<f64>(&profile, place, &path, stations, 3, Tol::witness())
        .unwrap_or_else(|e| panic!("a {turns}-turn helical sweep must build now: {e:?}"));
    let m =
        topo::props::mass_properties(&swept.body, Tol::witness()).expect("mass properties certify");
    assert!(
        m.volume_pad < 1e-9,
        "{turns}-turn helix @ {stations} stations: the certified enclosure must \
         stay far tighter than the discretization bracket (pad {})",
        m.volume_pad
    );
    let area = (2.0 * H) * (2.0 * H);
    let oracle = area * chordal_length(&path, 20_000);
    let rel = ((m.volume - oracle) / oracle).abs();
    println!(
        "helix {turns} turns @ {stations} stations | certified {} (pad {}) | A*L {oracle} | rel {rel}",
        m.volume, m.volume_pad
    );
    rel
}

/// The convergence pin (module docs): the coarse gap sits at the
/// measured `θ_slab²/4` level, and doubling the stations shrinks it
/// like the second-order quantity it is.
fn assert_second_order(what: &str, coarse: f64, fine: f64, cap: f64) {
    assert!(
        coarse <= cap,
        "{what}: coarse A·L gap {coarse} above the measured discretization \
         level {cap} — the body is not the tube the oracle prices"
    );
    assert!(
        fine <= coarse / 3.0,
        "{what}: refining 2x only moved the A·L gap {coarse} -> {fine} — \
         not second-order convergence; the certified volume is off for a \
         reason discretization does not explain"
    );
}

/// **The flip this unit exists for**: a FULL-TURN helical sweep — a
/// complete revolution of frame roll, straight through the
/// near-antipodal band that used to collapse the chord meter — builds
/// and certifies, and its volume converges to the independent Pappus
/// oracle at second order in the station spacing.
#[test]
fn a_full_turn_helical_sweep_builds_and_certifies() {
    let coarse = helix_oracle_gap(1.0, 33);
    let fine = helix_oracle_gap(1.0, 65);
    assert_second_order("full turn", coarse, fine, 1.2e-2);
    // The analytic continuum length brackets the interpolant's from
    // above (a chord never beats its arc), and tightly.
    let analytic = std::f64::consts::TAU * (R * R + (PITCH / std::f64::consts::TAU).powi(2)).sqrt();
    let interp = chordal_length(&helix_path(1.0), 20_000);
    assert!(
        interp <= analytic && (analytic - interp) / analytic < 1e-3,
        "interpolant length {interp} vs analytic helix length {analytic}"
    );
}

/// The half-turn signature itself — the exact threshold the issue
/// filed (`≥ ~0.5 revolutions refuse`) — now builds and certifies.
#[test]
fn a_half_turn_helical_sweep_builds_and_certifies() {
    // The cap is looser than the longer rows' (measured 0.0147 at 17
    // stations vs 0.0099): a half-turn tube is short enough that its
    // two end slabs — where the roll is one-sided — carry real weight.
    let coarse = helix_oracle_gap(0.5, 17);
    let fine = helix_oracle_gap(0.5, 33);
    assert_second_order("half turn", coarse, fine, 2e-2);
}

/// Multi-revolution: two full turns of frame roll (four antipodal
/// crossings). If this row ever regresses, the frontier follow-up
/// rule applies — pin the refusal and file it.
#[test]
fn a_two_turn_helical_sweep_builds_and_certifies() {
    let coarse = helix_oracle_gap(2.0, 65);
    let fine = helix_oracle_gap(2.0, 129);
    assert_second_order("two turns", coarse, fine, 1.2e-2);
}

// ---------------------------------------------------------------------
// Orientation: every wall of a rolling chart faces out of the material
// ---------------------------------------------------------------------

/// The step off a wall, both ways, that the material-side probe takes.
/// A quarter of the profile's half-width [`H`], so an inward step from
/// a column at `u = 0.25` or `0.75` stays `0.04` clear of the two
/// neighbouring walls, and fourteen orders above the level rings'
/// measured planarity and chord error (`4e-16` and `2e-16` on these
/// fixtures — the section's walls are straight lines).
const PROBE_DELTA: f64 = 0.02;

/// **Every wall of a `turns`-revolution helical sweep faces out of the
/// material, at every level of a chart that rolls the whole way.**
///
/// The claim is the one `m5_s11_concave_sense` makes of a loft and of
/// the quarter-turn elbow: a wall's `sense_sign · (S_u × S_v)` has
/// material against it and void along it. Nothing here reads that
/// datum to decide the question — the oracle
/// (`common::orient::LevelIndex`) sees only positions off the shipped
/// charts.
///
/// The walls, the CAPS and the full tier ladder. A body whose two cap
/// bits are inverted leaves the point set untouched, every wall bit
/// honest, and every wall probe passing — measured — so the caps are
/// probed rather than argued about, and tier 3 is run rather than
/// stopped short of.
///
/// Two ANTI-VACUITY conditions, because a row that never reaches its
/// own premise passes for the wrong reason:
///
/// - **the frame really rolls.** The elbow row's form — `n(0)·n(1)`
///   near zero — is worthless here: after a whole turn the two ends
///   coincide at `cos = 1`, exactly as on a straight path. The LEVEL
///   PLANES' turn is accumulated instead, against the one law both
///   swept suites take their bar from ([`min_roll_turn`]). One WALL's
///   normal is the wrong quantity and reads far less:
///   `side_faces[0][0]`'s outward normal starts within a degree of the
///   helix axis, which is near what the minimal-rotation frame turns
///   about, so it accumulates 0.09 rad over a quarter turn while the
///   frame sweeps 90°.
///
///   The accumulation is UNSIGNED, which costs and buys something. It
///   buys catching a path that turns and comes back; it costs counting
///   the interpolated spine's wobble as turn, so the shipped bodies
///   read about 1.7% ABOVE the smooth-helix figure (6.3769 against
///   6.2705 at one turn). Harmless under a ten-per-cent lower bound —
///   but it means a body could in principle clear the bar on wobble
///   with no net turn, which is why the second condition is not
///   optional.
/// - **no fixed axis orients these level planes**, so the fixed-chord
///   index could not have answered and this row is not a restatement of
///   it. That index needs EVERY level plane's normal within
///   [`FIXED_AXIS_GUARD_COS`] of the stacking chord and refuses on the
///   first one that is not, so the quantity to assert on is the
///   MINIMUM over the levels: measured `0.0575` on the whole turn and
///   `0.0111` on the half. The maximum is nowhere near it (`0.129` and
///   `0.993`) — [`common::orient::LevelIndex`] carries the whole range,
///   and a row that read one level would read `0.0635` and prove less
///   than it looked.
fn assert_helix_walls_face_out(turns: f64, stations: usize) {
    let path = helix_path(turns);
    let place = start_place(&path);
    let profile = quad([(-H, -H), (H, -H), (H, H), (-H, H)]);
    let swept = sweep_body::<f64>(&profile, place, &path, stations, 3, Tol::witness())
        .unwrap_or_else(|e| panic!("a {turns}-turn helical sweep must build: {e:?}"));
    assert_eq!(topo::validate(&swept.body), Ok(()), "{turns} turns: tier 1");
    assert_eq!(
        topo::validate_closed(&swept.body),
        Ok(()),
        "{turns} turns: tier 2"
    );

    let index = LevelIndex::build(&swept);
    let turned = index.total_turn();
    let want = min_roll_turn(turns, R, PITCH / std::f64::consts::TAU);
    assert!(
        turned >= want,
        "{turns} turns: the level planes accumulated only {turned} rad of turn \
         along v, under the {want} this fixture exists to exercise — the chart is \
         not rolling and the rows below would pass on a straight tube"
    );

    let axis = stack_axis(&swept);
    let worst = index
        .planes()
        .iter()
        .map(|&(_, n)| (n.dot(axis) / axis.norm()).abs())
        .fold(f64::INFINITY, f64::min);
    assert!(
        worst < FIXED_AXIS_GUARD_COS,
        "{turns} turns: the level planes must NOT be orientable against the \
         stacking chord at their WORST level (cos {worst}), or the fixed-chord \
         index would answer here and this row restates it instead of reaching \
         past it"
    );

    let oracle = |q| index.contains(q);
    assert_walls_face_out(&swept, &oracle, &along_v(), PROBE_DELTA, 4);
    assert_caps_face_out(&swept, &oracle, PROBE_DELTA);
    assert_eq!(
        topo::validate::validate_geometric(&swept.body, Tol::witness()),
        Ok(()),
        "{turns} turns: tier 3"
    );
}

/// The half-turn signature — the threshold the frontier issue filed —
/// carries an honest orientation bit on every wall.
#[test]
fn a_half_turn_helical_sweeps_walls_face_out() {
    assert_helix_walls_face_out(0.5, 17);
}

/// A whole revolution of frame roll: the ends of the chart coincide in
/// direction, and the level planes have swept a complete cone.
#[test]
fn a_full_turn_helical_sweeps_walls_face_out() {
    assert_helix_walls_face_out(1.0, 33);
}

/// Two revolutions — the row where a query point lies on as many as
/// nine level planes at once, which is the configuration the loft
/// corpus's single-root index has no answer for.
#[test]
fn a_two_turn_helical_sweeps_walls_face_out() {
    assert_helix_walls_face_out(2.0, 65);
}

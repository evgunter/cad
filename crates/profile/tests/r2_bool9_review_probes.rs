//! Independent-review probes for the RawLoop demotion (review round 2).
//!
//! Four questions the unit's own rows leave open, each executed here
//! rather than reasoned about:
//!
//! 1. `embed` is claimed to reproduce the two retired per-site walks
//!    "bit for bit". The unit's receipt runs at `f64`, where the
//!    crossing is the identity — so it measures the walk and not the
//!    conversion. These rows re-spell BOTH retired walks verbatim and
//!    compare, on a loop with arcs, bulges, declared joints and a
//!    reversed orientation, at `f64` and (under `--features interval`)
//!    at the certified interval scalar.
//! 2. The door census's writer pattern is a fixed needle list. This
//!    row replays that list against a call spelling the tree actually
//!    uses and shows what it cannot see.
//! 3. The lift's new value-equal census row has no ceiling of its own,
//!    while the module header states that every value-equal row is
//!    pinned against one.
//! 4. The widened lift is asked, over a family, whether it can hand
//!    back a program that replays to a DIFFERENT table without saying
//!    so.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::rounded_rect;
use geom_core::{Point2, Real, Tol};
use profile::{
    Fidelity, LiftOutcome, Open, ProfileLoop, ProfileVertex, RawLoop, Start, lift_checked,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A loop that is NOT the identity-shaped fixture: two fillet arcs
/// with nonzero bulges of both signs, two declared joints, a stray
/// declaration order that is not sorted, and read back reversed so the
/// walk meets a different index order than it was authored in.
fn awkward() -> ProfileLoop<f64> {
    let raw: ProfileLoop<f64> = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.41421356237309503),
        ProfileVertex::new(p2(3.0, 0.25), -0.13165249758739583),
        ProfileVertex::new(p2(2.5, 2.0), 0.0),
        ProfileVertex::new(p2(0.125, 1.75), 0.0),
    ])
    .with_tangent_joints(vec![2, 0]);
    raw.reversed()
}

// ------------------------------------------------------------------
// 1. The materialization door against the walks it replaced
// ------------------------------------------------------------------

/// `crates/sweep/src/loft.rs::end_profile`'s retired walk, verbatim.
fn loft_walk<T: Real>(lp: &ProfileLoop<f64>) -> ProfileLoop<T> {
    <ProfileLoop<T> as RawLoop<T>>::new(
        lp.vertices()
            .iter()
            .map(|v| ProfileVertex::new(v.pos().map(T::from_f64), T::from_f64(v.bulge())))
            .collect(),
    )
    .with_tangent_joints(lp.tangent_joints().to_vec())
}

/// `crates/editor-core/src/eval/anchor.rs::embed_profile`'s retired
/// walk, verbatim — note it spelled the position crossing out
/// coordinate by coordinate rather than through `Point2::map`.
fn anchor_walk<T: Real>(lp: &ProfileLoop<f64>) -> ProfileLoop<T> {
    <ProfileLoop<T> as RawLoop<T>>::new(
        lp.vertices()
            .iter()
            .map(|vx| {
                ProfileVertex::new(
                    Point2::new(T::from_f64(vx.pos().x), T::from_f64(vx.pos().y)),
                    T::from_f64(vx.bulge()),
                )
            })
            .collect(),
    )
    .with_tangent_joints(lp.tangent_joints().to_vec())
}

fn same_bits(a: &ProfileLoop<f64>, b: &ProfileLoop<f64>, what: &str) {
    assert_eq!(a.vertices().len(), b.vertices().len(), "{what}: length");
    for (i, (x, y)) in a.vertices().iter().zip(b.vertices().iter()).enumerate() {
        assert_eq!(x.pos().x.to_bits(), y.pos().x.to_bits(), "{what}: v{i}.x");
        assert_eq!(x.pos().y.to_bits(), y.pos().y.to_bits(), "{what}: v{i}.y");
        assert_eq!(
            x.bulge().to_bits(),
            y.bulge().to_bits(),
            "{what}: v{i}.bulge"
        );
    }
    assert_eq!(a.tangent_joints(), b.tangent_joints(), "{what}: joints");
}

/// **The door reproduces BOTH retired walks, on a loop that could tell
/// them apart.** The unit's own receipt uses an all-tangent stadium;
/// this one carries bulges of both signs, an unsorted declaration list
/// and a reversed index order, so a walk that sorted, re-derived or
/// re-indexed anything would show here.
#[test]
fn r2_embed_is_both_retired_walks_bit_for_bit_at_f64() {
    let src = awkward();
    let door: ProfileLoop<f64> = src.embed();
    same_bits(&loft_walk::<f64>(&src), &door, "loft");
    same_bits(&anchor_walk::<f64>(&src), &door, "anchor");
    // And the door is the identity at f64, which is the premise the
    // unit's receipt rests on.
    same_bits(&src, &door, "identity");
}

/// The declaration list travels as DATA — order preserved, duplicates
/// preserved, out-of-range preserved. The unit's row shows one
/// out-of-range index; this shows the door does not sort or dedupe
/// either, which a `BTreeSet`-shaped rewrite of `embed` would break
/// while still passing that row.
#[test]
fn r2_embed_carries_the_declaration_list_unnormalised() {
    let odd: ProfileLoop<f64> =
        <ProfileLoop<f64> as RawLoop<f64>>::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0)])
            .with_tangent_joints(vec![2, 0, 2, 9]);
    let crossed: ProfileLoop<f64> = odd.embed();
    assert_eq!(crossed.tangent_joints(), [2, 0, 2, 9]);
}

/// The same comparison at the CERTIFIED INTERVAL scalar, where
/// `from_f64` is no longer the identity — the arm the unit's receipt
/// does not reach, and the one both production callers actually use
/// (`loft`'s `Decide` scalar, `embed_profile`'s evaluation scalar).
#[cfg(feature = "interval")]
#[test]
fn r2_embed_is_both_retired_walks_at_the_interval_scalar() {
    use geom_core::{Bounds, Interval};

    let src = awkward();
    let door: ProfileLoop<Interval> = src.embed();
    let loft = loft_walk::<Interval>(&src);
    let anchor = anchor_walk::<Interval>(&src);

    assert_eq!(door.vertices().len(), src.vertices().len());
    for (i, v) in door.vertices().iter().enumerate() {
        for (name, other) in [("loft", &loft), ("anchor", &anchor)] {
            let w = other.vertices()[i];
            assert_eq!(
                v.pos().x.lo().to_bits(),
                w.pos().x.lo().to_bits(),
                "{name} v{i}.x.lo"
            );
            assert_eq!(
                v.pos().x.hi().to_bits(),
                w.pos().x.hi().to_bits(),
                "{name} v{i}.x.hi"
            );
            assert_eq!(
                v.pos().y.lo().to_bits(),
                w.pos().y.lo().to_bits(),
                "{name} v{i}.y.lo"
            );
            assert_eq!(
                v.pos().y.hi().to_bits(),
                w.pos().y.hi().to_bits(),
                "{name} v{i}.y.hi"
            );
            assert_eq!(
                v.bulge().lo().to_bits(),
                w.bulge().lo().to_bits(),
                "{name} v{i}.b.lo"
            );
            assert_eq!(
                v.bulge().hi().to_bits(),
                w.bulge().hi().to_bits(),
                "{name} v{i}.b.hi"
            );
        }
        // And the crossing is exact: every interval is a point.
        assert_eq!(
            v.pos().x.lo().to_bits(),
            v.pos().x.hi().to_bits(),
            "v{i}.x is thin"
        );
        assert_eq!(
            v.bulge().lo().to_bits(),
            v.bulge().hi().to_bits(),
            "v{i}.b is thin"
        );
    }
    assert_eq!(door.tangent_joints(), loft.tangent_joints());
    assert_eq!(door.tangent_joints(), anchor.tangent_joints());
}

// ------------------------------------------------------------------
// 2. What the door census's writer pattern cannot see
// ------------------------------------------------------------------

/// `raw_door_census::is_writer`'s needle list, replayed verbatim.
fn census_is_writer(line: &str) -> bool {
    let squashed: String = line.chars().filter(|c| !c.is_whitespace()).collect();
    [
        "RawLoop::new(",
        "RawLoop::polygon(",
        "ProfileLoop::new(",
        "ProfileLoop::polygon(",
        ".with_tangent_joints(",
    ]
    .iter()
    .any(|m| squashed.contains(m))
}

/// **The census's needle list is blind to the qualified-call spelling,
/// which is the tree's other live spelling of the same door.**
///
/// The lines below are lifted from the tree as it stands: the first
/// from `crates/mesh/src/curved.rs` (a production `src/` file, benign
/// today only because the call sits inside a `#[cfg(test)] mod`), the
/// second from this unit's own `bool9_probes.rs`. Both mint a vertex
/// table; neither matches. A type alias is the same hole one step
/// further on.
#[test]
fn r2_the_door_censuss_writer_pattern_misses_the_qualified_spelling() {
    let missed = [
        "        let lp = <ProfileLoop<f64> as RawLoop<f64>>::polygon([",
        "    let odd: ProfileLoop<f64> = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![",
        "    type Tbl = ProfileLoop<f64>;",
        "    let t: Tbl = Tbl::new(Vec::new());",
    ];
    for line in missed {
        assert!(
            !census_is_writer(line),
            "the census would have caught this after all: {line}"
        );
    }
    // The control: the spelling it does catch.
    assert!(census_is_writer("    ProfileLoop::new(Vec::new())"));
}

// ------------------------------------------------------------------
// 3. The new value-equal census row and its (absent) ceiling
// ------------------------------------------------------------------

/// **The unit's new `Value` row is held only by the coarse bucket.**
///
/// `lift.rs`'s header says "the tool's actual accuracy claim is pinned
/// independently by the census suite, which asserts each value-equal
/// row's `worst_abs` against its own tight ceiling". Two of the three
/// value-equal rows have one (`bracket` < 1e-12,
/// `the_carrier_phase_is_the_surviving_angle_residue` < 1e-15). The
/// third — `rounded_rect`, added by this unit — has none, so what
/// actually holds it is `VALUE_EQUAL_ABS` = 1e-12. This row measures
/// the gap: the observed figure and the only ceiling above it.
#[test]
fn r2_the_new_value_row_is_held_only_by_the_coarse_bucket() {
    match lift_checked(&rounded_rect(4.0, 3.0, 0.5), Tol::witness()) {
        LiftOutcome::Lifted {
            fidelity,
            worst_abs,
            worst_ulps,
            ..
        } => {
            assert_eq!(fidelity, Fidelity::ValueEqual);
            println!("r2: rounded_rect worst_abs = {worst_abs:e}, worst_ulps = {worst_ulps}");
            // As measured on this head.
            assert!(worst_abs < 1e-14, "the observed figure: {worst_abs:e}");
            // And the headroom nothing in the suite closes: this row
            // could degrade by two orders of magnitude and every
            // assertion in `lift_census` would stay green.
            assert!(
                worst_abs > 1e-15,
                "if this ever drops below the carrier-phase ceiling the \
                 gap this row reports has closed on its own: {worst_abs:e}"
            );
        }
        other => panic!("rounded_rect should lift value-equal: {other:?}"),
    }
}

// ------------------------------------------------------------------
// 4. Can the widened lift author a different table quietly?
// ------------------------------------------------------------------

/// One rung of the ladder: a triangle whose fourth vertex sits `off`
/// away from the exactly-collinear subdivision point on the closing
/// side, with the closing joint declared or not.
fn rung(off: f64, declared: bool) -> ProfileLoop<f64> {
    let lp: ProfileLoop<f64> = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(1.0, 1.0),
        p2(0.5 + off, 0.5),
    ]);
    if declared {
        lp.with_tangent_joints(vec![3])
    } else {
        lp
    }
}

/// **The widening, swept — and issue 433's two questions, executed as
/// a pair.**
///
/// `DeclaredJointBeforeClosingLine` and `AllJointsDeclared` both
/// retired, so a declared closing joint now spells `continue_to`,
/// whose target the DRIVER measures against the departing ray. Two
/// worries, one sweep, over a ladder of offsets from the closing
/// side's exact subdivision point:
///
/// 1. **The widening must not author quietly.** Every declared rung
///    must be a faithful lift, the driver's refusal, or a `Mismatch`
///    that SAYS the table moved. A `Lifted` whose table does not match
///    is the state `lift_checked`'s differential exists to make
///    impossible, and the widening opened exactly this family.
/// 2. **The declaration is what asks the authoring question.** The
///    undeclared twin of each rung must never lift where the declared
///    one refuses — the declaration may only ever open spellings, and
///    an undeclared zero-turn seam is the junction the lattice refuses
///    while `validate` accepts the identical table as data. The
///    assertion is taken at the EXACTLY collinear rung, where a zero
///    turn is tangent at any tolerance, so the row means the same
///    thing at every eps; the wider counts are printed, since which
///    rungs fall inside the band is an eps question. (At the default
///    tolerance the split is total: 23 declared lifts, 0 undeclared.
///    At 1e-12 the far rungs invert, because a FALSE declaration
///    correctly CLOSES a spelling — which is the widening's own
///    contract, not a violation of it.)
#[test]
fn r2_the_widened_lift_never_lifts_a_loop_whose_table_moved() {
    let tol = Tol::witness();
    let (mut d_lift, mut d_wall, mut u_lift, mut u_wall, mut mism) = (0, 0, 0, 0, 0);
    for k in 0..24 {
        let off = if k == 0 {
            0.0
        } else {
            1e-16 * f64::powi(2.0, k)
        };
        let mut lifted_here = [false; 2];
        for (slot, declared) in [(0usize, true), (1usize, false)] {
            match lift_checked(&rung(off, declared), tol) {
                LiftOutcome::Lifted {
                    worst_abs, program, ..
                } => {
                    lifted_here[slot] = true;
                    if declared {
                        d_lift += 1;
                    } else {
                        u_lift += 1;
                    }
                    assert!(
                        worst_abs < 1e-12,
                        "k={k} declared={declared}: a lift is only a lift if the \
                         table survives it: {worst_abs:e} {program:?}"
                    );
                }
                LiftOutcome::ReplayRefused { .. } => {
                    if declared {
                        d_wall += 1;
                    } else {
                        u_wall += 1;
                    }
                }
                LiftOutcome::Refused(r) => {
                    panic!("k={k} declared={declared}: unexpected structural wall {r:?}")
                }
                LiftOutcome::Mismatch {
                    worst_abs, program, ..
                } => {
                    // Reported, never silent — which is the contract.
                    mism += 1;
                    println!("r2: k={k} declared={declared} MISMATCH {worst_abs:e} {program:?}");
                }
            }
        }
        if k == 0 {
            // The EXACTLY collinear rung: a zero turn is tangent at
            // any tolerance, so this pair is eps-robust and it is
            // issue 433's subject in one line. Declared, the lattice
            // has a spelling; undeclared, it walls — and `validate`
            // accepts both tables as data either way.
            assert!(lifted_here[0], "the declared exact seam must lift");
            assert!(!lifted_here[1], "the undeclared exact seam must wall");
        }
    }
    println!(
        "r2: ladder — declared: lifted {d_lift}, walled {d_wall}; \
         undeclared: lifted {u_lift}, walled {u_wall}; mismatch {mism}"
    );
    assert_eq!(mism, 0, "no rung authored a different table");
    assert_eq!(d_lift + d_wall, 24, "every declared rung got an outcome");
    assert_eq!(u_lift + u_wall, 24, "every undeclared rung got an outcome");
    assert!(d_lift > 0 && u_wall > 0);
}

/// The lattice is the only authoring door a shipped build has, and it
/// is the one the unit's doctests now use. This replays the square
/// from `ProfileLoop`'s own doctest and pins that the lattice-authored
/// table is bit-for-bit the one the retired `polygon` sugar produced —
/// the migration receipt the unit reports per-site, taken here on the
/// door the façade actually presents.
#[test]
fn r2_the_lattice_square_is_the_retired_polygon_sugars_table() {
    let tol = Tol::witness();
    let pts = [p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)];
    let authored: ProfileLoop<f64> = Open
        .at(pts[0])
        .line_to(pts[1], tol)
        .unwrap()
        .line_to(pts[2], tol)
        .unwrap()
        .line_to(pts[3], tol)
        .unwrap()
        .line_to(Start, tol)
        .unwrap()
        .into();
    let raw: ProfileLoop<f64> = <ProfileLoop<f64> as RawLoop<f64>>::polygon(pts);
    same_bits(&raw, &authored, "lattice square vs polygon sugar");
}

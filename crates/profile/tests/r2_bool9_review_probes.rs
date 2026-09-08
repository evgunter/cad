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

/// **The widening, swept.** `DeclaredJointBeforeClosingLine` and
/// `AllJointsDeclared` both retired, so a declared closing joint now
/// spells `continue_to`, whose target the DRIVER measures against the
/// departing ray. The worry is a loop the driver ACCEPTS while the
/// program replays to a different table.
///
/// The sweep walks a triangle whose subdivision vertex is nudged off
/// the collinear line by a widening ladder, with the joint declared
/// throughout — so the declaration is exactly true at 0 and
/// progressively falser after it. Every outcome must be one of: a
/// faithful lift, the driver's refusal, or a `Mismatch` that SAYS the
/// table moved. What must never happen is `Lifted` with a table that
/// does not match, which is the state `lift_checked`'s differential
/// exists to make impossible — this row is that claim executed over
/// the family the widening opened.
#[test]
fn r2_the_widened_lift_never_lifts_a_loop_whose_table_moved() {
    let mut lifted = 0;
    let mut refused = 0;
    let mut mismatched = 0;
    for k in 0..24 {
        let off = if k == 0 {
            0.0
        } else {
            1e-16 * f64::powi(2.0, k)
        };
        let loop_: ProfileLoop<f64> = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
            p2(0.0, 0.0),
            p2(2.0, 0.0),
            p2(1.0, 1.0),
            p2(0.5 + off, 0.5),
        ])
        .with_tangent_joints(vec![3]);
        match lift_checked(&loop_, Tol::witness()) {
            LiftOutcome::Lifted {
                worst_abs, program, ..
            } => {
                lifted += 1;
                assert!(
                    worst_abs < 1e-12,
                    "k={k}: a lift is only a lift if the table survives it: \
                     {worst_abs:e} {program:?}"
                );
            }
            LiftOutcome::ReplayRefused { .. } => refused += 1,
            LiftOutcome::Refused(r) => panic!("k={k}: unexpected structural wall {r:?}"),
            LiftOutcome::Mismatch {
                worst_abs, program, ..
            } => {
                // Reported, not silent — which is the contract. Counted
                // so the row says whether the family reaches it at all.
                mismatched += 1;
                println!("r2: k={k} MISMATCH worst_abs={worst_abs:e} {program:?}");
            }
        }
    }
    println!("r2: widened-lift sweep — lifted {lifted}, refused {refused}, mismatch {mismatched}");
    assert!(
        lifted > 0,
        "the exactly-collinear end of the ladder must lift"
    );
    assert!(
        refused > 0,
        "and the far end must reach the driver's wall rather than lifting"
    );
}

/// **The undeclared twin, and issue 433's original subject executed.**
///
/// The identical ladder with the declaration REMOVED: not one rung
/// lifts. Every one reaches the driver's undeclared-zero-turn-junction
/// wall (or its ambiguity band at the far end), while the declared
/// ladder above lifts 23 of the same 24. `validate` accepts every loop
/// in both families as data. That is the whole of "two questions, not
/// one rule with two answers", measured rather than asserted in
/// prose — and it is what the demotion buys: the authoring question is
/// only ever asked at the lattice, and the lattice is now the only
/// door a shipped build has.
#[test]
fn r2_the_widening_is_reached_by_the_declaration_and_never_by_default() {
    let mut refused = 0;
    let mut lifted = 0;
    for k in 0..24 {
        let off = if k == 0 {
            0.0
        } else {
            1e-16 * f64::powi(2.0, k)
        };
        let loop_: ProfileLoop<f64> = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
            p2(0.0, 0.0),
            p2(2.0, 0.0),
            p2(1.0, 1.0),
            p2(0.5 + off, 0.5),
        ]);
        match lift_checked(&loop_, Tol::witness()) {
            LiftOutcome::Lifted {
                fidelity,
                worst_ulps,
                ..
            } => {
                lifted += 1;
                assert_eq!(fidelity, Fidelity::BitIdentical, "k={k}");
                assert_eq!(worst_ulps, 0, "k={k}");
            }
            LiftOutcome::ReplayRefused { error, .. } => {
                refused += 1;
                let text = format!("{error}");
                println!(
                    "r2: undeclared k={k} walls: {}",
                    &text[..text.len().min(90)]
                );
                assert!(
                    text.contains("this junction is tangent")
                        || text.contains("path_junction_turn"),
                    "k={k}: the only walls this family may reach are the \
                     undeclared zero-turn junction and its ambiguity band: {error}"
                );
            }
            other => panic!("k={k}: an undeclared polygon must lift or refuse: {other:?}"),
        }
    }
    println!("r2: undeclared sweep — refused {refused}, lifted {lifted}");
    // The point of the pair: the DECLARED ladder above lifts at the
    // exactly-collinear rung; this one WALLS there, in the driver's
    // own zero-turn-junction words. Two questions, and the
    // declaration is which one is being asked — which is issue 433's
    // subject, executed.
    assert_eq!(lifted, 0, "not one rung of the undeclared ladder lifts");
    assert_eq!(refused, 24, "every rung reaches the driver's junction wall");
}

/// The lattice is still the only authoring door a shipped build has,
/// and it is the one the doctests now use — this replays the square
/// from `ProfileLoop`'s own doctest and pins that the lattice-authored
/// table is the one the retired `polygon` sugar produced.
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

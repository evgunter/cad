//! **The exhaustiveness receipt's metre reading is the bare
//! arithmetic, per lane.**
//!
//! `Exhaustiveness` and `ExhaustivenessRefusal` — the receipt and the
//! payload of `SsiError::ExhaustivenessInconclusive` — state their
//! lengths in the units their own lane measured cells in, and carry
//! the lane that says which. The metre readings (`floor_meters`,
//! `cell_width_meters`) are one `match` over that tag: the ℝ³ arm hands the value back untouched and the chart arm is
//! `SupSpeed::to_meters`, one multiply. These rows make that a checked
//! claim rather than a sentence — they compare **bits**, not `==`,
//! because `==` cannot see a signed zero and calls every NaN unequal,
//! and both are values a collapsed rate or a poisoned box can put on a
//! receipt.
//!
//! **Poison flows through, not around.** A rate of `0`, `-0`, `∞` or
//! NaN reaches these readings exactly as the bare product would leave
//! it. The tag is a tag, and a reading that sanitized a collapsed
//! carrier would hide from a caller the very thing the refusal exists
//! to show.
//!
//! The `Display` rows are the other half: a length on a receipt is
//! meaningless without its lane and its unit word, so the lane word
//! and the unit word are the text's claim and are asserted on both
//! lanes, for both types. What they cannot see is the *number* being
//! right, which is what the bit rows above and the fixture rows in
//! `m5_pr7_ssi.rs` are for.
//!
//! **What no row here can see**, stated because it looks like a gap:
//! these rows construct receipts directly, so they say nothing about
//! which lane a given SSI door actually reports. That is the fixture
//! rows' claim — `the_floor_clamped_chart_run_refuses_typed_with_a_banked_tube_set`
//! and `the_floor_clamped_planted_fixture_refuses_typed` each drive a
//! real operation and read its lane's text back out.

#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use geom_brep::{ExhaustLane, Exhaustiveness, ExhaustivenessRefusal, SsiError};
use geom_core::SupSpeed;

/// Every value a rate or a length can be, including the ones only a
/// bit comparison can tell apart. Written down rather than drawn: a
/// witness set, not a counterexample search
/// (`memories/test-suite-cost.md`), so it is the same set every run.
fn sample() -> Vec<f64> {
    vec![
        0.0,
        -0.0,
        1.0,
        -1.0,
        0.5,
        3.0,
        1e-9,
        1e9,
        f64::MIN_POSITIVE,
        f64::MIN_POSITIVE / 2.0, // subnormal
        f64::MAX,
        f64::MIN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
    ]
}

/// A receipt on `lane` whose floor is `floor`; the counts are a fixed
/// arrangement that satisfies the receipt's own add-up identity, so
/// nothing here depends on them.
fn receipt(lane: ExhaustLane, floor: f64) -> Exhaustiveness {
    Exhaustiveness {
        lane,
        examined: 7,
        excluded: 3,
        accounted: 2,
        refined: 2,
        max_depth: 4,
        floor,
    }
}

fn refusal(lane: ExhaustLane, cell_width: f64, floor: f64) -> ExhaustivenessRefusal {
    ExhaustivenessRefusal {
        lane,
        cell_width,
        floor,
        examined: 11,
    }
}

/// The same refusal as the error a caller actually catches.
fn refusal_error(lane: ExhaustLane, cell_width: f64, floor: f64) -> SsiError {
    SsiError::ExhaustivenessInconclusive(refusal(lane, cell_width, floor))
}

/// **The ℝ³ lane's reading is the value itself** — no rate exists on
/// that lane, so a metre reading that multiplied by anything at all
/// would be inventing one.
#[test]
fn the_r3_lane_reads_its_own_floor_back_bit_for_bit() {
    for floor in sample() {
        let e = receipt(ExhaustLane::R3, floor);
        assert_eq!(
            e.floor_meters().to_bits(),
            floor.to_bits(),
            "the ℝ³ receipt's floor is already meters: {floor:e}"
        );
        let err = refusal(ExhaustLane::R3, floor, floor);
        assert_eq!(err.cell_width_meters().to_bits(), floor.to_bits());
        assert_eq!(err.floor_meters().to_bits(), floor.to_bits());
    }
}

/// **The chart lane's reading is the bare product**, over every
/// rate/length pair in the sample — `floor * speed`, one multiply, the
/// same bits.
#[test]
fn the_chart_lane_reads_the_bare_product_bit_for_bit() {
    for rate in sample() {
        let lane = ExhaustLane::Chart {
            speed: SupSpeed::new(rate),
        };
        for x in sample() {
            let bare = x * rate;
            assert_eq!(
                receipt(lane, x).floor_meters().to_bits(),
                bare.to_bits(),
                "floor {x:e} at rate {rate:e}"
            );
            let err = refusal(lane, x, x);
            assert_eq!(
                err.cell_width_meters().to_bits(),
                bare.to_bits(),
                "cell width {x:e} at rate {rate:e}"
            );
            assert_eq!(err.floor_meters().to_bits(), bare.to_bits());
        }
    }
}

/// **Both texts name their lane and their unit word.** A caller acting
/// on a refusal reads the text, and a width of `2.5` is a different
/// claim on each lane.
#[test]
fn both_texts_name_the_lane_and_the_unit() {
    let r3 = format!("{}", receipt(ExhaustLane::R3, 0.25));
    assert!(r3.contains("on the ℝ³ lane"), "{r3}");
    assert!(r3.contains("2.5e-1 m"), "{r3}");
    assert!(!r3.contains("chart"), "{r3}");

    let chart = ExhaustLane::Chart {
        speed: SupSpeed::new(2.0),
    };
    let ch = format!("{}", receipt(chart, 0.25));
    assert!(ch.contains("on the chart lane"), "{ch}");
    assert!(ch.contains("2.5e-1 chart units"), "{ch}");
    // The meters reading and the rate that produced it, beside the
    // chart-unit floor: this is what makes a receipt diagnosable
    // without the caller holding the speed. Anchored on the opening
    // parenthesis and the word after the unit, because a bare
    // `"5e-1 m"` is a substring of `"2.5e-1 m"` — which is exactly the
    // text a reading that handed the chart-unit number back as metres
    // would print.
    assert!(ch.contains("(5e-1 m at"), "{ch}");
    assert!(ch.contains("2e0 m per chart unit"), "{ch}");
    // The chart lane's negatives, the counterpart of the ℝ³ rows':
    // this text names no other lane, and it never states the
    // chart-unit floor as a length in metres.
    assert!(!ch.contains("ℝ³"), "{ch}");
    assert!(!ch.contains("2.5e-1 m"), "{ch}");

    let r3e = format!("{}", refusal_error(ExhaustLane::R3, 0.125, 0.25));
    assert!(r3e.contains("on the ℝ³ lane"), "{r3e}");
    assert!(r3e.contains("width 1.25e-1 m"), "{r3e}");
    assert!(r3e.contains("floor 2.5e-1 m"), "{r3e}");
    assert!(!r3e.contains("chart"), "{r3e}");

    let che = format!("{}", refusal_error(chart, 0.125, 0.25));
    assert!(che.contains("on the chart lane"), "{che}");
    assert!(
        che.contains("width 1.25e-1 chart units (2.5e-1 m)"),
        "{che}"
    );
    // The refusal names the rate too, once, on its floor — the reading
    // a caller needs to tell a fine floor from a slow chart.
    assert!(
        che.contains(
            "floor 2.5e-1 chart units (5e-1 m at a certified chart speed of 2e0 m per \
             chart unit)"
        ),
        "{che}"
    );
    assert!(!che.contains("ℝ³"), "{che}");
    assert!(!che.contains("(1.25e-1 m"), "{che}");
}

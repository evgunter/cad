//! Unit tests: table pins, construction identities, formatter pins,
//! and the crate-local half of the parse/format round-trip (the full
//! pin through the real text parser lives in editor-core's suite).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use proptest::prelude::*;

use crate::{
    Angle, CENTI, CM, DEG, FmtQuantityError, IN, Length, M, MILLI, MM, RAD, UNITS, UnitDef,
    UnitQuantity, fmt_angle, fmt_length, unit_by_symbol,
};

#[test]
fn the_unit_table_is_the_whole_closed_set_and_reads_as_data() {
    let symbols: Vec<&str> = UNITS.iter().map(|u| u.symbol()).collect();
    assert_eq!(symbols, ["mm", "cm", "m", "in", "deg", "rad"]);
    assert_eq!(MM.factor(), MILLI);
    assert_eq!(MILLI, 1e-3);
    assert_eq!(CM.factor(), CENTI);
    assert_eq!(CENTI, 1e-2);
    assert_eq!(M.factor(), 1.0);
    assert_eq!(IN.factor(), 0.0254);
    assert_eq!(RAD.factor(), 1.0);
    // fl(π/180), pinned by bits: DEG is inexact by nature and this
    // constant is its identity.
    assert_eq!(DEG.factor(), 0.017_453_292_519_943_295_f64);
    let mm = unit_by_symbol("mm").expect("mm row");
    assert_eq!(mm.quantity(), UnitQuantity::Length);
    assert_eq!(mm.factor(), MILLI);
    assert_eq!(
        unit_by_symbol("deg").expect("deg row").quantity(),
        UnitQuantity::Angle
    );
    assert_eq!(unit_by_symbol("furlong"), None);
    assert_eq!(
        unit_by_symbol("MM"),
        None,
        "symbols are case-sensitive data"
    );
}

#[test]
fn unit_constant_construction_lands_on_canonical_units() {
    assert_eq!((25.0 * MM).meters(), 0.025);
    assert_eq!((25.0 * MM).meters(), (MM * 25.0).meters());
    assert_eq!((1.0 * M).meters(), 1.0);
    assert_eq!((1.0 * IN).meters(), 0.0254);
    assert_eq!((90.0 * DEG).radians(), 90.0 * DEG.factor());
    assert_eq!((1.5 * RAD).radians(), 1.5);
    // The heatsink migration's three literals: prefixed-metre decimal
    // values whose f64 multiply lands on the exact dyadic the tour
    // hand-wrote in meters (the SAID change's bit-identity basis).
    assert_eq!((250.0 * MM).meters(), 0.25);
    assert_eq!((812.5 * MM).meters(), 0.8125);
    assert_eq!((312.5 * MM).meters(), 0.3125);
    // Display-side conversion.
    assert_eq!((25.0 * MM).in_unit(MM), 25.0);
}

#[test]
fn quantity_arithmetic_is_the_infallible_subset() {
    assert_eq!((1.0 * M + 50.0 * CM).meters(), 1.5);
    assert_eq!((1.0 * M - 250.0 * MM).meters(), 0.75);
    assert_eq!((-(1.0 * M)).meters(), -1.0);
    assert_eq!((2.0 * (1.0 * M) * 3.0).meters(), 6.0);
    assert_eq!(((1.0 * M) / 4.0).meters(), 0.25);
    assert!(90.0 * DEG < 2.0 * RAD);
    assert_eq!(crate::Count::new(5).get(), 5);
}

#[test]
fn formatter_pins_the_spec_surface_shape() {
    assert_eq!(fmt_length(0.25, MM).unwrap(), "250 mm");
    assert_eq!(fmt_length(0.025, MM).unwrap(), "25 mm");
    assert_eq!(fmt_length(0.25, M).unwrap(), "0.25 m");
    assert_eq!(fmt_length(0.0254, IN).unwrap(), "1 in");
    assert_eq!(fmt_length(-0.25, MM).unwrap(), "-250 mm");
    assert_eq!(fmt_length(0.0, MM).unwrap(), "0 mm");
    assert_eq!(fmt_angle(DEG.factor(), DEG).unwrap(), "1 deg");
    assert_eq!(fmt_angle(1.5, RAD).unwrap(), "1.5 rad");
    assert!(matches!(
        fmt_length(f64::NAN, MM),
        Err(FmtQuantityError::NonFinite { value }) if value.is_nan()
    ));
    assert!(matches!(
        fmt_angle(f64::INFINITY, DEG),
        Err(FmtQuantityError::NonFinite { value }) if value == f64::INFINITY
    ));
}

/// The crate-local mirror of the parser's literal semantics: decimal
/// digits via `from_str`, then ONE f64 multiply by the factor (a
/// leading `-` is the parser's unary minus: negate after). Kept in
/// lockstep with `editor-core`'s parser by the round-trip suite there.
fn parse_back(text: &str, expect_symbol_or_canonical: [&str; 2]) -> f64 {
    let (num, sym) = text.split_once(' ').expect("value then symbol");
    assert!(expect_symbol_or_canonical.contains(&sym), "suffix {sym:?}");
    let factor = unit_by_symbol(sym).expect("table row").factor();
    let (mag, neg) = match num.strip_prefix('-') {
        Some(m) => (m, true),
        None => (num, false),
    };
    let v: f64 = mag.parse().expect("digits");
    let v = v * factor;
    if neg { -v } else { v }
}

proptest! {
    #[test]
    // "Sampled" is honest (NOTE-2 of the PR #267 review): the DOMAIN
    // is all finite f64 × all six units; a run samples proptest's
    // configured case count from it.
    fn fmt_round_trip_bit_exact_sampled_over_finite_values_and_units(
        value in proptest::num::f64::ANY.prop_filter("finite", |v| v.is_finite()),
        unit_idx in 0usize..6,
    ) {
        let u = UNITS[unit_idx];
        let (text, canonical) = match u.quantity() {
            UnitQuantity::Length => (
                fmt_length(value, u.as_length().expect("a Length row has a length view"))
                    .unwrap(),
                "m",
            ),
            UnitQuantity::Angle => (
                fmt_angle(value, u.as_angle().expect("an Angle row has an angle view"))
                    .unwrap(),
                "rad",
            ),
        };
        let back = parse_back(&text, [u.symbol(), canonical]);
        prop_assert_eq!(back.to_bits(), value.to_bits(), "text {:?}", text);
    }
}

#[test]
fn values_with_no_preimage_in_the_asked_unit_fall_back_to_canonical() {
    // Walk quotients d upward from 1024 (the census'd skip region for
    // MM: ulp(d)·10⁻³ > ulp(d·10⁻³) there) until fl(d·f) jumps 2 ulps
    // — the skipped value has NO mm preimage, so the formatter must
    // fall back to meters, bit-exactly.
    let f = MM.factor();
    let mut d = 1024.0f64;
    for _ in 0..200_000 {
        let x1 = d * f;
        let x2 = d.next_up() * f;
        if x2 > x1.next_up() {
            let skipped = x1.next_up();
            let text = fmt_length(skipped, MM).unwrap();
            assert!(
                text.ends_with(" m"),
                "expected canonical fallback, got {text:?}"
            );
            assert_eq!(parse_back(&text, ["m", "m"]).to_bits(), skipped.to_bits());
            return;
        }
        d = d.next_up();
    }
    panic!("no 2-ulp step found in 200k quotients — fallback untested");
}

/// The seal's own claim, as an assertion rather than as prose: the
/// symbol DETERMINES the row. Every row a caller can obtain comes from
/// the table, so resolving its symbol must return the same row — which
/// is the premise `editor-core`'s symbol-only `UnitSym::from_def`
/// lookup relies on (issue #650).
///
/// It goes red if the table ever gains two rows sharing a symbol, which
/// is the one way a sealed `UnitDef` could still be ambiguous.
#[test]
fn the_symbol_determines_the_row() {
    for row in UNITS {
        let resolved = unit_by_symbol(row.symbol()).expect("a table row resolves by its symbol");
        assert_eq!(
            resolved,
            row,
            "unit_by_symbol({:?}) is not the row it came from",
            row.symbol()
        );
    }
}

/// **The #669 seal, as the property it exists to buy: every typed view
/// a caller can hold pairs its symbol with the TABLE's factor.**
///
/// The mint is closed by the type — `LengthUnit`/`AngleUnit` wrap a
/// `UnitDef` and have no public constructor, pinned by `compile_fail`
/// doctests on the two types (a runtime test cannot assert that a
/// struct literal does not compile). What this test adds is the other
/// half: that the doors which REPLACE the literal cannot reintroduce
/// the illegal pairing, stated over the whole table so a unit added to
/// `quantity` is covered the day it lands.
///
/// Every route in is here: the six constants, `UnitDef::as_length` /
/// `as_angle`, and — for the constants — the round trip back through
/// `unit_by_symbol`. The three consequences #669 named are asserted at
/// the boundary itself rather than on the fields: the `Mul` value, the
/// `in_unit` value, and the formatter's suffix.
#[test]
fn every_obtainable_typed_view_pairs_its_symbol_with_the_tables_factor() {
    let mut lengths = 0_u32;
    let mut angles = 0_u32;
    for row in UNITS {
        match row.quantity() {
            UnitQuantity::Length => {
                let view = row.as_length().expect("a Length row has a length view");
                assert_eq!(row.as_angle(), None, "{} is not an angle", row.symbol());
                assert_eq!(view.symbol(), row.symbol());
                assert_eq!(view.factor(), row.factor());
                // The D6 boundary: 2.5 rather than 1.0, since `1.0 * f`
                // is `f` bitwise and cannot tell "applied the factor"
                // from "returned it".
                assert_eq!(
                    (2.5 * view).meters().to_bits(),
                    (2.5 * row.factor()).to_bits(),
                    "{} multiplied by the wrong factor",
                    row.symbol()
                );
                // `in_unit` DIVIDES by the table's factor. Asserted
                // against the row rather than as `x * f / f == x`,
                // which holds for any factor at all and is an exact
                // float comparison across a divide besides.
                assert_eq!(
                    Length::from_meters(2.5).in_unit(view).to_bits(),
                    (2.5_f64 / row.factor()).to_bits(),
                    "{} divided by the wrong factor",
                    row.symbol()
                );
                // The suffix names the factor that was applied — the
                // `parse(fmt(x, unit))` pin's other half.
                assert_eq!(
                    fmt_length(2.5 * row.factor(), view).unwrap(),
                    format!("2.5 {}", row.symbol())
                );
                assert_eq!(
                    unit_by_symbol(view.symbol()).and_then(UnitDef::as_length),
                    Some(view),
                    "{} does not resolve back to itself",
                    row.symbol()
                );
                lengths += 1;
            }
            UnitQuantity::Angle => {
                let view = row.as_angle().expect("an Angle row has an angle view");
                assert_eq!(row.as_length(), None, "{} is not a length", row.symbol());
                assert_eq!(view.symbol(), row.symbol());
                assert_eq!(view.factor(), row.factor());
                assert_eq!(
                    (2.5 * view).radians().to_bits(),
                    (2.5 * row.factor()).to_bits(),
                    "{} multiplied by the wrong factor",
                    row.symbol()
                );
                assert_eq!(
                    Angle::from_radians(2.5).in_unit(view).to_bits(),
                    (2.5_f64 / row.factor()).to_bits(),
                    "{} divided by the wrong factor",
                    row.symbol()
                );
                assert_eq!(
                    fmt_angle(2.5 * row.factor(), view).unwrap(),
                    format!("2.5 {}", row.symbol())
                );
                assert_eq!(
                    unit_by_symbol(view.symbol()).and_then(UnitDef::as_angle),
                    Some(view),
                    "{} does not resolve back to itself",
                    row.symbol()
                );
                angles += 1;
            }
        }
    }
    // Counts, so a table that quietly stopped being exercised cannot
    // pass as green — and so that both arms above are known to have run.
    assert_eq!(lengths, 4, "four length rows");
    assert_eq!(angles, 2, "two angle rows");
    // The six exported constants are exactly the six rows, so the loop
    // above covers all of them. What makes that a closure rather than a
    // coincidence is upstream of this assertion: a typed view is an
    // INDEX into `UNITS`, so a constant naming a unit the table does
    // not have fails to compile, and one naming a row it does have is
    // already in the loop. This row catches the remaining case — a row
    // added to the table with no constant exported for it.
    let constants: [&str; 6] = [
        MM.symbol(),
        CM.symbol(),
        M.symbol(),
        IN.symbol(),
        DEG.symbol(),
        RAD.symbol(),
    ];
    let mut tabled: Vec<&str> = UNITS.iter().map(UnitDef::symbol).collect();
    let mut named = constants.to_vec();
    tabled.sort_unstable();
    named.sort_unstable();
    assert_eq!(named, tabled, "the constants must cover exactly the table");
}

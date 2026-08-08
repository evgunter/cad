//! Unit tests: table pins, construction identities, formatter pins,
//! and the crate-local half of the parse/format round-trip (the full
//! pin through the real text parser lives in editor-core's suite).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use proptest::prelude::*;

use crate::{
    CENTI, CM, DEG, FmtQuantityError, IN, M, MILLI, MM, RAD, UNITS, UnitQuantity, fmt_angle,
    fmt_length, unit_by_symbol,
};

#[test]
fn the_unit_table_is_the_whole_closed_set_and_reads_as_data() {
    let symbols: Vec<&str> = UNITS.iter().map(|u| u.symbol).collect();
    assert_eq!(symbols, ["mm", "cm", "m", "in", "deg", "rad"]);
    assert_eq!(MM.factor, MILLI);
    assert_eq!(MILLI, 1e-3);
    assert_eq!(CM.factor, CENTI);
    assert_eq!(CENTI, 1e-2);
    assert_eq!(M.factor, 1.0);
    assert_eq!(IN.factor, 0.0254);
    assert_eq!(RAD.factor, 1.0);
    // fl(π/180), pinned by bits: DEG is inexact by nature and this
    // constant is its identity.
    assert_eq!(DEG.factor, 0.017_453_292_519_943_295_f64);
    let mm = unit_by_symbol("mm").expect("mm row");
    assert_eq!(mm.quantity, UnitQuantity::Length);
    assert_eq!(mm.factor, MILLI);
    assert_eq!(unit_by_symbol("deg").expect("deg row").quantity, UnitQuantity::Angle);
    assert_eq!(unit_by_symbol("furlong"), None);
    assert_eq!(unit_by_symbol("MM"), None, "symbols are case-sensitive data");
}

#[test]
fn unit_constant_construction_lands_on_canonical_units() {
    assert_eq!((25.0 * MM).meters(), 0.025);
    assert_eq!((25.0 * MM).meters(), (MM * 25.0).meters());
    assert_eq!((1.0 * M).meters(), 1.0);
    assert_eq!((1.0 * IN).meters(), 0.0254);
    assert_eq!((90.0 * DEG).radians(), 90.0 * DEG.factor);
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
    assert_eq!(fmt_angle(DEG.factor, DEG).unwrap(), "1 deg");
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
    let factor = unit_by_symbol(sym).expect("table row").factor;
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
    fn fmt_round_trips_bit_exactly_for_every_finite_value_and_unit(
        value in proptest::num::f64::ANY.prop_filter("finite", |v| v.is_finite()),
        unit_idx in 0usize..6,
    ) {
        let u = UNITS[unit_idx];
        let (text, canonical) = match u.quantity {
            UnitQuantity::Length => (
                fmt_length(value, crate::LengthUnit { symbol: u.symbol, factor: u.factor })
                    .unwrap(),
                "m",
            ),
            UnitQuantity::Angle => (
                fmt_angle(value, crate::AngleUnit { symbol: u.symbol, factor: u.factor })
                    .unwrap(),
                "rad",
            ),
        };
        let back = parse_back(&text, [u.symbol, canonical]);
        prop_assert_eq!(back.to_bits(), value.to_bits(), "text {:?}", text);
    }
}

#[test]
fn values_with_no_preimage_in_the_asked_unit_fall_back_to_canonical() {
    // Walk quotients d upward from 1024 (the census'd skip region for
    // MM: ulp(d)·10⁻³ > ulp(d·10⁻³) there) until fl(d·f) jumps 2 ulps
    // — the skipped value has NO mm preimage, so the formatter must
    // fall back to meters, bit-exactly.
    let f = MM.factor;
    let mut d = 1024.0f64;
    for _ in 0..200_000 {
        let x1 = d * f;
        let x2 = d.next_up() * f;
        if x2 > x1.next_up() {
            let skipped = x1.next_up();
            let text = fmt_length(skipped, MM).unwrap();
            assert!(text.ends_with(" m"), "expected canonical fallback, got {text:?}");
            assert_eq!(parse_back(&text, ["m", "m"]).to_bits(), skipped.to_bits());
            return;
        }
        d = d.next_up();
    }
    panic!("no 2-ulp step found in 200k quotients — fallback untested");
}

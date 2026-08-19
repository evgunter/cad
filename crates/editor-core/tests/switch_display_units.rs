//! **LIB-SWITCH §4g (U8b fold-in): per-literal display-unit STORAGE.**
//!
//! The full "25 mm" acceptance row — parse → literal(0.025, Length,
//! unit = mm) → persist wire form → load → format → "25 mm" — plus the
//! D7 hard rules pinned in both directions: the display unit round-trips
//! as presentation metadata, and it NEVER enters expression identity
//! (`PartialEq`, `bit_eq`, `literal_bits`) — two expressions differing
//! only in display units are the same expression.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{Dimension, DimensionError, Expr, parse_expr};

fn no_params() -> std::collections::BTreeMap<editor_core::ParamName, Dimension> {
    std::collections::BTreeMap::new()
}

/// The §4g acceptance ladder, end to end on one literal.
#[test]
fn twenty_five_mm_round_trips_value_and_unit() {
    let e = parse_expr("25 mm", &no_params()).unwrap();
    assert_eq!(e.dim(), Dimension::Length);
    // Canonical value: 25 · 1e-3, one multiply.
    assert_eq!(e.literal_value().unwrap().to_bits(), 0.025_f64.to_bits());
    let unit = e.display_unit().expect("the authored unit is stored");
    assert_eq!(unit.symbol, "mm");
    // Persist (the wire form) and load: both halves survive.
    let json = serde_json::to_string(&e).unwrap();
    assert!(
        json.contains("\"unit\":\"mm\""),
        "the symbol is on the wire: {json}"
    );
    let back: Expr = serde_json::from_str(&json).unwrap();
    assert_eq!(back.literal_value().unwrap().to_bits(), 0.025_f64.to_bits());
    let back_unit = back.display_unit().expect("unit survives the load door");
    assert_eq!(back_unit.symbol, "mm");
    // Format: the READ-BACK unit (not a hardcoded constant) drives
    // the renderer back to the source text (review NOTE-5).
    let render = quantity::LengthUnit {
        symbol: back_unit.symbol,
        factor: back_unit.factor,
    };
    assert_eq!(
        quantity::fmt_length(back.literal_value().unwrap(), render).unwrap(),
        "25 mm"
    );
}

/// D7 blindness, pinned in both directions: display units are excluded
/// from `PartialEq`, `bit_eq`, and `literal_bits` — and a VALUE
/// difference still shows through all three.
#[test]
fn display_units_never_enter_expression_identity() {
    let plain = Expr::literal(0.025, Dimension::Length).unwrap();
    let with_mm = parse_expr("25 mm", &no_params()).unwrap();
    let with_cm = parse_expr("2.5 cm", &no_params()).unwrap();
    // Same canonical bits, three different display units (none/mm/cm):
    // one expression, all comparators agree.
    assert_eq!(plain, with_mm);
    assert!(plain.bit_eq(&with_mm), "bit_eq is display-unit-blind (D7)");
    assert!(with_mm.bit_eq(&with_cm));
    let bits = |e: &Expr| {
        let mut out = Vec::new();
        e.literal_bits(&mut out);
        out
    };
    assert_eq!(bits(&plain), bits(&with_mm));
    assert_eq!(bits(&with_mm), bits(&with_cm));
    // Direction two: a real value difference is NOT hidden.
    let other = Expr::literal(0.026, Dimension::Length).unwrap();
    assert_ne!(plain, other);
    assert!(!plain.bit_eq(&other));
    assert_ne!(bits(&plain), bits(&other));
}

/// The construction door: a unit whose quantity disagrees with the
/// literal's dimension is corrupt data, refused typed.
#[test]
fn mismatched_display_unit_refuses_at_construction() {
    match Expr::literal_with_unit(0.5, Dimension::Angle, quantity::MM.def()) {
        Err(DimensionError::DisplayUnitMismatch {
            unit: Dimension::Length,
            literal: Dimension::Angle,
        }) => {}
        other => panic!("mm on an Angle literal must refuse, got {other:?}"),
    }
    match Expr::literal_with_unit(0.5, Dimension::Scalar, quantity::DEG.def()) {
        Err(DimensionError::DisplayUnitMismatch { .. }) => {}
        other => panic!("a unit on a Scalar literal must refuse, got {other:?}"),
    }
    // literal()'s own doors still run underneath.
    assert!(matches!(
        Expr::literal_with_unit(f64::NAN, Dimension::Length, quantity::MM.def()),
        Err(DimensionError::NonFiniteLiteral)
    ));
}

/// The load door stays strict: an unknown display-unit SYMBOL refuses
/// (closed vocabulary), an unknown FIELD refuses (`deny_unknown_fields`
/// kept), and a unitless literal serializes WITHOUT the field (golden
/// byte-stability: absence is the canonical spelling of "no unit").
#[test]
fn wire_door_refuses_unknown_units_and_omits_absent_ones() {
    let bad_symbol = r#"{"Literal":{"value":0.025,"dim":"Length","unit":"furlong"}}"#;
    let err = serde_json::from_str::<Expr>(bad_symbol).unwrap_err();
    assert!(
        err.to_string().contains("furlong"),
        "unknown symbol names itself: {err}"
    );
    let bad_field = r#"{"Literal":{"value":0.025,"dim":"Length","units":"mm"}}"#;
    assert!(serde_json::from_str::<Expr>(bad_field).is_err());
    // Absence round-trips as absence.
    let plain = Expr::literal(0.025, Dimension::Length).unwrap();
    let json = serde_json::to_string(&plain).unwrap();
    assert!(
        !json.contains("unit"),
        "no unit ⇒ no field on the wire: {json}"
    );
    let back: Expr = serde_json::from_str(&json).unwrap();
    assert_eq!(back.display_unit(), None);
}

/// A unit-bearing literal inside a COMPOUND expression keeps its unit
/// through the wire, and the compound's identity stays display-blind.
#[test]
fn units_survive_inside_compound_expressions() {
    let params = no_params();
    let a = parse_expr("25 mm + 1 in", &params).unwrap();
    let json = serde_json::to_string(&a).unwrap();
    let back: Expr = serde_json::from_str(&json).unwrap();
    assert!(a.bit_eq(&back));
    // The two leaves kept their units through the round-trip.
    let left = back.descend(&[0]).unwrap();
    let right = back.descend(&[1]).unwrap();
    assert_eq!(left.display_unit().unwrap().symbol, "mm");
    assert_eq!(right.display_unit().unwrap().symbol, "in");
    // And a canonically-spelled twin (same values, `m` suffixes — a
    // bare real would be Scalar) is the SAME expression.
    let twin = parse_expr("0.025 m + 0.0254 m", &params).unwrap();
    assert!(a.bit_eq(&twin));
}

/// **The display-unit vocabulary IS `quantity::UNITS`** — the row that
/// goes red if editor-core ever holds a second opinion about it.
///
/// The loop is over the TABLE, not over a hand-written list of
/// symbols, so a unit added to `quantity` is covered here the day it
/// lands and a unit renamed there cannot leave a stale spelling behind
/// in this crate. Each row is walked through every door that touches
/// the display unit: construction (`literal_with_unit`), the wire
/// (serialize → load), the read-back accessor, and the text parser —
/// so a code↔row mapping that disagreed with the table, at any one of
/// them, fails here. (Before this row only `mm`, `cm` and `in` were
/// exercised at all; `m`, `deg` and `rad` had no round-trip anywhere.)
#[test]
fn every_row_of_the_closed_table_is_a_working_display_unit() {
    assert_eq!(
        quantity::UNITS.len(),
        6,
        "the closed table is six rows; a change here is a vocabulary change"
    );
    for row in quantity::UNITS {
        let dim = match row.quantity {
            quantity::UnitQuantity::Length => Dimension::Length,
            quantity::UnitQuantity::Angle => Dimension::Angle,
        };
        // Construction stores the row and reads back the SAME row —
        // symbol, quantity and factor, not just the symbol.
        let e = Expr::literal_with_unit(1.0, dim, row)
            .unwrap_or_else(|err| panic!("{} is a table row: {err:?}", row.symbol));
        assert_eq!(
            e.display_unit().expect("the authored unit is stored"),
            row,
            "{} read back as a different row",
            row.symbol
        );
        // The wire carries the symbol and the load door resolves it
        // back to the same row.
        let json = serde_json::to_string(&e).unwrap();
        assert!(
            json.contains(&format!("\"unit\":\"{}\"", row.symbol)),
            "{} is not on the wire under its own symbol: {json}",
            row.symbol
        );
        let back: Expr = serde_json::from_str(&json).unwrap();
        assert_eq!(
            back.display_unit().expect("unit survives the load door"),
            row,
            "{} did not survive the wire",
            row.symbol
        );
        // And the text parser reaches the same row from the suffix.
        let parsed = parse_expr(&format!("1 {}", row.symbol), &no_params())
            .unwrap_or_else(|err| panic!("`1 {}` must parse: {err:?}", row.symbol));
        assert_eq!(parsed.dim(), dim, "{} parsed at the wrong dimension", row.symbol);
        assert_eq!(
            parsed.display_unit().expect("the parser stores the suffix"),
            row,
            "the parser reached a different row for {}",
            row.symbol
        );
        // The canonical value is the decimal times the row's factor,
        // one multiply (the parser's stated contract).
        assert_eq!(
            parsed.literal_value().unwrap().to_bits(),
            (1.0_f64 * row.factor).to_bits(),
            "{} did not land on one f64 multiply",
            row.symbol
        );
    }
}

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
    // Format: the stored unit drives the renderer back to the source.
    assert_eq!(
        quantity::fmt_length(back.literal_value().unwrap(), quantity::MM).unwrap(),
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

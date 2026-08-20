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

/// A table row by symbol. `quantity::UnitDef` is sealed (issue #650),
/// as are its typed views (issue #669), so this is the public route to
/// a row, and it is the same route the wire door takes.
fn table_row(symbol: &str) -> quantity::UnitDef {
    quantity::unit_by_symbol(symbol).unwrap_or_else(|| panic!("{symbol} is a table row"))
}

/// The literal dimension a table row's quantity implies.
fn dim_of(row: quantity::UnitDef) -> Dimension {
    match row.quantity() {
        quantity::UnitQuantity::Length => Dimension::Length,
        quantity::UnitQuantity::Angle => Dimension::Angle,
    }
}

/// The §4g acceptance ladder, end to end on one literal.
#[test]
fn twenty_five_mm_round_trips_value_and_unit() {
    let e = parse_expr("25 mm", &no_params()).unwrap();
    assert_eq!(e.dim(), Dimension::Length);
    // Canonical value: 25 · 1e-3, one multiply.
    assert_eq!(e.literal_value().unwrap().to_bits(), 0.025_f64.to_bits());
    let unit = e.display_unit().expect("the authored unit is stored");
    assert_eq!(unit.symbol(), "mm");
    // Persist (the wire form) and load: both halves survive.
    let json = serde_json::to_string(&e).unwrap();
    assert!(
        json.contains("\"unit\":\"mm\""),
        "the symbol is on the wire: {json}"
    );
    let back: Expr = serde_json::from_str(&json).unwrap();
    assert_eq!(back.literal_value().unwrap().to_bits(), 0.025_f64.to_bits());
    let back_unit = back.display_unit().expect("unit survives the load door");
    assert_eq!(back_unit.symbol(), "mm");
    // Format: the READ-BACK unit (not a hardcoded constant) drives
    // the renderer back to the source text (review NOTE-5). The row is
    // converted to its typed view rather than re-assembled from its
    // fields: since #669 sealed `LengthUnit` the fields cannot be
    // re-paired, and `as_length` is the only door.
    let render = back_unit
        .as_length()
        .expect("mm is a length row, so it has a length view");
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
    match Expr::literal_with_unit(0.5, Dimension::Angle, table_row("mm")) {
        Err(DimensionError::DisplayUnitMismatch {
            unit: Dimension::Length,
            literal: Dimension::Angle,
        }) => {}
        other => panic!("mm on an Angle literal must refuse, got {other:?}"),
    }
    match Expr::literal_with_unit(0.5, Dimension::Scalar, table_row("deg")) {
        Err(DimensionError::DisplayUnitMismatch { .. }) => {}
        other => panic!("a unit on a Scalar literal must refuse, got {other:?}"),
    }
    // literal()'s own doors still run underneath.
    assert!(matches!(
        Expr::literal_with_unit(f64::NAN, Dimension::Length, table_row("mm")),
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

/// **The LOAD door refuses a TABLED symbol carried on the wrong
/// dimension** — `{"dim":"Angle","unit":"mm"}`, a pairing no row of the
/// table exists for.
///
/// Since #650 sealed `quantity::UnitDef` this is the
/// production-realistic route to a corrupt document: no caller can
/// build the mismatched row any more, but a hand-edited or
/// externally-produced `.cad` file can still carry the pairing, and
/// this is the door that has to refuse it — the "file they cannot
/// open" #650 described, arriving from the only direction still open.
///
/// Its sibling above covers the unknown-SYMBOL arm and
/// `a_display_unit_is_accepted_exactly_on_its_own_dimension` covers the
/// CONSTRUCTOR arm. Neither reaches this one, which is exactly the
/// shape #646 and #650 both recorded as the reason the original defect
/// hid: the refusal reads as covered because a DIFFERENT construction
/// site is covered.
#[test]
fn wire_door_refuses_a_tabled_unit_on_the_wrong_dimension() {
    for (json, unit_dim, literal_dim) in [
        (
            r#"{"Literal":{"value":0.5,"dim":"Angle","unit":"mm"}}"#,
            "Length",
            "Angle",
        ),
        (
            r#"{"Literal":{"value":0.5,"dim":"Length","unit":"deg"}}"#,
            "Angle",
            "Length",
        ),
        (
            r#"{"Literal":{"value":0.5,"dim":"Scalar","unit":"mm"}}"#,
            "Length",
            "Scalar",
        ),
    ] {
        let err = serde_json::from_str::<Expr>(json)
            .expect_err("a tabled unit on the wrong dimension must refuse");
        let text = err.to_string();
        assert!(
            text.contains("DisplayUnitMismatch")
                && text.contains(&format!("unit: {unit_dim}"))
                && text.contains(&format!("literal: {literal_dim}")),
            "the refusal must name the pair it read, got {text}"
        );
    }
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
    assert_eq!(left.display_unit().unwrap().symbol(), "mm");
    assert_eq!(right.display_unit().unwrap().symbol(), "in");
    // And a canonically-spelled twin (same values, `m` suffixes — a
    // bare real would be Scalar) is the SAME expression.
    let twin = parse_expr("0.025 m + 0.0254 m", &params).unwrap();
    assert!(a.bit_eq(&twin));
}

/// **The display-unit vocabulary IS `quantity::UNITS`** — one loop over
/// the table carrying every row through every door that touches the
/// display unit: construction (`literal_with_unit`), the read-back
/// accessor, the text parser, and the wire in both directions AND as
/// exact bytes.
///
/// The loop is over the TABLE, not a hand-written list of symbols, so a
/// unit added to `quantity` is covered here the day it lands and a unit
/// renamed there cannot leave a stale spelling behind in this crate.
/// [`UNIT_WIRE_GOLDEN`] is the one hand-written list; the membership
/// assertion at the end holds it to the table's contents, as a SET —
/// `UnitSym`'s rustdoc promises that a REORDER in `quantity` needs no
/// edit here, and an order-sensitive comparison would falsify it.
///
/// What it newly covers, stated exactly: `m`, `deg` and `rad` DO have a
/// round-trip elsewhere — `u8a_parse.rs`'s `fmt_parse_round_trip` rows
/// carry all six through fmt → parse → eval with a bit-exact value and
/// dimension. What none of the six but `mm`, `cm` and `in` had was the
/// **`display_unit()` door**: the stored code resolving back to its own
/// table row, through construction, the wire and the parser. That, and
/// the exact bytes, are what this row adds.
///
/// There is deliberately no `UNITS.len() == 6` assertion: the
/// vocabulary is pinned once, in `quantity`'s own suite.
#[test]
fn every_row_of_the_closed_table_is_a_working_display_unit() {
    for row in quantity::UNITS {
        let dim = dim_of(row);
        // Construction stores the row and reads back the SAME row —
        // symbol, quantity and factor, not just the symbol.
        // 2.5, not 1: `1.0 * f == f` bitwise, so a probe of 1 cannot
        // tell "applied the factor" from "returned the factor", and
        // for `m` and `rad` (factor exactly 1.0) it degenerates to
        // `1.0 == 1.0`. 2.5 distinguishes all three.
        let e = Expr::literal_with_unit(2.5, dim, row)
            .unwrap_or_else(|err| panic!("{} is a table row: {err:?}", row.symbol()));
        assert_eq!(
            e.display_unit().expect("the authored unit is stored"),
            row,
            "{} read back as a different row",
            row.symbol()
        );
        // The text parser reaches the same row from the suffix, and
        // the canonical value is the decimal times the row's factor,
        // one multiply (the parser's stated contract).
        let parsed = parse_expr(&format!("2.5 {}", row.symbol()), &no_params())
            .unwrap_or_else(|err| panic!("`2.5 {}` must parse: {err:?}", row.symbol()));
        assert_eq!(
            parsed.dim(),
            dim,
            "{} parsed at the wrong dimension",
            row.symbol()
        );
        assert_eq!(
            parsed.display_unit().expect("the parser stores the suffix"),
            row,
            "the parser reached a different row for {}",
            row.symbol()
        );
        assert_eq!(
            parsed.literal_value().unwrap().to_bits(),
            (2.5_f64 * row.factor()).to_bits(),
            "{} did not land on one f64 multiply",
            row.symbol()
        );
        // The wire, to the byte. The seal changed how a `UnitDef`'s
        // symbol is READ on the write side (`u.symbol` → `u.symbol()`)
        // and nothing else on this path, so these bytes are the claim
        // "byte-identical" made checkable.
        let golden = golden_wire_form(row.symbol());
        let bytes = serde_json::to_vec(&parsed).expect("a literal serializes");
        assert_eq!(
            bytes,
            golden.as_bytes(),
            "{}: wire bytes moved — got {}",
            row.symbol(),
            String::from_utf8_lossy(&bytes)
        );
        // The load door resolves the symbol back to the same row, and
        // re-serializes to the same bytes: a fixed point, not a
        // one-way match.
        let back: Expr = serde_json::from_slice(&bytes).expect("the load door accepts them");
        assert_eq!(
            back.display_unit().expect("unit survives the load door"),
            row,
            "{} did not survive the wire",
            row.symbol()
        );
        assert_eq!(serde_json::to_vec(&back).unwrap(), golden.as_bytes());
    }
    // The golden and the table agree on MEMBERSHIP (not on order), so
    // an added unit fails here rather than going silently unpinned.
    let mut pinned = UNIT_WIRE_GOLDEN.map(|(s, _)| s).to_vec();
    let mut tabled = quantity::UNITS
        .iter()
        .map(|u| u.symbol())
        .collect::<Vec<_>>();
    pinned.sort_unstable();
    tabled.sort_unstable();
    assert_eq!(pinned, tabled, "the golden must cover exactly the table");
}

/// The exact wire bytes of `2.5 <symbol>` for every row of the closed
/// table, in both directions — the byte pin folded into the loop above.
///
/// Derived from the factors rather than blessed from a run, and they
/// passed first try, which is a small extra check on the float
/// rendering. The golden `.cad` documents (`tests/golden/v*.cad`) carry
/// a `"unit": "mm"` literal each and are the same evidence at document
/// scale — for one of these six rows at one value.
const UNIT_WIRE_GOLDEN: [(&str, &str); 6] = [
    (
        "mm",
        r#"{"Literal":{"value":0.0025,"dim":"Length","unit":"mm"}}"#,
    ),
    (
        "cm",
        r#"{"Literal":{"value":0.025,"dim":"Length","unit":"cm"}}"#,
    ),
    (
        "m",
        r#"{"Literal":{"value":2.5,"dim":"Length","unit":"m"}}"#,
    ),
    (
        "in",
        r#"{"Literal":{"value":0.0635,"dim":"Length","unit":"in"}}"#,
    ),
    (
        "deg",
        r#"{"Literal":{"value":0.04363323129985824,"dim":"Angle","unit":"deg"}}"#,
    ),
    (
        "rad",
        r#"{"Literal":{"value":2.5,"dim":"Angle","unit":"rad"}}"#,
    ),
];

fn golden_wire_form(symbol: &str) -> &'static str {
    UNIT_WIRE_GOLDEN
        .into_iter()
        .find(|(s, _)| *s == symbol)
        .unwrap_or_else(|| panic!("{symbol} has no pinned wire form"))
        .1
}

/// **Issue #650's property, over the whole cross-product: a display
/// unit is accepted exactly on its OWN dimension and refused on every
/// other.**
///
/// #650 was a caller-built `UnitDef` whose `symbol` named a table row
/// but whose `quantity` was not that row's. It is closed STRUCTURALLY —
/// the seal, and why no whole-row re-check was added, live on
/// `quantity::UnitDef`'s rustdoc, pinned there by `compile_fail`
/// doctests in a LIBRARY crate (doctests on an integration-test item
/// never run).
///
/// What is left for THIS crate to pin is the guard the seal leaves
/// standing, stated over 6 rows × 4 dimensions rather than at one
/// fixture. The complementary half — an ACCEPTED row is stored
/// unsubstituted, all three fields — belongs to
/// `every_row_of_the_closed_table_is_a_working_display_unit` and is not
/// restated here. Counts are asserted, so a table that quietly stopped
/// being exercised cannot pass as green.
#[test]
fn a_display_unit_is_accepted_exactly_on_its_own_dimension() {
    let dims = [
        Dimension::Length,
        Dimension::Angle,
        Dimension::Scalar,
        Dimension::Count,
    ];
    let mut accepted = 0_u32;
    let mut refused = 0_u32;
    for r in quantity::UNITS {
        let row_dim = dim_of(r);
        for dim in dims {
            match Expr::literal_with_unit(2.5, dim, r) {
                Ok(_) => {
                    assert_eq!(
                        dim,
                        row_dim,
                        "{} was accepted on a {dim:?} literal",
                        r.symbol()
                    );
                    accepted += 1;
                }
                Err(DimensionError::DisplayUnitMismatch { unit, literal }) => {
                    assert_ne!(
                        dim,
                        row_dim,
                        "{} must not refuse its own dimension",
                        r.symbol()
                    );
                    // The refusal reports the pair it actually read,
                    // not a fixed string.
                    assert_eq!((unit, literal), (row_dim, dim));
                    refused += 1;
                }
                other => panic!("{} on {dim:?} gave {other:?}", r.symbol()),
            }
        }
    }
    assert_eq!(
        accepted,
        quantity::UNITS.len() as u32,
        "one dimension per row accepts"
    );
    assert_eq!(
        refused,
        (quantity::UNITS.len() * (dims.len() - 1)) as u32,
        "every other dimension refuses"
    );
}

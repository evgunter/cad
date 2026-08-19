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

/// A table row by symbol. `quantity::UnitDef` is sealed (issue #650) —
/// `LengthUnit::def()` is `pub(crate)` to `quantity`, so this is the
/// public route to a row, and it is the same route the wire door takes.
fn table_row(symbol: &str) -> quantity::UnitDef {
    quantity::unit_by_symbol(symbol).unwrap_or_else(|| panic!("{symbol} is a table row"))
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
    // the renderer back to the source text (review NOTE-5).
    let render = quantity::LengthUnit {
        symbol: back_unit.symbol(),
        factor: back_unit.factor(),
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
/// them, fails here.
///
/// What it newly covers, stated exactly: `m`, `deg` and `rad` DO have
/// a round-trip elsewhere — `u8a_parse.rs`'s `fmt_parse_round_trip`
/// rows carry all six through fmt → parse → eval with a bit-exact
/// value and dimension. What none of the six but `mm`, `cm` and `in`
/// had was coverage of the **`display_unit()` door**: the stored code
/// resolving back to its own table row, through construction, the
/// wire and the parser. That is what this row adds.
///
/// There is deliberately no `UNITS.len() == 6` assertion here: it
/// would make an ADDED unit fail this row before the loop covered it,
/// which is the opposite of table-driven. The vocabulary is pinned
/// once, in `quantity`'s own suite.
#[test]
fn every_row_of_the_closed_table_is_a_working_display_unit() {
    for row in quantity::UNITS {
        let dim = match row.quantity() {
            quantity::UnitQuantity::Length => Dimension::Length,
            quantity::UnitQuantity::Angle => Dimension::Angle,
        };
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
        // The wire carries the symbol and the load door resolves it
        // back to the same row.
        let json = serde_json::to_string(&e).unwrap();
        assert!(
            json.contains(&format!("\"unit\":\"{}\"", row.symbol())),
            "{} is not on the wire under its own symbol: {json}",
            row.symbol()
        );
        let back: Expr = serde_json::from_str(&json).unwrap();
        assert_eq!(
            back.display_unit().expect("unit survives the load door"),
            row,
            "{} did not survive the wire",
            row.symbol()
        );
        // And the text parser reaches the same row from the suffix.
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
        // The canonical value is the decimal times the row's factor,
        // one multiply (the parser's stated contract).
        assert_eq!(
            parsed.literal_value().unwrap().to_bits(),
            (2.5_f64 * row.factor()).to_bits(),
            "{} did not land on one f64 multiply",
            row.symbol()
        );
    }
}

/// The closed-vocabulary door **refuses** a `UnitDef` that is not a
/// table row: `UnitSym::from_def`'s `None` arm, raised by
/// `literal_with_unit` as `UnknownDisplayUnit`.
///
/// This branch had no coverage at all before #646. The only tested
/// `UnknownDisplayUnit` is the WIRE door's own
/// (`wire_door_refuses_unknown_units_and_omits_absent_ones`), built
/// from `quantity::unit_by_symbol` — a different construction site —
/// which is exactly why the refusal read as covered. Mutating
/// `from_def` to `.position(...).or(Some(0))`, so every off-table
/// `UnitDef` silently becomes `mm` and the refusal is dead code,
/// leaves the whole editor-core battery green without this row.
///
/// **Why the rows are minted, not built.** Issue #650 sealed
/// `quantity::UnitDef`: private fields, no public constructor, and
/// `LengthUnit::def` / `AngleUnit::def` demoted to `pub(crate)`. An
/// off-table row is therefore unrepresentable from production surface
/// — which is the fix, and which would also make this refusal
/// untestable if that were the whole story. `mint_untabled` is
/// `quantity`'s `units-testing` door (`#[doc(hidden)]`, turned on only
/// through dev-dependencies, `topo`'s `sweep-testing` shape): it hands
/// the test the one thing no caller can build, so the door that
/// refuses it stays proven rather than merely believed.
#[test]
fn a_unit_outside_the_closed_table_is_refused_rather_than_mapped() {
    let furlong =
        quantity::UnitDef::mint_untabled("furlong", quantity::UnitQuantity::Length, 201.168);
    match Expr::literal_with_unit(1.0, Dimension::Length, furlong) {
        Err(DimensionError::UnknownDisplayUnit { symbol }) => {
            assert_eq!(symbol, "furlong", "the refusal names the symbol it read")
        }
        other => panic!("an off-table unit must be refused, got {other:?}"),
    }
    // The table is DATA and its symbols are case-sensitive: "MM" is
    // not a row of it, however plausible it looks.
    let shouty = quantity::UnitDef::mint_untabled("MM", quantity::UnitQuantity::Length, 1e-3);
    assert!(
        matches!(
            Expr::literal_with_unit(1.0, Dimension::Length, shouty),
            Err(DimensionError::UnknownDisplayUnit { .. })
        ),
        "symbols are case-sensitive data, not a fuzzy match"
    );
}

/// **Issue #650: `literal_with_unit` never STORES a row that
/// disagrees with the literal's dimension.**
///
/// The defect was a caller-built `UnitDef` whose `symbol` named a
/// table row but whose `quantity` was a different quantity. It
/// defeated the dimension guard because the guard read the CALLER's
/// `quantity` and `from_def` then threw that row away for the table's,
/// which was never re-checked — producing an `Expr` that serialized
/// into a document editor-core's own load door refused
/// (`DisplayUnitMismatch`): a round-trip break, not a rejected call.
///
/// It is closed STRUCTURALLY. `quantity::UnitDef` is sealed (private
/// fields, no public constructor, `LengthUnit::def`/`AngleUnit::def`
/// demoted to `pub(crate)`), so the mismatched row is unrepresentable
/// and no whole-row re-check was added here — a check for a state the
/// type system excludes is dead code pretending to be a guard. The
/// unrepresentability itself is pinned where a doctest actually runs,
/// on `quantity::UnitDef`'s rustdoc (`compile_fail` blocks in a
/// LIBRARY crate; doctests on an integration-test item never run).
/// `quantity`'s test-only mint deliberately cannot rebuild the row
/// either — it panics on a tabled symbol — so the defect has no door
/// left, not even in test builds.
///
/// What is left for THIS crate to pin is the property the seal is
/// supposed to deliver, stated over the whole cross-product rather
/// than at one fixture: for every table row and every dimension,
/// `literal_with_unit` either refuses or stores THAT row, and the
/// stored row's quantity always agrees with the literal's dimension.
/// It goes red if the guard at `expr.rs` is dropped, if `from_def`
/// starts selecting a different row, or if a future table gains two
/// rows sharing a symbol.
#[test]
fn a_stored_display_unit_always_agrees_with_the_literal_dimension() {
    let dims = [
        Dimension::Length,
        Dimension::Angle,
        Dimension::Scalar,
        Dimension::Count,
    ];
    let mut agreed = 0_u32;
    let mut refused = 0_u32;
    for r in quantity::UNITS {
        let row_dim = match r.quantity() {
            quantity::UnitQuantity::Length => Dimension::Length,
            quantity::UnitQuantity::Angle => Dimension::Angle,
        };
        for dim in dims {
            match Expr::literal_with_unit(2.5, dim, r) {
                Ok(e) => {
                    assert_eq!(
                        dim,
                        row_dim,
                        "{} was accepted on a {dim:?} literal",
                        r.symbol()
                    );
                    let stored = e.display_unit().expect("an accepted unit is stored");
                    // No substitution: the row that went in is the row
                    // that came out, all three fields.
                    assert_eq!(stored, r, "{} was replaced by another row", r.symbol());
                    // And the stored row agrees with the dimension it
                    // now travels with — the #650 property, read off
                    // the STORED row rather than the caller's.
                    let stored_dim = match stored.quantity() {
                        quantity::UnitQuantity::Length => Dimension::Length,
                        quantity::UnitQuantity::Angle => Dimension::Angle,
                    };
                    assert_eq!(
                        stored_dim,
                        e.dim(),
                        "{} is stored on a {:?} literal — #650 is back",
                        stored.symbol(),
                        e.dim()
                    );
                    agreed += 1;
                }
                Err(DimensionError::DisplayUnitMismatch { .. }) => {
                    assert_ne!(
                        dim,
                        row_dim,
                        "{} must not refuse its own dimension",
                        r.symbol()
                    );
                    refused += 1;
                }
                other => panic!("{} on {dim:?} gave {other:?}", r.symbol()),
            }
        }
    }
    // The counts are the shape of the cross-product, so a table that
    // silently stopped being exercised cannot pass as green.
    assert_eq!(
        agreed,
        quantity::UNITS.len() as u32,
        "one dimension per row accepts"
    );
    assert_eq!(
        refused,
        (quantity::UNITS.len() * (dims.len() - 1)) as u32,
        "every other dimension refuses"
    );
}

/// **Wire byte-identity across the #650 seal, as bytes rather than as
/// an argument.**
///
/// The seal changed how a `UnitDef`'s symbol is READ (`u.symbol` →
/// `u.symbol()`) on the wire's write side (`persist::wire`), and
/// nothing else on that path: `Lit` still stores the one-byte
/// `UnitSym` code, the wire still carries the SYMBOL, and the load
/// door still resolves it through `quantity::unit_by_symbol`. This row
/// pins the resulting bytes for every row of the closed table, exactly,
/// so "byte-identical" is a checked claim.
///
/// These literals are the ones the golden `.cad` documents embed
/// (`tests/golden/v*.cad` carry `"unit": "mm"`), so those files are the
/// same evidence at document scale; this row is the same evidence at
/// the scale the change actually touched.
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

#[test]
fn the_display_unit_wire_form_is_byte_for_byte_what_it_was() {
    for (symbol, golden) in UNIT_WIRE_GOLDEN {
        // Authored through the TEXT parser, the door a user reaches.
        let e = parse_expr(&format!("2.5 {symbol}"), &no_params())
            .unwrap_or_else(|err| panic!("`2.5 {symbol}` must parse: {err:?}"));
        let bytes = serde_json::to_vec(&e).expect("a literal serializes");
        assert_eq!(
            bytes,
            golden.as_bytes(),
            "{symbol}: wire bytes moved — got {}",
            String::from_utf8_lossy(&bytes)
        );
        // And the same bytes come back out of the load door unchanged,
        // so the identity is a fixed point, not a one-way match.
        let back: Expr = serde_json::from_slice(&bytes).expect("the load door accepts them");
        assert_eq!(serde_json::to_vec(&back).unwrap(), golden.as_bytes());
        assert_eq!(back.display_unit().expect("unit survives").symbol(), symbol);
    }
    // The table and the golden agree on membership, so an ADDED unit
    // fails here rather than being silently unpinned.
    assert_eq!(
        UNIT_WIRE_GOLDEN.map(|(s, _)| s).to_vec(),
        quantity::UNITS
            .iter()
            .map(|u| u.symbol())
            .collect::<Vec<_>>(),
        "the golden must cover exactly the closed table"
    );
}

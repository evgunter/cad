//! LIB-U8a: the expression text door — grammar pins, child-order
//! (descend) pins, every-DimensionError-through-the-parser pins, and
//! the parse/format round-trip property against quantity's formatter.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use editor_core::{
    Dimension, DimensionError, Expr, ParamEnv, ParamName, ParseError, eval, eval_count, parse_expr,
    unparse,
};
use proptest::prelude::*;

fn no_params() -> BTreeMap<ParamName, Dimension> {
    BTreeMap::new()
}

fn p(src: &str) -> Expr {
    parse_expr(src, &no_params()).expect(src)
}

fn perr(src: &str) -> ParseError {
    parse_expr(src, &no_params()).expect_err(src)
}

fn bits(e: &Expr) -> Vec<u64> {
    let mut out = Vec::new();
    e.literal_bits(&mut out);
    out
}

fn ev(e: &Expr) -> f64 {
    eval::<f64>(e, &ParamEnv::default()).expect("finite eval")
}

#[test]
fn unit_suffixed_literals_land_in_canonical_units() {
    let e = p("25 mm");
    assert_eq!(e.dim(), Dimension::Length);
    assert_eq!(bits(&e), vec![(25.0 * quantity::MM).meters().to_bits()]);
    assert_eq!(p("2 deg").dim(), Dimension::Angle);
    assert_eq!(
        bits(&p("2 deg")),
        vec![(2.0 * quantity::DEG).radians().to_bits()]
    );
    // Every table symbol, juxtaposed form included.
    for (src, dim) in [
        ("1 mm", Dimension::Length),
        ("1 cm", Dimension::Length),
        ("1 m", Dimension::Length),
        ("1 in", Dimension::Length),
        ("1 deg", Dimension::Angle),
        ("1 rad", Dimension::Angle),
        ("1 pi rad", Dimension::Angle),
        ("25mm", Dimension::Length),
        ("2.5e1 mm", Dimension::Length),
    ] {
        assert_eq!(p(src).dim(), dim, "{src}");
    }
    // The two-word symbol is a LONGEST MATCH over adjacent
    // identifiers, so it reaches the half-turn row and not the
    // canonical one it ends in.
    assert_eq!(
        p("1 pi rad").display_unit().expect("the suffix is stored"),
        quantity::PI.def()
    );
    assert_eq!(
        bits(&p("1 pi rad")),
        vec![(1.0 * quantity::PI).radians().to_bits()]
    );
    // The heatsink migration's basis: the mm spellings are bit-equal
    // to the canonical-meter dyadics the tour hand-wrote.
    assert_eq!(bits(&p("250 mm")), vec![0.25f64.to_bits()]);
    assert_eq!(bits(&p("812.5 mm")), vec![0.8125f64.to_bits()]);
    assert_eq!(bits(&p("312.5 mm")), vec![0.3125f64.to_bits()]);
}

#[test]
fn bare_integers_are_counts_and_bare_reals_are_scalars() {
    let five = p("5");
    assert_eq!(five.dim(), Dimension::Count);
    assert_eq!(eval_count(&five, &ParamEnv::<f64>::default()), Ok(5));
    for src in ["5.0", "5.", "1e3", "2.5e-3", "0.5"] {
        assert_eq!(p(src).dim(), Dimension::Scalar, "{src}");
    }
    assert_eq!(ev(&p("1e3")).to_bits(), 1000.0f64.to_bits());
    // Count arithmetic stays Count; explicit promotion is `scalar`.
    assert_eq!(p("5 + 2").dim(), Dimension::Count);
    assert_eq!(p("5 * 2").dim(), Dimension::Count);
    assert_eq!(p("scalar(5)").dim(), Dimension::Scalar);
    assert_eq!(p("-5").dim(), Dimension::Count);
    assert!(matches!(
        perr("99999999999999999999"),
        ParseError::IntegerOverflow { pos: 0, .. }
    ));
}

#[test]
fn precedence_and_child_order_match_the_ast_descend_indices() {
    // 1.0 + 2.0 * 3.0 — Mul binds tighter and sits as child 1 of Add.
    let e = p("1.0 + 2.0 * 3.0");
    assert_eq!(ev(&e), 7.0);
    assert_eq!(
        bits(&e),
        vec![1.0f64.to_bits(), 2.0f64.to_bits(), 3.0f64.to_bits()]
    );
    assert_eq!(
        bits(e.descend(&[1]).expect("mul at child 1")),
        vec![2.0f64.to_bits(), 3.0f64.to_bits()]
    );
    assert_eq!(ev(&p("(1.0 + 2.0) * 3.0")), 9.0);
    // Left association: 1.0 - 2.0 - 3.0 = (1.0 - 2.0) - 3.0.
    let e = p("1.0 - 2.0 - 3.0");
    assert_eq!(ev(&e), -4.0);
    assert_eq!(
        bits(e.descend(&[0, 1]).expect("inner rhs")),
        vec![2.0f64.to_bits()]
    );
    // Argument order IS child order: atan2(y, x) puts y at index 0 —
    // persisted ExprPaths depend on this.
    let e = p("atan2(7.0, 9.0)");
    assert_eq!(bits(e.descend(&[0]).expect("y")), vec![7.0f64.to_bits()]);
    assert_eq!(bits(e.descend(&[1]).expect("x")), vec![9.0f64.to_bits()]);
    assert!(e.descend(&[2]).is_none());
    let e = p("1.0 / 2.0");
    assert_eq!(
        bits(e.descend(&[0]).expect("dividend")),
        vec![1.0f64.to_bits()]
    );
    // Division is not commutative and not right-associative here.
    assert_eq!(ev(&p("8.0 / 2.0 / 2.0")), 2.0);
    // Unary minus binds tighter than the binary operators.
    assert_eq!(ev(&p("-2.5")), -2.5);
    assert_eq!(ev(&p("--2.5")), 2.5);
    assert_eq!(ev(&p("3.0 - -2.0")), 5.0);
    assert_eq!(ev(&p("-2.0 * 3.0")), -6.0);
    let neg_len = p("-2.5 mm");
    assert_eq!(neg_len.dim(), Dimension::Length);
    assert_eq!(
        ev(&neg_len).to_bits(),
        (-(2.5 * quantity::MM.factor())).to_bits()
    );
}

#[test]
fn params_resolve_against_the_callers_table() {
    let mut params = no_params();
    params.insert(ParamName::new("width"), Dimension::Length);
    params.insert(ParamName::new("n"), Dimension::Count);
    params.insert(ParamName::new("mm"), Dimension::Scalar);
    let e = parse_expr("width + 25 mm", &params).unwrap();
    assert_eq!(e.dim(), Dimension::Length);
    let mut refs = Vec::new();
    e.param_refs(&mut refs);
    assert_eq!(refs, vec![(ParamName::new("width"), Dimension::Length)]);
    assert_eq!(parse_expr("n", &params).unwrap().dim(), Dimension::Count);
    // A param may share a unit's name: position disambiguates (an
    // ident is a unit only DIRECTLY after a number).
    assert_eq!(
        parse_expr("mm + 1.0", &params).unwrap().dim(),
        Dimension::Scalar
    );
    assert!(matches!(
        parse_expr("q", &params),
        Err(ParseError::UnknownParam { pos: 0, name }) if name == "q"
    ));
    // `sin` not followed by `(` is an ident like any other.
    assert!(matches!(perr("sin"), ParseError::UnknownParam { .. }));
}

#[test]
fn calls_cover_the_whole_ast_vocabulary_and_nothing_more() {
    assert_eq!(p("sin(30 deg)").dim(), Dimension::Scalar);
    assert_eq!(p("cos(0.5 rad)").dim(), Dimension::Scalar);
    assert_eq!(p("tan(1 deg)").dim(), Dimension::Scalar);
    assert_eq!(p("atan2(1 mm, 2 mm)").dim(), Dimension::Angle);
    assert_eq!(p("atan2(1.0, 2.0)").dim(), Dimension::Angle);
    assert_eq!(p("min(1 mm, 2 cm)").dim(), Dimension::Length);
    assert_eq!(p("max(1.0, 2.0)").dim(), Dimension::Scalar);
    assert_eq!(p("min(1, 2)").dim(), Dimension::Count);
    assert_eq!(ev(&p("scalar(5) * 2.0")), 10.0);
    assert!(matches!(
        perr("pow(1.0, 2.0)"),
        ParseError::UnknownFunction { pos: 0, name } if name == "pow"
    ));
    assert!(matches!(
        perr("sin(1 rad, 2 rad)"),
        ParseError::WrongArity {
            name: "sin",
            expected: 1,
            found: 2,
            ..
        }
    ));
    assert!(matches!(
        perr("min(1.0)"),
        ParseError::WrongArity {
            name: "min",
            expected: 2,
            found: 1,
            ..
        }
    ));
}

#[test]
fn syntax_refusals_are_typed_and_positioned() {
    assert!(matches!(perr(""), ParseError::UnexpectedEnd { pos: 0, .. }));
    assert!(matches!(
        perr("(1.0"),
        ParseError::UnexpectedEnd { pos: 4, .. }
    ));
    assert!(matches!(
        perr("1.0)"),
        ParseError::TrailingInput { pos: 3, found } if found == ")"
    ));
    assert!(matches!(
        perr("1 2"),
        ParseError::TrailingInput { pos: 2, .. }
    ));
    assert!(matches!(
        perr(")"),
        ParseError::UnexpectedToken { pos: 0, .. }
    ));
    assert!(matches!(
        perr("1.0 + "),
        ParseError::UnexpectedEnd { pos: 6, .. }
    ));
    assert!(matches!(
        perr("1.0 @"),
        ParseError::UnexpectedChar { pos: 4, ch: '@' }
    ));
    assert!(matches!(
        perr("25 furlong"),
        ParseError::UnknownUnit { pos: 3, symbol } if symbol == "furlong"
    ));
    // Longest match refuses at the FIRST identifier and its offset —
    // the token the reader has to change — not at the two-word phrase
    // the table was also asked about.
    assert!(matches!(
        perr("25 furlong rad"),
        ParseError::UnknownUnit { pos: 3, symbol } if symbol == "furlong"
    ));
    // The fallback is real: `rad` matches alone, and the identifier
    // that follows is then simply out of the grammar.
    assert!(matches!(
        perr("25 rad furlong"),
        ParseError::UnexpectedToken { pos: 7, .. }
    ));
}

/// Every [`DimensionError`] variant the constructors can produce is
/// reachable through the text door and surfaces inside
/// [`ParseError::Dimension`] — the parser refuses exactly what the
/// constructors refuse, nothing less.
#[test]
fn every_dimension_error_reaches_through_the_parser() {
    let dim_err = |src: &str| match perr(src) {
        ParseError::Dimension { error, .. } => error,
        other => panic!("{src}: expected Dimension, got {other:?}"),
    };
    assert_eq!(
        dim_err("5 mm + 3 deg"),
        DimensionError::Mismatch {
            op: "add",
            left: Dimension::Length,
            right: Dimension::Angle,
        }
    );
    assert_eq!(
        dim_err("atan2(1 mm, 1 rad)"),
        DimensionError::Mismatch {
            op: "atan2",
            left: Dimension::Length,
            right: Dimension::Angle,
        }
    );
    assert_eq!(
        dim_err("25 mm * 4 mm"),
        DimensionError::MulNeedsScalar {
            left: Dimension::Length,
            right: Dimension::Length,
        }
    );
    assert_eq!(
        dim_err("25 mm / 4 mm"),
        DimensionError::DivNeedsScalarDivisor {
            left: Dimension::Length,
            right: Dimension::Length,
        }
    );
    assert_eq!(
        dim_err("sin(5 mm)"),
        DimensionError::TrigNeedsAngle {
            op: "sin",
            found: Dimension::Length,
        }
    );
    assert_eq!(
        dim_err("2 * 5 mm"),
        DimensionError::CountNeedsExplicitPromotion { op: "mul" }
    );
    assert_eq!(
        dim_err("5 / 2"),
        DimensionError::CountNeedsExplicitPromotion { op: "div" }
    );
    assert_eq!(
        dim_err("atan2(1, 2)"),
        DimensionError::CountNeedsExplicitPromotion { op: "atan2" }
    );
    assert_eq!(
        dim_err("scalar(2.5)"),
        DimensionError::NotCount {
            found: Dimension::Scalar,
        }
    );
    assert_eq!(dim_err("1e999"), DimensionError::NonFiniteLiteral);
    assert_eq!(dim_err("1e400 mm"), DimensionError::NonFiniteLiteral);
    // DEVIATION (reported): `LiteralCountIsInteger` is STRUCTURALLY
    // unreachable through text — bare integers route to `Expr::count`
    // and a unit suffix always makes a continuous dimension, so no
    // source string can ask for a Count-dimension `Expr::literal`.
    // Pinned at the constructor door instead, so the variant's refusal
    // stays exercised from this suite.
    assert_eq!(
        Expr::literal(2.0, Dimension::Count).unwrap_err(),
        DimensionError::LiteralCountIsInteger
    );
}

/// THE round-trip pin (spec deliverable 3): for every finite value and
/// display unit, quantity's formatter renders text the parser reads
/// back to the exact bits — through the REAL text door, unary minus
/// and canonical-unit fallback included.
#[test]
fn fmt_parse_round_trip_is_bit_exact() {
    fn round_trip(value: f64, text: &str, want_dim: Dimension) {
        let e = p(text);
        assert_eq!(e.dim(), want_dim, "{text}");
        assert_eq!(ev(&e).to_bits(), value.to_bits(), "{text}");
    }
    // Deterministic spot rows, then the property below.
    for (value, unit) in [(0.25, quantity::MM), (0.0254, quantity::IN)] {
        round_trip(
            value,
            &quantity::fmt_length(value, unit).unwrap(),
            Dimension::Length,
        );
    }
    round_trip(
        1.5,
        &quantity::fmt_angle(1.5, quantity::DEG).unwrap(),
        Dimension::Angle,
    );
}

proptest! {
    #[test]
    fn fmt_parse_round_trip_property(
        value in proptest::num::f64::ANY.prop_filter("finite", |v| v.is_finite()),
        unit_idx in 0usize..6,
    ) {
        let (text, dim) = match unit_idx {
            0 => (quantity::fmt_length(value, quantity::MM), Dimension::Length),
            1 => (quantity::fmt_length(value, quantity::CM), Dimension::Length),
            2 => (quantity::fmt_length(value, quantity::M), Dimension::Length),
            3 => (quantity::fmt_length(value, quantity::IN), Dimension::Length),
            4 => (quantity::fmt_angle(value, quantity::DEG), Dimension::Angle),
            _ => (quantity::fmt_angle(value, quantity::RAD), Dimension::Angle),
        };
        let text = text.unwrap();
        let e = parse_expr(&text, &no_params()).expect(&text);
        prop_assert_eq!(e.dim(), dim, "{}", &text);
        let back = eval::<f64>(&e, &ParamEnv::default()).expect(&text);
        prop_assert_eq!(back.to_bits(), value.to_bits(), "{}", &text);
    }
}

// --- proptest over the grammar: random well-formed source text ------

/// A finite numeric literal's text (via `{:?}`, which always carries a
/// `.` or an exponent — so it lexes as a REAL, never as a Count).
fn arb_real_text() -> impl Strategy<Value = String> {
    (-1.0e6f64..1.0e6).prop_map(|v| format!("{v:?}"))
}

/// Random well-formed source of the given dimension, exercising every
/// production the AST has (and only those): suffixed and bare
/// literals, params, all operators, all calls, parens, unary minus.
fn arb_text_of(dim: Dimension, depth: u32) -> BoxedStrategy<String> {
    let leaf: BoxedStrategy<String> = match dim {
        Dimension::Length => (arb_real_text(), prop_oneof!["mm", "cm", "m", "in"])
            .prop_map(|(n, u)| format!("{n} {u}"))
            .boxed(),
        Dimension::Angle => (arb_real_text(), prop_oneof!["deg", "rad", "pi rad"])
            .prop_map(|(n, u)| format!("{n} {u}"))
            .boxed(),
        Dimension::Scalar => prop_oneof![arb_real_text(), Just("S".to_string())].boxed(),
        Dimension::Count => prop_oneof![
            (-1000i64..1000).prop_map(|n| n.to_string()),
            Just("N".to_string()),
        ]
        .boxed(),
    };
    if depth == 0 {
        return leaf;
    }
    let a = arb_text_of(dim, depth - 1);
    let b = arb_text_of(dim, depth - 1);
    let mut choices = vec![
        leaf,
        (a.clone(), b.clone())
            .prop_map(|(a, b)| format!("({a} + {b})"))
            .boxed(),
        (a.clone(), b.clone())
            .prop_map(|(a, b)| format!("({a} - {b})"))
            .boxed(),
        (a.clone(), b.clone())
            .prop_map(|(a, b)| format!("min({a}, {b})"))
            .boxed(),
        (a.clone(), b)
            .prop_map(|(a, b)| format!("max({a}, {b})"))
            .boxed(),
        a.clone().prop_map(|a| format!("-{a}")).boxed(),
    ];
    if dim == Dimension::Count {
        // Count is closed under Mul only with itself.
        let (x, y) = (arb_text_of(dim, depth - 1), arb_text_of(dim, depth - 1));
        choices.push((x, y).prop_map(|(a, b)| format!("({a} * {b})")).boxed());
    } else {
        let s = arb_text_of(Dimension::Scalar, depth - 1);
        choices.push(
            (a.clone(), s.clone())
                .prop_map(|(a, s)| format!("({a} * {s})"))
                .boxed(),
        );
        choices.push((a, s).prop_map(|(a, s)| format!("({a} / {s})")).boxed());
    }
    if dim == Dimension::Scalar {
        let ang = arb_text_of(Dimension::Angle, depth - 1);
        let cnt = arb_text_of(Dimension::Count, depth - 1);
        choices.push(
            (prop_oneof!["sin", "cos", "tan"], ang)
                .prop_map(|(f, a)| format!("{f}({a})"))
                .boxed(),
        );
        choices.push(cnt.prop_map(|c| format!("scalar({c})")).boxed());
    }
    if dim == Dimension::Angle {
        let len = arb_text_of(Dimension::Length, depth - 1);
        let len2 = arb_text_of(Dimension::Length, depth - 1);
        choices.push(
            (len, len2)
                .prop_map(|(y, x)| format!("atan2({y}, {x})"))
                .boxed(),
        );
    }
    proptest::strategy::Union::new(choices).boxed()
}

proptest! {
    /// Any text the grammar can produce parses, lands on the expected
    /// dimension, and its parse is total under the params table —
    /// constructor refusals cannot fire on dimension-correct source.
    #[test]
    fn grammar_generated_text_parses_to_the_expected_dimension(
        (dim, src) in prop_oneof![
            Just(Dimension::Length),
            Just(Dimension::Angle),
            Just(Dimension::Scalar),
            Just(Dimension::Count),
        ]
        .prop_flat_map(|dim| arb_text_of(dim, 3).prop_map(move |src| (dim, src))),
    ) {
        let mut params = BTreeMap::new();
        params.insert(ParamName::new("S"), Dimension::Scalar);
        params.insert(ParamName::new("N"), Dimension::Count);
        let e = parse_expr(&src, &params).expect(&src);
        prop_assert_eq!(e.dim(), dim, "{}", &src);
    }
}

// --- The door OUTWARD (issue #1103): `unparse` -----------------
//
// The pin is the ROUND TRIP, structurally: `parse_expr(unparse(e))`
// is `bit_eq` to `e`. `bit_eq` is display-unit-blind by design, so the
// units get their own assertions rather than riding along.

/// The declared parameters the unparse suites resolve names against.
fn rt_params() -> BTreeMap<ParamName, Dimension> {
    [
        ("w", Dimension::Length),
        ("a", Dimension::Angle),
        ("s", Dimension::Scalar),
        ("n", Dimension::Count),
    ]
    .into_iter()
    .map(|(name, dim)| (ParamName::new(name), dim))
    .collect()
}

fn rp(src: &str) -> Expr {
    parse_expr(src, &rt_params()).expect(src)
}

/// `e`'s source text, checked to read back as `e` itself.
fn round_trip(e: &Expr) -> String {
    let text = unparse(e);
    let back = parse_expr(&text, &rt_params()).expect(&text);
    assert!(
        back.bit_eq(e),
        "{text:?} reparsed to a different expression:\n  {back:?}\n  {e:?}"
    );
    text
}

#[test]
fn unparse_round_trips_the_whole_vocabulary() {
    // Every precedence pairing (each operator family on each side of
    // each other), the unary arms, the calls, every unit in the table,
    // params of all four dimensions, and counts.
    for src in [
        // Leaves.
        "25 mm",
        "1 cm",
        "0.25 m",
        "1 in",
        "90 deg",
        "1.5 rad",
        "0.5 pi rad",
        "2.0",
        "1e-9",
        "7",
        "w",
        "a",
        "s",
        "n",
        // Sums, left-nested (the parser's own associativity) and
        // right-nested (which only parentheses can express).
        "w + 3 mm",
        "w - 3 mm - 1 mm",
        "w - (3 mm - 1 mm)",
        "w + (3 mm + 1 mm)",
        "w + 3 mm - 1 mm",
        // Products against sums, both sides.
        "(w + 3 mm) * 2.0",
        "2.0 * (w + 3 mm)",
        "(w + 3 mm) / 2.0",
        "w * 2.0 + 3 mm",
        "3 mm + w * 2.0",
        // Products against products.
        "2.0 * 3.0 * 4.0",
        "2.0 * (3.0 * 4.0)",
        "24.0 / 2.0 / 3.0",
        "24.0 / (2.0 * 3.0)",
        "24.0 / (2.0 / 3.0)",
        "w / 2.0 * 3.0",
        // Negation against everything.
        "-w",
        "--w",
        "-(w + 3 mm)",
        "-(w * 2.0)",
        "-w * 2.0",
        "w * -2.0",
        "w / -2.0",
        "w - -3 mm",
        "-7",
        "-n + 2",
        // Calls, and calls as operands.
        "sin(a)",
        "cos(a) * w",
        "tan(a + 30 deg)",
        "atan2(w, 3 mm)",
        "min(w, 3 mm) + max(1 mm, w)",
        "scalar(n) * w",
        "scalar(n + 2) * 2.0",
        "min(w - 1 mm, -(w + 1 mm))",
        "sin(atan2(w, 3 mm)) * 2.0",
        // Counts are their own closed arithmetic.
        "n * n + 3",
        "max(n, 4) - 1",
    ] {
        let e = rp(src);
        round_trip(&e);
    }
}

#[test]
fn unparse_parenthesises_exactly_where_the_grammar_needs_it() {
    // The text is pinned for the shapes where a naive rendering would
    // reparse to a DIFFERENT tree, and the naive rendering is checked
    // to actually be wrong — a parenthesis nobody needs is noise, and
    // one that is missing is a silent edit to the user's document.
    for (src, expected, naive) in [
        (
            "24.0 / (2.0 * 3.0)",
            "24.0 / (2.0 * 3.0)",
            "24.0 / 2.0 * 3.0",
        ),
        ("-(w + 3 mm)", "-(w + 3 mm)", "-w + 3 mm"),
        ("w - (3 mm - 1 mm)", "w - (3 mm - 1 mm)", "w - 3 mm - 1 mm"),
        ("w + (3 mm + 1 mm)", "w + (3 mm + 1 mm)", "w + 3 mm + 1 mm"),
        ("2.0 * (3.0 * 4.0)", "2.0 * (3.0 * 4.0)", "2.0 * 3.0 * 4.0"),
        ("(w + 3 mm) * 2.0", "(w + 3 mm) * 2.0", "w + 3 mm * 2.0"),
        ("-(w * 2.0)", "-(w * 2.0)", "-w * 2.0"),
    ] {
        let e = rp(src);
        assert_eq!(unparse(&e), expected);
        assert!(
            !rp(naive).bit_eq(&e),
            "{naive:?} is not actually a wrong reading of {src:?}"
        );
    }
    // And the ones that need NO parentheses do not grow any: the
    // parser's own associativity already says them.
    for src in [
        "w - 3 mm - 1 mm",
        "w * 2.0 + 3 mm",
        "3 mm + w * 2.0",
        "24.0 / 2.0 / 3.0",
        "-w * 2.0",
        "w * -2.0",
        "sin(a) * 2.0",
    ] {
        assert_eq!(unparse(&rp(src)), src);
    }
}

#[test]
fn unparse_writes_a_literal_in_the_unit_it_remembers() {
    // `bit_eq` is display-unit-blind (D7: the unit is presentation
    // metadata), so the notation is asserted on its own.
    for (src, symbol) in [
        ("25 mm", "mm"),
        ("2.5 cm", "cm"),
        ("1 in", "in"),
        ("0.25 m", "m"),
        ("90 deg", "deg"),
        ("1.5 rad", "rad"),
        ("0.5 pi rad", "pi rad"),
    ] {
        let e = rp(src);
        let text = round_trip(&e);
        assert_eq!(text, src);
        assert_eq!(
            parse_expr(&text, &rt_params())
                .expect(src)
                .display_unit()
                .map(|u| u.symbol()),
            Some(symbol)
        );
    }
    // A literal that remembers NO unit is written canonically and
    // comes back remembering the canonical one — the suffix is not
    // optional (a bare real is a Scalar in this grammar), so the round
    // trip is bit-exact on the value and names `m`/`rad` on the way.
    let bare = Expr::literal(0.025, Dimension::Length).expect("finite length");
    assert_eq!(bare.display_unit(), None);
    let text = round_trip(&bare);
    assert_eq!(text, "0.025 m");
    assert_eq!(
        rp(&text).display_unit().map(|u| u.symbol()),
        Some("m"),
        "the canonical suffix is what the reparse remembers"
    );
}

#[test]
fn a_negative_literal_is_the_one_shape_this_grammar_cannot_spell() {
    // The grammar has no negative number TOKEN — `-` is always an
    // operator — so a negative literal's own source text reads back as
    // the negation of its magnitude: same value, one node deeper.
    // Pinned rather than papered over (`unparse`'s docs state it).
    let negative = Expr::literal(-0.025, Dimension::Length).expect("finite length");
    let text = unparse(&negative);
    assert_eq!(text, "-0.025 m");
    let back = rp(&text);
    assert!(!back.bit_eq(&negative));
    assert_eq!(
        ev(&back),
        ev(&negative),
        "the two spellings evaluate identically"
    );
    // The same for a negative count, and `i64::MIN` refuses outright —
    // its magnitude is one past `i64::MAX`, the corner
    // `ParseError::IntegerOverflow`'s docs already record.
    let count = Expr::count(-7);
    assert_eq!(unparse(&count), "-7");
    assert!(!rp("-7").bit_eq(&count));
    assert_eq!(eval_count::<f64>(&rp("-7"), &ParamEnv::default()), Ok(-7));
    assert!(matches!(
        parse_expr(&unparse(&Expr::count(i64::MIN)), &rt_params()),
        Err(ParseError::IntegerOverflow { .. })
    ));
}

proptest! {
    /// The round trip over the grammar's whole generated span: any
    /// source the generator emits parses, unparses, and reparses to
    /// the identical tree and the identical literal bits.
    #[test]
    fn any_grammar_text_survives_a_parse_unparse_parse(
        src in prop_oneof![
            Just(Dimension::Length),
            Just(Dimension::Angle),
            Just(Dimension::Scalar),
            Just(Dimension::Count),
        ]
        .prop_flat_map(|dim| arb_text_of(dim, 3)),
    ) {
        let mut params = BTreeMap::new();
        params.insert(ParamName::new("S"), Dimension::Scalar);
        params.insert(ParamName::new("N"), Dimension::Count);
        let e = parse_expr(&src, &params).expect(&src);
        let text = unparse(&e);
        let back = parse_expr(&text, &params).expect(&text);
        prop_assert!(back.bit_eq(&e), "{} -> {}", &src, &text);
    }
}

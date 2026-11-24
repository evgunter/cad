//! The shape report's texts: what one decision records at most, and
//! what a text says of the part it leaves out.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin};
use geom_core::sym::report::{
    DecisionShape, RENDER_BUDGET, ShapeOutcome, explain_depth, name_param, render_of,
    start_shape_report, take_shape_report,
};
use geom_core::sym::with_session;
use geom_core::{ParamSymbol, Real, Sym, Tol};

/// One text's share of the budget.
const SHARE: usize = RENDER_BUDGET / 3;

/// A parameter rendered by its name. The name is registered on a
/// thread-local nothing clears, which is safe because each row is its
/// own process and a name here always stands for one symbol.
fn param(name: &str, v: f64) -> Sym<f64> {
    let symbol = ParamSymbol::new(test_utils::symbol_id(name));
    name_param(symbol, name);
    Sym::param(symbol, v)
}

/// A parameter with no name registered.
fn unnamed(v: f64) -> Sym<f64> {
    Sym::param(ParamSymbol::new(test_utils::symbol_id("unnamed")), v)
}

/// A polynomial of `fan` square-root atoms, each over the polynomial
/// one level down, `levels` deep over one parameter: about `3·fan`
/// nodes a level, and `fan^levels` atom sites where every site is
/// spelled.
fn tower(fan: usize, levels: usize) -> Sym<f64> {
    let mut x = param("x", 2.0);
    for _ in 0..levels {
        x = (1..=fan)
            .map(|i| (x + Sym::from_f64(i as f64)).sqrt())
            .reduce(|a, b| a + b)
            .expect("a positive fan");
    }
    x
}

/// Every product of two of `n` parameters, each named in `name` bytes:
/// `n·(n−1)/2` terms with no atom among them.
fn wide(tag: char, n: u32, name: usize) -> Sym<f64> {
    let ps: Vec<Sym<f64>> = (0..n)
        .map(|i| {
            let name = format!("{}{i:02}", String::from(tag).repeat(name - 2));
            param(&name, 1.0 + f64::from(i))
        })
        .collect();
    let mut sum = Sym::from_f64(0.0);
    for (i, a) in ps.iter().enumerate() {
        for b in ps.iter().skip(i + 1) {
            sum = sum + *a * *b;
        }
    }
    sum
}

fn rendered(build: impl FnOnce() -> Sym<f64>) -> String {
    with_session(crate::m10_10_r2_probes::shipped_budget(), || {
        render_of(build().node()).expect("a session is installed")
    })
    .0
}

/// The one decision of `build`'s value against itself as a literal
/// (zero to the numeric channel, and no theorem), explained `levels`
/// deep.
fn decided(levels: usize, build: impl FnOnce() -> Sym<f64>) -> DecisionShape {
    let (mut shapes, _) = with_session(crate::m10_10_r2_probes::shipped_budget(), || {
        explain_depth(levels);
        start_shape_report();
        let t = build();
        let m = t - Sym::from_f64(t.value());
        let band = Band::linear(Tol::witness()).expect("a linear band");
        let _ = geom_core::k_stats::decide("report_budget", Margin::of(m), band);
        explain_depth(0);
        take_shape_report()
    });
    assert_eq!(shapes.len(), 1, "one decision was taken");
    let shape = shapes.remove(0);
    assert_eq!(
        shape.outcome,
        ShapeOutcome::NumericZero,
        "the decision stayed numeric, so its residual is rendered"
    );
    shape
}

/// How deep the text's parentheses nest.
fn nesting(text: &str) -> usize {
    let (mut depth, mut deepest) = (0usize, 0usize);
    for c in text.chars() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        deepest = deepest.max(depth);
    }
    deepest
}

/// How many terms the mark ending `text` counts: `‹7 terms›` or
/// ` + ‹7 more terms›`.
fn counted(mark: &str) -> usize {
    mark.trim_start_matches(" + ")
        .strip_prefix('‹')
        .and_then(|m| m.strip_suffix('›'))
        .and_then(|m| m.split(' ').next())
        .and_then(|n| n.parse().ok())
        .unwrap_or_else(|| panic!("a mark counting terms, not {mark:?}"))
}

/// 241 nodes whose texts, every site spelled, are 16.7 MB each: the
/// decision records at most the budget, spelled as deep as fits, and
/// names the atoms it does not spell.
#[test]
fn a_deep_forms_decision_is_rendered_within_the_budget_and_as_deep_as_fits() {
    let shape = decided(6, || tower(16, 5));
    let plain = shape.form.as_deref().expect("a blocked decision's form");
    let early = shape
        .early_form
        .as_deref()
        .expect("the shipped rules run an early walk");
    let explain = shape
        .explain
        .as_deref()
        .expect("an explanation was asked for");
    let total = plain.len() + early.len() + explain.len();
    assert!(
        total <= RENDER_BUDGET,
        "one decision's texts take {total} bytes (plain {}, early {}, explain {}), past the \
         budget of {RENDER_BUDGET}",
        plain.len(),
        early.len(),
        explain.len()
    );
    assert!(
        plain.contains(" + 1·sqrt@") && plain.contains("‹17 terms›"),
        "an atom left unspelled is named by its head, its atom id and its argument's terms \
         (sixteen roots and a constant): {}",
        plain.chars().take(300).collect::<String>()
    );
    // Each level spelled multiplies the text by the fan-out, so the
    // level below the one chosen is the test of "deepest".
    assert_eq!(
        nesting(plain),
        2,
        "two levels of atoms are spelled: a third does not fit and one alone wastes the share"
    );
    assert!(
        16 * plain.len() > SHARE,
        "a third level, sixteen times these {} bytes, is past the share",
        plain.len()
    );
    let again = decided(6, || tower(16, 5));
    assert_eq!(
        (shape.form, shape.early_form, shape.explain),
        (again.form, again.early_form, again.explain),
        "the texts are a function of the form: a second session renders the same bytes"
    );
}

/// Polynomials whose terms alone pass a text's share, with no atom to
/// leave unspelled: each ends by counting the terms it left, and a
/// quotient keeps its frame around both counts.
#[test]
fn a_form_wider_than_the_budget_counts_the_terms_it_leaves() {
    // 66 terms of 4.8 kB.
    let num = || wide('p', 12, 2400);
    let text = rendered(num);
    assert!(
        text.len() <= SHARE,
        "a polynomial takes {} bytes",
        text.len()
    );
    let (written, mark) = text.rsplit_once(" + ").expect("more than one term");
    assert_eq!(
        written.split(" + ").count() + counted(mark),
        66,
        "the terms written and the terms counted are the polynomial's: {mark}"
    );

    // The numerator takes the share, so not one of the denominator's
    // 28 terms is written.
    let text = rendered(|| num() / wide('q', 8, 2400));
    assert!(text.len() <= SHARE, "a quotient takes {} bytes", text.len());
    let (top, bottom) = text
        .split_once(") / (")
        .expect("a quotient keeps its frame");
    assert!(top.starts_with("(1·p"), "the numerator opens the frame");
    assert_eq!(bottom, "‹28 terms›)", "the denominator, whole, is counted");
    let (written, mark) = top.rsplit_once(" + ").expect("more than one term");
    assert_eq!(
        written.split(" + ").count() + counted(mark),
        66,
        "the numerator's terms, written and counted: {mark}"
    );

    // A numerator that fits leaves the share to the denominator.
    let text = rendered(|| param("a", 1.25) / (num() + Sym::from_f64(1.0)));
    assert!(
        text.len() <= SHARE,
        "a wide denominator takes {} bytes",
        text.len()
    );
    let bottom = text
        .strip_prefix("(1·a) / (1 + 1·p")
        .and_then(|t| t.strip_suffix(')'))
        .expect("the numerator whole, then the denominator's first terms");
    let (written, mark) = bottom.rsplit_once(" + ").expect("more than one term");
    assert_eq!(
        2 + written.matches(" + ").count() + counted(mark),
        67,
        "the denominator's terms, written and counted: {mark}"
    );
}

/// Forty nodes each explained in a page and forty more over them: the
/// explanation ends inside its share, on a line that says where.
#[test]
fn an_explanation_past_its_share_ends_by_naming_the_node_it_stopped_before() {
    let shape = decided(64, || {
        let page = (0..40)
            .map(|i| param(&format!("{}{i:02}", "n".repeat(98)), 1.0 + f64::from(i)))
            .reduce(|a, b| a + b)
            .expect("forty parameters");
        (1..=40)
            .map(|k| (page * Sym::from_f64(f64::from(k))).sqrt())
            .reduce(|a, b| a + b)
            .expect("forty roots")
    });
    let explain = shape
        .explain
        .as_deref()
        .expect("an explanation was asked for");
    assert!(
        explain.len() <= SHARE,
        "the explanation takes {} bytes",
        explain.len()
    );
    let last = explain.lines().last().expect("a last line");
    let stack: usize = last
        .strip_prefix("‹the explanation ends before node #")
        .and_then(|l| l.strip_suffix(" more›"))
        .and_then(|l| l.split_once("; the walk's stack held "))
        .filter(|(node, _)| node.len() == 8 && node.chars().all(|c| c.is_ascii_hexdigit()))
        .and_then(|(_, stack)| stack.parse().ok())
        .unwrap_or_else(|| panic!("the last line says where the explanation ends: {last:?}"));
    assert!(stack > 0, "nodes were still to be walked when it ended");
}

/// Within the budget nothing is named or counted: every atom is
/// spelled to the nesting limit and `(…)` stands below it.
#[test]
fn a_form_within_the_budget_is_spelled_to_the_nesting_limit() {
    let a = || param("a", 1.25);
    let b = || param("b", 0.75);
    let one = || Sym::from_f64(1.0);
    for (what, text, expected) in [
        (
            "fan 2, three levels",
            rendered(|| tower(2, 3)),
            "1·sqrt(2 + 1·sqrt(1 + 1·sqrt(2 + 1·x) + 1·sqrt(1 + 1·x)) + 1·sqrt(2 + 1·sqrt(2 + \
             1·x) + 1·sqrt(1 + 1·x))) + 1·sqrt(1 + 1·sqrt(1 + 1·sqrt(2 + 1·x) + 1·sqrt(1 + 1·x)) \
             + 1·sqrt(2 + 1·sqrt(2 + 1·x) + 1·sqrt(1 + 1·x)))",
        ),
        (
            "fan 1, six levels: the fifth atom is past the nesting limit",
            rendered(|| tower(1, 6)),
            "1·sqrt(1 + 1·sqrt(1 + 1·sqrt(1 + 1·sqrt(1 + 1·sqrt(…)))))",
        ),
        (
            "a quotient",
            rendered(|| (a() + one()) / (b() + Sym::from_f64(3.0))),
            "(1 + 1·a) / (3 + 1·b)",
        ),
        (
            "powers, one of a parameter with no name",
            rendered(|| a().powi(3) * b().powi(2) + unnamed(0.5).powi(5)),
            "1·?#a08a1ecd^5 + 1·a^3·b^2",
        ),
        (
            "an atom of two arguments",
            rendered(|| a().sqrt().atan2(b() + one())),
            "1·atan2(1·sqrt(1·a), 1 + 1·b)",
        ),
        (
            "min under max",
            rendered(|| a().min(b()).max(unnamed(0.5).abs())),
            "1·max(1·min(1·a, 1·b), 1·abs(1·?#a08a1ecd))",
        ),
        (
            "trigonometric atoms and π",
            rendered(|| {
                a().sin() * a().cos() + (a() / Sym::from_f64(7.0)).tan() + Sym::<f64>::pi()
            }),
            "1·cos(1·a)·sin(1·a) + 1·π + 1·tan((1·a) / (7))",
        ),
        (
            "a quotient inside an atom",
            rendered(|| (a() / (b() + one())).sqrt().powi(3)),
            "1·sqrt((1·a) / (1 + 1·b))^3",
        ),
    ] {
        assert_eq!(text, expected, "{what}");
    }
}

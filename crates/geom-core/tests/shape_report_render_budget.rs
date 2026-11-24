//! **The shape report's rendering of one decision is bounded as a
//! whole** (`geom_core::sym::report::RENDER_BUDGET`): an atom spells
//! its argument, so a text's size is the product of the fan-out over
//! the nesting, and a limit on the nesting alone bounds nothing.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin};
use geom_core::sym::report::{
    RENDER_BUDGET, ShapeOutcome, explain_depth, name_param, render_of, start_shape_report,
    take_shape_report,
};
use geom_core::sym::with_session;
use geom_core::{ParamSymbol, Real, Sym, Tol};

fn param(name: &str, v: f64) -> Sym<f64> {
    let symbol = ParamSymbol::new(test_utils::symbol_id(name));
    name_param(symbol, name);
    Sym::param(symbol, v)
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

fn rendered(build: impl FnOnce() -> Sym<f64>) -> String {
    with_session(crate::m10_10_r2_probes::shipped_budget(), || {
        render_of(build().node()).expect("a session is installed")
    })
    .0
}

/// 241 nodes whose text, every site spelled four atoms deep, is
/// 16.7 MB: the decision that reads it records at most the budget, and
/// what it leaves out it names.
#[test]
fn a_deep_forms_decision_is_rendered_within_the_budget() {
    let decide = || {
        with_session(crate::m10_10_r2_probes::shipped_budget(), || {
            start_shape_report();
            explain_depth(6);
            let t = tower(16, 5);
            // The tower against its own value as a literal: zero to
            // the numeric channel, and no theorem.
            let m = t - Sym::from_f64(t.value());
            let band = Band::linear(Tol::witness()).expect("a linear band");
            let _ = geom_core::k_stats::decide("report_budget", Margin::of(m), band);
            explain_depth(0);
            take_shape_report()
        })
        .0
    };
    let shapes = decide();
    let [shape] = shapes.as_slice() else {
        panic!("one decision was taken, the report holds {}", shapes.len());
    };
    assert_eq!(
        shape.outcome,
        ShapeOutcome::NumericZero,
        "the decision stayed numeric, so its residual is rendered"
    );
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
        plain.contains("sqrt#") && plain.contains(" terms›"),
        "an atom the budget left unspelled is named by its head, its node and its size: {}",
        &plain[..plain.len().min(300)]
    );
    let again = decide();
    assert_eq!(
        (plain, early, explain),
        (
            again[0].form.as_deref().expect("form"),
            again[0].early_form.as_deref().expect("early"),
            again[0].explain.as_deref().expect("explain"),
        ),
        "the texts are a function of the form: a second session renders the same bytes"
    );
}

/// A polynomial whose terms alone pass a text's share, with no atom to
/// leave unspelled: the text ends by counting the terms it left.
#[test]
fn a_form_wider_than_the_budget_counts_the_terms_it_leaves() {
    // Every product of two of 40 parameters, each named in 240 bytes:
    // 780 terms.
    let text = rendered(|| {
        let ps: Vec<Sym<f64>> = (0..40)
            .map(|i| param(&format!("{}{i:02}", "p".repeat(238)), 1.0 + f64::from(i)))
            .collect();
        let mut sum = Sym::from_f64(0.0);
        for (i, a) in ps.iter().enumerate() {
            for b in ps.iter().skip(i + 1) {
                sum = sum + *a * *b;
            }
        }
        sum
    });
    assert!(
        text.len() <= RENDER_BUDGET / 3,
        "780 terms render to {} bytes, past one text's share of the budget",
        text.len()
    );
    let tail = text.rsplit(" + ").next().expect("a last term");
    let left: usize = tail
        .strip_prefix("‹")
        .and_then(|t| t.strip_suffix(" more terms›"))
        .and_then(|n| n.parse().ok())
        .unwrap_or_else(|| panic!("the text ends by counting what it left, not with {tail:?}"));
    assert_eq!(
        text.split(" + ").count() - 1 + left,
        780,
        "the terms spelled and the terms counted are the form's terms"
    );
}

/// Within the budget nothing is named or counted: every atom is spelled
/// to the nesting limit and `(…)` stands below it.
#[test]
fn a_form_within_the_budget_is_spelled_to_the_nesting_limit() {
    assert_eq!(
        rendered(|| tower(2, 3)),
        "1·sqrt(2 + 1·sqrt(1 + 1·sqrt(2 + 1·x) + 1·sqrt(1 + 1·x)) + 1·sqrt(2 + 1·sqrt(2 + 1·x) \
         + 1·sqrt(1 + 1·x))) + 1·sqrt(1 + 1·sqrt(1 + 1·sqrt(2 + 1·x) + 1·sqrt(1 + 1·x)) + \
         1·sqrt(2 + 1·sqrt(2 + 1·x) + 1·sqrt(1 + 1·x)))",
        "fan 2, three levels"
    );
    assert_eq!(
        rendered(|| tower(1, 6)),
        "1·sqrt(1 + 1·sqrt(1 + 1·sqrt(1 + 1·sqrt(1 + 1·sqrt(…)))))",
        "fan 1, six levels: the fifth atom is past the nesting limit"
    );
}

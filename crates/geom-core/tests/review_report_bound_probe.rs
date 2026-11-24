//! Probe: the shape report's texts over a spread of form shapes, dumped
//! to the file `REPORT_PROBE_OUT` names so two renderers can be compared
//! byte for byte. Asserts nothing; ignored outside that comparison
//! (`REPORT_PROBE_OUT=/path cargo nextest run -p geom-core -E
//! 'test(report_texts_are_dumped)' --run-ignored all`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin};
use geom_core::sym::report::{
    explain_depth, name_param, render_early_of, render_of, start_shape_report, take_shape_report,
};
use geom_core::sym::with_session;
use geom_core::{ParamSymbol, Real, Sym, Tol};

fn param(name: &str, v: f64, named: bool) -> Sym<f64> {
    let symbol = ParamSymbol::new(test_utils::symbol_id(name));
    if named {
        name_param(symbol, name);
    }
    Sym::param(symbol, v)
}

fn tower(fan: usize, levels: usize) -> Sym<f64> {
    let mut x = param("x", 2.0, true);
    for _ in 0..levels {
        x = (1..=fan)
            .map(|i| (x + Sym::from_f64(i as f64)).sqrt())
            .reduce(|a, b| a + b)
            .expect("a positive fan");
    }
    x
}

/// Every product of two of `n` parameters, each named in 240 bytes.
fn wide(n: u32) -> Sym<f64> {
    let ps: Vec<Sym<f64>> = (0..n)
        .map(|i| {
            param(
                &format!("{}{i:02}", "p".repeat(238)),
                1.0 + f64::from(i),
                true,
            )
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

type Build = fn() -> Sym<f64>;

fn shapes() -> Vec<(&'static str, Build)> {
    let a = || param("a", 1.25, true);
    let b = || param("b", 0.75, true);
    let u = || param("unnamed", 0.5, false);
    let _ = (a, b, u);
    vec![
        ("zero", || param("a", 1.25, true) - param("a", 1.25, true)),
        ("quotient", || {
            (param("a", 1.25, true) + Sym::from_f64(1.0))
                / (param("b", 0.75, true) + Sym::from_f64(3.0))
        }),
        ("third", || param("a", 1.25, true) / Sym::from_f64(3.0)),
        ("tenth", || param("a", 1.25, true) * Sym::from_f64(0.1)),
        ("big", || param("a", 1.25, true) * Sym::from_f64(1.0e30)),
        ("powers", || {
            param("a", 1.25, true).powi(3) * param("b", 0.75, true).powi(2)
                + param("unnamed", 0.5, false).powi(5)
        }),
        ("atan2", || {
            (param("a", 1.25, true).sqrt()).atan2(param("b", 0.75, true) + Sym::from_f64(1.0))
        }),
        ("minmax", || {
            param("a", 1.25, true)
                .min(param("b", 0.75, true))
                .max(param("unnamed", 0.5, false).abs())
        }),
        ("copysign", || {
            param("a", 1.25, true).copysign(param("b", 0.75, true) - Sym::from_f64(2.0))
        }),
        ("trig", || {
            let t = param("a", 1.25, true);
            t.sin() * t.cos() + (t / Sym::from_f64(7.0)).tan() + Sym::<f64>::pi()
        }),
        ("quotient_in_atom", || {
            (param("a", 1.25, true) / (param("b", 0.75, true) + Sym::from_f64(1.0)))
                .sqrt()
                .powi(3)
        }),
        ("nest6", || tower(1, 6)),
        ("nest_mixed", || {
            let mut x = param("a", 1.25, true);
            for i in 0..7 {
                x = if i % 2 == 0 {
                    (x + Sym::from_f64(1.0)).sqrt() + x.abs()
                } else {
                    (x * param("b", 0.75, true)).cos() / (x + Sym::from_f64(2.0))
                };
            }
            x
        }),
        ("tower_2_3", || tower(2, 3)),
        ("tower_3_5", || tower(3, 5)),
        ("tower_4_5", || tower(4, 5)),
        ("tower_4_8", || tower(4, 8)),
        ("tower_6_5", || tower(6, 5)),
        ("tower_7_5", || tower(7, 5)),
        ("tower_8_5", || tower(8, 5)),
        ("wide_quotient", || {
            wide(40) / (wide(30) + Sym::from_f64(1.0))
        }),
        ("wide_den", || {
            param("a", 1.25, true) / (wide(40) + Sym::from_f64(1.0))
        }),
    ]
}

#[test]
#[ignore = "a comparison artefact: run with REPORT_PROBE_OUT set"]
fn report_texts_are_dumped() {
    let Ok(out) = std::env::var("REPORT_PROBE_OUT") else {
        panic!("REPORT_PROBE_OUT names the file to write");
    };
    let mut dump = String::new();
    for (name, build) in shapes() {
        let (plain, early, shape) = with_session(crate::m10_10_r2_probes::shipped_budget(), || {
            let node = build().node();
            let plain = render_of(node).expect("a session");
            let early = render_early_of(node);
            start_shape_report();
            explain_depth(5);
            let t = build();
            let m = t - Sym::from_f64(t.value());
            let band = Band::linear(Tol::witness()).expect("a linear band");
            let _ = geom_core::k_stats::decide("probe", Margin::of(m), band);
            explain_depth(0);
            (plain, early, take_shape_report())
        })
        .0;
        dump.push_str(&format!("== {name} plain {}\n{plain}\n", plain.len()));
        dump.push_str(&format!("== {name} early\n{}\n", early.unwrap_or_default()));
        for s in shape {
            dump.push_str(&format!(
                "== {name} decision {:?}\nform: {}\nearly: {}\nexplain:\n{}\n",
                s.outcome,
                s.form.unwrap_or_default(),
                s.early_form.unwrap_or_default(),
                s.explain.unwrap_or_default()
            ));
        }
    }
    std::fs::write(out, dump).expect("the dump is written");
}

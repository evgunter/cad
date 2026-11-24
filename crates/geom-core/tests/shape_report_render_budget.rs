//! The shape report's rendering of one decision is bounded as a whole.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::sym::report::render_of;
use geom_core::sym::with_session;
use geom_core::{ParamSymbol, Real, Sym};

/// A polynomial of `fan` square-root atoms, each over the polynomial
/// one level down, `levels` deep over one parameter: `3·fan` nodes a
/// level, and `fan^levels` atom sites when every site is spelled.
fn tower(fan: usize, levels: usize) -> Sym<f64> {
    let mut x = Sym::param(ParamSymbol::new(test_utils::symbol_id("x")), 2.0);
    for _ in 0..levels {
        x = (1..=fan)
            .map(|i| (x + Sym::from_f64(i as f64)).sqrt())
            .reduce(|a, b| a + b)
            .expect("a positive fan");
    }
    x
}

fn rendered(fan: usize, levels: usize) -> String {
    let (text, _) = with_session(crate::m10_10_r2_probes::shipped_budget(), || {
        render_of(tower(fan, levels).node()).expect("a session is installed")
    });
    text
}

#[test]
#[ignore = "measurement"]
fn measure() {
    for (fan, levels) in [(2, 3), (4, 5), (8, 5), (12, 5), (16, 5), (8, 8)] {
        let t = std::time::Instant::now();
        let text = rendered(fan, levels);
        eprintln!(
            "fan {fan} levels {levels}: nodes ~{} rendered {} bytes in {:?}",
            3 * fan * levels + 1,
            text.len(),
            t.elapsed()
        );
        if text.len() < 400 {
            eprintln!("  {text}");
        }
    }
}

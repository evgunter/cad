//! **The bisected reads answer as the scans they replace.** A part of a
//! many-span net finds the knots it crosses ([`projected::span_stretches`])
//! and the span holding it ([`holding_span`]) by bisection, so the
//! pcurve door's reads cost the parts times a logarithm rather than the
//! parts times the spans. Each read is held, over a table of ranges at
//! and between the knots of a vector with repeated knots, to the linear
//! scan it replaced.

use super::*;
use geom_core::spline::ParamRange;

/// A clamped cubic vector with a double and a triple interior knot.
fn knots() -> KnotVector {
    let knots = vec![
        0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 0.5, 0.625, 0.75, 0.75, 0.75, 1.0, 1.0, 1.0, 1.0,
    ];
    KnotVector::clamped(knots, 3).unwrap()
}

/// Every pair `(a, b)`, `a ≤ b`, of the knots, the midpoints between
/// them and the domain's two ends pushed a little past.
fn ranges() -> Vec<(f64, f64)> {
    let mut at: Vec<f64> = knots().breakpoints().collect();
    let mids: Vec<f64> = at.windows(2).map(|w| 0.5 * (w[0] + w[1])).collect();
    at.extend(mids);
    at.extend([-0.1, 1.1]);
    at.sort_by(f64::total_cmp);
    let mut out = Vec::new();
    for (i, &a) in at.iter().enumerate() {
        for &b in &at[i..] {
            out.push((a, b));
        }
    }
    out
}

/// The scan [`projected::span_stretches`] replaced.
fn stretches_by_scan(kv: &KnotVector, range: ParamRange) -> Option<Vec<ParamRange>> {
    let (a, b) = (range.start(), range.end());
    let mut breaks = vec![a];
    breaks.extend(kv.breakpoints().filter(|&k| k > a && k < b));
    breaks.push(b);
    breaks
        .windows(2)
        .map(|w| ParamRange::new(w[0], w[1]))
        .collect()
}

#[test]
fn a_ranges_stretches_are_the_scans() {
    let kv = knots();
    let mut read = 0;
    for (a, b) in ranges() {
        let Some(range) = ParamRange::new(a, b) else {
            continue;
        };
        let got = projected::span_stretches(&kv, range);
        assert_eq!(
            got,
            stretches_by_scan(&kv, range),
            "stretches of [{a}, {b}]"
        );
        read += 1;
    }
    assert!(read > 80, "the table read {read} ranges");
}

/// The scan [`holding_span`] replaced.
fn holding_by_scan(breaks: &[f64], range: (f64, f64)) -> Option<usize> {
    breaks
        .windows(2)
        .position(|w| w[0] <= range.0 && range.1 <= w[1])
}

#[test]
fn a_parts_holding_span_is_the_scans() {
    // The distinct breakpoints, as a composite's breaks are, and the
    // knot list itself, whose repeated knots name empty spans.
    let kv = knots();
    let distinct: Vec<f64> = kv.breakpoints().collect();
    let mut found = 0;
    for breaks in [distinct.as_slice(), kv.knots()] {
        for range in ranges()
            .into_iter()
            .chain([(f64::NAN, 0.5), (0.25, f64::NAN)])
        {
            let got = holding_span(breaks, range);
            assert_eq!(
                got,
                holding_by_scan(breaks, range),
                "the span holding {range:?}"
            );
            found += usize::from(got.is_some());
        }
    }
    assert!(found > 20, "the table found {found} holding spans");
}

//! Reviewer probes (signed radius, r2). Each prints what it measured;
//! the one assertion is claim 1's: no arc a verb stores over a box holds
//! a radius whose enclosure reaches zero or below.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::sym::with_session_rules;
use geom_core::{Bounds, Interval, ParamSymbol, Point2, Real, Sym, SymBudget, SymRules, Tol};
use profile::test_support::{ARC_VERBS, arc_verb_loop};
use profile::{Bulge, Open, Profile, ProfileLoop, Segment, SketchPlane, Start};

fn tol() -> Tol {
    Tol::witness()
}

type Lane = Sym<Interval>;

fn session<R>(f: impl FnOnce() -> R) -> R {
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    with_session_rules(budget, SymRules::shipped(), f).0
}

fn param(name: &str, c: f64, h: f64) -> Lane {
    Sym::param(
        ParamSymbol::new(test_utils::symbol_id(name)),
        Interval::from_bounds(c - h, c + h),
    )
}

fn radii<T: Bounds + Real>(lp: &ProfileLoop<T>) -> Vec<(f64, f64)> {
    lp.segments()
        .iter()
        .filter_map(|s| match s {
            Segment::Arc(a) => Some((a.radius.lo(), a.radius.hi())),
            Segment::Line => None,
        })
        .collect()
}

/// Claim 1 as a search: every verb over boxes growing from its own to
/// where its door stops building, at `Interval` and at `Sym<Interval>`,
/// centred on its quantity and pushed toward each side.
#[test]
fn review_r2_no_verb_stores_a_radius_reaching_zero_over_a_box() {
    let mut bad = Vec::new();
    let mut built = 0;
    let mut refused = 0_u32;
    for (verb, c, h) in ARC_VERBS {
        for grow in [1.0, 3.0, 9.0, 27.0, 81.0] {
            for shift in [-0.9, 0.0, 0.9] {
                let (centre, half) = (c + shift * grow * h, grow * h);
                let plain = std::thread::spawn(move || {
                    radii(&arc_verb_loop::<Interval>(
                        verb,
                        Interval::from_bounds(centre - half, centre + half),
                        tol(),
                    ))
                })
                .join();
                let sym = std::thread::spawn(move || {
                    session(|| {
                        radii(&arc_verb_loop::<Lane>(
                            verb,
                            param("q", centre, half),
                            tol(),
                        ))
                    })
                })
                .join();
                for (lane, got) in [("interval", plain), ("sym", sym)] {
                    let got = got.map_err(|e| {
                        e.downcast_ref::<String>()
                            .cloned()
                            .or_else(|| e.downcast_ref::<&str>().map(|s| (*s).to_owned()))
                            .unwrap_or_default()
                    });
                    match got {
                        Ok(rs) => {
                            built += 1;
                            for (lo, hi) in rs {
                                if lo <= 0.0 || lo.is_nan() {
                                    bad.push(format!(
                                        "{verb} {lane} q {centre}±{half}: radius [{lo:e}, {hi:e}]"
                                    ));
                                }
                            }
                        }
                        Err(msg) => {
                            refused += 1;
                            if grow == 1.0 || grow == 3.0 {
                                let m: String = msg.chars().take(240).collect();
                                println!("{verb} {lane} q {centre}±{half}: {m}");
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{built} loops built, {refused} refused at the door");
    assert!(bad.is_empty(), "radii reaching zero:\n{}", bad.join("\n"));
}

/// Claim 4: a bulge boxed across zero through `arc_to(Bulge)` (the tie
/// arm), at `Interval` and `Sym<Interval>`, through the door and
/// validation.
#[test]
fn review_r2_a_bulge_boxed_across_zero() {
    for (lo, hi) in [(-0.05, 0.05), (-1e-12, 1e-12), (-0.05, 0.4), (1e-12, 0.4)] {
        let run = |b: Interval| {
            let p = |x: f64, y: f64| Point2::new(Interval::from_f64(x), Interval::from_f64(y));
            let lp = Open
                .at(p(0.0, 0.0))
                .line_to(p(4.0, 0.0), tol())
                .and_then(|o| o.arc_to(Bulge { p: p(4.0, 4.0), b }, tol()))
                .and_then(|o| o.line_to(p(0.0, 4.0), tol()))
                .and_then(|o| o.line_to(Start, tol()));
            match lp {
                Err(e) => format!(
                    "door: {}",
                    format!("{e:?}").chars().take(400).collect::<String>()
                ),
                Ok(c) => {
                    let lp = c.loop_.into_loop();
                    let r = radii(&lp);
                    let dirs: Vec<bool> = lp.directions().iter().map(Option::is_some).collect();
                    let v = Profile::new(SketchPlane::xy(), vec![lp]).validate(tol());
                    format!(
                        "radii {r:?} recorded {dirs:?} validate {}",
                        v.as_ref()
                            .map(|_| "ok".to_owned())
                            .unwrap_or_else(|e| format!("{e:?}"))
                            .chars()
                            .take(300)
                            .collect::<String>()
                    )
                }
            }
        };
        println!(
            "bulge [{lo}, {hi}] interval: {}",
            run(Interval::from_bounds(lo, hi))
        );
    }
}

/// Claim 1's edges: a via point boxed across its chord, a tangent
/// target boxed across the departure line.
#[test]
fn review_r2_a_side_boxed_across_its_line() {
    for (verb, c, h) in [
        ("via", 4.0, 0.1),
        ("via", 4.0, 1e-12),
        ("tangent", 0.0, 0.1),
        ("tangent", 0.0, 1e-12),
    ] {
        let got = std::thread::spawn(move || {
            session(|| radii(&arc_verb_loop::<Lane>(verb, param("q", c, h), tol())))
        })
        .join()
        .map_err(|e| {
            e.downcast_ref::<String>()
                .map(|s| s.chars().take(300).collect::<String>())
                .unwrap_or_default()
        });
        println!("{verb} q {c}±{h}: {got:?}");
    }
}

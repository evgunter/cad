//! Third review probe (PR 7710): sign factors in both normals, through
//! the tree's own ladder and pair door only, so the one file runs on the
//! head and on the merge base's sources.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr,
    clippy::type_complexity
)]

use super::*;
use crate::plane_eq::{PlaneDesc, plane_ladder};
use geom_core::{Band, Point3, Real, Sym, Tol, Vec3};

type S = Sym<f64>;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn row(
    max_terms: usize,
    build: &dyn Fn(&[S]) -> [PlaneDesc<S>; 2],
    exchanged: bool,
) -> [String; 3] {
    use geom_core::sym::deferred::{discharge_or_why, with_deferred_session};
    use geom_core::{ParamSymbol, SymBudget, SymRetry, SymRules};
    let budget = SymBudget {
        max_terms,
        max_degree: 64,
    };
    with_deferred_session(budget, SymRules::shipped(), SymRetry::none(), || {
        let vars: Vec<S> = (0..24)
            .map(|i| {
                S::param(
                    ParamSymbol::new(test_utils::symbol_id(&format!("x{i}"))),
                    1.0 + 0.1 * f64::from(i),
                )
            })
            .collect();
        let [p1, p2] = build(&vars);
        let (a, b) = if exchanged { (p2, p1) } else { (p1, p2) };
        let extent = ConsumedExtent::arm(S::from_f64(1.0));
        let ladder = plane_ladder(&a, &b, &extent, band())
            .ok()
            .and_then(CarrierVerdict::one);
        let door = pair_door_reading(
            &CarrierDesc::Plane(a),
            &CarrierDesc::Plane(b),
            &extent,
            band(),
        )
        .ok()
        .and_then(|r| r.verdict().one());
        // A designer's form, for claim 9: the feet's separation written
        // cross-wise, `‖d₁·n₂ − d₂·n₁‖`, linear in each normal's factor.
        let centre = extent.reach.center();
        let (d1, d2) = (
            a.normal.dot(a.origin - centre),
            b.normal.dot(b.origin - centre),
        );
        let crosswise = (b.normal * d1 - a.normal * d2).norm();
        let cw = match CoincidenceMeasure::decide(
            "plane_eq_offset",
            geom_core::Margin::of(crosswise),
            band(),
        ) {
            Ok(geom_core::Decided {
                sign: geom_core::Sign::Zero,
                margin,
            }) => match discharge_or_why(margin) {
                Ok(()) => "crosswise/proven".to_string(),
                Err(why) => format!("crosswise/{why:?}"),
            },
            other => format!("crosswise/{:?}", other.map(|d| d.sign).ok()),
        };
        let [l, d] = [ladder, door].map(|o| match o {
            None => "not-one".to_string(),
            Some((rel, m)) => match discharge_or_why(m) {
                Ok(()) => format!("{rel:?}/proven"),
                Err(why) => format!("{rel:?}/{why:?}"),
            },
        });
        [l, d, cw]
    })
    .0
}

fn sum(x: &[S], r: std::ops::Range<usize>) -> S {
    x[r].iter().copied().fold(S::from_f64(0.0), |a, b| a + b)
}

fn side(size: usize, x: &[S], s: usize) -> S {
    let k = S::from_f64;
    match size {
        0 => x[s],
        1 => x[s] * x[s + 1] - x[s + 2] * x[s + 3] + k(5.0),
        2 => sum(x, s..s + 4) * sum(x, s + 4..s + 8) - sum(x, s..s + 3) * sum(x, s + 3..s + 6),
        3 => sum(x, s..s + 8) * sum(x, s..s + 8) * sum(x, s..s + 4) - sum(x, s..s + 5),
        4 => sum(x, 0..24) - k(20.0),
        5 => sum(x, s..s + 4) * sum(x, s + 4..s + 8) * sum(x, s..s + 2) - sum(x, s..s + 5),
        6 => sum(x, 0..23) - k(20.0),
        7 => sum(x, 0..22) - k(20.0),
        _ => sum(x, 0..16) - k(10.0),
    }
}

#[test]
fn review3_factors_in_both_normals() {
    let k = S::from_f64;
    let unitise = |v: S| v / (v * v).sqrt();
    for size in 0..9usize {
        let shapes: Vec<(&str, Box<dyn Fn(&[S]) -> [PlaneDesc<S>; 2]>)> = vec![
            (
                "same factor",
                Box::new(move |x: &[S]| {
                    let lam = unitise(side(size, x, 0));
                    let y = -(x[20] / k(2.0));
                    [
                        PlaneDesc {
                            origin: Point3::new(k(0.0), k(0.0), y),
                            normal: Vec3::new(k(0.0), k(0.0), lam),
                        },
                        PlaneDesc {
                            origin: Point3::new(x[7], k(0.0), y),
                            normal: Vec3::new(k(0.0), k(0.0), lam),
                        },
                    ]
                }),
            ),
            (
                "same factor, opposed",
                Box::new(move |x: &[S]| {
                    let v = side(size, x, 0);
                    let y = -(x[20] / k(2.0));
                    [
                        PlaneDesc {
                            origin: Point3::new(k(0.0), k(0.0), y),
                            normal: Vec3::new(k(0.0), k(0.0), unitise(v)),
                        },
                        PlaneDesc {
                            origin: Point3::new(x[7], k(0.0), y),
                            normal: Vec3::new(k(0.0), k(0.0), unitise(-v)),
                        },
                    ]
                }),
            ),
            (
                "two factors",
                Box::new(move |x: &[S]| {
                    let la = unitise(side(size, x, 0));
                    let lb = unitise(side(size, x, 8));
                    let y = -(x[20] / k(2.0));
                    [
                        PlaneDesc {
                            origin: Point3::new(k(0.0), k(0.0), y),
                            normal: Vec3::new(k(0.0), k(0.0), la),
                        },
                        PlaneDesc {
                            origin: Point3::new(x[7], k(0.0), y),
                            normal: Vec3::new(k(0.0), k(0.0), lb),
                        },
                    ]
                }),
            ),
            (
                "literal against a factor",
                Box::new(move |x: &[S]| {
                    let lb = unitise(side(size, x, 0));
                    let y = -(x[20] / k(2.0));
                    [
                        PlaneDesc {
                            origin: Point3::new(k(0.0), k(0.0), y),
                            normal: Vec3::new(k(0.0), k(0.0), k(1.0)),
                        },
                        PlaneDesc {
                            origin: Point3::new(x[7], k(0.0), y),
                            normal: Vec3::new(k(0.0), k(0.0), lb),
                        },
                    ]
                }),
            ),
            (
                "one sloped chord's normal in both walls",
                Box::new(move |x: &[S]| {
                    let (dx, dy) = (side(size, x, 0), side(size.min(2), x, 8));
                    let len = (dx * dx + dy * dy).sqrt();
                    let n = Vec3::new(-(dy / len), dx / len, k(0.0));
                    let p = Point3::new(x[20], x[21], k(0.0));
                    let t = x[22];
                    [
                        PlaneDesc {
                            origin: p,
                            normal: n,
                        },
                        PlaneDesc {
                            origin: Point3::new(p.x + t * dx, p.y + t * dy, x[23]),
                            normal: n,
                        },
                    ]
                }),
            ),
        ];
        for (name, build) in &shapes {
            for max_terms in [256usize, 4096] {
                let stated = row(max_terms, build.as_ref(), false);
                let exchanged = row(max_terms, build.as_ref(), true);
                eprintln!(
                    "REVIEW3 size {size} | {name} | {max_terms} | ladder,door {stated:?} | \
                     exchanged {exchanged:?}"
                );
            }
        }
    }
}

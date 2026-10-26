//! Designer probe: three plates in a row, each neighbour pair flush
//! within ε, the two ends a band apart; an honest left fold of the
//! pair union in every member order.
#![allow(
    clippy::panic,
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::print_stdout
)]

use crate::common;
use common::{brick, finished};
use geom_core::Tol;
use topo::BooleanResult;

#[test]
fn honest_fold_of_three_plates_in_every_order() {
    let tol = Tol::witness();
    let eps = tol.eps();
    for frac in [0.3, 0.45, 0.6, 0.8, 2.0] {
        let d = frac * eps;
        let plate = |i: usize| {
            let x = i as f64;
            let z = d * i as f64;
            finished(
                "p",
                brick::<f64>((x, x + 1.0), (0.0, 1.0), (z, 1.0 + z), tol),
                tol,
            )
        };
        let ps = [plate(0), plate(1), plate(2)];
        for (i, j) in [(0, 1), (1, 2), (0, 2)] {
            let r = topo::union(&ps[i], &ps[j], tol);
            println!(
                "d={frac}ε pair {i}{j}: {}",
                match &r {
                    Ok(BooleanResult::Body(b)) => format!(
                        "body, {} faces, {} rows",
                        b.body.faces().count(),
                        b.coincidences.len()
                    ),
                    Ok(BooleanResult::Empty) => "empty".into(),
                    Err(e) => format!("refused {e:?}").chars().take(160).collect(),
                }
            );
        }
        for order in [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            let first = topo::union(&ps[order[0]], &ps[order[1]], tol);
            let out = match first {
                Ok(BooleanResult::Body(b)) => {
                    let acc = b.body;
                    match topo::union(&acc, &ps[order[2]], tol) {
                        Ok(BooleanResult::Body(b)) => {
                            format!("body, {} faces", b.body.faces().count())
                        }
                        Ok(BooleanResult::Empty) => "empty".into(),
                        Err(e) => format!("step 2 refused {e:?}").chars().take(160).collect(),
                    }
                }
                Ok(BooleanResult::Empty) => "empty".into(),
                Err(e) => format!("step 1 refused {e:?}").chars().take(160).collect(),
            };
            println!("d={frac}ε order {order:?}: {out}");
        }
    }
}

/// A and B a band apart across x = 1; C covers both faces.
#[test]
fn honest_fold_of_a_covered_in_band_gap_in_every_order() {
    let tol = Tol::witness();
    let g = 3.0 * tol.eps();
    let ps = [
        finished(
            "a",
            brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
            tol,
        ),
        finished(
            "b",
            brick::<f64>((1.0 + g, 2.0), (0.0, 1.0), (0.0, 1.0), tol),
            tol,
        ),
        finished(
            "c",
            brick::<f64>((0.5, 1.5), (-0.5, 1.5), (-0.5, 1.5), tol),
            tol,
        ),
    ];
    let show = |r: Result<BooleanResult<f64>, topo::BooleanError>| -> (String, Option<topo::AtRestBody<f64>>) {
        match r {
            Ok(BooleanResult::Body(b)) => (format!("body {} faces, {} rows", b.body.faces().count(), b.coincidences.len()), Some(b.body)),
            Ok(BooleanResult::Empty) => ("empty".into(), None),
            Err(e) => (format!("refused {e:?}").chars().take(110).collect(), None),
        }
    };
    for (i, j) in [(0, 1), (0, 2), (1, 2)] {
        println!(
            "covered pair {i}{j}: {}",
            show(topo::union(&ps[i], &ps[j], tol)).0
        );
    }
    for order in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let (s1, acc) = show(topo::union(&ps[order[0]], &ps[order[1]], tol));
        let out = match acc {
            Some(acc) => format!("step 2: {}", show(topo::union(&acc, &ps[order[2]], tol)).0),
            None => format!("step 1: {s1}"),
        };
        println!("covered order {order:?}: {out}");
    }
}

/// P1 ten long, P2 and P3 unit plates in a row along x; P3 rolled by
/// θ about the x-axis line through (·, 0, 1), so its wall against P2
/// (x = 11) maps to itself. Every point of P3 lies within θ·1 of the
/// level plane z = 1 (and every point of P1/P2 within θ·1 of P3's).
#[test]
fn honest_fold_of_a_rolled_plate_in_every_order() {
    let tol = Tol::witness();
    for frac in [0.2, 0.4, 0.8] {
        let th = frac * tol.eps();
        let roll = geom_core::RigidMotion::rotation_about_axis(
            geom_core::Point3::new(0.0, 0.0, 1.0),
            geom_core::UnitVec3::exact_x(),
            th,
        )
        .unwrap();
        let p3 = topo::transform_rigid(
            &brick::<f64>((11.0, 12.0), (0.0, 1.0), (0.0, 1.0), tol),
            &roll,
            tol,
        )
        .unwrap();
        let ps = [
            finished(
                "p1",
                brick::<f64>((0.0, 10.0), (0.0, 1.0), (0.0, 1.0), tol),
                tol,
            ),
            finished(
                "p2",
                brick::<f64>((10.0, 11.0), (0.0, 1.0), (0.0, 1.0), tol),
                tol,
            ),
            finished("p3", p3, tol),
        ];
        let show = |r: Result<BooleanResult<f64>, topo::BooleanError>| -> (String, Option<topo::AtRestBody<f64>>) {
            match r {
                Ok(BooleanResult::Body(b)) => (format!("body {} faces", b.body.faces().count()), Some(b.body)),
                Ok(BooleanResult::Empty) => ("empty".into(), None),
                Err(e) => {
                    let s = format!("{e:?}");
                    let m = s.find("margin").map_or("", |i| &s[i..]);
                    (format!("refused {} | {}", s.chars().take(70).collect::<String>(), m.chars().take(60).collect::<String>()), None)
                }
            }
        };
        for (i, j) in [(0, 1), (1, 2), (0, 2)] {
            println!(
                "roll {frac}ε pair {i}{j}: {}",
                show(topo::union(&ps[i], &ps[j], tol)).0
            );
        }
        for order in [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            let (s1, acc) = show(topo::union(&ps[order[0]], &ps[order[1]], tol));
            let out = match acc {
                Some(acc) => format!("step 2: {}", show(topo::union(&acc, &ps[order[2]], tol)).0),
                None => format!("step 1: {s1}"),
            };
            println!("roll {frac}ε order {order:?}: {out}");
        }
    }
}

/// One pair: a ten-unit plate and a unit plate beside it rolled by θ.
#[test]
fn a_long_plate_and_a_rolled_unit_plate_as_one_pair() {
    let tol = Tol::witness();
    for frac in [0.2, 0.4, 0.8] {
        let th = frac * tol.eps();
        let roll = geom_core::RigidMotion::rotation_about_axis(
            geom_core::Point3::new(0.0, 0.0, 1.0),
            geom_core::UnitVec3::exact_x(),
            th,
        )
        .unwrap();
        let q = finished(
            "q",
            topo::transform_rigid(
                &brick::<f64>((10.0, 11.0), (0.0, 1.0), (0.0, 1.0), tol),
                &roll,
                tol,
            )
            .unwrap(),
            tol,
        );
        let p = finished(
            "p",
            brick::<f64>((0.0, 10.0), (0.0, 1.0), (0.0, 1.0), tol),
            tol,
        );
        let r = match topo::union(&p, &q, tol) {
            Ok(BooleanResult::Body(b)) => format!("body {} faces", b.body.faces().count()),
            Ok(BooleanResult::Empty) => "empty".into(),
            Err(e) => {
                let s = format!("{e:?}");
                let m = s.find("margin").map_or("", |i| &s[i..]);
                format!(
                    "refused {} | {}",
                    s.chars().take(70).collect::<String>(),
                    m.chars().take(50).collect::<String>()
                )
            }
        };
        println!("long pair roll {frac}ε: {r}");
    }
}

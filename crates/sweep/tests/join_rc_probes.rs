//! The reflex-corner probe's rows, on [`crate::common::differential`]'s
//! poses: `a` the 315° reflex prism, `b` a sheared prism whose bottom
//! cap passes through `a`'s reflex corner `(0, 0, 1)`, flush-declared.
//! `join1_r1_probes::join1_r1_reflex_battery` runs the probe's twelve
//! profiles unturned; `rc_wide_battery` turns each about the corner as
//! well. `rc_detail` runs the one pose
//! `RC_CASE="<profile> <rot> <sx> <sy> <op>"` names (`cargo test -p
//! sweep --test all rc_detail -- --ignored --nocapture`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;

use crate::common::differential::{REFLEX_OPS, REFLEX_PROFILES, outcome, reflex_pose, reflex_run};

fn tol() -> Tol {
    Tol::witness()
}

#[test]
#[ignore = "detail probe: RC_CASE=\"<profile> <rot> <sx> <sy> <op>\""]
fn rc_detail() {
    let case = std::env::var("RC_CASE").expect("RC_CASE");
    let w: Vec<&str> = case.split_whitespace().collect();
    let num = |i: usize| -> f64 { w[i].parse().unwrap() };
    let p = reflex_pose(w[0], num(1), num(2), num(3), tol());
    let k = REFLEX_OPS.iter().position(|o| *o == w[4]).unwrap();
    let r = reflex_run(&p, w[4], tol());
    if let Ok(topo::BooleanResult::Body(bb)) = &r {
        println!(
            "{case} tier 3′: {:?}",
            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
        );
    }
    println!("{case} => {}", outcome(r, p.want[k], tol()));
}

/// The reflex probe widened: every profile also turned about the corner
/// by 0.003°, 7°, −20°, 33°, 100° and 190°, eleven shears per axis, all
/// four ops; one line per pose. `RCW_SHARD="k/n"` runs one shard of the
/// (profile, turn) pairs.
#[test]
#[ignore = "differential battery; run with --ignored --nocapture"]
fn rc_wide_battery() {
    let shard = std::env::var("RCW_SHARD").unwrap_or_else(|_| "0/1".into());
    let (k, n): (usize, usize) = {
        let w: Vec<usize> = shard.split('/').map(|x| x.parse().unwrap()).collect();
        (w[0], w[1])
    };
    let shears = [
        -0.75, -0.5, -0.3, -0.25, -0.1, 0.0, 0.1, 0.25, 0.3, 0.5, 0.75,
    ];
    let rots = [0.0, 0.003, 7.0, -20.0, 33.0, 100.0, 190.0];
    let mut idx = 0;
    for (name, _) in REFLEX_PROFILES {
        for rot in rots {
            idx += 1;
            if idx % n != k {
                continue;
            }
            for &sx in &shears {
                for &sy in &shears {
                    if sx == 0.0 && sy == 0.0 {
                        continue;
                    }
                    let p = reflex_pose(name, rot, sx, sy, tol());
                    for (op, want) in REFLEX_OPS.iter().zip(p.want) {
                        println!(
                            "RCW {name} rot={rot} sx={sx} sy={sy} {op} => {}",
                            outcome(reflex_run(&p, op, tol()), want, tol())
                        );
                    }
                }
            }
        }
    }
}

/// **A strut at the reflex corner faces its germs by their true angle
/// from the arrival edge.** Each pose puts both of a strut's germs in
/// `a`'s 315° top face, at least one more than a half-turn from the
/// corner's arrival edge `+x` (measured into the face); the turned
/// poses put a germ a hair from a half-turn bound. Every op builds a
/// body that passes tiers 2 and 3′ and the at-rest certificate, has the
/// closed-form volume, and is a legal operand.
#[test]
fn reflex_corner_struts_past_a_half_turn_build_sound() {
    for (profile, rot, sx, sy) in [
        ("sqQ1", 0.0, 0.5, -0.25),
        ("sqQ2", 0.0, 0.25, -0.25),
        ("sqQ2", 0.0, 0.5, 0.5),
        ("dUp", 0.0, 0.5, 0.0),
        ("dLeft", 0.0, 0.5, -0.5),
        ("eBot", 0.0, 0.25, 0.5),
        ("eLeft", 0.0, 0.5, -0.25),
        ("eRight", 0.0, -0.5, -0.5),
        ("sqQ2", 0.003, 0.25, -0.25),
        ("eRight", 0.003, -0.5, -0.5),
    ] {
        let p = reflex_pose(profile, rot, sx, sy, tol());
        for (k, op) in REFLEX_OPS[..3].iter().enumerate() {
            let what = format!("{profile} turned {rot}° (sx, sy) = ({sx}, {sy}) {op}");
            let bb = match reflex_run(&p, op, tol()) {
                Ok(topo::BooleanResult::Body(bb)) => bb,
                other => panic!("{what}: {other:?}"),
            };
            assert_eq!(topo::validate_closed(&bb.body), Ok(()), "{what}: tier 2");
            assert_eq!(
                topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()),
                Ok(()),
                "{what}: tier 3′"
            );
            assert!(
                topo::validate_geometric_certificate(&bb.body, tol()).is_ok(),
                "{what}: the at-rest certificate"
            );
            let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
            assert!(
                (v - p.want[k]).abs() < 1e-9,
                "{what}: volume {v} against the closed form {}",
                p.want[k]
            );
            sweep::test_support::assert_legal_operand(&what, &bb.body, tol());
        }
    }
}

/// **A flush-declared reflex union never ships the overlap twice.** At
/// these shears the corner's join refuses, and the declared-REST zip
/// reads the join's own segments, so it refuses with it. It used to
/// zip a body of volume `vol a + vol b`, which every gate passed. Each
/// pose either refuses or builds at the closed form, sound.
#[test]
fn flush_declared_reflex_unions_never_ship_the_overlap_twice() {
    for (sx, sy) in [(-0.5, 0.25), (-0.75, 0.1), (-0.3, 0.25), (-0.25, 0.1)] {
        let p = reflex_pose("sqQ1", 0.0, sx, sy, tol());
        let line = outcome(reflex_run(&p, "U", tol()), p.want[1], tol());
        assert!(
            line.starts_with("ERR") || line.starts_with("OK SOUND"),
            "sqQ1 (sx, sy) = ({sx}, {sy}) ∪: {line}"
        );
    }
}

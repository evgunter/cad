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

use crate::common::differential::{
    REFLEX_OPS, REFLEX_PROFILES, ReflexPose, outcome, reflex_pose, reflex_run,
};

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
        for op in &REFLEX_OPS[..3] {
            assert_sound(
                &p,
                op,
                reflex_run(&p, op, tol()),
                &format!("{profile} turned {rot}° (sx, sy) = ({sx}, {sy}) {op}"),
            );
        }
    }
}

/// **A hair off flush, the reflex corner's result passes its own tier
/// 3′.** Turned 0.003°, sqQ1's and dRight's corner edges run 5.2e-5 off
/// `a`'s, so `b`'s side faces sit 5.2e-5 off a schedule axis. The
/// census's in-plane walks on them read a far vertex's side
/// (`point_in_loop_side`, the sx = 0 rows) or, on the 1e-4 face where
/// the first row's `b` corner pokes 2.6e-5 through `a`'s floor, the
/// first member's arm (`point_in_loop_arm`), in band. Each is a reading
/// about one ray, and the walk takes the next.
///
/// At a coarse ε (1e-6) the hair itself is within ten bands, and the
/// boolean may refuse it as an in-band coincidence (`Escalated`); what
/// it builds must still be sound.
#[test]
fn reflex_corner_a_hair_off_flush_passes_its_own_census() {
    let coarse = tol().eps() >= 1e-7;
    let mut refused = 0;
    for (profile, sx, sy, op) in [
        ("sqQ1", -0.25, -0.75, "U"),
        ("sqQ1", 0.0, -0.1, "U"),
        ("sqQ1", 0.0, 0.5, "U"),
        ("dRight", 0.0, -0.5, "S_ab"),
    ] {
        let p = reflex_pose(profile, 0.003, sx, sy, tol());
        let what = format!("{profile} turned 0.003° (sx, sy) = ({sx}, {sy}) {op}");
        match reflex_run(&p, op, tol()) {
            Err(e @ topo::BooleanError::Escalated { .. }) if coarse => {
                println!("{what}: refused in band at ε = {}: {e:?}", tol().eps());
                refused += 1;
            }
            r => assert_sound(&p, op, r, &what),
        }
    }
    println!("{refused} of 4 refused in band");
}

/// `r`, `op` on `p`, is a body that passes tiers 2 and 3′ and the
/// at-rest certificate, has the closed-form volume, and is a legal
/// operand.
fn assert_sound(
    p: &ReflexPose,
    op: &str,
    r: Result<topo::BooleanResult<f64>, topo::BooleanError>,
    what: &str,
) {
    let k = REFLEX_OPS.iter().position(|o| *o == op).unwrap();
    let bb = match r {
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
    sweep::test_support::assert_legal_operand(what, &bb.body, tol());
}

//! **A move that carries a face through its neighbours inverts the
//! body, and the at-rest gate is what refuses it.**
//!
//! `topo::replace_face_offset` is a construction step: tier 2 in, tier
//! 2 out, `d` read along the chart's stored normal
//! (`crates/topo/README.md`, "Shell and offset surgery"). Tier 2 does
//! not read orientation, so a face moved past the face across the
//! solid from it builds a closed, certified, inside-out body and the
//! door answers `Ok`. That result becomes finished only through
//! `AtRestBody::validate`, which refuses it.
//!
//! Two fixtures, both annuli of revolution about `y`:
//!
//! - the frustum `(0.2, 0)-(0.4, 0)-(0.6, 0.6)-(0.2, 0.6)` (the
//!   opening-nappe fixture of `offd_r1_probes`), its cone moved `−0.3`:
//!   the cone's radii shrink by `0.3 / cos α = √10 / 10` and pass the
//!   bore at `r = 0.2`;
//! - the tube `[0.4, 0.8] × [0, 0.6]`, its outer cylinder moved `−0.5`
//!   to `r = 0.3`, past the inner one.
//!
//! The public verb over these moves is `topo::shell`, which moves every
//! face by one thickness rather than one face alone, so it cannot spell
//! either single-face move. It reaches the same class with a wall
//! thicker than half the material: the cavity's faces pass each other,
//! and the verb's closing at-rest gate refuses it typed.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom::Surface;
use geom_core::Tol;
use sweep::Revolution;
use sweep::test_support::{corners, revolved_about_y};
use topo::{AtRestBody, Body, FaceKey, ShellError, ValidationError, mass_properties};

fn revolved(points: &[(f64, f64)]) -> Body<f64> {
    revolved_about_y(corners(points), Revolution::Full, Tol::witness())
}

fn frustum() -> Body<f64> {
    revolved(&[(0.2, 0.0), (0.4, 0.0), (0.6, 0.6), (0.2, 0.6)])
}

fn tube() -> Body<f64> {
    revolved(&[(0.4, 0.0), (0.8, 0.0), (0.8, 0.6), (0.4, 0.6)])
}

fn face_where(body: &Body<f64>, want: impl Fn(&Surface<f64>) -> bool) -> FaceKey {
    body.faces()
        .find(|(_, f)| body.get_surface(f.surface).is_some_and(&want))
        .map(|(k, _)| k)
        .expect("the fixture has the face")
}

fn volume(body: &Body<f64>) -> f64 {
    mass_properties(body, Tol::witness()).unwrap().volume
}

/// Every error is `RingOutsideOuter`, and there are `n` of them.
fn rings_outside(what: &str, errors: &[ValidationError], n: usize) {
    assert!(
        errors.len() == n
            && errors
                .iter()
                .all(|e| matches!(e, ValidationError::RingOutsideOuter { .. })),
        "{what}: want {n} RingOutsideOuter, got {errors:?}"
    );
}

/// The door answers `Ok` on a move through a neighbour; the result's
/// signed volume is the closed form of the moved meridian, negative;
/// and `AtRestBody::validate` refuses it on the bore's rings, which lie
/// outside the moved face's.
#[test]
fn the_door_builds_the_inverted_body_and_the_gate_refuses_it() {
    let tol = Tol::witness();
    // The frustum's cone, moved along its outward chart normal: radii
    // `a`, `b` at the caps, the bore `0.2` under them.
    let s = 10f64.sqrt() / 10.0;
    let (a, b) = (0.4 - s, 0.6 - s);
    let frustum_v = 0.2 * PI * (a * a + a * b + b * b) - PI * 0.2 * 0.2 * 0.6;
    let tube_v = PI * (0.3f64.powi(2) - 0.4f64.powi(2)) * 0.6;
    let rows = [
        (
            "the frustum's cone at −0.3",
            frustum(),
            (|s: &Surface<f64>| matches!(s, Surface::Cone { .. })) as fn(&Surface<f64>) -> bool,
            -0.3,
            frustum_v,
            1,
        ),
        (
            "the tube's outer cylinder at −0.5",
            tube(),
            |s: &Surface<f64>| matches!(s, Surface::Cylinder { radius, .. } if *radius == 0.8),
            -0.5,
            tube_v,
            2,
        ),
    ];
    for (what, operand, pick, d, want_v, rings) in rows {
        AtRestBody::validate(operand.clone(), tol).expect("the operand is finished");
        let face = face_where(&operand, pick);
        let mut moved = operand;
        topo::replace_face_offset(&mut moved, face, d, tol)
            .unwrap_or_else(|e| panic!("{what}: the door builds through the neighbour, got {e}"));
        topo::validate_closed(&moved)
            .unwrap_or_else(|e| panic!("{what}: the result is tier 2, got {e:?}"));
        let v = volume(&moved);
        assert!(
            want_v < 0.0 && (v - want_v).abs() <= 1e-12,
            "{what}: signed volume {v}, want the moved meridian's {want_v}"
        );
        let errors = AtRestBody::validate(moved, tol)
            .expect_err(&format!("{what}: the inverted body is not finished"));
        rings_outside(what, &errors, rings);
    }
}

/// `topo::shell` reaches the class with a wall past half the material:
/// the cavity's faces cross, and the verb's closing gate refuses typed.
/// Under that wall the verb finishes, so the refusal is the crossing.
#[test]
fn the_shell_verb_refuses_the_crossing_at_its_closing_gate() {
    let tol = Tol::witness();
    let rows = [
        ("the frustum", frustum(), 0.1, 0.15, 1),
        ("the tube", tube(), 0.15, 0.25, 2),
    ];
    for (what, operand, finishes, crosses, rings) in rows {
        let operand = AtRestBody::validate(operand, tol).expect("the operand is finished");
        let shelled = topo::shell(&operand, finishes, tol)
            .unwrap_or_else(|e| panic!("{what}: a {finishes} wall shells, got {e}"));
        AtRestBody::validate(shelled.body, tol)
            .unwrap_or_else(|e| panic!("{what}: the {finishes} shell is finished, got {e:?}"));
        match topo::shell(&operand, crosses, tol) {
            Err(ShellError::NotValid { errors }) => rings_outside(what, &errors, rings),
            other => panic!(
                "{what}: a {crosses} wall crosses the cavity and refuses NotValid, got {:?}",
                other.map(|s| volume(&s.body))
            ),
        }
    }
    // The tube's finishing wall, closed form: the tube less its cavity
    // `[0.55, 0.65] × [0.15, 0.45]`.
    let tube = AtRestBody::validate(tube(), tol).unwrap();
    let v = volume(&topo::shell(&tube, 0.15, tol).unwrap().body);
    let want = PI * (0.8f64.powi(2) - 0.4f64.powi(2)) * 0.6
        - PI * (0.65f64.powi(2) - 0.55f64.powi(2)) * 0.3;
    assert!(
        (v - want).abs() <= 1e-12,
        "the tube's 0.15 shell: volume {v}, want {want}"
    );
}

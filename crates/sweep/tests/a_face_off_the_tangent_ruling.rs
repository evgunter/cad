//! **A face that does not reach the tangent ruling is no tangency**:
//! two carriers in band of tangent along a ruling, one of whose faces
//! stands off the ruling across it, read through every op in both
//! operand orders. The glue door reads the carriers over a reach that
//! encloses both faces, so the in-band reading is the carriers'; the
//! faces' own extents across the ruling settle it
//! (`geom_brep::tangent_locus_of_faces`). Each pose's control turns the
//! face back over the ruling, where the escalation stands.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Tol};

use crate::common::operands::{about_y, turned_about_z};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

fn at_rest(b: topo::Body<f64>) -> topo::AtRestBody<f64> {
    topo::AtRestBody::validate(b, tol()).unwrap()
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> topo::Body<f64> {
    topo::test_support::brick(x, y, z, tol())
}

fn op(
    k: u8,
    a: &topo::AtRestBody<f64>,
    b: &topo::AtRestBody<f64>,
) -> Result<(), topo::BooleanError> {
    match k {
        0 => topo::union(a, b, tol()).map(|_| ()),
        1 => topo::subtract(a, b, tol()).map(|_| ()),
        _ => topo::intersect(a, b, tol()).map(|_| ()),
    }
}

fn said(r: &Result<(), topo::BooleanError>) -> String {
    match r {
        Ok(()) => "Ok".into(),
        Err(e) => {
            let (key, _) = topo::test_support::offer_key(e);
            let pred = match e {
                topo::BooleanError::Escalated { diag, .. } => format!("{:?}", diag.predicate),
                _ => String::new(),
            };
            format!("{key} {pred}")
        }
    }
}

/// Every op in both operand orders, each answer as [`said`] spells it.
fn table(x: topo::Body<f64>, y: topo::Body<f64>) -> Vec<[String; 2]> {
    let (x, y) = (at_rest(x), at_rest(y));
    (0..3)
        .map(|k| [said(&op(k, &x, &y)), said(&op(k, &y, &x))])
        .collect()
}

/// Every answer of `rows` is `want`.
fn all_say(rows: &[[String; 2]], want: &str, why: &str) {
    for (k, row) in rows.iter().enumerate() {
        for (order, got) in row.iter().enumerate() {
            assert_eq!(got, want, "{why}: op {k}, operand order {order}");
        }
    }
}

/// The brick's top `5.5·ε` below the wall's carrier, slid off the
/// ruling by `dx`: the top face never reaches the ruling and stands
/// about `dx²/2` below the wall at its nearest edge, centimetres at
/// `dx = 0.3`, half a tenth of a millimetre at `dx = 0.01`.
#[test]
fn a_brick_beside_the_tube_ruling_is_no_tangency() {
    let gap = 5.5 * band().zero();
    for dx in [0.3, 0.1, 0.01] {
        let rows = table(
            about_y(tol(), (0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0),
            brick((dx, dx + 0.5), (0.2, 0.8), (-2.0, -1.0 - gap)),
        );
        all_say(&rows, "Ok", &format!("the brick {dx} m off the ruling"));
    }
}

/// The two cylinder arms. Inside: a quarter rod in a tube's bore, its
/// carrier `5.5·ε` inside the bore's tangency, its wall turned to face
/// away from the ruling. Outside: a quarter tube beside a tube, its
/// carrier `5.5·ε` clear of the outer tangency, its wall turned away,
/// the pair turned off the world's axes so the faces' boxes meet and
/// the door is asked. Each control turns the wall back over the ruling.
#[test]
fn a_cylinder_wall_turned_off_the_ruling_is_no_tangency() {
    let gap = 5.5 * band().zero();
    let quarter = core::f64::consts::FRAC_PI_2;
    let bore = || about_y(tol(), (0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0);
    let rod = |spin| {
        about_y(
            tol(),
            (0.0, 0.2),
            (0.2, 0.8),
            Some(quarter),
            spin,
            0.1 - gap,
        )
    };
    all_say(
        &table(bore(), rod(0.5 * quarter)),
        "Ok",
        "the rod's wall off the bore's ruling",
    );
    all_say(
        &table(bore(), rod(-0.5 * quarter)),
        "Coincidence(TangentLocus) Some(\"tangent_locus_internal_gap\")",
        "the rod's wall over the ruling",
    );
    let turn = 0.5;
    let tube = || {
        turned_about_z(
            tol(),
            about_y(tol(), (0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0),
            turn,
        )
    };
    let wall = |spin| {
        turned_about_z(
            tol(),
            about_y(
                tol(),
                (0.3, 1.0),
                (0.2, 0.8),
                Some(quarter),
                spin,
                2.0 + gap,
            ),
            turn,
        )
    };
    all_say(
        &table(tube(), wall(0.5 * quarter)),
        "Ok",
        "the outer wall off the ruling",
    );
    all_say(
        &table(tube(), wall(1.5 * quarter)),
        "Coincidence(TangentLocus) Some(\"cc_parallel_gap\")",
        "the outer wall over the ruling",
    );
}

/// No answer of `rows` is `Ok`.
fn none_ok(rows: &[[String; 2]]) -> bool {
    rows.iter().flatten().all(|r| r != "Ok")
}

/// No answer of `rows` is a frontier refusal.
fn no_frontier(rows: &[[String; 2]]) -> bool {
    rows.iter().flatten().all(|r| {
        !r.starts_with("CurvedBooleanUnsupported") && !r.starts_with("CurvedPierceUnsupported")
    })
}

/// **A face ending at the ruling is never read clear.** The brick's
/// top `5.5·ε` below the wall's carrier, its edge at `x = dx`: the
/// faces' least distance is `√(1 + dx²) − 1 + 5.5·ε`. Where the face
/// straddles the ruling or that distance is in band, no op answers
/// `Ok`; where it is well clear, every op does.
#[test]
fn a_brick_ending_at_the_ruling_is_read_by_its_reach() {
    let b = band();
    let gap = 5.5 * b.zero();
    for dx in [-0.1, 0.0, 1e-3 * b.zero().sqrt(), 1e2 * b.zero().sqrt()] {
        let rows = table(
            about_y(tol(), (0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0),
            brick((dx, dx + 0.5), (0.2, 0.8), (-2.0, -1.0 - gap)),
        );
        let apart = (1.0f64 + dx * dx).sqrt() - 1.0 + gap;
        if dx <= 0.0 || apart < b.escalate() {
            assert!(
                none_ok(&rows),
                "dx = {dx:e}, faces {apart:e} apart: {rows:?}"
            );
        } else {
            all_say(&rows, "Ok", &format!("dx = {dx:e}, faces {apart:e} apart"));
        }
    }
}

/// The two cylinder arms' walls ending `delta` off the ruling, on
/// either side of it: a quarter tube beside a tube (the pair turned
/// 0.5 rad about `z`) and a quarter rod in the bore. A wall reaching
/// the ruling, or ending in band of it, never answers `Ok`, and none
/// meets the frontier refusal.
#[test]
fn a_cylinder_wall_ending_at_the_ruling_is_read_by_its_reach() {
    let b = band();
    let gap = 5.5 * b.zero();
    let (q, pi) = (core::f64::consts::FRAC_PI_2, core::f64::consts::PI);
    // The walls bend apart by about `delta²` (outside) and at least
    // `0.01·delta²` (the rod in the bore) off the ruling.
    for delta in [0.0, 1e-3 * b.zero().sqrt(), 0.05] {
        let tube = || {
            turned_about_z(
                tol(),
                about_y(tol(), (0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0),
                0.5,
            )
        };
        for spin in [pi + delta, q - delta] {
            let wall = turned_about_z(
                tol(),
                about_y(tol(), (0.3, 1.0), (0.2, 0.8), Some(q), spin, 2.0 + gap),
                0.5,
            );
            let rows = table(tube(), wall);
            if delta * delta + gap < b.escalate() {
                assert!(none_ok(&rows), "outside, delta = {delta:e}: {rows:?}");
            }
            assert!(no_frontier(&rows), "outside, delta = {delta:e}: {rows:?}");
        }
        for spin in [delta, -q - delta] {
            let rows = table(
                about_y(tol(), (0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0),
                about_y(tol(), (0.0, 0.2), (0.2, 0.8), Some(q), spin, 0.1 - gap),
            );
            if 0.01 * delta * delta + gap < b.escalate() {
                assert!(none_ok(&rows), "inside, delta = {delta:e}: {rows:?}");
            }
            assert!(no_frontier(&rows), "inside, delta = {delta:e}: {rows:?}");
        }
    }
}

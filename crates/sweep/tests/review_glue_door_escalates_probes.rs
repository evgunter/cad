//! **PR 4918 review probes** — poses beyond the PR's three, read for
//! what the glue door now does with an in-band tangency. Each row
//! prints its outcome; the asserting rows pin what the review found.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Band, Point2, Tol, Vec3};
use sweep::test_support::revolved_about_y;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

fn at_rest(b: topo::Body<f64>) -> topo::AtRestBody<f64> {
    topo::AtRestBody::validate(b, tol()).unwrap()
}

/// A tube about `y`: radii `inner` (0 for a rod) to `outer`, `y` over
/// `[lo, hi]`, its axis moved to `(x, ·, 0)`.
fn tube(inner: f64, outer: f64, lo: f64, hi: f64, x: f64) -> topo::AtRestBody<f64> {
    let profile = if inner == 0.0 {
        vec![
            (Point2::new(0.0, lo), 0.0),
            (Point2::new(outer, lo), 0.0),
            (Point2::new(outer, hi), 0.0),
            (Point2::new(0.0, hi), 0.0),
        ]
    } else {
        vec![
            (Point2::new(inner, lo), 0.0),
            (Point2::new(outer, lo), 0.0),
            (Point2::new(outer, hi), 0.0),
            (Point2::new(inner, hi), 0.0),
        ]
    };
    let body = revolved_about_y(profile, sweep::Revolution::Full, tol());
    let body = if x == 0.0 {
        body
    } else {
        topo::transform_rigid(&body, &Affine3::translation(Vec3::new(x, 0.0, 0.0)), tol()).unwrap()
    };
    at_rest(body)
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> topo::AtRestBody<f64> {
    at_rest(topo::test_support::brick(x, y, z, tol()))
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

/// Every op, both operand orders.
fn table(name: &str, x: &topo::AtRestBody<f64>, y: &topo::AtRestBody<f64>) -> Vec<String> {
    let mut out = Vec::new();
    for k in 0..3 {
        let xy = op(k, x, y);
        let yx = op(k, y, x);
        let row = format!("{name} op{k}: [{}] / [{}]", said(&xy), said(&yx));
        println!("{row}");
        out.push(row);
    }
    out
}

/// Claim 1: plane×cylinder, outer wall, the brick below at multiples of
/// the band's zero.
#[test]
fn probe_brick_below_a_tube_at_multiples() {
    let b = band();
    println!("zero {:e} escalate {:e}", b.zero(), b.escalate());
    for m in [0.5, 2.0, 5.5, 9.0, 12.0] {
        let gap = m * b.zero();
        table(
            &format!("below gap={m}z"),
            &tube(0.3, 1.0, 0.0, 1.0, 0.0),
            &brick((-0.5, 0.5), (0.2, 0.8), (-2.0, -1.0 - gap)),
        );
    }
}

/// Claim 1: cylinder×cylinder outside tangency, two tubes side by side.
#[test]
fn probe_two_tubes_outside() {
    let b = band();
    for m in [0.5, 2.0, 5.5, 9.0] {
        let gap = m * b.zero();
        table(
            &format!("cc outside gap={m}z"),
            &tube(0.3, 1.0, 0.0, 1.0, 0.0),
            &tube(0.3, 1.0, 0.2, 0.8, 2.0 + gap),
        );
    }
}

/// Claim 1: cylinder×cylinder inside tangency, a rod in a tube's bore
/// leaning on the bore.
#[test]
fn probe_rod_in_a_bore() {
    let b = band();
    for m in [0.5, 2.0, 5.5, 9.0] {
        let gap = m * b.zero();
        table(
            &format!("cc inside gap={m}z"),
            &tube(0.3, 1.0, 0.0, 1.0, 0.0),
            &tube(0.0, 0.2, 0.2, 0.8, 0.1 - gap),
        );
    }
}

/// Claim 2: the brick's top in band of the tube's carrier but past the
/// tube's END by `s`, a definite distance beyond the band yet inside
/// the two boxes' pads: the faces certainly do not touch.
#[test]
fn probe_brick_past_the_tube_end() {
    let b = band();
    let gap = 5.5 * b.zero();
    for s_mul in [1.5, 2.2, 3.0] {
        let s = s_mul * b.escalate();
        table(
            &format!("past end s={s_mul}esc"),
            &tube(0.3, 1.0, 0.0, 1.0, 0.0),
            &brick((-0.5, 0.5), (1.0 + s, 1.6), (-2.0, -1.0 - gap)),
        );
    }
}

/// The same, far past the end: control (boxes do not meet).
#[test]
fn probe_brick_far_past_the_tube_end() {
    let b = band();
    let gap = 5.5 * b.zero();
    table(
        "far past end",
        &tube(0.3, 1.0, 0.0, 1.0, 0.0),
        &brick((-0.5, 0.5), (1.1, 1.6), (-2.0, -1.0 - gap)),
    );
}

/// Claim 2: the brick's top in band of the tube's carrier, but the
/// brick moved sideways off the tangent ruling by `dx`: its top face
/// never reaches the ruling, and stands `≈ dx²/2` below the wall at its
/// nearest edge, definitely clear for every `dx` here.
#[test]
fn probe_brick_beside_the_ruling() {
    let b = band();
    let gap = 5.5 * b.zero();
    let mut refused = Vec::new();
    for dx in [0.3, 0.1, 0.01] {
        let rows = table(
            &format!("beside dx={dx}"),
            &tube(0.3, 1.0, 0.0, 1.0, 0.0),
            &brick((dx, dx + 0.5), (0.2, 0.8), (-2.0, -1.0 - gap)),
        );
        refused.extend(rows.into_iter().filter(|r| !r.ends_with("[Ok] / [Ok]")));
    }
    // Every op passed before PR 4918 (measured with the door's swallow
    // restored); the faces are centimetres apart at dx = 0.3.
    assert!(
        refused.is_empty(),
        "faces that never touch refuse:\n{}",
        refused.join("\n")
    );
}

/// The same brick beside the ruling, its top in the ZERO band of the
/// tube's carrier: the door reads the carriers tangent though the brick
/// face never reaches the ruling. Printed only (pre-existing door).
#[test]
fn probe_brick_beside_the_ruling_in_the_zero_band() {
    let b = band();
    let gap = 0.5 * b.zero();
    for dx in [0.3, 0.01] {
        table(
            &format!("beside0 dx={dx}"),
            &tube(0.3, 1.0, 0.0, 1.0, 0.0),
            &brick((dx, dx + 0.5), (0.2, 0.8), (-2.0, -1.0 - gap)),
        );
    }
}

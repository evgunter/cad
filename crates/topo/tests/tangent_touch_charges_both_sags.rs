//! **The touch's span line charges the SUM of the two surfaces' sags.**
//! A unit sphere in a unit bore, its centre `δ` off the bore's axis,
//! read along the circle halfway between them: the pair's separation is
//! `−δ·cos t` (crossing by `δ` at `t = 0`, apart by `δ` at `t = π`), each
//! residual half of it. Every sample reads the touch within ε when
//! `δ ≤ ε`; the span line `max|sᵢ| + gap_sag` is what refuses near the
//! band's edge, and only because `gap_sag` adds both surfaces' second
//! derivatives (the larger residual sag alone would pass it). The table
//! reads a span line past ε again on finer schedules, so the refusal
//! stands only within the finest schedule's sag of the band's edge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::boolean::contact_verify::tangent_locus_relation;
use topo::contact::ContactRefusal;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn off_axis_ball(delta: f64) -> (Surface<f64>, Surface<f64>, Curve3<f64>) {
    let ball = Surface::Sphere {
        center: Point3::new(delta, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let bore = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let halfway = Curve3::Circle {
        center: Point3::new(0.5 * delta, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    (ball, bore, halfway)
}

fn read(delta: f64) -> Result<geom_core::MarginDiag, ContactRefusal> {
    let (ball, bore, halfway) = off_axis_ball(delta);
    // The bore's face looks into the hole: its outward normal is the
    // cylinder's inward one, opposed to the ball's.
    tangent_locus_relation(
        &ball,
        true,
        &bore,
        false,
        &halfway,
        0.0,
        core::f64::consts::TAU,
        band(),
    )
}

#[test]
fn the_touch_between_samples_charges_both_surfaces_sags() {
    let eps = band().zero();
    let tau = core::f64::consts::TAU;
    let (ball, bore, halfway) = off_axis_ball(0.95 * eps);
    let bounds = geom_brep::tangent_span_bounds(&ball, &bore, &halfway, 0.0, tau).unwrap();
    // The nine-sample schedule's span line is in band at 0.95ε: a finer
    // schedule settles it, the true separation being within ε.
    assert!(0.95 * eps + bounds.gap_sag > eps, "{bounds:?}");
    for delta in [0.5 * eps, 0.95 * eps] {
        assert!(read(delta).is_ok(), "{delta:e}: {:?}", read(delta));
    }
    // The table's finest schedule: `2^REFINE_HALVINGS` spans to each
    // of the nine-sample schedule's.
    // ε, the span line escalates on the sum and the max would pass.
    let finest =
        geom_brep::tangent_span_bounds(&ball, &bore, &halfway, 0.0, tau / finest_spans()).unwrap();
    // The window scales with ε; a unit circle's samples round by about
    // an ulp of 1, so under that the row has no window to stand in.
    let window = finest.gap_sag - finest.residual_sag;
    if window < 8.0 * f64::EPSILON {
        println!(
            "SKIPPED at ε = {eps:e}: the finest schedule's window between its sags, \
             {window:e}, is under the samples' rounding; the span line's sum is not \
             read at this ε"
        );
        return;
    }
    let delta = eps - 0.5 * (finest.residual_sag + finest.gap_sag);
    assert!(delta + finest.residual_sag <= eps, "{finest:?}");
    assert!(delta + finest.gap_sag > eps, "{finest:?}");
    match read(delta) {
        Err(ContactRefusal::Escalated { diag }) => {
            assert_eq!(diag.predicate, Some("contact_tangent_touch"), "{diag:?}");
        }
        other => panic!("{delta:e}: the span line refuses on the sag: {other:?}"),
    }
}

/// How many spans the table's finest schedule reads to each of the
/// nine-sample schedule's.
fn finest_spans() -> f64 {
    f64::from(1u32 << topo::boolean::contact_verify::REFINE_HALVINGS)
}

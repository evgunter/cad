//! Review of PR 4640: the touch's span line charges the SUM of the two
//! surfaces' sags. A unit sphere in a unit bore, its centre `δ` off the
//! bore's axis, read along the circle halfway between them: the pair's
//! separation is `−δ·cos t` (crossing by `δ` at `t = 0`, apart by `δ`
//! at `t = π`), each residual half of it. Every sample reads the touch
//! within ε when `δ ≤ ε`; the span line `max|sᵢ| + gap_sag` is what
//! refuses near the band's edge, and only because `gap_sag` adds both
//! surfaces' second derivatives (each residual's own sag, the larger of
//! the two, would pass it).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::boolean::contact_verify::tangent_locus_relation;
use topo::contact::{ContactRefusal, ContactVerdict};

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

fn read(delta: f64) -> Result<ContactVerdict, ContactRefusal> {
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
        true,
        band(),
    )
}

#[test]
fn the_touch_between_samples_charges_both_surfaces_sags() {
    let eps = band().zero();
    let (ball, bore, halfway) = off_axis_ball(0.95 * eps);
    let bounds =
        geom_brep::tangent_span_bounds(&ball, &bore, &halfway, 0.0, core::f64::consts::TAU)
            .unwrap();
    // The fixture's premise: the sum sag puts the span line in band at
    // 0.95ε, the max sag would not.
    assert!(0.95 * eps + bounds.gap_sag > eps, "{bounds:?}");
    assert!(0.95 * eps + bounds.residual_sag <= eps, "{bounds:?}");
    // Well inside the band: verified.
    assert!(
        matches!(read(0.5 * eps), Ok(_)),
        "0.5ε: {:?}",
        read(0.5 * eps)
    );
    // Every sample within ε, the span past it: the span line escalates.
    match read(0.95 * eps) {
        Err(ContactRefusal::Escalated { diag }) => {
            assert_eq!(diag.predicate, Some("contact_tangent_touch"), "{diag:?}");
        }
        other => panic!("0.95ε: the span line refuses on the sag: {other:?}"),
    }
}

//! **The shell serves finished bodies only.** `shell`, `shell_open` and
//! `ShellDoor::open` take an `AtRestBody`, so an operand tier 3 refuses
//! cannot be handed to them at all: the refusal is the gate that builds
//! the operand (`AtRestBody::validate`), and the door has no runtime
//! arm for it. What pins the door is its type, below: an `AtRestBody`
//! derefs to a `Body`, so a door re-spelled over `&Body` would still
//! take every caller's argument, and only these coercions stop
//! compiling.
//!
//! The inside-out wedge is a prism over a clockwise profile, closed and
//! tier-2 clean with its faces pointing inward (volume −0.2349). Taken
//! as a `&Body`, `shell(&wedge, 0.02)` answered `Ok` with a valid body
//! of volume 0.05961: a wall built for the complement, where the
//! counterclockwise wedge's is 0.05097. The rows below show the gate
//! refusing that wedge and the counterclockwise one shelling.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Tol, Vec2};
use topo::{
    AtRestBody, Body, FaceKey, ShellDoor, ShellError, Shelled, ValidationError, mass_properties,
    shell,
};

/// What every shell door answers at `f64`.
type Answer = Result<Shelled<f64>, ShellError<f64>>;
/// The sealed door's operand type at a certifying scalar.
const _: fn(&AtRestBody<f64>, f64, Tol) -> Answer = shell::<f64>;
/// The opened door's.
const _: fn(&AtRestBody<f64>, f64, &[FaceKey], Tol) -> Answer = topo::shell_open::<f64>;
/// The seat's door, which hands its operand to `shell_open`.
type Door = ShellDoor<f64>;
const _: fn(Door, &AtRestBody<f64>, f64, &[FaceKey], Tol) -> Answer = Door::open;

/// The triangle (0,0), 80°, 190° on the unit circle, counterclockwise
/// when `ccw`.
fn wedge_profile(ccw: bool) -> [(f64, f64); 3] {
    let at = |deg: f64| (deg.to_radians().cos(), deg.to_radians().sin());
    if ccw {
        [(0.0, 0.0), at(80.0), at(190.0)]
    } else {
        [(0.0, 0.0), at(190.0), at(80.0)]
    }
}

/// The wedge prism over z ∈ (0.5, 1).
fn wedge(ccw: bool) -> Body<f64> {
    common::prism_z::<f64>(&wedge_profile(ccw), 0.5, 1.0, Tol::witness()).body
}

/// **The clockwise wedge is refused at the gate the shell's operand
/// passes**, on check 7 alone and naming its one solid.
#[test]
fn the_clockwise_wedge_is_refused_before_the_shell() {
    let body = wedge(false);
    let (solid, _) = body.solids().next().expect("the wedge is one solid");
    let errors = AtRestBody::validate(body, Tol::witness())
        .expect_err("an inside-out wedge is not a finished body");
    assert_eq!(
        errors,
        vec![ValidationError::NegativeVolume { solid }],
        "the gate refuses the wedge's one solid inside-out"
    );
}

/// **The counterclockwise wedge shells to the wall its own material
/// holds**: the prism less its inward offset, a triangle inset by `t`
/// (similar about the incentre, scale `(r − t)/r`) over a height short
/// by `2t`.
#[test]
fn the_counterclockwise_wedge_shells_to_its_closed_form_wall() {
    let tol = Tol::witness();
    let t = 0.02;
    let [o, p, q] = wedge_profile(true).map(|(x, y)| Vec2::new(x, y));
    let area = 0.5 * ((p - o).x * (q - o).y - (p - o).y * (q - o).x);
    let semi = 0.5 * ((p - o).norm() + (q - p).norm() + (o - q).norm());
    let inradius = area / semi;
    let scale = (inradius - t) / inradius;
    let height = 0.5;
    let wall = area * height - area * scale * scale * (height - 2.0 * t);

    let operand = AtRestBody::validate(wedge(true), tol).expect("the wedge is a finished body");
    let shelled = shell(&operand, t, tol).expect("the counterclockwise wedge shells");
    let volume = mass_properties(&shelled.body, tol).unwrap().volume;
    assert!(
        ((volume - wall) / wall).abs() < 1e-12,
        "the wall is {volume} against the closed form {wall}"
    );
}

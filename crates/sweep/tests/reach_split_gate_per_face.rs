//! The split's operand gate is per face: a body carrying a face of a
//! kind the split has no arm for splits wherever the plane cannot reach
//! that face, and refuses naming it wherever the plane may.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::revolve_common::{axis_y, validated};
use geom_core::{Band, Point2, Point3, Tol, UnitVec3, Vec3};
use profile::{ArcSweep, bulge_from_center, test_support::bulge_loop};
use sweep::{Revolution, revolve};
use topo::splitting::{SplitPart, SplitPlane, SplitResult, split};
use topo::{Body, DATUM_UNIT_NORM, validate, validate_closed, validate_geometric};

fn revolved(chain: Vec<(Point2<f64>, f64)>) -> Body<f64> {
    revolve(
        &validated(vec![bulge_loop(chain)]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// A cut normal minted the way a caller holding a direction mints one.
fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
    let band = Band::linear(Tol::witness()).expect("the witness tolerance forms a band");
    UnitVec3::new(v, DATUM_UNIT_NORM, band).expect("a cut normal has a length")
}

/// The plane through `(0, qy, 0)` with normal `(sin φ, cos φ, 0)`.
fn plane(phi: f64, qy: f64) -> SplitPlane<f64> {
    SplitPlane {
        origin: Point3::new(0.0, qy, 0.0),
        normal: unit(Vec3::new(phi.sin(), phi.cos(), 0.0)),
    }
}

/// The unit cylinder `y ∈ [0, 1]` under a spherical cap: the arc
/// `(1, 1) → (0, 1.5)` about `(0, 0.25)` (radius 5/4), revolved about `y`.
fn capped_cylinder() -> Body<f64> {
    let (a, b) = (Point2::new(1.0, 1.0), Point2::new(0.0, 1.5));
    let bulge = bulge_from_center(a, b, Point2::new(0.0, 0.25), ArcSweep::Ccw);
    revolved(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (a, bulge),
        (b, 0.0),
    ])
}

/// The cap's volume, `π·h²·(3R − h)/3` at `R = 5/4`, `h = 1/2`.
const CAP_VOLUME: f64 = PI * 0.25 * (3.75 - 0.5) / 3.0;

fn halves(result: &SplitResult<f64>, what: &str) -> (Body<f64>, Body<f64>) {
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("{what}: both sides carry material");
    };
    for part in [above, below] {
        assert_eq!(validate(part), Ok(()), "{what}: tier 1");
        assert_eq!(validate_closed(part), Ok(()), "{what}: tier 2");
        if let Err(errs) = validate_geometric(part, Tol::witness()) {
            panic!("{what}: tier 3: {errs:?}");
        }
    }
    (above.clone(), below.clone())
}

#[test]
fn probe_capped_cylinder() {
    let body = capped_cylinder();
    let r = split(&body, &plane(0.3, 0.5), Tol::witness());
    match r {
        Ok(res) => {
            let (a, b) = halves(&res, "probe");
            let p = |b: &Body<f64>| topo::props::mass_properties(b, Tol::witness()).unwrap();
            eprintln!("above {:?}\nbelow {:?}", p(&a), p(&b));
            eprintln!("cap {CAP_VOLUME}");
        }
        Err(e) => panic!("{e:?} / {e}"),
    }
}

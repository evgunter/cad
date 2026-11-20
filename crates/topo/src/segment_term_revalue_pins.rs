//! **Every door that reads a conic arc's segment term re-values to its
//! own bits** ([`crate::revalue_pin`]): `closed_segment_term` on a
//! circle and on an ellipse, each way round, and `arc_segment_term`.
//! The term is `Δ − sin Δ` ([`geom_core::Real::less_sin`]), which a
//! recording scalar keeps as one node and re-values through the door
//! that valued it.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use crate::revalue_pin::{S, revalued};
use geom_core::{Point3, Real, Vec3};

fn ellipse(a: S, b: S) -> geom::Curve3<S> {
    let f = geom::Frame::new(
        Point3::new(S::from_f64(3.0), S::from_f64(-1.0e4), S::zero()),
        Vec3::new(S::zero(), S::zero(), S::one()),
        Vec3::new(S::one(), S::zero(), S::zero()),
    );
    geom::Curve3::Ellipse(geom::Ellipse::new(f, a, b))
}
fn circle(r: S) -> geom::Curve3<S> {
    let f = geom::Frame::new(
        Point3::new(S::zero(), S::from_f64(-1.0e3), S::zero()),
        Vec3::new(S::zero(), S::zero(), S::one()),
        Vec3::new(S::zero(), S::one(), S::zero()),
    );
    geom::Curve3::Circle(geom::Circle::new(f, r))
}

#[test]
fn every_segment_door_revalues_to_its_own_bits() {
    for fwd in [true, false] {
        let found = revalued([0.5, 0.5 + 1.0e-3], |_, p| {
            let t1 = p * S::from_f64(1.0e-4);
            let mut out = Vec::new();
            for c in [
                circle(S::from_f64(1.0e3)),
                ellipse(S::from_f64(2.0e4), S::from_f64(1.0e4)),
            ] {
                let term =
                    geom_brep::props::closed_segment_term((&c, (S::zero(), t1)), fwd).unwrap();
                out.extend([term.bulge.x, term.bulge.y, term.bulge.z, term.length]);
            }
            let v = geom_brep::props::arc_segment_term(
                Vec3::new(S::zero(), S::zero(), S::one()),
                p,
                t1,
            );
            out.push(v.z);
            out
        });
        assert_eq!(found.laundered, 0, "{found:?}");
        assert!(found.kept > 0, "{found:?}");
    }
}

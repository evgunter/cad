//! **sphere × torus on the ℝ³ implicit pair** (`ssi::sphere_torus_ssi`):
//! the torus read through its quartic, at every limb and in the
//! accounting.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom::Surface;
use geom_brep::ssi::{self, SsiDomain, SsiError, TangentPair};
use geom_core::{Point3, Vec3};

use crate::shared::surf;
use crate::shared::tol::band;

fn sphere(c: [f64; 3], r: f64) -> Surface<f64> {
    surf::sphere_at(c, r)
}

fn torus(c: [f64; 3], big: f64, small: f64) -> Surface<f64> {
    surf::torus_at(c, big, small)
}

/// The teapot's handle torus: axis `+z` through `(5/64, 4/64, 0)`,
/// `R = 6/256`, `r = 1/128`.
fn handle() -> Surface<f64> {
    torus([5.0 / 64.0, 4.0 / 64.0, 0.0], 6.0 / 256.0, 1.0 / 128.0)
}

fn about_handle() -> SsiDomain {
    SsiDomain {
        center: Point3::new(5.0 / 64.0, 4.0 / 64.0, 0.0),
        half_extent: 6.0 / 256.0 + 1.0 / 128.0 + 1.0 / 256.0,
        extent: 2.0 * (6.0 / 256.0 + 1.0 / 128.0),
        floor_scale: 1.0,
    }
}

/// The worst of each operand's meters residual over `samples` points of
/// each branch's carrier.
fn worst_residuals(out: &geom_brep::SsiOutcome, a: &Surface<f64>, b: &Surface<f64>) -> [f64; 2] {
    let mut worst = [0.0f64; 2];
    for branch in &out.branches {
        let (t0, t1) = branch.params;
        for k in 0..=256 {
            let p = branch.carrier.eval(t0 + (t1 - t0) * f64::from(k) / 256.0);
            for (w, s) in worst.iter_mut().zip([a, b]) {
                *w = w.max(geom_brep::implicit::implicit_residual(s, p).abs());
            }
        }
    }
    worst
}

/// **The teapot's handle meets both of the pot's spheres in two certified
/// loops.** The handle torus (axis tangent to the outer sphere) against
/// the outer belly sphere (radius `5/64`) and the shelled inner one
/// (`9/128`): two branches each, all three limbs and the accounting to
/// the band's floor. Sampled along every carrier, both residuals are
/// within the band the certificate claims (a limb 2 that bounded less
/// than the residual reads red), and the accounting stays within a
/// budget the root-free enclosure meets with room (the direct quartic's
/// band, about `2R/r` fatter, examines many times more).
#[test]
fn the_teapots_handle_meets_each_belly_sphere_in_two_certified_loops() {
    for (name, rs) in [("outer", 5.0 / 64.0), ("inner", 9.0 / 128.0)] {
        let s = sphere([0.0, 4.0 / 64.0, 0.0], rs);
        let out = ssi::sphere_torus_ssi(&s, &handle(), about_handle(), band())
            .unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!(out.branches.len(), 2, "{name}: two loops");
        let [ws, wt] = worst_residuals(&out, &s, &handle());
        assert!(
            ws <= band().zero() && wt <= band().zero(),
            "{name}: sampled residuals {ws:e} (sphere), {wt:e} (torus) past the band"
        );
        let examined = out.exhaustiveness.examined;
        assert!(examined < 16_000, "{name}: {examined} cells examined");
        let reversed = ssi::sphere_torus_ssi(&handle(), &s, about_handle(), band())
            .unwrap_or_else(|e| panic!("{name}, torus first: {e}"));
        assert_eq!(reversed.branches.len(), 2, "{name}, torus first");
    }
}

/// **A thin torus certifies too.** `R/r = 10`, a sphere of radius
/// `0.15` crossing the tube: the band the enclosure has to resolve is
/// the thinnest relative to the torus's size.
#[test]
fn a_thin_torus_and_a_sphere_crossing_its_tube_certify() {
    let t = torus([0.0; 3], 1.0, 0.1);
    let s = sphere([1.0, 0.0, 0.1], 0.15);
    let domain = SsiDomain {
        center: Point3::origin(),
        half_extent: 1.2,
        extent: 2.2,
        floor_scale: 1.0,
    };
    let out = ssi::sphere_torus_ssi(&s, &t, domain, band()).unwrap_or_else(|e| panic!("{e}"));
    assert!(!out.branches.is_empty(), "the sphere crosses the tube");
    let [ws, wt] = worst_residuals(&out, &s, &t);
    assert!(ws <= band().zero() && wt <= band().zero(), "{ws:e}, {wt:e}");
}

/// **A tangent pair refuses before any rung**, on each of the gap's
/// poses: the sphere about the torus's centre through its inner equator
/// (tangent around that circle), and a sphere touching the outer
/// equator from outside at one point.
#[test]
fn a_sphere_tangent_to_a_torus_refuses_as_the_pairs_tangency() {
    let t = torus([0.0; 3], 1.0, 0.25);
    let domain = SsiDomain {
        center: Point3::origin(),
        half_extent: 2.0,
        extent: 2.5,
        floor_scale: 1.0,
    };
    for (name, s) in [
        ("inner equator", sphere([0.0; 3], 0.75)),
        ("outer equator", sphere([1.55, 0.0, 0.0], 0.3)),
    ] {
        let got = ssi::sphere_torus_ssi(&s, &t, domain, band());
        assert!(
            matches!(
                got,
                Err(SsiError::PairTangent {
                    pair: TangentPair::SphereTorus,
                    ..
                })
            ),
            "{name}: {got:?}"
        );
        let shown = got.unwrap_err().to_string();
        assert!(
            shown.contains("the sphere and the torus"),
            "{name}: {shown}"
        );
    }
}

/// **Each door takes its own pair only.**
#[test]
fn the_sphere_torus_door_refuses_any_other_pair() {
    let t = handle();
    let s = sphere([0.0, 4.0 / 64.0, 0.0], 5.0 / 64.0);
    let c = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 0.05,
        u_ref: Vec3::unit_x(),
    };
    for (a, b) in [(&s, &c), (&t, &t), (&s, &s)] {
        let got = ssi::sphere_torus_ssi(a, b, about_handle(), band());
        assert!(matches!(got, Err(SsiError::WrongLane { .. })), "{got:?}");
    }
    let got = ssi::cylinder_sphere_ssi(&s, &t, about_handle(), band());
    assert!(matches!(got, Err(SsiError::WrongLane { .. })), "{got:?}");
}

/// **A torus outside its ring convention refuses at the door**, as
/// `route_pose` refuses its pose: a spindle (`r > R`) and a horn
/// (`r = R`) torus, whose `|q|² + R² − r²` reaches zero, which the
/// torus's enclosures divide by.
#[test]
fn a_spindle_or_horn_torus_refuses_at_the_door() {
    let s = sphere([0.5, 0.0, 0.0], 0.3);
    let domain = SsiDomain {
        center: Point3::origin(),
        half_extent: 2.0,
        extent: 2.0,
        floor_scale: 1.0,
    };
    for (name, t) in [
        ("spindle", torus([0.0; 3], 0.5, 0.8)),
        ("horn", torus([0.0; 3], 0.6, 0.6)),
    ] {
        let got = ssi::sphere_torus_ssi(&s, &t, domain, band());
        assert!(
            matches!(got, Err(SsiError::DegenerateTorus(_))),
            "{name}: {got:?}"
        );
    }
}

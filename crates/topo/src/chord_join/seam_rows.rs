//! **A chord's ellipse is seamed at the principal vertex nearest its
//! arc** ([`SectionConic::seamed_near`], [`chord_arc`]): an ellipse of
//! semi-axes `1e4` along `x` and `1` along `y` about `z`, whose arcs
//! near the minor vertex `(0, 1, 0)` sit at `θ ≈ π/2` in its own frame,
//! where the parameter resolves only to `ulp(π/2)·1e4 ≈ 2e-12`.

use super::*;

const A: f64 = 1e4;

fn conic() -> SectionConic<f64> {
    section_conic(geom::Curve3::Ellipse {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        semi_u: A,
        semi_v: 1.0,
        u_ref: Vec3::unit_x(),
    })
    .unwrap()
}

/// The ellipse's point at eccentric anomaly `θ` in its stored frame,
/// formed from exact sines and cosines of the offset from the minor
/// vertex, so it carries no `ulp(π/2)` of its own.
fn near_minor(d: f64) -> Point3<f64> {
    Point3::new(-A * d.sin(), d.cos(), 0.0)
}

/// Each middle lands on the vertex the rule names, the dominant one of
/// its cosine and sine, the quarter-turned frame's semi-axes swapped.
#[test]
fn the_seam_goes_to_the_nearest_principal_vertex() {
    let pi = core::f64::consts::PI;
    for (theta, seam, sa) in [
        (0.1, Vec3::unit_x(), A),
        (0.7, Vec3::unit_x(), A),
        (2.4, -Vec3::unit_x(), A),
        (0.9, Vec3::unit_y(), 1.0),
        (pi / 2.0 + 0.3, Vec3::unit_y(), 1.0),
        (-pi / 2.0, -Vec3::unit_y(), 1.0),
        (pi, -Vec3::unit_x(), A),
    ] {
        let s = conic().seamed_near(theta);
        assert!(
            (s.seam - seam).norm() == 0.0 && s.sa == sa,
            "θ = {theta}: {:?} {}",
            s.seam,
            s.sa
        );
        let geom::Curve3::Ellipse {
            semi_u,
            semi_v,
            u_ref,
            ..
        } = s.carrier
        else {
            panic!("an ellipse stays one")
        };
        assert!(
            semi_u == s.sa && semi_v == s.sb && (u_ref - s.seam).norm() == 0.0,
            "θ = {theta}: the carrier is the frame"
        );
        for t in [0.0, 1.0, 2.5, -2.0] {
            let p = s.carrier.eval(t);
            let back = conic().carrier.eval(conic().param(p));
            assert!(
                p.distance(back) < 1e-10,
                "θ = {theta}: the same locus at {t}"
            );
        }
    }
    let circle = section_conic(geom::Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    })
    .unwrap()
    .seamed_near(core::f64::consts::FRAC_PI_2);
    assert_eq!(
        (circle.seam - Vec3::unit_x()).norm(),
        0.0,
        "a circle keeps its seam"
    );
}

/// **An arc near the minor vertex is minted about it**, either way
/// round: its window is a short signed span about `t = 0` (never a turn
/// less the other way's span), and each end lands on its vertex to
/// within the arc's own rounding, far below the `2e-12` the stored frame
/// resolves there.
#[test]
fn an_arc_near_a_minor_vertex_resolves_at_its_own_scale() {
    let (p1, p2) = (near_minor(-1e-3), near_minor(2e-3));
    for ccw in [true, false] {
        let (a, b) = if ccw { (p1, p2) } else { (p2, p1) };
        let (carrier, t0, t1) = chord_arc(
            SectionCurve::Conic(conic()),
            FaceKey::default(),
            a,
            Some(b),
            ccw,
        )
        .unwrap();
        assert!(
            t0.abs() < 3e-3 && t1.abs() < 3e-3 && (t1 - t0 - 3e-3).abs() < 1e-12,
            "ccw {ccw}: the window [{t0}, {t1}]"
        );
        for (t, p) in [(t0, a), (t1, b)] {
            let miss = carrier.eval(t).distance(p);
            assert!(
                miss < 1e-14,
                "ccw {ccw}: an end misses its vertex by {miss:e}"
            );
        }
    }
}

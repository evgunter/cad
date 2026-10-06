//! Reviewer probe (reach-dual4135-r1): a pose where the far-nappe
//! tell-off DECIDES. A quarter-revolved widening frustum (apex `y = −1`,
//! face on the opening nappe over `y ∈ [0, 1]`) is a PARTIAL cone face,
//! whose trim the chart door does not express (`PartialConeFace` →
//! `Trim(None)`, the frontier). A thin brick from beside the frustum
//! (inside its box, outside the cone) runs down past the apex into the
//! MIRROR nappe's interior: every crossing of the double cone along it
//! is on the mirror nappe, none on the face. With the tell-off, those
//! roots are crossed elsewhere and the sweep answers no wall crossing;
//! without it the trim is asked and the frontier refuses. Mounted
//! temporarily into `crates/sweep/tests/all.rs`.
#![allow(clippy::unwrap_used, clippy::panic)]
use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::test_support::finished;
use sweep::{Revolution, revolve};

#[test]
fn r1_far_nappe_tell_off_decides() {
    let tol = Tol::witness();
    let lp = ProfileLoop::polygon([(0.0, 0.0), (0.5, 0.0), (1.0, 1.0), (0.0, 1.0)].iter().map(|&(x, y)| Point2::new(x, y)));
    let half = revolve(&validated(vec![lp]), axis_y(), Revolution::Partial(core::f64::consts::FRAC_PI_2), tol).unwrap().body;
    let cone = finished("half frustum", half, tol);
    // The brick: a thin box along the segment A = (0.9, 0.2, ·) →
    // B = (0, −4, ·), at z ∈ [0.29, 0.31] or its mirror (whichever side
    // the half-turn covers, both are tried).
    for z in [0.3, -0.3] {
        let (a, b) = (Point3::new(0.9, 0.2, z), Point3::new(0.0, -4.0, z));
        let len = (b - a).norm();
        let brick = sweep::test_support::brick((-0.01, 0.01), (0.0, len), (-0.01, 0.01), tol);
        let dir = (b - a) / len;
        let ang = Vec3::new(0.0, 1.0, 0.0).dot(dir).acos();
        let axis = Vec3::new(0.0, 1.0, 0.0).cross(dir).normalize();
        let pose = Affine3::translation(Vec3::new(a.x, a.y, a.z)) * Affine3::rotation_about_axis(Point3::origin(), axis, ang);
        let other = finished("brick", topo::transform_rigid(&brick, &pose, tol).unwrap(), tol);
        // Oracle: along each long edge, the double cone's crossings and
        // which nappe each is on.
        let elev = |q: Point3<f64>| q.x.hypot(q.z) * (0.5f64).atan().cos() - (q.y + 1.0).abs() * (0.5f64).atan().sin();
        let mut nappes = Vec::new();
        for (_, e) in other.edges() {
            let c = other.get_curve_geom(e.curve).and_then(topo::CurveGeom::certified).unwrap();
            let (t0, t1) = c.params();
            let n = 20_000;
            for k in 0..n {
                let ta = t0 + (t1 - t0) * f64::from(k) / f64::from(n);
                let tb = t0 + (t1 - t0) * f64::from(k + 1) / f64::from(n);
                let (pa, pb) = (c.carrier().eval(ta), c.carrier().eval(tb));
                if (elev(pa) < 0.0) != (elev(pb) < 0.0) {
                    nappes.push(if pa.y > -1.0 { "opening" } else { "mirror" });
                }
            }
        }
        for (order, swapped) in [("cone first", false), ("brick first", true)] {
            let (x, y) = if swapped { (&*other, &*cone) } else { (&*cone, &*other) };
            let got = topo::sweep_split_admitting_cones(x, y, tol);
            let summary = match &got {
                Ok((sa, sb, _, _)) => {
                    let split = if swapped { sa } else { sb };
                    format!("Ok: brick vertices {} -> {}", other.vertex_points().count(), split.vertex_points().count())
                }
                Err(e) => format!("Err {:?}", e).chars().take(170).collect(),
            };
            eprintln!("R1NAPPE z={z} {order}: oracle crossings by nappe {nappes:?}; sweep {summary}");
        }
    }
}

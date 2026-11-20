//! A cylinder of radius 3 whose wall passes through the reflex corner
//! `v = (2, 1, 0)` of the notched prism, its axis across the corner's
//! outward direction, united with the prism. The union's cylinder face
//! keeps a quarter turn, and its boundary passes `v` twice, once on each
//! side of the notch: the trimmed lane reads `v` off two pcurves, which
//! agree only to rounding. At δ = 1 that face has one interior lattice
//! column. Pinned: the face meshes, and the mesh is closed and manifold.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, RigidMotion, UnitVec3, Vec3};
use topo::test_support as fixtures;
use topo::{AtRestBody, BooleanError};

use crate::join_pierce_runs_sweep::{finished, tol};

type V3 = [f64; 3];

/// `ε`, the witness band's zero.
fn eps() -> f64 {
    Band::linear(tol()).expect("the witness band").zero()
}

fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn unit(a: V3) -> V3 {
    let k = 1.0 / dot(a, a).sqrt();
    a.map(|c| c * k)
}

/// The cylinder of radius 3 and length 8 whose wall holds `v`, its
/// centre `3 m` from `v` and its axis `⊥ m`, turned `psi` about `m`
/// from the first of `m`'s basis vectors, its arcs' junctions `zoff`
/// round the wall from `v`.
fn cylinder_through(v: V3, m: V3, psi: f64, zoff: f64) -> AtRestBody<f64> {
    let (r, half) = (3.0, 4.0);
    let seed = if m[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let u0 = unit(cross(seed, m));
    let w0 = cross(m, u0);
    let a = [0, 1, 2].map(|i| psi.cos() * u0[i] + psi.sin() * w0[i]);
    let c = [0, 1, 2].map(|i| v[i] + r * m[i]);
    let ax = unit(cross([0.0, 0.0, 1.0], a));
    let band = Band::linear(tol()).unwrap();
    let axis = UnitVec3::new(Vec3::new(ax[0], ax[1], ax[2]), "the tilt axis", band).unwrap();
    let tilt = RigidMotion::rotation(axis, a[2].clamp(-1.0, 1.0).acos()).unwrap();
    let back = tilt
        .inverse()
        .unwrap()
        .transform_vec(Vec3::new(-m[0], -m[1], -m[2]));
    let spin = RigidMotion::rotation(UnitVec3::exact_z(), back.y.atan2(back.x) - zoff).unwrap();
    let motion = RigidMotion::translation(Vec3::new(c[0], c[1], c[2]))
        .unwrap()
        .compose(tilt)
        .unwrap()
        .compose(spin)
        .unwrap();
    let raw = sweep::test_support::cylinder_of_arcs_at(
        4,
        r,
        Point2::new(0.0, 0.0),
        -half,
        2.0 * half,
        tol(),
    );
    finished(
        "the cylinder",
        topo::transform_rigid(&raw, &motion).unwrap(),
    )
}

#[test]
fn a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta() {
    let notch = [(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0), (0.0, 2.0)];
    let prism = finished("the prism", fixtures::prism::<f64>(&notch, 1.0, tol()).body);
    // The 16th of 60 Fibonacci directions, the battery's `fib15`.
    let (n, i) = (60.0, 15.0);
    let z: f64 = 1.0 - 2.0 * (i + 0.5) / n;
    let t = std::f64::consts::PI * (3.0 - 5f64.sqrt()) * i;
    let m = unit([
        (1.0 - z * z).sqrt() * t.cos(),
        (1.0 - z * z).sqrt() * t.sin(),
        z,
    ]);
    let cylinder = cylinder_through([2.0, 1.0, 0.0], m, 0.9, 0.9);
    // The pierce's curvature side reads a margin of 1.47e-6: inside the
    // band at the 1e-6 row, where the union escalates, typed, and there
    // is no body to mesh; clear of it at the finer rows.
    let union = match topo::union(&prism, &cylinder, tol()) {
        Err(BooleanError::Escalated {
            decision: topo::BooleanDecision::PierceCurvature,
            ..
        }) if eps() >= 1e-6 => {
            println!(
                "eps {}: the union escalates its pierce curvature, no mesh",
                eps()
            );
            return;
        }
        r => r.unwrap_or_else(|e| panic!("the union builds at eps {}: {e:?}", eps())),
    };
    let body = &union.body().expect("the union is a body").body;
    let mesh = mesh::tessellate(body, 1.0, tol()).expect("the union meshes at delta 1");
    mesh::validate::check_mesh(&mesh).expect("the mesh is closed and manifold");
}

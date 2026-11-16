use super::{DescribedChart, Orbit, joint_has_room};
use geom::Surface;
use geom_core::{Band, Point3, Vec3};

const EPS: f64 = 1e-9;

/// The tightest band the kernel admits in practice: `K = 1.5`.
fn tight() -> Band {
    Band::new(EPS, 1.5 * EPS).unwrap()
}

/// The fence at `vertex`, on the orbit a harmonic image on
/// `surface` walks (a sphere's, half periods with its twin).
fn room(surface: &Surface<f64>, vertex: Point3<f64>) -> bool {
    // A rim winding once: the second channel is an angle on a
    // sphere, a length on a cone.
    let v = match surface {
        Surface::Sphere { .. } | Surface::Torus { .. } => geom_brep::Channel::fixed_angle(0.0),
        _ => geom_brep::Channel::constant(0.0),
    };
    let image = geom_brep::Pcurve::Harmonic {
        u: geom_brep::Channel::Angle {
            phase: 0.0,
            winding: 1,
        },
        v,
    };
    joint_has_room(
        DescribedChart::of(surface).unwrap(),
        vertex,
        Some(geom_brep::Period::new(core::f64::consts::TAU)),
        Orbit::of(surface, &image),
        tight(),
    )
}

/// **The room fence holds at a small `K`.** A wrong orbit pick needs
/// the joint's two chart ends half a step apart in azimuth, and
/// those ends sit within `2ε` of the vertex, so the fence decides
/// the chord they would span (`2·(d − 2ε)·sin(step/4)`) against the
/// joint bound `4ε`. At `K = 1.5` a cone vertex `3ε` from the axis
/// has an arc room `(π/2)·d` past four times the escalation, which
/// the arc reading admitted, while a circle through it is under
/// `4ε` across from the ends' least lever, so the integer pins
/// nothing: it refuses. At `6ε` it has room.
#[test]
fn at_a_small_k_a_cone_joint_near_the_axis_has_no_room() {
    let alpha: f64 = 0.3;
    let cone = Surface::Cone {
        apex: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        half_angle: alpha,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let at = |d: f64| Point3::new(d, 0.0, d / alpha.tan());
    assert!(!room(&cone, at(3.0 * EPS)), "3ε from the axis: no room");
    assert!(room(&cone, at(6.0 * EPS)), "6ε from the axis: room");
}

/// The sphere's step is half a period, so its chord is the
/// `sin(π/4)` one: at `K = 1.5` a vertex `4.5ε` from the axis, which
/// the arc reading admitted, refuses, and one `8ε` off has room.
#[test]
fn at_a_small_k_a_sphere_joint_near_a_pole_has_no_room() {
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let at = |d: f64| Point3::new(d, 0.0, (1.0 - d * d).sqrt());
    assert!(!room(&sphere, at(4.5 * EPS)), "4.5ε from the axis: no room");
    assert!(room(&sphere, at(8.0 * EPS)), "8ε from the axis: room");
}

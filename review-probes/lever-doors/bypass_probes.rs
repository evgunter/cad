//! Review probes for the lever-doors gate (scripts/gates/lever-doors.sh):
//! each `bypass_*` hands a lever a length the caller chose, compiles from
//! outside the homes, and is not counted by the gate when written in
//! production code. The rows assert the arbitrary length arrives.
use geom::cover::Cover;
use geom::{Circle, Surface};
use geom_brep::{ExtentBall, Reach};
use geom_core::{Lever, Point3, Vec3};

type P = Point3<f64>;
const LEN: f64 = 12345.0;

fn o() -> P {
    Point3::new(0.0, 0.0, 0.0)
}
fn z() -> Vec3<f64> {
    Vec3::new(0.0, 0.0, 1.0)
}
fn x() -> Vec3<f64> {
    Vec3::new(1.0, 0.0, 0.0)
}
fn unit_sphere() -> Surface<f64> {
    Surface::Sphere { center: o(), radius: 1.0, axis: z(), u_ref: x() }
}

// J1: a qualified path, not `ExtentBall::point`.
fn bypass_qualified_path(p: P, q: P) -> Lever<f64> {
    <ExtentBall<f64>>::point(q).lever_from(p)
}
// J3: a real cover, and a "normal" that is a scale.
fn bypass_scaled_normal(c: &Cover<f64>, p: P, len: f64) -> Lever<f64> {
    ExtentBall::of_cover(c).lever_from_point_and_plane(p, (Point3::new(-1.0, 0.0, 0.0), x() * len))
}
// J5: a macro's expansion.
macro_rules! door {
    ($t:ident, $f:ident, $x:expr) => {
        geom_brep::$t::$f($x)
    };
}
fn bypass_macro(p: P, q: P) -> Lever<f64> {
    door!(ExtentBall, point, q).lever_from(p)
}
// J6: `Self::` in a trait impl for Reach written in another crate.
trait Mk {
    fn mk(p: P, a: f64, l: Lever<f64>) -> Self;
}
impl Mk for Reach<f64> {
    fn mk(p: P, a: f64, l: Lever<f64>) -> Self {
        Self::of_face(p, (a, a), l)
    }
}
// J7: a real surface, a point the caller chose.
fn bypass_parallel_point(s: &Surface<f64>, len: f64) -> Lever<f64> {
    geom_brep::parallel_curvature_arm(s, Point3::new(len, 0.0, 0.0))
}
// J8/J9: geometry built by hand (the hole the PR concedes).
fn bypass_hand_circle(len: f64) -> Lever<f64> {
    geom_brep::circle_curvature_arm(&Circle { frame: geom::Frame::new(o(), z(), x()), radius: len })
}
fn bypass_hand_sphere(len: f64) -> Lever<f64> {
    geom_brep::uniform_curvature_arm(&Surface::Sphere { center: o(), radius: len, axis: z(), u_ref: x() })
}
fn bypass_hand_cover(p: P, q: P) -> Lever<f64> {
    ExtentBall::of_cover(&Cover::segment(p, q).unwrap()).lever_from(p)
}
// `Reach::of_frame` takes any surface: a plane reads f64::MAX.
fn bypass_of_frame_plane(p: P) -> Lever<f64> {
    Reach::of_frame(p, &Surface::Plane { origin: o(), normal: z(), u_ref: x() }).lever_from(p)
}

#[test]
fn each_bypass_delivers_the_callers_length() {
    let far = Point3::new(LEN, 0.0, 0.0);
    assert_eq!(bypass_qualified_path(o(), far).length(), LEN);
    let c = Cover::segment(o(), Point3::new(0.0, 1e-3, 0.0)).unwrap();
    assert!(bypass_scaled_normal(&c, o(), LEN).length() >= LEN);
    assert_eq!(bypass_macro(o(), far).length(), LEN);
    let _ = <Reach<f64> as Mk>::mk(o(), LEN, Lever::session_box());
    assert_eq!(bypass_parallel_point(&unit_sphere(), LEN).length(), LEN);
    assert_eq!(bypass_hand_circle(LEN).length(), LEN);
    assert_eq!(bypass_hand_sphere(LEN).length(), LEN);
    assert!((bypass_hand_cover(o(), far).length() - LEN).abs() < 1e-6);
    println!("of_frame(plane) lever = {:e}", bypass_of_frame_plane(o()).length());
}

#[test]
fn the_curvature_readers_side_by_side() {
    let cyl = Surface::Cylinder { origin: o(), axis: z(), radius: 2.0, u_ref: x() };
    let cone = Surface::Cone { apex: o(), axis: z(), half_angle: 0.5, u_ref: x() };
    let fat = Surface::Torus { center: o(), axis: z(), major_radius: 3.0, minor_radius: 2.0, u_ref: x() };
    let pt = |s: &Surface<f64>, u: f64, v: f64| s.eval(u, v);
    for (name, s, p) in [
        ("sphere", unit_sphere(), pt(&unit_sphere(), 0.3, 0.4)),
        ("cylinder", cyl.clone(), pt(&cyl, 0.3, 0.4)),
        ("cone", cone.clone(), pt(&cone, 0.3, 1.5)),
        ("torus inner", fat.clone(), pt(&fat, 0.3, std::f64::consts::PI)),
        ("torus outer", fat.clone(), pt(&fat, 0.3, 0.0)),
    ] {
        println!(
            "{name}: at_point={:e} uniform={:e} parallel={:e} min_radius_of_curvature={:e} 1/max_normal_curvature={:e}",
            geom_brep::curvature_lever_arm(&s, p).length(),
            geom_brep::uniform_curvature_arm(&s).length(),
            geom_brep::parallel_curvature_arm(&s, p).length(),
            geom_brep::min_radius_of_curvature(&s, p),
            1.0 / geom_brep::implicit_max_normal_curvature(&s, p),
        );
    }
    let neg = Surface::Cylinder { origin: o(), axis: z(), radius: -2.0, u_ref: x() };
    println!("negative-radius cylinder arm = {:e}", geom_brep::uniform_curvature_arm(&neg).length());
}

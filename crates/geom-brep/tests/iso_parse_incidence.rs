//! **The cone and cylinder iso parses decide a circle's incidence at
//! its true distance from the surface, and whether it is the iso rim as
//! the inventory's question.** A circle within band of the surface but
//! not the iso rim — tilted to second order on a cylinder or a needle
//! cone, its centre off the axis of a near-flat cone — lies on the
//! surface and is refused `NotIsoRectangle`; the same circle moved off
//! the surface is refused `OffSurface`. Each in-band row is sized from
//! the run's ε: the true distance at most `ε/10`, the deviation from the
//! iso rim at least `2·K·ε`.
//!
//! **Lines likewise**: a line within band of the cylinder or cone but
//! not a ruling or generator — tipped in the tangent plane, which leaves
//! the surface to second order — is refused `NotIsoRectangle`, and the
//! same tip toward the normal `OffSurface`, at `f64` and at the interval
//! scalar.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use crate::shared::topo;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, PropsError, curved_face_loops, require_iso_rectangle};
use geom_core::{Decide, Interval, Point3, Real, Vec3};

const TAU: f64 = core::f64::consts::TAU;
const HALF_PI: f64 = core::f64::consts::FRAC_PI_2;

/// A full rim about `+z`-ish: centre `(off, 0, z)`, radius `r`, its
/// axis tipped `tilt` toward `+x`.
fn rim(z: f64, r: f64, off: f64, tilt: f64, fwd: bool, tag: u32) -> LoopEdge<f64> {
    let (s, c) = tilt.sin_cos();
    LoopEdge::hand_built(
        Curve3::Circle {
            center: Point3::new(off, 0.0, z),
            axis: Vec3::new(s, 0.0, c),
            radius: r,
            u_ref: Vec3::new(c, 0.0, -s),
        },
        0.0,
        TAU,
        fwd,
        tag,
        tag,
    )
}

fn cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    }
}

fn cone(alpha: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: alpha,
        u_ref: Vec3::unit_x(),
    }
}

/// A band of `cone(alpha)` between the rims of radius 1 and 2, the
/// upper one tipped `tilt` and moved `off` off the axis.
fn cone_band(alpha: f64, off: f64, tilt: f64) -> Vec<LoopEdge<f64>> {
    let h = |r: f64| r / alpha.tan();
    vec![
        rim(h(1.0), 1.0, 0.0, 0.0, true, 0),
        rim(h(2.0), 2.0, off, tilt, false, 1),
    ]
}

/// A unit cylinder's band `z ∈ [0, 1]`, the upper rim tipped `tilt`.
fn cylinder_band(tilt: f64) -> Vec<LoopEdge<f64>> {
    vec![
        rim(0.0, 1.0, 0.0, 0.0, true, 0),
        rim(1.0, 1.0, 0.0, tilt, false, 1),
    ]
}

fn not_iso(what: &'static str) -> Result<(), PropsError> {
    Err(PropsError::NotIsoRectangle { what })
}

fn off_surface(what: &'static str) -> Result<(), PropsError> {
    Err(PropsError::OffSurface { what })
}

/// The untouched bands parse: the rows below perturb a face the parse
/// accepts.
#[test]
fn the_unperturbed_bands_parse() {
    assert_eq!(
        require_iso_rectangle(&cylinder(), &cylinder_band(0.0), band()),
        Ok(())
    );
    for alpha in [1e-3, HALF_PI - 1e-3] {
        assert_eq!(
            require_iso_rectangle(&cone(alpha), &cone_band(alpha, 0.0, 0.0), band()),
            Ok(()),
            "alpha = {alpha:e}"
        );
    }
}

/// A unit cylinder's rim tipped `θ = √(ε/20)`: off the cylinder by
/// `1 − cos θ ≈ ε/40`, off the iso rim by `sin θ`, past `2·K·ε` at
/// every committed ε. Tipped `0.05` it is off the cylinder.
#[test]
fn a_cylinder_rim_tipped_to_second_order_is_on_it_and_not_iso() {
    let theta = (eps() / 20.0).sqrt();
    assert_eq!(
        require_iso_rectangle(&cylinder(), &cylinder_band(theta), band()),
        not_iso("props_rim_axis_parallel"),
        "tipped by {theta:e}"
    );
    assert_eq!(
        require_iso_rectangle(&cylinder(), &cylinder_band(0.05), band()),
        off_surface("props_cylinder_circle_tilt"),
    );
}

/// Toward the needle (`α = 10⁻³`), a radius-2 rim tipped `θ = 20·ε`: off
/// the cone by `2·(θ·sin α + (1 − cos θ)·cos α) ≈ 0.04·ε`, off the iso
/// rim by `2·sin θ = 40·ε`. Tipped `0.05` it is off the cone.
#[test]
fn a_needle_cone_rim_tipped_within_band_is_on_it_and_not_iso() {
    let alpha = 1e-3;
    let theta = 20.0 * eps();
    assert_eq!(
        require_iso_rectangle(&cone(alpha), &cone_band(alpha, 0.0, theta), band()),
        not_iso("props_rim_axis_parallel"),
    );
    assert_eq!(
        require_iso_rectangle(&cone(alpha), &cone_band(alpha, 0.0, 0.05), band()),
        off_surface("props_cone_circle_tilt"),
    );
}

/// Toward the flat (`α = π/2 − 10⁻³`), a rim moved `o = 40·ε` off the
/// axis: off the cone by `o·cos α ≈ 0.04·ε`, off the iso rim by `o`.
/// Moved `0.05` it is off the cone.
#[test]
fn a_near_flat_cone_rim_off_axis_within_band_is_on_it_and_not_iso() {
    let alpha = HALF_PI - 1e-3;
    let o = 40.0 * eps();
    assert_eq!(
        require_iso_rectangle(&cone(alpha), &cone_band(alpha, o, 0.0), band()),
        not_iso("props_rim_center_on_axis"),
    );
    assert_eq!(
        require_iso_rectangle(&cone(alpha), &cone_band(alpha, 0.05, 0.0), band()),
        off_surface("props_cone_circle_off_axis"),
    );
}

/// A loop as `(carrier, from, to)` edges at `f64`.
type Spec = Vec<(Curve3<f64>, f64, f64)>;

/// The iso parse of `spec` on `surface`, both lifted exactly into `T`.
fn parse<T: Decide>(surface: &Surface<f64>, spec: &Spec) -> Result<(), PropsError> {
    let n = u32::try_from(spec.len()).unwrap();
    let edges: Vec<LoopEdge<T>> = spec
        .iter()
        .zip(0..)
        .map(|((e, a, b), i)| topo::edge(e.map_scalar(T::from_f64), *a, *b, i, (i + 1) % n))
        .collect();
    require_iso_rectangle(&surface.map_scalar(T::from_f64), &edges, band())
}

/// `spec` parses to `want` at `f64` and at the interval scalar.
fn parses_to(surface: &Surface<f64>, spec: &Spec, want: &Result<(), PropsError>, what: &str) {
    assert_eq!(&parse::<f64>(surface, spec), want, "{what} at f64");
    assert_eq!(
        &parse::<Interval>(surface, spec),
        want,
        "{what} at the interval scalar"
    );
}

fn unit(v: [f64; 3]) -> Vec3<f64> {
    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    Vec3::new(v[0] / n, v[1] / n, v[2] / n)
}

/// A line from `p` along the unit `d` over `[0, len]`.
fn line(p: Point3<f64>, d: Vec3<f64>, len: f64) -> (Curve3<f64>, f64, f64) {
    (Curve3::Line { origin: p, dir: d }, 0.0, len)
}

/// A coaxial arc about `+z` at height `z`, radius `r`, from azimuth `a`
/// to `b`.
fn arc(z: f64, r: f64, a: f64, b: f64) -> (Curve3<f64>, f64, f64) {
    (
        Curve3::Circle {
            center: Point3::new(0.0, 0.0, z),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        },
        a,
        b,
    )
}

/// The unit cylinder's wall `u ∈ [0, 1]`, `z ∈ [0, 1]`, its ruling at
/// `u = 0` replaced by `first` and stated first.
fn cylinder_wall(first: (Curve3<f64>, f64, f64)) -> Spec {
    let (s, c) = 1.0_f64.sin_cos();
    vec![
        first,
        arc(1.0, 1.0, 0.0, 1.0),
        line(Point3::new(c, s, 1.0), Vec3::new(0.0, 0.0, -1.0), 1.0),
        arc(0.0, 1.0, 1.0, 0.0),
    ]
}

/// The ruling at `u = 0` from `z = 0`, its direction tipped by `θ`
/// toward `toward` (a unit vector normal to the axis).
fn tipped_ruling(theta: f64, toward: [f64; 3]) -> (Curve3<f64>, f64, f64) {
    let (s, c) = theta.sin_cos();
    line(
        Point3::origin() + Vec3::unit_x(),
        unit([s * toward[0], s * toward[1], c]),
        1.0,
    )
}

/// The ruling tipped by `θ = √(ε/20)` in the tangent plane ends
/// `θ²/2 = ε/40` off the cylinder and `sin θ` off the ruling, past
/// `2·K·ε` at every committed ε: on the cylinder, not a ruling. Tipped
/// as far toward the normal, outward or inward, it ends `sin θ` off. A
/// chord between two rim points has its ends on the cylinder and sags
/// `1 − cos ½` inside between them.
#[test]
fn a_cylinder_line_tipped_in_the_tangent_plane_is_on_it_and_not_a_ruling() {
    let cyl = cylinder();
    let theta = (eps() / 20.0).sqrt();
    parses_to(
        &cyl,
        &cylinder_wall(tipped_ruling(0.0, [0.0, 1.0, 0.0])),
        &Ok(()),
        "the wall",
    );
    parses_to(
        &cyl,
        &cylinder_wall(tipped_ruling(theta, [0.0, 1.0, 0.0])),
        &not_iso("props_meridian_axial"),
        "tipped tangentially",
    );
    for toward in [[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]] {
        parses_to(
            &cyl,
            &cylinder_wall(tipped_ruling(theta, toward)),
            &off_surface("props_meridian_on_surface"),
            &format!("tipped toward {toward:?}"),
        );
    }
    let (s, c) = 1.0_f64.sin_cos();
    let chord = [c - 1.0, s, 0.0];
    let len = (chord[0] * chord[0] + chord[1] * chord[1]).sqrt();
    parses_to(
        &cyl,
        &cylinder_wall(line(Point3::origin() + Vec3::unit_x(), unit(chord), len)),
        &off_surface("props_meridian_on_surface"),
        "a chord",
    );
}

/// The wall `u ∈ [0, 1]`, slant `v ∈ [1, 2]` of `cone(alpha)`, its
/// generator at `u = 0` replaced by `first` and stated first.
fn cone_wall(alpha: f64, first: (Curve3<f64>, f64, f64)) -> Spec {
    let (sa, ca) = alpha.sin_cos();
    let (s, c) = 1.0_f64.sin_cos();
    vec![
        first,
        arc(2.0 * ca, 2.0 * sa, 0.0, 1.0),
        line(
            Point3::new(2.0 * sa * c, 2.0 * sa * s, 2.0 * ca),
            Vec3::new(-sa * c, -sa * s, -ca),
            1.0,
        ),
        arc(ca, sa, 1.0, 0.0),
    ]
}

/// The generator at `u = 0` from slant 1 to 2, its direction turned
/// about its near end by `θ` toward `toward`.
fn tipped_generator(alpha: f64, theta: f64, toward: Vec3<f64>) -> (Curve3<f64>, f64, f64) {
    let (sa, ca) = alpha.sin_cos();
    let (s, c) = theta.sin_cos();
    let g = Vec3::new(sa, 0.0, ca);
    let d = g * c + toward * s;
    line(Point3::new(sa, 0.0, ca), unit([d.x, d.y, d.z]), 1.0)
}

/// At `α = ½`, the generator turned by `θ = √(ε/20)` in the tangent
/// plane ends `cos α·θ²/(4·sin α) < ε/40` off the cone and misses the
/// apex by `sin θ`, past `2·K·ε` at every committed ε: on the cone, not
/// a generator. Turned as far toward the normal, it ends `sin θ` off.
/// The line through both nappes at the generators' radius has its ends
/// on the cone and is `sin α·cos α` off it where it crosses the apex's
/// plane.
#[test]
fn a_cone_line_tipped_in_the_tangent_plane_is_on_it_and_not_a_generator() {
    let alpha = 0.5;
    let (sa, ca) = alpha.sin_cos();
    let cone = cone(alpha);
    let theta = (eps() / 20.0).sqrt();
    let tangent = Vec3::unit_y();
    let normal = Vec3::new(ca, 0.0, -sa);
    parses_to(
        &cone,
        &cone_wall(alpha, tipped_generator(alpha, 0.0, tangent)),
        &Ok(()),
        "the wall",
    );
    parses_to(
        &cone,
        &cone_wall(alpha, tipped_generator(alpha, theta, tangent)),
        &not_iso("props_meridian_apex"),
        "turned tangentially",
    );
    for toward in [normal, -normal] {
        parses_to(
            &cone,
            &cone_wall(alpha, tipped_generator(alpha, theta, toward)),
            &off_surface("props_meridian_on_surface"),
            &format!("turned toward {toward:?}"),
        );
    }
    parses_to(
        &cone,
        &cone_wall(
            alpha,
            line(
                Point3::new(sa, 0.0, ca),
                Vec3::new(0.0, 0.0, -1.0),
                2.0 * ca,
            ),
        ),
        &off_surface("props_meridian_on_surface"),
        "through both nappes",
    );
}

/// The sector `u ∈ [0, 1]`, slant `v ∈ [0, 2]` of the cone of
/// half-angle ½ about the skew axis `(0.3, −0.4, 1)` through the apex
/// `(0.2, −0.1, −1)`, its frame and carriers computed in `T`, so that at
/// the interval scalar every one is an enclosure of some width.
fn skew_sector<T: Real>() -> (Surface<T>, Vec<LoopEdge<T>>) {
    let f = T::from_f64;
    let (sa, ca) = f(0.5).sin_cos();
    let apex = Point3::new(f(0.2), f(-0.1), f(-1.0));
    let a = Vec3::new(f(0.3), f(-0.4), f(1.0));
    let a = a * (T::one() / a.norm());
    let e1 = Vec3::new(a.z, T::zero(), T::zero() - a.x);
    let e1 = e1 * (T::one() / e1.norm());
    let e2 = a.cross(e1);
    let g = |u: T| e1 * (sa * u.cos()) + e2 * (sa * u.sin()) + a * ca;
    let two = f(2.0);
    let surface = Surface::Cone {
        apex,
        axis: a,
        half_angle: f(0.5),
        u_ref: e1,
    };
    let edge = |carrier, t1: T, tag: u32| {
        LoopEdge::hand_built(carrier, T::zero(), t1, true, tag, (tag + 1) % 3)
    };
    let edges = vec![
        edge(
            Curve3::Line {
                origin: apex,
                dir: g(T::zero()),
            },
            two,
            0,
        ),
        edge(
            Curve3::Circle {
                center: apex + a * (two * ca),
                axis: a,
                radius: two * sa,
                u_ref: e1,
            },
            T::one(),
            1,
        ),
        edge(
            Curve3::Line {
                origin: apex + g(T::one()) * two,
                dir: Vec3::zero() - g(T::one()),
            },
            two,
            2,
        ),
    ];
    (surface, edges)
}

/// **A generator that ends at the apex is on the cone.** Its piece on
/// the apex's side of the apex's plane has both ends on the axis, so the
/// radial sag over it has nothing to divide by, and at the interval
/// scalar the axis's distance from them is an enclosure of zero: the
/// skew sector through the apex measures by the iso parse and by the
/// flux lane, at `f64` and at the interval scalar.
#[test]
fn a_cone_sector_through_its_apex_measures() {
    fn both<T: Decide>(scalar: &str) {
        let (surface, edges) = skew_sector::<T>();
        assert_eq!(
            require_iso_rectangle(&surface, &edges, band()),
            Ok(()),
            "the sector's iso parse at {scalar}"
        );
        assert_eq!(
            curved_face_loops(&surface, &[&edges], true, band()).map(|_| ()),
            Ok(()),
            "the sector's flux at {scalar}"
        );
    }
    both::<f64>("f64");
    both::<Interval>("the interval scalar");
}

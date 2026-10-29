//! **The pole form**: how a face meets a chart singularity of its
//! surface, a sphere's pole or a cone's apex.
//!
//! A chart singularity in a face's closure is a vertex of it, and the
//! kernel's form for that vertex is two **half-turn meridians**: the
//! singular point is an end of two meridian edges whose azimuths about
//! the surface axis are `d` and `−d`, each the other's half turn about
//! the axis, and a face that reaches the point reaches it between the
//! two. A full revolve sweeps its wire in two π-bands, so each pole it
//! makes ends the angle-0 and angle-π meridians (`sweep`'s
//! `revolve::full::build_wire`); import mints the same pair wherever a
//! file states the singularity some other way (`step-import`'s
//! edge-free sphere and its `normalize` pass).
//!
//! This module is the description those doors share: the azimuth pair
//! ([`azimuths`]), the half turn that carries one meridian onto the
//! other ([`half_turn`], [`half_turn_curve`]), and the meridian
//! carriers ([`sphere_meridian`], [`cone_generator`]). It says what the
//! cut is, not where a door makes it.

use geom_core::{Point3, Real, Vec3};

use crate::Curve3;

/// The pole form's two azimuths from one of them: `d` and its
/// [`half_turn`] about the axis. For `d` perpendicular to the axis, the
/// premise here, the half turn is exactly `−d`, which this spells with
/// nothing to round: `half_turn`'s `2(d·â)â − d` reads the rounding of
/// `d·â` into the result. One operation, two spellings, because the
/// meridian carriers take an azimuth and must state it bit for bit.
pub fn azimuths<T: Real>(d: Vec3<T>) -> [Vec3<T>; 2] {
    [d, -d]
}

/// `w` turned a half turn about the unit direction `axis`: the f64
/// identity `2(w·â)â − w`, with no trigonometry to round.
pub fn half_turn<T: Real>(w: Vec3<T>, axis: Vec3<T>) -> Vec3<T> {
    axis * (T::from_f64(2.0) * w.dot(axis)) - w
}

/// `carrier` turned a half turn about the line through `origin` along
/// `axis`: the other meridian of the pair, given one. `point` and
/// `vector` are applied to each placed point and direction after the
/// turn, for a caller that normalizes what it mints (signed zeros, for
/// one). Only a line or a circle has a pole-form twin; anything else
/// answers `None`.
pub fn half_turn_curve<T: Real>(
    carrier: &Curve3<T>,
    origin: Point3<T>,
    axis: Vec3<T>,
    point: impl Fn(Point3<T>) -> Point3<T>,
    vector: impl Fn(Vec3<T>) -> Vec3<T>,
) -> Option<Curve3<T>> {
    let place = |p: Point3<T>| point(origin + half_turn(p - origin, axis));
    let turn = |w: Vec3<T>| vector(half_turn(w, axis));
    match *carrier {
        Curve3::Line { origin: o, dir } => Some(Curve3::Line {
            origin: place(o),
            dir: turn(dir),
        }),
        Curve3::Circle {
            center,
            axis: n,
            radius,
            u_ref,
        } => Some(Curve3::Circle {
            center: place(center),
            axis: turn(n),
            radius,
            u_ref: turn(u_ref),
        }),
        _ => None,
    }
}

/// The sphere's meridian at azimuth `d` (a unit direction perpendicular
/// to `axis`): the circle through both poles with normal `axis × d` and
/// reference direction `axis`, so `θ = 0` is the pole `center +
/// axis·radius`, `θ = π/2` the equator point `center + d·radius`, and
/// `θ = π` the other pole.
pub fn sphere_meridian<T: Real>(
    center: Point3<T>,
    radius: T,
    axis: Vec3<T>,
    d: Vec3<T>,
) -> Curve3<T> {
    Curve3::Circle {
        center,
        axis: axis.cross(d),
        radius,
        u_ref: axis,
    }
}

/// The cone's generator at azimuth `d` (a unit direction perpendicular
/// to `axis`): the line through the apex whose parameter is the
/// surface's own slant `v`, `apex + (axis·cos α + d·sin α)·v`, so
/// `v > 0` runs on the nappe opening along `axis` and `v < 0` on the
/// other.
pub fn cone_generator<T: Real>(
    apex: Point3<T>,
    axis: Vec3<T>,
    half_angle: T,
    d: Vec3<T>,
) -> Curve3<T> {
    let (s, c) = half_angle.sin_cos();
    Curve3::Line {
        origin: apex,
        dir: axis * c + d * s,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    use crate::Surface;

    /// Each carrier runs on its surface through the singular point, at
    /// the azimuth it names, and the half turn carries one meridian of
    /// the pair onto the other.
    #[test]
    fn the_meridian_carriers_run_through_the_singularity_at_their_azimuth() {
        let (center, radius) = (Point3::new(1.0, -2.0, 0.5), 3.0);
        let axis = Vec3::new(0.0, 0.6, 0.8);
        let u_ref = Vec3::new(1.0, 0.0, 0.0);
        let sphere = Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        };
        for (k, d) in azimuths(u_ref).into_iter().enumerate() {
            let m = sphere_meridian(center, radius, axis, d);
            assert!(
                (m.eval(0.0) - (center + axis * radius)).norm() < 1e-15,
                "{k}: θ = 0"
            );
            let at = m.eval(core::f64::consts::FRAC_PI_2);
            assert!((at - (center + d * radius)).norm() < 1e-14, "{k}: θ = π/2");
            assert!(
                (m.eval(core::f64::consts::PI) - (center - axis * radius)).norm() < 1e-14,
                "{k}: θ = π"
            );
            let u = if k == 0 { 0.0 } else { core::f64::consts::PI };
            assert!((sphere.eval(u, 0.0) - at).norm() < 1e-14, "{k}: azimuth");
        }
        let twin = half_turn_curve(
            &sphere_meridian(center, radius, axis, u_ref),
            center,
            axis,
            |p| p,
            |w| w,
        )
        .unwrap();
        let other = sphere_meridian(center, radius, axis, -u_ref);
        for t in [0.3, 1.1, 2.9] {
            assert!(
                (twin.eval(t) - other.eval(t)).norm() < 1e-14,
                "the twin at {t}"
            );
        }

        let (apex, alpha) = (Point3::new(0.0, 0.0, 2.0), 0.4);
        let cone = Surface::Cone {
            apex,
            axis: Vec3::new(0.0, 0.0, -1.0),
            half_angle: alpha,
            u_ref,
        };
        let Surface::Cone { axis: ca, .. } = cone else {
            unreachable!()
        };
        for (k, d) in azimuths(u_ref).into_iter().enumerate() {
            let g = cone_generator(apex, ca, alpha, d);
            let u = if k == 0 { 0.0 } else { core::f64::consts::PI };
            for v in [-1.5, 0.0, 0.7] {
                assert!((g.eval(v) - cone.eval(u, v)).norm() < 1e-14, "{k}: v = {v}");
            }
        }
    }
}

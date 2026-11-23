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

use geom_core::{Point3, Real, RigidMotion, Vec3};

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
/// `axis`: the other meridian of the pair, given one. `entry` is applied
/// to each placed coordinate after the turn, for a caller that states
/// one representative of what it mints (a signed zero folded), and must
/// keep each entry's value. Only a line or a circle has a pole-form
/// twin; anything else answers `None`.
pub fn half_turn_curve<T: Real>(
    carrier: &Curve3<T>,
    axis: crate::Line<T>,
    entry: impl Fn(T) -> T,
) -> Option<Curve3<T>> {
    let crate::Line { origin, dir: axis } = axis;
    let vector = |w: Vec3<T>| Vec3::new(entry(w.x), entry(w.y), entry(w.z));
    let place = |p: Point3<T>| {
        let q = origin + half_turn(p - origin, axis.get());
        Point3::new(entry(q.x), entry(q.y), entry(q.z))
    };
    let turn = |w: Vec3<T>| vector(half_turn(w, axis.get()));
    match *carrier {
        // A half turn is rigid: the direction's own door
        // ([`UnitVec3::half_turned`]), its entries folded in their kind.
        Curve3::Line(crate::Line { origin: o, dir }) => Some(Curve3::line(
            place(o),
            dir.half_turned(axis).map_scalar(&entry),
        )),
        Curve3::Circle(c) => Some(Curve3::circle(
            place(c.frame().origin()),
            turn(c.frame().axis()),
            c.radius(),
            turn(c.frame().u_ref()),
        )),
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
    Curve3::circle(center, axis.cross(d), radius, axis)
}

/// The cone's generator in `frame`'s `w`–`u` plane: the line through the
/// apex `frame.origin()` whose parameter is the surface's own slant `v`,
/// `apex + (w·cos α + u·sin α)·v` ([`RigidMotion::swing`]), so `v > 0`
/// runs on the nappe opening along the axis `w` and `v < 0` on the
/// other, at the azimuth `u`. A caller holding an axis and an azimuth
/// squares the azimuth about the axis first
/// ([`RigidMotion::squared_about`]).
pub fn cone_generator<T: Real>(frame: RigidMotion<T>, half_angle: T) -> Curve3<T> {
    let (s, c) = half_angle.sin_cos();
    Curve3::line(frame.origin(), frame.swing(c, s))
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
            frame: crate::Frame::new(center, axis, u_ref),
            radius,
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
            crate::Line {
                origin: center,
                dir: crate::stated_direction(axis),
            },
            |x| x,
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
            frame: crate::Frame::new(apex, Vec3::new(0.0, 0.0, -1.0), u_ref),
            half_angle: alpha,
        };
        let Surface::Cone { frame, .. } = cone else {
            unreachable!()
        };
        let _ca = frame.axis();
        for (k, d) in azimuths(u_ref).into_iter().enumerate() {
            let g = cone_generator(
                RigidMotion::squared_about(apex, frame.axis_unit(), d),
                alpha,
            );
            let u = if k == 0 { 0.0 } else { core::f64::consts::PI };
            for v in [-1.5, 0.0, 0.7] {
                assert!((g.eval(v) - cone.eval(u, v)).norm() < 1e-14, "{k}: v = {v}");
            }
        }
    }

    /// **A generator at an azimuth read near the axis, far along it,
    /// opens at its half-angle**: the azimuth of a point `50 m` along a
    /// tilted axis and `1e-5 m` off it, read as the importer reads one
    /// (the point's offset from the axis, normalized), stands off square
    /// to the axis by the axial distance's rounding over the radius,
    /// millions of ulps. Squared about the axis
    /// ([`RigidMotion::squared_about`]), the generator swings in one
    /// frame's plane: unit to its rounding, and `α` off the axis to a few
    /// ulps at every azimuth tried. Red under the generator swung toward
    /// the raw azimuth, `w·cos α + d·sin α`, whose angle to the axis is
    /// off by the azimuth's tilt.
    #[test]
    fn a_generator_read_near_the_axis_far_along_it_opens_at_its_half_angle() {
        let apex = Point3::new(0.3, -1.2, 0.7);
        let axis = crate::stated_direction(Vec3::new(2.0, -1.0, 2.0) / 3.0);
        let across = Vec3::new(1.0, 2.0, 0.0) / 5.0_f64.sqrt();
        let alpha = 0.35_f64;
        let mut tilted = 0;
        for k in 0..64 {
            let t = f64::from(k) * 0.098;
            let ring = across * t.cos() + axis.get().cross(across) * t.sin();
            let p = apex + axis.get() * 50.0 + ring * 1e-5;
            let d = crate::Line {
                origin: apex,
                dir: axis,
            }
            .offset(p)
            .normalize();
            tilted += usize::from(d.dot(axis.get()).abs() > 1e3 * f64::EPSILON);
            let Curve3::Line(g) = cone_generator(RigidMotion::squared_about(apex, axis, d), alpha)
            else {
                unreachable!("a generator is a line")
            };
            assert!(
                geom_core::UnitVec3::stated(g.dir.get()).is_some(),
                "{k}: unit"
            );
            let off = (g.dir.get().dot(axis.get()) - alpha.cos()).abs();
            assert!(off <= 4.0 * f64::EPSILON, "{k}: {off:e} off cos α");
        }
        assert!(tilted > 0, "some azimuth stands off square to the axis");
    }
}

//! **Geom's readings of the region an entity can occupy**: a bounded
//! carrier whole ([`Surface::cover`]) and a curve over a span
//! ([`Curve3::cover_over`]). Each is one exhaustive match over its kind,
//! and the only kind matches a cover has: a kind added to either enum
//! does not compile until its arm says what region it occupies.
//!
//! A carrier's data enters a piece as given: each piece reads its own
//! radii's signs, once, and holds any frame its carrier hands it. Data
//! that is not a finite number names no bounded region, and refuses
//! [`Unknown::NonFinite`].

use geom_core::{Point3, Real};

use super::{Cover, Covered, Entity, ParamWindow, Uncovered, Unknown};
use crate::{Curve3, Line, Surface};

impl<T: Real> Surface<T> {
    /// **The region the whole carrier occupies**: a sphere's or a torus's
    /// tube (the sphere's with no spine; along a unit `e` in a unit
    /// orthogonal frame the support is `c·e + R·|e⊥| + r`), a spline's
    /// control net (the fit's, for an approximating surface).
    ///
    /// # Errors
    ///
    /// [`Uncovered`] naming the kind: [`Unknown::Unbounded`] for a plane,
    /// a cylinder or a cone, [`Unknown::PoisonedNet`] for a net carrying
    /// poison anywhere (the placeholder among them), and
    /// [`Unknown::NonFinite`] for data that is not a finite number.
    pub fn cover(&self) -> Covered<T> {
        let entity = Entity::Surface(self.kind());
        let refuse = |what| Uncovered::new(entity, what);
        match self {
            Surface::Plane { .. } | Surface::Cylinder { .. } | Surface::Cone { .. } => {
                Err(refuse(Unknown::Unbounded))
            }
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => Cover::tube(*center, *axis, *u_ref, T::zero(), *radius)
                .ok_or_else(|| refuse(Unknown::NonFinite)),
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => Cover::tube(*center, *axis, *u_ref, *major_radius, *minor_radius)
                .ok_or_else(|| refuse(Unknown::NonFinite)),
            Surface::Nurbs(patch) => net(entity, patch.control()),
            Surface::Approx(approx) => net(entity, approx.fit().control()),
        }
    }
}

impl<T: Real> Curve3<T> {
    /// **The region the curve occupies over `window`**: a line's segment
    /// between its points at the two ends, a circle's or an ellipse's arc
    /// over the span, a spiric's tube met with its period's rectangle
    /// (the construction [`crate::spiric_period_box`] reads), a spline's
    /// control net.
    ///
    /// # Errors
    ///
    /// [`Uncovered`] naming the kind: [`Unknown::PoisonedNet`] for a
    /// spline whose net carries poison anywhere, and
    /// [`Unknown::NonFinite`] for data, a window end or a computed point
    /// that is not a finite number (a spiric off its regime among them).
    pub fn cover_over(&self, window: ParamWindow<T>) -> Covered<T> {
        let entity = Entity::Curve(self.kind());
        let non_finite = || Uncovered::new(entity, Unknown::NonFinite);
        match self {
            Curve3::Line(Line { origin, dir }) => {
                Cover::segment(*origin + *dir * window.lo, *origin + *dir * window.hi)
                    .ok_or_else(non_finite)
            }
            Curve3::Circle(circle) => Cover::arc(
                circle.frame.center,
                (circle.frame.u_ref, circle.frame.v_ref()),
                (circle.radius, circle.radius),
                window,
            )
            .ok_or_else(non_finite),
            Curve3::Ellipse(ellipse) => Cover::arc(
                ellipse.frame.center,
                (ellipse.frame.u_ref, ellipse.frame.v_ref()),
                (ellipse.semi_u, ellipse.semi_v),
                window,
            )
            .ok_or_else(non_finite),
            Curve3::Spiric {
                center,
                axis,
                u_ref,
                major_radius,
                minor_radius,
                offset,
            } => {
                let tube = Cover::tube(*center, *axis, *u_ref, *major_radius, *minor_radius);
                let rectangle = crate::curves::spiric_rectangle(
                    *center,
                    *axis,
                    *u_ref,
                    *minor_radius,
                    *offset,
                    crate::spiric_f_range(*major_radius, *minor_radius, *offset),
                    T::zero(),
                );
                tube.zip(rectangle)
                    .map(|(tube, rectangle)| tube.meet(rectangle))
                    .ok_or_else(non_finite)
            }
            Curve3::Nurbs(spline) => net(entity, spline.control()),
        }
    }
}

/// A control net's hull, where it bounds a region: no point carries
/// poison in any channel (`net::any_poison`, the question a box asks),
/// there is a point, and every coordinate is finite.
fn net<T: Real>(entity: Entity, control: &[Point3<T>]) -> Covered<T> {
    if control.is_empty() || crate::net::any_poison(control) {
        return Err(Uncovered::new(entity, Unknown::PoisonedNet));
    }
    Cover::net(control.to_vec()).ok_or_else(|| Uncovered::new(entity, Unknown::NonFinite))
}

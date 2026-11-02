//! **Geom's readings of the region an entity can occupy**: a bounded
//! carrier whole ([`Surface::cover`]) and a curve over a span
//! ([`Curve3::cover_over`]). Each is one exhaustive match over its kind,
//! and the only kind matches a cover has: a kind added to either enum
//! does not compile until its arm says what region it occupies.
//!
//! Sign handling is here, once: a radius enters a piece as its
//! magnitude, so a carrier carrying `−r` occupies the region `r` does.

use geom_core::{Point3, Real};

use super::{Cover, Covered, Entity, Node, ParamWindow, Piece, Uncovered, Unknown};
use crate::{Curve3, Line, Surface};

impl<T: Real> Surface<T> {
    /// **The region the whole carrier occupies**: a sphere's ball, a
    /// torus's tube (its support along a unit `e` is `c·e + R·|e⊥| + r`
    /// exactly), a spline's control net (the fit's, for an approximating
    /// surface).
    ///
    /// # Errors
    ///
    /// [`Uncovered`] naming the kind: [`Unknown::Unbounded`] for a plane,
    /// a cylinder or a cone, and [`Unknown::PoisonedNet`] for a net
    /// carrying poison anywhere (the placeholder among them).
    pub fn cover(&self) -> Covered<T> {
        let refuse = |what| Err(Uncovered::new(Entity::Surface(self.kind()), what));
        match self {
            Surface::Plane { .. } | Surface::Cylinder { .. } | Surface::Cone { .. } => {
                refuse(Unknown::Unbounded)
            }
            Surface::Sphere { center, radius, .. } => Ok(Cover::whole_ball(*center, *radius)),
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => Ok(Cover::tube(
                *center,
                *axis,
                *u_ref,
                *major_radius,
                *minor_radius,
            )),
            Surface::Nurbs(patch) => {
                net(patch.control()).map_or_else(|| refuse(Unknown::PoisonedNet), Ok)
            }
            Surface::Approx(approx) => {
                net(approx.fit().control()).map_or_else(|| refuse(Unknown::PoisonedNet), Ok)
            }
        }
    }
}

impl<T: Real> Curve3<T> {
    /// **The region the curve occupies over `window`**: a line's segment
    /// between its points at the two ends, a circle's or an ellipse's arc
    /// over the span, a spiric's tube met with its period's rectangle
    /// (the oval lies in its plane at `f ∈ [f_min, f_max]` along
    /// `axis × u_ref` and within `±r` along the axis), a spline's control
    /// net.
    ///
    /// # Errors
    ///
    /// [`Uncovered`] with [`Unknown::PoisonedNet`] for a spline whose net
    /// carries poison anywhere.
    pub fn cover_over(&self, window: ParamWindow<T>) -> Covered<T> {
        match self {
            Curve3::Line(Line { origin, dir }) => Ok(Cover::segment(
                *origin + *dir * window.lo,
                *origin + *dir * window.hi,
            )),
            Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => Ok(Cover::arc(
                *center,
                (*u_ref, axis.cross(*u_ref)),
                (*radius, *radius),
                window,
            )),
            Curve3::Ellipse {
                center,
                axis,
                semi_u,
                semi_v,
                u_ref,
            } => Ok(Cover::arc(
                *center,
                (*u_ref, axis.cross(*u_ref)),
                (*semi_u, *semi_v),
                window,
            )),
            Curve3::Spiric {
                center,
                axis,
                u_ref,
                major_radius,
                minor_radius,
                offset,
            } => {
                let (f_lo, f_hi) =
                    crate::curves::spiric_f_enclosed(*major_radius, *minor_radius, *offset);
                let m = axis.cross(*u_ref);
                let base = *center + *u_ref * *offset;
                let tip = *axis * minor_radius.abs();
                let corners = vec![
                    base + m * f_lo + tip,
                    base + m * f_lo - tip,
                    base + m * f_hi + tip,
                    base + m * f_hi - tip,
                ];
                let scale = (*center - Point3::origin()).norm()
                    + u_ref.norm() * offset.abs()
                    + m.norm() * f_hi
                    + axis.norm() * minor_radius.abs();
                let rectangle = Cover(Node::Piece(Piece::Net {
                    points: corners,
                    slack: super::curved_charge(scale),
                }));
                Ok(
                    Cover::tube(*center, *axis, *u_ref, *major_radius, *minor_radius)
                        .meet(rectangle),
                )
            }
            Curve3::Nurbs(spline) => net(spline.control())
                .ok_or_else(|| Uncovered::new(Entity::Curve(self.kind()), Unknown::PoisonedNet)),
        }
    }
}

/// A control net's hull, where it bounds a region: no point carries
/// poison in any channel (`net::any_poison`, the question a box asks),
/// and there is a point.
fn net<T: Real>(control: &[Point3<T>]) -> Option<Cover<T>> {
    if crate::net::any_poison(control) {
        return None;
    }
    Cover::net(control.to_vec())
}

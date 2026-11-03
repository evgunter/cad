//! **Geom's readings of the region an entity can occupy**: a bounded
//! carrier whole ([`Surface::cover`], [`Curve3::cover`]) and a curve
//! over a span ([`Curve3::cover_over`]). Each is one exhaustive match over its kind,
//! and the only kind matches a cover has: a kind added to either enum
//! does not compile until its arm says what region it occupies.
//!
//! A carrier's data enters a piece as given: each piece reads its own
//! radii's signs, once, and holds any frame its carrier hands it. Data
//! that is not a finite number names no bounded region, and refuses
//! [`Unknown::NonFinite`].

use geom_core::{Point3, Real};

use geom_core::spline::SpanLocate;

use super::{Cover, Covered, Entity, Lattice, ParamWindow, Uncovered, Unknown};
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

impl<T: SpanLocate> Curve3<T> {
    /// **The region the whole carrier occupies**: a circle's or an
    /// ellipse's whole turn, a spiric's whole period, a spline's control
    /// net over its domain. The whole-carrier sibling of
    /// [`Curve3::cover_over`], and of [`Surface::cover`].
    ///
    /// # Errors
    ///
    /// As [`Curve3::cover_over`], and [`Unknown::Unbounded`] for a line.
    pub fn cover(&self) -> Covered<T> {
        match self {
            Curve3::Line(_) => Err(Uncovered::new(
                Entity::Curve(self.kind()),
                Unknown::Unbounded,
            )),
            Curve3::Circle(_) | Curve3::Ellipse(_) | Curve3::Spiric { .. } => {
                self.cover_over(ParamWindow::new(T::zero(), T::tau()))
            }
            Curve3::Nurbs(spline) => net(Entity::Curve(self.kind()), spline.control()),
        }
    }

    /// **The region the curve occupies over `window`**: a line's segment
    /// between its points at the two ends, a circle's or an ellipse's arc
    /// over the span, a spiric's tube met with the window's rectangle
    /// (the range of `f` and of the axial channel over the window), a
    /// spline's control net met with the hull of its pieces' nets over
    /// the window cut in [`SPLINE_PIECES`]
    /// ([`NurbsCurve3::restricted_nets`](crate::NurbsCurve3::restricted_nets)).
    /// Each narrows with the window.
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
                    (*major_radius, *minor_radius),
                    *offset,
                    window,
                );
                tube.zip(rectangle)
                    .map(|(tube, rectangle)| tube.meet(rectangle))
                    .ok_or_else(non_finite)
            }
            Curve3::Nurbs(spline) => {
                let whole = net(entity, spline.control())?;
                // The window cut in `SPLINE_PIECES`, each piece's nets over
                // it: a refusal here (an end the knots cannot locate, a net
                // point that is not finite) leaves the whole net, which
                // holds the curve already.
                let (lo, hi) = (window.lo, window.hi);
                let step = (hi - lo) / T::from_f64(f64::from(SPLINE_PIECES));
                let end = |k: u32| match k {
                    0 => lo,
                    k if k == SPLINE_PIECES => hi,
                    k => lo + step * T::from_f64(f64::from(k)),
                };
                let pieces = (0..SPLINE_PIECES)
                    .map(|k| spline.restricted_nets(end(k), end(k + 1)))
                    .collect::<Option<Vec<_>>>()
                    .and_then(|nets| {
                        nets.into_iter()
                            .flatten()
                            .map(Cover::net)
                            .reduce(|a, b| Some(a?.hull(b?)))
                            .flatten()
                    })
                    .ok_or_else(non_finite);
                Ok(whole).meet(pieces)
            }
        }
    }
}

/// **How many equal pieces a spline's window is cut into**, each read
/// by its own pieces' nets. A piece's net stands off the curve by the
/// square of its width times the curve's bend, so cutting narrows the
/// region quadratically where the window's own net (a whole carrier's,
/// for a window over the whole domain) can stand off it by the net's
/// size: four pieces read a quadratic arc within a sixteenth of its
/// window's net's standoff.
pub const SPLINE_PIECES: u32 = 4;

/// A control net's hull, where it bounds a region: no point carries
/// poison in any channel (`net::any_poison`, the question a box asks)
/// and every coordinate is finite.
///
/// # Panics
///
/// On a net with no point: a spline's knots name `p + 1 ≥ 1` control
/// points (its constructor's count check), so an empty net is a kernel
/// bug, not a carrier.
fn net<T: Real>(entity: Entity, control: &[Point3<T>]) -> Covered<T> {
    assert!(
        !control.is_empty(),
        "a spline carrier's net has a point: its knots name p + 1 control points"
    );
    if crate::net::any_poison(control) {
        return Err(Uncovered::new(entity, Unknown::PoisonedNet));
    }
    Cover::net(control.to_vec()).ok_or_else(|| Uncovered::new(entity, Unknown::NonFinite))
}

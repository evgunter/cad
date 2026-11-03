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
    /// **The region the whole carrier occupies**: a sphere's, a torus's or
    /// a spindle sheet's tube (the sphere's with no spine, the sheet's
    /// about the spine `|R|`, which holds its whole revolved circle; along
    /// a unit `e` in a unit orthogonal frame the support is
    /// `c·e + R·|e⊥| + r`), a spline's
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
            // The sheet lies on its whole revolved circle, which is the
            // tube about the circle of radius `|R|`: the signed centre
            // and its mirror across the axis sweep the same set.
            Surface::Spindle {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => Cover::tube(*center, *axis, *u_ref, major_radius.abs(), *minor_radius)
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
    /// the window cut in [`SPLINE_PIECES`] (`piece_nets`, the scalar's
    /// own restriction).
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
                // The cuts are points the cover chooses: each interior one
                // computed from the window's ends and entered as the
                // constant it is, so the pieces meet end to end at every
                // scalar. A window end that is no single point leaves the
                // whole net.
                let ends = window.lo.point_value().zip(window.hi.point_value());
                let end = |k: u32| {
                    let (lo, hi) = ends?;
                    Some(match k {
                        0 => (lo, window.lo),
                        k if k == SPLINE_PIECES => (hi, window.hi),
                        k => {
                            let cut = lo + (hi - lo) * f64::from(k) / f64::from(SPLINE_PIECES);
                            (cut, T::from_f64(cut))
                        }
                    })
                };
                let pieces = (0..SPLINE_PIECES)
                    .map(|k| piece_nets(spline, (end(k)?, end(k + 1)?)))
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

/// **A spline's pieces over the window between `a` and `b`** (either
/// order), each end its `f64` point and its value in the scalar, as
/// control nets: for each knot span the window meets, the
/// dehomogenized Bernstein net of the piece over the meet, each
/// homogeneous channel `(w·P, w)` restricted by the scalar's own door
/// ([`SpanLocate::restrict_window`]: de Boor's recurrence at `f64`, the
/// certified restriction at `Interval`). A meet end that is a knot is
/// that knot's held value, and a window end is the window's own. With
/// positive weights each piece lies in its net's hull.
///
/// `None`, which leaves the whole net, for a held knot that is not a
/// single point, a meet the door refuses, and a restricted weight not
/// positive for every value it stands for.
fn piece_nets<T: SpanLocate>(
    spline: &crate::NurbsCurve3<T>,
    (a, b): ((f64, T), (f64, T)),
) -> Option<Vec<Vec<Point3<T>>>> {
    let ((lo, lo_t), (hi, hi_t)) = if a.0 <= b.0 { (a, b) } else { (b, a) };
    let knots = spline.knot_values();
    let (first, _) = knots.locate(lo_t)?;
    let (_, last) = knots.locate(hi_t)?;
    let held = knots.values();
    let held_at = |x: f64, own: T| match knots.value_at(x) {
        Ok(Some(k)) => k,
        _ => own,
    };
    let mut nets = Vec::new();
    for index in first.span().index()..=last.span().index() {
        let Some(span) = knots.span(index).map(|s| s.span()) else {
            continue;
        };
        let (k0, k1) = (held[index].point_value()?, held[index + 1].point_value()?);
        let ends = (lo.max(k0), hi.min(k1));
        let held_ends = (held_at(ends.0, lo_t), held_at(ends.1, hi_t));
        let homogeneous: Vec<[T; 4]> = span
            .window()
            .map(|i| {
                let (w, c) = (spline.weights()[i], spline.control()[i]);
                [c.x * w, c.y * w, c.z * w, w]
            })
            .collect();
        let channel = |c: usize| {
            let line: Vec<T> = homogeneous.iter().map(|h| h[c]).collect();
            T::restrict_window(&line, span, held, ends, held_ends)
        };
        let [x, y, z, w] = [0, 1, 2, 3].map(channel);
        if !w.iter().all(|&w| w.structure_positive()) {
            return None;
        }
        nets.push(
            (0..w.len())
                .map(|k| Point3::new(x[k] / w[k], y[k] / w[k], z[k] / w[k]))
                .collect(),
        );
    }
    (!nets.is_empty()).then_some(nets)
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

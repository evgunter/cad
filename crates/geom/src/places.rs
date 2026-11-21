//! **A place, handed to the evaluator as an `f64`** (DL5.2's place row,
//! `geom_core`'s README, "Reading a bracket where nothing certifies").
//!
//! A place is an `f64` the code chose to evaluate at: a sample, a
//! station, a cell corner, a foot that is only read. The evaluators
//! here take it as an `f64` and answer at the geometry's own scalar,
//! and they exist only where that scalar is its own value channel
//! (`T: SpanLocate<Value = T>`, a bound no dual meets): a curve that
//! carries a tangent has no place-taking evaluator. Such a curve is
//! first read on its value channel ([`Real::view`], each type's
//! `value_view()`), and evaluated there.
//!
//! The one step from the `f64` to the scalar is [`at_place`], private
//! to this module, so no caller holds a place as a scalar.
//!
//! ```compile_fail,E0599
//! fn at_a_dual(c: &geom::NurbsCurve3<geom_core::Dual64>) {
//!     let _ = c.eval_at(0.5);
//! }
//! ```
//!
//! ```compile_fail,E0599
//! fn at_a_dual(s: &geom::Surface<geom_core::DualInterval>) {
//!     let _ = s.eval_at(0.5, 0.5);
//! }
//! ```
//!
//! ```compile_fail,E0603
//! let _: f64 = geom::places::at_place(0.5);
//! ```
//!
//! ```
//! fn on_its_values(c: &geom::NurbsCurve3<geom_core::Dual64>) -> f64 {
//!     c.value_view().eval_at(0.5).x
//! }
//! ```

use geom_core::spline::SpanLocate;
use geom_core::{CertifiedBounds, Point2, Point3, Real, Vec2, Vec3};

use crate::curves::{Curve3, CurveWindow2, CurveWindow3, NurbsCurve2, NurbsCurve3};
use crate::surfaces::{NurbsSurface, Surface, SurfaceProjection, SurfaceProjectionInconclusive};

/// **The step from a place to the scalar**, at a scalar that is its own
/// value channel. What it means at each scalar the bound admits:
///
/// - `f64` and the probe: the number itself.
/// - `Interval`: the point `[x, x]`; what is evaluated there encloses
///   the geometry's exact value at that point.
/// - `Sym`: its own unknown. The place was chosen at the nominal
///   values, so it is no function of the document's unknowns anything
///   tracks; a re-valuation of what is read there at other values has
///   no place to read at and refuses, where an exact constant would
///   re-value at a place nobody would have chosen there.
fn at_place<T: Real<Value = T>>(x: f64) -> T {
    T::from_computed(x)
}

macro_rules! value_views {
    ($($family:ident: $($ty:ident)::+;)*) => {$(
        /// The family [`Real::view`] maps this type through.
        struct $family;

        impl geom_core::OverScalar for $family {
            type At<S: Real> = $($ty)::+<S>;

            fn map<S: Real, U: Real>(x: &$($ty)::+<S>, f: impl Fn(S) -> U) -> $($ty)::+<U> {
                x.map_scalar(f)
            }
        }

        impl<T: Real> $($ty)::+<T> {
            /// This geometry without its tangent ([`Real::view`]): itself,
            /// borrowed, where the scalar carries none, and a copy at a
            /// dual. Places are evaluated on it.
            #[must_use]
            pub fn value_view(&self) -> geom_core::ValueView<'_, $($ty)::+<T::Value>> {
                T::view::<$family>(self)
            }
        }
    )*};
}

value_views! {
    CurveValues: crate::curves::Curve3;
    SurfaceValues: crate::surfaces::Surface;
    NurbsCurve2Values: crate::curves::NurbsCurve2;
    NurbsCurve3Values: crate::curves::NurbsCurve3;
    NurbsSurfaceValues: crate::surfaces::NurbsSurface;
}

macro_rules! curve_places {
    ($Curve:ident, $Window:ident, $Point:ident, $Vector:ident) => {
        impl<T: SpanLocate<Value = T>> $Curve<T> {
            /// The point at the place `t`.
            #[must_use]
            pub fn eval_at(&self, t: f64) -> $Point<T> {
                self.eval(at_place(t))
            }

            /// The first derivative at the place `t`.
            #[must_use]
            pub fn deriv_at(&self, t: f64) -> $Vector<T> {
                self.deriv(at_place(t))
            }
        }

        impl<T: Real<Value = T>> $Window<'_, T> {
            /// The point at the place `t` inside this span.
            #[must_use]
            pub fn eval_in_span_at(self, t: f64) -> $Point<T> {
                self.eval_in_span(at_place(t))
            }
        }
    };
}

curve_places!(NurbsCurve2, CurveWindow2, Point2, Vec2);
curve_places!(NurbsCurve3, CurveWindow3, Point3, Vec3);

impl<T: SpanLocate<Value = T>> Curve3<T> {
    /// The point at the place `t`.
    #[must_use]
    pub fn eval_at(&self, t: f64) -> Point3<T> {
        self.eval(at_place(t))
    }
}

impl<T: SpanLocate<Value = T>> NurbsSurface<T> {
    /// The point at the place `(u, v)`.
    #[must_use]
    pub fn eval_at(&self, u: f64, v: f64) -> Point3<T> {
        self.eval(at_place(u), at_place(v))
    }
}

impl<T: SpanLocate<Value = T>> Surface<T> {
    /// The point at the place `(u, v)`.
    #[must_use]
    pub fn eval_at(&self, u: f64, v: f64) -> Point3<T> {
        self.eval(at_place(u), at_place(v))
    }
}

impl<T: CertifiedBounds> NurbsSurface<T> {
    /// [`Self::project`] of the place `p`, a point in space the caller
    /// chose to project from.
    ///
    /// # Errors
    ///
    /// As [`Self::project`].
    pub fn project_at(
        &self,
        p: Point3<f64>,
    ) -> Result<SurfaceProjection<T>, SurfaceProjectionInconclusive> {
        self.project(p.map(at_place))
    }
}

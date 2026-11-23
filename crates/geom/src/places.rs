//! **A place, handed to the evaluator as an `f64`** (DL5.2's place row,
//! `geom_core`'s README, "Reading a bracket where nothing certifies").
//!
//! A place is an `f64` the code chose to evaluate at: a sample, a
//! station, a cell corner, a foot that is only read. The evaluators of
//! this module take it as an `f64` and answer at the geometry's own
//! scalar, and they exist only where that scalar carries no tangent:
//! `T: SpanLocate<Value = T>` for the curves and surfaces,
//! `T: Real<Value = T>` for a curve's span window, `T: CertifiedBounds`
//! for [`NurbsSurface::project_at`]. No dual meets any of the three, so
//! a geometry that carries a tangent has no place-taking evaluator. It
//! is first read without its tangent, through its `value_view()` (for
//! instance [`NurbsCurve3::value_view`]), and evaluated there.
//!
//! The one step from the `f64` to the scalar is private to this module,
//! so no caller holds a place as a scalar. At `Sym` that step is its own
//! unknown: the place was chosen at the nominal values, so a
//! re-valuation of what is read there at other values refuses rather
//! than reading at a place nobody would have chosen there.
//!
//! ```compile_fail,E0599
//! fn at_a_dual(c: &geom::NurbsCurve3<geom_core::Dual64>) {
//!     let _ = c.eval_at(0.5);
//! }
//! ```
//!
//! ```
//! fn at_f64(c: &geom::NurbsCurve3<f64>) -> f64 {
//!     c.eval_at(0.5).x
//! }
//! ```
//!
//! ```compile_fail,E0599
//! fn at_a_dual(s: &geom::Surface<geom_core::DualInterval>) {
//!     let _ = s.eval_at(0.5, 0.5);
//! }
//! ```
//!
//! ```
//! fn at_an_interval(s: &geom::Surface<geom_core::Interval>) -> geom_core::Interval {
//!     s.eval_at(0.5, 0.5).x
//! }
//! ```
//!
//! The step is private to the function, not only to the module:
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

/// **The step from a place to the scalar**, at a scalar that carries no
/// tangent. Private, and called only in this file
/// (`the_step_to_a_place_is_this_files_own` pins both). What it means at
/// each scalar the bound admits:
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
        /// The family `Real::view` maps this type through.
        struct $family;

        impl geom_core::OverScalar for $family {
            type At<S: Real> = $($ty)::+<S>;

            fn map<S: Real, U: Real>(x: &$($ty)::+<S>, f: impl Fn(S) -> U) -> $($ty)::+<U> {
                x.map_scalar(f)
            }
        }

        impl<T: Real> $($ty)::+<T> {
            /// This geometry without its tangent ([`Real::without_tangent`]
            /// of every scalar it holds): itself, borrowed, where the
            /// scalar carries none, and a copy at a dual. Places are
            /// evaluated on it.
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

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    /// **The step to a place is this file's own**: `at_place` is declared
    /// without `pub` of any kind, and no other file of the crate names it.
    /// Red on a `pub`, `pub(crate)` or `pub(super)` step, and on a caller
    /// elsewhere, which the E0603 doctest outside the crate cannot see.
    #[test]
    fn the_step_to_a_place_is_this_files_own() {
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let own = test_utils::source::code_only(
            &std::fs::read_to_string(src.join("places.rs")).expect("places.rs reads"),
        );
        let body = own.split("#[cfg(test)]").next().unwrap_or_default();
        let declared: Vec<&str> = body.lines().filter(|l| l.contains("fn at_place")).collect();
        assert_eq!(
            declared,
            ["fn at_place<T: Real<Value = T>>(x: f64) -> T {"],
            "at_place is declared once, at column zero, with no visibility"
        );
        let elsewhere: Vec<std::path::PathBuf> = test_utils::source::rust_sources(&src)
            .into_iter()
            .filter(|path| {
                !path.ends_with("places.rs")
                    && test_utils::source::code_only(
                        &std::fs::read_to_string(path).expect("a source file reads"),
                    )
                    .contains("at_place")
            })
            .collect();
        assert!(
            elsewhere.is_empty(),
            "at_place named outside places.rs: {elsewhere:?}"
        );
    }
}

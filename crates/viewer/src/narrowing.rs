//! The display seam: this crate's one `f64` → `f32` narrowing.
//!
//! The kernel's geometry is `f64` and a GPU consumes `f32`, so every
//! position, matrix and uniform this crate hands a renderer crosses one
//! conversion. [`Narrow`] IS that conversion. It is a single door
//! rather than a cast written at each site because the rule the sites
//! share — what the seam does with a value the narrowing cannot carry —
//! is one rule, and a rule spelled at five sites is five rules that
//! happen to agree today.
//!
//! # It refuses, and it judges the RESULT
//!
//! `f32::MAX` is about `3.40e38`, so an ordinary finite `f64` narrows
//! to an infinity — `7e307_f64 as f32` is `inf`, and `7e307` is a
//! number a person can type into the add-profile form. Guards sited
//! upstream cannot see that: they ask whether the INPUT is finite, and
//! it is. [`Narrow::narrow`] asks the only question the seam is about,
//! which is whether the value the GPU will hold is a number, and
//! answers `None` when it is not. That one test subsumes the poisoned
//! input as well, because a `NaN` or an infinity narrows to one.
//!
//! **Underflow is not the same fact and is not refused.** A coordinate
//! below about `1e-45` narrows to `0.0`, and `0.0` IS the nearest
//! `f32` to it — the value moves by less than the seam's own
//! resolution, which is what narrowing means. An infinity is the
//! nearest `f32` to nothing at all, and that asymmetry is the whole of
//! what this door decides.
//!
//! # What a refusal MEANS belongs to the caller
//!
//! The door says a value cannot cross; it does not say what to draw
//! instead, because that answer differs by lane and each lane already
//! has a vocabulary for it. A scene refuses as a whole
//! ([`crate::scene::SceneError::UndrawablePosition`]) because a part of
//! a solid drawn without the rest is a lie about the solid; an overlay
//! leg is simply not drawn, which is what an empty overlay lane already
//! means; a frame whose projection will not narrow is the projection
//! refusal the pane already reports. What must not differ is the TEST,
//! and that is here.
//!
//! # The one home, and what a reader cannot grep for
//!
//! Every `f64` → `f32` in this crate goes through this module. That
//! claim has no mechanical guard and can have none cheaply: the
//! property is a cast between two FLOAT types and the only instrument
//! short of a type check is `rg 'as f32'`, which cannot tell
//! `crate::pane::features`' `usize as f32` indent step — the one other
//! `as f32` under `crates/viewer/src`, and not this seam — from a
//! narrowing. What holds the rule instead is that nothing outside this
//! file needs to spell the cast: the shapes below cover the positions,
//! the pairs, the matrices and the scalars the seam actually carries.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::geom_core::Point3;

/// **The display seam's narrowing**, for one shape of `f64` value.
///
/// Implemented for `f64` itself, for an array of anything that narrows
/// — which covers a pair, a triple and a column-major 4x4 matrix in one
/// impl — and for [`Point3<f64>`], whose narrowed shape is the `[f32; 3]`
/// a GPU position buffer holds.
pub trait Narrow {
    /// The `f32`-valued shape this narrows to.
    type Narrowed;

    /// This value at `f32`, or `None` when any component of the RESULT
    /// is not finite.
    ///
    /// The module header says why the test is on the result: a finite
    /// `f64` past `f32::MAX` narrows to an infinity, and no check on
    /// the input can see it.
    #[must_use]
    fn narrow(self) -> Option<Self::Narrowed>;
}

impl Narrow for f64 {
    type Narrowed = f32;

    fn narrow(self) -> Option<f32> {
        // The crate's one `as f32`. Every other narrowing reaches it
        // through this line, which is what makes the refusal below the
        // seam's single policy rather than one of several.
        let narrowed = self as f32;
        narrowed.is_finite().then_some(narrowed)
    }
}

impl<T, const N: usize> Narrow for [T; N]
where
    T: Narrow + Copy,
    T::Narrowed: Copy + Default,
{
    type Narrowed = [T::Narrowed; N];

    /// **All of it or none of it.** A position with two components the
    /// GPU can hold and one it cannot is not a position, so the first
    /// component that refuses refuses the whole array — which is also
    /// what makes a matrix's answer the matrix's and not a per-entry
    /// question the caller has to re-ask.
    fn narrow(self) -> Option<[T::Narrowed; N]> {
        let mut out = [T::Narrowed::default(); N];
        for (slot, value) in out.iter_mut().zip(self) {
            *slot = value.narrow()?;
        }
        Some(out)
    }
}

impl Narrow for Point3<f64> {
    type Narrowed = [f32; 3];

    fn narrow(self) -> Option<[f32; 3]> {
        [self.x, self.y, self.z].narrow()
    }
}

/// **What the door decides**, over the values that decide it.
///
/// The rows are the seam's boundary and its two failure directions.
/// Every figure here is executed rather than quoted: the module header
/// names `7e307` as a coordinate the add-profile form accepts, and
/// [`a_finite_coordinate_a_person_can_author_does_not_narrow`] is what
/// holds that sentence to the arithmetic.
#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::Narrow;
    use pncad::geom_core::Point3;

    /// An ordinary value crosses, and crosses as itself.
    #[test]
    fn an_ordinary_coordinate_narrows_to_itself() {
        assert_eq!(0.5_f64.narrow(), Some(0.5_f32));
        assert_eq!([1.0_f64, -2.0, 3.0].narrow(), Some([1.0_f32, -2.0, 3.0]));
        assert_eq!(
            Point3::new(1.0_f64, 2.0, 3.0).narrow(),
            Some([1.0_f32, 2.0, 3.0])
        );
    }

    /// **The defect the door exists for**: every input is a number, at
    /// every guard, and the value the GPU would hold is not.
    ///
    /// `7e307` is the add-profile form's witness — a corner a person
    /// can type, which replays, flattens and reaches
    /// `crate::pane::viewport`'s overlay lanes.
    #[test]
    fn a_finite_coordinate_a_person_can_author_does_not_narrow() {
        for value in [7.0e307_f64, 8.0e307, 1.0e308, f64::from(f32::MAX) * 2.0] {
            assert!(
                value.is_finite(),
                "the witness has to be a number at every guard: {value:e}"
            );
            assert_eq!(
                value.narrow(),
                None,
                "{value:e} narrows to an infinity and the seam must refuse it"
            );
            assert_eq!(
                [0.0_f64, value, 0.0].narrow(),
                None,
                "one bad component \
                 refuses the whole position"
            );
        }
    }

    /// A poisoned input is the same refusal, by the same test.
    #[test]
    fn a_poisoned_value_does_not_narrow() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(value.narrow(), None);
        }
        assert_eq!(Point3::new(0.0, f64::NAN, 0.0).narrow(), None);
    }

    /// **The boundary, both sides of it.** `f32::MAX` itself crosses;
    /// so does a value above it that ROUNDS to it, because rounding to
    /// the nearest `f32` is what narrowing is. What refuses is the
    /// value whose nearest `f32` is an infinity.
    #[test]
    fn the_boundary_is_where_the_rounding_stops_landing_on_a_number() {
        assert_eq!(f64::from(f32::MAX).narrow(), Some(f32::MAX));
        // Halfway between `f32::MAX` and the first `f32` above it,
        // which does not exist: below this the round is to `f32::MAX`.
        let last = f64::from(f32::MAX) * (1.0 + f64::from(f32::EPSILON) / 4.0);
        assert_eq!(
            last.narrow(),
            Some(f32::MAX),
            "{last:e} rounds to f32::MAX and is a narrowing, not an overflow"
        );
        let over = f64::from(f32::MAX) * (1.0 + f64::from(f32::EPSILON) / 2.0);
        assert_eq!(
            over.narrow(),
            None,
            "{over:e} rounds to an infinity and is not a narrowing of anything"
        );
    }

    /// **Underflow is a narrowing and is not refused**, which the
    /// module header argues is the asymmetry the door is about.
    #[test]
    fn a_value_below_the_smallest_f32_narrows_to_zero_and_is_allowed() {
        assert_eq!(1.0e-300_f64.narrow(), Some(0.0_f32));
        assert_eq!((-1.0e-300_f64).narrow(), Some(-0.0_f32));
    }

    /// A matrix is the array impl twice over: one entry refuses all
    /// sixteen, so a caller asks once.
    #[test]
    fn a_matrix_refuses_whole() {
        let mut matrix = [[0.0_f64; 4]; 4];
        for (column, index) in matrix.iter_mut().zip(0..) {
            column[index] = 1.0;
        }
        assert_eq!(
            matrix.narrow(),
            Some([
                [1.0_f32, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ])
        );
        matrix[2][3] = 1.0e308;
        assert_eq!(matrix.narrow(), None);
    }
}

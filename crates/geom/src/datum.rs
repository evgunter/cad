//! **An analytic kind's stored data, as a walk**: the value shapes a
//! stored datum has ([`DatumValue`]) and the fixed-width walk of one
//! analytic kind's fields ([`AnalyticData`]), named by the datum type
//! of whatever carries them ([`crate::SurfaceDatum`],
//! [`crate::CurveDatum`]).

use geom_core::{Point3, Real, Vec3};

/// One stored datum's value, by shape — what an [`AnalyticData`]
/// yields beside the datum's name.
#[derive(Clone, Copy, Debug)]
pub enum DatumValue<T: Real> {
    /// A location.
    Point(Point3<T>),
    /// A direction.
    Direction(Vec3<T>),
    /// A number.
    Scalar(T),
}

impl<T: Real> DatumValue<T> {
    /// The datum's scalars: a point's or direction's `x`, `y`, `z`, or
    /// the number alone.
    pub fn scalars(self) -> impl Iterator<Item = T> {
        match self {
            Self::Point(p) => [Some(p.x), Some(p.y), Some(p.z)],
            Self::Direction(v) => [Some(v.x), Some(v.y), Some(v.z)],
            Self::Scalar(s) => [Some(s), None, None],
        }
        .into_iter()
        .flatten()
    }
}

/// The most data any analytic kind stores (a spiric's six).
pub(crate) const ANALYTIC_DATA_MAX: usize = 6;

/// **Every stored datum of one analytic kind**, named by `D`, in the
/// variant's field order, read by iterating it.
#[derive(Clone, Copy, Debug)]
pub struct AnalyticData<T: Real, D>([Option<(D, DatumValue<T>)>; ANALYTIC_DATA_MAX]);

impl<T: Real, D> AnalyticData<T, D> {
    /// The walk of `N` data, `N` at most [`ANALYTIC_DATA_MAX`].
    ///
    /// The bound is a `const` assertion, evaluated only when `new` is
    /// monomorphized: a kind that outgrows the width fails the build of
    /// the first crate that instantiates its walk at a concrete scalar
    /// (`topo`, `mesh`, and every test build of them), not `geom`'s own
    /// build, whose walks are generic over `T`; `cargo check` and
    /// clippy stay silent.
    pub(crate) fn new<const N: usize>(data: [(D, DatumValue<T>); N]) -> Self {
        const {
            assert!(
                N <= ANALYTIC_DATA_MAX,
                "an analytic kind outgrew ANALYTIC_DATA_MAX"
            )
        };
        let mut slots = [const { None }; ANALYTIC_DATA_MAX];
        for (slot, datum) in slots.iter_mut().zip(data) {
            *slot = Some(datum);
        }
        Self(slots)
    }
}

impl<T: Real, D> IntoIterator for AnalyticData<T, D> {
    type Item = (D, DatumValue<T>);
    type IntoIter =
        core::iter::Flatten<core::array::IntoIter<Option<(D, DatumValue<T>)>, ANALYTIC_DATA_MAX>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().flatten()
    }
}

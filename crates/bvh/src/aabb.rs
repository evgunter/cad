//! Axis-aligned bounding boxes with a **containment contract**: a box
//! is a certified-conservative CACHE of "the entity's true locus lies
//! inside me", never a tolerance object (C10). Constructors here only
//! ever err outward:
//!
//! - brackets come from [`geom_core::Bounds`] (`lo()`/`hi()` enclose
//!   every real number the scalar stands for — exact at `f64`, the
//!   correctly-rounded enclosure at the interval scalar);
//! - padding rounds outward by construction ([`Aabb::padded`] widens
//!   its arithmetic by one ulp per side);
//! - poison (NaN anywhere) yields a poison box, and a poison box can
//!   never prove disjointness — [`Aabb::overlaps`] answers `true`, so
//!   poisoned geometry is never pruned (fail-safe direction of D4 ¶2).
//!
//! Raw `f64` comparisons are deliberate and sound here: the boxes are
//! conservative by construction, and overlap tests only ever *prune* —
//! no Q1 predicate, nothing semantic, is decided in this crate (crate
//! docs; D9's "results a function of exact tests only").

use geom_core::{Bounds, Point3};

/// A coordinate axis — named, total, no index arithmetic (the linalg
/// charter's no-indexing rule).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    /// The x axis.
    X,
    /// The y axis.
    Y,
    /// The z axis.
    Z,
}

/// An axis-aligned box `[min_x, max_x] × [min_y, max_y] × [min_z,
/// max_z]` (closed on every face — touching boxes overlap).
///
/// Plain public data (D2): the fields carry the convention. An
/// inverted box (`min > max` on some axis) is *empty* — it overlaps
/// nothing; a NaN bound makes the box *poison* — it overlaps
/// everything (module docs: poison never prunes).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    /// Lower x bound.
    pub min_x: f64,
    /// Lower y bound.
    pub min_y: f64,
    /// Lower z bound.
    pub min_z: f64,
    /// Upper x bound.
    pub max_x: f64,
    /// Upper y bound.
    pub max_y: f64,
    /// Upper z bound.
    pub max_z: f64,
}

/// NaN-propagating minimum (`f64::min` *ignores* NaN, which would
/// silently drop poison — D4 ¶2 forbids exactly that).
fn pmin(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.min(b)
    }
}

/// NaN-propagating maximum ([`pmin`]'s dual).
fn pmax(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.max(b)
    }
}

impl Aabb {
    /// The tightest box containing every point's bracket — the
    /// vertex-extent constructor (planar faces and `Line` edges build
    /// their boxes from exactly this plus [`Aabb::padded`]).
    ///
    /// `None` for an empty iterator (there is no empty-set box). Any
    /// poisoned coordinate poisons the whole box (module docs).
    ///
    /// Fold order is the iterator's order with a fixed left fold —
    /// min/max folds are order-insensitive in value, but the
    /// association order is fixed anyway (D9 discipline).
    pub fn from_points<T: Bounds>(points: impl IntoIterator<Item = Point3<T>>) -> Option<Self> {
        let mut it = points.into_iter();
        let first = it.next()?;
        let mut b = Self {
            min_x: first.x.lo(),
            min_y: first.y.lo(),
            min_z: first.z.lo(),
            max_x: first.x.hi(),
            max_y: first.y.hi(),
            max_z: first.z.hi(),
        };
        for p in it {
            b.min_x = pmin(b.min_x, p.x.lo());
            b.min_y = pmin(b.min_y, p.y.lo());
            b.min_z = pmin(b.min_z, p.z.lo());
            b.max_x = pmax(b.max_x, p.x.hi());
            b.max_y = pmax(b.max_y, p.y.hi());
            b.max_z = pmax(b.max_z, p.z.hi());
        }
        Some(b)
    }

    /// The hull of two boxes (used for internal-node boxes; poison
    /// propagates).
    pub fn hull(&self, other: &Self) -> Self {
        Self {
            min_x: pmin(self.min_x, other.min_x),
            min_y: pmin(self.min_y, other.min_y),
            min_z: pmin(self.min_z, other.min_z),
            max_x: pmax(self.max_x, other.max_x),
            max_y: pmax(self.max_y, other.max_y),
            max_z: pmax(self.max_z, other.max_z),
        }
    }

    /// Outward padding by `pad` meters on every face, then one ulp
    /// further outward per bound ([`f64::next_down`]/[`f64::next_up`])
    /// so the padding arithmetic's own rounding can never shave the
    /// pad. Conservative-only: a negative or NaN `pad` yields the
    /// poison box (which never prunes) rather than ever shrinking.
    pub fn padded(&self, pad: f64) -> Self {
        if pad.is_nan() || pad < 0.0 {
            return Self::poison();
        }
        Self {
            min_x: (self.min_x - pad).next_down(),
            min_y: (self.min_y - pad).next_down(),
            min_z: (self.min_z - pad).next_down(),
            max_x: (self.max_x + pad).next_up(),
            max_y: (self.max_y + pad).next_up(),
            max_z: (self.max_z + pad).next_up(),
        }
    }

    /// The all-NaN poison box: overlaps everything, prunes nothing.
    pub fn poison() -> Self {
        Self {
            min_x: f64::NAN,
            min_y: f64::NAN,
            min_z: f64::NAN,
            max_x: f64::NAN,
            max_y: f64::NAN,
            max_z: f64::NAN,
        }
    }

    /// Closed-box overlap, outward-safe: `true` unless the boxes are
    /// **definitely** disjoint on some axis. Every comparison is a
    /// strict `<` between finite-or-NaN `f64`s, so a NaN bound (poison)
    /// can never witness disjointness — poison never prunes.
    pub fn overlaps(&self, other: &Self) -> bool {
        !(self.max_x < other.min_x
            || other.max_x < self.min_x
            || self.max_y < other.min_y
            || other.max_y < self.min_y
            || self.max_z < other.min_z
            || other.max_z < self.min_z)
    }

    /// The lower bound on `axis`.
    pub fn min(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.min_x,
            Axis::Y => self.min_y,
            Axis::Z => self.min_z,
        }
    }

    /// The upper bound on `axis`.
    pub fn max(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.max_x,
            Axis::Y => self.max_y,
            Axis::Z => self.max_z,
        }
    }

    /// The box's centroid coordinate on `axis` (`(min + max) · 0.5`,
    /// exactly as written). NaN for poison bounds — the build's
    /// `total_cmp` ordering stays total regardless.
    pub fn centroid(&self, axis: Axis) -> f64 {
        (self.min(axis) + self.max(axis)) * 0.5
    }
}
